// Includes actual CLI implementation and unchanged repository Montgomery code.
pub use crypto_lib::{ct_bignum, ecc};
#[path = "../src/bin/ecbench/isogeny_eval.rs"]
#[allow(dead_code)]
mod isogeny_eval;
