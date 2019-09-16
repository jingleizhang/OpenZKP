mod constraint;
mod expression;
mod fraction;
mod sparse_polynomial_expression;
mod trace_multinomial;

pub use constraint::Constraint;
pub use expression::{
    Expression,
    Other::{self, PeriodicColumn},
    Term::{self, Trace},
};

pub use constraint::combine_constraints;
pub use sparse_polynomial_expression::{Constant, X};
