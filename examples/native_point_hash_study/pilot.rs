//! Compact input recipes, balanced schedules and fail-stop child orchestration.
//! All clocks remain unadmitted. Analysis deliberately reports counts only.

use super::*;
use crypto_lib::cryptanalysis::ecbench::stats::linearised_ratio;
use crypto_lib::cryptanalysis::ic_boundary::roster_prime_instance;
use hashed_ecdl_tensor::arithmetic::{hash_prime, random_below};
use std::ffi::OsString;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const ARMS: [Arm; 6] = [
    Arm::Current,
    Arm::CurrentAa,
    Arm::Prepared,
    Arm::Ordered,
    Arm::Hashed,
    Arm::Fingerprint,
];
const SCHEMA: &str = "native-point-hash.pilot-plan/v1";
const MAX_LEDGER_BYTES: u64 = 16_777_216;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Profile {
    Pilot12And16,
    DevelopmentF17,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Context {
    id: String,
    bits: u32,
    threshold: u64,
    base_seed: u64,
    query_seed: u64,
    curve: CurveInput,
    bases: [BaseInput; 2],
    queries_sha256: String,
    controls_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    schema: String,
    profile: Profile,
    warmups: usize,
    rounds: usize,
    query_count: usize,
    order_seed: u64,
    seconds: u64,
    contexts: Vec<Context>,
    planned_attempts: usize,
    schedule_sha256: String,
    expanded_inputs_sha256: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Kind {
    Control,
    ForcedCollision,
    Counted,
    Warmup,
    Measured,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Row {
    ordinal: usize,
    context: usize,
    arm: Arm,
    kind: Kind,
    round: usize,
}

fn digest(value: &impl Serialize) -> Result<String> {
    Ok(format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(value).map_err(|e| e.to_string())?)
    ))
}
fn draw(seed: u64, label: &str, index: u64, bound: u64) -> u64 {
    assert!(bound > 0);
    let zone = u64::MAX - u64::MAX % bound;
    for nonce in 0u64.. {
        let bytes = Sha256::digest(format!(
            "native-point-hash-pilot/v1/{seed}/{label}/{index}/{nonce}"
        ));
        let word = u64::from_le_bytes(bytes[..8].try_into().expect("eight bytes"));
        if word < zone {
            return word % bound;
        }
    }
    unreachable!()
}
fn shuffled<T>(mut values: Vec<T>, seed: u64, label: &str) -> Vec<T> {
    for i in (1..values.len()).rev() {
        let j = draw(seed, label, i as u64, (i + 1) as u64) as usize;
        values.swap(i, j);
    }
    values
}
fn queries(context: &Context, count: usize) -> Vec<WirePoint> {
    let c = &context.curve;
    let curve = PrimeCurve {
        p: c.p,
        a: c.a,
        b: c.b,
    };
    (0..count)
        .map(|i| {
            let scalar = draw(context.query_seed, "query", i as u64, c.order);
            curve
                .mul(&mut GroupOps::default(), c.generator.word(), scalar)
                .into()
        })
        .collect()
}
fn template(context: &Context, query_points: Vec<WirePoint>, arm: Arm, mode: Mode) -> Input {
    Input {
        schema: "native-point-hash.batch-input/v1".into(),
        id: context.id.clone(),
        arm,
        mode,
        force_hash_collisions: false,
        curve: context.curve.clone(),
        bases: context.bases,
        queries: query_points,
        limits: WorkLimits {
            steps: 1_000_000_000,
            vector: 4096,
            bits: 128,
        },
    }
}
fn control_queries(context: &Context) -> Result<Vec<WirePoint>> {
    let input = template(
        context,
        vec![context.curve.generator],
        Arm::Current,
        Mode::Counts,
    );
    let (curve, bases) = word_bases(&input)?;
    let key = |p: PrimePoint| (p.infinity, p.x, p.y);
    let mut sums = BTreeSet::new();
    let mut result = Vec::new();
    for p in &bases[0] {
        for q in &bases[1] {
            let sum = curve.add(&mut GroupOps::default(), *p, *q);
            if result.is_empty() {
                result.push(sum.into());
            }
            sums.insert(key(sum));
        }
    }
    let mut point = PrimePoint::INFINITY;
    for _ in 0..context.curve.order {
        if !sums.contains(&key(point)) {
            result.push(point.into());
            break;
        }
        point = curve.add(
            &mut GroupOps::default(),
            point,
            context.curve.generator.word(),
        );
    }
    // Empty bases have only the negative control; a full sumset only positive.
    // Keep those contexts, without resampling. The complete checker is retained.
    if result.is_empty() {
        return Err("control construction returned no query".into());
    }
    Ok(result)
}
impl Plan {
    fn rows(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        let mut push = |context, arm, kind, round| {
            rows.push(Row {
                ordinal: rows.len(),
                context,
                arm,
                kind,
                round,
            });
        };
        for kind in [Kind::Control, Kind::ForcedCollision, Kind::Counted] {
            for context in 0..self.contexts.len() {
                for arm in ARMS {
                    if kind == Kind::ForcedCollision
                        && ![Arm::Hashed, Arm::Fingerprint].contains(&arm)
                    {
                        continue;
                    }
                    push(context, arm, kind, 0);
                }
            }
        }
        for (kind, rounds) in [(Kind::Warmup, self.warmups), (Kind::Measured, self.rounds)] {
            for pair in 0..rounds / 2 {
                let label = format!("{kind:?}/{pair}");
                let contexts =
                    shuffled((0..self.contexts.len()).collect(), self.order_seed, &label);
                let mut order = Vec::new();
                for context in contexts {
                    let arms = shuffled(
                        ARMS.to_vec(),
                        self.order_seed,
                        &format!("{label}/{context}"),
                    );
                    order.extend(arms.into_iter().map(|arm| (context, arm)));
                }
                for (context, arm) in order.iter().copied() {
                    push(context, arm, kind, 2 * pair);
                }
                for (context, arm) in order.iter().rev().copied() {
                    push(context, arm, kind, 2 * pair + 1);
                }
            }
        }
        rows
    }
    fn expand(&self) -> Result<Vec<Input>> {
        let mut ordinary = Vec::new();
        let mut controls = Vec::new();
        for context in &self.contexts {
            let q = queries(context, self.query_count);
            let c = control_queries(context)?;
            if digest(&q)? != context.queries_sha256 || digest(&c)? != context.controls_sha256 {
                return Err("query/control recipe hash mismatch".into());
            }
            ordinary.push(q);
            controls.push(c);
        }
        self.rows()
            .iter()
            .map(|row| {
                let mode = if self.profile == Profile::Pilot12And16
                    && [Kind::Warmup, Kind::Measured].contains(&row.kind)
                {
                    Mode::Clocks
                } else {
                    Mode::Counts
                };
                let q = if row.kind == Kind::Control {
                    &controls[row.context]
                } else {
                    &ordinary[row.context]
                };
                let mut input = template(&self.contexts[row.context], q.clone(), row.arm, mode);
                input.id = format!("{}-{:06}", input.id, row.ordinal);
                input.force_hash_collisions = row.kind == Kind::ForcedCollision;
                input.validate()?;
                Ok(input)
            })
            .collect()
    }
    fn seal(&mut self) -> Result<()> {
        self.planned_attempts = self.rows().len();
        self.schedule_sha256 = digest(&self.rows())?;
        self.expanded_inputs_sha256 = digest(
            &self
                .expand()?
                .iter()
                .map(Input::hash)
                .collect::<Result<Vec<_>>>()?,
        )?;
        Ok(())
    }
    fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA
            || self.seconds == 0
            || self.seconds > 1800
            || self.rounds < 2
            || self.rounds > 8
            || !num_integer::Integer::is_multiple_of(&self.rounds, &2)
            || self.warmups != 2
            || !(1..=256).contains(&self.query_count)
            || self.contexts.is_empty()
            || self.contexts.len() > 32
        {
            return Err("invalid bounded pilot profile".into());
        }
        let mut keys = BTreeSet::new();
        let mut ids = BTreeSet::new();
        for c in &self.contexts {
            if !keys.insert((c.bits, c.threshold, c.base_seed)) || !ids.insert(&c.id) {
                return Err("duplicate context identity".into());
            }
        }
        if self.profile == Profile::Pilot12And16 {
            let expected: BTreeSet<_> = [12, 16]
                .into_iter()
                .flat_map(|bits| {
                    [32, 128]
                        .into_iter()
                        .flat_map(move |threshold| (0..8).map(move |seed| (bits, threshold, seed)))
                })
                .collect();
            if keys != expected || self.rounds != 8 || self.query_count != 256 {
                return Err("the frozen pilot grid was changed".into());
            }
            for c in &self.contexts {
                let expected = make_context(c.bits, c.threshold, c.base_seed)?;
                if digest(c)? != digest(&expected)? {
                    return Err("pilot context differs from fixed recipe".into());
                }
            }
        } else if self
            .contexts
            .iter()
            .any(|c| c.bits != 5 || c.curve.p != 17 || c.curve.order != 19)
        {
            return Err("development profile is restricted to the public F17 control".into());
        }
        if self.rows().len() != self.planned_attempts
            || digest(&self.rows())? != self.schedule_sha256
        {
            return Err("schedule identity mismatch".into());
        }
        let hashes = self
            .expand()?
            .iter()
            .map(Input::hash)
            .collect::<Result<Vec<_>>>()?;
        if digest(&hashes)? != self.expanded_inputs_sha256 {
            return Err("expanded input identity mismatch".into());
        }
        Ok(())
    }
}
fn make_context(bits: u32, threshold: u64, base_seed: u64) -> Result<Context> {
    let instance = roster_prime_instance(bits).ok_or("missing registered prime roster")?;
    if instance.cofactor != 1 {
        return Err("pilot requires cofactor one".into());
    }
    let seed = draw(base_seed, &format!("base/{bits}/{threshold}"), 0, u64::MAX);
    let mut rng = Seeded(seed);
    let mut meter = Meter::new(Limits {
        steps: 10_000_000,
        vector: 4096,
        bits: 128,
    });
    let prime =
        hash_prime(&instance.curve.p.into(), &mut rng, &mut meter).map_err(|e| e.to_string())?;
    let mut bases = Vec::new();
    for _ in 0..2 {
        let alpha = random_below(&(&prime - BigUint::from(1u8)), &mut rng, &mut meter)
            .map_err(|e| e.to_string())?
            + BigUint::from(1u8);
        let beta = random_below(&prime, &mut rng, &mut meter).map_err(|e| e.to_string())?;
        bases.push(BaseInput {
            prime: prime.to_u64().ok_or("hash prime exceeds word")?,
            threshold,
            alpha: alpha.to_u64().ok_or("alpha exceeds word")?,
            beta: beta.to_u64().ok_or("beta exceeds word")?,
        });
    }
    let mut context = Context {
        id: format!("p{bits}-t{threshold}-s{base_seed}"),
        bits,
        threshold,
        base_seed,
        query_seed: draw(
            base_seed,
            &format!("queries/{bits}/{threshold}"),
            0,
            u64::MAX,
        ),
        curve: CurveInput {
            p: instance.curve.p,
            a: instance.curve.a,
            b: instance.curve.b,
            order: instance.r,
            generator: instance.generator_point().into(),
        },
        bases: [bases[0], bases[1]],
        queries_sha256: String::new(),
        controls_sha256: String::new(),
    };
    context.queries_sha256 = digest(&queries(&context, 256))?;
    context.controls_sha256 = digest(&control_queries(&context)?)?;
    Ok(context)
}
fn make_plan() -> Result<Plan> {
    let mut contexts = Vec::new();
    for bits in [12, 16] {
        for threshold in [32, 128] {
            for seed in 0..8 {
                contexts.push(make_context(bits, threshold, seed)?);
            }
        }
    }
    let mut plan = Plan {
        schema: SCHEMA.into(),
        profile: Profile::Pilot12And16,
        warmups: 2,
        rounds: 8,
        query_count: 256,
        order_seed: 0x706f696e74686173,
        seconds: 1800,
        contexts,
        planned_attempts: 0,
        schedule_sha256: String::new(),
        expanded_inputs_sha256: String::new(),
    };
    plan.seal()?;
    Ok(plan)
}

fn make_development_plan() -> Result<Plan> {
    let mut contexts = Vec::new();
    for seed in 0..2 {
        let mut context = Context {
            id: format!("development-{seed}"),
            bits: 5,
            threshold: 12,
            base_seed: seed,
            query_seed: seed,
            curve: CurveInput {
                p: 17,
                a: 2,
                b: 2,
                order: 19,
                generator: PrimePoint::affine(5, 1).into(),
            },
            bases: [
                BaseInput {
                    prime: 37,
                    threshold: 24,
                    alpha: 1,
                    beta: 2,
                },
                BaseInput {
                    prime: 37,
                    threshold: 12,
                    alpha: 3,
                    beta: 4,
                },
            ],
            queries_sha256: String::new(),
            controls_sha256: String::new(),
        };
        context.queries_sha256 = digest(&queries(&context, 4))?;
        context.controls_sha256 = digest(&control_queries(&context)?)?;
        contexts.push(context);
    }
    let mut plan = Plan {
        schema: SCHEMA.into(),
        profile: Profile::DevelopmentF17,
        warmups: 2,
        rounds: 2,
        query_count: 4,
        order_seed: 17,
        seconds: 30,
        contexts,
        planned_attempts: 0,
        schedule_sha256: String::new(),
        expanded_inputs_sha256: String::new(),
    };
    plan.seal()?;
    Ok(plan)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ProcessReceipt {
    exit_code: Option<i32>,
    timed_out: bool,
    elapsed_ns: Option<u64>,
    error: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "event", rename_all = "snake_case", deny_unknown_fields)]
enum Event {
    Started {
        ordinal: usize,
        input_sha256: String,
    },
    Finished {
        ordinal: usize,
        input_sha256: String,
        status: String,
        process: ProcessReceipt,
        record_sha256: Option<String>,
        stdout_sha256: Option<String>,
        stderr_sha256: Option<String>,
        detail: Option<String>,
    },
}
fn sha_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut hash = Sha256::new();
    let mut bytes = [0u8; 65_536];
    loop {
        let n = file.read(&mut bytes).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        hash.update(&bytes[..n]);
    }
    Ok(format!("{:x}", hash.finalize()))
}
fn append_event(file: &mut fs::File, event: &Event) -> Result<()> {
    serde_json::to_writer(&mut *file, event).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    file.sync_data().map_err(|e| e.to_string())
}
fn wait_child(mut process: Child, deadline: Instant) -> Result<(Option<i32>, bool)> {
    loop {
        match process.try_wait() {
            Ok(Some(status)) => return Ok((status.code(), false)),
            Ok(None) => {}
            Err(error) => {
                let _ = process.kill();
                let _ = process.wait();
                return Err(error.to_string());
            }
        }
        if Instant::now() >= deadline {
            let _ = process.kill();
            let status = process.wait().map_err(|e| e.to_string())?;
            return Ok((status.code(), true));
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
fn child(
    program: &Path,
    input: &Path,
    output: &Path,
    dir: &Path,
    deadline: Instant,
) -> ProcessReceipt {
    let start = Instant::now();
    let result = (|| -> Result<(Option<i32>, bool)> {
        let stdout = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(dir.join("stdout.log"))
            .map_err(|e| e.to_string())?;
        let stderr = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(dir.join("stderr.log"))
            .map_err(|e| e.to_string())?;
        let process = Command::new(program)
            .arg("run")
            .arg(input)
            .arg(output)
            .stdin(Stdio::null())
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .map_err(|e| e.to_string())?;
        wait_child(process, deadline)
    })();
    let elapsed_ns = u64::try_from(start.elapsed().as_nanos()).ok();
    match result {
        Ok((exit_code, timed_out)) => ProcessReceipt {
            exit_code,
            timed_out,
            elapsed_ns,
            error: None,
        },
        Err(error) => ProcessReceipt {
            exit_code: None,
            timed_out: false,
            elapsed_ns,
            error: Some(error),
        },
    }
}
fn execute_with(
    plan: &Plan,
    out: &Path,
    deadline_unix: u64,
    binary_sha256: &str,
    mut run: impl FnMut(&Input, &Path, &Path, &Path, Instant) -> ProcessReceipt,
) -> Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    let remaining = deadline_unix
        .checked_sub(now)
        .filter(|n| *n > 0 && *n <= plan.seconds)
        .ok_or("original absolute deadline expired or exceeds the pilot ceiling")?;
    let deadline = Instant::now() + Duration::from_secs(remaining);
    plan.validate()?;
    let inputs = plan.expand()?;
    if Instant::now() >= deadline {
        return Err("pilot original deadline exhausted during input validation".into());
    }
    fs::create_dir(out)
        .map_err(|e| format!("refusing existing/uncreatable output directory: {e}"))?;
    write_new(
        &out.join("run.json"),
        &serde_json::json!({
            "schema":"native-point-hash.pilot-run/v1", "plan_sha256":digest(plan)?,
            "binary_sha256":binary_sha256, "profile":plan.profile, "deadline_unix":deadline_unix,
            "automatic_retries":0, "automatic_restores":0,
            "clock_admission":"unadmitted; external native L2, host/resource and canonical protocol verification remain required"
        }),
    )?;
    let mut ledger = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("events.jsonl"))
        .map_err(|e| e.to_string())?;
    for (row, input) in plan.rows().iter().zip(&inputs) {
        if Instant::now() >= deadline {
            return Err("pilot original deadline exhausted before next child".into());
        }
        let dir = out.join(format!("{:06}", row.ordinal));
        fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let input_path = dir.join("input.json");
        let output_path = dir.join("record.json");
        write_new(&input_path, input)?;
        let input_sha256 = input.hash()?;
        append_event(
            &mut ledger,
            &Event::Started {
                ordinal: row.ordinal,
                input_sha256: input_sha256.clone(),
            },
        )?;
        let process = run(input, &input_path, &output_path, &dir, deadline);
        let mut status = "passed";
        let mut detail = process.error.clone();
        if process.timed_out {
            status = "timed_out";
        } else if process.error.is_some() {
            status = "infrastructure_error";
        } else if process.exit_code != Some(0) {
            status = "child_failure";
        } else {
            match read_json::<Record>(&output_path).and_then(|record| {
                check_record(input, &record)?;
                if record
                    .outcomes
                    .iter()
                    .any(|o| matches!(o, Outcome::Unknown { .. } | Outcome::NotAttempted))
                {
                    return Err("resource_limit".into());
                }
                Ok(())
            }) {
                Ok(()) => {}
                Err(error) => {
                    status = if error == "resource_limit" {
                        "resource_limit"
                    } else {
                        "replay_failure"
                    };
                    detail = Some(error);
                }
            }
        }
        append_event(
            &mut ledger,
            &Event::Finished {
                ordinal: row.ordinal,
                input_sha256,
                status: status.into(),
                process,
                record_sha256: sha_file(&output_path).ok(),
                stdout_sha256: sha_file(&dir.join("stdout.log")).ok(),
                stderr_sha256: sha_file(&dir.join("stderr.log")).ok(),
                detail,
            },
        )?;
        if status != "passed" {
            return Err(format!(
                "pilot stopped at attempt {}: {status}; no retry",
                row.ordinal
            ));
        }
    }
    Ok(())
}

fn read_events(path: &Path) -> Result<Vec<Event>> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    file.take(MAX_LEDGER_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_LEDGER_BYTES {
        return Err("ledger exceeds bounded read".into());
    }
    bytes
        .lines()
        .map(|line| {
            serde_json::from_str(&line.map_err(|e| e.to_string())?).map_err(|e| e.to_string())
        })
        .collect()
}
fn analyze(plan: &Plan, dir: &Path) -> Result<serde_json::Value> {
    plan.validate()?;
    let header: serde_json::Value = read_json(&dir.join("run.json"))?;
    if header["schema"] != "native-point-hash.pilot-run/v1"
        || header["plan_sha256"] != digest(plan)?
    {
        return Err("run/plan binding mismatch".into());
    }
    let rows = plan.rows();
    let inputs = plan.expand()?;
    let events = read_events(&dir.join("events.jsonl"))?;
    let mut started = 0usize;
    let mut finished = 0usize;
    let mut passed = 0usize;
    let mut terminal = false;
    let mut diagnostic = BTreeMap::new();
    let (mut control_contexts, mut positives, mut negatives, mut empty_bases, mut full_sumsets) =
        (0, 0, 0, 0, 0);
    for event in events {
        match event {
            Event::Started {
                ordinal,
                input_sha256,
            } => {
                if terminal
                    || started != finished
                    || ordinal != started
                    || ordinal >= rows.len()
                    || input_sha256 != inputs[ordinal].hash()?
                {
                    return Err("duplicate, out-of-order or wrongly bound start".into());
                }
                let actual_input: Input = read_json(&dir.join(format!("{ordinal:06}/input.json")))?;
                if actual_input.hash()? != input_sha256 {
                    return Err("input artifact hash mismatch".into());
                }
                started += 1;
            }
            Event::Finished {
                ordinal,
                input_sha256,
                status,
                process,
                record_sha256,
                stdout_sha256,
                stderr_sha256,
                ..
            } => {
                if terminal
                    || started != finished + 1
                    || ordinal != finished
                    || input_sha256 != inputs[ordinal].hash()?
                {
                    return Err("duplicate, out-of-order or wrongly bound finish".into());
                }
                finished += 1;
                let row_dir = dir.join(format!("{ordinal:06}"));
                if status == "passed"
                    && (record_sha256.is_none()
                        || stdout_sha256.is_none()
                        || stderr_sha256.is_none())
                {
                    return Err("successful child omitted required artifact hashes".into());
                }
                for (path, hash) in [
                    (row_dir.join("record.json"), record_sha256),
                    (row_dir.join("stdout.log"), stdout_sha256),
                    (row_dir.join("stderr.log"), stderr_sha256),
                ] {
                    if let Some(hash) = hash {
                        if sha_file(&path)? != hash {
                            return Err("artifact hash mismatch".into());
                        }
                    }
                }
                match status.as_str() {
                    "passed" => {
                        if process.exit_code != Some(0)
                            || process.timed_out
                            || process.error.is_some()
                        {
                            return Err("false successful child receipt".into());
                        }
                        let record: Record = read_json(&row_dir.join("record.json"))?;
                        check_record(&inputs[ordinal], &record)?;
                        if record
                            .outcomes
                            .iter()
                            .any(|o| matches!(o, Outcome::Unknown { .. } | Outcome::NotAttempted))
                        {
                            return Err("incomplete result labelled passed".into());
                        }
                        if rows[ordinal].kind == Kind::Counted {
                            diagnostic.insert((rows[ordinal].context, rows[ordinal].arm), record);
                        } else if rows[ordinal].kind == Kind::Control
                            && rows[ordinal].arm == Arm::Current
                        {
                            control_contexts += 1;
                            positives += record
                                .outcomes
                                .iter()
                                .filter(|o| matches!(o, Outcome::Witness { .. }))
                                .count();
                            negatives += record
                                .outcomes
                                .iter()
                                .filter(|o| matches!(o, Outcome::Absent))
                                .count();
                            empty_bases += usize::from(record.base_sizes.contains(&0));
                            full_sumsets += usize::from(
                                record.outcomes.len() == 1
                                    && matches!(record.outcomes[0], Outcome::Witness { .. }),
                            );
                        }
                        passed += 1;
                    }
                    "timed_out"
                    | "infrastructure_error"
                    | "child_failure"
                    | "replay_failure"
                    | "resource_limit" => {
                        terminal = true;
                    }
                    _ => return Err("unknown finish status".into()),
                }
            }
        }
    }
    let complete = passed == rows.len();
    let mut contrasts = Vec::new();
    if complete {
        for bits in [5, 12, 16] {
            for threshold in [12, 24, 32, 128] {
                for arm in ARMS.into_iter().filter(|a| *a != Arm::Current) {
                    for prefix_count in [1, 4, 16, 64, 256] {
                        for metric in ["meter_steps", "curve_additions", "root_requests"] {
                            let mut clusters = Vec::new();
                            for (i, context) in plan
                                .contexts
                                .iter()
                                .enumerate()
                                .filter(|(_, c)| c.bits == bits && c.threshold == threshold)
                            {
                                let select = |record: &Record| -> Option<f64> {
                                    let p = record
                                        .prefixes
                                        .iter()
                                        .find(|p| p.queries == prefix_count)?;
                                    Some(match metric {
                                        "meter_steps" => p.meter_steps,
                                        "curve_additions" => p.curve_additions,
                                        _ => p.root_requests,
                                    } as f64)
                                };
                                let _ = context;
                                if let (Some(a), Some(b)) = (
                                    diagnostic.get(&(i, Arm::Current)).and_then(select),
                                    diagnostic.get(&(i, arm)).and_then(select),
                                ) {
                                    clusters.push((a, b));
                                }
                            }
                            if clusters.is_empty() {
                                continue;
                            }
                            let estimate = linearised_ratio(&clusters);
                            contrasts.push(serde_json::json!({
                            "bits":bits,"threshold":threshold,"arm":arm,"prefix_queries":prefix_count,
                            "metric":metric,"independent_base_seed_clusters":clusters.len(),
                            "candidate_over_current":estimate.as_ref().map(|x| x.estimate),
                            "descriptive_pointwise_95_interval":estimate.and_then(|x| x.ci95_log()),
                            "inferential_promotion":"none; unpriced diagnostics and no multiplicity/power qualification"
                        }));
                        }
                    }
                }
            }
        }
    }
    Ok(serde_json::json!({
        "schema":"native-point-hash.pilot-analysis/v1", "plan_sha256":digest(plan)?, "profile":plan.profile,
        "flow":{"planned":rows.len(),"attempted":started,"finished":finished,"passed":passed,
            "failed":finished-passed,"inflight":started-finished,"not_attempted":rows.len()-started},
        "complete":complete,"count_contrasts":contrasts,
        "control_coverage":{"contexts_checked":control_contexts,"positive_queries":positives,"negative_queries":negatives,
            "positive_unavailable_empty_base_contexts":empty_bases,"negative_unavailable_full_sumset_contexts":full_sumsets},
        "timing_comparisons":"refused: native L2/host/resource admission integration is pending",
        "scope":"paired diagnostic count ratios only; fresh-target ECDLP and runtime gains not established",
        "standing":"producer analysis; not a signed PCC or independent audit verdict"
    }))
}

pub(super) fn cli(args: &[OsString]) -> Result<()> {
    let path = |i| {
        args.get(i)
            .map(PathBuf::from)
            .ok_or("missing path argument".to_string())
    };
    match args.get(1).and_then(|s| s.to_str()) {
        Some("make-plan") if args.len() == 3 => write_new(&path(2)?, &make_plan()?),
        Some("make-development-plan") if args.len() == 3 => write_new(&path(2)?, &make_development_plan()?),
        Some("check-plan") if args.len() == 3 => {
            let plan: Plan = read_json(&path(2)?)?; plan.validate()?;
            println!("{}", serde_json::json!({"plan_sha256":digest(&plan)?,"planned_attempts":plan.planned_attempts,"measurements_taken":false}));
            Ok(())
        }
        Some("execute-plan") if args.len() == 5 => {
            let plan: Plan = read_json(&path(2)?)?;
            let deadline = args[4].to_str().ok_or("non-UTF8 deadline")?.parse().map_err(|_| "invalid deadline")?;
            let program = std::env::current_exe().map_err(|e| e.to_string())?;
            execute_with(&plan, &path(3)?, deadline, &sha_file(&program)?, |_, input, output, dir, stop| child(&program, input, output, dir, stop))
        }
        Some("analyze-plan") if args.len() == 5 => {
            let plan: Plan = read_json(&path(2)?)?;
            write_new(&path(4)?, &analyze(&plan, &path(3)?)?)
        }
        _ => Err("usage: make-plan OUT | make-development-plan OUT | check-plan PLAN | execute-plan PLAN NEW_DIR ABSOLUTE_UNIX_DEADLINE | analyze-plan PLAN RUN_DIR OUT".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let dir =
                std::env::temp_dir().join(format!("point-pilot-{}-{nonce}", std::process::id()));
            fs::create_dir(&dir).unwrap();
            Self(dir)
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn development() -> Plan {
        let mut contexts = Vec::new();
        for seed in 0..2 {
            let mut c = Context {
                id: format!("development-{seed}"),
                bits: 5,
                threshold: 12,
                base_seed: seed,
                query_seed: seed,
                curve: CurveInput {
                    p: 17,
                    a: 2,
                    b: 2,
                    order: 19,
                    generator: PrimePoint::affine(5, 1).into(),
                },
                bases: [
                    BaseInput {
                        prime: 37,
                        threshold: 24,
                        alpha: 1,
                        beta: 2,
                    },
                    BaseInput {
                        prime: 37,
                        threshold: 12,
                        alpha: 3,
                        beta: 4,
                    },
                ],
                queries_sha256: String::new(),
                controls_sha256: String::new(),
            };
            c.queries_sha256 = digest(&queries(&c, 4)).unwrap();
            c.controls_sha256 = digest(&control_queries(&c).unwrap()).unwrap();
            contexts.push(c);
        }
        let mut p = Plan {
            schema: SCHEMA.into(),
            profile: Profile::DevelopmentF17,
            warmups: 2,
            rounds: 2,
            query_count: 4,
            order_seed: 17,
            seconds: 30,
            contexts,
            planned_attempts: 0,
            schedule_sha256: String::new(),
            expanded_inputs_sha256: String::new(),
        };
        p.seal().unwrap();
        p
    }
    fn mock_success(input: &Input, _: &Path, out: &Path, dir: &Path, _: Instant) -> ProcessReceipt {
        assert_eq!(input.mode, Mode::Counts);
        write_new(out, &run_batch(input).unwrap()).unwrap();
        fs::write(dir.join("stdout.log"), b"").unwrap();
        fs::write(dir.join("stderr.log"), b"").unwrap();
        ProcessReceipt {
            exit_code: Some(0),
            timed_out: false,
            elapsed_ns: None,
            error: None,
        }
    }
    fn future() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 30
    }
    #[test]
    fn recipe_is_deterministic_and_reversal_balanced() {
        let p = development();
        p.validate().unwrap();
        assert_eq!(digest(&p).unwrap(), digest(&development()).unwrap());
        assert_eq!(
            digest(&p).unwrap(),
            digest(&make_development_plan().unwrap()).unwrap()
        );
        for kind in [Kind::Warmup, Kind::Measured] {
            let a: Vec<_> = p
                .rows()
                .into_iter()
                .filter(|r| r.kind == kind && r.round == 0)
                .map(|r| (r.context, r.arm))
                .collect();
            let b: Vec<_> = p
                .rows()
                .into_iter()
                .filter(|r| r.kind == kind && r.round == 1)
                .map(|r| (r.context, r.arm))
                .collect();
            assert_eq!(a, b.into_iter().rev().collect::<Vec<_>>());
        }
        assert_eq!(
            p.rows()
                .iter()
                .filter(|r| r.kind == Kind::ForcedCollision)
                .count(),
            4
        );
        let mut changed = p.clone();
        changed.contexts[0].query_seed += 1;
        assert!(changed.validate().is_err());
    }
    #[test]
    fn complete_mock_flow_is_bound_and_analyzable() {
        let t = Temp::new();
        let p = development();
        let out = t.0.join("run");
        execute_with(&p, &out, future(), "development-mock", mock_success).unwrap();
        let a = analyze(&p, &out).unwrap();
        assert_eq!(a["flow"]["passed"], p.planned_attempts);
        assert_eq!(a["complete"], true);
        assert_eq!(a["control_coverage"]["contexts_checked"], 2);
        let coverage = &a["control_coverage"];
        assert_eq!(
            coverage["positive_queries"].as_u64().unwrap()
                + coverage["positive_unavailable_empty_base_contexts"]
                    .as_u64()
                    .unwrap(),
            2
        );
        assert_eq!(
            coverage["negative_queries"].as_u64().unwrap()
                + coverage["negative_unavailable_full_sumset_contexts"]
                    .as_u64()
                    .unwrap(),
            2
        );
        assert!(!a["count_contrasts"].as_array().unwrap().is_empty());
        assert!(execute_with(&p, &out, future(), "development-mock", mock_success).is_err());
        fs::write(out.join("000000/record.json"), b"{}").unwrap();
        assert!(analyze(&p, &out).is_err());
    }
    #[test]
    fn failure_stops_once_and_keeps_unattempted_flow() {
        let t = Temp::new();
        let p = development();
        let out = t.0.join("run");
        let mut calls = 0;
        let r = execute_with(&p, &out, future(), "development-mock", |_, _, _, _, _| {
            calls += 1;
            ProcessReceipt {
                exit_code: Some(7),
                timed_out: false,
                elapsed_ns: None,
                error: None,
            }
        });
        assert!(r.is_err());
        assert_eq!(calls, 1);
        let a = analyze(&p, &out).unwrap();
        assert_eq!(a["flow"]["failed"], 1);
        assert_eq!(a["flow"]["not_attempted"], p.planned_attempts - 1);
        assert!(a["count_contrasts"].as_array().unwrap().is_empty());
    }
    #[test]
    fn expired_deadline_creates_no_run() {
        let t = Temp::new();
        let p = development();
        let out = t.0.join("run");
        assert!(execute_with(&p, &out, 0, "development-mock", mock_success).is_err());
        assert!(!out.exists());
    }
    #[test]
    fn diagnostic_statistics_are_paired_by_base_seed() {
        let a = linearised_ratio(&[(10., 20.), (20., 40.), (30., 60.)]).unwrap();
        assert_eq!(a.clusters, 3);
        assert_eq!(a.estimate, 2.);
        assert_eq!(a.ci95_log(), Some((2., 2.)));
    }
    #[test]
    fn inflight_and_duplicate_events_are_not_silent_successes() {
        let t = Temp::new();
        let p = development();
        let out = t.0.join("run");
        execute_with(&p, &out, future(), "development-mock", mock_success).unwrap();
        let path = out.join("events.jsonl");
        let events = read_events(&path).unwrap();
        let mut file = fs::File::create(&path).unwrap();
        append_event(&mut file, &events[0]).unwrap();
        drop(file);
        let flow = analyze(&p, &out).unwrap();
        assert_eq!(flow["flow"]["inflight"], 1);
        assert_eq!(flow["complete"], false);
        let mut file = OpenOptions::new().append(true).open(&path).unwrap();
        append_event(&mut file, &events[0]).unwrap();
        drop(file);
        assert!(analyze(&p, &out).is_err());
    }
    #[cfg(unix)]
    #[test]
    fn timed_out_child_is_killed_and_reaped() {
        let process = Command::new("/bin/sleep").arg("1").spawn().unwrap();
        let pid = process.id();
        let (_, timed_out) =
            wait_child(process, Instant::now() + Duration::from_millis(10)).unwrap();
        assert!(timed_out);
        assert!(!Path::new(&format!("/proc/{pid}")).exists());
    }
}
