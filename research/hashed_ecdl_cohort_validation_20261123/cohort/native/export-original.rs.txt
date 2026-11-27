//! Post-run metadata only. Never executes a solver, replay, or timing analysis.
//! Complete-panel adapter: partial analysis remains intact; no synthetic attempts
//! are created to work around export-v1's attempts>=1/flow vocabulary.
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    io::Write,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const NAMES: [&str; 17] = [
    "bits",
    "tau",
    "scalar",
    "relations",
    "relation_attempts",
    "complete_seconds",
    "bases_seconds",
    "targets_seconds",
    "oracle_seconds",
    "linear_seconds",
    "internal_replay_seconds",
    "second_replay_seconds",
    "preparations",
    "built_rows",
    "probed_rows",
    "query_inversions",
    "table_bytes",
];
fn check(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn load(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn reference(p: &Path, kind: &str) -> Result<Value> {
    let bytes = fs::read(p)?;
    Ok(
        json!({"path":p.to_str().ok_or("UTF8path")?,"bytes":bytes.len(),"sha256":hash(&bytes),"kind":kind}),
    )
}
fn collect(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for e in fs::read_dir(dir)? {
        let e = e?;
        let t = e.file_type()?;
        check(!t.is_symlink(), "raw payload symlink")?;
        if t.is_dir() {
            collect(&e.path(), out)?;
        } else if t.is_file() {
            out.push(e.path());
        }
    }
    Ok(())
}
fn outside_raw(output: &Path, raw: &Path) -> Result<()> {
    check(!output.exists(), "new output required")?;
    let parent = output.parent().ok_or("output parent")?.canonicalize()?;
    let raw = raw.canonicalize()?;
    check(
        !parent.starts_with(raw),
        "output parent resolves into immutable main",
    )
}
/// Parse the original JSON number token, never through a binary floating value.
fn number(token: &str) -> Result<Value> {
    let value: Value = serde_json::from_str(token)?;
    check(value.is_number(), "number required")?;
    let serialized = serde_json::to_vec(&value)?;
    check(
        serialized == token.as_bytes(),
        "numeric token normalization",
    )?;
    Ok(value)
}
fn counters(columns: &[Value], status: &Value) -> Result<Value> {
    check(columns.len() == 24, "24native columns")?;
    check(
        columns[0] == "u"
            && columns[3] == "0"
            && columns[4] == "false"
            && columns[5] == "verified"
            && columns[23] == "L0",
        "native metadata",
    )?;
    let mut map = Map::new();
    for (i, name) in NAMES.iter().enumerate() {
        map.insert(
            (*name).into(),
            number(columns[i + 6].as_str().ok_or("raw token string")?)?,
        );
    }
    map.insert("native_replicate".into(), json!(0));
    // Counters cannot contain booleans: retain the raw literal as a string.
    map.insert("native_warmup".into(), json!("false"));
    let seconds = status["driver_seconds"].clone();
    check(seconds.is_number(), "driver seconds")?;
    map.insert("driver_seconds".into(), seconds);
    Ok(Value::Object(map))
}
fn validate_rows(analysis: &Value) -> Result<()> {
    check(analysis["planned"]==360 && analysis["attempted"]==360 && analysis["completed"]==360 && analysis["not_attempted"]==0 && analysis["failed_or_incomplete"]==0 && analysis["distinct_targets"]==12 && analysis["exact_certificate_and_counter_equality"]==true,"requires complete accepted native analysis; partial raw flow is retained without manufactured attempts")?;
    let rows = analysis["rows"].as_array().ok_or("analysis rows")?;
    check(rows.len() == 360, "360rows")?;
    let mut keys = BTreeSet::new();
    let mut groups = std::collections::BTreeMap::new();
    for r in rows {
        let arm = r["arm"].as_str().ok_or("arm")?;
        let case = r["case"].as_u64().ok_or("case")?;
        let seed = r["seed"].as_u64().ok_or("seed")?;
        let round = r["replicate"].as_u64().ok_or("round")?;
        check(
            ["a", "b", "c"].contains(&arm) && case < 6 && [101, 202].contains(&seed) && round < 10,
            "identity bounds",
        )?;
        check(
            r["id"] == format!("{arm}-{case}-{seed}-{round}")
                && r["instance_id"] == format!("{case}-{seed}")
                && r["warmup"] == (round == 0)
                && r["attempted"] == true,
            "bound canonical identity",
        )?;
        check(
            keys.insert((arm, case, seed, round)),
            "duplicate canonical cell",
        )?;
        *groups.entry((arm, case, seed, round == 0)).or_insert(0) += 1;
        let s = &r["status"];
        check(
            s["worker_exit"] == 0
                && s["replay_exit"] == 0
                && s["worker_success"] == true
                && s["replay_success"] == true,
            "successful exit binding",
        )?;
        let cols = r["raw_columns"].as_array().ok_or("columns")?;
        let case_token = case.to_string();
        let seed_token = seed.to_string();
        check(
            cols.len() == 24
                && cols[1].as_str() == Some(case_token.as_str())
                && cols[2].as_str() == Some(seed_token.as_str()),
            "raw cell binding",
        )?;
        counters(cols, s)?;
    }
    check(
        groups.len() == 72
            && groups
                .iter()
                .all(|((_, _, _, warm), n)| *n == if *warm { 1 } else { 9 }),
        "1warmup9measured per target/arm",
    )
}
/// Typeset the already-frozen summary values; no statistic is recomputed.
fn report_table(raw: &Path) -> Result<String> {
    let csv = fs::read_to_string(raw.join("summary.csv"))?;
    let mut lines = csv.lines();
    check(lines.next()==Some("case,field_bits,seed,arm,n,complete_min_s,complete_median_s,complete_max_s,driver_min_s,driver_median_s,driver_max_s,oracle_min_s,oracle_median_s,oracle_max_s,parent_a_over_arm_complete,parent_c_over_arm_complete"),"summary header")?;
    let mut table=String::from("\\begin{tabular}{rrrrrrrrr}\\toprule\nBits & Seed & Arm & $n$ & Min (s) & Median (s) & Max (s) & $a$/arm & $c$/arm \\\\ \\midrule\n");
    let mut rows = 0;
    for line in lines {
        let v = line.split(',').collect::<Vec<_>>();
        check(v.len() == 16 && v[4] == "9", "complete36n9summary")?;
        let times = [5, 6, 7]
            .map(|i| v[i].parse::<f64>())
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let ratios = [14, 15]
            .map(|i| v[i].parse::<f64>())
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()?;
        check(
            times
                .iter()
                .chain(&ratios)
                .all(|x| x.is_finite() && *x >= 0.),
            "finite report values",
        )?;
        table.push_str(&format!(
            "{} & {} & {} & {} & {:.6} & {:.6} & {:.6} & {:.3} & {:.3} \\\\\n",
            v[1], v[2], v[3], v[4], times[0], times[1], times[2], ratios[0], ratios[1]
        ));
        rows += 1;
    }
    check(rows == 36, "36summary rows")?;
    table.push_str("\\bottomrule\\end{tabular}\n");
    Ok(table)
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    check(a.len() == 6, "ROOT STUDY OUTPUT UTC EXPORTER_SOURCE")?;
    let root = Path::new(&a[1]);
    let study = Path::new(&a[2]);
    let output = Path::new(&a[3]);
    outside_raw(output, &root.join("main"))?;
    check(
        a[4].len() == 20 && a[4].ends_with('Z') && a[4].as_bytes()[10] == b'T',
        "UTC timestamp",
    )?;
    check(
        root.join("main-supervisor-finished.txt").exists(),
        "supervisor must be terminal",
    )?;
    let terminal = load(&root.join("main-supervisor-terminal.json"))?;
    check(
        terminal["exit_code"] == 0,
        "observed supervisor terminal exit must pass",
    )?;
    let disposition = load(&root.join("main-disposition.json"))?;
    check(
        disposition == json!({"driver_exit":0,"analysis_exit":0,"point_audit_exit":0}),
        "all3phases must pass; partial records remain unchanged",
    )?;
    let post = fs::read_to_string(root.join("main-post-call-check.log"))?;
    check(
        post.lines().count() == 295 && post.lines().all(|l| l.ends_with(": OK")),
        "post-call295pins",
    )?;
    let raw = root.join("main");
    let analysis = load(&raw.join("analysis.json"))?;
    validate_rows(&analysis)?;
    let audit = load(&raw.join("point-audit/summary.json"))?;
    for key in [
        "cells",
        "positive_checks_passed",
        "wrong_scalar_rejected",
        "consistent_substitution_rejected",
    ] {
        check(audit[key] == 360, "360point/control checks per class")?;
    }
    let protocol_path = study.join("protocol.frozen.json");
    let protocol = load(&protocol_path)?;
    check(
        protocol["freeze"]["protocol_sha256"]
            == "5bcd31006a76336912ac3687a371d4bc3f291feeb1cb4058d07e0f2f0645d625",
        "accepted frozen protocol",
    )?;
    let mut runs = Vec::new();
    for r in analysis["rows"].as_array().ok_or("rows")? {
        let id = r["id"].as_str().ok_or("id")?;
        let dir = raw.join(id);
        let columns = r["raw_columns"].as_array().ok_or("columns")?;
        let csv = fs::read_to_string(dir.join("row.csv"))?;
        let actual = csv
            .trim_end()
            .split(',')
            .map(|s| json!(s))
            .collect::<Vec<_>>();
        check(
            actual == *columns,
            "raw CSV versus native analysis,all24tokens",
        )?;
        let status = load(&dir.join("status.json"))?;
        check(status == r["status"], "lossless status readback")?;
        let cert = dir.join(format!("scale-u-{}-{}.json", r["case"], r["seed"]));
        check(
            hash(&fs::read(&cert)?) == r["certificate_sha256"].as_str().ok_or("cert hash")?,
            "native certificate bytes",
        )?;
        let mut artifacts = Vec::new();
        let mut paths = Vec::new();
        collect(&dir, &mut paths)?;
        paths.sort();
        for p in paths {
            artifacts.push(reference(&p, "raw")?);
        }
        for mode in ["positive", "wrong-scalar", "substituted-target"] {
            for stream in ["stdout", "stderr"] {
                artifacts.push(reference(
                    &raw.join("point-audit")
                        .join(format!("{id}.{mode}.{stream}")),
                    "receipt",
                )?);
            }
        }
        let cs = counters(columns, &status)?;
        runs.push(json!({"run_id":id,"arm":r["arm"],"instance_id":r["instance_id"],"seed":r["seed"],"replicate":r["replicate"],"warmup":r["warmup"],"status":"succeeded","outcome_class":"completed","attempts":1,"resumed":0,"checkpoint":null,
            "measurements":{"counted":{"completed_calls":1},"wall_seconds":cs["complete_seconds"],"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":cs},
            "isolation":{"tier":"L0","contended":null,"blockers":["Shared local host; descriptive clocks; fixed worker/replayCPU4"]},"verification":{"status":"verified","verifier":"Recorded Native internal checks/full external replay and separate OpenSSL public-point/scalar checks; unsigned execution provenance"},"exclusion":{"excluded":false,"rule":null},"artifacts":artifacts,"started_at":null,"finished_at":null,"worker":"local-single-worker","backend_ref":id}));
    }
    let mut paths = Vec::new();
    collect(&raw, &mut paths)?;
    for phase in ["08-main-run", "09-main-finalize", "10-main-point-audit"] {
        collect(&root.join(phase), &mut paths)?;
    }
    for name in [
        "main-supervisor-started.txt",
        "main-supervisor-finished.txt",
        "main-supervisor-terminal.json",
        "main-disposition.json",
        "main-post-call-check.log",
        "main-execution-before.sha256",
        "main-pre-call.json",
        "main-command.json",
    ] {
        paths.push(root.join(name));
    }
    paths.sort();
    let mut files = Vec::new();
    for p in paths {
        files.push(reference(&p, "raw")?);
    }
    files.push(reference(&protocol_path, "protocol")?);
    files.push(reference(Path::new(&a[5]), "other")?);
    let host_path = root.join("main-host-before/receipt.json");
    let host = reference(&host_path, "host")?;
    let cpu = fs::read_to_string(root.join("main-host-before/cpuinfo"))?;
    let model = cpu
        .lines()
        .find(|l| l.starts_with("model name"))
        .and_then(|l| l.split_once(':'))
        .map(|(_, v)| v.trim());
    let arms=protocol["arms"].as_array().ok_or("arms")?.iter().map(|r|json!({"id":r["id"],"role":r["role"],"commit":r["instrument"]["commit"],"binary_sha256":r["instrument"]["binary_sha256"]})).collect::<Vec<_>>();
    let table = report_table(&raw)?;
    let table_path = output.with_extension("table.tex");
    outside_raw(&table_path, &raw)?;
    let mut tf = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&table_path)?;
    tf.write_all(table.as_bytes())?;
    files.push(reference(&table_path, "report")?);
    let export = json!({"schema":"experiment.export/v1","export_id":"HASHED-ECDL-CPU-SCRATCH-LOCAL-20261123-LOSSLESS","created":a[4],"protocol":{"path":protocol_path,"sha256":protocol["freeze"]["protocol_sha256"],"protocol_id":protocol["protocol_id"],"version":protocol["version"]},"producer":{"runtime":"codex","model":"Codex","session":"native-cpu-scratch-main-20261123","role":"executor"},"source":{"repo":"isogeny-labs/crypto","commit":"51ac3d745a0a7512ef3622f87fa536ae165c8cec","dirty":true},"backend":"local",
        "host":{"path":host["path"],"sha256":host["sha256"],"class_id":"L0-local-cpu4","summary":{"cpu_model":model,"logical_cpus":5,"memory_bytes":17179869184u64,"kernel":fs::read_to_string(root.join("main-host-before/kernel.txt"))?.trim(),"hypervisor":null,"isolation_tier_max":"L0"}},"arms":arms,"runs":runs,
        "flow":{"planned":360,"attempted":360,"completed":360,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]},"files":files,"analysis":{"script":reference(&study.join("native/src/bin/panel-v2.rs"),"analysis")?,"blinded":false,"blinding_key_sha256":null,"outputs":[reference(&raw.join("analysis.json"),"analysis")?,reference(&raw.join("summary.csv"),"summary")?,reference(&raw.join("latency.svg"),"summary")?],"unmasked_at":null},"backend_record":disposition,
        "notes":[{"kind":"observation","by":"native lossless adapter","text":"360fresh-process calls on12fixed public targets;36warmups/324measured; nine measured observations per arm/target. Armsa/c same parent,b reverse-pass numerator candidate. Full cert bytes/structural counters equal. Nativeu/replicate0/warmupfalse preserved separately from logicalarm/case-seed/round. No preflight,synthetic or historical data pooled."},{"kind":"observation","by":"native lossless adapter","text":"CSV numeric tokens parsed directly with arbitrary_precision; JSON driver clock parsed losslessly and cross-checked against status. Every24-column row token and full status matches original native analysis. No solver,replay or primary timing analysis is executed by this adapter; packaging source was prepared during main and qualified/invoked only afterwards."},{"kind":"observation","by":"native lossless adapter","text":"wall_seconds is internal complete interval; driver_seconds separately encloses worker process plus externalNative replay; oracle_seconds is an included internal subphase. OpenSSL point audit is separate post-run validation. CPUtime,per-processRSS and total-operationS are unmeasured. All wall_validfalse(sharedL0); no ranked timing or exponent inference."},{"kind":"observation","by":"native lossless adapter","text":"Frozen source packet51ac; launch working tree has journal-only uncommitted metadata. Worker source snapshots/solelibdelta and runtime pins are separate exact manifest records. Application checkpoint is pre-run archive only,never a restored worker; source and artifact consistency do not authenticate execution."}],"reviews_dir":"review"});
    let bytes = serde_json::to_vec_pretty(&export)?;
    let readback: Value = serde_json::from_slice(&bytes)?;
    check(readback == export, "lossless serialized JSON readback")?;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    f.write_all(&bytes)?;
    f.write_all(b"\n")?;
    println!("PASS lossless360-cell metadata export; no solver/replay/analysis calls");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_long_fraction_retains_lexeme() {
        let s = "1.9178180999999999";
        assert_eq!(number(s).unwrap().to_string(), s);
    }
    #[test]
    fn integer_beyond_f64_retains_lexeme() {
        let s = "9007199254740993";
        assert_eq!(number(s).unwrap().to_string(), s);
    }
    #[test]
    fn boolean_string_null_are_not_numbers() {
        for s in ["true", "null", "\"0.1\""] {
            assert!(number(s).is_err());
        }
    }
    #[test]
    fn partial_panel_refused_without_manufactured_attempts() {
        assert!(validate_rows(&json!({"planned":360,"attempted":1,"completed":0})).is_err());
    }
    #[test]
    fn resolved_parent_blocks_dotdot_and_symlink_aliases() {
        let p = std::env::temp_dir().join(format!(
            "scratch-export-path-control-{}",
            std::process::id()
        ));
        fs::create_dir(&p).unwrap();
        fs::create_dir(p.join("main")).unwrap();
        fs::create_dir(p.join("outside")).unwrap();
        std::os::unix::fs::symlink(p.join("main"), p.join("alias")).unwrap();
        assert!(outside_raw(&p.join("outside/../main/new.json"), &p.join("main")).is_err());
        assert!(outside_raw(&p.join("alias/new.json"), &p.join("main")).is_err());
        assert!(outside_raw(&p.join("outside/new.json"), &p.join("main")).is_ok());
        fs::remove_dir_all(&p).unwrap();
    }
    #[test]
    fn native_literal_metadata_retained_separately() {
        let mut v = vec![json!("1"); 24];
        for (i, s) in [
            (0, "u"),
            (3, "0"),
            (4, "false"),
            (5, "verified"),
            (11, "1.9178180999999999"),
            (23, "L0"),
        ] {
            v[i] = json!(s);
        }
        let s: Value = serde_json::from_str("{\"driver_seconds\":1.9178180999999999}").unwrap();
        let c = counters(&v, &s).unwrap();
        assert_eq!(c["complete_seconds"].to_string(), "1.9178180999999999");
        assert_eq!(c["driver_seconds"].to_string(), "1.9178180999999999");
        assert_eq!(c["native_replicate"], 0);
        assert_eq!(c["native_warmup"], "false");
    }
}
