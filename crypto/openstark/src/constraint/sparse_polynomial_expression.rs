use crate::{
    constraint::{
        fraction::{from, Fraction},
        trace_multinomial::TraceMultinomial,
    },
    polynomial::{DensePolynomial, SparsePolynomial},
};
use core::cmp::Ordering;
use lazy_static::*;
use primefield::FieldElement;
use std::{
    cmp::{max, Ord},
    collections::BTreeSet,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct SparsePolynomialExpression(SparsePolynomial);

pub fn X() -> SparsePolynomialExpression {
    SparsePolynomialExpression(SparsePolynomial::new(&[(FieldElement::ONE, 1)]))
}

pub fn Constant(x: isize) -> SparsePolynomialExpression {
    SparsePolynomialExpression(SparsePolynomial::new(&[(x.into(), 0)]))
}

pub fn PeriodicColumn(p: SparsePolynomial) -> SparsePolynomialExpression {
    SparsePolynomialExpression(p.clone())
}

impl SparsePolynomialExpression {
    pub fn pow(&self, n: usize) -> Self {
        SparsePolynomialExpression(self.0.pow(n))
    }

    pub fn degree(&self) -> usize {
        self.0.degree()
    }
}

impl Ord for SparsePolynomialExpression {
    fn cmp(&self, other: &Self) -> Ordering {
        self.degree().cmp(&other.degree())
    }
}

impl Sub for SparsePolynomialExpression {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        SparsePolynomialExpression(self.0 - other.0)
    }
}

impl Sub<FieldElement> for SparsePolynomialExpression {
    type Output = Self;

    fn sub(self, other: FieldElement) -> Self {
        self - SparsePolynomialExpression(SparsePolynomial::new(&[(other, 0)]))
    }
}

impl Sub<isize> for SparsePolynomialExpression {
    type Output = Self;

    fn sub(self, other: isize) -> Self {
        self - Constant(other)
    }
}
