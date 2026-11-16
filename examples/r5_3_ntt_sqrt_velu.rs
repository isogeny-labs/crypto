//! Round 5, lead R5.3: the vertical-margin crossover with asymptotically
//! fast √élu.
//!
//! Round-4 lead 16.3 measured √élu at a cost exponent of 0.82 in `ℓ` because
//! its polynomial products were Karatsuba, and generic kernel finding (an
//! `x`-only ladder over `F_{p^k}`, `k = (ℓ − 1)/2`) at `ℓ^2.6`.  This example
//! re-measures both with the library's three-prime NTT
//! (`crypto_lib::cryptanalysis::isogeny_eval::ntt`) and with the round-4
//! Karatsuba arithmetic side by side, on the same `j = −3375` vertical
//! instances, and recomputes the §17.3 crossover table.
//!
//! Instances.  The generator (`make_instance`, `finish_instance`,
//! `find_rational_kernel`) is ported from
//! `examples/r4_16_3_vertical_margin.rs` unchanged in behaviour: the same
//! RNG calls in the same order with the same seeds, so the same `(ℓ, seed)`
//! gives the same prime, curve and kernel point (checked against the
//! round-4 records in `analyze`).  Only the field type changed (the
//! library's counted `PrimeField` instead of the example's own).
//!
//! Unit.  `M` = one multiplication or squaring in `F_p` (61-bit `p`),
//! including every coefficient product inside schoolbook/Karatsuba
//! polynomial arithmetic; `W` = one word multiplication (Montgomery product)
//! modulo a 62-bit NTT prime inside an NTT product, converted at `1 W = 1 M`
//! (stated, see the `ntt` module; `threshold` mode records the measured wall
//! ratio); `I` = one `F_p` inversion, converted at `I = 91 M` as in round 4.
//! `Meq = M + W + 91 I`.  Additions are not counted.  Wall times are
//! contended (four agents share four cores).
//!
//! Usage:
//!   r5_3_ntt_sqrt_velu threshold <out.json>
//!   r5_3_ntt_sqrt_velu rational <min_log2> <max_log2> <seeds> <velu_max_log2> <out.json>
//!   r5_3_ntt_sqrt_velu generic <min_ell> <max_ell> <step_log2> <cap_s> <karatsuba|auto|ntt> <out.json>
//!   r5_3_ntt_sqrt_velu analyze <dir> <round4_dir> <out.json>

use crypto_lib::cryptanalysis::isogeny_eval::montgomery::{xadd, xdbl, FieldArith};
use crypto_lib::cryptanalysis::isogeny_eval::ntt::{mul_ntt, ntt_mul_cost};
use crypto_lib::cryptanalysis::isogeny_eval::poly::{
    self, mul_karatsuba, KARATSUBA_THRESHOLD, NTT_THRESHOLD,
};
use crypto_lib::cryptanalysis::isogeny_eval::velu::kernel_polynomial;
use crypto_lib::cryptanalysis::isogeny_eval::{
    sqrt_velu, velu, MontgomeryCurve, OpCounts, OpTally, PolyMul, PrimeField, XZ,
};
use crypto_lib::cryptanalysis::isogeny_walk::curve::Model;
use crypto_lib::cryptanalysis::isogeny_walk::field::Field as LField;
use crypto_lib::cryptanalysis::isogeny_walk::kernel;
use num_bigint::{BigInt, BigUint};
use num_traits::{Signed, ToPrimitive, Zero};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::time::Instant;

type Fp = PrimeField<OpCounts>;

const INV_COST: u64 = 91;

fn meq_of(t: &OpTally) -> u64 {
    t.mul_equivalents(INV_COST)
}

fn tjs(t: &OpTally) -> Value {
    json!({"M": t.mul, "W": t.word, "I": t.inv, "Meq": meq_of(t)})
}

fn field(p: u64, m: PolyMul) -> Fp {
    PrimeField::counted(p)
        .expect("prime in [5, 2^63)")
        .with_poly_mul(m)
}

fn red(f: &Fp, v: i128) -> u64 {
    v.rem_euclid(f.p() as i128) as u64
}

fn inv(f: &Fp, a: u64) -> u64 {
    f.inv(a).expect("inverse of zero")
}

fn trim(mut a: Vec<u64>) -> Vec<u64> {
    poly::trim(&mut a);
    a
}

fn j_of_a(f: &Fp, a: u64) -> u64 {
    let a2 = f.mul(a, a);
    let t = f.sub(a2, 3);
    let num = f.mul(256, f.mul(t, f.mul(t, t)));
    f.mul(num, inv(f, f.sub(a2, 4)))
}

// ---------------------------------------------------------------------------
// Number theory helpers (as in round 4).

fn is_prime_u64(n: u64) -> bool {
    crypto_lib::cryptanalysis::isogeny_eval::counted_fp::is_prime_u64(n)
}

fn next_prime(mut n: u64) -> u64 {
    while !is_prime_u64(n) {
        n += 1;
    }
    n
}

fn primitive_root(l: u64) -> u64 {
    let mut fac = Vec::new();
    let mut m = l - 1;
    let mut d = 2;
    while d * d <= m {
        if m.is_multiple_of(d) {
            fac.push(d);
            while m.is_multiple_of(d) {
                m /= d;
            }
        }
        d += 1;
    }
    if m > 1 {
        fac.push(m);
    }
    let pw = |a: u64, mut e: u64| {
        let mut r = 1u128;
        let mut b = a as u128;
        while e > 0 {
            if e & 1 == 1 {
                r = r * b % l as u128;
            }
            b = b * b % l as u128;
            e >>= 1;
        }
        r as u64
    };
    (2..l)
        .find(|&g| fac.iter().all(|&q| pw(g, (l - 1) / q) != 1))
        .unwrap()
}

fn val(n: &BigUint, l: u64) -> (u32, BigUint) {
    let lb = BigUint::from(l);
    let mut n = n.clone();
    let mut e = 0;
    while !n.is_zero() && (&n % &lb).is_zero() {
        n /= &lb;
        e += 1;
    }
    (e, n)
}

// ---------------------------------------------------------------------------
// x-only ladder with a big scalar, generic over the library's FieldArith.

fn ladder_big<F: FieldArith>(f: &F, p: &XZ<F::Elem>, n: &BigUint, a24: &F::Elem) -> XZ<F::Elem> {
    if n.is_zero() {
        return XZ {
            x: p.x.clone(),
            z: f.zero(),
        };
    }
    let mut r0 = p.clone();
    let mut r1 = xdbl(f, p, a24);
    for i in (0..n.bits() - 1).rev() {
        if n.bit(i) {
            r0 = xadd(f, &r1, &r0, p);
            r1 = xdbl(f, &r1, a24);
        } else {
            r1 = xadd(f, &r1, &r0, p);
            r0 = xdbl(f, &r0, a24);
        }
    }
    r0
}

fn affine(f: &Fp, p: &XZ<u64>) -> u64 {
    f.mul(p.x, inv(f, p.z))
}

// ---------------------------------------------------------------------------
// Instances (ported from round 4 unchanged in behaviour).

#[derive(Clone, Debug)]
struct Inst {
    ell: u64,
    lambda: u64,
    p: u64,
    t: i64,
    v: u64,
    n: u128,
    a: u64,
    a24: u64,
    chi_n: i32,
    alpha: u64,
    s: u64,
    jcrater: u64,
}

fn make_instance(ell: u64, lambda: u64, rng: &mut StdRng) -> Inst {
    let lo: u128 = 1 << 62;
    let hi: u128 = 1 << 63;
    loop {
        let wmax = (((hi - 1) / 7) as f64).sqrt() as u128 / ell as u128;
        let w = rng.gen_range(1..=wmax.max(1)) as u64;
        let v = ell * w;
        let sv = 7u128 * (v as u128) * (v as u128);
        if sv >= hi {
            continue;
        }
        let tmax = ((hi - 1 - sv) as f64).sqrt() as i128;
        let tmin = if sv >= lo {
            0
        } else {
            ((lo - sv) as f64).sqrt().ceil() as i128
        };
        if tmin > tmax {
            continue;
        }
        let l = ell as i128;
        let r_l = (2 * lambda as i128) % l;
        let r2 = (v % 2) as i128;
        let r = if r_l % 2 == r2 { r_l } else { r_l + l };
        let span = (tmax - tmin) / (2 * l);
        if span < 1 {
            continue;
        }
        let j = rng.gen_range(0..=span);
        let mut t = tmin - tmin.rem_euclid(2 * l) + r + 2 * l * j;
        if t < tmin {
            t += 2 * l;
        }
        if t > tmax {
            continue;
        }
        let four_p = (t * t) as u128 + sv;
        if !four_p.is_multiple_of(4) || four_p < lo || four_p >= hi {
            continue;
        }
        let p = (four_p / 4) as u64;
        if !is_prime_u64(p) {
            continue;
        }
        let n = (p as i128 + 1 - t) as u128;
        if let Some(inst) = finish_instance(ell, lambda, p, t as i64, v, n, rng) {
            return inst;
        }
    }
}

fn finish_instance(
    ell: u64,
    lambda: u64,
    p: u64,
    t: i64,
    v: u64,
    n: u128,
    rng: &mut StdRng,
) -> Option<Inst> {
    let f = field(p, PolyMul::Karatsuba);
    let aw = red(&f, -35);
    let mut roots = vec![7u64];
    let disc = red(&f, -7);
    if let Some(sq) = f.sqrt(disc) {
        let inv2 = inv(&f, 2);
        roots.push(f.mul(f.sub(sq, 7), inv2));
        roots.push(f.mul(f.sub(f.neg(sq), 7), inv2));
    }
    for alpha in roots {
        let w = f.add(f.mul(3, f.mul(alpha, alpha)), aw);
        let Some(s) = f.sqrt(w) else { continue };
        if s == 0 {
            continue;
        }
        let a = f.mul(f.mul(3, alpha), inv(&f, s));
        let a24 = f.mul(f.add(a, 2), inv(&f, 4));
        let jcrater = j_of_a(&f, a);
        assert_eq!(jcrater, red(&f, -3375), "CM curve must have j = −3375");
        let nb = BigUint::from(n);
        let mut chi_n = 0;
        for _ in 0..8 {
            let x = rng.gen_range(2..p);
            let rhs = f.mul(x, f.add(f.mul(x, x), f.add(f.mul(a, x), 1)));
            let ch = f.legendre(rhs);
            if ch == 0 {
                continue;
            }
            let q = ladder_big(&f, &XZ { x, z: 1 }, &nb, &a24);
            if q.z == 0 {
                if chi_n != 0 && chi_n != ch {
                    return None;
                }
                chi_n = ch;
            } else if chi_n == ch {
                return None;
            }
        }
        if chi_n == 0 {
            return None;
        }
        return Some(Inst {
            ell,
            lambda,
            p,
            t,
            v,
            n,
            a,
            a24,
            chi_n,
            alpha,
            s,
            jcrater,
        });
    }
    None
}

fn find_rational_kernel(f: &Fp, inst: &Inst, rng: &mut StdRng) -> (u64, u32) {
    let (e, m) = val(&BigUint::from(inst.n), inst.ell);
    assert!(e >= 2, "vertical λ=1 instance must have ℓ² | N");
    let lb = BigUint::from(inst.ell);
    let mut tries = 0;
    loop {
        tries += 1;
        let x = rng.gen_range(2..inst.p);
        let rhs = f.mul(x, f.add(f.mul(x, x), f.add(f.mul(inst.a, x), 1)));
        if f.legendre(rhs) != inst.chi_n {
            continue;
        }
        let mut q = ladder_big(f, &XZ { x, z: 1 }, &m, &inst.a24);
        if q.z == 0 {
            continue;
        }
        for _ in 0..e {
            let r = ladder_big(f, &q, &lb, &inst.a24);
            if r.z == 0 {
                return (affine(f, &q), tries);
            }
            q = r;
        }
        panic!("ℓ-part larger than v_ℓ(N)");
    }
}

fn order_check(f: &Fp, a_new: u64, x: u64, n: u128) -> bool {
    let a24 = f.mul(f.add(a_new, 2), inv(f, 4));
    ladder_big(f, &XZ { x, z: 1 }, &BigUint::from(n), &a24).z == 0
}

fn lib_field(p: u64) -> LField {
    LField::new(&BigUint::from(p)).expect("prime field")
}

/// Short-Weierstrass kernel polynomial from a Montgomery-x one:
/// `h_w(X) = Σ c_j s^{d−j} (X − α)^j`.
fn to_weierstrass_kernel(f: &Fp, hm: &[u64], alpha: u64, s: u64) -> Vec<u64> {
    let d = hm.len() - 1;
    let mut spow = vec![1u64; d + 1];
    for i in 1..=d {
        spow[i] = f.mul(spow[i - 1], s);
    }
    let mut h: Vec<u64> = Vec::new();
    for j in (0..=d).rev() {
        let mut nh = vec![0u64; h.len() + 1];
        for (i, &c) in h.iter().enumerate() {
            nh[i + 1] = f.add(nh[i + 1], c);
            nh[i] = f.sub(nh[i], f.mul(c, alpha));
        }
        nh[0] = f.add(nh[0], f.mul(hm[j], spow[d - j]));
        h = nh;
    }
    trim(h)
}

/// Certify `h_w` with the library's division-polynomial test and return
/// the library's codomain `j`.
fn lib_certify(p: u64, hw: &[u64], ell: u64) -> Result<u64, String> {
    let lf = lib_field(p);
    let e = Model {
        a: lf.from_i64(-35),
        b: lf.from_i64(-98),
    };
    let h: Vec<_> = hw.iter().map(|&c| lf.from_u64(c)).collect();
    let cod = kernel::verify_kernel(&lf, &e, ell, &h).map_err(|e| format!("{e:?}"))?;
    let j = cod.j(&lf).ok_or("singular codomain")?;
    Ok(lf.to_big(&j).to_u64().unwrap())
}

/// The round-4 Edwards codomain from `Π(x_s + 1)` and `Π(x_s − 1)` (up to
/// common factors removed by eighth powers).
fn edwards_codomain(f: &Fp, a: u64, ell: u64, prod_plus: u64, prod_minus: u64) -> u64 {
    let pow8 = |v: u64| {
        let v2 = f.mul(v, v);
        let v4 = f.mul(v2, v2);
        f.mul(v4, v4)
    };
    let ae = f.mul(f.pow(f.add(a, 2), ell as u128), pow8(prod_plus));
    let de = f.mul(f.pow(f.sub(a, 2), ell as u128), pow8(prod_minus));
    f.mul(f.mul(2, f.add(ae, de)), inv(f, f.sub(ae, de)))
}

// ---------------------------------------------------------------------------
// Mode: the NTT threshold (counted cost and contended wall time).

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn time_it(reps: usize, mut f: impl FnMut()) -> f64 {
    let mut ts = Vec::new();
    for _ in 0..5 {
        let t = Instant::now();
        for _ in 0..reps {
            f();
        }
        ts.push(t.elapsed().as_secs_f64() / reps as f64);
    }
    median(ts)
}

fn mode_threshold() -> Value {
    let mut rng = StdRng::seed_from_u64(0x5303);
    let mut rows = Vec::new();
    // p = 2^61 − 1 (the 61-bit size of the instances) and a 63-bit prime.
    for &p in &[2_305_843_009_213_693_951u64, (1u64 << 63) - 25] {
        let fc = field(p, PolyMul::Karatsuba);
        let fp = PrimeField::new(p).unwrap();
        // Wall time of one F_p multiplication (u128 % p) for the ratio.
        let xs: Vec<u64> = (0..4096).map(|_| rng.gen_range(0..p)).collect();
        let mut sink = 0u64;
        let t_fp = time_it(200, || {
            for w in xs.windows(2) {
                sink ^= fp.mul(w[0], w[1]);
            }
        }) / 4095.0;
        for &(la_mult, lb_div) in &[(1usize, 1usize), (8, 1)] {
            let mut n = 8usize;
            while n <= 8192 {
                let lb = n / lb_div;
                let la = n * la_mult;
                let a: Vec<u64> = (0..la).map(|_| rng.gen_range(0..p)).collect();
                let b: Vec<u64> = (0..lb).map(|_| rng.gen_range(0..p)).collect();
                fc.reset_counts();
                let ck = mul_karatsuba(&fc, &a, &b);
                let kara = fc.tally();
                fc.reset_counts();
                let cn = mul_ntt(&fc, &a, &b);
                let nt = fc.tally();
                assert_eq!(ck, cn, "NTT ≠ Karatsuba at {la}x{lb}");
                assert_eq!((nt.word, nt.mul), ntt_mul_cost(la, lb, false));
                let reps = (200_000 / (la + lb)).max(1);
                let wk = time_it(reps, || {
                    sink ^= mul_karatsuba(&fp, &a, &b)[0];
                });
                let wn = time_it(reps, || {
                    sink ^= mul_ntt(&fp, &a, &b)[0];
                });
                // Words per second of the NTT, against M per second.
                let row = json!({
                    "p": p, "la": la, "lb": lb,
                    "karatsuba": tjs(&kara), "ntt": tjs(&nt),
                    "count_ratio_ntt_over_karatsuba": meq_of(&nt) as f64 / meq_of(&kara) as f64,
                    "wall_s_karatsuba_contended": wk, "wall_s_ntt_contended": wn,
                    "wall_ratio_ntt_over_karatsuba": wn / wk,
                    "wall_ns_per_counted_M_karatsuba": 1e9 * wk / kara.mul as f64,
                    "wall_ns_per_counted_Meq_ntt": 1e9 * wn / meq_of(&nt) as f64,
                });
                eprintln!(
                    "p={p} {la}x{lb}: kara {} ntt {} (ratio {:.3}); wall {:.2e} vs {:.2e}",
                    kara.mul,
                    meq_of(&nt),
                    meq_of(&nt) as f64 / meq_of(&kara) as f64,
                    wk,
                    wn
                );
                rows.push(row);
                n = if n.is_power_of_two() {
                    n * 3 / 2
                } else {
                    n * 4 / 3
                };
            }
        }
        rows.push(json!({"p": p, "wall_ns_per_fp_mul_contended": 1e9 * t_fp, "sink": sink}));
    }
    // Threshold by counts: smallest balanced (and square) length from which
    // the NTT's Meq stays below Karatsuba's at every larger grid length.
    let mut thr = Vec::new();
    for &p in &[2_305_843_009_213_693_951u64, (1u64 << 63) - 25] {
        for unbalanced in [false, true] {
            let pts: Vec<(u64, f64, f64)> = rows
                .iter()
                .filter(|r| r["p"].as_u64() == Some(p) && r.get("la").is_some())
                .filter(|r| (r["la"].as_u64() != r["lb"].as_u64()) == unbalanced)
                .map(|r| {
                    (
                        r["lb"].as_u64().unwrap(),
                        r["count_ratio_ntt_over_karatsuba"].as_f64().unwrap(),
                        r["wall_ratio_ntt_over_karatsuba"].as_f64().unwrap(),
                    )
                })
                .collect();
            let first_stay = |sel: &dyn Fn(&(u64, f64, f64)) -> f64| -> Option<u64> {
                (0..pts.len())
                    .find(|&i| pts[i..].iter().all(|q| sel(q) <= 1.0))
                    .map(|i| pts[i].0)
            };
            thr.push(json!({
                "p": p, "shape": if unbalanced { "8n x n" } else { "n x n" },
                "count_threshold_shorter_len": first_stay(&|q| q.1),
                "wall_threshold_shorter_len_contended": first_stay(&|q| q.2),
            }));
        }
    }
    eprintln!("{}", serde_json::to_string_pretty(&thr).unwrap());
    json!({"mode": "threshold", "rows": rows, "thresholds": thr,
           "library_NTT_THRESHOLD": NTT_THRESHOLD, "KARATSUBA_THRESHOLD": KARATSUBA_THRESHOLD})
}

// ---------------------------------------------------------------------------
// Mode: rational kernel (λ = 1): Vélu, √élu (Karatsuba), √élu (NTT).

fn mode_rational(min_log2: f64, max_log2: f64, seeds: u64, velu_max_log2: f64) -> Value {
    let mut ells = Vec::new();
    let mut e = 3.0;
    while e <= max_log2 + 1e-9 {
        let l = next_prime(2f64.powf(e).round() as u64);
        if l != 7 && !ells.contains(&l) && e >= min_log2 - 1e-9 {
            ells.push(l);
        }
        e += 0.5;
    }
    let mut rows = Vec::new();
    for &ell in &ells {
        for seed in 0..seeds {
            let mut rng = StdRng::seed_from_u64(0x163 ^ (ell << 8) ^ seed);
            let mut rejected = 0u64;
            let inst = loop {
                let inst = make_instance(ell, 1, &mut rng);
                if val(&BigUint::from(inst.n), ell).0 == 2 {
                    break inst;
                }
                rejected += 1;
            };
            let f = field(inst.p, PolyMul::Auto);
            let fk = field(inst.p, PolyMul::Karatsuba);
            let curve = MontgomeryCurve {
                a: inst.a,
                a24: inst.a24,
            };
            let mut pts = Vec::new();
            while pts.len() < 2 {
                let x = rng.gen_range(2..inst.p);
                let rhs = f.mul(x, f.add(f.mul(x, x), f.add(f.mul(inst.a, x), 1)));
                if f.legendre(rhs) == inst.chi_n {
                    pts.push(x);
                }
            }
            // Kernel finding, retried if the line is horizontal (the check
            // consumes no randomness, so the draws match round 4).
            let mut find = OpTally::default();
            let mut horizontal_retries = 0;
            let (xp, tries) = loop {
                let c0 = f.tally();
                let (xp, tries) = find_rational_kernel(&f, &inst, &mut rng);
                find = OpTally {
                    mul: find.mul + f.tally().since(c0).mul,
                    inv: find.inv + f.tally().since(c0).inv,
                    word: find.word + f.tally().since(c0).word,
                };
                let chk = PrimeField::new(inst.p).unwrap();
                let (o, _) = sqrt_velu(&chk, &curve, ell, xp, &[]).expect("√élu");
                if j_of_a(&f, o.codomain.a) == inst.jcrater {
                    horizontal_retries += 1;
                    continue;
                }
                break (xp, tries);
            };
            let run_sq = |fld: &Fp, k: usize| {
                let c = fld.tally();
                let w = Instant::now();
                let (o, shape) = sqrt_velu(fld, &curve, ell, xp, &pts[..k]).expect("√élu");
                (o, shape, fld.tally().since(c), w.elapsed().as_secs_f64())
            };
            let (sk1, shape, ck1, wk1) = run_sq(&fk, 1);
            let (sk2, _, ck2, wk2) = run_sq(&fk, 2);
            let (sn1, _, cn1, wn1) = run_sq(&f, 1);
            let (sn2, _, cn2, wn2) = run_sq(&f, 2);
            assert_eq!(sk1, sn1, "NTT √élu ≠ Karatsuba √élu (1 pt)");
            assert_eq!(sk2, sn2, "NTT √élu ≠ Karatsuba √élu (2 pt)");
            assert_eq!(sk2.images[0], sk1.images[0]);
            let mut checks = vec!["sqrtvelu_ntt_eq_sqrtvelu_karatsuba"];
            let mut velu_js = (Value::Null, Value::Null, Value::Null);
            if (ell as f64).log2() <= velu_max_log2 + 1e-9 {
                let c = fk.tally();
                let w = Instant::now();
                let v1 = velu(&fk, &curve, ell, xp, &pts[..1]).expect("Vélu");
                let cv1 = fk.tally().since(c);
                let wv = w.elapsed().as_secs_f64();
                let c = fk.tally();
                let v2 = velu(&fk, &curve, ell, xp, &pts).expect("Vélu");
                let cv2 = fk.tally().since(c);
                assert_eq!(v1, sk1, "Vélu ≠ √élu (1 pt)");
                assert_eq!(v2, sk2, "Vélu ≠ √élu (2 pt)");
                checks.push("velu_eq_sqrtvelu");
                velu_js = (tjs(&cv1), tjs(&cv2), json!(wv));
            }
            let a_new = sk2.codomain.a;
            let jn = j_of_a(&f, a_new);
            for img in &sk2.images {
                let x = img.expect("non-kernel point");
                assert!(order_check(&f, a_new, x, inst.n), "image has wrong order");
            }
            checks.push("images_order_N_on_codomain");
            if ell <= 151 {
                let plain = PrimeField::new(inst.p).unwrap();
                let hm = kernel_polynomial(&plain, &curve, ell, xp).expect("kernel polynomial");
                let hw = to_weierstrass_kernel(&f, &hm, inst.alpha, inst.s);
                let lj = lib_certify(inst.p, &hw, ell).expect("library rejects kernel");
                assert_eq!(lj, jn, "library Vélu j ≠ Montgomery j");
                checks.push("lib_verify_kernel_and_j");
            }
            eprintln!(
                "rational ℓ={ell} seed={seed} find={} sqrtvelu kara={} ntt={} (W {}) velu={}",
                meq_of(&find),
                meq_of(&ck1),
                meq_of(&cn1),
                cn1.word,
                velu_js.0
            );
            rows.push(json!({
                "ell": ell, "log2_ell": (ell as f64).log2(), "seed": seed, "lambda": inst.lambda,
                "instances_rejected": rejected, "p": inst.p, "t": inst.t, "v": inst.v,
                "N": inst.n.to_string(), "find": tjs(&find), "find_point_tries": tries,
                "horizontal_retries": horizontal_retries,
                "velu_1pt": velu_js.0, "velu_2pt": velu_js.1, "velu_wall_s_contended": velu_js.2,
                "sqrtvelu_karatsuba_1pt": tjs(&ck1), "sqrtvelu_karatsuba_2pt": tjs(&ck2),
                "sqrtvelu_ntt_1pt": tjs(&cn1), "sqrtvelu_ntt_2pt": tjs(&cn2),
                "sqrtvelu_karatsuba_wall_s_contended": [wk1, wk2],
                "sqrtvelu_ntt_wall_s_contended": [wn1, wn2],
                "sqrtvelu_sets": {"b": shape.b, "b_prime": shape.b_prime, "I": shape.i_len,
                                   "J": shape.j_len, "K": shape.k_len},
                "codomain_j": jn, "crater_j": inst.jcrater, "checks": checks,
            }));
        }
    }
    json!({"mode": "rational", "rows": rows, "NTT_THRESHOLD": NTT_THRESHOLD})
}

// ---------------------------------------------------------------------------
// Mode: generic kernel (λ primitive), torsion point over F_{p^k}.

/// `F_{p^k} = F_p[X]/(X^k + X^m + c)`, products by the field's PolyMul.
#[derive(Clone, Debug)]
struct Fpk<'a> {
    f: &'a Fp,
    k: usize,
    m: usize,
    c: u64,
}

impl Fpk<'_> {
    fn reduce(&self, mut r: Vec<u64>) -> Vec<u64> {
        let k = self.k;
        let f = self.f;
        r.resize((2 * k).max(k), 0);
        for i in (k..r.len()).rev() {
            let q = r[i];
            if q == 0 {
                continue;
            }
            r[i] = 0;
            let j = i - k + self.m;
            r[j] = f.sub(r[j], q);
            let t = f.mul(q, self.c);
            r[i - k] = f.sub(r[i - k], t);
        }
        r.truncate(k);
        r
    }
    fn modulus(&self) -> Vec<u64> {
        let mut v = vec![0u64; self.k + 1];
        v[0] = self.c;
        v[self.m] = self.f.add(v[self.m], 1);
        v[self.k] = 1;
        v
    }
    fn x(&self) -> Vec<u64> {
        let mut v = vec![0u64; self.k];
        if self.k > 1 {
            v[1] = 1;
        } else {
            v[0] = self.f.neg(self.c);
        }
        v
    }
    fn pow_big(&self, a: &[u64], e: &BigUint) -> Vec<u64> {
        let mut r = FieldArith::one(self);
        let a = a.to_vec();
        for i in (0..e.bits()).rev() {
            r = FieldArith::mul(self, &r, &r);
            if e.bit(i) {
                r = FieldArith::mul(self, &r, &a);
            }
        }
        r
    }
    fn inv(&self, a: &[u64]) -> Vec<u64> {
        let mut g = poly_xgcd_inv(self.f, &trim(a.to_vec()), &self.modulus())
            .expect("non-invertible extension element");
        g.resize(self.k, 0);
        g
    }
}

impl FieldArith for Fpk<'_> {
    type Elem = Vec<u64>;
    fn mul(&self, a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
        // Same reference ⇒ the NTT squares (two transforms per prime).
        let prod = if std::ptr::eq(a, b) {
            poly::mul(self.f, a, a)
        } else {
            poly::mul(self.f, a, b)
        };
        self.reduce(prod)
    }
    fn add(&self, a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
        a.iter().zip(b).map(|(x, y)| self.f.add(*x, *y)).collect()
    }
    fn sub(&self, a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
        a.iter().zip(b).map(|(x, y)| self.f.sub(*x, *y)).collect()
    }
    fn zero(&self) -> Vec<u64> {
        vec![0u64; self.k]
    }
    fn one(&self) -> Vec<u64> {
        let mut v = vec![0u64; self.k];
        v[0] = 1;
        v
    }
    fn is_zero(&self, a: &Vec<u64>) -> bool {
        a.iter().all(|v| *v == 0)
    }
}

fn psub(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    poly::sub(f, a, b)
}

fn poly_rem(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let b = trim(b.to_vec());
    let db = b.len() - 1;
    let lc_inv = inv(f, b[db]);
    let mut r = trim(a.to_vec());
    while r.len() > db {
        let i = r.len() - 1;
        let q = f.mul(r[i], lc_inv);
        for j in 0..=db {
            let t = f.mul(q, b[j]);
            r[i - db + j] = f.sub(r[i - db + j], t);
        }
        r = trim(r);
    }
    r
}

fn poly_gcd(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut a = trim(a.to_vec());
    let mut b = trim(b.to_vec());
    while !b.is_empty() {
        let r = poly_rem(f, &a, &b);
        a = b;
        b = r;
    }
    a
}

fn poly_xgcd_inv(f: &Fp, a: &[u64], m: &[u64]) -> Option<Vec<u64>> {
    let (mut r0, mut r1) = (trim(m.to_vec()), trim(a.to_vec()));
    let (mut s0, mut s1): (Vec<u64>, Vec<u64>) = (Vec::new(), vec![1]);
    while !r1.is_empty() {
        let d1 = r1.len() - 1;
        let lc_inv = inv(f, r1[d1]);
        let mut r = r0.clone();
        let mut q = vec![0u64; r0.len().saturating_sub(d1).max(1)];
        while r.len() > d1 {
            let i = r.len() - 1;
            let c = f.mul(r[i], lc_inv);
            q[i - d1] = c;
            for j in 0..=d1 {
                let t = f.mul(c, r1[j]);
                r[i - d1 + j] = f.sub(r[i - d1 + j], t);
            }
            r = trim(r);
        }
        let ns = psub(f, &s0, &trim(poly::mul(f, &trim(q), &s1)));
        (r0, r1) = (r1, r);
        (s0, s1) = (s1, trim(ns));
    }
    if r0.len() != 1 {
        return None;
    }
    let c = inv(f, r0[0]);
    Some(s0.iter().map(|v| f.mul(*v, c)).collect())
}

fn find_irreducible<'a>(f: &'a Fp, k: usize, rng: &mut StdRng) -> (Fpk<'a>, u64) {
    let mut cands = 0;
    loop {
        cands += 1;
        let m = if k > 1 { rng.gen_range(1..k) } else { 0 };
        let c = rng.gen_range(1..f.p());
        let fk = Fpk { f, k, m, c };
        if k == 1 {
            return (fk, cands);
        }
        let modp = fk.modulus();
        let x = fk.x();
        let pb = BigUint::from(f.p());
        let mut xq = x.clone();
        let mut ok = true;
        for _ in 1..=k / 2 {
            xq = fk.pow_big(&xq, &pb);
            let g = poly_gcd(f, &modp, &trim(psub(f, &xq, &x)));
            if g.len() > 1 {
                ok = false;
                break;
            }
        }
        if ok {
            return (fk, cands);
        }
    }
}

fn berlekamp_massey(f: &Fp, s: &[u64]) -> Vec<u64> {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let mut l = 0usize;
    let mut m = 1usize;
    let mut bb = 1u64;
    for n in 0..s.len() {
        let mut d = s[n];
        for i in 1..=l {
            d = f.add(d, f.mul(*c.get(i).unwrap_or(&0), s[n - i]));
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = f.mul(d, inv(f, bb));
        let mut nc = c.clone();
        nc.resize(nc.len().max(b.len() + m), 0);
        for (i, &bi) in b.iter().enumerate() {
            nc[i + m] = f.sub(nc[i + m], f.mul(coef, bi));
        }
        if 2 * l <= n {
            l = n + 1 - l;
            b = c;
            bb = d;
            m = 1;
        } else {
            m += 1;
        }
        c = nc;
    }
    c.resize(l + 1, 0);
    c
}

fn orders_over_ext(inst: &Inst, k: usize) -> (BigUint, BigUint) {
    let pb = BigInt::from(inst.p);
    let (mut t0, mut t1) = (BigInt::from(2), BigInt::from(inst.t));
    for _ in 1..k {
        let t2 = BigInt::from(inst.t) * &t1 - &pb * &t0;
        t0 = t1;
        t1 = t2;
    }
    let pk = pb.pow(k as u32);
    let one = BigInt::from(1);
    let n_e = (&pk + &one - &t1).abs().to_biguint().unwrap();
    let n_t = (&pk + &one + &t1).abs().to_biguint().unwrap();
    (n_e, n_t)
}

fn generic_ells(min_ell: u64, max_ell: u64, step_log2: f64) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();
    let mut e = 5.0f64;
    while 2f64.powf(e) <= max_ell as f64 * 1.0001 {
        let l = next_prime(2f64.powf(e).round() as u64);
        if l >= min_ell && l <= max_ell && l != 7 && !v.contains(&l) {
            v.push(l);
        }
        e += step_log2;
    }
    v
}

fn mode_generic(min_ell: u64, max_ell: u64, step_log2: f64, cap_s: f64, arm: PolyMul) -> Value {
    let mut rows = Vec::new();
    let t_start = Instant::now();
    for ell in generic_ells(min_ell, max_ell, step_log2) {
        if t_start.elapsed().as_secs_f64() > cap_s {
            eprintln!("generic: time cap reached before ℓ={ell}");
            break;
        }
        // Same seeds as round 4 (salt 0): identical instances for equal ℓ.
        let mut rng = StdRng::seed_from_u64(0x1630 ^ ell);
        let lambda = primitive_root(ell);
        let k = ((ell - 1) / 2) as usize;
        let mut rejected = 0u64;
        let (inst, n_e, n_t, ve, vt) = loop {
            let inst = make_instance(ell, lambda, &mut rng);
            let (n_e, n_t) = orders_over_ext(&inst, k);
            let (ve, _) = val(&n_e, ell);
            let (vt, _) = val(&n_t, ell);
            if ve == 0 && vt == 2 {
                break (inst, n_e, n_t, ve, vt);
            }
            rejected += 1;
        };
        let f = field(inst.p, arm);
        let wall = Instant::now();
        let c_setup = f.tally();
        let (fk, cands) = find_irreducible(&f, k, &mut rng);
        let setup = f.tally().since(c_setup);
        let (ntarget, side) = if vt >= 2 { (n_t, "twist") } else { (n_e, "E") };
        let (e, m) = val(&ntarget, ell);
        assert!(e >= 2, "ℓ² must divide the order over F_(p^k)");
        let mut a24k = fk.zero();
        a24k[0] = inst.a24;
        let lb = BigUint::from(ell);
        let mut pts = Vec::new();
        while pts.len() < 2 {
            let x = rng.gen_range(2..inst.p);
            let rhs = f.mul(x, f.add(f.mul(x, x), f.add(f.mul(inst.a, x), 1)));
            if f.legendre(rhs) == inst.chi_n {
                pts.push(x);
            }
        }
        let horner =
            |h: &[u64], x: u64| h.iter().rev().fold(0u64, |acc, c| f.add(f.mul(acc, x), *c));
        let mut horizontal_retries = 0u64;
        let mut attempts = 0;
        let mut att_final: u64;
        let (ladder_cnt, minpoly_cnt, find_cnt, find_wall, minpoly, a_new) = loop {
            let c_find = f.tally();
            att_final = 0;
            let q = 'outer: loop {
                attempts += 1;
                att_final += 1;
                let x: Vec<u64> = (0..k).map(|_| rng.gen_range(0..inst.p)).collect();
                let mut q = ladder_big(&fk, &XZ { x, z: fk.one() }, &m, &a24k);
                if fk.is_zero(&q.z) {
                    continue;
                }
                for _ in 0..e {
                    let r = ladder_big(&fk, &q, &lb, &a24k);
                    if fk.is_zero(&r.z) {
                        break 'outer q;
                    }
                    q = r;
                }
            };
            let ladder_cnt = f.tally().since(c_find);
            let c_mp = f.tally();
            let x1 = FieldArith::mul(&fk, &q.x, &fk.inv(&q.z));
            let proj: Vec<u64> = (0..k).map(|_| rng.gen_range(0..inst.p)).collect();
            let mut pw = fk.one();
            let mut seq = Vec::with_capacity(2 * k);
            for _ in 0..2 * k {
                let s = pw
                    .iter()
                    .zip(&proj)
                    .fold(0u64, |acc, (a, b)| f.add(acc, f.mul(*a, *b)));
                seq.push(s);
                pw = FieldArith::mul(&fk, &pw, &x1);
            }
            let conn = berlekamp_massey(&f, &seq);
            let minpoly: Vec<u64> = conn.iter().rev().copied().collect();
            let minpoly_cnt = f.tally().since(c_mp);
            let find_cnt = f.tally().since(c_find);
            let find_wall = wall.elapsed().as_secs_f64();
            assert_eq!(minpoly.len() - 1, k, "minimal polynomial degree ≠ (ℓ−1)/2");
            let mut acc = fk.zero();
            for &c in minpoly.iter().rev() {
                acc = FieldArith::mul(&fk, &acc, &x1);
                acc[0] = f.add(acc[0], c);
            }
            assert!(fk.is_zero(&acc), "minpoly does not vanish at x1");
            let h1 = horner(&minpoly, 1);
            let hm1 = horner(&minpoly, f.neg(1));
            let a_new = edwards_codomain(&f, inst.a, ell, hm1, h1);
            if j_of_a(&f, a_new) == inst.jcrater {
                horizontal_retries += 1;
                continue;
            }
            break (ladder_cnt, minpoly_cnt, find_cnt, find_wall, minpoly, a_new);
        };
        let rev: Vec<u64> = minpoly.iter().rev().copied().collect();
        let mut images = Vec::new();
        for &x in &pts {
            let den = horner(&minpoly, x);
            let num = horner(&rev, x);
            images.push(f.mul(f.mul(x, f.mul(num, num)), inv(&f, f.mul(den, den))));
        }
        let jn = j_of_a(&f, a_new);
        for &x in &images {
            assert!(order_check(&f, a_new, x, inst.n), "image has wrong order");
        }
        let mut checks = vec!["minpoly_vanishes", "images_order_N_on_codomain"];
        if ell <= 211 {
            let hw = to_weierstrass_kernel(&f, &minpoly, inst.alpha, inst.s);
            let lj = lib_certify(inst.p, &hw, ell).expect("library rejects kernel");
            assert_eq!(lj, jn, "library j ≠ Montgomery j");
            checks.push("lib_verify_kernel_and_j");
        }
        // One F_(p^k) product and one square, counted, for the per-op cost.
        let probe: Vec<u64> = (0..k as u64).map(|i| f.elem(i * 7 + 3)).collect();
        let probe2: Vec<u64> = (0..k as u64).map(|i| f.elem(i * 11 + 5)).collect();
        let c_op = f.tally();
        let _ = FieldArith::mul(&fk, &probe, &probe2);
        let ext_mul = f.tally().since(c_op);
        let c_op = f.tally();
        let _ = FieldArith::mul(&fk, &probe, &probe);
        let ext_sqr = f.tally().since(c_op);
        let per_attempt = meq_of(&ladder_cnt) as f64 / att_final as f64;
        eprintln!(
            "generic[{arm:?}] ℓ={ell} k={k} setup={} ladder/attempt={per_attempt:.3e} minpoly={} find={} wall={find_wall:.1}s",
            meq_of(&setup),
            meq_of(&minpoly_cnt),
            meq_of(&find_cnt)
        );
        rows.push(json!({
            "ell": ell, "k": k, "lambda": lambda, "p": inst.p, "t": inst.t, "v": inst.v,
            "side": side, "v_ell_E": ve, "v_ell_twist": vt, "instances_rejected": rejected,
            "irreducible_candidates": cands, "ext_modulus": [k, fk.m, fk.c],
            "setup": tjs(&setup), "ladder": tjs(&ladder_cnt),
            "ladder_attempts_total": attempts, "ladder_attempts_final_find": att_final,
            "ladder_per_attempt_Meq": per_attempt,
            "cofactor_bits": m.bits(), "minpoly": tjs(&minpoly_cnt), "find": tjs(&find_cnt),
            "find_wall_s_contended": find_wall,
            "ext_mul": tjs(&ext_mul), "ext_sqr": tjs(&ext_sqr),
            "codomain_j": jn, "crater_j": inst.jcrater, "vertical": jn != inst.jcrater,
            "horizontal_retries": horizontal_retries, "checks": checks,
        }));
    }
    json!({"mode": "generic", "arm": format!("{arm:?}"), "rows": rows, "NTT_THRESHOLD": NTT_THRESHOLD})
}

// ---------------------------------------------------------------------------
// Mode: fits and the derived crossover table.

fn t975(df: f64) -> f64 {
    let z: f64 = 1.959964;
    z + (z.powi(3) + z) / (4.0 * df)
        + (5.0 * z.powi(5) + 16.0 * z.powi(3) + 3.0 * z) / (96.0 * df * df)
        + (3.0 * z.powi(7) + 19.0 * z.powi(5) + 17.0 * z.powi(3) - 15.0 * z) / (384.0 * df.powi(3))
}

#[derive(Clone, Copy, Debug)]
struct Fit {
    slope: f64,
    slope_lo: f64,
    slope_hi: f64,
    icpt: f64,
    n: usize,
    r2: f64,
}

impl Fit {
    fn eval_log2(&self, log2_l: f64) -> f64 {
        self.icpt + self.slope * log2_l
    }
    fn js(&self, name: &str, range: &str) -> Value {
        json!({"series": name, "range": range, "exponent": self.slope,
               "exponent_ci95": [self.slope_lo, self.slope_hi],
               "log2_constant": self.icpt, "n": self.n, "r2": self.r2})
    }
}

/// OLS of `y` on `x` (already transformed).
fn ols(xs: &[f64], ys: &[f64]) -> Fit {
    let n = xs.len();
    assert!(n >= 3, "need at least three points to fit");
    let mx = xs.iter().sum::<f64>() / n as f64;
    let my = ys.iter().sum::<f64>() / n as f64;
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let slope = sxy / sxx;
    let icpt = my - slope * mx;
    let sse: f64 = xs
        .iter()
        .zip(ys)
        .map(|(x, y)| (y - icpt - slope * x).powi(2))
        .sum();
    let syy: f64 = ys.iter().map(|y| (y - my).powi(2)).sum();
    let se = (sse / (n as f64 - 2.0) / sxx).sqrt();
    let t = t975(n as f64 - 2.0);
    Fit {
        slope,
        slope_lo: slope - t * se,
        slope_hi: slope + t * se,
        icpt,
        n,
        r2: if syy > 0.0 { 1.0 - sse / syy } else { 1.0 },
    }
}

fn fit_loglog(pts: &[(f64, f64)]) -> Fit {
    let xs: Vec<f64> = pts.iter().map(|p| p.0.log2()).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1.log2()).collect();
    ols(&xs, &ys)
}

/// Log-factor fit: `log2(cost) − e·log2 ℓ = c + k·log2(log2 ℓ)`; the slope
/// is the power `k` of `log ℓ` on top of `ℓ^e`.
fn fit_logpower(pts: &[(f64, f64)], e: f64) -> Fit {
    let xs: Vec<f64> = pts.iter().map(|p| p.0.log2().log2()).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1.log2() - e * p.0.log2()).collect();
    ols(&xs, &ys)
}

fn meq(v: &Value) -> Option<f64> {
    v.get("Meq").and_then(|x| x.as_f64())
}

fn per_ell(rows: &[Value], get: impl Fn(&Value) -> Option<f64>) -> Vec<(f64, f64)> {
    let mut m: std::collections::BTreeMap<u64, (f64, f64)> = Default::default();
    for r in rows {
        if let Some(y) = get(r) {
            let e = m.entry(r["ell"].as_u64().unwrap()).or_insert((0.0, 0.0));
            e.0 += y;
            e.1 += 1.0;
        }
    }
    m.into_iter().map(|(l, (s, c))| (l as f64, s / c)).collect()
}

fn crossover(cost: impl Fn(f64) -> f64, target: f64, lo: f64, hi: f64) -> Option<f64> {
    if cost(hi) < target {
        return None;
    }
    if cost(lo) >= target {
        return Some(lo);
    }
    let (mut a, mut b) = (lo, hi);
    for _ in 0..200 {
        let m = 0.5 * (a + b);
        if cost(m) >= target {
            b = m;
        } else {
            a = m;
        }
    }
    Some(b)
}

fn sel(v: &[(f64, f64)], lo: f64) -> Vec<(f64, f64)> {
    v.iter().filter(|p| p.0 >= lo).copied().collect()
}

fn load(path: &str) -> Value {
    let s = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    serde_json::from_str(&s).expect("parse json")
}

fn rows_of(dir: &str, names: &[&str]) -> Vec<Value> {
    let mut out = Vec::new();
    for n in names {
        let path = format!("{dir}/{n}");
        if std::path::Path::new(&path).exists() {
            out.extend(load(&path)["rows"].as_array().unwrap().iter().cloned());
        }
    }
    out
}

fn mode_analyze(dir: &str, r4: &str, out: &str) -> Value {
    let rr = rows_of(dir, &["rational.json", "rational_hi.json"]);
    let gk = rows_of(dir, &["generic_karatsuba.json"]);
    let gn = rows_of(dir, &["generic_ntt.json", "generic_ntt_hi.json"]);
    let r4rat = load(&format!("{r4}/rational.json"))["rows"]
        .as_array()
        .unwrap()
        .clone();
    let r4gen = load(&format!("{r4}/generic.json"))["rows"]
        .as_array()
        .unwrap()
        .clone();
    let mut text = String::new();
    let mut fits = Vec::new();

    // Reproduction of round 4: same instances, same Karatsuba counts?
    let mut rat_match = (0, 0, 0, 0);
    for r in &rr {
        if let Some(o) = r4rat
            .iter()
            .find(|o| o["ell"] == r["ell"] && o["seed"] == r["seed"])
        {
            rat_match.0 += 1;
            if o["p"] == r["p"] && o["codomain_j"] == r["codomain_j"] {
                rat_match.1 += 1;
            }
            if o["sqrtvelu_1pt"]["M"] == r["sqrtvelu_karatsuba_1pt"]["M"]
                && o["sqrtvelu_1pt"]["I"] == r["sqrtvelu_karatsuba_1pt"]["I"]
            {
                rat_match.2 += 1;
            }
            if o["find"]["M"] == r["find"]["M"] && o["find"]["I"] == r["find"]["I"] {
                rat_match.3 += 1;
            }
        }
    }
    let mut gen_match = (0, 0, 0);
    for r in &gk {
        if let Some(o) = r4gen.iter().find(|o| o["ell"] == r["ell"]) {
            gen_match.0 += 1;
            if o["p"] == r["p"] && o["codomain_j"] == r["codomain_j"] {
                gen_match.1 += 1;
            }
            if o["ladder"]["M"] == r["ladder"]["M"] && o["ladder"]["I"] == r["ladder"]["I"] {
                gen_match.2 += 1;
            }
        }
    }
    text.push_str(&format!(
        "Round-4 reproduction: rational rows matched {} (same p and codomain j: {}, same Karatsuba sqrt-elu M and I: {}, same find M and I: {}); generic Karatsuba rows matched {} (same p and j: {}, same ladder M and I: {})\n\n",
        rat_match.0, rat_match.1, rat_match.2, rat_match.3, gen_match.0, gen_match.1, gen_match.2
    ));

    // Rational case.
    let find_r = per_ell(&rr, |r| meq(&r["find"]));
    let velu = per_ell(&rr, |r| meq(&r["velu_1pt"]));
    let sqk = per_ell(&rr, |r| meq(&r["sqrtvelu_karatsuba_1pt"]));
    let sqn = per_ell(&rr, |r| meq(&r["sqrtvelu_ntt_1pt"]));
    let sqn_marg = per_ell(&rr, |r| {
        Some(meq(&r["sqrtvelu_ntt_2pt"])? - meq(&r["sqrtvelu_ntt_1pt"])?)
    });
    let sqk_marg = per_ell(&rr, |r| {
        Some(meq(&r["sqrtvelu_karatsuba_2pt"])? - meq(&r["sqrtvelu_karatsuba_1pt"])?)
    });
    let sqn_wall = per_ell(&rr, |r| r["sqrtvelu_ntt_wall_s_contended"][0].as_f64());
    let sqk_wall = per_ell(&rr, |r| {
        r["sqrtvelu_karatsuba_wall_s_contended"][0].as_f64()
    });
    let l_max_meas = sqn.last().unwrap().0;
    let f_velu = fit_loglog(&sel(&velu, 1024.0));
    let f_k10 = fit_loglog(&sel(&sqk, 1024.0));
    let f_k16 = fit_loglog(&sel(&sqk, 65536.0));
    let f_n10 = fit_loglog(&sel(&sqn, 1024.0));
    let f_n16 = fit_loglog(&sel(&sqn, 65536.0));
    let f_n20 = fit_loglog(&sel(&sqn, 1_048_576.0));
    let f_n_m = fit_loglog(&sel(&sqn_marg, 1024.0));
    let f_k_m = fit_loglog(&sel(&sqk_marg, 1024.0));
    let f_nw = fit_loglog(&sel(&sqn_wall, 65536.0));
    let f_kw = fit_loglog(&sel(&sqk_wall, 65536.0));
    let f_find = fit_loglog(&find_r);
    let top = format!("2^16 <= ell <= 2^{:.1}", l_max_meas.log2());
    fits.push(f_find.js("rational_find (cofactor ladder)", "all ell"));
    fits.push(f_velu.js("velu_1pt (codomain + 1 image)", "ell >= 2^10"));
    fits.push(f_k10.js("sqrtvelu_karatsuba_1pt", "ell >= 2^10"));
    fits.push(f_k16.js("sqrtvelu_karatsuba_1pt", &top));
    fits.push(f_n10.js("sqrtvelu_ntt_1pt", "ell >= 2^10"));
    fits.push(f_n16.js("sqrtvelu_ntt_1pt", &top));
    fits.push(f_n20.js("sqrtvelu_ntt_1pt", "ell >= 2^20"));
    fits.push(f_k_m.js("sqrtvelu_karatsuba_marginal_per_extra_point", "ell >= 2^10"));
    fits.push(f_n_m.js("sqrtvelu_ntt_marginal_per_extra_point", "ell >= 2^10"));
    fits.push(f_kw.js("sqrtvelu_karatsuba_1pt_wall_s_contended", "ell >= 2^16"));
    fits.push(f_nw.js("sqrtvelu_ntt_1pt_wall_s_contended", "ell >= 2^16"));
    // Log-factor power on top of sqrt(ell).
    let f_n_logk = fit_logpower(&sel(&sqn, 65536.0), 0.5);
    fits.push(f_n_logk.js(
        "sqrtvelu_ntt_1pt: k in cost = c sqrt(ell) (log2 ell)^k (slope reported as exponent)",
        &top,
    ));
    // Local exponents over sliding 4-octave windows.
    let mut local = Vec::new();
    let mut lo = 10.0;
    while lo + 4.0 <= l_max_meas.log2() + 0.26 {
        let win = |v: &[(f64, f64)]| -> Vec<(f64, f64)> {
            v.iter()
                .filter(|p| p.0.log2() >= lo - 0.01 && p.0.log2() <= lo + 4.01)
                .copied()
                .collect()
        };
        let fk = fit_loglog(&win(&sqk));
        let fnn = fit_loglog(&win(&sqn));
        local.push(json!({"log2_ell_from": lo, "log2_ell_to": lo + 4.0,
            "karatsuba": [fk.slope, fk.slope_lo, fk.slope_hi],
            "ntt": [fnn.slope, fnn.slope_lo, fnn.slope_hi]}));
        lo += 1.0;
    }
    let find_r_max = find_r.iter().map(|p| p.1).fold(0.0, f64::max);
    let (l_top, sqn_top) = *sqn.last().unwrap();
    let sqk_top = sqk.last().unwrap().1;

    // Generic case: per-attempt ladder, minpoly, expected find, per arm.
    let gen_series = |rows: &[Value]| {
        let lpa = per_ell(rows, |r| r["ladder_per_attempt_Meq"].as_f64());
        let minp = per_ell(rows, |r| meq(&r["minpoly"]));
        let setup = per_ell(rows, |r| meq(&r["setup"]));
        let ext = per_ell(rows, |r| meq(&r["ext_mul"]));
        let gfind: Vec<(f64, f64)> = lpa
            .iter()
            .zip(&minp)
            .map(|(a, b)| (a.0, 2.0 * a.1 + b.1))
            .collect();
        (lpa, minp, setup, ext, gfind)
    };
    let (_, _, _, ext_k, gfind_k) = gen_series(&gk);
    let (lpa_n, minp_n, setup_n, ext_n, gfind_n) = gen_series(&gn);
    // Round-4 generic fit (Karatsuba, all primes 31..433) for reference.
    let r4lpa = per_ell(&r4gen, |r| {
        Some(meq(&r["ladder"])? / r["ladder_attempts_final_find"].as_f64()?)
    });
    let r4minp = per_ell(&r4gen, |r| meq(&r["minpoly"]));
    let r4find: Vec<(f64, f64)> = r4lpa
        .iter()
        .zip(&r4minp)
        .map(|(a, b)| (a.0, 2.0 * a.1 + b.1))
        .collect();
    let f_r4find = fit_loglog(&sel(&r4find, 30.0));
    let r4evalh = per_ell(&r4gen, |r| {
        r["eval_2pt_from_h"]["M"].as_f64().map(|m| m / 2.0)
    });
    let f_evalh = fit_loglog(&sel(&r4evalh, 30.0));
    let f_gk = fit_loglog(&sel(&gfind_k, 30.0));
    let f_gn = fit_loglog(&sel(&gfind_n, 30.0));
    let gn_hi_from = 256.0;
    let f_gn_hi = fit_loglog(&sel(&gfind_n, gn_hi_from));
    let f_lpa_n = fit_loglog(&sel(&lpa_n, 30.0));
    let f_minp_n = fit_loglog(&sel(&minp_n, 30.0));
    let f_setup_n = fit_loglog(&sel(&setup_n, 30.0));
    let f_ext_n = fit_loglog(&sel(&ext_n, 128.0));
    let f_ext_k = fit_loglog(&sel(&ext_k, 30.0));
    let f_gn_log = fit_logpower(&sel(&gfind_n, gn_hi_from), 2.0);
    let lg_top = gfind_n.last().unwrap().0;
    let gn_top = gfind_n.last().unwrap().1;
    fits.push(f_r4find.js(
        "round4_generic_find_expected (Karatsuba)",
        "31 <= ell <= 433",
    ));
    fits.push(f_gk.js(
        "generic_find_expected, Karatsuba arm (this run)",
        "ell >= 30",
    ));
    fits.push(f_gn.js("generic_find_expected, NTT arm", "ell >= 30"));
    fits.push(f_gn_hi.js("generic_find_expected, NTT arm", "ell >= 256"));
    fits.push(f_lpa_n.js("generic_ladder_per_attempt, NTT arm", "ell >= 30"));
    fits.push(f_minp_n.js("generic_minpoly, NTT arm", "ell >= 30"));
    fits.push(f_setup_n.js("generic_ext_setup (Ben-Or), NTT arm", "ell >= 30"));
    fits.push(f_ext_k.js("F_p^k product (vs ell), Karatsuba arm", "ell >= 30"));
    fits.push(f_ext_n.js("F_p^k product (vs ell), NTT arm", "ell >= 128"));
    fits.push(f_gn_log.js(
        "generic_find NTT: k in cost = c ell^2 (log2 ell)^k (slope reported as exponent)",
        "ell >= 256",
    ));
    fits.push(f_evalh.js(
        "round4_generic_eval_per_point_from_h (M; +1 I)",
        "ell >= 30",
    ));

    text.push_str("Fits: log2(cost) = log2_constant + exponent * log2(ell), OLS, 95% t-interval\n");
    for f in &fits {
        text.push_str(&format!(
            "  {:<84} {:<22} exponent {:.3} [{:.3}, {:.3}]  log2 c {:.2}  n={} r2={:.4}\n",
            f["series"].as_str().unwrap(),
            f["range"].as_str().unwrap(),
            f["exponent"].as_f64().unwrap(),
            f["exponent_ci95"][0].as_f64().unwrap(),
            f["exponent_ci95"][1].as_f64().unwrap(),
            f["log2_constant"].as_f64().unwrap(),
            f["n"].as_u64().unwrap(),
            f["r2"].as_f64().unwrap()
        ));
    }
    text.push_str("\nLocal sqrt-elu exponents over 4-octave windows (Karatsuba | NTT):\n");
    for w in &local {
        text.push_str(&format!(
            "  log2 ell {:>4.1}..{:>4.1}: {:.3} [{:.3}, {:.3}] | {:.3} [{:.3}, {:.3}]\n",
            w["log2_ell_from"].as_f64().unwrap(),
            w["log2_ell_to"].as_f64().unwrap(),
            w["karatsuba"][0].as_f64().unwrap(),
            w["karatsuba"][1].as_f64().unwrap(),
            w["karatsuba"][2].as_f64().unwrap(),
            w["ntt"][0].as_f64().unwrap(),
            w["ntt"][1].as_f64().unwrap(),
            w["ntt"][2].as_f64().unwrap()
        ));
    }
    text.push_str(&format!(
        "\nsqrt-elu at the top (ell = {l_top}): Karatsuba {sqk_top:.4e} Meq, NTT {sqn_top:.4e} Meq (ratio {:.2})\n",
        sqk_top / sqn_top
    ));

    // Derived crossover table.
    let mut table = Vec::new();
    text.push_str("\nDerived (extrapolated) crossover: smallest ell with find+evaluate >= g * sqrt(pi r / 4) F_p mults\n");
    let k_sq_n = sqn_top / (l_top.sqrt() * l_top.log2().powi(2));
    let k_g_n = gn_top / (lg_top * lg_top * lg_top.log2());
    for &g in &[6.0f64, 12.0] {
        for &rb in &[64.0f64, 128.0, 256.0] {
            let log2_rho = (g * (std::f64::consts::PI / 4.0).sqrt()).log2() + rb / 2.0;
            let s = rb / 61.0;
            let log2_lmax = rb / 2.0 + 1.0;
            let rho = 2f64.powf(log2_rho);
            let ev = |x: f64| 2f64.powf(f_evalh.eval_log2(x)) + 91.0;
            let c_velu = |x: f64| s * find_r_max + 2f64.powf(f_velu.eval_log2(x));
            let c_sqk = |x: f64| s * find_r_max + 2f64.powf(f_k16.eval_log2(x));
            let c_sqn = |x: f64| s * find_r_max + 2f64.powf(f_n16.eval_log2(x));
            let c_sqn_hi = |x: f64| s * find_r_max + 2f64.powf(f_n20.eval_log2(x));
            let c_sqn_asym = |x: f64| s * find_r_max + k_sq_n * 2f64.powf(x / 2.0) * x * x;
            let c_gr4 = |x: f64| s * 2f64.powf(f_r4find.eval_log2(x)) + ev(x);
            let c_gn = |x: f64| s * 2f64.powf(f_gn_hi.eval_log2(x)) + ev(x);
            let c_gn_lo = |x: f64| {
                let lg = f_gn_hi.icpt
                    + f_gn_hi.slope_lo * x
                    + (f_gn_hi.slope - f_gn_hi.slope_lo) * lg_top.log2();
                s * 2f64.powf(lg) + ev(x)
            };
            let c_gn_asym = |x: f64| s * k_g_n * 2f64.powf(2.0 * x) * x + ev(x);
            let row = |name: &str, c: &dyn Fn(f64) -> f64| -> Value {
                let x = crossover(c, rho, 1.0, 400.0);
                let note = match x {
                    Some(v) if v > log2_lmax => {
                        "beyond ell_max = 2 sqrt(p): no vertical gap that large exists"
                    }
                    Some(_) => "crossover inside the possible range",
                    None => "no crossover below 2^400",
                };
                json!({"model": name, "log2_ell_star": x, "note": note})
            };
            let rows = vec![
                row("rational kernel, Velu (fit ell>=2^10)", &c_velu),
                row(
                    "rational kernel, sqrt-elu Karatsuba (fit ell>=2^16, this run)",
                    &c_sqk,
                ),
                row("rational kernel, sqrt-elu NTT (fit ell>=2^16)", &c_sqn),
                row("rational kernel, sqrt-elu NTT (fit ell>=2^20)", &c_sqn_hi),
                row(
                    "rational kernel, sqrt-elu NTT (model sqrt(l) log^2 l, calibrated at top)",
                    &c_sqn_asym,
                ),
                row(
                    "generic kernel, round-4 Karatsuba fit (31..433, central)",
                    &c_gr4,
                ),
                row("generic kernel, NTT fit (ell>=256, central)", &c_gn),
                row(
                    "generic kernel, NTT fit (ell>=256, exponent at CI low end)",
                    &c_gn_lo,
                ),
                row(
                    "generic kernel, NTT model l^2 log l (calibrated at top)",
                    &c_gn_asym,
                ),
            ];
            text.push_str(&format!(
                "  g={g} M/op, r=2^{rb}: rho = 2^{log2_rho:.2} M, ell_max = 2^{log2_lmax:.0}\n"
            ));
            for r in &rows {
                text.push_str(&format!(
                    "    {:<76} log2 ell* = {:>8}  ({})\n",
                    r["model"].as_str().unwrap(),
                    r["log2_ell_star"]
                        .as_f64()
                        .map(|v| format!("{v:.2}"))
                        .unwrap_or("-".into()),
                    r["note"].as_str().unwrap()
                ));
            }
            table.push(
                json!({"g_mults_per_group_op": g, "log2_r": rb, "log2_rho_mults": log2_rho,
                              "log2_ell_max": log2_lmax, "logp_scale": s, "rows": rows}),
            );
        }
    }

    // Intermediate k: find(k) from the NTT generic fit at ℓ = 2k + 1 (or
    // the rational find for k = 1), evaluation √élu-NTT model × m(k).
    let p61 = 2_305_843_009_213_693_951u64;
    let fa = field(p61, PolyMul::Auto);
    let mut kt = Vec::new();
    text.push_str(
        "\nDerived: kernel over F_(p^k), r = 2^128, g = 6 (find(k) + sqrt-elu NTT model(ell) x m(k))\n",
    );
    let log2_rho128 = (6.0 * (std::f64::consts::PI / 4.0).sqrt()).log2() + 64.0;
    for &k in &[1usize, 2, 4, 8, 16, 32, 64, 128, 256] {
        let fk = Fpk {
            f: &fa,
            k,
            m: if k > 1 { 1 } else { 0 },
            c: 3,
        };
        let a: Vec<u64> = (0..k as u64).map(|i| i + 2).collect();
        let b: Vec<u64> = (0..k as u64).map(|i| i + 5).collect();
        fa.reset_counts();
        let _ = FieldArith::mul(&fk, &a, &b);
        let mkv = meq_of(&fa.tally()) as f64;
        let findk = if k == 1 {
            find_r_max
        } else {
            2f64.powf(f_gn_hi.eval_log2(((2 * k + 1) as f64).log2()))
        };
        let s = 128.0 / 61.0;
        let c = |x: f64| s * findk + mkv * k_sq_n * 2f64.powf(x / 2.0) * x * x;
        let x = crossover(c, 2f64.powf(log2_rho128), 1.0, 400.0);
        text.push_str(&format!(
            "  k={k:<4} m(k)={mkv:<7} find(k)=2^{:.1} M  log2 ell* = {}\n",
            (s * findk).log2(),
            x.map(|v| format!("{v:.1}")).unwrap_or("-".into())
        ));
        kt.push(json!({"k": k, "ext_mult_cost_Meq": mkv,
                       "log2_find_M_scaled": (s * findk).log2(), "log2_ell_star": x}));
    }
    text.push_str(&format!(
        "\nrational find: max {find_r_max:.0} Meq per kernel point (61-bit p)\n"
    ));
    print!("{text}");
    let res = json!({
        "mode": "analyze", "fits": fits, "local_exponents": local, "crossover": table,
        "intermediate_k_r128_g6": kt,
        "round4_reproduction": {
            "rational_rows_matched": rat_match.0, "rational_same_instance": rat_match.1,
            "rational_same_karatsuba_sqrtvelu_counts": rat_match.2,
            "rational_same_find_counts": rat_match.3,
            "generic_rows_matched": gen_match.0, "generic_same_instance": gen_match.1,
            "generic_same_ladder_counts": gen_match.2,
        },
        "rational_find_max_Meq": find_r_max,
        "sqrtvelu_ntt_calibration": {"ell": l_top, "Meq": sqn_top, "k_sqrt_log2sq": k_sq_n},
        "generic_ntt_calibration": {"ell": lg_top, "Meq": gn_top, "k_l2_log": k_g_n},
        "assumptions": {
            "rho_group_ops": "sqrt(pi r / 4)",
            "g": "field mults per group op: 6 (affine add with batched inversion), 12 as sensitivity",
            "logp_scale": "ladder-type phases scale with log2 p / 61, p ~ r; Velu/sqrt-elu counts taken as p-independent",
            "ell_max": "ell^2 | t^2 - 4p implies ell <= 2 sqrt(p)",
            "ntt_word": "1 NTT word multiplication (mod a 62-bit prime) = 1 M",
            "generic_eval": "kernel-polynomial evaluation from the round-4 fit (O(ell) per point)",
        },
        "text": text,
    });
    std::fs::write(out, serde_json::to_string_pretty(&res).unwrap()).expect("write");
    res
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let usage = || -> ! {
        eprintln!(
            "usage: {0} threshold <out>\n       {0} rational <min_log2> <max_log2> <seeds> <velu_max_log2> <out>\n       {0} generic <min_ell> <max_ell> <step_log2> <cap_s> <karatsuba|auto|ntt> <out>\n       {0} analyze <dir> <round4_dir> <out>",
            args[0]
        );
        std::process::exit(2)
    };
    if args.len() < 3 {
        usage();
    }
    let t0 = Instant::now();
    let (mut out, path) = match args[1].as_str() {
        "threshold" => (mode_threshold(), &args[2]),
        "rational" if args.len() == 7 => (
            mode_rational(
                args[2].parse().unwrap(),
                args[3].parse().unwrap(),
                args[4].parse().unwrap(),
                args[5].parse().unwrap(),
            ),
            &args[6],
        ),
        "generic" if args.len() == 8 => {
            let arm = match args[6].as_str() {
                "karatsuba" => PolyMul::Karatsuba,
                "auto" => PolyMul::Auto,
                "ntt" => PolyMul::Ntt,
                _ => usage(),
            };
            (
                mode_generic(
                    args[2].parse().unwrap(),
                    args[3].parse().unwrap(),
                    args[4].parse().unwrap(),
                    args[5].parse().unwrap(),
                    arm,
                ),
                &args[7],
            )
        }
        "analyze" if args.len() == 5 => (mode_analyze(&args[2], &args[3], &args[4]), &args[4]),
        _ => usage(),
    };
    if args[1] == "analyze" {
        return;
    }
    out["argv"] = json!(args);
    out["unit"] = json!("M = F_p mults+squarings (incl. schoolbook/Karatsuba coefficient products); W = word mults mod 62-bit NTT primes, counted as 1 M; I = F_p inversions; Meq = M + W + 91 I");
    out["field_bits"] = json!(61);
    out["total_wall_s_contended"] = json!(t0.elapsed().as_secs_f64());
    std::fs::write(path, serde_json::to_string_pretty(&out).unwrap()).expect("write output");
}
