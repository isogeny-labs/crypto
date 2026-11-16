#[path="../hec_kernel631_20261115/arith.rs"] mod arith;
use arith::{base,Curve,Div,Field,Pol,E,O,P,Z};
use hec_native_ic::{fp::Fp,jac,poly::Poly};
use std::{fs,collections::HashMap};
include!("functions.rs");
include!("gate.rs");
include!("controls.rs");
fn main(){run_theta();}
