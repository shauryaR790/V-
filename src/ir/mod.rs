//! Thin v++ intermediate representation.

mod lower;
mod types;

pub use lower::{lower_program, lower_program_with_enums};
pub use types::*;
