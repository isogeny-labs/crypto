//! Matched exposed-residual component worker. Validation answers never enter
//! run_arm or its solver closure. Retain every attempted interval.
use super::{jv_nagao_template as template, jv_nagao_template_solver as solver, nagao_unit};
use crypto_lib::cryptanalysis::{f4_fp as f4, jv_bitops, jv_cover as cover};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const RAW_SHA: &str = "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d";
const BASES_SHA: &str = "e63a6126b3309bc7e387347f4f8629d78f12897a7c55c928b7152e75c21e76f1";
fn ensure(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn digest(value: &impl Serialize) -> Result<String> {
    Ok(hash(&serde_json::to_vec(value)?))
}
fn write_new(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut f, value)?;
    writeln!(f)?;
    f.sync_all()?;
    Ok(())
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Point {
    x: [[u64; 2]; 3],
    y: [[u64; 2]; 3],
    inf: bool,
}
impl Point {
    fn native(&self) -> cover::PtE6 {
        cover::PtE6 {
            x: cover::E6(self.x.map(cover::E2)),
            y: cover::E6(self.y.map(cover::E2)),
            inf: self.inf,
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Curve {
    p: u64,
    l: u64,
    alpha: [[u64; 2]; 3],
    g: Point,
    q: Point,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Residual {
    index: usize,
    u: Vec<[u64; 2]>,
    v: Vec<[u64; 2]>,
    input_sha256: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Fixture {
    schema: String,
    raw_sha256: String,
    bases_sha256: String,
    curve_sha256: String,
    curve: Curve,
    residuals: Vec<Residual>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Expected {
    public_sha256: String,
    roots: Vec<Vec<Vec<u64>>>,
}

fn prepare(raw_path: &str, bases_path: &str, curve_path: &str, out: &str) -> Result<()> {
    ensure(!Path::new(out).exists(), "exclusive fixture directory")?;
    let raw_bytes = fs::read(raw_path)?;
    let bases_bytes = fs::read(bases_path)?;
    let curve_bytes = fs::read(curve_path)?;
    ensure(
        hash(&raw_bytes) == RAW_SHA && hash(&bases_bytes) == BASES_SHA,
        "source hashes",
    )?;
    let raw: Value = serde_json::from_slice(&raw_bytes)?;
    let attempts = raw["work"]["nagao_attempts"]
        .as_array()
        .ok_or("attempts array")?;
    ensure(attempts.len() == 3415, "original attempt denominator")?;
    let bases: Vec<Value> = BufReader::new(bases_bytes.as_slice())
        .lines()
        .take(2)
        .map(|line| Ok(serde_json::from_str(&line?)?))
        .collect::<Result<_>>()?;
    ensure(bases.len() == 2, "two bases")?;
    let mut residuals = Vec::new();
    let mut roots = Vec::new();
    for (i, a) in attempts.iter().take(2).enumerate() {
        let input: Vec<f4::Poly> = serde_json::from_value(a["audit"]["original"].clone())?;
        ensure(
            serde_json::to_value(&input)? == bases[i]["input"],
            "original equations match H1",
        )?;
        let mut r: Vec<Vec<u64>> = serde_json::from_value(a["audit"]["lifted_roots"].clone())?;
        r.sort();
        r.dedup();
        ensure(
            serde_json::to_value(&r)? == bases[i]["expected_roots"],
            "saved validator roots match",
        )?;
        roots.push(r);
        residuals.push(Residual {
            index: i,
            u: serde_json::from_value(a["u"].clone())?,
            v: serde_json::from_value(a["v"].clone())?,
            input_sha256: digest(&input)?,
        });
    }
    let fixture = Fixture {
        schema: "nagao.template-cost.public/v1".into(),
        raw_sha256: RAW_SHA.into(),
        bases_sha256: BASES_SHA.into(),
        curve_sha256: hash(&curve_bytes),
        curve: serde_json::from_slice(&curve_bytes)?,
        residuals,
    };
    fs::create_dir(out)?;
    write_new(&Path::new(out).join("public.json"), &fixture)?;
    let public_sha256 = hash(&fs::read(Path::new(out).join("public.json"))?);
    write_new(
        &Path::new(out).join("expected.json"),
        &Expected {
            public_sha256,
            roots,
        },
    )?;
    Ok(())
}
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum Arm {
    Plain,
    Traced,
    Template,
}
fn schedule() -> Vec<Arm> {
    use Arm::*;
    [
        [Plain, Traced, Template],
        [Traced, Template, Plain],
        [Template, Plain, Traced],
        [Plain, Template, Traced],
        [Template, Traced, Plain],
    ]
    .into_iter()
    .flatten()
    .collect()
}
fn validate_fixture(f: &Fixture) -> Result<()> {
    ensure(
        f.schema == "nagao.template-cost.public/v1"
            && f.raw_sha256 == RAW_SHA
            && f.bases_sha256 == BASES_SHA,
        "fixture schema/sources",
    )?;
    ensure(
        f.curve.p == 1009 && f.curve.l == 263807419614139553,
        "fixed curve domain",
    )?;
    ensure(
        f.residuals.len() == 2
            && f.residuals
                .iter()
                .enumerate()
                .all(|(i, r)| r.index == i && r.u.len() == 4 && r.input_sha256.len() == 64),
        "fixed residual prefix",
    )?;
    Ok(())
}

// Deliberately accepts only public data. The expected object is owned by the
// controller and supplied to validation only after this interval returns.
fn run_arm(fixture: &Fixture, arm: Arm, ordinal: usize) -> Result<Value> {
    let begin = Instant::now();
    cover::clear_traces();
    cover::set_trace_scope(0x7431_0000 + ordinal as u64);
    struct Clear;
    impl Drop for Clear {
        fn drop(&mut self) {
            cover::clear_traces();
            cover::set_trace_scope(0);
        }
    }
    let clear = Clear;
    jv_bitops::enable(false);
    let c = &fixture.curve;
    let spec = cover::spec_from_curve(
        c.p,
        cover::E6(c.alpha.map(cover::E2)),
        c.l,
        c.g.native(),
        c.q.native(),
        0,
    )
    .ok_or("public curve construction")?;
    let ctx = cover::Ctx::new(&spec);
    let base = cover::factor_base(&ctx, &mut StdRng::seed_from_u64(13));
    let by_x: HashMap<_, _> = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
    let base_identity: Vec<_> = base
        .iter()
        .map(|b| {
            (
                b.x,
                b.y.0,
                b.d.u.iter().map(|e| e.0).collect::<Vec<_>>(),
                b.d.v.iter().map(|e| e.0).collect::<Vec<_>>(),
            )
        })
        .collect();
    let base_sha256 = digest(&base_identity)?;
    let mut compiled = None;
    let mut cases = Vec::new();
    for r in &fixture.residuals {
        let residual = cover::Div {
            u: r.u.iter().copied().map(cover::E2).collect(),
            v: r.v.iter().copied().map(cover::E2).collect(),
        };
        let options = cover::opts_for_mode(24, 60.0, cover::StopMode::Staircase(64));
        let mut observed_roots = Vec::new();
        let mut root_stats = Value::Null;
        let mut template_audit = Value::Null;
        let mut learning = Value::Null;
        let result = nagao_unit::decompose_using(
            &ctx,
            &base,
            &by_x,
            &residual,
            &options,
            &mut StdRng::seed_from_u64(389),
            |input, p, opts, rng| {
                if digest(&input).map_err(|_| "input serialization")? != r.input_sha256 {
                    return Err("recomputed original equations differ");
                }
                let (roots, mut stats) = if arm == Arm::Template && r.index == 0 {
                    let learning_start = Instant::now();
                    let mut full = opts.clone();
                    full.stop_staircase = None;
                    let report = f4::f4(input, 6, p, &full);
                    learning = json!({"f4_muls":report.field_ops,"f4_ms":report.ms,"timed_out":report.timed_out,"pairs_above_bound":report.pairs_above_bound,"staircase":report.staircase_at_stop,"basis_size":report.basis.len()});
                    if report.timed_out
                        || report.inconsistent
                        || report.pairs_above_bound != 0
                        || report.staircase_at_stop.is_some()
                    {
                        return Err("fresh development F4 incomplete");
                    }
                    let before = certified_relations::counters();
                    let t = template::Template::principal_syzygy(
                        input,
                        &report.basis,
                        6,
                        p,
                        7,
                        template::Limits {
                            rows: 3072,
                            columns: 2048,
                            cells: 16_000_000,
                            degree: 16,
                        },
                    )
                    .map_err(|_| "template compilation refused")?;
                    let after = certified_relations::counters();
                    let preparation_products = after
                        .modular_products
                        .checked_sub(before.modular_products)
                        .ok_or("compilation counter reset")?;
                    learning["compilation_checker_products"] = json!(preparation_products);
                    learning["compiled_rows"] = json!(t.rows());
                    learning["compiled_columns"] = json!(t.columns());
                    learning["preparation_ms"] =
                        json!(learning_start.elapsed().as_secs_f64() * 1000.0);
                    compiled = Some(t);
                    let mut stats = cover::ZeroDimStats {
                        f4_muls: report.field_ops + preparation_products,
                        f4_ms: learning_start.elapsed().as_secs_f64() * 1000.0,
                        degree_reached: report.degree_reached,
                        max_rows: report.max_rows,
                        max_cols: report.max_cols,
                        ..Default::default()
                    };
                    let roots = cover::linear_stage_from_basis(
                        &report.basis,
                        input,
                        6,
                        p,
                        rng,
                        cover::LinMode::Fast,
                        &mut stats,
                        Instant::now(),
                    );
                    (roots, stats)
                } else if arm == Arm::Template {
                    let solved = solver::solve(
                        compiled.as_ref().ok_or("no compiled template")?,
                        input,
                        p,
                        opts,
                        rng,
                        solver::Fallback::Traced,
                    )?;
                    template_audit =
                        serde_json::to_value(&solved.audit).map_err(|_| "audit serialization")?;
                    (solved.roots, solved.stats)
                } else {
                    cover::solve_zero_dim_mode(
                        input,
                        6,
                        p,
                        opts,
                        rng,
                        arm == Arm::Traced,
                        cover::LinMode::Fast,
                    )
                };
                if opts.deadline.is_some_and(|d| Instant::now() >= d) {
                    stats.timed_out = true;
                }
                observed_roots = roots.clone();
                root_stats = serde_json::to_value(&stats).map_err(|_| "stats serialization")?;
                Ok((roots, stats, None))
            },
        );
        match result {
            Ok((rows,cost,_)) => {
                let divisor_checks = rows.iter().all(|d| cover::verify_dec(&ctx.jac(), &base, &residual,d));
                cases.push(json!({"index":r.index,"roots":observed_roots,"root_stats":root_stats,"decompositions":rows,"original_divisor_checks":divisor_checks,"cost":cost,"template_audit":template_audit,"learning":learning}));
            }
            Err(error) => cases.push(json!({"index":r.index,"error":error,"roots":observed_roots,"root_stats":root_stats,"template_audit":template_audit,"learning":learning})),
        }
    }
    let setup_context_products = ctx.f.muls();
    drop(compiled);
    drop(by_x);
    drop(base);
    drop(ctx);
    drop(spec);
    drop(clear);
    let elapsed_s = begin.elapsed().as_secs_f64();
    Ok(
        json!({"arm":arm,"ordinal":ordinal,"wall_s":elapsed_s,"factor_base_x_sha256":base_sha256,"context_products_total":setup_context_products,"cases":cases,"full_native_cost_available":false}),
    )
}
fn validate(sample: &Value, expected: &Expected) -> Result<()> {
    let cases = sample["cases"].as_array().ok_or("missing cases")?;
    ensure(
        cases.len() == 2 && expected.roots.len() == 2,
        "case denominator",
    )?;
    for (i, c) in cases.iter().enumerate() {
        ensure(c.get("error").is_none(), "solver/materializer error")?;
        let mut actual: Vec<Vec<u64>> = serde_json::from_value(c["roots"].clone())?;
        actual.sort();
        actual.dedup();
        ensure(
            actual == expected.roots[i],
            "original saved root set mismatch",
        )?;
        ensure(
            c["original_divisor_checks"] == true,
            "original divisor check",
        )?;
        let s = &c["root_stats"];
        ensure(
            s["timed_out"] == false
                && s["incomplete"] == false
                && s["nonseparating"] == 0
                && s["inconsistent"] == false,
            "unresolved root status",
        )?;
    }
    Ok(())
}
fn event(log: &mut File, value: &Value) -> Result<()> {
    writeln!(log, "{value}")?;
    log.flush()?;
    log.sync_data()?;
    Ok(())
}
fn aa_ratio(a: f64, b: f64) -> Result<f64> {
    ensure(
        a.is_finite() && b.is_finite() && a > 0.0 && b > 0.0,
        "invalid A/A interval",
    )?;
    Ok(a.max(b) / a.min(b))
}
fn materialization_fingerprint(sample: &Value) -> Result<String> {
    let cases = sample["cases"].as_array().ok_or("case fingerprint")?;
    digest(
        &json!({"base":sample["factor_base_x_sha256"],"cases":cases.iter().map(|c|
        json!({"index":c["index"],"roots":c["roots"],"decompositions":c["decompositions"]})).collect::<Vec<_>>()}),
    )
}
fn run(public_path: &str, expected_path: &str, out: &str) -> Result<()> {
    ensure(
        cfg!(feature = "nagao-template-counters"),
        "counted worker required",
    )?;
    let public_bytes = fs::read(public_path)?;
    let f: Fixture = serde_json::from_slice(&public_bytes)?;
    validate_fixture(&f)?;
    let expected: Expected = serde_json::from_slice(&fs::read(expected_path)?)?;
    ensure(
        expected.public_sha256 == hash(&public_bytes),
        "validator binds public fixture",
    )?;
    ensure(!Path::new(out).exists(), "exclusive output directory")?;
    fs::create_dir(out)?;
    write_new(
        &Path::new(out).join("started.json"),
        &json!({"schema":"nagao.template-cost.started/v1","public_sha256":hash(&public_bytes),"warmups":1,"aa_intervals":2,"planned_arms":schedule(),"watchdog_seconds":600,"virtual_memory_bytes":8589934592u64}),
    )?;
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(600));
        eprintln!("component watchdog expired");
        std::process::exit(124);
    });
    let mut log = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(Path::new(out).join("events.jsonl"))?;
    let experiment_start = Instant::now();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build()?;
    let mut samples = Vec::new();
    let mut aa = Vec::new();
    let mut stopped = None;
    let mut reference_fingerprint = None;
    let orders: Vec<_> = [Arm::Traced; 3].into_iter().chain(schedule()).collect();
    for (i, arm) in orders.iter().copied().enumerate() {
        let phase = if i == 0 {
            "warmup"
        } else if i < 3 {
            "aa"
        } else {
            "panel"
        };
        event(
            &mut log,
            &json!({"event":"start","ordinal":i,"phase":phase,"arm":arm}),
        )?;
        let result = pool.install(|| run_arm(&f, arm, i).map_err(|e| e.to_string()));
        let mut sample = match result {
            Ok(s) => s,
            Err(error) => json!({"arm":arm,"ordinal":i,"error":error}),
        };
        sample["phase"] = json!(phase);
        let validation = validate(&sample, &expected).and_then(|()| {
            let fingerprint = materialization_fingerprint(&sample)?;
            if let Some(reference) = &reference_fingerprint {
                ensure(
                    reference == &fingerprint,
                    "materialization/base differs from warmup",
                )?;
            } else {
                reference_fingerprint = Some(fingerprint);
            }
            Ok(())
        });
        sample["correct"] = json!(validation.is_ok());
        if let Err(error) = &validation {
            sample["validation_error"] = json!(error.to_string());
        }
        event(&mut log, &sample)?;
        let correct = validation.is_ok();
        if phase == "aa" {
            aa.push(sample["wall_s"].as_f64().unwrap_or(0.0));
        }
        samples.push(sample);
        if !correct && i < 3 {
            stopped = Some("warmup/A/A correctness failure");
            break;
        }
        if i == 2 && aa_ratio(aa[0], aa[1])? > 1.10 {
            stopped = Some("A/A spread exceeds 10 percent");
            break;
        }
    }
    let attempted = samples.len();
    write_new(
        &Path::new(out).join("summary.json"),
        &json!({"schema":"nagao.template-cost.result/v1","public_sha256":hash(&public_bytes),"samples":samples,"stopped":stopped,"planned_intervals":orders.len(),"attempted_intervals":attempted,"not_run_intervals":orders.len()-attempted,"experiment_wall_s_including_common_warmup":experiment_start.elapsed().as_secs_f64(),"aa_max_over_min":if aa.len()==2 {Some(aa_ratio(aa[0],aa[1])?)} else {None},"process_cold":false,"algorithm_caches_cold":true,"native_bitop_hooks_enabled":false,"checker_counters":true,"full_native_cost_available":false,"signed_admission":false}),
    )?;
    Ok(())
}
pub fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("prepare") if a.len()==6 => prepare(&a[2],&a[3],&a[4],&a[5]),
        Some("run") if a.len()==5 => run(&a[2],&a[3],&a[4]),
        _ => Err("prepare RAW BASES CURVE NEW_DIR | run PUBLIC EXPECTED NEW_DIR; requires frozen protocol".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn schedule_has_five_rounds_and_each_arm_once_per_round() {
        let s = schedule();
        assert_eq!(s.len(), 15);
        for round in s.chunks(3) {
            assert_eq!(round.iter().filter(|&&a| a == Arm::Plain).count(), 1);
            assert_eq!(round.iter().filter(|&&a| a == Arm::Traced).count(), 1);
            assert_eq!(round.iter().filter(|&&a| a == Arm::Template).count(), 1);
        }
    }
    #[test]
    fn noise_ratio_and_invalid_samples_are_explicit() {
        assert_eq!(aa_ratio(1.0, 1.05).unwrap(), 1.05);
        assert!(aa_ratio(1.0, 1.11).unwrap() > 1.10);
        assert!(aa_ratio(0.0, 1.0).is_err());
        assert!(aa_ratio(f64::NAN, 1.0).is_err());
        assert!(validate(
            &json!({"error":"retained"}),
            &Expected {
                public_sha256: String::new(),
                roots: Vec::new()
            }
        )
        .is_err());
    }
    #[test]
    fn solver_public_types_reject_answer_fields() {
        assert!(serde_json::from_value::<Residual>(
            json!({"index":0,"u":[],"v":[],"input_sha256":"x","roots":[[1]]})
        )
        .is_err());
        assert!(serde_json::from_value::<Point>(
            json!({"x":vec![[0,0];3],"y":vec![[0,0];3],"inf":false,"d":7})
        )
        .is_err());
    }
}
