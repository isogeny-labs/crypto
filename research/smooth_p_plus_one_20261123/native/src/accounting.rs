//! Development-only cold-recovery accounting on the two public toy fixtures.
//! No timer or partial counter in this module is an admitted benchmark result.

use super::*;
use prime31_four_summand::relation_adapter::{RelationAdapter, SignedRelation};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::FileExt;
use std::os::unix::process::CommandExt;
use std::process::{Child, ExitStatus};

const PHASES: [&str; 8] = [
    "input_validation",
    "base_setup",
    "oracle_setup",
    "consumer_setup",
    "mask_generation",
    "oracle_build_solve_extract",
    "consume_relation",
    "final_verification",
];
const RECORD_CAP: usize = 1 << 20;
const OUTPUT_CAP: u64 = 32 << 20;
const CHILD_LIMIT: Duration = Duration::from_secs(120);

struct Ledger(BTreeMap<&'static str, u64>);

impl Ledger {
    fn new() -> Self {
        Self(
            PHASES
                .into_iter()
                .chain(["record_emission"])
                .map(|key| (key, 0))
                .collect(),
        )
    }

    fn measure<T>(&mut self, key: &'static str, work: impl FnOnce() -> T) -> T {
        let start = Instant::now();
        let result = work();
        *self.0.get_mut(key).expect("declared phase") += nanos(start.elapsed());
        result
    }

    fn finish(&self, start: Instant) -> Value {
        let total = nanos(start.elapsed());
        let attributed: u64 = self.0.values().sum();
        json!({"phase_ns":self.0, "unattributed_ns":total.checked_sub(attributed).expect("disjoint timers"),
            "total_to_summary_ns":total,
            "excludes":"summary serialization and process teardown; parent receipt includes both"})
    }
}

fn nanos(duration: Duration) -> u64 {
    duration
        .as_nanos()
        .try_into()
        .expect("interval fits u64 nanoseconds")
}

fn emit(writer: &mut impl Write, value: &Value) -> Result<(), String> {
    let mut bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    if bytes.len() > RECORD_CAP {
        return Err("accounting record exceeds 1 MiB; no partial record emitted".into());
    }
    writer
        .write_all(&bytes)
        .and_then(|_| writer.flush())
        .map_err(|e| e.to_string())
}

#[derive(Clone, Copy)]
struct Run<'a> {
    curve: Fixture,
    depth: usize,
    solver: &'a Path,
    method: Option<Method>,
    seed: u64,
    draws: usize,
}

impl Run<'_> {
    fn label(self) -> String {
        self.method.map_or_else(|| "Direct".into(), Method::name)
    }

    fn scope(self) -> &'static str {
        if self.method.is_some_and(|m| m.encoding() == Mode::Distinct) {
            "four_distinct_coarse_images_target_disjoint"
        } else {
            "four_distinct_x_target_disjoint"
        }
    }
}

fn run(spec: Run<'_>, start: Instant, writer: &mut impl Write) -> Result<Value, String> {
    let Run {
        curve,
        depth,
        solver,
        method,
        seed,
        draws: max_draws,
    } = spec;
    let mut ledger = Ledger::new();
    ledger.measure("input_validation", || {
        if curve.multiply(curve.n, curve.g).is_some() || curve.multiply(curve.n, curve.q).is_some()
        {
            Err("fixture subgroup validation failed")
        } else {
            Ok(())
        }
    })?;
    let base = ledger.measure("base_setup", || Base::new(curve, depth));
    let (scratch, pair_index) = ledger.measure("oracle_setup", || {
        (Scratch::new(), method.is_none().then(|| base.pair_index()))
    });
    let scratch = scratch?;
    let mut consumer = ledger
        .measure("consumer_setup", || {
            let (public_curve, points) = production::public_input(&base);
            RelationAdapter::new(
                public_curve,
                points,
                Some((curve.q.x as u64, curve.q.y as u64)),
            )
        })
        .map_err(|e| e.to_string())?;
    let mut rng = seed;
    let mut seen = BTreeSet::new();
    let (mut draws, mut queries, mut selected, mut incomplete, mut skipped) = (0, 0, 0, 0, 0);
    let mut fatal = None;
    for draw in 1..=max_draws {
        draws = draw;
        let (a, b, residual, fresh) = ledger.measure("mask_generation", || {
            let a = next_mask(&mut rng, curve.n);
            let b = next_mask(&mut rng, curve.n);
            let residual = curve.add(curve.multiply(a, curve.g), curve.multiply(b, curve.q));
            (a, b, residual, seen.insert(residual))
        });
        let rank_before = consumer.rank();
        let mut systems = vec![];
        let mut charts = None;
        let result = if residual.is_none() || !fresh {
            skipped += 1;
            json!({"status":"skipped", "reason":if residual.is_none() {"identity"} else {"duplicate_target"}})
        } else {
            queries += 1;
            let target = residual.unwrap();
            let rows = ledger.measure("oracle_build_solve_extract", || {
                if let Some(method) = method {
                    let found = discover(
                        &base,
                        target,
                        method,
                        solver,
                        &scratch,
                        Duration::from_secs(3),
                    );
                    systems = found.systems;
                    charts = Some((found.planned, found.completed));
                    found.result
                } else {
                    Ok(base.reference(pair_index.as_ref().unwrap(), target))
                }
            });
            match rows {
                Err(error) => {
                    incomplete += 1;
                    json!({"status":"undetermined", "reason":error})
                }
                Ok(rows) => {
                    let row = rows.first().copied();
                    let receipt = if let Some(row) = row {
                        selected += 1;
                        let accepted = ledger.measure("consume_relation", || {
                            let relation = SignedRelation {
                                terms: row
                                    .into_iter()
                                    .map(|label| {
                                        (
                                            label.unsigned_abs() as usize - 1,
                                            i64::from(label.signum()),
                                        )
                                    })
                                    .collect(),
                            };
                            consumer.accept(&relation, a as u64, b as u64)
                        });
                        match accepted {
                            Ok(receipt) => json!(receipt),
                            Err(error) => {
                                fatal = Some(error.to_string());
                                json!({"error":error})
                            }
                        }
                    } else {
                        Value::Null
                    };
                    json!({"status":if fatal.is_some() {"consumer_error"} else {"complete_query"},
                        "relations":rows.len(), "selected":row, "consumer":receipt})
                }
            }
        };
        ledger.measure("record_emission", || {
            emit(
                writer,
                &json!({
                    "kind":"account_attempt", "format_version":1, "mode":spec.label(), "seed":seed,
                    "prime":curve.p, "depth":depth, "draw":draw, "a":a, "b":b,
                    "target":residual.map(|p| [p.x,p.y]), "result":result, "systems":systems,
                    "chart_counts":charts, "rank_before":rank_before, "rank_after":consumer.rank(),
                }),
            )
        })?;
        if fatal.is_some() || consumer.scalar().is_some() {
            break;
        }
    }
    let scalar = consumer.scalar();
    let replay = scalar.map(|k| {
        ledger.measure("final_verification", || {
            curve.multiply(k as i64, curve.g) == Some(curve.q)
        })
    });
    if replay == Some(false) {
        fatal = Some("fresh final scalar replay failed".into());
    }
    // Prepare summary metadata BEFORE closing the child interval. Only its final
    // timing-value insertion, serialization and teardown are left to the parent.
    let mut summary = json!({
        "kind":"account_summary", "format_version":1, "mode":spec.label(), "seed":seed,
        "prime":curve.p, "depth":depth, "scope":spec.scope(),
        "status":if fatal.is_some() {"error"} else if scalar.is_some() {"recovered"} else {"incomplete"},
        "error":fatal, "draws":draws, "max_draws":max_draws, "calls":queries,
        "skipped":skipped, "incomplete_queries":incomplete, "selected":selected,
        "base_size":base.unsigned.len(), "consumer_rank":consumer.rank(),
        "rank_coordinates":"base coefficients plus the target scalar", "scalar":scalar,
        "final_scalar_replay":replay, "consumer_work":consumer.work(),
        "consumer_arithmetic":consumer.arithmetic_counts(),
        "consumer_setup_arithmetic":consumer.setup_arithmetic_counts(),
        "counter_scope":"partial consumer counters; not a total arithmetic price",
        "accounting":"local_development_only", "query_budget_ms":3000,
        "consumer_interval":"canonicalization, equality replay, dedup, rank, linear algebra and consumer scalar replay",
    });
    summary["wall"] = ledger.finish(start);
    emit(writer, &summary)?;
    if let Some(error) = fatal {
        Err(error)
    } else {
        Ok(summary)
    }
}

fn validate_summary(spec: Run<'_>, records: &[Value], parent_ns: u64) -> Result<(), String> {
    let summaries: Vec<_> = records
        .iter()
        .filter(|v| v["kind"] == "account_summary")
        .collect();
    if summaries.len() != 1 || records.last() != summaries.first().copied() {
        return Err("expected exactly one final child summary".into());
    }
    let summary = summaries[0];
    if summary["format_version"] != 1
        || summary["mode"] != spec.label()
        || summary["prime"] != spec.curve.p
        || summary["depth"] != spec.depth
        || summary["seed"] != spec.seed
        || summary["max_draws"] != spec.draws
        || summary["scope"] != spec.scope()
        || summary["accounting"] != "local_development_only"
    {
        return Err("child summary identity mismatch".into());
    }
    let recovered = summary["status"] == "recovered";
    if !(recovered || summary["status"] == "incomplete")
        || (recovered
            && (summary["scalar"].as_u64().is_none() || summary["final_scalar_replay"] != true))
        || (!recovered
            && (!summary["scalar"].is_null() || !summary["final_scalar_replay"].is_null()))
    {
        return Err("invalid child outcome".into());
    }
    let draws = summary["draws"].as_u64().ok_or("missing draw count")?;
    if draws == 0
        || draws > spec.draws as u64
        || draws + 1 != records.len() as u64
        || (!recovered && draws != spec.draws as u64)
    {
        return Err("missing or extra draw records".into());
    }
    let (mut queries, mut skipped, mut incomplete, mut selected) = (0u64, 0u64, 0u64, 0u64);
    for (i, record) in records[..records.len() - 1].iter().enumerate() {
        if record["kind"] != "account_attempt"
            || record["draw"] != i + 1
            || record["mode"] != spec.label()
            || record["seed"] != spec.seed
            || record["prime"] != spec.curve.p
            || record["depth"] != spec.depth
        {
            return Err("child draw identity mismatch".into());
        }
        match record["result"]["status"].as_str() {
            Some("skipped") => skipped += 1,
            Some("undetermined") => {
                queries += 1;
                incomplete += 1;
            }
            Some("complete_query") => {
                queries += 1;
                selected += u64::from(!record["result"]["selected"].is_null());
            }
            _ => return Err("invalid draw status".into()),
        }
    }
    for (key, count) in [
        ("calls", queries),
        ("skipped", skipped),
        ("incomplete_queries", incomplete),
        ("selected", selected),
    ] {
        if summary[key] != count {
            return Err(format!("child {key} count mismatch"));
        }
    }
    let wall = &summary["wall"];
    let total = wall["total_to_summary_ns"]
        .as_u64()
        .ok_or("missing child wall interval")?;
    let phases = wall["phase_ns"]
        .as_object()
        .ok_or("missing child phase ledger")?;
    if phases.len() != PHASES.len() + 1
        || PHASES
            .into_iter()
            .chain(["record_emission"])
            .any(|key| !phases.contains_key(key))
    {
        return Err("missing or unexpected phase".into());
    }
    let sum = phases.values().try_fold(
        wall["unattributed_ns"]
            .as_u64()
            .ok_or("missing unattributed time")?,
        |sum, v| {
            sum.checked_add(v.as_u64().ok_or("invalid phase time")?)
                .ok_or("phase time overflow")
        },
    )?;
    if total != sum || total > parent_ns {
        return Err("child accounting interval does not reconcile".into());
    }
    Ok(())
}

fn wait_bounded(
    child: &mut Child,
    start: Instant,
    limit: Duration,
    paths: &[&Path],
) -> Result<(ExitStatus, Option<String>), String> {
    loop {
        let mut stop_reason = match child.try_wait() {
            Ok(Some(status)) => return Ok((status, None)),
            Ok(None) => None,
            Err(error) => Some(format!("child wait failed: {error}")),
        };
        if start.elapsed() >= limit {
            stop_reason = Some("child deadline exceeded".into());
        }
        for path in paths {
            match fs::metadata(path) {
                Ok(meta) if meta.len() > OUTPUT_CAP => {
                    stop_reason = Some("child output cap exceeded".into())
                }
                Err(error) => stop_reason = Some(format!("child output unavailable: {error}")),
                _ => {}
            }
        }
        if stop_reason.is_some() {
            // Child owns a new process group: kill its external solver too.
            let killed = Command::new("/bin/kill")
                .args(["-KILL", "--", &format!("-{}", child.id())])
                .status();
            if !matches!(killed,Ok(status) if status.success()) {
                let _ = child.kill();
            }
            return child
                .wait()
                .map(|status| (status, stop_reason))
                .map_err(|e| e.to_string());
        }
        thread::sleep(Duration::from_millis(5));
    }
}

fn terminated_output(path: &Path) -> Result<(), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let len = file.metadata().map_err(|e| e.to_string())?.len();
    if len == 0 {
        return Err("empty child output".into());
    }
    let mut last = [0];
    file.read_exact_at(&mut last, len - 1)
        .map_err(|e| e.to_string())?;
    if last != [b'\n'] {
        return Err("unterminated child record".into());
    }
    Ok(())
}

fn suite(spec: Run<'_>, labels: &[String], writer: &mut impl Write) -> Result<(), String> {
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let scratch = Scratch::new()?;
    for (index, label) in labels.iter().enumerate() {
        let child_spec = Run {
            method: if label == "direct" { None } else { mode(label) },
            ..spec
        };
        let stdout_path = scratch.0.join("child.jsonl");
        let stderr_path = scratch.0.join("child.stderr");
        let mut command = Command::new(&executable);
        command
            .args([spec.curve.p.to_string(), spec.depth.to_string()])
            .arg(spec.solver)
            .args([
                format!("account={}", spec.seed),
                format!("draws={}", spec.draws),
                label.clone(),
            ])
            .env("RAYON_NUM_THREADS", "1")
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(File::create(&stdout_path).map_err(|e| e.to_string())?)
            .stderr(File::create(&stderr_path).map_err(|e| e.to_string())?);
        let start = Instant::now();
        let mut child = command.spawn().map_err(|e| e.to_string())?;
        let (status, stop_reason) = wait_bounded(
            &mut child,
            start,
            CHILD_LIMIT,
            &[&stdout_path, &stderr_path],
        )?;
        let parent_ns = nanos(start.elapsed());
        let mut records = vec![];
        let mut parse_error = terminated_output(&stdout_path).err();
        let reader = BufReader::new(
            File::open(&stdout_path)
                .map_err(|e| e.to_string())?
                .take(OUTPUT_CAP + 1),
        );
        for line in reader.split(b'\n') {
            let line = line.map_err(|e| e.to_string())?;
            if line.is_empty() {
                continue;
            }
            if line.len() + 1 > RECORD_CAP {
                parse_error = Some("child record exceeded cap".to_string());
                break;
            }
            match serde_json::from_slice::<Value>(&line) {
                Ok(record) => {
                    emit(writer, &record)?;
                    records.push(record);
                }
                Err(error) => {
                    parse_error = Some(format!("invalid child JSON: {error}"));
                    emit(
                        writer,
                        &json!({"kind":"account_invalid_output", "mode":child_spec.label(),
                        "raw_prefix":String::from_utf8_lossy(&line[..line.len().min(2048)]),
                        "omitted_bytes":line.len().saturating_sub(2048)}),
                    )?;
                    break;
                }
            }
        }
        let mut stderr = vec![];
        File::open(&stderr_path)
            .map_err(|e| e.to_string())?
            .take(65_536)
            .read_to_end(&mut stderr)
            .map_err(|e| e.to_string())?;
        let stderr_bytes = fs::metadata(&stderr_path).map_err(|e| e.to_string())?.len();
        let valid = if let Some(reason) = stop_reason {
            Err(reason)
        } else if !status.success() {
            Err("child exited unsuccessfully".into())
        } else if fs::metadata(&stdout_path).map_err(|e| e.to_string())?.len() > OUTPUT_CAP {
            Err("child output cap exceeded".into())
        } else if let Some(reason) = parse_error {
            Err(reason)
        } else {
            validate_summary(child_spec, &records, parent_ns)
        };
        emit(
            writer,
            &json!({"kind":"account_process_receipt", "format_version":1,
                "mode":child_spec.label(), "prime":spec.curve.p, "depth":spec.depth, "seed":spec.seed,
                "exit_code":status.code(), "receipt_valid":valid.is_ok(), "error":valid.as_ref().err(),
                "spawn_through_exit_ns":parent_ns, "deadline_seconds":CHILD_LIMIT.as_secs(),
                "stderr":String::from_utf8_lossy(&stderr), "stderr_omitted_bytes":stderr_bytes.saturating_sub(stderr.len() as u64),
                "accounting":"local_development_only", "fresh_algorithm_state":true, "os_cache_flushed":false,
                "interval":"spawn through observed exit, including up to 5 ms polling latency; excludes parent parsing and forwarding",
                "unrun_arms":if valid.is_err() {&labels[index+1..]} else {&[]},
            }),
        )?;
        valid?;
    }
    Ok(())
}

pub(super) fn dispatch(
    curve: Fixture,
    depth: usize,
    solver: &Path,
    args: &[String],
    start: Instant,
) -> Option<Result<(), String>> {
    let first = args.first()?;
    let (is_suite, seed) = if let Some(seed) = first.strip_prefix("account-suite=") {
        (true, seed)
    } else if let Some(seed) = first.strip_prefix("account=") {
        (false, seed)
    } else {
        return None;
    };
    Some((|| {
        let seed = seed.parse::<u64>().map_err(|_| "invalid accounting seed")?;
        let (draws, labels) =
            if let Some(limit) = args.get(1).and_then(|s| s.strip_prefix("draws=")) {
                (
                    limit.parse::<usize>().map_err(|_| "invalid draw limit")?,
                    &args[2..],
                )
            } else {
                (30, &args[1..])
            };
        if !(1..=256).contains(&draws) {
            return Err("draw limit outside 1..=256".into());
        }
        if labels.is_empty() || labels.len() > if is_suite { 8 } else { 1 } {
            return Err(
                "account needs exactly one explicit arm; account-suite needs one to eight".into(),
            );
        }
        let mut unique = BTreeSet::new();
        for label in labels {
            if !unique.insert(label) {
                return Err("duplicate accounting arm".into());
            }
            let method = mode(label);
            if label != "direct" && method.is_none() {
                return Err("unknown accounting arm".into());
            }
            if depth > 4 && method == Some(Method::Single(Mode::Expanded)) {
                return Err("expanded depth exceeds four".into());
            }
            if depth < 3 && method == Some(Method::Refined) {
                return Err("refinement needs depth at least three".into());
            }
        }
        let spec = Run {
            curve,
            depth,
            solver,
            method: mode(&labels[0]),
            seed,
            draws,
        };
        let stdout = std::io::stdout();
        let mut writer = stdout.lock();
        if is_suite {
            suite(spec, labels, &mut writer)
        } else {
            run(spec, start, &mut writer).map(|_| ())
        }
    })())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn spec() -> Run<'static> {
        Run {
            curve: FIXTURES[0],
            depth: 4,
            solver: Path::new("/no-solver-needed"),
            method: None,
            seed: 781815,
            draws: 30,
        }
    }

    fn records(bytes: &[u8]) -> Vec<Value> {
        bytes
            .split(|b| *b == b'\n')
            .filter(|s| !s.is_empty())
            .map(|s| serde_json::from_slice(s).unwrap())
            .collect()
    }

    #[test]
    fn direct_recovers_through_production_consumer_and_accounts_every_draw() {
        let start = Instant::now();
        let mut bytes = vec![];
        let summary = run(spec(), start, &mut bytes).unwrap();
        let rows = records(&bytes);
        validate_summary(spec(), &rows, nanos(start.elapsed())).unwrap();
        assert_eq!(summary["scalar"], 69);
        assert_eq!(summary["calls"], 9);
        assert_eq!(summary["selected"], 9);
        assert_eq!(summary["consumer_work"]["submitted"], 9);
        assert_eq!(summary["consumer_work"]["scalar_replays"], 1);
        // Same chosen rows recover under the historical base-only reducer.
        let mut old = Recovery {
            width: summary["base_size"].as_u64().unwrap() as usize,
            order: 109,
            pivots: BTreeMap::new(),
        };
        let mut old_scalar = None;
        for record in &rows[..rows.len() - 1] {
            if let Ok(row) =
                serde_json::from_value::<Relation>(record["result"]["selected"].clone())
            {
                old_scalar = old.insert(
                    row,
                    record["a"].as_i64().unwrap(),
                    record["b"].as_i64().unwrap(),
                );
            }
        }
        assert_eq!(old_scalar, Some(69));
    }

    #[test]
    fn failing_solver_retains_failures_and_never_falls_back() {
        let scratch = Scratch::new().unwrap();
        let solver = scratch.0.join("fails.sh");
        fs::write(&solver, "#!/bin/sh\nexit 7\n").unwrap();
        fs::set_permissions(&solver, fs::Permissions::from_mode(0o700)).unwrap();
        let spec = Run {
            solver: &solver,
            method: Some(Method::Refined),
            draws: 3,
            ..spec()
        };
        let start = Instant::now();
        let mut bytes = vec![];
        let summary = run(spec, start, &mut bytes).unwrap();
        validate_summary(spec, &records(&bytes), nanos(start.elapsed())).unwrap();
        assert_eq!(summary["status"], "incomplete");
        assert_eq!(summary["calls"], 3);
        assert_eq!(summary["incomplete_queries"], 3);
        assert_eq!(summary["consumer_rank"], 0);
        assert_eq!(summary["wall"]["phase_ns"]["consume_relation"], 0);
        assert_eq!(summary["scalar"], Value::Null);
    }

    #[test]
    fn parent_rejects_missing_mismatched_and_unreconciled_receipts() {
        let start = Instant::now();
        let mut bytes = vec![];
        run(spec(), start, &mut bytes).unwrap();
        let rows = records(&bytes);
        let total = nanos(start.elapsed());
        assert!(validate_summary(spec(), &rows[..rows.len() - 1], total).is_err());
        assert!(validate_summary(Run { seed: 1, ..spec() }, &rows, total).is_err());
        assert!(validate_summary(spec(), &rows, 0).is_err());
        let mut changed = rows.clone();
        changed.last_mut().unwrap()["wall"]["unattributed_ns"] = json!(u64::MAX);
        assert!(validate_summary(spec(), &changed, total).is_err());
        changed = rows.clone();
        changed.remove(0);
        assert!(validate_summary(spec(), &changed, total).is_err());
    }

    #[test]
    fn failed_queries_also_retain_identity_and_duplicate_draws() {
        let spec = Run {
            method: Some(Method::Refined),
            draws: 256,
            ..spec()
        };
        let start = Instant::now();
        let mut bytes = vec![];
        let summary = run(spec, start, &mut bytes).unwrap();
        let rows = records(&bytes);
        validate_summary(spec, &rows, nanos(start.elapsed())).unwrap();
        assert_eq!(summary["draws"], 256);
        assert_eq!(summary["consumer_rank"], 0);
        assert!(rows.iter().any(|row| row["result"]["reason"] == "identity"));
        assert!(rows
            .iter()
            .any(|row| row["result"]["reason"] == "duplicate_target"));
        assert!(summary["skipped"].as_u64().unwrap() > 0);
    }

    #[test]
    fn timeout_kills_the_child_process_group_and_truncated_records_are_rejected() {
        let scratch = Scratch::new().unwrap();
        let output = scratch.0.join("child.txt");
        let mut child = Command::new("/bin/sh")
            .args(["-c", "sleep 30 & printf '%s\\n' \"$!\"; wait"])
            .process_group(0)
            .stdout(File::create(&output).unwrap())
            .spawn()
            .unwrap();
        let (status, reason) = wait_bounded(
            &mut child,
            Instant::now(),
            Duration::from_millis(100),
            &[&output],
        )
        .unwrap();
        assert!(!status.success());
        assert_eq!(reason.as_deref(), Some("child deadline exceeded"));
        let pid = fs::read_to_string(&output).unwrap();
        let stat_path = format!("/proc/{}/stat", pid.trim());
        // A killed grandchild may remain a zombie until the container init reaps it.
        for _ in 0..100 {
            let running = fs::read_to_string(&stat_path).ok().is_some_and(|s| {
                s.rsplit_once(')').unwrap().1.trim_start().chars().next() != Some('Z')
            });
            if !running {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
        if let Ok(stat) = fs::read_to_string(&stat_path) {
            assert_eq!(
                stat.rsplit_once(')').unwrap().1.trim_start().chars().next(),
                Some('Z')
            );
        }
        fs::write(&output, b"{}\n").unwrap();
        terminated_output(&output).unwrap();
        fs::write(&output, b"{}").unwrap();
        assert_eq!(
            terminated_output(&output).unwrap_err(),
            "unterminated child record"
        );
    }
}
