//! Unit tests U1-U11 of SPEC 11. They run inside the `l10 selftest` subcommand because the
//! included lab file's own #[cfg(test)] module needs serde_json (see DEVIATIONS.md).
use std::process::Command;

fn run(name: &str) {
    let st = Command::new(env!("CARGO_BIN_EXE_l10")).args(["selftest", "--name", name]).status().unwrap();
    assert!(st.success(), "selftest {} failed", name);
}

#[test]
fn u1_eval_counts() {
    run("u1_eval_counts");
}

#[test]
fn u2_zeta_and_lambda() {
    run("u2_zeta_and_lambda");
}

#[test]
fn u3_jac() {
    run("u3_jac");
}

#[test]
fn u4_line_trials() {
    run("u4_line_trials");
}

#[test]
fn u5_krylov() {
    run("u5_krylov");
}

#[test]
fn u6_bm_fib() {
    run("u6_bm_fib");
}

#[test]
fn u7_identity_quotient() {
    run("u7_identity_quotient");
}

#[test]
fn u8_keys() {
    run("u8_keys");
}

#[test]
fn u9_enum_p13() {
    run("u9_enum_p13");
}

#[test]
fn u10_t5_p31() {
    run("u10_t5_p31");
}

#[test]
fn u11_family_q() {
    run("u11_family_q");
}

