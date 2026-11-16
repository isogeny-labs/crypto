//! Output-only formatter for the frozen fall certificates; never executes trials.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn save(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap()
}
fn artifact(base: &Path, path: &str, kind: &str) -> Value {
    let b = fs::read(base.join(path)).unwrap();
    json!({"path":path,"sha256":sha(&b),"bytes":b.len(),"kind":kind})
}
fn show(r: &Value) -> String {
    if let Some(d) = r["first_fall_degree"].as_u64() {
        d.to_string()
    } else if let Some(d) = r["first_fall_lower_bound"].as_u64() {
        format!(">={d} (bound)")
    } else {
        "unresolved".into()
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 3, "fall_report FALL_STUDY RAW");
    let base = Path::new(&args[1]);
    let raw = Path::new(&args[2]);
    let p = read(&base.join("protocol.json"));
    assert_eq!(p, read(&raw.join("protocol.json")));
    let data = read(&raw.join("certificates/results.json"));
    let rows = data.as_array().unwrap();
    assert_eq!(rows.len(), 48);
    let replays: Vec<Value> = fs::read_to_string(raw.join("replay.jsonl"))
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(replays.len(), 48);
    let replay_ids: BTreeSet<_> = replays
        .iter()
        .map(|r| {
            assert_eq!(r["verified"], true);
            r["input"].as_str().unwrap()
        })
        .collect();
    assert_eq!(replay_ids.len(), 48);
    let mut seen = BTreeSet::new();
    let mut runs = Vec::new();
    let terminal = read(&base.join("terminal.json"));
    assert_eq!(terminal["job"]["state"], "succeeded");
    let pod = &terminal["pods"][0];
    let worker = pod["name"].as_str().unwrap();
    let request = read(&base.join("request.json"));
    assert_eq!(
        pod["status"]["containerStatuses"][0]["imageID"],
        request["image"]
    );
    let source = fs::read_to_string(raw.join("source-revision.txt")).unwrap();
    assert_eq!(source.trim(), p["arms"][0]["instrument"]["commit"]);
    let binary = fs::read_to_string(raw.join("binary.sha256")).unwrap();
    assert_eq!(
        binary.split_whitespace().next().unwrap(),
        p["arms"][0]["instrument"]["binary_sha256"]
    );
    let controls = read(&raw.join("controls.json"));
    assert_eq!(controls["positive"]["first_fall_degree"], 2);
    for k in [
        "duplicate_negative",
        "homogeneous_negative",
        "zero_negative",
    ] {
        assert_eq!(controls[k]["new_lower_degree_dimension"], 0);
        assert!(controls[k]["first_fall_degree"].is_null());
    }
    let uri = format!(
        "s3://isolab-results-104186811093-us-east-2/runs/n83-first-fall-m2-m9-20261027-r3/{}",
        pod["uid"].as_str().unwrap()
    );
    for r in rows {
        let i = &r["input"];
        let id = i["id"].as_str().unwrap();
        let family = i["family"].as_str().unwrap();
        let m = i["m"].as_u64().unwrap();
        let kind = i["target_kind"].as_str().unwrap();
        assert!(["koblitz", "binary", "prime"].contains(&family));
        assert!((2..=9).contains(&m));
        assert_eq!(i["ell"], 83 / m);
        assert!(["planted", "natural"].contains(&kind));
        assert!(seen.insert((family, m, kind)));
        assert!(replay_ids.contains(id));
        assert_eq!(r, &read(&raw.join(format!("certificates/{id}.json"))));
        assert!(r["last_fall_degree"].is_null());
        let mut counted = serde_json::Map::new();
        for k in ["first_fall_degree", "first_fall_lower_bound"] {
            if !r[k].is_null() {
                counted.insert(k.into(), r[k].clone());
            }
        }
        runs.push(json!({"run_id":id,"arm":family,"instance_id":((m-2)*2+u64::from(kind=="natural")).to_string(),"seed":20261027,"replicate":0,"warmup":false,"status":"succeeded","outcome_class":"completed","attempts":1,"resumed":0,"checkpoint":null,"measurements":{"counted":counted,"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":{}},"isolation":{"tier":"L0","contended":null,"blockers":["Ordinary EKS Spot process; no timing qualification"]},"verification":{"status":"verified","verifier":"Explicit saved-equation witness or coefficient-minor replay in fresh process; not independent research review"},"exclusion":{"excluded":false,"rule":null},"artifacts":[],"started_at":null,"finished_at":null,"worker":worker,"backend_ref":format!("{uri}/certificates/{id}.json")}));
    }
    fs::create_dir_all(base.join("evidence")).unwrap();
    fs::create_dir_all(base.join("reports")).unwrap();
    for (src, dst) in [
        ("certificates/results.json", "results.json"),
        ("manifest.sha256", "raw-manifest.sha256"),
        ("controls.json", "controls.json"),
        ("host.txt", "host.txt"),
        ("binary.sha256", "binary.sha256"),
        ("source-revision.txt", "source-revision.txt"),
        ("producer.jsonl", "producer.jsonl"),
        ("replay.jsonl", "replay.jsonl"),
        ("input-list.sha256", "input-list.sha256"),
        ("input-list-verified.txt", "input-list-verified.txt"),
    ] {
        fs::copy(raw.join(src), base.join("evidence").join(dst)).unwrap();
    }
    let host = fs::read_to_string(raw.join("host.txt")).unwrap();
    let cpu = host
        .lines()
        .find_map(|l| l.strip_prefix("Model name:"))
        .map(str::trim);
    let logical = host
        .lines()
        .find_map(|l| l.strip_prefix("CPU(s):"))
        .and_then(|s| s.trim().parse::<u64>().ok());
    let exact = rows
        .iter()
        .filter(|r| !r["first_fall_degree"].is_null())
        .count();
    let files = [
        ("evidence/results.json", "raw"),
        ("evidence/controls.json", "raw"),
        ("evidence/replay.jsonl", "raw"),
        ("evidence/producer.jsonl", "raw"),
        ("evidence/raw-manifest.sha256", "other"),
        ("request.json", "receipt"),
        ("submission.json", "receipt"),
        ("terminal.json", "receipt"),
        ("../src/bin/fall_report.rs", "analysis"),
    ]
    .map(|(f, k)| artifact(base, f, k));
    let e = json!({"schema":"experiment.export/v1","export_id":"N83-FIRST-FALL-M2-M9-20261027-R3","created":"2026-10-27","protocol":{"path":"protocol.json","sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":1},"producer":{"runtime":"codex","model":null,"session":"n83-first-fall-m2-m9-20261027-r3","role":"producer"},"source":{"repo":"crypto","commit":source.trim(),"dirty":false},"backend":"isolab-k8s","host":{"path":"evidence/host.txt","sha256":sha(host.as_bytes()),"class_id":"eks-spot-L0","summary":{"cpu_model":cpu,"logical_cpus":logical,"memory_bytes":null,"kernel":host.lines().next(),"hypervisor":"Amazon EC2","isolation_tier_max":"L0"}},"arms":p["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"],"image_digest":request["image"]})).collect::<Vec<_>>(),"runs":runs,"flow":{"planned":48,"attempted":48,"completed":48,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]},"files":files,"analysis":{"script":artifact(base,"../src/bin/fall.rs","analysis"),"blinded":false,"blinding_key_sha256":null,"outputs":[artifact(base,"evidence/results.json","summary")],"unmasked_at":null},"notes":[{"kind":"observation","by":"producer","text":format!("48 certificate computations and fresh-process replays completed; {exact} attained first-fall values; {} lower bounds only. No last fall or regularity value follows. Active full goal remains unresolved for bound-only cells.",48-exact)}],"reviews_dir":"reviews"});
    save(&base.join("experiment.export.json"), &e);
    let mut md=format!("# N83 first-fall certificate sweep\n\n{exact} exact first-fall certificates; {} bound-only results; all 48 certificate computations and fresh-process replays completed. Last fall remains unmeasured. These are observations and checked algebraic certificates, not an independent research review.\n\nBalanced ell=floor(83/m), candidate x-domain 2^ell. P/N denotes planted/natural. A bound is not an attained degree.\n\n| m | ell | Koblitz P/N | Binary P/N | Prime P/N |\n|---:|---:|---|---|---|\n",48-exact);
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"550\"><rect width=\"1000\" height=\"550\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#193349\"><text x=\"30\" y=\"35\" font-size=\"24\">N83 first fall: exact values and lower bounds</text><text x=\"30\" y=\"63\" font-size=\"14\">Planted / natural targets; all saved input equations checked</text>");
    for (j, f) in ["Koblitz", "Binary", "Prime"].iter().enumerate() {
        svg += &format!(
            "<text x=\"{}\" y=\"98\" font-size=\"20\">{f}</text>",
            140 + j * 280
        );
    }
    for m in 2..=9 {
        md += &format!("| {m} | {} |", 83 / m);
        let y = 115 + (m - 2) * 44;
        svg += &format!(
            "<text x=\"30\" y=\"{}\" font-size=\"17\">m={m}</text>",
            y + 25
        );
        for (j, f) in ["koblitz", "binary", "prime"].iter().enumerate() {
            let pair: Vec<_> = ["planted", "natural"]
                .iter()
                .map(|k| {
                    rows.iter()
                        .find(|r| {
                            r["input"]["family"] == *f
                                && r["input"]["m"] == m
                                && r["input"]["target_kind"] == *k
                        })
                        .unwrap()
                })
                .collect();
            let labels = format!("{} / {}", show(pair[0]), show(pair[1]));
            md += &format!(" {labels} |");
            let color = if pair.iter().all(|r| !r["first_fall_degree"].is_null()) {
                "#d5eee4"
            } else {
                "#f6e8c8"
            };
            let escaped = labels.replace('>', "&gt;");
            svg+=&format!("<rect x=\"{}\" y=\"{y}\" width=\"265\" height=\"37\" rx=\"4\" fill=\"{color}\"/><text x=\"{}\" y=\"{}\" font-size=\"15\">{escaped}</text>",125+j*280,140+j*280,y+25);
        }
        md += "\n";
    }
    svg+="<text x=\"30\" y=\"495\" font-size=\"14\">Green: attained degree with explicit witness. Amber: exclusion bound; exact value unresolved.</text><text x=\"30\" y=\"522\" font-size=\"13\">Source: evidence/results.json. Unit: total polynomial degree. No timing or statistical claim.</text></g></svg>";
    md+="\nBinary method: no nonzero input generator below degree two; a constant combination of quadratic rows produces a nonzero linear polynomial. Both pivot orders agree and the saved combination is reconstructed from the original equations in a separate process. Prime method: the selector ideal has no degree falls and a nonsingular degree-four coefficient minor excludes additional falls through degree four. Last fall and regularity do not follow from these certificates.\n\nAll exact values depend on the specified S3-chain presentation. The binary rank difference counts lower-degree vectors in the original quadratic span, not the full closed V(F,2). Candidate domains have not been exhaustively lifted to curve points.\n\nControls: degree-two-to-linear positive; duplicate, homogeneous and zero-row negatives; seven native unit tests. Source, binary, input-list and per-system hashes are pinned in protocol.json. Same-code fresh-process replay is not an independent scientific review. Wall times are invalid for performance claims.\n\nEvidence: experiment.export.json, evidence/results.json, evidence/replay.jsonl, terminal.json and DESIGN.md. Earlier regularity failures are preserved in the parent study. Exact prime first/last values remain part of the active goal; lower bounds do not complete it.\n\nReferences: [Kosters and Yeo](https://arxiv.org/html/1503.08001); [Caminata and Gorla](https://arxiv.org/html/2112.05579); [Kousidis and Wiemers](https://arxiv.org/html/1906.05594).\n";
    fs::write(base.join("reports/report.md"), md).unwrap();
    fs::write(base.join("reports/degree-grid.svg"), svg).unwrap();
    println!(
        "48 certificates, {exact} attained degrees, {} bounds, 48 fresh-process replays",
        48 - exact
    );
}
