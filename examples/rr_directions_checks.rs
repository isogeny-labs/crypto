//! Exact checks behind two "RR directions" notes (Rust reproductions of
//! `research/rr_directions_20261115/pick1_torsion_quotient/isogeny_check.py` and
//! `.../pick3_lattice_count/lattice_count.py`).
//!
//!     rr_directions_checks isogeny
//!     rr_directions_checks lattice [--targets 8]
//!
//! `isogeny`: on `y² + xy = x³ + 1` over GF(2^m), for every affine point P with x ≠ 0
//! (T = (0,1) is 2-torsion), checks (a) x(P+T) = 1/x(P); (b) t = x + 1/x is the same for
//! P and P+T; (c) x(P + τP) = t(P) + c with one constant c over all P.
//!
//! `lattice`: over F_p, y² = x³ + 2x + 3, m = 3, B = round(p^{1/3}); per random target the
//! box count N_box, the Vieta-image count N_split and the relation count N_rel over all
//! (a, b) ∈ F_p².  The RNG is a seeded `StdRng` (seed 20261115 + p), NOT Python's
//! `random.Random`, so individual targets differ from the Python run; only the statistics
//! are comparable.
use crypto_lib::binary_ecc::curve::point_add;
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet};

fn word(e: &F2mElement) -> u64 {
    e.raw_bits().first().copied().unwrap_or(0)
}

/// Affine points with abscissa `x != 0`.
fn points_over(curve: &BinaryCurve, x: &F2mElement) -> Vec<BinaryPoint> {
    let (n, irr) = (curve.m, &curve.irreducible);
    // y = x z, z² + z = x + a + 1/x²
    let c = x
        .add(&curve.a)
        .add(&x.square(irr).flt_inverse(irr).expect("x != 0"));
    if c.trace(irr) != 0 {
        return vec![];
    }
    let mut z = c.clone();
    let mut t = c.clone();
    for _ in 0..(n - 1) / 2 {
        t = t.square_k_times(2, irr);
        z = z.add(&t);
    }
    let y = x.mul(&z, irr);
    let pts = vec![
        BinaryPoint::Affine { x: x.clone(), y: y.clone() },
        BinaryPoint::Affine { x: x.clone(), y: y.add(x) },
    ];
    assert!(pts.iter().all(|p| curve.is_on_curve(p)), "point over x");
    pts
}

fn isogeny() {
    for m in [5u32, 7, 9, 11, 13] {
        // KoblitzCurve::new(0, 11) is None (r^2 | #E), so the curve is built directly with
        // a fixed irreducible polynomial; the field is the same abstract GF(2^m).
        let low: Vec<u32> = match m {
            5 => vec![0, 2],
            7 => vec![0, 1],
            9 => vec![0, 4],
            11 => vec![0, 2],
            13 => vec![0, 1, 3, 4],
            _ => unreachable!(),
        };
        let curve = BinaryCurve {
            m,
            irreducible: IrreduciblePoly { degree: m, low_terms: low },
            a: F2mElement::zero(m),
            b: F2mElement::one(m),
            generator: BinaryPoint::Infinity,
            order: BigUint::from(1u32),
            cofactor: BigUint::from(1u32),
        };
        let irr: &IrreduciblePoly = &curve.irreducible;
        let add = |p: &BinaryPoint, q: &BinaryPoint| point_add(&curve, p, q);
        let one = F2mElement::one(m);
        let t_pt = BinaryPoint::Affine { x: F2mElement::zero(m), y: one.clone() };
        assert!(curve.is_on_curve(&t_pt));
        assert!(matches!(add(&t_pt, &t_pt), BinaryPoint::Infinity), "T has order 2");
        let mut pts = Vec::new();
        for xi in 1u64..(1u64 << m) {
            pts.extend(points_over(&curve, &F2mElement::from_words(&[xi], m)));
        }
        let (mut ok_a, mut ok_b, mut ok_c) = (0u64, 0u64, 0u64);
        let mut cst: Option<F2mElement> = None;
        for p in &pts {
            let BinaryPoint::Affine { x, y } = p else { unreachable!() };
            let xinv = x.flt_inverse(irr).expect("x != 0");
            let q = add(p, &t_pt);
            let BinaryPoint::Affine { x: qx, .. } = &q else {
                panic!("P+T = O")
            };
            if *qx == xinv {
                ok_a += 1;
            }
            let t = x.add(&xinv);
            let tq = qx.add(&qx.flt_inverse(irr).expect("qx != 0"));
            if tq == t {
                ok_b += 1;
            }
            let frob = BinaryPoint::Affine { x: x.square(irr), y: y.square(irr) };
            if let BinaryPoint::Affine { x: sx, .. } = add(p, &frob) {
                let d = sx.add(&t);
                let c0 = cst.get_or_insert_with(|| d.clone());
                if d == *c0 {
                    ok_c += 1;
                }
            }
        }
        let c = cst.expect("some point");
        let c_str = if c == one { "one".to_string() } else { format!("{:#x}", word(&c)) };
        println!(
            "m={m}: {} affine points; x(P+T)=1/x: {ok_a} ok; t invariant: {ok_b} ok; x(P+tauP) = t + c with c = {c_str} (word {}): {ok_c} ok",
            pts.len(),
            word(&c)
        );
        println!(
            r#"{{"check":"isogeny","m":{m},"affine_points":{},"a_ok":{ok_a},"b_ok":{ok_b},"c_ok":{ok_c},"c_word":{},"c_is_zero":{}}}"#,
            pts.len(),
            word(&c),
            c.is_zero()
        );
    }
}

fn powmod(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}

fn sqrt3mod4(v: u64, p: u64) -> Option<u64> {
    let r = powmod(v, (p + 1) / 4, p);
    (r * r % p == v % p).then_some(r)
}

fn lattice(targets: usize) {
    const A: u64 = 2;
    const BC: u64 = 3;
    for p in [503u64, 1019, 2027, 4091, 8191] {
        assert!(p % 4 == 3);
        let mut rng = StdRng::seed_from_u64(20261115 + p);
        let bb = (p as f64).powf(1.0 / 3.0).round() as u64;
        let (box1, box2, box3) = (3 * bb + 1, 3 * bb * bb + 1, bb * bb * bb + 1);
        let rhs = |x: u64| (x * x % p * x + A * x + BC) % p;
        let fb: Vec<u64> = (0..=bb).filter(|&x| sqrt3mod4(rhs(x), p).is_some()).collect();
        let fbset: HashSet<u64> = fb.iter().copied().collect();
        let mut vieta: HashMap<(u64, u64, u64), Vec<(u64, u64, u64)>> = HashMap::new();
        for x1 in 0..=bb {
            for x2 in x1..=bb {
                for x3 in x2..=bb {
                    let key = (
                        (x1 + x2 + x3) % p,
                        (x1 * x2 + x1 * x3 + x2 * x3) % p,
                        x1 * x2 * x3 % p,
                    );
                    vieta.entry(key).or_default().push((x1, x2, x3));
                }
            }
        }
        let (mut sb, mut ss, mut sr) = (0u64, 0u64, 0u64);
        for _ in 0..targets {
            let (xr, yr) = loop {
                let xr = rng.gen_range(bb + 1..p);
                if let Some(yr) = sqrt3mod4(rhs(xr), p) {
                    break (xr, yr);
                }
            };
            let (mut n_box, mut n_split, mut n_rel) = (0u64, 0u64, 0u64);
            for a in 0..p {
                let axr = a * xr % p;
                for b in 0..p {
                    let c = (2 * p * p - xr * xr % p - axr + b * yr) % p;
                    let b2 = b * b % p;
                    let n3 = (2 * a + p - b2) % p;
                    let n2 = (a * a + 2 * c) % p;
                    let n1 = (2 * a % p * c % p + p * p - A * b2) % p;
                    let n0 = (c * c % p + p * p - BC * b2) % p;
                    let f2 = (n3 + xr) % p;
                    let f1 = (n2 + xr * f2) % p;
                    let f0 = (n1 + xr * f1) % p;
                    let rem = (n0 + xr * f0) % p;
                    assert_eq!(rem, 0, "division by (x - x_R) is exact");
                    let (e1, e2, e3) = ((p - f2) % p, f1, (p - f0) % p);
                    if e1 < box1 && e2 < box2 && e3 < box3 {
                        n_box += 1;
                        if let Some(trips) = vieta.get(&(e1, e2, e3)) {
                            n_split += 1;
                            if trips.iter().any(|t| {
                                fbset.contains(&t.0) && fbset.contains(&t.1) && fbset.contains(&t.2)
                            }) {
                                n_rel += 1;
                            }
                        }
                    }
                }
            }
            sb += n_box;
            ss += n_split;
            sr += n_rel;
        }
        let t = targets as f64;
        let pf = p as f64;
        let heur = pf * pf * (box1 as f64 / pf) * (box2 as f64 / pf) * ((box3.min(p)) as f64 / pf);
        let cfb = (fb.len() * (fb.len().saturating_sub(1)) * (fb.len().saturating_sub(2))) as f64 / 6.0 / pf;
        println!(
            "p={p} B={bb} |fb|={} boxes=[{box1},{box2},{box3}] N_box_mean={:.1} heuristic={:.1} N_split_mean={:.3} N_rel_mean={:.3} C(fb,3)/p={:.3}",
            fb.len(),
            sb as f64 / t,
            heur,
            ss as f64 / t,
            sr as f64 / t,
            cfb
        );
        println!(
            r#"{{"check":"lattice","p":{p},"B":{bb},"factor_base_size":{},"boxes":[{box1},{box2},{box3}],"targets":{targets},"rng":"StdRng(20261115+p)","N_box_mean":{},"N_box_heuristic":{heur},"N_split_mean":{},"N_rel_mean":{},"N_rel_heuristic_C(fb,3)/p":{cfb}}}"#,
            fb.len(),
            sb as f64 / t,
            ss as f64 / t,
            sr as f64 / t
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("isogeny") => isogeny(),
        Some("lattice") => {
            let targets = args
                .iter()
                .position(|a| a == "--targets")
                .and_then(|i| args.get(i + 1))
                .map_or(8, |v| v.parse().expect("bad --targets"));
            lattice(targets)
        }
        _ => eprintln!("usage: rr_directions_checks isogeny | lattice [--targets 8]"),
    }
}
