//! Local L0 pilot; see LOCAL_COMPACT_PILOT_PROTOCOL.md before execution.
use certified_relations::{workload, Poly, System};
use compact_speed_analysis::{aa_control, analyze, Arm, Case, Sample};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
use compact_speed_analysis::cas_formats::{msolve_basis, singular_basis, Basis};
const P: u64 = 2147483647;
const ARMS: [Arm; 3] = [Arm::F4, Arm::Certified, Arm::Compact];
const ORDERS: [&str; 6] = ["FEC", "ECF", "CFE", "CEF", "EFC", "FCE"];
#[path = "../matched_fresh.rs"]
mod matched_fresh;
#[path = "../relation_ablations.rs"]
mod relation_ablations;
#[path = "../rank_relations.rs"]
mod rank_relations;
#[path = "../direct_paired.rs"]
mod direct_paired;
#[path = "../identity_paired.rs"]
mod identity_paired;
#[path = "../callgrind_capture.rs"]
mod callgrind_capture;
#[path = "../phase_capture.rs"]
mod phase_capture;
#[path = "../replay_capture.rs"]
mod replay_capture;
#[path = "../root_paired.rs"]
mod root_paired;
#[path = "../rust_capture.rs"]
mod rust_capture;
#[path = "../sparse_capture.rs"]
mod sparse_capture;
fn arm_name(a: Arm) -> &'static str {
    match a {
        Arm::F4 => "f4",
        Arm::Certified => "certified",
        Arm::Compact => "compact",
    }
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn file_hash(p: &Path) -> Result<String> {
    Ok(hash(&fs::read(p)?))
}
fn ns(start: Instant) -> u64 {
    start.elapsed().as_nanos().min(u64::MAX as u128) as u64
}
fn write_new(p: &Path, b: &[u8]) -> Result<()> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)?
        .write_all(b)?;
    Ok(())
}
fn save(p: &Path, v: &impl serde::Serialize) -> Result<()> {
    write_new(p, &serde_json::to_vec_pretty(v)?)
}
fn expr(f: &Poly, names: &[String]) -> String {
    f.iter()
        .map(|(m, c)| {
            let mut s = c.to_string();
            for (e, v) in m.iter().zip(names) {
                if *e > 0 {
                    s += &format!("*{v}^{e}");
                }
            }
            s
        })
        .collect::<Vec<_>>()
        .join("+")
}
fn singular_code(s: &System, f: &[Poly], algorithm: &str) -> String {
    format!("ring r={},({}),dp;\noption(redSB);\nideal I={};\nideal G={algorithm}(I);\nint i;poly f;\nfor(i=1;i<=size(G);i++){{print(\"POLY\");f=G[i];while(f!=0){{print(\"TERM:\"+string(leadcoef(f))+\":\"+string(leadexp(f)));f=f-lead(f);}}print(\"ENDPOLY\");}}\nprint(\"DONE\");\nquit;\n",s.prime,s.variables.join(","),f.iter().map(|f|expr(f,&s.variables)).collect::<Vec<_>>().join(","))
}
fn native_basis(value: &Value, n: usize) -> Result<Basis> {
    let raw: Vec<Vec<(Vec<usize>, u64)>> = serde_json::from_value(value.clone())?;
    let mut out = Basis::new();
    for row in raw {
        let mut f = Poly::new();
        for (m, c) in row {
            if m.len() != n
                || m.iter()
                    .try_fold(0usize, |s, e| s.checked_add(*e))
                    .is_none_or(|d| d > 64)
                || c == 0
                || c >= P
                || f.insert(m, c).is_some()
            {
                return Err("bad native term".into());
            }
        }
        if f.is_empty() || !out.insert(f) {
            return Err("empty/duplicate native polynomial".into());
        }
    }
    if out.is_empty() {
        return Err("missing native basis".into());
    }
    Ok(out)
}
fn basis_hash(b: &Basis) -> String {
    let rows: Vec<Vec<_>> = b
        .iter()
        .map(|f| f.iter().map(|(m, c)| (m, *c)).collect())
        .collect();
    hash(&serde_json::to_vec(&rows).unwrap())
}
fn prepare(root: &Path, mode: &str) -> Result<()> {
    if mode == "graph-screen" {
        return prepare_graphs(root, false, "graph-screen");
    }
    if mode == "scale-screen" {
        return prepare_graphs(root, true, "scale-screen");
    }
    if mode == "replay-screen" {
        return prepare_graphs(root, true, "replay-screen");
    }
    if mode == "sparse-screen" {
        return prepare_graphs(root, true, "sparse-screen");
    }
    if mode == "rust-consumer-screen" {
        return prepare_graphs(root, true, "rust-consumer-screen");
    }
    let roster: Vec<_> = match mode {
        "qualification" => [3].into_iter().map(|n| (n, 7)).collect(),
        "pilot" => [3, 4, 6]
            .into_iter()
            .flat_map(|n| [101, 307, 911].map(|s| (n, s)))
            .collect(),
        _ => return Err("mode must be qualification or pilot".into()),
    };
    fs::create_dir(root)?;
    let mut cases = Vec::new();
    for (n, seed) in roster {
        let start = Instant::now();
        let s = workload::generate(P, n, seed)?.system;
        let dir = root.join(format!("n{n}-s{seed}"));
        fs::create_dir(&dir)?;
        let f = s.decode()?;
        let bytes = serde_json::to_vec(&s)?;
        write_new(&dir.join("system.json"), &bytes)?;
        for algorithm in ["std", "slimgb"] {
            write_new(
                &dir.join(format!("{algorithm}.sing")),
                singular_code(&s, &f, algorithm).as_bytes(),
            )?;
        }
        let ms = format!(
            "{}\n{}\n{}\n",
            s.variables.join(","),
            P,
            f.iter()
                .map(|f| expr(f, &s.variables))
                .collect::<Vec<_>>()
                .join(",\n")
        );
        write_new(&dir.join("input.ms"), ms.as_bytes())?;
        cases.push(json!({"id":format!("n{n}-s{seed}"),"variables":n,"seed":seed,"input_sha256":hash(&bytes),"preparation_ns":ns(start)}));
    }
    save(
        &root.join("manifest.json"),
        &json!({"schema":"local-compact-inputs/v1","mode":mode,"cases":cases}),
    )
}
type GraphDefinition = (&'static str, Vec<(usize, usize)>);
fn graph_roster(n: usize) -> Result<Vec<GraphDefinition>> {
    let (path, balanced, cycle) = match n {
        6 => (
            vec![0, 1, 2, 5, 3, 4],
            vec![(5, 3), (5, 4), (3, 0), (3, 1), (4, 2)],
            vec![0, 3, 1, 5, 2, 4],
        ),
        7 => (
            vec![0, 1, 2, 6, 3, 4, 5],
            vec![(6, 4), (6, 5), (4, 0), (4, 1), (5, 2), (5, 3)],
            vec![0, 4, 1, 6, 2, 5, 3],
        ),
        8 => (
            vec![0, 1, 2, 3, 7, 4, 5, 6],
            vec![(7, 5), (7, 6), (5, 2), (5, 3), (6, 4), (2, 0), (4, 1)],
            vec![0, 4, 1, 6, 2, 7, 3, 5],
        ),
        _ => return Err("graph screen n must be 6/7/8".into()),
    };
    let path = path.windows(2).map(|w| (w[0], w[1])).collect();
    let mut cycle_edges: Vec<_> = cycle.windows(2).map(|w| (w[0], w[1])).collect();
    cycle_edges.push((*cycle.last().unwrap(), cycle[0]));
    Ok(vec![
        ("path", path),
        ("balanced", balanced),
        ("star", (0..n - 1).map(|i| (i, n - 1)).collect()),
        ("permuted-cycle", cycle_edges),
    ])
}
fn screen_score(s: &Value) -> Option<f64> {
    if s["status"] != "screen_complete" || s["eligible_for_preregistered_followup"] != true {
        return None;
    }
    let rows = s["comparisons"].as_array()?;
    if rows.len() != 2 || rows[0]["control"] != "f4" || rows[1]["control"] != "certified" {
        return None;
    }
    let mut score = f64::INFINITY;
    for row in rows {
        let r = row["full_cost_single_call_ratio"].as_f64()?;
        if !r.is_finite() || r < 10. {
            return None;
        }
        score = score.min(r);
    }
    Some(score)
}
fn select_screen_family(summaries: &[Value]) -> Value {
    let mut selected = None;
    let mut best = f64::NEG_INFINITY;
    for s in summaries {
        if let Some(score) = screen_score(s) {
            if score > best {
                best = score;
                selected = Some(s["id"].clone());
            }
        }
    }
    selected.unwrap_or(Value::Null)
}
fn graph_plan() -> Result<Vec<(usize, GraphDefinition)>> {
    let mut plan = Vec::new();
    for n in [6, 7, 8] {
        for graph in graph_roster(n)? {
            plan.push((n, graph));
        }
    }
    for n in [6, 7, 8] {
        let balanced = match n {
            6 => vec![(5, 0), (5, 1), (0, 2), (0, 3), (1, 4)],
            7 => vec![(6, 0), (6, 1), (0, 2), (0, 3), (1, 4), (1, 5)],
            _ => vec![(7, 0), (7, 1), (0, 2), (0, 3), (1, 4), (2, 5), (4, 6)],
        };
        plan.push((n, ("star-first", (1..n).map(|i| (0, i)).collect())));
        plan.push((n, ("balanced-first", balanced)));
    }
    Ok(plan)
}
fn scale_plan() -> Vec<(usize, GraphDefinition)> {
    [10, 12, 16]
        .into_iter()
        .flat_map(|n| {
            [
                (n, ("star-first", (1..n).map(|i| (0, i)).collect())),
                (n, ("star-last", (0..n - 1).map(|i| (i, n - 1)).collect())),
            ]
        })
        .collect()
}
/// Graph descriptor for x0-first elimination; not a grevlex solving degree.
fn induced_width(n: usize, edges: &[(usize, usize)]) -> usize {
    let mut neighbors = vec![BTreeSet::new(); n];
    for &(a, b) in edges {
        neighbors[a].insert(b);
        neighbors[b].insert(a);
    }
    let mut width = 0;
    for axis in 0..n {
        let later: Vec<_> = neighbors[axis]
            .iter()
            .copied()
            .filter(|v| *v > axis)
            .collect();
        width = width.max(later.len());
        for (i, &a) in later.iter().enumerate() {
            for &b in &later[i + 1..] {
                neighbors[a].insert(b);
                neighbors[b].insert(a);
            }
        }
    }
    width
}
fn prepare_graphs(root: &Path, scale: bool, mode: &str) -> Result<()> {
    if scale && !cfg!(feature = "scale-screen") {
        return Err("scale-screen requires the explicit research feature".into());
    }
    let plan = if scale { scale_plan() } else { graph_plan()? };
    fs::create_dir(root)?;
    let mut cases = Vec::new();
    for (n, (name, edges)) in plan {
        let start = Instant::now();
        let system = if scale {
            #[cfg(feature = "scale-screen")]
            {
                workload::generate_scale_graph(P, n, 7, &edges)?.system
            }
            #[cfg(not(feature = "scale-screen"))]
            {
                return Err("scale-screen feature disabled".into());
            }
        } else {
            workload::generate_graph(P, n, 7, &edges)?.system
        };
        let id = format!("{name}-n{n}-s7");
        let dir = root.join(&id);
        fs::create_dir(&dir)?;
        let f = system.decode()?;
        let bytes = serde_json::to_vec(&system)?;
        if matches!(
            mode,
            "replay-screen" | "sparse-screen" | "rust-consumer-screen"
        ) {
            let declared: Value =
                serde_json::from_str(include_str!("../../../scale-screen-01/input-manifest.json"))?;
            let expected = declared["cases"]
                .as_array()
                .ok_or("published roster")?
                .iter()
                .find(|c| c["id"] == id)
                .ok_or("published case")?;
            if expected["input_sha256"] != hash(&bytes) {
                return Err("replay generation differs from declared public input".into());
            }
        }
        write_new(&dir.join("system.json"), &bytes)?;
        for algorithm in ["std", "slimgb"] {
            write_new(
                &dir.join(format!("{algorithm}.sing")),
                singular_code(&system, &f, algorithm).as_bytes(),
            )?;
        }
        let ms = format!(
            "{}\n{}\n{}\n",
            system.variables.join(","),
            P,
            f.iter()
                .map(|f| expr(f, &system.variables))
                .collect::<Vec<_>>()
                .join(",\n")
        );
        write_new(&dir.join("input.ms"), ms.as_bytes())?;
        cases.push(json!({"id":id,"topology":name,"variables":n,"seed":7,"edges":edges,"induced_width_x0_first":induced_width(n,&edges),"input_sha256":hash(&bytes),"preparation_ns":ns(start)}));
    }
    save(
        &root.join("manifest.json"),
        &json!({"schema":"graph-screen-inputs/v1","mode":mode,"cases":cases}),
    )
}
struct Runner {
    worker: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    cpu: String,
    worker_hash: String,
    global: Instant,
    case: Instant,
    global_limit: Duration,
    case_limit: Duration,
}
impl Runner {
    fn process(
        &self,
        exe: &Path,
        args: &[String],
        input: Option<&Path>,
        prefix: &Path,
    ) -> Result<(std::process::ExitStatus, Vec<u8>, u64)> {
        let remaining = self
            .global_limit
            .saturating_sub(self.global.elapsed())
            .min(self.case_limit.saturating_sub(self.case.elapsed()))
            .min(Duration::from_secs(30));
        if remaining.is_zero() {
            return Err("original pilot/case deadline exhausted".into());
        }
        let start = Instant::now();
        let stdout = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(prefix.with_extension("stdout"))?;
        let stderr = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(prefix.with_extension("stderr"))?;
        let mut cmd = Command::new("timeout");
        cmd.args([
            "--kill-after=1s",
            &format!("{}s", remaining.as_secs_f64()),
            "prlimit",
            "--as=2147483648",
            "--fsize=16777216",
            "--",
            "taskset",
            "-c",
            &self.cpu,
        ]);
        cmd.arg(exe)
            .args(args)
            .env("RAYON_NUM_THREADS", "1")
            .env("OMP_NUM_THREADS", "1")
            .env("OPENBLAS_NUM_THREADS", "1")
            .env("SINGULARPATH", "/workspace/native-cas/share/singular/LIB")
            .env(
                "LD_LIBRARY_PATH",
                "/workspace/msolve-source/src/neogb/.libs:/workspace/native-cas/lib",
            )
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr));
        cmd.stdin(match input {
            Some(p) => Stdio::from(File::open(p)?),
            None => Stdio::null(),
        });
        let command = format!("{cmd:?}");
        let status = cmd.status()?;
        let bytes = fs::read(prefix.with_extension("stdout"))?;
        let wall = ns(start);
        save(
            &prefix.with_extension("process.json"),
            &json!({"command":command,"exit_code":status.code(),"success":status.success(),"parent_process_wall_ns":wall,"scope":"launch through bounded stdout consumption; parser/checking charged by caller"}),
        )?;
        Ok((status, bytes, wall))
    }
    fn references(&self, input: &Path, out: &Path, s: &System, prep: u64) -> Result<(Basis, u64)> {
        let start = Instant::now();
        let mut first = None;
        let mut routes = Vec::new();
        for algorithm in ["std", "slimgb", "msolve"] {
            let route = Instant::now();
            let prefix = out.join(format!("setup-{algorithm}"));
            let (exe, args) = if algorithm == "msolve" {
                (
                    &self.msolve,
                    vec![
                        "-g".into(),
                        "2".into(),
                        "-t".into(),
                        "1".into(),
                        "-v".into(),
                        "0".into(),
                        "-f".into(),
                        input.join("input.ms").display().to_string(),
                        "-o".into(),
                        out.join("msolve-basis.txt").display().to_string(),
                    ],
                )
            } else {
                (
                    &self.singular,
                    vec![
                        "-q".into(),
                        input
                            .join(format!("{algorithm}.sing"))
                            .display()
                            .to_string(),
                    ],
                )
            };
            let (status, bytes, _) = self.process(exe, &args, None, &prefix)?;
            if !status.success() || !fs::read(prefix.with_extension("stderr"))?.is_empty() {
                return Err(format!("reference {algorithm} failed: {status}").into());
            }
            let basis = if algorithm == "msolve" {
                msolve_basis(
                    &fs::read_to_string(out.join("msolve-basis.txt"))?,
                    &s.variables,
                )?
            } else {
                singular_basis(std::str::from_utf8(&bytes)?, s.variables.len())?
            };
            if first.as_ref().is_some_and(|b| *b != basis) {
                return Err("mature bases disagree".into());
            }
            if first.is_none() {
                first = Some(basis);
            }
            routes.push(json!({"route":algorithm,"whole_route_ns":ns(route)}));
        }
        let basis = first.unwrap();
        let reference_hash = basis_hash(&basis);
        let fastest = routes
            .iter()
            .min_by_key(|r| r["whole_route_ns"].as_u64().unwrap())
            .unwrap()["route"]
            .clone();
        save(
            &out.join("references.json"),
            &json!({"routes":routes,"fastest_successful_route":fastest,"reference_basis_sha256":reference_hash,"basis":basis.iter().map(|f|f.iter().collect::<Vec<_>>()).collect::<Vec<_>>()}),
        )?;
        Ok((basis, prep.checked_add(ns(start)).ok_or("setup overflow")?))
    }
    fn sample(
        &self,
        a: Arm,
        input: &Path,
        out: &Path,
        n: usize,
        reference: &Basis,
        input_hash: &str,
    ) -> Result<Sample> {
        let start = Instant::now();
        let (status, bytes, _) = self.process(
            &self.worker,
            &[
                arm_name(a).into(),
                "--degree-cap".into(),
                "12".into(),
                "--deadline-ms".into(),
                "10000".into(),
            ],
            Some(&input.join("system.json")),
            out,
        )?;
        let value: Value = serde_json::from_slice(&bytes)?;
        let complete = status.success()
            && fs::read(out.with_extension("stderr"))?.is_empty()
            && value["status"] == "complete"
            && value["basis"].as_array().is_some_and(|a| !a.is_empty());
        let worker_wall_ns = ns(start);
        let validation = Instant::now();
        let basis = if complete {
            native_basis(&value["basis"], n).ok()
        } else {
            None
        };
        let equal = basis.as_ref().is_some_and(|b| b == reference);
        let basis_sha256 = basis.as_ref().map(basis_hash).unwrap_or_default();
        let external_basis_sha256 = basis_hash(reference);
        let validation_wall_ns = ns(validation).max(1);
        save(
            &out.with_extension("call.json"),
            &json!({"arm":a,"complete":complete,"mature_basis_equal":equal,"worker_wall_ns":worker_wall_ns,"validation_wall_ns":validation_wall_ns,"basis_sha256":basis_sha256,"reference_basis_sha256":external_basis_sha256,"worker_output":value}),
        )?;
        if !complete || !equal {
            return Err(format!("{} incomplete or mature basis mismatch", arm_name(a)).into());
        }
        Ok(Sample {
            arm: a,
            complete: true,
            input_sha256: input_hash.into(),
            worker_sha256: self.worker_hash.clone(),
            validation_input_sha256: input_hash.into(),
            worker_wall_ns,
            validation_wall_ns,
            basis_sha256,
            external_basis_sha256,
        })
    }
}
fn run(
    prepared: &Path,
    out: &Path,
    worker: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    cpu: String,
) -> Result<()> {
    let manifest: Value = serde_json::from_slice(&fs::read(prepared.join("manifest.json"))?)?;
    let qualification = manifest["mode"] == "qualification";
    let graph_screen = manifest["mode"] == "graph-screen" || manifest["mode"] == "scale-screen";
    if !qualification && !graph_screen && manifest["mode"] != "pilot" {
        return Err("unknown prepared mode".into());
    }
    let worker_hash = file_hash(&worker)?;
    let singular_hash = file_hash(&singular)?;
    let msolve_hash = file_hash(&msolve)?;
    fs::create_dir(out)?;
    let mut runner = Runner {
        worker,
        singular,
        msolve,
        cpu,
        worker_hash,
        global: Instant::now(),
        case: Instant::now(),
        global_limit: Duration::from_secs(if graph_screen { 600 } else { 1800 }),
        case_limit: Duration::from_secs(if graph_screen { 180 } else { 1200 }),
    };
    save(
        &out.join("pre-call.json"),
        &json!({"schema":"local-compact-run/v1","mode":manifest["mode"],"prepared_manifest_sha256":file_hash(&prepared.join("manifest.json"))?,"adapter_sha256":file_hash(&std::env::current_exe()?)?,"worker_sha256":runner.worker_hash,"singular_sha256":singular_hash,"msolve_sha256":msolve_hash,"cpu":runner.cpu,"backend":"local L0; no exclusive-core/L2 claim","checkpoint":"waived by user in this thread","cpuinfo":fs::read_to_string("/proc/cpuinfo")?}),
    )?;
    let mut summaries = Vec::new();
    for item in manifest["cases"].as_array().ok_or("missing cases")? {
        let id = item["id"].as_str().ok_or("missing id")?;
        let input = prepared.join(id);
        let dir = out.join(id);
        fs::create_dir(&dir)?;
        runner.case = Instant::now();
        let mut transcript = None;
        let outcome = (|| -> Result<Value> {
            let bytes = fs::read(input.join("system.json"))?;
            let input_hash = hash(&bytes);
            if Some(input_hash.as_str()) != item["input_sha256"].as_str() {
                return Err("changed prepared input".into());
            }
            let s: System = serde_json::from_slice(&bytes)?;
            let n = s.variables.len();
            let (reference, setup) = runner.references(
                &input,
                &dir,
                &s,
                item["preparation_ns"]
                    .as_u64()
                    .ok_or("missing preparation cost")?,
            )?;
            if graph_screen {
                let mut calls = Vec::new();
                let mut successful = Vec::new();
                for arm in ARMS {
                    match runner.sample(
                        arm,
                        &input,
                        &dir.join(format!("screen-{}", arm_name(arm))),
                        n,
                        &reference,
                        &input_hash,
                    ) {
                        Ok(sample) => {
                            calls.push(json!({"arm":arm,"status":"complete","sample":sample}));
                            successful.push(sample);
                        }
                        Err(e) => {
                            calls.push(json!({"arm":arm,"status":"failed","reason":e.to_string()}))
                        }
                    }
                }
                let complete = successful.len() == 3;
                let comparisons = if complete {
                    let paid = |s: &Sample| -> Result<u64> {
                        Ok(s.worker_wall_ns
                            .checked_add(s.validation_wall_ns)
                            .and_then(|x| x.checked_add(setup))
                            .ok_or("screen cost overflow")?)
                    };
                    let candidate = &successful[2];
                    let c_full = paid(candidate)?;
                    let rows:Vec<_>=successful[..2].iter().map(|s|Ok(json!({"control":s.arm,"native_single_call_ratio":s.worker_wall_ns as f64/candidate.worker_wall_ns as f64,"full_cost_single_call_ratio":paid(s)? as f64/c_full as f64}))).collect::<Result<_>>()?;
                    json!(rows)
                } else {
                    Value::Null
                };
                let eligible = complete
                    && comparisons
                        .as_array()
                        .unwrap()
                        .iter()
                        .all(|r| r["full_cost_single_call_ratio"].as_f64().unwrap() >= 10.);
                let screen = json!({"schema":"graph-screen-case/v1","id":id,"topology":item["topology"],"variables":n,"seed":7,"edges":item["edges"],"input_sha256":input_hash,"worker_sha256":runner.worker_hash,"reference_basis_sha256":basis_hash(&reference),"setup_wall_ns":setup,"calls":calls,"comparisons":comparisons,"eligible_for_preregistered_followup":eligible,"status":if complete{"screen_complete"}else{"screen_incomplete"},"standing":"one planned call per arm; no A/A, confidence interval, confirmed gain or family standing"});
                let screen = {
                    let mut value = screen;
                    value["induced_width_x0_first"] = item["induced_width_x0_first"].clone();
                    value
                };
                save(&dir.join("screen-transcript.json"), &screen)?;
                return Ok(screen);
            }
            transcript = Some(Case {
                schema: "compact-speed-transcript/v1".into(),
                prime: P,
                variables: n,
                seed: item["seed"].as_u64().ok_or("missing seed")?,
                input_sha256: input_hash.clone(),
                worker_sha256: runner.worker_hash.clone(),
                reference_basis_sha256: basis_hash(&reference),
                setup_wall_ns: setup,
                warmups: Vec::new(),
                aa_pairs: std::array::from_fn(|_| Vec::new()),
                blocks: Vec::new(),
            });
            let case = transcript.as_mut().unwrap();
            for a in ARMS {
                case.warmups.push(runner.sample(
                    a,
                    &input,
                    &dir.join(format!("warmup-{}", arm_name(a))),
                    n,
                    &reference,
                    &input_hash,
                )?);
            }
            if qualification {
                return Ok(json!({"id":id,"status":"qualification_complete","ratios":null}));
            }
            for (i, a) in ARMS.iter().enumerate() {
                for j in 0..7 {
                    let left = runner.sample(
                        *a,
                        &input,
                        &dir.join(format!("aa-{}-{j}-0", arm_name(*a))),
                        n,
                        &reference,
                        &input_hash,
                    )?;
                    let right = runner.sample(
                        *a,
                        &input,
                        &dir.join(format!("aa-{}-{j}-1", arm_name(*a))),
                        n,
                        &reference,
                        &input_hash,
                    )?;
                    case.aa_pairs[i].push([left, right]);
                }
                let aa = aa_control(*a, &case.aa_pairs[i], setup)?;
                save(&dir.join(format!("aa-{}.json", arm_name(*a))), &aa)?;
                if !aa.admitted {
                    return Err(format!("{} A/A rejected", arm_name(*a)).into());
                }
            }
            for i in 0..21 {
                let mut block = Vec::new();
                for c in ORDERS[i % 6].chars() {
                    let a = match c {
                        'F' => Arm::F4,
                        'E' => Arm::Certified,
                        _ => Arm::Compact,
                    };
                    block.push(runner.sample(
                        a,
                        &input,
                        &dir.join(format!("block-{i}-{}", arm_name(a))),
                        n,
                        &reference,
                        &input_hash,
                    )?);
                }
                case.blocks.push(block);
            }
            let report = analyze(case);
            let tenfold = report.comparisons.as_ref().is_some_and(|v| {
                v.iter()
                    .all(|c| c.full_cost.adjusted_lower_1_over_720 > 10.)
            });
            save(&dir.join("analysis.json"), &report)?;
            Ok(
                json!({"id":id,"status":report.status,"conditional_numerical_15x":report.numerical_rule_passes,"conditional_numerical_10x":tenfold,"comparisons":report.comparisons,"setup_wall_ns":setup}),
            )
        })();
        if let Some(case) = transcript {
            save(&dir.join("transcript.json"), &case)?;
        }
        let summary = match outcome {
            Ok(s) => s,
            Err(e) => {
                json!({"id":id,"status":"case_failed","reason":e.to_string(),"missing_remainder":"not replaced","comparisons":null})
            }
        };
        save(&dir.join("outcome.json"), &summary)?;
        println!("{id}: {}", summary["status"]);
        summaries.push(summary);
    }
    let unchanged = file_hash(&runner.worker)? == runner.worker_hash
        && file_hash(&runner.singular)? == singular_hash
        && file_hash(&runner.msolve)? == msolve_hash;
    let selected = if graph_screen {
        select_screen_family(&summaries)
    } else {
        Value::Null
    };
    save(
        &out.join("summary.json"),
        &json!({"schema":"local-compact-summary/v1","mode":manifest["mode"],"identities_unchanged":unchanged,"scope":"unsigned producer L0 exploration; not formal protocol admission, independent replay or fresh family confirmation","selected_followup_family":selected,"cases":summaries}),
    )?;
    if !unchanged {
        return Err("executable identity changed during run".into());
    }
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str){
    Some("prepare")if a.len()==4=>prepare(Path::new(&a[2]),&a[3]),
    Some("identity-crosscheck")if a.len()==4=>identity_paired::crosscheck(Path::new(&a[2]),Path::new(&a[3])),
    Some("identity-paired")if a.len()==5=>identity_paired::run(Path::new(&a[2]),Path::new(&a[3]),&a[4]),
    Some("identity-paired-report")if a.len()==5=>identity_paired::report(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("direct-paired")if a.len()==5=>direct_paired::run(Path::new(&a[2]),Path::new(&a[3]),&a[4]),
    Some("direct-paired-report")if a.len()==5=>direct_paired::report(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("rank-matrix-probe")if a.len()==3=>rank_relations::matrix_probe(Path::new(&a[2])),
    Some("rank-matrix-audit")if a.len()==4=>rank_relations::matrix_audit(Path::new(&a[2]),Path::new(&a[3])),
    Some("rank-study")if a.len()==5=>rank_relations::run(Path::new(&a[2]),Path::new(&a[3]),&a[4]),
    Some("rank-build")if a.len()==5=>rank_relations::build(Path::new(&a[2]),Path::new(&a[3]),a[4].parse()?),
    Some("rank-report")if a.len()==5=>rank_relations::report(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("relation-ablations")if a.len()==5=>relation_ablations::run(Path::new(&a[2]),Path::new(&a[3]),&a[4]),
    Some("relation-diagnostics")if a.len()==6=>relation_ablations::diagnostics(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]),a[5].parse()?),
    Some("relation-ablation-report")if a.len()==5=>relation_ablations::report(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("fresh-native")if a.len()==5=>matched_fresh::run(Path::new(&a[2]),Path::new(&a[3]),&a[4]),
    Some("fresh-native-report")if a.len()==6=>matched_fresh::report(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]),&a[5]),
    Some("run")if a.len()==8=>run(Path::new(&a[2]),Path::new(&a[3]),PathBuf::from(&a[4]),PathBuf::from(&a[5]),PathBuf::from(&a[6]),a[7].clone()),
    Some("replay-screen")if a.len()==9=>replay_capture::run(Path::new(&a[2]),Path::new(&a[3]),PathBuf::from(&a[4]),PathBuf::from(&a[5]),PathBuf::from(&a[6]),PathBuf::from(&a[7]),a[8].clone()),
    Some("sparse-screen")if a.len()==10=>sparse_capture::run(Path::new(&a[2]),Path::new(&a[3]),sparse_capture::Paths{worker:PathBuf::from(&a[4]),legacy_cli:PathBuf::from(&a[5]),fused:PathBuf::from(&a[6]),singular:PathBuf::from(&a[7]),msolve:PathBuf::from(&a[8]),cpu:a[9].clone()}),
    Some("phase-profile")if a.len()==8=>phase_capture::run(Path::new(&a[2]),Path::new(&a[3]),PathBuf::from(&a[4]),PathBuf::from(&a[5]),PathBuf::from(&a[6]),a[7].clone()),
    Some("phase-render")if a.len()==4=>phase_capture::render(Path::new(&a[2]),Path::new(&a[3])),
    Some("rust-screen")if a.len()==5=>rust_capture::run(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("m31-screen")if a.len()==5=>rust_capture::run_m31(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("export-screen")if a.len()==5=>rust_capture::run_export(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("kernel-screen")if a.len()==5=>rust_capture::run_kernels(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("update-screen")if a.len()==5=>rust_capture::run_updates(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("lead-screen")if a.len()==5=>rust_capture::run_leads(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("fused-screen")if a.len()==5=>rust_capture::run_fused(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("frontier-screen")if a.len()==5=>rust_capture::run_frontier(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("callgrind-profile")if a.len()==5=>callgrind_capture::run(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("fused-profile")if a.len()==5=>callgrind_capture::run_fused(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("root-paired")if a.len()==5=>root_paired::run(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("update-paired")if a.len()==5=>root_paired::run_last(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    _=>Err("usage: local-pilot prepare NEW_DIR qualification|pilot|graph-screen|scale-screen OR local-pilot run INPUT_DIR NEW_OUTPUT WORKER SINGULAR MSOLVE CPU".into())}
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_star_roster_preserves_shallow_depth_and_input_budget() {
        let plan = scale_plan();
        assert_eq!(plan.len(), 6);
        for (index, (n, (name, edges))) in plan.into_iter().enumerate() {
            assert_eq!(n, [10, 12, 16][index / 2]);
            assert_eq!(edges.len(), n - 1);
            assert!(edges.len() * 2 <= 32);
            if index % 2 == 0 {
                assert_eq!(name, "star-first");
                assert!(edges.iter().all(|(a, b)| *a == 0 && *b > 0));
                assert_eq!(induced_width(n, &edges), n - 1);
            } else {
                assert_eq!(name, "star-last");
                assert!(edges.iter().all(|(a, b)| *a < n - 1 && *b == n - 1));
                assert_eq!(induced_width(n, &edges), 1);
            }
        }
    }
    #[test]
    fn legacy_cycles_match_original_measured_input_hashes() {
        let expected = [
            (
                3,
                101,
                "03df9a50153c547305c76f2b29228e592b3437c39ae61a28051e0985731fd184",
            ),
            (
                3,
                307,
                "6205704ddf23b2c8e90e97c1985ab0e8dc15f40a0343accbb4e466ccb748dfa3",
            ),
            (
                3,
                911,
                "2295ce56363ecdc00187dfd6be090fcfa28b7eef8eb1abde7baceb28a0620b76",
            ),
            (
                4,
                101,
                "b216e2697ed9e620c0eb6287c2952359b15e4a8704dbbceedeac43a00bb35c08",
            ),
            (
                4,
                307,
                "1304d8c53c17e1e70210f5b1f256e25b4e25e5ff7f31274ef7d0ccf6222302ad",
            ),
            (
                4,
                911,
                "fc26b0269928209598b755c26e1967bb3ab2d6031e317c80c231538a06e6a506",
            ),
            (
                6,
                101,
                "25d5cc7ef27544bdb7a89dc4f86e9d3bbe6e3ff678f9169eeda8eb029e01294d",
            ),
            (
                6,
                307,
                "c6fa6935298c3760b480f6493581cb13f1afa7f5fc62fdef18f91ffa48dcb750",
            ),
            (
                6,
                911,
                "cef8a012c9c5c9d1e4c03cf0755df2bcc75af6d4e9e13b384ccb2477966ca12d",
            ),
        ];
        for (n, seed, golden) in expected {
            let s = workload::generate(P, n, seed).unwrap().system;
            assert_eq!(hash(&serde_json::to_vec(&s).unwrap()), golden);
        }
    }
    #[test]
    fn graph_roster_and_paid_selection_are_fixed() {
        for n in [6, 7, 8] {
            let rows = graph_roster(n).unwrap();
            assert_eq!(
                rows.iter().map(|r| r.0).collect::<Vec<_>>(),
                ["path", "balanced", "star", "permuted-cycle"]
            );
            assert_eq!(rows[0].1.len(), n - 1);
            assert_eq!(rows[3].1.len(), n);
        }
        let record = |id, f, e, status| json!({"id":id,"status":status,"eligible_for_preregistered_followup":true,"comparisons":[{"control":"f4","full_cost_single_call_ratio":f,"native_single_call_ratio":100.},{"control":"certified","full_cost_single_call_ratio":e,"native_single_call_ratio":100.}]});
        assert!(
            select_screen_family(&[record("synthetic-low", 2., 20., "screen_complete")]).is_null()
        );
        assert_eq!(
            select_screen_family(&[
                record("synthetic-first", 12., 20., "screen_complete"),
                record("synthetic-tie", 12., 30., "screen_complete"),
                record("synthetic-failed", 100., 100., "screen_incomplete")
            ]),
            json!("synthetic-first")
        );
    }
    #[test]
    fn exact_signed_singular_and_msolve_agree() {
        let s = singular_basis("POLY\nTERM:1:1,0\nTERM:-1:0,0\nENDPOLY\nDONE", 2).unwrap();
        let m = msolve_basis("[1*x1^1+2147483646]:", &["x1".into(), "x2".into()]).unwrap();
        assert_eq!(s, m);
    }
    #[test]
    fn amended_plan_preserves_original_order_and_exercises_fill() {
        let plan = graph_plan().unwrap();
        assert_eq!(plan.len(), 18);
        for (i, n) in [6, 7, 8].into_iter().enumerate() {
            assert_eq!(plan[4 * i].0, n);
            assert_eq!(plan[4 * i].1 .0, "path");
            assert_eq!(plan[12 + 2 * i].0, n);
            assert_eq!(plan[12 + 2 * i].1 .0, "star-first");
            assert_eq!(induced_width(n, &plan[12 + 2 * i].1 .1), n - 1);
            assert_eq!(induced_width(n, &plan[4 * i + 2].1 .1), 1);
            assert_eq!(induced_width(n, &plan[13 + 2 * i].1 .1), 3);
        }
    }
    #[test]
    fn incomplete_and_duplicate_cas_outputs_rejected() {
        for s in [
            "POLY\nTERM:1:1,0\nENDPOLY",
            "POLY\nTERM:1:1,0\nTERM:2:1,0\nENDPOLY\nDONE",
            "DONE",
        ] {
            assert!(singular_basis(s, 2).is_err());
        }
        assert!(msolve_basis("[1*x1^1+2*x1^1]:", &["x1".into()]).is_err());
    }
    #[test]
    fn native_shape_residue_and_duplicate_rejected() {
        assert!(native_basis(&json!([[[[1, 0], 1], [[1, 0], 2]]]), 2).is_err());
        assert!(native_basis(&json!([[[[1], P]]]), 1).is_err());
        assert!(native_basis(&json!([]), 2).is_err());
    }
}
