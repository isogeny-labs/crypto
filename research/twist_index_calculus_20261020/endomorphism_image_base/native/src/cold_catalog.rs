//! Assemble immutable, resource-valid whole timing rounds after interruptions.
//! Selection is chronological; elapsed time is never a selection criterion.
use super::*;
use std::path::PathBuf;

fn read(path: &Path) -> Result<Value, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))
}
pub(super) fn resource_valid(v: &Value) -> bool {
    let command = v["run"]["command"].as_array();
    v["schema"] == "isolated-bench/1"
        && v["run"]["exit_status"] == 0
        && v["run"]["contended"] == false
        && v["left_on_reserved"]["user_threads"]
            .as_array()
            .is_some_and(|x| x.is_empty())
        && v["reserved_cpus"].as_array().is_some_and(|x| x.len() == 2)
        && v["reserved_cpus"] == v["run"]["cpus"]
        && v["host"]["logical_cpus"] == 8
        && v["host"]["cpu_model"] == "Intel(R) Xeon(R) Platinum 8488C"
        && command.is_some_and(|c| {
            c.iter().any(|s| s == "--as=8589934592") && c.iter().any(|s| s == "1200s")
        })
}
#[derive(Clone)]
struct Record {
    parent: PathBuf,
    origin: String,
    arm: String,
    sha256: String,
}
fn record(
    parent: &Path,
    arm: &str,
    p: u64,
    seed: u64,
    measured: bool,
    job: &str,
) -> Result<Record, String> {
    let exit = std::fs::read_to_string(parent.join(format!("{arm}.exit")))
        .map_err(|_| format!("missing {arm} exit"))?;
    if exit.trim() != "0" {
        return Err(format!("{arm} exit {}", exit.trim()));
    }
    let path = parent.join(arm).join("cold.json");
    let v = read(&path)?;
    let d = &v["deterministic"];
    if d["p"] != p
        || d["seed"] != seed
        || d["arm"] != arm
        || d["input_sha256"] != INPUT_SHA
        || d["undetermined"] != 0
    {
        return Err(format!("{arm} identity or completeness mismatch"));
    }
    let t = &v["timing"];
    let n = |key: &str| {
        t[key]
            .as_u64()
            .ok_or_else(|| format!("{arm} missing {key}"))
    };
    let online = [
        "point_queries_ns",
        "decomposition_ns",
        "matrix_ns",
        "recovery_ns",
        "verification_ns",
    ]
    .iter()
    .map(|k| n(k))
    .collect::<Result<Vec<_>, _>>()?
    .into_iter()
    .map(u128::from)
    .sum::<u128>();
    if online != n("online_ns")? as u128 || n("cold_ns")? as u128 != online + n("setup_ns")? as u128
    {
        return Err(format!("{arm} nonexclusive timing intervals"));
    }
    if measured && !resource_valid(&read(&parent.join(format!("{arm}.isolation.json")))?) {
        return Err(format!("{arm} resource receipt refused"));
    }
    let origin = [
        parent.join(format!("{arm}.origin-job.txt")),
        parent.join("job-id.txt"),
    ]
    .into_iter()
    .find_map(|path| std::fs::read_to_string(path).ok())
    .unwrap_or_else(|| job.to_owned())
    .trim()
    .to_owned();
    if origin.is_empty() {
        return Err("empty originating job".into());
    }
    Ok(Record {
        parent: parent.into(),
        origin,
        arm: arm.into(),
        sha256: hex::encode(crate::hash::sha256::sha256(&std::fs::read(path).unwrap())),
    })
}
fn same_origin(records: &[Record]) -> bool {
    records.len() == 11 && records.iter().all(|r| r.origin == records[0].origin)
}
fn copy(r: &Record, dest_parent: &Path, pass: &str, p: u64, seed: u64, from_job: &str) -> Value {
    std::fs::create_dir_all(dest_parent.join(&r.arm)).unwrap();
    std::fs::copy(
        r.parent.join(&r.arm).join("cold.json"),
        dest_parent.join(&r.arm).join("cold.json"),
    )
    .unwrap();
    for suffix in ["exit", "stdout", "stderr", "isolation.json"] {
        let name = format!("{}.{suffix}", r.arm);
        if r.parent.join(&name).exists() {
            std::fs::copy(r.parent.join(&name), dest_parent.join(&name)).unwrap();
        }
    }
    std::fs::write(
        dest_parent.join(format!("{}.origin-job.txt", r.arm)),
        format!("{}\n", r.origin),
    )
    .unwrap();
    json!({"p":p,"seed":seed,"pass":pass,"arm":r.arm,"origin_job":r.origin,"checkpoint_job":from_job,"source":r.parent.join(&r.arm).join("cold.json"),"cold_sha256":r.sha256})
}
pub(crate) fn assemble(manifest: &Path, out: &Path) {
    assert!(!out.exists(), "derived evidence must use a fresh directory");
    let m = read(manifest).unwrap();
    let mut jobs = m["jobs"].as_array().unwrap().clone();
    jobs.sort_by(|a, b| a["id"].as_str().unwrap().cmp(b["id"].as_str().unwrap()));
    std::fs::create_dir_all(out).unwrap();
    let mut copied = Vec::new();
    let mut inventory = Vec::new();
    let mut missing = Vec::new();
    let mut rounds = 0;
    let mut replays = 0;
    for p in PRIMES {
        for seed in 0..3u64 {
            let stem = format!("p{p}-seed{seed}");
            for pass in ["warmup", "first", "aa"] {
                for arm in ARMS {
                    let mut chosen = None;
                    for j in &jobs {
                        let id = j["id"].as_str().unwrap();
                        let parent = Path::new(j["out"].as_str().unwrap()).join(pass).join(&stem);
                        if let Ok(r) = record(&parent, arm, p, seed, false, id) {
                            chosen = Some((r, id));
                            break;
                        }
                    }
                    if let Some((r, id)) = chosen {
                        copied.push(copy(&r, &out.join(pass).join(&stem), pass, p, seed, id));
                        replays += 1;
                    } else {
                        missing.push(json!({"p":p,"seed":seed,"pass":pass,"arm":arm}));
                    }
                }
            }
            for round in 0..5 {
                let pass = format!("round{round}");
                let aa_pass = format!("round{round}-aa");
                let mut chosen = None;
                for j in &jobs {
                    let id = j["id"].as_str().unwrap();
                    let root = Path::new(j["out"].as_str().unwrap());
                    let parent = root.join(&pass).join(&stem);
                    if !parent.exists() {
                        continue;
                    }
                    let mut records = Vec::new();
                    let mut reasons = Vec::new();
                    for arm in ARMS {
                        match record(&parent, arm, p, seed, true, id) {
                            Ok(r) => records.push(r),
                            Err(e) => reasons.push(e),
                        }
                    }
                    match record(
                        &root.join(&aa_pass).join(&stem),
                        "original",
                        p,
                        seed,
                        true,
                        id,
                    ) {
                        Ok(r) => records.push(r),
                        Err(e) => reasons.push(e),
                    }
                    let admitted = reasons.is_empty() && same_origin(&records);
                    inventory.push(json!({"job":id,"p":p,"seed":seed,"round":round,"admitted_whole_round":admitted,"reasons":reasons}));
                    if admitted && chosen.is_none() {
                        chosen = Some((records, id));
                    }
                }
                if let Some((records, id)) = chosen {
                    let parent = out.join(&pass).join(&stem);
                    let aa_parent = out.join(&aa_pass).join(&stem);
                    for (i, r) in records.iter().enumerate() {
                        copied.push(copy(
                            r,
                            if i < 10 { &parent } else { &aa_parent },
                            if i < 10 { &pass } else { &aa_pass },
                            p,
                            seed,
                            id,
                        ));
                    }
                    for dest in [&parent, &aa_parent] {
                        std::fs::write(dest.join("job-id.txt"), format!("{}\n", records[0].origin))
                            .unwrap();
                    }
                    rounds += 1;
                } else {
                    missing.push(json!({"p":p,"seed":seed,"round":round,"required":"all ten arms and original A/A, one origin, all resources admitted"}));
                }
            }
        }
    }
    let status = if missing.is_empty() {
        "pass"
    } else {
        "partial"
    };
    let result = json!({"schema":"twist-image-cold-assembly/v1","status":status,"selection":"first chronological complete and admitted unit; no elapsed-time selection","manifest_sha256":hex::encode(crate::hash::sha256::sha256(&std::fs::read(manifest).unwrap())),"completed_whole_rounds":rounds,"required_whole_rounds":105,"completed_nontiming_records":replays,"required_nontiming_records":630,"records":copied,"inventory":inventory,"missing":missing});
    write(&out.join("provenance.json"), &result);
    println!(
        "{}",
        json!({"status":status,"whole_rounds":rounds,"nontiming_records":replays,"missing":missing})
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refuses_contended_or_different_resource_envelope() {
        let good = json!({"schema":"isolated-bench/1","run":{"exit_status":0,"contended":false,"cpus":[1,5],"command":["prlimit","--as=8589934592","timeout","1200s"]},"reserved_cpus":[1,5],"left_on_reserved":{"user_threads":[]},"host":{"logical_cpus":8,"cpu_model":"Intel(R) Xeon(R) Platinum 8488C"}});
        assert!(resource_valid(&good));
        let mut bad = good.clone();
        bad["run"]["contended"] = json!(true);
        assert!(!resource_valid(&bad));
        let mut bad = good.clone();
        bad["left_on_reserved"]["user_threads"] = json!([42]);
        assert!(!resource_valid(&bad));
        let mut bad = good.clone();
        bad["run"]["command"] = json!(["timeout", "1200s"]);
        assert!(!resource_valid(&bad));
        let mut bad = good;
        bad["host"]["logical_cpus"] = json!(16);
        assert!(!resource_valid(&bad));
    }
    #[test]
    fn refuses_partial_rounds_and_mixed_origins() {
        let r = Record {
            parent: PathBuf::new(),
            origin: "job-a".into(),
            arm: "original".into(),
            sha256: String::new(),
        };
        assert!(!same_origin(&vec![r.clone(); 10]));
        let mut round = vec![r; 11];
        assert!(same_origin(&round));
        round[10].origin = "job-b".into();
        assert!(!same_origin(&round));
    }
}
