//! Counted, matched weighted-slot diagnostics on the cubic Weil restriction.
//! No whole-field/group tables. No planted scalar reaches the solver.
use crypto_lib::cryptanalysis::curve_id;
use crypto_lib::cryptanalysis::gaudry_cubic::{
    generate_instance3, weil_s3_pair_test, Instance3, Pt3, SubspaceBase,
};
use crypto_lib::cryptanalysis::residual_walk::RelationSystem;
use num_bigint::BigUint;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

type Terms = Vec<(usize, i64)>;

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq)]
struct Counts {
    budget_exhausted: bool,
    pair_tests: u64,
    pair_fp_muls_legacy: u64,
    oracle_group_ops: u64,
    oracle_field_muls_legacy: u64,
    reference_checks: u64,
    successful_decompositions: u64,
    verification_group_ops: u64,
}

fn digest(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}

fn normalize(terms: impl IntoIterator<Item = (usize, i64)>) -> Terms {
    let mut out = BTreeMap::new();
    for (i, c) in terms {
        *out.entry(i).or_insert(0) += c;
    }
    out.into_iter().filter(|(_, c)| *c != 0).collect()
}

fn signed_base(inst: &Instance3, base: &SubspaceBase) -> Vec<Pt3> {
    base.points
        .iter()
        .flat_map(|p| [*p, inst.curve.neg(p)])
        .collect()
}

fn pair_reference(inst: &Instance3, base: &SubspaceBase) -> HashSet<Pt3> {
    let b = signed_base(inst, base);
    let mut pairs = HashSet::new();
    for (i, p) in b.iter().enumerate() {
        for q in &b[i..] {
            pairs.insert(inst.curve.add(p, q));
        }
    }
    pairs
}

fn reference_hit(inst: &Instance3, weighted: &[Pt3], pairs: &HashSet<Pt3>, r: &Pt3) -> bool {
    weighted
        .iter()
        .any(|p| pairs.contains(&inst.curve.sub(r, p)) || pairs.contains(&inst.curve.add(r, p)))
}

fn weighted_slot(inst: &Instance3, base: &SubspaceBase, c: u64) -> Vec<Pt3> {
    base.points.iter().map(|p| inst.curve.mul(p, c)).collect()
}

fn oracle(
    inst: &Instance3,
    base: &SubspaceBase,
    weighted: &[Pt3],
    c: u64,
    r: &Pt3,
    rng: &mut StdRng,
    counts: &mut Counts,
    max_pair_tests: u64,
) -> Option<Terms> {
    let curve = &inst.curve;
    let ops = curve.ops();
    let muls = curve.field.muls();
    let mut result = None;
    'slots: for (k, p) in weighted.iter().enumerate() {
        for sign in [1i64, -1] {
            if counts.pair_tests >= max_pair_tests {
                counts.budget_exhausted = true;
                break 'slots;
            }
            let residual = if sign == 1 {
                curve.sub(r, p)
            } else {
                curve.add(r, p)
            };
            if residual.inf {
                result = Some(vec![(k, sign * c as i64)]);
                break 'slots;
            }
            counts.pair_tests += 1;
            let (pairs, m) = weil_s3_pair_test(inst, base, &residual, rng);
            counts.pair_fp_muls_legacy += m;
            if let Some(pair) = pairs.into_iter().next() {
                result = Some(normalize(pair.into_iter().chain([(k, sign * c as i64)])));
                break 'slots;
            }
        }
    }
    counts.oracle_group_ops += curve.ops() - ops;
    counts.oracle_field_muls_legacy += curve.field.muls() - muls;
    result
}

fn verify(inst: &Instance3, base: &SubspaceBase, r: &Pt3, terms: &Terms) -> bool {
    let mut sum = Pt3::INFINITY;
    for &(i, c) in terms {
        sum = inst
            .curve
            .add(&sum, &inst.curve.mul_signed(&base.points[i], c));
    }
    sum == *r
}

fn verify_against_reference(
    inst: &Instance3,
    base: &SubspaceBase,
    weighted: &[Pt3],
    pairs: &HashSet<Pt3>,
    r: &Pt3,
    result: &Option<Terms>,
    counts: &mut Counts,
) -> Result<(), String> {
    counts.reference_checks += 1;
    if result.is_some() != reference_hit(inst, weighted, pairs, r) {
        return Err(format!("reference disagreement on {r:?}"));
    }
    if let Some(terms) = result {
        let start = inst.curve.ops();
        if !verify(inst, base, r, terms) {
            return Err(format!("invalid group relation on {r:?}"));
        }
        counts.verification_group_ops += inst.curve.ops() - start;
        counts.successful_decompositions += 1;
    }
    Ok(())
}

fn stage(
    inst: &Instance3,
    base: &SubspaceBase,
    targets: &[Pt3],
    pairs: &HashSet<Pt3>,
    c: u64,
    seed: u64,
) -> Result<(Value, Counts), String> {
    inst.curve.reset();
    let weighted = weighted_slot(inst, base, c);
    let setup = json!({"group_ops":inst.curve.ops(),"field_muls_legacy":inst.curve.field.muls()});
    inst.curve.reset();
    let mut rng = StdRng::seed_from_u64(seed ^ 0x5745494c);
    let mut counts = Counts::default();
    let mut decisions = Vec::new();
    for target in targets {
        let result = oracle(
            inst,
            base,
            &weighted,
            c,
            target,
            &mut rng,
            &mut counts,
            u64::MAX,
        );
        verify_against_reference(inst, base, &weighted, pairs, target, &result, &mut counts)?;
        decisions.push(result.is_some());
    }
    let verified = counts.successful_decompositions;
    Ok((
        json!({"coefficient":c,"target_count":targets.len(),"counts":counts,
        "setup":setup,"decisions":decisions,
        "pair_tests_per_verified_relation":if verified>0 {Some(counts.pair_tests as f64/verified as f64)} else {None},
        "yield_fraction":verified as f64/targets.len() as f64,
        "end_to_end_s":null,"end_to_end_speedup":null,"ratio_to_rho":null,
        "classification":"stage_diagnostic","wall_time_admitted":false}),
        counts,
    ))
}

fn recover(
    inst: &Instance3,
    base: &SubspaceBase,
    pairs: &HashSet<Pt3>,
    c: u64,
    seed: u64,
    max_residuals: u64,
    max_pairs: u64,
) -> Result<Value, String> {
    // inst.d must be zero: verification of the planted answer is outside.
    assert_eq!(inst.d, 0);
    inst.curve.reset();
    let weighted = weighted_slot(inst, base, c);
    let setup = json!({"group_ops":inst.curve.ops(),"field_muls_legacy":inst.curve.field.muls()});
    inst.curve.reset();
    let mut query_rng = StdRng::seed_from_u64(seed ^ 0x5155455259);
    let mut solver_rng = StdRng::seed_from_u64(seed ^ 0x534f4c5645);
    let mut system = RelationSystem::new(inst.curve.n, base.len() + 1);
    let mut counts = Counts::default();
    let mut generation_ops = 0;
    let mut residuals = 0;
    let mut recovered = None;
    for attempt in 0..max_residuals {
        if counts.pair_tests >= max_pairs {
            break;
        }
        residuals = attempt + 1;
        let a = query_rng.gen_range(0..inst.curve.n);
        let b = query_rng.gen_range(0..inst.curve.n);
        let start = inst.curve.ops();
        let r = inst.curve.add(
            &inst.curve.mul(&inst.curve.g, a),
            &inst.curve.mul(&inst.q, b),
        );
        generation_ops += inst.curve.ops() - start;
        let result = oracle(
            inst,
            base,
            &weighted,
            c,
            &r,
            &mut solver_rng,
            &mut counts,
            max_pairs,
        );
        if counts.budget_exhausted {
            break;
        }
        verify_against_reference(inst, base, &weighted, pairs, &r, &result, &mut counts)?;
        if let Some(terms) = result {
            let mut row = vec![0; base.len() + 1];
            for (i, coef) in terms {
                row[i] = (coef as i128).rem_euclid(inst.curve.n as i128) as u64;
            }
            row[base.len()] = (inst.curve.n - b) % inst.curve.n;
            system.insert(row, a);
            if let Some(answer) = system.solved(base.len()) {
                recovered = Some(answer);
                break;
            }
        }
    }
    let check_start = inst.curve.ops();
    let verified = recovered
        .map(|d| inst.curve.mul(&inst.curve.g, d) == inst.q)
        .unwrap_or(false);
    let check_ops = inst.curve.ops() - check_start;
    Ok(
        json!({"coefficient":c,"residuals":residuals,"rank":system.rank(),
        "unknowns":base.len()+1,"recovered_scalar":recovered,"verified":verified,
        "status":if verified {"verified_recovery"} else {"budget_exhausted"},
        "counts":counts,"setup":setup,"residual_generation_group_ops":generation_ops,
        "linear_algebra_mod_n_multiplications":system.ops(),"recovery_check_group_ops":check_ops,
        "solver_received_secret":false,"end_to_end_s":null,"end_to_end_speedup":null,
        "ratio_to_rho":null,"accounting_complete":false}),
    )
}

fn args_list(args: &[String], name: &str, default: &str) -> String {
    args.windows(2)
        .find(|w| w[0] == name)
        .map(|w| w[1].clone())
        .unwrap_or(default.into())
}

fn analyze(directory: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(directory);
    let records: Vec<Value> = fs::read_to_string(root.join("records.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    let mut report = String::from("---\ntitle: Weighted Weil-restriction decompositions\ndate: 2026-10-19\ngeometry: margin=18mm\nfontsize: 10pt\n---\n\n# Native larger-curve diagnostic\n\nThis run uses the repository's native cubic-field arithmetic and S3 resultant pair solver. It constructs no covering curve and no whole-field or whole-group operation table. Each decision is cross-checked with a separate signed-pair dictionary unavailable to the solver.\n\nThe local reference is the identical hybrid with c=1, not the best complete IC pipeline. Counts below are stage diagnostics, not end-to-end speedups. Complete S, ratios to rho and external fastest-tool comparisons are unestablished. Wall time is not admitted at L0.\n\n![Recorded pair queries per verified relation](counts.png)\n\n| p | construction seed | group order | unknowns | weight | verified / targets | pair tests | pair tests / relation |\n|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    let mut plot_labels = Vec::new();
    let mut plot_values = [Vec::new(), Vec::new(), Vec::new()];
    let mut total_checks = 0;
    let mut completed = 0;
    let mut exhausted = 0;
    let mut point_commands = String::new();
    let mut identities = Vec::new();
    let mut recovery_rows=String::from("\n# Synthetic one-target recoveries\n\n| p | seed | weight | residuals | independent rank | status |\n|---:|---:|---:|---:|---:|:---|\n");
    for record in &records {
        let p = record["p"].as_u64().unwrap();
        let seed = record["seed"].as_u64().unwrap();
        let n = record["curve"]["n"].as_u64().unwrap();
        plot_labels.push(format!("{p} / {seed}"));
        for (plot_i, arm_i) in [0, 2, 3].into_iter().enumerate() {
            let row = &record["stage"][arm_i];
            let counts = &row["counts"];
            let per = row["pair_tests_per_verified_relation"].as_f64();
            plot_values[plot_i].push(per);
            report.push_str(&format!(
                "| {p} | {seed} | {n} | {} | {} | {} / {} | {} | {} |\n",
                record["base_size"],
                row["coefficient"],
                counts["successful_decompositions"],
                row["target_count"],
                counts["pair_tests"],
                per.map(|v| format!("{v:.3}"))
                    .unwrap_or_else(|| "undefined".into())
            ));
        }
        for row in record["stage"].as_array().unwrap() {
            total_checks += row["counts"]["reference_checks"].as_u64().unwrap();
        }
        for row in record["recoveries"].as_array().unwrap() {
            total_checks += row["counts"]["reference_checks"].as_u64().unwrap();
            if row["verified"] == true {
                completed += 1;
            } else {
                exhausted += 1;
            }
            recovery_rows.push_str(&format!(
                "| {p} | {seed} | {} | {} | {} | {} |\n",
                row["coefficient"],
                row["residuals"],
                row["rank"],
                row["status"].as_str().unwrap()
            ));
        }
        let coord = |value: &Value| -> String {
            let a = value.as_array().unwrap();
            format!("({}+{}*z+{}*z^2)", a[0], a[1], a[2])
        };
        let curve = &record["curve"];
        let g = &curve["g"];
        let q = &record["target"];
        point_commands.push_str(&format!("{{p={p};z=ffgen(Mod(1,p)*(x^3-{}),'z);E=ellinit([{},{}]);P=[{},{}];Q=[{},{}];[ellcard(E)=={n},ellmul(E,P,{n})==[0],ellmul(E,P,{})==Q]}}\n",
            curve["field"]["c"],coord(&curve["a"]),coord(&curve["b"]),coord(&g["x"]),coord(&g["y"]),coord(&q["x"]),coord(&q["y"]),record["known_answer"]));
        identities.push(record["curve_id"].clone());
    }
    report.push_str(&recovery_rows);
    report.push_str(&format!("\nCompleted {completed} recoveries; {exhausted} budget-exhausted recoveries. Total independently checked PDP decisions, including A/A controls and recovery attempts: {total_checks}. The independent comparison is against a separate dictionary, not an independent host.\n"));
    report.push_str("\n# Limits and next checks\n\nThe legacy pair API does not emit internal lookup, polynomial addition/division, allocation or RNG counts. Its F_p multiplication counter also uses legacy inverse accounting. These missing counts prevent a complete pricing band, S, or ranking against rho. No strongest-external-solver comparison was run: Magma, msolve and FGb are unavailable here. The reusable base/dictionary and all excluded verification/setup ledgers are explicit in the raw records. No binary-field or ECC2K transfer was tested.\n\nThe next discriminating step is complete native counters plus matched ecbench one-target rho measurements, with independent-host replay and the fastest available external algebraic reference. These obligations remain open; verified recoveries alone do not discharge them.\n\n# Reproduction and provenance\n\nSee config.json, host.txt, SHA256SUMS, records.jsonl, the committed PROTOCOL.md and examples/weil_weighted_decomposition.rs. Seed 936451 is the holdout. Inputs and outputs are public synthetic fixtures. Histories from the earlier Python experiment remain historical and are not runtime baselines. All A/A decisions and deterministic counters were asserted equal.\n");
    fs::write(root.join("REPORT.md"), &report)?;
    fs::write(root.join("RUN_REPORT.md"), &report)?;
    fs::write(root.join("pari_checks.gp"), point_commands)?;
    fs::write(
        root.join("curve-identities.json"),
        serde_json::to_vec_pretty(&identities)?,
    )?;
    fs::write(
        root.join("summary.json"),
        serde_json::to_vec_pretty(&json!({
        "schema":"weil-weighted-summary/v1","curves":records.len(),
        "completed_recoveries":completed,"exhausted_recoveries":exhausted,"reference_checks":total_checks,
        "plot":{"labels":plot_labels,"baseline":plot_values[0],"weight2":plot_values[1],"weight4":plot_values[2]},
        "end_to_end_speedup":null,"ratio_to_rho":null,"wall_time_admitted":false}))?,
    )?;
    println!("cases={} verified_recoveries={completed} exhausted={exhausted} reference_checks={total_checks}",records.len());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let summary_dir = args_list(&args, "--summarize", "");
    if !summary_dir.is_empty() {
        return analyze(&summary_dir);
    }
    if args.iter().any(|a| a == "--help") {
        println!("--primes 31,61,127,241,499,1009 --seeds 20261019,936451 --queries 128 --dlp-max-p 241 --max-residuals 10000 --max-pair-tests 5000000 --out NEW_DIR");
        return Ok(());
    }
    let primes: Vec<u64> = args_list(&args, "--primes", "31,61,127,241,499,1009")
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    let seeds: Vec<u64> = args_list(&args, "--seeds", "20261019,936451")
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    let queries: usize = args_list(&args, "--queries", "128").parse()?;
    let dlp_max: u64 = args_list(&args, "--dlp-max-p", "241").parse()?;
    let max_residuals: u64 = args_list(&args, "--max-residuals", "10000").parse()?;
    let max_pairs: u64 = args_list(&args, "--max-pair-tests", "5000000").parse()?;
    let out = args_list(&args, "--out", "");
    if out.is_empty() || Path::new(&out).exists() {
        return Err("--out must be a new directory".into());
    }
    fs::create_dir_all(&out)?;
    fs::write(
        Path::new(&out).join("config.json"),
        serde_json::to_vec_pretty(&json!({
        "schema":"weil-weighted-native/v1","argv":args,"primes":primes,"seeds":seeds,
        "queries":queries,"dlp_max_p":dlp_max,"coefficients":[1,1,2,4],
        "max_residuals":max_residuals,"max_pair_tests":max_pairs,
        "scope":"stage diagnostics and public synthetic verified recoveries",
        "unpriced":{"legacy_solver_internal_lookups":"not emitted by API",
            "polynomial_additions_and_divisions":"not emitted by API",
            "rng_and_allocations":"not priced"},
        "wall_time_admitted":false,"external_comparison":"not run; no fastest-tool claim"}))?,
    )?;
    let mut records = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(Path::new(&out).join("records.jsonl"))?;
    for p in primes {
        for &seed in &seeds {
            let known = generate_instance3(p, seed);
            let id = curve_id::extension(
                p,
                &[(p - known.curve.field.c) % p, 0, 0],
                &known.curve.a.0,
                &known.curve.b.0,
                &BigUint::from(known.curve.n),
            )
            .ok_or("invalid curve identity")?;
            let inst = Instance3 {
                curve: known.curve.clone(),
                q: known.q,
                d: 0,
            };
            let mut base_rng = StdRng::seed_from_u64(seed ^ 0x42415345);
            inst.curve.reset();
            let base = SubspaceBase::build(&inst, &mut base_rng);
            let base_setup =
                json!({"field_muls_legacy":inst.curve.field.muls(),"group_ops":inst.curve.ops()});
            let pair_ref = pair_reference(&inst, &base);
            let mut target_rng = StdRng::seed_from_u64(seed ^ 0x544152474554);
            let targets: Vec<_> = (0..queries)
                .map(|_| {
                    inst.curve
                        .mul(&inst.curve.g, target_rng.gen_range(0..inst.curve.n))
                })
                .collect();
            let mut rows: Vec<Value> = Vec::new();
            let mut dlp = Vec::new();
            let mut aa = None;
            for (arm, c) in [
                ("baseline", 1),
                ("aa_control", 1),
                ("weight2", 2),
                ("weight4", 4),
            ] {
                let (mut result, counts) = stage(&inst, &base, &targets, &pair_ref, c, seed)?;
                result["arm"] = json!(arm);
                if arm == "baseline" {
                    aa = Some(counts);
                } else if arm == "aa_control" {
                    assert_eq!(aa.as_ref(), Some(&counts));
                    assert_eq!(rows[0]["decisions"], result["decisions"]);
                }
                rows.push(result);
                if p <= dlp_max && arm != "aa_control" {
                    let mut result =
                        recover(&inst, &base, &pair_ref, c, seed, max_residuals, max_pairs)?;
                    if result["verified"] == true {
                        assert_eq!(result["recovered_scalar"].as_u64(), Some(known.d));
                    }
                    result["arm"] = json!(arm);
                    dlp.push(result);
                }
            }
            let record = json!({"schema":"weil-weighted-native-case/v1","p":p,"seed":seed,
            "curve_id":{"slug":id.slug,"icv1":id.icv1,"model_json":id.model_json},
            "curve":inst.curve,"target":inst.q,"known_answer":known.d,
            "base_size":base.len(),"signed_base_size":2*base.len(),"base_setup":base_setup,
            "base_points":base.points,"reference_pair_entries":pair_ref.len(),
            "reference_role":"validation only; unavailable to solver",
            "target_hash":digest(&serde_json::to_vec(&targets)?),"stage":rows,"recoveries":dlp,
            "end_to_end_speedup":null,"ratio_to_rho":null});
            writeln!(records, "{}", serde_json::to_string(&record)?)?;
            records.flush()?;
            println!(
                "p={p} seed={seed} order={} base={} stages=4 recoveries={}",
                inst.curve.n,
                base.len(),
                record["recoveries"].as_array().unwrap().len()
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exhaustive_weighted_decisions_match_independent_dictionary() {
        for seed in [7, 19] {
            let original = generate_instance3(7, seed);
            let inst = Instance3 {
                curve: original.curve,
                q: original.q,
                d: 0,
            };
            let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(85));
            let pairs = pair_reference(&inst, &base);
            for c in [1, 2, 4] {
                let weighted = weighted_slot(&inst, &base, c);
                let mut r = Pt3::INFINITY;
                let mut rng = StdRng::seed_from_u64(900);
                for _ in 0..inst.curve.n {
                    let mut count = Counts::default();
                    let result = oracle(
                        &inst,
                        &base,
                        &weighted,
                        c,
                        &r,
                        &mut rng,
                        &mut count,
                        u64::MAX,
                    );
                    verify_against_reference(
                        &inst, &base, &weighted, &pairs, &r, &result, &mut count,
                    )
                    .unwrap();
                    r = inst.curve.add(&r, &inst.curve.g);
                }
            }
        }
    }
    #[test]
    fn native_recovery_never_receives_the_known_answer() {
        let original = generate_instance3(31, 20261019);
        let inst = Instance3 {
            curve: original.curve,
            q: original.q,
            d: 0,
        };
        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(321));
        let pairs = pair_reference(&inst, &base);
        for c in [1, 2, 4] {
            let result = recover(&inst, &base, &pairs, c, 71, 10000, 1000000).unwrap();
            assert_eq!(result["verified"], true);
            assert_eq!(result["recovered_scalar"].as_u64(), Some(original.d));
        }
    }
}
