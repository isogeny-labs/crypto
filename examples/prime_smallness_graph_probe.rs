//! Three probes on two questions about prime-field ECDLP: can index calculus
//! be recast as a graph problem, and is there a usable notion of "small" in
//! `F_p` other than a subfield?  Protocol:
//! `research/prime_smallness_graph_20261023/PROTOCOL.md`.
//!
//! **Arm G: large-prime graph IC-2.**  Direct-subtraction 2-decomposition
//! (the `prime_fast::collect_relations` sweep) with the single-large-prime
//! variation: a candidate `T = R ∓ F_i` whose `x(T)` lies in the large-prime
//! set `L = {x ≤ X_L}` becomes an edge between the target and the vertex
//! `x(T)`.  Every vertex reached twice closes a cycle; the cycles are
//! eliminated by `large_prime_filter`, the complete rows solved mod `n`, and
//! the logarithm checked against the planted scalar.  Every group operation
//! is counted, every native operation the group-operation count does not
//! cover is emitted as an unpriced counter, and a matched rho (same binary,
//! same curve, same targets, negation map) is the reference.
//!
//! **Arm L: lattice smallness.**  A factor base of points with
//! `x ≡ a/b (mod p)`, `|a| ≤ X`, `1 ≤ b ≤ X`, attacked with the bihomogeneous
//! Coppersmith lattice for `H(a₁,b₁,a₂,b₂) = b₁²b₂²·S₃(a₁/b₁, a₂/b₂, x_R)`,
//! beside the interval lattice of `examples/prime_ecdlp_breakthrough_probe.rs`
//! probe B (`x ≤ B`) as positive control, through one lattice routine, at
//! equal factor-base size.
//!
//! **Arm F: fake subfields under F4.**  The owner's F4 (`f4_fp::f4`) on
//! `⟨S₃(x₁,x₂,x_R), L(x₁), L(x₂)⟩` for three factor-base sets of the same
//! size `B` (interval, multiplicative subgroup `μ_B`, small fractions), with
//! a planted decomposition; counted `F_p` multiplications against the
//! `7B` of the direct-subtraction sweep, and the solution count over `F̄_p`
//! (program note §4.2's falsifier).
//!
//! ```bash
//! cargo run --release --example prime_smallness_graph_probe -- \
//!     --arm g|l|f|s [--bits 20,24,28,32] [--seeds 3] [--out DIR] [--quick]
//! ```
//!
//! Public synthetic known-answer instances only.  Every row carries its
//! seed; every logarithm, relation and root is checked, and a row whose
//! check failed is recorded as such, never dropped.

use std::collections::HashMap;
use std::io::Write as _;
use std::time::{Duration, Instant};

use crypto_lib::cryptanalysis::curve_id;
use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering as MonoOrder, Poly};
use crypto_lib::cryptanalysis::large_prime_filter::{
    filter_large_primes, replay_complete_relation, CompleteRelation, LargePrimeFilterOptions,
    PartialRelation,
};
use crypto_lib::cryptanalysis::lattice::lll_reduce;
use crypto_lib::cryptanalysis::prime_fast::{
    find_a3_curve, point_order_hasse, rho_parallel, FastCurve, FastFactorBase, Fp64, Pt,
    RhoConfig,
};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, ToPrimitive, Zero};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

// ── shared helpers ─────────────────────────────────────────────────────

fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

fn addmod(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 + b as u128) % n as u128) as u64
}

fn submod(a: u64, b: u64, n: u64) -> u64 {
    addmod(a, n - b % n, n)
}

fn powmod(mut a: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1 % n;
    a %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, n);
        }
        a = mulmod(a, a, n);
        e >>= 1;
    }
    r
}

fn invmod(a: u64, n: u64) -> u64 {
    // n prime
    powmod(a, n - 2, n)
}

fn signed_mod(s: i64, n: u64) -> u64 {
    if s >= 0 {
        (s as u64) % n
    } else {
        (n - ((-s) as u64) % n) % n
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Registry record for a prime-order curve this probe measures (the curve
/// registry harvests `p, a, b, order` records from tracked JSON).
fn curve_record(curve: &FastCurve, g: Pt, call: &str) -> Value {
    let f = curve.f;
    let (p, a, b, n) = (f.p, f.from_mont(curve.a), f.from_mont(curve.b), curve.n);
    let id = curve_id::prime(
        &BigUint::from(p),
        &BigUint::from(a),
        &BigUint::from(b),
        &BigUint::from(n),
    );
    let (gx, gy) = curve.canonical(g);
    json!({
        "name": curve.name,
        "p": p.to_string(), "a": a.to_string(), "b": b.to_string(),
        "order": n.to_string(), "subgroup_order": n.to_string(), "cofactor": 1,
        "generator": {"x": gx.to_string(), "y": gy.to_string()},
        "generator_call": call,
        "icv1": id.as_ref().map(|i| i.icv1.clone()),
        "slug": id.as_ref().map(|i| i.slug.clone()),
    })
}

fn slug_of(curve: &FastCurve) -> String {
    let f = curve.f;
    curve_id::prime(
        &BigUint::from(f.p),
        &BigUint::from(f.from_mont(curve.a)),
        &BigUint::from(f.from_mont(curve.b)),
        &BigUint::from(curve.n),
    )
    .map(|i| i.slug)
    .unwrap_or_else(|| "unidentified".into())
}

/// `a = −3` prime-order curve over the largest prime `p < 2^bits` with
/// `p ≡ 1 (mod 2^k)`, `b` the smallest positive value giving prime order
/// (arm F needs `μ_B ⊂ F_p^*` for `B | 2^k`).
fn find_a3_curve_2adic(bits: u32, k: u32) -> Option<(FastCurve, Pt)> {
    let m = 1u64 << k;
    let mut p = ((1u64 << bits) - 1) / m * m + 1;
    while p >= (1u64 << bits) || !is_prime_u64(p) {
        p -= m;
    }
    let a = p - 3;
    for b in 1u64..1_000_000 {
        let f = Fp64::new(p);
        let am = f.to_mont(a);
        let bm = f.to_mont(b);
        let disc = f.add(
            f.mul(f.to_mont(4), f.mul(f.sqr(am), am)),
            f.mul(f.to_mont(27), f.sqr(bm)),
        );
        if disc == 0 {
            continue;
        }
        let probe = FastCurve::new("probe", p, a, b, 1);
        let Some(g) = (1u64..10_000).find_map(|x| probe.lift_x(x)) else {
            continue;
        };
        let Some(order) = point_order_hasse(&probe, g) else {
            continue;
        };
        if !is_prime_u64(order) {
            continue;
        }
        let curve = FastCurve::new(&format!("a3-2adic{k}-{bits}bit"), p, a, b, order);
        let g = curve.lift_x(probe.canonical(g).0)?;
        if curve.mul(Some(g), order).is_some() {
            continue;
        }
        return Some((curve, g));
    }
    None
}

// ── Arm G: large-prime graph IC-2 ──────────────────────────────────────

/// Large-prime set: canonical `x ≤ x_l_max`, outside the factor base.
/// `None` = no large primes (the IC-2 baseline).
#[derive(Clone, Copy, Debug)]
struct GraphCell {
    fb_size: usize,
    /// `X_L = p >> shift`; `None` = baseline.
    lp_shift: Option<u32>,
}

#[derive(Clone, Debug, Default)]
struct GraphRun {
    solved: bool,
    log_correct: bool,
    stop_reason: String,
    targets: u64,
    // counted group operations (GAE)
    gae_sweep: u64,
    gae_step: u64,
    gae_setup: u64,
    gae_verify: u64,
    // unpriced native operations
    sqrt_fb: u64,
    hash_ops: u64,
    filter_row_ops: u64,
    la_mults: u64,
    // graph statistics
    full_relations: u64,
    partials: u64,
    vertices: u64,
    vertices_deg2: u64,
    cycles_available: u64,
    partials_into_filter: u64,
    filter_complete: u64,
    filter_cycles_closed: u64,
    replay_failures: u64,
    relations_used: u64,
    la_attempts: u64,
    lp_set_size: u64,
    fb_max_x: u64,
    wall_ms: f64,
    // derived per-phase wall
    collect_ms: f64,
    filter_ms: f64,
    la_ms: f64,
}

/// Dense Gauss–Jordan mod prime `n` on rows over columns `0..=cols-1`
/// (`cols − 1` is `x`); returns `x` when pinned.  Counts multiplications.
fn solve_dense(rows: &[(Vec<(u32, u64)>, u64)], cols: usize, n: u64, mults: &mut u64) -> Option<u64> {
    let mut m: Vec<Vec<u64>> = Vec::with_capacity(rows.len());
    let mut rhs: Vec<u64> = Vec::with_capacity(rows.len());
    for (entries, r) in rows {
        let mut row = vec![0u64; cols];
        for &(c, v) in entries {
            row[c as usize] = addmod(row[c as usize], v, n);
        }
        m.push(row);
        rhs.push(*r % n);
    }
    let nr = m.len();
    let mut piv_row_of = vec![usize::MAX; cols];
    let mut r0 = 0usize;
    for col in 0..cols {
        if r0 >= nr {
            break;
        }
        let Some(pr) = (r0..nr).find(|&r| m[r][col] != 0) else {
            continue;
        };
        m.swap(r0, pr);
        rhs.swap(r0, pr);
        let inv = invmod(m[r0][col], n);
        *mults += 2 * 62; // one inversion by exponentiation, priced as ~2·log₂n multiplications
        for c in col..cols {
            if m[r0][c] != 0 {
                m[r0][c] = mulmod(m[r0][c], inv, n);
                *mults += 1;
            }
        }
        rhs[r0] = mulmod(rhs[r0], inv, n);
        *mults += 1;
        let pivot = m[r0].clone();
        let prhs = rhs[r0];
        let nz: Vec<usize> = (col..cols).filter(|&c| pivot[c] != 0).collect();
        for r in 0..nr {
            if r == r0 || m[r][col] == 0 {
                continue;
            }
            let fct = m[r][col];
            for &c in &nz {
                m[r][c] = submod(m[r][c], mulmod(fct, pivot[c], n), n);
            }
            rhs[r] = submod(rhs[r], mulmod(fct, prhs, n), n);
            *mults += nz.len() as u64 + 1;
        }
        piv_row_of[col] = r0;
        r0 += 1;
    }
    for r in r0..nr {
        if rhs[r] != 0 && m[r].iter().all(|&v| v == 0) {
            return None;
        }
    }
    let pr = piv_row_of[cols - 1];
    if pr == usize::MAX {
        return None;
    }
    // x pinned only if its row has no free column left
    if m[pr]
        .iter()
        .enumerate()
        .any(|(c, &v)| c != cols - 1 && v != 0 && piv_row_of[c] == usize::MAX)
    {
        return None;
    }
    Some(rhs[pr])
}

/// One end-to-end graph-IC-2 solve of `Q = [k]G`.
fn graph_ic2(
    curve: &FastCurve,
    g: Pt,
    q: Pt,
    k_planted: u64,
    cell: GraphCell,
    extra: usize,
    max_gae: u64,
    seed: u64,
) -> GraphRun {
    let t_all = Instant::now();
    let f = curve.f;
    let n = curve.n;
    let bits = 64 - f.p.leading_zeros() as u64;
    let mut run = GraphRun::default();

    // Factor base: smallest x on the curve; one Tonelli–Shanks per x tried.
    let fb = FastFactorBase::build(curve, cell.fb_size);
    run.sqrt_fb = fb.max_x;
    run.fb_max_x = fb.max_x;
    let b_len = fb.len();
    let x_l_max: u64 = match cell.lp_shift {
        None => 0,
        Some(s) => (f.p >> s).max(fb.max_x),
    };
    run.lp_set_size = x_l_max.saturating_sub(fb.max_x);
    let lp_on = x_l_max > fb.max_x;
    let mut needed = b_len + 1 + extra;

    let mut rng = StdRng::seed_from_u64(seed);
    let a0 = rng.gen_range(1..n);
    let b0 = rng.gen_range(1..n);
    let mut a = a0;
    let b = b0;
    let Some(mut r) = curve.combine(g, q, a, b) else {
        run.stop_reason = "degenerate start".into();
        return run;
    };
    // Straus–Shamir over ~bits doublings and ~0.75·bits additions.
    run.gae_setup += (bits as f64 * 1.75).ceil() as u64;

    let mut fulls: Vec<(Vec<(u32, u64)>, u64)> = Vec::new();
    let mut partials: Vec<PartialRelation> = Vec::new();
    let mut deg: HashMap<u64, u32> = HashMap::new();
    let mut d = vec![0u64; b_len];
    let mut degenerate = vec![false; b_len];
    let mut scratch = Vec::with_capacity(b_len);
    let neg_b = (n - b % n) % n;
    let col_x = b_len as u32;
    let available = |fulls: usize, cyc: u64| fulls as u64 + cyc;

    'attempt: loop {
    let t_collect = Instant::now();
    while available(fulls.len(), run.cycles_available) < needed as u64 {
        if run.gae_sweep + run.gae_step + run.gae_verify >= max_gae {
            run.stop_reason = "gae budget".into();
            break;
        }
        run.targets += 1;
        for i in 0..b_len {
            let dd = f.sub(r.x, fb.pts[i].x);
            degenerate[i] = dd == 0;
            d[i] = if dd == 0 { f.one } else { dd };
        }
        f.batch_inv(&mut d, &mut scratch);
        run.gae_sweep += b_len as u64;
        for i in 0..b_len {
            let fi = fb.pts[i];
            if degenerate[i] {
                let s: i64 = if r.y == fi.y { 1 } else { -1 };
                fulls.push((vec![(i as u32, signed_mod(s, n)), (col_x, neg_b)], a));
                continue;
            }
            let inv = d[i];
            let lam_minus = f.mul(f.add(r.y, fi.y), inv);
            let lam_plus = f.mul(f.sub(r.y, fi.y), inv);
            for (eps, lam) in [(1i64, lam_minus), (-1i64, lam_plus)] {
                let x3 = f.sub(f.sub(f.sqr(lam), r.x), fi.x);
                let xc = f.from_mont(x3);
                if let Some(j) = fb.lookup(xc) {
                    if j >= i {
                        let y = f.sub(f.mul(lam, f.sub(r.x, x3)), r.y);
                        let tau: i64 = if y == fb.pts[j].y { 1 } else { -1 };
                        // verify R = eps·F_i + tau·F_j in the group
                        let fi_s = if eps < 0 { curve.neg(fi) } else { fi };
                        let fj_s = if tau < 0 { curve.neg(fb.pts[j]) } else { fb.pts[j] };
                        run.gae_verify += 1;
                        if curve.add(Some(fi_s), Some(fj_s)) != Some(r) {
                            continue;
                        }
                        let mut e: Vec<(u32, u64)> = Vec::new();
                        if i == j {
                            e.push((i as u32, signed_mod(eps + tau, n)));
                        } else {
                            e.push((i as u32, signed_mod(eps, n)));
                            e.push((j as u32, signed_mod(tau, n)));
                        }
                        e.retain(|t| t.1 != 0);
                        e.push((col_x, neg_b));
                        fulls.push((e, a));
                        run.full_relations += 1;
                    }
                } else if lp_on && xc > fb.max_x && xc <= x_l_max {
                    let y = f.sub(f.mul(lam, f.sub(r.x, x3)), r.y);
                    let t = Pt { x: x3, y };
                    let (tc, flipped) = curve.canonical_sign(t);
                    let tau: i64 = if flipped { -1 } else { 1 };
                    // verify R = eps·F_i + tau·T_c
                    let fi_s = if eps < 0 { curve.neg(fi) } else { fi };
                    let t_s = if tau < 0 { curve.neg(tc) } else { tc };
                    run.gae_verify += 1;
                    if curve.add(Some(fi_s), Some(t_s)) != Some(r) {
                        continue;
                    }
                    run.partials += 1;
                    run.hash_ops += 1;
                    let c = deg.entry(xc).or_insert(0);
                    *c += 1;
                    if *c >= 2 {
                        run.cycles_available += 1;
                    }
                    partials.push(PartialRelation {
                        id: partials.len() as u64,
                        small: vec![(i as u32, signed_mod(eps, n)), (col_x, neg_b)],
                        large: vec![(xc, signed_mod(tau, n))],
                        rhs: a,
                    });
                }
            }
        }
        // next target R ← R + G (one addition), a ← a + 1
        match curve.add(Some(r), Some(g)) {
            Some(nr) => {
                r = nr;
                a = addmod(a, 1, n);
                run.gae_step += 1;
            }
            None => {
                run.stop_reason = "walk hit O".into();
                break;
            }
        }
    }
    run.collect_ms += t_collect.elapsed().as_secs_f64() * 1e3;
    run.vertices = deg.len() as u64;
    run.vertices_deg2 = deg.values().filter(|&&c| c >= 2).count() as u64;
    if available(fulls.len(), run.cycles_available) < needed as u64 {
        if run.stop_reason.is_empty() {
            run.stop_reason = "insufficient relations".into();
        }
        run.wall_ms = t_all.elapsed().as_secs_f64() * 1e3;
        return run;
    }

    // Peel degree-one vertices ourselves (one hash lookup per partial), then
    // eliminate the 2-core with the shared filter.  Every attempt is priced.
    let t_filter = Instant::now();
    let core: Vec<PartialRelation> = partials
        .iter()
        .filter(|pr| deg.get(&pr.large[0].0).copied().unwrap_or(0) >= 2)
        .cloned()
        .collect();
    run.hash_ops += partials.len() as u64;
    run.partials_into_filter = core.len() as u64;
    let mut complete: Vec<CompleteRelation> = Vec::new();
    if !core.is_empty() {
        let opts = LargePrimeFilterOptions {
            modulus: n,
            ..Default::default()
        };
        match filter_large_primes(core.clone(), &opts) {
            Ok((c, rep)) => {
                run.filter_complete = c.len() as u64;
                run.filter_cycles_closed = rep.cycles_closed as u64;
                run.filter_row_ops += (rep.merges + rep.pivots + rep.peeled_relations) as u64;
                complete = c;
            }
            Err(e) => {
                run.stop_reason = format!("filter error {e:?}");
                run.wall_ms = t_all.elapsed().as_secs_f64() * 1e3;
                return run;
            }
        }
        // Replay every complete row against its sources (algebraic witness).
        for c in &complete {
            let ok = replay_complete_relation(c, &core, n).is_some_and(|rc| {
                let mut a = rc.small.clone();
                let mut b = c.small.clone();
                a.sort_unstable();
                b.sort_unstable();
                a == b && rc.rhs == c.rhs
            });
            if !ok {
                run.replay_failures += 1;
            }
        }
    }
    run.filter_ms += t_filter.elapsed().as_secs_f64() * 1e3;

    let mut rows: Vec<(Vec<(u32, u64)>, u64)> = fulls.clone();
    for c in &complete {
        rows.push((c.small.clone(), c.rhs));
    }
    run.relations_used = rows.len() as u64;
    let t_la = Instant::now();
    let mut mults = 0u64;
    let x = solve_dense(&rows, b_len + 1, n, &mut mults);
    run.la_mults += mults;
    run.la_ms += t_la.elapsed().as_secs_f64() * 1e3;
    match x {
        Some(x) => {
            run.solved = true;
            run.gae_verify += (bits as f64 * 1.5).ceil() as u64; // [x]G == Q
            run.log_correct = x == k_planted && curve.mul(Some(g), x) == Some(q);
            run.stop_reason = if run.log_correct {
                "solved".into()
            } else {
                "wrong log".into()
            };
            break 'attempt;
        }
        None => {
            run.la_attempts += 1;
            if run.la_attempts >= 4 {
                run.stop_reason = "x not pinned after 4 attempts".into();
                break 'attempt;
            }
            // rank deficiency: collect `extra` more relations and retry
            needed += extra.max(8);
        }
    }
    }
    run.wall_ms = t_all.elapsed().as_secs_f64() * 1e3;
    run
}

fn graph_row(curve: &FastCurve, bits: u32, seed: u64, cell: GraphCell, run: &GraphRun) -> Value {
    let sq = (curve.n as f64).sqrt();
    let gae = run.gae_sweep + run.gae_step + run.gae_setup + run.gae_verify;
    let unpriced = run.sqrt_fb + run.hash_ops + run.filter_row_ops + run.la_mults;
    let s = |w: f64| (gae as f64 + w * unpriced as f64) / sq;
    json!({
        "arm": "G", "bits": bits, "curve_slug": slug_of(curve), "n": curve.n.to_string(),
        "seed": seed, "fb_size": cell.fb_size,
        "lp_shift": cell.lp_shift, "lp_set_size": run.lp_set_size, "fb_max_x": run.fb_max_x,
        "solved": run.solved, "log_correct": run.log_correct, "stop_reason": run.stop_reason,
        "targets": run.targets,
        "gae": {"sweep": run.gae_sweep, "step": run.gae_step, "setup": run.gae_setup,
                "verify": run.gae_verify, "total": gae},
        "unpriced": {"fb_sqrt": run.sqrt_fb, "hash_ops": run.hash_ops,
                     "filter_row_ops": run.filter_row_ops, "la_mults": run.la_mults,
                     "total": unpriced},
        "S_w0": s(0.0), "S_w01": s(0.1), "S_w1": s(1.0),
        "graph": {"full_relations": run.full_relations, "partials": run.partials,
                  "vertices": run.vertices, "vertices_deg2": run.vertices_deg2,
                  "cycles_available": run.cycles_available,
                  "partials_into_filter": run.partials_into_filter,
                  "filter_complete": run.filter_complete,
                  "filter_cycles_closed": run.filter_cycles_closed,
                  "replay_failures": run.replay_failures,
                  "relations_used": run.relations_used,
                  "failed_la_attempts": run.la_attempts},
        "wall_ms": {"total": run.wall_ms, "collect": run.collect_ms,
                    "filter": run.filter_ms, "la": run.la_ms},
    })
}

fn rho_row(curve: &FastCurve, bits: u32, g: Pt, q: Pt, k: u64, seed: u64) -> Value {
    let mut cfg = RhoConfig::for_bits(bits, true);
    cfg.seed = seed;
    cfg.threads = cfg.threads.min(2);
    let sq = (curve.n as f64).sqrt();
    let walkers_total = (cfg.threads * cfg.walkers) as u64;
    // setup: 2^table_bits table entries and one start per walker, each a
    // Straus–Shamir combine (~1.75·bits GAE).
    let setup = ((1u64 << cfg.table_bits) + walkers_total) as f64 * 1.75 * bits as f64;
    match rho_parallel(curve, g, q, &cfg) {
        Some(r) => {
            let ok = r.log == k;
            let steps = r.total_steps as f64;
            let unpriced = r.distinguished_points as f64;
            json!({"arm": "rho", "bits": bits, "curve_slug": slug_of(curve), "n": curve.n.to_string(),
                "seed": seed, "log_correct": ok, "steps": r.total_steps,
                "distinguished_points": r.distinguished_points, "setup_gae": setup,
                "walkers_total": walkers_total, "dp_bits": r.dp_bits,
                "S_steps": steps / sq,
                "S_w0": (steps + setup) / sq,
                "S_w01": (steps + setup + 0.1 * unpriced) / sq,
                "S_w1": (steps + setup + unpriced) / sq,
                "wall_ms": r.wall_secs * 1e3})
        }
        None => json!({"arm": "rho", "bits": bits, "seed": seed, "log_correct": false,
                        "stop_reason": "rho returned None"}),
    }
}

fn arm_g(bits_list: &[u32], seeds: u64, quick: bool, out: &mut Vec<Value>, curves: &mut Vec<Value>) {
    let fb_sizes: &[usize] = if quick { &[16, 64] } else { &[16, 64, 256] };
    let shifts: &[Option<u32>] = &[None, Some(12), Some(8), Some(4), Some(0)];
    for &bits in bits_list {
        let Some((curve, g)) = find_a3_curve(bits, 3) else {
            eprintln!("no curve at {bits} bits");
            continue;
        };
        curves.push(curve_record(&curve, g, &format!("find_a3_curve({bits}, 3)")));
        eprintln!("[G] {bits} bits, n = {}, slug {}", curve.n, slug_of(&curve));
        // planted targets, shared by every arm at this size
        let targets: Vec<(u64, u64, Pt)> = (0..seeds)
            .filter_map(|s| {
                let seed = 0x6A11_0000 + bits as u64 * 1000 + s;
                let mut rng = StdRng::seed_from_u64(seed);
                let k = rng.gen_range(1..curve.n);
                curve.mul(Some(g), k).map(|q| (seed, k, q))
            })
            .collect();
        for &(seed, k, q) in &targets {
            let row = rho_row(&curve, bits, g, q, k, seed);
            eprintln!("  rho seed {seed:x}: S_steps {:.3}", row["S_steps"].as_f64().unwrap_or(f64::NAN));
            out.push(row);
        }
        for &fb_size in fb_sizes {
            for &shift in shifts {
                let cell = GraphCell { fb_size, lp_shift: shift };
                // Preregistered skip: the baseline's predicted cost ~p GAE.
                let predicted = match shift {
                    None => curve.f.p as f64,
                    Some(s) => {
                        let ell = ((curve.f.p >> s) as f64).max(1.0);
                        (curve.f.p as f64) * (fb_size as f64 / (2.0 * ell)).sqrt() + (curve.f.p as f64).sqrt()
                    }
                };
                let budget = if quick { 4.0e8 } else { 6.0e9 };
                if predicted > budget {
                    out.push(json!({"arm": "G", "bits": bits, "fb_size": fb_size, "lp_shift": shift,
                        "skipped": true, "predicted_gae": predicted, "budget_gae": budget}));
                    eprintln!("  G B={fb_size} shift={shift:?}: skipped (predicted {predicted:.2e} GAE)");
                    continue;
                }
                for &(seed, k, q) in &targets {
                    let run = graph_ic2(&curve, g, q, k, cell, 8, (budget * 2.0) as u64, seed ^ 0x5EED);
                    let row = graph_row(&curve, bits, seed, cell, &run);
                    eprintln!(
                        "  G B={fb_size} shift={shift:?} seed {seed:x}: {} S_w0 {:.2} targets {} partials {} wall {:.0} ms",
                        run.stop_reason,
                        row["S_w0"].as_f64().unwrap_or(f64::NAN),
                        run.targets,
                        run.partials,
                        run.wall_ms
                    );
                    out.push(row);
                }
            }
        }
    }
}

// ── Arm L: lattice smallness ───────────────────────────────────────────

/// Coefficients of `S₃(u, v, z)` in `(u, v)`, canonical integers, `[a][b]`
/// for `u^a v^b` (the probe-B formula).
fn s3_coeffs(curve: &FastCurve, z_mont: u64) -> [[u64; 3]; 3] {
    let f = curve.f;
    let z = z_mont;
    let two = f.to_mont(2);
    let four = f.to_mont(4);
    let a = curve.a;
    let cb = curve.b;
    let z2 = f.sqr(z);
    let m2z = f.neg(f.mul(two, z));
    let c = |v: u64| f.from_mont(v);
    let mut out = [[0u64; 3]; 3];
    out[2][2] = c(f.one);
    out[2][1] = c(m2z);
    out[1][2] = c(m2z);
    out[2][0] = c(z2);
    out[0][2] = c(z2);
    out[1][1] = c(f.sub(f.neg(f.mul(two, z2)), f.mul(two, a)));
    let lin = f.sub(f.neg(f.mul(f.mul(two, a), z)), f.mul(four, cb));
    out[1][0] = c(lin);
    out[0][1] = c(lin);
    out[0][0] = c(f.add(f.neg(f.mul(f.mul(four, cb), z)), f.sqr(a)));
    out
}

type MPoly = HashMap<Vec<u32>, BigInt>;

fn mpoly_mul(p: &MPoly, q: &MPoly) -> MPoly {
    let mut out: MPoly = HashMap::new();
    for (e1, c1) in p {
        for (e2, c2) in q {
            let e: Vec<u32> = e1.iter().zip(e2).map(|(a, b)| a + b).collect();
            *out.entry(e).or_insert_with(BigInt::zero) += c1 * c2;
        }
    }
    out.retain(|_, c| !c.is_zero());
    out
}

fn mpoly_eval(p: &MPoly, x: &[BigInt]) -> BigInt {
    let mut acc = BigInt::zero();
    for (e, c) in p {
        let mut t = c.clone();
        for (xi, &k) in x.iter().zip(e) {
            if k > 0 {
                t *= xi.pow(k);
            }
        }
        acc += t;
    }
    acc
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum LatKind {
    /// Interval factor base `x ≤ B`, unknowns `(x₁, x₂)`, bounds `(B, B)`.
    Interval,
    /// Fraction factor base `x ≡ a/b`, unknowns `(a₁, b₁, a₂, b₂)`, bound `X`.
    Fraction,
}

/// One planted trial: returns (vanishing rows, HG on first two, dim).
/// Both kinds go through the same lattice construction, LLL and test.
fn lattice_trial(
    curve: &FastCurve,
    kind: LatKind,
    m: usize,
    bound: u64,
    rng: &mut StdRng,
) -> Option<(usize, bool, usize)> {
    let p = curve.f.p;
    let pb = BigInt::from(p);
    // plant two factor-base points; keep the integer representation
    let pick = |rng: &mut StdRng| -> Option<(Pt, Vec<BigInt>)> {
        for _ in 0..100_000 {
            match kind {
                LatKind::Interval => {
                    let x = rng.gen_range(1..=bound);
                    if let Some(pt) = curve.lift_x(x) {
                        return Some((pt, vec![BigInt::from(x)]));
                    }
                }
                LatKind::Fraction => {
                    let a: i64 = rng.gen_range(-(bound as i64)..=(bound as i64));
                    let b: u64 = rng.gen_range(1..=bound);
                    if gcd(a.unsigned_abs(), b) != 1 {
                        continue;
                    }
                    let x = mulmod(signed_mod(a, p), invmod(b % p, p), p);
                    if let Some(pt) = curve.lift_x(x) {
                        return Some((pt, vec![BigInt::from(a), BigInt::from(b)]));
                    }
                }
            }
        }
        None
    };
    let (fi, ri) = pick(rng)?;
    let (fj, rj) = pick(rng)?;
    let fj_signed = if rng.gen_bool(0.5) { fj } else { curve.neg(fj) };
    let r = curve.add(Some(fi), Some(fj_signed))?;
    let c = s3_coeffs(curve, r.x);
    let nv = match kind {
        LatKind::Interval => 2,
        LatKind::Fraction => 4,
    };
    // the polynomial f (monic in its leading monomial u²v² / a₁²a₂²)
    let mut f: MPoly = HashMap::new();
    for i in 0..3u32 {
        for j in 0..3u32 {
            if c[i as usize][j as usize] == 0 {
                continue;
            }
            let e = match kind {
                LatKind::Interval => vec![i, j],
                LatKind::Fraction => vec![i, 2 - i, j, 2 - j],
            };
            f.insert(e, BigInt::from(c[i as usize][j as usize]));
        }
    }
    let root: Vec<BigInt> = match kind {
        LatKind::Interval => vec![ri[0].clone(), rj[0].clone()],
        LatKind::Fraction => vec![ri[0].clone(), ri[1].clone(), rj[0].clone(), rj[1].clone()],
    };
    {
        let v = mpoly_eval(&f, &root);
        if !(((v % &pb) + &pb) % &pb).is_zero() {
            return None; // instrument error; counted by caller as a failed trial
        }
    }
    let mut fpow: Vec<MPoly> = vec![HashMap::from([(vec![0u32; nv], BigInt::one())])];
    for k in 1..=m {
        let next = mpoly_mul(&fpow[k - 1], &f);
        fpow.push(next);
    }
    // rows: one per (α, β) ∈ [0, 2m]², k = min(⌊α/2⌋, ⌊β/2⌋, m)
    let mm = 2 * m as u32;
    let mut row_polys: Vec<MPoly> = Vec::new();
    for alpha in 0..=mm {
        for beta in 0..=mm {
            let k = (alpha / 2).min(beta / 2).min(m as u32);
            let shift: Vec<u32> = match kind {
                LatKind::Interval => vec![alpha - 2 * k, beta - 2 * k],
                LatKind::Fraction => vec![alpha - 2 * k, mm - alpha, beta - 2 * k, mm - beta],
            };
            let pk = pb.pow(m as u32 - k);
            let mut poly: MPoly = HashMap::new();
            for (e, cf) in &fpow[k as usize] {
                let e2: Vec<u32> = e.iter().zip(&shift).map(|(a, b)| a + b).collect();
                poly.insert(e2, cf * &pk);
            }
            row_polys.push(poly);
        }
    }
    let mut monos: Vec<Vec<u32>> = row_polys.iter().flat_map(|p| p.keys().cloned()).collect();
    monos.sort();
    monos.dedup();
    let dim = monos.len();
    if dim != row_polys.len() {
        return None; // non-square: shape error
    }
    let index: HashMap<Vec<u32>, usize> = monos.iter().cloned().enumerate().map(|(i, e)| (e, i)).collect();
    let bnd = BigInt::from(bound);
    let scale = |e: &[u32]| -> BigInt {
        let deg: u32 = e.iter().sum();
        bnd.pow(deg)
    };
    let mut basis: Vec<Vec<BigInt>> = Vec::with_capacity(dim);
    for poly in &row_polys {
        let mut row = vec![BigInt::zero(); dim];
        for (e, cf) in poly {
            row[index[e]] = cf * scale(e);
        }
        basis.push(row);
    }
    if lll_reduce(&mut basis, 0.99).is_err() {
        return None;
    }
    let pm = pb.pow(m as u32);
    let dimf = (dim as f64).sqrt();
    let mut vanishing = 0usize;
    let mut hg = true;
    for (ri, row) in basis.iter().enumerate() {
        let mut poly: MPoly = HashMap::new();
        let mut norm2 = BigInt::zero();
        for (col, e) in monos.iter().enumerate() {
            let cf = &row[col];
            if cf.is_zero() {
                continue;
            }
            norm2 += cf * cf;
            poly.insert(e.clone(), cf / scale(e));
        }
        if poly.is_empty() {
            continue;
        }
        if mpoly_eval(&poly, &root).is_zero() {
            vanishing += 1;
        }
        if ri < 2 {
            let norm = norm2.to_f64().unwrap_or(f64::INFINITY).sqrt();
            hg &= norm * dimf < pm.to_f64().unwrap_or(f64::INFINITY);
        }
    }
    Some((vanishing, hg, dim))
}

/// Number of distinct `a/b` with `|a| ≤ X`, `1 ≤ b ≤ X`, `gcd(a, b) = 1`.
fn fraction_count(x: u64) -> u64 {
    let mut cnt = 0u64;
    for b in 1..=x {
        for a in 0..=x {
            if gcd(a, b) == 1 {
                cnt += if a == 0 { 1 } else { 2 };
            }
        }
    }
    cnt
}

/// Largest `X` with `fraction_count(X) ≤ target`.
fn fraction_bound_for(target: u64) -> u64 {
    let mut x = 1u64;
    while fraction_count(x + 1) <= target {
        x += 1;
    }
    x
}

fn arm_l(bits_list: &[u32], trials: usize, levels: &[usize], out: &mut Vec<Value>, curves: &mut Vec<Value>) {
    let deltas = [0.06, 0.08, 0.10, 0.12, 0.14, 0.16, 0.18, 0.20];
    for &bits in bits_list {
        let Some((curve, g)) = find_a3_curve(bits, 3) else {
            continue;
        };
        curves.push(curve_record(&curve, g, &format!("find_a3_curve({bits}, 3)")));
        let p = curve.f.p as f64;
        for &m in levels {
            if m == 3 && bits > 40 {
                continue;
            }
            for kind in [LatKind::Interval, LatKind::Fraction] {
                for &delta in &deltas {
                    let target = p.powf(delta).floor().max(2.0) as u64;
                    let (bound, set_size) = match kind {
                        LatKind::Interval => (target, target),
                        LatKind::Fraction => {
                            let x = fraction_bound_for(target);
                            (x, fraction_count(x))
                        }
                    };
                    // same seed for both kinds at a cell: same R-sign coin flips
                    let seed = 0x1A77 + bits as u64 * 1000 + m as u64 * 100 + (delta * 100.0).round() as u64;
                    let mut rng = StdRng::seed_from_u64(seed);
                    let (mut succ, mut hgc, mut vsum, mut done, mut dim, mut failed) = (0, 0, 0, 0, 0, 0);
                    let t0 = Instant::now();
                    for _ in 0..trials {
                        match lattice_trial(&curve, kind, m, bound, &mut rng) {
                            Some((v, h, d)) => {
                                done += 1;
                                vsum += v;
                                dim = d;
                                if v >= 2 {
                                    succ += 1;
                                }
                                if h {
                                    hgc += 1;
                                }
                            }
                            None => failed += 1,
                        }
                    }
                    let ms = t0.elapsed().as_secs_f64() * 1e3 / done.max(1) as f64;
                    let d_eff = (set_size as f64).ln() / p.ln();
                    eprintln!(
                        "[L] {bits}b m={m} {kind:?} δ={delta:.2} δ_eff={d_eff:.3} bound={bound} |set|={set_size}: {succ}/{done} HG {hgc}/{done} ({ms:.0} ms)"
                    );
                    out.push(json!({"arm": "L", "bits": bits, "curve_slug": slug_of(&curve),
                        "kind": format!("{kind:?}"), "level": m, "dim": dim, "delta": delta,
                        "delta_eff": d_eff, "bound": bound, "set_size": set_size, "seed": seed,
                        "trials": done, "trial_failures": failed, "successes": succ,
                        "hg_first_two": hgc, "mean_vanishing": vsum as f64 / done.max(1) as f64,
                        "ms_per_trial": ms}));
                }
            }
        }
    }
}

// ── Arm F: fake subfields under the owner's F4 ─────────────────────────

#[derive(Clone, Copy, Debug, PartialEq)]
enum FbFamily {
    Interval,
    Subgroup,
    Fraction,
}

/// The factor-base x-set of size exactly `bsize` (not all on the curve).
fn fb_xset(p: u64, family: FbFamily, bsize: usize) -> Vec<u64> {
    match family {
        FbFamily::Interval => (1..=bsize as u64).collect(),
        FbFamily::Subgroup => {
            // generator of μ_B: g^((p−1)/B) for a primitive root candidate
            let e = (p - 1) / bsize as u64;
            for c in 2..p {
                let h = powmod(c, e, p);
                // order exactly B: h^(B/2) ≠ 1 (B a power of two)
                if powmod(h, bsize as u64 / 2, p) != 1 {
                    let mut v = Vec::with_capacity(bsize);
                    let mut t = 1u64;
                    for _ in 0..bsize {
                        v.push(t);
                        t = mulmod(t, h, p);
                    }
                    return v;
                }
            }
            unreachable!()
        }
        FbFamily::Fraction => {
            // fractions in order of height max(|a|, b), then b, then a
            let mut v: Vec<u64> = Vec::new();
            let mut seen = std::collections::HashSet::new();
            let mut h = 1u64;
            while v.len() < bsize {
                for b in 1..=h {
                    for a in -(h as i64)..=(h as i64) {
                        if a.unsigned_abs().max(b) != h || gcd(a.unsigned_abs(), b) != 1 {
                            continue;
                        }
                        let x = mulmod(signed_mod(a, p), invmod(b, p), p);
                        if seen.insert(x) && v.len() < bsize {
                            v.push(x);
                        }
                    }
                }
                h += 1;
            }
            v
        }
    }
}

/// `Π (X − r)` over `F_p` as dense coefficients, low degree first.
fn membership_poly(p: u64, roots: &[u64]) -> Vec<u64> {
    let mut c = vec![1u64];
    for &r in roots {
        let mut next = vec![0u64; c.len() + 1];
        for (i, &ci) in c.iter().enumerate() {
            next[i + 1] = addmod(next[i + 1], ci, p);
            next[i] = submod(next[i], mulmod(ci, r, p), p);
        }
        c = next;
    }
    c
}

fn standard_monomials_2var(basis: &[Poly]) -> Option<u64> {
    let lms: Vec<&Vec<u32>> = basis.iter().filter_map(|f| f.first().map(|t| &t.0)).collect();
    let a = lms.iter().filter(|e| e[1] == 0).map(|e| e[0]).min()?;
    let b = lms.iter().filter(|e| e[0] == 0).map(|e| e[1]).min()?;
    let mut cnt = 0u64;
    for i in 0..a {
        for j in 0..b {
            if !lms.iter().any(|e| e[0] <= i && e[1] <= j) {
                cnt += 1;
            }
        }
    }
    Some(cnt)
}

fn arm_f(bsizes: &[usize], seeds: u64, budget_s: u64, out: &mut Vec<Value>, curves: &mut Vec<Value>) {
    let bits = 24u32;
    let Some((curve, g)) = find_a3_curve_2adic(bits, 10) else {
        eprintln!("no 2-adic curve");
        return;
    };
    curves.push(curve_record(&curve, g, &format!("find_a3_curve_2adic({bits}, 10)")));
    let p = curve.f.p;
    eprintln!("[F] p = {p}, slug {}", slug_of(&curve));
    let ord = MonoOrder::Grevlex;
    for &bsize in bsizes {
        for family in [FbFamily::Interval, FbFamily::Subgroup, FbFamily::Fraction] {
            let xs = fb_xset(p, family, bsize);
            let on_curve: Vec<(u64, Pt)> = xs.iter().filter_map(|&x| curve.lift_x(x).map(|pt| (x, pt))).collect();
            let lpoly = membership_poly(p, &xs);
            for s in 0..seeds {
                let seed = 0xF4F4_0000 + bsize as u64 * 100 + s;
                let mut rng = StdRng::seed_from_u64(seed);
                if on_curve.len() < 2 {
                    continue;
                }
                let (u, pu) = on_curve[rng.gen_range(0..on_curve.len())];
                let (v, pv) = on_curve[rng.gen_range(0..on_curve.len())];
                let pv_s = if rng.gen_bool(0.5) { pv } else { curve.neg(pv) };
                let Some(r) = curve.add(Some(pu), Some(pv_s)) else {
                    continue;
                };
                let c = s3_coeffs(&curve, r.x);
                let mut s3: Vec<(Vec<u32>, u64)> = Vec::new();
                for i in 0..3u32 {
                    for j in 0..3u32 {
                        s3.push((vec![i, j], c[i as usize][j as usize]));
                    }
                }
                let l1: Vec<(Vec<u32>, u64)> = lpoly.iter().enumerate().map(|(k, &cf)| (vec![k as u32, 0], cf)).collect();
                let l2: Vec<(Vec<u32>, u64)> = lpoly.iter().enumerate().map(|(k, &cf)| (vec![0, k as u32], cf)).collect();
                let input: Vec<Poly> = vec![
                    f4_fp::normalise(&s3, p, ord),
                    f4_fp::normalise(&l1, p, ord),
                    f4_fp::normalise(&l2, p, ord),
                ];
                let planted_ok = input.iter().all(|f| f4_fp::eval(f, &[u, v], p) == 0);
                let opts = F4Options::new(ord, 2 * bsize as u32 + 8).with_budget(Duration::from_secs(budget_s));
                let rep = f4_fp::f4(&input, 2, p, &opts);
                let root_in_variety = rep.basis.iter().all(|f| f4_fp::eval(f, &[u, v], p) == 0);
                let sols = if rep.timed_out { None } else { standard_monomials_2var(&rep.basis) };
                // direct subtraction: 7 multiplications per factor-base point
                // (3 in the shared inversion, 2 slopes, 2 squarings), on the
                // on-curve part of the set; the inversion itself ~2·log₂p.
                let ds = 7 * on_curve.len() as u64 + 2 * bits as u64;
                eprintln!(
                    "[F] B={bsize} {family:?} seed {s}: ops {} deg {} cols {} {:.0} ms sols {:?} root {} timed_out {}",
                    rep.field_ops, rep.solving_degree_max, rep.max_cols, rep.ms, sols, root_in_variety, rep.timed_out
                );
                out.push(json!({"arm": "F", "p_bits": bits, "curve_slug": slug_of(&curve),
                    "family": format!("{family:?}"), "B": bsize, "on_curve": on_curve.len(),
                    "seed": seed, "planted_ok": planted_ok, "root_in_variety": root_in_variety,
                    "inconsistent": rep.inconsistent, "timed_out": rep.timed_out,
                    "pairs_above_bound": rep.pairs_above_bound,
                    "field_ops": rep.field_ops, "solving_degree": rep.solving_degree_max,
                    "max_rows": rep.max_rows, "max_cols": rep.max_cols, "steps": rep.steps,
                    "ms": rep.ms, "solutions_with_multiplicity": sols,
                    "direct_subtraction_mults": ds,
                    "ratio_f4_over_direct": rep.field_ops as f64 / ds as f64}));
            }
        }
    }
}

// ── Summary: the protocol's decision rules, applied to the raw rows ────

fn read_rows(path: &str) -> Vec<Value> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    text.lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter(|v| v.get("meta").is_none())
        .collect()
}

fn median(mut v: Vec<f64>) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}

/// Least-squares slope of `ln y` against `ln x`.
fn loglog_slope(pts: &[(f64, f64)]) -> Option<f64> {
    if pts.len() < 2 {
        return None;
    }
    let xs: Vec<f64> = pts.iter().map(|p| p.0.ln()).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1.ln()).collect();
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let sxy: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    (sxx > 0.0).then(|| sxy / sxx)
}

const WS: [&str; 3] = ["S_w0", "S_w01", "S_w1"];

fn summarize_g(rows: &[Value]) -> Value {
    use std::collections::BTreeMap;
    // cells keyed by (bits, B, shift or -1)
    let mut cells: BTreeMap<(u64, u64, i64), Vec<&Value>> = BTreeMap::new();
    let mut skipped = Vec::new();
    let mut rho: BTreeMap<u64, Vec<&Value>> = BTreeMap::new();
    for r in rows {
        let bits = r["bits"].as_u64().unwrap_or(0);
        match r["arm"].as_str() {
            Some("rho") => rho.entry(bits).or_default().push(r),
            Some("G") if r["skipped"].as_bool() == Some(true) => skipped.push(r.clone()),
            Some("G") => {
                let b = r["fb_size"].as_u64().unwrap_or(0);
                let s = r["lp_shift"].as_i64().unwrap_or(-1);
                cells.entry((bits, b, s)).or_default().push(r);
            }
            _ => {}
        }
    }
    let rho_med: BTreeMap<u64, Value> = rho
        .iter()
        .map(|(&bits, rs)| {
            let ok = rs.iter().all(|r| r["log_correct"].as_bool() == Some(true));
            let mut m = serde_json::Map::new();
            m.insert("all_correct".into(), json!(ok));
            m.insert("runs".into(), json!(rs.len()));
            for k in WS.iter().chain(["S_steps"].iter()) {
                m.insert(k.to_string(), json!(median(rs.iter().filter_map(|r| r[*k].as_f64()).collect())));
            }
            (bits, Value::Object(m))
        })
        .collect();
    let mut cell_out = Vec::new();
    let mut med: BTreeMap<(u64, u64, i64), [f64; 3]> = BTreeMap::new();
    for (&(bits, b, s), rs) in &cells {
        let valid = rs.len() >= 3 && rs.iter().all(|r| r["log_correct"].as_bool() == Some(true));
        let m: Vec<f64> = WS
            .iter()
            .map(|k| median(rs.iter().filter_map(|r| r[*k].as_f64()).collect()))
            .collect();
        let ell = rs[0]["lp_set_size"].as_u64().unwrap_or(0);
        if valid {
            med.insert((bits, b, s), [m[0], m[1], m[2]]);
        }
        let rho_w0 = rho_med.get(&bits).and_then(|v| v["S_w0"].as_f64()).unwrap_or(f64::NAN);
        cell_out.push(json!({"bits": bits, "fb_size": b, "lp_shift": if s < 0 { Value::Null } else { json!(s) },
            "lp_set_size": ell, "runs": rs.len(), "valid": valid,
            "median_targets": median(rs.iter().filter_map(|r| r["targets"].as_f64()).collect()),
            "median_S_w0": m[0], "median_S_w01": m[1], "median_S_w1": m[2],
            "ratio_to_rho_w0": m[0] / rho_w0,
            "median_partials": median(rs.iter().filter_map(|r| r["graph"]["partials"].as_f64()).collect()),
            "median_full_relations": median(rs.iter().filter_map(|r| r["graph"]["full_relations"].as_f64()).collect()),
            "median_cycles_closed": median(rs.iter().filter_map(|r| r["graph"]["filter_cycles_closed"].as_f64()).collect()),
            "stop_reasons": rs.iter().map(|r| r["stop_reason"].clone()).collect::<Vec<_>>()}));
    }
    // G1
    let mut g1 = Vec::new();
    for (&(bits, b, s), m) in &med {
        if s < 0 {
            continue;
        }
        let verdict = match med.get(&(bits, b, -1)) {
            Some(base) => {
                if (0..3).all(|i| m[i] < base[i]) {
                    "lowers IC-2 cost"
                } else if (0..3).all(|i| m[i] >= base[i]) {
                    "does not lower IC-2 cost"
                } else {
                    "unresolved in band"
                }
            }
            None => "no baseline (skipped): extrapolated-only",
        };
        g1.push(json!({"bits": bits, "fb_size": b, "lp_shift": s, "verdict": verdict,
            "ratio_to_baseline_w0": med.get(&(bits, b, -1)).map(|base| m[0] / base[0])}));
    }
    // G2
    let mut g2 = Vec::new();
    let keys: std::collections::BTreeSet<(u64, u64)> = med.keys().map(|k| (k.0, k.1)).collect();
    for (bits, b) in keys {
        let pts: Vec<(f64, f64)> = cells
            .iter()
            .filter(|(k, _)| k.0 == bits && k.1 == b && k.2 >= 0 && med.contains_key(k))
            .map(|(k, rs)| (rs[0]["lp_set_size"].as_f64().unwrap_or(0.0), med[k][0]))
            .filter(|p| p.0 > 0.0)
            .collect();
        if pts.len() < 3 {
            g2.push(json!({"bits": bits, "fb_size": b, "points": pts.len(), "verdict": "too few cells"}));
            continue;
        }
        let slope = loglog_slope(&pts).unwrap_or(f64::NAN);
        let verdict = if (-0.65..=-0.35).contains(&slope) { "model holds" } else { "model fails" };
        g2.push(json!({"bits": bits, "fb_size": b, "points": pts.len(), "slope": slope, "verdict": verdict}));
    }
    // G3
    let mut g3 = Vec::new();
    let sizes: Vec<u64> = rho_med.keys().copied().collect();
    let mut below: Vec<bool> = Vec::new();
    for &bits in &sizes {
        let r = &rho_med[&bits];
        let mut ratios = Vec::new();
        let mut best_cell = Vec::new();
        for (i, k) in WS.iter().enumerate() {
            let rho_s = r[*k].as_f64().unwrap_or(f64::NAN);
            let best = med
                .iter()
                .filter(|(c, _)| c.0 == bits)
                .min_by(|a, b| a.1[i].partial_cmp(&b.1[i]).unwrap());
            match best {
                Some((c, m)) => {
                    ratios.push(m[i] / rho_s);
                    best_cell.push(json!({"fb_size": c.1, "lp_shift": if c.2 < 0 { Value::Null } else { json!(c.2) }}));
                }
                None => {
                    ratios.push(f64::NAN);
                    best_cell.push(Value::Null);
                }
            }
        }
        below.push(ratios.iter().all(|&x| x < 1.0));
        g3.push(json!({"bits": bits, "rho_all_correct": r["all_correct"], "ratio_best_to_rho": {"w0": ratios[0], "w01": ratios[1], "w1": ratios[2]},
            "best_cell": best_cell}));
    }
    let n = below.len();
    let below_rho = n >= 2 && below[n - 1] && below[n - 2] && sizes.last() == Some(&32);
    json!({"cells": cell_out, "rho": rho_med.iter().map(|(b, v)| json!({"bits": b, "summary": v})).collect::<Vec<_>>(),
        "skipped": skipped, "G1": g1, "G2": g2, "G3": g3,
        "G3_verdict": if below_rho { "below rho at two consecutive sizes: AUDIT REQUIRED" } else { "not below rho: distance reported" }})
}

fn summarize_l(rows: &[Value]) -> Value {
    use std::collections::BTreeMap;
    let mut by: BTreeMap<(u64, u64, String), Vec<&Value>> = BTreeMap::new();
    for r in rows.iter().filter(|r| r["arm"] == "L") {
        by.entry((
            r["bits"].as_u64().unwrap_or(0),
            r["level"].as_u64().unwrap_or(0),
            r["kind"].as_str().unwrap_or("").to_string(),
        ))
        .or_default()
        .push(r);
    }
    let mut dstar: BTreeMap<(u64, u64, String), f64> = BTreeMap::new();
    let mut pos_ok = true;
    let mut series = Vec::new();
    for (k, rs) in &by {
        let mut best = 0.0f64;
        for r in rs {
            let s = r["successes"].as_u64().unwrap_or(0);
            let t = r["trials"].as_u64().unwrap_or(0);
            let d = r["delta"].as_f64().unwrap_or(0.0);
            if t > 0 && 2 * s >= t && d > best {
                best = d;
            }
            if k.2 == "Interval" && d <= 0.10 + 1e-9 && (t == 0 || (s as f64) < 0.875 * t as f64) {
                pos_ok = false;
            }
        }
        dstar.insert(k.clone(), best);
        series.push(json!({"bits": k.0, "level": k.1, "kind": k.2, "delta_star": best,
            "cells": rs.iter().map(|r| json!({"delta": r["delta"], "delta_eff": r["delta_eff"], "successes": r["successes"],
                "trials": r["trials"], "hg_first_two": r["hg_first_two"], "set_size": r["set_size"]})).collect::<Vec<_>>()}));
    }
    // per (bits, level) difference
    let mut diffs: BTreeMap<(u64, u64), f64> = BTreeMap::new();
    for ((b, l, kind), d) in &dstar {
        if kind == "Fraction" {
            if let Some(di) = dstar.get(&(*b, *l, "Interval".to_string())) {
                diffs.insert((*b, *l), d - di);
            }
        }
    }
    let levels: std::collections::BTreeSet<u64> = diffs.keys().map(|k| k.1).collect();
    let sizes: std::collections::BTreeSet<u64> = diffs.keys().map(|k| k.0).collect();
    let both = |l: u64, pred: &dyn Fn(f64) -> bool| sizes.len() >= 2 && sizes.iter().all(|b| diffs.get(&(*b, l)).is_some_and(|d| pred(*d)));
    let extends = levels.iter().any(|&l| both(l, &|d| d >= 0.04 - 1e-9));
    let loses = levels.iter().any(|&l| both(l, &|d| d <= -0.04 + 1e-9));
    let same = diffs.values().all(|d| d.abs() <= 0.02 + 1e-9);
    let verdict = if !pos_ok {
        "positive control failed: invalid instrument"
    } else if extends {
        "Fraction extends reach"
    } else if loses {
        "Fraction loses reach"
    } else if same {
        "same reach"
    } else {
        "inconclusive"
    };
    let milestone = rows.iter().any(|r| {
        r["arm"] == "L" && r["kind"] == "Fraction" && r["delta"].as_f64().unwrap_or(0.0) >= 0.20 - 1e-9 && {
            let s = r["successes"].as_u64().unwrap_or(0);
            let t = r["trials"].as_u64().unwrap_or(0);
            t > 0 && 2 * s >= t
        }
    });
    json!({"series": series, "positive_control_ok": pos_ok,
        "delta_star_diff_fraction_minus_interval": diffs.iter().map(|(k, d)| json!({"bits": k.0, "level": k.1, "diff": d})).collect::<Vec<_>>(),
        "verdict": verdict, "milestone_s4_followup": milestone})
}

fn summarize_f(rows: &[Value]) -> Value {
    use std::collections::BTreeMap;
    let mut by: BTreeMap<(String, u64), Vec<&Value>> = BTreeMap::new();
    for r in rows.iter().filter(|r| r["arm"] == "F") {
        by.entry((r["family"].as_str().unwrap_or("").to_string(), r["B"].as_u64().unwrap_or(0)))
            .or_default()
            .push(r);
    }
    let mut pos_ok = true;
    let mut max_sol_ratio = 0.0f64;
    let mut fam: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut pts: BTreeMap<String, Vec<(f64, f64, f64)>> = BTreeMap::new();
    for ((family, b), rs) in &by {
        let done: Vec<&&Value> = rs.iter().filter(|r| r["timed_out"].as_bool() == Some(false)).collect();
        for r in &done {
            if r["root_in_variety"].as_bool() != Some(true) || r["planted_ok"].as_bool() != Some(true) {
                pos_ok = false;
            }
            if let Some(s) = r["solutions_with_multiplicity"].as_f64() {
                max_sol_ratio = max_sol_ratio.max(s / *b as f64);
            }
        }
        let complete = done.len() == rs.len() && !rs.is_empty();
        let ops = median(done.iter().filter_map(|r| r["field_ops"].as_f64()).collect());
        let ratio = median(done.iter().filter_map(|r| r["ratio_f4_over_direct"].as_f64()).collect());
        if complete {
            pts.entry(family.clone()).or_default().push((*b as f64, ops, ratio));
        }
        fam.entry(family.clone()).or_default().push(json!({"B": b, "runs": rs.len(), "completed": done.len(),
            "median_field_ops": ops, "median_ratio_to_direct": ratio,
            "median_solving_degree": median(done.iter().filter_map(|r| r["solving_degree"].as_f64()).collect()),
            "median_max_cols": median(done.iter().filter_map(|r| r["max_cols"].as_f64()).collect()),
            "median_ms": median(done.iter().filter_map(|r| r["ms"].as_f64()).collect()),
            "median_solutions": median(done.iter().filter_map(|r| r["solutions_with_multiplicity"].as_f64()).collect())}));
    }
    let mut verdicts = Vec::new();
    for (family, p) in &pts {
        let slope = loglog_slope(&p.iter().map(|x| (x.0, x.1)).collect::<Vec<_>>());
        let last_ratio = p.last().map(|x| x.2).unwrap_or(f64::NAN);
        let verdict = if !pos_ok {
            "positive control failed: invalid instrument"
        } else if p.len() < 4 {
            "inconclusive (fewer than 4 completed B)"
        } else if slope.is_some_and(|s| s < 1.0) && last_ratio < 1.0 {
            "sub-enumeration (advance candidate)"
        } else if slope.is_some_and(|s| s >= 1.0) && p.iter().all(|x| x.2 > 1.0) {
            "no lever"
        } else {
            "inconclusive"
        };
        verdicts.push(json!({"family": family, "completed_B": p.len(), "slope": slope, "ratio_at_largest_B": last_ratio, "verdict": verdict}));
    }
    json!({"families": fam, "verdicts": verdicts, "positive_control_ok": pos_ok, "max_solutions_over_B": max_sol_ratio})
}

fn summarize(dir: &str) {
    let g = read_rows(&format!("{dir}/arm_g.jsonl"));
    let l = read_rows(&format!("{dir}/arm_l.jsonl"));
    let f = read_rows(&format!("{dir}/arm_f.jsonl"));
    let out = json!({
        "protocol": "research/prime_smallness_graph_20261023/PROTOCOL.md",
        "computed_by": "prime_smallness_graph_probe --arm s",
        "G": summarize_g(&g), "L": summarize_l(&l), "F": summarize_f(&f),
    });
    let path = format!("{dir}/summary.json");
    std::fs::write(&path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    eprintln!("wrote {path}");
}

// ── main ───────────────────────────────────────────────────────────────

fn parse_list<T: std::str::FromStr>(s: Option<&String>) -> Option<Vec<T>> {
    s.map(|s| s.split(',').filter_map(|x| x.parse().ok()).collect())
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let mut arm = "glf".to_string();
    let mut bits: Option<Vec<u32>> = None;
    let mut seeds: Option<u64> = None;
    let mut out_dir = ".".to_string();
    let mut quick = false;
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--arm" => {
                i += 1;
                arm = argv.get(i).cloned().unwrap_or(arm);
            }
            "--bits" => {
                i += 1;
                bits = parse_list(argv.get(i));
            }
            "--seeds" => {
                i += 1;
                seeds = argv.get(i).and_then(|s| s.parse().ok());
            }
            "--out" => {
                i += 1;
                out_dir = argv.get(i).cloned().unwrap_or(out_dir);
            }
            "--quick" => quick = true,
            other => eprintln!("ignoring {other}"),
        }
        i += 1;
    }
    std::fs::create_dir_all(&out_dir).expect("out dir");
    if arm == "s" {
        summarize(&out_dir);
        return;
    }
    let started = Instant::now();
    let mut curves: Vec<Value> = Vec::new();
    let meta = json!({
        "probe": "prime_smallness_graph_probe",
        "protocol": "research/prime_smallness_graph_20261023/PROTOCOL.md",
        "threads_available": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
        "quick": quick,
        "git_commit": option_env!("GIT_COMMIT").unwrap_or("see run report"),
    });
    let write = |name: &str, rows: &[Value], meta: &Value| {
        let path = format!("{out_dir}/{name}");
        let mut f = std::fs::File::create(&path).expect("create");
        writeln!(f, "{}", serde_json::to_string(&json!({"meta": meta})).unwrap()).unwrap();
        for r in rows {
            writeln!(f, "{}", serde_json::to_string(r).unwrap()).unwrap();
        }
        eprintln!("wrote {path} ({} rows)", rows.len());
    };
    if arm.contains('g') {
        let mut rows = Vec::new();
        let b = bits.clone().unwrap_or_else(|| if quick { vec![20, 24] } else { vec![20, 24, 28, 32] });
        arm_g(&b, seeds.unwrap_or(3), quick, &mut rows, &mut curves);
        write("arm_g.jsonl", &rows, &meta);
    }
    if arm.contains('l') {
        let mut rows = Vec::new();
        let b = bits.clone().unwrap_or_else(|| vec![40, 56]);
        let levels: &[usize] = if quick { &[1] } else { &[1, 2, 3] };
        arm_l(&b, if quick { 4 } else { seeds.map(|s| s as usize).unwrap_or(16) }, levels, &mut rows, &mut curves);
        write("arm_l.jsonl", &rows, &meta);
    }
    if arm.contains('f') {
        let mut rows = Vec::new();
        let bs: &[usize] = if quick { &[8, 16] } else { &[8, 16, 32, 64, 128] };
        arm_f(bs, seeds.unwrap_or(3), if quick { 30 } else { 600 }, &mut rows, &mut curves);
        write("arm_f.jsonl", &rows, &meta);
    }
    curves.sort_by_key(|c| c["slug"].to_string());
    curves.dedup_by_key(|c| c["slug"].to_string());
    if !curves.is_empty() {
        let path = format!("{out_dir}/curves.json");
        std::fs::write(&path, serde_json::to_string_pretty(&json!({"curves": curves})).unwrap()).unwrap();
    }
    eprintln!("done in {:.1} s", started.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_ic2_recovers_planted_log_with_and_without_large_primes() {
        let (curve, g) = find_a3_curve(16, 3).unwrap();
        let k = 12_345 % curve.n;
        let q = curve.mul(Some(g), k).unwrap();
        for shift in [None, Some(4), Some(0)] {
            let run = graph_ic2(&curve, g, q, k, GraphCell { fb_size: 16, lp_shift: shift }, 8, 1 << 30, 7);
            assert!(run.log_correct, "{shift:?}: {}", run.stop_reason);
            assert_eq!(run.replay_failures, 0);
        }
    }

    #[test]
    fn fraction_lattice_root_satisfies_h_mod_p() {
        let (curve, _) = find_a3_curve(40, 3).unwrap();
        let mut rng = StdRng::seed_from_u64(1);
        // lattice_trial returns None if the planted root does not satisfy H mod p
        let r = lattice_trial(&curve, LatKind::Fraction, 1, 3, &mut rng);
        assert!(r.is_some());
        let (_, _, dim) = r.unwrap();
        assert_eq!(dim, 9);
    }

    #[test]
    fn small_fraction_lattice_finds_root() {
        // positive control: at a tiny bound both kinds succeed
        let (curve, _) = find_a3_curve(40, 3).unwrap();
        let mut rng = StdRng::seed_from_u64(2);
        for kind in [LatKind::Interval, LatKind::Fraction] {
            let (v, _, _) = lattice_trial(&curve, kind, 1, 3, &mut rng).unwrap();
            assert!(v >= 2, "{kind:?}");
        }
    }

    #[test]
    fn fraction_count_matches_brute_force() {
        let p = 1_000_003u64;
        for x in 1..8u64 {
            let mut set = std::collections::HashSet::new();
            for b in 1..=x {
                for a in -(x as i64)..=(x as i64) {
                    set.insert(mulmod(signed_mod(a, p), invmod(b, p), p));
                }
            }
            assert_eq!(fraction_count(x), set.len() as u64);
        }
    }

    #[test]
    fn f4_planted_root_lies_in_variety_for_each_family() {
        let (curve, _) = find_a3_curve_2adic(20, 6).unwrap();
        let p = curve.f.p;
        let ord = MonoOrder::Grevlex;
        for family in [FbFamily::Interval, FbFamily::Subgroup, FbFamily::Fraction] {
            let xs = fb_xset(p, family, 8);
            assert_eq!(xs.len(), 8);
            let on: Vec<(u64, Pt)> = xs.iter().filter_map(|&x| curve.lift_x(x).map(|pt| (x, pt))).collect();
            let (u, pu) = on[0];
            let (v, pv) = on[on.len() - 1];
            let r = curve.add(Some(pu), Some(pv)).unwrap();
            let c = s3_coeffs(&curve, r.x);
            let s3: Vec<(Vec<u32>, u64)> = (0..9).map(|t| (vec![t / 3, t % 3], c[(t / 3) as usize][(t % 3) as usize])).collect();
            let l = membership_poly(p, &xs);
            let l1: Vec<_> = l.iter().enumerate().map(|(k, &cf)| (vec![k as u32, 0], cf)).collect();
            let l2: Vec<_> = l.iter().enumerate().map(|(k, &cf)| (vec![0, k as u32], cf)).collect();
            let input = vec![f4_fp::normalise(&s3, p, ord), f4_fp::normalise(&l1, p, ord), f4_fp::normalise(&l2, p, ord)];
            assert!(input.iter().all(|f| f4_fp::eval(f, &[u, v], p) == 0));
            let rep = f4_fp::f4(&input, 2, p, &F4Options::new(ord, 24));
            assert!(!rep.inconsistent);
            assert!(rep.basis.iter().all(|f| f4_fp::eval(f, &[u, v], p) == 0), "{family:?}");
            assert!(standard_monomials_2var(&rep.basis).unwrap() >= 1);
        }
    }
}
