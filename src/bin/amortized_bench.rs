//! `amortized_bench`: amortized (precompute-once) genus-2 index calculus on the
//! transferred instance, against plain rho and Bernstein-Lange precomputed rho.
//!
//! ```text
//! amortized_bench [--json out.json] [--svg out.svg] [--targets 128]
//!                 [--primes 211,419,...] [--time-limit-s 300]
//! ```
//!
//! Protocol: `research/chained_prime_transfer_20261111/amortized/PROTOCOL.md`
//! (frozen before measurement). The transfer (cover construction, isogeny
//! search, `phi^*`) is reported in its own columns and never enters a total.
//! Every figure printed here is a `[pcc:unverified]` assertion until a pcc
//! gate signs it.

use std::process::ExitCode;
use std::time::Instant;

use crypto_lib::cryptanalysis::chained_transfer::amortized::{
    bl_best_k, bl_online, bl_precompute, certify_dlog, ic_descent, ic_precompute, BlStop, BlTable,
    IcPre, Relgen, Rng64,
};
use crypto_lib::cryptanalysis::chained_transfer::bielliptic::{Bielliptic, Quotient};
use crypto_lib::cryptanalysis::chained_transfer::chain::{Chain, Elt, Hop};
use crypto_lib::cryptanalysis::chained_transfer::ec::{Curve, Pt, VeluIsogeny};
use crypto_lib::cryptanalysis::chained_transfer::field::is_prime;
use crypto_lib::cryptanalysis::chained_transfer::measure::{jac_instance, rho_on_jac};
use crypto_lib::cryptanalysis::genus2_fp::{Divisor, Genus2Curve};
use serde_json::{json, Value};

// ── curve selection: copied from chained_transfer_bench (same loop, seeds) ──

/// Largest prime `r` with `r² ∤ n` (copy of `chained_transfer_bench`).
fn largest_cyclic_prime(mut n: u64) -> u64 {
    let mut best = 1u64;
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            let mut e = 0;
            while n.is_multiple_of(d) {
                n /= d;
                e += 1;
            }
            if e == 1 {
                best = best.max(d);
            }
        }
        d += 1;
    }
    if n > 1 {
        best = best.max(n);
    }
    best
}

/// A generator of the order-`r` subgroup of `E` and `[x]gen` (copy).
fn ec_instance(e: &Curve, r: u64, x: u64) -> Option<(Pt, Pt)> {
    let order = e.order();
    if !order.is_multiple_of(r) {
        return None;
    }
    let cof = order / r;
    for s in 1..5000u64 {
        let g = e.mul(&e.point_from_seed(s.wrapping_mul(0x9E37_79B9)), cof as u128);
        if g.is_some() && e.mul(&g, r as u128).is_none() {
            return Some((g, e.mul(&g, x as u128)));
        }
    }
    None
}

/// Prime-degree isogeny `E -> E'` from a rational `ell`-torsion point (copy).
fn find_isogeny(e: &Curve, r: u64) -> Option<VeluIsogeny> {
    let order = e.order();
    for ell in [2u64, 3, 5, 7] {
        if !order.is_multiple_of(ell) || r == ell {
            continue;
        }
        let cof = order / ell;
        for s in 1..2000u64 {
            let g = e.mul(
                &e.point_from_seed(s.wrapping_mul(2_654_435_761)),
                cof as u128,
            );
            if g.is_none() {
                continue;
            }
            if let Some(iso) = VeluIsogeny::from_generator(e, &g) {
                return Some(iso);
            }
        }
    }
    None
}

/// The transferred instance for one curve.
struct Inst {
    p: u64,
    coeffs: [u64; 4],
    seed: u64,
    r: u64,
    e1_order: u64,
    e2_order: u64,
    b: Bielliptic,
    jac: Genus2Curve,
    gen_e: Pt,
    g: Divisor,
    /// Wall ms of one-time transfer work in the accepted candidate (cover,
    /// isogeny search and hop, pull-back hop and chain verification).
    transfer_ms: f64,
    /// Candidates rejected before this one (reported, not hidden).
    rejected: u64,
}

/// Replays `chained_transfer_bench`'s per-candidate checks except the rho/IC
/// ones (not needed here): same coefficient formula, seeds and conditions.
fn try_candidate(p: u64, c: [u64; 4], seed: u64) -> Option<Inst> {
    let t0 = Instant::now();
    let b = Bielliptic::new(p, c).ok()?;
    let e1 = *b.curve(Quotient::E1);
    let e2 = *b.curve(Quotient::E2);
    let jac = b.jac.clone();
    let e1_order = e1.order();
    let e2_order = e2.order();
    let jac_order = e1_order * e2_order;
    let r = largest_cyclic_prime(e1_order);
    if r < 50 || e2_order.is_multiple_of(r) {
        return None;
    }
    let x = (seed.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
    let (gen, target) = ec_instance(&e1, r, x)?;
    // isogeny hop (the original requires it to verify)
    let iso = find_isogeny(&e1, r)?;
    let hop = Hop::isogeny(iso, &gen, &target, r).ok()?;
    if !Chain::new(vec![hop]).verify(&Elt::E(gen), r, x as u128) {
        return None;
    }
    // pull-back hop
    let hop = Hop::pullback(&b, Quotient::E1, &gen, r).ok()?;
    if !Chain::new(vec![hop]).verify(&Elt::E(gen), r, x as u128) {
        return None;
    }
    jac_instance(&jac, jac_order, r, x, seed)?;
    let transfer_ms = t0.elapsed().as_secs_f64() * 1e3;
    // the real pulled-back generator
    if !b.fiber_generic(Quotient::E1, &gen) {
        return None;
    }
    let g = b.pull(Quotient::E1, &gen);
    if g.is_identity() || !jac.mul(&g, r).is_identity() {
        return None;
    }
    Some(Inst {
        p,
        coeffs: c,
        seed,
        r,
        e1_order,
        e2_order,
        b,
        jac,
        gen_e: gen,
        g,
        transfer_ms,
        rejected: 0,
    })
}

fn select_curve(i: usize, p: u64) -> Option<Inst> {
    for (rejected, s) in (0..4000u64).enumerate() {
        let c = [
            1 + (s.wrapping_mul(2_654_435_761) >> 3) % (p - 1),
            (s.wrapping_mul(40_503) >> 5) % p,
            (s.wrapping_mul(2_246_822_519) >> 7) % p,
            1 + (s.wrapping_mul(3_266_489_917) >> 9) % (p - 1),
        ];
        if let Some(mut inst) = try_candidate(p, c, 0xC0FFEE + i as u64 * 101 + s) {
            inst.rejected = rejected as u64;
            return Some(inst);
        }
    }
    None
}

// ── statistics ──────────────────────────────────────────────────────────────

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len().max(1) as f64
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n == 0 {
        f64::NAN
    } else if n % 2 == 1 {
        s[n / 2]
    } else {
        (s[n / 2 - 1] + s[n / 2]) / 2.0
    }
}

fn dist(v: &[f64]) -> Value {
    let mn = v.iter().cloned().fold(f64::INFINITY, f64::min);
    let mx = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    json!({"n": v.len(), "mean": mean(v), "median": median(v), "min": mn, "max": mx})
}

/// 95% percentile bootstrap interval of the mean of `d` (2000 resamples).
fn boot_ci(d: &[f64], seed: u64) -> (f64, f64) {
    let mut rng = Rng64::new(seed);
    let n = d.len();
    let mut means: Vec<f64> = (0..2000)
        .map(|_| (0..n).map(|_| d[rng.below(n as u64) as usize]).sum::<f64>() / n as f64)
        .collect();
    means.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (means[49], means[1949])
}

// ── per-arm measurement ─────────────────────────────────────────────────────

/// One arm's per-target results.
struct ArmRun {
    name: String,
    /// One-time Cantor ops.
    ot_group_ops: u64,
    /// One-time LA modular ops (total) and multiplications.
    ot_la_total: u64,
    ot_la_mulmods: u64,
    ot_wall_ms: f64,
    /// Online ops per target including the charged certificate (rho: its own).
    online: Vec<f64>,
    /// Online ops per target without any certificate charge.
    online_nocert: Vec<f64>,
    ok: bool,
    extra: Value,
}

fn run_ic(
    inst: &Inst,
    name: &str,
    relgen: Relgen,
    q: f64,
    targets: &[(u64, Divisor)],
    seed: u64,
) -> (ArmRun, IcPre) {
    let t0 = Instant::now();
    let cof = inst.e1_order * inst.e2_order / inst.r;
    let pre = ic_precompute(
        &inst.jac,
        &inst.g,
        inst.r,
        cof,
        relgen,
        q,
        200 * inst.p,
        seed,
    );
    let ot_wall = t0.elapsed().as_secs_f64() * 1e3;
    let mut online = vec![];
    let mut nocert = vec![];
    let mut trials = vec![];
    let mut ok = pre.inconsistent == 0 && pre.reached_target;
    let t1 = Instant::now();
    for (i, (x, t)) in targets.iter().enumerate() {
        let d = ic_descent(
            &inst.jac,
            &pre,
            t,
            inst.r,
            seed ^ (0xD35C + i as u64),
            1_000_000,
        );
        let (good, cops) = match d.x {
            Some(k) => {
                let (h, c) = certify_dlog(&inst.jac, &inst.g, t, k);
                (h && k == *x, c)
            }
            None => (false, 0),
        };
        ok &= good;
        online.push((d.group_ops + cops) as f64);
        nocert.push(d.group_ops as f64);
        trials.push(d.trials as f64);
    }
    let on_wall = t1.elapsed().as_secs_f64() * 1e3;
    let extra = json!({
        "relgen": format!("{relgen:?}"), "q_target": q,
        "fb_size": pre.fb.len(), "determined": pre.determined,
        "trials": pre.trials, "relations": pre.relations,
        "inconsistent": pre.inconsistent, "reached_target": pre.reached_target,
        "descent_trials_mean": mean(&trials),
        "online_wall_ms_total": on_wall,
    });
    (
        ArmRun {
            name: name.into(),
            ot_group_ops: pre.group_ops,
            ot_la_total: pre.la.total,
            ot_la_mulmods: pre.la.mulmods,
            ot_wall_ms: ot_wall,
            online,
            online_nocert: nocert,
            ok,
            extra,
        },
        pre,
    )
}

fn run_bl(
    inst: &Inst,
    name: &str,
    stop: BlStop,
    targets: &[(u64, Divisor)],
    tune: &[Divisor],
    seed: u64,
) -> (ArmRun, BlTable) {
    let max_ops = 400 * inst.r + 100_000;
    let (k, scores) = bl_best_k(&inst.jac, &inst.g, inst.r, stop, tune, seed, max_ops);
    let t0 = Instant::now();
    let tab: BlTable = bl_precompute(&inst.jac, &inst.g, inst.r, k, stop, seed ^ 0xB1);
    let ot_wall = t0.elapsed().as_secs_f64() * 1e3;
    let mut online = vec![];
    let mut nocert = vec![];
    let mut ok = true;
    for (i, (x, t)) in targets.iter().enumerate() {
        let o = bl_online(
            &inst.jac,
            &tab,
            t,
            inst.r,
            seed ^ (0xB10 + i as u64),
            max_ops,
        );
        let (good, cops) = match o.x {
            Some(kk) => {
                let (h, c) = certify_dlog(&inst.jac, &inst.g, t, kk);
                (h && kk == *x, c)
            }
            None => (false, 0),
        };
        ok &= good;
        online.push((o.group_ops + cops) as f64);
        nocert.push(o.group_ops as f64);
    }
    let extra = json!({
        "stop": format!("{stop:?}"), "k": k, "entries": tab.entries,
        "saturated": tab.saturated, "walks": tab.walks, "steps": tab.steps,
        "k_scores_mean_online_ops": scores,
    });
    (
        ArmRun {
            name: name.into(),
            ot_group_ops: tab.group_ops,
            ot_la_total: 0,
            ot_la_mulmods: 0,
            ot_wall_ms: ot_wall,
            online,
            online_nocert: nocert,
            ok,
            extra,
        },
        tab,
    )
}

fn run_rho(inst: &Inst, targets: &[(u64, Divisor)], seed: u64) -> ArmRun {
    let mut online = vec![];
    let mut ok = true;
    let t0 = Instant::now();
    for (i, (x, t)) in targets.iter().enumerate() {
        let res = rho_on_jac(&inst.jac, inst.g, *t, inst.r, seed ^ (0x4040 + i as u64));
        let good = match res.recovered {
            Some(k) => certify_dlog(&inst.jac, &inst.g, t, k).0 && k == *x,
            None => false,
        };
        ok &= good;
        online.push((res.group_ops.adds + res.group_ops.doubles) as f64);
    }
    let wall = t0.elapsed().as_secs_f64() * 1e3;
    ArmRun {
        name: "plain_rho".into(),
        ot_group_ops: 0,
        ot_la_total: 0,
        ot_la_mulmods: 0,
        ot_wall_ms: 0.0,
        online_nocert: online.clone(),
        online,
        ok,
        extra: json!({"online_wall_ms_total": wall}),
    }
}

// ── comparison under the frozen rule ────────────────────────────────────────

/// `IC` against `other` at one price: the paired difference of online costs and
/// the break-even in targets.
fn compare(ic: &ArmRun, other: &ArmRun, price: f64, seed: u64) -> Value {
    let ot_ic = ic.ot_group_ops as f64 + price * ic.ot_la_total as f64;
    let ot_x = other.ot_group_ops as f64 + price * other.ot_la_total as f64;
    let on_ic = mean(&ic.online);
    let on_x = mean(&other.online);
    let diff: Vec<f64> = other
        .online
        .iter()
        .zip(&ic.online)
        .map(|(a, b)| a - b)
        .collect();
    let (lo, hi) = boot_ci(&diff, seed);
    let (regime, n0): (&str, Option<f64>) = if on_ic < on_x {
        if ot_ic <= ot_x {
            ("ic_always_better", Some(0.0))
        } else {
            (
                "ic_better_for_N_above",
                Some((ot_ic - ot_x) / (on_x - on_ic)),
            )
        }
    } else if ot_ic < ot_x {
        (
            "ic_better_only_for_N_below",
            Some((ot_x - ot_ic) / (on_ic - on_x)),
        )
    } else {
        ("ic_never_better", None)
    };
    // smallest integer N at which IC's total is strictly lower
    let nstar = match (regime, n0) {
        ("ic_always_better", _) => Some(1.0),
        ("ic_better_for_N_above", Some(n)) => Some(n.floor() + 1.0),
        _ => None,
    };
    let beats = on_ic < on_x && lo > 0.0 && nstar.is_some();
    let verdict = if beats {
        "beats"
    } else if on_ic >= on_x || nstar.is_none() {
        "loses"
    } else {
        "mixed"
    };
    let total = |n: f64| (ot_ic + n * on_ic, ot_x + n * on_x);
    json!({
        "other": other.name, "price_gae_per_la_op": price,
        "ic_online_mean": on_ic, "other_online_mean": on_x,
        "ic_over_other_online_ratio": on_ic / on_x,
        "paired_diff_other_minus_ic_mean": mean(&diff),
        "paired_diff_ci95": [lo, hi],
        "ic_one_time_priced": ot_ic, "other_one_time_priced": ot_x,
        "regime": regime, "crossing_N": n0, "nstar": nstar, "verdict": verdict,
        "total_at_N": {
            "64": {"ic": total(64.0).0, "other": total(64.0).1},
            "1000": {"ic": total(1000.0).0, "other": total(1000.0).1},
            "1000000": {"ic": total(1e6).0, "other": total(1e6).1},
        },
    })
}

fn arm_json(a: &ArmRun) -> Value {
    json!({
        "name": a.name,
        "one_time": {
            "group_ops": a.ot_group_ops, "la_ops_total": a.ot_la_total,
            "la_mulmods": a.ot_la_mulmods, "wall_ms": a.ot_wall_ms,
            "priced_0": a.ot_group_ops as f64,
            "priced_0_1": a.ot_group_ops as f64 + 0.1 * a.ot_la_total as f64,
            "priced_1": a.ot_group_ops as f64 + a.ot_la_total as f64,
        },
        "online_with_cert": dist(&a.online),
        "online_no_cert": dist(&a.online_nocert),
        "online_per_target": a.online,
        "all_certified_and_equal_planted": a.ok,
        "detail": a.extra,
    })
}

fn measure_row(i: usize, inst: &Inst, ntargets: usize) -> (Value, f64, bool, bool) {
    let r = inst.r;
    let mut rng = Rng64::new(0xA77A ^ (inst.p << 8) ^ i as u64);
    // targets: phi^*([x_t] G_E), the real transported instance
    let e1 = *inst.b.curve(Quotient::E1);
    let mut targets: Vec<(u64, Divisor)> = vec![];
    let (mut dropped, mut pull_ns) = (0u64, 0f64);
    while targets.len() < ntargets {
        let x = 1 + rng.below(r - 1);
        let te = e1.mul(&inst.gen_e, x as u128);
        if !inst.b.fiber_generic(Quotient::E1, &te) {
            dropped += 1;
            continue;
        }
        let t0 = Instant::now();
        let tp = inst.b.pull(Quotient::E1, &te);
        pull_ns += t0.elapsed().as_secs_f64() * 1e9;
        // setup check (uncharged): the transported target is [x]G
        if !certify_dlog(&inst.jac, &inst.g, &tp, x).0 {
            dropped += 1;
            continue;
        }
        targets.push((x, tp));
    }
    // tuning targets (disjoint seed), plain multiples of G
    let tune: Vec<Divisor> = (0..24)
        .map(|_| inst.jac.mul(&inst.g, 1 + rng.below(r - 1)))
        .collect();
    let seed = 0xFACE_0000 ^ inst.p;
    let mut valid = true;

    let rho = run_rho(inst, &targets, seed);
    let (ic_head, pre_head) = run_ic(inst, "ic_comb_q90", Relgen::Comb, 0.90, &targets, seed);
    let (ic_walk, _) = run_ic(inst, "ic_walk_q90", Relgen::Walk, 0.90, &targets, seed ^ 1);
    let (ic_q99, _) = run_ic(inst, "ic_comb_q99", Relgen::Comb, 0.99, &targets, seed ^ 2);
    let ics = [&ic_head, &ic_walk, &ic_q99];
    let fb_n = pre_head.fb.len();
    let (bl_b, _) = run_bl(
        inst,
        "bl_b_memory",
        BlStop::Entries(fb_n),
        &targets,
        &tune,
        seed ^ 3,
    );
    let mut bl_a = vec![];
    for (name, price) in [("bl_a0", 0.0), ("bl_a01", 0.1), ("bl_a1", 1.0)] {
        let budget = (ic_head.ot_group_ops as f64 + price * ic_head.ot_la_total as f64) as u64;
        let (mut arm, _) = run_bl(inst, name, BlStop::Ops(budget), &targets, &tune, seed ^ 4);
        arm.extra["budget_group_ops"] = json!(budget);
        arm.extra["budget_price"] = json!(price);
        bl_a.push(arm);
    }
    // frozen rule: IC* = lowest online mean over the IC variants
    let star = ics
        .iter()
        .min_by(|a, b| mean(&a.online).partial_cmp(&mean(&b.online)).unwrap())
        .unwrap();
    // Amendment 1 (see PROTOCOL.md): BL arms whose budget is matched to the
    // one-time cost of IC* itself (not only to the headline variant).
    let mut bl_s = vec![];
    for (name, price) in [("bl_s0", 0.0), ("bl_s01", 0.1), ("bl_s1", 1.0)] {
        let budget = (star.ot_group_ops as f64 + price * star.ot_la_total as f64) as u64;
        let (mut arm, _) = run_bl(inst, name, BlStop::Ops(budget), &targets, &tune, seed ^ 5);
        arm.extra["budget_group_ops"] = json!(budget);
        arm.extra["budget_price"] = json!(price);
        arm.extra["matched_to"] = json!(star.name);
        bl_s.push(arm);
    }
    for a in ics
        .iter()
        .map(|a| a.ok)
        .chain([rho.ok, bl_b.ok])
        .chain(bl_a.iter().map(|a| a.ok))
        .chain(bl_s.iter().map(|a| a.ok))
    {
        valid &= a;
    }
    let mut comparisons = vec![];
    let mut beats_bl_all = true;
    let mut beats_plain_all = true;
    for (pi, price) in [0.0f64, 0.1, 1.0].iter().enumerate() {
        let cs = 0x5EED + i as u64 * 10 + pi as u64;
        let c_plain = compare(star, &rho, *price, cs);
        let c_b = compare(star, &bl_b, *price, cs + 1);
        let c_a = compare(star, &bl_a[pi], *price, cs + 2);
        let c_s = compare(star, &bl_s[pi], *price, cs + 3);
        beats_plain_all &= c_plain["verdict"] == "beats";
        beats_bl_all &=
            c_b["verdict"] == "beats" && c_a["verdict"] == "beats" && c_s["verdict"] == "beats";
        comparisons.push(c_plain);
        comparisons.push(c_b);
        comparisons.push(c_a);
        comparisons.push(c_s);
        // every IC variant against every BL arm too (distance remaining)
    }
    let mut all_variants = vec![];
    for ic in ics {
        for price in [0.0f64, 0.1, 1.0] {
            all_variants.push(compare(ic, &bl_b, price, 11));
            all_variants.push(compare(ic, &rho, price, 12));
        }
    }
    let row = json!({
        "p": inst.p, "r": r, "e1_order": inst.e1_order, "e2_order": inst.e2_order,
        "coeffs": inst.coeffs, "seed": inst.seed, "rejected_candidates_before": inst.rejected,
        "fb_size": fb_n, "targets": targets.len(), "targets_dropped_setup": dropped,
        "transfer_one_time": {
            "cover_isogeny_hops_wall_ms": inst.transfer_ms,
            "phi_star_wall_us_per_target_mean": pull_ns / 1e3 / targets.len() as f64,
            "note": "reported only; never added to any total",
        },
        "arms": {
            "plain_rho": arm_json(&rho),
            "ic_comb_q90": arm_json(&ic_head),
            "ic_walk_q90": arm_json(&ic_walk),
            "ic_comb_q99": arm_json(&ic_q99),
            "bl_b_memory": arm_json(&bl_b),
            "bl_a0": arm_json(&bl_a[0]),
            "bl_a01": arm_json(&bl_a[1]),
            "bl_a1": arm_json(&bl_a[2]),
            "bl_s0": arm_json(&bl_s[0]),
            "bl_s01": arm_json(&bl_s[1]),
            "bl_s1": arm_json(&bl_s[2]),
        },
        "ic_star": star.name,
        "comparisons_ic_star": comparisons,
        "comparisons_all_ic_variants": all_variants,
        "row_beats_plain_rho_all_prices": beats_plain_all,
        "row_beats_matched_precomputed_rho_all_prices": beats_bl_all,
        "row_valid": valid,
    });
    (row, mean(&star.online), beats_bl_all, valid)
}

fn render_svg(rows: &[Value]) -> String {
    let get = |r: &Value, a: &str| r["arms"][a]["online_with_cert"]["mean"].as_f64().unwrap();
    let series: [(&str, &str, &str, &str); 4] = [
        ("plain_rho", "#2c3e50", "none", "plain rho"),
        ("ic_star", "#c0392b", "none", "amortized IC (best variant)"),
        ("bl_b_memory", "#27ae60", "5,3", "BL rho, memory = fb size"),
        (
            "bl_a0",
            "#8e44ad",
            "2,3",
            "BL rho, precompute = IC one-time",
        ),
    ];
    let val = |r: &Value, name: &str| -> f64 {
        if name == "ic_star" {
            get(r, r["ic_star"].as_str().unwrap())
        } else {
            get(r, name)
        }
    };
    let ps: Vec<f64> = rows.iter().map(|r| r["p"].as_f64().unwrap()).collect();
    let mut ymin = f64::INFINITY;
    let mut ymax = 0f64;
    for r in rows {
        for (n, ..) in series {
            let v = val(r, n).max(1.0);
            ymin = ymin.min(v);
            ymax = ymax.max(v);
        }
    }
    let (x0, x1) = (ps[0].ln(), ps[ps.len() - 1].ln().max(ps[0].ln() + 1.0));
    let (y0, y1) = ((ymin * 0.7).ln(), (ymax * 1.4).ln());
    let xp = |p: f64| 80.0 + (p.ln() - x0) / (x1 - x0) * 520.0;
    let yp = |v: f64| 340.0 - (v.max(1.0).ln() - y0) / (y1 - y0) * 270.0;
    let h = 420 + 14 * rows.len();
    let mut s = String::new();
    s.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"660\" height=\"{h}\" viewBox=\"0 0 660 {h}\" font-family=\"Helvetica,Arial,sans-serif\" font-size=\"11\">\n<rect width=\"660\" height=\"{h}\" fill=\"#fff\"/>\n"));
    s.push_str("<text x=\"330\" y=\"22\" text-anchor=\"middle\" font-size=\"14\" font-weight=\"bold\">Per-target online cost, amortized regime (Cantor ops, with certificate)</text>\n");
    s.push_str("<text x=\"330\" y=\"38\" text-anchor=\"middle\" fill=\"#555\">log-log; mean over targets; values [pcc:unverified]; one-time costs below</text>\n");
    for e in 0..9 {
        let v = 10f64.powi(e);
        if v.ln() > y0 && v.ln() < y1 {
            let y = yp(v);
            s.push_str(&format!("<line x1=\"80\" y1=\"{y:.1}\" x2=\"600\" y2=\"{y:.1}\" stroke=\"#e5e5e5\"/><text x=\"74\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#444\">{v}</text>\n", y + 4.0));
        }
    }
    for p in &ps {
        let x = xp(*p);
        s.push_str(&format!("<line x1=\"{x:.1}\" y1=\"50\" x2=\"{x:.1}\" y2=\"340\" stroke=\"#f0f0f0\"/><text x=\"{x:.1}\" y=\"356\" text-anchor=\"middle\" fill=\"#444\" transform=\"rotate(-35 {x:.1} 356)\">{p}</text>\n"));
    }
    s.push_str("<line x1=\"80\" y1=\"340\" x2=\"600\" y2=\"340\" stroke=\"#999\"/><line x1=\"80\" y1=\"50\" x2=\"80\" y2=\"340\" stroke=\"#999\"/>\n");
    s.push_str("<text x=\"340\" y=\"392\" text-anchor=\"middle\">field size p</text><text x=\"20\" y=\"195\" text-anchor=\"middle\" transform=\"rotate(-90 20 195)\">online group operations per target</text>\n");
    for (li, (name, color, dash, label)) in series.iter().enumerate() {
        let pts: Vec<(f64, f64)> = rows
            .iter()
            .zip(&ps)
            .map(|(r, p)| (xp(*p), yp(val(r, name))))
            .collect();
        let poly: Vec<String> = pts.iter().map(|(x, y)| format!("{x:.1},{y:.1}")).collect();
        s.push_str(&format!("<polyline fill=\"none\" stroke=\"{color}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\" points=\"{}\"/>\n", poly.join(" ")));
        for (x, y) in &pts {
            s.push_str(&format!(
                "<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"3\" fill=\"{color}\"/>"
            ));
        }
        let ly = 62 + 14 * li;
        s.push_str(&format!("\n<line x1=\"92\" y1=\"{ly}\" x2=\"116\" y2=\"{ly}\" stroke=\"{color}\" stroke-width=\"2\" stroke-dasharray=\"{dash}\"/><text x=\"122\" y=\"{}\">{label}</text>\n", ly + 4));
    }
    s.push_str("<text x=\"80\" y=\"412\" font-weight=\"bold\">One-time cost (group ops + LA ops): IC best variant | BL memory-matched</text>\n");
    for (k, r) in rows.iter().enumerate() {
        let star = r["ic_star"].as_str().unwrap();
        let ic = &r["arms"][star]["one_time"];
        let bl = &r["arms"]["bl_b_memory"]["one_time"];
        s.push_str(&format!(
            "<text x=\"80\" y=\"{}\">p={}: IC {} + {} LA | BL {}</text>\n",
            428 + 14 * k,
            r["p"],
            ic["group_ops"],
            ic["la_ops_total"],
            bl["group_ops"]
        ));
    }
    s.push_str("</svg>\n");
    s
}

fn next_prime(mut n: u64) -> u64 {
    while !is_prime(n) {
        n += 1;
    }
    n
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let arg = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let ntargets: usize = arg("--targets").and_then(|s| s.parse().ok()).unwrap_or(128);
    let limit_s: f64 = arg("--time-limit-s")
        .and_then(|s| s.parse().ok())
        .unwrap_or(300.0);
    let primes: Vec<u64> = match arg("--primes") {
        Some(s) => s.split(',').filter_map(|t| t.trim().parse().ok()).collect(),
        None => {
            let mut v = vec![211u64, 419, 821, 1619, 3203, 6421, 12841];
            for _ in 0..4 {
                let n = next_prime(2 * v[v.len() - 1]);
                v.push(n);
            }
            v
        }
    };
    let mut rows: Vec<Value> = vec![];
    let mut any_invalid = false;
    for (i, &p) in primes.iter().enumerate() {
        let t0 = Instant::now();
        let Some(inst) = select_curve(i, p) else {
            eprintln!("no suitable bielliptic curve found at p = {p}");
            continue;
        };
        let (row, star_online, beats, valid) = measure_row(i, &inst, ntargets);
        let wall = t0.elapsed().as_secs_f64();
        any_invalid |= !valid;
        let a = &row["arms"];
        let m = |arm: &str| a[arm]["online_with_cert"]["mean"].as_f64().unwrap();
        let ot = |arm: &str| a[arm]["one_time"]["group_ops"].as_u64().unwrap();
        println!(
            "p={:>7} r={:>7} fb={:>6} | online/target: rho {:>8.1} IC* {:>7.1} ({}) BLb {:>7.1} BLa0 {:>7.1} | one-time ops: IC {} (+{} LA) BLb {} BLa0 {} | beats_BL={} valid={} wall={:.1}s",
            p, inst.r, row["fb_size"], m("plain_rho"), star_online, row["ic_star"].as_str().unwrap(),
            m("bl_b_memory"), m("bl_a0"), ot("ic_comb_q90"), a["ic_comb_q90"]["one_time"]["la_ops_total"],
            ot("bl_b_memory"), ot("bl_a0"), beats, valid, wall
        );
        rows.push(row);
        if wall > limit_s {
            eprintln!("p = {p} took {wall:.0}s > {limit_s}s: stopping the size ladder");
            break;
        }
    }

    let big: Vec<&Value> = rows
        .iter()
        .filter(|r| r["p"].as_u64().unwrap() >= 1619)
        .collect();
    let overall_beats = !big.is_empty()
        && big
            .iter()
            .all(|r| r["row_beats_matched_precomputed_rho_all_prices"] == true);
    let out = json!({
        "schema": "amortized_bench/v1",
        "protocol": "research/chained_prime_transfer_20261111/amortized/PROTOCOL.md",
        "unit": "Cantor group operations (1 GAE each); LA priced at 0, 0.1, 1 GAE per modular operation",
        "targets_per_curve": ntargets,
        "rows": rows,
        "overall_ic_beats_matched_precomputed_rho": overall_beats,
        "any_row_invalid": any_invalid,
        "status": "all values [pcc:unverified]",
    });
    println!(
        "\noverall verdict (frozen rule, rows with p >= 1619): IC amortized beats matched precomputed rho = {overall_beats}; invalid rows = {any_invalid}"
    );
    if let Some(path) = arg("--svg") {
        std::fs::write(&path, render_svg(out["rows"].as_array().unwrap())).expect("write svg");
        eprintln!("wrote {path}");
    }
    if let Some(path) = arg("--json") {
        std::fs::write(&path, serde_json::to_string_pretty(&out).unwrap()).expect("write json");
        eprintln!("wrote {path}");
    }
    ExitCode::SUCCESS
}
