//! Round-7 R7.1: the smooth GHS descended curve for m >= 2 and R6.9's
//! relation-rate measurement redone on the real descended Jacobian.
//!
//! Library: `crypto_lib::cryptanalysis::ghs_smooth` (construction, Phi, point
//! counts).  This example only selects curves, checks them, and measures.
//!
//! Subcommands (JSON on `--out`):
//!   scan  L N          census of (a, beta) with a in {0,1}: magic number, genus, type
//!   run   L N [--curves C] [--samples S] [--out F]
//!                      pick C curves with magic >= 2, verify the construction
//!                      (genus, smoothness, #J, divisibility, homomorphism, order of
//!                      Phi(G), Frobenius relation) and run the relation-rate test
//!                      on the real Jacobian: (A) walk on J0 = <points, Phi(E(K))> as in
//!                      R6.9, (B) exact enumeration + walk in H = <Phi(E(K))>.

#![recursion_limit = "1024"]

use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;

use crypto_lib::binary_ecc::{F2mElement, F2mPoly, IrreduciblePoly, MumfordDivisor};
use crypto_lib::cryptanalysis::descent_expansion::enumerate_irreducibles;
use crypto_lib::cryptanalysis::ec_trapdoor::{ghs_type_i, magic_number_full, FieldTower};
use crypto_lib::cryptanalysis::ghs_descent::{ECurve, Pt};
use crypto_lib::cryptanalysis::ghs_smooth::{
    count_e_points, count_points_extension, jacobian_from_counts, lift_x, predicted_count,
    CoverModel, SmoothCover, SmoothCoverError,
};
use num_bigint::BigUint;
use serde_json::{json, Value};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn factor(mut x: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut p = 2;
    while p * p <= x {
        if x % p == 0 {
            let mut e = 0;
            while x % p == 0 {
                x /= p;
                e += 1;
            }
            out.push((p, e));
        }
        p += 1;
    }
    if x > 1 {
        out.push((x, 1));
    }
    out
}

fn kirr(l: u32) -> IrreduciblePoly {
    if l == 1 {
        IrreduciblePoly {
            degree: 1,
            low_terms: vec![0],
        }
    } else {
        enumerate_irreducibles(l, 1).remove(0)
    }
}

fn setup(l: u32, n: u32) -> FieldTower {
    let nn = l * n;
    FieldTower::new(nn, n, l, enumerate_irreducibles(nn, 1).remove(0))
}

// ── scan ───────────────────────────────────────────────────────────────

fn scan(l: u32, n: u32) -> Value {
    let tower = setup(l, n);
    let nn = tower.big_n;
    let mut table: BTreeMap<(u32, bool), u64> = BTreeMap::new();
    for bits in 1u64..(1u64 << nn) {
        let beta = F2mElement::from_words(&[bits], nn);
        let b = beta.square(&tower.irr);
        let m = magic_number_full(&tower, &F2mElement::zero(nn), &b);
        let ti = ghs_type_i(&tower, &b);
        *table.entry((m, ti)).or_default() += 1;
    }
    let rows: Vec<Value> = table
        .iter()
        .map(|(&(m, ti), &c)| {
            let g = if m == 0 {
                0
            } else if ti {
                (1u32 << (m - 1)) - 1
            } else {
                1u32 << (m - 1)
            };
            json!({"magic": m, "type_I": ti, "genus": g, "count_b": c})
        })
        .collect();
    json!({"l": l, "n": n, "N": nn, "irr_K": format!("{:?}", tower.irr.low_terms), "rows": rows})
}

// ── polynomial helpers over F_q (library representation) ──────────────

fn elems(l: u32) -> Vec<F2mElement> {
    (0..(1u64 << l)).map(|w| F2mElement::from_words(&[w], l)).collect()
}

/// u splits into linear factors over F_q (multiplicity counted).
fn splits(u: &F2mPoly, irr: &IrreduciblePoly, es: &[F2mElement]) -> bool {
    let mut p = u.clone();
    let l = irr.degree;
    for x in es {
        loop {
            if p.degree().unwrap_or(0) == 0 {
                return true;
            }
            if !p.eval(x, irr).is_zero() {
                break;
            }
            let lin = F2mPoly::from_coeffs(vec![x.clone(), F2mElement::one(l)], l);
            p = p.divrem(&lin, irr).0;
        }
    }
    p.degree().unwrap_or(0) == 0
}

/// Number of fully split reduced divisors (R6.9's exact model).
fn split_count(ram: u64, pairs: u64, g: usize) -> u128 {
    let mut c = vec![0u128; g + 1];
    c[0] = 1;
    for _ in 0..ram {
        for d in (1..=g).rev() {
            c[d] += c[d - 1];
        }
    }
    for _ in 0..pairs {
        let old = c.clone();
        for d in 1..=g {
            let mut s = 0u128;
            for k in 1..=d {
                s += 2 * old[d - k];
            }
            c[d] = old[d] + s;
        }
    }
    c.iter().sum()
}

fn ln_factorial(g: u64) -> f64 {
    (1..=g).map(|i| (i as f64).ln()).sum()
}

// ── small modular polynomial arithmetic for the Frobenius check ────────

fn mulmod(a: u128, b: u128, p: u128) -> u128 {
    a * b % p
}

fn powmod(mut a: u128, mut e: u64, p: u128) -> u128 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, p);
        }
        a = mulmod(a, a, p);
        e >>= 1;
    }
    r
}

fn pgcd_deg(mut a: Vec<u128>, mut b: Vec<u128>, p: u128) -> usize {
    let trim = |v: &mut Vec<u128>| {
        while v.last() == Some(&0) {
            v.pop();
        }
    };
    trim(&mut a);
    trim(&mut b);
    while !b.is_empty() {
        let inv = powmod(*b.last().unwrap(), (p - 2) as u64, p);
        while a.len() >= b.len() {
            let c = mulmod(*a.last().unwrap(), inv, p);
            let sh = a.len() - b.len();
            for i in 0..b.len() {
                a[sh + i] = (a[sh + i] + p - mulmod(c, b[i], p)) % p;
            }
            trim(&mut a);
            if a.is_empty() {
                break;
            }
        }
        std::mem::swap(&mut a, &mut b);
    }
    a.len().saturating_sub(1)
}

/// Degree of gcd(chi(x), x^{2n} - t x^n + q^n) in F_p[x], chi the
/// characteristic polynomial of Frobenius on Jac(C)/F_q (reverse of L).
fn frobenius_gcd_degree(l_poly: &[i128], n: u32, t_e: i128, qn: i128, p: u128) -> usize {
    let md = |v: i128| -> u128 { v.rem_euclid(p as i128) as u128 };
    let d = l_poly.len() - 1;
    let chi: Vec<u128> = (0..=d).map(|i| md(l_poly[d - i])).collect();
    let mut psi = vec![0u128; 2 * n as usize + 1];
    psi[2 * n as usize] = 1;
    psi[n as usize] = md(-t_e);
    psi[0] = md(qn);
    pgcd_deg(chi, psi, p)
}

// ── group helpers ──────────────────────────────────────────────────────

fn mkey(d: &MumfordDivisor) -> Vec<u64> {
    let mut k = Vec::new();
    for c in &d.u.coeffs {
        k.push(c.raw_bits()[0]);
    }
    k.push(u64::MAX);
    for c in &d.v.coeffs {
        k.push(c.raw_bits()[0]);
    }
    k
}

fn order_of(d: &MumfordDivisor, bound: u64, curve: &crypto_lib::binary_ecc::HyperellipticCurve) -> Option<u64> {
    let mut acc = d.clone();
    for k in 1..=bound {
        if acc.is_identity() {
            return Some(k);
        }
        acc = acc.add(d, curve);
    }
    None
}

/// Closure of the subgroup generated by `gens`.
fn subgroup(
    gens: &[MumfordDivisor],
    curve: &crypto_lib::binary_ecc::HyperellipticCurve,
    cap: usize,
) -> Vec<MumfordDivisor> {
    let mut elems = vec![MumfordDivisor::identity(curve.m)];
    let mut seen: HashSet<Vec<u64>> = HashSet::new();
    seen.insert(mkey(&elems[0]));
    let mut i = 0;
    while i < elems.len() && elems.len() < cap {
        for g in gens {
            let s = elems[i].add(g, curve);
            if seen.insert(mkey(&s)) {
                elems.push(s);
            }
        }
        i += 1;
    }
    elems
}

// ── per-curve run ──────────────────────────────────────────────────────

fn run_curve(
    tower: &FieldTower,
    a: &F2mElement,
    beta: &F2mElement,
    samples: u64,
    seed: u64,
) -> Result<Value, SmoothCoverError> {
    let nn = tower.big_n;
    let l = tower.l;
    let n = tower.n;
    let q = 1i128 << l;
    let b = beta.square(&tower.irr);
    let cov = SmoothCover::new(tower, a, beta)?;
    let g = cov.genus as usize;
    let cur = &cov.curve_k;
    let es = elems(l);
    let kidx = kirr(l);
    assert_eq!(kidx.low_terms, cov.k_irr.low_terms);
    let mut rng = Rng(seed | 1);

    // magic number / genus cross-check
    let magic_lib = magic_number_full(tower, a, &b);
    let type_i = ghs_type_i(tower, &b);

    // point counts N_1..N_{g+1}; L-polynomial from N_1..N_g; N_{g+1} is the independent genus check
    let counts: Vec<i128> = (1..=g as u32 + 1)
        .map(|k| count_points_extension(cur, k))
        .collect();
    let jd = jacobian_from_counts(q, g, &counts[..g]);
    let pred = predicted_count(q, &jd.l_poly, g + 1);
    let n_e = count_e_points(tower, a, &b);
    let t_e = (1i128 << nn) + 1 - n_e as i128;
    let facs = factor(n_e);
    let ell = facs.last().map(|x| x.0).unwrap_or(1);
    let gcd1 = frobenius_gcd_degree(&jd.l_poly, n, t_e, 1i128 << nn, (1u128 << 61) - 1);
    let gcd2 = frobenius_gcd_degree(&jd.l_poly, n, t_e, 1i128 << nn, 1_000_000_007);
    // negative controls: perturb the trace
    let ctrl1 = frobenius_gcd_degree(&jd.l_poly, n, t_e + 1, 1i128 << nn, (1u128 << 61) - 1);
    let ctrl2 = frobenius_gcd_degree(&jd.l_poly, n, t_e - 1, 1i128 << nn, (1u128 << 61) - 1);

    // E(K) points with x != 0
    let ec = ECurve::new(nn, tower.irr.clone(), a.clone(), b.clone());
    let mut pts: Vec<Pt> = Vec::new();
    while pts.len() < 40 {
        let x = F2mElement::from_words(&[rng.next() % (1u64 << nn)], nn);
        pts.extend(lift_x(tower, a, &b, &x));
    }
    // homomorphism checks
    let (mut hom_ok, mut hom_fail, mut hom_skipped) = (0u32, 0u32, 0u32);
    for w in pts.windows(2).take(20) {
        let sum = ec.add(&w[0], &w[1]);
        match (cov.image(&w[0]), cov.image(&w[1]), cov.image(&sum)) {
            (Ok(a1), Ok(b1), Ok(s1)) => {
                if a1.add(&b1, cur) == s1 {
                    hom_ok += 1
                } else {
                    hom_fail += 1
                }
            }
            _ => hom_skipped += 1,
        }
    }
    // [#E] Phi(P) = 0 and order of image of a generator of the l-part
    let mut ord_checks = 0u32;
    let mut ord_fail = 0u32;
    let mut images: Vec<MumfordDivisor> = Vec::new();
    for p in pts.iter().take(12) {
        if let Ok(f) = cov.image(p) {
            if !f.scalar_mul(&BigUint::from(n_e), cur).is_identity() {
                ord_fail += 1;
            }
            ord_checks += 1;
            images.push(f);
        }
    }
    // l-part generator
    let cof = n_e / ell;
    let mut ell_gen: Option<(Pt, MumfordDivisor)> = None;
    for p in &pts {
        let gp = ec.scalar_mul(p, &BigUint::from(cof));
        if matches!(gp, Pt::Inf) {
            continue;
        }
        if let Ok(f) = cov.image(&gp) {
            ell_gen = Some((gp, f));
            break;
        }
    }
    let (ell_div_j, ell_img_order) = match &ell_gen {
        Some((_, f)) => (
            jd.order % ell as i128 == 0,
            order_of(f, ell.min(100_000), cur),
        ),
        None => (false, None),
    };
    // is the image of the l-part nontrivial and of order exactly l?
    let jac_big = BigUint::from(jd.order as u128);
    let jac_check = images
        .iter()
        .all(|f| f.scalar_mul(&jac_big, cur).is_identity());

    // ── relation rate ──
    let hc = &cur.h;
    let fc = &cur.f;
    let mut ram = 0u64;
    let mut affine_pts: Vec<(F2mElement, F2mElement)> = Vec::new();
    for x in &es {
        let hx = hc.eval(x, &cur.irr);
        let fx = fc.eval(x, &cur.irr);
        for y in &es {
            if y.square(&cur.irr).add(&hx.mul(y, &cur.irr)) == fx {
                affine_pts.push((x.clone(), y.clone()));
            }
        }
        if hx.is_zero() {
            ram += 1;
        }
    }
    let pairs = (affine_pts.len() as u64 - ram) / 2;
    let sc = split_count(ram, pairs, g);
    let exact_j = sc as f64 / jd.order as f64;
    let heur = (-ln_factorial(g as u64)).exp();

    let walk = |rng: &mut Rng, pool: &dyn Fn(&mut Rng) -> MumfordDivisor, steps: u64| -> (u64, usize) {
        let r = 20usize;
        let rs: Vec<MumfordDivisor> = (0..r).map(|_| pool(rng)).collect();
        let mut w = pool(rng);
        let mut smooth = 0u64;
        let mut seen: HashSet<Vec<u64>> = HashSet::new();
        // burn-in so the count starts from a mixed position, not from the pool's first element
        for _ in 0..(steps / 5).max(2000) {
            w = w.add(&rs[(rng.next() % r as u64) as usize], cur);
        }
        for _ in 0..steps {
            if splits(&w.u, &cur.irr, &es) {
                smooth += 1;
            }
            seen.insert(mkey(&w));
            w = w.add(&rs[(rng.next() % r as u64) as usize], cur);
        }
        (smooth, seen.len())
    };

    // (A2) subgroup of Jac(C)(F_q) generated by the divisors of affine F_q-points and Phi(E(K)):
    // exact enumeration; when it equals Jac(C)(F_q) it checks the closed-form split count independently
    let mut gens_j: Vec<MumfordDivisor> = affine_pts
        .iter()
        .map(|(x, y)| MumfordDivisor::from_point(cur, x, y).expect("point"))
        .collect();
    gens_j.extend(images.iter().cloned());
    let jgen = subgroup(&gens_j, cur, 400_000);
    let jgen_split = jgen.iter().filter(|d| splits(&d.u, &cur.irr, &es)).count();
    let jgen_json = json!({
        "generators": gens_j.len(), "size": jgen.len(), "jacobian_order": jd.order.to_string(),
        "equals_whole_jacobian": jgen.len() as i128 == jd.order,
        "split_elements": jgen_split,
        "exact_split_fraction": jgen_split as f64 / jgen.len() as f64,
        "index_in_jacobian": if jgen.len() > 0 && jd.order as usize % jgen.len() == 0 { Value::from(jd.order as usize / jgen.len()) } else { Value::Null },
        "all_closed_form_split_divisors_lie_in_J0": jgen_split as u128 == sc,
    });

    // (A) walk on the subgroup J0 of Jac(C)(F_q) generated by the affine F_q-point divisors and
    // Phi(E(K)), whose elements are enumerated exactly; the walk steps are random elements of J0
    let pool_j = |rng: &mut Rng| -> MumfordDivisor {
        jgen[(rng.next() as usize) % jgen.len()].clone()
    };
    let a_part = if !jgen.is_empty() {
        let (sm, vis) = walk(&mut rng, &pool_j, samples);
        let meas = sm as f64 / samples as f64;
        json!({
            "walk_steps": samples, "smooth_steps": sm, "measured_rate": meas,
            "measured_rate_se": (meas * (1.0 - meas) / samples as f64).sqrt(),
            "exact_split_fraction_in_J0": jgen_split as f64 / jgen.len() as f64,
            "ratio_measured_over_exact_J0": meas / (jgen_split as f64 / jgen.len() as f64),
            "ratio_measured_over_exact_J": meas / exact_j,
            "ratio_measured_over_heuristic": meas / heur,
            "distinct_visited": vis, "visited_fraction_of_J0": vis as f64 / jgen.len() as f64,
        })
    } else {
        json!({"skipped": "empty subgroup"})
    };

    // (B) subgroup H = <Phi(P_i)>: exact enumeration + walk
    let h_elems = subgroup(&images, cur, 20_000);
    let h_size = h_elems.len();
    let h_split = h_elems.iter().filter(|d| splits(&d.u, &cur.irr, &es)).count();
    let h_exact = h_split as f64 / h_size as f64;
    let pool_h = |rng: &mut Rng| -> MumfordDivisor {
        h_elems[(rng.next() as usize) % h_size].clone()
    };
    let (sm_h, vis_h) = walk(&mut rng, &pool_h, samples);
    let meas_h = sm_h as f64 / samples as f64;
    // l-part subgroup
    let ell_part = match &ell_gen {
        Some((_, f)) => {
            let sub = subgroup(std::slice::from_ref(f), cur, 20_000);
            let sp = sub.iter().filter(|d| splits(&d.u, &cur.irr, &es)).count();
            json!({"size": sub.len(), "split": sp, "exact_split_fraction": sp as f64 / sub.len() as f64})
        }
        None => Value::Null,
    };
    let mut deg_hist = vec![0u64; g + 1];
    for d in &h_elems {
        deg_hist[d.u.degree().unwrap_or(0)] += 1;
    }

    Ok(json!({
        "l": l, "n": n, "N": nn, "q": q as u64,
        "a": cov.a.raw_bits()[0], "beta": beta.raw_bits()[0],
        "magic_number": cov.magic, "magic_number_ec_trapdoor": magic_lib,
        "type_I_ec_trapdoor": type_i, "c1_is_zero": cov.c1.is_zero(),
        "model": match cov.model { CoverModel::V => "v-model (c1!=0)", CoverModel::T => "t-model (c1=0)" },
        "genus": g,
        "h": cur.h.coeffs.iter().map(|c| c.raw_bits()[0]).collect::<Vec<_>>(),
        "f": cur.f.coeffs.iter().map(|c| c.raw_bits()[0]).collect::<Vec<_>>(),
        "smoothness_checked_by_library": true,
        "point_counts_N_k": counts.iter().map(|&x| x as i64).collect::<Vec<_>>(),
        "N_gplus1_predicted_from_L": pred as i64,
        "genus_confirmed_by_point_count": pred == counts[g],
        "l_poly": jd.l_poly.iter().map(|&x| x.to_string()).collect::<Vec<_>>(),
        "jacobian_order": jd.order.to_string(),
        "E_order": n_e, "E_trace": t_e as i64, "E_order_factorisation": facs,
        "largest_prime_factor": ell,
        "ell_divides_jacobian_order": ell_div_j,
        "ell_part_image_order": ell_img_order,
        "E_order_divides_jacobian_order": jd.order % n_e as i128 == 0,
        "frobenius_relation_gcd_degree": [gcd1, gcd2],
        "frobenius_relation_control_gcd_degree": [ctrl1, ctrl2],
        "homomorphism_checks_ok": hom_ok, "homomorphism_checks_failed": hom_fail,
        "homomorphism_checks_skipped": hom_skipped,
        "order_checks": ord_checks, "order_check_failures": ord_fail,
        "jacobian_order_annihilates_images": jac_check,
        "affine_points": affine_pts.len(), "ramified_points": ram, "point_pairs": pairs,
        "split_reduced_divisors_exact": sc.to_string(),
        "model_exact_split_fraction": exact_j,
        "model_heuristic_inv_gfact": heur,
        "rate_A_walk_on_J0": a_part,
        "subgroup_generated_by_points_and_images": jgen_json,
        "rate_B_subgroup_H": {
            "H_order": h_size, "H_over_J": h_size as f64 / jd.order as f64,
            "exact_split_fraction_in_H": h_exact,
            "split_in_H": h_split,
            "walk_steps": samples, "smooth_steps": sm_h,
            "measured_rate": meas_h,
            "measured_rate_se": (meas_h * (1.0 - meas_h) / samples as f64).sqrt(),
            "ratio_exactH_over_exactJ": h_exact / exact_j,
            "ratio_measured_over_exactH": meas_h / h_exact,
            "ratio_measured_over_heuristic": meas_h / heur,
            "distinct_visited": vis_h, "deg_u_histogram_H": deg_hist,
        },
        "rate_B_ell_part": ell_part,
    }))
}

fn arg(args: &[String], name: &str, default: &str) -> String {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let out = arg(&args, "--out", "");
    let mode = args.first().cloned().unwrap_or_default();
    let l: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(2);
    let n: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(6);
    let result = match mode.as_str() {
        "scan" => scan(l, n),
        "run" => {
            let ncurves: usize = arg(&args, "--curves", "4").parse().unwrap();
            let samples: u64 = arg(&args, "--samples", "40000").parse().unwrap();
            let want_m: u32 = arg(&args, "--magic", "0").parse().unwrap();
            let tower = setup(l, n);
            let nn = tower.big_n;
            let mut chosen: Vec<(u64, u64)> = Vec::new();
            let mut canon: HashSet<u64> = HashSet::new();
            // prefer variety: one candidate per (genus type, #E) first
            let mut seen_class: HashSet<(u32, bool, u64)> = HashSet::new();
            'outer: for pass in 0..2 {
                for a_bit in [0u64, 1] {
                    let a = F2mElement::from_words(&[a_bit], nn);
                    for bits in 1u64..(1u64 << nn) {
                        let beta = F2mElement::from_words(&[bits], nn);
                        // canonical under absolute Frobenius
                        let mut c = bits;
                        let mut cur = beta.clone();
                        for _ in 0..nn {
                            cur = cur.square(&tower.irr);
                            c = c.min(cur.raw_bits()[0]);
                        }
                        let b = beta.square(&tower.irr);
                        let m = magic_number_full(&tower, &a, &b);
                        if m < 2 || (want_m != 0 && m != want_m) || canon.contains(&(c * 2 + a_bit)) {
                            continue;
                        }
                        let ti = ghs_type_i(&tower, &b);
                        let ne = count_e_points(&tower, &a, &b);
                        let lp = factor(ne).last().map(|x| x.0).unwrap_or(1);
                        if lp < 20 {
                            continue;
                        }
                        let class = (m, ti, ne);
                        if pass == 0 && !seen_class.insert((m, ti, 0)) {
                            continue;
                        }
                        if pass == 1 && !seen_class.insert(class) {
                            continue;
                        }
                        canon.insert(c * 2 + a_bit);
                        chosen.push((a_bit, bits));
                        if chosen.len() >= ncurves {
                            break 'outer;
                        }
                    }
                }
            }
            let mut runs = Vec::new();
            for (i, &(a_bit, bits)) in chosen.iter().enumerate() {
                let a = F2mElement::from_words(&[a_bit], nn);
                let beta = F2mElement::from_words(&[bits], nn);
                eprintln!("curve {i}: a={a_bit} beta={bits:#x}");
                match run_curve(&tower, &a, &beta, samples, 0x7a11 + i as u64) {
                    Ok(v) => runs.push(v),
                    Err(e) => runs.push(json!({"a": a_bit, "beta": bits, "error": format!("{e:?}")})),
                }
            }
            json!({"l": l, "n": n, "N": nn, "irr_K_low_terms": tower.irr.low_terms, "samples": samples, "curves": runs})
        }
        _ => {
            eprintln!("usage: scan L N | run L N [--curves C] [--samples S] [--magic M] [--out F]");
            std::process::exit(2);
        }
    };
    let text = serde_json::to_string_pretty(&result).unwrap();
    if out.is_empty() {
        println!("{text}");
    } else {
        fs::write(&out, text).unwrap();
    }
}
