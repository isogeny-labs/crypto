//! Frozen progression plan, bounded Ir transport and independent finite audit.
//! The audit does not import the producer's field, curve, base or solver code.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    os::unix::process::ExitStatusExt,
    path::Path,
    process::{Command, Stdio},
    time::{Instant, SystemTime, UNIX_EPOCH},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Point = Option<(u64, u64)>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn write(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    Ok(fs::write(p, serde_json::to_string_pretty(v)? + "\n")?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
fn count_ir(s: &str) -> Option<u64> {
    if !s
        .lines()
        .any(|s| s.split_whitespace().collect::<Vec<_>>() == ["events:", "Ir"])
    {
        return None;
    }
    s.lines()
        .rev()
        .find_map(|s| s.strip_prefix("totals:"))
        .or_else(|| s.lines().rev().find_map(|s| s.strip_prefix("summary:")))?
        .trim()
        .parse()
        .ok()
}
fn bytes(p: &Path) -> u64 {
    fs::read_dir(p)
        .unwrap()
        .map(|e| {
            let p = e.unwrap().path();
            if p.is_dir() {
                bytes(&p)
            } else {
                fs::metadata(p).unwrap().len()
            }
        })
        .sum()
}
fn plan(prior: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh plan required".into());
    }
    let old = read(prior)?;
    let mut inputs = BTreeMap::new();
    for cell in old["cells"].as_array().ok_or("prior cells")? {
        let t = &cell["task"];
        if t["arity"] == 2 {
            inputs.insert(t["fixture"]["id"].as_str().unwrap().to_owned(), t.clone());
        }
    }
    assert_eq!(inputs.len(), 10);
    let mut cells = vec![];
    for (fixture, t) in &inputs {
        for family in ["twist-orbit", "curve-orbit", "field-ap", "uniform"] {
            for shift in [1, 3] {
                for m in [2, 3] {
                    let workload = format!("{fixture}-{family}-c{shift}-b8-m{m}-s0");
                    let mut arms = vec![
                        ("direct-aa", "direct-chain"),
                        ("direct-chain", "direct-chain"),
                        ("hybrid-eight", "hybrid-eight"),
                        ("group-aa", "group-mitm"),
                        ("group-mitm", "group-mitm"),
                    ];
                    if family.ends_with("orbit") {
                        arms.push(("companion-curve", "companion-curve"));
                    }
                    for (arm, algorithm) in arms {
                        cells.push(json!({"index":cells.len(),"workload":workload,"fixture":fixture,"family":family,"shift":shift,"arity":m,"arm":arm,
                    "task":{"workload":workload,"fixture":t["fixture"],"target":t["target"],"family":family,"shift":shift,"B":8,"arity":m,"arm":algorithm,"linear_roots":true}}));
                    }
                }
            }
        }
    }
    assert_eq!(cells.len(), 880);
    write(
        out,
        &json!({"schema":"twist.progression-plan/v1","prior_plan":prior,"prior_plan_sha256":sha(prior)?,
        "requested":cells.len(),"cells":cells,"solver_budget_ms":2000,"hard_timeout_s":4,"global_budget_s":2400,
        "memory_limit_KiB":4194304,"file_limit_bytes":33554432,"raw_limit_bytes":1073741824,"target_label":0,"final_E28_labels_opened":false}),
    )
}
fn run(plan_file: &Path, worker: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh run required".into());
    }
    fs::create_dir_all(out)?;
    let plan = read(plan_file)?;
    let started = Instant::now();
    let began = epoch();
    let mut records = vec![];
    let vgroot = "/workspace/shared/twist-tools-20261025-e26/root/usr";
    for cell in plan["cells"].as_array().unwrap() {
        let mut row = cell.clone();
        if started.elapsed().as_secs() >= plan["global_budget_s"].as_u64().unwrap()
            || bytes(out) >= 1073741824
        {
            row["status"] = json!("not-started-global-cap");
            records.push(row);
            continue;
        }
        let dir = out.join(format!("cell-{:03}", cell["index"].as_u64().unwrap()));
        fs::create_dir(&dir)?;
        let input = dir.join("task.json");
        let output = dir.join("worker.json");
        let trace = dir.join("worker.callgrind");
        write(&input, &cell["task"])?;
        let args = vec![
            "followup".to_owned(),
            "progression-worker".into(),
            input.display().to_string(),
            output.display().to_string(),
            "2000".into(),
        ];
        let launch = epoch();
        let mut command = Command::new("/bin/sh");
        command
            .args([
                "-c",
                "ulimit -c 0; ulimit -v 4194304; ulimit -f 65536; exec \"$@\"",
                "progression-cost",
                "/usr/bin/timeout",
                "--signal=TERM",
                "--kill-after=1s",
                "4s",
            ])
            .arg(format!("{vgroot}/bin/valgrind"))
            .args([
                "--tool=callgrind",
                "--cache-sim=no",
                "--branch-sim=no",
                "--error-exitcode=99",
            ])
            .arg(format!("--callgrind-out-file={}", trace.display()))
            .arg(worker)
            .args(&args)
            .env("VALGRIND_LIB", format!("{vgroot}/libexec/valgrind"))
            .env("F4_FP_GM", "1")
            .env("F4_FP_LINEAR_ROOTS", "1")
            .env("RAYON_NUM_THREADS", "1")
            .env("OMP_NUM_THREADS", "1")
            .env("OPENBLAS_NUM_THREADS", "1")
            .stdout(Stdio::from(fs::File::create(dir.join("worker.stdout"))?))
            .stderr(Stdio::from(fs::File::create(dir.join("worker.stderr"))?));
        let full_argv = std::iter::once(command.get_program())
            .chain(command.get_args())
            .map(|s| s.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        let status = command.status()?;
        let result = read(&output).ok();
        let ir = fs::read_to_string(&trace).ok().and_then(|s| count_ir(&s));
        row["status"] = json!(if status.success()
            && result
                .as_ref()
                .is_some_and(|r| r["status"] == "unsupported-underfilled-base")
        {
            "unsupported-underfilled-base"
        } else if status.success()
            && ir.is_some()
            && result
                .as_ref()
                .is_some_and(|r| r["complete_projection"] == true)
        {
            "complete-awaiting-independent-audit"
        } else {
            "incomplete"
        });
        row["exit_code"] = json!(status.code());
        row["signal"] = json!(status.signal());
        row["started_epoch_s"] = json!(launch);
        row["finished_epoch_s"] = json!(epoch());
        row["worker_argv"] = json!(std::iter::once(worker.display().to_string())
            .chain(args)
            .collect::<Vec<_>>());
        row["full_argv"] = json!(full_argv);
        row["environment_overrides"] = json!({"VALGRIND_LIB":format!("{vgroot}/libexec/valgrind"),"F4_FP_GM":"1","F4_FP_LINEAR_ROOTS":"1","RAYON_NUM_THREADS":"1","OMP_NUM_THREADS":"1","OPENBLAS_NUM_THREADS":"1"});
        row["input_sha256"] = json!(sha(&input)?);
        row["worker_sha256"] = json!(sha(worker)?);
        row["output_sha256"] = json!(if output.exists() {
            Some(sha(&output)?)
        } else {
            None
        });
        row["trace_sha256"] = json!(if trace.exists() {
            Some(sha(&trace)?)
        } else {
            None
        });
        row["Ir"] = json!(ir);
        row["stdout_sha256"] = json!(sha(dir.join("worker.stdout"))?);
        row["stderr_sha256"] = json!(sha(dir.join("worker.stderr"))?);
        row["directory"] = json!(dir);
        records.push(row);
        write(out.join("RECORDS.json"), &json!(records))?;
        if records.len() % 20 == 0 {
            println!(
                "{{\"progression_cells_recorded\":{},\"requested\":880}}",
                records.len()
            );
        }
    }
    write(out.join("RECORDS.json"), &json!(records))?;
    write(
        out.join("TRANSPORT.json"),
        &json!({"schema":"twist.progression-transport/v1","started_epoch_s":began,"finished_epoch_s":epoch(),
        "plan_sha256":sha(plan_file)?,"worker_sha256":sha(worker)?,"valgrind_sha256":sha(format!("{vgroot}/bin/valgrind"))?,
        "raw_bytes":bytes(out),"requested":880,"recorded":records.len(),"wall_time_admitted":false,"unit":"simulated guest instructions Ir"}),
    )
}

// Independent small-field implementation: no producer arithmetic is imported.
fn inverse(x: u64, p: u64) -> u64 {
    assert_ne!(x, 0);
    let (mut a, mut b) = (p as i128, x as i128);
    let (mut u, mut v) = (0i128, 1i128);
    while b != 0 {
        let q = a / b;
        (a, b) = (b, a - q * b);
        (u, v) = (v, u - q * v);
    }
    assert_eq!(a, 1);
    u.rem_euclid(p as i128) as u64
}
fn rhs(x: u64, a: u64, b: u64, p: u64) -> u64 {
    (x * x % p * x + a * x + b) % p
}
fn neg(x: Point, p: u64) -> Point {
    x.map(|(x, y)| (x, (p - y) % p))
}
fn add(left: Point, right: Point, a: u64, p: u64) -> Point {
    let (Some((x, y)), Some((u, v))) = (left, right) else {
        return left.or(right);
    };
    if x == u && (y + v) % p == 0 {
        return None;
    }
    let slope = if x == u {
        (3 * x % p * x + a) % p * inverse(2 * y % p, p) % p
    } else {
        (v + p - y) % p * inverse((u + p - x) % p, p) % p
    };
    let z = (slope * slope % p + 2 * p - x - u) % p;
    Some((z, (slope * ((x + p - z) % p) % p + p - y) % p))
}
fn point(v: &Value) -> Result<Point> {
    if v.is_null() {
        return Ok(None);
    }
    let p: Vec<Vec<u64>> = serde_json::from_value(v.clone())?;
    if p.len() != 2 || p.iter().any(|a| a.len() != 2 || a[1] != 0) {
        return Err("nonrational point".into());
    }
    Ok(Some((p[0][0], p[1][0])))
}
fn smix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
fn roots(p: u64) -> Vec<Option<u64>> {
    let mut roots = vec![None; p as usize];
    for y in (0..p).rev() {
        roots[(y * y % p) as usize] = Some(y);
    }
    roots
}
fn independent_base(f: &Value, family: &str, c: u64, b: usize) -> Result<(Vec<u64>, Value)> {
    let p = f["p"].as_u64().unwrap();
    let a = f["a"].as_u64().unwrap();
    let b0 = f["b"].as_u64().unwrap();
    let d = f["d"].as_u64().unwrap();
    let sq = roots(p);
    let twist = family == "twist-orbit";
    let orbit = family.ends_with("orbit");
    let aa = if twist { a * d % p * d % p } else { a };
    let bb = if twist {
        b0 * d % p * d % p * d % p
    } else {
        b0
    };
    let mut seed_trials = 0;
    let g = if twist {
        (0..p).find_map(|x| {
            seed_trials += 1;
            sq[rhs(x, aa, bb, p) as usize]
                .filter(|y| *y != 0)
                .map(|y| (x, y))
        })
    } else if orbit {
        Some((
            f["generator"][0].as_u64().unwrap(),
            f["generator"][1].as_u64().unwrap(),
        ))
    } else {
        None
    };
    let n = f["order"].as_u64().unwrap();
    let cap = if orbit {
        if twist {
            2 * (p + 1) - n
        } else {
            n
        }
    } else if family == "field-ap" {
        p
    } else {
        8 * p
    };
    let di = if twist { inverse(d, p) } else { 1 };
    let mut state = 202610260001u64 ^ p ^ (b0 << 16) ^ c;
    let mut q = g;
    let mut xs = BTreeSet::new();
    let mut attempts = vec![];
    let mut reason = "candidate-cap";
    for i in 0..cap {
        if orbit && q.is_none() {
            reason = "orbit-identity";
            break;
        }
        let raw = if orbit {
            q.unwrap().0
        } else if family == "field-ap" {
            i
        } else {
            smix(&mut state) % p
        };
        let u = raw * di % p;
        let x = (u + c) % p;
        let lift = sq[rhs(x, a, b0, p) as usize].is_some_and(|y| y != 0);
        let fresh = lift && xs.insert(x);
        attempts.push(
            json!({"term":i+1,"raw_x":raw,"u":u,"x":x,"source_lift":lift,"new_coordinate":fresh}),
        );
        if xs.len() == b {
            reason = "filled";
            break;
        }
        if orbit {
            q = add(q, g, aa, p);
        }
    }
    let gjson = g
        .map(|(x, y)| json!([[x, 0], [y, 0]]))
        .unwrap_or(Value::Null);
    let xs = xs.into_iter().collect::<Vec<_>>();
    Ok((
        xs.clone(),
        json!({"base":xs,"filled":xs.len()==b,"attempts":attempts,"stop_reason":reason,"orbit_generator":gjson,
        "seed_trials":seed_trials,"candidate_cap":cap,"inverse_d":di,"orbit_model":{"p":p,"a":aa,"b":bb}}),
    ))
}
fn check_base(production: &Value, independent: &Value) -> Result<()> {
    for key in [
        "base",
        "filled",
        "attempts",
        "stop_reason",
        "orbit_generator",
        "seed_trials",
        "candidate_cap",
        "inverse_d",
        "orbit_model",
    ] {
        if production[key] != independent[key] {
            return Err(format!("independent base mismatch: {key}").into());
        }
    }
    Ok(())
}
fn all_rows(
    f: &Value,
    xs: &[u64],
    arity: usize,
    targets: &[Point],
) -> Vec<BTreeMap<Vec<u64>, u64>> {
    let p = f["p"].as_u64().unwrap();
    let a = f["a"].as_u64().unwrap();
    let b = f["b"].as_u64().unwrap();
    let sq = roots(p);
    let points = xs
        .iter()
        .map(|x| Some((*x, sq[rhs(*x, a, b, p) as usize].unwrap())))
        .collect::<Vec<_>>();
    let mut rows = vec![BTreeMap::<Vec<u64>, u64>::new(); targets.len()];
    let indices =
        targets
            .iter()
            .enumerate()
            .fold(BTreeMap::<Point, Vec<usize>>::new(), |mut map, (i, t)| {
                map.entry(*t).or_default().push(i);
                map
            });
    for i in 0..xs.len() {
        for j in i..xs.len() {
            for k in 0..if arity == 3 { xs.len() } else { 1 } {
                let tuple = if arity == 2 {
                    vec![xs[i], xs[j]]
                } else {
                    vec![xs[i], xs[j], xs[k]]
                };
                let selected = if arity == 2 {
                    vec![points[i], points[j]]
                } else {
                    vec![points[i], points[j], points[k]]
                };
                for mask in 0..(1 << arity) {
                    let sum = selected.iter().enumerate().fold(None, |s, (j, t)| {
                        add(s, if mask >> j & 1 == 0 { *t } else { neg(*t, p) }, a, p)
                    });
                    if let Some(labels) = indices.get(&sum) {
                        for label in labels {
                            rows[*label]
                                .entry(tuple.clone())
                                .and_modify(|old| *old = (*old).min(mask))
                                .or_insert(mask);
                        }
                    }
                }
            }
        }
    }
    rows
}
fn witnesses(
    rows: &BTreeMap<Vec<u64>, u64>,
    production_y: &BTreeMap<u64, u64>,
    f: &Value,
    target: Point,
) -> Result<Value> {
    let p = f["p"].as_u64().unwrap();
    let a = f["a"].as_u64().unwrap();
    let mut out = vec![];
    // Re-select the minimum sign mask in the producer's declared ordinate convention.
    for tuple in rows.keys() {
        let ys = tuple.iter().map(|x| production_y[x]).collect::<Vec<_>>();
        let mask = (0..1 << tuple.len())
            .find(|mask| {
                tuple
                    .iter()
                    .zip(&ys)
                    .enumerate()
                    .fold(None, |s, (j, (x, y))| {
                        add(
                            s,
                            Some((*x, if mask >> j & 1 == 0 { *y } else { (p - y) % p })),
                            a,
                            p,
                        )
                    })
                    == target
            })
            .ok_or("no signed witness")?;
        out.push(json!({"x":tuple,"y":ys,"sign_mask":mask}));
    }
    Ok(json!(out))
}
fn distribution(mut values: Vec<f64>) -> Value {
    values.sort_by(f64::total_cmp);
    let n = values.len();
    json!({"pairs":n,"fewer":values.iter().filter(|x|**x<1.0).count(),
        "min":values.first(),"median":if n==0{None}else{Some((values[(n-1)/2]+values[n/2])/2.0)},"max":values.last()})
}
fn audit(plan_file: &Path, run: &Path, census: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh audit required".into());
    }
    fs::create_dir_all(out)?;
    let plan = read(plan_file)?;
    let transport = read(run.join("TRANSPORT.json"))?;
    let raw = read(run.join("RECORDS.json"))?;
    if transport["plan_sha256"] != sha(plan_file)? || raw.as_array().unwrap().len() != 880 {
        return Err("transport/plan mismatch".into());
    }
    let geometry = read(census.join("ROWS.json"))?;
    let mut geo_checks = vec![];
    let mut yield_rows = vec![];
    let mut gp = String::from("default(parisizemax,4000000000);good=0;bad=0;\n");
    let mut positive = 0usize;
    let mut exact = BTreeMap::<String, (Vec<u64>, BTreeMap<Vec<u64>, u64>)>::new();
    for row in geometry.as_array().unwrap() {
        let f = &row["fixture"];
        let c = &row["construction"];
        let family = c["family"].as_str().unwrap();
        let shift = c["shift"].as_u64().unwrap();
        let size = c["B"].as_u64().unwrap() as usize;
        let (xs, ib) = independent_base(f, family, shift, size)?;
        check_base(c, &ib)?;
        let p = f["p"].as_u64().unwrap();
        let a = f["a"].as_u64().unwrap();
        let b = f["b"].as_u64().unwrap();
        let sq = roots(p);
        let stored = census
            .join("bases")
            .join(format!("{}.json", row["id"].as_str().unwrap()));
        if read(&stored)? != *row {
            return Err("geometry record copy differs".into());
        }
        for arity in [2, 3] {
            let queries = row["queries"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|q| q["arity"] == arity)
                .collect::<Vec<_>>();
            if queries.len() != 16 {
                return Err("geometry query count".into());
            }
            let targets = queries
                .iter()
                .map(|q| point(&q["target"]))
                .collect::<Result<Vec<_>>>()?;
            for (label, q) in queries.iter().enumerate() {
                if q["label"] != label {
                    return Err("geometry label order".into());
                }
                let mut state = 202610210000u64 + label as u64 + p * 65537;
                let expected_x = loop {
                    let x = smix(&mut state) % p;
                    if sq[rhs(x, a, b, p) as usize].is_some_and(|y| y != 0) {
                        break x;
                    }
                };
                if targets[label].map(|t| t.0) != Some(expected_x) {
                    return Err("target x generator differs".into());
                }
                if label == 0 {
                    let old = plan["cells"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|c| c["fixture"] == f["id"])
                        .ok_or("fixture not planned")?;
                    if q["target"] != old["task"]["target"] {
                        return Err("target zero differs from frozen prior".into());
                    }
                }
            }
            if targets.iter().any(|t| {
                t.is_none_or(|(x, y)| x >= p || y >= p || y == 0 || y * y % p != rhs(x, a, b, p))
            }) {
                return Err("invalid geometry target".into());
            }
            let rows = if ib["filled"] == true {
                all_rows(f, &xs, arity, &targets)
            } else {
                vec![BTreeMap::new(); 16]
            };
            for (i, q) in queries.iter().enumerate() {
                if ib["filled"] == true && q["canonical_relation_tuples"] != rows[i].len() {
                    return Err("geometry yield mismatch".into());
                }
                if ib["filled"] != true && !q["canonical_relation_tuples"].is_null() {
                    return Err("underfilled query masquerades as empty".into());
                }
                yield_rows.push(json!({"id":row["id"],"fixture":f["id"],"family":family,"shift":shift,"B":size,"arity":arity,"label":i,
                    "target":q["target"],"filled":ib["filled"],"relations":if ib["filled"]==true{Some(rows[i].len())}else{None}}));
            }
            if size == 8 && [1, 3].contains(&shift) {
                let workload = format!(
                    "{}-{family}-c{shift}-b8-m{arity}-s0",
                    f["id"].as_str().unwrap()
                );
                exact.insert(workload, (xs.clone(), rows[0].clone()));
            }
        }
        let mut companion_valid = true;
        if family.ends_with("orbit") {
            let delta = if family == "twist-orbit" {
                f["d"].as_u64().unwrap()
            } else {
                1
            };
            companion_valid = xs.iter().all(|x| {
                sq[(rhs((x + p - shift) % p, a, b, p) * inverse(delta, p) % p) as usize].is_some()
            });
            if !companion_valid {
                return Err("companion finite validity".into());
            }
        }
        geo_checks.push(json!({"id":row["id"],"independent_base_equal":true,"all_32_query_counts_equal":true,"companion_valid":companion_valid,
            "filled":ib["filled"],"row_sha256":sha(census.join("bases").join(format!("{}.json",row["id"].as_str().unwrap())))?}));
    }
    if geo_checks.len() != 240 || yield_rows.len() != 7680 {
        return Err("geometry coverage".into());
    }
    let mut accepted = BTreeMap::<(String, String), u64>::new();
    let mut checks = vec![];
    for (cell, row) in plan["cells"]
        .as_array()
        .unwrap()
        .iter()
        .zip(raw.as_array().unwrap())
    {
        if cell["index"] != row["index"] || cell["task"] != row["task"] {
            return Err("plan/record mismatch".into());
        }
        let index = cell["index"].as_u64().unwrap();
        let dir = run.join(format!("cell-{index:03}"));
        let workload = cell["workload"].as_str().unwrap();
        let arm = cell["arm"].as_str().unwrap();
        if row["status"] == "not-started-global-cap" {
            checks.push(json!({"index":index,"accepted":false,"reason":"not-started-global-cap"}));
            continue;
        }
        let task = read(dir.join("task.json"))?;
        if task != cell["task"] || row["input_sha256"] != sha(dir.join("task.json"))? {
            return Err("task changed".into());
        }
        if row["worker_sha256"] != transport["worker_sha256"]
            || row["stdout_sha256"] != sha(dir.join("worker.stdout"))?
            || row["stderr_sha256"] != sha(dir.join("worker.stderr"))?
        {
            return Err("worker/log provenance mismatch".into());
        }
        let output_path = dir.join("worker.json");
        let trace_path = dir.join("worker.callgrind");
        let result = read(&output_path).ok();
        if output_path.exists() && row["output_sha256"] != sha(&output_path)? {
            return Err("worker output changed".into());
        }
        if trace_path.exists() && row["trace_sha256"] != sha(&trace_path)? {
            return Err("Ir trace changed".into());
        }
        let instructions = fs::read_to_string(&trace_path)
            .ok()
            .and_then(|s| count_ir(&s));
        if json!(instructions) != row["Ir"] {
            return Err("Ir mismatch".into());
        }
        let f = &task["fixture"];
        let (xs, ib) = independent_base(
            f,
            task["family"].as_str().unwrap(),
            task["shift"].as_u64().unwrap(),
            8,
        )?;
        let mut valid = false;
        let mut why = "incomplete";
        if let Some(r) = &result {
            check_base(&r["base_provenance"], &ib)?;
            if r["branches"]
                .as_array()
                .is_some_and(|bs| bs.iter().any(|b| b["timed_out"] == true))
            {
                why = "cooperative-budget";
            }
            if ib["filled"] != true {
                if r["status"] != "unsupported-underfilled-base" {
                    return Err("underfilled worker status".into());
                }
                why = "unsupported-underfilled-base";
            } else if row["exit_code"] == 0
                && row["signal"].is_null()
                && r["complete_projection"] == true
                && instructions.is_some()
            {
                let (expected_xs, expected) = exact.get(workload).ok_or("missing exact census")?;
                if xs != *expected_xs || r["base"] != json!(xs) || r["target"] != task["target"] {
                    return Err("base/target mismatch".into());
                }
                let actual: Vec<Vec<u64>> =
                    serde_json::from_value(r["canonical_projected_tuples"].clone())?;
                if actual != expected.keys().cloned().collect::<Vec<_>>() {
                    return Err(format!("complete tuple mismatch {index}").into());
                }
                let p = f["p"].as_u64().unwrap();
                let a = f["a"].as_u64().unwrap();
                let b = f["b"].as_u64().unwrap();
                let target = point(&task["target"])?;
                let mut ys = BTreeMap::new();
                for w in r["signed_witnesses"].as_array().ok_or("witnesses")? {
                    let wx: Vec<u64> = serde_json::from_value(w["x"].clone())?;
                    let wy: Vec<u64> = serde_json::from_value(w["y"].clone())?;
                    if wx.len() != wy.len() {
                        return Err("witness length".into());
                    }
                    for (x, y) in wx.iter().zip(wy) {
                        if y == 0 || y >= p || y * y % p != rhs(*x, a, b, p) {
                            return Err("invalid source ordinate".into());
                        }
                        if ys.insert(*x, y).is_some_and(|old| old != y) {
                            return Err("inconsistent ordinate convention".into());
                        }
                    }
                }
                if r["signed_witnesses"] != witnesses(expected, &ys, f, target)? {
                    return Err("minimum-sign rows differ".into());
                }
                // PARI replays every admitted row and rejects flipping one nonzero ordinate.
                let (tx, ty) = target.unwrap();
                gp.push_str(&format!(
                    "E=ellinit([Mod({a},{p}),Mod({b},{p})]);T=[Mod({tx},{p}),Mod({ty},{p})];\n"
                ));
                for w in r["signed_witnesses"].as_array().unwrap() {
                    let wx: Vec<u64> = serde_json::from_value(w["x"].clone())?;
                    let wy: Vec<u64> = serde_json::from_value(w["y"].clone())?;
                    let mask = w["sign_mask"].as_u64().unwrap();
                    gp.push_str("S=[0];U=[0];\n");
                    for i in 0..wx.len() {
                        let sy = if mask >> i & 1 == 0 { wy[i] } else { p - wy[i] };
                        let uy = if i == 0 { p - sy } else { sy };
                        gp.push_str(&format!("S=elladd(E,S,[Mod({},{p}),Mod({sy},{p})]);U=elladd(E,U,[Mod({},{p}),Mod({uy},{p})]);\n",wx[i],wx[i]));
                    }
                    gp.push_str("if(S!=T,error(\"signed sum\"));if(U==T,error(\"wrong sign accepted\"));good++;bad++;\n");
                    positive += 1;
                }
                valid = true;
                why = "accepted-exact-finite-rows";
                if accepted
                    .insert((workload.to_owned(), arm.to_owned()), instructions.unwrap())
                    .is_some()
                {
                    return Err("duplicate workload/arm".into());
                }
            }
        }
        if !valid && (row["exit_code"] == 124 || !row["signal"].is_null()) {
            why = "hard-budget-or-signal";
        }
        checks.push(json!({"index":index,"workload":workload,"arm":arm,"accepted":valid,"reason":why,"Ir":instructions,
            "output_sha256":row["output_sha256"],"trace_sha256":row["trace_sha256"]}));
    }
    gp.push_str(&format!("if(good!={positive}||bad!={positive},error(\"replay count\"));print(\"ALL_PROGRESSION_REPLAY_OK \",good,\" \",bad);\n"));
    fs::write(out.join("verify.gp"), gp)?;
    let mut comparisons = vec![];
    let mut aa = vec![];
    let workloads = plan["cells"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["workload"].as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    let mut distribution_rows = BTreeMap::<String, Vec<f64>>::new();
    for w in &workloads {
        let get = |a: &str| accepted.get(&(w.clone(), a.to_owned())).copied();
        let gate = |a: &str, b: &str| match (get(a), get(b)) {
            (Some(x), Some(y)) => Some((x.max(y) as f64 / x.min(y) as f64) <= 1.05),
            _ => None,
        };
        let direct_aa = gate("direct-chain", "direct-aa");
        let group_aa = gate("group-mitm", "group-aa");
        aa.push(json!({"workload":w,"direct_AA":direct_aa,"group_AA":group_aa}));
        for (candidate, control) in [
            ("hybrid-eight", "direct-chain"),
            ("companion-curve", "direct-chain"),
            ("direct-chain", "group-mitm"),
            ("hybrid-eight", "group-mitm"),
            ("companion-curve", "group-mitm"),
        ] {
            if candidate == "companion-curve"
                && !plan["cells"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|c| c["workload"] == *w && c["arm"] == candidate)
            {
                continue;
            }
            let ratio = match (get(candidate), get(control)) {
                (Some(x), Some(y))
                    if direct_aa == Some(true)
                        && (control != "group-mitm" || group_aa == Some(true)) =>
                {
                    Some(x as f64 / y as f64)
                }
                _ => None,
            };
            if let Some(r) = ratio {
                distribution_rows
                    .entry(format!("{candidate}/{control}"))
                    .or_default()
                    .push(r);
            }
            comparisons
                .push(json!({"workload":w,"candidate":candidate,"control":control,"ratio":ratio}));
        }
    }
    let distributions = distribution_rows
        .into_iter()
        .map(|(comparison, ratios)| json!({"comparison":comparison,"ratio":distribution(ratios)}))
        .collect::<Vec<_>>();
    let mut cross_distributions = BTreeMap::<String, Vec<f64>>::new();
    let mut cross_rows = vec![];
    for cell in plan["cells"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["family"] == "twist-orbit" && c["arm"] == "direct-chain")
    {
        let w = cell["workload"].as_str().unwrap();
        for ordinary in ["curve-orbit", "field-ap", "uniform"] {
            let other = w.replace("twist-orbit", ordinary);
            for algorithm in ["direct-chain", "hybrid-eight", "group-mitm"] {
                let gate_arm = if algorithm == "group-mitm" {
                    "group_AA"
                } else {
                    "direct_AA"
                };
                let gates = [w, other.as_str()].iter().all(|name| {
                    aa.iter()
                        .any(|a| a["workload"] == *name && a[gate_arm] == true)
                });
                let ratio = match (
                    accepted.get(&(w.to_owned(), algorithm.into())),
                    accepted.get(&(other.clone(), algorithm.into())),
                ) {
                    (Some(x), Some(y)) if gates => Some(*x as f64 / *y as f64),
                    _ => None,
                };
                if let Some(r) = ratio {
                    cross_distributions
                        .entry(format!("twist-orbit/{ordinary}:{algorithm}"))
                        .or_default()
                        .push(r);
                }
                cross_rows.push(json!({"twist_workload":w,"ordinary_workload":other,"algorithm":algorithm,"cost_ratio":ratio,"same_output_speedup":false}));
            }
        }
    }
    let cross_distributions=cross_distributions.into_iter().map(|(comparison,values)|json!({"comparison":comparison,"descriptive_ratio":distribution(values)})).collect::<Vec<_>>();
    let mut by_arm = BTreeMap::<String, (usize, usize)>::new();
    for (cell, check) in plan["cells"].as_array().unwrap().iter().zip(&checks) {
        let c = by_arm
            .entry(cell["arm"].as_str().unwrap().to_owned())
            .or_default();
        c.0 += 1;
        c.1 += usize::from(check["accepted"] == true);
    }
    let mut yield_aggregate = vec![];
    for family in ["twist-orbit", "curve-orbit", "field-ap", "uniform"] {
        for shift in [0, 1, 3] {
            for size in [8, 16] {
                for arity in [2, 3] {
                    let rows = yield_rows
                        .iter()
                        .filter(|r| {
                            r["family"] == family
                                && r["shift"] == shift
                                && r["B"] == size
                                && r["arity"] == arity
                        })
                        .collect::<Vec<_>>();
                    let values = rows
                        .iter()
                        .filter_map(|r| r["relations"].as_u64())
                        .collect::<Vec<_>>();
                    yield_aggregate.push(json!({"family":family,"shift":shift,"B":size,"arity":arity,"requested_queries":rows.len(),"supported_queries":values.len(),
            "nonempty_queries":values.iter().filter(|n|**n>0).count(),"total_relations":values.iter().sum::<u64>(),"relation_count_min":values.iter().min(),"relation_count_max":values.iter().max()}));
                }
            }
        }
    }
    write(out.join("GEOMETRY_CHECKS.json"), &json!(geo_checks))?;
    write(out.join("QUERY_YIELDS.json"), &json!(yield_rows))?;
    write(out.join("COST_CHECKS.json"), &json!(checks))?;
    write(out.join("AA_CHECKS.json"), &json!(aa))?;
    write(out.join("COMPARISONS.json"), &json!(comparisons))?;
    write(out.join("CROSS_CONSTRUCTOR_COSTS.json"), &json!(cross_rows))?;
    write(
        out.join("AUDIT.json"),
        &json!({"schema":"twist.progression-audit/v1","plan_sha256":sha(plan_file)?,"records_sha256":sha(run.join("RECORDS.json"))?,
        "geometry_rows_sha256":sha(census.join("ROWS.json"))?,"construction_records":240,"geometry_queries":7680,
        "filled_bases":geo_checks.iter().filter(|r|r["filled"]==true).count(),"requested_cells":880,"accepted_cells":accepted.len(),
        "by_arm":by_arm.into_iter().map(|(arm,(requested,accepted))|json!({"arm":arm,"requested":requested,"accepted":accepted})).collect::<Vec<_>>(),
        "ratio_distributions":distributions,"cross_constructor_cost_distributions":cross_distributions,"query_yield_aggregates":yield_aggregate,"PARI_positive_and_wrong_sign_expected_each":positive,
        "replay_script_sha256":sha(out.join("verify.gp"))?,
        "PARI_replay_admitted":false,"unit":"simulated guest instructions Ir","wall_time_admitted":false,"complete_IC_cost":false,
        "different_constructor_outputs_identical":false,"cross_constructor_speedup_claim":false,"novelty_established":false,"final_E28_labels_opened":false}),
    )
}
fn admit(out: &Path, receipt_path: &Path) -> Result<()> {
    if out.join("ADMISSION.json").exists() {
        return Err("fresh admission required".into());
    }
    let audit = read(out.join("AUDIT.json"))?;
    let receipt = read(receipt_path)?;
    let stdout = Path::new(receipt["stdout"].as_str().ok_or("stdout path")?);
    let stderr = Path::new(receipt["stderr"].as_str().ok_or("stderr path")?);
    let script = out.join("verify.gp");
    let expected = audit["PARI_positive_and_wrong_sign_expected_each"]
        .as_u64()
        .unwrap();
    if receipt["exit_code"] != 0
        || !receipt["signal"].is_null()
        || receipt["script_sha256"] != sha(&script)?
        || audit["replay_script_sha256"] != sha(&script)?
        || receipt["stdout_sha256"] != sha(stdout)?
        || receipt["stderr_sha256"] != sha(stderr)?
    {
        return Err("PARI receipt/hash/exit gate".into());
    }
    let output = fs::read_to_string(stdout)?;
    let errors = fs::read_to_string(stderr)?;
    if output.trim() != format!("ALL_PROGRESSION_REPLAY_OK {expected} {expected}")
        || !errors
            .lines()
            .all(|l| l.trim().is_empty() || l.contains("Warning: new maximum stack size ="))
    {
        return Err("PARI clean-output gate".into());
    }
    write(
        out.join("ADMISSION.json"),
        &json!({"schema":"twist.progression-admission/v1","status":"independent-finite-census-raw-Ir-and-PARI-replay-verified",
        "audit_sha256":sha(out.join("AUDIT.json"))?,"receipt_sha256":sha(receipt_path)?,"PARI_positive_and_wrong_sign_each":expected,
        "accepted_cost_cells":audit["accepted_cells"],"geometry_bases":240,"geometry_queries":7680,"complete_IC_or_novelty_claim":false}),
    )
}
fn main() -> Result<()> {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    match a.first().map(String::as_str){
    Some("plan")if a.len()==3=>plan(Path::new(&a[1]),Path::new(&a[2])),
    Some("run")if a.len()==4=>run(Path::new(&a[1]),Path::new(&a[2]),Path::new(&a[3])),
    Some("audit")if a.len()==5=>audit(Path::new(&a[1]),Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),
    Some("admit")if a.len()==3=>admit(Path::new(&a[1]),Path::new(&a[2])),
    _=>Err("usage: twist-progression plan PRIOR OUT | run PLAN WORKER OUT | audit PLAN RUN CENSUS OUT | admit AUDIT RECEIPT".into())}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_arithmetic_handles_identity_and_verticals() {
        for p in [7, 59] {
            for x in 0..p {
                for y in 1..p {
                    let q = Some((x, y));
                    assert_eq!(add(q, neg(q, p), 1, p), None);
                    assert_eq!(add(None, q, 1, p), q);
                }
            }
        }
    }
    #[test]
    fn inverse_is_euclidean() {
        for p in [7, 59, 211] {
            for x in 1..p {
                assert_eq!(x * inverse(x, p) % p, 1);
            }
        }
    }
    #[test]
    fn malformed_ir_is_not_admitted() {
        assert_eq!(count_ir("events: Ir Dr\nsummary: 1\n"), None);
        assert_eq!(count_ir("events: Ir\nsummary: 123\n"), Some(123));
    }
    #[test]
    fn independent_unshifted_twist_is_empty() {
        let f = json!({"p":59,"a":1,"b":13,"d":2,"order":67,"generator":[1,29]});
        let (xs, v) = independent_base(&f, "twist-orbit", 0, 8).unwrap();
        assert!(xs.is_empty());
        assert_eq!(v["filled"], false);
    }
    #[test]
    fn duplicate_targets_receive_identical_counts() {
        let f = json!({"p":59,"a":1,"b":13,"d":2,"order":67,"generator":[1,29]});
        let (xs, _) = independent_base(&f, "field-ap", 1, 8).unwrap();
        let rows = all_rows(&f, &xs, 2, &[Some((1, 29)), Some((1, 29))]);
        assert_eq!(rows[0], rows[1]);
    }
}
