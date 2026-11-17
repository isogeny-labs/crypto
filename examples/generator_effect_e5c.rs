//! E5c of research/generator_effect_ic_20261019: is the standard generator a
//! small multiple of the first point?  For P-192 and P-224, `[h]·P` for
//! `1 ≤ h ≤ 2^24` is compared with `±G`, for the lexicographically first point
//! `P_first` (both roots) and for the first 16 valid abscissae.
//!
//! `cargo run --release --example generator_effect_e5c -- OUT.json`
//!
//! Frozen in `research/generator_effect_ic_20261019/e5/PROTOCOL_E5c.md`; this
//! program implements that text and nothing else.  It closes trapdoor-program
//! slot S4, rule E4a "`G = [h]·(first point)`", for `h ≤ 2^24` only.  No ECDLP
//! run, no security claim.
//!
//! Arithmetic (from `generator_effect_e5a.rs`): 4-limb Montgomery field
//! elements, Jacobian points with `a = −3` doubling and mixed addition, one
//! batched inversion per chunk of `2^16` consecutive multiples.  The BigUint
//! arithmetic of `src/ecc` checks the results and plants the positive control.
use crypto_lib::ct_bignum::{MontgomeryContext, Uint};
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::point::Point;
use num_bigint::BigUint;
use rayon::prelude::*;
use serde_json::{json, Value};
use std::time::Instant;

type U = Uint<4>;

const CHUNK_BITS: u32 = 16;
const CHUNK: usize = 1 << CHUNK_BITS;
const LOG2_H: u32 = 24;
const N_CHUNKS: usize = 1 << (LOG2_H - CHUNK_BITS);
const N_ABSCISSAE: usize = 16;
const CONTROL_H: u64 = 12345;
const CHECKPOINTS: [u64; 9] = [
    1,
    2,
    3,
    1000,
    CONTROL_H,
    1 << 16,
    (1 << 16) + 1,
    1 << 20,
    1 << 24,
];

// ── field ────────────────────────────────────────────────────────────────

struct Fp {
    ctx: MontgomeryContext<4>,
    p: U,
    one: U,
    bits: u32,
}

impl Fp {
    fn new(p: &BigUint) -> Self {
        let pu = U::from_biguint(p);
        let ctx = MontgomeryContext::new(pu).expect("odd prime");
        let one = ctx.r_mod_n;
        Fp {
            ctx,
            p: pu,
            one,
            bits: p.bits() as u32,
        }
    }
    fn mul(&self, a: &U, b: &U) -> U {
        self.ctx.mont_mul(a, b)
    }
    fn sqr(&self, a: &U) -> U {
        self.ctx.mont_sqr(a)
    }
    fn add(&self, a: &U, b: &U) -> U {
        a.add_mod(b, &self.p)
    }
    fn sub(&self, a: &U, b: &U) -> U {
        a.sub_mod(b, &self.p)
    }
    fn dbl(&self, a: &U) -> U {
        self.add(a, a)
    }
    fn to_m(&self, a: &U) -> U {
        self.ctx.to_montgomery(a)
    }
    fn canon(&self, a: &U) -> U {
        self.ctx.from_montgomery(a)
    }
    /// Variable-time `a^e` in Montgomery form; `e` canonical.
    fn pow(&self, a: &U, e: &U) -> U {
        let mut acc = self.one;
        for bit in (0..256).rev() {
            acc = self.sqr(&acc);
            if (e.0[bit / 64] >> (bit % 64)) & 1 == 1 {
                acc = self.mul(&acc, a);
            }
        }
        acc
    }
    fn inv(&self, a: &U) -> U {
        let pm2 = U::from_biguint(&(self.p.to_biguint() - 2u32));
        self.pow(a, &pm2)
    }
}

fn is_zero(a: &U) -> bool {
    a.0.iter().all(|&l| l == 0)
}

fn bitlen(a: &U) -> u32 {
    for k in (0..4).rev() {
        if a.0[k] != 0 {
            return 64 * k as u32 + 64 - a.0[k].leading_zeros();
        }
    }
    0
}

fn xor(a: &U, b: &U) -> U {
    let mut v = U::ZERO;
    for k in 0..4 {
        v.0[k] = a.0[k] ^ b.0[k];
    }
    v
}

fn hex(a: &U) -> String {
    format!("0x{}", a.to_biguint().to_str_radix(16))
}

// ── points ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct Jac {
    x: U,
    y: U,
    z: U,
}

/// Affine point; coordinates in Montgomery form.
#[derive(Clone, Copy)]
struct Aff {
    x: U,
    y: U,
}

struct Curve {
    f: Fp,
    b: U,
    three: U,
}

impl Curve {
    fn new(c: &CurveParams) -> Self {
        let f = Fp::new(&c.p);
        let b = f.to_m(&U::from_biguint(&c.b));
        let three = f.to_m(&U::from_biguint(&BigUint::from(3u32)));
        Curve { f, b, three }
    }

    fn aff(&self, x: &BigUint, y: &BigUint) -> Aff {
        Aff {
            x: self.f.to_m(&U::from_biguint(x)),
            y: self.f.to_m(&U::from_biguint(y)),
        }
    }

    fn inf(&self) -> Jac {
        Jac {
            x: self.f.one,
            y: self.f.one,
            z: U::ZERO,
        }
    }

    /// dbl-2001-b, `a = −3`.
    fn double(&self, p: &Jac) -> Jac {
        let f = &self.f;
        if is_zero(&p.z) || is_zero(&p.y) {
            return self.inf();
        }
        let delta = f.sqr(&p.z);
        let gamma = f.sqr(&p.y);
        let beta = f.mul(&p.x, &gamma);
        let alpha = f.mul(
            &self.three,
            &f.mul(&f.sub(&p.x, &delta), &f.add(&p.x, &delta)),
        );
        let beta4 = f.dbl(&f.dbl(&beta));
        let x3 = f.sub(&f.sqr(&alpha), &f.dbl(&beta4));
        let yz = f.add(&p.y, &p.z);
        let z3 = f.sub(&f.sub(&f.sqr(&yz), &gamma), &delta);
        let g2 = f.sqr(&gamma);
        let g8 = f.dbl(&f.dbl(&f.dbl(&g2)));
        let y3 = f.sub(&f.mul(&alpha, &f.sub(&beta4, &x3)), &g8);
        Jac {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// madd-2007-bl, with the `P = ±Q` cases handled.
    fn madd(&self, p: &Jac, q: &Aff) -> Jac {
        let f = &self.f;
        if is_zero(&p.z) {
            return Jac {
                x: q.x,
                y: q.y,
                z: f.one,
            };
        }
        let z1z1 = f.sqr(&p.z);
        let u2 = f.mul(&q.x, &z1z1);
        let s2 = f.mul(&q.y, &f.mul(&p.z, &z1z1));
        let h = f.sub(&u2, &p.x);
        let r = f.dbl(&f.sub(&s2, &p.y));
        if is_zero(&h) {
            return if is_zero(&r) {
                self.double(p)
            } else {
                self.inf()
            };
        }
        let hh = f.sqr(&h);
        let i = f.dbl(&f.dbl(&hh));
        let j = f.mul(&h, &i);
        let v = f.mul(&p.x, &i);
        let x3 = f.sub(&f.sub(&f.sqr(&r), &j), &f.dbl(&v));
        let y3 = f.sub(&f.mul(&r, &f.sub(&v, &x3)), &f.dbl(&f.mul(&p.y, &j)));
        let zh = f.add(&p.z, &h);
        let z3 = f.sub(&f.sub(&f.sqr(&zh), &z1z1), &hh);
        Jac {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Batched normalisation to affine (Montgomery form); no point may be O.
    fn normalize(&self, pts: &[Jac]) -> Vec<Aff> {
        let f = &self.f;
        let mut prefix = Vec::with_capacity(pts.len());
        let mut acc = f.one;
        for p in pts {
            assert!(!is_zero(&p.z), "point at infinity in a batch");
            prefix.push(acc);
            acc = f.mul(&acc, &p.z);
        }
        let mut inv = f.inv(&acc);
        let mut out = vec![
            Aff {
                x: U::ZERO,
                y: U::ZERO
            };
            pts.len()
        ];
        for k in (0..pts.len()).rev() {
            let zi = f.mul(&inv, &prefix[k]);
            inv = f.mul(&inv, &pts[k].z);
            let zi2 = f.sqr(&zi);
            out[k] = Aff {
                x: f.mul(&pts[k].x, &zi2),
                y: f.mul(&pts[k].y, &f.mul(&zi2, &zi)),
            };
        }
        out
    }

    /// Left-to-right double-and-add, `k ≥ 1`.
    fn mul_u64(&self, p: &Aff, k: u64) -> Jac {
        let mut acc = self.inf();
        for bit in (0..64).rev() {
            acc = self.double(&acc);
            if (k >> bit) & 1 == 1 {
                acc = self.madd(&acc, p);
            }
        }
        acc
    }

    /// `x³ − 3x + b` in Montgomery form; `x` canonical.
    fn rhs(&self, x: &U) -> U {
        let f = &self.f;
        let xm = f.to_m(x);
        f.add(
            &f.sub(&f.mul(&f.sqr(&xm), &xm), &f.mul(&self.three, &xm)),
            &self.b,
        )
    }

    /// `x³ − 3x + b` is a square (or zero) in `F_p`; `x` canonical.
    fn x_on_curve(&self, x: &U) -> bool {
        let f = &self.f;
        let rhs = self.rhs(x);
        if is_zero(&rhs) {
            return true;
        }
        let e = U::from_biguint(&((f.p.to_biguint() - 1u32) >> 1));
        f.canon(&f.pow(&rhs, &e)).0 == U::ONE.0
    }
}

/// Tonelli–Shanks square root in BigUint (P-224 has `p ≡ 1 mod 2^96`).
fn sqrt_mod(a: &BigUint, p: &BigUint) -> Option<BigUint> {
    let zero = BigUint::from(0u32);
    let one = BigUint::from(1u32);
    if *a == zero {
        return Some(zero);
    }
    let pm1 = p - &one;
    if a.modpow(&(&pm1 >> 1), p) != one {
        return None;
    }
    let s = pm1.trailing_zeros().unwrap();
    let q = &pm1 >> s;
    let mut z = BigUint::from(2u32);
    while z.modpow(&(&pm1 >> 1), p) != pm1 {
        z += 1u32;
    }
    let mut m = s;
    let mut c = z.modpow(&q, p);
    let mut t = a.modpow(&q, p);
    let mut r = a.modpow(&((&q + &one) >> 1), p);
    while t != one {
        let mut i = 0u64;
        let mut tt = t.clone();
        while tt != one {
            tt = &tt * &tt % p;
            i += 1;
        }
        let b = c.modpow(&(BigUint::from(1u32) << (m - i - 1)), p);
        m = i;
        c = &b * &b % p;
        t = &t * &c % p;
        r = &r * &b % p;
    }
    Some(r)
}

// ── scan ─────────────────────────────────────────────────────────────────

struct Scan {
    /// (h, +1 for `[h]P = T`, −1 for `[h]P = −T`)
    matches: Vec<(u64, i32)>,
    /// `c = ℓ − bitlen(x([h]P) XOR T.x)`, leading bits shared with `T.x`
    shared: Vec<u64>,
    /// (h, canonical x, canonical y) at the checkpoints
    samples: Vec<(u64, U, U)>,
}

/// Every `[h]P`, `1 ≤ h ≤ 2^24`, against `±T`; `P`, `T` in Montgomery form.
fn scan(cv: &Curve, p: &Aff, t: &Aff) -> Scan {
    let f = &cv.f;
    let ell = f.bits;
    let tx = f.canon(&t.x);
    let ty = f.canon(&t.y);
    let neg_ty = f.canon(&f.sub(&U::ZERO, &t.y));
    let parts: Vec<Scan> = (0..N_CHUNKS)
        .into_par_iter()
        .map(|c| {
            let h0 = (c * CHUNK) as u64 + 1;
            let mut pts = Vec::with_capacity(CHUNK);
            let mut q = cv.mul_u64(p, h0);
            pts.push(q);
            for _ in 1..CHUNK {
                q = cv.madd(&q, p);
                pts.push(q);
            }
            let aff = cv.normalize(&pts);
            let mut out = Scan {
                matches: Vec::new(),
                shared: vec![0; 257],
                samples: Vec::new(),
            };
            for (k, a) in aff.iter().enumerate() {
                let h = h0 + k as u64;
                let x = f.canon(&a.x);
                out.shared[(ell - bitlen(&xor(&x, &tx))) as usize] += 1;
                if x.0 == tx.0 {
                    let y = f.canon(&a.y);
                    if y.0 == ty.0 {
                        out.matches.push((h, 1));
                    } else if y.0 == neg_ty.0 {
                        out.matches.push((h, -1));
                    }
                }
                if CHECKPOINTS.contains(&h) {
                    out.samples.push((h, x, f.canon(&a.y)));
                }
            }
            out
        })
        .collect();
    let mut all = Scan {
        matches: Vec::new(),
        shared: vec![0; 257],
        samples: Vec::new(),
    };
    for s in parts {
        all.matches.extend(s.matches);
        for k in 0..257 {
            all.shared[k] += s.shared[k];
        }
        all.samples.extend(s.samples);
    }
    all
}

fn scan_json(s: &Scan, ell: u32, seconds: f64) -> Value {
    let max_c = (0..=ell as usize)
        .rev()
        .find(|&k| s.shared[k] > 0)
        .unwrap_or(0);
    json!({
        "h_max": 1u64 << LOG2_H,
        "matches": s.matches.iter().map(|(h, sg)| json!({"h": h, "sign": sg})).collect::<Vec<_>>(),
        "max_shared_bits": max_c,
        "shared_bits_hist": s.shared[..=max_c].to_vec(),
        "seconds": seconds,
    })
}

fn affine_big(pt: &Point) -> (BigUint, BigUint) {
    match pt {
        Point::Affine { x, y } => (x.value.clone(), y.value.clone()),
        Point::Infinity => panic!("unexpected point at infinity"),
    }
}

fn main() {
    let out = std::env::args()
        .nth(1)
        .expect("usage: generator_effect_e5c OUT.json");
    let curves = [CurveParams::p192(), CurveParams::p224()];
    let mut curve_out = Vec::new();
    let mut all_checks_pass = true;
    let mut any_match = false;

    for cp in &curves {
        let t_curve = Instant::now();
        let cv = Curve::new(cp);
        let ell = cv.f.bits;
        let a_fe = cp.a_fe();
        let g = cp.generator();
        let g_aff = cv.aff(&cp.gx, &cp.gy);

        // Check 1: G on the curve, [n]G = O.
        let g_on = cp.is_on_curve(&g);
        let ng_inf = matches!(g.scalar_mul_vartime(&cp.n, &a_fe), Point::Infinity);

        // The first N_ABSCISSAE valid abscissae and their roots.
        let mut bases: Vec<(BigUint, BigUint)> = Vec::new();
        let mut x = U::ZERO;
        while bases.len() < N_ABSCISSAE {
            if cv.x_on_curve(&x) {
                let xb = x.to_biguint();
                let rhs = cv.f.canon(&cv.rhs(&x)).to_biguint();
                let y = sqrt_mod(&rhs, &cp.p).expect("square");
                let y_small = y.clone().min(&cp.p - &y);
                bases.push((xb, y_small));
            }
            x = x.add_mod(&U::ONE, &cv.f.p);
        }
        let bases_on = bases.iter().all(|(x, y)| {
            cp.is_on_curve(&Point::Affine {
                x: cp.fe(x.clone()),
                y: cp.fe(y.clone()),
            })
        });
        let (x_first, y_first) = bases[0].clone();
        let p_first = Point::Affine {
            x: cp.fe(x_first.clone()),
            y: cp.fe(y_first.clone()),
        };
        let pf_aff = cv.aff(&x_first, &y_first);

        // Check 3 (positive control): T = [12345]P_first by BigUint, then ±T.
        let tpt = p_first.scalar_mul_vartime(&BigUint::from(CONTROL_H), &a_fe);
        let (tx, ty) = affine_big(&tpt);
        let mut controls = Vec::new();
        let mut controls_pass = true;
        for (label, ty_s, want) in [("+T", ty.clone(), 1i32), ("-T", &cp.p - &ty, -1i32)] {
            let ts = Instant::now();
            let s = scan(&cv, &pf_aff, &cv.aff(&tx, &ty_s));
            let ok = s.matches == vec![(CONTROL_H, want)];
            controls_pass &= ok;
            eprintln!(
                "{} control {label}: matches {:?} ({:.1} s)",
                cp.name,
                s.matches,
                ts.elapsed().as_secs_f64()
            );
            let mut v = scan_json(&s, ell, ts.elapsed().as_secs_f64());
            v["target"] = json!(label);
            v["planted_h"] = json!(CONTROL_H);
            v["expected_sign"] = json!(want);
            v["pass"] = json!(ok);
            controls.push(v);
        }

        // Primary (both roots of P_first) and extension (15 more abscissae, smaller root).
        let mut runs: Vec<(usize, String, BigUint, BigUint)> = vec![
            (0, "small".into(), x_first.clone(), y_first.clone()),
            (0, "large".into(), x_first.clone(), &cp.p - &y_first),
        ];
        for (k, (x, y)) in bases.iter().enumerate().skip(1) {
            runs.push((k, "small".into(), x.clone(), y.clone()));
        }
        let mut base_out = Vec::new();
        let mut mism = 0usize;
        let mut checked = 0usize;
        for (k, root, x, y) in &runs {
            let ts = Instant::now();
            let s = scan(&cv, &cv.aff(x, y), &g_aff);
            let secs = ts.elapsed().as_secs_f64();
            any_match |= !s.matches.is_empty();
            eprintln!(
                "{} base {k} (x = {x}, {root} y): {} matches, max shared {} bits ({secs:.1} s)",
                cp.name,
                s.matches.len(),
                (0..=ell as usize).rev().find(|&c| s.shared[c] > 0).unwrap()
            );
            // Check 2: the incremental path against BigUint, on P_first's two roots.
            if *k == 0 {
                let bp = Point::Affine {
                    x: cp.fe(x.clone()),
                    y: cp.fe(y.clone()),
                };
                for (h, sx, sy) in &s.samples {
                    checked += 1;
                    let (bx, by) = affine_big(&bp.scalar_mul_vartime(&BigUint::from(*h), &a_fe));
                    if bx != sx.to_biguint() || by != sy.to_biguint() {
                        mism += 1;
                    }
                }
            }
            let mut v = scan_json(&s, ell, secs);
            v["abscissa_index"] = json!(k);
            v["x"] = json!(format!("0x{}", x.to_str_radix(16)));
            v["y"] = json!(format!("0x{}", y.to_str_radix(16)));
            v["root"] = json!(root);
            v["primary"] = json!(*k == 0);
            base_out.push(v);
        }
        let checks_pass = g_on
            && ng_inf
            && bases_on
            && mism == 0
            && checked == 2 * CHECKPOINTS.len()
            && controls_pass;
        all_checks_pass &= checks_pass;

        curve_out.push(json!({
            "curve": cp.name,
            "ell": ell,
            "gx": hex(&U::from_biguint(&cp.gx)),
            "x_first": format!("0x{}", x_first.to_str_radix(16)),
            "checks": {
                "g_on_curve": g_on,
                "n_g_is_infinity": ng_inf,
                "bases_on_curve": bases_on,
                "biguint_crosscheck": {"checked": checked, "mismatches": mism,
                                        "h": CHECKPOINTS.to_vec()},
                "positive_control_pass": controls_pass,
                "pass": checks_pass,
            },
            "positive_control": controls,
            "bases": base_out,
            "seconds": t_curve.elapsed().as_secs_f64(),
        }));
    }

    let result = json!({
        "experiment": "E5c",
        "protocol": "research/generator_effect_ic_20261019/e5/PROTOCOL_E5c.md",
        "rule": "G = ±[h]·P, 1 ≤ h ≤ 2^24, P in the first 16 valid abscissae (P_first: both roots)",
        "threads": rayon::current_num_threads(),
        "all_checks_pass": all_checks_pass,
        "any_match": any_match,
        "curves": curve_out,
    });
    std::fs::write(&out, serde_json::to_string_pretty(&result).unwrap()).unwrap();
    eprintln!("checks pass: {all_checks_pass}; any match: {any_match}; wrote {out}");
}
