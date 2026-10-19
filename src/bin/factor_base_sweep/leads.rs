//! Bounded, replayable comparisons of three decomposition constructions.
//! Counts are stage diagnostics; no complete pricing or regularity claim.
use super::{
    leads_algebra as algebra,
    model::{read, write_new, Result},
};
use crypto_lib::cryptanalysis::{
    ecbench::canonical::{derive_u64, sha256_hex},
    f4_fp::{self, F4Options, Ordering, Verdict},
    ic_boundary::{koblitz_instance, BinaryInstance},
    koblitz_fast::FastPoint,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
    process::{Command, Stdio},
    sync::mpsc,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Pt {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}
impl From<FastPoint> for Pt {
    fn from(p: FastPoint) -> Self {
        Self {
            x: p.x,
            y: p.y,
            infinity: p.infinity,
        }
    }
}
impl From<Pt> for FastPoint {
    fn from(p: Pt) -> Self {
        Self {
            x: p.x,
            y: p.y,
            infinity: p.infinity,
        }
    }
}
fn neg(p: Pt) -> Pt {
    if p.infinity {
        p
    } else {
        Pt { y: p.x ^ p.y, ..p }
    }
}
fn add(i: &BinaryInstance, p: Pt, q: Pt) -> Pt {
    i.fast.add(p.into(), q.into()).into()
}
fn scale(i: &BinaryInstance, p: Pt, k: u64) -> Pt {
    i.fast.mul_u64(p.into(), k).into()
}
fn psi(i: &BinaryInstance, p: Pt) -> Pt {
    i.fast.frobenius_k(i.fast.double(p.into()), i.n - 1).into()
}
fn hex(s: &str) -> Result<u64> {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|e| e.to_string())
}
fn sha(path: &Path) -> Result<String> {
    Ok(sha256_hex(&fs::read(path).map_err(|e| e.to_string())?))
}
fn sorted(points: impl IntoIterator<Item = Pt>) -> Vec<Pt> {
    points
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeadCase {
    pub id: String,
    pub kind: String,
    pub left: String,
    pub right: Option<String>,
    pub left_take: usize,
    pub right_take: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeadPlan {
    pub schema: String,
    pub seed: u64,
    pub train: u32,
    pub holdout: u32,
    pub positives: u32,
    pub max_degree: u32,
    pub solve_millis: u64,
    pub hard_millis: u64,
    pub max_matrix_cells: u64,
    pub memory_mib: u64,
    pub cases: Vec<LeadCase>,
}
impl LeadPlan {
    fn validate(&self) -> Result<()> {
        if self.schema != "factor_base_sweep.leads/v1"
            || self.cases.is_empty()
            || self.cases.len() > 32
            || self.train > 256
            || self.holdout > 256
            || !(1..=16).contains(&self.positives)
            || !(2..=8).contains(&self.max_degree)
            || !(10..=10_000).contains(&self.solve_millis)
            || self.hard_millis < self.solve_millis
            || self.hard_millis > 15_000
            || !(1..=10_000_000).contains(&self.max_matrix_cells)
            || !(256..=4096).contains(&self.memory_mib)
        {
            return Err("invalid bounded lead plan".into());
        }
        let mut ids = BTreeSet::new();
        for c in &self.cases {
            if c.id.is_empty()
                || c.id.len() > 64
                || !c.id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
                || !ids.insert(&c.id)
                || !matches!(c.kind.as_str(), "charts" | "torsion")
                || (c.kind == "charts" && c.right.is_none())
                || c.left_take > 64
                || c.right_take > 64
            {
                return Err("invalid or duplicate lead case".into());
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Job {
    plan: LeadPlan,
    a: u8,
    n: u32,
    slug: String,
    arm: String,
    left: Vec<Pt>,
    right: Vec<Pt>,
    charts: Option<(Vec<u64>, Vec<u64>)>,
    target: Pt,
}
fn instance(job: &Job) -> Result<BinaryInstance> {
    let i = koblitz_instance(job.a, job.n).ok_or("unsupported binary instance")?;
    if i.curve_id().slug != job.slug {
        return Err("curve identity mismatch".into());
    }
    Ok(i)
}
fn selected(base: &Value, take: usize) -> Result<Vec<Pt>> {
    let mut pts = Vec::new();
    for p in base["points"].as_array().ok_or("missing points")? {
        let x = hex(p["x"].as_str().ok_or("missing x")?)?;
        if x != 0 {
            pts.push(Pt {
                x,
                y: hex(p["y"].as_str().ok_or("missing y")?)?,
                infinity: false,
            });
        }
    }
    let xs: Vec<_> = pts
        .iter()
        .map(|p| p.x)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let keep: BTreeSet<_> = xs
        .into_iter()
        .take(if take == 0 { usize::MAX } else { take })
        .collect();
    let out = sorted(pts.into_iter().filter(|p| keep.contains(&p.x)));
    if out.is_empty() || out.len() > 128 {
        return Err("empty or over-budget lead inventory".into());
    }
    Ok(out)
}
fn truth(i: &BinaryInstance, left: &[Pt], right: &[Pt], target: Pt) -> Option<[Pt; 2]> {
    for &a in left {
        for &b in right {
            if add(i, a, b) == target {
                return Some([a, b]);
            }
        }
    }
    None
}
fn support(i: &BinaryInstance, points: &[Pt]) -> Vec<Pt> {
    sorted(
        points
            .iter()
            .flat_map(|a| points.iter().map(move |b| add(i, *a, *b)))
            .filter(|p| !p.infinity && scale(i, *p, i.r).infinity),
    )
}
fn check_inventory(i: &BinaryInstance, p: &[Pt]) -> Result<()> {
    for q in p {
        if q.infinity || !i.fast.is_on_curve((*q).into()) || !p.contains(&neg(*q)) {
            return Err("inventory must be finite, on curve and signed".into());
        }
    }
    Ok(())
}
fn emit(v: &Value) -> Result<()> {
    let mut out = std::io::stdout().lock();
    writeln!(out, "{v}")
        .and_then(|_| out.flush())
        .map_err(|e| e.to_string())
}
fn solve_branch(
    job: &Job,
    i: &BinaryInstance,
    dir: &Path,
    index: usize,
    left: &[Pt],
    right: &[Pt],
    charts: Option<(&[u64], &[u64])>,
) -> Result<Value> {
    let system = algebra::encode(
        i,
        &algebra::xs(left),
        &algebra::xs(right),
        charts,
        job.target.x,
    );
    write_new(&dir.join(format!("system-{index}.json")), &system)?;
    let (tx, rx) = mpsc::channel();
    let relay = std::thread::spawn(move || {
        let mut degree = 0;
        for event in rx {
            let e: f4_fp::F4Progress = event;
            if e.event == "complete" {
                degree = degree.max(e.productive_degree);
            }
            let _ = emit(
                &json!({"event":"solver","branch":index,"kind":e.event,"degree":e.degree,"productive_degree":e.productive_degree,"rows":e.rows,"columns":e.columns,"field_multiplications":e.field_multiplications}),
            );
        }
        degree
    });
    let solved = f4_fp::observe_progress(tx, || {
        f4_fp::solve(
            &system.equations,
            system.variables,
            2,
            &F4Options::new(Ordering::Grevlex, job.plan.max_degree)
                .with_budget(Duration::from_millis(job.plan.solve_millis)),
        )
    });
    let productive = relay.join().map_err(|_| "lead progress relay panicked")?;
    let expected = truth(i, left, right, job.target);
    let mut witness = None;
    let mut roots = Vec::new();
    let mut extraction_pairs = 0u64;
    let status = match &solved.verdict {
        Verdict::Undetermined => "unresolved",
        Verdict::Inconsistent => {
            if expected.is_some() {
                return Err("false refutation of independently verified point pair".into());
            }
            "absent"
        }
        Verdict::Solutions(found) => {
            for root in found {
                if system
                    .equations
                    .iter()
                    .any(|f| f4_fp::eval(f, root, 2) != 0)
                {
                    return Err("invalid algebraic root".into());
                }
                roots.push(root.clone());
                let x = algebra::decode(root, &system.basis_left, 0);
                let y = algebra::decode(root, &system.basis_right, system.basis_left.len());
                for &p in left.iter().filter(|p| p.x == x) {
                    for &q in right.iter().filter(|p| p.x == y) {
                        extraction_pairs += 1;
                        if add(i, p, q) == job.target && witness.is_none() {
                            witness = Some([p, q]);
                        }
                    }
                }
            }
            if witness.is_none() && expected.is_some() {
                return Err("completed solver missed a real decomposition".into());
            }
            if witness.is_some() {
                "verified"
            } else {
                "absent"
            }
        }
    };
    Ok(
        json!({"status":status,"ground_truth":expected,"witness":witness,"roots":roots,"variables":system.variables,"encoding_term_products":system.encoding_term_products,"field_reduction_multiplications":solved.field_ops,"f4_runs":solved.f4_runs,"max_rows":solved.max_rows,"max_columns":solved.max_cols,"degree_processed":solved.degree_reached,"observed_productive_degree":productive,"input_degree":system.equations.iter().flat_map(|f|f.iter().map(|(m,_)|m.iter().sum::<u32>())).max().unwrap_or(0),"extraction_pair_checks":extraction_pairs,"timed_out":solved.timed_out,"system":format!("system-{index}.json"),"system_sha256":sha(&dir.join(format!("system-{index}.json")))?,"certified_degree_of_regularity":null}),
    )
}
pub fn worker(path: &Path) -> Result<()> {
    let job: Job = read(path)?;
    job.plan.validate()?;
    let i = instance(&job)?;
    check_inventory(&i, &job.left)?;
    check_inventory(&i, &job.right)?;
    let dir = path.parent().ok_or("job has no parent")?;
    let mut branches = Vec::new();
    if job.arm == "asymmetric-branches" {
        let mut order = [
            (&job.left, &job.left),
            (&job.left, &job.right),
            (&job.right, &job.right),
        ];
        // Fixed size-based order, before inspecting the target's truth.
        order.sort_by_key(|(a, b)| a.len() * b.len());
        for (index, (a, b)) in order.iter().enumerate() {
            let v = solve_branch(&job, &i, dir, index, a, b, None)?;
            let done = v["status"] == "verified";
            branches.push(v);
            if done {
                break;
            }
        }
    } else {
        branches.push(solve_branch(
            &job,
            &i,
            dir,
            0,
            &job.left,
            &job.right,
            job.charts
                .as_ref()
                .map(|(a, b)| (a.as_slice(), b.as_slice())),
        )?);
    }
    let witness = branches
        .iter()
        .find_map(|b| b["witness"].as_array().map(|_| b["witness"].clone()));
    let status = if witness.is_some() {
        "verified"
    } else if branches.iter().any(|b| b["status"] == "unresolved") {
        "unresolved"
    } else {
        "absent"
    };
    write_new(
        &dir.join("result.json"),
        &json!({"status":status,"witness":witness,"branches":branches}),
    )?;
    emit(&json!({"event":"complete","status":status}))
}
fn execute(job: &Job, dir: &Path) -> Result<Value> {
    fs::create_dir(dir).map_err(|e| e.to_string())?;
    write_new(&dir.join("job.json"), job)?;
    let err = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join("stderr.log"))
        .map_err(|e| e.to_string())?;
    let mut raw = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join("events.jsonl"))
        .map_err(|e| e.to_string())?;
    let mut cmd = Command::new(std::env::current_exe().map_err(|e| e.to_string())?);
    cmd.args(["leads-worker", "--job"])
        .arg(dir.join("job.json"))
        .env("RAYON_NUM_THREADS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::from(err));
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::process::CommandExt;
        let bytes = job.plan.memory_mib * 1024 * 1024;
        unsafe {
            cmd.pre_exec(move || {
                let limit = libc::rlimit {
                    rlim_cur: bytes as libc::rlim_t,
                    rlim_max: bytes as libc::rlim_t,
                };
                if libc::setrlimit(libc::RLIMIT_AS, &limit) < 0
                    || libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL) < 0
                {
                    return Err(std::io::Error::last_os_error());
                }
                if libc::getppid() == 1 {
                    return Err(std::io::Error::other("lead supervisor exited"));
                }
                Ok(())
            });
        }
    }
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    let out = child.stdout.take().ok_or("worker stdout missing")?;
    let (tx, rx) = mpsc::channel();
    let reader = std::thread::spawn(move || {
        for line in BufReader::new(out).lines() {
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    let began = Instant::now();
    let mut stop = None;
    let mut closed = false;
    loop {
        match rx.recv_timeout(Duration::from_millis(20)) {
            Ok(line) => {
                let line = line.map_err(|e| e.to_string())?;
                writeln!(raw, "{line}").map_err(|e| e.to_string())?;
                let v: Value = serde_json::from_str(&line).map_err(|e| e.to_string())?;
                if v["event"] == "solver" && v["kind"] == "matrix" {
                    eprintln!(
                        "Info:Lead:Solver:{}: branch={} D={} matrix={}x{} reduction_muls={}",
                        job.arm,
                        v["branch"],
                        v["degree"],
                        v["rows"],
                        v["columns"],
                        v["field_multiplications"]
                    );
                }
                if v["kind"] == "matrix"
                    && v["rows"]
                        .as_u64()
                        .unwrap_or(0)
                        .saturating_mul(v["columns"].as_u64().unwrap_or(0))
                        > job.plan.max_matrix_cells
                {
                    stop = Some("matrix-cap");
                    let _ = child.kill();
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => closed = true,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
        // Each branch has the same budget. A three-branch arm gets three
        // calls, and all are retained/charged; it does not get a free solve.
        let calls = if job.arm == "asymmetric-branches" {
            3
        } else {
            1
        };
        if began.elapsed() > Duration::from_millis(job.plan.hard_millis * calls) && stop.is_none() {
            stop = Some("hard-timeout");
            let _ = child.kill();
        }
        if closed && child.try_wait().map_err(|e| e.to_string())?.is_some() {
            break;
        }
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    reader.join().map_err(|_| "lead reader panicked")?;
    if let Some(reason) = stop {
        return Ok(
            json!({"status":"resource-limited","reason":reason,"exit_code":status.code(),"partial_counters":"events.jsonl; incomplete, not zero"}),
        );
    }
    if !status.success() {
        return Err(format!(
            "lead worker failed at {}; retained stderr and events",
            dir.display()
        ));
    }
    let value: Value = read(&dir.join("result.json"))?;
    Ok(value)
}

#[derive(Default)]
struct Rank {
    rows: BTreeMap<usize, Vec<u64>>,
}
fn pow(mut a: u64, mut e: u64, r: u64) -> u64 {
    let mut b = 1;
    while e != 0 {
        if e & 1 != 0 {
            b = b * a % r;
        }
        a = a * a % r;
        e >>= 1;
    }
    b
}
impl Rank {
    fn insert(&mut self, mut row: Vec<u64>, r: u64) -> usize {
        let before = self.rows.len();
        for (&p, b) in &self.rows {
            let a = row[p];
            for (x, y) in row.iter_mut().zip(b) {
                *x = (*x + r - a * y % r) % r;
            }
        }
        if let Some(p) = row.iter().position(|x| *x != 0) {
            let inv = pow(row[p], r - 2, r);
            for x in &mut row {
                *x = *x * inv % r;
            }
            self.rows.insert(p, row);
        }
        self.rows.len() - before
    }
}

fn files(dir: &Path, root: &Path, out: &mut BTreeMap<String, String>) -> Result<()> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let p = entry.map_err(|e| e.to_string())?.path();
        if p.is_dir() {
            files(&p, root, out)?;
        } else {
            let rel = p
                .strip_prefix(root)
                .map_err(|e| e.to_string())?
                .to_str()
                .ok_or("nonutf8 member")?
                .replace('\\', "/");
            if rel != "leads-manifest.json" {
                out.insert(rel, sha(&p)?);
            }
        }
    }
    Ok(())
}
pub fn verify(dir: &Path) -> Result<Value> {
    let manifest: Value = read(&dir.join("leads-manifest.json"))?;
    if manifest["schema"] != "factor_base_sweep.leads-seal/v1" {
        return Err("unknown lead seal schema".into());
    }
    let mut found = BTreeMap::new();
    files(dir, dir, &mut found)?;
    if serde_json::to_value(&found).map_err(|e| e.to_string())? != manifest["members"] {
        return Err("lead member/hash mismatch".into());
    }
    let result: Value = read(&dir.join("summary.json"))?;
    let mut witnesses = 0;
    for case in result["cases"].as_array().ok_or("missing lead cases")? {
        let i = koblitz_instance(
            case["a"].as_u64().ok_or("missing a")? as u8,
            case["n"].as_u64().ok_or("missing n")? as u32,
        )
        .ok_or("unsupported binary instance")?;
        if i.curve_id().slug != case["curve"].as_str().ok_or("missing curve")? {
            return Err("sealed curve identity mismatch".into());
        }
        for trial in case["trials"].as_array().ok_or("missing trials")? {
            let t: Pt =
                serde_json::from_value(trial["target"].clone()).map_err(|e| e.to_string())?;
            if t.infinity || !scale(&i, t, i.r).infinity {
                return Err("target outside nonidentity prime subgroup".into());
            }
            if !trial["original_witness"].is_null() {
                let w: [Pt; 2] = serde_json::from_value(trial["original_witness"].clone())
                    .map_err(|e| e.to_string())?;
                if add(&i, w[0], w[1]) != t {
                    return Err("bad lifted witness".into());
                }
                witnesses += 1;
            }
        }
    }
    Ok(
        json!({"schema":"factor_base_sweep.leads-verification/v1","members":found.len(),"point_witnesses":witnesses,"verification":"all hashes and returned original-curve point witnesses; no F4 rerun"}),
    )
}

pub fn run(plan_path: &Path, out: &Path) -> Result<Value> {
    let plan: LeadPlan = read(plan_path)?;
    plan.validate()?;
    let root = plan_path.parent().ok_or("plan parent missing")?;
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::create_dir(out).map_err(|e| format!("output must not exist: {e}"))?;
    write_new(&out.join("plan.json"), &plan)?;
    let mut input_hashes = BTreeMap::new();
    input_hashes.insert(plan_path.display().to_string(), sha(plan_path)?);
    let mut cases = Vec::new();
    for spec in &plan.cases {
        let p = root.join(&spec.left);
        input_hashes.insert(p.display().to_string(), sha(&p)?);
        let base: Value = read(&p)?;
        let n = base["curve"]["field_degree"]
            .as_u64()
            .ok_or("missing field degree")? as u32;
        if !(3..=13).contains(&n) {
            return Err(
                "lead comparison supports only bounded binary fields of degree 3..13".into(),
            );
        }
        let construction = base["curve"]["construction"]
            .as_str()
            .ok_or("missing construction")?;
        let a = if construction.starts_with("KoblitzCurve::new(0,") {
            0
        } else if construction.starts_with("KoblitzCurve::new(1,") {
            1
        } else {
            return Err("lead input must name an explicit Koblitz constructor".into());
        };
        let i = koblitz_instance(a, n).ok_or("unsupported binary instance")?;
        let slug = i.curve_id().slug;
        if base["curve"]["slug"] != slug
            || base["curve"]["r"] != i.r
            || base["curve"]["cofactor"] != i.cofactor
        {
            return Err("lead input model/subgroup metadata mismatch".into());
        }
        let (left, right, union, quotient, lift_checks) = if spec.kind == "charts" {
            let rpath = root.join(spec.right.as_ref().ok_or("missing right inventory")?);
            input_hashes.insert(rpath.display().to_string(), sha(&rpath)?);
            let b: Value = read(&rpath)?;
            if b["curve"] != base["curve"] {
                return Err("chart models differ".into());
            }
            let l = selected(&base, spec.left_take)?;
            let r = selected(&b, spec.right_take)?;
            let u = sorted(l.iter().chain(&r).copied());
            (l, r, u, Vec::new(), 0)
        } else {
            let mut c = Vec::new();
            for k in 1..=2 {
                let p = scale(&i, i.generator.into(), k);
                c.extend([p, neg(p)]);
            }
            let c = sorted(c);
            let t = Pt {
                x: 0,
                y: 1,
                infinity: false,
            };
            let mut b = Vec::new();
            let mut checks = 0;
            for q in &c {
                let f: Pt = i.fast.frobenius_k((*q).into(), 1).into();
                let p = scale(&i, f, i.r.div_ceil(2));
                for v in [p, add(&i, p, t)] {
                    if psi(&i, v) != *q {
                        return Err("torsion inverse/lift map failed".into());
                    }
                    b.push(v);
                    checks += 1;
                }
            }
            let b = sorted(b);
            (b.clone(), b.clone(), b, c, checks)
        };
        check_inventory(&i, &left)?;
        check_inventory(&i, &right)?;
        check_inventory(&i, &union)?;
        let supported = support(&i, &union);
        if supported.is_empty() {
            return Err("comparison has no nonidentity subgroup positive control".into());
        }
        let dir = out.join(&spec.id);
        fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let original_columns = sorted(union.iter().map(|p| (*p).min(neg(*p))));
        // Kernel translates have the same projected logarithm. Collapse
        // them before measuring rank, and drop zero projected columns.
        let columns = sorted(
            union
                .iter()
                .map(|p| scale(&i, *p, i.cofactor))
                .filter(|p| !p.infinity)
                .map(|p| p.min(neg(p))),
        );
        write_new(
            &dir.join("inventory.json"),
            &json!({"left":left,"right":right,"union":union,"quotient":quotient,"projected_columns":columns,"subgroup_order":i.r,"cofactor":i.cofactor,"lift_checks":lift_checks,"support":supported}),
        )?;
        let mut targets = Vec::new();
        for (j, t) in supported.iter().take(plan.positives as usize).enumerate() {
            targets.push(("positive-control", j as u32, *t, None));
        }
        let mut used = BTreeSet::new();
        for (phase, count) in [("train", plan.train), ("holdout", plan.holdout)] {
            for j in 0..count {
                // Distinct targets across train/holdout, rejection sampled by scalar.
                let mut k = 0;
                let scalar = loop {
                    let s = 1 + derive_u64(
                        &format!("factor-base-leads/{}/{phase}/v1", spec.id),
                        &[plan.seed, j.into(), k],
                    ) % (i.r - 1);
                    if used.insert(s) {
                        break s;
                    }
                    k += 1;
                    if used.len() as u64 >= i.r - 1 {
                        return Err("requested more distinct probes than subgroup targets".into());
                    }
                };
                targets.push((
                    phase,
                    j,
                    scale(&i, i.generator.into(), scalar),
                    Some(scalar),
                ));
            }
        }
        let arms = if spec.kind == "charts" {
            vec![
                "product-membership",
                "asymmetric-branches",
                "selector-membership",
            ]
        } else {
            vec!["direct-lift-base", "torsion-quotient"]
        };
        let mut ranks: BTreeMap<String, Rank> = BTreeMap::new();
        let mut trials = Vec::new();
        for (phase, j, target, scalar) in targets {
            for arm in arms
                .iter()
                .copied()
                .chain((phase == "positive-control").then_some("reference-repeat"))
            {
                let is_quotient = arm == "torsion-quotient";
                let (l, r) = if is_quotient {
                    (&quotient, &quotient)
                } else if arm == "asymmetric-branches" {
                    (&left, &right)
                } else {
                    (&union, &union)
                };
                let job = Job {
                    plan: plan.clone(),
                    a,
                    n,
                    slug: slug.clone(),
                    arm: arm.into(),
                    left: l.clone(),
                    right: r.clone(),
                    charts: (arm == "selector-membership")
                        .then(|| (algebra::xs(&left), algebra::xs(&right))),
                    target: if is_quotient { psi(&i, target) } else { target },
                };
                let path = dir.join(format!("{phase}-{j:03}-{arm}"));
                eprintln!("Info:Lead:{}:{phase}:{j}:{arm}: encoding and bounded F4; original target=({:x},{:x})",spec.id,target.x,target.y);
                let result = execute(&job, &path)?;
                let mut witness: Option<[Pt; 2]> = if result["witness"].is_null() {
                    None
                } else {
                    Some(
                        serde_json::from_value(result["witness"].clone())
                            .map_err(|e| e.to_string())?,
                    )
                };
                let mut lift_maps = 0u64;
                let mut rank_gain = 0;
                let mut row = None;
                if is_quotient {
                    if let Some(w) = witness {
                        let mut p = *union
                            .iter()
                            .find(|p| {
                                lift_maps += 1;
                                psi(&i, **p) == w[0]
                            })
                            .ok_or("no first quotient preimage")?;
                        let q = *union
                            .iter()
                            .find(|p| {
                                lift_maps += 1;
                                psi(&i, **p) == w[1]
                            })
                            .ok_or("no second quotient preimage")?;
                        if add(&i, p, q) != target {
                            p = add(
                                &i,
                                p,
                                Pt {
                                    x: 0,
                                    y: 1,
                                    infinity: false,
                                },
                            );
                        }
                        witness = Some([p, q]);
                    }
                }
                let expected = truth(&i, &union, &union, target);
                if result["status"] == "absent" && expected.is_some() {
                    return Err("pipeline falsely excludes original decomposition".into());
                }
                if let Some(w) = witness {
                    if !union.contains(&w[0])
                        || !union.contains(&w[1])
                        || add(&i, w[0], w[1]) != target
                    {
                        return Err("returned original-curve witness invalid".into());
                    }
                    if phase == "train" {
                        let mut v = vec![0; columns.len()];
                        for p in w {
                            let p = scale(&i, p, i.cofactor);
                            if p.infinity {
                                continue;
                            }
                            let canonical = p.min(neg(p));
                            let col = columns
                                .binary_search(&canonical)
                                .map_err(|_| "missing relation column")?;
                            v[col] = (v[col] + if p == canonical { 1 } else { i.r - 1 }) % i.r;
                        }
                        rank_gain = ranks.entry(arm.into()).or_default().insert(v.clone(), i.r);
                        row = Some(v);
                    }
                }
                eprintln!(
                    "Info:Lead:{}:{phase}:{j}:{arm}: status={} verified={} rank_gain={rank_gain}",
                    spec.id,
                    result["status"],
                    witness.is_some()
                );
                trials.push(json!({"phase":phase,"trial":j,"arm":arm,"target":target,"public_control_scalar":scalar,"ground_truth":expected,"original_witness":witness,"relation_row":row,"rank_gain":rank_gain,"lift_map_evaluations":lift_maps,"result":result,"job_directory":path.strip_prefix(out).map_err(|e|e.to_string())?}));
            }
        }
        cases.push(json!({"id":spec.id,"kind":spec.kind,"curve":slug,"ec1":base["curve"]["ec1"],"icv1":base["curve"]["icv1"],"a":a,"n":n,"r":i.r,"cofactor":i.cofactor,"generator":Pt::from(i.generator),"left_points":left.len(),"right_points":right.len(),"signed_points":union.len(),"columns":columns.len(),"original_columns":original_columns.len(),"column_semantics":"distinct nonzero signed [h] projections; kernel-equivalent columns collapsed","support":supported.len(),"quotient_points":quotient.len(),"lift_checks":lift_checks,"trials":trials}));
    }
    let summary = json!({"schema":"factor_base_sweep.leads-result/v1","cases":cases,"input_hashes":input_hashes,"binary_sha256":sha(&std::env::current_exe().map_err(|e|e.to_string())?)?,"threads":1,"certified_degree_of_regularity":null,"full_pipeline_speedup":null,"cost_status":"row-reduction counts and encoding term products only; symbolic, allocator, map and rank costs not completely priced; no total-cost winner","wall_time_status":"not measured for comparison; deadlines are resource limits"});
    write_new(&out.join("summary.json"), &summary)?;
    let mut members = BTreeMap::new();
    files(out, out, &mut members)?;
    write_new(
        &out.join("leads-manifest.json"),
        &json!({"schema":"factor_base_sweep.leads-seal/v1","members":members}),
    )?;
    verify(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn torsion_map_and_inverse_preserve_prime_subgroup_and_lift_point_relations() {
        for (a, n) in [(1, 7), (0, 13)] {
            let i = koblitz_instance(a, n).unwrap();
            let t = Pt {
                x: 0,
                y: 1,
                infinity: false,
            };
            for k in 1..20 {
                let q = scale(&i, i.generator.into(), k);
                let f = i.fast.frobenius_k(q.into(), 1);
                let p = scale(&i, f.into(), i.r.div_ceil(2));
                assert_eq!(psi(&i, p), q);
                assert_eq!(psi(&i, add(&i, p, t)), q);
                assert!(scale(&i, q, i.r).infinity);
                assert_eq!(psi(&i, t), FastPoint::INFINITY.into());
            }
        }
    }
    #[test]
    fn plan_rejects_duplicate_cases_and_unbounded_limits() {
        let p = LeadPlan {
            schema: "factor_base_sweep.leads/v1".into(),
            seed: 1,
            train: 2,
            holdout: 2,
            positives: 1,
            max_degree: 4,
            solve_millis: 10,
            hard_millis: 100,
            max_matrix_cells: 1000,
            memory_mib: 512,
            cases: vec![LeadCase {
                id: "test".into(),
                kind: "torsion".into(),
                left: "base.json".into(),
                right: None,
                left_take: 0,
                right_take: 0,
            }],
        };
        assert!(p.validate().is_ok());
        let mut q = p.clone();
        q.cases.push(p.cases[0].clone());
        assert!(q.validate().is_err());
        let mut q = p;
        q.solve_millis = 0;
        assert!(q.validate().is_err());
    }
    #[test]
    fn cofactor_projection_collapses_torsion_translates_before_rank() {
        let i = koblitz_instance(1, 7).unwrap();
        let p: Pt = i.generator.into();
        let t = Pt {
            x: 0,
            y: 1,
            infinity: false,
        };
        let q = add(&i, p, t);
        assert_ne!(p, q);
        assert_eq!(scale(&i, p, i.cofactor), scale(&i, q, i.cofactor));
        let mut rank = Rank::default();
        assert_eq!(rank.insert(vec![1, 0], i.r), 1);
        assert_eq!(rank.insert(vec![1, 0], i.r), 0);
    }
}
