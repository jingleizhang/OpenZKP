use crate::{
    constraint::fraction::{from, Fraction},
    polynomial::{DensePolynomial, SparsePolynomial},
    constraint::trace_multinomial::TraceMultinomial,
};
use primefield::FieldElement;
use std::{
    cmp::max,
    collections::BTreeSet,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Debug)]
// TODO: maybe turn these all into sparse polynomials immediately?
pub enum Other {
    X,
    Constant(FieldElement),
    PeriodicColumn(SparsePolynomial),
}

#[derive(Clone, Debug)]
pub enum Term {
    Trace(usize, isize),
    Other(Other),
}

#[derive(Clone, Debug)]
pub enum Expression {
    Term(Term),
    Operation(Operation),
}

#[derive(Clone, Debug)]
pub enum Operation {
    Pow(Box<Term>, usize),
    Neg(Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

impl Term {
    pub fn degree(&self, trace_length: usize) -> usize {
        match self {
            Term::Trace(i, j) => trace_length - 1,
            Term::Other(other) => {
                match other {
                    Other::X => 1,
                    Other::Constant(c) => 0,
                    Other::PeriodicColumn(p) => p.degree(),
                }
            }
        }
    }
}

impl Expression {
    pub fn degree(&self, trace_length: usize) -> usize {
        match self {
            Expression::Term(term) => term.degree(trace_length),
            Expression::Operation(operation) => {
                match operation {
                    Operation::Add(a, b) => max(a.degree(trace_length), b.degree(trace_length)),
                    Operation::Mul(a, b) => a.degree(trace_length) + b.degree(trace_length),
                    Operation::Neg(a) => a.degree(trace_length),
                    Operation::Pow(a, n) => a.degree(trace_length) * n,
                    Operation::Div(a, b) => a.degree(trace_length) - b.degree(trace_length),
                }
            }
        }
    }

    pub fn eval_on_domain(
        &self,
        trace_table: &dyn Fn(usize, isize) -> DensePolynomial,
    ) -> DensePolynomial {
        let multinomial = TraceMultinomial::from(self.clone());

        let mut result = DensePolynomial::new(&[FieldElement::ZERO]);
        for (indices, coefficients) in multinomial.0 {
            let product = indices
                .iter()
                .fold(DensePolynomial::new(&[FieldElement::ONE]), |x, (i, j)| {
                    x * trace_table(*i, *j)
                });

            let numerator = SparsePolynomial::from(coefficients.numerator.clone());
            println!("{:?}", indices.clone());
            println!("# numerator terms = {:?}", numerator.len());
            println!("numerator degree = {:?}", numerator.degree());
            println!(
                "# of factors in denominator = {:?}",
                coefficients.denominator.len()
            );
            println!("");

            let mut increment = product;
            if numerator.len() < 20 {
                increment *= numerator;
            } else {
                increment *= DensePolynomial::from(numerator);
            }
            for factor in coefficients.denominator {
                increment /= factor;
            }

            result += increment;
        }
        result
    }

    pub fn eval(
        &self,
        trace_table: &dyn Fn(usize, isize) -> FieldElement,
        x: &FieldElement,
    ) -> FieldElement {
        match self {
            Expression::Term(term) => term.eval(trace_table, x),
            Expression::Operation(operation) => {
                match operation {
                    Operation::Add(a, b) => a.eval(trace_table, x) + b.eval(trace_table, x),
                    Operation::Mul(a, b) => a.eval(trace_table, x) * b.eval(trace_table, x),
                    Operation::Neg(a) => -&a.eval(trace_table, x),
                    Operation::Pow(a, n) => a.pow(*n).eval(trace_table, x),
                    Operation::Div(a, b) => a.eval(trace_table, x) / b.eval(trace_table, x),
                }
            }
        }
    }
}

impl Term {
    pub fn pow(&self, exponent: usize) -> Expression {
        match self {
            Term::Trace(i, j) => {
                match exponent {
                    2 => self.clone() * self.clone(),
                    _ => panic!(),
                }
            }
            Term::Other(other) => other.pow(exponent),
        }
    }

    pub fn eval(
        &self,
        trace_table: &dyn Fn(usize, isize) -> FieldElement,
        x: &FieldElement,
    ) -> FieldElement {
        match self {
            &Term::Trace(i, j) => trace_table(i, j),
            Term::Other(other) => {
                match other {
                    Other::X => x.clone(),
                    Other::Constant(c) => c.clone(),
                    Other::PeriodicColumn(p) => p.evaluate(&x),
                }
            }
        }
    }
}

impl Other {
    pub fn pow(&self, exponent: usize) -> Expression {
        SparsePolynomial::from(self.clone()).pow(exponent).into()
    }
}

impl From<i32> for Expression {
    fn from(value: i32) -> Self {
        Expression::Term(Term::Other(Other::Constant(value.into())))
    }
}

impl From<&FieldElement> for Expression {
    fn from(value: &FieldElement) -> Self {
        Expression::Term(Term::Other(Other::Constant(value.clone())))
    }
}

impl From<SparsePolynomial> for Expression {
    fn from(value: SparsePolynomial) -> Self {
        Expression::Term(Term::Other(Other::PeriodicColumn(value.clone())))
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Expression::Operation(Operation::Add(Box::new(self), Box::new(other)))
    }
}

impl Sub for Expression {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Expression::Operation(Operation::Add(
            Box::new(self),
            Box::new(Expression::Operation(Operation::Neg(Box::new(other)))),
        ))
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Expression::Operation(Operation::Mul(Box::new(self), Box::new(other)))
    }
}

impl Mul<Expression> for Term {
    type Output = Expression;

    fn mul(self, other: Expression) -> Expression {
        Expression::Term(self) * other
    }
}

impl Add<Expression> for Term {
    type Output = Expression;

    fn add(self, other: Expression) -> Expression {
        Expression::Term(self) + other
    }
}

impl Add<Term> for Expression {
    type Output = Expression;

    fn add(self, other: Term) -> Expression {
        other + self
    }
}

impl Sub<Expression> for Term {
    type Output = Expression;

    fn sub(self, other: Expression) -> Expression {
        Expression::Term(self) - other
    }
}

impl Add<Other> for Term {
    type Output = Expression;

    fn add(self, other: Other) -> Expression {
        Expression::Term(self) + Expression::Term(Term::Other(other))
    }
}

impl Sub<Other> for Term {
    type Output = Expression;

    fn sub(self, other: Other) -> Expression {
        Expression::Term(self) - Expression::Term(Term::Other(other))
    }
}

impl Sub<Term> for Other {
    type Output = Expression;

    fn sub(self, other: Term) -> Expression {
        Expression::Term(Term::Other(self)) - Expression::Term(other)
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Expression::Operation(Operation::Div(Box::new(self), Box::new(other)))
    }
}

impl Add for Term {
    type Output = Expression;

    fn add(self, other: Self) -> Expression {
        Expression::Term(self) + Expression::Term(other)
    }
}

impl Sub for Term {
    type Output = Expression;

    fn sub(self, other: Self) -> Expression {
        Expression::Term(self) - Expression::Term(other)
    }
}

impl Sub<Term> for Expression {
    type Output = Expression;

    fn sub(self, other: Term) -> Expression {
        self - Expression::Term(other)
    }
}

impl Mul for Term {
    type Output = Expression;

    fn mul(self, other: Self) -> Expression {
        Expression::Term(self) * Expression::Term(other)
    }
}

impl From<i32> for Term {
    fn from(value: i32) -> Self {
        Term::Other(Other::Constant(value.into()))
    }
}

impl From<&FieldElement> for Term {
    fn from(value: &FieldElement) -> Self {
        Term::Other(Other::Constant(value.clone()))
    }
}

impl From<FieldElement> for Term {
    fn from(value: FieldElement) -> Self {
        Term::Other(Other::Constant(value.clone()))
    }
}

impl From<SparsePolynomial> for Term {
    fn from(value: SparsePolynomial) -> Self {
        Term::Other(Other::PeriodicColumn(value.clone()))
    }
}

impl Sub for Other {
    type Output = Expression;

    fn sub(self, other: Self) -> Expression {
        Expression::Term(Term::Other(self)) - Expression::Term(Term::Other(other))
    }
}

impl Sub<Expression> for Other {
    type Output = Expression;

    fn sub(self, other: Expression) -> Expression {
        Expression::Term(Term::Other(self)) - other
    }
}

impl Sub<Other> for Expression {
    type Output = Expression;

    fn sub(self, other: Other) -> Expression {
        self - Expression::Term(Term::Other(other))
    }
}

impl Mul<Other> for Expression {
    type Output = Expression;

    fn mul(self, other: Other) -> Expression {
        self * Expression::Term(Term::Other(other))
    }
}

impl Mul<Expression> for Other {
    type Output = Expression;

    fn mul(self, other: Expression) -> Expression {
        other * self
    }
}

impl From<Other> for SparsePolynomial {
    fn from(other: Other) -> Self {
        match other {
            Other::X => Self::new(&[(FieldElement::ONE, 1)]),
            Other::Constant(c) => Self::new(&[(c.clone(), 0)]),
            Other::PeriodicColumn(p) => p.clone(),
        }
    }
}

impl From<Expression> for SparsePolynomial {
    fn from(expression: Expression) -> Self {
        match expression {
            Expression::Term(term) => {
                match term {
                    Term::Trace(i, j) => panic!(),
                    Term::Other(other) => other.into(),
                }
            }
            Expression::Operation(operation) => {
                match operation {
                    Operation::Add(a, b) => Self::from(*a) + Self::from(*b),
                    Operation::Mul(a, b) => Self::from(*a) * Self::from(*b),
                    Operation::Neg(a) => Self::from(Expression::from(-1)) * Self::from(*a),
                    Operation::Pow(a, n) => Self::from(a.pow(n)),
                    Operation::Div(..) => panic!(),
                }
            }
        }
    }
}
