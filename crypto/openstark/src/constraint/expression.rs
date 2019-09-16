use crate::{
    constraint::{
        fraction::{from, Fraction},
        trace_multinomial::TraceMultinomial,
    },
    polynomial::{DensePolynomial, SparsePolynomial},
};
use primefield::FieldElement;
use std::{
    cmp::max,
    collections::BTreeSet,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

enum PolynomialExpression {
    X,
    Constant,
    PeriodicColumn,
    Operation(Op<Self>),
}

enum TraceExpression {
    PolynomialExpression(PolynomialExpression),
    Trace(usize, isize),
    Operation(Op<Self>),
}

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
pub enum Op<T> {
    Pow(Box<Term>, usize),
    Neg(Box<T>),
    Add(Box<T>, Box<T>),
    Mul(Box<T>, Box<T>),
    Div(Box<T>, Box<T>),
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

fn mul_assign(p: &mut DensePolynomial, other_polynomial: SparsePolynomial) {
    if other_polynomial.len() < 20 {
        *p *= other_polynomial;
    } else {
        *p *= DensePolynomial::from(other_polynomial);
    }
}

// struct GroupedExpression(BTreeStruct<(SparsePolynomial, SparsePolynomial),
// TraceExpression>); impl GroupedExpression {
//     fn from_constraints(constraints: &[Constraint]) -> Self {
//         let mut map = BTreeMap::new();
//         for constraint in constraints {
//             map.entry((constraint.numerator, constraint.denominator))
//                 .or_insert(TraceExpression::ZERO) += constraint.base;
//         }
//         Self(map)
//     }
// }

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
        let mut partial_result = DensePolynomial::new(&[FieldElement::ZERO]);
        let mut last_outer_indices = (0, 0);
        let mut divisors: BTreeSet<SparsePolynomial> = BTreeSet::new();
        for (i, (indices, coefficients)) in multinomial.0.iter().enumerate() {
            let numerator = SparsePolynomial::from(coefficients.numerator.clone());
            println!("{:?}", indices.clone());
            println!("# numerator terms = {:?}", numerator.len());
            println!("numerator degree = {:?}", numerator.degree());
            println!(
                "# of factors in denominator = {:?}",
                coefficients.denominator.len()
            );
            println!("");

            match indices.len() {
                0 => {
                    let mut increment = DensePolynomial::from(numerator);
                    for factor in coefficients.denominator.clone() {
                        increment /= factor;
                    }
                    result += increment;
                }
                1 => {
                    let (i, j) = indices[0];
                    let mut increment = trace_table(i, j);
                    mul_assign(&mut increment, numerator);
                    for factor in coefficients.denominator.clone() {
                        increment /= factor;
                    }
                    result += increment;
                }
                2 => {
                    if indices[0] != last_outer_indices {
                        // panic!();
                        // println!("last_outer_indices = {:?}", last_outer_indices);
                        let (i, j) = last_outer_indices;
                        let mut semi_result = trace_table(i, j) * partial_result;
                        for divisor in &divisors {
                            semi_result /= divisor.clone();
                        }
                        result += semi_result;

                        partial_result = DensePolynomial::new(&[FieldElement::ZERO]);
                        last_outer_indices = indices[0];
                        divisors.clear();
                    }
                    // assert_eq!(indices[0], (0, 0));
                    // assert_eq!(indices[1], (0, 0));
                    let (i, j) = indices[1];
                    let mut increment = trace_table(i, j);
                    mul_assign(&mut increment, numerator);

                    for factor in coefficients.denominator.difference(&divisors) {
                        // panic!();
                        partial_result *= factor.clone();
                    }
                    for factor in divisors.difference(&coefficients.denominator) {
                        // panic!();
                        increment *= factor.clone();
                    }
                    // println!("divisors = {:?}", divisors);
                    divisors.extend(coefficients.denominator.clone());
                    // println!("after extending, divisors = {:?}", divisors);

                    partial_result += increment;
                }
                _ => panic!(),
            }
            // if i == 2 {break;}
        }
        // bd3997ab0e0e4c33e39fbb9691a7af0436351386000000000000000000000000
        // 73d41b70735c412f11990e6662618c34ef3fa3d7000000000000000000000000
        // should be e6b1e1ee4d722870e3861798c5af768517dd582d000000000000000000000000

        // i = 2:
        // e6b1e1ee4d722870e3861798c5af768517dd582d000000000000000000000000
        // 73d41b70735c412f11990e6662618c34ef3fa3d7000000000000000000000000
        // println!("last_outer_indices = {:?}", last_outer_indices);
        let (i, j) = last_outer_indices;
        let mut semi_result = trace_table(i, j) * partial_result;
        for divisor in &divisors {
            // println!("divisor = {:?}", divisor);
            semi_result /= divisor.clone();
        }
        result += semi_result;
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

impl AddAssign for Expression {
    fn add_assign(&mut self, other: Self) {
        *self = Expression::Operation(Operation::Add(Box::new(self.clone()), Box::new(other)));
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
