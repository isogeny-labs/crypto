//! **The three-pair S₄ join, measured — the "concrete open design" of
//! `RESEARCH_ECC2K130_IC_FEASIBILITY.md` §4.7.**
//!
//! A 6-summand relation over a factor base is a 3-sum over its pair-sum
//! table `T`.  This driver implements and measures both probe shapes on the
//! Koblitz `a = 0` curve at a completable scale (n = 37, subgroup
//! `r ≈ 2^35`, dim-8 standard-subspace base):
//!
//! - **Group join** (control): sample two pair sums, subtract both from the
//!   target in the group, look the residual abscissa up in `T`.  Cost per
//!   relation ≈ `4r/|T|` trials.
//! - **S₄-chain probe** (the note's design): sample two pair-sum *abscissae*
//!   `x₁, x₂`, solve `S₃(x₁, x₂, T) = 0` for the intermediate abscissa (one
//!   binary-field quadratic, half-trace), then solve `S₃(X, x_R, T) = 0`
//!   for the third pair's abscissa (a second quadratic), and look the ≤ 4
//!   roots up in `T`'s x-key table.  No group arithmetic per probe; every
//!   x-hit is verified in the group.
//!
//! It also measures the 4-sum density at the same base (`E₄ = |T|²/2·4r`)
//! against the 6-sum density (`E₆ = |T|³/6·4r`) — the arity/density trade the
//! feasibility note's K⁴ model rests on.
//!
//! Known-answer only: targets are fresh random subgroup points.
//!
//! ```bash
//! cargo run --release --example koblitz_sixsum_s4join_n37 -- 8 6
//! ```

use crypto_lib::binary_ecc::curve::{point_add, point_neg, scalar_mul, BinaryPoint};
use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_standard_subspace_factor_base, cofactor_project_factor_base, KoblitzCurve,
};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Instant;

fn elem_bits(v: &F2mElement) -> u128 {
    let raw = v.raw_bits();
    let mut b = raw[0] as u128;
    if raw.len() > 1 {
        b |= (raw[1] as u128) << 64;
    }
    b
}

fn point_x(p: &BinaryPoint) -> Option<F2mElement> {
    match p {
        BinaryPoint::Affine { x, .. } => Some(x.clone()),
        BinaryPoint::Infinity => None,
    }
}

/// Roots of `S₃(a, b, T) = (a+b)²T² + ab·T + (ab)² + b_curve = 0`.
/// Returns the distinct roots in the field (0, 1 or 2 of them).
fn s3_roots(
    a: &F2mElement,
    b: &F2mElement,
    b_curve: &F2mElement,
    irr: &IrreduciblePoly,
    n: u32,
) -> Vec<F2mElement> {
    let sum = a.add(b);
    let prod = a.mul(b, irr);
    if sum.is_zero() {
        // linear: prod·T + prod² + b = 0 (prod != 0 unless a or b is 0)
        if prod.is_zero() {
            return Vec::new();
        }
        let t = prod.mul(&prod, irr).add(b_curve);
        let t = t.mul(&prod.flt_inverse(irr).expect("prod nonzero"), irr);
        return vec![t];
    }
    let inv2 = sum.mul(&sum, irr).flt_inverse(irr).expect("sum nonzero");
    // T^2 + p T + q = 0 with p = prod/sum^2, q = (prod^2 + b)/sum^2
    let p = prod.mul(&inv2, irr);
    let q = prod.mul(&prod, irr).add(b_curve).mul(&inv2, irr);
    if p.is_zero() {
        // T^2 = q: unique root iff q is a square (q^(2^(n-1)) = 1)
        let mut sq = F2mElement::one(n);
        for _ in 0..(n - 1) {
            sq = sq.mul(&q, irr);
            sq = sq.mul(&sq, irr);
        }
        if sq.is_zero() {
            return vec![q];
        }
        return Vec::new();
    }
    let p2 = p.mul(&p, irr);
    let c = q.mul(&p2.flt_inverse(irr).expect("p nonzero"), irr);
    if c.trace(irr) != 0 {
        return Vec::new();
    }
    let h = c.half_trace(irr).expect("odd n");
    let mut out = Vec::new();
    for z in [F2mElement::zero(n), F2mElement::one(n)] {
        let t = p.mul(&h.add(&z), irr);
        if !out.iter().any(|e| *e == t) {
            out.push(t);
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ell: u32 = args.first().and_then(|a| a.parse().ok()).unwrap_or(8);
    let n_relations: usize = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(6);
    let seed: u64 = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(0x5437_0037);
    let trial_cap: u64 = args.get(3).and_then(|a| a.parse().ok()).unwrap_or(50_000_000);

    let kc = KoblitzCurve::new(0, 37).expect("a=0 n=37 Koblitz curve");
    let curve = &kc.curve;
    let n = kc.n;
    let irr = curve.irreducible.clone();
    let b_curve = curve.b.clone();
    let r = kc.subgroup_order.clone();
    println!("curve a=0 n={n} r={r} ({} bits)", r.bits());

    let parent = build_standard_subspace_factor_base(&kc, ell)
        .unwrap_or_else(|err| panic!("ell={ell}: {err}"));
    let base = cofactor_project_factor_base(&kc, &parent)
        .unwrap_or_else(|err| panic!("ell={ell} projection: {err}"));
    let pts: Vec<BinaryPoint> = base
        .points
        .iter()
        .filter(|p| matches!(p, BinaryPoint::Affine { .. }))
        .cloned()
        .collect();
    let b = pts.len();

    // pair table: x-key -> list of (i, j); and the pair sums as points
    let t0 = Instant::now();
    let mut entries: Vec<(u128, u32, u32)> = Vec::new();
    let mut sums: Vec<BinaryPoint> = Vec::new();
    for i in 0..b {
        for j in i..b {
            let s = point_add(curve, &pts[i], &pts[j]);
            if let Some(x) = point_x(&s) {
                entries.push((elem_bits(&x), i as u32, j as u32));
                sums.push(s);
            }
        }
    }
    entries.sort_unstable_by_key(|e| e.0);
    let tsize = entries.len();
    let region = BigUint::from(4u32) * &r;
    let e4 = (tsize as f64).powi(2) / (2.0 * 4.0 * r.to_string().parse::<f64>().unwrap());
    let e6 = (tsize as f64).powi(3) / (6.0 * 4.0 * r.to_string().parse::<f64>().unwrap());
    println!(
        "base={b} points, |T|={tsize} pair sums built in {:.1}s; E4(4-sums/target)={:.3e}, E6(6-sums/target)={:.3e}",
        t0.elapsed().as_secs_f32(), e4, e6
    );

    let mut rng = StdRng::seed_from_u64(seed);
    let generator = kc.generator().clone();
    let mut report: Vec<serde_json::Value> = Vec::new();

    for rel in 0..n_relations {
        let k = (BigUint::from(rng.gen::<u64>()) * &r) >> 64;
        let target = scalar_mul(curve, &generator, &k);
        let target = match target {
            BinaryPoint::Affine { .. } => target,
            _ => continue,
        };
        let target_x = point_x(&target).expect("affine target");

        // ── arm A: group join (control) ────────────────────────────────
        let ta = Instant::now();
        let mut trials_a = 0u64;
        let mut found_a = false;
        while trials_a < trial_cap {
            trials_a += 1;
            let e1 = entries[rng.gen_range(0..tsize)];
            let e2 = entries[rng.gen_range(0..tsize)];
            let p1 = point_add(curve, &pts[e1.1 as usize], &pts[e1.2 as usize]);
            let p2 = point_add(curve, &pts[e2.1 as usize], &pts[e2.2 as usize]);
            let mut res = point_add(curve, &target, &point_neg(&p1));
            res = point_add(curve, &res, &point_neg(&p2));
            let Some(xr) = point_x(&res) else { continue };
            let xb = elem_bits(&xr);
            if let Ok(pos) = entries.binary_search_by_key(&xb, |e| e.0) {
                let mut p = pos;
                while p < tsize && entries[p].0 == xb {
                    let (m, nn) = (entries[p].1 as usize, entries[p].2 as usize);
                    let s = point_add(curve, &point_add(curve, &p1, &p2), &point_add(curve, &pts[m], &pts[nn]));
                    if s == target {
                        found_a = true;
                        break;
                    }
                    p += 1;
                }
                if found_a {
                    break;
                }
            }
        }
        let wall_a = ta.elapsed().as_secs_f64();

        // ── arm B: S4-chain probe (x-level, two quadratics + lookups) ──
        let tb = Instant::now();
        let mut trials_b = 0u64;
        let mut probes_b = 0u64;
        let mut found_b = false;
        let xkeys: Vec<u128> = entries.iter().map(|e| e.0).collect();
        while trials_b < trial_cap {
            trials_b += 1;
            let xa = xkeys[rng.gen_range(0..tsize)];
            let xb2 = xkeys[rng.gen_range(0..tsize)];
            let ea = F2mElement::from_words(&[(xa & u128::from(u64::MAX)) as u64, (xa >> 64) as u64], n);
            let eb = F2mElement::from_words(&[(xb2 & u128::from(u64::MAX)) as u64, (xb2 >> 64) as u64], n);
            // S3(xa, xb, T) = 0: intermediate abscissae
            for t_mid in s3_roots(&ea, &eb, &b_curve, &irr, n) {
                // S3(X, x_R, T) = 0: third pair's abscissa
                for x_c in s3_roots(&t_mid, &target_x, &b_curve, &irr, n) {
                    probes_b += 1;
                    let xc = elem_bits(&x_c);
                    if let Ok(pos) = entries.binary_search_by_key(&xc, |e| e.0) {
                        let mut p = pos;
                        while p < tsize && entries[p].0 == xc {
                            // verify in the group: pair(xa), pair(xb), pair(xc)
                            let e1 = entries[p];
                            // recover the first two pairs from their x-keys
                            // (any pair with that abscissa works for the
                            // group check; we search the entries again)
                            if let (Some(i1), Some(i2)) = (
                                entries.iter().position(|e| e.0 == xa),
                                entries.iter().position(|e| e.0 == xb2),
                            ) {
                                let (a1, a2) = (entries[i1].1 as usize, entries[i1].2 as usize);
                                let (c1, c2) = (entries[i2].1 as usize, entries[i2].2 as usize);
                                let (m1, m2) = (e1.1 as usize, e1.2 as usize);
                                let s = point_add(
                                    curve,
                                    &point_add(
                                        curve,
                                        &point_add(curve, &pts[a1], &pts[a2]),
                                        &point_add(curve, &pts[c1], &pts[c2]),
                                    ),
                                    &point_add(curve, &pts[m1], &pts[m2]),
                                );
                                if s == target {
                                    found_b = true;
                                }
                            }
                            p += 1;
                            if found_b {
                                break;
                            }
                        }
                    }
                    if found_b {
                        break;
                    }
                }
                if found_b {
                    break;
                }
            }
            if found_b {
                break;
            }
        }
        let wall_b = tb.elapsed().as_secs_f64();

        let row = serde_json::json!({
            "relation_index": rel,
            "group_join": {
                "trials": trials_a, "wall_s": wall_a,
                "trials_per_s": trials_a as f64 / wall_a.max(1e-9),
                "found": found_a,
            },
            "s4_chain": {
                "trials": trials_b, "lookups": probes_b, "wall_s": wall_b,
                "trials_per_s": trials_b as f64 / wall_b.max(1e-9),
                "found": found_b,
                "us_per_trial": wall_b * 1e6 / trials_b.max(1) as f64,
            },
        });
        println!("{row}");
        report.push(row);
    }

    if let Some(pos) = std::env::args().position(|a| a == "--json") {
        if let Some(path) = std::env::args().nth(pos + 1) {
            std::fs::write(
                &path,
                serde_json::to_string_pretty(&serde_json::json!({
                    "schema": "koblitz-sixsum-s4join-n37-v1",
                    "ell": ell, "base_points": b, "pair_table": tsize,
                    "e4_4sums_per_target": e4, "e6_6sums_per_target": e6,
                    "region_4r": region.to_string(),
                    "rows": report,
                }))
                .unwrap(),
            )
            .unwrap();
            println!("wrote {path}");
        }
    }
}
