//! The IC candidate tournament (`research/ic_candidate_tournament_20260915`),
//! natively: `icprog tournament prepare | run | status | verify | propose`.
//!
//! This is the schema-1 improvement protocol of the frozen evaluator
//! `tournament.py` `1e5b2774…` (rounds 0019–0025), with its checker
//! `oracle.py` in the three versions those rounds pinned
//! (`tournament_oracle`).  Same records and hashes: canonical JSON
//! (`objhash`), the same contract, seal, fixtures, receipts, stage
//! summaries and decision; the same A/A, smoke, development, selection,
//! confirmation and replay stages, nested curve/target bootstrap with
//! `random.Random(751203)`, promotion and matched-rho gates; failures
//! retained, no retries, nothing overwritten.  `verify` re-derives a
//! historical Python round's summaries and decision from its trial
//! receipts and must agree with them field for field.
//!
//! What is not ported is refused rather than approximated: the
//! scientific-admission contract (`schema_version` 2), the bounded campaign
//! rules (`campaign_rules*.py`), generic workers and reference
//! qualification (see `NATIVE_PORT.md`).  `run` executes only rounds this
//! evaluator prepared, never a Python-evaluated or closed registration.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use clap::{Subcommand, ValueEnum};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::Zero;

use super::json::{self, py_eq, J};
use super::pyrandom::Random;
use super::stats;
use super::tournament_exec as exec;
use super::tournament_oracle::{self as oracle, require};
use super::tournament_store::{self as store, Store};

pub const UNIT: &str = "valgrind-3.22-amd64-Ir";
pub const STAGES: [&str; 6] = [
    "aa",
    "smoke",
    "development",
    "selection",
    "confirmation",
    "replay",
];
const PHASES: [&str; 10] = [
    "startup_and_input",
    "curve_and_targets",
    "factor_base_and_tables",
    "collection_and_decomposition",
    "verify_filter_and_linear_algebra",
    "log_certification",
    "individual_log",
    "final_verification",
    "reporting_and_cleanup",
    "rho_solve",
];
/// The frozen evaluator whose semantics this module reproduces.
pub const TOURNAMENT_1E5B: &str =
    "1e5b27743bf3bf99a3834882ddde4992f658874438576e7532646feb655651cc";
pub const PORT: &str = "icprog-tournament-schema1-v1";

/// Seeds of registrations that are consumed or closed: every retained
/// schema-1 round (`runs/round-*`), the three bounded improvement attempts,
/// and the generic-backend qualification registrations (ic-autolab skill).
/// `prepare` refuses them and `run` refuses any round that carries one.
pub const CLOSED_SEEDS: &[i128] = &[
    20260915, 2026091603, 2026091604, 2026091605, 2026091606, 2026091607, 2026091608, 2026091609,
    2026091610, 2026091611, 2026091612, 2026091613, 2026091614, 2026091615, 2026091616, 2026091717,
    2026091818, 2026092119, 2026092120, 2026092323, 2026092424, 2026092525, 2026092551, 2026092552,
    2026092553, 2026092901, 2026092902, 2026093001,
];

type R<T> = Result<T, String>;

// ── records ─────────────────────────────────────────────────────────

pub fn objhash(v: &J) -> String {
    store::sha256_hex(json::dumps_compact(v, true, true).as_bytes())
}

fn finite(v: &J) -> R<()> {
    match v {
        J::Float(x) if !x.is_finite() => {
            Err("Out of range float values are not JSON compliant".into())
        }
        J::Arr(a) => a.iter().try_for_each(finite),
        J::Obj(kv) => kv.iter().try_for_each(|(_, v)| finite(v)),
        _ => Ok(()),
    }
}

/// `write(path, value, exclusive=…)`: sorted keys, two-space indent.
pub fn write(path: &Path, v: &J, exclusive: bool) -> R<()> {
    finite(v)?;
    let data = json::dumps(&json::sorted(v), 2) + "\n";
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    if exclusive {
        use std::io::Write as _;
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        f.write_all(data.as_bytes()).map_err(|e| e.to_string())
    } else {
        let tmp = path.with_file_name(format!(
            "{}.tmp",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
        std::fs::write(&tmp, data).map_err(|e| e.to_string())?;
        std::fs::rename(&tmp, path).map_err(|e| e.to_string())
    }
}

fn read(st: &Store, rel: &str) -> R<J> {
    json::parse(&st.read_text(rel)?).map_err(|e| format!("{rel}: {e}"))
}

fn s(v: &str) -> J {
    J::Str(v.into())
}

fn at<'a>(v: &'a J, key: &str) -> R<&'a J> {
    v.get(key).ok_or_else(|| format!("KeyError: '{key}'"))
}

fn str_at<'a>(v: &'a J, key: &str) -> R<&'a str> {
    at(v, key)?
        .as_str()
        .ok_or_else(|| format!("TypeError: {key} is not a string"))
}

fn set(v: &mut J, key: &str, value: J) {
    if let J::Obj(kv) = v {
        if let Some(slot) = kv.iter_mut().find(|(k, _)| k == key) {
            slot.1 = value;
        } else {
            kv.push((key.to_string(), value));
        }
    }
}

fn remove(v: &mut J, key: &str) -> Option<J> {
    if let J::Obj(kv) = v {
        if let Some(i) = kv.iter().position(|(k, _)| k == key) {
            return Some(kv.remove(i).1);
        }
    }
    None
}

fn num(v: &J) -> R<f64> {
    v.as_f64()
        .ok_or_else(|| "TypeError: not a number".to_string())
}

// ── Python numerics ─────────────────────────────────────────────────

/// `statistics.mean` of floats: the exact rational mean, correctly
/// rounded.
pub fn py_mean(xs: &[f64]) -> R<f64> {
    require(!xs.is_empty(), "mean requires at least one data point")?;
    let parts: Vec<(BigInt, u32)> = xs.iter().map(|&x| stats::exact(x)).collect();
    let k = parts.iter().map(|p| p.1).max().unwrap_or(0);
    let mut total = BigInt::zero();
    for (m, e) in parts {
        total += m << (k - e) as usize;
    }
    let (sign, mag) = total.into_parts();
    let den = BigUint::from(xs.len()) << k as usize;
    let q = stats::div_to_f64(&mag, &den);
    Ok(if sign == Sign::Minus { -q } else { q })
}

#[derive(Clone, Copy, Debug)]
enum Num {
    Int(i128),
    Float(f64),
}

fn as_num(v: &J) -> R<Num> {
    match v {
        J::Int(i) => Ok(Num::Int(*i)),
        J::Float(x) => Ok(Num::Float(*x)),
        J::Bool(b) => Ok(Num::Int(i128::from(*b))),
        _ => Err("TypeError: not a number".into()),
    }
}

fn int_to_f64(i: i128) -> f64 {
    let (n, d) = (BigUint::from(i.unsigned_abs()), BigUint::from(1u8));
    let q = stats::div_to_f64(&n, &d);
    if i < 0 {
        -q
    } else {
        q
    }
}

/// Python's true division `a / b` of ints and floats.
fn py_div(a: Num, b: Num) -> R<f64> {
    match (a, b) {
        (_, Num::Int(0)) => Err("ZeroDivisionError: division by zero".into()),
        (_, Num::Float(0.0)) => Err("ZeroDivisionError: float division by zero".into()),
        (Num::Int(x), Num::Int(y)) => {
            let q = stats::div_to_f64(
                &BigUint::from(x.unsigned_abs()),
                &BigUint::from(y.unsigned_abs()),
            );
            Ok(if (x < 0) != (y < 0) && x != 0 { -q } else { q })
        }
        (Num::Int(x), Num::Float(y)) => Ok(int_to_f64(x) / y),
        (Num::Float(x), Num::Int(y)) => Ok(x / int_to_f64(y)),
        (Num::Float(x), Num::Float(y)) => Ok(x / y),
    }
}

/// `statistics.median`: the middle value, or the mean of the middle two
/// (`(a + b) / 2` in the values' own types).
fn py_median(values: &[Num]) -> R<Num> {
    require(!values.is_empty(), "no median for empty data")?;
    let mut v = values.to_vec();
    v.sort_by(|a, b| {
        let f = |n: &Num| match n {
            Num::Int(i) => *i as f64,
            Num::Float(x) => *x,
        };
        f(a).partial_cmp(&f(b)).expect("no NaN")
    });
    let n = v.len();
    if n % 2 == 1 {
        return Ok(v[n / 2]);
    }
    let sum = match (v[n / 2 - 1], v[n / 2]) {
        (Num::Int(a), Num::Int(b)) => Num::Int(a + b),
        (Num::Int(a), Num::Float(b)) | (Num::Float(b), Num::Int(a)) => {
            Num::Float(int_to_f64(a) + b)
        }
        (Num::Float(a), Num::Float(b)) => Num::Float(a + b),
    };
    Ok(Num::Float(py_div(sum, Num::Int(2))?))
}

/// CPython 3.12's builtin `sum` over floats: Neumaier compensation, the
/// compensation added once at the end.
pub fn py_sum(xs: &[f64]) -> f64 {
    let mut iter = xs.iter();
    let Some(&first) = iter.next() else {
        return 0.0;
    };
    let (mut f, mut c) = (first, 0.0f64);
    for &x in iter {
        let t = f + x;
        if f.abs() >= x.abs() {
            c += (f - t) + x;
        } else {
            c += (x - t) + f;
        }
        f = t;
    }
    if c != 0.0 && c.is_finite() {
        f += c;
    }
    f
}

// ── the frozen inputs ───────────────────────────────────────────────

/// Which evaluator a round was frozen with, and so which semantics apply.
#[derive(Clone, Debug)]
pub struct Protocol {
    pub oracle: oracle::Version,
    /// Frozen by this native evaluator rather than a Python one.
    #[allow(dead_code)]
    pub native: bool,
}

/// `frozen_inputs`: the contract against its seal, every pinned file, the
/// frozen evaluator and checker.  `evaluator` is the running executable,
/// which a native round requires to be its own frozen copy.
pub fn frozen_inputs(st: &Store, evaluator: Option<&Path>) -> R<(J, J, Vec<J>, Protocol)> {
    let c = read(st, "contract.json")?;
    let seal = read(st, "seal.json")?;
    require(
        seal.get("contract_sha256") == Some(&J::Str(objhash(&c))),
        "changed contract",
    )?;
    for (rel, expected) in at(&c, "pinned_files")?
        .as_obj()
        .ok_or("TypeError: pinned_files")?
    {
        require(
            st.exists(rel) && Some(st.digest(rel)?.as_str()) == expected.as_str(),
            &format!("changed pinned artifact {rel}"),
        )?;
    }
    let shas = at(&c, "evaluator_sha256")?;
    let protocol = if let Some(native) = c.get("native_evaluator") {
        require(
            native.get("port") == Some(&s(PORT)),
            "round was frozen by a different native evaluator",
        )?;
        let want = str_at(shas, "icprog")?;
        require(
            st.digest("evaluator/icprog")? == want,
            "changed evaluator copy",
        )?;
        if let Some(exe) = evaluator {
            require(
                store::digest_file(exe)? == want,
                &format!(
                    "run the frozen evaluator copy: {}/evaluator/icprog",
                    st.label()
                ),
            )?;
        }
        Protocol {
            oracle: oracle::Version::Vae8f9b2a,
            native: true,
        }
    } else {
        let t = str_at(shas, "tournament.py")?;
        require(
            t == TOURNAMENT_1E5B,
            &format!(
                "this native port reproduces tournament.py {TOURNAMENT_1E5B}; the round pins \
                 {t}, which stays with its own frozen evaluator"
            ),
        )?;
        require(
            st.digest("evaluator/tournament.py")? == t,
            "run the frozen evaluator copy",
        )?;
        let o = str_at(shas, "oracle.py")?;
        require(st.digest("evaluator/oracle.py")? == o, "changed checker")?;
        Protocol {
            oracle: oracle::Version::from_sha256(o)
                .ok_or_else(|| format!("unported checker version oracle.py {o}"))?,
            native: false,
        }
    };
    require(
        c.get("schema_version")
            .is_some_and(|v| py_eq(v, &J::Int(1))),
        "only the schema-1 improvement contract is ported",
    )?;
    let fixtures = read(st, "fixtures.json")?;
    let arms = read(st, "candidates.json")?
        .as_arr()
        .ok_or("TypeError: candidates")?
        .to_vec();
    Ok((c, fixtures, arms, protocol))
}

// ── profiles ────────────────────────────────────────────────────────

fn py_isspace(c: char) -> bool {
    c.is_whitespace() || ('\x1c'..='\x1f').contains(&c)
}

/// `str.splitlines()`.
fn splitlines(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        let breaks = matches!(
            c,
            '\n' | '\r'
                | '\x0b'
                | '\x0c'
                | '\x1c'
                | '\x1d'
                | '\x1e'
                | '\u{85}'
                | '\u{2028}'
                | '\u{2029}'
        );
        if breaks {
            out.push(&text[start..i]);
            let mut end = i + c.len_utf8();
            if c == '\r' {
                if let Some(&(j, '\n')) = chars.peek() {
                    chars.next();
                    end = j + 1;
                }
            }
            start = end;
        }
    }
    if start < text.len() {
        out.push(&text[start..]);
    }
    out
}

/// `re.findall(r'Collected\s*:\s*(\d+)', text)`.
fn collected(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(i) = rest.find("Collected") {
        let mut t = rest[i + "Collected".len()..].trim_start_matches(py_isspace);
        if let Some(after) = t.strip_prefix(':') {
            t = after.trim_start_matches(py_isspace);
            let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() {
                out.push(digits);
            }
        }
        rest = &rest[i + "Collected".len()..];
    }
    out
}

fn py_int_str(s: &str) -> R<i128> {
    oracle::py_int(&J::Str(s.into()))
}

/// `parse_profiles(directory, compressed=…)`: phase costs by interval.
pub fn parse_profiles(st: &Store, dir: &str, compressed: bool) -> R<BTreeMap<String, i128>> {
    let prefix = format!("{dir}/");
    let mut paths: Vec<String> = st
        .files_under(dir)
        .into_iter()
        .filter(|p| {
            let Some(name) = p.strip_prefix(&prefix) else {
                return false;
            };
            !name.contains('/')
                && name.starts_with("callgrind.out")
                && (!compressed || name.ends_with(".gz"))
        })
        .collect();
    paths.sort();
    require(!paths.is_empty(), "missing instruction profiles")?;
    let mut phases: BTreeMap<String, i128> = BTreeMap::new();
    let mut terminated = 0;
    let mut parts = BTreeSet::new();
    for path in paths {
        let name = &path[prefix.len()..];
        if !compressed && Path::new(name).extension().is_some_and(|e| e == "gz") {
            continue;
        }
        let bytes = if compressed {
            st.gunzip(&path)?
        } else {
            st.read(&path)?
        };
        let text = String::from_utf8(bytes).map_err(|e| format!("UnicodeDecodeError: {e}"))?;
        let lines = splitlines(&text);
        let one = |key: &str| -> R<String> {
            let values: Vec<&str> = lines
                .iter()
                .filter_map(|l| l.strip_prefix(key))
                .map(|v| v.trim_matches(py_isspace))
                .collect();
            require(
                values.len() == 1,
                &format!("missing/duplicate profile field {key}"),
            )?;
            Ok(values[0].to_string())
        };
        require(one("events:")? == "Ir", "incompatible instruction unit")?;
        let part = one("part:")?;
        require(!parts.contains(&part), "duplicate profile interval")?;
        parts.insert(part);
        let total = py_int_str(&one("summary:")?)?;
        require(
            total >= 0 && py_int_str(&one("totals:")?)? == total,
            "invalid profile total",
        )?;
        let trigger = one("desc: Trigger:")?;
        let phase = if trigger == "Program termination" {
            terminated += 1;
            "reporting_and_cleanup".to_string()
        } else {
            require(
                trigger.starts_with("Client Request: "),
                "unknown profiling boundary",
            )?;
            trigger["Client Request: ".len()..].to_string()
        };
        require(PHASES.contains(&phase.as_str()), "unknown cost phase")?;
        *phases.entry(phase).or_insert(0) += total;
    }
    require(terminated == 1, "missing or duplicate final interval")?;
    let stderr = st.read_text(&format!("{dir}/stderr.txt"))?;
    let found = collected(&stderr);
    require(
        found.len() == 1,
        "missing whole-process instruction checksum",
    )?;
    require(
        phases.values().sum::<i128>() == py_int_str(&found[0])?,
        "phase costs do not sum to process cost",
    )?;
    Ok(phases)
}

fn phases_json(p: &BTreeMap<String, i128>) -> J {
    J::Obj(p.iter().map(|(k, v)| (k.clone(), J::Int(*v))).collect())
}

// ── trials ──────────────────────────────────────────────────────────

fn trial_rel(stage: &str, case: &J, arm: &str, rep: usize) -> R<String> {
    Ok(format!(
        "runs/{stage}/{}/{arm}/rep-{rep}",
        str_at(case, "id")?
    ))
}

fn summands(arm: &J) -> R<&J> {
    at(at(arm, "config")?, "summands")
}

/// `verify_receipt(directory, c, case, arm)`.
pub fn verify_receipt(st: &Store, dir: &str, c: &J, case: &J, arm: &J, p: &Protocol) -> R<J> {
    let r = read(st, &format!("{dir}/receipt.json"))?;
    require(
        r.get("case") == case.get("id") && r.get("cell") == case.get("cell"),
        "changed case identity",
    )?;
    require(
        r.get("arm") == arm.get("id") && r.get("unit") == c.get("unit"),
        "changed arm or unit",
    )?;
    let rep = dir
        .rsplit('/')
        .next()
        .unwrap_or("")
        .trim_start_matches("rep-");
    let rep = J::Int(py_int_str(rep)?);
    require(
        r.get("repetition").is_some_and(|v| py_eq(v, &rep)),
        "changed repetition",
    )?;
    require(
        r.get("case_sha256") == Some(&J::Str(objhash(case))),
        "changed case binding",
    )?;
    require(
        r.get("arm_sha256") == Some(&J::Str(objhash(arm))),
        "changed candidate binding",
    )?;
    for (path, h) in at(&r, "artifacts")?
        .as_obj()
        .ok_or("TypeError: artifacts")?
    {
        let rel = format!("{dir}/{path}");
        require(
            st.exists(&rel) && Some(st.digest(&rel)?.as_str()) == h.as_str(),
            &format!("changed trial artifact {path}"),
        )?;
    }
    if r.get("status") == Some(&s("VERIFIED")) {
        let mode = str_at(&r, "mode")?;
        let fixture = at(case, "fixture")?;
        let report = read(st, &format!("{dir}/profile/stdout.json"))?;
        let proof = oracle::verify(&report, fixture, mode, summands(arm)?, p.oracle)?;
        require(
            r.get("certificate").is_some_and(|v| py_eq(v, &proof)),
            "certificate summary changed",
        )?;
        let costs = parse_profiles(st, &format!("{dir}/profile"), true)?;
        require(
            r.get("phase_costs")
                .is_some_and(|v| py_eq(v, &phases_json(&costs)))
                && r.get("total_operations")
                    .is_some_and(|v| py_eq(v, &J::Int(costs.values().sum()))),
            "changed cost summary",
        )?;
        let native = read(st, &format!("{dir}/native/stdout.json"))?;
        let native_proof = oracle::verify(&native, fixture, mode, summands(arm)?, p.oracle)?;
        require(
            py_eq(at(&native_proof, "solutions")?, at(&proof, "solutions")?),
            "native/profile mismatch",
        )?;
    }
    Ok(r)
}

fn executed_job(case: &J, arm: &J) -> R<J> {
    let mut job = at(case, "job")?.clone();
    let mode = if arm.get("id") == Some(&s("rho")) {
        "rho"
    } else {
        "ic"
    };
    set(&mut job, "mode", s(mode));
    set(&mut job, "config", at(arm, "config")?.clone());
    Ok(job)
}

/// `run_trial(root, c, stage, case, arm, repetition)`.
fn run_trial(root: &Path, c: &J, stage: &str, case: &J, arm: &J, rep: usize, p: &Protocol) -> R<J> {
    let st = Store::Dir(root.to_path_buf());
    let rel = trial_rel(stage, case, str_at(arm, "id")?, rep)?;
    let dir = root.join(&rel);
    if dir.join("receipt.json").exists() {
        return verify_receipt(&st, &rel, c, case, arm, p);
    }
    require(
        !dir.exists(),
        "interrupted trial retained; start a new campaign rather than overwrite it",
    )?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let job = executed_job(case, arm)?;
    write(&dir.join("job.json"), &job, true)?;
    let binary = root.join(
        arm.get("binary_relative")
            .and_then(J::as_str)
            .unwrap_or("worker"),
    );
    let lim = at(c, "limits")?;
    let timeout = num(at(lim, "timeout_seconds")?)?;
    let memory = at(lim, "memory_bytes")?
        .as_i128()
        .ok_or("TypeError: memory")? as u64;
    let cpu = at(lim, "cpu")?.as_i128().ok_or("TypeError: cpu")? as usize;
    let profile_dir = dir.join("profile");
    let command: Vec<String> = [
        "valgrind",
        "--tool=callgrind",
        "--cache-sim=no",
        "--branch-sim=no",
        "--separate-threads=no",
        "--collect-atstart=yes",
        "--instr-atstart=yes",
    ]
    .iter()
    .map(|x| x.to_string())
    .chain([
        format!(
            "--callgrind-out-file={}",
            profile_dir.join("callgrind.out").display()
        ),
        binary.display().to_string(),
    ])
    .collect();
    let run = exec::execute(&command, &job, &profile_dir, timeout, memory, cpu)?;
    let mode = str_at(&job, "mode")?.to_string();
    let mut r = J::Obj(vec![
        ("schema_version".into(), J::Int(1)),
        ("stage".into(), s(stage)),
        ("arm".into(), at(arm, "id")?.clone()),
        ("case".into(), at(case, "id")?.clone()),
        ("cell".into(), at(case, "cell")?.clone()),
        ("case_sha256".into(), J::Str(objhash(case))),
        ("arm_sha256".into(), J::Str(objhash(arm))),
        ("repetition".into(), J::Int(rep as i128)),
        ("unit".into(), at(c, "unit")?.clone()),
        ("mode".into(), s(&mode)),
        ("status".into(), s("ERROR")),
        ("profile_process".into(), run.clone()),
        ("total_operations".into(), J::Null),
        ("phase_costs".into(), J::Null),
        ("certificate".into(), J::Null),
    ]);
    let fixture = at(case, "fixture")?;
    let summ = at(at(&job, "config")?, "summands")?.clone();
    let attempt = (|| -> R<()> {
        require(run.get("process_status") != Some(&s("TIMEOUT")), "TIMEOUT")?;
        require(
            run.get("exit_code") == Some(&J::Int(0)),
            "worker failed or incomplete",
        )?;
        let report = read(&st, &format!("{rel}/profile/stdout.json"))?;
        let proof = oracle::verify(&report, fixture, &mode, &summ, p.oracle)?;
        let costs = parse_profiles(&st, &format!("{rel}/profile"), false)?;
        let mut expected: BTreeSet<&str> = [
            "startup_and_input",
            "curve_and_targets",
            "final_verification",
            "reporting_and_cleanup",
        ]
        .into();
        if mode == "rho" {
            expected.insert("rho_solve");
        } else {
            expected.extend(PHASES.iter().filter(|&&x| x != "rho_solve"));
        }
        require(
            costs.keys().map(String::as_str).collect::<BTreeSet<_>>() == expected,
            "missing exclusive phases",
        )?;
        let native = exec::execute(
            &[binary.display().to_string()],
            &job,
            &dir.join("native"),
            timeout,
            memory,
            cpu,
        )?;
        set(&mut r, "native_process", native.clone());
        require(
            native.get("exit_code") == Some(&J::Int(0))
                && native.get("process_status") == Some(&s("EXITED")),
            "native worker failed",
        )?;
        let nreport = read(&st, &format!("{rel}/native/stdout.json"))?;
        let nproof = oracle::verify(&nreport, fixture, &mode, &summ, p.oracle)?;
        require(
            py_eq(at(&nproof, "solutions")?, at(&proof, "solutions")?),
            "native/profile answers differ",
        )?;
        let total: i128 = costs.values().sum();
        let order = oracle::py_int(at(fixture, "subgroup_order")?)?;
        let rank = proof.get("rank").cloned().unwrap_or(J::Null);
        let ratio = match &rank {
            J::Int(k) if *k != 0 => J::Float(py_div(Num::Int(total), Num::Int(*k))?),
            _ => J::Null,
        };
        set(&mut r, "status", s("VERIFIED"));
        set(&mut r, "total_operations", J::Int(total));
        set(&mut r, "phase_costs", phases_json(&costs));
        set(&mut r, "certificate", proof);
        set(
            &mut r,
            "normalized_S",
            J::Float(int_to_f64(total) / int_to_f64(order).sqrt()),
        );
        set(&mut r, "floor_operations", rank);
        set(&mut r, "ratio_to_floor", ratio);
        Ok(())
    })();
    if let Err(reason) = attempt {
        set(&mut r, "reason", J::Str(reason));
        let stderr = std::fs::read_to_string(profile_dir.join("stderr.txt")).unwrap_or_default();
        let status = if run.get("process_status") == Some(&s("TIMEOUT")) {
            "TIMEOUT"
        } else if stderr.contains("memory allocation") {
            "OOM"
        } else {
            "INVALID_OR_INCOMPLETE"
        };
        set(&mut r, "status", s(status));
        for key in ["normalized_S", "floor_operations", "ratio_to_floor"] {
            remove(&mut r, key);
        }
        set(&mut r, "total_operations", J::Null);
        set(&mut r, "phase_costs", J::Null);
        set(&mut r, "certificate", J::Null);
    }
    let mut profiles: Vec<PathBuf> = std::fs::read_dir(&profile_dir)
        .map_err(|e| e.to_string())?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with("callgrind.out"))
        })
        .collect();
    profiles.sort();
    for path in profiles {
        let data = std::fs::read(&path).map_err(|e| e.to_string())?;
        let zipped = path.with_file_name(format!(
            "{}.gz",
            path.file_name().expect("name").to_string_lossy()
        ));
        std::fs::write(&zipped, store::gzip_mtime0(&data)?).map_err(|e| e.to_string())?;
        std::fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    let prefix = format!("{rel}/");
    let artifacts: Vec<(String, J)> = st
        .files_under(&rel)
        .into_iter()
        .map(|f| {
            let short = f[prefix.len()..].to_string();
            st.digest(&f).map(|d| (short, J::Str(d)))
        })
        .collect::<R<_>>()?;
    set(&mut r, "artifacts", J::Obj(artifacts));
    write(&dir.join("receipt.json"), &r, true)?;
    Ok(r)
}

fn synthetic_arm(name: &str, base: &J) -> J {
    let mut arm = base.clone();
    set(&mut arm, "id", s(name));
    arm
}

fn load_stage(st: &Store, stage: &str, fixtures: &[J], arms: &[J], reps: usize) -> R<Vec<J>> {
    let mut rows = Vec::new();
    for case in fixtures {
        for arm in arms {
            for rep in 0..reps {
                let rel = format!(
                    "{}/receipt.json",
                    trial_rel(stage, case, str_at(arm, "id")?, rep)?
                );
                require(st.exists(&rel), "missing scheduled trial")?;
                rows.push(read(st, &rel)?);
            }
        }
    }
    Ok(rows)
}

// ── statistics and gates ────────────────────────────────────────────

fn ineligible(candidate: &str, reason: &str) -> J {
    J::Obj(vec![
        ("candidate".into(), s(candidate)),
        ("eligible".into(), J::Bool(false)),
        ("reason".into(), s(reason)),
    ])
}

fn exp_f(x: f64) -> f64 {
    x.exp()
}

fn per_cell(map: &[(String, f64)]) -> J {
    J::Obj(map.iter().map(|(k, v)| (k.clone(), J::Float(*v))).collect())
}

/// `comparison(rows, candidate_id, baseline=…, draws=…, match_support=…)`:
/// the paired geometric-mean ratio and its nested curve/target bootstrap.
pub fn comparison(
    rows: &[J],
    candidate: &str,
    baseline: &str,
    draws: usize,
    match_support: bool,
) -> R<J> {
    let arm_of = |r: &J| r.get("arm").and_then(J::as_str).map(str::to_string);
    let cases: BTreeSet<String> = rows
        .iter()
        .map(|r| str_at(r, "case").map(str::to_string))
        .collect::<R<_>>()?;
    let mut grouped: HashMap<(String, String), Vec<&J>> = HashMap::new();
    for r in rows {
        let a = arm_of(r).unwrap_or_default();
        if a == candidate || a == baseline {
            grouped
                .entry((str_at(r, "case")?.to_string(), a))
                .or_default()
                .push(r);
        }
    }
    // Cells in first-seen order (Python dict insertion order).
    let mut logs: Vec<(String, Vec<f64>)> = Vec::new();
    let mut native_logs: Vec<(String, Vec<f64>)> = Vec::new();
    let empty = Vec::new();
    for case in &cases {
        let a = grouped
            .get(&(case.clone(), baseline.to_string()))
            .unwrap_or(&empty);
        let b = grouped
            .get(&(case.clone(), candidate.to_string()))
            .unwrap_or(&empty);
        let reps = |xs: &[&J]| -> R<BTreeSet<i128>> {
            xs.iter()
                .map(|x| {
                    at(x, "repetition")?
                        .as_i128()
                        .ok_or_else(|| "TypeError: repetition".to_string())
                })
                .collect()
        };
        if a.is_empty()
            || b.is_empty()
            || a.len() != b.len()
            || reps(a)?.len() != a.len()
            || reps(b)?.len() != b.len()
            || reps(a)? != reps(b)?
        {
            return Ok(ineligible(candidate, "missing paired trials"));
        }
        let both: Vec<&J> = a.iter().chain(b.iter()).copied().collect();
        if both.iter().any(|x| {
            x.get("status") != Some(&s("VERIFIED"))
                || matches!(x.get("total_operations"), None | Some(J::Null))
        }) {
            return Ok(ineligible(candidate, "unverified or unpriced workload"));
        }
        let shas: BTreeSet<String> = both
            .iter()
            .map(|x| str_at(x, "case_sha256").map(str::to_string))
            .collect::<R<_>>()?;
        require(shas.len() == 1, "changed paired fixtures")?;
        if candidate != "rho" && baseline != "rho" {
            let fb = |xs: &[&J]| -> R<BTreeSet<String>> {
                xs.iter()
                    .map(|x| {
                        str_at(at(x, "certificate")?, "factor_base_sha256").map(str::to_string)
                    })
                    .collect()
            };
            if match_support {
                require(fb(&both)?.len() == 1, "changed paired factor-base support")?;
            } else {
                require(
                    fb(a)?.len() == 1 && fb(b)?.len() == 1,
                    "changed factor-base support within a policy arm",
                )?;
            }
        }
        let cell = str_at(a[0], "cell")?.to_string();
        let ops = |xs: &[&J]| -> R<Vec<Num>> {
            xs.iter()
                .map(|x| as_num(at(x, "total_operations")?))
                .collect()
        };
        let walls = |xs: &[&J]| -> R<Vec<Num>> {
            xs.iter()
                .map(|x| as_num(at(at(x, "native_process")?, "process_wall_seconds")?))
                .collect()
        };
        let ratio = py_div(py_median(&ops(b)?)?, py_median(&ops(a)?)?)?;
        push_cell(&mut logs, &cell, ratio.ln());
        let wall = py_div(py_median(&walls(b)?)?, py_median(&walls(a)?)?)?;
        push_cell(&mut native_logs, &cell, wall.ln());
    }
    require(!logs.is_empty(), "empty comparison")?;
    let cell_values: Vec<(String, f64)> = logs
        .iter()
        .map(|(k, xs)| py_mean(xs).map(|m| (k.clone(), m)))
        .collect::<R<_>>()?;
    let estimate = exp_f(py_mean(
        &cell_values.iter().map(|x| x.1).collect::<Vec<_>>(),
    )?);
    let mut rng = Random::new(751203);
    let mut cells: Vec<&String> = logs.iter().map(|(k, _)| k).collect();
    cells.sort();
    let lookup: HashMap<&str, (&Vec<f64>, &Vec<f64>)> = logs
        .iter()
        .zip(&native_logs)
        .map(|((k, l), (_, w))| (k.as_str(), (l, w)))
        .collect();
    let mut boot = Vec::with_capacity(draws);
    let mut native_boot = Vec::with_capacity(draws);
    for _ in 0..draws {
        let mut means = Vec::with_capacity(cells.len());
        let mut wall_means = Vec::with_capacity(cells.len());
        for _ in 0..cells.len() {
            let cell = cells[rng.randbelow(cells.len() as u32) as usize];
            let (l, w) = lookup[cell.as_str()];
            let idx = rng.choices(l.len(), l.len());
            means.push(py_mean(&idx.iter().map(|&i| l[i]).collect::<Vec<_>>())?);
            wall_means.push(py_mean(&idx.iter().map(|&i| w[i]).collect::<Vec<_>>())?);
        }
        boot.push(exp_f(py_mean(&means)?));
        native_boot.push(exp_f(py_mean(&wall_means)?));
    }
    let cmp = |a: &f64, b: &f64| a.partial_cmp(b).expect("no NaN");
    boot.sort_by(cmp);
    native_boot.sort_by(cmp);
    let lo = (0.025 * draws as f64) as usize;
    let hi = (draws - 1).min((0.975 * draws as f64) as usize);
    let native_cells: Vec<(String, f64)> = native_logs
        .iter()
        .map(|(k, xs)| py_mean(xs).map(|m| (k.clone(), m)))
        .collect::<R<_>>()?;
    let native_estimate = exp_f(py_mean(
        &native_cells.iter().map(|x| x.1).collect::<Vec<_>>(),
    )?);
    Ok(J::Obj(vec![
        ("candidate".into(), s(candidate)),
        ("eligible".into(), J::Bool(true)),
        ("candidate_over_baseline".into(), J::Float(estimate)),
        ("speedup".into(), J::Float(1.0 / estimate)),
        (
            "ci95".into(),
            J::Arr(vec![J::Float(boot[lo]), J::Float(boot[hi])]),
        ),
        (
            "per_cell".into(),
            per_cell(
                &cell_values
                    .iter()
                    .map(|(k, v)| (k.clone(), exp_f(*v)))
                    .collect::<Vec<_>>(),
            ),
        ),
        (
            "native_wall_candidate_over_baseline".into(),
            J::Float(native_estimate),
        ),
        (
            "native_wall_ci95".into(),
            J::Arr(vec![J::Float(native_boot[lo]), J::Float(native_boot[hi])]),
        ),
        (
            "native_wall_per_cell".into(),
            per_cell(
                &native_cells
                    .iter()
                    .map(|(k, v)| (k.clone(), exp_f(*v)))
                    .collect::<Vec<_>>(),
            ),
        ),
        (
            "native_wall_status".into(),
            s("paired cold-process timing; nested curve/target bootstrap"),
        ),
        ("paired_cases".into(), J::Int(cases.len() as i128)),
        (
            "independent_curve_blocks".into(),
            J::Int(cells.len() as i128),
        ),
    ]))
}

fn push_cell(logs: &mut Vec<(String, Vec<f64>)>, cell: &str, x: f64) {
    if let Some(slot) = logs.iter_mut().find(|(k, _)| k == cell) {
        slot.1.push(x);
    } else {
        logs.push((cell.to_string(), vec![x]));
    }
}

fn f(v: Option<&J>) -> f64 {
    v.and_then(J::as_f64).unwrap_or(f64::NAN)
}

fn max_values(v: Option<&J>) -> f64 {
    v.and_then(J::as_obj)
        .map(|kv| {
            kv.iter()
                .map(|(_, x)| f(Some(x)))
                .fold(f64::NEG_INFINITY, f64::max)
        })
        .unwrap_or(f64::NAN)
}

fn upper(v: Option<&J>) -> f64 {
    f(v.and_then(J::as_arr).and_then(|a| a.get(1)))
}

fn truthy(v: Option<&J>) -> bool {
    v.is_some_and(J::truthy)
}

/// `gate(result, c)`: promotion over the incumbent.  A wiring control
/// never promotes.
pub fn gate(result: &J, c: &J) -> bool {
    if c.get("purpose") == Some(&s("wiring-control")) {
        return false;
    }
    if !truthy(result.get("eligible")) {
        return false;
    }
    let max_cell = f(c.get("max_cell_ratio"));
    let cells_ok = max_values(result.get("per_cell")) <= max_cell;
    let objective = c
        .get("objective")
        .and_then(J::as_str)
        .unwrap_or("incumbent");
    if objective == "rho" {
        let no_regression = c
            .get("no_regression_ratio")
            .and_then(J::as_f64)
            .unwrap_or(0.98);
        return f(result.get("candidate_over_baseline")) <= no_regression
            && upper(result.get("ci95")) < 1.0
            && cells_ok
            && upper(result.get("native_wall_ci95")) < 1.0
            && max_values(result.get("native_wall_per_cell")) <= max_cell;
    }
    let ratio = f(c.get("confirmation_ratio"));
    f(result.get("candidate_over_baseline")) <= ratio
        && upper(result.get("ci95")) < 1.0
        && cells_ok
        && (!truthy(c.get("require_native_progress"))
            || (upper(result.get("native_wall_ci95")) < 1.0
                && f(result.get("native_wall_candidate_over_baseline")) <= ratio
                && max_values(result.get("native_wall_per_cell")) <= max_cell))
}

/// `rho_gate(paired)`: strictly below matched rho on both metrics, every
/// upper limit and every cell.
pub fn rho_gate(paired: &J) -> bool {
    truthy(paired.get("eligible"))
        && upper(paired.get("ci95")) < 1.0
        && upper(paired.get("native_wall_ci95")) < 1.0
        && max_values(paired.get("per_cell")) < 1.0
        && max_values(paired.get("native_wall_per_cell")) < 1.0
}

fn arm_by_id<'a>(arms: &'a [J], id: &str) -> R<&'a J> {
    arms.iter()
        .find(|a| a.get("id") == Some(&s(id)))
        .ok_or_else(|| "StopIteration".to_string())
}

/// `stage_arms(root, stage, arms)`: which arms a stage schedules.
pub fn stage_arms(st: &Store, stage: &str, arms: &[J]) -> R<Vec<J>> {
    let base = &arms[0];
    let rho = synthetic_arm("rho", base);
    match stage {
        "aa" => Ok(vec![base.clone(), synthetic_arm("aa_control", base)]),
        "smoke" => Ok(arms.iter().cloned().chain([rho]).collect()),
        "development" => {
            let smoke = read(st, "summaries/smoke.json")?;
            let failed: BTreeSet<String> = at(&smoke, "failures")?
                .as_arr()
                .ok_or("TypeError: failures")?
                .iter()
                .filter_map(|r| r.get("arm").and_then(J::as_str).map(str::to_string))
                .collect();
            require(!failed.contains("incumbent"), "incumbent failed smoke")?;
            Ok(arms
                .iter()
                .filter(|a| {
                    !a.get("id")
                        .and_then(J::as_str)
                        .is_some_and(|id| failed.contains(id))
                })
                .cloned()
                .chain([rho])
                .collect())
        }
        "selection" => {
            let d = read(st, "summaries/development.json")?;
            let mut eligible: Vec<&J> = at(&d, "comparisons")?
                .as_arr()
                .ok_or("TypeError: comparisons")?
                .iter()
                .filter(|r| truthy(r.get("eligible")))
                .collect();
            eligible.sort_by(|a, b| {
                f(a.get("candidate_over_baseline"))
                    .partial_cmp(&f(b.get("candidate_over_baseline")))
                    .expect("no NaN")
            });
            let mut out = vec![base.clone()];
            for r in eligible.into_iter().take(2) {
                out.push(arm_by_id(arms, str_at(r, "candidate")?)?.clone());
            }
            out.push(rho);
            Ok(out)
        }
        _ => {
            let d = read(st, "summaries/selection.json")?;
            let mut out = vec![base.clone()];
            if let Some(J::Str(p)) = d.get("provisional_challenger") {
                out.push(arm_by_id(arms, p)?.clone());
            }
            out.push(rho);
            Ok(out)
        }
    }
}

fn draws(c: &J) -> R<usize> {
    Ok(at(c, "bootstrap_draws")?
        .as_i128()
        .ok_or("TypeError: bootstrap_draws")? as usize)
}

fn repetitions(c: &J) -> R<usize> {
    Ok(at(c, "repetitions")?
        .as_i128()
        .ok_or("TypeError: repetitions")? as usize)
}

/// `summarize(root, c, stage, fixtures, arms, save=False)`.
pub fn summarize(st: &Store, c: &J, stage: &str, fixtures: &[J], arms: &[J]) -> R<J> {
    let rows = load_stage(st, stage, fixtures, arms, repetitions(c)?)?;
    let match_support = c
        .get("comparison_kind")
        .and_then(J::as_str)
        .unwrap_or("fixed-support")
        != "factor-base-policy";
    let n = draws(c)?;
    let mut comps = Vec::new();
    for a in arms {
        let id = str_at(a, "id")?;
        if id != "incumbent" && id != "rho" {
            comps.push(comparison(&rows, id, "incumbent", n, match_support)?);
        }
    }
    let rho = if arms.iter().any(|a| a.get("id") == Some(&s("rho"))) {
        comparison(&rows, "rho", "incumbent", n, true)?
    } else {
        J::Null
    };
    let walls: Vec<f64> = rows
        .iter()
        .map(|r| {
            let profile = num(at(at(r, "profile_process")?, "process_wall_seconds")?)?;
            let native = match r.get("native_process") {
                Some(p) => num(at(p, "process_wall_seconds")?)?,
                None => 0.0,
            };
            Ok(profile + native)
        })
        .collect::<R<_>>()?;
    let failures: Vec<J> = rows
        .iter()
        .filter(|r| r.get("status") != Some(&s("VERIFIED")))
        .map(|r| {
            J::Obj(
                ["case", "arm", "repetition", "status", "reason"]
                    .iter()
                    .map(|k| (k.to_string(), r.get(k).cloned().unwrap_or(J::Null)))
                    .collect(),
            )
        })
        .collect();
    let mut result = J::Obj(vec![
        ("stage".into(), s(stage)),
        ("runs".into(), J::Int(rows.len() as i128)),
        (
            "verified_runs".into(),
            J::Int(
                rows.iter()
                    .filter(|r| r.get("status") == Some(&s("VERIFIED")))
                    .count() as i128,
            ),
        ),
        ("comparisons".into(), J::Arr(comps.clone())),
        ("rho_over_incumbent".into(), rho),
        (
            "process_wall_seconds_including_profiling".into(),
            J::Float(py_sum(&walls)),
        ),
        ("failures".into(), J::Arr(failures)),
    ]);
    if stage == "aa" {
        let aa = comps.first().ok_or("IndexError: no A/A comparison")?;
        let within = aa
            .get("per_cell")
            .and_then(J::as_obj)
            .is_some_and(|kv| kv.iter().all(|(_, r)| (0.95..=1.05).contains(&f(Some(r)))));
        set(
            &mut result,
            "passed",
            J::Bool(truthy(aa.get("eligible")) && within && !gate(aa, c)),
        );
    }
    if stage == "selection" {
        let mut eligible: Vec<&J> = comps.iter().filter(|r| truthy(r.get("eligible"))).collect();
        eligible.sort_by(|a, b| {
            f(a.get("candidate_over_baseline"))
                .partial_cmp(&f(b.get("candidate_over_baseline")))
                .expect("no NaN")
        });
        set(
            &mut result,
            "provisional_challenger",
            eligible
                .first()
                .and_then(|r| r.get("candidate"))
                .cloned()
                .unwrap_or(J::Null),
        );
    }
    Ok(result)
}

fn find_comparison(summary: &J, challenger: &J) -> J {
    summary
        .get("comparisons")
        .and_then(J::as_arr)
        .and_then(|cs| cs.iter().find(|r| r.get("candidate") == Some(challenger)))
        .cloned()
        .unwrap_or(J::Obj(vec![]))
}

fn stage_fixtures<'a>(fixtures: &'a J, stage: &str) -> R<&'a [J]> {
    at(fixtures, stage)?
        .as_arr()
        .ok_or_else(|| format!("TypeError: fixtures[{stage}]"))
}

/// `decision(root, c, fixtures, all_arms, save=False)`.
pub fn decision(st: &Store, c: &J, fixtures: &J, all_arms: &[J]) -> R<J> {
    let select = read(st, "summaries/selection.json")?;
    let confirm = read(st, "summaries/confirmation.json")?;
    let replay = read(st, "summaries/replay.json")?;
    let challenger = at(&select, "provisional_challenger")?.clone();
    let conf = find_comparison(&confirm, &challenger);
    let rep = find_comparison(&replay, &challenger);
    let mut passed = gate(&conf, c) && gate(&rep, c);
    let reps = repetitions(c)?;
    let n = draws(c)?;
    let objective = c
        .get("objective")
        .and_then(J::as_str)
        .unwrap_or("incumbent");
    let paired_against = |id: &str, baseline: &str| -> R<Vec<J>> {
        ["confirmation", "replay"]
            .iter()
            .map(|stage| {
                let active = stage_arms(st, stage, all_arms)?;
                let rows = load_stage(st, stage, stage_fixtures(fixtures, stage)?, &active, reps)?;
                comparison(&rows, id, baseline, n, true)
            })
            .collect()
    };
    let mut challenger_over_rho = Vec::new();
    if challenger.truthy() && objective == "rho" {
        challenger_over_rho = paired_against(challenger.as_str().ok_or("TypeError")?, "rho")?;
        passed = passed && challenger_over_rho.iter().all(rho_gate);
    }
    let qualified = truthy(conf.get("eligible")) && truthy(rep.get("eligible"));
    let mut base_complete = true;
    for stage in ["confirmation", "replay"] {
        for case in stage_fixtures(fixtures, stage)? {
            for repetition in 0..reps {
                let receipt = read(
                    st,
                    &format!(
                        "{}/receipt.json",
                        trial_rel(stage, case, "incumbent", repetition)?
                    ),
                )?;
                base_complete &= receipt.get("status") == Some(&s("VERIFIED"))
                    && !matches!(receipt.get("total_operations"), None | Some(J::Null));
            }
        }
    }
    let choice = if passed {
        challenger.clone()
    } else if base_complete && (qualified || !challenger.truthy()) {
        s("incumbent")
    } else {
        J::Null
    };
    let mut reasons = Vec::new();
    if !passed {
        reasons.push(s(
            "No challenger passed every confirmation and replay threshold.",
        ));
        if !challenger_over_rho.is_empty() && !challenger_over_rho.iter().all(rho_gate) {
            reasons.push(s("The provisional challenger did not beat matched rho on both metrics with every upper 95% limit and every cell below one."));
        }
    }
    let status = if passed {
        "promoted"
    } else if choice.truthy() {
        "retained"
    } else {
        "inconclusive"
    };
    let rc = at(&confirm, "rho_over_incumbent")?.clone();
    let mut result = J::Obj(vec![
        ("status".into(), s(status)),
        ("winner".into(), choice.clone()),
        ("provisional_challenger".into(), challenger.clone()),
        ("confirmation".into(), conf.clone()),
        ("replay".into(), rep.clone()),
        ("unit".into(), at(c, "unit")?.clone()),
        (
            "classification".into(),
            s(if passed { "engineering" } else { "accounting" }),
        ),
        ("scope".into(), at(c, "evidence_scope")?.clone()),
        ("family_wide_or_complexity_claim".into(), J::Bool(false)),
        ("arithmetic_operation_speedup".into(), J::Null),
        (
            "instruction_speedup".into(),
            if passed {
                conf.get("speedup").cloned().unwrap_or(J::Null)
            } else {
                J::Null
            },
        ),
        ("reasons".into(), J::Arr(reasons)),
        (
            "independent_replay".into(),
            s(if c.get("native_evaluator").is_some() {
                "fresh processes, frozen executable, same final cases; independent native group and rank checker"
            } else {
                "fresh processes, frozen executable, same final cases; independent Python group and rank checker"
            }),
        ),
        ("rho_comparison".into(), rc.clone()),
        ("beats_rho".into(), J::Bool(false)),
    ]);
    if choice.truthy() && rc.truthy() && truthy(rc.get("eligible")) {
        let ratio = if passed {
            f(conf.get("candidate_over_baseline"))
        } else {
            1.0
        };
        let rho_over_winner = f(rc.get("candidate_over_baseline")) / ratio;
        set(&mut result, "rho_over_winner", J::Float(rho_over_winner));
        set(&mut result, "beats_rho", J::Bool(rho_over_winner > 1.0));
        set(
            &mut result,
            "rho_verdict_scope",
            s("observed complete instruction cost; no asymptotic crossover claim"),
        );
    }
    if choice.truthy() {
        let parity = paired_against(choice.as_str().ok_or("TypeError")?, "rho")?;
        let margin = f(c.get("parity_margin"));
        set(
            &mut result,
            "winner_over_rho",
            J::Obj(vec![
                ("confirmation".into(), parity[0].clone()),
                ("replay".into(), parity[1].clone()),
            ]),
        );
        set(
            &mut result,
            "rho_parity",
            J::Bool(parity.iter().all(|p| {
                truthy(p.get("eligible"))
                    && upper(p.get("ci95")) <= margin
                    && upper(p.get("native_wall_ci95")) <= margin
                    && max_values(p.get("per_cell")) <= margin
                    && max_values(p.get("native_wall_per_cell")) <= margin
            })),
        );
        set(&mut result, "parity_definition", s("Both candidate/rho upper paired 95% limits and every cell ratio <= 1.10, instructions and native process wall, confirmation and replay."));
        set(
            &mut result,
            "beats_rho_strict",
            J::Bool(parity.iter().all(rho_gate)),
        );
        set(&mut result, "beats_rho_strict_definition", s("Winner/rho upper paired 95% limits and every cell ratio < 1 in both instructions and native process wall, on confirmation and replay. beats_rho alone is the confirmation point estimate of rho/winner exceeding one."));
    }
    if !challenger_over_rho.is_empty() {
        set(
            &mut result,
            "challenger_over_rho",
            J::Obj(vec![
                ("confirmation".into(), challenger_over_rho[0].clone()),
                ("replay".into(), challenger_over_rho[1].clone()),
            ]),
        );
    }
    set(&mut result, "objective", s(objective));
    Ok(result)
}

// ── verify ──────────────────────────────────────────────────────────

/// `audit(args)`: every receipt, every source snapshot, every stage summary
/// and the decision re-derived.  The result is `tournament.py verify`'s
/// output.
pub fn audit(st: &Store, evaluator: Option<&Path>) -> R<J> {
    let (c, fixtures, arms, p) = frozen_inputs(st, evaluator)?;
    let manifest = read(st, "source-manifest.json")?;
    let mut pairs = BTreeSet::new();
    for a in &arms {
        pairs.insert((
            a.get("source_manifest_relative")
                .and_then(J::as_str)
                .unwrap_or("source-manifest.json")
                .to_string(),
            a.get("source_directory")
                .and_then(J::as_str)
                .unwrap_or("source")
                .to_string(),
        ));
    }
    for (manifest_path, source) in &pairs {
        for (name, h) in read(st, manifest_path)?
            .as_obj()
            .ok_or("TypeError: manifest")?
        {
            let rel = format!("{source}/{name}");
            require(
                st.exists(&rel) && Some(st.digest(&rel)?.as_str()) == h.as_str(),
                &format!("changed source snapshot {name}"),
            )?;
        }
    }
    let reps = repetitions(&c)?;
    let mut count = 0;
    for stage in STAGES {
        let summary = format!("summaries/{stage}.json");
        if !st.exists(&summary) {
            continue;
        }
        let active = stage_arms(st, stage, &arms)?;
        let cases = stage_fixtures(&fixtures, stage)?;
        for case in cases {
            for arm in &active {
                for rep in 0..reps {
                    let rel = trial_rel(stage, case, str_at(arm, "id")?, rep)?;
                    verify_receipt(st, &rel, &c, case, arm, &p)
                        .map_err(|e| format!("{rel}: {e}"))?;
                    count += 1;
                }
            }
        }
        let saved = read(st, &summary)?;
        let derived = summarize(st, &c, stage, cases, &active)?;
        if !py_eq(&saved, &derived) {
            return Err(format!(
                "changed stage summary or provisional selection ({stage}): {}",
                first_difference(&saved, &derived, "")
            ));
        }
    }
    if st.exists("decision.json") {
        let saved = read(st, "decision.json")?;
        let derived = decision(st, &c, &fixtures, &arms)?;
        if !py_eq(&saved, &derived) {
            return Err(format!(
                "changed decision: {}",
                first_difference(&saved, &derived, "")
            ));
        }
    }
    let files = manifest.as_obj().map_or(0, |kv| kv.len());
    Ok(J::Obj(vec![
        ("status".into(), s("VERIFIED")),
        ("trial_receipts".into(), J::Int(count)),
        ("source_files".into(), J::Int(files as i128)),
    ]))
}

/// Where two records first differ, for a refusal that says what changed.
fn first_difference(a: &J, b: &J, path: &str) -> String {
    match (a, b) {
        (J::Obj(x), J::Obj(y)) => {
            for (k, v) in x {
                match y.iter().find(|(k2, _)| k2 == k) {
                    None => return format!("{path}/{k} missing from the re-derivation"),
                    Some((_, w)) if !py_eq(v, w) => {
                        return first_difference(v, w, &format!("{path}/{k}"))
                    }
                    _ => {}
                }
            }
            for (k, _) in y {
                if !x.iter().any(|(k2, _)| k2 == k) {
                    return format!("{path}/{k} only in the re-derivation");
                }
            }
            format!("{path}: no difference")
        }
        (J::Arr(x), J::Arr(y)) if x.len() == y.len() => x
            .iter()
            .zip(y)
            .enumerate()
            .find(|(_, (p, q))| !py_eq(p, q))
            .map(|(i, (p, q))| first_difference(p, q, &format!("{path}/{i}")))
            .unwrap_or_else(|| format!("{path}: no difference")),
        _ => format!(
            "{path}: saved {} re-derived {}",
            json::dumps_compact(a, true, true),
            json::dumps_compact(b, true, true)
        ),
    }
}

// ── run ─────────────────────────────────────────────────────────────

/// Whether `run` may execute this round at all: only rounds this native
/// evaluator froze, under a registered protocol or as a wiring control,
/// never a consumed seed and never a bounded or qualification campaign.
pub fn runnable(c: &J) -> R<()> {
    require(
        c.get("native_evaluator").is_some(),
        "run refuses rounds frozen by a Python evaluator: historical rounds, the bounded \
         attempts 1-3 and every closed registration replay with `verify` only",
    )?;
    let seed = at(c, "seed")?.as_i128().ok_or("TypeError: seed")?;
    require(
        !CLOSED_SEEDS.contains(&seed),
        &format!("seed {seed} belongs to a consumed or closed registration"),
    )?;
    require(
        c.get("attempt_number").is_none() && c.get("familywise_rule").is_none(),
        "bounded campaign attempts are closed and not ported",
    )?;
    let purpose = c.get("purpose").and_then(J::as_str).unwrap_or("");
    require(
        purpose == "improvement" || purpose == "wiring-control",
        &format!("purpose {purpose:?} is not runnable by this port"),
    )?;
    let kind = c
        .get("registration")
        .and_then(|r| r.get("kind"))
        .and_then(J::as_str)
        .unwrap_or("");
    require(
        (purpose == "improvement" && kind == "registered")
            || (purpose == "wiring-control" && kind == "wiring-control"),
        "registration kind does not match the round's purpose",
    )
}

/// Held for its `flock`, released on drop.
struct Lock(#[allow(dead_code)] std::fs::File);

fn lock(root: &Path) -> R<Lock> {
    use std::os::fd::AsRawFd;
    let f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(root.join("operation.lock"))
        .map_err(|e| format!("operation.lock: {e}"))?;
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(format!(
            "BlockingIOError: operation.lock: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(Lock(f))
}

fn now_unix() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

/// `run_campaign(args)`: the declared stages in order, each trial once,
/// resuming where a previous invocation stopped.
pub fn run_campaign(
    root: &Path,
    stage_arg: &str,
    evaluator: &Path,
    log: &mut dyn FnMut(&J),
) -> R<()> {
    let _lock = lock(root)?;
    let st = Store::Dir(root.to_path_buf());
    let (c, fixtures, arms, p) = frozen_inputs(&st, Some(evaluator))?;
    runnable(&c)?;
    require(
        stage_arg == "all" || STAGES.contains(&stage_arg),
        "stage is not in this frozen campaign",
    )?;
    let mut completed = st
        .files_under("runs")
        .iter()
        .filter(|f| f.ends_with("/receipt.json"))
        .count() as i128;
    let reps = repetitions(&c)?;
    let budget = at(at(&c, "limits")?, "max_profiled_jobs")?
        .as_i128()
        .ok_or("TypeError: max_profiled_jobs")?;
    let seed = at(&c, "seed")?.as_i128().ok_or("TypeError: seed")?;
    for (index, stage) in STAGES.iter().enumerate() {
        if stage_arg != "all" && stage_arg != *stage {
            continue;
        }
        for prior in &STAGES[..index] {
            require(
                st.exists(&format!("summaries/{prior}.json")),
                &format!("complete earlier stage {prior}"),
            )?;
        }
        if st.exists(&format!("summaries/{stage}.json")) {
            if *stage == "aa" {
                require(
                    truthy(read(&st, "summaries/aa.json")?.get("passed")),
                    "A/A gate failed",
                )?;
            }
            continue;
        }
        let active = stage_arms(&st, stage, &arms)?;
        let cases = stage_fixtures(&fixtures, stage)?;
        let mut schedule: Vec<(usize, usize)> = (0..cases.len())
            .flat_map(|c| (0..reps).map(move |r| (c, r)))
            .collect();
        let mut rng = Random::new((seed + index as i128) as u128);
        rng.shuffle(&mut schedule);
        for (ci, rep) in schedule {
            let case = &cases[ci];
            let mut order: Vec<&J> = active.iter().collect();
            rng.shuffle(&mut order);
            for arm in order {
                let id = str_at(arm, "id")?;
                let present = root
                    .join(trial_rel(stage, case, id, rep)?)
                    .join("receipt.json")
                    .exists();
                require(
                    present || completed < budget,
                    "campaign process budget exhausted",
                )?;
                let r = run_trial(root, &c, stage, case, arm, rep, &p)?;
                completed += i128::from(!present);
                write(
                    &root.join("state.json"),
                    &J::Obj(vec![
                        ("stage".into(), s(stage)),
                        ("completed_jobs".into(), J::Int(completed)),
                        ("last_case".into(), at(case, "id")?.clone()),
                        ("last_arm".into(), s(id)),
                        ("last_status".into(), at(&r, "status")?.clone()),
                        ("updated_unix".into(), J::Float(now_unix())),
                    ]),
                    false,
                )?;
                log(&J::Obj(vec![
                    ("stage".into(), s(stage)),
                    ("arm".into(), s(id)),
                    ("case".into(), at(case, "id")?.clone()),
                    ("rep".into(), J::Int(rep as i128)),
                    ("status".into(), at(&r, "status")?.clone()),
                    ("operations".into(), at(&r, "total_operations")?.clone()),
                ]));
            }
        }
        let summary = summarize(&st, &c, stage, cases, &active)?;
        write(
            &root.join(format!("summaries/{stage}.json")),
            &summary,
            true,
        )?;
        if *stage == "aa" {
            require(
                truthy(summary.get("passed")),
                "A/A gate failed; preserve evidence and investigate",
            )?;
        }
    }
    if st.exists("summaries/replay.json") && !st.exists("decision.json") {
        let d = decision(&st, &c, &fixtures, &arms)?;
        write(&root.join("decision.json"), &d, true)?;
        log(&d);
    }
    Ok(())
}

// ── prepare ─────────────────────────────────────────────────────────

pub fn base_config() -> J {
    J::Obj(vec![
        ("solver".into(), s("pair_table")),
        ("linear_algebra".into(), s("sparse")),
        ("batch_trials".into(), J::Int(64)),
        ("max_trials".into(), J::Int(4096)),
        ("summands".into(), J::Int(3)),
    ])
}

fn merged(base: &J, delta: &J) -> J {
    let mut out = base.clone();
    if let J::Obj(kv) = delta {
        for (k, v) in kv {
            set(&mut out, k, v.clone());
        }
    }
    out
}

/// `candidates()`: the default registry.
pub fn default_candidates() -> J {
    let changes: [(&str, J, &str); 6] = [
        (
            "batch16",
            json::obj([("batch_trials", J::Int(16))]),
            "Reduce surplus collection and verification before a rank check.",
        ),
        (
            "batch32",
            json::obj([("batch_trials", J::Int(32))]),
            "Balance surplus relations against repeated rank checks.",
        ),
        (
            "batch128",
            json::obj([("batch_trials", J::Int(128))]),
            "Amortize rank checks when relation yield is low.",
        ),
        (
            "dense",
            json::obj([("linear_algebra", s("dense"))]),
            "Avoid sparse-solver setup on small relation matrices.",
        ),
        (
            "excess8",
            json::obj([(
                "sparse",
                json::obj([("filter", json::obj([("target_excess", J::Int(8))]))]),
            )]),
            "Reduce the filtered core by retaining fewer surplus rows.",
        ),
        (
            "window8",
            json::obj([("collection_window", J::Int(8))]),
            "Trade extra walked probes for fewer pair-table scans.",
        ),
    ];
    let mut out = vec![json::obj([
        ("id", s("incumbent")),
        ("parent", J::Null),
        ("hypothesis", s("Frozen complete library pipeline.")),
        ("config", base_config()),
    ])];
    for (name, delta, hypothesis) in changes {
        out.push(json::obj([
            ("id", s(name)),
            ("parent", s("incumbent")),
            ("hypothesis", s(hypothesis)),
            (
                "falsification",
                s("Fails correctness, total-cost or per-cell confirmation gate."),
            ),
            ("changed_parameters", delta.clone()),
            ("config", merged(&base_config(), &delta)),
        ]));
    }
    J::Arr(out)
}

/// `proposals_from_previous(root)`: a successor registry around a round's
/// retained or promoted winner.
pub fn proposals_from_previous(st: &Store, root_label: &str) -> R<(J, String)> {
    let result = read(st, "decision.json")?;
    let status = result.get("status").and_then(J::as_str).unwrap_or("");
    require(
        (status == "promoted" || status == "retained") && truthy(result.get("winner")),
        "finish and audit the previous decision before generating a successor",
    )?;
    let previous = read(st, "candidates.json")?;
    let previous = previous.as_arr().ok_or("TypeError: candidates")?;
    let parent = arm_by_id(previous, str_at(&result, "winner")?)?;
    let config = at(parent, "config")?.clone();
    let mut out = vec![json::obj([
        ("id", s("incumbent")),
        ("parent", at(parent, "id")?.clone()),
        ("parent_round", s(root_label)),
        (
            "hypothesis",
            s("Retain the previous selected complete algorithm."),
        ),
        ("config", config.clone()),
    ])];
    let mut seen: BTreeSet<String> = previous
        .iter()
        .map(|a| at(a, "config").map(objhash))
        .collect::<R<_>>()?;
    let batch = config
        .get("batch_trials")
        .and_then(J::as_i128)
        .unwrap_or(64);
    let max_trials = config
        .get("max_trials")
        .and_then(J::as_i128)
        .unwrap_or(4096);
    let mut changes: Vec<(String, J, &str)> = Vec::new();
    for value in [
        batch.div_euclid(2),
        batch.div_euclid(4),
        batch * 2,
        batch * 4,
        1,
        2,
        8,
        256,
    ] {
        if 1 <= value && value <= max_trials.min(4096) {
            changes.push((
                format!("batch{value}"),
                json::obj([("batch_trials", J::Int(value))]),
                "Test the next collection/rank-check tradeoff around the selected batch.",
            ));
        }
    }
    let mode = if config
        .get("linear_algebra")
        .and_then(J::as_str)
        .unwrap_or("sparse")
        == "sparse"
    {
        "dense"
    } else {
        "sparse"
    };
    changes.push((
        mode.into(),
        json::obj([("linear_algebra", s(mode))]),
        "Test linear-algebra setup against the selected collection policy.",
    ));
    for value in [4, 16, 32] {
        changes.push((
            format!("window{value}"),
            json::obj([("collection_window", J::Int(value))]),
            "Test a new walked-probe scan window with all failed attempts charged.",
        ));
    }
    for (name, delta, hypothesis) in changes {
        let cfg = merged(&config, &delta);
        if !seen.insert(objhash(&cfg)) {
            continue;
        }
        out.push(json::obj([
            ("id", s(&name)),
            ("parent", s("incumbent")),
            ("parent_round", s(root_label)),
            ("hypothesis", s(hypothesis)),
            ("changed_parameters", delta),
            ("config", cfg),
            (
                "falsification",
                s("Fails the unchanged complete-workload confirmation gate."),
            ),
        ]));
        if out.len() == 7 {
            break;
        }
    }
    require(out.len() > 1, "no distinct configured candidates remain")?;
    let source = parent
        .get("source_directory")
        .and_then(J::as_str)
        .unwrap_or("source");
    Ok((J::Arr(out), format!("{root_label}/{source}")))
}

/// `parse_cells('13a0,17a1')`.
pub fn parse_cells(text: &str) -> R<Vec<(i128, i128)>> {
    let mut out = Vec::new();
    for token in text.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        let body = token.strip_prefix('n').unwrap_or(token);
        let parsed = body.split_once('a').and_then(|(d, a)| {
            let ok = (1..=2).contains(&d.len())
                && d.chars().all(|c| c.is_ascii_digit())
                && (a == "0" || a == "1");
            ok.then(|| (d.parse::<i128>().ok(), a.parse::<i128>().ok()))
        });
        let Some((Some(d), Some(a))) = parsed else {
            return Err(format!("cell '{token}' is not <degree>a<curve_a>"));
        };
        require(
            !out.contains(&(d, a)),
            &format!("cell '{token}' named twice"),
        )?;
        out.push((d, a));
    }
    require(!out.is_empty(), "empty cell list")?;
    Ok(out)
}

/// `parse_confirmation_allocation('n23a1=48', floor, declared)`: raises
/// only.
pub fn parse_confirmation_allocation(
    text: &str,
    floor: i128,
    declared: &[String],
) -> R<Vec<(String, i128)>> {
    let mut out: Vec<(String, i128)> = Vec::new();
    for item in text.split(',') {
        if item.trim().is_empty() {
            continue;
        }
        let Some((name, value)) = item.split_once('=') else {
            return Err(format!(
                "--confirmation-cases wants cell=count, got '{item}'"
            ));
        };
        let name = name.trim().to_string();
        require(
            !out.iter().any(|(k, _)| *k == name),
            &format!("{name}: named twice in --confirmation-cases"),
        )?;
        let n: i128 = value
            .trim()
            .parse()
            .map_err(|_| format!("invalid literal for int() with base 10: '{value}'"))?;
        require(
            n >= floor,
            &format!("{name}: {n} confirmation cases is below the profile floor {floor}"),
        )?;
        out.push((name, n));
    }
    let mut unknown: Vec<&String> = out
        .iter()
        .map(|(k, _)| k)
        .filter(|k| !declared.contains(k))
        .collect();
    unknown.sort();
    require(
        unknown.is_empty(),
        &format!("--confirmation-cases names cells outside the panel: {unknown:?}"),
    )?;
    Ok(out)
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
pub enum Profile {
    Pilot,
    Standard,
    /// One case per stage and cell: a wiring control only.
    Wiring,
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
pub enum ComparisonKind {
    FixedSupport,
    FactorBasePolicy,
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
pub enum Objective {
    Incumbent,
    Rho,
}

pub struct PrepareArgs {
    pub out: PathBuf,
    pub source_root: PathBuf,
    pub comparison_kind: ComparisonKind,
    pub candidates: Option<PathBuf>,
    pub profile: Profile,
    pub confirmation_cases: String,
    pub cells: String,
    pub holdout_cells: String,
    pub seed: u64,
    pub cpu: Option<usize>,
    pub timeout: f64,
    pub max_processes: i128,
    pub require_native_progress: bool,
    pub targets: i128,
    pub objective: Objective,
    pub protocol: Option<PathBuf>,
    pub wiring_control: bool,
    pub worker_binary: Option<PathBuf>,
    pub evaluator: PathBuf,
}

fn copy(from: &Path, to: &Path) -> R<()> {
    if let Some(p) = to.parent() {
        std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
    }
    std::fs::copy(from, to)
        .map(|_| ())
        .map_err(|e| format!("{} -> {}: {e}", from.display(), to.display()))
}

fn copytree(from: &Path, to: &Path) -> R<()> {
    std::fs::create_dir_all(to).map_err(|e| e.to_string())?;
    for e in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))? {
        // `shutil.copytree` follows symbolic links by default.
        let e = e.map_err(|e| e.to_string())?;
        let path = e.path();
        if path.is_dir() {
            copytree(&path, &to.join(e.file_name()))?;
        } else if path.is_file() {
            copy(&path, &to.join(e.file_name()))?;
        }
    }
    Ok(())
}

/// The literal `include!`, `include_str!` and `include_bytes!` paths of a
/// Rust source.
fn includes(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for macro_name in ["include!", "include_str!", "include_bytes!"] {
        let mut rest = text;
        while let Some(i) = rest.find(macro_name) {
            let after = rest[i + macro_name.len()..].trim_start();
            if let Some(after) = after.strip_prefix('(') {
                let after = after.trim_start();
                if let Some(after) = after.strip_prefix('"') {
                    if let Some(end) = after.find('"') {
                        if end > 0 {
                            out.push(after[..end].to_string());
                        }
                    }
                }
            }
            rest = &rest[i + macro_name.len()..];
        }
    }
    out
}

fn manifest_of(dir: &Path) -> R<J> {
    let st = Store::Dir(dir.to_path_buf());
    let files = st.files_under("");
    Ok(J::Obj(
        files
            .into_iter()
            .map(|f| st.digest(&f).map(|d| (f, J::Str(d))))
            .collect::<R<_>>()?,
    ))
}

/// `snapshot_build(source, destination)`: freeze the worker's sources and
/// build it from the snapshot alone.  With `prebuilt` (a wiring control
/// only) the snapshot holds the manifests and worker source but the worker
/// is the supplied binary, and the build log says so.
fn snapshot_build(source: &Path, destination: &Path, prebuilt: Option<&Path>) -> R<(PathBuf, J)> {
    let snap = destination.join("source");
    std::fs::create_dir(&snap).map_err(|e| format!("{}: {e}", snap.display()))?;
    for name in ["Cargo.toml", "Cargo.lock", "build.rs", ".cargo/config.toml"] {
        if source.join(name).exists() {
            copy(&source.join(name), &snap.join(name))?;
        }
    }
    if prebuilt.is_none() {
        copytree(&source.join("src"), &snap.join("src"))?;
        // The root Cargo manifest names local crates. A registered worker
        // must build from their sealed source too, with no checkout fallback.
        for rel in ["tools/factor-base", "tools/volcano-cli", "isogeny_algos"] {
            copytree(&source.join(rel), &snap.join(rel))?;
        }
        let st = Store::Dir(source.join("src"));
        for rel in st.files_under("") {
            if !rel.ends_with(".rs") {
                continue;
            }
            let rust = source.join("src").join(&rel);
            let text = std::fs::read_to_string(&rust).map_err(|e| e.to_string())?;
            for inc in includes(&text) {
                // This lexical scan also sees include-like text in tests and
                // strings. Missing incidental paths are harmless; the offline
                // rebuild below rejects any real compile-time input omitted
                // from the frozen tree.
                let Ok(dep) = rust.parent().expect("parent").join(&inc).canonicalize() else {
                    continue;
                };
                let root = source.canonicalize().map_err(|e| e.to_string())?;
                let rel_dep = dep
                    .strip_prefix(&root)
                    .map_err(|_| "include escapes source root".to_string())?;
                copy(&dep, &snap.join(rel_dep))?;
            }
        }
    }
    copy(
        &source.join("examples/ic_tournament_worker.rs"),
        &snap.join("examples/ic_tournament_worker.rs"),
    )?;
    let manifest = manifest_of(&snap)?;
    write(&destination.join("source-manifest.json"), &manifest, true)?;
    let binary = destination.join("worker");
    let log_path = destination.join("build.log");
    if let Some(prebuilt) = prebuilt {
        std::fs::write(
            &log_path,
            format!(
                "WIRING CONTROL: worker supplied prebuilt from {} (sha256 {}); it was not built \
                 from this snapshot, which holds only the manifests and the worker source. Not \
                 admissible for a registered round.\n",
                prebuilt.display(),
                store::digest_file(prebuilt)?
            ),
        )
        .map_err(|e| e.to_string())?;
        copy(prebuilt, &binary)?;
        return Ok((binary, manifest));
    }
    let log = std::fs::File::create(&log_path).map_err(|e| e.to_string())?;
    let status = std::process::Command::new("cargo")
        .args([
            "build",
            "--release",
            "--offline",
            "--locked",
            "--jobs",
            "2",
            "--example",
            "ic_tournament_worker",
            "--target-dir",
        ])
        .arg(destination.join("build"))
        .current_dir(&snap)
        .env_clear()
        .envs(exec::child_env())
        .stdout(log.try_clone().map_err(|e| e.to_string())?)
        .stderr(log)
        .status()
        .map_err(|e| format!("cargo: {e}"))?;
    require(
        status.success(),
        "CalledProcessError: snapshot build failed; see build.log",
    )?;
    let build = destination.join("build");
    let mut found = vec![build.join("release/examples/ic_tournament_worker")];
    if let Ok(rd) = std::fs::read_dir(&build) {
        let mut extra: Vec<PathBuf> = rd
            .flatten()
            .map(|e| e.path().join("release/examples/ic_tournament_worker"))
            .collect();
        extra.sort();
        found.extend(extra);
    }
    found.retain(|p| p.is_file());
    require(
        found.len() == 1,
        &format!("expected exactly one built worker, found {}", found.len()),
    )?;
    copy(&found[0], &binary)?;
    Ok((binary, manifest))
}

fn command_output(program: &str, args: &[&str]) -> R<String> {
    let out = std::process::Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("{program}: {e}"))?;
    require(out.status.success(), &format!("{program} failed"))?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn uname() -> J {
    unsafe {
        let mut u: libc::utsname = std::mem::zeroed();
        libc::uname(&mut u);
        let field = |f: &[libc::c_char]| {
            let bytes: Vec<u8> = f
                .iter()
                .take_while(|&&c| c != 0)
                .map(|&c| c as u8)
                .collect();
            J::Str(String::from_utf8_lossy(&bytes).into_owned())
        };
        let machine = field(&u.machine);
        J::Obj(vec![
            ("system".into(), field(&u.sysname)),
            ("node".into(), field(&u.nodename)),
            ("release".into(), field(&u.release)),
            ("version".into(), field(&u.version)),
            ("machine".into(), machine.clone()),
            ("processor".into(), machine),
        ])
    }
}

/// `prepare(args)`: freeze a round.  Nothing is executed beyond the
/// snapshot build and fixture generation.
pub fn prepare(args: &PrepareArgs) -> R<J> {
    require(
        args.timeout > 0.0 && args.max_processes > 0,
        "positive limits required",
    )?;
    require(
        args.targets == 1,
        "the current worker measures exactly one supplied public point; --targets must be 1",
    )?;
    require(
        args.wiring_control != args.protocol.is_some(),
        "freeze either a registered round (--protocol FILE) or a --wiring-control",
    )?;
    require(
        args.worker_binary.is_none() || args.wiring_control,
        "a prebuilt worker is admissible only in a wiring control",
    )?;
    require(
        (args.profile == Profile::Wiring) == args.wiring_control,
        "the wiring profile is reserved for wiring controls, and a wiring control uses it",
    )?;
    require(
        !CLOSED_SEEDS.contains(&(args.seed as i128)),
        &format!(
            "seed {} belongs to a consumed or closed registration",
            args.seed
        ),
    )?;
    let out = std::path::absolute(&args.out).map_err(|e| e.to_string())?;
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::create_dir(&out).map_err(|e| format!("{}: {e}", out.display()))?;
    require(
        std::env::consts::ARCH == "x86_64",
        "instruction protocol currently supports amd64 only",
    )?;
    let version =
        command_output("valgrind", &["--version"]).map_err(|_| "Valgrind required".to_string())?;
    require(
        version == "valgrind-3.22.0",
        "version requires a new calibrated protocol",
    )?;
    let evaluator_copy = out.join("evaluator/icprog");
    copy(&args.evaluator, &evaluator_copy)?;
    let source = std::path::absolute(&args.source_root).map_err(|e| e.to_string())?;
    let (binary, manifest) = snapshot_build(&source, &out, args.worker_binary.as_deref())?;
    let registry = match &args.candidates {
        Some(path) => json::read(path)?,
        None => default_candidates(),
    };
    let mut arms = registry
        .as_arr()
        .ok_or("candidate count must be 2..16")?
        .to_vec();
    require(
        (2..=16).contains(&arms.len()),
        "candidate count must be 2..16",
    )?;
    require(
        arms[0].get("id") == Some(&s("incumbent")),
        "first candidate must be incumbent",
    )?;
    let mut ids = BTreeSet::new();
    let mut hashes = BTreeSet::new();
    let mut built: HashMap<PathBuf, (PathBuf, PathBuf, J)> = HashMap::new();
    for arm in arms.iter_mut() {
        let name = str_at(arm, "id")?.to_string();
        let valid = name.len() <= 32
            && name.chars().next().is_some_and(|c| c.is_ascii_lowercase())
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');
        require(valid && !ids.contains(&name), "invalid candidate id")?;
        require(
            name != "rho" && name != "aa_control",
            "reserved candidate id",
        )?;
        let arm_manifest;
        if let Some(J::Str(root)) = arm.get("source_root").cloned() {
            require(name != "incumbent", "use --source-root for the incumbent")?;
            require(
                !args.wiring_control,
                "a wiring control has a single prebuilt worker",
            )?;
            let candidate_source = std::path::absolute(&root).map_err(|e| e.to_string())?;
            let (destination, arm_binary, m) = match built.get(&candidate_source) {
                Some(x) => x.clone(),
                None => {
                    let destination = out.join("source_candidates").join(&name);
                    std::fs::create_dir_all(&destination).map_err(|e| e.to_string())?;
                    let (b, m) = snapshot_build(&candidate_source, &destination, None)?;
                    built.insert(
                        candidate_source.clone(),
                        (destination.clone(), b.clone(), m.clone()),
                    );
                    (destination, b, m)
                }
            };
            let rel = |p: &Path| p.strip_prefix(&out).expect("inside").display().to_string();
            set(arm, "binary_relative", s(&rel(&arm_binary)));
            set(
                arm,
                "source_directory",
                s(&rel(&destination.join("source"))),
            );
            set(
                arm,
                "source_manifest_relative",
                s(&rel(&destination.join("source-manifest.json"))),
            );
            remove(arm, "source_root");
            arm_manifest = m;
        } else {
            arm_manifest = manifest.clone();
            set(arm, "binary_relative", s("worker"));
            set(arm, "source_directory", s("source"));
            set(arm, "source_manifest_relative", s("source-manifest.json"));
        }
        let h = objhash(at(arm, "config")?);
        let identity = objhash(&J::Arr(vec![
            J::Str(objhash(&arm_manifest)),
            J::Str(h.clone()),
        ]));
        require(
            !hashes.contains(&identity),
            "duplicate candidate source/configuration",
        )?;
        ids.insert(name);
        hashes.insert(identity);
        set(arm, "configuration_sha256", J::Str(h));
        set(
            arm,
            "source_manifest_sha256",
            J::Str(objhash(&arm_manifest)),
        );
    }
    write(&out.join("candidates.json"), &J::Arr(arms.clone()), true)?;
    let cpus = exec::affinity()?;
    let cpu = args
        .cpu
        .unwrap_or(*cpus.last().ok_or("no CPU in affinity")?);
    require(cpus.contains(&cpu), "CPU outside permitted affinity")?;
    let cells = parse_cells(&args.cells)?;
    let holdout = parse_cells(&args.holdout_cells)?;
    require(
        !cells.iter().any(|c| holdout.contains(c)),
        "a holdout cell is already a development cell",
    )?;
    let profile: [(&str, i128); 3] = match args.profile {
        Profile::Pilot => [("development", 3), ("selection", 3), ("confirmation", 12)],
        Profile::Standard => [
            ("development", 30),
            ("selection", 30),
            ("confirmation", 100),
        ],
        Profile::Wiring => [("development", 1), ("selection", 1), ("confirmation", 1)],
    };
    let count_for = |stage: &str| profile.iter().find(|(k, _)| *k == stage).map_or(1, |x| x.1);
    let name = |(n, a): &(i128, i128)| format!("n{n}a{a}");
    let all_cells: Vec<(i128, i128)> = cells.iter().chain(&holdout).copied().collect();
    let declared: Vec<String> = all_cells.iter().map(name).collect();
    let extra = parse_confirmation_allocation(
        &args.confirmation_cases,
        count_for("confirmation"),
        &declared,
    )?;
    let extra_for = |cell: &str| extra.iter().find(|(k, _)| k == cell).map(|x| x.1);
    let limits = J::Obj(vec![
        ("timeout_seconds".into(), J::Float(args.timeout)),
        ("memory_bytes".into(), J::Int(8 * 1024 * 1024 * 1024)),
        ("cpu".into(), J::Int(cpu as i128)),
        ("worker_threads".into(), J::Int(1)),
        ("max_profiled_jobs".into(), J::Int(args.max_processes)),
    ]);
    let mut rng = Random::new(args.seed as u128);
    let mut fixtures: Vec<(String, J)> = Vec::new();
    for stage in STAGES {
        if stage == "replay" {
            let confirmation = fixtures
                .iter()
                .find(|(k, _)| k == "confirmation")
                .expect("confirmation")
                .1
                .clone();
            fixtures.push((stage.into(), confirmation));
            continue;
        }
        let stage_cells: &[(i128, i128)] = if stage == "confirmation" {
            &all_cells
        } else {
            &cells
        };
        let count = count_for(stage);
        let mut cases = Vec::new();
        for cell in stage_cells {
            let cell_name = name(cell);
            let n_cases = if stage == "confirmation" {
                extra_for(&cell_name).unwrap_or(count)
            } else {
                count
            };
            for index in 0..n_cases {
                let public_seed = rng.getrandbits64();
                let mut target_seeds = vec![J::Int(public_seed as i128)];
                for _ in 1..args.targets {
                    target_seeds.push(J::Int(rng.getrandbits64() as i128));
                }
                let algorithm_seed = rng.getrandbits64();
                let mut job = J::Obj(vec![
                    ("mode".into(), s("fixture")),
                    ("degree".into(), J::Int(cell.0)),
                    ("curve_a".into(), J::Int(cell.1)),
                    ("target_seeds".into(), J::Arr(target_seeds)),
                    ("algorithm_seed".into(), J::Int(algorithm_seed as i128)),
                    (
                        "factor_base".into(),
                        J::Obj(vec![
                            ("kind".into(), s("subgroup_orbits")),
                            ("seed".into(), J::Int(43)),
                            ("points".into(), J::Int(6 * cell.0)),
                        ]),
                    ),
                    ("config".into(), base_config()),
                ]);
                let raw = exec::run_capture(&binary, &job, args.timeout)?;
                let fixture = at(&raw, "fixture")?.clone();
                oracle::Curve::new(&fixture, oracle::Version::Vae8f9b2a)?;
                // The current worker measures a supplied public point only.
                set(&mut job, "public_targets", at(&fixture, "targets")?.clone());
                let fixture_sha256 = objhash(&fixture);
                cases.push(J::Obj(vec![
                    ("id".into(), J::Str(format!("{cell_name}-{index:03}"))),
                    ("cell".into(), J::Str(cell_name.clone())),
                    ("job".into(), job),
                    ("fixture".into(), fixture),
                    ("fixture_sha256".into(), J::Str(fixture_sha256)),
                ]));
            }
        }
        fixtures.push((stage.into(), J::Arr(cases)));
    }
    let fixtures = J::Obj(fixtures);
    write(&out.join("fixtures.json"), &fixtures, true)?;
    write(
        &out.join("calibration.json"),
        &json::obj([
            ("unit", s(UNIT)),
            ("conversion", s("1 Ir event = 1 user-space guest instruction; identity, no curve-op conversion")),
            ("all_intervals_sum_to_collected_required", J::Bool(true)),
            ("boundary", s("complete worker user-space execution, including startup, allocations, reporting and all algorithm phases")),
            ("excluded", s("kernel/device work, profiler implementation and external independent audit; reported separately, not silently converted")),
            ("restrictions", s("CPU only; fixed compiler/ISA/profiler. This is implementation cost, not a hardware-independent arithmetic complexity claim.")),
            ("floor", s("For this required full-rank collector, K columns require at least K relation-producing trials and at least K guest instructions. Weak implementation-specific floor; never evidence of a non-generic advance.")),
            ("paired_aa_gate", s("same executable/config; both complete, no >=20% promotion, each cell within 5%")),
        ]),
        true,
    )?;
    let allocation: Vec<(String, J)> = declared
        .iter()
        .map(|c| {
            (
                c.clone(),
                J::Int(extra_for(c).unwrap_or(count_for("confirmation"))),
            )
        })
        .collect();
    let confirmation_cases: i128 = allocation
        .iter()
        .map(|(_, v)| v.as_i128().unwrap_or(0))
        .sum();
    let registration = if let Some(protocol) = &args.protocol {
        copy(protocol, &out.join("protocol.md"))?;
        json::obj([
            ("kind", s("registered")),
            ("protocol", s("protocol.md")),
            (
                "protocol_sha256",
                J::Str(store::digest_file(&out.join("protocol.md"))?),
            ),
        ])
    } else {
        json::obj([
            ("kind", s("wiring-control")),
            (
                "promotion",
                s("never: a wiring control checks the evaluator's plumbing and is not a result"),
            ),
        ])
    };
    let comparison_kind = match args.comparison_kind {
        ComparisonKind::FixedSupport => "fixed-support",
        ComparisonKind::FactorBasePolicy => "factor-base-policy",
    };
    let mut pinned: BTreeSet<String> = [
        "worker",
        "source-manifest.json",
        "candidates.json",
        "fixtures.json",
        "calibration.json",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    for a in &arms {
        pinned.insert(str_at(a, "binary_relative")?.to_string());
        pinned.insert(str_at(a, "source_manifest_relative")?.to_string());
    }
    if args.protocol.is_some() {
        pinned.insert("protocol.md".into());
    }
    let st = Store::Dir(out.clone());
    let pinned_files = J::Obj(
        pinned
            .into_iter()
            .map(|n| st.digest(&n).map(|d| (n, J::Str(d))))
            .collect::<R<_>>()?,
    );
    let c = J::Obj(vec![
        ("schema_version".into(), J::Int(1)),
        ("purpose".into(), s(if args.wiring_control { "wiring-control" } else { "improvement" })),
        ("registration".into(), registration),
        (
            "native_evaluator".into(),
            json::obj([
                ("port", s(PORT)),
                ("binary", s("evaluator/icprog")),
                (
                    "semantics",
                    json::obj([
                        ("tournament.py", s(TOURNAMENT_1E5B)),
                        ("oracle.py", s(oracle::ORACLE_AE8F9B2A)),
                    ]),
                ),
                ("job_addition", s("public_targets: the fixture's own points, which the current worker requires")),
            ]),
        ),
        ("profile".into(), s(match args.profile {
            Profile::Pilot => "pilot",
            Profile::Standard => "standard",
            Profile::Wiring => "wiring",
        })),
        ("seed".into(), J::Int(args.seed as i128)),
        ("created_unix".into(), J::Float(now_unix())),
        ("cells".into(), J::Arr(cells.iter().map(|c| J::Str(name(c))).collect())),
        ("holdout_cells".into(), J::Arr(holdout.iter().map(|c| J::Str(name(c))).collect())),
        ("confirmation_cases".into(), J::Int(confirmation_cases)),
        ("confirmation_cases_per_cell".into(), J::Obj(allocation)),
        (
            "confirmation_allocation".into(),
            s(if extra.is_empty() {
                "flat"
            } else {
                "per-cell; raised at the cells whose measured spread a flat count cannot resolve, never lowered, from frozen prior rounds only; estimator and gate unchanged"
            }),
        ),
        ("unit".into(), s(UNIT)),
        ("evidence_scope".into(), s("bounded public-hash ECDLP configuration tournament")),
        ("metric_class".into(), s("implementation_instruction_cost")),
        ("family_wide_or_scaling_claim".into(), J::Bool(false)),
        (
            "equivalent_suite_reason".into(),
            J::Str(format!("Point-base collector/rank/descent API, not WDSat ANF/conflict protocol. Fresh reference/candidates, independent point and scalar-field certificates; the final suite has {confirmation_cases} inputs.")),
        ),
        ("curve_diversity_limit".into(), s("One Koblitz curve per development degree; additional holdout curve in confirmation. Does not meet three curves per size for a broad family claim.")),
        ("limits".into(), limits),
        ("repetitions".into(), J::Int(3)),
        ("confirmation_ratio".into(), J::Float(0.8)),
        ("max_cell_ratio".into(), J::Float(1.1)),
        ("require_native_progress".into(), J::Bool(args.require_native_progress)),
        ("parity_margin".into(), J::Float(1.10)),
        ("objective".into(), s(if args.objective == Objective::Rho { "rho" } else { "incumbent" })),
        ("no_regression_ratio".into(), J::Float(0.98)),
        ("comparison_kind".into(), s(comparison_kind)),
        (
            "support_contract".into(),
            s(if args.comparison_kind == ComparisonKind::FactorBasePolicy {
                "Stable support within each arm/case; base policy varies across arms; same public ECDLPs and per-arm floors."
            } else {
                "Identical factor-base support across implementation arms."
            }),
        ),
        (
            "native_timing_protocol".into(),
            J::Str(format!("blocking process reap with independent watchdog; complete cold process wall; {}", exec::SPAWN)),
        ),
        ("ci_level".into(), J::Float(0.95)),
        ("bootstrap_draws".into(), J::Int(2000)),
        ("host".into(), uname()),
        ("profiler_version".into(), J::Str(version)),
        ("compiler".into(), J::Str(command_output("rustc", &["--version"])?)),
        (
            "evaluator_sha256".into(),
            json::obj([("icprog", J::Str(store::digest_file(&evaluator_copy)?))]),
        ),
        ("source_manifest_sha256".into(), J::Str(objhash(&manifest))),
        ("target_count".into(), J::Int(args.targets)),
        ("workload".into(), s("complete cold batch with all setup charged once and every target verified")),
        ("native_timings".into(), s("paired native reruns retained; no runtime claim from profiled elapsed time")),
        ("pinned_files".into(), pinned_files),
    ]);
    write(&out.join("contract.json"), &c, true)?;
    write(
        &out.join("seal.json"),
        &json::obj([("contract_sha256", J::Str(objhash(&c)))]),
        true,
    )?;
    Ok(json::obj([
        ("status", s("prepared")),
        ("round", J::Str(out.display().to_string())),
        (
            "command",
            J::Str(format!(
                "{} tournament run --round {}",
                evaluator_copy.display(),
                out.display()
            )),
        ),
    ]))
}

// ── the command line ────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum Step {
    /// Freeze a round: evaluator copy, source snapshot and worker build,
    /// candidates, fixtures, calibration, contract and seal.
    Prepare {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        source_root: PathBuf,
        #[arg(long, value_enum, default_value = "fixed-support")]
        comparison_kind: ComparisonKind,
        #[arg(long)]
        candidates: Option<PathBuf>,
        #[arg(long, value_enum, default_value = "pilot")]
        profile: Profile,
        #[arg(long, default_value = "")]
        confirmation_cases: String,
        #[arg(long, default_value = "13a0,17a1,19a0,23a0")]
        cells: String,
        #[arg(long, default_value = "19a1")]
        holdout_cells: String,
        /// The registration's own frozen seed; there is no default.
        #[arg(long)]
        seed: u64,
        #[arg(long)]
        cpu: Option<usize>,
        #[arg(long, default_value_t = 60.0)]
        timeout: f64,
        #[arg(long, default_value_t = 1800)]
        max_processes: i128,
        #[arg(long)]
        require_native_progress: bool,
        #[arg(long, default_value_t = 1)]
        targets: i128,
        #[arg(long, value_enum, default_value = "incumbent")]
        objective: Objective,
        /// The registered round's frozen protocol, pinned into the round.
        #[arg(long)]
        protocol: Option<PathBuf>,
        /// Freeze a wiring control instead: never promotes, not a result.
        #[arg(long)]
        wiring_control: bool,
        /// A wiring control's prebuilt worker, in place of a snapshot build.
        #[arg(long)]
        worker_binary: Option<PathBuf>,
    },
    /// Run a prepared round's stages, resuming where it stopped.
    Run {
        #[arg(long)]
        round: PathBuf,
        #[arg(long, default_value = "all")]
        stage: String,
    },
    /// The round's decision, else its state, else `prepared`.
    Status {
        #[arg(long)]
        round: PathBuf,
    },
    /// Re-check every receipt and re-derive every summary and the decision,
    /// from a round directory or from a committed evidence archive.
    Verify {
        #[arg(long, conflicts_with = "archive")]
        round: Option<PathBuf>,
        /// An `evidence/*.tar.zst` archive, read in memory.
        #[arg(long, requires = "member")]
        archive: Option<PathBuf>,
        /// The round's directory inside the archive, e.g. `runs/round-0020`.
        #[arg(long)]
        member: Option<String>,
        /// `evidence/manifest.json`, whose entry the archive must match.
        #[arg(long)]
        manifest: Option<PathBuf>,
    },
    /// A candidate registry: the defaults, or successors of a round's winner.
    Propose {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        from_round: Option<PathBuf>,
    },
}

pub fn main(step: Step) -> R<String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    match step {
        Step::Prepare {
            out,
            source_root,
            comparison_kind,
            candidates,
            profile,
            confirmation_cases,
            cells,
            holdout_cells,
            seed,
            cpu,
            timeout,
            max_processes,
            require_native_progress,
            targets,
            objective,
            protocol,
            wiring_control,
            worker_binary,
        } => {
            let result = prepare(&PrepareArgs {
                out,
                source_root,
                comparison_kind,
                candidates,
                profile,
                confirmation_cases,
                cells,
                holdout_cells,
                seed,
                cpu,
                timeout,
                max_processes,
                require_native_progress,
                targets,
                objective,
                protocol,
                wiring_control,
                worker_binary,
                evaluator: exe,
            })?;
            Ok(json::dumps_line(&result, false))
        }
        Step::Run { round, stage } => {
            let root = std::path::absolute(&round).map_err(|e| e.to_string())?;
            run_campaign(&root, &stage, &exe, &mut |line| {
                println!("{}", json::dumps_line(line, false))
            })?;
            Ok(String::new())
        }
        Step::Status { round } => {
            let st = Store::Dir(round);
            let v = if st.exists("decision.json") {
                read(&st, "decision.json")?
            } else if st.exists("state.json") {
                read(&st, "state.json")?
            } else {
                json::obj([("status", s("prepared"))])
            };
            Ok(json::dumps(&v, 2))
        }
        Step::Verify {
            round,
            archive,
            member,
            manifest,
        } => {
            let st = match (round, archive) {
                (Some(r), None) => Store::Dir(std::path::absolute(&r).map_err(|e| e.to_string())?),
                (None, Some(a)) => {
                    let entry = match &manifest {
                        Some(m) => {
                            let file = a
                                .file_name()
                                .map(|n| n.to_string_lossy().into_owned())
                                .unwrap_or_default();
                            let doc = json::read(m)?;
                            let e = at(&doc, "archives")?
                                .as_arr()
                                .and_then(|xs| {
                                    xs.iter()
                                        .find(|e| e.get("file") == Some(&J::Str(file.clone())))
                                })
                                .cloned()
                                .ok_or_else(|| format!("manifest lists no {file}"))?;
                            Some((
                                str_at(&e, "sha256")?.to_string(),
                                at(&e, "files")?.as_i128().unwrap_or(-1) as u64,
                                at(&e, "uncompressed_file_bytes")?.as_i128().unwrap_or(-1) as u64,
                            ))
                        }
                        None => None,
                    };
                    let expected = entry.as_ref().map(|(sha, n, b)| (sha.as_str(), *n, *b));
                    store::load_archive(&a, member.as_deref().expect("required"), expected)?.0
                }
                _ => return Err("name --round DIR or --archive FILE --member DIR".into()),
            };
            // A native round requires its own frozen evaluator copy; a
            // historical round is checked against its pinned Python copies.
            let native = read(&st, "contract.json")?
                .get("native_evaluator")
                .is_some();
            let result = audit(&st, native.then_some(exe.as_path()))?;
            Ok(json::dumps_line(&result, false))
        }
        Step::Propose { out, from_round } => {
            if let Some(round) = from_round {
                let root = std::path::absolute(&round).map_err(|e| e.to_string())?;
                let st = Store::Dir(root.clone());
                let (proposed, source) = proposals_from_previous(&st, &root.display().to_string())?;
                write(&out, &proposed, true)?;
                let parent = read(&st, "contract.json")?;
                Ok(json::dumps_line(
                    &json::obj([
                        ("candidates", J::Str(out.display().to_string())),
                        ("baseline_source_root", J::Str(source)),
                        ("target_count", parent.get("target_count").cloned().unwrap_or(J::Int(1))),
                        ("comparison_kind", parent.get("comparison_kind").cloned().unwrap_or(s("fixed-support"))),
                        ("require_native_progress", parent.get("require_native_progress").cloned().unwrap_or(J::Bool(false))),
                        ("next_step", s("Use this source, preserve target count, comparison kind and metric gates, and choose a new campaign seed. A different target count is a separate workload panel.")),
                    ]),
                    false,
                ))
            } else {
                write(&out, &default_candidates(), true)?;
                Ok(out.display().to_string())
            }
        }
    }
}

#[cfg(test)]
#[path = "tournament_tests.rs"]
mod tests;
