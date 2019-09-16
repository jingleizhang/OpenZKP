use crate::{
    constraint::{
        sparse_polynomial_expression::PolynomialExpression,
        sparse_polynomial_expression::PolynomialExpression::X,
    },
    polynomial::{DensePolynomial, SparsePolynomial},
};
use crate::constraint::expression::TraceExpression;
use crate::constraint::sparse_polynomial_expression::PolynomialExpression::Constant;
use primefield::FieldElement;
use std::{collections::BTreeMap, prelude::v1::*};

pub struct Constraint {
    pub base:        TraceExpression,
    pub denominator: PolynomialExpression,
    pub numerator:   PolynomialExpression,
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
    BTreeMap<(PolynomialExpression, PolynomialExpression), TraceExpression>,
);

impl GroupedConstraints {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, key: (PolynomialExpression, PolynomialExpression), value: TraceExpression) {
        *self.0.entry(key).or_insert(TraceExpression::from(0)) += value;
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
