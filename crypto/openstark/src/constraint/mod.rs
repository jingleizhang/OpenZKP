mod constraint;
mod expression;
mod fraction;
mod trace_multinomial;

pub use constraint::Constraint;
pub use expression::Expression;
pub use expression::Other::Constant;
pub use expression::Other::X;
pub use expression::Term;
pub use expression::Term::Trace;
pub use expression::Other;
pub use expression::Other::PeriodicColumn;

pub use constraint::combine_constraints;
