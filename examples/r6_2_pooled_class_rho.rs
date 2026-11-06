//! R6.2: pooled multi-target rho on an isogeny class, and the cost of
//! finding the path (`research/isogeny_fibers_20261010/round6/PROGRAM.md`,
//! "R6.2"; results in `round6/r6_2_pooled_rho/`).
//!
//! Toy instances only; no sub-rho claim and nothing about a deployed curve.
//!
//! * `--mode batch`: a base curve `E₀` with `#E₀ = h·r`, `r` prime of
//!   `rbits` bits, a class of curves built by breadth-first search over
//!   `F_p`-rational kernels of degree `ℓ ∈ {2, 3, 5, 7}`
//!   (`isogeny_eval::small_degree`, every edge with a certified dual), and
//!   per trial three arms, all on the library walk that ecbench's
//!   `rho.multitarget_pooled` and `rho.negation` run
//!   (`ic_boundary::rho_batch_with` / `rho_walk_with` on `{P, −P}`):
//!   - **S** (single): one planted key on `E₀`, `rho_walk_with`
//!     (ecbench `rho.negation`); the independent-solve cost is `L·S`.
//!   - **B** (transport): keys planted on `E₁ … E_L`, mapped to `E₀` along
//!     the certified duals, solved in one pooled batch, each log mapped back
//!     and verified on its own curve.
//!   - **C** (control): `L` keys planted on `E₀`, the same pooled batch
//!     with the same algorithm seed.
//! * `--mode paths` (schema `r6_2.paths/v2`, the corrected search): whole
//!   components of the *certified-edge* graph of toy curves, and for random
//!   pairs of curves in a component a bidirectional breadth-first search that
//!   finds a path from scratch, counting neighbour enumerations (one per
//!   (curve, ℓ): the rational roots of the ℓ-division polynomial, the role
//!   `Φ_ℓ(j, Y)` root finding plays in an Elkies walk). An edge `u → v` is
//!   in the graph only if `SmallIsogeny::dual` certifies the reverse edge
//!   `v → u` (every ℓ = 2, 3 edge; ℓ = 5, 7 edges only when p ≡ ±1 mod ℓ), so
//!   expanding outgoing edges from the target side really constructs
//!   predecessors. Each certification is counted (`cert_by_ell`), and every
//!   returned route is re-validated edge by edge in the forward direction
//!   from `a` (re-enumerating the kernels on the chained curves, counted as
//!   `validation_enumerations`). The pre-fix search (outgoing edges only on
//!   both sides) runs alongside on the same pairs as the `legacy` control.
//! * `--mode paths-legacy` (schema `r6_2.paths/v1`): the search as first
//!   committed, kept so the frozen `paths.json` stays reproducible. Its
//!   target-side expansion follows outgoing edges, which are not predecessors
//!   for ℓ = 5, 7 with p ≢ ±1 mod ℓ; its lengths are not executable-route
//!   lengths.
//!
//! Every number written is a count (group operations, field `M`/`I`,
//! neighbour enumerations); wall times are recorded as contended notes.

use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    rho_batch_with, rho_cap, rho_walk_with, CountedGroup, GroupOps, NegationClasses, PrimeCurve,
    PrimePoint, RhoWalk,
};
use crypto_lib::cryptanalysis::isogeny_eval::counted_fp::is_prime_u64;
use crypto_lib::cryptanalysis::isogeny_eval::small_degree::{isomorphisms, kernel_representatives};
use crypto_lib::cryptanalysis::isogeny_eval::{
    OpCounter, PrimeField, ShortWeierstrass, SmallIsogeny, WPoint,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

const ELLS: [u64; 4] = [2, 3, 5, 7];

fn arg<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn splitmix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut acc = 1u64 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            acc = mulmod(acc, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    acc
}

/// `a⁻¹ mod r` for prime `r`.
fn invmod(a: u64, r: u64) -> u64 {
    powmod(a, r - 2, r)
}

fn random_point(f: &PrimeField, e: &ShortWeierstrass, rng: &mut StdRng) -> (u64, u64) {
    loop {
        let x = rng.gen_range(0..f.p());
        if let Some(y) = f.sqrt(e.rhs(f, x)) {
            if y != 0 {
                return (x, y);
            }
        }
    }
}

/// `#E(F_p)` by baby-step giant-step over the Hasse interval (Mestre
/// style), confirmed on three further random points; `None` if ambiguous.
fn count_points(f: &PrimeField, e: &ShortWeierstrass, rng: &mut StdRng) -> Option<u64> {
    let p = f.p();
    let w = (2.0 * (p as f64).sqrt()).ceil() as u64 + 1;
    let low = p + 1 - w;
    for _ in 0..4 {
        let pt = Some(random_point(f, e, rng));
        let q = e.mul(f, pt, low as u128);
        let m = ((2 * w + 1) as f64).sqrt().ceil() as u64 + 1;
        let mut baby: HashMap<WPoint, u64> = HashMap::with_capacity(m as usize);
        let mut cur: WPoint = None;
        for i in 0..m {
            baby.entry(cur).or_insert(i);
            cur = e.add(f, cur, pt);
        }
        let step = e.neg(f, cur);
        let mut g = e.neg(f, q);
        let mut sols = Vec::new();
        for k in 0..=m {
            if let Some(&i) = baby.get(&g) {
                let jj = k * m + i;
                if jj <= 2 * w {
                    sols.push(jj);
                }
            }
            g = e.add(f, g, step);
        }
        sols.sort_unstable();
        sols.dedup();
        if sols.len() != 1 {
            continue;
        }
        let n = low + sols[0];
        if (0..3).all(|_| e.mul(f, Some(random_point(f, e, rng)), n as u128).is_none()) {
            return Some(n);
        }
    }
    None
}

/// The largest prime factor `c` of `n` when `n / c` has only prime factors
/// below 4096: `(h, c)`.
fn smooth_split(n: u64) -> Option<(u64, u64)> {
    let mut c = n;
    let mut d = 2u64;
    while d < 4096 && d * d <= c {
        while c.is_multiple_of(d) && c > d {
            c /= d;
        }
        d += 1;
    }
    is_prime_u64(c).then_some((n / c, c))
}

/// Rational neighbours of `e` by degree `ell` (one per kernel), with the
/// codomain models; the count of root-finding calls is the caller's.
fn neighbours(f: &PrimeField, e: &ShortWeierstrass, ell: u64) -> Vec<SmallIsogeny> {
    kernel_representatives(f, e, ell)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|x0| SmallIsogeny::new(f, e, ell, x0).ok())
        .collect()
}

// ------------------------------------------------------------- the class ----

struct Node {
    e: ShortWeierstrass,
    j: u64,
    parent: usize,
    depth: u32,
    /// Product of the degrees from `E₀`.
    deg: u64,
    /// `ψ_i(P₀)`, a generator of the order-`r` subgroup of this curve.
    gen: (u64, u64),
    /// Certified dual of the edge into this node: this curve → parent.
    dual: Option<SmallIsogeny>,
    ell: u64,
}

struct Class {
    f: PrimeField,
    p: u64,
    e0: ShortWeierstrass,
    n: u64,
    r: u64,
    h: u64,
    nodes: Vec<Node>,
    curves_tried: u64,
    /// Neighbour enumerations made while building, per ℓ.
    enumerations: [u64; 4],
    /// Edges found (to a new curve) per ℓ.
    edges: [u64; 4],
    /// Certifications done: `dual∘φ = [ℓ]` on a point of order > 4ℓ² (by
    /// `SmallIsogeny::dual`) and `dual(φ(P_i)) = [ℓ]P_parent`.
    duals_certified: u64,
    /// Edges dropped because `SmallIsogeny::dual` has no rational kernel
    /// abscissas (`ℓ ≥ 5` with `p ≢ ±1 mod ℓ`); transport needs a dual.
    edges_without_dual: [u64; 4],
}

fn build_class(rbits: u32, seed: u64, want: usize) -> Class {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut tried = 0u64;
    loop {
        tried += 1;
        let p = loop {
            let c = rng.gen_range((1u64 << (rbits + 4))..(1u64 << (rbits + 8))) | 1;
            if is_prime_u64(c) {
                break c;
            }
        };
        let f = PrimeField::new(p).expect("prime");
        let Ok(e) = ShortWeierstrass::new(&f, rng.gen_range(1..p), rng.gen_range(1..p)) else {
            continue;
        };
        let Some(n) = count_points(&f, &e, &mut rng) else {
            continue;
        };
        let Some((h, r)) = smooth_split(n) else {
            continue;
        };
        if r < (1u64 << rbits) || r >= (1u64 << (rbits + 1)) || h % r == 0 {
            continue;
        }
        // Every degree must have a rational kernel at E₀.
        if ELLS
            .iter()
            .any(|&l| kernel_representatives(&f, &e, l).unwrap().is_empty())
        {
            continue;
        }
        let g0 = loop {
            let q = Some(random_point(&f, &e, &mut rng));
            if let Some(g) = e.mul(&f, q, h as u128) {
                assert!(e.mul(&f, Some(g), r as u128).is_none());
                break g;
            }
        };
        let mut class = Class {
            p,
            e0: e,
            n,
            r,
            h,
            nodes: vec![Node {
                e,
                j: e.j_invariant(&f),
                parent: 0,
                depth: 0,
                deg: 1,
                gen: g0,
                dual: None,
                ell: 1,
            }],
            curves_tried: tried,
            enumerations: [0; 4],
            edges: [0; 4],
            duals_certified: 0,
            edges_without_dual: [0; 4],
            f,
        };
        let f = &class.f;
        let mut seen: HashSet<u64> = HashSet::new();
        seen.insert(class.nodes[0].j);
        let mut qi = 0;
        'bfs: while qi < class.nodes.len() {
            let (cur, cur_gen, cur_deg, cur_depth) = {
                let nd = &class.nodes[qi];
                (nd.e, nd.gen, nd.deg, nd.depth)
            };
            for (li, &ell) in ELLS.iter().enumerate() {
                class.enumerations[li] += 1;
                for phi in neighbours(f, &cur, ell) {
                    let cod = *phi.codomain();
                    let jc = cod.j_invariant(f);
                    if seen.contains(&jc) || jc == 0 || jc == 1728 {
                        continue;
                    }
                    let Ok(dual) = phi.dual(f) else {
                        class.edges_without_dual[li] += 1;
                        continue;
                    };
                    let gen = phi.eval(f, Some(cur_gen)).expect("generator image");
                    assert!(cod.contains(f, Some(gen)));
                    // dual(φ(P)) = [ℓ]P on this generator as well.
                    assert_eq!(
                        dual.eval(f, Some(gen)),
                        cur.mul(f, Some(cur_gen), ell as u128)
                    );
                    class.duals_certified += 1;
                    class.edges[li] += 1;
                    seen.insert(jc);
                    class.nodes.push(Node {
                        e: cod,
                        j: jc,
                        parent: qi,
                        depth: cur_depth + 1,
                        deg: mulmod(cur_deg, ell, r),
                        gen,
                        dual: Some(dual),
                        ell,
                    });
                    if class.nodes.len() >= want {
                        break 'bfs;
                    }
                }
            }
            qi += 1;
        }
        return class;
    }
}

/// Map `q` on node `i` to `E₀` along the duals; returns the image.
fn transport<C: OpCounter>(
    class: &Class,
    f: &PrimeField<C>,
    i: usize,
    q: (u64, u64),
) -> (u64, u64) {
    let mut cur = Some(q);
    let mut k = i;
    while k != 0 {
        let nd = &class.nodes[k];
        cur = nd.dual.as_ref().unwrap().eval(f, cur);
        k = nd.parent;
    }
    cur.expect("an order-r point stays finite")
}

fn pp(x: (u64, u64)) -> PrimePoint {
    PrimePoint::affine(x.0, x.1)
}

fn mean_se(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    let var = v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n - 1.0).max(1.0);
    (m, (var / n).sqrt())
}

/// Kuhn–Struik on `{P, −P}`: `Σ_{i<L} C(2i, i)·4^{−i}·√(π/4)` in `S`.
fn ks_reference(l: usize) -> f64 {
    let mut c = 1.0f64;
    let mut s = 0.0;
    for i in 0..l {
        if i > 0 {
            c *= (2 * i - 1) as f64 / (2 * i) as f64;
        }
        s += c;
    }
    s * (std::f64::consts::PI / 4.0).sqrt()
}

fn batch_mode(args: &[String]) {
    let rbits: u32 = arg(args, "--rbits", 28);
    let trials: usize = arg(args, "--trials", 20);
    let seed: u64 = arg(args, "--seed", 1);
    let out_path: String = arg(args, "--out", String::from("r6_2_batch.json"));
    let ls: Vec<usize> = arg(args, "--ls", String::from("1,4,16,64,256"))
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let lmax = *ls.iter().max().unwrap();
    let t0 = Instant::now();
    let class = build_class(rbits, seed, lmax + 1);
    let build_wall = t0.elapsed().as_secs_f64();
    let f = &class.f;
    let r = class.r;
    let nn = class.nodes.len();
    eprintln!(
        "class: p={} r={} (2^{:.2}) h={} nodes={} edges={:?} enumerations={:?} tried={} ({:.1}s)",
        class.p,
        r,
        (r as f64).log2(),
        class.h,
        nn,
        class.edges,
        class.enumerations,
        class.curves_tried,
        build_wall
    );
    let g = PrimeCurve {
        p: class.p,
        a: class.e0.a,
        b: class.e0.b,
    };
    let p0 = pp(class.nodes[0].gen);
    let cap = rho_cap(r, 64.0);
    let sqrt_r = (r as f64).sqrt();
    let counted = PrimeField::counted(class.p).unwrap();
    let mut records = Vec::new();
    let mut verified_logs = 0u64;
    let mut summary = Vec::new();
    for &l in &ls {
        let (mut sb, mut sc, mut ss, mut ratios) = (vec![], vec![], vec![], vec![]);
        let (mut tm, mut ti) = (0u64, 0u64);
        for t in 0..trials {
            let tseed = splitmix(seed ^ splitmix((l as u64) << 32 | t as u64));
            let mut krng = StdRng::seed_from_u64(tseed);
            let rho_seed = splitmix(tseed ^ 0x5eed);
            // ---- B: keys on E_1 … E_L (cyclically if the class is smaller).
            let mut planted = Vec::with_capacity(l);
            let mut on_e0 = Vec::with_capacity(l);
            let start = counted.tally();
            for i in 0..l {
                let node = 1 + i % (nn - 1);
                let k = krng.gen_range(1..r);
                let nd = &class.nodes[node];
                let q = nd.e.mul(f, Some(nd.gen), k as u128).unwrap();
                planted.push((node, k, q));
                on_e0.push(pp(transport(&class, &counted, node, q)));
            }
            let tr = counted.tally().since(start);
            tm += tr.mul;
            ti += tr.inv;
            let res_b = rho_batch_with(
                &g,
                &NegationClasses { r },
                p0,
                &on_e0,
                r,
                rho_seed,
                cap,
                RhoWalk::negation(),
            );
            for ((node, k, q), t_res) in planted.iter().zip(&res_b.per_target) {
                let m = t_res.recovered.expect("B target solved");
                let d = class.nodes[*node].deg;
                let kk = mulmod(m, invmod(d, r), r);
                let nd = &class.nodes[*node];
                // Verified on its original curve E_i, and against the plant.
                assert_eq!(nd.e.mul(f, Some(nd.gen), kk as u128), Some(*q));
                assert_eq!(kk, *k);
                verified_logs += 1;
            }
            // ---- C: L keys on E₀, same algorithm seed.
            let keys_c: Vec<u64> = (0..l).map(|_| krng.gen_range(1..r)).collect();
            let targets_c: Vec<PrimePoint> = keys_c
                .iter()
                .map(|&k| {
                    let mut o = GroupOps::default();
                    g.mul(&mut o, p0, k)
                })
                .collect();
            let res_c = rho_batch_with(
                &g,
                &NegationClasses { r },
                p0,
                &targets_c,
                r,
                rho_seed,
                cap,
                RhoWalk::negation(),
            );
            for ((k, q), t_res) in keys_c.iter().zip(&targets_c).zip(&res_c.per_target) {
                let m = t_res.recovered.expect("C target solved");
                // Verified on E₀ with the isogeny_eval arithmetic, and
                // against the plant.
                assert_eq!(
                    class.e0.mul(f, Some(class.nodes[0].gen), m as u128),
                    Some((q.x, q.y))
                );
                assert_eq!(m, *k);
                verified_logs += 1;
            }
            // ---- S: one key, the single-target walk (ecbench rho.negation).
            let ks = krng.gen_range(1..r);
            let qs = {
                let mut o = GroupOps::default();
                g.mul(&mut o, p0, ks)
            };
            let res_s = rho_walk_with(
                &g,
                &NegationClasses { r },
                p0,
                qs,
                r,
                rho_seed,
                cap,
                RhoWalk::negation(),
            );
            assert_eq!(res_s.recovered, Some(ks));
            verified_logs += 1;
            let (b, c, s) = (res_b.gae / sqrt_r, res_c.gae / sqrt_r, res_s.gae / sqrt_r);
            sb.push(b);
            sc.push(c);
            ss.push(s);
            ratios.push(b / c);
            records.push(json!({
                "L": l, "trial": t, "rho_seed": rho_seed,
                "B": {"gae": res_b.gae, "adds": res_b.group_ops.adds, "doubles": res_b.group_ops.doubles,
                      "field_ops": {"M": res_b.group_ops.field_ops.muls, "S": res_b.group_ops.field_ops.sqrs, "I": res_b.group_ops.field_ops.invs},
                      "setup_gae": res_b.setup_ops.gae(),
                      "solved_on_earlier_trail": res_b.counters.get("solved_on_an_earlier_trail"),
                      "per_target_gae": res_b.per_target.iter().map(|x| x.gae).collect::<Vec<_>>(),
                      "transport": {"M": tr.mul, "I": tr.inv},
                      "nodes": planted.iter().map(|x| x.0).collect::<Vec<_>>()},
                "C": {"gae": res_c.gae, "adds": res_c.group_ops.adds, "doubles": res_c.group_ops.doubles,
                      "field_ops": {"M": res_c.group_ops.field_ops.muls, "S": res_c.group_ops.field_ops.sqrs, "I": res_c.group_ops.field_ops.invs},
                      "setup_gae": res_c.setup_ops.gae(),
                      "solved_on_earlier_trail": res_c.counters.get("solved_on_an_earlier_trail"),
                      "per_target_gae": res_c.per_target.iter().map(|x| x.gae).collect::<Vec<_>>()},
                "S_single": {"gae": res_s.gae},
            }));
        }
        let (mb, eb) = mean_se(&sb);
        let (mc, ec) = mean_se(&sc);
        let (ms, es) = mean_se(&ss);
        eprintln!(
            "L={l:4} B={mb:.3}±{eb:.3} C={mc:.3}±{ec:.3} single={ms:.3}±{es:.3} KS={:.3} transport/target={:.1}M+{:.1}I ({:.0}s)",
            ks_reference(l),
            tm as f64 / (trials * l) as f64,
            ti as f64 / (trials * l) as f64,
            t0.elapsed().as_secs_f64()
        );
        summary.push(json!({
            "L": l, "trials": trials,
            "S_B": sb, "S_C": sc, "S_single": ss,
            "mean_B": mb, "se_B": eb, "mean_C": mc, "se_C": ec, "mean_single": ms, "se_single": es,
            "ks_reference": ks_reference(l),
            "transport_M_per_target": tm as f64 / (trials * l) as f64,
            "transport_I_per_target": ti as f64 / (trials * l) as f64,
        }));
    }
    let depth_max = class.nodes.iter().map(|n| n.depth).max().unwrap_or(0);
    let ell_count: Vec<u64> = ELLS
        .iter()
        .map(|&l| class.nodes.iter().filter(|n| n.ell == l).count() as u64)
        .collect();
    let out = json!({
        "schema": "r6_2.batch/v1",
        "args": args[1..].to_vec(),
        "base_curve": {"p": class.p, "a": class.e0.a, "b": class.e0.b, "order": class.n, "r": r,
                        "h": class.h, "rbits_actual": (r as f64).log2(), "generator": class.nodes[0].gen},
        "class": {"nodes": nn, "depth_max": depth_max, "edges_by_ell": {"2": ell_count[0], "3": ell_count[1], "5": ell_count[2], "7": ell_count[3]},
                  "enumerations_by_ell": class.enumerations, "duals_certified": class.duals_certified,
                  "edges_without_dual_by_ell": class.edges_without_dual,
                  "curves_tried": class.curves_tried,
                  "curves": class.nodes.iter().map(|n| json!([n.e.a, n.e.b, n.parent, n.ell, n.depth])).collect::<Vec<_>>()},
        "rho": {"walk": "ic_boundary::rho_batch_with(NegationClasses, RhoWalk::negation()) = ecbench rho.multitarget_pooled; single: rho_walk_with(..) = ecbench rho.negation",
                "cap": cap},
        "summary": summary,
        "trials": records,
        "verified_logs": verified_logs,
        "wall_seconds_contended": t0.elapsed().as_secs_f64(),
    });
    std::fs::write(&out_path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    eprintln!("verified logs: {verified_logs}; wrote {out_path}");
}

// ------------------------------------------------------------ path finding ----
//
// `neighbour_js`, `component` and `bidirectional` below are the search as
// first committed (schema v1, `--mode paths-legacy`); the corrected search
// follows them in "certified path finding".

/// One curve of a component, keyed by its j-invariant (the walk is on
/// j-invariants; a model is kept to keep walking).
fn neighbour_js(
    f: &PrimeField,
    e: &ShortWeierstrass,
    counts: &mut [u64; 4],
) -> Vec<(u64, ShortWeierstrass)> {
    let mut out = Vec::new();
    for (li, &ell) in ELLS.iter().enumerate() {
        counts[li] += 1;
        for phi in neighbours(f, e, ell) {
            let cod = *phi.codomain();
            out.push((cod.j_invariant(f), cod));
        }
    }
    out
}

/// The whole component of `e`, up to `cap` curves: j → model.
fn component(
    f: &PrimeField,
    e: ShortWeierstrass,
    cap: usize,
) -> Option<(Vec<u64>, HashMap<u64, ShortWeierstrass>)> {
    let mut models = HashMap::new();
    let j0 = e.j_invariant(f);
    models.insert(j0, e);
    let mut order = vec![j0];
    let mut q = VecDeque::from([e]);
    let mut c = [0u64; 4];
    while let Some(cur) = q.pop_front() {
        for (j, m) in neighbour_js(f, &cur, &mut c) {
            if let std::collections::hash_map::Entry::Vacant(v) = models.entry(j) {
                v.insert(m);
                order.push(j);
                q.push_back(m);
                if order.len() > cap {
                    return None;
                }
            }
        }
    }
    Some((order, models))
}

/// Bidirectional breadth-first search from `a` to `b`, level by level,
/// expanding the smaller frontier.  Returns (path length, enumerations).
fn bidirectional(
    f: &PrimeField,
    a: ShortWeierstrass,
    b: ShortWeierstrass,
    counts: &mut [u64; 4],
) -> Option<u32> {
    let (ja, jb) = (a.j_invariant(f), b.j_invariant(f));
    if ja == jb {
        return Some(0);
    }
    let mut dist = [HashMap::new(), HashMap::new()];
    dist[0].insert(ja, 0u32);
    dist[1].insert(jb, 0u32);
    let mut front = [vec![a], vec![b]];
    loop {
        let side = if front[0].len() <= front[1].len() {
            0
        } else {
            1
        };
        if front[side].is_empty() {
            return None;
        }
        let mut next = Vec::new();
        let mut best: Option<u32> = None;
        for cur in std::mem::take(&mut front[side]) {
            let dc = dist[side][&cur.j_invariant(f)];
            for (j, m) in neighbour_js(f, &cur, counts) {
                if let Some(&d2) = dist[1 - side].get(&j) {
                    let len = dc + 1 + d2;
                    best = Some(best.map_or(len, |b| b.min(len)));
                }
                if let std::collections::hash_map::Entry::Vacant(v) = dist[side].entry(j) {
                    v.insert(dc + 1);
                    next.push(m);
                }
            }
        }
        if best.is_some() {
            return best;
        }
        front[side] = next;
    }
}

fn paths_legacy_mode(args: &[String]) {
    let pbits: Vec<u32> = arg(args, "--pbits", String::from("24,28,32"))
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let per_bits: usize = arg(args, "--classes", 6);
    let pairs: usize = arg(args, "--pairs", 10);
    let cap: usize = arg(args, "--cap", 3000);
    let seed: u64 = arg(args, "--seed", 7);
    let out_path: String = arg(args, "--out", String::from("r6_2_paths.json"));
    let t0 = Instant::now();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut classes = Vec::new();
    for &bits in &pbits {
        let mut done = 0;
        let mut skipped_large = 0u64;
        while done < per_bits {
            let p = loop {
                let c = rng.gen_range((1u64 << (bits - 1))..(1u64 << bits)) | 1;
                if is_prime_u64(c) {
                    break c;
                }
            };
            let f = PrimeField::new(p).unwrap();
            let Ok(e) = ShortWeierstrass::new(&f, rng.gen_range(1..p), rng.gen_range(1..p)) else {
                continue;
            };
            let j = e.j_invariant(&f);
            if j == 0 || j == 1728 {
                continue;
            }
            let Some(n) = count_points(&f, &e, &mut rng) else {
                continue;
            };
            let Some((h, r)) = smooth_split(n) else {
                continue;
            };
            if ELLS
                .iter()
                .filter(|&&l| !kernel_representatives(&f, &e, l).unwrap().is_empty())
                .count()
                < 2
            {
                continue;
            }
            let Some((order, models)) = component(&f, e, cap) else {
                skipped_large += 1;
                continue;
            };
            if order.len() < 8 {
                continue;
            }
            let size = order.len();
            let mut runs = Vec::new();
            for _ in 0..pairs {
                let a = models[&order[rng.gen_range(0..size)]];
                let b = models[&order[rng.gen_range(0..size)]];
                let mut c = [0u64; 4];
                let len = bidirectional(&f, a, b, &mut c).expect("same component");
                runs.push(
                    json!({"length": len, "enumerations": c.iter().sum::<u64>(), "by_ell": c}),
                );
            }
            let mean_enum = runs
                .iter()
                .map(|v| v["enumerations"].as_u64().unwrap() as f64)
                .sum::<f64>()
                / pairs as f64;
            eprintln!(
                "pbits={bits} p={p} r=2^{:.1} h={h} component={size} mean enumerations/pair={mean_enum:.1} ({:.0}s)",
                (r as f64).log2(),
                t0.elapsed().as_secs_f64()
            );
            classes.push(json!({
                "pbits": bits, "p": p, "a": e.a, "b": e.b, "order": n, "r": r, "h": h,
                "component_size": size, "pairs": runs, "mean_enumerations": mean_enum,
            }));
            done += 1;
        }
        eprintln!("pbits={bits}: {skipped_large} components over the cap {cap} skipped");
        classes.push(json!({"pbits": bits, "skipped_over_cap": skipped_large}));
    }
    let out = json!({
        "schema": "r6_2.paths/v1",
        "args": args[1..].to_vec(),
        "cap": cap,
        "classes": classes,
        "wall_seconds_contended": t0.elapsed().as_secs_f64(),
    });
    std::fs::write(&out_path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    eprintln!("wrote {out_path}");
}

// ------------------------------------------------- certified path finding ----

/// Which edges a search follows out of a curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EdgeSet {
    /// Every outgoing edge `SmallIsogeny::new` realises (the pre-fix
    /// search). Used on both sides it treats `u → v` as `v → u`.
    Outgoing,
    /// Only edges whose reverse edge `SmallIsogeny::dual` certifies, so the
    /// edge set is symmetric and an outgoing edge of `b` is a real
    /// predecessor edge into `b`.
    Certified,
}

/// Counted work of one search, per ℓ in [`ELLS`] order.
#[derive(Clone, Copy, Debug, Default)]
struct Work {
    /// Neighbour enumerations of the search (one per expanded (curve, ℓ)).
    enumerations: [u64; 4],
    /// `SmallIsogeny::dual` calls made to certify an edge: each re-enumerates
    /// the kernels at the codomain and builds a certification point, so it
    /// is at least one further (curve, ℓ) enumeration.
    certifications: [u64; 4],
}

impl Work {
    fn search(&self) -> u64 {
        self.enumerations.iter().sum()
    }
    fn cert(&self) -> u64 {
        self.certifications.iter().sum()
    }
}

struct Edge {
    cod: ShortWeierstrass,
    j: u64,
    ell: u64,
}

/// The edges out of `e` in `set`.
fn edges_of(f: &PrimeField, e: &ShortWeierstrass, set: EdgeSet, w: &mut Work) -> Vec<Edge> {
    let mut out = Vec::new();
    for (li, &ell) in ELLS.iter().enumerate() {
        w.enumerations[li] += 1;
        for phi in neighbours(f, e, ell) {
            if set == EdgeSet::Certified {
                w.certifications[li] += 1;
                if phi.dual(f).is_err() {
                    continue;
                }
            }
            let cod = *phi.codomain();
            out.push(Edge {
                cod,
                j: cod.j_invariant(f),
                ell,
            });
        }
    }
    out
}

/// The whole component of `e` under `set`, up to `cap` curves: j-invariants
/// in discovery order and j → model.
fn component_in(
    f: &PrimeField,
    e: ShortWeierstrass,
    cap: usize,
    set: EdgeSet,
) -> Option<(Vec<u64>, HashMap<u64, ShortWeierstrass>)> {
    let mut models = HashMap::new();
    let j0 = e.j_invariant(f);
    models.insert(j0, e);
    let mut order = vec![j0];
    let mut q = VecDeque::from([e]);
    let mut w = Work::default();
    while let Some(cur) = q.pop_front() {
        for edge in edges_of(f, &cur, set, &mut w) {
            if let Entry::Vacant(v) = models.entry(edge.j) {
                v.insert(edge.cod);
                order.push(edge.j);
                q.push_back(edge.cod);
                if order.len() > cap {
                    return None;
                }
            }
        }
    }
    Some((order, models))
}

struct TreeNode {
    d: u32,
    model: ShortWeierstrass,
    /// `(parent j, ℓ)`: the edge `parent → this` that discovered it.
    parent: Option<(u64, u64)>,
}

/// A route as `(ℓ, j of the codomain)` hops from `a` towards `b`.
type Route = Vec<(u64, u64)>;

struct Found {
    len: u32,
    route: Route,
}

/// Bidirectional breadth-first search from `a` to `b`, level by level,
/// expanding the smaller frontier, over the edge set `set`, with the route
/// that the meeting implies. The `a` side follows edges forward. The `b`
/// side follows outgoing edges of `b`'s tree and *reads each as the reverse
/// hop* (child → parent), which is a real forward hop only if the edge set
/// is symmetric: [`EdgeSet::Certified`] makes that so (the reverse edge is
/// constructed by `SmallIsogeny::dual`), [`EdgeSet::Outgoing`] does not.
fn search(
    f: &PrimeField,
    a: ShortWeierstrass,
    b: ShortWeierstrass,
    set: EdgeSet,
    w: &mut Work,
) -> Option<Found> {
    let (ja, jb) = (a.j_invariant(f), b.j_invariant(f));
    if ja == jb {
        return Some(Found {
            len: 0,
            route: Vec::new(),
        });
    }
    let mut tree: [HashMap<u64, TreeNode>; 2] = [HashMap::new(), HashMap::new()];
    tree[0].insert(
        ja,
        TreeNode {
            d: 0,
            model: a,
            parent: None,
        },
    );
    tree[1].insert(
        jb,
        TreeNode {
            d: 0,
            model: b,
            parent: None,
        },
    );
    let mut front: [Vec<u64>; 2] = [vec![ja], vec![jb]];
    loop {
        let side = if front[0].len() <= front[1].len() {
            0
        } else {
            1
        };
        if front[side].is_empty() {
            return None;
        }
        let mut next = Vec::new();
        // (length, expanded j, edge ℓ, neighbour j)
        let mut best: Option<(u32, u64, u64, u64)> = None;
        for cur_j in std::mem::take(&mut front[side]) {
            let (dc, cur) = {
                let n = &tree[side][&cur_j];
                (n.d, n.model)
            };
            for edge in edges_of(f, &cur, set, w) {
                if let Some(o) = tree[1 - side].get(&edge.j) {
                    let len = dc + 1 + o.d;
                    if best.is_none_or(|b| len < b.0) {
                        best = Some((len, cur_j, edge.ell, edge.j));
                    }
                }
                if let Entry::Vacant(v) = tree[side].entry(edge.j) {
                    v.insert(TreeNode {
                        d: dc + 1,
                        model: edge.cod,
                        parent: Some((cur_j, edge.ell)),
                    });
                    next.push(edge.j);
                }
            }
        }
        if let Some((len, cur_j, ell, nb_j)) = best {
            // Forward hops root(a) → x along the a tree.
            let chain_a = |mut x: u64| {
                let mut v = Vec::new();
                while let Some((pj, l)) = tree[0][&x].parent {
                    v.push((l, x));
                    x = pj;
                }
                v.reverse();
                v
            };
            // Forward hops x → root(b): each tree-1 edge `parent → child`
            // is read as the hop `child → parent`.
            let chain_b = |mut x: u64| {
                let mut v = Vec::new();
                while let Some((pj, l)) = tree[1][&x].parent {
                    v.push((l, pj));
                    x = pj;
                }
                v
            };
            let route: Route = if side == 0 {
                let mut r = chain_a(cur_j);
                r.push((ell, nb_j));
                r.extend(chain_b(nb_j));
                r
            } else {
                let mut r = chain_a(nb_j);
                r.push((ell, cur_j));
                r.extend(chain_b(cur_j));
                r
            };
            debug_assert_eq!(route.len() as u32, len);
            return Some(Found { len, route });
        }
        front[side] = next;
    }
}

/// How a route failed forward validation.
#[derive(Debug, PartialEq, Eq)]
enum RouteError {
    /// No degree-ℓ rational-kernel isogeny out of the chained curve reaches
    /// the hop's j-invariant: the hop is not an executable edge.
    NoForwardEdge { hop: usize },
    /// The hops ended on a curve whose j-invariant is not `b`'s.
    WrongEndpoint,
}

/// Re-validate `route` from `a` in the forward direction: at each hop
/// re-enumerate the degree-ℓ kernels of the *chained* curve (the actual
/// codomain of the previous hop, not a model remembered from the search) and
/// take one whose codomain has the hop's j-invariant. Returns whether the
/// route ends on `b` only up to quadratic twist (`j` equal, no
/// `F_p`-isomorphism). The enumerations are added to `w.enumerations`.
fn validate_route(
    f: &PrimeField,
    a: ShortWeierstrass,
    b: ShortWeierstrass,
    route: &[(u64, u64)],
    w: &mut Work,
) -> Result<bool, RouteError> {
    let mut cur = a;
    for (hop, &(ell, to_j)) in route.iter().enumerate() {
        let li = ELLS.iter().position(|&l| l == ell).expect("ℓ in ELLS");
        w.enumerations[li] += 1;
        let step = neighbours(f, &cur, ell)
            .into_iter()
            .map(|phi| *phi.codomain())
            .find(|c| c.j_invariant(f) == to_j)
            .ok_or(RouteError::NoForwardEdge { hop })?;
        cur = step;
    }
    if cur.j_invariant(f) != b.j_invariant(f) {
        return Err(RouteError::WrongEndpoint);
    }
    Ok(isomorphisms(f, &cur, &b).is_empty())
}

fn work_json(w: &Work) -> serde_json::Value {
    json!({"by_ell": w.enumerations, "cert_by_ell": w.certifications})
}

/// The curves of one component run: where they come from.
struct PathClass {
    pbits: u32,
    p: u64,
    e: ShortWeierstrass,
    n: u64,
    /// Component size of the pre-fix (outgoing-edge) enumeration when the
    /// class is a replay of a frozen v1 record.
    v1_component_size: Option<u64>,
}

/// Measure one class: the certified component, then `pairs` random pairs,
/// each searched by the corrected search and by the pre-fix search.
fn measure_class(
    f: &PrimeField,
    class: &PathClass,
    order: &[u64],
    models: &HashMap<u64, ShortWeierstrass>,
    pairs: usize,
    rng: &mut StdRng,
) -> serde_json::Value {
    let size = order.len();
    let (h, r) = smooth_split(class.n).unwrap_or((class.n, 1));
    let mut runs = Vec::new();
    let (mut tot_search, mut tot_cert, mut tot_val) = (0u64, 0u64, 0u64);
    let (mut legacy_bad, mut legacy_shorter) = (0u64, 0u64);
    for _ in 0..pairs {
        let a = models[&order[rng.gen_range(0..size)]];
        let b = models[&order[rng.gen_range(0..size)]];
        // Corrected search over the symmetric certified edges.
        let mut w = Work::default();
        let found = search(f, a, b, EdgeSet::Certified, &mut w)
            .expect("a and b share a certified component");
        let mut wv = Work::default();
        let validated = validate_route(f, a, b, &found.route, &mut wv);
        assert!(
            validated.is_ok(),
            "a returned route failed forward validation: {validated:?}"
        );
        // The pre-fix search on the same pair.
        let mut wl = Work::default();
        let legacy = search(f, a, b, EdgeSet::Outgoing, &mut wl);
        let legacy_json = match &legacy {
            Some(l) => {
                let mut wlv = Work::default();
                let v = validate_route(f, a, b, &l.route, &mut wlv);
                if v.is_err() {
                    legacy_bad += 1;
                }
                if l.len < found.len {
                    legacy_shorter += 1;
                }
                json!({"length": l.len, "route_executable": v.is_ok(),
                       "failure": v.as_ref().err().map(|e| format!("{e:?}")),
                       "enumerations": wl.search(), "by_ell": wl.enumerations})
            }
            None => json!({"length": null, "enumerations": wl.search(), "by_ell": wl.enumerations}),
        };
        tot_search += w.search();
        tot_cert += w.cert();
        tot_val += wv.search();
        runs.push(json!({
            "length": found.len,
            "route_validated_forward": true,
            "ends_on_twist_of_b": validated.unwrap_or(false),
            "enumerations": w.search(),
            "certifications": w.cert(),
            "validation_enumerations": wv.search(),
            "total_enumerations": w.search() + w.cert() + wv.search(),
            "work": work_json(&w),
            "route": found.route.iter().map(|&(l, j)| json!([l, j])).collect::<Vec<_>>(),
            "legacy": legacy_json,
        }));
    }
    let pf = pairs as f64;
    json!({
        "pbits": class.pbits, "p": class.p, "a": class.e.a, "b": class.e.b,
        "order": class.n, "r": r, "h": h,
        "component_size": size,
        "v1_component_size": class.v1_component_size,
        "pairs": runs,
        "mean_enumerations": tot_search as f64 / pf,
        "mean_certifications": tot_cert as f64 / pf,
        "mean_validation_enumerations": tot_val as f64 / pf,
        "mean_total_enumerations": (tot_search + tot_cert + tot_val) as f64 / pf,
        "legacy_routes_not_executable": legacy_bad,
        "legacy_shorter_than_corrected": legacy_shorter,
    })
}

/// `--mode paths`: the corrected search. Fresh classes follow the frozen
/// run's protocol (`--pbits/--classes/--pairs/--cap/--seed`); with
/// `--replay <paths.json>` the classes are exactly the curves recorded in
/// that file (a v1 record), so the corrected result is paired with it.
fn paths_mode(args: &[String]) {
    let pbits: Vec<u32> = arg(args, "--pbits", String::from("24,28,32"))
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let per_bits: usize = arg(args, "--classes", 6);
    let pairs: usize = arg(args, "--pairs", 10);
    let cap: usize = arg(args, "--cap", 3000);
    let seed: u64 = arg(args, "--seed", 7);
    let replay: String = arg(args, "--replay", String::new());
    let out_path: String = arg(args, "--out", String::from("r6_2_paths_v2.json"));
    let t0 = Instant::now();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut classes = Vec::new();
    if !replay.is_empty() {
        let v1: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&replay).expect("read --replay file"))
                .expect("--replay is JSON");
        let recorded = v1["classes"].as_array().expect("v1 classes");
        for (idx, c) in recorded.iter().filter(|c| c.get("p").is_some()).enumerate() {
            let p = c["p"].as_u64().unwrap();
            let f = PrimeField::new(p).unwrap();
            let e = ShortWeierstrass::new(&f, c["a"].as_u64().unwrap(), c["b"].as_u64().unwrap())
                .unwrap();
            let class = PathClass {
                pbits: c["pbits"].as_u64().unwrap() as u32,
                p,
                e,
                n: c["order"].as_u64().unwrap(),
                v1_component_size: c["component_size"].as_u64(),
            };
            let mut crng = StdRng::seed_from_u64(splitmix(seed ^ splitmix(idx as u64)));
            let Some((order, models)) = component_in(&f, e, usize::MAX, EdgeSet::Certified) else {
                unreachable!()
            };
            let rec = measure_class(&f, &class, &order, &models, pairs, &mut crng);
            eprintln!(
                "replay pbits={} p={p} v1 component={:?} certified component={} mean enumerations/pair={:.1} (+{:.1} cert) ({:.0}s)",
                class.pbits,
                class.v1_component_size,
                order.len(),
                rec["mean_enumerations"].as_f64().unwrap(),
                rec["mean_certifications"].as_f64().unwrap(),
                t0.elapsed().as_secs_f64()
            );
            classes.push(rec);
        }
    } else {
        for &bits in &pbits {
            let mut done = 0;
            let mut skipped_large = 0u64;
            let mut skipped_small = 0u64;
            while done < per_bits {
                let p = loop {
                    let c = rng.gen_range((1u64 << (bits - 1))..(1u64 << bits)) | 1;
                    if is_prime_u64(c) {
                        break c;
                    }
                };
                let f = PrimeField::new(p).unwrap();
                let Ok(e) = ShortWeierstrass::new(&f, rng.gen_range(1..p), rng.gen_range(1..p))
                else {
                    continue;
                };
                let j = e.j_invariant(&f);
                if j == 0 || j == 1728 {
                    continue;
                }
                let Some(n) = count_points(&f, &e, &mut rng) else {
                    continue;
                };
                if smooth_split(n).is_none() {
                    continue;
                }
                if ELLS
                    .iter()
                    .filter(|&&l| !kernel_representatives(&f, &e, l).unwrap().is_empty())
                    .count()
                    < 2
                {
                    continue;
                }
                let Some((order, models)) = component_in(&f, e, cap, EdgeSet::Certified) else {
                    skipped_large += 1;
                    continue;
                };
                if order.len() < 8 {
                    skipped_small += 1;
                    continue;
                }
                let class = PathClass {
                    pbits: bits,
                    p,
                    e,
                    n,
                    v1_component_size: None,
                };
                let rec = measure_class(&f, &class, &order, &models, pairs, &mut rng);
                eprintln!(
                    "pbits={bits} p={p} component={} mean enumerations/pair={:.1} (+{:.1} cert) ({:.0}s)",
                    order.len(),
                    rec["mean_enumerations"].as_f64().unwrap(),
                    rec["mean_certifications"].as_f64().unwrap(),
                    t0.elapsed().as_secs_f64()
                );
                classes.push(rec);
                done += 1;
            }
            eprintln!(
                "pbits={bits}: {skipped_large} components over the cap {cap} skipped, {skipped_small} under 8 curves skipped"
            );
            classes.push(json!({"pbits": bits, "skipped_over_cap": skipped_large,
                                "skipped_under_8": skipped_small}));
        }
    }
    let out = json!({
        "schema": "r6_2.paths/v2",
        "args": args[1..].to_vec(),
        "cap": cap,
        "edge_set": "certified: u -> v kept only when SmallIsogeny::dual certifies v -> u",
        "classes": classes,
        "wall_seconds_contended": t0.elapsed().as_secs_f64(),
    });
    std::fs::write(&out_path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    eprintln!("wrote {out_path}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode: String = arg(&args, "--mode", String::from("batch"));
    match mode.as_str() {
        "batch" => batch_mode(&args),
        "paths" => paths_mode(&args),
        "paths-legacy" => paths_legacy_mode(&args),
        other => panic!("unknown mode {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Primes in `lo..hi` by residue class: `symmetric` selects
    /// `p ≡ ±1 (mod 5)` and `(mod 7)` (every ℓ ∈ {2, 3, 5, 7} edge has a
    /// dual), otherwise `p ≢ ±1 (mod 5)` and `(mod 7)` (no ℓ = 5, 7 edge has
    /// one).
    fn primes(lo: u64, hi: u64, symmetric: bool) -> Vec<u64> {
        let pm1 = |p: u64, l: u64| p % l == 1 || p % l == l - 1;
        (lo..hi)
            .filter(|&p| is_prime_u64(p))
            .filter(|&p| {
                if symmetric {
                    pm1(p, 5) && pm1(p, 7)
                } else {
                    !pm1(p, 5) && !pm1(p, 7)
                }
            })
            .collect()
    }

    fn random_curve(f: &PrimeField, rng: &mut StdRng) -> Option<ShortWeierstrass> {
        let p = f.p();
        let e = ShortWeierstrass::new(f, rng.gen_range(1..p), rng.gen_range(1..p)).ok()?;
        let j = e.j_invariant(f);
        (j != 0 && j != 1728).then_some(e)
    }

    /// The asymmetric edge itself: for `p ≢ ±1 (mod 5)` a rational-kernel
    /// degree-5 edge `e → c` exists, `SmallIsogeny::dual` fails, and the
    /// enumerator at `c` has no degree-5 edge back to `j(e)`; the certified
    /// edge set drops it and the outgoing edge set keeps it.
    #[test]
    fn a_degree_five_edge_without_a_reverse_edge_is_excluded_from_the_certified_set() {
        let mut rng = StdRng::seed_from_u64(5);
        let mut found = 0;
        let mut returns_by_another_kernel = 0;
        for p in primes(500, 3000, false).into_iter().take(6) {
            let f = PrimeField::new(p).unwrap();
            for _ in 0..40 {
                let Some(e) = random_curve(&f, &mut rng) else {
                    continue;
                };
                let je = e.j_invariant(&f);
                let mut w = Work::default();
                let certified = edges_of(&f, &e, EdgeSet::Certified, &mut w);
                let outgoing = edges_of(&f, &e, EdgeSet::Outgoing, &mut w);
                for phi in neighbours(&f, &e, 5) {
                    let cod = *phi.codomain();
                    let jc = cod.j_invariant(&f);
                    let back_exists = neighbours(&f, &cod, 5)
                        .iter()
                        .any(|psi| psi.codomain().j_invariant(&f) == je);
                    if phi.dual(&f).is_ok() {
                        continue;
                    }
                    // `dual` failed. When the enumerator at `c` also has no
                    // edge back to `j(e)`, the edge is one-way: the outgoing
                    // set keeps it and the certified set must drop it (a
                    // different dual-certified edge to the same j would
                    // itself be a reverse edge). A cycle that returns to
                    // `j(e)` through another kernel is a real reverse edge,
                    // counted and set aside.
                    if back_exists {
                        returns_by_another_kernel += 1;
                        continue;
                    }
                    assert!(outgoing.iter().any(|x| x.ell == 5 && x.j == jc));
                    assert!(
                        !certified.iter().any(|x| x.ell == 5 && x.j == jc),
                        "certified set kept a one-way edge at p = {p}"
                    );
                    found += 1;
                }
                // Every certified edge has a forward reverse edge.
                for edge in &certified {
                    let back = neighbours(&f, &edge.cod, edge.ell)
                        .iter()
                        .any(|psi| psi.codomain().j_invariant(&f) == je);
                    assert!(back, "certified edge without a reverse edge");
                }
            }
        }
        assert!(
            found > 0,
            "no one-way degree-5 edge found on the test primes \
             ({returns_by_another_kernel} returned by another kernel)"
        );
    }

    /// The control the pre-fix search fails: on primes with `p ≢ ±1`
    /// (mod 5, 7) the outgoing-edge bidirectional search returns routes
    /// that are not executable forward (a meeting `a → c ← b`, or a hop
    /// read backwards), while every route of the corrected search validates
    /// edge by edge.
    #[test]
    fn asymmetric_edges_break_the_outgoing_search_and_not_the_certified_one() {
        let mut rng = StdRng::seed_from_u64(11);
        let (mut legacy_total, mut legacy_bad, mut certified_total) = (0u32, 0u32, 0u32);
        for p in primes(500, 4000, false).into_iter().take(3) {
            let f = PrimeField::new(p).unwrap();
            for _ in 0..8 {
                let Some(e) = random_curve(&f, &mut rng) else {
                    continue;
                };
                // Pairs from the outgoing-edge component, as the pre-fix run drew them.
                let Some((order, models)) = component_in(&f, e, 150, EdgeSet::Outgoing) else {
                    continue;
                };
                if order.len() < 4 {
                    continue;
                }
                for _ in 0..4 {
                    let a = models[&order[rng.gen_range(0..order.len())]];
                    let b = models[&order[rng.gen_range(0..order.len())]];
                    let mut w = Work::default();
                    if let Some(l) = search(&f, a, b, EdgeSet::Outgoing, &mut w) {
                        legacy_total += 1;
                        if validate_route(&f, a, b, &l.route, &mut Work::default()).is_err() {
                            legacy_bad += 1;
                        }
                    }
                    let mut w = Work::default();
                    if let Some(c) = search(&f, a, b, EdgeSet::Certified, &mut w) {
                        certified_total += 1;
                        assert_eq!(c.route.len() as u32, c.len);
                        assert!(
                            validate_route(&f, a, b, &c.route, &mut Work::default()).is_ok(),
                            "certified route failed forward validation (p = {p})"
                        );
                    }
                }
            }
        }
        assert!(certified_total > 0 && legacy_total > 0);
        assert!(
            legacy_bad > 0,
            "the pre-fix search returned only executable routes on {legacy_total} pairs; the asymmetric control has no force"
        );
    }

    /// With `p ≡ ±1 (mod 5)` and `(mod 7)` every edge is symmetric, so the
    /// two edge sets agree: outgoing routes validate too (the failures above
    /// are the asymmetry, not the validator).
    #[test]
    fn symmetric_primes_validate_under_both_edge_sets() {
        let mut rng = StdRng::seed_from_u64(13);
        let mut pairs = 0;
        for p in primes(3000, 8000, true).into_iter().take(2) {
            let f = PrimeField::new(p).unwrap();
            for _ in 0..5 {
                let Some(e) = random_curve(&f, &mut rng) else {
                    continue;
                };
                let Some((order, models)) = component_in(&f, e, 40, EdgeSet::Certified) else {
                    continue;
                };
                if order.len() < 4 {
                    continue;
                }
                for _ in 0..3 {
                    let a = models[&order[rng.gen_range(0..order.len())]];
                    let b = models[&order[rng.gen_range(0..order.len())]];
                    for set in [EdgeSet::Certified, EdgeSet::Outgoing] {
                        let mut w = Work::default();
                        let r = search(&f, a, b, set, &mut w).expect("connected");
                        validate_route(&f, a, b, &r.route, &mut Work::default())
                            .unwrap_or_else(|e| panic!("{set:?} route invalid: {e:?} (p = {p})"));
                    }
                    pairs += 1;
                }
            }
        }
        assert!(pairs > 0);
    }

    /// The refactored outgoing search keeps the frozen lengths: it agrees
    /// with the pre-fix `bidirectional` on its length.
    #[test]
    fn the_outgoing_control_reproduces_the_frozen_search_lengths() {
        let mut rng = StdRng::seed_from_u64(17);
        let mut checked = 0;
        for p in primes(500, 2500, false).into_iter().take(3) {
            let f = PrimeField::new(p).unwrap();
            for _ in 0..8 {
                let Some(e) = random_curve(&f, &mut rng) else {
                    continue;
                };
                let Some((order, models)) = component_in(&f, e, 150, EdgeSet::Outgoing) else {
                    continue;
                };
                for _ in 0..3 {
                    let a = models[&order[rng.gen_range(0..order.len())]];
                    let b = models[&order[rng.gen_range(0..order.len())]];
                    let old = bidirectional(&f, a, b, &mut [0u64; 4]);
                    let mut w = Work::default();
                    let new = search(&f, a, b, EdgeSet::Outgoing, &mut w).map(|r| r.len);
                    assert_eq!(old, new, "p = {p}");
                    checked += 1;
                }
            }
        }
        assert!(checked > 0);
    }
}
