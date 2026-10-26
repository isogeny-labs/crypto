//! Cold finite decomposition tasks; expected tuples are checked only in parent.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    os::unix::process::ExitStatusExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn write(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    Ok(fs::write(p, serde_json::to_string_pretty(v)? + "\n")?)
}
fn count_ir(s: &str) -> Option<u64> {
    if s.lines()
        .find_map(|s| s.strip_prefix("events:"))
        .map(str::trim)
        != Some("Ir")
    {
        return None;
    }
    s.lines()
        .rev()
        .find_map(|s| s.strip_prefix("totals:"))
        .or_else(|| s.lines().rev().find_map(|s| s.strip_prefix("summary:")))
        .and_then(|s| s.trim().parse().ok())
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
fn plan(fixtures: &Path, census: &Path, base: u64) -> Result<Value> {
    let f = read(fixtures)?;
    let ws = read(census.join("WORKLOADS.json"))?;
    let mut cells = vec![];
    for w in ws
        .as_array()
        .unwrap()
        .iter()
        .filter(|w| w["arity"].as_u64().unwrap() <= 3 && (base == 0 || w["B"] == base))
    {
        let fixture = f["fixtures"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["id"] == w["fixture"])
            .ok_or("fixture missing")?;
        for arm in [
            "direct-aa",
            "direct-chain",
            "hybrid-eight",
            "first-pair-invariant",
        ] {
            let input = json!({"workload":w["workload"],"fixture":fixture,"B":w["B"],"arity":w["arity"],"target":w["target"],"arm":if arm=="direct-aa"{"direct-chain"}else{arm}});
            let expected = census
                .join("witnesses")
                .join(format!("{}.json", w["workload"].as_str().unwrap()));
            cells.push(json!({"index":cells.len(),"workload":w["workload"],"fixture":w["fixture"],"p":w["p"],"B":w["B"],"arity":w["arity"],"arm":arm,"task":input,"expected":expected,"expected_sha256":sha(&expected)?}));
        }
    }
    Ok(
        json!({"schema":"twist.decomposition-plan/v1","fixtures_sha256":sha(fixtures)?,"census_workloads_sha256":sha(census.join("WORKLOADS.json"))?,"requested":cells.len(),"cells":cells,"target_label":0,"B_filter_zero_means_all":base,"new_targets_opened":false}),
    )
}
fn controls(fixtures: &Path, census: &Path) -> Result<Value> {
    let old = plan(fixtures, census, 8)?;
    let mut cells = vec![];
    for chunk in old["cells"].as_array().unwrap().chunks(4) {
        for method in ["search", "propagate"] {
            for cell in chunk {
                let mut cell = cell.clone();
                cell["index"] = json!(cells.len());
                cell["task"]["lift_method"] = json!(method);
                if method == "propagate" {
                    cell["arm"] = json!(format!("{}-propagate", cell["arm"].as_str().unwrap()));
                }
                cells.push(cell);
            }
        }
        for arm in ["group-aa", "group-mitm"] {
            let mut cell = chunk[0].clone();
            cell["index"] = json!(cells.len());
            cell["arm"] = json!(arm);
            cell["task"]["arm"] = json!("group-mitm");
            cell["task"]["lift_method"] = json!("search");
            cells.push(cell);
        }
    }
    let mut comparisons = vec![];
    for candidate in [
        "hybrid-eight",
        "first-pair-invariant",
        "group-mitm",
        "direct-chain-propagate",
        "hybrid-eight-propagate",
        "first-pair-invariant-propagate",
    ] {
        comparisons.push(json!({"candidate":candidate,"control":"direct-chain"}));
    }
    for arm in ["direct-chain", "hybrid-eight", "first-pair-invariant"] {
        comparisons.push(json!({"candidate":format!("{arm}-propagate"),"control":arm}));
        comparisons.push(json!({"candidate":format!("{arm}-propagate"),"control":"group-mitm"}));
    }
    Ok(
        json!({"schema":"twist.decomposition-control-plan/v1","requested":cells.len(),"cells":cells,"target_label":0,"B":8,"strict_signed_witness_equality":true,"global_budget_s":3800,"new_targets_opened":false,"AA_pairs":[["direct-chain","direct-aa"],["direct-chain-propagate","direct-aa-propagate"],["group-mitm","group-aa"]],"comparisons":comparisons,"fixtures_sha256":old["fixtures_sha256"],"census_workloads_sha256":old["census_workloads_sha256"]}),
    )
}
fn incidence_plan(fixtures: &Path, frontier: &Path) -> Result<Value> {
    let f = read(fixtures)?;
    let rows = read(frontier.join("ROWS.json"))?;
    let mut cells = vec![];
    for row in rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["h"] == r["k"] && [8, 16].contains(&r["h"].as_u64().unwrap()))
    {
        let expected = frontier.join(format!("{}.json", row["case"].as_str().unwrap()));
        if sha(&expected)? != row["record_sha256"] {
            return Err("frontier changed".into());
        }
        let truth = read(&expected)?;
        let fixture = f["fixtures"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["id"] == row["fixture"])
            .ok_or("fixture missing")?;
        for (arm, algorithm) in [
            ("direct-aa", "direct-product"),
            ("direct-product", "direct-product"),
            ("full-mask", "full-mask"),
            ("reduced-u-aa", "reduced-u"),
            ("reduced-u", "reduced-u"),
            ("group-aa", "group-mitm"),
            ("group-mitm", "group-mitm"),
        ] {
            cells.push(json!({"index":cells.len(),"workload":row["case"],"fixture":row["fixture"],"h":row["h"],"k":row["k"],"lambda":row["lambda"],"B":row["B"],"arm":arm,"task":{"case":row["case"],"fixture":fixture,"h":row["h"],"k":row["k"],"lambda":row["lambda"],"target":truth["target"],"arm":algorithm},"expected":expected,"expected_sha256":sha(&expected)?}));
        }
    }
    assert_eq!(cells.len(), 560);
    Ok(
        json!({"schema":"twist.incidence-cost-plan/v1","kind":"incidence","requested":cells.len(),"cells":cells,"solver_budget_ms":2500,"hard_timeout_s":4,"global_budget_s":2100,"target_label":0,"new_targets_opened":false,"selected_grid":"all ten fixtures; H=K in {8,16}; every lambda in {1,2,3,H}; no degree/outcome selection","full_600_cost_grid_complete":false,"fixtures_sha256":sha(fixtures)?,"frontier_rows_sha256":sha(frontier.join("ROWS.json"))?}),
    )
}
fn run(plan_file: &Path, worker: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh output required".into());
    }
    let plan = read(plan_file)?;
    let incidence = plan["kind"] == "incidence";
    fs::create_dir_all(out)?;
    let vgroot = "/workspace/shared/twist-tools-20261025-e26/root/usr";
    let vg = format!("{vgroot}/bin/valgrind");
    let lib = format!("{vgroot}/libexec/valgrind");
    let mut records = vec![];
    let start = Instant::now();
    let mut gp = String::from("default(parisizemax,4000000000);\ngood=0;bad=0;\n");
    for cell in plan["cells"].as_array().unwrap() {
        let mut row = cell.clone();
        if start.elapsed().as_secs() >= plan["global_budget_s"].as_u64().unwrap_or(1800)
            || bytes(out) >= 1_073_741_824
        {
            row["status"] = json!("not-started-global-cap");
            records.push(row);
            continue;
        }
        let index = cell["index"].as_u64().unwrap();
        let dir = out.join(format!("cell-{index:03}"));
        fs::create_dir(&dir)?;
        let input = dir.join("task.json");
        write(&input, &cell["task"])?;
        let output = dir.join("worker.json");
        let trace = dir.join("worker.callgrind");
        let args = vec![
            "followup".into(),
            if incidence {
                "incidence-cost-worker".into()
            } else {
                "decomposition-worker".into()
            },
            input.display().to_string(),
            output.display().to_string(),
            plan["solver_budget_ms"]
                .as_u64()
                .unwrap_or(15000)
                .to_string(),
        ];
        let status = Command::new("/bin/sh")
            .args([
                "-c",
                "ulimit -c 0; ulimit -v 4194304; ulimit -f 65536; exec \"$@\"",
                "decomposition-cost",
                "/usr/bin/timeout",
                "--signal=TERM",
                "--kill-after=1s",
                &format!("{}s", plan["hard_timeout_s"].as_u64().unwrap_or(18)),
            ])
            .arg(&vg)
            .args([
                "--tool=callgrind",
                "--cache-sim=no",
                "--branch-sim=no",
                "--error-exitcode=99",
                &format!("--callgrind-out-file={}", trace.display()),
            ])
            .arg(worker)
            .args(&args)
            .env("VALGRIND_LIB", &lib)
            .env("F4_FP_GM", "1")
            .env("RAYON_NUM_THREADS", "1")
            .env("OMP_NUM_THREADS", "1")
            .env("OPENBLAS_NUM_THREADS", "1")
            .stdout(Stdio::from(fs::File::create(dir.join("worker.stdout"))?))
            .stderr(Stdio::from(fs::File::create(dir.join("worker.stderr"))?))
            .status()?;
        let expected_file = Path::new(cell["expected"].as_str().unwrap());
        if sha(expected_file)? != cell["expected_sha256"] {
            return Err("expected census changed".into());
        }
        let expected = read(expected_file)?;
        let result = read(&output).ok();
        let ir = fs::read_to_string(&trace).ok().and_then(|s| count_ir(&s));
        let matched = status.success()
            && result.as_ref().is_some_and(|r| {
                r["complete_projection"] == true
                    && if incidence {
                        r["canonical_pairs"] == expected["canonical_pairs"]
                            && r["explicit_y_witnesses"] == expected["explicit_y_witnesses"]
                            && r["U"] == expected["U"]
                    } else {
                        r["canonical_projected_tuples"] == expected["canonical_projected_tuples"]
                            && r["base"] == expected["base"]
                    }
                    && r["target"] == expected["target"]
                    && (plan["strict_signed_witness_equality"] != true
                        || r["signed_witnesses"] == expected["witnesses"])
            });
        if matched {
            let r = result.as_ref().unwrap();
            let f = &cell["task"]["fixture"];
            let p = f["p"].as_u64().unwrap();
            let target: Vec<Vec<u64>> = serde_json::from_value(cell["task"]["target"].clone())?;
            gp.push_str(&format!(
                "E=ellinit([Mod({},{}),Mod({}, {})]);T=[Mod({},{}),Mod({}, {})];\n",
                f["a"], p, f["b"], p, target[0][0], p, target[1][0], p
            ));
            let witnesses = if incidence {
                &r["explicit_y_witnesses"]
            } else {
                &r["signed_witnesses"]
            };
            for w in witnesses.as_array().unwrap() {
                let x: Vec<u64> = serde_json::from_value(w["x"].clone())?;
                let y: Vec<u64> = serde_json::from_value(w["y"].clone())?;
                let mask = w["sign_mask"].as_u64().unwrap_or(0);
                gp.push_str("S=[0];U=[0];\n");
                for i in 0..x.len() {
                    let sy = if mask >> i & 1 == 0 {
                        y[i]
                    } else {
                        (p - y[i]) % p
                    };
                    let by = if i == 0 { (p - sy) % p } else { sy };
                    gp.push_str(&format!("P=[Mod({},{}),Mod({}, {})];if(!ellisoncurve(E,P),error(\"point lift\"));S=elladd(E,S,P);U=elladd(E,U,[Mod({},{}),Mod({}, {})]);\n",x[i],p,sy,p,x[i],p,by,p));
                }
                gp.push_str("if(S!=T,error(\"signed decomposition\"));if(U==T,error(\"wrong sign accepted\"));good++;bad++;\n");
            }
        }
        row["status"] = json!(if matched && ir.is_some() {
            "complete-projection-matched-awaiting-independent-replay"
        } else {
            "incomplete-or-projection-mismatch"
        });
        row["projection_matched"] = json!(matched);
        row["guest_instructions_Ir"] = json!(ir);
        row["exit_code"] = json!(status.code());
        row["signal"] = json!(status.signal());
        row["task_sha256"] = json!(sha(&input)?);
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
        row["stdout_sha256"] = json!(sha(dir.join("worker.stdout"))?);
        row["stderr_sha256"] = json!(sha(dir.join("worker.stderr"))?);
        row["argv"] = json!(args);
        println!(
            "{}",
            json!({"cell":index+1,"workload":cell["workload"],"arm":cell["arm"],"matched":matched})
        );
        records.push(row);
        write(out.join("RECORDS.json"), &json!(records))?;
    }
    gp.push_str(&format!(
        "if(good==0||bad==0,error(\"no replay controls\"));print(\"{} \",good,\" \",bad);quit;\n",
        if incidence {
            "ALL_INCIDENCE_COST_REPLAY_OK"
        } else {
            "ALL_DECOMPOSITION_REPLAY_OK"
        }
    ));
    fs::write(out.join("verify.gp"), gp)?;
    write(out.join("RECORDS.json"), &json!(records))?;
    write(
        out.join("SUMMARY.json"),
        &json!({"schema":"twist.decomposition-cost/v1","requested":plan["requested"],"recorded":records.len(),"matched_projection_cells":records.iter().filter(|r|r["projection_matched"]==true).count(),"unit":"Callgrind simulated guest instructions Ir","worker_sha256":sha(worker)?,"harness_sha256":sha(std::env::current_exe()?)?,"plan_sha256":sha(plan_file)?,"GP_replay_executed":false,"complete_IC_cost":false,"wall_time_admitted":false,"speedup_established":false}),
    )
}
fn main() -> Result<()> {
    let a = std::env::args().collect::<Vec<_>>();
    match a.get(1).map(String::as_str) {
        Some("plan") if a.len() == 6 => {
            if Path::new(&a[5]).exists() {
                return Err("fresh plan required".into());
            }
            write(
                &a[5],
                &plan(Path::new(&a[2]), Path::new(&a[3]), a[4].parse()?)?,
            )
        }
        Some("plan-controls") if a.len() == 5 => {
            if Path::new(&a[4]).exists() { return Err("fresh plan required".into()); }
            write(&a[4], &controls(Path::new(&a[2]), Path::new(&a[3]))?)
        }
        Some("plan-incidence") if a.len()==5 => {
            if Path::new(&a[4]).exists(){return Err("fresh plan required".into());}
            write(&a[4],&incidence_plan(Path::new(&a[2]),Path::new(&a[3]))?)
        }
        Some("run") if a.len() == 5 => run(Path::new(&a[2]), Path::new(&a[3]), Path::new(&a[4])),
        _ => {
            Err("usage: plan FIXTURES CENSUS B_OR_0 FRESH_PLAN | plan-controls FIXTURES CENSUS FRESH_PLAN | run PLAN WORKER FRESH_OUT".into())
        }
    }
}
