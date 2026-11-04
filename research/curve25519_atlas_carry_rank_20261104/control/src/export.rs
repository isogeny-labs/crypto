use super::*;
fn load(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn string<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v[key]
        .as_str()
        .ok_or_else(|| format!("missing {key}").into())
}
fn relative(from: &Path, to: &Path) -> Result<String> {
    let from = from.canonicalize()?;
    let to = to.canonicalize()?;
    let a: Vec<_> = from.components().collect();
    let b: Vec<_> = to.components().collect();
    let common = a.iter().zip(&b).take_while(|(a, b)| a == b).count();
    let mut result = PathBuf::new();
    for _ in common..a.len() {
        result.push("..");
    }
    for c in &b[common..] {
        result.push(c.as_os_str());
    }
    Ok(result.to_str().ok_or("non-UTF8 path")?.to_owned())
}
fn file(path: &Path, base: &Path) -> Result<Value> {
    Ok(json!({"path":relative(base,path)?,"sha256":sha(path)?,"bytes":fs::metadata(path)?.len()}))
}
fn unchanged(path: &Path, expected: &Value) -> Result<()> {
    check(
        sha(path)? == expected.as_str().ok_or("missing artifact hash")?,
        "artifact changed since execution",
    )
}
fn now() -> Result<String> {
    let o = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()?;
    check(o.status.success(), "UTC timestamp unavailable")?;
    Ok(String::from_utf8(o.stdout)?.trim().into())
}
pub fn write(input: &Path, run: &Path, protocol_path: &Path, out: &Path) -> Result<()> {
    let input = input.canonicalize()?;
    let run = run.canonicalize()?;
    let parent = out.parent().unwrap_or(Path::new(".")).canonicalize()?;
    let p = load(protocol_path)?;
    check(
        p["schema"] == "experiment.protocol/v1" && p["freeze"]["status"] == "frozen",
        "export requires frozen canonical protocol",
    )?;
    let mut body = p.clone();
    body.as_object_mut()
        .ok_or("protocol object")?
        .remove("freeze");
    let phash = format!("{:x}", Sha256::digest(serde_json::to_vec(&body)?));
    check(
        p["freeze"]["protocol_sha256"] == phash,
        "frozen protocol hash mismatch",
    )?;
    let summary = load(&run.join("summary.json"))?;
    let identity = &summary["identity"];
    check(
        load(&run.join("identity.json"))? == *identity,
        "summary and execution identity differ",
    )?;
    check(
        identity["timeout_seconds"] == p["execution"]["placement"]["per_query_timeout_seconds"],
        "execution deadline differs from protocol",
    )?;
    unchanged(
        &input.join("queries.tsv"),
        &identity["query_manifest_sha256"],
    )?;
    let config = load(&input.join("configuration.json"))?;
    check(
        config == identity["configuration"],
        "input configuration changed",
    )?;
    check(
        config["source_revision"] == p["freeze"]["commit"],
        "input build revision differs from frozen protocol",
    )?;
    check(
        config["source_dirty"].is_boolean(),
        "input source-dirty state missing",
    )?;
    if p["execution"]["backend"] != "local" {
        check(
            config["source_dirty"] == false,
            "remote protocol requires clean published sources",
        )?;
    }
    check(
        config["targets_per_panel"] == p["sample_size"]["instances"],
        "protocol query count mismatch",
    )?;
    let manifest = queries(&input)?;
    let rows = summary["rows"].as_array().ok_or("missing rows")?;
    check(
        rows.len() == manifest.len(),
        "incomplete run: export only complete summaries; retain partial row receipts separately",
    )?;
    let mut results = vec![];
    let mut arms = BTreeSet::new();
    let mut refs = vec![
        file(&run.join("identity.json"), &parent)?,
        file(&input.join("queries.tsv"), &parent)?,
        file(&input.join("configuration.json"), &parent)?,
        file(&input.join("generic.csv"), &parent)?,
        file(&input.join("selection.txt"), &parent)?,
    ];
    for (index, (r, q)) in rows.iter().zip(&manifest).enumerate() {
        check(
            load(&run.join(format!("{index:04}.json")))? == *r,
            "summary and individual receipt differ",
        )?;
        let f = &q.fields;
        check(
            r["index"].as_u64() == Some(index as u64)
                && r["query"] == f[0]
                && r["arm"] == f[1]
                && r["encoding"] == f[2]
                && r["expected"] == f[11],
            "row/query identity mismatch",
        )?;
        let query = member(&input, &f[0])?;
        let base = member(&input, &f[7])?;
        unchanged(&query, &r["query_sha256"])?;
        unchanged(&base, &r["base_sha256"])?;
        let arm = format!("{}-{}", f[1], f[2]);
        arms.insert(arm.clone());
        let pa = p["arms"]
            .as_array()
            .ok_or("protocol arms")?
            .iter()
            .find(|a| a["id"] == arm)
            .ok_or("run arm absent from protocol")?;
        check(
            pa["parameters"]["p"] == config["p"] && pa["parameters"]["seed"] == config["seed"],
            "protocol field/seed mismatch",
        )?;
        check(
            config["codec"] == "bv" && pa["instrument"]["flags"]["codec"] == "bounded_bv",
            "protocol solver codec mismatch",
        )?;
        let column_arm = if arm == "source-aa-mod" {
            p["arms"]
                .as_array()
                .unwrap()
                .iter()
                .find(|a| a["id"] == "source-mod")
                .ok_or("missing A/A reference")?
        } else {
            pa
        };
        check(
            column_arm["parameters"]["columns"] == config["columns"],
            "protocol base column count mismatch",
        )?;
        let mut artifacts = vec![
            file(&query, &parent)?,
            file(&base, &parent)?,
            file(&run.join(format!("{index:04}.json")), &parent)?,
        ];
        for a in r["artifacts"]
            .as_array()
            .ok_or("solver artifacts")?
            .iter()
            .chain(r["replay_artifacts"].as_array().ok_or("replay artifacts")?)
        {
            let path = member(&run, string(a, "path")?)?;
            unchanged(&path, &a["sha256"])?;
            check(
                fs::metadata(&path)?.len() == a["bytes"].as_u64().ok_or("artifact size")?,
                "artifact size changed",
            )?;
            artifacts.push(file(&path, &parent)?);
        }
        let raw = fs::read_to_string(run.join(format!("{index:04}.solver.out")))?;
        check(
            raw.lines().next().unwrap_or("empty") == string(r, "observed")?,
            "raw solver status differs from receipt",
        )?;
        check(
            rlimit(&raw).ok() == r["solver_rlimit"].as_u64(),
            "solver resource counter differs from raw output",
        )?;
        let observed = string(r, "observed")?;
        let valid = r["producer_check_passed"] == true;
        let status = if valid {
            "succeeded"
        } else if observed == "timeout" {
            "timeout"
        } else {
            "failed"
        };
        let completed = status != "timeout";
        let instance = f[0]
            .split("-holdout-")
            .nth(1)
            .and_then(|s| s.split('-').next())
            .ok_or("query instance label")?;
        let mut counted = json!({"producer_check_passed":u8::from(valid)});
        if let Some(n) = r["solver_rlimit"].as_u64() {
            counted["solver_rlimit"] = json!(n);
        }
        results.push(json!({"run_id":format!("atlas-{index:04}"),"arm":arm,"instance_id":format!("i{instance}"),"seed":config["seed"],"replicate":0,"warmup":false,"status":status,"outcome_class":if completed{"completed"}else{"not_completed"},"attempts":1,"resumed":0,"checkpoint":null,"measurements":{"counted":counted,"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":{}},"isolation":{"tier":"none","contended":null,"blockers":["No clock or independent execution standing is admitted"]},"verification":{"status":"no_claim","verifier":null},"exclusion":{"excluded":false,"rule":null},"artifacts":artifacts,"started_at":null,"finished_at":null,"worker":null}));
    }
    let export_arms:Vec<Value>=arms.iter().map(|id|{
        let a=p["arms"].as_array().unwrap().iter().find(|a|a["id"]==*id).unwrap();
        json!({"id":id,"role":a["role"],"binary_sha256":identity["solver_binary_sha256"],"commit":p["freeze"]["commit"]})
    }).collect();
    let host = json!({"cpu_model":null,"logical_cpus":std::thread::available_parallelism().ok().map(|n|n.get()),"memory_bytes":null,"kernel":null,"hypervisor":null,"isolation_tier_max":"none"});
    let host_path = run.join("export-host.json");
    write_new(&host_path, &host)?;
    refs.push(file(&host_path, &parent)?);
    refs.push(file(&run.join("summary.json"), &parent)?);
    refs.push(file(protocol_path, &parent)?);
    let completed = results
        .iter()
        .filter(|r| r["outcome_class"] == "completed")
        .count();
    let e = json!({"schema":"experiment.export/v1","export_id":format!("{}-export",string(&p,"protocol_id")?),"created":now()?,"protocol":{"path":relative(&parent,protocol_path)?,"sha256":phash,"protocol_id":p["protocol_id"],"version":p["version"]},"producer":{"runtime":"native-atlas-control","model":null,"session":null,"role":"executor"},"source":{"repo":"crypto","commit":config["source_revision"],"dirty":config["source_dirty"]},"backend":p["execution"]["backend"],"host":{"path":relative(&parent,&host_path)?,"sha256":sha(&host_path)?,"class_id":null,"summary":host},"arms":export_arms,"flow":{"planned":results.len(),"attempted":results.len(),"completed":completed,"not_completed":results.len()-completed,"interrupted":0,"resumed":0,"excluded":[]},"runs":results,"files":refs,"analysis":{"script":null,"blinded":false,"blinding_key_sha256":null,"outputs":[],"unmasked_at":null},"notes":[{"kind":"observation","text":"Producer-native checks only; verification.status=no_claim preserves unverified standing. Local unit examples are format/integration tests, not scientific or timing evidence. Unknown host facts and all clocks remain null.","by":"native-atlas-control"}],"reviews_dir":"reviews"});
    write_new(out, &e)?;
    Ok(())
}
