//! `repsel`: the native tooling of research/ic-representative-selection_20261105.
//!
//!   repsel ascend  --class-dir D                       transport.jsonl for every floor curve (arms B, B0)
//!   repsel specs   --class-dir D --design F --tier T --class ID [--with-c] --out DIR
//!   repsel select  --records LIST --labels LIST --expected-runs N --out F
//!   repsel hold    --class-dir D --design F --tier T --class ID --selection F --out DIR
//!   repsel mask    --labels LIST --comparisons F --nonce HEX --out-dir DIR
//!   repsel analyse --records LIST --labels LIST --comparisons F [--boot N] [--perm N] [--seed S] --out F
//!   repsel unmask  --key F --in F --out F
//!
//! LIST is comma-separated paths or `@file` with one path per line.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use repsel::analyse::{self, Rng};
use repsel::{ascend, select, specs};
use serde_json::{json, Value};

fn die(msg: &str) -> ! {
    eprintln!("repsel: {msg}");
    std::process::exit(2)
}

struct Args(HashMap<String, String>, BTreeSet<String>);

impl Args {
    fn parse(v: &[String]) -> Args {
        let mut kv = HashMap::new();
        let mut flags = BTreeSet::new();
        let mut i = 0;
        while i < v.len() {
            let k = v[i].trim_start_matches("--").to_string();
            if k == "with-c" {
                flags.insert(k);
                i += 1;
                continue;
            }
            let val = v
                .get(i + 1)
                .cloned()
                .unwrap_or_else(|| die(&format!("--{k} needs a value")));
            kv.insert(k, val);
            i += 2;
        }
        Args(kv, flags)
    }
    fn get(&self, k: &str) -> &str {
        self.0
            .get(k)
            .map(String::as_str)
            .unwrap_or_else(|| die(&format!("missing --{k}")))
    }
    fn num(&self, k: &str, d: u64) -> u64 {
        self.0
            .get(k)
            .map(|v| {
                v.parse()
                    .unwrap_or_else(|_| die(&format!("--{k} takes an integer")))
            })
            .unwrap_or(d)
    }
}

fn read(p: &str) -> String {
    fs::read_to_string(p).unwrap_or_else(|e| die(&format!("{p}: {e}")))
}

fn json_file(p: &str) -> Value {
    serde_json::from_str(&read(p)).unwrap_or_else(|e| die(&format!("{p}: {e}")))
}

fn list(arg: &str) -> Vec<String> {
    if let Some(f) = arg.strip_prefix('@') {
        read(f)
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect()
    } else {
        arg.split(',')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }
}

fn write_json(p: &str, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n")
        .unwrap_or_else(|e| die(&format!("{p}: {e}")));
}

fn cmd_ascend(a: &Args) {
    let rows =
        ascend::write_transport(&PathBuf::from(a.get("class-dir"))).unwrap_or_else(|e| die(&e));
    let ops = |k: &str| {
        rows.iter()
            .map(|t| t[k]["ops_total"].as_u64().unwrap())
            .collect::<Vec<_>>()
    };
    let (b, b0) = (ops("B"), ops("B0"));
    println!(
        "{}",
        json!({"floor_curves": rows.len(), "B_ops_min": b.iter().min(), "B_ops_max": b.iter().max(),
                          "B0_ops_min": b0.iter().min(), "B0_ops_max": b0.iter().max()})
    );
}

fn cmd_specs(a: &Args, hold: bool) {
    let design = json_file(a.get("design"));
    let dir = PathBuf::from(a.get("class-dir"));
    let out = PathBuf::from(a.get("out"));
    let lines = if hold {
        specs::post_selection(
            &dir,
            &design,
            a.get("tier"),
            a.get("class"),
            &json_file(a.get("selection")),
            &out,
        )
    } else {
        specs::pre_selection(
            &dir,
            &design,
            a.get("tier"),
            a.get("class"),
            a.1.contains("with-c"),
            &out,
        )
    }
    .unwrap_or_else(|e| die(&e));
    for l in lines {
        println!("{l}");
    }
}

fn cmd_select(a: &Args) {
    let recs: Vec<String> = list(a.get("records")).iter().map(|p| read(p)).collect();
    let labs: Vec<String> = list(a.get("labels")).iter().map(|p| read(p)).collect();
    let s = select::select(&recs, &labs, a.num("expected-runs", 40) as usize)
        .unwrap_or_else(|e| die(&e));
    write_json(a.get("out"), &s);
}

fn cmd_mask(a: &Args) {
    let out = PathBuf::from(a.get("out-dir"));
    fs::create_dir_all(&out).unwrap();
    let files = list(a.get("labels"));
    let mut arms = BTreeSet::new();
    let mut parsed = Vec::new();
    for p in &files {
        let rows: Vec<Value> = read(p)
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        for r in &rows {
            for l in r["labels"].as_array().unwrap() {
                arms.insert(l["arm"].as_str().unwrap().to_string());
            }
        }
        parsed.push((p.clone(), rows));
    }
    let nonce = u64::from_str_radix(a.get("nonce").trim_start_matches("0x"), 16)
        .unwrap_or_else(|_| die("--nonce takes hex"));
    let mut names: Vec<String> = arms.into_iter().collect();
    let mut rng = Rng::new(nonce);
    for i in (1..names.len()).rev() {
        let j = rng.below(i + 1);
        names.swap(i, j);
    }
    let map: HashMap<String, String> = names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), format!("K{:02}", i + 1)))
        .collect();
    for (p, rows) in parsed {
        let mut s = String::new();
        for mut r in rows {
            analyse::relabel(&mut r, &map);
            r["job"] = json!("masked");
            s += &r.to_string();
            s.push('\n');
        }
        fs::write(out.join(Path::new(&p).file_name().unwrap()), s).unwrap();
    }
    let mut c = json_file(a.get("comparisons"));
    analyse::relabel(&mut c, &map);
    write_json(out.join("comparisons.json").to_str().unwrap(), &c);
    let key: BTreeMap<String, String> = map.into_iter().map(|(k, v)| (v, k)).collect();
    write_json(
        out.join("key.json").to_str().unwrap(),
        &json!({"schema": "repsel.mask-key/v1", "code_to_arm": key}),
    );
}

fn cmd_unmask(a: &Args) {
    let key = json_file(a.get("key"));
    let map: HashMap<String, String> = key["code_to_arm"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
        .collect();
    let mut v = json_file(a.get("in"));
    analyse::relabel(&mut v, &map);
    v["unmasked_with"] = json!(a.get("key"));
    write_json(a.get("out"), &v);
}

fn cmd_analyse(a: &Args) {
    let recs: Vec<String> = list(a.get("records")).iter().map(|p| read(p)).collect();
    let labs: Vec<String> = list(a.get("labels")).iter().map(|p| read(p)).collect();
    let spec = json_file(a.get("comparisons"));
    let (boot, perm, seed) = (
        a.num("boot", 4000) as usize,
        a.num("perm", 10000) as usize,
        a.num("seed", 20261105),
    );
    let labels = analyse::read_labels(&labs).unwrap_or_else(|e| die(&e));
    let (obs, unlabelled) = analyse::observations(&recs, &labels).unwrap_or_else(|e| die(&e));
    let all_runs: Vec<analyse::Run> = recs
        .iter()
        .flat_map(|t| {
            t.lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| analyse::parse_run(l).unwrap())
        })
        .collect();
    let mut checks = BTreeMap::new();
    checks.insert(
        "check:determinism".to_string(),
        analyse::check_determinism(&all_runs),
    );
    let tiers: BTreeSet<String> = obs.iter().map(|o| o.tier.clone()).collect();
    for t in &tiers {
        checks.insert(
            format!("check:transport:{t}"),
            analyse::check_transport(&obs, t),
        );
    }
    let mut results = BTreeMap::new();
    for (i, c) in spec["comparisons"]
        .as_array()
        .unwrap_or_else(|| die("no comparisons"))
        .iter()
        .enumerate()
    {
        let r = analyse::compare(&obs, c, boot, perm, seed.wrapping_add(1000 * i as u64))
            .unwrap_or_else(|e| die(&e));
        eprintln!("{} done", c["id"]);
        results.insert(c["id"].as_str().unwrap().to_string(), r);
    }
    let decisions = analyse::decide(&spec, &results, &checks).unwrap_or_else(|e| die(&e));
    // Secondary hypotheses: their own Holm family, outside the primary one.
    let secondary = match spec.get("secondary_hypotheses") {
        Some(h) => analyse::decide(
            &json!({"family_alpha": spec["secondary_family_alpha"], "hypotheses": h}),
            &results,
            &checks,
        )
        .unwrap_or_else(|e| die(&e)),
        None => json!([]),
    };
    // Per tier, label and method: counts by status and mean priced S at each weight.
    let mut summary: BTreeMap<String, (usize, usize, [f64; 3], f64)> = BTreeMap::new();
    for o in obs.iter().filter(|o| !o.run.warmup) {
        let e = summary
            .entry(format!("{}|{}|{}", o.tier, o.label, o.run.arm))
            .or_insert((0, 0, [0.0; 3], 0.0));
        e.0 += 1;
        if o.run.ok {
            e.1 += 1;
            for (k, w) in analyse::WEIGHTS.iter().enumerate() {
                e.2[k] += o.priced_s(*w).unwrap_or(f64::NAN);
            }
            e.3 += o.run.total_gae;
        }
    }
    let summary: Vec<Value> = summary
        .into_iter()
        .map(|(k, (n, ok, s, g))| {
            let p: Vec<&str> = k.split('|').collect();
            json!({"tier": p[0], "arm": p[1], "method": p[2], "runs": n, "verified": ok,
                   "mean_s_by_weight": s.iter().map(|x| x / ok.max(1) as f64).collect::<Vec<_>>(),
                   "total_gae_sum": g})
        })
        .collect();
    write_json(
        a.get("out"),
        &json!({
            "schema": "repsel.analysis/v1", "boot": boot, "perm": perm, "seed": seed, "weights": analyse::WEIGHTS,
            "record_files": recs.len(), "unlabelled_runs": unlabelled.len(),
            "checks": checks, "comparisons": results, "hypotheses": decisions, "secondary_hypotheses": secondary, "summary": summary,
        }),
    );
}

fn main() {
    let v: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = v.first() else {
        die("usage: see the header of src/main.rs")
    };
    let a = Args::parse(&v[1..]);
    match cmd.as_str() {
        "ascend" => cmd_ascend(&a),
        "specs" => cmd_specs(&a, false),
        "hold" => cmd_specs(&a, true),
        "select" => cmd_select(&a),
        "mask" => cmd_mask(&a),
        "unmask" => cmd_unmask(&a),
        "analyse" => cmd_analyse(&a),
        other => die(&format!("unknown command {other}")),
    }
}
