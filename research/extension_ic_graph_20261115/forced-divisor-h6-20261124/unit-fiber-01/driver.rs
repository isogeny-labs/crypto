//! Supplied-state count study. Reference answers never enter solve_backend.
use super::{
    jv_forced_divisor::Family,
    jv_forced_divisor_unit::{CoefficientWork, UnitSystem},
};
use crypto_lib::cryptanalysis::{
    f4_fp::{self, F4Options},
    jv_bitops,
    jv_cover::{self as cover, Div, Fld, Fq, Hyp, LinMode, ZeroDimStats, E2},
    jv_sieve::FpRing,
};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn ensure(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn digest(v: &impl Serialize) -> Result<String> {
    Ok(sha(&serde_json::to_vec(v)?))
}
fn native(v: &[[u64; 2]]) -> Vec<E2> {
    v.iter().copied().map(E2).collect()
}
fn encode(v: &[E2]) -> Vec<[u64; 2]> {
    v.iter().map(|e| e.0).collect()
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Small {
    #[serde(rename = "original_index")]
    _original_index: usize,
    x: u64,
    y: [u64; 2],
    sign: i8,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Large {
    key: [u64; 6],
    coefficient: i8,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Seed {
    schema: String,
    raw_sha256: String,
    p: u64,
    source_id: usize,
    h: Vec<[u64; 2]>,
    a: Vec<[u64; 2]>,
    c: Vec<[u64; 2]>,
    norm: Vec<u64>,
    small: Vec<Small>,
    remainders: Vec<Large>,
}
fn divisor(f: &Fq, r: &Large) -> Result<Div<E2>> {
    ensure(
        [-1, 1].contains(&r.coefficient) && r.key.iter().all(|&x| x < f.p),
        "canonical quadratic",
    )?;
    let mut v = vec![E2([r.key[2], r.key[3]]), E2([r.key[4], r.key[5]])];
    if r.coefficient == -1 {
        for x in &mut v {
            *x = f.neg(x);
        }
    }
    while v.last() == Some(&E2::ZERO) {
        v.pop();
    }
    Ok(Div {
        u: vec![f.from_fp(r.key[0]), f.from_fp(r.key[1]), E2::ONE],
        v,
    })
}
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Mode {
    #[cfg(test)]
    FullReference,
    Plain,
    Traced,
}
fn options(mode: Mode) -> F4Options {
    cover::opts_for_mode(
        24,
        10.0,
        match mode {
            #[cfg(test)]
            Mode::FullReference => cover::StopMode::Full,
            _ => cover::StopMode::Staircase(128),
        },
    )
}
fn solve_backend(
    input: &[f4_fp::Poly],
    p: u64,
    opts: &F4Options,
    rng: &mut StdRng,
    mode: Mode,
) -> (Vec<Vec<u64>>, ZeroDimStats) {
    if opts.deadline.is_some_and(|d| Instant::now() >= d) {
        return (
            vec![],
            ZeroDimStats {
                timed_out: true,
                ..Default::default()
            },
        );
    }
    let (mut roots, mut stats) = cover::solve_zero_dim_mode(
        input,
        6,
        p,
        opts,
        rng,
        matches!(mode, Mode::Traced),
        LinMode::Fast,
    );
    roots.sort();
    roots.dedup();
    if opts.deadline.is_some_and(|d| Instant::now() >= d) {
        stats.timed_out = true;
    }
    (roots, stats)
}
fn resolved(s: &ZeroDimStats) -> bool {
    !s.timed_out && !s.incomplete && s.nonseparating == 0
}
fn event(log: &mut File, v: &Value) -> Result<()> {
    writeln!(log, "{v}")?;
    log.flush()?;
    log.sync_data()?;
    Ok(())
}
fn expected_support(seed: &Seed, f: &Fq) -> Result<Value> {
    let small: Vec<_> = seed
        .small
        .iter()
        .map(|b| {
            let y = if b.sign == 1 {
                E2(b.y)
            } else {
                f.neg(&E2(b.y))
            };
            json!({"x":b.x,"y":y.0})
        })
        .collect();
    let quadratic = seed
        .remainders
        .iter()
        .map(|r| {
            let d = divisor(f, r)?;
            Ok(json!({"u":encode(&d.u),"v":encode(&d.v)}))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(json!({"small":small,"quadratic":quadratic}))
}
fn run_case(
    f: &Fq,
    h: &[E2],
    family: &Family,
    unit: &UnitSystem,
    parameter: u64,
    index: usize,
    mode: Mode,
) -> Result<Value> {
    let opts = options(mode); // same budget covers specialization, roots and checks
    let (input, specialization_work) = unit.specialize(parameter)?;
    let input_sha256 = digest(&input)?;
    let (reduced_roots, mut stats) = solve_backend(
        &input,
        f.p,
        &opts,
        &mut StdRng::seed_from_u64(0x9470_0000 ^ index as u64),
        mode,
    );
    let mut lifting_work = CoefficientWork::default();
    let mut roots = Vec::new();
    for root in &reduced_roots {
        ensure(
            root.len() == 6 && root.iter().all(|&x| x < f.p),
            "canonical reduced root",
        )?;
        ensure(
            input.iter().all(|q| f4_fp::eval(q, root, f.p) == 0),
            "reduced root replay",
        )?;
        let (point, work) = unit.lift(parameter, root)?;
        lifting_work.products += work.products;
        lifting_work.additions += work.additions;
        roots.push(point[1..].to_vec());
    }
    roots.sort();
    roots.dedup();
    let before = f.muls();
    let ring = FpRing::new(f.p);
    let jac = Hyp {
        f,
        h: h.to_vec(),
        g: 3,
    };
    let mut functions = Vec::new();
    let mut invalid = false;
    for root in &roots {
        ensure(
            root.len() == 7 && root.iter().all(|&x| x < f.p),
            "canonical seven-coordinate root",
        )?;
        let mut point = root.clone();
        point.insert(0, parameter);
        if family.evaluate(&point)?.iter().any(|&v| v != 0) {
            invalid = true;
        }
        let mut row = json!({"point":point});
        match family.instantiate(f, &point) {
            Err(error) => {
                row["function_refusal"] = json!(error);
                invalid |= error != "C is not invertible on forced divisor";
            }
            Ok(value) => {
                let polynomial: Vec<_> = value.quotient.iter().map(|v| v.0[0]).collect();
                let small_roots = ring.roots_by_table(&polynomial);
                row["small_roots"] = json!(small_roots);
                row["normalized_a"] = json!(encode(&value.a));
                row["normalized_c"] = json!(encode(&value.c));
                row["norm"] = json!(encode(&value.norm));
                match family.materialize(f, &value, &small_roots) {
                    Err(error) => {
                        row["support_refusal"] = json!(error);
                        invalid |= [
                            "candidate replay mismatch",
                            "complete support factorization",
                            "small root exact division",
                            "remaining original Mumford identity",
                        ]
                        .contains(&error);
                    }
                    Ok(support) => {
                        let mut sum = jac.identity();
                        for (x, y) in &support.small {
                            sum = jac.add(
                                &sum,
                                &Div {
                                    u: vec![f.neg(&f.from_fp(*x)), E2::ONE],
                                    v: vec![*y],
                                },
                            );
                        }
                        for d in &support.quadratic {
                            sum = jac.add(&sum, d);
                        }
                        let group = jac.is_identity(&sum);
                        invalid |= !group;
                        row["support"] = json!({"small":support.small.iter().map(|(x,y)|json!({"x":x,"y":y.0})).collect::<Vec<_>>(),"quadratic":support.quadratic.iter().map(|d|json!({"u":encode(&d.u),"v":encode(&d.v)})).collect::<Vec<_>>()});
                        row["original_group_replay"] = json!(group);
                    }
                }
            }
        }
        functions.push(row);
    }
    if opts.deadline.is_some_and(|d| Instant::now() >= d) {
        stats.timed_out = true;
    }
    let is_resolved = resolved(&stats);
    Ok(
        json!({"case":index,"kind":if index==0 {"retained-positive-control"} else {"development"},"parameter":parameter,"input_sha256":input_sha256,"roots":roots,"reduced_roots":reduced_roots,"specialization_work":specialization_work,"lifting_work":lifting_work,"root_stats":stats,"resolved":is_resolved,"functions":functions,"invalid_original_replay":invalid,"post_legacy_fq_products":f.muls()-before}),
    )
}
fn write_new(path: &Path, v: &Value) -> Result<()> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut f, v)?;
    writeln!(f)?;
    f.sync_all()?;
    Ok(())
}
fn core(
    seed: &Seed,
    geometry: &Value,
    frozen_reference: &Value,
    out: &Path,
    seed_sha: &str,
    geometry_sha: &str,
) -> Result<()> {
    jv_bitops::enable(false);
    let f = Fq::new(seed.p);
    let h = native(&seed.h);
    let forced = divisor(&f, &seed.remainders[0])?;
    let admission_family = Family::new(&f, &h, &forced.u, &forced.v)?;
    let planted = admission_family.recover_parameters(&f, &native(&seed.a), &native(&seed.c))?;
    ensure(
        geometry["coordinates"] == serde_json::to_value(&planted)?
            && geometry["equations"] == serde_json::to_value(admission_family.polynomials())?,
        "unchanged geometry and equation source",
    )?;
    ensure(
        geometry["selected_development_parameter"] == 0 && geometry["jacobian_rank"] == 7,
        "frozen development coordinate",
    )?;
    let expected = expected_support(seed, &f)?;
    let scale = f.inv(&f.sq(&native(&seed.c)[1]));
    let expected_norm: Vec<_> = seed
        .norm
        .iter()
        .map(|&n| f.mul(&f.from_fp(n), &scale))
        .collect();
    let positive = admission_family.instantiate(&f, &planted)?;
    ensure(positive.norm == expected_norm, "retained norm binding")?;
    let admission_products = f.muls();
    let parameters: Vec<_> = std::iter::once(planted[0]).chain(0..8).collect();
    let mut log = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("events.jsonl"))?;
    let mut samples = Vec::new();
    let reference: Vec<_> = frozen_reference["samples"]
        .as_array()
        .ok_or("reference samples")?
        .iter()
        .filter(|s| s["mode"] == "full-reference")
        .cloned()
        .collect();
    ensure(
        reference.len() == 9
            && frozen_reference["complete_reference_comparison"] == true
            && frozen_reference["parameters"] == json!(parameters),
        "complete frozen reference",
    )?;
    let mut stopped = None;
    let mut comparisons_pass = true;
    let mut setups = Vec::new();
    'arms: for (arm, mode) in [Mode::Plain, Mode::Traced].into_iter().enumerate() {
        cover::clear_traces();
        cover::set_trace_scope(0x4836_0100 + arm as u64);
        let f = Fq::new(seed.p);
        let family = Family::new(&f, &h, &forced.u, &forced.v)?;
        let (unit, unit_work) = UnitSystem::new(&family)?;
        setups.push(json!({"mode":mode,"constructor_legacy_fq_products":f.muls(),"unit_coefficient_work":unit_work}));
        for (index, &parameter) in parameters.iter().enumerate() {
            event(
                &mut log,
                &json!({"event":"start","mode":mode,"case":index,"parameter":parameter}),
            )?;
            let mut sample = match run_case(&f, &h, &family, &unit, parameter, index, mode) {
                Ok(s) => s,
                Err(error) => {
                    json!({"case":index,"parameter":parameter,"error":error.to_string(),"resolved":false,"invalid_original_replay":true})
                }
            };
            sample["mode"] = json!(mode);
            if index == 0 {
                let has_known = sample["roots"]
                    .as_array()
                    .is_some_and(|r| r.iter().any(|r| r == &json!(planted[1..].to_vec())));
                let known_support = sample["functions"].as_array().is_some_and(|rows| {
                    rows.iter().any(|r| {
                        r["point"] == json!(planted)
                            && r["support"] == expected
                            && r["original_group_replay"] == true
                    })
                });
                sample["retained_positive_control_pass"] =
                    json!(has_known && known_support && sample["resolved"] == true);
                if sample["retained_positive_control_pass"] != true {
                    stopped = Some("unit positive control unresolved or missing");
                }
            }
            {
                let r = &reference[index];
                let same_flags = ["timed_out", "incomplete", "inconsistent", "nonseparating"]
                    .iter()
                    .all(|k| sample["root_stats"][k] == r["root_stats"][k]);
                sample["resolution_flags_match_full_reference"] = json!(same_flags);
                let matches = sample["roots"] == r["roots"]
                    && sample["functions"] == r["functions"]
                    && sample["resolved"] == true
                    && r["resolved"] == true
                    && sample["invalid_original_replay"] == false
                    && r["invalid_original_replay"] == false;
                sample["matches_full_reference"] = json!(matches && same_flags);
                comparisons_pass &= matches && same_flags;
                if index == 0 && !(matches && same_flags) {
                    stopped = Some("unit positive full-reference mismatch");
                }
            }
            event(&mut log, &sample)?;
            samples.push(sample);
            if stopped.is_some() {
                comparisons_pass = false;
                break 'arms;
            }
        }
    }
    cover::clear_traces();
    cover::set_trace_scope(0);
    let attempted = samples.len();
    write_new(
        &out.join("summary.json"),
        &json!({"schema":"graph-ic.h6-unit-fiber-count/v1","seed_sha256":seed_sha,"geometry_sha256":geometry_sha,"reference_sha256":"71d049019f747b85ee1f71086fadf878ec9b33e42cceb8d4d282b38686c7a3db","source_id":seed.source_id,"parameters":parameters,"samples":samples,"setups":setups,"admission_and_positive_binding_legacy_fq_products":admission_products,"stopped":stopped,"planned_calls":18,"attempted_calls":attempted,"not_run_calls":18-attempted,"complete_reference_comparison":comparisons_pass,"full_native_cost_available":false,"wall_comparison":false,"signed_verification":false,"provided_state_excluded":"original graph collection and fixture extraction; frozen reference validation was measured separately"}),
    )
}
fn run(seed_path: &str, geometry_path: &str, reference_path: &str, out: &str) -> Result<()> {
    ensure(!Path::new(out).exists(), "exclusive pilot directory")?;
    let seed_bytes = fs::read(seed_path)?;
    let geometry_bytes = fs::read(geometry_path)?;
    let reference_bytes = fs::read(reference_path)?;
    ensure(
        sha(&reference_bytes) == "71d049019f747b85ee1f71086fadf878ec9b33e42cceb8d4d282b38686c7a3db",
        "frozen reference hash",
    )?;
    let frozen_reference: Value = serde_json::from_slice(&reference_bytes)?;
    let seed: Seed = serde_json::from_slice(&seed_bytes)?;
    let geometry: Value = serde_json::from_slice(&geometry_bytes)?;
    ensure(
        seed.schema == "graph-ic.forced-divisor.fixture/v1"
            && seed.raw_sha256
                == "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d"
            && seed.p == 1009
            && seed.source_id == 1
            && seed.remainders.len() == 2
            && seed.small.len() == 5,
        "frozen supplied state",
    )?;
    ensure(
        geometry["fixture_sha256"] == sha(&seed_bytes),
        "geometry/fixture binding",
    )?;
    fs::create_dir(out)?;
    write_new(
        &Path::new(out).join("started.json"),
        &json!({"schema":"graph-ic.h6-unit-fiber-started/v1","seed_sha256":sha(&seed_bytes),"geometry_sha256":sha(&geometry_bytes),"reference_sha256":sha(&reference_bytes),"planned_calls":18,"watchdog_seconds":360,"virtual_memory_bytes":8589934592u64}),
    )?;
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(360));
        eprintln!("fiber watchdog expired");
        std::process::exit(124);
    });
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build()?;
    pool.install(|| {
        core(
            &seed,
            &geometry,
            &frozen_reference,
            Path::new(out),
            &sha(&seed_bytes),
            &sha(&geometry_bytes),
        )
        .map_err(|e| e.to_string())
    })
    .map_err(Into::into)
}
pub fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() == 6 && a[1] == "run" {
        run(&a[2], &a[3], &a[4], &a[5])
    } else {
        Err("run PUBLIC_SEED GEOMETRY FROZEN_REFERENCE NEW_OUTPUT; requires frozen protocol".into())
    }
}
#[cfg(test)]
mod controls {
    use super::*;
    fn shape() -> Vec<f4_fp::Poly> {
        let mut input = Vec::new();
        for i in 0..5 {
            let mut a = vec![0; 6];
            a[i] = 1;
            let mut b = vec![0; 6];
            b[5] = 1;
            input.push(vec![(a, 1), (b, 100)]);
        }
        let mut a = vec![0; 6];
        a[5] = 2;
        input.push(vec![(a, 1), (vec![0; 6], 100)]);
        input
    }
    #[test]
    fn six_variable_root_callback_and_checked_trace_match_known_points() {
        let input = shape();
        let expected = vec![vec![1; 6], vec![100; 6]];
        cover::clear_traces();
        cover::set_trace_scope(0x4836_ffff);
        for mode in [Mode::FullReference, Mode::Plain, Mode::Traced, Mode::Traced] {
            let (roots, stats) = solve_backend(
                &input,
                101,
                &options(mode),
                &mut StdRng::seed_from_u64(947),
                mode,
            );
            assert_eq!(roots, expected);
            assert!(resolved(&stats));
            assert!(roots
                .iter()
                .all(|r| input.iter().all(|p| f4_fp::eval(p, r, 101) == 0)));
        }
        cover::clear_traces();
        cover::set_trace_scope(0);
    }
    #[test]
    fn expired_callback_never_starts_fresh_work() {
        let mut opts = options(Mode::Traced);
        opts.deadline = Some(Instant::now() - Duration::from_secs(1));
        let (roots, stats) = solve_backend(
            &shape(),
            101,
            &opts,
            &mut StdRng::seed_from_u64(947),
            Mode::Traced,
        );
        assert!(roots.is_empty() && stats.timed_out);
        assert_eq!(stats.f4_muls, 0);
        assert!(serde_json::from_value::<Large>(
            json!({"key":[0,0,0,0,0,0],"coefficient":1,"actual_scalar":17})
        )
        .is_err());
    }
}
