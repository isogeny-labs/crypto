use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
    time::Instant,
};
use trace_zero_experiments::Case;
const ARMS: [&str; 3] = ["baseline", "baseline-aa", "candidate"];
fn same(a: &Value, b: &Value) {
    for key in [
        "outcome",
        "indices",
        "points",
        "models",
        "patterns_attempted",
        "patterns_refuted",
        "reason",
        "diagnostics",
    ] {
        assert_eq!(a[key], b[key], "different {key}");
    }
}
fn command(exe: &Path, input: &str, id: &str, arm: &str) -> Command {
    let mut c = Command::new(exe);
    c.args(["one", input, id, arm]);
    c
}
fn call(input: &str, case: &Case, arm: &str, out: &Path) -> Value {
    fs::create_dir(out).unwrap();
    let exe = env::current_exe().unwrap();
    let counted = command(&exe, input, &case.id, arm).output().unwrap();
    fs::write(out.join("counted.stdout"), &counted.stdout).unwrap();
    fs::write(out.join("counted.stderr"), &counted.stderr).unwrap();
    assert!(counted.status.success(), "counted child failure");
    let counted: Value = serde_json::from_slice(&counted.stdout).unwrap();
    let vg = env::var("VALGRIND").expect("explicit valgrind path");
    let prof = Command::new(vg)
        .args(["--tool=callgrind", "--cache-sim=no", "--branch-sim=no"])
        .arg(format!(
            "--callgrind-out-file={}",
            out.join("callgrind.out").display()
        ))
        .arg(&exe)
        .args(["one", input, &case.id, arm])
        .env("TRACE_DISABLE_TALLY", "1")
        .output()
        .unwrap();
    fs::write(out.join("profile.stdout"), &prof.stdout).unwrap();
    fs::write(out.join("profile.stderr"), &prof.stderr).unwrap();
    assert!(prof.status.success(), "profile child failure");
    let prof: Value = serde_json::from_slice(&prof.stdout).unwrap();
    same(&counted["result"], &prof["result"]);
    let text = fs::read_to_string(out.join("callgrind.out")).unwrap();
    let ir: u64 = text
        .lines()
        .find_map(|l| l.strip_prefix("summary: "))
        .expect("instruction total")
        .trim()
        .parse()
        .unwrap();
    let mut r = counted["result"].clone();
    r["instructions"] = json!(ir);
    let prefix = format!("f2m{}_", case.n);
    let field = &r["field_operations"];
    let metrics = json!({"instructions":ir,"field_mul":field[format!("{prefix}mul")].as_u64().unwrap_or(0),"field_square":field[format!("{prefix}square")].as_u64().unwrap_or(0),"inverse_informational":field[format!("{prefix}inverse_informational")].as_u64().unwrap_or(0)});
    r["metrics"] = metrics;
    r
}
fn summarize(rows: &[Value]) -> Value {
    let mut groups: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    let mut outcomes = BTreeMap::<String, usize>::new();
    for r in rows {
        assert_eq!(r["result"]["valid"], true);
        groups
            .entry(format!(
                "{}/{}",
                r["instance"].as_str().unwrap(),
                r["arm"].as_str().unwrap()
            ))
            .or_default()
            .push(r);
        *outcomes
            .entry(format!(
                "{}/{}",
                r["arm"].as_str().unwrap(),
                r["result"]["outcome"].as_str().unwrap()
            ))
            .or_default() += 1;
    }
    let mut cells = BTreeMap::new();
    for (k, rs) in groups {
        let mut ir: Vec<_> = rs
            .iter()
            .map(|r| r["instructions"].as_u64().unwrap())
            .collect();
        ir.sort();
        let mut fields = BTreeMap::new();
        for metric in ["field_mul", "field_square", "inverse_informational"] {
            let mut x: Vec<_> = rs.iter().map(|r| r[metric].as_u64().unwrap()).collect();
            x.sort();
            fields.insert(
                metric,
                json!({"min":x[0],"max":x[x.len()-1],"median":x[x.len()/2]}),
            );
        }
        cells.insert(k,json!({"rows":rs.len(),"instructions":{"min":ir[0],"median":ir[ir.len()/2],"max":ir[ir.len()-1]},"field":fields}));
    }
    let ids: std::collections::BTreeSet<_> = rows
        .iter()
        .map(|r| r["instance"].as_str().unwrap())
        .collect();
    let mut comparisons = Vec::new();
    for id in ids {
        let b = &cells[&format!("{id}/baseline")];
        let aa = &cells[&format!("{id}/baseline-aa")];
        let c = &cells[&format!("{id}/candidate")];
        let bm = b["instructions"]["median"].as_u64().unwrap();
        let cm = c["instructions"]["median"].as_u64().unwrap();
        let am = aa["instructions"]["median"].as_u64().unwrap();
        comparisons.push(json!({"instance":id,"baseline_median_ir":bm,"baseline_aa_median_ir":am,"candidate_median_ir":cm,"instruction_ratio":bm as f64/cm as f64,"aa_relative_median_difference":bm.abs_diff(am)as f64/bm as f64,"instructions_removed_fraction":1.-cm as f64/bm as f64}));
    }
    json!({"schema":"trace-zero-fast-summary/v1","rows":rows.len(),"outcomes":outcomes,"cells":cells,"comparisons":comparisons,"scope":"Whole cold-process simulated instruction counts and exact native field work, paired fixed controls; no native-time, cycles, fastest-external, population or scaling inference."})
}
fn main() {
    let a: Vec<_> = env::args().skip(1).collect();
    if a[0] == "analyze" {
        let rows: Vec<Value> = fs::read_to_string(&a[1])
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&summarize(&rows)).unwrap()
        );
        return;
    }
    let input: Value = serde_json::from_slice(&fs::read(&a[1]).unwrap()).unwrap();
    let cases: Vec<Case> = serde_json::from_value(input["cases"].clone()).unwrap();
    if a[0] == "one" {
        let c = cases.iter().find(|c| c.id == a[2]).unwrap();
        let r = trace_zero_fast::execute(c, &a[3]);
        assert_eq!(r["valid"], true);
        println!(
            "{}",
            json!({"instance":c.id,"n":c.n,"dimension":c.dimension,"kind":c.kind,"arm":a[3],"result":r})
        );
        return;
    }
    let root = Path::new(&a[2]);
    fs::create_dir_all(root).unwrap();
    let start = Instant::now();
    if a[0] == "pilot" {
        let mut rows = Vec::new();
        for n in [83, 131] {
            for kind in ["distinct-planted-1", "uniform"] {
                let id = format!("scale-n{n}-d12-{kind}");
                let c = cases.iter().find(|c| c.id == id).unwrap();
                for arm in ARMS {
                    let begin = Instant::now();
                    let r = call(&a[1], c, arm, &root.join(format!("n{n}-{kind}-{arm}")));
                    rows.push(json!({"instance":id,"n":n,"arm":arm,"result":r,"engineering_elapsed_seconds":begin.elapsed().as_secs_f64()}));
                }
            }
        }
        println!(
            "{}",
            serde_json::to_string_pretty(
                &json!({"rows":rows,"engineering_total_seconds":start.elapsed().as_secs_f64()})
            )
            .unwrap()
        );
        return;
    }
    assert_eq!(a[0], "run");
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[3])
        .unwrap();
    let mut reference = BTreeMap::<String, Value>::new();
    let mut count = 0;
    for rep in 0..5 {
        for (n_i, n) in [83, 131].iter().enumerate() {
            for (k_i, kind) in ["distinct-planted-0", "uniform", "cancellation"]
                .iter()
                .enumerate()
            {
                let id = format!("scale-n{n}-d12-{kind}");
                let c = cases.iter().find(|c| c.id == id).unwrap();
                for j in 0..3 {
                    assert!(
                        start.elapsed().as_secs() < 590,
                        "global guard; retain raw prefix"
                    );
                    let arm = ARMS[(rep + n_i + k_i + j) % 3];
                    let r = call(&a[1], c, arm, &root.join(format!("r{rep}-{id}-{arm}")));
                    if let Some(old) = reference.get(&id) {
                        same(old, &r);
                    } else {
                        reference.insert(id.clone(), r.clone());
                    }
                    let m = &r["metrics"];
                    let row = json!({"schema":"trace-zero-fast-row/v1","instance":id,"n":c.n,"dimension":c.dimension,"kind":c.kind,"seed":c.seed,"arm":arm,"replicate":rep,"warmup":false,"status":"succeeded","verification":"verified","wall_time_valid":false,"instructions":m["instructions"],"field_mul":m["field_mul"],"field_square":m["field_square"],"inverse_informational":m["inverse_informational"],"result":r});
                    writeln!(f, "{}", row).unwrap();
                    f.flush().unwrap();
                    count += 1;
                    eprintln!("{count}/90 r{rep} {id} {arm}");
                }
            }
        }
    }
}
