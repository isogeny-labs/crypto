//! Specifications, plans, sessions, sealed records and the replay audit.
//!
//! A specification (`isobench.spec/v1`) is frozen before any run: one problem with its
//! parameters, the sizes, instances per size, the arms (method + role) and the measurement
//! (rounds, seed). [`plan`] expands it deterministically: every (size, instance, round) gets one
//! algorithm seed shared by all arms, so arms are paired, and the arm order alternates by round.
//! Each execution runs in a fresh child process (`isobench exec`): a counted run (problem unit)
//! and, for algebraic problems, a separately timed plain run whose answer must equal the counted
//! run's. The parent checks every answer with the problem's independent verifier and appends a
//! sealed record. `audit` re-hashes everything, re-plans the spec, re-verifies every answer and
//! re-executes chosen runs, which must reproduce answer, cost, tally and work exactly.
use super::counted::{tally_json, Calibration, Tally};
use super::problems::{self, Params};
use crate::json::Json;
use crate::sha256::sha256_hex;
use std::collections::BTreeMap;
use std::path::Path;

pub const SPEC_SCHEMA: &str = "isobench.spec/v1";
pub const RECORD_SCHEMA: &str = "isobench.record/v1";

/// Seal a document: `prefix` + the first 32 hex digits of SHA-256 of its compact form.
pub fn seal(prefix: &str, doc: &Json) -> String {
    format!("{prefix}{}", &sha256_hex(doc.dump().as_bytes())[..32])
}

/// Add an `id` field (the seal of the document without it) at the front.
pub fn sealed(prefix: &str, doc: Json) -> Json {
    let id = seal(prefix, &doc);
    match doc {
        Json::Obj(mut kv) => {
            kv.insert(0, ("id".into(), Json::str(id)));
            Json::Obj(kv)
        }
        other => other,
    }
}

/// The seal a sealed document should carry (its `id` recomputed).
pub fn reseal(prefix: &str, doc: &Json) -> Option<(String, String)> {
    let Json::Obj(kv) = doc else { return None };
    let claimed = doc.get("id")?.as_str()?.to_string();
    let rest = Json::Obj(kv.iter().filter(|(k, _)| k != "id").cloned().collect());
    Some((claimed, seal(prefix, &rest)))
}

#[derive(Clone, Debug)]
pub struct Arm {
    pub name: String,
    /// reference | baseline | candidate | control
    pub role: String,
    pub method: String,
}

#[derive(Clone, Debug)]
pub struct Spec {
    pub doc: Json,
    pub id: String,
    pub label: String,
    pub problem: String,
    pub params: Params,
    pub sizes: Vec<u64>,
    pub instances: u64,
    pub arms: Vec<Arm>,
    pub rounds: u64,
    pub seed: u64,
}

fn s<'a>(j: &'a Json, k: &str) -> Result<&'a str, String> {
    j.get(k)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("spec: {k} missing"))
}
fn n(j: &Json, k: &str) -> Result<u64, String> {
    j.get(k)
        .and_then(|v| v.as_f64())
        .map(|x| x as u64)
        .ok_or_else(|| format!("spec: {k} missing"))
}

impl Spec {
    pub fn parse(text: &str) -> Result<Spec, String> {
        let doc = Json::parse(text)?;
        if s(&doc, "schema")? != SPEC_SCHEMA {
            return Err(format!("not an {SPEC_SCHEMA} document"));
        }
        let problem = s(&doc, "problem")?.to_string();
        let decl = problems::problem(&problem)?;
        let mut given = Params::new();
        if let Some(Json::Obj(kv)) = doc.get("params") {
            for (k, v) in kv {
                let v = v
                    .as_str()
                    .map(String::from)
                    .or_else(|| v.as_f64().map(|x| (x as i64).to_string()))
                    .ok_or("spec: params are strings or integers")?;
                given.insert(k.clone(), v);
            }
        }
        let params = problems::resolve_params(decl, &given)?;
        let Some(Json::Arr(sz)) = doc.get("sizes") else {
            return Err("spec: sizes missing".into());
        };
        let sizes: Vec<u64> = sz
            .iter()
            .filter_map(|v| v.as_f64())
            .map(|x| x as u64)
            .collect();
        if sizes.is_empty() {
            return Err("spec: no sizes".into());
        }
        let Some(Json::Arr(aj)) = doc.get("arms") else {
            return Err("spec: arms missing".into());
        };
        let mut arms = vec![];
        for a in aj {
            let arm = Arm {
                name: s(a, "name")?.into(),
                role: s(a, "role")?.into(),
                method: s(a, "method")?.into(),
            };
            if !["reference", "baseline", "candidate", "control"].contains(&arm.role.as_str()) {
                return Err(format!("arm {}: unknown role {}", arm.name, arm.role));
            }
            if !decl.methods.contains(&arm.method.as_str()) {
                return Err(format!(
                    "arm {}: {problem} has no method {}",
                    arm.name, arm.method
                ));
            }
            if arms.iter().any(|b: &Arm| b.name == arm.name) {
                return Err(format!("duplicate arm {}", arm.name));
            }
            arms.push(arm);
        }
        if arms.is_empty() {
            return Err("spec: no arms".into());
        }
        let m = doc.get("measurement").ok_or("spec: measurement missing")?;
        Ok(Spec {
            id: seal("ISS1h", &doc),
            label: s(&doc, "label")?.into(),
            instances: n(&doc, "instances_per_size")?,
            rounds: n(m, "rounds")?,
            seed: n(m, "seed")?,
            doc,
            problem,
            params,
            sizes,
            arms,
        })
    }
}

/// One planned execution.
#[derive(Clone, Debug, PartialEq)]
pub struct Exec {
    pub no: usize,
    pub size: u64,
    pub index: u64,
    pub round: u64,
    pub arm: usize,
    pub seed: u64,
}

/// Expand a spec: size-major, then instance, then round; within one (size, instance, round)
/// every arm with the same seed, in spec order on even rounds and reversed on odd ones.
pub fn plan(spec: &Spec) -> Vec<Exec> {
    let mut out = vec![];
    for &size in &spec.sizes {
        for index in 0..spec.instances {
            for round in 0..spec.rounds {
                let h = sha256_hex(format!("{}/{size}/{index}/{round}", spec.seed).as_bytes());
                let seed = u64::from_str_radix(&h[..16], 16).unwrap();
                let mut order: Vec<usize> = (0..spec.arms.len()).collect();
                if round % 2 == 1 {
                    order.reverse();
                }
                for arm in order {
                    out.push(Exec {
                        no: out.len(),
                        size,
                        index,
                        round,
                        arm,
                        seed,
                    });
                }
            }
        }
    }
    out
}

pub fn plan_json(spec: &Spec, p: &[Exec]) -> Json {
    Json::obj(vec![
        ("spec", Json::str(&spec.id)),
        (
            "executions",
            Json::Arr(
                p.iter()
                    .map(|e| {
                        Json::Arr(vec![
                            Json::Num(e.no as i64),
                            Json::Num(e.size as i64),
                            Json::Num(e.index as i64),
                            Json::Num(e.round as i64),
                            Json::str(&spec.arms[e.arm].name),
                            Json::str(e.seed),
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
}

// ------------------------------------------------------------------ child

/// What one child execution reports.
#[derive(Clone, Debug)]
pub struct ChildOut {
    pub answer: Option<String>,
    pub cost: f64,
    pub tally: Option<Tally>,
    pub work: BTreeMap<String, u64>,
    /// wall time of the method call in the timed (plain) run, ms
    pub wall_ms: f64,
    pub error: Option<String>,
}

/// The child's work: the counted run, and for counted problems a timed plain run whose answer
/// must be the same.
pub fn child(
    problem: &str,
    method: &str,
    params: &Params,
    size: u64,
    index: u64,
    seed: u64,
    cal: &Calibration,
) -> ChildOut {
    let decl = match problems::problem(problem) {
        Ok(d) => d,
        Err(e) => return failed(e),
    };
    let fme = decl.unit == super::counted::UNIT;
    let first = problems::run(problem, method, params, size, index, seed, fme, cal);
    let mut wall = first.as_ref().map_or(0.0, |o| o.ms);
    let mut out = match first {
        Ok(o) => ChildOut {
            answer: o.answer,
            cost: o.cost,
            tally: o.tally,
            work: o.work,
            wall_ms: 0.0,
            error: None,
        },
        Err(e) => return failed(e),
    };
    if fme {
        match problems::run(problem, method, params, size, index, seed, false, cal) {
            Ok(o) if o.answer == out.answer => wall = o.ms,
            Ok(_) => out.error = Some("the counted and the timed run disagree".into()),
            Err(e) => out.error = Some(e),
        }
    }
    out.wall_ms = wall;
    out
}

fn failed(e: String) -> ChildOut {
    ChildOut {
        answer: None,
        cost: 0.0,
        tally: None,
        work: BTreeMap::new(),
        wall_ms: 0.0,
        error: Some(e),
    }
}

pub fn child_json(o: &ChildOut) -> Json {
    Json::obj(vec![
        ("answer", o.answer.as_ref().map_or(Json::Null, Json::str)),
        ("cost", Json::Float(o.cost)),
        ("tally", o.tally.as_ref().map_or(Json::Null, tally_json)),
        (
            "work",
            Json::Obj(
                o.work
                    .iter()
                    .map(|(k, v)| (k.clone(), Json::Num(*v as i64)))
                    .collect(),
            ),
        ),
        (
            "wall_ms",
            Json::Float((o.wall_ms * 1000.0).round() / 1000.0),
        ),
        ("error", o.error.as_ref().map_or(Json::Null, Json::str)),
    ])
}

// ------------------------------------------------------------------ records

/// Build the sealed record of one execution (the parent verifies the answer here).
#[allow(clippy::too_many_arguments)]
pub fn record(spec: &Spec, e: &Exec, child: &Json, cal: &Calibration, isolation: Json) -> Json {
    let arm = &spec.arms[e.arm];
    let decl = problems::problem(&spec.problem).unwrap();
    let answer = child
        .get("answer")
        .and_then(|a| a.as_str())
        .map(String::from);
    let error = child
        .get("error")
        .and_then(|a| a.as_str())
        .map(String::from);
    let (status, detail) = match (&error, &answer) {
        (Some(err), _) => ("error", err.clone()),
        (None, None) => (
            "exhausted",
            "the method gave up within its budget".to_string(),
        ),
        (None, Some(a)) => {
            match problems::verify(&spec.problem, &arm.method, &spec.params, e.size, e.index, a) {
                Ok(d) => ("verified", d),
                Err(d) => ("wrong_answer", d),
            }
        }
    };
    let doc = Json::obj(vec![
        ("schema", Json::str(RECORD_SCHEMA)),
        ("spec", Json::str(&spec.id)),
        ("exec", Json::Num(e.no as i64)),
        ("problem", Json::str(&spec.problem)),
        ("size", Json::Num(e.size as i64)),
        ("index", Json::Num(e.index as i64)),
        (
            "instance",
            Json::str(problems::instance_id(
                &spec.problem,
                &spec.params,
                e.size,
                e.index,
            )),
        ),
        ("round", Json::Num(e.round as i64)),
        ("arm", Json::str(&arm.name)),
        ("role", Json::str(&arm.role)),
        ("method", Json::str(&arm.method)),
        ("seed", Json::str(e.seed)),
        ("status", Json::str(status)),
        ("check", Json::str(detail)),
        ("unit", Json::str(decl.unit)),
        ("cost", child.get("cost").cloned().unwrap_or(Json::Null)),
        (
            "calibration",
            if decl.unit == super::counted::UNIT {
                Json::str(&cal.id)
            } else {
                Json::Null
            },
        ),
        ("tally", child.get("tally").cloned().unwrap_or(Json::Null)),
        ("work", child.get("work").cloned().unwrap_or(Json::Null)),
        (
            "wall_ms",
            child.get("wall_ms").cloned().unwrap_or(Json::Null),
        ),
        ("isolation", isolation),
        (
            "answer_sha256",
            answer
                .as_ref()
                .map_or(Json::Null, |a| Json::str(sha256_hex(a.as_bytes()))),
        ),
        ("answer", answer.map_or(Json::Null, Json::str)),
    ]);
    sealed("ISR1h", doc)
}

/// Host facts for `host.json`.
pub fn host_json() -> Json {
    let cpu = std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|t| {
            t.lines()
                .find(|l| l.starts_with("model name"))
                .map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string())
        })
        .unwrap_or_default();
    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(0);
    let exe = std::env::current_exe()
        .ok()
        .and_then(|p| std::fs::read(p).ok())
        .map(|b| sha256_hex(&b))
        .unwrap_or_default();
    // the commit the session ran at: exact replays of a committed session are made there
    let commit = std::env::var("ISOBENCH_COMMIT").ok().or_else(|| {
        std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    });
    Json::obj(vec![
        ("commit", commit.map_or(Json::Null, Json::str)),
        ("os", Json::str(std::env::consts::OS)),
        ("arch", Json::str(std::env::consts::ARCH)),
        ("cpu_model", Json::str(cpu)),
        ("logical_cpus", Json::Num(cpus as i64)),
        ("ifma", Json::Bool(crate::ntt::simd_active())),
        ("binary_sha256", Json::str(exe)),
        ("version", Json::str(env!("CARGO_PKG_VERSION"))),
    ])
}

pub fn file_sha(p: &Path) -> Result<String, String> {
    std::fs::read(p)
        .map(|b| sha256_hex(&b))
        .map_err(|e| format!("{}: {e}", p.display()))
}

/// Read a session's records (one JSON object per line).
pub fn read_records(dir: &Path) -> Result<Vec<Json>, String> {
    let text = std::fs::read_to_string(dir.join("records.jsonl"))
        .map_err(|e| format!("records.jsonl: {e}"))?;
    text.lines()
        .filter(|l| !l.is_empty())
        .map(Json::parse)
        .collect()
}

/// `session.json`: what was run, with the hashes of every file.
pub fn session_json(
    dir: &Path,
    spec: &Spec,
    cal: &Calibration,
    records: &[Json],
) -> Result<Json, String> {
    let mut by_status: BTreeMap<String, i64> = BTreeMap::new();
    for r in records {
        *by_status
            .entry(
                r.get("status")
                    .and_then(|s| s.as_str())
                    .unwrap_or("?")
                    .into(),
            )
            .or_default() += 1;
    }
    let doc = Json::obj(vec![
        ("schema", Json::str("isobench.session/v1")),
        ("spec", Json::str(&spec.id)),
        ("label", Json::str(&spec.label)),
        ("problem", Json::str(&spec.problem)),
        ("calibration", Json::str(&cal.id)),
        ("records", Json::Num(records.len() as i64)),
        (
            "status",
            Json::Obj(
                by_status
                    .into_iter()
                    .map(|(k, v)| (k, Json::Num(v)))
                    .collect(),
            ),
        ),
        (
            "files",
            Json::obj(
                ["spec.json", "plan.json", "host.json", "records.jsonl"]
                    .iter()
                    .map(|f| Ok((f.to_string(), Json::str(file_sha(&dir.join(f))?))))
                    .collect::<Result<Vec<_>, String>>()?,
            ),
        ),
    ]);
    Ok(sealed("ISSN1h", doc))
}

// ------------------------------------------------------------------ audit

/// One problem found by [`audit`].
pub type Finding = String;

/// Re-hash every file, re-seal every record, re-plan the spec, re-verify every answer, and
/// re-execute the records chosen by `replay` (via `rerun`, which must reproduce the child
/// output). Returns the findings (empty: the session audits clean) and the number replayed.
pub fn audit(
    dir: &Path,
    cal: &Calibration,
    replay: Option<usize>,
    rerun: &dyn Fn(&Spec, &Exec) -> Result<Json, String>,
) -> Result<(Vec<Finding>, usize), String> {
    let mut bad = vec![];
    let spec_text =
        std::fs::read_to_string(dir.join("spec.json")).map_err(|e| format!("spec.json: {e}"))?;
    let spec = Spec::parse(&spec_text)?;
    let session = Json::parse(
        &std::fs::read_to_string(dir.join("session.json")).map_err(|e| e.to_string())?,
    )?;
    match reseal("ISSN1h", &session) {
        Some((a, b)) if a == b => {}
        _ => bad.push("session.json: seal does not match".into()),
    }
    if let Some(Json::Obj(files)) = session.get("files") {
        for (f, h) in files {
            if Some(file_sha(&dir.join(f))?.as_str()) != h.as_str() {
                bad.push(format!("{f}: hash differs from session.json"));
            }
        }
    }
    if session.get("calibration").and_then(|c| c.as_str()) != Some(cal.id.as_str()) {
        bad.push(format!(
            "calibration {} differs from the session's {}",
            cal.id,
            session
                .get("calibration")
                .and_then(|c| c.as_str())
                .unwrap_or("?")
        ));
    }
    let p = plan(&spec);
    let pj =
        Json::parse(&std::fs::read_to_string(dir.join("plan.json")).map_err(|e| e.to_string())?)?;
    if pj.dump() != plan_json(&spec, &p).dump() {
        bad.push("plan.json differs from the re-planned spec".into());
    }
    let records = read_records(dir)?;
    if records.len() != p.len() {
        bad.push(format!(
            "{} records for {} planned executions",
            records.len(),
            p.len()
        ));
    }
    for (i, r) in records.iter().enumerate() {
        match reseal("ISR1h", r) {
            Some((a, b)) if a == b => {}
            _ => bad.push(format!("record {i}: seal does not match")),
        }
        let Some(e) = p.get(i) else { continue };
        if r.get("exec").and_then(|v| v.as_f64()) != Some(e.no as f64)
            || r.get("arm").and_then(|v| v.as_str()) != Some(spec.arms[e.arm].name.as_str())
            || r.get("seed").and_then(|v| v.as_str()) != Some(e.seed.to_string().as_str())
        {
            bad.push(format!(
                "record {i}: does not match execution {i} of the plan"
            ));
            continue;
        }
        let status = r.get("status").and_then(|s| s.as_str()).unwrap_or("");
        if status == "verified" {
            let a = r.get("answer").and_then(|a| a.as_str()).unwrap_or("");
            if let Err(d) = problems::verify(
                &spec.problem,
                &spec.arms[e.arm].method,
                &spec.params,
                e.size,
                e.index,
                a,
            ) {
                bad.push(format!("record {i}: answer no longer verifies: {d}"));
            }
        }
    }
    let picks: Vec<usize> = match replay {
        None => (0..p.len()).collect(),
        Some(0) => vec![],
        Some(k) => {
            let k = k.min(p.len());
            (0..k).map(|t| t * p.len() / k).collect()
        }
    };
    for &i in &picks {
        let out = rerun(&spec, &p[i])?;
        let r = &records[i];
        for key in ["answer", "cost", "tally", "work"] {
            let want = r.get(key).cloned().unwrap_or(Json::Null);
            let got = out.get(key).cloned().unwrap_or(Json::Null);
            if !same(&want, &got) {
                bad.push(format!("record {i}: replay {key} differs"));
            }
        }
    }
    Ok((bad, picks.len()))
}

/// Equality of record values, numbers compared by value (parsed vs built).
fn same(a: &Json, b: &Json) -> bool {
    match (a, b) {
        (Json::Arr(x), Json::Arr(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| same(p, q))
        }
        (Json::Obj(x), Json::Obj(y)) => {
            x.len() == y.len()
                && x.iter()
                    .zip(y)
                    .all(|((k1, v1), (k2, v2))| k1 == k2 && same(v1, v2))
        }
        _ => match (a.as_f64(), b.as_f64()) {
            (Some(x), Some(y)) => x == y,
            _ => a == b,
        },
    }
}
