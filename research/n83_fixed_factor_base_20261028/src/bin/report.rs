//! Deterministic presentation of already generated point data; never generates points.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path};
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn digest(p: &Path) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = vec![0; 1024 * 1024];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn file(root: &Path, path: &str, kind: &str) -> Value {
    json!({"path":path,"kind":kind,"sha256":digest(&root.join(path)),"bytes":fs::metadata(root.join(path)).unwrap().len()})
}
fn save(p: &Path, v: &Value) {
    fs::write(p, format!("{}\n", serde_json::to_string_pretty(v).unwrap())).unwrap();
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root = Path::new(&args[1]);
    let protocol = read(&root.join("protocol-r4.json"));
    let index = read(&root.join("data/index.json"));
    let receipt = read(&root.join("terminal-r4.json"));
    let request = read(&root.join("request-r4.json"));
    assert_eq!(receipt["state"], "succeeded");
    assert_eq!(index["complete"], true);
    let podname = receipt["host"].as_str().unwrap();
    let uid = receipt["run_id"].as_str().unwrap();
    let uri = "s3://setec-astronomy-artifacts/runs/n83-fixed-factor-base-e27-20261028-r4/local/";
    let mut files = Vec::new();
    let mut runs = Vec::new();
    let mut counts = Vec::new();
    for family in index["families"].as_array().unwrap() {
        let name = family["family"].as_str().unwrap();
        let path = format!("data/{name}/manifest.json");
        assert_eq!(
            digest(&root.join(&path)),
            family["manifest_sha256"].as_str().unwrap()
        );
        let manifest = read(&root.join(&path));
        files.push(file(root, &path, "summary"));
        let mut end = 0u64;
        let mut count = 0u64;
        let mut xs = 0u64;
        for (i, row) in manifest["shards"].as_array().unwrap().iter().enumerate() {
            assert_eq!(row["start"].as_u64().unwrap(), end);
            end = row["end"].as_u64().unwrap();
            assert_eq!(row["candidates"], 1048576);
            assert_eq!(row["verified_all_candidates"], true);
            let binpath = format!("data/{name}/{}", row["file"].as_str().unwrap());
            let artifact = file(root, &binpath, "raw");
            assert_eq!(artifact["sha256"], row["sha256"]);
            assert_eq!(artifact["bytes"], row["bytes"]);
            files.push(artifact.clone());
            let certpath = format!("data/{name}/part-{i:04}.json");
            assert_eq!(read(&root.join(&certpath)), *row);
            files.push(file(root, &certpath, "raw"));
            let n = row["point_count"].as_u64().unwrap();
            let x = row["liftable_x"].as_u64().unwrap();
            count += n;
            xs += x;
            runs.push(json!({"arm":name,"artifacts":[artifact],"attempts":1,"backend_ref":format!("{uri}{binpath}"),
                "checkpoint":null,"exclusion":{"excluded":false,"rule":null},"finished_at":null,"instance_id":i.to_string(),
                "isolation":{"blockers":["Ordinary Spot; no performance measurement"],"contended":null,"tier":"L0"},
                "measurements":{"counted":{"point_count":n,"liftable_x":x},"counters":{},"cpu_sys_seconds":null,"cpu_user_seconds":null,"max_rss_bytes":null,"wall_seconds":null,"wall_valid":false},
                "outcome_class":"completed","replicate":0,"resumed":0,"run_id":format!("{name}-{i:04}"),"seed":20261028,
                "started_at":null,"status":"succeeded","verification":{"status":"verified","verifier":"Every candidate checked for complete valid sign pairs or no lift; distributed independent slow arithmetic; compressed byte hash checked against the manifest"},"warmup":false,"worker":podname}));
        }
        assert_eq!(end, 1 << 27);
        assert_eq!(count, family["point_count"].as_u64().unwrap());
        assert_eq!(xs, family["liftable_x"].as_u64().unwrap());
        counts.push((
            name.to_owned(),
            count,
            xs,
            family["compressed_bytes"].as_u64().unwrap(),
        ));
    }
    assert_eq!(runs.len(), 384);
    for (p, k) in [
        ("data/index.json", "summary"),
        ("evidence/controls.json", "raw"),
        ("evidence/host.txt", "other"),
        ("evidence/binary.sha256", "other"),
        ("evidence/source-revision.txt", "other"),
        ("evidence/raw-manifest.sha256", "other"),
        ("request-r4.json", "receipt"),
        ("terminal-r4.json", "receipt"),
        ("src/main.rs", "analysis"),
        ("src/field.rs", "analysis"),
    ] {
        files.push(file(root, p, k));
    }
    let source = protocol["arms"][0]["instrument"]["commit"].clone();
    assert_eq!(
        fs::read_to_string(root.join("evidence/source-revision.txt"))
            .unwrap()
            .trim(),
        source.as_str().unwrap()
    );
    let arms=protocol["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"commit":source,"binary_sha256":a["instrument"]["binary_sha256"],"image_digest":request["image"]})).collect::<Vec<_>>();
    let export = json!({"schema":"experiment.export/v1","export_id":"N83-FIXED-FACTOR-BASE-E27-20261028-R4","created":"2026-10-28","backend":"local",
        "protocol":{"path":"protocol-r4.json","protocol_id":protocol["protocol_id"],"sha256":protocol["freeze"]["protocol_sha256"],"version":4},
        "source":{"repo":"crypto","commit":source,"dirty":false},"producer":{"runtime":"codex","role":"producer","model":null,"session":"n83-fixed-factor-base-e27-20261028-r4"},
        "arms":arms,"runs":runs,"files":files,"flow":{"planned":384,"attempted":384,"completed":384,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]},
        "host":{"class_id":"existing-spot-host-L0","path":"evidence/host.txt","sha256":digest(&root.join("evidence/host.txt")),"summary":{"cpu_model":"See host.txt","hypervisor":"Amazon EC2","isolation_tier_max":"L0","kernel":"See host.txt","logical_cpus":null,"memory_bytes":null}},
        "analysis":{"blinded":false,"blinding_key_sha256":null,"unmasked_at":null,"script":file(root,"src/main.rs","analysis"),"outputs":[file(root,"data/index.json","summary")]},
        "notes": (["R1 never scheduled because of Spot quota; cancelled without production data. R2 was rejected before job creation; R3 failed before enumeration due to shared-bucket denial; R4 uses6threads on existing workspace Spot under pre-data amendment.","Each per-curve manifest is reused unchanged for m2..9. Both signs are stored. No subgroup filter.","Old varying-ell degree results do not apply to this fixed domain. No new degree or performance claim."].map(|text| json!({"kind":"observation","text":text,"by":"producer"}))),"reviews_dir":"reviews"});
    save(&root.join("experiment.export.json"), &export);
    let dir = root.join("reports");
    fs::create_dir_all(&dir).unwrap();
    let mut md=String::from("# Fixed n83 factor-base points\n\nObservations from complete enumeration; no performance or degree claim.\n\nAll 134,217,728 candidate x-values per curve were enumerated with ell=27. Every m=2..9 uses the same per-curve manifest. Both signs of each affine lift are stored; infinity and subgroup filtering are excluded.\n\n| Curve | Liftable x | Stored points | Compressed bytes |\n|---|---:|---:|---:|\n");
    for (n, p, x, b) in &counts {
        md.push_str(&format!("| {n} | {x} | {p} | {b} |\n"));
    }
    md.push_str(&format!("\n384/384 shards completed. All candidates checked, including omitted x-values; slow-arithmetic cross-checks and mutation controls pass. Stored compressed bytes match SHA-256.\n\nR1 was cancelled before scheduling after MaxSpotInstanceCountExceeded. R2 admission rejected an odd CPU count. R3 failed before enumeration due to storage permissions. R4 used6threads on existing workspace Spot,8Gi virtual-memory limit and3300s producer timeout. Process PID: {}. Run: {uid}.\n\nProtocol: {}. Source: {}. Image (null for native host run): {}.\n\nRaw data: {uri}\n\nThe standard first-fall sweep used varying ell and is not rerun here. No subgroup membership, end-to-end IC, independent research review, or timing claim follows.\n\nSources: [SEC1 v2, section2.3.4](https://www.secg.org/sec1-v2.pdf); [Taverne et al., binary arithmetic section2](https://cacr.uwaterloo.ca/techreports/2011/cacr2011-07.pdf).\n",receipt["pid"],protocol["freeze"]["protocol_sha256"],source,request["image"]));
    fs::write(dir.join("report.md"), md).unwrap();
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"900\" height=\"300\" viewBox=\"0 0 900 300\"><rect width=\"900\" height=\"300\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#172c43\"><text x=\"20\" y=\"30\" font-size=\"22\">Stored factor-base points — fixed ell=27</text>");
    for (i, (n, p, _, _)) in counts.iter().enumerate() {
        let y = 65 + i * 65;
        let w = *p as f64 / 140000000.0 * 530.0;
        svg.push_str(&format!("<text x=\"20\" y=\"{}\" font-size=\"18\">{n}</text><rect x=\"135\" y=\"{y}\" width=\"{w}\" height=\"32\" fill=\"#287d9c\"/><text x=\"690\" y=\"{}\" font-size=\"18\">{p}</text>",y+22,y+22));
    }
    svg.push_str("<text x=\"20\" y=\"280\" font-size=\"15\">Exact counts; both signs. Same manifest reused for m=2 through 9. R4 / 384 shards.</text></g></svg>\n");
    fs::write(dir.join("counts.svg"), svg).unwrap();
    println!("{}", serde_json::to_string_pretty(&index).unwrap());
}
