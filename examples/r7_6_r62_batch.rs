//! R7.6: finish R6.2 — the pooled-rho batch half re-run with the corrected
//! code and paired intervals, and the corrected path-finding cost fitted
//! against component size (`research/isogeny_fibers_20261010/round7/
//! r7_6_r62_batch/`).
//!
//! Toy instances only; no sub-rho claim and nothing about a deployed curve.
//! Every number written is a count (group additions plus doublings,
//! field `M`/`I`, neighbour enumerations); wall times are contended notes.
//!
//! The class, the transport and the search are the R6.2 code
//! (`examples/r6_2_pooled_class_rho.rs`, frozen) copied here, with the
//! corrected certified-edge rule: an ℓ-isogeny edge enters the class graph
//! only when `SmallIsogeny::dual` certifies its reverse edge, so ℓ = 5, 7
//! edges are present only where that holds (`--sym 1` restricts to
//! `p ≡ ±1 mod 5` and `mod 7`, where it does).
//!
//! * `--mode batch --rbits R --ls 2,4,…,256 --trials T [--sym 0|1]`:
//!   per trial, one planted key on each of the first `Lmax` curves of a
//!   breadth-first class of `E₀`, transported to `E₀` along the certified
//!   duals. Arms, all on the library walk that ecbench's
//!   `rho.multitarget_pooled` and `rho.negation` run
//!   (`ic_boundary::rho_batch_with` / `rho_walk_with` on `{P, −P}`):
//!   - **separate**: each transported target solved by its own
//!     `rho_walk_with` (own set-up, own seed `splitmix(rho_seed ^ i)`);
//!   - **pooled B(L)**: the first `L` transported targets (one per curve of
//!     the first `L` curves of the class, `C = L`) in one `rho_batch_with`
//!     with the trial's seed; every log is mapped back and verified on its
//!     own curve;
//!   - **pooled C(L)**: `L` fresh keys on `E₀`, the same pooled batch with
//!     the same seed (the control).
//!
//!   The separate cost at `L` is the prefix sum over the first `L` targets,
//!   so B and the separate arm are paired on the planted keys and seeds.
//!   Intervals are percentile bootstraps over trials (the pairing unit).
//! * `--mode scaling`: whole components of the certified-edge graph at
//!   many `p` sizes, random pairs searched by the corrected bidirectional
//!   search (`search` over `EdgeSet::Certified`, routes re-validated
//!   forward); the cost per pair (search enumerations + certifications +
//!   validation enumerations) is fitted against the component size on a
//!   log-log scale with a bootstrap interval over classes and pairs.

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

/// `p ≡ ±1 (mod 5)` and `(mod 7)`: the residues at which the ℓ = 5, 7
/// edges have a rational-kernel dual (`SmallIsogeny::dual`).
fn pm1_mod_5_and_7(p: u64) -> bool {
    matches!(p % 5, 1 | 4) && matches!(p % 7, 1 | 6)
}

fn build_class(rbits: u32, seed: u64, want: usize, sym: bool) -> Class {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut tried = 0u64;
    loop {
        tried += 1;
        let p = loop {
            let c = rng.gen_range((1u64 << (rbits + 4))..(1u64 << (rbits + 8))) | 1;
            if is_prime_u64(c) && (!sym || pm1_mod_5_and_7(c)) {
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
        // A component smaller than `want` cannot carry the pool: draw again.
        if class.nodes.len() >= want {
            return class;
        }
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

// ------------------------------------------------- certified path finding ----

/// Which edges a search follows out of a curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EdgeSet {
    /// Every outgoing edge `SmallIsogeny::new` realises (the pre-fix
    /// search). Used on both sides it treats `u → v` as `v → u`.
    #[allow(dead_code)] // the pre-fix set, kept so the type matches the R6.2 search
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

// ------------------------------------------------------------- statistics ----

/// Percentile bootstrap over `n` resampling units: `stat` is called with the
/// resampled unit indices. Returns (point estimate on all units, 2.5 %, 97.5 %).
fn boot_ci(n: usize, nboot: usize, seed: u64, stat: &dyn Fn(&[usize]) -> f64) -> (f64, f64, f64) {
    let all: Vec<usize> = (0..n).collect();
    let point = stat(&all);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut v: Vec<f64> = (0..nboot)
        .map(|_| {
            let idx: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
            stat(&idx)
        })
        .filter(|x| x.is_finite())
        .collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if v.is_empty() {
        return (point, f64::NAN, f64::NAN);
    }
    let q = |p: f64| v[(((v.len() - 1) as f64) * p).round() as usize];
    (point, q(0.025), q(0.975))
}

/// Least-squares slope and intercept of `y` on `x`.
fn ols(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let sxx: f64 = x.iter().map(|a| (a - mx).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let b = sxy / sxx;
    (b, my - b * mx)
}

/// `ks_reference(l)` is the Kuhn–Struik total in `S` units for `l` targets;
/// the ratio of the pooled total to `l` separate walks, in the same
/// idealised units, is `Σ_{i<l} C(2i,i)·4^{−i} / l`.
fn ks_ratio(l: usize) -> f64 {
    ks_reference(l) / ((std::f64::consts::PI / 4.0).sqrt() * l as f64)
}

fn jv(x: (f64, f64, f64)) -> serde_json::Value {
    json!({"est": x.0, "lo": x.1, "hi": x.2})
}

// ------------------------------------------------------------------ batch ----

/// Price transport field operations in affine-addition equivalents: an add
/// is `1 I + 3 M` (S counted as M), with `I` priced at `w` `M`.
fn transport_gae(m: f64, i: f64, w: f64) -> f64 {
    (m + w * i) / (3.0 + w)
}

/// Ratio of totals over the resampled trials `idx`.
fn ratio_totals(num: &dyn Fn(usize) -> f64, den: &dyn Fn(usize) -> f64, idx: &[usize]) -> f64 {
    idx.iter().map(|&t| num(t)).sum::<f64>() / idx.iter().map(|&t| den(t)).sum::<f64>()
}

/// Slope of `ln series(idx, li)` on `lx` over the L values.
fn slope_of(lx: &[f64], series: &dyn Fn(&[usize], usize) -> f64, idx: &[usize]) -> f64 {
    let y: Vec<f64> = (0..lx.len()).map(|li| series(idx, li).ln()).collect();
    ols(lx, &y).0
}

fn batch_mode(args: &[String]) {
    let rbits: u32 = arg(args, "--rbits", 24);
    let trials: usize = arg(args, "--trials", 20);
    let seed: u64 = arg(args, "--seed", 1);
    let sym: bool = arg::<u32>(args, "--sym", 0) == 1;
    let nboot: usize = arg(args, "--nboot", 4000);
    let out_path: String = arg(args, "--out", String::from("r7_6_batch.json"));
    let ls: Vec<usize> = arg(args, "--ls", String::from("2,4,8,16,32,64,128,256"))
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let lmax = *ls.iter().max().unwrap();
    let t0 = Instant::now();
    let class = build_class(rbits, seed, lmax, sym);
    let f = &class.f;
    let r = class.r;
    let nn = class.nodes.len();
    assert!(nn >= lmax, "class has {nn} curves, need {lmax}");
    let ell_count: Vec<u64> = ELLS
        .iter()
        .map(|&l| class.nodes.iter().filter(|n| n.ell == l).count() as u64)
        .collect();
    eprintln!(
        "class: p={} r={} (2^{:.2}) h={} nodes={} edges_by_ell={:?} without_dual={:?} tried={} ({:.1}s)",
        class.p,
        r,
        (r as f64).log2(),
        class.h,
        nn,
        ell_count,
        class.edges_without_dual,
        class.curves_tried,
        t0.elapsed().as_secs_f64()
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
    let mut verified_logs = 0u64;

    // Per trial: separate gae per target, pooled B and C gae per L,
    // transport M and I per target.
    let mut sep: Vec<Vec<f64>> = Vec::new();
    let mut pooled_b: Vec<Vec<f64>> = Vec::new();
    let mut pooled_c: Vec<Vec<f64>> = Vec::new();
    let mut tmi: Vec<Vec<(u64, u64)>> = Vec::new();
    let mut records = Vec::new();
    for t in 0..trials {
        let tseed = splitmix(seed ^ splitmix(0x7600 ^ t as u64));
        let mut krng = StdRng::seed_from_u64(tseed);
        let rho_seed = splitmix(tseed ^ 0x5eed);
        // One planted key on each of the first lmax curves, transported to E₀.
        let mut planted = Vec::with_capacity(lmax);
        let mut on_e0 = Vec::with_capacity(lmax);
        let mut tr = Vec::with_capacity(lmax);
        for node in 0..lmax {
            let k = krng.gen_range(1..r);
            let nd = &class.nodes[node];
            let q = nd.e.mul(f, Some(nd.gen), k as u128).unwrap();
            let start = counted.tally();
            on_e0.push(pp(transport(&class, &counted, node, q)));
            let d = counted.tally().since(start);
            tr.push((d.mul, d.inv));
            planted.push((node, k, q));
        }
        // Separate arm: one walk per transported target.
        let mut sep_t = Vec::with_capacity(lmax);
        for (i, tg) in on_e0.iter().enumerate() {
            let res = rho_walk_with(
                &g,
                &NegationClasses { r },
                p0,
                *tg,
                r,
                splitmix(rho_seed ^ ((i as u64 + 1) << 20)),
                cap,
                RhoWalk::negation(),
            );
            let (node, k, q) = &planted[i];
            let m = res.recovered.expect("separate target solved");
            let nd = &class.nodes[*node];
            let kk = mulmod(m, invmod(nd.deg, r), r);
            assert_eq!(nd.e.mul(f, Some(nd.gen), kk as u128), Some(*q));
            assert_eq!(kk, *k);
            verified_logs += 1;
            sep_t.push(res.gae);
        }
        // Pooled arms per L.
        let mut b_t = Vec::new();
        let mut c_t = Vec::new();
        let keys_c: Vec<u64> = (0..lmax).map(|_| krng.gen_range(1..r)).collect();
        let targets_c: Vec<PrimePoint> = keys_c
            .iter()
            .map(|&k| {
                let mut o = GroupOps::default();
                g.mul(&mut o, p0, k)
            })
            .collect();
        for &l in &ls {
            let res_b = rho_batch_with(
                &g,
                &NegationClasses { r },
                p0,
                &on_e0[..l],
                r,
                rho_seed,
                cap,
                RhoWalk::negation(),
            );
            for ((node, k, q), t_res) in planted[..l].iter().zip(&res_b.per_target) {
                let m = t_res.recovered.expect("B target solved");
                let nd = &class.nodes[*node];
                let kk = mulmod(m, invmod(nd.deg, r), r);
                assert_eq!(nd.e.mul(f, Some(nd.gen), kk as u128), Some(*q));
                assert_eq!(kk, *k);
                verified_logs += 1;
            }
            let res_c = rho_batch_with(
                &g,
                &NegationClasses { r },
                p0,
                &targets_c[..l],
                r,
                rho_seed,
                cap,
                RhoWalk::negation(),
            );
            for ((k, q), t_res) in keys_c.iter().zip(&targets_c).zip(&res_c.per_target) {
                let m = t_res.recovered.expect("C target solved");
                assert_eq!(
                    class.e0.mul(f, Some(class.nodes[0].gen), m as u128),
                    Some((q.x, q.y))
                );
                assert_eq!(m, *k);
                verified_logs += 1;
            }
            b_t.push(res_b.gae);
            c_t.push(res_c.gae);
        }
        let sep_at: Vec<f64> = ls.iter().map(|&l| sep_t[..l].iter().sum()).collect();
        eprintln!(
            "trial {t}: L={} pooled B={:.3} C={:.3} separate={:.3} (S units) ({:.0}s)",
            lmax,
            b_t.last().unwrap() / sqrt_r,
            c_t.last().unwrap() / sqrt_r,
            sep_at.last().unwrap() / sqrt_r,
            t0.elapsed().as_secs_f64()
        );
        records.push(json!({
            "trial": t, "tseed": tseed, "rho_seed": rho_seed,
            "separate_gae_per_target": sep_t,
            "pooled_B_gae": b_t, "pooled_C_gae": c_t,
            "transport_M_I_per_target": tr.iter().map(|x| json!([x.0, x.1])).collect::<Vec<_>>(),
        }));
        sep.push(sep_t);
        pooled_b.push(b_t);
        pooled_c.push(c_t);
        tmi.push(tr);
    }

    // ---- summaries (all natively computed) -------------------------------
    let sep_pref = |t: usize, li: usize| -> f64 { sep[t][..ls[li]].iter().sum() };
    let mut summary = Vec::new();
    for (li, &l) in ls.iter().enumerate() {
        let bsd = |t: usize| pooled_b[t][li];
        let csd = |t: usize| pooled_c[t][li];
        let ssd = |t: usize| sep_pref(t, li);
        let r_bs = boot_ci(trials, nboot, 11, &|idx: &[usize]| {
            ratio_totals(&bsd, &ssd, idx)
        });
        let r_cs = boot_ci(trials, nboot, 12, &|idx: &[usize]| {
            ratio_totals(&csd, &ssd, idx)
        });
        let r_bc = boot_ci(trials, nboot, 13, &|idx: &[usize]| {
            ratio_totals(&bsd, &csd, idx)
        });
        let ksr = ks_ratio(l);
        let strict = 1.0 / (l as f64).sqrt();
        let r_over_strict = boot_ci(trials, nboot, 14, &|idx: &[usize]| {
            ratio_totals(&bsd, &ssd, idx) / strict
        });
        let r_over_ks = boot_ci(trials, nboot, 15, &|idx: &[usize]| {
            ratio_totals(&bsd, &ssd, idx) / ksr
        });
        // Transport priced into the pooled arm (affine-add equivalents).
        let allin = |w: f64| {
            let tmi = &tmi;
            let (bsd, ssd) = (&bsd, &ssd);
            move |idx: &[usize]| -> f64 {
                let num: f64 = idx
                    .iter()
                    .map(|&t| {
                        let (m, i) = tmi[t][..l]
                            .iter()
                            .fold((0.0, 0.0), |a, x| (a.0 + x.0 as f64, a.1 + x.1 as f64));
                        bsd(t) + transport_gae(m, i, w)
                    })
                    .sum();
                num / idx.iter().map(|&t| ssd(t)).sum::<f64>()
            }
        };
        let r_allin = [3.0, 10.0, 30.0]
            .iter()
            .map(|&w| (w, boot_ci(trials, nboot, 16, &allin(w))))
            .collect::<Vec<_>>();
        let mean_s =
            |v: &dyn Fn(usize) -> f64| (0..trials).map(v).sum::<f64>() / trials as f64 / sqrt_r;
        let (tm, ti) = tmi.iter().fold((0.0, 0.0), |a, v| {
            let s = v[..l]
                .iter()
                .fold((0.0, 0.0), |a, x| (a.0 + x.0 as f64, a.1 + x.1 as f64));
            (a.0 + s.0, a.1 + s.1)
        });
        eprintln!(
            "L={l:4} pooled/separate={:.4} [{:.4},{:.4}] gain={:.2} gain/√L={:.3} KS-ratio={:.4} B/C={:.3} [{:.3},{:.3}]",
            r_bs.0, r_bs.1, r_bs.2, 1.0 / r_bs.0, (1.0 / r_bs.0) / (l as f64).sqrt(), ksr, r_bc.0, r_bc.1, r_bc.2
        );
        summary.push(json!({
            "L": l, "trials": trials,
            "mean_S_pooled_B": mean_s(&bsd), "mean_S_pooled_C": mean_s(&csd), "mean_S_separate": mean_s(&ssd),
            "ratio_pooled_over_separate": jv(r_bs),
            "gain_separate_over_pooled": jv((1.0 / r_bs.0, 1.0 / r_bs.2, 1.0 / r_bs.1)),
            "gain_over_sqrt_L": jv(((1.0 / r_bs.0) / (l as f64).sqrt(), (1.0 / r_bs.2) / (l as f64).sqrt(), (1.0 / r_bs.1) / (l as f64).sqrt())),
            "ratio_control_pooled_C_over_separate": jv(r_cs),
            "ratio_B_over_C": jv(r_bc),
            "B_over_C_within_0.95_1.05": r_bc.1 >= 0.95 && r_bc.2 <= 1.05,
            "B_over_C_interval_excludes_1": r_bc.1 > 1.0 || r_bc.2 < 1.0,
            "strict_prediction_ratio_1_over_sqrt_L": strict,
            "ratio_over_strict_prediction": jv(r_over_strict),
            "ks_prediction_ratio": ksr,
            "ratio_over_ks_prediction": jv(r_over_ks),
            "transport_M_per_target": tm / (trials * l) as f64,
            "transport_I_per_target": ti / (trials * l) as f64,
            "ratio_with_transport_priced": r_allin.iter().map(|(w, ci)| json!({"I_in_M": w, "ratio": jv(*ci)})).collect::<Vec<_>>(),
        }));
    }
    // Exponents over L: ln(gain) and ln(pooled mean cost) on ln L.
    let lx: Vec<f64> = ls.iter().map(|&l| (l as f64).ln()).collect();
    let gain_series = |idx: &[usize], li: usize| -> f64 {
        idx.iter().map(|&t| sep_pref(t, li)).sum::<f64>()
            / idx.iter().map(|&t| pooled_b[t][li]).sum::<f64>()
    };
    let pooled_series = |idx: &[usize], li: usize| -> f64 {
        idx.iter().map(|&t| pooled_b[t][li]).sum::<f64>() / idx.len() as f64
    };
    let sep_series = |idx: &[usize], li: usize| -> f64 {
        idx.iter().map(|&t| sep_pref(t, li)).sum::<f64>() / idx.len() as f64
    };
    let slope_gain = boot_ci(trials, nboot, 21, &|idx: &[usize]| {
        slope_of(&lx, &gain_series, idx)
    });
    let slope_pooled = boot_ci(trials, nboot, 22, &|idx: &[usize]| {
        slope_of(&lx, &pooled_series, idx)
    });
    let slope_sep = boot_ci(trials, nboot, 23, &|idx: &[usize]| {
        slope_of(&lx, &sep_series, idx)
    });
    eprintln!(
        "exponent over L: gain {:.3} [{:.3},{:.3}] (predicted 0.5); pooled cost {:.3} [{:.3},{:.3}] (0.5); separate {:.3} [{:.3},{:.3}] (1)",
        slope_gain.0, slope_gain.1, slope_gain.2, slope_pooled.0, slope_pooled.1, slope_pooled.2, slope_sep.0, slope_sep.1, slope_sep.2
    );
    let depth_max = class.nodes.iter().map(|n| n.depth).max().unwrap_or(0);
    let out = json!({
        "schema": "r7_6.batch/v1",
        "args": args[1..].to_vec(),
        "base_curve": {"p": class.p, "a": class.e0.a, "b": class.e0.b, "order": class.n, "r": r,
                        "h": class.h, "rbits_actual": (r as f64).log2(), "generator": class.nodes[0].gen},
        "class": {"nodes": nn, "depth_max": depth_max, "sym_primes_only": sym,
                  "edges_by_ell": {"2": ell_count[0], "3": ell_count[1], "5": ell_count[2], "7": ell_count[3]},
                  "enumerations_by_ell": class.enumerations, "duals_certified": class.duals_certified,
                  "edges_without_dual_by_ell": class.edges_without_dual,
                  "curves_tried": class.curves_tried,
                  "curves": class.nodes.iter().take(lmax).map(|n| json!([n.e.a, n.e.b, n.parent, n.ell, n.depth])).collect::<Vec<_>>()},
        "walk": "ic_boundary::rho_batch_with(NegationClasses, RhoWalk::negation()) = ecbench rho.multitarget_pooled; separate: rho_walk_with(..) = ecbench rho.negation",
        "cap": cap,
        "bootstrap": {"method": "percentile, resampling trials", "nboot": nboot},
        "summary": summary,
        "exponents_over_L": {
            "L_values": ls,
            "gain_ln_on_ln_L": jv(slope_gain), "predicted_gain": 0.5,
            "pooled_cost_ln_on_ln_L": jv(slope_pooled), "predicted_pooled": 0.5,
            "separate_cost_ln_on_ln_L": jv(slope_sep), "predicted_separate": 1.0,
        },
        "trials": records,
        "verified_logs": verified_logs,
        "wall_seconds_contended": t0.elapsed().as_secs_f64(),
    });
    std::fs::write(&out_path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    eprintln!("verified logs: {verified_logs}; wrote {out_path}");
}

// ---------------------------------------------------------------- scaling ----

struct PairCost {
    len: u32,
    search: u64,
    cert: u64,
    val: u64,
}

impl PairCost {
    fn total(&self) -> f64 {
        (self.search + self.cert + self.val) as f64
    }
}

struct ClassCost {
    pbits: u32,
    p: u64,
    a: u64,
    b: u64,
    n: u64,
    size: usize,
    ell_split: usize,
    pairs: Vec<PairCost>,
}

fn class_mean(c: &ClassCost, idx: &[usize], total: bool) -> f64 {
    idx.iter()
        .map(|&i| {
            let p = &c.pairs[i];
            if total {
                p.total()
            } else {
                p.search as f64
            }
        })
        .sum::<f64>()
        / idx.len() as f64
}

/// Fit `ln(cost) = a + b·ln(size)` over classes (class means), bootstrap
/// over classes and, within a class, over its pairs.
fn fit_exponent(
    classes: &[&ClassCost],
    total: bool,
    nboot: usize,
    seed: u64,
) -> (f64, f64, f64, f64) {
    let est = |cs: &[(usize, Vec<usize>)]| -> (f64, f64) {
        let x: Vec<f64> = cs
            .iter()
            .map(|(c, _)| (classes[*c].size as f64).ln())
            .collect();
        let y: Vec<f64> = cs
            .iter()
            .map(|(c, pidx)| class_mean(classes[*c], pidx, total).ln())
            .collect();
        ols(&x, &y)
    };
    let full: Vec<(usize, Vec<usize>)> = classes
        .iter()
        .enumerate()
        .map(|(i, c)| (i, (0..c.pairs.len()).collect()))
        .collect();
    let (b0, a0) = est(&full);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut v = Vec::with_capacity(nboot);
    for _ in 0..nboot {
        let cs: Vec<(usize, Vec<usize>)> = (0..classes.len())
            .map(|_| {
                let c = rng.gen_range(0..classes.len());
                let np = classes[c].pairs.len();
                (c, (0..np).map(|_| rng.gen_range(0..np)).collect())
            })
            .collect();
        let (b, _) = est(&cs);
        if b.is_finite() {
            v.push(b);
        }
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| v[(((v.len() - 1) as f64) * p).round() as usize];
    (b0, q(0.025), q(0.975), a0)
}

fn scaling_mode(args: &[String]) {
    let pbits: Vec<u32> = arg(
        args,
        "--pbits",
        String::from("12,14,16,18,20,22,24,26,28,30,32,34"),
    )
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect();
    let per_bits: usize = arg(args, "--classes", 3);
    let pairs: usize = arg(args, "--pairs", 8);
    let cap: usize = arg(args, "--cap", 40000);
    let attempts: usize = arg(args, "--attempts", 60);
    let seed: u64 = arg(args, "--seed", 7);
    let nboot: usize = arg(args, "--nboot", 4000);
    let out_path: String = arg(args, "--out", String::from("r7_6_scaling.json"));
    let t0 = Instant::now();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut classes: Vec<ClassCost> = Vec::new();
    let mut skipped = Vec::new();
    for &bits in &pbits {
        let mut done = 0;
        let (mut over_cap, mut under_8, mut tried) = (0u64, 0u64, 0usize);
        while done < per_bits && tried < attempts {
            tried += 1;
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
            let ell_split = ELLS
                .iter()
                .filter(|&&l| !kernel_representatives(&f, &e, l).unwrap().is_empty())
                .count();
            if ell_split < 2 {
                continue;
            }
            let Some((order, models)) = component_in(&f, e, cap, EdgeSet::Certified) else {
                over_cap += 1;
                continue;
            };
            if order.len() < 8 {
                under_8 += 1;
                continue;
            }
            let size = order.len();
            let mut crng = StdRng::seed_from_u64(splitmix(seed ^ splitmix(classes.len() as u64)));
            let mut pc = Vec::with_capacity(pairs);
            for _ in 0..pairs {
                let a = models[&order[crng.gen_range(0..size)]];
                let b = models[&order[crng.gen_range(0..size)]];
                let mut w = Work::default();
                let found = search(&f, a, b, EdgeSet::Certified, &mut w)
                    .expect("a and b share a certified component");
                let mut wv = Work::default();
                let v = validate_route(&f, a, b, &found.route, &mut wv);
                assert!(
                    v.is_ok(),
                    "a returned route failed forward validation: {v:?}"
                );
                pc.push(PairCost {
                    len: found.len,
                    search: w.search(),
                    cert: w.cert(),
                    val: wv.search(),
                });
            }
            let mt = pc.iter().map(|x| x.total()).sum::<f64>() / pairs as f64;
            eprintln!(
                "pbits={bits} p={p} component={size} mean total/pair={mt:.1} ({:.2}×size) ({:.0}s)",
                mt / size as f64,
                t0.elapsed().as_secs_f64()
            );
            classes.push(ClassCost {
                pbits: bits,
                p,
                a: e.a,
                b: e.b,
                n,
                size,
                ell_split,
                pairs: pc,
            });
            done += 1;
        }
        eprintln!("pbits={bits}: accepted {done}, over cap {over_cap}, under 8 curves {under_8}");
        skipped.push(json!({"pbits": bits, "accepted": done, "over_cap": over_cap, "under_8": under_8, "attempts": tried}));
    }
    let all: Vec<&ClassCost> = classes.iter().collect();
    let sizes: Vec<usize> = classes.iter().map(|c| c.size).collect();
    let (smin, smax) = (*sizes.iter().min().unwrap(), *sizes.iter().max().unwrap());
    let mut distinct = sizes.clone();
    distinct.sort_unstable();
    distinct.dedup();
    let mut fits = Vec::new();
    let subsets: [(&str, usize, usize); 3] = [
        ("all", 0, usize::MAX),
        ("size_ge_100", 100, usize::MAX),
        ("size_lt_1000", 0, 1000),
    ];
    for (name, lo, hi) in subsets {
        let sub: Vec<&ClassCost> = all
            .iter()
            .copied()
            .filter(|c| c.size >= lo && c.size < hi)
            .collect();
        if sub.len() < 4 {
            continue;
        }
        for (what, total) in [("total", true), ("search_only", false)] {
            let (b, blo, bhi, a) = fit_exponent(&sub, total, nboot, 31);
            let ssub: Vec<usize> = sub.iter().map(|c| c.size).collect();
            eprintln!(
                "fit {name} {what}: exponent {b:.3} [{blo:.3},{bhi:.3}] over {} classes, sizes {}..{}",
                sub.len(),
                ssub.iter().min().unwrap(),
                ssub.iter().max().unwrap()
            );
            fits.push(json!({
                "subset": name, "cost": what, "classes": sub.len(),
                "size_min": ssub.iter().min(), "size_max": ssub.iter().max(),
                "exponent": b, "lo": blo, "hi": bhi, "intercept_ln": a,
                "sqrt_in_interval": blo <= 0.5 && 0.5 <= bhi,
                "one_in_interval": blo <= 1.0 && 1.0 <= bhi,
            }));
        }
    }
    let out = json!({
        "schema": "r7_6.scaling/v1",
        "args": args[1..].to_vec(),
        "cap": cap,
        "edge_set": "certified: u -> v kept only when SmallIsogeny::dual certifies v -> u",
        "cost": "enumerations of the bidirectional search + dual certifications + forward validation enumerations, per pair",
        "bootstrap": {"method": "percentile, resampling classes and, within a class, its pairs", "nboot": nboot},
        "sizes": {"classes": classes.len(), "distinct_sizes": distinct.len(), "min": smin, "max": smax,
                  "decades": (smax as f64 / smin as f64).log10()},
        "sampling": skipped,
        "fits": fits,
        "classes": classes.iter().map(|c| json!({
            "pbits": c.pbits, "p": c.p, "a": c.a, "b": c.b, "order": c.n,
            "component_size": c.size, "ells_with_rational_kernel_at_start": c.ell_split,
            "pairs": c.pairs.iter().map(|x| json!({"length": x.len, "enumerations": x.search,
                      "certifications": x.cert, "validation_enumerations": x.val,
                      "total": x.search + x.cert + x.val})).collect::<Vec<_>>(),
            "mean_total": class_mean(c, &(0..c.pairs.len()).collect::<Vec<_>>(), true),
            "mean_search": class_mean(c, &(0..c.pairs.len()).collect::<Vec<_>>(), false),
        })).collect::<Vec<_>>(),
        "wall_seconds_contended": t0.elapsed().as_secs_f64(),
    });
    std::fs::write(&out_path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    eprintln!("wrote {out_path}");
}

/// Read the classes of `r7_6.scaling/v1` records.
fn read_classes(paths: &[String]) -> Vec<ClassCost> {
    let mut out = Vec::new();
    for path in paths {
        let v: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(path).expect("read --inputs file"))
                .expect("--inputs is JSON");
        assert_eq!(v["schema"], "r7_6.scaling/v1", "{path}");
        for c in v["classes"].as_array().expect("classes") {
            out.push(ClassCost {
                pbits: c["pbits"].as_u64().unwrap() as u32,
                p: c["p"].as_u64().unwrap(),
                a: c["a"].as_u64().unwrap(),
                b: c["b"].as_u64().unwrap(),
                n: c["order"].as_u64().unwrap(),
                size: c["component_size"].as_u64().unwrap() as usize,
                ell_split: c["ells_with_rational_kernel_at_start"].as_u64().unwrap() as usize,
                pairs: c["pairs"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| PairCost {
                        len: x["length"].as_u64().unwrap() as u32,
                        search: x["enumerations"].as_u64().unwrap(),
                        cert: x["certifications"].as_u64().unwrap(),
                        val: x["validation_enumerations"].as_u64().unwrap(),
                    })
                    .collect(),
            });
        }
    }
    out
}

/// `--mode fit --inputs a.json,b.json`: the exponent fits over the merged
/// classes of several `scaling` records, size-bin means, and the exponent of
/// the ideal Kuhn–Struik gain over `L = 2..256` (the batch half's reference).
fn fit_mode(args: &[String]) {
    let inputs: Vec<String> = arg(args, "--inputs", String::new())
        .split(',')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    let nboot: usize = arg(args, "--nboot", 10000);
    let out_path: String = arg(args, "--out", String::from("r7_6_scaling_fit.json"));
    let classes = read_classes(&inputs);
    let all: Vec<&ClassCost> = classes.iter().collect();
    let mut fits = Vec::new();
    let subsets: [(&str, usize, usize, usize); 6] = [
        ("all", 0, usize::MAX, 0),
        ("size_ge_100", 100, usize::MAX, 0),
        ("size_ge_1000", 1000, usize::MAX, 0),
        ("size_lt_1000", 0, 1000, 0),
        ("ells_split_ge_3", 0, usize::MAX, 3),
        ("ells_split_le_2", 0, usize::MAX, 99),
    ];
    for (name, lo, hi, ells) in subsets {
        let sub: Vec<&ClassCost> = all
            .iter()
            .copied()
            .filter(|c| {
                c.size >= lo
                    && c.size < hi
                    && match ells {
                        0 => true,
                        99 => c.ell_split <= 2,
                        k => c.ell_split >= k,
                    }
            })
            .collect();
        if sub.len() < 4 {
            continue;
        }
        for (what, total) in [("total", true), ("search_only", false)] {
            let (b, blo, bhi, a) = fit_exponent(&sub, total, nboot, 31);
            let sizes: Vec<usize> = sub.iter().map(|c| c.size).collect();
            eprintln!(
                "fit {name} {what}: exponent {b:.3} [{blo:.3},{bhi:.3}] over {} classes, sizes {}..{}",
                sub.len(),
                sizes.iter().min().unwrap(),
                sizes.iter().max().unwrap()
            );
            fits.push(json!({
                "subset": name, "cost": what, "classes": sub.len(),
                "size_min": sizes.iter().min(), "size_max": sizes.iter().max(),
                "exponent": b, "lo": blo, "hi": bhi, "intercept_ln": a,
                "sqrt_in_interval": blo <= 0.5 && 0.5 <= bhi,
                "one_in_interval": blo <= 1.0 && 1.0 <= bhi,
            }));
        }
    }
    // Half-decade size bins: class means.
    let edges = [8usize, 30, 100, 300, 1000, 3000, 10_000, 100_000];
    let mut bins = Vec::new();
    for w in edges.windows(2) {
        let sub: Vec<&ClassCost> = all
            .iter()
            .copied()
            .filter(|c| c.size >= w[0] && c.size < w[1])
            .collect();
        if sub.is_empty() {
            continue;
        }
        let n = sub.len() as f64;
        let mean_size = sub.iter().map(|c| c.size as f64).sum::<f64>() / n;
        let cost = |c: &ClassCost| class_mean(c, &(0..c.pairs.len()).collect::<Vec<_>>(), true);
        let mean_cost = sub.iter().map(|c| cost(c)).sum::<f64>() / n;
        let ratio: Vec<f64> = sub.iter().map(|c| cost(c) / c.size as f64).collect();
        let (rlo, rhi) = ratio
            .iter()
            .fold((f64::MAX, 0f64), |a, &x| (a.0.min(x), a.1.max(x)));
        bins.push(json!({
            "size_from": w[0], "size_to": w[1], "classes": sub.len(),
            "mean_size": mean_size, "mean_total_cost": mean_cost,
            "mean_cost_over_size": ratio.iter().sum::<f64>() / n,
            "min_cost_over_size": rlo, "max_cost_over_size": rhi,
            "mean_cost_over_sqrt_size": sub.iter().map(|c| cost(c) / (c.size as f64).sqrt()).sum::<f64>() / n,
        }));
    }
    let ks_slope = {
        let ls = [2usize, 4, 8, 16, 32, 64, 128, 256];
        let x: Vec<f64> = ls.iter().map(|&l| (l as f64).ln()).collect();
        let y: Vec<f64> = ls.iter().map(|&l| (1.0 / ks_ratio(l)).ln()).collect();
        ols(&x, &y).0
    };
    let sizes: Vec<usize> = classes.iter().map(|c| c.size).collect();
    let (smin, smax) = (*sizes.iter().min().unwrap(), *sizes.iter().max().unwrap());
    let mut distinct = sizes.clone();
    distinct.sort_unstable();
    distinct.dedup();
    let out = json!({
        "schema": "r7_6.scaling_fit/v1",
        "inputs": inputs,
        "bootstrap": {"method": "percentile, resampling classes and, within a class, its pairs", "nboot": nboot},
        "sizes": {"classes": classes.len(), "distinct_sizes": distinct.len(), "min": smin, "max": smax,
                  "decades": (smax as f64 / smin as f64).log10()},
        "fits": fits,
        "size_bins": bins,
        "ks_ideal_gain_exponent_L2_to_256": ks_slope,
    });
    std::fs::write(&out_path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    eprintln!("ks ideal gain exponent over L=2..256: {ks_slope:.4}; wrote {out_path}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode: String = arg(&args, "--mode", String::from("batch"));
    match mode.as_str() {
        "batch" => batch_mode(&args),
        "scaling" => scaling_mode(&args),
        "fit" => fit_mode(&args),
        other => panic!("unknown mode {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kuhn_struik_ratio_starts_at_one_and_falls_like_inverse_sqrt() {
        assert!((ks_ratio(1) - 1.0).abs() < 1e-12);
        // 2/√(π L) asymptotically.
        let l = 4096;
        let asym = 2.0 / (std::f64::consts::PI * l as f64).sqrt();
        assert!((ks_ratio(l) / asym - 1.0).abs() < 0.01);
    }

    #[test]
    fn bootstrap_interval_brackets_a_known_mean() {
        let v: Vec<f64> = (0..200)
            .map(|i| 1.0 + ((i * 37) % 11) as f64 * 0.01)
            .collect();
        let m = v.iter().sum::<f64>() / v.len() as f64;
        let (p, lo, hi) = boot_ci(v.len(), 500, 3, &|idx: &[usize]| {
            idx.iter().map(|&i| v[i]).sum::<f64>() / idx.len() as f64
        });
        assert!((p - m).abs() < 1e-12 && lo < m && m < hi && hi - lo < 0.05);
    }

    #[test]
    fn ols_recovers_a_power_law() {
        let x: Vec<f64> = (1..20).map(|i| (i as f64).ln()).collect();
        let y: Vec<f64> = x.iter().map(|a| 3.0 + 0.5 * a).collect();
        let (b, a) = ols(&x, &y);
        assert!((b - 0.5).abs() < 1e-12 && (a - 3.0).abs() < 1e-12);
    }

    /// The corrected search validates forward on a small asymmetric prime
    /// (p ≢ ±1 mod 5, 7), and `--sym` primes keep the ℓ = 5, 7 edges.
    #[test]
    fn certified_routes_validate_and_sym_primes_keep_l5_l7_edges() {
        let mut rng = StdRng::seed_from_u64(3);
        let p = (30000u64..40000)
            .find(|&p| is_prime_u64(p) && !pm1_mod_5_and_7(p) && p % 5 != 1 && p % 5 != 4)
            .unwrap();
        let f = PrimeField::new(p).unwrap();
        let mut validated = 0;
        for _ in 0..10 {
            let e = ShortWeierstrass::new(&f, rng.gen_range(1..p), rng.gen_range(1..p));
            let Ok(e) = e else { continue };
            let Some((order, models)) = component_in(&f, e, 120, EdgeSet::Certified) else {
                continue;
            };
            if order.len() < 4 {
                continue;
            }
            let a = models[&order[0]];
            let b = models[&order[order.len() - 1]];
            let mut w = Work::default();
            let found = search(&f, a, b, EdgeSet::Certified, &mut w).unwrap();
            assert!(validate_route(&f, a, b, &found.route, &mut Work::default()).is_ok());
            validated += 1;
        }
        assert!(validated > 0);
        // On p = ±1 mod 5 and mod 7 the l = 5, 7 edges have a certified dual.
        let q = (30000u64..40000)
            .find(|&q| is_prime_u64(q) && pm1_mod_5_and_7(q))
            .unwrap();
        let g = PrimeField::new(q).unwrap();
        let (mut e5, mut e7) = (0, 0);
        for _ in 0..60 {
            let Ok(e) = ShortWeierstrass::new(&g, rng.gen_range(1..q), rng.gen_range(1..q)) else {
                continue;
            };
            for edge in edges_of(&g, &e, EdgeSet::Certified, &mut Work::default()) {
                e5 += (edge.ell == 5) as u32;
                e7 += (edge.ell == 7) as u32;
            }
        }
        assert!(e5 > 0 && e7 > 0, "no certified l = 5 or 7 edge on p = {q}");
    }
}
