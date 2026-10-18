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
//! * `--mode paths`: whole components of the rational `ℓ ∈ {2, 3, 5, 7}`
//!   graph of toy curves, and for random pairs of curves in a component a
//!   bidirectional breadth-first search that finds a path from scratch,
//!   counting neighbour enumerations (one per (curve, ℓ): the rational roots
//!   of the ℓ-division polynomial, the role `Φ_ℓ(j, Y)` root finding plays in
//!   an Elkies walk).
//!
//! Every number written is a count (group operations, field `M`/`I`,
//! neighbour enumerations); wall times are recorded as contended notes.

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    rho_batch_with, rho_cap, rho_walk_with, CountedGroup, GroupOps, NegationClasses, PrimeCurve,
    PrimePoint, RhoWalk,
};
use crypto_lib::cryptanalysis::isogeny_eval::counted_fp::is_prime_u64;
use crypto_lib::cryptanalysis::isogeny_eval::small_degree::kernel_representatives;
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
        while c % d == 0 && c > d {
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

fn paths_mode(args: &[String]) {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode: String = arg(&args, "--mode", String::from("batch"));
    match mode.as_str() {
        "batch" => batch_mode(&args),
        "paths" => paths_mode(&args),
        other => panic!("unknown mode {other}"),
    }
}
