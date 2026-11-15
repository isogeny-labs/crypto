//! Native instrument: one PKM Kummer-tower point-decomposition instance exported and solved in
//! several algebraically equivalent presentations (`[pcc:unverified]` instrument construction, no claim).
//!
//! Subcommands (all outputs JSON with a sealed `sha256`):
//!   cell --t T --seed S --out DIR [--ratio R] [--prime P] [--require-breal N] [--slug NAME] [--nested-ts 2,3]
//!   targets --cell DIR --n-sat N --n-unsat N [--planted N] [--planted-distinct N] --seed S --m M --out DIR
//!   export --cell DIR --targets DIR --target NAME|all --m M --presentation P|all --out DIR [--format ms|native|both]
//!   solve --system FILE [--max-degree D (default 64)] --deadline SECS --out FILE [--extract] [--extract-max-q Q (default 4096)]
//!   verify-msolve --system-json FILE --msolve-out FILE --out FILE [--seed S]
//!   describe --system FILE [--out FILE]
//! P is one of chain-tower, tree-tower, mono-tower, chain-direct, mono-direct, sym-direct, sym-tower.
//! `cell` and `targets` wrap the m8 derivations (no L1 reuse); the modules of that example and
//! `pkm_oracle_program/algebra.rs` are included unchanged by `#[path]`. See `present.rs` for the
//! presentations and their solution semantics.
#![allow(dead_code)]
#![recursion_limit = "512"]

#[path = "pkm_oracle_program/algebra.rs"]
mod algebra;
#[path = "pkm_m8_panel/cell.rs"]
mod cell;
#[path = "pkm_present_panel/cli.rs"]
mod cli;
#[path = "pkm_present_panel/decode.rs"]
mod decode;
#[path = "pkm_m8_panel/ec.rs"]
mod ec;
#[path = "pkm_present_panel/mpoly.rs"]
mod mpoly;
#[path = "pkm_m8_panel/native_solve.rs"]
mod native_solve;
#[path = "pkm_present_panel/present.rs"]
mod present;
#[path = "pkm_m8_panel/system.rs"]
mod system;
#[path = "pkm_m8_panel/targets.rs"]
mod targets;
#[path = "pkm_m8_panel/util.rs"]
mod util;
#[path = "pkm_m8_panel/verify.rs"]
mod verify;
#[cfg(test)]
#[path = "pkm_present_panel/tests.rs"]
mod tests;

use serde_json::{json, Value};

pub fn arg_opt(args: &[String], name: &str) -> Option<String> {
    let find = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    find(name).or_else(|| if name == "--seed" { find("--master-seed") } else { None })
}
pub fn arg_req(args: &[String], name: &str) -> Result<String, String> {
    arg_opt(args, name).ok_or_else(|| format!("missing {name}"))
}
pub fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

/// Identity of this build: hash of its own sources and of the included modules, plus the git commit.
pub fn tool_info() -> Value {
    let sources = [
        include_str!("pkm_present_panel.rs"),
        include_str!("pkm_present_panel/cli.rs"),
        include_str!("pkm_present_panel/decode.rs"),
        include_str!("pkm_present_panel/mpoly.rs"),
        include_str!("pkm_present_panel/present.rs"),
        include_str!("pkm_m8_panel/cell.rs"),
        include_str!("pkm_m8_panel/ec.rs"),
        include_str!("pkm_m8_panel/native_solve.rs"),
        include_str!("pkm_m8_panel/system.rs"),
        include_str!("pkm_m8_panel/targets.rs"),
        include_str!("pkm_m8_panel/util.rs"),
        include_str!("pkm_m8_panel/verify.rs"),
        include_str!("pkm_oracle_program/algebra.rs"),
    ];
    let mut all = String::new();
    for s in sources {
        all.push_str(&util::sha256_hex(s.as_bytes()));
    }
    let git = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .or_else(|| option_env!("PKM_PRESENT_GIT_COMMIT").map(str::to_string));
    json!({
        "name": "pkm_present_panel",
        "source_sha256": util::sha256_hex(all.as_bytes()),
        "git_commit": git,
        "git_note": "null when run outside a git checkout and PKM_PRESENT_GIT_COMMIT was not set at build",
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first().cloned() else {
        eprintln!("usage: pkm_present_panel <cell|targets|export|solve|verify-msolve|describe> ...");
        std::process::exit(2);
    };
    let rest = &args[1..];
    let res = match cmd.as_str() {
        "cell" => cli::run_cell(rest),
        "targets" => cli::run_targets(rest),
        "export" => cli::run_export(rest),
        "solve" => cli::run_solve(rest),
        "verify-msolve" => cli::run_verify_msolve(rest),
        "describe" => cli::run_describe(rest),
        other => Err(format!("unknown subcommand {other}")),
    };
    if let Err(e) = res {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
