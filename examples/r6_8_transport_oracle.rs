//! Round-6 lead R6.8: transport oracles on factor-base points
//! (Proposition 5′ of `research/isogeny_fibers_20261010/round6/r6_8_transport_oracle/RESULTS.md`).
//!
//! An attacker with a transport oracle `T_ψ` may move the factor base `F′`
//! of an isogenous neighbour `E′` back to `E` along the dual, and work with
//! the union `F ∪ ψ̂(F′)`.  This example measures, by exact census, what
//! that union is worth against the single bases, on
//!
//! * the `j = 0` toy curves of §15.2 (`y² = x³ + b`, `N = 3r`) with the
//!   3-isogeny `E → E/⟨T⟩`, `T = (0, √b)`;
//! * a toy curve `y² = x³ + ax + b` with `N = 6r`, `r` prime, with one
//!   2-isogenous and one 3-isogenous neighbour.
//!
//! Isogenies and certified duals come from
//! `crypto_lib::cryptanalysis::isogeny_eval::small_degree`.
//!
//! Arms (each a signed base of `--size` points unless the pullback merged
//! points): `plain` small-x base on `E`; `neighbour_l<ℓ>`, the small-x base
//! `F′` on `E′` (what the attacker would use on the far side, the CJ-DLP
//! side); `pullback_l<ℓ>` = `ψ̂(F′)` on `E`; `filtered_l<ℓ>E`, a small-x
//! base on `E` filtered to the image `ℓE(F_p)` (the no-oracle control of
//! Prop. 8); `union` = `F ∪ ⋃ ψ̂(F′_ℓ)`; `union_pullbacks` = `⋃ ψ̂(F′_ℓ)`.
//!
//! Per arm and `m ∈ {2, 3}`: unknowns `U` (orbits of `[h]P` under `±`),
//! the exact number `D_m` of distinct `m`-sums landing in `G`, relations
//! per target `D_m / r`, class factor `c = D_m·m!/|F|^m`, the class-count
//! prediction of `c` (fraction of ordered `m`-tuples whose cofactor classes
//! sum to zero), `c / U`, and the §15.2 cost `U·(r/D_m)·L_m` with `L_2 = 1`,
//! `L_3 = |F|`.  Exact transport identities and a sampled relation-transport
//! simulation are also recorded.  Every count is exact; only the simulation
//! targets are sampled.
//!
//! ```sh
//! cargo run --release --example r6_8_transport_oracle -- \
//!     --family j0 --p 195709 --b 3 --size 144 --seed 7 > out.json
//! cargo run --release --example r6_8_transport_oracle -- \
//!     --family 6r --pmin 1048576 --size 144 --seed 7 > out.json
//! ```

use crypto_lib::cryptanalysis::curve_id;
use crypto_lib::cryptanalysis::isogeny_eval::small_degree::kernel_x_coordinates;
use crypto_lib::cryptanalysis::isogeny_eval::{PrimeField, ShortWeierstrass, SmallIsogeny, WPoint};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

type F = PrimeField;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_u64(args: &[String], name: &str, default: u64) -> u64 {
    arg(args, name)
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

/// `#E(F_p)` by summing Legendre symbols (exact; `p ≈ 2^20`).
fn point_count(f: &F, e: &ShortWeierstrass) -> u64 {
    let s: i64 = (0..f.p()).map(|x| f.legendre(e.rhs(f, x)) as i64).sum();
    (f.p() as i64 + 1 + s) as u64
}

fn random_point(f: &F, e: &ShortWeierstrass, rng: &mut StdRng) -> WPoint {
    loop {
        let x = rng.gen_range(0..f.p());
        if let Some(y) = f.sqrt(e.rhs(f, x)) {
            if y != 0 {
                return Some(if rng.gen::<bool>() {
                    (x, y)
                } else {
                    (x, f.neg(y))
                });
            }
        }
    }
}

/// A curve with its group data: `N = h·r`, `r` prime, the `h`-part cyclic
/// with generator `gh`, and the class index map `P ↦ k` with `[r]P = k·gh`.
struct Group {
    e: ShortWeierstrass,
    n: u64,
    h: u64,
    r: u64,
    /// Multiples `0·gh, 1·gh, …, (h−1)·gh` of a generator of `E(F_p)[h]`.
    h_table: Vec<WPoint>,
}

impl Group {
    fn new(f: &F, e: ShortWeierstrass, n: u64, h: u64, rng: &mut StdRng) -> Option<Self> {
        let r = n / h;
        if h * r != n || !is_prime(r) || r.is_multiple_of(h) {
            return None;
        }
        // generator of the h-part: [r]P of order exactly h (cyclic h-part)
        let mut gh: WPoint = None;
        for _ in 0..1000 {
            let q = e.mul(f, random_point(f, &e, rng), r as u128);
            let mut ok = q.is_some();
            for d in 2..h {
                if h.is_multiple_of(d) && e.mul(f, q, d as u128).is_none() {
                    ok = false;
                }
            }
            if ok && e.mul(f, q, h as u128).is_none() {
                gh = q;
                break;
            }
        }
        gh?;
        let mut h_table = vec![None];
        let mut acc = gh;
        for _ in 1..h {
            h_table.push(acc);
            acc = e.add(f, acc, gh);
        }
        if acc.is_some() {
            return None;
        }
        Some(Group {
            e,
            n,
            h,
            r,
            h_table,
        })
    }

    /// The cofactor class `k ∈ Z/h` of `P`: `[r]P = k·gh`.
    fn cls(&self, f: &F, p: WPoint) -> u64 {
        let q = self.e.mul(f, p, self.r as u128);
        self.h_table
            .iter()
            .position(|&t| t == q)
            .expect("h-part is cyclic") as u64
    }

    fn in_g(&self, f: &F, p: WPoint) -> bool {
        self.e.mul(f, p, self.r as u128).is_none()
    }

    /// Signed small-x base of `size` points (`size` even), keeping only
    /// points `P` with `filter(cls(P))`.
    fn small_x_base(&self, f: &F, size: usize, filter: impl Fn(u64) -> bool) -> Vec<WPoint> {
        let mut v = Vec::new();
        let mut x = 1u64;
        while v.len() < size && x < f.p() {
            if let Some(y) = f.sqrt(self.e.rhs(f, x)) {
                if y != 0 {
                    let p = Some((x, y));
                    if filter(self.cls(f, p)) {
                        v.push(p);
                        v.push(self.e.neg(f, p));
                    }
                }
            }
            x += 1;
        }
        v
    }
}

/// Factorial as f64.
fn fact(m: usize) -> f64 {
    (1..=m).map(|k| k as f64).product()
}

/// Exact census of one base on one curve for one `m`.
struct Census {
    size: usize,
    columns: usize,
    class_hist: Vec<u64>,
    d_in_g: usize,
    c: f64,
    c_pred: f64,
    cost: f64,
}

fn census(f: &F, g: &Group, base: &[WPoint], m: usize) -> Census {
    let h = g.h as usize;
    let cls: Vec<u64> = base.iter().map(|&p| g.cls(f, p)).collect();
    let mut class_hist = vec![0u64; h];
    for &k in &cls {
        class_hist[k as usize] += 1;
    }
    // columns: orbits of [h]P under ±
    let mut cols: HashSet<WPoint> = HashSet::new();
    for &p in base {
        let q = g.e.mul(f, p, g.h as u128);
        if let Some((x, y)) = q {
            let yy = y.min(f.neg(y));
            cols.insert(Some((x, yy)));
        }
    }
    // by-class index lists
    let mut by_class: Vec<Vec<usize>> = vec![Vec::new(); h];
    for (i, &k) in cls.iter().enumerate() {
        by_class[k as usize].push(i);
    }
    let mut sums: HashSet<WPoint> = HashSet::new();
    match m {
        2 => {
            for i in 0..base.len() {
                let need = (h as u64 - cls[i]) % h as u64;
                for &j in &by_class[need as usize] {
                    if j < i {
                        continue;
                    }
                    let s = g.e.add(f, base[i], base[j]);
                    if s.is_some() {
                        sums.insert(s);
                    }
                }
            }
        }
        3 => {
            for i in 0..base.len() {
                for j in i..base.len() {
                    let s2 = g.e.add(f, base[i], base[j]);
                    let need = (2 * h as u64 - cls[i] - cls[j]) % h as u64;
                    for &k in &by_class[need as usize] {
                        if k < j {
                            continue;
                        }
                        let s = g.e.add(f, s2, base[k]);
                        if s.is_some() {
                            sums.insert(s);
                        }
                    }
                }
            }
        }
        _ => panic!("m must be 2 or 3"),
    }
    // every recorded sum is in G by construction of the class filter; check
    debug_assert!(sums.iter().all(|&s| g.in_g(f, s)));
    let d = sums.len();
    let size = base.len();
    let c = d as f64 * fact(m) / (size as f64).powi(m as i32);
    // class-count prediction: fraction of ordered m-tuples with class sum 0
    let mut conv: Vec<f64> = class_hist.iter().map(|&v| v as f64).collect();
    for _ in 1..m {
        let mut nxt = vec![0f64; h];
        for (a, &va) in conv.iter().enumerate() {
            for (b, &vb) in class_hist.iter().enumerate() {
                nxt[(a + b) % h] += va * vb as f64;
            }
        }
        conv = nxt;
    }
    let c_pred = conv[0] / (size as f64).powi(m as i32);
    let l_m = if m == 2 { 1.0 } else { size as f64 };
    let cost = if d > 0 {
        cols.len() as f64 * (g.r as f64 / d as f64) * l_m
    } else {
        f64::INFINITY
    };
    Census {
        size,
        columns: cols.len(),
        class_hist,
        d_in_g: d,
        c,
        c_pred,
        cost,
    }
}

/// All distinct `m`-sums of `base` (no class filter), as a set.
fn all_sums(f: &F, e: &ShortWeierstrass, base: &[WPoint], m: usize) -> HashSet<WPoint> {
    let mut cur: HashSet<WPoint> = HashSet::new();
    cur.insert(None);
    for _ in 0..m {
        let mut nxt = HashSet::new();
        for &s in &cur {
            for &b in base {
                nxt.insert(e.add(f, s, b));
            }
        }
        cur = nxt;
    }
    cur
}

struct Neighbour {
    ell: u64,
    psi: SmallIsogeny,
    dual: SmallIsogeny,
    g2: Group,
    base2: Vec<WPoint>,
    pullback: Vec<WPoint>,
    info: Value,
}

fn curve_identity(f: &F, e: &ShortWeierstrass, n: u64) -> (String, String) {
    curve_id::prime(
        &BigUint::from(f.p()),
        &BigUint::from(e.a),
        &BigUint::from(e.b),
        &BigUint::from(n),
    )
    .map(|id| (id.slug, id.icv1))
    .unwrap_or_default()
}

fn curve_json(f: &F, g: &Group) -> Value {
    let (slug, icv1) = curve_identity(f, &g.e, g.n);
    json!({
        "p": f.p(), "a": g.e.a, "b": g.e.b, "model": "y^2 = x^3 + a x + b",
        "order": g.n, "h": g.h, "r": g.r, "j": g.e.j_invariant(f),
        "slug": slug, "icv1": icv1,
    })
}

fn build_neighbour(
    f: &F,
    g: &Group,
    ell: u64,
    kernel_x: u64,
    size: usize,
    rng: &mut StdRng,
) -> Neighbour {
    let psi = SmallIsogeny::new(f, &g.e, ell, kernel_x).expect("kernel point");
    let dual = psi.dual(f).expect("certified dual");
    let e2 = *psi.codomain();
    let n2 = point_count(f, &e2);
    assert_eq!(n2, g.n, "isogenous curves have equal order");
    let g2 = Group::new(f, e2, n2, g.h, rng).expect("neighbour group data");
    // checks on random points
    let samples = 500;
    let mut dual_psi_ok = 0;
    let mut psi_dual_ok = 0;
    let mut hom_ok = 0;
    let mut on_ok = 0;
    for _ in 0..samples {
        let s = random_point(f, &g.e, rng);
        let t = random_point(f, &g.e, rng);
        let fs = psi.eval(f, s);
        if e2.contains(f, fs) && g.e.contains(f, dual.eval(f, fs)) {
            on_ok += 1;
        }
        if dual.eval(f, fs) == g.e.mul(f, s, ell as u128) {
            dual_psi_ok += 1;
        }
        if psi.eval(f, g.e.add(f, s, t)) == e2.add(f, fs, psi.eval(f, t)) {
            hom_ok += 1;
        }
        let s2 = random_point(f, &e2, rng);
        if psi.eval(f, dual.eval(f, s2)) == e2.mul(f, s2, ell as u128) {
            psi_dual_ok += 1;
        }
    }
    // rational kernel of ψ (pointwise): points of E(F_p) killed by ψ
    let kernel_pointwise_rational = f.sqrt(g.e.rhs(f, kernel_x)).is_some();
    // rational kernel of the dual: points of E′(F_p) killed by ψ̂
    let dual_kernel_x = dual.kernel_x();
    let dual_kernel_pointwise_rational = f.sqrt(e2.rhs(f, dual_kernel_x)).is_some();
    // image index of ψ̂: |ker ψ̂ ∩ E′(F_p)|
    let dual_rational_kernel_size = if dual_kernel_pointwise_rational {
        ell
    } else {
        1
    };
    let base2 = g2.small_x_base(f, size, |_| true);
    let mut seen = HashSet::new();
    let mut pullback = Vec::new();
    let mut to_infinity = 0usize;
    for &q in &base2 {
        let p = dual.eval(f, q);
        if p.is_none() {
            to_infinity += 1;
            continue;
        }
        if seen.insert(p) {
            pullback.push(p);
        }
    }
    let in_ell_e = pullback
        .iter()
        .filter(|&&p| g.cls(f, p).is_multiple_of(ell))
        .count();
    let (slug2, icv2) = curve_identity(f, &e2, n2);
    let info = json!({
        "ell": ell,
        "kernel_x": kernel_x,
        "kernel_pointwise_rational": kernel_pointwise_rational,
        "codomain": {"a": e2.a, "b": e2.b, "order": n2, "j": e2.j_invariant(f), "slug": slug2, "icv1": icv2},
        "dual_certified": true,
        "dual_kernel_x": dual_kernel_x,
        "dual_kernel_pointwise_rational": dual_kernel_pointwise_rational,
        "dual_image_index_predicted": dual_rational_kernel_size,
        "checks": {"samples": samples, "on_curve": on_ok, "homomorphism": hom_ok,
                   "dual_after_psi_is_ell": dual_psi_ok, "psi_after_dual_is_ell": psi_dual_ok},
        "neighbour_base_size": base2.len(),
        "pullback_size": pullback.len(),
        "pullback_points_mapped_to_infinity": to_infinity,
        "pullback_points_in_ellE": in_ell_e,
    });
    Neighbour {
        ell,
        psi,
        dual,
        g2,
        base2,
        pullback,
        info,
    }
}

fn census_json(c: &Census, g: &Group, m: usize) -> Value {
    json!({
        "m": m, "size": c.size, "unknowns": c.columns, "class_hist": c.class_hist,
        "D_in_G": c.d_in_g, "relations_per_target": c.d_in_g as f64 / g.r as f64,
        "class_factor": c.c, "class_factor_predicted_by_classes": c.c_pred,
        "class_factor_per_unknown": c.c / c.columns as f64,
        "cost_model": c.cost,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let family = arg(&args, "--family").unwrap_or_else(|| "j0".to_string());
    let size = arg_u64(&args, "--size", 144) as usize;
    let seed = arg_u64(&args, "--seed", 7);
    let sim_targets = arg_u64(&args, "--targets", 2000);
    let mut rng = StdRng::seed_from_u64(seed);

    // ---- the curve
    let (f, g, ells, kernel_xs): (F, Group, Vec<u64>, Vec<u64>) = match family.as_str() {
        "j0" => {
            let p = arg_u64(&args, "--p", 195709);
            let b = arg_u64(&args, "--b", 3);
            let f = F::new(p).expect("prime p");
            let e = ShortWeierstrass::new(&f, 0, b).expect("curve");
            let n = point_count(&f, &e);
            assert!(
                n.is_multiple_of(3) && !n.is_multiple_of(9),
                "N = 3r expected"
            );
            let g = Group::new(&f, e, n, 3, &mut rng).expect("N = 3r, r prime");
            assert!(f.sqrt(b).is_some(), "T = (0, sqrt b) must be rational");
            (f, g, vec![3], vec![0])
        }
        "6r" => {
            let pmin = arg_u64(&args, "--pmin", 1 << 20);
            let mut p = pmin + 1;
            while !is_prime(p) {
                p += 1;
            }
            let f = F::new(p).expect("prime p");
            let mut found = None;
            'search: for a in 1..40u64 {
                for b in 1..40u64 {
                    let Ok(e) = ShortWeierstrass::new(&f, a, b) else {
                        continue;
                    };
                    let two = kernel_x_coordinates(&f, &e, 2).expect("ell = 2");
                    if two.len() != 1 {
                        continue;
                    }
                    let n = point_count(&f, &e);
                    if !n.is_multiple_of(6) || n.is_multiple_of(4) || n.is_multiple_of(9) {
                        continue;
                    }
                    if !is_prime(n / 6) {
                        continue;
                    }
                    // pointwise-rational 3-kernel
                    let three = kernel_x_coordinates(&f, &e, 3).expect("ell = 3");
                    let Some(&x3) = three.iter().find(|&&x| f.sqrt(e.rhs(&f, x)).is_some()) else {
                        continue;
                    };
                    found = Some((e, n, two[0], x3));
                    break 'search;
                }
            }
            let (e, n, x2, x3) = found.expect("a curve with N = 6r");
            let g = Group::new(&f, e, n, 6, &mut rng).expect("N = 6r, r prime, cyclic 6-part");
            (f, g, vec![2, 3], vec![x2, x3])
        }
        other => panic!("unknown family {other}"),
    };
    let all_kernels: Vec<Value> = ells
        .iter()
        .map(|&l| {
            let roots = kernel_x_coordinates(&f, &g.e, l).unwrap_or_default();
            json!({"ell": l, "rational_kernel_x": roots})
        })
        .collect();

    // ---- neighbours and pulled-back bases
    let neighbours: Vec<Neighbour> = ells
        .iter()
        .zip(kernel_xs.iter())
        .map(|(&l, &x0)| build_neighbour(&f, &g, l, x0, size, &mut rng))
        .collect();
    let n_psi = neighbours.len() as f64; // |Ψ|, counted as neighbour pairs {ψ, ψ̂}

    // ---- arms
    let plain = g.small_x_base(&f, size, |_| true);
    let mut arms: Vec<(String, &Group, Vec<WPoint>)> = vec![("plain".into(), &g, plain.clone())];
    for nb in &neighbours {
        arms.push((format!("neighbour_l{}", nb.ell), &nb.g2, nb.base2.clone()));
        arms.push((format!("pullback_l{}", nb.ell), &g, nb.pullback.clone()));
        let l = nb.ell;
        arms.push((
            format!("filtered_l{}E", nb.ell),
            &g,
            g.small_x_base(&f, size, |k| k.is_multiple_of(l)),
        ));
    }
    let mut union: Vec<WPoint> = plain.clone();
    let mut union_pb: Vec<WPoint> = Vec::new();
    for nb in &neighbours {
        for &p in &nb.pullback {
            if !union.contains(&p) {
                union.push(p);
            }
            if !union_pb.contains(&p) {
                union_pb.push(p);
            }
        }
    }
    arms.push(("union".into(), &g, union));
    if neighbours.len() > 1 {
        arms.push(("union_pullbacks".into(), &g, union_pb));
    }

    let mut rows = Vec::new();
    let mut table: HashMap<(String, usize), Census> = HashMap::new();
    for (name, grp, base) in &arms {
        for m in [2usize, 3] {
            let c = census(&f, grp, base, m);
            let mut row = census_json(&c, grp, m);
            row["arm"] = json!(name);
            row["curve"] = json!(if std::ptr::eq(*grp, &g) { "E" } else { "E'" });
            rows.push(row);
            table.insert((name.clone(), m), c);
        }
    }

    // ---- exact transport identities per neighbour
    let mut identities = Vec::new();
    for nb in &neighbours {
        for m in [2usize, 3] {
            let d_far = table[&(format!("neighbour_l{}", nb.ell), m)].d_in_g;
            let d_pull = table[&(format!("pullback_l{}", nb.ell), m)].d_in_g;
            // distinct non-zero ψ̂-images of all m-sums of F′ (every coset)
            let far_sums = all_sums(&f, &nb.g2.e, &nb.base2, m);
            let far_sums_in_g2 = far_sums.iter().filter(|&&s| nb.g2.in_g(&f, s)).count();
            let images: HashSet<WPoint> = far_sums
                .iter()
                .map(|&s| nb.dual.eval(&f, s))
                .filter(|p| p.is_some())
                .collect();
            identities.push(json!({
                "ell": nb.ell, "m": m,
                "D_G'(F')": d_far,
                "distinct_m_sums_of_F'_all_cosets": far_sums.len(),
                "distinct_m_sums_of_F'_in_G'": far_sums_in_g2,
                "D_G(pullback)": d_pull,
                "distinct_nonzero_dual_images_of_all_m_sums": images.len(),
                "identity_D_G_pullback_equals_dual_images": d_pull == images.len(),
                "pullback_grows": d_pull >= d_far,
                "growth_ratio": d_pull as f64 / d_far.max(1) as f64,
            }));
        }
    }

    // ---- sampled relation-transport simulation (m = 2 and 3)
    let gen = loop {
        let q = g.e.mul(&f, random_point(&f, &g.e, &mut rng), g.h as u128);
        if q.is_some() {
            break q;
        }
    };
    assert!(g.e.mul(&f, gen, g.r as u128).is_none());
    let mut sims = Vec::new();
    for nb in &neighbours {
        let gen2 = nb.psi.eval(&f, gen); // λ_ψ = 1 by normalisation
        let pairs2: HashMap<WPoint, (usize, usize)> = {
            let mut t = HashMap::new();
            for i in 0..nb.base2.len() {
                for j in i..nb.base2.len() {
                    let s = nb.g2.e.add(&f, nb.base2[i], nb.base2[j]);
                    t.entry(s).or_insert((i, j));
                }
            }
            t
        };
        let pairs_pb: HashSet<WPoint> = {
            let mut t = HashSet::new();
            for i in 0..nb.pullback.len() {
                for j in i..nb.pullback.len() {
                    t.insert(g.e.add(&f, nb.pullback[i], nb.pullback[j]));
                }
            }
            t
        };
        let mut agm_ok = 0u64;
        let mut a_hits = [0u64; 2];
        let mut b_hits = [0u64; 2];
        let mut relation_ok = [0u64; 2];
        let mut b_covers_a = [0u64; 2];
        for _ in 0..sim_targets {
            let x = rng.gen_range(1..g.r);
            let target = g.e.mul(&f, gen, x as u128);
            let far = nb.psi.eval(&f, target);
            // Theorem 1: the algebraic query is answered by x·ψ(P)
            if far == nb.g2.e.mul(&f, gen2, x as u128) {
                agm_ok += 1;
            }
            let ell_target = g.e.mul(&f, target, nb.ell as u128);
            for (mi, m) in [2usize, 3].iter().enumerate() {
                // A: decompose ψ(R) over F′ on E′
                let dec: Option<Vec<WPoint>> = if *m == 2 {
                    pairs2
                        .get(&far)
                        .map(|&(i, j)| vec![nb.base2[i], nb.base2[j]])
                } else {
                    nb.base2.iter().find_map(|&q| {
                        let rest = nb.g2.e.add(&f, far, nb.g2.e.neg(&f, q));
                        pairs2
                            .get(&rest)
                            .map(|&(i, j)| vec![nb.base2[i], nb.base2[j], q])
                    })
                };
                // B: decompose [ℓ]R over ψ̂(F′) on E
                let b_hit = if *m == 2 {
                    pairs_pb.contains(&ell_target)
                } else {
                    nb.pullback.iter().any(|&q| {
                        let rest = g.e.add(&f, ell_target, g.e.neg(&f, q));
                        pairs_pb.contains(&rest)
                    })
                };
                if b_hit {
                    b_hits[mi] += 1;
                }
                if let Some(qs) = dec {
                    a_hits[mi] += 1;
                    // pulled-back relation: Σ ψ̂(Q_i) = ψ̂(ψ(R)) = [ℓ]R
                    let mut s: WPoint = None;
                    for q in qs {
                        s = g.e.add(&f, s, nb.dual.eval(&f, q));
                    }
                    if s == ell_target {
                        relation_ok[mi] += 1;
                    }
                    if b_hit {
                        b_covers_a[mi] += 1;
                    }
                }
            }
        }
        sims.push(json!({
            "ell": nb.ell, "targets": sim_targets,
            "theorem1_simulated_answers_agree": agm_ok,
            "m2": {"A_hits_on_E'": a_hits[0], "pulled_back_relation_holds": relation_ok[0],
                   "B_hits_on_E_pullback_base": b_hits[0], "B_covers_A": b_covers_a[0]},
            "m3": {"A_hits_on_E'": a_hits[1], "pulled_back_relation_holds": relation_ok[1],
                   "B_hits_on_E_pullback_base": b_hits[1], "B_covers_A": b_covers_a[1]},
        }));
    }

    // ---- falsifier: union class factor per unknown vs best single base
    let mut falsifier = Vec::new();
    for m in [2usize, 3] {
        let single: Vec<(String, f64)> = std::iter::once("plain".to_string())
            .chain(neighbours.iter().map(|nb| format!("pullback_l{}", nb.ell)))
            .map(|name| {
                let c = &table[&(name.clone(), m)];
                (name, c.c / c.columns as f64)
            })
            .collect();
        let (best_name, best) =
            single.iter().cloned().fold(
                (String::new(), 0.0),
                |acc, v| if v.1 > acc.1 { v } else { acc },
            );
        let u = &table[&("union".to_string(), m)];
        let union_per_unknown = u.c / u.columns as f64;
        let ratio = union_per_unknown / best;
        let best_cost = single
            .iter()
            .map(|(name, _)| table[&(name.clone(), m)].cost)
            .fold(f64::INFINITY, f64::min);
        let plain_cost = table[&("plain".to_string(), m)].cost;
        falsifier.push(json!({
            "m": m,
            "single_bases_class_factor_per_unknown": single,
            "best_single": best_name, "best_single_class_factor_per_unknown": best,
            "union_class_factor_per_unknown": union_per_unknown,
            "ratio_union_over_best": ratio,
            "psi_count": n_psi,
            "falsifier_fires": ratio > n_psi,
            "union_cost_over_plain_cost": u.cost / plain_cost,
            "union_cost_over_best_single_cost": u.cost / best_cost,
            "union_size_over_plain_size": u.size as f64 / table[&("plain".to_string(), m)].size as f64,
        }));
    }

    let out = json!({
        "schema": "r6_8_transport_oracle/v1",
        "family": family, "seed": seed, "size": size,
        "curve_E": curve_json(&f, &g),
        "rational_kernels": all_kernels,
        "neighbours": neighbours.iter().map(|nb| nb.info.clone()).collect::<Vec<_>>(),
        "census": rows,
        "transport_identities": identities,
        "relation_transport_simulation": sims,
        "falsifier": falsifier,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
