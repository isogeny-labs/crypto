#![allow(dead_code)]
mod sparse; mod scheduler; mod bridge; mod bqi; mod check; mod continuation;
mod equations {include!("equation_limits.rs");}
mod original {
 include!("budget_core.rs");
 use crate::{equations,continuation,scheduler,check,bridge,bqi};
 use scheduler::search::SearchLimits;
 #[derive(Clone,Copy)] struct Caps {expansion:equations::Limits,continuation:continuation::Limits}
 include!("policy_core.rs");
}
