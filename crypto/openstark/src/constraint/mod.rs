mod constraint;
mod expression;
mod fraction;
mod trace_multinomial;

pub use constraint::Constraint;
pub use expression::{
    Expression,
    Other::{self, Constant, PeriodicColumn, X},
    Term::{self, Trace},
};

pub use constraint::combine_constraints;
