use hec_native_ic::jac::{Curve,Div};use crate::{core::Primitive,linear::Proof};
#[derive(Clone,Debug)]pub struct Context{pub c:Curve,pub d:Div,pub target:Div,pub ell:u64,pub sm:u64}
#[derive(Clone,Debug)]pub struct Cover{pub context:Context,pub z:u64,pub reps:Vec<(u64,u64)>,pub primitives:Vec<Primitive>,pub branches:Vec<(u64,Proof)>,pub k:u64}
