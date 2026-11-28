//! Only protocol/run metadata, hashes and unit conversion; no crypto/statistics.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::HashSet, fs, path::Path};
const ROOT: &str = "research/vieta_coset_rust_20261115/speedup-20261117";
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn pin(p: &Path, kind: &str) -> Value {
    let b = fs::read(p).unwrap();
    json!({"path":p.strip_prefix(ROOT).unwrap().to_string_lossy(),"sha256":format!("{:x}",Sha256::digest(&b)),"bytes":b.len(),"kind":kind})
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 6, "export PROTOCOL RAW_ROOT OUTPUT UTC");
    assert_eq!(a[1], "export");
    let protocol = Path::new(&a[2]);
    let p = load(protocol);
    let raw = Path::new(&a[3]);
    let host = raw.join("isolation.jsonl");
    let host_text = fs::read_to_string(&host).unwrap();
    let host_record: Value = serde_json::from_str(host_text.lines().last().unwrap()).unwrap();
    let contended = host_record["run"]["contended"].as_bool().unwrap();
    let mut files = vec![];
    let mut runs = vec![];
    let (mut success, mut failed) = (0, 0);
    let mut seen = HashSet::new();
    for dir in fs::read_dir(raw).unwrap() {
        let dir = dir.unwrap().path();
        if !dir.is_dir() {
            continue;
        }
        for entry in fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|x| x != "json") {
                continue;
            }
            let name = path.file_name().unwrap().to_str().unwrap();
            if !name.starts_with("cell-") {
                files.push(pin(&path, "raw"));
                continue;
            }
            let r = load(&path);
            let c = &r["config"];
            let arm = r["arm"].as_str().unwrap();
            let warm = r["warmup"] == true;
            let rep = if warm {
                r["input_index"].as_u64().unwrap()
            } else {
                r["round"].as_u64().unwrap()
            };
            let id = c["id"].as_str().unwrap();
            let key = format!("{id}:{arm}:{warm}:{rep}");
            assert!(seen.insert(key.clone()));
            let status = if r["status"] == "ok" {
                success += 1;
                "succeeded"
            } else {
                failed += 1;
                if r["process_exit"] == 124
                    || r["error"].as_str().is_some_and(|e| e.contains("deadline"))
                {
                    "timeout"
                } else {
                    "failed"
                }
            };
            runs.push(json!({"run_id":key,"arm":arm,"instance_id":id,"seed":c["seed"].to_string(),"replicate":rep,"warmup":warm,"status":status,"outcome_class":if status=="succeeded"{"completed"}else{"not_completed"},"attempts":1,"resumed":0,"checkpoint":null,"measurements":{"counted":{"muls":r["query_work"]["muls"]},"wall_seconds":r["cold_ns"].as_str().map(|s|s.parse::<f64>().unwrap()/1e9),"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":r["query_work"]},"isolation":{"tier":"L0","contended":contended,"blockers":["No L2 admission; actual host wrapper stored","Local producer observation; no independent signed standing"]},"verification":{"status":"no_claim","verifier":"Native complete-set oracle in parent; producer-owned, no independent signature"},"exclusion":{"excluded":false,"rule":null},"artifacts":[pin(&path,"raw")],"started_at":null,"finished_at":null,"worker":"local CPU4","backend_ref":name}));
        }
    }
    let planned = p["sample_size"]["instances"].as_u64().unwrap()
        * p["arms"].as_array().unwrap().len() as u64
        * (p["sample_size"]["replicates"].as_u64().unwrap()
            + p["sample_size"]["warmups"].as_u64().unwrap());
    assert_eq!(
        runs.len() as u64,
        planned,
        "fixed-N shortfall, preserve raw and report instead of imputing"
    );
    let source = &p["arms"][0]["instrument"]["commit"];
    let arms:Vec<_>=p["arms"].as_array().unwrap().iter().map(|x|json!({"id":x["id"],"role":x["role"],"binary_sha256":x["instrument"]["binary_sha256"],"commit":x["instrument"]["commit"]})).collect();
    let analysis = Path::new(ROOT).join(p["analysis_plan"]["script"]["path"].as_str().unwrap());
    let hostpin = pin(&host, "host");
    files.push(hostpin.clone());
    let actual = runs.len();
    let e = json!({"schema":"experiment.export/v1","export_id":format!("{}-EXPORT",p["protocol_id"].as_str().unwrap()),"created":a[5],"protocol":{"path":protocol.strip_prefix(ROOT).unwrap().to_string_lossy(),"sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]},"producer":{"runtime":"codex","model":"GPT-6","session":"vieta-speedup-20261117","role":"executor"},"source":{"repo":"crypto","commit":source,"dirty":null},"backend":"local","host":{"path":hostpin["path"],"sha256":hostpin["sha256"],"class_id":"local-epyc9v74-kvm-cpu4","summary":{"cpu_model":"AMD EPYC9V74","logical_cpus":5,"memory_bytes":null,"kernel":"6.18.57","hypervisor":"KVM","isolation_tier_max":"L0"}},"arms":arms,"runs":runs,"flow":{"planned":actual,"attempted":actual,"completed":success,"not_completed":failed,"interrupted":0,"resumed":0,"excluded":[]},"files":files,"analysis":{"script":pin(&analysis,"analysis"),"blinded":false,"blinding_key_sha256":null,"outputs":[],"unmasked_at":null},"notes":[{"kind":"observation","by":"producer","text":"Query primitive counters are complete evaluator counts; fast field constructor/precomputation is inside cold clocks but its internal field operations are unpriced. No fully priced fast cold-count claim, L2 timing, independent signature or S3 availability. Original source/configs/raw outputs remain pinned."}],"reviews_dir":"reviews"});
    let out = Path::new(&a[4]);
    assert!(!out.exists());
    fs::write(out, serde_json::to_vec(&e).unwrap()).unwrap();
    println!("export {actual}/{planned}, success{success}, failed{failed}");
}
