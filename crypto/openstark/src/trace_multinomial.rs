use crate::{
    expression::{Expression, Operation, Other, Term},
    fraction::Fraction,
    polynomial::{DensePolynomial, SparsePolynomial},
};
use std::{
    cmp::{max, Ordering},
    collections::BTreeMap,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
};
use u256::{commutative_binop, noncommutative_binop};

#[derive(Clone, Debug)]
pub struct TraceMultinomial(pub BTreeMap<Vec<(usize, isize)>, Fraction>);

// TODO: unify with SparsePolynomial?
impl TraceMultinomial {
    fn new(trace_arguments: Vec<(usize, isize)>, coefficient: Fraction) -> TraceMultinomial {
        let mut m = BTreeMap::new();
        m.insert(trace_arguments, coefficient);
        TraceMultinomial(m)
    }
}

impl AddAssign<&Self> for TraceMultinomial {
    fn add_assign(&mut self, other: &Self) {
        for (degree, coefficient) in &other.0 {
            *self.0.entry(degree.clone()).or_insert(Fraction::zero()) += coefficient;
        }
    }
}

impl MulAssign<&Self> for TraceMultinomial {
    fn mul_assign(&mut self, other: &Self) {
        let mut result = BTreeMap::new();
        for (degree, coefficient) in &self.0 {
            for (other_degree, other_coefficient) in &other.0 {
                let mut new_indices = [&degree[..], &other_degree[..]].concat();
                new_indices.sort();

                *result.entry(new_indices).or_insert(Fraction::zero()) +=
                    coefficient * other_coefficient;
            }
        }
        *self = Self(result);
    }
}

commutative_binop!(TraceMultinomial, Add, add, AddAssign, add_assign);
commutative_binop!(TraceMultinomial, Mul, mul, MulAssign, mul_assign);

impl Div<SparsePolynomial> for TraceMultinomial {
    type Output = TraceMultinomial;

    fn div(self, denominator: SparsePolynomial) -> Self {
        let mut result = self.clone();
        for coefficient in result.0.values_mut() {
            *coefficient /= denominator.clone();
        }
        result
    }
}

impl From<Expression> for TraceMultinomial {
    fn from(expression: Expression) -> Self {
        match expression {
            Expression::Term(term) => {
                match term {
                    Term::Trace(i, j) => Self::new(vec![(i, j)], Fraction::one()),
                    Term::Other(other) => Self::new(vec![], other.into()),
                }
            }
            Expression::Operation(operation) => {
                match operation {
                    Operation::Add(a, b) => Self::from(*a) + Self::from(*b),
                    Operation::Mul(a, b) => Self::from(*a) * Self::from(*b),
                    Operation::Neg(a) => Self::from(Expression::from(-1)) * Self::from(*a),
                    Operation::Pow(a, n) => Self::from(a.pow(n)),
                    Operation::Div(a, b) => Self::from(*a) / SparsePolynomial::from(*b),
                }
            }
        }
    }
}
