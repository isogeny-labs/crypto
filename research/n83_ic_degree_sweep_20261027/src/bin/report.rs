//! Format recorded outputs only. Does not generate inputs or invoke a solver.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};

fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn write(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap();
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn artifact(base: &Path, path: &str, kind: &str) -> Value {
    let bytes = fs::read(base.join(path)).unwrap();
    json!({"path":path,"sha256":sha(&bytes),"bytes":bytes.len(),"kind":kind})
}
fn show(r: &Value) -> String {
    if r["status"] != "complete" {
        if r["exit_code"] == 14 {
            return "memory limit".into();
        }
        if r["status"] == "resource_limit" {
            return "time limit".into();
        }
        return r["status"].as_str().unwrap().to_string();
    }
    if r["mode"] == "top" {
        if let Some(d) = r["degree_of_regularity_top"].as_i64() {
            return d.to_string();
        }
        return format!("positive dimension ({})", r["top_dimension"]);
    }
    r["basis_max_degree"].to_string()
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 4, "report STUDY RAW REPORT_RELATIVE_PATH");
    let study = Path::new(&args[1]);
    let raw = Path::new(&args[2]);
    let report = study.join(&args[3]);
    fs::create_dir_all(&report).unwrap();
    let evidence = study.join("evidence");
    fs::create_dir_all(&evidence).unwrap();
    let p = read(&study.join("protocol-v2.json"));
    let rows = read(&raw.join("inputs/results.json"));
    let rows = rows.as_array().unwrap();
    assert_eq!(
        rows.len(),
        96,
        "Incomplete grid must be reported before producing a complete-grid export"
    );
    let mut cells = BTreeSet::new();
    for r in rows {
        let i = &r["input"];
        let family = i["family"].as_str().unwrap();
        let m = i["m"].as_u64().unwrap();
        assert!(["koblitz", "binary", "prime"].contains(&family));
        assert!((2..=9).contains(&m));
        assert_eq!(i["ell"].as_u64().unwrap(), 83 / m);
        assert!(["planted", "natural"].contains(&i["target_kind"].as_str().unwrap()));
        assert!(["top", "affine"].contains(&r["mode"].as_str().unwrap()));
        assert!(cells.insert((
            family.to_string(),
            m,
            i["target_kind"].to_string(),
            r["mode"].to_string()
        )));
        assert_eq!(r["solving_degree"], Value::Null);
        if r["status"] != "complete" {
            assert!(r["degree_of_regularity_top"].is_null() && r["basis_max_degree"].is_null());
        }
        let log = fs::read(
            raw.join("inputs")
                .join(i["id"].as_str().unwrap())
                .join(r["mode"].as_str().unwrap())
                .join("stdout.log"),
        )
        .unwrap();
        assert_eq!(sha(&log), r["log_sha256"].as_str().unwrap());
    }
    for (src, dst) in [
        ("inputs/results.json", "results.json"),
        ("inputs/inputs.json", "inputs.json"),
        ("inputs/prime-certificate.json", "prime-certificate.json"),
        ("host.txt", "host.txt"),
        ("singular-version.txt", "singular-version.txt"),
        ("binary.sha256", "binary.sha256"),
        ("source-revision.txt", "source-revision.txt"),
        ("controls.jsonl", "controls.jsonl"),
        ("manifest.sha256", "raw-manifest.sha256"),
    ] {
        fs::copy(raw.join(src), evidence.join(dst)).unwrap();
    }
    assert_eq!(read(&raw.join("protocol.json")), p);
    let binary = fs::read_to_string(evidence.join("binary.sha256")).unwrap();
    assert_eq!(
        binary.split_whitespace().next().unwrap(),
        p["arms"][0]["instrument"]["binary_sha256"]
            .as_str()
            .unwrap()
    );
    let controls: Vec<Value> = fs::read_to_string(evidence.join("controls.jsonl"))
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(controls.len(), 8);
    assert!(controls.iter().all(|r| r["status"] == "complete"));
    let request = read(&study.join("request-v2.json"));
    let receipt = read(&study.join("terminal-v2.json"));
    let pod = &receipt["pods"][0];
    let worker = pod["name"].as_str().unwrap();
    assert_eq!(
        pod["status"]["containerStatuses"][0]["imageID"],
        request["image"]
    );
    let source = fs::read_to_string(evidence.join("source-revision.txt")).unwrap();
    let host = fs::read_to_string(evidence.join("host.txt")).unwrap();
    let cpu = host
        .lines()
        .find_map(|l| l.strip_prefix("Model name:"))
        .map(str::trim);
    let logical = host
        .lines()
        .find_map(|l| l.strip_prefix("CPU(s):"))
        .and_then(|v| v.trim().parse::<u64>().ok());
    let mut runs = Vec::new();
    let mut observations = Vec::new();
    for r in rows {
        let i = &r["input"];
        let mode = r["mode"].as_str().unwrap();
        let log_path = raw
            .join("inputs")
            .join(i["id"].as_str().unwrap())
            .join(mode)
            .join("stdout.log");
        let log = fs::read_to_string(log_path).unwrap();
        let memory_limit = log.contains("Singular error: no more memory");
        if r["exit_code"] == 14 {
            assert!(memory_limit, "Exit 14 without memory diagnostic");
        }
        let reason = if r["status"] == "resource_limit" {
            "time_limit"
        } else if memory_limit {
            "memory_limit"
        } else {
            r["status"].as_str().unwrap()
        };
        let phase = if log.contains("SOLVER_END") {
            "post_solver"
        } else if log.contains("SOLVER_BEGIN") {
            "standard_basis"
        } else {
            "input_expansion_or_witness"
        };
        let status = match r["status"].as_str().unwrap() {
            "complete" => "succeeded",
            "resource_limit" => "timeout",
            "solver_error" => "failed",
            _ => "failed",
        };
        let mut counted = serde_json::Map::new();
        for k in [
            "degree_of_regularity_top",
            "basis_max_degree",
            "top_dimension",
            "basis_size",
        ] {
            if !r[k].is_null() {
                counted.insert(k.into(), r[k].clone());
            }
        }
        let id = format!("{}-{mode}", i["id"].as_str().unwrap());
        let instance = ((i["m"].as_u64().unwrap() - 2) * 2
            + u64::from(i["target_kind"] == "natural"))
        .to_string();
        runs.push(json!({"run_id":id,"arm":i["family"],"instance_id":instance,"seed":20261027,"replicate":u8::from(mode=="affine"),"warmup":false,"status":status,"outcome_class":if ["succeeded","failed"].contains(&status){"completed"}else{"not_completed"},"attempts":1,"resumed":0,"checkpoint":null,"measurements":{"counted":counted,"wall_seconds":r["diagnostic_wall_seconds"],"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":{}},"isolation":{"tier":"L0","contended":null,"blockers":["Ordinary Spot VM process; no timing qualification"]},"verification":{"status":if status=="succeeded"{"verified"}else{"no_claim"},"verifier":"Frozen producer COMPLETE/error checks, affine input reduction and planted witness; not independent replay"},"exclusion":{"excluded":false,"rule":null},"artifacts":[],"started_at":null,"finished_at":null,"worker":worker,"backend_ref":format!("{}/inputs/{}/{mode}/result.json",raw_uri(&pod["uid"]),i["id"].as_str().unwrap())}));
        observations.push(json!({"id":id,"family":i["family"],"m":i["m"],"ell":i["ell"],"n_vars":i["n_vars"],"target_kind":i["target_kind"],"mode":mode,"status":r["status"],"exit_code":r["exit_code"],"diagnostic_reason":reason,"memory_diagnostic":memory_limit,"last_observed_phase":phase,"degree_of_regularity_top":r["degree_of_regularity_top"],"top_dimension":r["top_dimension"],"basis_max_degree":r["basis_max_degree"],"solving_degree":null}));
    }
    write(&evidence.join("observations.json"), &json!(observations));
    let ncomplete = rows.iter().filter(|r| r["status"] == "complete").count();
    let nfinite = rows
        .iter()
        .filter(|r| !r["degree_of_regularity_top"].is_null())
        .count();
    let ntimeout = rows
        .iter()
        .filter(|r| r["status"] == "resource_limit")
        .count();
    let nmemory = rows.iter().filter(|r| r["exit_code"] == 14).count();
    let nfailed = runs
        .iter()
        .filter(|r| r["outcome_class"] == "completed")
        .count();
    let ninterrupted = runs.iter().filter(|r| r["status"] == "interrupted").count();
    let files = [
        ("evidence/results.json", "raw"),
        ("evidence/inputs.json", "raw"),
        ("evidence/observations.json", "summary"),
        ("evidence/raw-manifest.sha256", "other"),
        ("evidence/controls.jsonl", "raw"),
        ("evidence/singular-version.txt", "other"),
        ("request-v2.json", "receipt"),
        ("submission-v2.json", "receipt"),
        ("terminal-v2.json", "receipt"),
        ("evidence/storage.json", "receipt"),
        ("src/bin/report.rs", "analysis"),
    ]
    .map(|(path, kind)| artifact(study, path, kind));
    let export = json!({"schema":"experiment.export/v1","export_id":"N83-IC-DEGREE-M2-M9-20261027-V2","created":"2026-10-27","protocol":{"path":"protocol-v2.json","sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":2},"producer":{"runtime":"codex","model":null,"session":"n83-degree-m2-m9-20261027-v2","role":"producer"},"source":{"repo":"crypto","commit":source.trim(),"dirty":false},"backend":"isolab-k8s","host":{"path":"evidence/host.txt","sha256":sha(host.as_bytes()),"class_id":"eks-spot-L0","summary":{"cpu_model":cpu,"logical_cpus":logical,"memory_bytes":null,"kernel":host.lines().next(),"hypervisor":"Amazon EC2","isolation_tier_max":"L0","spot":true}},"arms":p["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"],"image_digest":request["image"]})).collect::<Vec<_>>(),"runs":runs,"flow":{"planned":96,"attempted":96,"completed":nfailed,"not_completed":96-nfailed,"interrupted":ninterrupted,"resumed":0,"excluded":[]},"files":files,"analysis":{"script":artifact(study,"src/main.rs","analysis"),"blinded":false,"blinding_key_sha256":null,"outputs":[artifact(study,"evidence/results.json","summary")],"unmasked_at":null},"notes":[{"kind":"observation","text":format!("96 recorded attempts; {ncomplete} completed computations, {nfinite} finite top-form degrees and {ntimeout} resource limits. Modes are different computations, not independent replicates. V1 startup failed before target data; receipt retained separately.")}],"reviews_dir":"reviews"});
    let mut export = export;
    export["notes"][0]["by"] = json!("producer");
    write(&study.join("experiment.export.json"), &export);
    let mut md=String::from("# N83 index-calculus degree sweep\n\nObservations only; this is a bounded decomposition-system run, not an evidence-review verdict or a full ECDLP result.\n\n");
    md+=&format!("All 96 computations were attempted. {ncomplete} completed; {nfinite} supplied a finite top-form degree; {ntimeout} reached the process time limit and {nmemory} exited with Singular's no-more-memory code 14. One time-limited attempt also printed a memory diagnostic; it remains classified by the timeout exit status.\n\n");
    md+="Balanced ell=floor(83/m). Candidate x-domain size is 2^ell; actual points must lift to the curve. Every cell has one planted and one natural target, with seed 20261027. Values below are observed exact degrees only when the solver completed. P/N means planted/natural.\n\n| m | ell | Koblitz top P/N | Binary top P/N | Prime top P/N |\n|---:|---:|---|---|---|\n";
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1120\" height=\"620\" viewBox=\"0 0 1120 620\"><rect width=\"1120\" height=\"620\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#142336\"><text x=\"35\" y=\"35\" font-size=\"24\">N83: recorded top-form regularity</text><text x=\"35\" y=\"62\" font-size=\"15\">One planted (P) and one natural (N) target per cell; blank degrees are not estimates</text>");
    for (j, family) in ["Koblitz", "Binary", "Prime"].iter().enumerate() {
        svg += &format!(
            "<text x=\"{}\" y=\"98\" font-size=\"20\">{family} (P / N)</text>",
            160 + j * 310
        );
    }
    for m in 2..=9 {
        md += &format!("| {m} | {} |", 83 / m);
        let y = 115 + (m - 2) * 51;
        svg+=&format!("<text x=\"35\" y=\"{}\" font-size=\"17\">m={m}</text><text x=\"35\" y=\"{}\" font-size=\"13\">ell={}</text>",y+20,y+37,83/m);
        for (j, family) in ["koblitz", "binary", "prime"].iter().enumerate() {
            let pair: Vec<_> = ["planted", "natural"]
                .iter()
                .map(|kind| {
                    rows.iter()
                        .find(|r| {
                            r["input"]["m"] == m
                                && r["input"]["family"] == *family
                                && r["input"]["target_kind"] == *kind
                                && r["mode"] == "top"
                        })
                        .unwrap()
                })
                .collect();
            md += &format!(" {} / {} |", show(pair[0]), show(pair[1]));
            let labels: Vec<_> = pair
                .iter()
                .map(|r| {
                    if r["status"] == "resource_limit" {
                        "limit".into()
                    } else {
                        show(r)
                    }
                })
                .collect();
            let color = if pair
                .iter()
                .all(|r| !r["degree_of_regularity_top"].is_null())
            {
                "#d5eee4"
            } else {
                "#f0e5d5"
            };
            svg+=&format!("<rect x=\"{}\" y=\"{y}\" width=\"295\" height=\"44\" rx=\"5\" fill=\"{color}\"/><text x=\"{}\" y=\"{}\" font-size=\"15\">{} / {}</text>",145+j*310,160+j*310,y+28,labels[0],labels[1]);
        }
        md += "\n";
    }
    svg+="<text x=\"35\" y=\"555\" font-size=\"15\">Unit: polynomial degree. Limit: 40 s process budget, 3 GiB virtual memory per worker.</text><text x=\"35\" y=\"580\" font-size=\"14\">Run: n83-degree-m2-m9-20261027-v2. Source: evidence/results.json. No statistical intervals.</text></g></svg>";
    fs::write(report.join("degree-grid.svg"), svg).unwrap();
    md+="\n## Affine output\n\nThe following values are maximum degrees in a completed final basis. They are not maximum intermediate solving degrees.\n\n| m | Koblitz P/N | Binary P/N | Prime P/N |\n|---:|---|---|---|\n";
    for m in 2..=9 {
        md += &format!("| {m} |");
        for family in ["koblitz", "binary", "prime"] {
            let v: Vec<_> = ["planted", "natural"]
                .iter()
                .map(|kind| {
                    show(
                        rows.iter()
                            .find(|r| {
                                r["input"]["m"] == m
                                    && r["input"]["family"] == family
                                    && r["input"]["target_kind"] == *kind
                                    && r["mode"] == "affine"
                            })
                            .unwrap(),
                    )
                })
                .collect();
            md += &format!(" {} / {} |", v[0], v[1]);
        }
        md += "\n";
    }
    md+="\n## Reproduction and scope\n\nFrozen design: ../../PROTOCOL.md and ../../protocol-v2.json. Image, argv and limits: ../../request-v2.json. Terminal pod/image identity: ../../terminal-v2.json. Unified export: ../../experiment.export.json. All raw paths and verified content hashes: ../../evidence/storage.json and ../../evidence/raw-manifest.sha256. Eight same-driver controls completed. Four native arithmetic/construction tests passed before freezing. No independent replay certificate is claimed.\n\nThe measured quantity is one plus the last nonzero graded degree of the quotient by the highest homogeneous components, where that quotient is zero-dimensional. It is representation dependent and is not the regularity of the saturated homogenized affine ideal. Positive dimension is reported explicitly. No timeout supplies a degree bound. Intermediate solving degree remains unmeasured. CPU user/system time and peak RSS were not recorded; wall times in the raw rows are diagnostic and invalid for performance claims.\n\nKoblitz uses y^2+xy=x^3+1 over GF(2^83), modulus z^83+z^45+z^2+z+1. General binary uses a=1, b=2^82+0x51b. Prime uses y^2=x^3+2x+3 and the Proth-certified prime in ../../evidence/prime-certificate.json. Binary coordinates use a polynomial-basis subspace; prime coordinates use a bit-encoded integer interval. Full-curve synthetic targets were generated; no subgroup attack is established, including for Koblitz. Roots of an S3 chain require point lifting and sign consistency before they become IC relations.\n\nMethods follow [Kosters and Yeo](https://arxiv.org/html/1503.08001) and the [Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf). The backend is Singular 4.3.1 std, degree reverse lexicographic order. The first EKS launch failed with a permissions error before input generation; its receipt and premeasurement correction are retained. This run preserves every requested n83 cell. Unresolved degrees remain outstanding research work.\n";
    fs::write(report.join("report.md"), md).unwrap();
    println!("96 unique rows, 8 controls and source/binary/image/log hashes checked; {ncomplete} complete; {nfinite} finite top degrees; {ntimeout} resource limits");
}
fn raw_uri(uid: &Value) -> String {
    format!(
        "s3://isolab-results-104186811093-us-east-2/runs/n83-degree-m2-m9-20261027-v2/{}",
        uid.as_str().unwrap()
    )
}
