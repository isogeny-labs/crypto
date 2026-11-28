//! Post-call metadata only. Never invokes field arithmetic or changes frozen inputs.
mod controls;
mod provenance;
mod table;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::HashSet, fs, path::Path};
const ROOT: &str = "research/vieta_coset_rust_20261115/speed-20261117";
fn load(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn pin(path: &str, kind: &str) -> Value {
    let b = fs::read(path).unwrap();
    json!({"path":path.strip_prefix(format!("{ROOT}/").as_str()).unwrap_or(path),"sha256":format!("{:x}",Sha256::digest(&b)),"bytes":b.len(),"kind":kind})
}
fn write_new(path: impl AsRef<Path>, data: &[u8]) {
    use std::io::Write;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap()
        .write_all(data)
        .unwrap();
}
fn export(panel: &str, created: &str) {
    assert!(["preflight", "main"].contains(&panel));
    let protocol_path = format!("{ROOT}/protocol-{panel}.json");
    let p = load(&protocol_path);
    let host_path = format!("{ROOT}/run/{panel}-isolation.jsonl");
    let host: Value = fs::read_to_string(&host_path)
        .unwrap()
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .next_back()
        .unwrap();
    let contended = &host["run"]["contended"];
    let mut dirs: Vec<_> = fs::read_dir(format!("{ROOT}/configs/{panel}"))
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect();
    dirs.sort();
    let (mut runs, mut files) = (vec![], vec![]);
    let (mut completed, mut failed) = (0, 0);
    let mut cells = HashSet::new();
    for config in dirs {
        let c = load(&config);
        let id = config.file_stem().unwrap().to_str().unwrap();
        let instance = p["randomization"]["seed_derivation"]["seeds"]
            .as_array()
            .unwrap()
            .iter()
            .position(|v| v.as_str() == Some(c["seed"].to_string().as_str()))
            .unwrap()
            .to_string();
        let raw = format!("{ROOT}/run/{panel}/{id}");
        if !Path::new(&raw).exists() {
            continue;
        }
        let mut paths: Vec<_> = fs::read_dir(&raw)
            .unwrap()
            .map(|x| x.unwrap().path())
            .collect();
        paths.sort();
        for path in paths {
            if path.extension().is_none_or(|x| x != "json") {
                continue;
            }
            let path = path.to_str().unwrap();
            if !Path::new(path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("cell-")
            {
                files.push(pin(path, "raw"));
                continue;
            }
            let r = load(path);
            let warm = r["warmup"].as_bool().unwrap();
            let rep = if warm {
                r["input_index"].as_u64().unwrap()
            } else {
                r["round"].as_u64().unwrap()
            };
            let arm = r["arm"].as_str().unwrap();
            let status = if r["status"] == "ok" {
                completed += 1;
                "succeeded"
            } else {
                failed += 1;
                if r["process_exit"] == 124
                    || r["error"]
                        .as_str()
                        .is_some_and(|e| e.contains("Deadline") || e.contains("deadline"))
                {
                    "timeout"
                } else {
                    "failed"
                }
            };
            let key = format!("{id}:{arm}:{warm}:{rep}");
            assert!(cells.insert(key.clone()), "duplicate raw cell");
            let seconds = r["cold_ns"]
                .as_str()
                .map(|n| n.parse::<f64>().unwrap() / 1e9);
            let artifacts = vec![pin(path, "raw")];
            runs.push(json!({"run_id":format!("{panel}:{key}"),"arm":arm,"instance_id":instance,"seed":c["seed"].to_string(),"replicate":rep,"warmup":warm,"status":status,"outcome_class":if status=="succeeded"{"completed"}else{"not_completed"},"attempts":1,"resumed":0,"checkpoint":null,"measurements":{"counted":{"muls":r["total_work"]["muls"],"relations":r["relations"].as_array().map(|x|x.len())},"wall_seconds":seconds,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":r["total_work"]},"isolation":{"tier":"L0","contended":contended,"blockers":["PSI unavailable","unmovable host threads","producer-owned diagnostic clocks belowL2"]},"verification":{"status":"no_claim","verifier":"Complete-set scalar replay; producer-owned, independent signed standing pending"},"exclusion":{"excluded":false,"rule":null},"artifacts":artifacts,"started_at":null,"finished_at":null,"worker":"local CPU4","backend_ref":path}));
        }
    }
    let actual = runs.len();
    let planned = p["sample_size"]["instances"].as_u64().unwrap()
        * p["arms"].as_array().unwrap().len() as u64
        * (p["sample_size"]["replicates"].as_u64().unwrap()
            + p["sample_size"]["warmups"].as_u64().unwrap());
    assert!(actual as u64 <= planned);
    let source = &p["arms"][0]["instrument"]["commit"];
    let original_record = load("benchmarks/records/20261117T0418Z-vieta-half-gcd-cold-speed.json");
    let dirty = &original_record["commit_dirty"];
    let arms:Vec<_>=p["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":source})).collect();
    files.push(pin(&host_path, "host"));
    let analysis = pin(&format!("{ROOT}/native/src/main.rs"), "analysis");
    let e = json!({"schema":"experiment.export/v1","export_id":format!("VIETA-SPEED-{}-20261117",panel.to_uppercase()),"created":created,"protocol":{"path":format!("protocol-{panel}.json"),"sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]},"producer":{"runtime":"codex","model":"GPT-6","session":"vieta-speed-20261117","role":"executor"},"source":{"repo":"crypto","commit":source,"dirty":dirty},"backend":"local","host":{"path":format!("run/{panel}-isolation.jsonl"),"sha256":files.last().unwrap()["sha256"],"class_id":"local-epyc9v74-kvm-5vcpu-cpu4","summary":{"cpu_model":"AMD EPYC 9V74","logical_cpus":5,"memory_bytes":null,"kernel":"6.18.57","hypervisor":"KVM","isolation_tier_max":"L0"}},"arms":arms,"runs":runs,"flow":{"planned":actual,"attempted":actual,"completed":completed,"not_completed":failed,"interrupted":0,"resumed":0,"excluded":[]},"files":files,"analysis":{"script":analysis,"blinded":false,"blinding_key_sha256":null,"outputs":[pin(&format!("{ROOT}/run/{}summary.json",if panel=="main"{""}else{"preflight-"}),"summary")],"unmasked_at":null},"notes":[{"kind":"observation","by":"producer","text":format!("Frozen fixed-N plan {planned} calls; {actual} actually attempted, {} missing; no imputation or retry. Warmups/failures remain. Frozen raw files carry both query/cold phases and populations; this export's scalar counted/wall fields are complete cold cost. Canonical explicit-list seed table conflicts with the prepublished per-configuration seed rule; actual raw seeds are retained and check-export failures remain. Instance indices follow the frozen configuration-list order; physical IDs stay in raw paths. Local diagnostic clocks only; Git-only custody authorized, no S3 availability claim, no independent signed standing. Source dirty flag copies the actual original benchmark record rather than inferring it from later metadata edits; exact measured files are pinned.",planned-actual as u64)}],"reviews_dir":"reviews"});
    write_new(
        format!("{ROOT}/export-{panel}.json"),
        &serde_json::to_vec(&e).unwrap(),
    );
    println!("exported {actual}/{planned}; completed={completed}, failed={failed}");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("export") => export(&a[2], &a[3]),
        Some("provenance") => {
            let v = provenance::audit(ROOT);
            write_new(
                format!("{ROOT}/run/seed-provenance-audit.json"),
                &serde_json::to_vec_pretty(&v).unwrap(),
            );
            println!("Published config hashes/scope match; canonical seed-table failure remains.");
        }
        Some("table") => {
            let s = load(format!("{ROOT}/run/summary.json"));
            let c = load(format!("{ROOT}/run/control-audit.json"));
            write_new(
                format!("{ROOT}/paired-costs.tsv"),
                table::tsv(&s).as_bytes(),
            );
            write_new(
                format!("{ROOT}/PAIRED_COSTS.md"),
                table::markdown(&s, &c).as_bytes(),
            );
        }
        Some("controls") => {
            let s = load(format!("{ROOT}/run/summary.json"));
            let v = controls::audit(&s);
            write_new(
                format!("{ROOT}/run/control-audit.json"),
                &serde_json::to_vec_pretty(&v).unwrap(),
            );
            println!("{}", v);
        }
        _ => panic!("usage: metadata export preflight|main UTC_TIMESTAMP"),
    }
}
