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

impl From<PolynomialExpression> for TraceExpression {
    fn from(p: PolynomialExpression) -> Self {
        Self::PolynomialExpression(p)
    }
}

impl From<FieldElement> for TraceExpression {
    fn from(x: FieldElement) -> Self {
        TraceExpression::PolynomialExpression(PolynomialExpression::Constant(x))
    }
}

impl From<isize> for TraceExpression {
    fn from(i: isize) -> Self {
        Self::from(i.into())
    }
}

impl<T: Into<TraceExpression>> Add<T> for TraceExpression {
    type Output = Self;

    fn add(self, other: T) -> TraceExpression {
        Self::Add(Box::new(self.clone()), Box::new(other.into()))
    }
}

impl<T: Into<TraceExpression>> Sub<T> for TraceExpression {
    type Output = Self;

    fn sub(self, other: T) -> TraceExpression {
        self + Self::Neg(Box::new(other.into()))
    }
}

impl<T: Into<TraceExpression>> Mul<T> for TraceExpression {
    type Output = Self;

    fn mul(self, other: T) -> TraceExpression {
        Self::Mul(Box::new(self.clone()), Box::new(other.into()))
    }
}

impl AddAssign<TraceExpression> for TraceExpression {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other.clone()
    }
}

impl Sub<TraceExpression> for FieldElement {
    type Output = TraceExpression;

    fn sub(self, other: TraceExpression) -> TraceExpression {
        TraceExpression::Neg(Box::new(other - self))
    }
}

impl Sub<TraceExpression> for isize {
    type Output = TraceExpression;

    fn sub(self, other: TraceExpression) -> TraceExpression {
        TraceExpression::Neg(Box::new(other - TraceExpression::from(self)))
    }
}

impl Mul<TraceExpression> for PolynomialExpression {
    type Output = TraceExpression;

    fn mul(self, other: TraceExpression) -> TraceExpression {
        other * self
    }
}

impl Mul<TraceExpression> for isize {
    type Output = TraceExpression;

    fn mul(self, other: TraceExpression) -> TraceExpression {
        other * TraceExpression::from(self)
    }
}
