//! Complete-set PKM development instrument. Historical drivers are unchanged.
//! CLI production, export and replay all require the unchanged checkpoint gate.
//! Native unit tests are development checks, never an admitted timing panel.
#[path = "pkm_oracle_program/algebra.rs"]
mod algebra;
#[path = "pkm_oracle_program/pipeline.rs"]
mod pipeline;
#[path = "pkm_oracle_program/reference.rs"]
mod reference;

use algebra::{Arithmetic, Instance};
use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};
use crypto_lib::cryptanalysis::glv_invariant_base::poly_roots_fp_counted_until;
use crypto_lib::cryptanalysis::prime_fast::{FastCurve, Pt};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
struct Point {
    x: u64,
    y: u64,
}
type Witness = Vec<Point>;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Arm {
    SymmetricS5,
    PlainS5,
    PlainChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Limits {
    solver_millis: u64,
    max_assignments: usize,
    max_witnesses: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema: String,
    p: u64,
    a: u64,
    b: u64,
    t: usize,
    g: u64,
    zeta: u64,
    m: usize,
    target: Point,
    arm: Arm,
    limits: Limits,
}

impl Request {
    fn instance(&self) -> Instance {
        Instance {
            p: self.p,
            a: self.a,
            b: self.b,
            t: self.t,
            g: self.g,
            zeta: self.zeta,
        }
    }
    fn validate(&self) -> Result<(), String> {
        if self.schema != "pkm.oracle-request/v1" {
            return Err("unsupported request schema".into());
        }
        algebra::validate_parameters(&self.instance(), self.m)?;
        if self.limits.max_assignments == 0 || self.limits.max_witnesses == 0 {
            return Err("output/assignment limits must be positive".into());
        }
        let mut independent = reference::Curve::new(self);
        if !independent.on_curve(self.target) {
            return Err("noncanonical or off-curve finite target".into());
        }
        Ok(())
    }
    fn digest(&self) -> String {
        format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(self).expect("request is serializable"))
        )
    }
}

#[derive(Default, Debug, Serialize)]
struct Phase {
    elapsed_ns: u128,
    /// Counts only arithmetic routed through the inherited constructor helpers.
    helper_arithmetic: Arithmetic,
    calls: u64,
}
impl Phase {
    fn finish(&mut self, start: Instant, counts: Arithmetic) {
        self.elapsed_ns += start.elapsed().as_nanos();
        let delta = algebra::counts().since(counts);
        self.helper_arithmetic.additions += delta.additions;
        self.helper_arithmetic.subtractions += delta.subtractions;
        self.helper_arithmetic.multiplications += delta.multiplications;
        self.helper_arithmetic.inverse_calls += delta.inverse_calls;
        self.helper_arithmetic.sqrt_calls += delta.sqrt_calls;
        self.calls += 1;
    }
}

#[derive(Default, Debug, Serialize)]
struct Ledger {
    validation: Phase,
    construction: Phase,
    solve_excluding_callbacks: Phase,
    root_decode: Phase,
    lift: Phase,
    signs: Phase,
    verification: Phase,
    total_ns: u128,
    unattributed_ns: u128,
    signed_tuples: u64,
    point_addition_calls: u64,
    decode_root_multiplications: u64,
    independently_verified_hits: u64,
    rejected_root_sets: u64,
    duplicate_witness_hits: u64,
    independent_verification_counts: reference::Counts,
}
impl Ledger {
    fn callbacks_ns(&self) -> u128 {
        self.root_decode.elapsed_ns
            + self.lift.elapsed_ns
            + self.signs.elapsed_ns
            + self.verification.elapsed_ns
    }
    fn finish(&mut self, start: Instant) {
        self.total_ns = start.elapsed().as_nanos();
        let charged = self.validation.elapsed_ns
            + self.construction.elapsed_ns
            + self.solve_excluding_callbacks.elapsed_ns
            + self.callbacks_ns();
        self.unattributed_ns = self
            .total_ns
            .checked_sub(charged)
            .expect("exclusive phase intervals fit total");
    }
}

#[derive(Debug, Serialize)]
struct Report {
    schema: &'static str,
    request_sha256: String,
    request: Request,
    complete: bool,
    status: &'static str,
    cutoff: Option<&'static str>,
    witnesses: Vec<Witness>,
    visitor: Option<f4_fp::VisitReport>,
    ledger: Ledger,
    system_sha256: Option<String>,
    system_variables: usize,
    comparison_admissible: bool,
    unpriced_work: Vec<&'static str>,
    process_peak_rss_kib: Option<u64>,
    memory_scope: &'static str,
    execution_scope: &'static str,
}

fn point(c: &FastCurve, q: Point) -> Pt {
    Pt {
        x: c.f.to_mont(q.x),
        y: c.f.to_mont(q.y),
    }
}
fn coords(c: &FastCurve, q: Pt) -> Point {
    Point {
        x: c.f.from_mont(q.x),
        y: c.f.from_mont(q.y),
    }
}

fn system(r: &Request) -> (Vec<Poly>, usize) {
    let (eqs, n) = match r.arm {
        Arm::SymmetricS5 => algebra::symmetric_system(&r.instance(), r.m, r.target.x),
        Arm::PlainS5 => algebra::plain_system(&r.instance(), r.m, r.target.x, false),
        Arm::PlainChain => algebra::plain_system(&r.instance(), r.m, r.target.x, true),
    };
    (eqs.iter().map(|f| f.to_poly(r.p)).collect(), n)
}

/// Same syntax as examples/gb_cas_export.rs; input export is not qualification
/// of an external solver or its F_p root extraction.
fn msolve_input(polys: &[Poly], n: usize, p: u64) -> String {
    let variables = (1..=n)
        .map(|i| format!("x{i}"))
        .collect::<Vec<_>>()
        .join(",");
    let mut equations = Vec::new();
    for poly in polys {
        let mut terms = Vec::new();
        for (powers, coefficient) in poly {
            if *coefficient == 0 {
                continue;
            }
            let mut factors = vec![coefficient.to_string()];
            for (i, &exponent) in powers.iter().enumerate() {
                if exponent == 1 {
                    factors.push(format!("x{}", i + 1));
                } else if exponent > 1 {
                    factors.push(format!("x{}^{exponent}", i + 1));
                }
            }
            terms.push(factors.join("*"));
        }
        equations.push(if terms.is_empty() {
            "0".into()
        } else {
            terms.join("+")
        });
    }
    format!("{variables}\n{p}\n{}\n", equations.join(",\n"))
}

fn peak_rss() -> Option<u64> {
    std::fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find_map(|s| {
            s.strip_prefix("VmHWM:")?
                .split_whitespace()
                .next()?
                .parse()
                .ok()
        })
}

fn run(r: &Request) -> Result<Report, String> {
    let total = Instant::now();
    let mut ledger = Ledger::default();
    let before = algebra::counts();
    let validation = Instant::now();
    r.validate()?;
    let curve = FastCurve::new("public-pkm-oracle", r.p, r.a, r.b, 1);
    ledger.validation.finish(validation, before);
    let mut found = BTreeSet::new();
    let mut cutoff = None;
    let mut visitor = None;
    let mut system_sha256 = None;
    let mut n_vars = r.m * r.t
        + if r.arm == Arm::PlainChain && r.m >= 3 {
            r.m - 2
        } else {
            0
        };
    let mut verification_failure = false;
    if r.limits.solver_millis == 0 {
        cutoff = Some("zero_solver_budget");
    } else {
        let before = algebra::counts();
        let construction = Instant::now();
        let (polys, n) = system(r);
        n_vars = n;
        system_sha256 = Some(format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&polys).expect("polynomials serialize"))
        ));
        ledger.construction.finish(construction, before);
        let budget = Duration::from_millis(r.limits.solver_millis);
        let opts = F4Options::new(Ordering::Grevlex, (n + 20) as u32).with_budget(budget);
        let deadline = opts.deadline;
        let solve = Instant::now();
        let mut callbacks = 0;
        let mut verifier = reference::Curve::new(r);
        let result = f4_fp::solve_visit(&polys, n, r.p, &opts, &vec![None; n], |assignment| {
            callbacks += 1;
            let before = algebra::counts();
            let decode = Instant::now();
            let roots = if r.arm == Arm::SymmetricS5 {
                let mut polynomial = vec![0; r.m + 1];
                polynomial[r.m] = 1;
                for k in 1..=r.m {
                    polynomial[r.m - k] = if k % 2 == 0 {
                        assignment[k - 1]
                    } else {
                        algebra::subm(0, assignment[k - 1], r.p)
                    };
                }
                poly_roots_fp_counted_until(
                    &polynomial,
                    r.p,
                    0x504b4d,
                    &mut ledger.decode_root_multiplications,
                    deadline,
                )
            } else {
                Some((0..r.m).map(|i| assignment[i * r.t]).collect())
            };
            let Some(mut roots) = roots else {
                ledger.root_decode.finish(decode, before);
                cutoff = Some("root_decode_deadline");
                return true;
            };
            roots.sort_unstable();
            let distinct = roots.windows(2).all(|w| w[0] != w[1]);
            let valid = roots.len() == r.m
                && distinct
                && roots.iter().all(|&w| algebra::powm(w, 1 << r.t, r.p) == 1);
            ledger.root_decode.finish(decode, before);
            if valid {
                let before = algebra::counts();
                let lift = Instant::now();
                let mut lifted = Vec::new();
                for w in roots {
                    let x = algebra::mulm(r.g, w, r.p);
                    let x2 = algebra::mulm(x, x, r.p);
                    let x3 = algebra::mulm(x2, x, r.p);
                    let ax = algebra::mulm(r.a, x, r.p);
                    let rhs = algebra::addm(x3, ax, r.p);
                    let rhs = algebra::addm(rhs, r.b, r.p);
                    let Some(y) = algebra::sqrtm(rhs, r.p) else {
                        lifted.clear();
                        break;
                    };
                    lifted.push(point(&curve, Point { x, y }));
                }
                ledger.lift.finish(lift, before);
                if lifted.len() == r.m {
                    for signs in 0..1usize << r.m {
                        let before = algebra::counts();
                        let sign_start = Instant::now();
                        ledger.signed_tuples += 1;
                        let mut sum = None;
                        let mut witness = Vec::new();
                        for (i, &q) in lifted.iter().enumerate() {
                            let q = if signs >> i & 1 == 0 { q } else { curve.neg(q) };
                            sum = curve.add(sum, Some(q));
                            ledger.point_addition_calls += 1;
                            witness.push(coords(&curve, q));
                        }
                        witness.sort_unstable();
                        let hit = sum == Some(point(&curve, r.target));
                        ledger.signs.finish(sign_start, before);
                        if hit {
                            let before = algebra::counts();
                            let verify = Instant::now();
                            let correct = verifier.verify(r, &witness);
                            ledger.verification.finish(verify, before);
                            if !correct {
                                verification_failure = true;
                                return true;
                            }
                            ledger.independently_verified_hits += 1;
                            if !found.insert(witness) {
                                ledger.duplicate_witness_hits += 1;
                            }
                            if found.len() >= r.limits.max_witnesses {
                                cutoff = Some("witness_limit");
                                return true;
                            }
                        }
                    }
                }
            } else {
                ledger.rejected_root_sets += 1;
            }
            if callbacks >= r.limits.max_assignments {
                cutoff = Some("assignment_limit");
                return true;
            }
            false
        })?;
        let inclusive_ns = solve.elapsed().as_nanos();
        ledger.solve_excluding_callbacks.elapsed_ns = inclusive_ns
            .checked_sub(ledger.callbacks_ns())
            .expect("callback intervals are inside visitor interval");
        ledger.solve_excluding_callbacks.calls = 1;
        ledger.independent_verification_counts = verifier.counts;
        visitor = Some(result);
    }
    let complete =
        visitor.as_ref().is_some_and(|v| v.complete) && cutoff.is_none() && !verification_failure;
    let status = if verification_failure {
        "verification_failed"
    } else if complete && found.is_empty() {
        "complete_miss"
    } else if complete {
        "complete_with_witnesses"
    } else if found.is_empty() {
        "unknown"
    } else {
        "incomplete_with_witnesses"
    };
    if cutoff.is_none() && visitor.as_ref().is_some_and(|v| v.timed_out) {
        cutoff = Some("solver_deadline");
    }
    ledger.finish(total);
    Ok(Report { schema: "pkm.oracle-output/v1", request_sha256: r.digest(), request: r.clone(), complete,
        status, cutoff, witnesses: found.into_iter().collect(), visitor, ledger, system_sha256,
        system_variables: n_vars, comparison_admissible: false,
        unpriced_work: vec!["F4 symbolic preprocessing and substitution arithmetic beyond emitted counters",
            "root finder additions, inversions and control work beyond emitted multiplication counts",
            "Montgomery curve/setup arithmetic beyond point-addition call counts",
            "allocation, hash/table operations, comparisons and serialization: counts unavailable"],
        process_peak_rss_kib: peak_rss(), memory_scope: "whole-process lifetime high-water RSS including earlier work; not isolated per-query peak",
        execution_scope: "development instrument; diagnostic clocks; no complete GAE, admitted L2, pipeline or speed claim" })
}

fn require_checkpoint() -> Result<(), String> {
    let result = std::process::Command::new("bash")
        .args(["scripts/checkpoint.sh", "gate"])
        .status()
        .map_err(|e| e.to_string())?;
    if result.success() {
        Ok(())
    } else {
        Err("checkpoint gate refused experiment or replay".into())
    }
}

fn read_request(path: &str) -> Result<Request, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    if bytes.len() > 1_048_576 {
        return Err("request exceeds 1 MiB".into());
    }
    let r: Request = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    Ok(r)
}

fn replay(r: &Request, record: &serde_json::Value) -> Result<reference::Certificate, String> {
    r.validate()?;
    if record["schema"] != "pkm.oracle-output/v1"
        || record["request_sha256"] != r.digest()
        || record["request"] != serde_json::to_value(r).map_err(|e| e.to_string())?
    {
        return Err("output/request binding mismatch".into());
    }
    if record["comparison_admissible"] != false {
        return Err("instrument cannot admit a cost comparison".into());
    }
    let witnesses: Vec<Witness> =
        serde_json::from_value(record["witnesses"].clone()).map_err(|e| e.to_string())?;
    let complete = record["complete"]
        .as_bool()
        .ok_or("missing completion flag")?;
    let visitor_complete = record["visitor"]["complete"].as_bool().unwrap_or(false);
    if complete
        != (visitor_complete
            && record["cutoff"].is_null()
            && record["status"] != "verification_failed")
    {
        return Err("completion/visitor/cutoff metadata disagree".into());
    }
    let expected_status = if complete && witnesses.is_empty() {
        "complete_miss"
    } else if complete {
        "complete_with_witnesses"
    } else if witnesses.is_empty() {
        "unknown"
    } else {
        "incomplete_with_witnesses"
    };
    if record["status"] != expected_status {
        return Err("status does not match completion/output".into());
    }
    let ledger = &record["ledger"];
    let number = |v: &serde_json::Value| {
        v.as_u64()
            .map(u128::from)
            .ok_or("invalid bounded nanosecond count")
    };
    let mut sum = number(&ledger["unattributed_ns"])?;
    for name in [
        "validation",
        "construction",
        "solve_excluding_callbacks",
        "root_decode",
        "lift",
        "signs",
        "verification",
    ] {
        sum += number(&ledger[name]["elapsed_ns"])?;
    }
    if sum != number(&ledger["total_ns"])? {
        return Err("exclusive phase ledger does not reconcile".into());
    }
    reference::check(r, &witnesses, complete)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: pkm_oracle_program run REQUEST.json | verify REQUEST.json OUTPUT.json | export REQUEST.json | toy-dlp DLP.json | verify-toy-dlp DLP.json OUTPUT.json");
        return Err("missing command/request".into());
    }
    require_checkpoint()?;
    if args[1] == "toy-dlp" && args.len() == 3 {
        let r: pipeline::Request = serde_json::from_slice(&std::fs::read(&args[2])?)?;
        let report = pipeline::run(&r)?;
        println!("{}", serde_json::to_string(&report)?);
        return Ok(());
    }
    if args[1] == "verify-toy-dlp" && args.len() == 4 {
        let r: pipeline::Request = serde_json::from_slice(&std::fs::read(&args[2])?)?;
        let record: serde_json::Value = serde_json::from_slice(&std::fs::read(&args[3])?)?;
        println!(
            "{}",
            serde_json::to_string(&pipeline::replay(&r, &record)?)?
        );
        return Ok(());
    }
    let r = read_request(&args[2])?;
    match args[1].as_str() {
        "run" if args.len() == 3 => {
            let report = run(&r)?;
            println!("{}", serde_json::to_string(&report)?);
            if report.status == "verification_failed" {
                return Err("independent verification failed".into());
            }
        }
        "export" if args.len() == 3 => {
            r.validate()?;
            let (polys, n) = system(&r);
            println!(
                "{}",
                serde_json::json!({"schema":"pkm.polynomial-system/v1","request_sha256":r.digest(),"p":r.p,"variables":n,"order":"grevlex","polynomials":polys,"scope":"canonical native system; external solver not qualified"})
            );
        }
        "export-msolve" if args.len() == 3 => {
            r.validate()?;
            let (polys, n) = system(&r);
            print!("{}", msolve_input(&polys, n, r.p));
        }
        "verify" if args.len() == 4 => {
            let record: serde_json::Value = serde_json::from_slice(&std::fs::read(&args[3])?)?;
            let certificate = replay(&r, &record)?;
            println!("{}", serde_json::to_string(&certificate)?);
        }
        _ => return Err("invalid command/argument count".into()),
    }
    Ok(())
}

#[cfg(test)]
#[path = "pkm_oracle_program/tests.rs"]
mod tests;
