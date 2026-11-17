//! optlevel: how close does an implementation run to what its machine can do?
//! Definition, formulas, tiers and failure modes: docs/OPTIMIZATION_LEVEL.md.
// Index loops mirror the C reference kernel line for line; the tables of controls and passes are plain data.
#![allow(clippy::needless_range_loop, clippy::type_complexity, clippy::should_implement_trait, clippy::new_without_default, clippy::field_reassign_with_default)]
// The per-isa kernel passes are `unsafe fn` for one reason, stated once at `kernels::passes`: the CPU must have the features they were compiled for.
#![allow(clippy::missing_safety_doc)]
pub mod cli;
pub mod cmd;
pub mod host;
pub mod kernels;
pub mod plot;
pub mod run;
pub mod score;
pub mod util;
