//! Native pinned-backend interface admission, never a curve solve or benchmark.
use crate::Result;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
fn execute(exe: &Path, input: &str) -> Result<(String, String)> {
    let mut child = Command::new(exe)
        .args([
            "--lang=smt2",
            "--ff-solver=split",
            "--ff-bitsum",
            "--tlimit=5000",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    child
        .stdin
        .take()
        .ok_or("missing backend stdin")?
        .write_all(input.as_bytes())
        .map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8(out.stdout).map_err(|e| e.to_string())?;
    let stderr = String::from_utf8(out.stderr).map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!(
            "backend admission exited {}: {stderr}; {stdout}",
            out.status
        ));
    }
    Ok((stdout, stderr))
}
fn scalar_model(text: &str) -> Result<BigUint> {
    let tokens: Vec<_> = text
        .split(|c: char| c.is_whitespace() || c == '(' || c == ')')
        .filter(|s| !s.is_empty())
        .collect();
    if tokens.first() != Some(&"sat") {
        return Err("backend did not return SAT".into());
    }
    let ff: Vec<_> = tokens.iter().filter_map(|t| t.strip_prefix("ff")).collect();
    if ff.len() != 1 || ff[0].is_empty() || !ff[0].bytes().all(|b| b.is_ascii_digit()) {
        return Err("malformed scalar field model".into());
    }
    BigUint::parse_bytes(ff[0].as_bytes(), 10).ok_or("bad scalar model decimal".into())
}
pub fn admit(exe: &Path, out: &mut impl Write) -> Result<()> {
    for p in [BigUint::from(101u32), torus_s4::curve25519_prime()] {
        let query=format!("(set-logic QF_FF)\n(set-option :produce-models true)\n(define-sort F () (_ FiniteField {p}))\n(declare-const x F)\n(assert (= (ff.mul x x) (as ff9 F)))\n(check-sat)\n(get-value (x))\n");
        let (stdout, stderr) = execute(exe, &query)?;
        let x = scalar_model(&stdout)?;
        if x >= p || (&x * &x) % &p != BigUint::from(9u32) {
            return Err("native scalar replay rejected backend model".into());
        }
        writeln!(out,"{}",json!({"kind":"backend_scalar_admission","p":p.to_string(),"query":query,"stdout":stdout,"stderr":stderr,"native_scalar_replay":true})).map_err(|e|e.to_string())?;
    }
    // The explicit bitsum/Boolean uniqueness example from the established method.
    let p = torus_s4::curve25519_prime();
    let mut query = format!("(set-logic QF_FF)\n(define-sort F () (_ FiniteField {p}))\n");
    for i in 0..15 {
        query.push_str(&format!("(declare-const b{i} F)\n(assert (= (ff.mul b{i} (ff.add b{i} (as ff{} F))) (as ff0 F)))\n",&p-1u32));
    }
    query.push_str(&format!(
        "(assert (= (ff.bitsum {}) (as ff0 F)))\n(assert (= b14 (as ff1 F)))\n(check-sat)\n",
        (0..15)
            .map(|i| format!("b{i}"))
            .collect::<Vec<_>>()
            .join(" ")
    ));
    let (stdout, stderr) = execute(exe, &query)?;
    if stdout.trim() != "unsat" {
        return Err("backend bitsum interface control not UNSAT".into());
    }
    let v: Value = json!({"kind":"backend_bitsum_admission","query":query,"stdout":stdout,"stderr":stderr,"scope":"fixed parser/backend control; no curve absence certificate or performance evidence"});
    writeln!(out, "{v}").map_err(|e| e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn model_parser_refuses_unknown_and_ambiguous() {
        assert!(scalar_model("unknown").is_err());
        assert!(scalar_model("sat ((x (as ff3 F)) (y (as ff4 F)))").is_err());
        assert!(scalar_model("sat ((x (as ff-3 F)))").is_err());
    }
}
