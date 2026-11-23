use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
    time::Instant,
};
use trace_zero_experiments::Case;
const ARMS: [&str; 7] = [
    "v1-eager",
    "v1-lazy",
    "projected-eager",
    "projected-lazy",
    "tuned-eager",
    "tuned-lazy",
    "projected-eager-aa",
];
fn same(a: &Value, b: &Value) {
    for key in [
        "outcome",
        "indices",
        "points",
        "models",
        "patterns_attempted",
        "patterns_refuted",
        "reason",
        "common",
    ] {
        assert_eq!(a[key], b[key], "different {key}");
    }
}
fn instructions(path: &Path) -> u64 {
    let s = fs::read_to_string(path).unwrap();
    s.lines()
        .find_map(|l| l.strip_prefix("summary: "))
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}
fn invoke(input: &Path, arm: &str, root: &Path, no_phases: bool) -> (Value, Value) {
    fs::create_dir(root).unwrap();
    let exe = env::current_exe().unwrap();
    let native = Command::new(&exe)
        .arg("one")
        .arg(input)
        .arg(arm)
        .output()
        .unwrap();
    fs::write(root.join("native.stdout"), &native.stdout).unwrap();
    fs::write(root.join("native.stderr"), &native.stderr).unwrap();
    assert!(native.status.success(), "native child");
    let result: Value = serde_json::from_slice(&native.stdout).unwrap();
    let mut cmd = Command::new(env::var("VALGRIND").unwrap());
    cmd.arg("--tool=callgrind")
        .arg(format!(
            "--callgrind-out-file={}",
            root.join("callgrind.out").display()
        ))
        .arg(&exe)
        .arg("one")
        .arg(input)
        .arg(arm)
        .env("TRACE_DISABLE_TALLY", "1");
    if no_phases {
        cmd.env("TRACE_NO_PHASE_DUMPS", "1");
    }
    let prof = cmd.output().unwrap();
    fs::write(root.join("profile.stdout"), &prof.stdout).unwrap();
    fs::write(root.join("profile.stderr"), &prof.stderr).unwrap();
    assert!(prof.status.success(), "Callgrind child");
    let observed: Value = serde_json::from_slice(&prof.stdout).unwrap();
    same(&result, &observed);
    let mut phases = BTreeMap::new();
    let mut total = 0;
    for entry in fs::read_dir(root).unwrap() {
        let p = entry.unwrap().path();
        if !p
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("callgrind.out")
        {
            continue;
        }
        let text = fs::read_to_string(&p).unwrap();
        let ir = instructions(&p);
        total += ir;
        let phase = if text.contains("Client Request: setup") {
            "setup"
        } else if text.contains("Client Request: oracle") {
            "oracle"
        } else {
            "post"
        };
        assert!(phases.insert(phase, ir).is_none(), "duplicate phase");
    }
    if !no_phases {
        assert_eq!(phases.len(), 3);
        assert!(phases["oracle"] > 0);
    }
    let input_metadata: Value = serde_json::from_slice(&fs::read(input).unwrap()).unwrap();
    let n = input_metadata["n"].as_u64().unwrap();
    let field_total = |key: &str| {
        result["setup_field"][format!("f2m{n}_{key}")]
            .as_u64()
            .unwrap_or(0)
            + result["oracle_and_wrapper_replay_field"][format!("f2m{n}_{key}")]
                .as_u64()
                .unwrap_or(0)
    };
    let metrics = json!({"instructions":total,"setup_instructions":phases.get("setup"),"oracle_instructions":phases.get("oracle"),"post_instructions":phases["post"],"field_mul":field_total("mul"),"field_square":field_total("square")});
    (result, metrics)
}
fn summarize(rows: &[Value]) -> Value {
    let mut groups = BTreeMap::<String, Vec<&Value>>::new();
    let mut seen = BTreeSet::new();
    for r in rows {
        assert_eq!(r["result"]["valid"], true);
        assert!(seen.insert((
            r["instance"].to_string(),
            r["arm"].to_string(),
            r["replicate"].to_string()
        )));
        groups
            .entry(format!(
                "{}/{}",
                r["instance"].as_str().unwrap(),
                r["arm"].as_str().unwrap()
            ))
            .or_default()
            .push(r);
    }
    let mut cells = BTreeMap::new();
    for (k, rs) in groups {
        let mut metrics = BTreeMap::new();
        for m in [
            "instructions",
            "setup_instructions",
            "oracle_instructions",
            "post_instructions",
            "field_mul",
            "field_square",
        ] {
            let mut v: Vec<_> = rs.iter().map(|r| r[m].as_u64().unwrap()).collect();
            v.sort();
            metrics.insert(
                m,
                json!({"min":v[0],"median":v[v.len()/2],"max":v[v.len()-1]}),
            );
        }
        cells.insert(k, json!({"rows":rs.len(),"metrics":metrics}));
    }
    let ids: BTreeSet<_> = rows
        .iter()
        .map(|r| r["instance"].as_str().unwrap())
        .collect();
    let mut contrasts = Vec::new();
    for id in ids {
        for (metric, left, right, scope) in [
            (
                "instructions",
                "v1-eager",
                "tuned-lazy",
                "combined tuning and setup versus v1",
            ),
            (
                "oracle_instructions",
                "v1-eager",
                "tuned-eager",
                "tuned versus v1, eager setup fixed",
            ),
            (
                "oracle_instructions",
                "projected-eager",
                "tuned-eager",
                "tuned versus projected, eager setup fixed",
            ),
            (
                "instructions",
                "projected-eager",
                "tuned-lazy",
                "combined tuning and setup versus projected",
            ),
            (
                "instructions",
                "tuned-eager",
                "tuned-lazy",
                "context eager/lazy, algorithm fixed",
            ),
            (
                "instructions",
                "projected-eager",
                "projected-eager-aa",
                "A/A",
            ),
        ] {
            let a = cells[&format!("{id}/{left}")]["metrics"][metric]["median"]
                .as_u64()
                .unwrap();
            let b = cells[&format!("{id}/{right}")]["metrics"][metric]["median"]
                .as_u64()
                .unwrap();
            contrasts.push(json!({"instance":id,"metric":metric,"left":left,"right":right,"scope":scope,"left_median":a,"right_median":b,"ratio":a as f64/b as f64,"relative_difference":a.abs_diff(b)as f64/a as f64}));
        }
    }
    json!({"schema":"trace-zero-projected-summary/v1","rows":rows.len(),"cells":cells,"contrasts":contrasts,"scope":"Disclosed fixed single-query payloads; simulated instructions with calibrated setup/oracle/post counter partitions. No native-time or external-best claim."})
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
    if a[0] == "one" {
        let c: Case = serde_json::from_slice(&fs::read(&a[1]).unwrap()).unwrap();
        let r = trace_zero_tuning::execute(&c, &a[2]);
        assert_eq!(r["valid"], true);
        println!("{}", r);
        return;
    }
    if a[0] == "prepare" {
        let v: Value = serde_json::from_slice(&fs::read(&a[1]).unwrap()).unwrap();
        let cases: Vec<Case> = serde_json::from_value(v["cases"].clone()).unwrap();
        fs::create_dir(&a[2]).unwrap();
        for c in cases
            .into_iter()
            .filter(|c| [83, 131].contains(&c.n) && c.dimension == 12)
        {
            fs::write(
                Path::new(&a[2]).join(format!("{}.json", c.id)),
                serde_json::to_vec(&c).unwrap(),
            )
            .unwrap();
        }
        return;
    }
    let inputs = Path::new(&a[1]);
    let root = Path::new(&a[2]);
    fs::create_dir_all(root).unwrap();
    let start = Instant::now();
    if a[0] == "calibrate" {
        let input = inputs.join("scale-n131-d12-distinct-planted-1.json");
        let (r, p) = invoke(&input, "v1-eager", &root.join("phased"), false);
        let (s, q) = invoke(&input, "v1-eager", &root.join("unphased"), true);
        same(&r, &s);
        let d = p["instructions"]
            .as_u64()
            .unwrap()
            .abs_diff(q["instructions"].as_u64().unwrap()) as f64
            / q["instructions"].as_u64().unwrap() as f64;
        assert!(
            d < 0.001,
            "dump sums must match unphased whole-process instruction count: {d}"
        );
        println!(
            "{}",
            serde_json::to_string_pretty(
                &json!({"phased":p,"unphased":q,"relative_difference":d,"passes":true})
            )
            .unwrap()
        );
        return;
    }
    if a[0] == "pilot" {
        let mut out = Vec::new();
        for n in [83, 131] {
            for kind in ["distinct-planted-1", "uniform"] {
                let id = format!("scale-n{n}-d12-{kind}");
                let input = inputs.join(format!("{id}.json"));
                for arm in ARMS {
                    let begin = Instant::now();
                    let (r, m) = invoke(&input, arm, &root.join(format!("{id}-{arm}")), false);
                    out.push(json!({"instance":id,"n":n,"arm":arm,"result":r,"metrics":m,"engineering_elapsed_seconds":begin.elapsed().as_secs_f64()}));
                }
            }
        }
        println!(
            "{}",
            serde_json::to_string_pretty(
                &json!({"rows":out,"engineering_total_seconds":start.elapsed().as_secs_f64()})
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
        for (ni, n) in [83, 131].iter().enumerate() {
            for (ki, kind) in ["distinct-planted-0", "uniform", "cancellation"]
                .iter()
                .enumerate()
            {
                let id = format!("scale-n{n}-d12-{kind}");
                let input = inputs.join(format!("{id}.json"));
                let c: Case = serde_json::from_slice(&fs::read(&input).unwrap()).unwrap();
                for j in 0..7 {
                    assert!(
                        start.elapsed().as_secs() < 590,
                        "retain global-limit prefix"
                    );
                    let arm = ARMS[(rep + ni + ki + j) % 7];
                    let (r, m) =
                        invoke(&input, arm, &root.join(format!("r{rep}-{id}-{arm}")), false);
                    if let Some(old) = reference.get(&id) {
                        same(old, &r);
                    } else {
                        reference.insert(id.clone(), r.clone());
                    }
                    let fcount = |key: &str| {
                        r["setup_field"][format!("f2m{n}_{key}")]
                            .as_u64()
                            .unwrap_or(0)
                            + r["oracle_and_wrapper_replay_field"][format!("f2m{n}_{key}")]
                                .as_u64()
                                .unwrap_or(0)
                    };
                    let row = json!({"schema":"trace-zero-projected-row/v1","instance":id,"n":n,"dimension":12,"kind":kind,"seed":c.seed,"arm":arm,"replicate":rep,"warmup":false,"status":"succeeded","verification":"verified","wall_time_valid":false,"instructions":m["instructions"],"setup_instructions":m["setup_instructions"],"oracle_instructions":m["oracle_instructions"],"post_instructions":m["post_instructions"],"field_mul":fcount("mul"),"field_square":fcount("square"),"result":r});
                    writeln!(f, "{}", row).unwrap();
                    f.flush().unwrap();
                    count += 1;
                    eprintln!("{count}/210 r{rep} {id} {arm}");
                }
            }
        }
    }
}
