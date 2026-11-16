//! B0: the point-decomposition SAT baseline, re-measured.
//!
//! Family: `y² + xy = x³ + x² + 1` over `F_{2^n}`, `m = 3` summands, symmetrised
//! Semaev `S′₄`, subspace factor base `⟨1, z, …, z^{ℓ−1}⟩`. For every target the
//! exact pairs-and-solve oracle supplies the verdict and the enumeration count
//! `T_enum`; each SAT arm is then read as `R = conflicts / T_enum`.
//!
//! Arms: `oracle` (always), `native` (CDCL + Gauss–Jordan XOR, `sat.rs`),
//! `native-aa` (same solver, scrambled saved phases: the A/A arm), `cms`
//! (CryptoMiniSat on the exported XOR-DIMACS) and `cadical` (CaDiCaL on the
//! exported CNF). The external arms need `--ext-bin` (tools/b0-ext-solvers) and
//! `--export DIR`. One JSON line per target goes to stdout (and `--out`).
//!
//! Example: `b0_pdp_sat_baseline --l 6 --uniform 16 --planted 4 --seed 20261115
//! --budget 4000000 --arms oracle,native,native-aa,cms,cadical --export /tmp/b0
//! --ext-bin tools/b0-ext-solvers/target/release/b0-ext-solvers`
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::{
    chain_s3_max_variables, ChainedS3Encoding,
};
use crypto_lib::cryptanalysis::binary_semaev_s4::{elementary_symmetric_3, symmetrised_s4_eval};
use crypto_lib::cryptanalysis::ic_corpus::plant_nondegenerate;
use crypto_lib::cryptanalysis::koblitz_index_calculus::{find_irreducible_sparse, points_with_x};
use crypto_lib::cryptanalysis::sat::{to_dimacs, to_dimacs_xor, SolveResult};
use crypto_lib::cryptanalysis::semaev_decomp::{Gf2, SubspaceOracle};
use crypto_lib::cryptanalysis::semaev_sat::{encode_semaev_s4_with, S4Options, XorEncoding};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::io::Write;

#[path = "support/pdp_lin_cdcl.rs"]
mod pdp_lin_cdcl;
use pdp_lin_cdcl::{ChainInstance, Result as LinResult};
use std::process::Command;
use std::time::Instant;

struct Cfg {
    l: u32,
    n: u32,
    uniform: usize,
    planted: usize,
    seed: u64,
    budget: u64,
    arms: Vec<String>,
    out: Option<String>,
    export: Option<String>,
    ext_bin: Option<String>,
}

fn parse_args() -> Cfg {
    let v: Vec<String> = std::env::args().collect();
    let mut c = Cfg {
        l: 6,
        n: 0,
        uniform: 16,
        planted: 4,
        seed: 20261115,
        budget: 4_000_000,
        arms: vec!["oracle".into(), "native".into(), "native-aa".into()],
        out: None,
        export: None,
        ext_bin: None,
    };
    let mut i = 1;
    while i < v.len() {
        let val = || {
            v.get(i + 1)
                .unwrap_or_else(|| panic!("{} needs a value", v[i]))
                .clone()
        };
        match v[i].as_str() {
            "--l" => c.l = val().parse().expect("l"),
            "--n" => c.n = val().parse().expect("n"),
            "--uniform" => c.uniform = val().parse().expect("uniform"),
            "--planted" => c.planted = val().parse().expect("planted"),
            "--seed" => c.seed = val().parse().expect("seed"),
            "--budget" => c.budget = val().parse().expect("budget"),
            "--arms" => c.arms = val().split(',').map(|s| s.trim().to_string()).collect(),
            "--out" => c.out = Some(val()),
            "--export" => c.export = Some(val()),
            "--ext-bin" => c.ext_bin = Some(val()),
            other => {
                eprintln!("unknown argument {other}");
                std::process::exit(2);
            }
        }
        i += 2;
    }
    if c.n == 0 {
        c.n = 3 * c.l - 1;
    }
    assert!(c.l < c.n && c.n <= 62, "need l < n <= 62");
    if c.arms.iter().any(|a| a == "cms" || a == "cadical") {
        assert!(
            c.ext_bin.is_some() && c.export.is_some(),
            "cms/cadical arms need --ext-bin and --export"
        );
    }
    c
}

fn verify(xs: &[F2mElement; 3], x_r: &F2mElement, irr: &IrreduciblePoly) -> bool {
    let (e1, e2, e3) = elementary_symmetric_3(&xs[0], &xs[1], &xs[2], irr);
    symmetrised_s4_eval(&e1, &e2, &e3, x_r, irr).is_zero()
}

/// Model bits of the `3ℓ` x-variables (`x_{i,j}` is variable `1 + i·ℓ + j`) to field elements.
fn xs_from_bits(bits: &[bool], l: u32, gf: &Gf2) -> [F2mElement; 3] {
    let word = |i: usize| -> u64 {
        (0..l as usize).fold(0u64, |acc, j| {
            acc | ((bits[i * l as usize + j] as u64) << j)
        })
    };
    [
        gf.to_element(word(0)),
        gf.to_element(word(1)),
        gf.to_element(word(2)),
    ]
}

fn run_native(
    n: u32,
    l: u32,
    irr: &IrreduciblePoly,
    x_r: &F2mElement,
    gf: &Gf2,
    budget: u64,
    aa_seed: Option<u64>,
) -> Value {
    let one = F2mElement::one(n);
    let t_enc = Instant::now();
    let mut enc = encode_semaev_s4_with(
        n,
        l,
        irr,
        &one,
        x_r,
        S4Options {
            encoding: XorEncoding::Native,
            break_symmetry: true,
        },
    );
    let encode_ms = t_enc.elapsed().as_secs_f64() * 1e3;
    if let Some(s) = aa_seed {
        enc.solver.scramble_saved_phases(s);
    }
    enc.solver.conflict_budget = budget;
    let t = Instant::now();
    let r = if enc.trivially_unsat {
        SolveResult::Unsat
    } else {
        enc.solver.solve()
    };
    let wall_ms = t.elapsed().as_secs_f64() * 1e3;
    let (status, verified) = match r {
        SolveResult::Sat => {
            let xs = enc.decode();
            let ok = verify(&xs, x_r, irr);
            // also check the decoded abscissae lie in the subspace (bits above ℓ are zero)
            let in_base = xs.iter().all(|x| gf.from_element(x) < (1u64 << l));
            ("sat", Some(ok && in_base))
        }
        SolveResult::Unsat => ("unsat", None),
        SolveResult::Unknown => ("unknown", None),
    };
    let st = &enc.solver.stats;
    json!({
        "status": status,
        "verified": verified,
        "conflicts": st.conflicts,
        "decisions": st.decisions,
        "propagations": st.propagations,
        "xor_propagations": st.xor_propagations,
        "xor_conflicts": st.xor_conflicts,
        "restarts": st.restarts,
        "word_ops": st.word_ops,
        "n_vars": enc.solver.n_vars(),
        "n_clauses": enc.solver.n_original_clauses(),
        "n_xors": enc.solver.n_xors(),
        "encode_ms": encode_ms,
        "wall_ms": wall_ms,
    })
}

/// Chained-S₃ arm: `S₃(x₁,x₂,t) = 0`, `S₃(t,x₃,x_R) = 0` with a full-width intermediate `t`,
/// summands confined to the subspace by domain clauses, `x₁ ≤ x₂ ≤ x₃` lex symmetry breaking,
/// both identity patterns solved (a refutation needs both UNSAT). `priority_first_pair` puts
/// only the `x₁, x₂` bits ahead of VSIDS, so `t` and `x₃` are reached by propagation.
fn run_chain(
    n: u32,
    l: u32,
    irr: &IrreduciblePoly,
    x_r: &F2mElement,
    gf: &Gf2,
    budget: u64,
    priority_first_pair: bool,
) -> Value {
    let one = F2mElement::one(n);
    let codes: Vec<BigUint> = (0..(1u64 << l)).map(BigUint::from).collect();
    let maxv = (chain_s3_max_variables(n, 3).expect("chain size") * 2 + 10_000) as u32;
    let (
        mut conflicts,
        mut decisions,
        mut propagations,
        mut xor_props,
        mut learnt,
        mut learnt_lits,
        mut wall_ms,
        mut n_vars,
    ) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0f64, 0u64);
    let mut status = "unsat";
    let mut verified: Option<bool> = None;
    let mut patterns_run = 0;
    for pattern in 0..2u32 {
        let mut enc = match ChainedS3Encoding::build(3, irr, &one, Some(x_r), pattern, maxv) {
            Ok(e) => e,
            Err(e) => return json!({"status": "error", "error": format!("{e:?}")}),
        };
        if let Err(e) = enc.constrain_summands(&codes, 50_000_000) {
            return json!({"status": "error", "error": format!("{e:?}")});
        }
        if let Err(e) = enc.order_all_summands(maxv) {
            return json!({"status": "error", "error": format!("{e:?}")});
        }
        if priority_first_pair {
            let v: Vec<u32> = (1..=2 * n).collect();
            enc.solver.set_branch_priority(&v);
        }
        enc.solver.conflict_budget = budget.saturating_sub(conflicts);
        let t = Instant::now();
        let r = enc.solver.solve();
        wall_ms += t.elapsed().as_secs_f64() * 1e3;
        patterns_run += 1;
        let st = &enc.solver.stats;
        conflicts += st.conflicts;
        decisions += st.decisions;
        propagations += st.propagations;
        xor_props += st.xor_propagations;
        learnt += st.learnt_clauses;
        learnt_lits += st.learnt_lits_kept;
        n_vars = n_vars.max(enc.solver.n_vars() as u64);
        match r {
            SolveResult::Sat => {
                let xs = enc.decode_summands();
                let arr = [xs[0].clone(), xs[1].clone(), xs[2].clone()];
                let in_base = arr.iter().all(|x| gf.from_element(x) < (1u64 << l));
                verified = Some(enc.verify_model() && verify(&arr, x_r, irr) && in_base);
                status = "sat";
                break;
            }
            SolveResult::Unknown => {
                status = "unknown";
                break;
            }
            SolveResult::Unsat => {}
        }
    }
    json!({
        "status": status, "verified": verified, "patterns_run": patterns_run,
        "conflicts": conflicts, "decisions": decisions, "propagations": propagations,
        "xor_propagations": xor_props, "learnt_clauses": learnt, "learnt_lits_kept": learnt_lits,
        "n_vars": n_vars, "wall_ms": wall_ms,
    })
}

/// New solver: CDCL + XOR engine with field-product gates (`pdp_lin_cdcl.rs`) on the chained-S₃
/// system; `lex` adds `x₁ ≤ x₂` so the pair count matches the oracle's `X₁ ≤ X₂` scan.
fn run_lin(
    n: u32,
    l: u32,
    irr: &IrreduciblePoly,
    x_r: &F2mElement,
    gf: &Gf2,
    budget: u64,
    lex: bool,
) -> Value {
    let irr_mask = irr
        .low_terms
        .iter()
        .fold(1u64 << n, |m, &t| m | (1u64 << t));
    let xr = gf.from_element(x_r);
    let t_build = Instant::now();
    let mut inst = ChainInstance::build(n, l, irr_mask, xr, 1, lex);
    let build_ms = t_build.elapsed().as_secs_f64() * 1e3;
    inst.solver.conflict_budget = budget;
    let t = Instant::now();
    let r = inst.solver.solve();
    let wall_ms = t.elapsed().as_secs_f64() * 1e3;
    let (status, verified, model) = match r {
        LinResult::Sat => {
            let xs = [
                inst.solver.model_block(&inst.x1),
                inst.solver.model_block(&inst.x2),
                inst.solver.model_block(&inst.x3),
            ];
            let arr = [
                gf.to_element(xs[0]),
                gf.to_element(xs[1]),
                gf.to_element(xs[2]),
            ];
            let in_base = xs.iter().all(|&x| x < (1u64 << l));
            ("sat", Some(verify(&arr, x_r, irr) && in_base), Some(xs))
        }
        LinResult::Unsat => ("unsat", None, None),
        LinResult::Unknown => ("unknown", None, None),
    };
    let st = &inst.solver.stats;
    json!({
        "status": status, "verified": verified, "model_xs": model,
        "conflicts": st.conflicts, "decisions": st.decisions, "propagations": st.propagations,
        "xor_propagations": st.xor_propagations, "gje_calls": st.gje_calls,
        "gate_linearisations": st.gate_linearisations, "learnt_clauses": st.learnt_clauses,
        "conflicts_t_stage": st.conflicts_stage_open, "conflicts_x3_stage": st.conflicts_stage_assigned,
        "ns_gje": st.ns_gje, "ns_clause_prop": st.ns_clause_prop, "ns_analyze": st.ns_analyze,
        "learnt_lits": st.learnt_lits, "learnt_priority_width": st.learnt_priority_width,
        "n_vars": inst.solver.n_vars, "build_ms": build_ms, "wall_ms": wall_ms,
    })
}

/// REUSE arm: one parametrised solver per rung; each target enters as assumptions on the `x_R`
/// block, so learnt clauses persist across targets. Reports per-target deltas of the counters.
fn run_lin_reuse(
    inst: &mut ChainInstance,
    l: u32,
    irr: &IrreduciblePoly,
    x_r: &F2mElement,
    gf: &Gf2,
    budget: u64,
    forget: bool,
) -> Value {
    let before = inst.solver.stats.clone();
    inst.set_target(gf.from_element(x_r), forget);
    inst.solver.conflict_budget = budget;
    let t = Instant::now();
    let r = inst.solver.solve();
    let wall_ms = t.elapsed().as_secs_f64() * 1e3;
    let (status, verified) = match r {
        LinResult::Sat => {
            let xs = [
                inst.solver.model_block(&inst.x1),
                inst.solver.model_block(&inst.x2),
                inst.solver.model_block(&inst.x3),
            ];
            let arr = [
                gf.to_element(xs[0]),
                gf.to_element(xs[1]),
                gf.to_element(xs[2]),
            ];
            let in_base = xs.iter().all(|&x| x < (1u64 << l));
            ("sat", Some(verify(&arr, x_r, irr) && in_base))
        }
        LinResult::Unsat => ("unsat", None),
        LinResult::Unknown => ("unknown", None),
    };
    let st = &inst.solver.stats;
    json!({
        "status": status, "verified": verified,
        "conflicts": st.conflicts - before.conflicts, "decisions": st.decisions - before.decisions,
        "propagations": st.propagations - before.propagations, "xor_propagations": st.xor_propagations - before.xor_propagations,
        "learnt_clauses": st.learnt_clauses - before.learnt_clauses, "cumulative_conflicts": st.conflicts,
        "conflicts_t_stage": st.conflicts_stage_open - before.conflicts_stage_open,
        "conflicts_x3_stage": st.conflicts_stage_assigned - before.conflicts_stage_assigned,
        "forgotten_clauses": st.forgotten_clauses - before.forgotten_clauses, "live_learnts": inst.solver.live_learnts(),
        "cumulative_learnt_clauses": st.learnt_clauses, "n_vars": inst.solver.n_vars, "wall_ms": wall_ms,
    })
}

fn export_instance(
    n: u32,
    l: u32,
    irr: &IrreduciblePoly,
    x_r: &F2mElement,
    dir: &str,
    idx: usize,
) -> (String, String) {
    let one = F2mElement::one(n);
    let enc_x = encode_semaev_s4_with(
        n,
        l,
        irr,
        &one,
        x_r,
        S4Options {
            encoding: XorEncoding::Native,
            break_symmetry: true,
        },
    );
    let enc_c = encode_semaev_s4_with(
        n,
        l,
        irr,
        &one,
        x_r,
        S4Options {
            encoding: XorEncoding::Cnf,
            break_symmetry: true,
        },
    );
    let px = format!("{dir}/t{idx:03}.xnf");
    let pc = format!("{dir}/t{idx:03}.cnf");
    std::fs::write(&px, to_dimacs_xor(&enc_x.solver)).expect("write xnf");
    std::fs::write(&pc, to_dimacs(&enc_c.solver)).expect("write cnf");
    (px, pc)
}

fn run_ext(
    bin: &str,
    solver: &str,
    file: &str,
    budget: u64,
    l: u32,
    x_r: &F2mElement,
    irr: &IrreduciblePoly,
    gf: &Gf2,
) -> Value {
    let out = Command::new(bin)
        .args([
            "--solver",
            solver,
            "--file",
            file,
            "--budget",
            &budget.to_string(),
            "--model-vars",
            &(3 * l).to_string(),
        ])
        .output()
        .expect("run external solver");
    if !out.status.success() {
        return json!({"status": "error", "stderr": String::from_utf8_lossy(&out.stderr)});
    }
    let mut v: Value = serde_json::from_slice(&out.stdout).expect("ext json");
    if v["status"] == "sat" {
        let bits: Vec<bool> = v["model"]
            .as_str()
            .unwrap_or("")
            .chars()
            .map(|c| c == '1')
            .collect();
        let verified = if bits.len() == 3 * l as usize {
            verify(&xs_from_bits(&bits, l, gf), x_r, irr)
        } else {
            false
        };
        v["verified"] = json!(verified);
    } else {
        v["verified"] = Value::Null;
    }
    v
}

fn median(xs: &mut [f64]) -> f64 {
    if xs.is_empty() {
        return f64::NAN;
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = xs.len() / 2;
    if xs.len() % 2 == 1 {
        xs[m]
    } else {
        0.5 * (xs[m - 1] + xs[m])
    }
}

fn main() {
    let cfg = parse_args();
    let (n, l) = (cfg.n, cfg.l);
    let irr = find_irreducible_sparse(n).expect("irreducible");
    let gf = Gf2::new(&irr);
    let one = F2mElement::one(n);
    let curve = BinaryCurve {
        m: n,
        irreducible: irr.clone(),
        a: one.clone(),
        b: one.clone(),
        generator: BinaryPoint::Infinity,
        order: BigUint::from(1u32),
        cofactor: BigUint::from(1u32),
    };
    let base: Vec<BinaryPoint> = (1..(1u64 << l))
        .flat_map(|x| points_with_x(&curve, &gf.to_element(x)))
        .collect();
    let oracle = SubspaceOracle::low_order(l, 1, &gf);
    let pow = 1u64 << l;
    let t_enum_oracle = pow * (pow + 1) / 2; // pairs X1 <= X2 the oracle scans on a refutation
    let t_enum_pairs = pow * (pow - 1) / 2; // C(2^l, 2)
    let t_enum_triples = pow * (pow - 1) * (pow - 2) / 6; // C(2^l, 3): the survey's "one conflict per triple" yardstick
    let mask = (1u64 << n) - 1;
    let mut rng = StdRng::seed_from_u64(cfg.seed ^ ((n as u64) << 32) ^ ((l as u64) << 16));
    if let Some(d) = &cfg.export {
        std::fs::create_dir_all(d).expect("export dir");
    }
    let mut out_file = cfg
        .out
        .as_ref()
        .map(|p| std::fs::File::create(p).expect("out file"));
    let wants = |a: &str| cfg.arms.iter().any(|x| x == a);

    // targets: uniform x-coordinates of curve points outside the base, then planted sums
    let mut targets: Vec<(String, F2mElement, Option<[u64; 3]>)> = Vec::new();
    while targets.len() < cfg.uniform {
        let x = rng.gen::<u64>() & mask;
        if x < pow {
            continue;
        }
        let xe = gf.to_element(x);
        if points_with_x(&curve, &xe).is_empty() {
            continue;
        }
        targets.push(("uniform".into(), xe, None));
    }
    for _ in 0..cfg.planted {
        let p = plant_nondegenerate(&curve, &base, l, &mut rng).expect("plant");
        targets.push(("planted".into(), p.x_r, Some(p.xs)));
    }

    let irr_mask = irr
        .low_terms
        .iter()
        .fold(1u64 << n, |m, &t| m | (1u64 << t));
    let mut reuse_inst = if wants("lin-reuse") {
        Some(ChainInstance::build_param(n, l, irr_mask, 1, true))
    } else {
        None
    };
    let mut reuse_forget_inst = if wants("lin-reuse-forget") {
        Some(ChainInstance::build_param(n, l, irr_mask, 1, true))
    } else {
        None
    };
    let mut r_by_arm: std::collections::BTreeMap<String, Vec<f64>> = Default::default();
    let mut r3_by_arm: std::collections::BTreeMap<String, Vec<f64>> = Default::default();
    let mut disagreements = 0usize;
    let mut unverified = 0usize;
    for (idx, (kind, x_r, plant)) in targets.iter().enumerate() {
        let t0 = Instant::now();
        let (wit, pairs) = oracle.decompose(gf.from_element(x_r), &gf);
        let oracle_ms = t0.elapsed().as_secs_f64() * 1e3;
        let oracle_sat = wit.is_some();
        if let Some(w) = wit {
            let xs = [
                gf.to_element(w[0]),
                gf.to_element(w[1]),
                gf.to_element(w[2]),
            ];
            assert!(
                verify(&xs, x_r, &irr),
                "oracle witness fails S'4 at target {idx}"
            );
        }
        let mut rec = json!({
            "schema": "b0.pdp.sat.target/v1",
            "idx": idx, "kind": kind, "n": n, "l": l, "seed": cfg.seed, "budget": cfg.budget,
            "irr_low_terms": irr.low_terms, "x_r": gf.from_element(x_r), "planted_xs": plant,
            "t_enum_oracle": t_enum_oracle, "t_enum_pairs": t_enum_pairs, "t_enum_triples": t_enum_triples,
            "oracle": {"sat": oracle_sat, "pairs": pairs, "witness": wit, "wall_ms": oracle_ms},
        });
        let files = cfg
            .export
            .as_ref()
            .map(|d| export_instance(n, l, &irr, x_r, d, idx));
        let mut arms = serde_json::Map::new();
        if wants("native") {
            arms.insert(
                "native".into(),
                run_native(n, l, &irr, x_r, &gf, cfg.budget, None),
            );
        }
        if wants("chain") {
            arms.insert(
                "chain".into(),
                run_chain(n, l, &irr, x_r, &gf, cfg.budget, false),
            );
        }
        if wants("chain-p12") {
            arms.insert(
                "chain-p12".into(),
                run_chain(n, l, &irr, x_r, &gf, cfg.budget, true),
            );
        }
        if wants("lin") {
            arms.insert(
                "lin".into(),
                run_lin(n, l, &irr, x_r, &gf, cfg.budget, true),
            );
        }
        if let Some(ri) = reuse_inst.as_mut() {
            arms.insert(
                "lin-reuse".into(),
                run_lin_reuse(ri, l, &irr, x_r, &gf, cfg.budget, false),
            );
        }
        if let Some(ri) = reuse_forget_inst.as_mut() {
            arms.insert(
                "lin-reuse-forget".into(),
                run_lin_reuse(ri, l, &irr, x_r, &gf, cfg.budget, true),
            );
        }
        if wants("lin-nolex") {
            arms.insert(
                "lin-nolex".into(),
                run_lin(n, l, &irr, x_r, &gf, cfg.budget, false),
            );
        }
        if wants("native-aa") {
            arms.insert(
                "native-aa".into(),
                run_native(
                    n,
                    l,
                    &irr,
                    x_r,
                    &gf,
                    cfg.budget,
                    Some(cfg.seed ^ 0xA5A5_5A5A ^ idx as u64),
                ),
            );
        }
        if let (Some(bin), Some((px, pc))) = (&cfg.ext_bin, &files) {
            if wants("cms") {
                arms.insert(
                    "cms".into(),
                    run_ext(bin, "cms", px, cfg.budget, l, x_r, &irr, &gf),
                );
            }
            if wants("cadical") {
                arms.insert(
                    "cadical".into(),
                    run_ext(bin, "cadical", pc, cfg.budget, l, x_r, &irr, &gf),
                );
            }
        }
        for (name, a) in arms.iter_mut() {
            let status = a["status"].as_str().unwrap_or("error").to_string();
            let agree = match status.as_str() {
                "sat" => Some(oracle_sat && a["verified"] == json!(true)),
                "unsat" => Some(!oracle_sat),
                _ => None,
            };
            if agree == Some(false) {
                disagreements += 1;
            }
            if status == "sat" && a["verified"] != json!(true) {
                unverified += 1;
            }
            a["agree"] = json!(agree);
            if let Some(c) = a["conflicts"].as_f64() {
                a["r_oracle"] = json!(c / t_enum_oracle as f64);
                a["r_pairs"] = json!(c / t_enum_pairs as f64);
                a["r_triples"] = json!(c / t_enum_triples as f64);
                if kind == "uniform" && status != "unknown" {
                    r_by_arm
                        .entry(name.clone())
                        .or_default()
                        .push(c / t_enum_oracle as f64);
                    r3_by_arm
                        .entry(name.clone())
                        .or_default()
                        .push(c / t_enum_triples as f64);
                }
            }
        }
        rec["arms"] = Value::Object(arms);
        if let Some(f) = &files {
            rec["files"] = json!({"xnf": f.0, "cnf": f.1});
        }
        let line = rec.to_string();
        println!("{line}");
        if let Some(f) = out_file.as_mut() {
            writeln!(f, "{line}").expect("write out");
        }
    }
    eprintln!("# n={n} l={l} targets={} (uniform {}, planted {}) T_enum(oracle)={t_enum_oracle} C(2^l,2)={t_enum_pairs} C(2^l,3)={t_enum_triples}", targets.len(), cfg.uniform, cfg.planted);
    for (arm, rs) in r_by_arm.iter_mut() {
        let k = rs.len();
        let r3 = median(r3_by_arm.get_mut(arm).unwrap());
        eprintln!("# {arm:10} decided uniform targets {k:3}  median R = conflicts/T_enum = {:.4}   median R3 = conflicts/C(2^l,3) = {:.4}", median(rs), r3);
    }
    eprintln!("# verdict disagreements with the oracle: {disagreements}; unverified SAT models: {unverified}");
}
