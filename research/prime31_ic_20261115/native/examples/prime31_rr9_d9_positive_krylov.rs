//! Public-only positive RR9 coefficient extraction attempt on the frozen B8 input.
#[path = "prime31_rr9_public_macaulay.rs"]
mod parent;

fn main() {
    println!("{}", serde_json::to_string_pretty(
        &parent::rr9_d9_positive_krylov()).unwrap());
}
