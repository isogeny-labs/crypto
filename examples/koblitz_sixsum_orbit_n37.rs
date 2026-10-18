//! **Steps 1–2 of the row-parity ledger: Frobenius-fold and orbit-state
//! concentration, measured on a compact-orbit factor base at n=37.**
//!
//! The ladder's base is a union of Frobenius orbits (B ≈ 2nK points from K
//! orbit seeds), not a subspace.  This driver builds that base shape and
//! measures, with matched arms on the same curve/targets:
//!
//! - **4-sum join** and **6-sum S₄-chain** per-relation cost at the orbit
//!   base, against the plain-join law `4r/|T|` — does the orbit structure
//!   beat the join, or only reorganize it?
//! - **Fold**: for each found 6-sum, count how many Frobenius images
//!   `π^k(R)` are free relations (the ladder's orbit amortization), and the
//!   orbit length of the base under `π` (signed).
//!
//! Known-answer only: targets are fresh random subgroup points.
//!
//! ```bash
//! cargo run --release --example koblitz_sixsum_orbit_n37 -- 4 4
//! ```

use crypto_lib::binary_ecc::curve::{point_add, point_neg, scalar_mul, BinaryPoint};
use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
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

fn frobenius(p: &BinaryPoint, irr: &IrreduciblePoly) -> BinaryPoint {
    match p {
        BinaryPoint::Affine { x, y } => BinaryPoint::Affine {
            x: x.square(irr),
            y: y.square(irr),
        },
        BinaryPoint::Infinity => BinaryPoint::Infinity,
    }
}

/// Roots of `S₃(a, b, T) = (a+b)²T² + ab·T + (ab)² + b_curve = 0`.
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
        if prod.is_zero() {
            return Vec::new();
        }
        let t = prod.mul(&prod, irr).add(b_curve);
        let t = t.mul(&prod.flt_inverse(irr).expect("prod nonzero"), irr);
        return vec![t];
    }
    let inv2 = sum.mul(&sum, irr).flt_inverse(irr).expect("sum nonzero");
    let p = prod.mul(&inv2, irr);
    let q = prod.mul(&prod, irr).add(b_curve).mul(&inv2, irr);
    if p.is_zero() {
        // T^2 = q
        let mut sq = q.clone();
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
    let k_orbits: usize = args.first().and_then(|a| a.parse().ok()).unwrap_or(4);
    let n_relations: usize = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(4);
    let seed: u64 = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(0x0AB1_7037);

    let kc = KoblitzCurve::new(0, 37).expect("a=0 n=37 Koblitz curve");
    let curve = &kc.curve;
    let n = kc.n;
    let irr = curve.irreducible.clone();
    let b_curve = curve.b.clone();
    let r = kc.subgroup_order.clone();
    println!("curve a=0 n={n} r={r} ({} bits), K={k_orbits} orbit seeds", r.bits());

    // compact-orbit base: close K random subgroup points under Frobenius
    // (n phases) and negation; plus the sign fold gives ~2nK points.
    let mut rng = StdRng::seed_from_u64(seed);
    let generator = kc.generator().clone();
    let mut base_pts: Vec<BinaryPoint> = Vec::new();
    for _ in 0..k_orbits {
        let k = (BigUint::from(rng.gen::<u64>()) * &r) >> 64;
        let p = scalar_mul(curve, &generator, &k);
        if matches!(p, BinaryPoint::Affine { .. }) {
            // full Frobenius orbit with both signs
            let mut q = p.clone();
            for _ in 0..n {
                if let BinaryPoint::Affine { x, .. } = &q {
                    if !base_pts
                        .iter()
                        .any(|e| point_x(e) == Some(x.clone()))
                    {
                        base_pts.push(q.clone());
                        base_pts.push(point_neg(&q));
                    }
                }
                q = frobenius(&q, &irr);
            }
        }
    }
    let b = base_pts.len();
    println!("orbit base: {b} points from {k_orbits} seeds (2nK = {})", 2 * n as usize * k_orbits);

    // pair table
    let t0 = Instant::now();
    let mut entries: Vec<(u128, u32, u32)> = Vec::new();
    let mut sums: Vec<BinaryPoint> = Vec::new();
    for i in 0..b {
        for j in i..b {
            let s = point_add(curve, &base_pts[i], &base_pts[j]);
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
        "|T|={tsize} pair sums ({:.1}s); E4={:.3e}, E6={:.3e}; join model 4r/|T| = {:.3e}",
        t0.elapsed().as_secs_f32(), e4, e6,
        4.0 * r.to_string().parse::<f64>().unwrap() / tsize as f64
    );

    let mut report: Vec<serde_json::Value> = Vec::new();
    for rel in 0..n_relations {
        let k = (BigUint::from(rng.gen::<u64>()) * &r) >> 64;
        let target = scalar_mul(curve, &generator, &k);
        let target = match target {
            BinaryPoint::Affine { .. } => target,
            _ => continue,
        };
        let target_x = point_x(&target).expect("affine target");

        // ── arm 0: planted 6-sum (diagnostic) ──────────────────────────
        {
            let i1 = rng.gen_range(0..tsize);
            let i2 = rng.gen_range(0..tsize);
            let i3 = rng.gen_range(0..tsize);
            let planted_target = point_add(curve, &point_add(curve, &sums[i1], &sums[i2]), &sums[i3]);
            let planted_target = match planted_target {
                BinaryPoint::Affine { .. } => planted_target,
                _ => BinaryPoint::Infinity,
            };
            if let BinaryPoint::Affine { x: ptx, .. } = &planted_target {
                let xa = elem_bits(&point_x(&sums[i1]).unwrap());
                let xb2 = elem_bits(&point_x(&sums[i2]).unwrap());
                let ea = F2mElement::from_words(&[(xa & u128::from(u64::MAX)) as u64, (xa >> 64) as u64], n);
                let eb = F2mElement::from_words(&[(xb2 & u128::from(u64::MAX)) as u64, (xb2 >> 64) as u64], n);
                let mut chain_hits = 0usize;
                let mut verified = false;
                for t_mid in s3_roots(&ea, &eb, &b_curve, &irr, n) {
                    for x_c in s3_roots(&t_mid, ptx, &b_curve, &irr, n) {
                        chain_hits += 1;
                        let xc = elem_bits(&x_c);
                        if entries.binary_search_by_key(&xc, |e| e.0).is_ok() {
                            verified = true;
                        }
                    }
                }
                println!("  planted: chain produced {chain_hits} candidate x-roots; a root in T: {verified}");
            }
        }

        // ── arm 0b: x-hit rate on a random target (pre-verification) ──
        {
            let mut xhits = 0usize;
            let ntr = 1_000_000u64;
            let xkeys0: Vec<u128> = entries.iter().map(|e| e.0).collect();
            for _ in 0..ntr {
                let xa = xkeys0[rng.gen_range(0..tsize)];
                let xb2 = xkeys0[rng.gen_range(0..tsize)];
                let ea = F2mElement::from_words(&[(xa & u128::from(u64::MAX)) as u64, (xa >> 64) as u64], n);
                let eb = F2mElement::from_words(&[(xb2 & u128::from(u64::MAX)) as u64, (xb2 >> 64) as u64], n);
                for t_mid in s3_roots(&ea, &eb, &b_curve, &irr, n) {
                    for x_c in s3_roots(&t_mid, &target_x, &b_curve, &irr, n) {
                        let xc = elem_bits(&x_c);
                        if entries.binary_search_by_key(&xc, |e| e.0).is_ok() {
                            xhits += 1;
                        }
                    }
                }
            }
            println!("  x-hit rate: {xhits} hits in {ntr} trials (model predicts ~{:.0})",
                6.0 * e6 * ntr as f64 / (tsize as f64).powi(2));
        }

        // ── arm 1: 4-sum join (t1, lookup) ──────────────────────────────
        let t1 = Instant::now();
        let mut trials4 = 0u64;
        let mut found4 = false;
        while trials4 < 30_000_000 {
            trials4 += 1;
            let idx = rng.gen_range(0..tsize);
            let res = point_add(curve, &target, &point_neg(&sums[idx]));
            let Some(xr) = point_x(&res) else { continue };
            let xb = elem_bits(&xr);
            if let Ok(pos) = entries.binary_search_by_key(&xb, |e| e.0) {
                let mut p = pos;
                while p < tsize && entries[p].0 == xb {
                    let (m, nn) = (entries[p].1 as usize, entries[p].2 as usize);
                    if point_add(curve, &sums[idx], &point_add(curve, &base_pts[m], &base_pts[nn])) == target {
                        found4 = true;
                        break;
                    }
                    p += 1;
                }
                if found4 {
                    break;
                }
            }
        }
        let wall4 = t1.elapsed().as_secs_f64();

        // ── arm 2: 6-sum S4-chain ────────────────────────────────────────
        let t2 = Instant::now();
        let mut trials6 = 0u64;
        let mut found6: Option<(usize, usize, usize)> = None;
        let xkeys: Vec<u128> = entries.iter().map(|e| e.0).collect();
        while trials6 < 30_000_000 {
            trials6 += 1;
            let xa = xkeys[rng.gen_range(0..tsize)];
            let xb2 = xkeys[rng.gen_range(0..tsize)];
            let ea = F2mElement::from_words(&[(xa & u128::from(u64::MAX)) as u64, (xa >> 64) as u64], n);
            let eb = F2mElement::from_words(&[(xb2 & u128::from(u64::MAX)) as u64, (xb2 >> 64) as u64], n);
            for t_mid in s3_roots(&ea, &eb, &b_curve, &irr, n) {
                for x_c in s3_roots(&t_mid, &target_x, &b_curve, &irr, n) {
                    let xc = elem_bits(&x_c);
                    if let Ok(pos) = entries.binary_search_by_key(&xc, |e| e.0) {
                        let mut p = pos;
                        while p < tsize && entries[p].0 == xc {
                            let hit = p;
                            if let (Some(i1), Some(i2)) = (
                                entries.iter().position(|e| e.0 == xa),
                                entries.iter().position(|e| e.0 == xb2),
                            ) {
                                // rebuild each pair sum from its stored base
                                // indices (entries are sorted; sums[] is not)
                                let mk = |e: &(u128, u32, u32)| {
                                    point_add(curve, &base_pts[e.1 as usize], &base_pts[e.2 as usize])
                                };
                                let s = point_add(
                                    curve,
                                    &point_add(curve, &mk(&entries[i1]), &mk(&entries[i2])),
                                    &mk(&entries[hit]),
                                );
                                if s == target {
                                    found6 = Some((i1, i2, hit));
                                }
                            }
                            p += 1;
                            if found6.is_some() {
                                break;
                            }
                        }
                    }
                    if found6.is_some() {
                        break;
                    }
                }
                if found6.is_some() {
                    break;
                }
            }
            if found6.is_some() {
                break;
            }
        }
        let wall6 = t2.elapsed().as_secs_f64();

        // ── arm 3: fold — free Frobenius images of the found 6-sum ──────
        let mut free_images = 0usize;
        let mut orbit_len = 0usize;
        if let Some((i1, i2, i3)) = found6 {
            let mut img = target.clone();
            for _ in 0..2 * n {
                img = frobenius(&img, &irr);
                if img == target {
                    orbit_len += 1;
                    break;
                }
                // π^k(R) decomposes with the image relation — check one
                let s = point_add(
                    curve,
                    &point_add(curve, &frobenius(&sums[i1], &irr), &frobenius(&sums[i2], &irr)),
                    &frobenius(&sums[i3], &irr),
                );
                if s == img {
                    free_images += 1;
                }
                orbit_len += 1;
            }
        }

        let row = serde_json::json!({
            "relation_index": rel,
            "four_sum_join": {"trials": trials4, "wall_s": wall4, "found": found4},
            "six_sum_s4chain": {"trials": trials6, "wall_s": wall6, "found": found6.is_some()},
            "fold": {"free_images_of_first_step": free_images, "target_orbit_len": orbit_len},
        });
        println!("{row}");
        report.push(row);
    }

    if let Some(pos) = std::env::args().position(|a| a == "--json") {
        if let Some(path) = std::env::args().nth(pos + 1) {
            std::fs::write(
                &path,
                serde_json::to_string_pretty(&serde_json::json!({
                    "schema": "koblitz-sixsum-orbit-n37-v1",
                    "k_orbits": k_orbits, "base_points": b, "pair_table": tsize,
                    "e4": e4, "e6": e6, "rows": report,
                }))
                .unwrap(),
            )
            .unwrap();
            println!("wrote {path}");
        }
    }
}
