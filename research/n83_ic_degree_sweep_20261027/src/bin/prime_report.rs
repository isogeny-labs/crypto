//! Output-only R4 formatter. Prime lower bounds are proof-assisted, not matrix measurements.
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
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 3);
    let base = Path::new(&a[1]);
    let raw = Path::new(&a[2]);
    let p = read(&base.join("protocol.json"));
    assert_eq!(p, read(&raw.join("protocol.json")));
    let data = read(&raw.join("certificates/results.json"));
    let rows = data.as_array().unwrap();
    assert_eq!(rows.len(), 16);
    let r3bytes = fs::read(raw.join("r3-results.json")).unwrap();
    assert_eq!(sha(&r3bytes), p["extensions"]["r3_results_sha256"]);
    let r3: Value = serde_json::from_slice(&r3bytes).unwrap();
    assert_eq!(r3.as_array().unwrap().len(), 48);
    let receipt = read(&base.join("terminal.json"));
    assert_eq!(receipt["job"]["state"], "succeeded");
    let pod = &receipt["pods"][0];
    let request = read(&base.join("request.json"));
    assert_eq!(
        pod["status"]["containerStatuses"][0]["imageID"],
        request["image"]
    );
    let worker = pod["name"].as_str().unwrap();
    let uri = format!(
        "s3://isolab-results-104186811093-us-east-2/runs/n83-prime-first-fall-m2-m9-20261027-r4/{}",
        pod["uid"].as_str().unwrap()
    );
    let replay: Vec<Value> = fs::read_to_string(raw.join("replay.jsonl"))
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(replay.len(), 16);
    let replay_ids: BTreeSet<_> = replay
        .iter()
        .map(|r| {
            assert_eq!(r["replayed"], true);
            r["input"].as_str().unwrap()
        })
        .collect();
    assert_eq!(replay_ids.len(), 16);
    let old_replay: Vec<Value> = fs::read_to_string(raw.join("producer.log"))
        .unwrap()
        .lines()
        .filter(|l| l.starts_with('{'))
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(old_replay.len(), 16);
    assert!(old_replay.iter().all(|r| r["verified"] == true));
    let controls = read(&raw.join("controls.json"));
    assert_eq!(controls["all_passed"], true);
    assert_eq!(
        controls["exact_small_matrix_controls"]
            .as_array()
            .unwrap()
            .len(),
        5
    );
    for c in controls["exact_small_matrix_controls"].as_array().unwrap() {
        assert_eq!(c["measured_first_nontrivial_syzygy"], c["predicted"]);
    }
    let source = fs::read_to_string(raw.join("source-revision.txt")).unwrap();
    assert_eq!(source.trim(), p["arms"][0]["instrument"]["commit"]);
    let binary = fs::read_to_string(raw.join("binary.sha256")).unwrap();
    assert_eq!(
        binary.split_whitespace().next().unwrap(),
        p["arms"][0]["instrument"]["binary_sha256"]
    );
    assert_eq!(
        binary
            .lines()
            .nth(1)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap(),
        p["extensions"]["r3_verifier_binary_sha256"]
    );
    let mut runs = Vec::new();
    let mut cells = BTreeSet::new();
    for r in rows {
        let i = &r["input"];
        let id = i["id"].as_str().unwrap();
        let m = i["m"].as_u64().unwrap();
        let kind = i["target_kind"].as_str().unwrap();
        assert_eq!(i["family"], "prime");
        assert!((2..=9).contains(&m));
        assert!(["planted", "natural"].contains(&kind));
        assert!(cells.insert((m, kind)));
        assert!(replay_ids.contains(id));
        assert_eq!(r, &read(&raw.join(format!("certificates/{id}.json"))));
        assert_eq!(r["lower_bound_is_full_matrix_measurement"], false);
        assert!(r["first_new_affine_fall_degree"].is_null() && r["last_fall_degree"].is_null());
        assert_eq!(r["affine_nonzero_product"]["nonzero"], true);
        runs.push(json!({"run_id":id,"arm":format!("prime_{kind}"),"instance_id":(m-2).to_string(),"seed":20261027,"replicate":0,"warmup":false,"status":"succeeded","outcome_class":"completed","attempts":1,"resumed":0,"checkpoint":null,"measurements":{"counted":{"first_fall_degree":r["first_fall_degree"],"affine_product_degree_upper_bound":r["affine_product_degree_upper_bound"]},"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":{}},"isolation":{"tier":"L0","contended":null,"blockers":["Ordinary Spot; no timing qualification"]},"verification":{"status":"verified","verifier":"Pinned original-presentation replay and fresh-process factored witness/hypothesis replay; written lower-bound derivation, full Lean theorem open; not independent research review"},"exclusion":{"excluded":false,"rule":null},"artifacts":[],"started_at":null,"finished_at":null,"worker":worker,"backend_ref":format!("{uri}/certificates/{id}.json")}));
    }
    fs::create_dir_all(base.join("evidence")).unwrap();
    fs::create_dir_all(base.join("reports")).unwrap();
    for (src, dst) in [
        ("certificates/results.json", "results.json"),
        ("r3-results.json", "r3-results.json"),
        ("manifest.sha256", "raw-manifest.sha256"),
        ("controls.json", "controls.json"),
        ("host.txt", "host.txt"),
        ("binary.sha256", "binary.sha256"),
        ("source-revision.txt", "source-revision.txt"),
        ("producer.log", "producer.log"),
        ("replay.jsonl", "replay.jsonl"),
        ("r3-results-verified.txt", "r3-results-verified.txt"),
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
    let files = [
        ("evidence/results.json", "raw"),
        ("evidence/r3-results.json", "raw"),
        ("evidence/controls.json", "raw"),
        ("evidence/replay.jsonl", "raw"),
        ("evidence/producer.log", "raw"),
        ("evidence/raw-manifest.sha256", "other"),
        ("request.json", "receipt"),
        ("submission.json", "receipt"),
        ("terminal.json", "receipt"),
        ("ARGUMENT.md", "other"),
        ("PROOF_MAP.md", "other"),
        ("../../src/bin/prime_report.rs", "analysis"),
    ]
    .map(|(f, k)| artifact(base, f, k));
    let export = json!({"schema":"experiment.export/v1","export_id":"N83-PRIME-LEADING-FIRST-FALL-20261027-R4","created":"2026-10-27","protocol":{"path":"protocol.json","sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":1},"producer":{"runtime":"codex","model":null,"session":"n83-prime-first-fall-m2-m9-20261027-r4","role":"producer"},"source":{"repo":"crypto","commit":source.trim(),"dirty":false},"backend":"isolab-k8s","host":{"path":"evidence/host.txt","sha256":sha(host.as_bytes()),"class_id":"eks-spot-L0","summary":{"cpu_model":cpu,"logical_cpus":logical,"memory_bytes":null,"kernel":host.lines().next(),"hypervisor":"Amazon EC2","isolation_tier_max":"L0"}},"arms":p["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"],"image_digest":request["image"]})).collect::<Vec<_>>(),"runs":runs,"flow":{"planned":16,"attempted":16,"completed":16,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]},"files":files,"analysis":{"script":artifact(base,"../../src/bin/prime_fall.rs","analysis"),"blinded":false,"blinding_key_sha256":null,"outputs":[artifact(base,"evidence/results.json","summary")],"unmasked_at":null},"notes":[{"kind":"observation","by":"producer","text":"All16 prime certificates, original-presentation replays and new fresh-process replays completed. Five exact small-matrix controls passed. Values are standard leading-syzygy first falls with proof-assisted lower bounds; no full n83 rank measurement. Earliest NEW affine fall and last fall remain null. Full Lean formalization open; one structural dependency checked without axioms."}],"reviews_dir":"reviews"});
    save(&base.join("experiment.export.json"), &export);
    let mut md=String::from("# N83 standard first-fall grid\n\nAll requested m=2..9 cells have a standard leading-syzygy first-fall value under the stated S3-chain presentation. Binary/Koblitz values come from direct quadratic elimination and explicit nonzero linear witnesses (R3). Prime values use the written Lefschetz/filtered-Koszul lower bound plus native-checked factored witnesses (R4); they are not full n83 matrix-rank measurements. Full Lean formalization and independent research review remain open.\n\n| m | ell=floor(83/m) | Candidate x-domain | Koblitz P/N | Binary P/N | Prime P/N (proof-assisted) |\n|---:|---:|---|---|---|---|\n");
    let mut combined = Vec::new();
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1050\" height=\"570\"><rect width=\"1050\" height=\"570\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#193349\"><text x=\"30\" y=\"35\" font-size=\"24\">N83 standard first-fall degree</text><text x=\"30\" y=\"62\" font-size=\"14\">Planted / natural targets; same balanced input domains</text>");
    for (j, f) in ["Koblitz", "Binary", "Prime (proof-assisted)"]
        .iter()
        .enumerate()
    {
        svg += &format!(
            "<text x=\"{}\" y=\"98\" font-size=\"18\">{f}</text>",
            155 + j * 285
        );
    }
    for m in 2..=9 {
        let ell = 83 / m;
        md += &format!("| {m} | {ell} | 2^{ell} | ");
        let y = 115 + (m - 2) * 43;
        svg+=&format!("<text x=\"30\" y=\"{}\" font-size=\"17\">m={m}</text><text x=\"30\" y=\"{}\" font-size=\"12\">ell={ell}</text>",y+18,y+33);
        for (j, family) in ["koblitz", "binary", "prime"].iter().enumerate() {
            let values:Vec<_>=["planted","natural"].iter().map(|kind|{let list=if *family=="prime"{rows}else{r3.as_array().unwrap()};let r=list.iter().find(|r|r["input"]["m"]==m&&r["input"]["family"]==*family&&r["input"]["target_kind"]==*kind).unwrap();combined.push(json!({"m":m,"ell":ell,"family":family,"target_kind":kind,"first_fall_degree":r["first_fall_degree"],"method":if *family=="prime"{"proof_assisted"}else{"direct_elimination"},"earliest_new_affine_fall_degree":if *family=="prime"{Value::Null}else{r["first_fall_degree"].clone()},"last_fall_degree":null}));r["first_fall_degree"].as_u64().unwrap()}).collect();
            let label = format!("{} / {}", values[0], values[1]);
            md += &format!("{label} | ");
            let color = if *family == "prime" {
                "#dfe7fa"
            } else {
                "#d5eee4"
            };
            svg+=&format!("<rect x=\"{}\" y=\"{y}\" width=\"270\" height=\"37\" rx=\"4\" fill=\"{color}\"/><text x=\"{}\" y=\"{}\" font-size=\"17\">{label}</text>",140+j*285,155+j*285,y+25);
        }
        md += "\n";
    }
    svg+="<text x=\"30\" y=\"495\" font-size=\"14\">Green: direct degree-two witness. Blue: derived lower bound plus checked factored witness.</text><text x=\"30\" y=\"520\" font-size=\"14\">Full Lean theorem open. First fall does not determine last fall, solving degree or regularity.</text><text x=\"30\" y=\"546\" font-size=\"12\">Sources: R3 and R4 evidence/results.json; 48 original inputs. Unit: total polynomial degree.</text></g></svg>";
    md+="\nPrime result: d_ff=floor(ell/2)+4 under the standard definition as the first nontrivial leading-form syzygy modulo Koszul. The explicit multiplier is a product of differences of paired normalized selector variables. Its product with the endpoint leading quartic vanishes, and its affine product is nonzero and has lower degree. The native certificate checks this for each original target. The lower-bound argument uses Phuong–Tran Theorem2.7 and a finite filtered Koszul complex whose leading operators act on disjoint tensor factors; see ARGUMENT.md.\n\nThis is distinct from the earliest NEW affine degree fall. The prime affine product's novelty in V(F,d-1) is not established. That quantity, all last-fall degrees and regularity remain unresolved. R3 binary value2 certifies both notions. The original96 regularity attempts remain resource-limited; these new results do not turn them into completed regularity measurements.\n\nValidation: five explicit small-matrix controls over65537, three native R4 tests, all16 original-presentation replays and16 new certificate replays, pinned source/binary/image and per-input hashes. R3 independently records32 binary witnesses and48 replays. Same-code replay is not independent scientific review. Lean4.34.1 checks one structural upper-bound dependency without axioms; PROOF_MAP.md lists missing quotient/Koszul/Lefschetz bridges. No complete formal theorem is claimed.\n\nFrozen R4 protocol6e12ffdda6e1610783f0e3d0caea595387737eb798900659f3466d50edcf79b7. Source0c9543a2b5a95266792dadf15a921ac24a779bf0. Exact job request, immutable image, terminal receipt and raw S3 location are preserved beside the unified export. Wall times are diagnostic. No statistical or ECDLP performance claim.\n\nPrimary references: [Caminata–Gorla, Definition1.3](https://arxiv.org/html/2112.05579); [Phuong–Tran, Theorem2.7](https://arxiv.org/html/2211.13548); [Kosters–Yeo, binary trace falls](https://arxiv.org/html/1503.08001).\n";
    save(&base.join("evidence/combined-grid.json"), &json!(combined));
    fs::write(base.join("reports/report.md"), md).unwrap();
    fs::write(base.join("reports/degree-grid.svg"), svg).unwrap();
    println!("16 R4 certificates verified; combined48-row first-fall grid generated with method and definition labels");
}
