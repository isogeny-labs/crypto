//! Shared plumbing: committed-file reads, hashing, the check ledger.

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::process::Command;

pub const REPO: &str = "/home/user/crypto";
pub const STUDY: &str = "research/postgb_linear_stage_20261026";

/// The bytes of `path` at `commit` (`git show commit:path`); the working tree is never read.
pub fn git_show(commit: &str, path: &str) -> Vec<u8> {
    let out = Command::new("git")
        .args(["show", &format!("{commit}:{path}")])
        .current_dir(REPO)
        .output()
        .expect("git");
    if !out.status.success() {
        panic!("git show {commit}:{path} failed: {}", String::from_utf8_lossy(&out.stderr));
    }
    out.stdout
}

pub fn git_try(commit: &str, path: &str) -> Option<Vec<u8>> {
    let out = Command::new("git")
        .args(["show", &format!("{commit}:{path}")])
        .current_dir(REPO)
        .output()
        .ok()?;
    out.status.success().then_some(out.stdout)
}

pub fn git_json(commit: &str, path: &str) -> Value {
    serde_json::from_slice(&git_show(commit, path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

pub fn git(args: &[&str]) -> (bool, String) {
    let out = Command::new("git").args(args).current_dir(REPO).output().expect("git");
    (out.status.success(), String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn full_commit(c: &str) -> String {
    let (ok, s) = git(&["rev-parse", c]);
    assert!(ok, "unknown commit {c}");
    s
}

pub fn sha_hex(b: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(b);
    h.finalize().iter().map(|x| format!("{x:02x}")).collect()
}

pub fn file_sha(p: &Path) -> Option<String> {
    std::fs::read(p).ok().map(|b| sha_hex(&b))
}

pub fn read_json(p: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap_or_else(|e| panic!("{}: {e}", p.display())))
        .unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

pub fn write_json(p: &Path, v: &Value) {
    let mut s = serde_json::to_string_pretty(v).unwrap();
    s.push('\n');
    std::fs::write(p, s).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
}

pub fn rel(a: f64, b: f64) -> f64 {
    if a == b {
        0.0
    } else {
        (a - b).abs() / a.abs().max(b.abs())
    }
}

pub fn num(v: &Value) -> f64 {
    v.as_f64().unwrap_or_else(|| panic!("not a number: {v}"))
}

/// The ledger of checks: every check is recorded, a failing one also as a problem.
#[derive(Default)]
pub struct Ledger {
    pub checks: Vec<Value>,
    pub problems: Vec<Value>,
}

impl Ledger {
    pub fn check(&mut self, name: &str, ok: bool, detail: Value) -> bool {
        if detail.is_null() {
            self.checks.push(json!({"check": name, "ok": ok}));
        } else {
            self.checks.push(json!({"check": name, "ok": ok, "detail": detail}));
        }
        if !ok {
            self.problems.push(json!({"check": name, "detail": detail}));
        }
        ok
    }
}

pub fn arg(args: &[String], key: &str) -> Option<String> {
    args.iter().position(|a| a == key).and_then(|i| args.get(i + 1).cloned())
}

pub fn need(args: &[String], key: &str) -> String {
    arg(args, key).unwrap_or_else(|| panic!("missing {key}"))
}
