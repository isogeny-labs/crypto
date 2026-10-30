// Compile the exact staged adapter module before runtime registration:
// rustc --edition 2021 --test -D warnings budget_harness.rs -o /tmp/koblitz-budget-tests
#[path = "../../src/ml_dataset/koblitz_m3.rs"]
mod koblitz_m3;
