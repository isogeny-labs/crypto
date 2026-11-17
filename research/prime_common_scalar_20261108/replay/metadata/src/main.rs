use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{env, fs, path::Path};
fn hash(b: &[u8]) -> String {
    Sha256::digest(b)
        .iter()
        .map(|v| format!("{v:02x}"))
        .collect()
}
fn read(p: &str) -> Result<Value, String> {
    serde_json::from_slice(&fs::read(p).map_err(|e| format!("{p}: {e}"))?)
        .map_err(|e| e.to_string())
}
fn write(p: &str, v: &Value) -> Result<(), String> {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).map_err(|e| e.to_string())
}
fn file(p: &str) -> Result<Value, String> {
    let b = fs::read(p).map_err(|e| format!("{p}: {e}"))?;
    Ok(json!({"path":p,"sha256":hash(&b),"bytes":b.len()}))
}
fn text(p: &str) -> Result<String, String> {
    Ok(fs::read_to_string(p)
        .map_err(|e| e.to_string())?
        .trim()
        .into())
}
fn require(ok: bool, s: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(s.into())
    }
}
fn canonical(p: &Value) -> String {
    let mut q = p.clone();
    q.as_object_mut().unwrap().remove("freeze");
    hash(&serde_json::to_vec(&q).unwrap())
}
fn pinned(v: &Value) -> Result<(), String> {
    let p = v["path"].as_str().ok_or("artifact path")?;
    require(
        file(p)?["sha256"] == v["sha256"],
        &format!("artifact SHA mismatch: {p}"),
    )
}
fn export(root: &str, utc: &str, out: &str) -> Result<(), String> {
    let p = read(&format!("{root}/protocol.json"))?;
    require(
        p["freeze"]["status"] == "frozen" && canonical(&p) == p["freeze"]["protocol_sha256"],
        "freeze identity",
    )?;
    let s = read(&format!("{root}/summary.json"))?;
    require(
        s["rows_recounted"] == 366 && s["byte_identical"] == true,
        "native recount outcome",
    )?;
    let mut runs = vec![];
    let mut files = vec![];
    let mut data = vec![];
    let mut bars = String::new();
    for run in 0..2 {
        let base = format!("{root}/runs/{run}");
        require(
            text(&format!("{base}/exit-code.txt"))? == "0",
            "native process nonzero",
        )?;
        let raw = fs::read_to_string(format!("{base}/panel.tsv")).map_err(|e| e.to_string())?;
        let mut positive = 0;
        let mut controls = 0;
        let mut accepted = 0;
        for line in raw.lines().skip(1) {
            let c: Vec<_> = line.split('\t').collect();
            require(c.len() == 13, "raw TSV shape")?;
            if c[0] == "case" {
                positive += 1
            } else {
                controls += 1
            }
            if c[12] == "1" {
                accepted += 1
            }
        }
        require(
            positive == 180 && controls == 3 && accepted == 183,
            "complete accepted record census",
        )?;
        let receipt = json!({"schema":"native-replay-execution/v1","run_id":run,"started_at":text(&format!("{base}/started-at.txt"))?,"finished_at":text(&format!("{base}/finished-at.txt"))?,"exit_code":0,"cpu_affinity":text(&format!("{base}/cpu-affinity.txt"))?,"address_space_limit_bytes":2147483648_u64,"timeout_seconds":600,"output_limit_bytes":33554432_u64,"retries":0,"checkpoint":"owner-authorized committed/pushed git origin","wall_valid":false,"standing":"unsigned producer receipt"});
        write(&format!("{base}/receipt.json"), &receipt)?;
        let artifacts = vec![
            file(&format!("{base}/panel.tsv"))?,
            file(&format!("{base}/stderr.log"))?,
            file(&format!("{base}/receipt.json"))?,
            file(&format!("{base}/exit-code.txt"))?,
        ];
        files.extend(artifacts.clone());
        let arm = if run == 0 { "native-census" } else { "replay" };
        runs.push(json!({"run_id":format!("{arm}/0"),"arm":arm,"instance_id":"0","seed":0,"replicate":0,"warmup":false,"status":"succeeded","outcome_class":"completed","attempts":1,"resumed":0,"checkpoint":null,"measurements":{"counted":{"rows_completed":183},"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":{}},"isolation":{"tier":"L0","contended":null,"blockers":["CPU affinity set; host co-tenants not isolated; exact correctness only"]},"verification":{"status":"no_claim","verifier":"Frozen producer C++ GMP recount; no independent signature"},"exclusion":{"excluded":false,"rule":null},"artifacts":artifacts,"started_at":receipt["started_at"],"finished_at":receipt["finished_at"],"worker":"local-codex","backend_ref":format!("local-native-{run}")}));
        data.push(json!({"execution":run,"positive_correct":positive,"positive_planned":180,"defects_detected":controls,"defects_planned":3,"accepted_rows":accepted,"raw_sha256":file(&format!("{base}/panel.tsv"))?["sha256"]}));
        for (j, label, n) in [
            (0, "Recovery cases", positive),
            (1, "Defective controls", controls),
        ] {
            let y = 90 + run * 145 + j * 55;
            bars+=&format!("<text x=\"20\" y=\"{}\">Run {run}: {label}</text><rect x=\"235\" y=\"{}\" width=\"400\" height=\"23\" fill=\"#287850\"/><text x=\"650\" y=\"{}\">{n}/{} (100%)</text>",y+17,y,y+17,if j==0{180}else{3});
        }
    }
    require(
        read(&format!("{root}/runs/0/receipt.json"))?["exit_code"] == 0
            && text(&format!("{root}/runs/analysis-exit-code.txt"))? == "0",
        "execution/analysis exit",
    )?;
    require(
        fs::read(format!("{root}/runs/0/panel.tsv")).unwrap()
            == fs::read(format!("{root}/runs/1/panel.tsv")).unwrap(),
        "raw A/A bytes",
    )?;
    fs::create_dir_all(format!("{root}/reports/native-20261109")).map_err(|e| e.to_string())?;
    write(
        &format!("{root}/reports/native-20261109/counts.json"),
        &json!({"schema":"native-census-plot/v1","protocol_sha256":p["freeze"]["protocol_sha256"],"metric":"exact accepted fraction within complete fixed census; no statistical uncertainty","executions":data}),
    )?;
    fs::write(format!("{root}/reports/native-20261109/census.svg"),format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"900\" height=\"470\" viewBox=\"0 0 900 470\"><rect width=\"900\" height=\"470\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"16\" fill=\"#17252a\"><text x=\"20\" y=\"35\" font-size=\"23\">Native Hermite recovery: complete frozen census</text>{bars}<text x=\"235\" y=\"397\">0%</text><text x=\"595\" y=\"397\">100%</text><text x=\"20\" y=\"430\">2 physical executions; 366 exact records; byte-identical replay.</text><text x=\"20\" y=\"457\">prime-hermite-native-20261109 v2. No timing or population inference.</text></g></svg>")).map_err(|e|e.to_string())?;
    let host = file(&format!("{root}/runs/host.txt"))?;
    files.push(host.clone());
    let arms:Vec<_>=p["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"]})).collect();
    let e = json!({"schema":"experiment.export/v1","export_id":"prime-hermite-native-20261109-v2","created":utc,"protocol":{"path":format!("{root}/protocol.json"),"sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]},"producer":{"runtime":"codex","model":null,"session":"hermite-native-20261109","role":"producer"},"source":{"repo":"crypto","commit":p["freeze"]["commit"],"dirty":true},"backend":"local","host":{"path":host["path"],"sha256":host["sha256"],"class_id":null,"summary":{"cpu_model":null,"logical_cpus":null,"memory_bytes":null,"kernel":null,"hypervisor":null,"isolation_tier_max":"L0"}},"arms":arms,"runs":runs,"flow":{"planned":2,"attempted":2,"completed":2,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]},"files":files,"analysis":{"script":file(&format!("{root}/analyze.cpp"))?,"blinded":false,"blinding_key_sha256":null,"outputs":[file(&format!("{root}/summary.json"))?],"unmasked_at":null},"notes":[{"kind":"observation","text":"Two complete physical executions; embedded180 known-target cases and3 defective controls each; exact replay. Owner-authorized git checkpoints replace unavailable S3. No clocks admissible.","by":"producer"}],"reviews_dir":"reviews"});
    write(out, &e)
}
fn check(root: &str, path: &str) -> Result<(), String> {
    let p = read(&format!("{root}/protocol.json"))?;
    let e = read(path)?;
    require(
        canonical(&p) == p["freeze"]["protocol_sha256"]
            && e["protocol"]["sha256"] == p["freeze"]["protocol_sha256"]
            && e["protocol"]["version"] == p["version"],
        "protocol/export binding",
    )?;
    require(
        e["runs"].as_array().map(|r| r.len()) == Some(2),
        "physical row count",
    )?;
    let summary = read(&format!("{root}/summary.json"))?;
    require(
        summary["rows_recounted"] == 366 && summary["byte_identical"] == true,
        "frozen analysis",
    )?;
    for (i, r) in e["runs"].as_array().unwrap().iter().enumerate() {
        require(
            r["arm"] == p["arms"][i]["id"]
                && r["measurements"]["counted"]["rows_completed"] == 183
                && r["measurements"]["wall_valid"] == false
                && r["attempts"] == 1
                && r["resumed"] == 0,
            "physical row semantics",
        )?;
        require(
            e["arms"][i]["binary_sha256"] == p["arms"][i]["instrument"]["binary_sha256"]
                && e["arms"][i]["commit"] == p["arms"][i]["instrument"]["commit"],
            "arm instrument binding",
        )?;
        for f in r["artifacts"].as_array().unwrap() {
            pinned(f)?
        }
    }
    for f in e["files"].as_array().unwrap() {
        pinned(f)?
    }
    pinned(&e["analysis"]["script"])?;
    require(
        e["analysis"]["script"]["sha256"] == p["analysis_plan"]["script"]["sha256"],
        "frozen analyzer source",
    )?;
    for f in e["analysis"]["outputs"].as_array().unwrap() {
        pinned(f)?
    }
    require(
        e["flow"]["attempted"] == 2 && e["flow"]["completed"] == 2,
        "flow",
    )?;
    require(
        Path::new(&format!("{root}/runs/0/panel.tsv")).exists(),
        "raw data",
    )?;
    println!("Export semantics and all artifact hashes PASS (unsigned producer check)");
    Ok(())
}
fn main() -> Result<(), String> {
    let a: Vec<_> = env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("export") if a.len() == 5 => export(&a[2], &a[3], &a[4]),
        Some("check") if a.len() == 4 => check(&a[2], &a[3]),
        _ => Err("export ROOT UTC OUT | check ROOT EXPORT".into()),
    }
}
