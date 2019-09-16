use crate::{
    constraint::{
        expression::{
            self, Expression,
            Other::{Constant, X},
        },
        sparse_polynomial_expression::SparsePolynomialExpression,
    },
    polynomial::{DensePolynomial, SparsePolynomial},
};
use primefield::FieldElement;
use std::{collections::BTreeMap, prelude::v1::*};

pub struct Constraint {
    pub base:        Expression,
    pub denominator: SparsePolynomialExpression,
    pub numerator:   SparsePolynomialExpression,
}

impl Constraint {
    pub fn degree(&self, trace_length: usize) -> usize {
        self.base.degree(trace_length) + self.numerator.degree() - self.denominator.degree()
    }
}

pub fn combine_constraints(
    constraints: &[Constraint],
    coefficients: &[FieldElement],
    trace_length: usize,
) -> GroupedConstraints {
    let max_degree: usize = constraints
        .iter()
        .map(|c| c.degree(trace_length))
        .max()
        .unwrap();
    let result_degree = max_degree.next_power_of_two() - 1;

    let mut result = GroupedConstraints::new();
    for (i, constraint) in constraints.iter().enumerate() {
        if i == 30 {
            break;
        }

        let degree_adjustment = X.pow(
            result_degree + constraint.denominator.degree()
                - constraint.base.degree(trace_length)
                - constraint.numerator.degree(),
        );

        result.insert(
            (constraint.numerator.clone(), constraint.denominator.clone()),
            Constant(coefficients[2 * i].clone()) * constraint.base.clone(),
        );
        result.insert(
            (constraint.numerator.clone(), constraint.denominator.clone()),
            Constant(coefficients[2 * i + 1].clone()) * constraint.base.clone() * degree_adjustment,
        );
    }
    // debug_assert_eq!(result.degree(trace_length), result_degree);
    result
}

pub struct GroupedConstraints(
    BTreeMap<(SparsePolynomialExpression, SparsePolynomialExpression), Expression>,
);

impl GroupedConstraints {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(
        &mut self,
        key: (SparsePolynomialExpression, SparsePolynomialExpression),
        value: Expression,
    ) {
        *self.0.entry(key).or_insert(Expression::from(0)) += value;
    }

    pub fn eval_on_domain(
        &self,
        trace_table: &dyn Fn(usize, isize) -> DensePolynomial,
    ) -> DensePolynomial {
        DensePolynomial::new(&[FieldElement::ZERO])
    }

    pub fn eval(
        &self,
        trace_table: &dyn Fn(usize, isize) -> FieldElement,
        x: &FieldElement,
    ) -> FieldElement {
        FieldElement::ZERO
    }
}

// TODO: Show expression
#[cfg(feature = "std")]
impl std::fmt::Debug for Constraint {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Constraint(...)")
    }
}
