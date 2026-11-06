//! `instr`: callgrind instruction counts for both binaries, restricted to the
//! timed region.  The count does not depend on load, so kernels can be
//! counted in parallel, and a kernel counted more than once must agree.

use crate::compare::{
    build_provenance, kernel_set, list_kernels, load_weights, provenance_problems, run_capture,
    TOOL_SCHEMA,
};
use crate::host::first_line;
use crate::summarize::{self, markdown, Arm, ArmRows, Params, Per};
use crate::util::{write_json, SplitMix64};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

#[derive(clap::Args, Clone, Debug)]
pub struct InstrArgs {
    #[arg(long)]
    pub filter: Vec<String>,
    #[arg(long)]
    pub full: bool,
    #[arg(long)]
    pub weights: Option<PathBuf>,
    #[arg(long)]
    pub allow_missing: bool,
    #[arg(long)]
    pub allow_provenance_mismatch: bool,
    /// Kernels counted at once (Ir does not depend on load).
    #[arg(long, default_value_t = 1)]
    pub jobs: usize,
    /// Count each kernel this many times and check that Ir agrees.
    #[arg(long, default_value_t = 1)]
    pub repeat: usize,
}

/// The number after `Collected :` in callgrind's summary.
pub fn parse_collected(stderr: &str) -> Option<u64> {
    let line = stderr.lines().find(|l| l.contains("Collected"))?;
    let after = line.split_once(':')?.1;
    after.split_whitespace().next()?.parse().ok()
}

pub fn instr_one(binary: &Path, kid: &str) -> Result<Value, String> {
    let mut cmd = Command::new("valgrind");
    cmd.args([
        "--tool=callgrind",
        "--callgrind-out-file=/dev/null",
        "--instr-atstart=no",
    ])
    .arg(binary)
    .args(["run", "--filter", kid, "--exact", "--instr"])
    .env("RAYON_NUM_THREADS", "1");
    let (stdout, stderr, ok) = run_capture(cmd, None)?;
    if !ok {
        return Err(format!(
            "callgrind failed for {kid}:\n{}",
            crate::compare::tail(&stderr, 3000)
        ));
    }
    let mut row = crate::compare::json_lines(&stdout)
        .into_iter()
        .next()
        .ok_or_else(|| format!("{kid}: no row"))?;
    let ir = parse_collected(&stderr)
        .ok_or_else(|| format!("{kid}: no 'Collected' line from callgrind"))?;
    row["ir"] = json!(ir);
    Ok(row)
}

/// Counts a kernel `repeat` times.  `Ir` is deterministic for one binary and
/// one input; a kernel whose count differs between runs is reported, not averaged.
fn instr_repeated(binary: &Path, kid: &str, repeat: usize) -> Result<(Value, Vec<u64>), String> {
    let mut rows = Vec::new();
    for _ in 0..repeat.max(1) {
        rows.push(instr_one(binary, kid)?);
    }
    let irs: Vec<u64> = rows.iter().map(|r| r["ir"].as_u64().unwrap_or(0)).collect();
    Ok((rows.swap_remove(0), irs))
}

pub fn run_instr(base: &Path, cand: &Path, out: &Path, a: &InstrArgs) -> Result<i32, String> {
    if first_line("valgrind", &["--version"]).is_empty() {
        eprintln!("refused: valgrind is not installed");
        return Ok(2);
    }
    let (base_prov, cand_prov) = (build_provenance(base), build_provenance(cand));
    let bad = provenance_problems(&base_prov, &cand_prov);
    if !bad.is_empty() && !a.allow_provenance_mismatch {
        for b in &bad {
            eprintln!("refused: {b}");
        }
        return Ok(2);
    }
    let (kernels, only_base, only_cand) = kernel_set(
        &list_kernels(base, a.full, &a.filter)?,
        &list_kernels(cand, a.full, &a.filter)?,
    );
    if (!only_base.is_empty() || !only_cand.is_empty()) && !a.allow_missing {
        eprintln!("refused: kernels on one side only. base only: {only_base:?}; candidate only: {only_cand:?}");
        return Ok(2);
    }
    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let t0 = std::time::Instant::now();
    type Counted = Result<((Value, Vec<u64>), (Value, Vec<u64>)), String>;
    let results: Mutex<Vec<Option<Counted>>> =
        Mutex::new((0..kernels.len()).map(|_| None).collect());
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..a.jobs.max(1) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= kernels.len() {
                    break;
                }
                let kid = &kernels[i].id;
                let r = instr_repeated(base, kid, a.repeat)
                    .and_then(|x| Ok((x, instr_repeated(cand, kid, a.repeat)?)));
                if let Ok(((_, ba), (_, ca))) = &r {
                    eprintln!("{kid}: base Ir={} cand Ir={}", ba[0], ca[0]);
                }
                results.lock().unwrap()[i] = Some(r);
            });
        }
    });
    let mut per: Per = Per::new();
    let mut nondeterministic: Vec<String> = Vec::new();
    let mut ir_doc = serde_json::Map::new();
    for (k, r) in kernels.iter().zip(results.into_inner().unwrap()) {
        let ((rb_row, ba), (rc_row, ca)) = r.ok_or("worker did not finish")??;
        let mut rows = ArmRows::default();
        let obs = |row: &Value, ir: u64| json!({"median_ns": ir, "fingerprint": row["fingerprint"], "round": 0});
        rows.arm_mut(Arm::A).insert(0, obs(&rb_row, ba[0]));
        rows.arm_mut(Arm::B).insert(0, obs(&rc_row, ca[0]));
        per.insert(k.id.clone(), rows);
        if ba.iter().any(|x| *x != ba[0]) || ca.iter().any(|x| *x != ca[0]) {
            nondeterministic.push(k.id.clone());
        }
        ir_doc.insert(k.id.clone(), json!({"base": ba, "cand": ca}));
    }
    let weights = load_weights(a.weights.as_deref())?;
    let params = Params {
        exclude_contended: false,
        deterministic: true,
        ..Params::default()
    };
    let mut s = summarize::summarize(&kernels, &per, &weights, &mut SplitMix64::new(0), &params);
    // A count that differs between repeated runs is not an exact observation.
    s.qualified = s.qualified && nondeterministic.is_empty();
    if !nondeterministic.is_empty() {
        s.warnings.push(format!(
            "Ir differs between repeated runs of {nondeterministic:?}: not deterministic"
        ));
    }
    if !only_base.is_empty() || !only_cand.is_empty() {
        s.warnings.push(format!(
            "kernels left out: base only {only_base:?}, candidate only {only_cand:?}"
        ));
        s.qualified = false;
    }
    for (k, v) in [
        ("schema", json!(TOOL_SCHEMA)),
        ("mode", json!("callgrind_ir")),
        ("unit", json!("Ir")),
        ("base", json!(base.display().to_string())),
        ("cand", json!(cand.display().to_string())),
        ("base_provenance", base_prov),
        ("cand_provenance", cand_prov),
        ("valgrind", json!(first_line("valgrind", &["--version"]))),
        ("repeat", json!(a.repeat)),
        ("jobs", json!(a.jobs)),
        (
            "seconds",
            json!((t0.elapsed().as_secs_f64() * 10.0).round() / 10.0),
        ),
        ("ir", Value::Object(ir_doc)),
    ] {
        s.extra.insert(k.into(), v);
    }
    write_json(
        &out.join("instr.json"),
        &serde_json::to_value(&s).map_err(|e| e.to_string())?,
    )?;
    let md = markdown(
        &s,
        "perfbench instruction-count comparison (callgrind Ir, 1 thread)",
    );
    std::fs::write(out.join("instr.md"), &md).map_err(|e| e.to_string())?;
    print!("{md}");
    Ok(if s.valid { 0 } else { 1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collected_is_read_from_the_summary_line() {
        assert_eq!(
            parse_collected("==1== Collected : 12345678\n==1== other"),
            Some(12345678)
        );
        assert_eq!(parse_collected("==12== Collected :   9"), Some(9));
        assert_eq!(parse_collected("nothing here"), None);
    }
}
