//! Post-run complete-ladder export and independent OpenSSL audit.
//! No solver is executed; the preregistered primary analyzer stays unchanged.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const HEADER:&str="arm,instance_id,seed,replicate,warmup,verification,bits,tau,scalar,relations,attempts,wall_seconds,bases_seconds,targets_seconds,oracle_seconds,linear_seconds,replay_seconds,second_replay_seconds,preparations,built_rows,probed_rows,query_inversions,table_bytes,tier";
fn check(c: bool, m: &str) -> Result<()> {
    if c {
        Ok(())
    } else {
        Err(m.into())
    }
}
fn hash(p: &Path) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(fs::read(p)?)))
}
fn reference(p: &Path, kind: &str) -> Result<Value> {
    Ok(
        json!({"path":p.to_str().ok_or("path UTF8")?,"sha256":hash(p)?,"bytes":fs::metadata(p)?.len(),"kind":kind}),
    )
}
fn load(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn complete_rows(csv: &str, status: &str) -> Result<Vec<Vec<String>>> {
    let mut lines = csv.lines();
    check(lines.next() == Some(HEADER), "row header")?;
    let mut rows = Vec::new();
    let mut keys = BTreeSet::new();
    let mut structural = BTreeMap::new();
    for line in lines {
        let v: Vec<String> = line.split(',').map(str::to_owned).collect();
        check(v.len() == 24, "row width")?;
        let c: usize = v[1].parse()?;
        let seed: u64 = v[2].parse()?;
        check(
            c < 6 && [101, 202].contains(&seed) && ["u", "x"].contains(&v[0].as_str()),
            "unexpected cell",
        )?;
        check(
            v[3] == "0" && v[4] == "false" && v[5] == "verified" && v[23] == "L0",
            "row state",
        )?;
        check(v[6].parse::<usize>()? == 20 + 4 * c, "field bits")?;
        check(keys.insert((v[0].clone(), c, seed)), "duplicate cell")?;
        for k in [6, 7, 8, 9, 10, 18, 19, 20, 21, 22] {
            let _: u64 = v[k].parse()?;
        }
        for value in v.iter().take(18).skip(11) {
            let t: f64 = value.parse()?;
            check(t.is_finite() && t >= 0.0, "invalid duration")?;
        }
        check(v[11].parse::<f64>()? > 0.0, "zero complete duration")?;
        let sig: Vec<_> = [6, 7, 8, 9, 10, 18, 19, 20, 21, 22]
            .into_iter()
            .map(|k| v[k].clone())
            .collect();
        if let Some(old) = structural.insert((c, seed), sig.clone()) {
            check(old == sig, "A/A structural mismatch")?;
        }
        rows.push(v);
    }
    check(rows.len() == 24, "complete export requires all24 rows")?;
    let mut ls = status.lines();
    check(
        ls.next() == Some("arm,case,seed,exit_code,replay_exit_code"),
        "status header",
    )?;
    let mut seen = BTreeSet::new();
    for line in ls {
        let v: Vec<_> = line.split(',').collect();
        check(v.len() == 5, "status width")?;
        check(
            v[3] == "0" && v[4] == "0",
            "failed status retained; cannot certify complete export",
        )?;
        let k = (
            v[0].to_owned(),
            v[1].parse::<usize>()?,
            v[2].parse::<u64>()?,
        );
        check(keys.contains(&k) && seen.insert(k), "status key")?;
    }
    check(seen == keys, "missing status")?;
    Ok(rows)
}
fn invoke(binary: &Path, args: &[String]) -> Result<std::process::Output> {
    Ok(Command::new("timeout")
        .arg("60")
        .arg(binary)
        .args(args)
        .output()?)
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    check(
        a.len() == 8,
        "STUDY RAW ANALYZER RUST_VERIFIER OPENSSL_CHECKER AUDIT_DIR UTC",
    )?;
    let study = PathBuf::from(&a[1]).canonicalize()?;
    let raw = PathBuf::from(&a[2]).canonicalize()?;
    let analyzer = PathBuf::from(&a[3]).canonicalize()?;
    let verifier = PathBuf::from(&a[4]).canonicalize()?;
    let openssl = PathBuf::from(&a[5]).canonicalize()?;
    let audit = PathBuf::from(&a[6]);
    let export_path = study.join("export.json");
    check(!export_path.exists(), "export already exists")?;
    check(!audit.exists(), "audit directory already exists")?;
    let protocol = load(&study.join("protocol.json"))?;
    let build = load(&study.join("build.json"))?;
    check(
        protocol["freeze"]["status"] == "frozen",
        "unfrozen protocol",
    )?;
    check(
        hash(&analyzer)?
            == build["binaries"]["analyze"]
                .as_str()
                .ok_or("analyzer hash")?,
        "analyzer binary mismatch",
    )?;
    check(
        hash(&study.join("analyze.rs"))?
            == protocol["analysis_plan"]["script"]["sha256"]
                .as_str()
                .ok_or("source hash")?,
        "analyzer source mismatch",
    )?;
    check(
        hash(&verifier)?
            == build["binaries"]["hashed-ecdl-cpu"]
                .as_str()
                .ok_or("verifier hash")?,
        "verifier binary mismatch",
    )?;
    check(
        fs::read(raw.join("protocol.json"))? == fs::read(study.join("protocol.json"))?,
        "returned protocol mismatch",
    )?;
    check(
        fs::read(raw.join("build.json"))? == fs::read(study.join("build.json"))?,
        "returned build mismatch",
    )?;
    let rows = complete_rows(
        &fs::read_to_string(raw.join("rows.csv"))?,
        &fs::read_to_string(raw.join("status.csv"))?,
    )?;
    fs::create_dir(&audit)?;
    let scratch = audit.join("primary-analysis-readback");
    fs::create_dir(&scratch)?;
    for e in fs::read_dir(&raw)? {
        let e = e?;
        if e.file_type()?.is_file() {
            fs::copy(e.path(), scratch.join(e.file_name()))?;
        }
    }
    let primary = Command::new(&analyzer)
        .arg(raw.join("rows.csv"))
        .arg(&scratch)
        .output()?;
    fs::write(audit.join("primary-analyzer.stdout"), &primary.stdout)?;
    fs::write(audit.join("primary-analyzer.stderr"), &primary.stderr)?;
    check(primary.status.success(), "primary native analyzer rejected")?;
    check(
        String::from_utf8_lossy(&primary.stdout).trim()
            == "planned=24 attempted=24 verified=24 failed=0 not_attempted=0",
        "primary flow mismatch",
    )?;
    check(
        fs::read(scratch.join("scale.svg"))? == fs::read(raw.join("scale.svg"))?,
        "primary SVG replay mismatch",
    )?;
    let columns: Vec<_> = HEADER.split(',').collect();
    let mut runs = Vec::new();
    let mut certs = BTreeMap::new();
    for row in rows {
        let id = format!("{}-{}-{}", row[0], row[1], row[2]);
        let cert_path = raw.join(format!("scale-{id}.json"));
        let bytes = fs::read(&cert_path)?;
        if let Some(old) = certs.insert((row[1].clone(), row[2].clone()), bytes.clone()) {
            check(old == bytes, "A/A certificate bytes mismatch")?;
        }
        let c: Value = serde_json::from_slice(&bytes)?;
        check(
            c["relations"].as_array().ok_or("relations")?.len() == row[9].parse::<usize>()?,
            "relation count mismatch",
        )?;
        check(
            c["attempts"].as_u64() == Some(row[10].parse()?)
                && c["scalar"].as_u64() == Some(row[8].parse()?),
            "certificate row mismatch",
        )?;
        let replay = invoke(
            &verifier,
            &[
                "--verify".into(),
                cert_path.to_str().ok_or("cert path")?.into(),
            ],
        )?;
        let replay_path = audit.join(format!("{id}.native-replay"));
        fs::write(&replay_path, &replay.stdout)?;
        check(
            replay.status.success()
                && String::from_utf8_lossy(&replay.stdout).trim()
                    == format!("VERIFIED scalar={}", row[8]),
            "native replay failed",
        )?;
        let curve = c["curve"].as_array().ok_or("curve")?;
        let g = c["generator"].as_array().ok_or("generator")?;
        let q = c["target"].as_array().ok_or("target")?;
        check(
            curve.len() == 4 && g.len() == 2 && q.len() == 2,
            "affine input shape",
        )?;
        let mut args = vec![row[1].clone(), row[2].clone()];
        for v in curve.iter().chain(g).chain(q) {
            args.push(v.as_u64().ok_or("coordinate")?.to_string());
        }
        args.push(row[8].clone());
        let good = invoke(&openssl, &args)?;
        check(
            good.status.success(),
            &format!(
                "OpenSSL check failed: {}",
                String::from_utf8_lossy(&good.stderr)
            ),
        )?;
        let positive = audit.join(format!("{id}.openssl-positive"));
        fs::write(&positive, &good.stdout)?;
        let r = curve[3].as_u64().ok_or("order")?;
        let k = c["scalar"].as_u64().ok_or("scalar")?;
        let mut bad = args.clone();
        bad[10] = ((k + 1) % r).to_string();
        let negative = invoke(&openssl, &bad)?;
        check(
            negative.status.code() == Some(1)
                && String::from_utf8_lossy(&negative.stderr).contains("SCALAR"),
            "wrong scalar control failed",
        )?;
        let neg1 = audit.join(format!("{id}.openssl-wrong-scalar"));
        fs::write(&neg1, &negative.stderr)?;
        let mut substituted = args.clone();
        let p = curve[0].as_u64().ok_or("modulus")?;
        let y = q[1].as_u64().ok_or("target y")?;
        check(k > 0 && y > 0, "nonidentity negation control")?;
        substituted[9] = (p - y).to_string();
        substituted[10] = (r - k).to_string();
        let binding = invoke(&openssl, &substituted)?;
        check(
            binding.status.code() == Some(1)
                && String::from_utf8_lossy(&binding.stderr).contains("BINDING public target"),
            "consistent substituted-target control failed",
        )?;
        let neg2 = audit.join(format!("{id}.openssl-substituted-target"));
        fs::write(&neg2, &binding.stderr)?;
        let mut counters = serde_json::Map::new();
        for j in 6..23 {
            let value = if (11..18).contains(&j) {
                json!(row[j].parse::<f64>()?)
            } else {
                json!(row[j].parse::<u64>()?)
            };
            counters.insert(columns[j].into(), value);
        }
        runs.push(json!({"run_id":id,"arm":row[0],"instance_id":row[1],"seed":row[2].parse::<u64>()?,"replicate":0,"warmup":false,"status":"succeeded","outcome_class":"completed","attempts":1,"resumed":0,"checkpoint":null,
            "measurements":{"counted":{"relations":row[9].parse::<u64>()?},"wall_seconds":row[11].parse::<f64>()?,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":counters},
            "isolation":{"tier":"L0","contended":null,"blockers":["Shared local host; clocks are descriptive only"]},"verification":{"status":"verified","verifier":"Native Archive replay plus independent OpenSSL public-target and scalar audit"},"exclusion":{"excluded":false,"rule":null},
            "artifacts":[reference(&cert_path,"raw")?,reference(&replay_path,"receipt")?,reference(&positive,"receipt")?,reference(&neg1,"receipt")?,reference(&neg2,"receipt")?],"started_at":null,"finished_at":null,"worker":"local-single-thread","backend_ref":"native-cpu-scale-20261123"}));
    }
    let mut files = Vec::new();
    for dir in [&raw, &audit] {
        let mut paths = Vec::new();
        for e in fs::read_dir(dir)? {
            let e = e?;
            if e.file_type()?.is_file() {
                paths.push(e.path());
            }
        }
        paths.sort();
        for p in paths {
            files.push(reference(&p, "raw")?);
        }
    }
    let cpu = fs::read_to_string(raw.join("cpuinfo.txt"))?;
    let model = cpu
        .lines()
        .find(|s| s.starts_with("model name"))
        .and_then(|s| s.split_once(':'))
        .map(|x| x.1.trim());
    let host = reference(&raw.join("cpuinfo.txt"), "host")?;
    let arms:Vec<_>=protocol["arms"].as_array().ok_or("arms")?.iter().map(|x|json!({"id":x["id"],"role":x["role"],"binary_sha256":x["instrument"]["binary_sha256"],"commit":x["instrument"]["commit"]})).collect();
    let export = json!({"schema":"experiment.export/v1","export_id":"HASHED-ECDL-CPU-SCALE-LOCAL-20261123","created":a[7],"protocol":{"path":study.join("protocol.json"),"sha256":protocol["freeze"]["protocol_sha256"],"protocol_id":protocol["protocol_id"],"version":protocol["version"]},
        "producer":{"runtime":"codex","model":"Codex","session":"native-cpu-scale-20261123","role":"executor"},"source":{"repo":"crypto","commit":build["source_commit"],"dirty":false},"backend":"local",
        "host":{"path":host["path"],"sha256":host["sha256"],"class_id":"L0-local-single-thread","summary":{"cpu_model":model,"logical_cpus":null,"memory_bytes":null,"kernel":fs::read_to_string(raw.join("kernel.txt"))?.trim(),"hypervisor":null,"isolation_tier_max":"L0"}},"arms":arms,"runs":runs,
        "flow":{"planned":24,"attempted":24,"completed":24,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]},"files":files,
        "analysis":{"script":reference(&study.join("analyze.rs"),"analysis")?,"blinded":false,"blinding_key_sha256":null,"outputs":[reference(&audit.join("primary-analyzer.stdout"),"summary")?,reference(&scratch.join("scale.svg"),"summary")?],"unmasked_at":null},
        "notes":[{"kind":"observation","by":"native exporter","text":"24 calls recover12 fixed public targets; no extra solver calls are made by this adapter. The primary frozen native analyzer and SVG are reproduced unchanged. Native Archive replay and independent OpenSSL auditing are post-run validation, not part of the original diagnostic clock."},{"kind":"observation","by":"native exporter","text":"Every OpenSSL audit rejects a changed scalar and an algebraically consistent negated scalar/target pair that violates the frozen public-target binding. Exact field/order admission remains the native deterministic-u64 primality/order check. No comparative timing or novelty claim."}],"reviews_dir":"reviews"});
    let bytes = serde_json::to_vec_pretty(&export)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(export_path)?;
    file.write_all(&bytes)?;
    file.write_all(b"\n")?;
    println!("PASS24complete calls,12public targets,24OpenSSL positives,48negative controls; native unified export written");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (String, String) {
        let mut csv = format!("{HEADER}\n");
        let mut status = "arm,case,seed,exit_code,replay_exit_code\n".to_owned();
        for c in 0..6 {
            for s in [101, 202] {
                for arm in ["u", "x"] {
                    let mut v = vec!["1".to_owned(); 24];
                    v[0] = arm.into();
                    v[1] = c.to_string();
                    v[2] = s.to_string();
                    v[3] = "0".into();
                    v[4] = "false".into();
                    v[5] = "verified".into();
                    v[6] = (20 + 4 * c).to_string();
                    v[23] = "L0".into();
                    csv.push_str(&v.join(","));
                    csv.push('\n');
                    status.push_str(&format!("{arm},{c},{s},0,0\n"));
                }
            }
        }
        (csv, status)
    }
    #[test]
    fn complete_metadata() {
        let (c, s) = fixture();
        assert_eq!(complete_rows(&c, &s).unwrap().len(), 24);
    }
    #[test]
    fn rejects_missing_row() {
        let (c, s) = fixture();
        let c = c.lines().take(24).collect::<Vec<_>>().join("\n");
        assert!(complete_rows(&c, &s).is_err());
    }
    #[test]
    fn rejects_duplicate_row() {
        let (mut c, s) = fixture();
        let first = c.lines().nth(1).unwrap().to_owned();
        c.push_str(&first);
        c.push('\n');
        assert!(complete_rows(&c, &s).is_err());
    }
    #[test]
    fn rejects_failed_replay() {
        let (c, s) = fixture();
        assert!(complete_rows(&c, &s.replacen(",0,0\n", ",0,1\n", 1)).is_err());
    }
    #[test]
    fn rejects_missing_status() {
        let (c, s) = fixture();
        let s = s.lines().take(24).collect::<Vec<_>>().join("\n");
        assert!(complete_rows(&c, &s).is_err());
    }
    #[test]
    fn rejects_unexpected_arm() {
        let (c, s) = fixture();
        assert!(complete_rows(&c.replacen("\nu,", "\nz,", 1), &s).is_err());
    }
    #[test]
    fn rejects_structural_mismatch() {
        let (c, s) = fixture();
        let mut lines: Vec<String> = c.lines().map(str::to_owned).collect();
        let mut row: Vec<String> = lines[2].split(',').map(str::to_owned).collect();
        row[20] = "2".into();
        lines[2] = row.join(",");
        assert!(complete_rows(&lines.join("\n"), &s).is_err());
    }
    #[test]
    fn rejects_nonfinite_clock() {
        let (c, s) = fixture();
        let mut lines: Vec<String> = c.lines().map(str::to_owned).collect();
        let mut row: Vec<String> = lines[1].split(',').map(str::to_owned).collect();
        row[11] = "NaN".into();
        lines[1] = row.join(",");
        assert!(complete_rows(&lines.join("\n"), &s).is_err());
    }
}
