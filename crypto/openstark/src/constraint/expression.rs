use crate::{
    constraint::sparse_polynomial_expression::PolynomialExpression,
    polynomial::{DensePolynomial, SparsePolynomial},
};
use primefield::FieldElement;
use std::{
    cmp::{max, Ord},
    collections::BTreeSet,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};
use u256::{commutative_binop, noncommutative_binop};

#[derive(Clone, Debug)]
pub enum TraceExpression {
    Trace(usize, isize),
    PolynomialExpression(PolynomialExpression),
    Neg(Box<TraceExpression>),
    Add(Box<TraceExpression>, Box<TraceExpression>),
    Mul(Box<TraceExpression>, Box<TraceExpression>),
}

impl TraceExpression {
    pub fn degree(&self, trace_length: usize) -> usize {
        // note trace_length is trace_degree + 1!!
        0
    }
}

impl AddAssign<&TraceExpression> for TraceExpression {
    fn add_assign(&mut self, other: &Self) {
        *self = Self::Add(Box::new(self.clone()), Box::new(other.clone()));
    }
}

impl<T: Into<TraceExpression>> Sub<T> for TraceExpression {
    type Output = Self;

    fn sub(self, other: T) -> TraceExpression {
        self + Self::Neg(Box::new(other.into()))
    }
}

impl MulAssign<&TraceExpression> for TraceExpression {
    fn mul_assign(&mut self, other: &Self) {
        *self = Self::Mul(Box::new(self.clone()), Box::new(other.clone()));
    }
}

commutative_binop!(TraceExpression, Add, add, AddAssign, add_assign);
commutative_binop!(TraceExpression, Mul, mul, MulAssign, mul_assign);

impl Add<PolynomialExpression> for TraceExpression {
    type Output = Self;

    fn add(self, other: PolynomialExpression) -> Self {
        self + Self::PolynomialExpression(other)
    }
}

impl Sub<PolynomialExpression> for TraceExpression {
    type Output = Self;

    fn sub(self, other: PolynomialExpression) -> Self {
        self - Self::PolynomialExpression(other)
    }
}

impl Mul<PolynomialExpression> for TraceExpression {
    type Output = Self;

    fn mul(self, other: PolynomialExpression) -> Self {
        self * Self::PolynomialExpression(other)
    }
}

impl Mul<TraceExpression> for PolynomialExpression {
    type Output = TraceExpression;

    fn mul(self, other: TraceExpression) -> TraceExpression {
        other * self
    }
}

impl Sub<FieldElement> for TraceExpression {
    type Output = Self;

    fn sub(self, other: FieldElement) -> Self {
        self - PolynomialExpression::Constant(other)
    }
}

impl Sub<TraceExpression> for FieldElement {
    type Output = TraceExpression;

    fn sub(self, other: TraceExpression) -> TraceExpression {
        TraceExpression::Neg(Box::new(other - self))
    }
}

impl From<isize> for TraceExpression {
    fn from(i: isize) -> Self {
        TraceExpression::PolynomialExpression(PolynomialExpression::Constant(i.into()))
    }
}

impl Sub<TraceExpression> for isize {
    type Output = TraceExpression;

    fn sub(self, other: TraceExpression) -> TraceExpression {
        TraceExpression::Neg(Box::new(other - TraceExpression::from(self)))
    }
}

impl Mul<TraceExpression> for isize {
    type Output = TraceExpression;

    fn mul(self, other: TraceExpression) -> TraceExpression {
        other * TraceExpression::from(self)
    }
}
