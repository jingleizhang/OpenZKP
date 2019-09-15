use crate::{
    constraint::expression::{
        self, Expression,
        Other::{Constant, X},
    },
    polynomial::SparsePolynomial,
};
use primefield::FieldElement;
use std::prelude::v1::*;

pub struct Constraint {
    pub base:        Expression,
    pub denominator: Expression,
    pub numerator:   Expression,
}

impl Constraint {
    pub fn degree(&self, trace_length: usize) -> usize {
        self.base.degree(trace_length) + self.numerator.degree(trace_length)
            - self.denominator.degree(trace_length)
    }
}

pub fn combine_constraints(
    constraints: &[Constraint],
    coefficients: &[FieldElement],
    trace_length: usize,
) -> Expression {
    let max_degree: usize = constraints
        .iter()
        .map(|c| c.degree(trace_length))
        .max()
        .unwrap();
    let result_degree = max_degree.next_power_of_two() - 1;

    let mut result = Expression::from(0);
    for (i, constraint) in constraints.iter().enumerate() {
        if i == 30 {
            break;
        }
        let x =
            constraint.base.clone() * constraint.numerator.clone() / constraint.denominator.clone();
        let degree_adjustment = X.pow(
            result_degree + constraint.denominator.degree(trace_length)
                - constraint.base.degree(trace_length)
                - constraint.numerator.degree(trace_length),
        );

        result = result + Constant(coefficients[2 * i].clone()) * x.clone();
        result = result + Constant(coefficients[2 * i + 1].clone()) * x * degree_adjustment;
    }
    debug_assert_eq!(result.degree(trace_length), result_degree);
    result
}

// TODO: Show expression
#[cfg(feature = "std")]
impl std::fmt::Debug for Constraint {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Constraint(...)")
    }
}
