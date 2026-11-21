use crate::{
    arithmetic::*,
    driver::{read, sha, write},
    engine::Task,
    Result,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    os::unix::process::{CommandExt, ExitStatusExt},
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
const VGROOT: &str = "/workspace/shared/twist-tools-20261025-e26/root/usr";
const S: &str = "research/twist_routes_1_3_20261021";
fn epoch() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}
fn digest(x: &[u8]) -> String {
    hex::encode(crate::source_sha256::sha256(x))
}
fn fingerprint(v: &Value) -> String {
    digest(&serde_json::to_vec(v).unwrap())
}
fn target(f: &Fixture, label: u64, heldout: bool) -> Point {
    if heldout {
        let d = crate::source_sha256::sha256(
            format!("twist-cold-heldout-20261029-v1|{}|{label}", f.id).as_bytes(),
        );
        let k = u64::from_be_bytes(d[..8].try_into().unwrap()) % (f.r - 1) + 1;
        f.curve().mul(f.g(), k)
    } else {
        let mut seed = 202610210000 + label + f.p * 65537;
        loop {
            let x = mix(&mut seed) % f.p;
            if let Some(y) = sqrt(f.curve().rhs(x), f.p) {
                if y != 0 {
                    return Some([x, if mix(&mut seed) & 1 == 0 { y } else { f.p - y }]);
                }
            }
        }
    }
}
fn log_for_manifest(f: &Fixture, q: Point) -> u64 {
    let e = f.curve();
    let size = isqrt(f.r) + 1;
    let mut baby = BTreeMap::new();
    let mut p = None;
    for i in 0..size {
        baby.insert(p, i);
        p = e.add(p, f.g());
    }
    let step = e.neg(e.mul(f.g(), size));
    let mut p = q;
    for j in 0..=size {
        if let Some(i) = baby.get(&p) {
            let k = (j * size + i) % f.r;
            if e.mul(f.g(), k) == q {
                return k;
            }
        }
        p = e.add(p, step);
    }
    panic!("known-answer workload manifest failed")
}
#[derive(Clone, Serialize, Deserialize)]
struct Cell {
    index: usize,
    task: Task,
    replicate: u64,
    cohort: String,
    expected_scalar: u64,
    aa_group: Option<String>,
    timeout_s: u64,
}
fn add(
    cells: &mut BTreeMap<String, Cell>,
    mut task: Task,
    rep: u64,
    cohort: &str,
    expected: u64,
    aa: Option<String>,
    timeout: u64,
) {
    let key = format!(
        "{cohort}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{rep}|{}",
        task.fixture.id,
        task.target_label,
        task.family,
        task.b,
        task.shift,
        task.alpha,
        task.evaluation,
        task.compression,
        task.method,
        aa.as_deref().unwrap_or("primary")
    );
    task.id = format!("{}-{}", cohort, &digest(key.as_bytes())[..20]);
    let c = Cell {
        index: 0,
        task,
        replicate: rep,
        cohort: cohort.into(),
        expected_scalar: expected,
        aa_group: aa,
        timeout_s: timeout,
    };
    cells.entry(key).or_insert(c);
}
fn template(f: &Fixture, label: u64, heldout: bool) -> Task {
    let q = target(f, label, heldout);
    let d = crate::source_sha256::sha256(format!("cold-rho-seed|{}|{label}", f.id).as_bytes());
    Task {
        id: String::new(),
        fixture: f.clone(),
        target: q,
        target_label: label,
        method: "group".into(),
        family: "low-x".into(),
        b: 8,
        shift: 1,
        alpha: "none".into(),
        evaluation: "map".into(),
        compression: false,
        queries: 16,
        seed: u64::from_be_bytes(d[..8].try_into().unwrap()),
    }
}
fn base_size(n: u64) -> usize {
    let mut b = 1usize;
    while (b as u64).pow(4) < n {
        b += 1;
    }
    ((2 * b + 7) / 8) * 8
}
fn plan(kind: &str, input: &Path, pins: &Path, out: &Path) -> Result<()> {
    let raw = read(input)?;
    let development: Vec<Fixture> = serde_json::from_value(raw["development"].clone())?;
    let heldout: Vec<Fixture> = serde_json::from_value(raw["heldout"].clone())?;
    let sizing: Vec<Fixture> = serde_json::from_value(raw["sizing_only"].clone())?;
    let mut cells = BTreeMap::new();
    let reps = if kind == "sizing" { 1 } else { 3 };
    if kind == "sizing" {
        let mut fixtures = sizing.clone();
        fixtures.extend(
            development
                .iter()
                .filter(|f| f.p == 59 || f.p == 827)
                .cloned(),
        );
        for f in fixtures {
            let mut t = template(&f, 0, false);
            let k = (f.r / 3 + 17) % (f.r - 1) + 1;
            t.target = f.curve().mul(f.g(), k);
            let b = if f.p < 1000 { 8 } else { base_size(f.r) };
            for family in ["low-x", "twist-max"] {
                for method in ["group", "poly", "gp-poly"] {
                    for compressed in [false, true] {
                        let mut x = t.clone();
                        x.family = family.into();
                        x.method = method.into();
                        x.compression = compressed;
                        x.b = b;
                        x.queries = if method == "group" { 8 * b } else { 16 };
                        add(&mut cells, x, 0, "sizing", k, None, 120);
                    }
                }
            }
            if f.family == "CM-degree17-candidate" || f.p == 827 {
                for family in ["low-x", "twist-max"] {
                    for ev in ["map", "scalar"] {
                        for comp in [false, true] {
                            let mut x = t.clone();
                            x.family = family.into();
                            x.alpha = "augment".into();
                            x.evaluation = ev.into();
                            x.b = b;
                            x.queries = 8 * b;
                            x.compression = comp;
                            add(&mut cells, x, 0, "sizing-alpha", k, None, 120);
                        }
                    }
                }
            }
            for method in ["rho", "rho-negation", "bsgs", "bsgs-negation", "gp-elllog"] {
                let mut x = t.clone();
                x.method = method.into();
                x.family = "reference".into();
                x.b = 0;
                x.queries = 0;
                add(&mut cells, x, 0, "sizing-reference", k, None, 120);
            }
        }
    } else if kind == "development" {
        for f in &development {
            for label in 0..6 {
                let t = template(f, label, false);
                let expected = log_for_manifest(f, t.target);
                for rep in 0..reps {
                    if label == 0 {
                        for b in [8, 16] {
                            for shift in [1, 3] {
                                for family in [
                                    "low-x",
                                    "curve-orbit",
                                    "uniform",
                                    "twist-first",
                                    "twist-max",
                                ] {
                                    for compressed in [false, true] {
                                        let mut x = t.clone();
                                        x.b = b;
                                        x.shift = shift;
                                        x.family = family.into();
                                        x.compression = compressed;
                                        add(
                                            &mut cells,
                                            x,
                                            rep,
                                            "development-cold",
                                            expected,
                                            None,
                                            120,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    for family in ["low-x", "twist-max"] {
                        for method in ["group", "poly", "gp-poly"] {
                            for comp in [false, true] {
                                let mut x = t.clone();
                                x.family = family.into();
                                x.method = method.into();
                                x.compression = comp;
                                add(&mut cells, x, rep, "regression-60", expected, None, 120);
                            }
                        }
                    }
                    if f.p == 827 && label == 0 {
                        for b in [8, 16] {
                            for shift in [1, 3] {
                                for family in [
                                    "low-x",
                                    "curve-orbit",
                                    "uniform",
                                    "twist-first",
                                    "twist-max",
                                ] {
                                    for alpha in ["links", "augment"] {
                                        for ev in ["map", "scalar"] {
                                            for comp in [false, true] {
                                                let mut x = t.clone();
                                                x.b = b;
                                                x.shift = shift;
                                                x.family = family.into();
                                                x.alpha = alpha.into();
                                                x.evaluation = ev.into();
                                                x.compression = comp;
                                                add(
                                                    &mut cells,
                                                    x,
                                                    rep,
                                                    "development-alpha",
                                                    expected,
                                                    None,
                                                    120,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        let mut x = t.clone();
                        x.family = "twist-max-scan".into();
                        add(
                            &mut cells,
                            x,
                            rep,
                            "archived-exhaustive-discovery",
                            expected,
                            None,
                            120,
                        );
                    }
                    for method in ["rho", "rho-negation", "bsgs", "bsgs-negation", "gp-elllog"] {
                        let mut x = t.clone();
                        x.method = method.into();
                        x.family = "reference".into();
                        x.b = 0;
                        x.queries = 0;
                        add(
                            &mut cells,
                            x,
                            rep,
                            "development-reference",
                            expected,
                            None,
                            120,
                        );
                    }
                }
            }
        }
    } else if kind == "heldout" {
        for f in &heldout {
            for label in 0..5 {
                let t = template(f, label, true);
                let expected = log_for_manifest(f, t.target);
                let b = base_size(f.r);
                for rep in 0..reps {
                    for family in ["low-x", "twist-max"] {
                        for alpha in ["none", "augment"] {
                            for comp in [false, true] {
                                let mut x = t.clone();
                                x.family = family.into();
                                x.b = b;
                                x.queries = 8 * b;
                                x.alpha = alpha.into();
                                x.compression = comp;
                                add(&mut cells, x, rep, "heldout", expected, None, 180);
                            }
                        }
                    }
                    if f.family == "CM-degree17-candidate" {
                        for family in ["low-x", "twist-max"] {
                            for comp in [false, true] {
                                let mut x = t.clone();
                                x.family = family.into();
                                x.alpha = "augment".into();
                                x.evaluation = "scalar".into();
                                x.b = b;
                                x.queries = 8 * b;
                                x.compression = comp;
                                add(
                                    &mut cells,
                                    x,
                                    rep,
                                    "heldout-scalar-control",
                                    expected,
                                    None,
                                    180,
                                );
                            }
                        }
                    }
                    for method in ["rho", "rho-negation", "bsgs", "bsgs-negation", "gp-elllog"] {
                        let mut x = t.clone();
                        x.method = method.into();
                        x.family = "reference".into();
                        x.b = 0;
                        x.queries = 0;
                        add(&mut cells, x, rep, "heldout-reference", expected, None, 180);
                    }
                }
            }
        }
    } else {
        return Err("plan kind sizing|development|heldout".into());
    }
    let aa_f = &development[0];
    let mut aa_t = template(aa_f, 0, false);
    aa_t.target = aa_f.curve().mul(aa_f.g(), 2);
    for method in ["group", "gp-poly"] {
        for n in 0..3 {
            let mut x = aa_t.clone();
            x.method = method.into();
            add(
                &mut cells,
                x,
                n,
                "AA",
                2,
                Some(format!("identical-cold-{method}")),
                120,
            );
        }
    }
    let mut cells: Vec<_> = cells.into_values().collect();
    cells.sort_by_key(|c| {
        let mut b = serde_json::to_vec(&c.task).unwrap();
        b.extend_from_slice(&c.replicate.to_be_bytes());
        (c.cohort != "AA", digest(&b))
    });
    for (i, c) in cells.iter_mut().enumerate() {
        c.index = i;
    }
    let mut counts = BTreeMap::<String, usize>::new();
    for c in &cells {
        *counts.entry(c.cohort.clone()).or_default() += 1;
    }
    write(
        out,
        &json!({"schema":"twist.cold-plan/v1","kind":kind,"source_pins":pins,"source_pins_sha256":sha(pins)?,"fixture_manifest":input,"fixture_manifest_sha256":sha(input)?,"planned_cells":cells.len(),"cohort_counts":counts,"cells":cells,"replicates":reps,"unit":"single-event user-space guest instructions Ir","cold_target_count":1,"memory_cap_bytes":4294967296u64,"disk_cap_bytes":1500000000u64,"global_budget_s":if kind=="sizing"{3600}else{21600},"no_cost_outcomes_used_to_choose_cases":true,"original_E28_final_labels_opened":false}),
    )
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Trace {
    path: String,
    sha256: String,
    bytes: u64,
    pid: u32,
    part: u64,
    trigger: String,
    ir: u64,
    events: String,
}
fn trace(path: &Path) -> Result<Trace> {
    let s = fs::read_to_string(path)?;
    let field = |prefix: &str| {
        s.lines()
            .find_map(|x| x.strip_prefix(prefix))
            .map(str::trim)
    };
    let events = field("events:").ok_or("trace events absent")?;
    if events != "Ir" {
        return Err(format!("not single-event Ir: {events}").into());
    }
    let ir = s
        .lines()
        .rev()
        .find_map(|x| x.strip_prefix("totals:"))
        .or_else(|| field("summary:"))
        .ok_or("trace Ir total missing")?
        .trim()
        .parse()?;
    Ok(Trace {
        path: path.file_name().unwrap().to_string_lossy().into(),
        sha256: sha(path)?,
        bytes: fs::metadata(path)?.len(),
        pid: field("pid:").ok_or("trace pid")?.parse()?,
        part: field("part:").ok_or("trace part")?.parse()?,
        trigger: field("desc: Trigger:").unwrap_or("").into(),
        ir,
        events: events.into(),
    })
}
fn traces(dir: &Path) -> Result<Vec<Trace>> {
    let mut v = vec![];
    for e in fs::read_dir(dir)? {
        let p = e?.path();
        if p.file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("callgrind.")
            && !p.to_string_lossy().ends_with(".xz")
        {
            v.push(trace(&p)?);
        }
    }
    v.sort_by_key(|t| (t.pid, t.part));
    Ok(v)
}
fn cost(result: &Value, traces: &[Trace]) -> Result<Value> {
    let native = result["native_pid"].as_u64().ok_or("native pid")? as u32;
    let gp = result["gp_pid"].as_u64().ok_or("GP pid")? as u32;
    let mut phases = vec![];
    let mut used = BTreeSet::new();
    for mark in result["result"]["markers"]
        .as_array()
        .ok_or("phase markers")?
    {
        let name = mark["phase"].as_str().unwrap();
        let xs: Vec<_> = traces
            .iter()
            .enumerate()
            .filter(|(_, t)| {
                t.pid == native && t.trigger.contains(&format!("Client Request: {name}"))
            })
            .collect();
        if xs.len() != 1 {
            return Err(format!("native phase {name}: {} traces", xs.len()).into());
        }
        let (index, t) = xs[0];
        if t.ir == 0 {
            return Err(format!("zero native ROI instructions for {name}").into());
        }
        used.insert(index);
        let mut ir = t.ir;
        let mut backend = 0;
        for part in
            mark["gp_calls_before"].as_u64().unwrap() + 1..=mark["gp_calls_after"].as_u64().unwrap()
        {
            let matches: Vec<_> = traces
                .iter()
                .enumerate()
                .filter(|(_, t)| {
                    t.pid == gp && t.part == part && t.trigger.contains("Client Request")
                })
                .collect();
            if matches.len() != 1 {
                return Err(format!("GP call part {part}: {} traces", matches.len()).into());
            }
            let (i, t) = matches[0];
            if t.ir == 0 {
                return Err(format!("zero backend ROI instructions for call {part}").into());
            }
            used.insert(i);
            backend += t.ir;
            ir += t.ir;
        }
        phases.push(json!({"phase":name,"native_ir":t.ir,"backend_ir":backend,"total_ir":ir}));
    }
    for (i, t) in traces.iter().enumerate() {
        if !used.contains(&i) && t.ir != 0 {
            return Err(format!(
                "unassigned measured instructions {} in {} trigger {}",
                t.ir, t.path, t.trigger
            )
            .into());
        }
    }
    let total: u64 = phases.iter().map(|p| p["total_ir"].as_u64().unwrap()).sum();
    let setup = phases.iter().find(|p| p["phase"] == "setup").unwrap()["total_ir"]
        .as_u64()
        .unwrap();
    Ok(
        json!({"unit":"Ir","total_ir":total,"setup_ir":setup,"online_ir":total-setup,"exclusive_phases":phases,"all_trace_instructions_assigned":true,"non_ROI_instructions":0}),
    )
}
fn compress_file(path: &Path) -> Result<Value> {
    let bytes = fs::read(path)?;
    let h = digest(&bytes);
    let xz = path.with_file_name(format!(
        "{}.xz",
        path.file_name().unwrap().to_string_lossy()
    ));
    if xz.exists() {
        return Err("fresh compressed file required".into());
    }
    let status = Command::new("xz")
        .args(["-6", "-T1", "-c"])
        .arg(path)
        .stdout(fs::File::create(&xz)?)
        .status()?;
    if !status.success() {
        return Err("xz compression failed".into());
    }
    let decoded = Command::new("xz").args(["-dc"]).arg(&xz).output()?;
    if !decoded.status.success() || digest(&decoded.stdout) != h {
        return Err("lossless compression verification failed".into());
    }
    let rec = json!({"path":xz.file_name().unwrap().to_string_lossy(),"sha256":sha(&xz)?,"bytes":fs::metadata(&xz)?.len(),"uncompressed_name":path.file_name().unwrap().to_string_lossy(),"uncompressed_sha256":h,"uncompressed_bytes":bytes.len(),"fresh_decompression_hash_verified":true});
    fs::remove_file(path)?;
    Ok(rec)
}
fn rss(pid: u32) -> u64 {
    let mut total = 0;
    let p = format!("/proc/{pid}/status");
    if let Ok(s) = fs::read_to_string(p) {
        if let Some(v) = s.lines().find_map(|l| l.strip_prefix("VmRSS:")) {
            total += v
                .split_whitespace()
                .next()
                .and_then(|x| x.parse::<u64>().ok())
                .unwrap_or(0)
                * 1024;
        }
    }
    if let Ok(s) = fs::read_to_string(format!("/proc/{pid}/task/{pid}/children")) {
        for c in s.split_whitespace().filter_map(|x| x.parse().ok()) {
            total += rss(c);
        }
    }
    total
}
fn bytes(dir: &Path) -> u64 {
    fs::read_dir(dir)
        .map(|it| {
            it.filter_map(std::result::Result::ok)
                .map(|e| {
                    let p = e.path();
                    if p.is_dir() {
                        bytes(&p)
                    } else {
                        e.metadata().map(|m| m.len()).unwrap_or(0)
                    }
                })
                .sum()
        })
        .unwrap_or(0)
}
fn guard(pins: &Value) -> Result<()> {
    for k in ["source_hashes", "input_hashes"] {
        for (p, h) in pins[k].as_object().ok_or("pin map")? {
            if sha(p)? != h.as_str().unwrap() {
                return Err(format!("frozen pin changed: {p}").into());
            }
        }
    }
    for k in ["binary", "gp", "roi_library"] {
        if sha(pins[k]["path"].as_str().unwrap())? != pins[k]["sha256"].as_str().unwrap() {
            return Err(format!("pinned {k} bytes changed").into());
        }
    }
    Ok(())
}
fn run(planfile: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh campaign directory required".into());
    }
    let plan = read(planfile)?;
    let pinsfile = Path::new(plan["source_pins"].as_str().unwrap());
    if sha(pinsfile)? != plan["source_pins_sha256"] {
        return Err("source pin record changed".into());
    }
    let pins = read(pinsfile)?;
    guard(&pins)?;
    if sha(plan["fixture_manifest"].as_str().unwrap())? != plan["fixture_manifest_sha256"] {
        return Err("input manifest changed".into());
    }
    fs::create_dir_all(out)?;
    let cells: Vec<Cell> = serde_json::from_value(plan["cells"].clone())?;
    let mut ledger = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("ROWS.jsonl"))?;
    let campaign = Instant::now();
    for c in &cells {
        if campaign.elapsed().as_secs() > plan["global_budget_s"].as_u64().unwrap()
            || bytes(out) > plan["disk_cap_bytes"].as_u64().unwrap()
        {
            writeln!(
                ledger,
                "{}",
                json!({"index":c.index,"task_id":c.task.id,"status":"not-started-global-cap","complete_cost":false})
            )?;
            ledger.flush()?;
            continue;
        }
        let dir = out.join(format!("cell-{:05}", c.index));
        fs::create_dir(&dir)?;
        write(dir.join("task.json"), &serde_json::to_value(&c.task)?)?;
        let result = dir.join("worker.json");
        let tracepath = dir.join("callgrind.%p");
        let log = dir.join("valgrind.%p.log");
        let started = epoch();
        let begin = Instant::now();
        let vg = format!("{VGROOT}/bin/valgrind");
        let argv = vec![
            "-c".to_string(),
            "0".into(),
            "/usr/bin/timeout".into(),
            "--signal=TERM".into(),
            "--kill-after=1s".into(),
            format!("{}s", c.timeout_s),
            vg,
            "--tool=callgrind".into(),
            "--instr-atstart=no".into(),
            "--trace-children=yes".into(),
            "--cache-sim=no".into(),
            "--branch-sim=no".into(),
            "--dump-line=no".into(),
            "--read-inline-info=no".into(),
            "--error-exitcode=99".into(),
            format!("--log-file={}", log.display()),
            format!("--callgrind-out-file={}", tracepath.display()),
            pins["binary"]["path"].as_str().unwrap().into(),
            "worker".into(),
            dir.join("task.json").display().to_string(),
            result.display().to_string(),
            pins["roi_library"]["path"].as_str().unwrap().into(),
        ];
        let mut child = Command::new("/usr/bin/taskset")
            .args(&argv)
            .env("VALGRIND_LIB", format!("{VGROOT}/libexec/valgrind"))
            .env("RAYON_NUM_THREADS", "1")
            .env("OMP_NUM_THREADS", "1")
            .env("OPENBLAS_NUM_THREADS", "1")
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(fs::File::create(dir.join("worker.stdout"))?)
            .stderr(fs::File::create(dir.join("worker.stderr"))?)
            .spawn()?;
        let (mut peak, mut memory_fired) = (0, false);
        let status = loop {
            peak = peak.max(rss(child.id()));
            if peak > 4294967296 {
                memory_fired = true;
                Command::new("/bin/kill")
                    .args(["-KILL", "--", &format!("-{}", child.id())])
                    .status()?;
            }
            if let Some(s) = child.try_wait()? {
                break s;
            }
            std::thread::sleep(Duration::from_millis(25));
        };
        let ts = traces(&dir)?;
        let mut instrument_error = None;
        let mut row = json!({"index":c.index,"task_id":c.task.id,"replicate":c.replicate,"cohort":c.cohort,"aa_group":c.aa_group,"started_epoch_s":started,"finished_epoch_s":epoch(),"exit_code":status.code(),"signal":status.signal(),"max_sampled_tree_rss_bytes":peak,"memory_limit_fired":memory_fired,"elapsed_s_for_resource_sizing_only":begin.elapsed().as_secs_f64(),"wall_time_admitted":false,"argv":["/usr/bin/taskset"],"worker_sha256":pins["binary"]["sha256"],"task_sha256":sha(dir.join("task.json"))?,"complete_cost":false,"complete_attempt_cost":false});
        row["argv"] = json!(std::iter::once("/usr/bin/taskset".to_string())
            .chain(argv)
            .collect::<Vec<_>>());
        if status.success() && !memory_fired && result.exists() {
            let data = read(&result)?;
            if data["result"]["measured_under_valgrind"] != true {
                return Err("worker not measured under Valgrind".into());
            }
            let gperr = fs::read_to_string(dir.join("gp.stderr"))?;
            if !gperr.trim().is_empty() {
                return Err(format!("GP stderr rejected: {gperr}").into());
            }
            row["status"] = data["result"]["status"].clone();
            match cost(&data, &ts) {
                Ok(cost) => {
                    row["cost"] = cost;
                    row["complete_attempt_cost"] = json!(true);
                }
                Err(err) => {
                    instrument_error = Some(err.to_string());
                    row["status"] = json!("counter-admission-failed");
                    row["counter_error"] = json!(instrument_error);
                }
            }
            row["scalar_verified"] = json!(false);
            if data["result"]["status"] == "recovered" {
                let k = data["result"]["recovered_scalar"]
                    .as_u64()
                    .ok_or("recovered scalar absent")?;
                if k != c.expected_scalar
                    || c.task.fixture.curve().mul(c.task.fixture.g(), k) != c.task.target
                {
                    return Err("parent original-point/expected-scalar check failed".into());
                }
                row["scalar_verified"] = json!(true);
                row["complete_cost"] = json!(instrument_error.is_none());
            }
            if c.cohort == "AA" && data["result"]["status"] != "recovered" {
                instrument_error = Some("positive Q=2G control did not recover".into());
                row["status"] = json!("positive-control-failed");
            }
            for k in [
                "base",
                "known_rows",
                "query_targets",
                "witnesses",
                "query_rows",
            ] {
                if !data["result"][k].is_null() {
                    row[format!("{k}_sha256")] = json!(fingerprint(&data["result"][k]));
                }
            }
            row["rank"] = data["result"]["fiber"]["rank"].clone();
            row["known_rank"] = data["result"]["known_rank"].clone();
            row["actual_B"] = json!(data["result"]["base"].as_array().map_or(0, Vec::len));
        } else {
            row["status"] = json!(if memory_fired {
                "memory-cap"
            } else if status.code() == Some(124) {
                "timeout"
            } else {
                "worker-failure"
            });
            row["available_partial_trace_ir"] = json!(ts.iter().map(|t| t.ir).sum::<u64>());
        }
        let mut compressed = vec![];
        for t in &ts {
            compressed.push(compress_file(&dir.join(&t.path))?);
        }
        for name in ["worker.json", "gp-transcript.txt"] {
            let p = dir.join(name);
            if p.exists() {
                compressed.push(compress_file(&p)?);
            }
        }
        write(
            dir.join("PROFILE_INDEX.json"),
            &json!({"schema":"twist.cold-profile-index/v1","traces":ts,"compressed_files":compressed}),
        )?;
        write(dir.join("RECEIPT.json"), &row)?;
        writeln!(ledger, "{}", row)?;
        ledger.flush()?;
        if let Some(err) = instrument_error {
            write(
                out.join("INSTRUMENT_FAILURE.json"),
                &json!({"index":c.index,"error":err,"remaining_cells_unattempted":cells.len()-c.index-1}),
            )?;
            return Err(
                "counter/positive control failed; raw attempt retained, no silent retry".into(),
            );
        }
        if c.index % 10 == 0 {
            println!(
                "completed cell {} / {} ({})",
                c.index + 1,
                cells.len(),
                row["status"]
            );
            std::io::stdout().flush()?;
        }
    }
    write(
        out.join("COMPLETE.json"),
        &json!({"schema":"twist.cold-campaign-complete/v1","plan_sha256":sha(planfile)?,"planned_cells":cells.len(),"rows_sha256":sha(out.join("ROWS.jsonl"))?,"no_clock_claim":true}),
    )
}
pub fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("plan") if a.len() == 5 => {
            plan(&a[1], Path::new(&a[2]), Path::new(&a[3]), Path::new(&a[4]))
        }
        Some("run") if a.len() == 3 => run(Path::new(&a[1]), Path::new(&a[2])),
        Some("audit") if a.len() == 4 => {
            crate::audit::audit(Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]))
        }
        _ => Err("plan KIND FIXTURES SOURCE_PINS FRESH_PLAN | run PLAN FRESH_OUT".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn t(pid: u32, part: u64, name: &str, ir: u64) -> Trace {
        Trace {
            path: format!("callgrind.{pid}.{part}"),
            sha256: "synthetic".into(),
            bytes: 1,
            pid,
            part,
            trigger: name.into(),
            ir,
            events: "Ir".into(),
        }
    }
    #[test]
    fn backend_cost_is_charged_once_and_every_instruction_assigned() {
        let result = json!({"native_pid":1,"gp_pid":2,"result":{"markers":[{"phase":"setup","gp_calls_before":0,"gp_calls_after":1},{"phase":"linear-algebra","gp_calls_before":1,"gp_calls_after":1}]}});
        let mut traces = vec![
            t(1, 1, "Client Request: setup", 10),
            t(1, 2, "Client Request: linear-algebra", 3),
            t(2, 1, "Client Request", 7),
            t(1, 3, "Program termination", 0),
            t(2, 2, "Program termination", 0),
        ];
        let c = cost(&result, &traces).unwrap();
        assert_eq!(c["total_ir"], 20);
        assert_eq!(c["setup_ir"], 17);
        assert_eq!(c["online_ir"], 3);
        traces.push(t(2, 3, "unassigned", 1));
        assert!(cost(&result, &traces).is_err());
    }
    #[test]
    fn missing_and_zero_backend_measurements_are_rejected() {
        let r = json!({"native_pid":1,"gp_pid":2,"result":{"markers":[{"phase":"setup","gp_calls_before":0,"gp_calls_after":1}]}});
        assert!(cost(&r, &[t(1, 1, "Client Request: setup", 10)]).is_err());
        assert!(cost(
            &r,
            &[
                t(1, 1, "Client Request: setup", 10),
                t(2, 1, "Client Request", 0)
            ]
        )
        .is_err());
    }
}
