//! Post-call metadata export only; preserves native raw measurements.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{BufRead, BufReader},
};
fn pin(path: &str, kind: &str) -> Value {
    let b = fs::read(path).unwrap();
    json!({"path":path,"sha256":format!("{:x}",Sha256::digest(&b)),"bytes":b.len(),"kind":kind})
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert!(a.len() >= 6, "export PROTOCOL PANEL KIND OUTPUT RAW...");
    let p: Value = serde_json::from_slice(&fs::read(&a[1]).unwrap()).unwrap();
    let base = std::env::var("VIETA_EXPORT_BASE").unwrap_or_else(|_| "next-20261116/run".into());
    let host = std::env::var("VIETA_EXPORT_HOST").unwrap_or_else(|_| {
        format!(
            "{base}/{}",
            if a[2] == "three-repair" {
                "repair/isolation.jsonl"
            } else {
                "main/isolation.jsonl"
            }
        )
    });
    let mut runs = vec![];
    let mut files = vec![];
    let mut completed = 0;
    let mut timeout = 0;
    let mut seen = HashSet::new();
    let mut warm_seen: HashMap<String, usize> = HashMap::new();
    for path in &a[5..] {
        files.push(pin(path, "raw"));
        for (i, line) in BufReader::new(fs::File::open(path).unwrap())
            .lines()
            .enumerate()
        {
            let r: Value = serde_json::from_str(&line.unwrap()).unwrap();
            if r["scope"]["panel"] != a[2] || r["kind"] != a[3] {
                continue;
            }
            let warm = r["warmup"] == true;
            let id = format!(
                "{}-{}-m{}",
                r["scope"]["representation"].as_str().unwrap(),
                r["scope"]["curve"].as_str().unwrap(),
                r["scope"]["m"]
            );
            let arm = r["arm"].as_str().unwrap();
            let rep = if warm {
                i / 4 - 2usize.min(i / 4)
            } else {
                r["round"].as_u64().unwrap() as usize
            };
            // Raw warmup round labels both saturate at zero, so use per-file order.
            let rep = if warm {
                let n = warm_seen.entry(format!("{id}:{arm}")).or_default();
                let r = *n;
                *n += 1;
                r
            } else {
                rep
            };
            let status = if r["status"] == "ok" {
                completed += 1;
                "succeeded"
            } else {
                timeout += 1;
                if r["error"]
                    .as_str()
                    .is_some_and(|s| s.contains("stage deadline"))
                {
                    "timeout"
                } else {
                    "failed"
                }
            };
            let key = format!("{id}:{arm}:{warm}:{rep}");
            assert!(seen.insert(key.clone()), "duplicate execution cell");
            let mut counted = json!({"muls":r["work"]["muls"]});
            if let Some(n) = r["relations"].as_u64() {
                counted["relations"] = json!(n);
            }
            runs.push(json!({"run_id":format!("{}:{key}",a[2]),"arm":arm,"instance_id":id,"seed":r["scope"]["fixture_seed_hex"],"replicate":rep,"warmup":warm,"status":status,"outcome_class":if status=="succeeded"{"completed"}else{"not_completed"},"attempts":1,"resumed":0,"checkpoint":null,"measurements":{"counted":counted,"wall_seconds":r["elapsed_ns"].as_str().map(|x|x.parse::<f64>().unwrap()/1e9),"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":r["work"]},"isolation":{"tier":"L0","contended":false,"blockers":["PSI unavailable","unmovable host threads","producer-only local diagnostic; no L2 claim"]},"verification":{"status":"no_claim","verifier":"Native wide scalar replay checks complete sets; producer-owned, independent signed PCC standing pending"},"exclusion":{"excluded":false,"rule":null},"artifacts":[],"started_at":null,"finished_at":null,"worker":"local CPU4","backend_ref":format!("{path}:{}",i+1)}));
        }
    }
    let planned = p["sample_size"]["instances"].as_u64().unwrap()
        * p["arms"].as_array().unwrap().len() as u64
        * (p["sample_size"]["replicates"].as_u64().unwrap()
            + p["sample_size"]["warmups"].as_u64().unwrap());
    let actual = runs.len();
    let missing = (planned as usize)
        .checked_sub(actual)
        .expect("more calls than protocol");
    let actual_source = if a[2] == "three-repair" {
        let receipt: Value =
            serde_json::from_slice(&fs::read(format!("{base}/repair-publication.json")).unwrap())
                .unwrap();
        receipt["source_revision"].clone()
    } else {
        p["arms"][0]["instrument"]["commit"].clone()
    };
    let arms:Vec<_>=p["arms"].as_array().unwrap().iter().map(|x|json!({"id":x["id"],"role":x["role"],"binary_sha256":x["instrument"]["binary_sha256"],"commit":actual_source})).collect();
    files.push(pin(&host, "host"));
    let analysis_path =
        std::env::var("VIETA_EXPORT_ANALYSIS").unwrap_or_else(|_| "native/next_compare.rs".into());
    let analysis = pin(&analysis_path, "analysis");
    let e = json!({"schema":"experiment.export/v1","export_id":format!("VIETA-{}",a[2]),"created":std::env::var("VIETA_EXPORT_CREATED").expect("set actual export timestamp"),"protocol":{"path":a[1],"sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]},"producer":{"runtime":"codex","model":"GPT-6","session":"vieta-next-20261116","role":"executor"},"source":{"repo":"crypto","commit":actual_source,"dirty":null},"backend":"local","host":{"path":host,"sha256":files.last().unwrap()["sha256"],"class_id":"local-epyc9v74-kvm-5vcpu-cpu4","summary":{"cpu_model":"AMD EPYC 9V74","logical_cpus":5,"memory_bytes":null,"kernel":"6.18.57","hypervisor":"KVM","isolation_tier_max":"L0"}},"arms":arms,"runs":runs,"flow":{"planned":actual,"attempted":actual,"completed":completed,"not_completed":timeout,"interrupted":0,"resumed":0,"excluded":[]},"files":files,"analysis":{"script":analysis,"blinded":false,"blinding_key_sha256":null,"outputs":[],"unmasked_at":null},"notes":[{"kind":"observation","by":"producer","text":format!("Frozen protocol plans {planned} rows; {actual} actual rows; {missing} missing under the frozen fixed-N plan if nonzero. Original records retained. No missing rows imputed. Local clocks diagnostic only; Git-only custody explicitly authorized. Independent signed standing pending.")}],"reviews_dir":"reviews"});
    let out = &a[4];
    assert!(!std::path::Path::new(out).exists(), "refuse replacement");
    fs::write(out, serde_json::to_vec(&e).unwrap()).unwrap();
    println!(
        "Exported {actual} rows, completed {completed}, timed out {timeout}, missing {missing}"
    );
}
