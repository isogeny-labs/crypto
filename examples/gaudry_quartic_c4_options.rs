//! **Plain `S₅` `C₄` with the `D₄` oracle's solver options, paired.**
//!
//! §11.17 of `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md`
//! measured `C₄ = 1.213·10¹²` for the plain `S₅` solve.  #1792 then cut the
//! `D₄`-symmetric oracle's `C₄` about 370× with four options, 26× of it from
//! sparsest pivots alone.  Three of those options (`sparsest`, `chains`,
//! shape replay) do not depend on the `D₄` form; this runner applies them to
//! the plain system on §11.17's instances.
//!
//! ```text
//! cargo run --release --example gaudry_quartic_c4_options -- --p 269 --seed 1 --residuals 4 \
//!     --arms committed,sparsest,sparsest+chains --json out.json
//! cargo run --release --example gaudry_quartic_c4_options -- --generic 4,5,6 \
//!     --arms committed,sparsest,sparsest+chains --json generic.json
//! ```
//!
//! Instances: the curve, base and residuals are §11.17's (`p = 269`, seed 1),
//! because the residual stream advances with the `committed` arm's solve, as
//! it did there.  Every arm of a residual starts from a clone of the same
//! generator state; the arm order rotates by one per residual.  Every solve
//! is checked against the meet-in-the-middle oracle.  `--constructed n`
//! makes the first `n` residuals planted four-point sums (default 1, as
//! §11.17), so replayed solves are checked on nonempty decompositions.  An arm ending in
//! `+replay` records its first solve's shape and replays it afterwards.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::{
    base_quadruples, factor_base, mitm_decompositions, solve_s5_subspace_with, solve_system4_with,
    Curve4, PlainOptions, Pt4, QuarticSolve, ShapeTrace, SymmetrisedS5, E4,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

#[derive(Clone, Debug)]
struct Arm {
    name: String,
    opts: PlainOptions,
    replay: bool,
}

fn parse_arm(s: &str) -> Arm {
    let mut opts = PlainOptions::default();
    let mut replay = false;
    for part in s.split('+') {
        match part {
            "committed" => {}
            "sparsest" => opts.sparsest = true,
            "chains" => opts.chains = true,
            "replay" => replay = true,
            other => panic!("unknown arm option {other}"),
        }
    }
    Arm {
        name: s.to_string(),
        opts,
        replay,
    }
}

#[derive(Serialize)]
struct Row {
    arm: String,
    options: PlainOptions,
    replay_arm: bool,
    /// Position of this arm in the residual's run order (0 = first).
    order: usize,
    solve: QuarticSolve,
    solver_quadruples: Vec<[u64; 4]>,
    agree: bool,
    wall_s: f64,
}

#[derive(Serialize)]
struct ResidualRows {
    index: usize,
    constructed: bool,
    planted: Option<[u64; 4]>,
    oracle_quadruples: Vec<[u64; 4]>,
    arms: Vec<Row>,
}

#[derive(Serialize)]
struct Report {
    p: u64,
    seed: u64,
    base: usize,
    fp_muls_per_add: f64,
    precompute_muls: u64,
    arms: Vec<String>,
    residuals: Vec<ResidualRows>,
}

#[derive(Serialize)]
struct GenericRow {
    equation_degree: u8,
    arm: String,
    order: usize,
    solve: QuarticSolve,
    planted_found: bool,
    wall_s: f64,
}

fn generic_system(d: u8, seed: u64, p: u64) -> (Vec<HashMap<[u8; 4], u64>>, [u64; 4], StdRng) {
    // The same planted system as `gaudry_quartic_c4 --generic`.
    let mut rng = StdRng::seed_from_u64(seed);
    let root: [u64; 4] = std::array::from_fn(|_| rng.gen_range(0..p));
    let mut monos = Vec::new();
    for a in 0..=d {
        for b in 0..=(d - a) {
            for c in 0..=(d - a - b) {
                for e in 0..=(d - a - b - c) {
                    monos.push([a, b, c, e]);
                }
            }
        }
    }
    let eval = |c: &HashMap<[u8; 4], u64>| -> u64 {
        c.iter().fold(0u64, |acc, (m, &k)| {
            let mut t = k;
            for i in 0..4 {
                for _ in 0..m[i] {
                    t = t * root[i] % p;
                }
            }
            (acc + t) % p
        })
    };
    let comps = (0..4)
        .map(|_| {
            let mut c: HashMap<[u8; 4], u64> =
                monos.iter().map(|m| (*m, rng.gen_range(0..p))).collect();
            let v = eval(&c);
            let e = c.entry([0, 0, 0, 0]).or_insert(0);
            *e = (*e + p - v) % p;
            c
        })
        .collect();
    (comps, root, rng)
}

// ── The matched rho reference (`--rho N`) ─────────────────────────────────

#[derive(Serialize)]
struct RhoRun {
    seed: u64,
    steps: u64,
    /// Group operations of the walk, its 32 precomputed steps and the
    /// look-ahead retries included; the final check excluded.
    group_ops: u64,
    /// `F_p` multiplications of the same, counted as `C₄` is.
    fp_muls: u64,
    fruitless_lookaheads: u64,
    restarts: u64,
    s: f64,
    verified: bool,
}

#[derive(Serialize)]
struct RhoReport {
    p: u64,
    seed: u64,
    group_order: u64,
    subgroup_order: u64,
    cofactor: u64,
    setup_fp_muls: u64,
    runs: Vec<RhoRun>,
    mean_fp_muls: f64,
    mean_steps: f64,
    mean_s: f64,
    all_verified: bool,
}

/// The order of `pt`, from a multiple `m` of it.
fn exact_order(curve: &Curve4, pt: &Pt4, mut m: u64) -> u64 {
    let mut q = 2u64;
    let mut rest = m;
    while q * q <= rest {
        while rest % q == 0 {
            rest /= q;
            while m % q == 0 && curve.mul(pt, m / q).inf {
                m /= q;
            }
        }
        q += 1;
    }
    if rest > 1 && m % rest == 0 && curve.mul(pt, m / rest).inf {
        m /= rest;
    }
    m
}

/// A multiple of `pt`'s order in the Hasse interval `q + 1 ± 2p²`, by
/// baby-step giant-step.
fn hasse_multiple(curve: &Curve4, pt: &Pt4, p: u64) -> u64 {
    let q = p.pow(4);
    let lo = q + 1 - 2 * p * p;
    let width = 4 * p * p;
    let steps = (width as f64).sqrt() as u64 + 1;
    let mut table: HashMap<(E4, E4), u64> = HashMap::new();
    let mut jp = Pt4::INF;
    for j in 0..steps {
        if !jp.inf {
            table.entry((jp.x, jp.y)).or_insert(j);
        }
        jp = curve.add(&jp, pt);
    }
    let giant = curve.mul(pt, steps);
    let mut t = curve.mul(pt, lo);
    let mut i = 0u64;
    loop {
        if t.inf {
            return lo + i * steps;
        }
        let neg = curve.neg(&t);
        if let Some(&j) = table.get(&(neg.x, neg.y)) {
            return lo + i * steps + j;
        }
        t = curve.add(&t, &giant);
        i += 1;
        assert!(
            i * steps <= width + 2 * steps,
            "no multiple in the Hasse interval"
        );
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

fn invmod(a: u64, n: u64) -> u64 {
    let (mut t, mut nt, mut r, mut nr) = (0i128, 1i128, n as i128, a as i128);
    while nr != 0 {
        let qq = r / nr;
        (t, nt) = (nt, t - qq * nt);
        (r, nr) = (nr, r - qq * nr);
    }
    (t.rem_euclid(n as i128)) as u64
}

/// `±R` represented by the point whose `y` is the smaller of `y`, `−y`;
/// `true` when that is `−R`.
fn canon(curve: &Curve4, pt: &Pt4) -> (Pt4, bool) {
    if pt.inf {
        return (*pt, false);
    }
    let ny = curve.f.neg(&pt.y);
    if ny < pt.y {
        (
            Pt4 {
                x: pt.x,
                y: ny,
                inf: false,
            },
            true,
        )
    } else {
        (*pt, false)
    }
}

fn class_index(pt: &Pt4) -> usize {
    let h = pt.x.0[0]
        ^ pt.x.0[1].wrapping_mul(0x9E37_79B9)
        ^ pt.x.0[2].rotate_left(17)
        ^ pt.x.0[3].rotate_left(41);
    let mut z = h.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    ((z ^ (z >> 31)) % 32) as usize
}

/// Pollard rho on `±`-classes with an r-adding walk (r = 32), exhaustive
/// storage, and look-ahead past the fruitless 2-cycles the negation map
/// creates; every group operation and `F_p` multiplication counted.
fn rho_negation(curve: &Curve4, g: &Pt4, q: &Pt4, n: u64, d: u64, seed: u64) -> RhoRun {
    let mut rng = StdRng::seed_from_u64(seed ^ 0x5E6A_7104);
    let f = &curve.f;
    curve.reset_ops();
    f.reset_muls();
    let mults: Vec<(u64, u64, Pt4)> = (0..32)
        .map(|_| {
            let al = rng.gen_range(0..n);
            let be = rng.gen_range(0..n);
            (al, be, curve.add(&curve.mul(g, al), &curve.mul(q, be)))
        })
        .collect();
    let mut table: HashMap<Pt4, (u64, u64)> = HashMap::new();
    let (mut steps, mut lookaheads, mut restarts) = (0u64, 0u64, 0u64);
    let mut found = None;
    'outer: loop {
        let mut a = rng.gen_range(0..n);
        let mut b = rng.gen_range(1..n);
        let start = curve.add(&curve.mul(g, a), &curve.mul(q, b));
        let (mut r, neg) = canon(curve, &start);
        if neg {
            a = (n - a) % n;
            b = (n - b) % n;
        }
        loop {
            steps += 1;
            if let Some(&(a2, b2)) = table.get(&r) {
                let db = (b + n - b2) % n;
                if db != 0 {
                    found = Some(mulmod((a2 + n - a) % n, invmod(db, n), n));
                    break 'outer;
                }
                restarts += 1;
                break;
            }
            table.insert(r, (a, b));
            // Look-ahead: a step that lands, negated, on a class choosing the
            // same multiplier would return to `r` (a fruitless 2-cycle).
            let mut j = class_index(&r);
            let (next, flip) = loop {
                let s = curve.add(&r, &mults[j].2);
                let (c, flip) = canon(curve, &s);
                if !(flip && class_index(&c) == j) {
                    break (c, flip);
                }
                lookaheads += 1;
                j = (j + 1) % 32;
            };
            a = (a + mults[j].0) % n;
            b = (b + mults[j].1) % n;
            if flip {
                a = (n - a) % n;
                b = (n - b) % n;
            }
            r = next;
            if steps > 64 * (n as f64).sqrt() as u64 + 1_000_000 {
                break 'outer;
            }
        }
    }
    let ops = curve.ops();
    let muls = f.muls();
    let verified = found == Some(d) && curve.mul(g, d) == *q;
    RhoRun {
        seed,
        steps,
        group_ops: ops,
        fp_muls: muls,
        fruitless_lookaheads: lookaheads,
        restarts,
        s: ops as f64 / (n as f64).sqrt(),
        verified,
    }
}

/// The rho reference on the measured curve itself: `#E` from point orders
/// in the Hasse interval, its largest prime factor `ℓ`, and `runs` one-target
/// negation-rho solves in the subgroup of order `ℓ`.
fn rho_reference(curve: &Curve4, p: u64, seed: u64, runs: u64, json: Option<&str>) {
    let mut rng = StdRng::seed_from_u64(seed ^ 0x0DE5_0F11);
    let f = &curve.f;
    f.reset_muls();
    let q = p.pow(4);
    let (lo, hi) = (q + 1 - 2 * p * p, q + 1 + 2 * p * p);
    let mut l = 1u64;
    let mut pts = Vec::new();
    // Orders of random points until their lcm has one multiple in the interval.
    let order = loop {
        let pt = loop {
            let x = f.random(&mut rng);
            if let Some(pt) = curve.lift_x(&x, &mut rng) {
                break pt;
            }
        };
        let o = exact_order(curve, &pt, hasse_multiple(curve, &pt, p));
        l = l / gcd(l, o) * o;
        pts.push(pt);
        let first = lo.div_ceil(l) * l;
        if first <= hi && first + l > hi {
            break first;
        }
        assert!(pts.len() < 64, "group order not determined");
    };
    let mut ell = 1u64;
    let mut rest = order;
    let mut k = 2u64;
    while k * k <= rest {
        while rest % k == 0 {
            ell = ell.max(k);
            rest /= k;
        }
        k += 1;
    }
    ell = ell.max(rest);
    let cof = order / ell;
    let g = loop {
        let x = f.random(&mut rng);
        if let Some(pt) = curve.lift_x(&x, &mut rng) {
            let g = curve.mul(&pt, cof);
            if !g.inf {
                assert!(curve.mul(&g, ell).inf);
                break g;
            }
        }
    };
    let setup = f.muls();
    println!(
        "p={p} seed={seed}: #E = {order} = {cof} · {ell} (ℓ prime, {:.2} bits), setup {setup} F_p muls",
        (ell as f64).log2()
    );
    let mut out = Vec::new();
    for s in 0..runs {
        let d = rng.gen_range(1..ell);
        let qpt = curve.mul(&g, d);
        let run = rho_negation(curve, &g, &qpt, ell, d, seed * 1000 + s);
        println!(
            "rho run {s}: steps {} ops {} F_p muls {} S {:.3} lookaheads {} restarts {} verified {}",
            run.steps, run.group_ops, run.fp_muls, run.s, run.fruitless_lookaheads, run.restarts, run.verified
        );
        out.push(run);
    }
    let m = out.len() as f64;
    let rep = RhoReport {
        p,
        seed,
        group_order: order,
        subgroup_order: ell,
        cofactor: cof,
        setup_fp_muls: setup,
        mean_fp_muls: out.iter().map(|r| r.fp_muls as f64).sum::<f64>() / m,
        mean_steps: out.iter().map(|r| r.steps as f64).sum::<f64>() / m,
        mean_s: out.iter().map(|r| r.s).sum::<f64>() / m,
        all_verified: out.iter().all(|r| r.verified),
        runs: out,
    };
    println!(
        "mean F_p muls {:.4e}, mean S {:.3}, all verified {}",
        rep.mean_fp_muls, rep.mean_s, rep.all_verified
    );
    if let Some(path) = json {
        fs::write(path, serde_json::to_string_pretty(&rep).unwrap()).expect("write");
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut p = 269u64;
    let mut seed = 1u64;
    let mut residuals = 4usize;
    let mut constructed_n = 1usize;
    let mut json: Option<String> = None;
    let mut generic_degrees: Option<Vec<u8>> = None;
    let mut rho_runs: Option<u64> = None;
    let mut arms: Vec<Arm> = vec![parse_arm("committed"), parse_arm("sparsest")];
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--p" => {
                i += 1;
                p = args[i].parse().expect("--p");
            }
            "--seed" => {
                i += 1;
                seed = args[i].parse().expect("--seed");
            }
            "--residuals" => {
                i += 1;
                residuals = args[i].parse().expect("--residuals");
            }
            "--constructed" => {
                i += 1;
                constructed_n = args[i].parse().expect("--constructed");
            }
            "--json" => {
                i += 1;
                json = Some(args[i].clone());
            }
            "--arms" => {
                i += 1;
                arms = args[i].split(',').map(parse_arm).collect();
            }
            "--rho" => {
                i += 1;
                rho_runs = Some(args[i].parse().expect("--rho"));
            }
            "--generic" => {
                i += 1;
                generic_degrees = Some(
                    args[i]
                        .split(',')
                        .map(|v| v.parse().expect("--generic"))
                        .collect(),
                );
            }
            other => panic!("unknown argument {other}"),
        }
        i += 1;
    }
    // The residual stream advances with the committed arm's solve, as in
    // §11.17; without that arm, with the first arm's (other instances).
    let committed = arms.iter().position(|a| a.name == "committed").unwrap_or(0);

    if let Some(ds) = generic_degrees {
        let mut out = Vec::new();
        for (di, &d) in ds.iter().enumerate() {
            for k in 0..arms.len() {
                let arm = &arms[(di + k) % arms.len()];
                let (comps, root, mut rng) = generic_system(d, seed, 269);
                let mut st = QuarticSolve::default();
                let t = Instant::now();
                let sols = solve_system4_with(
                    &comps,
                    arm.opts,
                    d,
                    4 * (d - 1) + 1,
                    269,
                    &mut rng,
                    &mut st,
                    false,
                    None,
                    None,
                );
                let wall_s = t.elapsed().as_secs_f64();
                let planted_found = sols.as_ref().is_some_and(|s| s.contains(&root));
                println!(
                    "generic d={d} {:<24} C = {:.4e} (echelon {:.3e}, nf {:.3e}, charpoly {:.3e}, eigen {:.3e}) dim {} planted {planted_found} | {wall_s:.1} s",
                    arm.name, st.total_muls as f64, st.echelon_muls as f64, st.nf_muls as f64,
                    st.charpoly_muls as f64, st.eigenvector_muls as f64, st.dim
                );
                out.push(GenericRow {
                    equation_degree: d,
                    arm: arm.name.clone(),
                    order: k,
                    solve: st,
                    planted_found,
                    wall_s,
                });
                if let Some(path) = &json {
                    fs::write(path, serde_json::to_string_pretty(&out).unwrap()).expect("write");
                }
            }
        }
        return;
    }

    let mut rng = StdRng::seed_from_u64(seed);
    let curve = Curve4::random(p, &mut rng);
    if let Some(runs) = rho_runs {
        rho_reference(&curve, p, seed, runs, json.as_deref());
        return;
    }
    let f = &curve.f;
    let base = factor_base(&curve, &mut rng);
    let fp_per_add = {
        let q = base[1];
        let mut acc = base[0];
        f.reset_muls();
        for _ in 0..64 {
            acc = curve.add(&acc, &q);
        }
        f.muls() as f64 / 64.0
    };
    f.reset_muls();
    let pre = SymmetrisedS5::precompute(&curve, &mut rng);
    println!(
        "p={p} seed={seed}: base {}, {fp_per_add:.1} F_p muls per addition, precompute {:.3e} muls; arms {:?}",
        base.len(),
        pre.precompute_muls as f64,
        arms.iter().map(|a| a.name.as_str()).collect::<Vec<_>>()
    );
    let mut traces: Vec<Option<ShapeTrace>> = vec![None; arms.len()];
    let mut report = Report {
        p,
        seed,
        base: base.len(),
        fp_muls_per_add: fp_per_add,
        precompute_muls: pre.precompute_muls,
        arms: arms.iter().map(|a| a.name.clone()).collect(),
        residuals: Vec::new(),
    };
    for idx in 0..residuals {
        // Residual generation exactly as `gaudry_quartic_c4`.
        let constructed = idx < constructed_n;
        let (r, planted) = if constructed {
            let mut ids: Vec<usize> = Vec::new();
            while ids.len() < 4 {
                let k = rng.gen_range(0..base.len());
                if !ids.contains(&k) {
                    ids.push(k);
                }
            }
            let r = ids
                .iter()
                .fold(Pt4::INF, |acc, &k| curve.add(&acc, &base[k]));
            let mut xs: [u64; 4] = std::array::from_fn(|k| base[ids[k]].x.0[0]);
            xs.sort_unstable();
            (r, Some(xs))
        } else {
            let r = loop {
                let x = f.random(&mut rng);
                if let Some(pt) = curve.lift_x(&x, &mut rng) {
                    break pt;
                }
            };
            (r, None)
        };
        let oracle_q: Vec<[u64; 4]> = mitm_decompositions(&curve, &base, &r).into_iter().collect();
        let start = rng.clone();
        let mut after_committed: Option<StdRng> = None;
        let mut rows: Vec<Row> = Vec::new();
        for k in 0..arms.len() {
            let ai = (idx + k) % arms.len();
            let arm = &arms[ai];
            let mut arm_rng = start.clone();
            let mut st = QuarticSolve::default();
            f.reset_muls();
            let t = Instant::now();
            let sols = if arm.replay {
                match &traces[ai] {
                    Some(tr) => solve_s5_subspace_with(
                        &curve,
                        &pre,
                        &r.x,
                        arm.opts,
                        Some(tr),
                        None,
                        &mut arm_rng,
                        &mut st,
                        false,
                    ),
                    None => {
                        let mut tr = ShapeTrace::default();
                        let s = solve_s5_subspace_with(
                            &curve,
                            &pre,
                            &r.x,
                            arm.opts,
                            None,
                            Some(&mut tr),
                            &mut arm_rng,
                            &mut st,
                            false,
                        );
                        if s.is_some() {
                            traces[ai] = Some(tr);
                        }
                        s
                    }
                }
            } else {
                solve_s5_subspace_with(
                    &curve,
                    &pre,
                    &r.x,
                    arm.opts,
                    None,
                    None,
                    &mut arm_rng,
                    &mut st,
                    false,
                )
            };
            let wall_s = t.elapsed().as_secs_f64();
            if ai == committed {
                after_committed = Some(arm_rng);
            }
            let solver_q: Vec<[u64; 4]> = sols
                .as_ref()
                .map(|s| base_quadruples(s, &base).into_iter().collect())
                .unwrap_or_default();
            let agree = sols.is_some() && solver_q == oracle_q;
            println!(
                "residual {idx} [{k}] {:<24} C4 = {:.4e} (echelon {:.3e}, nf {:.3e}, charpoly {:.3e}, eigen {:.3e}) dim {} replay {:?} | solver {:?} oracle {:?} agree {agree} | {wall_s:.0} s",
                arm.name, st.total_muls as f64, st.echelon_muls as f64, st.nf_muls as f64,
                st.charpoly_muls as f64, st.eigenvector_muls as f64, st.dim, st.replay,
                solver_q, oracle_q
            );
            rows.push(Row {
                arm: arm.name.clone(),
                options: arm.opts,
                replay_arm: arm.replay,
                order: k,
                solve: st,
                solver_quadruples: solver_q,
                agree,
                wall_s,
            });
            report.residuals.retain(|r| r.index != idx);
            report.residuals.push(ResidualRows {
                index: idx,
                constructed,
                planted,
                oracle_quadruples: oracle_q.clone(),
                arms: rows
                    .iter()
                    .map(|r| Row {
                        arm: r.arm.clone(),
                        options: r.options,
                        replay_arm: r.replay_arm,
                        order: r.order,
                        solve: r.solve.clone(),
                        solver_quadruples: r.solver_quadruples.clone(),
                        agree: r.agree,
                        wall_s: r.wall_s,
                    })
                    .collect(),
            });
            if let Some(path) = &json {
                fs::write(path, serde_json::to_string_pretty(&report).unwrap()).expect("write");
            }
        }
        rng = after_committed.expect("committed arm ran");
    }
}
