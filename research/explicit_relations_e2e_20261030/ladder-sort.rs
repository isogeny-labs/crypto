// Frozen native recovery ladder. Every failed/limited/timeout execution is a row.
use std::{
    env, fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    time::Instant,
};
fn main() {
    let args: Vec<String> = env::args().collect();
    let out = Path::new(&args[1]);
    let driver = &args[2];
    let gpu = &args[3];
    let baseline = &args[4];
    let target = &args[5];
    let dense_thrust = &args[6];
    let curves = fs::read_to_string(out.join("curves.txt")).unwrap();
    let mut csv = fs::File::create(out.join("rows.csv")).unwrap();
    writeln!(csv,"arm,instance_id,seed,replicate,warmup,status,verification,recoveries,wall_seconds,solver_seconds,verify_seconds,relations,attempts,bases_ms,targets_ms,oracle_ms,linear_ms,replay_ms,tier").unwrap();
    let mut complete_bits = 0;
    for line in curves.lines() {
        let v: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(v.len(), 7);
        let (bits, p, a, b, r, px, py) = (v[0], v[1], v[2], v[3], v[4], v[5], v[6]);
        let mut passed = true;
        for (replicate, seed) in [101u64, 202].into_iter().enumerate() {
            let arms = if replicate == 0 {
                if bits <= 52 {
                    vec!["b", "u", "v", "x"]
                } else {
                    vec!["v"]
                }
            } else {
                if bits <= 52 {
                    vec!["x", "v", "u", "b"]
                } else {
                    vec!["v"]
                }
            };
            for arm in arms {
                let id = format!("{bits}-{seed}-{arm}");
                let start = Instant::now();
                let q = Command::new(target)
                    .args([
                        p.to_string(),
                        a.to_string(),
                        b.to_string(),
                        (seed * 1009).to_string(),
                    ])
                    .output()
                    .unwrap();
                assert!(q.status.success());
                let qs = String::from_utf8(q.stdout).unwrap();
                let qv: Vec<&str> = qs.split_whitespace().collect();
                assert_eq!(qv.len(), 2);
                // ceil(r^(1/8)), computed exactly with saturating products.
                let (mut lo, mut hi) = (0u64, r);
                while lo + 1 < hi {
                    let mid = (lo + hi) / 2;
                    let z = (0..8).fold(1u128, |x, _| x.saturating_mul(mid as u128));
                    if z < r as u128 {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                let tau = 2 * hi + 2;
                let params = vec![
                    p.to_string(),
                    a.to_string(),
                    b.to_string(),
                    r.to_string(),
                    px.to_string(),
                    py.to_string(),
                    qv[0].to_owned(),
                    qv[1].to_owned(),
                    "8".into(),
                    tau.to_string(),
                    seed.to_string(),
                    "4294967296".into(),
                    "1048576".into(),
                    "16384".into(),
                    "17179869184".into(),
                    "16".into(),
                ];
                fs::write(out.join(format!("{id}.input")), params.join(" ") + "\n").unwrap();
                let cert = out.join(format!("{id}.cert"));
                let log = out.join(format!("{id}.log"));
                let result = Command::new("timeout")
                    .args([
                        "--signal=TERM",
                        "--kill-after=5s",
                        if bits <= 52 {
                            "180"
                        } else if bits == 61 {
                            "2700"
                        } else {
                            "5400"
                        },
                        driver,
                        if arm == "b" {
                            baseline
                        } else if arm == "u" {
                            dense_thrust
                        } else {
                            gpu
                        },
                    ])
                    .args(&params)
                    .stdout(Stdio::from(fs::File::create(&cert).unwrap()))
                    .stderr(Stdio::from(fs::File::create(&log).unwrap()))
                    .status()
                    .unwrap();
                let solver_seconds = start.elapsed().as_secs_f64();
                let mut verified = false;
                let mut verify_seconds = 0.0;
                let logtext = fs::read_to_string(&log).unwrap();
                let status = if result.success() {
                    let tick = Instant::now();
                    let replay = Command::new("timeout")
                        .args(["60", driver, "--verify"])
                        .arg(&cert)
                        .output()
                        .unwrap();
                    verify_seconds = tick.elapsed().as_secs_f64();
                    fs::write(out.join(format!("{id}.verify")), &replay.stdout).unwrap();
                    assert!(replay.status.success(), "certificate replay failed: {id}");
                    let text = fs::read_to_string(&cert).unwrap();
                    let summary = text.lines().find(|s| s.starts_with("summary ")).unwrap();
                    let mut words: Vec<String> =
                        summary.split_whitespace().map(str::to_owned).collect();
                    let scalar: u64 = words[1].parse().unwrap();
                    assert!(scalar < r);
                    words[1] = ((scalar + 1) % r).to_string();
                    let corrupt = out.join(format!("{id}.corrupt"));
                    fs::write(&corrupt, text.replacen(summary, &words.join(" "), 1)).unwrap();
                    let negative = Command::new("timeout")
                        .args(["60", driver, "--verify"])
                        .arg(&corrupt)
                        .output()
                        .unwrap();
                    assert_eq!(
                        negative.status.code(),
                        Some(1),
                        "corrupt certificate accepted or timed out"
                    );
                    fs::write(out.join(format!("{id}.negative")), negative.stderr).unwrap();
                    fs::remove_file(corrupt).unwrap();
                    let canonical = out.join(format!("{bits}-{seed}.canonical"));
                    if canonical.exists() {
                        assert_eq!(
                            fs::read(&canonical).unwrap(),
                            text.as_bytes(),
                            "cross-arm certificate differs"
                        );
                    } else {
                        fs::write(canonical, &text).unwrap();
                    }
                    verified = true;
                    "succeeded"
                } else if [Some(124), Some(137)].contains(&result.code()) {
                    passed = false;
                    "timeout"
                } else if logtext.contains("Limit(") || logtext.contains("LIMIT") {
                    passed = false;
                    "limited"
                } else {
                    panic!("non-limit solver failure {id}: {logtext}");
                };
                let seconds = start.elapsed().as_secs_f64();
                let mut fields = std::collections::BTreeMap::new();
                if let Some(line) = logtext.lines().find(|s| s.starts_with("E2E ")) {
                    for word in line.split_whitespace().skip(1) {
                        if let Some((k, v)) = word.split_once('=') {
                            fields.insert(k, v);
                        }
                    }
                }
                let get = |k| fields.get(k).copied().unwrap_or("");
                writeln!(csv,"{arm},{bits},{seed},{replicate},false,{status},{},{},{seconds},{solver_seconds},{verify_seconds},{},{},{},{},{},{},{},L0",
                    if verified{"verified"}else{"not_run"},u8::from(verified),get("relations"),get("attempts"),get("bases_ms"),get("targets_ms"),get("oracle_ms"),get("linear_ms"),get("replay_ms")).unwrap();
                csv.flush().unwrap();
                println!(
                    "CELL {id} {status} total_seconds={seconds:.3} oracle_ms={} linear_ms={}",
                    get("oracle_ms"),
                    get("linear_ms")
                );
            }
        }
        if !passed {
            println!("FRONTIER stopped at {bits} bits after completing this rung; largest complete rung={complete_bits}");
            break;
        }
        complete_bits = bits;
    }
    fs::write(
        out.join("frontier.txt"),
        format!("largest_complete_bits={complete_bits}\n"),
    )
    .unwrap();
}
