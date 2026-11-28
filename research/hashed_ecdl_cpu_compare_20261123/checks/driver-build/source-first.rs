//! Native orchestration/analysis of unchanged scale workers. No solver implementation.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Cell {
    arm: char,
    case: usize,
    seed: u64,
    round: usize,
}
impl Cell {
    fn id(&self) -> String {
        format!("{}-{}-{}-{}", self.arm, self.case, self.seed, self.round)
    }
}
fn check(ok: bool, msg: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(msg.into())
    }
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn write_new(p: impl AsRef<Path>, bytes: &[u8]) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)?;
    f.write_all(bytes)?;
    Ok(())
}
fn schedule(rounds: usize) -> Vec<Cell> {
    let mut out = Vec::new();
    for round in 0..rounds {
        for case in 0..6 {
            for (s, seed) in [101, 202].into_iter().enumerate() {
                for slot in 0..3 {
                    let mut i = (slot + round + case + s) % 3;
                    if round % 2 == 1 {
                        i = 2 - i;
                    }
                    out.push(Cell {
                        arm: ['a', 'b', 'c'][i],
                        case,
                        seed,
                        round,
                    });
                }
            }
        }
    }
    out
}
fn verify_manifest(path: &Path) -> Result<()> {
    for line in fs::read_to_string(path)?.lines() {
        let (expected, file) = line.split_once("  ").ok_or("manifest row")?;
        check(
            sha(&fs::read(file)?) == expected,
            "frozen binary/source changed",
        )?;
    }
    Ok(())
}
fn row(text: &str, c: &Cell) -> Result<Vec<String>> {
    check(text.lines().count() == 1, "one raw row required")?;
    let v: Vec<_> = text.trim_end().split(',').map(str::to_owned).collect();
    check(v.len() == 24, "raw width")?;
    check(
        v[0] == "u"
            && v[1] == c.case.to_string()
            && v[2] == c.seed.to_string()
            && v[3] == "0"
            && v[4] == "false"
            && v[5] == "verified"
            && v[23] == "L0",
        "native cell binding",
    )?;
    for t in &v[11..18] {
        let f = t.parse::<f64>()?;
        check(f.is_finite() && f >= 0., "nonfinite/negative clock")?;
    }
    for i in [6, 7, 8, 9, 10, 18, 19, 20, 21, 22] {
        v[i].parse::<u64>()?;
    }
    Ok(v)
}
fn signature(v: &[String]) -> Vec<String> {
    [8, 9, 10, 18, 19, 20, 21, 22]
        .into_iter()
        .map(|i| v[i].clone())
        .collect()
}
fn run(out: &Path, config: &Value, rounds: usize) -> Result<()> {
    check(
        rounds > 0 && rounds <= 10 && config["rounds"].as_u64() == Some(rounds as u64),
        "round count must match fixed config",
    )?;
    fs::create_dir(out)?;
    verify_manifest(Path::new(config["manifest"].as_str().ok_or("manifest")?))?;
    let plan = schedule(rounds);
    write_new(
        out.join("schedule.tsv"),
        plan.iter()
            .map(|c| {
                format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    c.id(),
                    c.arm,
                    c.case,
                    c.seed,
                    c.round
                )
            })
            .collect::<String>()
            .as_bytes(),
    )?;
    write_new(out.join("config.json"), &serde_json::to_vec_pretty(config)?)?;
    let mut events = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("events.jsonl"))?;
    for c in plan {
        let dir = out.join(c.id());
        fs::create_dir(&dir)?;
        writeln!(
            events,
            "{}",
            json!({"event":"begin","id":c.id(),"arm":c.arm.to_string(),"case":c.case,"seed":c.seed,"round":c.round})
        )?;
        events.flush()?;
        write_new(dir.join("started"), b"attempted\n")?;
        let worker = config[if c.arm == 'b' { "candidate" } else { "parent" }]
            .as_str()
            .ok_or("worker")?;
        let start = Instant::now();
        let status = Command::new(config["worker_wrapper"].as_str().ok_or("wrapper")?)
            .args([
                worker,
                "u",
                &c.case.to_string(),
                &c.seed.to_string(),
                dir.to_str().ok_or("out path")?,
            ])
            .stdout(Stdio::from(
                fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(dir.join("row.csv"))?,
            ))
            .stderr(Stdio::from(
                fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(dir.join("worker.stderr"))?,
            ))
            .status()?;
        let mut replay = None;
        if status.success() {
            replay = Some(
                Command::new("timeout")
                    .args([
                        "--signal=TERM",
                        "--kill-after=5",
                        "60",
                        "taskset",
                        "-c",
                        "4",
                        config["verifier"].as_str().ok_or("verifier")?,
                        "--verify",
                        dir.join(format!("scale-u-{}-{}.json", c.case, c.seed))
                            .to_str()
                            .ok_or("cert path")?,
                    ])
                    .stdout(Stdio::from(
                        fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(dir.join("verify.stdout"))?,
                    ))
                    .stderr(Stdio::from(
                        fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(dir.join("verify.stderr"))?,
                    ))
                    .status()?,
            );
        }
        let elapsed = start.elapsed().as_secs_f64();
        let receipt = json!({"worker_exit":status.code(),"worker_success":status.success(),"replay_exit":replay.as_ref().and_then(|s|s.code()),"replay_success":replay.as_ref().map(|s|s.success()).unwrap_or(false),"driver_seconds":elapsed,"native_arm":"u","native_replicate":0,"native_warmup":false});
        write_new(
            dir.join("status.json"),
            &serde_json::to_vec_pretty(&receipt)?,
        )?;
        writeln!(
            events,
            "{}",
            json!({"event":"end","id":c.id(),"receipt":receipt})
        )?;
        events.flush()?;
        println!(
            "END {} worker={} replay={}",
            c.id(),
            status.success(),
            receipt["replay_success"]
        );
        check(
            status.success() && receipt["replay_success"] == true,
            "cell failed; stop without retry",
        )?;
        row(&fs::read_to_string(dir.join("row.csv"))?, &c)?;
    }
    verify_manifest(Path::new(config["manifest"].as_str().ok_or("manifest")?))?;
    Ok(())
}
fn stats(v: &mut [f64]) -> Result<(f64, f64, f64)> {
    check(
        !v.is_empty() && v.iter().all(|f| f.is_finite() && *f >= 0.),
        "invalid samples",
    )?;
    v.sort_by(f64::total_cmp);
    Ok((v[0], v[v.len() / 2], v[v.len() - 1]))
}
fn ratio(n: Option<f64>, d: f64) -> String {
    if d > 0. {
        n.map(|n| (n / d).to_string()).unwrap_or_default()
    } else {
        String::new()
    }
}
fn analyze(out: &Path, rounds: usize) -> Result<()> {
    let config: Value = serde_json::from_slice(&fs::read(out.join("config.json"))?)?;
    check(
        rounds > 0 && rounds <= 10 && config["rounds"].as_u64() == Some(rounds as u64),
        "analysis rounds must match fixed config",
    )?;
    let expected = schedule(rounds);
    let mut records = Vec::new();
    let mut attempted = 0;
    let mut complete = 0;
    let mut errors = Vec::new();
    let mut baseline: BTreeMap<(usize, u64), (Vec<u8>, Vec<String>)> = BTreeMap::new();
    let mut groups: BTreeMap<(usize, u64, char), Vec<(f64, f64, f64)>> = BTreeMap::new();
    for c in &expected {
        let dir = out.join(c.id());
        let started = dir.join("started").exists();
        if started {
            attempted += 1;
        }
        let mut record = json!({"id":c.id(),"arm":c.arm.to_string(),"case":c.case,"instance_id":format!("{}-{}",c.case,c.seed),"seed":c.seed,"replicate":c.round,"warmup":c.round==0,"attempted":started});
        let read = (|| -> Result<(Value, Vec<String>, Vec<u8>)> {
            check(started, "not attempted")?;
            let status: Value = serde_json::from_slice(&fs::read(dir.join("status.json"))?)?;
            check(
                status["worker_success"] == true && status["replay_success"] == true,
                "worker/replay failed",
            )?;
            let v = row(&fs::read_to_string(dir.join("row.csv"))?, c)?;
            let cert = fs::read(dir.join(format!("scale-u-{}-{}.json", c.case, c.seed)))?;
            check(!cert.is_empty(), "empty certificate")?;
            Ok((status, v, cert))
        })();
        match read {
            Ok((status, v, cert)) => {
                let sig = signature(&v);
                if let Some((old, structural)) = baseline.get(&(c.case, c.seed)) {
                    if old != &cert || structural != &sig {
                        errors.push(format!("certificate/structural mismatch {}", c.id()));
                    }
                } else {
                    baseline.insert((c.case, c.seed), (cert.clone(), sig));
                }
                complete += 1;
                record["status"] = status.clone();
                record["raw_columns"] = json!(v);
                record["certificate_sha256"] = json!(sha(&cert));
                if c.round > 0 {
                    groups.entry((c.case, c.seed, c.arm)).or_default().push((
                        v[11].parse()?,
                        status["driver_seconds"].as_f64().ok_or("driver clock")?,
                        v[14].parse()?,
                    ));
                }
            }
            Err(e) => {
                record["failure"] = json!(e.to_string());
                errors.push(format!("{}: {e}", c.id()));
            }
        }
        records.push(record);
    }
    let mut csv=String::from("case,field_bits,seed,arm,n,complete_min_s,complete_median_s,complete_max_s,driver_min_s,driver_median_s,driver_max_s,oracle_min_s,oracle_median_s,oracle_max_s,parent_a_over_arm_complete,parent_c_over_arm_complete\n");
    let mut summaries = BTreeMap::new();
    for (key, values) in &groups {
        let mut a = values.iter().map(|v| v.0).collect::<Vec<_>>();
        let mut b = values.iter().map(|v| v.1).collect::<Vec<_>>();
        let mut d = values.iter().map(|v| v.2).collect::<Vec<_>>();
        summaries.insert(
            *key,
            (stats(&mut a)?, stats(&mut b)?, stats(&mut d)?, values.len()),
        );
    }
    let mut graph=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"1180\"><rect width=\"1000\" height=\"1180\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"14\"><text x=\"20\" y=\"25\">CPU parent / numerator-scratch candidate: complete seconds, median [full range]</text><text x=\"20\" y=\"48\">Fixed public targets; arms a/c parent A/A, b candidate; shared-host L0 diagnostics</text>");
    let max = summaries.values().map(|v| v.0 .2).fold(0.001, f64::max);
    for (index, ((case, seed, arm), (a, b, d, n))) in summaries.iter().enumerate() {
        let pa = summaries.get(&(*case, *seed, 'a')).map(|v| v.0 .1);
        let pc = summaries.get(&(*case, *seed, 'c')).map(|v| v.0 .1);
        csv.push_str(&format!(
            "{case},{},{seed},{arm},{n},{},{},{},{},{},{},{},{},{},{},{}\n",
            20 + 4 * case,
            a.0,
            a.1,
            a.2,
            b.0,
            b.1,
            b.2,
            d.0,
            d.1,
            d.2,
            ratio(pa, a.1),
            ratio(pc, a.1)
        ));
        let y = 75 + 30 * index;
        let color = if *arm == 'b' { "#26794f" } else { "#3678a8" };
        graph.push_str(&format!("<text x=\"20\" y=\"{}\">{}b/{seed}/{arm} n={n}</text><rect x=\"220\" y=\"{y}\" width=\"{}\" height=\"18\" fill=\"{color}\"/><line x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\" stroke=\"black\"/><text x=\"810\" y=\"{}\">{:.6}</text>",y+14,20+4*case,550.*a.1/max,220.+550.*a.0/max,220.+550.*a.2/max,y+9,y+9,y+14,a.1));
    }
    graph.push_str("</g></svg>");
    let result = json!({"schema":"native-cpu-scratch-analysis/v1","planned":expected.len(),"attempted":attempted,"completed":complete,"not_attempted":expected.len()-attempted,"failed_or_incomplete":attempted-complete,"distinct_targets":baseline.len(),"warmups_per_arm_target":1,"measured_per_arm_target":rounds.saturating_sub(1),"exact_certificate_and_counter_equality":errors.is_empty()&&complete==expected.len(),"errors":errors,"wall_claim":"L0 descriptive only; no ranked timing inference, S or total-work price","rows":records});
    write_new(
        out.join("analysis.json"),
        &serde_json::to_vec_pretty(&result)?,
    )?;
    write_new(out.join("summary.csv"), csv.as_bytes())?;
    write_new(out.join("latency.svg"), graph.as_bytes())?;
    println!(
        "planned={} attempted={attempted} complete={complete} exact_equal={} errors={}",
        expected.len(),
        result["exact_certificate_and_counter_equality"],
        result["errors"].as_array().ok_or("errors")?.len()
    );
    check(
        result["exact_certificate_and_counter_equality"] == true,
        "panel incomplete or mismatched; preserved outputs",
    )
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    check(a.len() >= 4, "run OUT CONFIG ROUNDS | analyze OUT ROUNDS")?;
    let out = PathBuf::from(&a[2]);
    match a[1].as_str() {
        "run" => {
            check(a.len() == 5, "run args")?;
            let c: Value = serde_json::from_slice(&fs::read(&a[3])?)?;
            run(&out, &c, a[4].parse()?)
        }
        "analyze" => analyze(&out, a[3].parse()?),
        _ => Err("unknown mode".into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn complete_schedule_is_unique() {
        let s = schedule(10);
        assert_eq!(s.len(), 360);
        assert_eq!(s.iter().cloned().collect::<BTreeSet<_>>().len(), 360);
    }
    #[test]
    fn each_target_has_one_warmup_nine_measurements_per_arm() {
        let mut n = BTreeMap::new();
        for c in schedule(10) {
            *n.entry((c.case, c.seed, c.arm, c.round == 0)).or_insert(0) += 1;
        }
        assert_eq!(n.len(), 72);
        for ((_, _, _, warmup), count) in n {
            assert_eq!(count, if warmup { 1 } else { 9 });
        }
    }
    #[test]
    fn every_block_has_all_three_arms() {
        for block in schedule(10).chunks_exact(3) {
            assert_eq!(
                block.iter().map(|c| c.arm).collect::<BTreeSet<_>>(),
                BTreeSet::from(['a', 'b', 'c'])
            );
        }
    }
    #[test]
    fn median_and_range_are_correct() {
        assert_eq!(stats(&mut [7., 1., 3., 9., 5.]).unwrap(), (1., 5., 9.));
        assert!(stats(&mut [f64::NAN]).is_err());
    }
    #[test]
    fn fractional_json_lexemes_preserve() {
        let s = "{\"time\":1.9178180999999999}";
        let x: Value = serde_json::from_str(s).unwrap();
        assert_eq!(serde_json::to_string(&x).unwrap(), s);
    }
    #[test]
    fn wrong_native_identity_rejected() {
        let c = Cell {
            arm: 'b',
            case: 0,
            seed: 101,
            round: 1,
        };
        let s = "x,0,101,0,false,verified,20,14,1,1,1,0.1,0,0,0,0,0,0,1,1,1,1,1,L0";
        assert!(row(s, &c).is_err());
    }
    #[test]
    fn missing_panel_writes_flow_and_never_promotes() {
        let p = std::env::temp_dir().join(format!("cpu-panel-missing-{}", std::process::id()));
        fs::create_dir(&p).unwrap();
        write_new(p.join("config.json"), b"{\"rounds\":2}").unwrap();
        assert!(analyze(&p, 2).is_err());
        let j: Value = serde_json::from_slice(&fs::read(p.join("analysis.json")).unwrap()).unwrap();
        assert_eq!(j["planned"], 72);
        assert_eq!(j["attempted"], 0);
        assert_eq!(j["completed"], 0);
        assert_eq!(j["exact_certificate_and_counter_equality"], false);
    }
}
