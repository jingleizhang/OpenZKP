mod constraint;
mod expression;
// mod fraction;
mod sparse_polynomial_expression;
// mod trace_multinomial;

pub use constraint::{combine_constraints, Constraint};
pub use expression::TraceExpression::Trace;
pub use sparse_polynomial_expression::PolynomialExpression::{Constant, PeriodicColumn, X};
