//! **Six-summand pair-table harvest on the n=83 gate curve — the row-parity
//! lever of `RESEARCH_ECC2K130_IC_FEASIBILITY.md` §4 (m=5/6), measured.**
//!
//! The feasibility note's unmeasured lever: higher-arity relations.  This
//! driver measures the cheapest harvest shape that exists today — a
//! randomized join over the pair-sum table of a standard-subspace factor
//! base: sample two random base pairs, subtract their four points from the
//! target, and look the residual's abscissa up in the pair table.  A hit
//! is a 6-summand decomposition; every hit is verified in the group.
//!
//! Model: with `|T|` distinct pair-sum abscissae and a target region of
//! size `4r`, the random-trial cost per verified relation is `≈ 4r/|T|`
//! trials (one subtraction chain + one table probe each). The guided
//! S4-tree extraction of the note would replace that with
//! `≈ 45·r/(n⁴K⁴)`; this driver measures the random-join constant.
//!
//! Known-answer only: targets are fresh random subgroup points.
//!
//! ```bash
//! cargo run --release --example koblitz_sixsum_harvest_n83 -- 14 4
//! ```

use crypto_lib::binary_ecc::curve::{point_add, point_neg, scalar_mul, BinaryPoint};
use crypto_lib::cryptanalysis::f6_wide_geometry::batch_add_fixed;
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_standard_subspace_factor_base, cofactor_project_factor_base, KoblitzCurve,
};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Instant;

fn point_x_bits(p: &BinaryPoint) -> Option<u128> {
    match p {
        BinaryPoint::Affine { x, .. } => {
            let raw = x.raw_bits();
            let mut b = raw[0] as u128;
            if raw.len() > 1 {
                b |= (raw[1] as u128) << 64;
            }
            Some(b)
        }
        BinaryPoint::Infinity => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ell: u32 = args.first().and_then(|a| a.parse().ok()).unwrap_or(14);
    let n_relations: usize = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(4);
    let seed: u64 = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(0x5160_0083);

    let kc = KoblitzCurve::known_n83_k0().expect("N83 k0 fixture");
    let curve = &kc.curve;
    let r = kc.subgroup_order.clone();

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
    let npairs = b * (b - 1) / 2 + b;
    println!(
        "curve n=83 r={r} ell={ell} usable_points={b} signed_columns={} unordered_pairs={npairs}",
        base.signed_orbits.len()
    );

    let t0 = Instant::now();
    let mut table: Vec<(u128, u32, u32)> = Vec::with_capacity(npairs);
    for i in 0..b {
        // one field inversion per row (batch_add_fixed), not per pair
        let row = batch_add_fixed(curve, &pts[i], &pts[i..]);
        for (off, s) in row.into_iter().enumerate() {
            if let Some(x) = point_x_bits(&s) {
                table.push((x, i as u32, (i + off) as u32));
            }
        }
    }
    table.sort_unstable_by_key(|e| e.0);
    table.dedup_by_key(|e| e.0);
    let tsize = table.len();
    println!(
        "pair table: {tsize} distinct abscissae built in {:.1}s ({:.0} MiB)",
        t0.elapsed().as_secs_f32(),
        table.len() as f64 * 20.0 / 1048576.0
    );

    let group_region = BigUint::from(4u32) * &r;
    let trials_model = (&group_region * BigUint::from(6u32)) / BigUint::from(tsize as u64);
    println!("model: trials per relation ~{trials_model}");

    let mut rng = StdRng::seed_from_u64(seed);
    let generator = kc.generator().clone();
    let mut rows: Vec<serde_json::Value> = Vec::new();
    for rel in 0..n_relations {
        let k = (BigUint::from(rng.gen::<u64>()) * &r) >> 64;
        let target = scalar_mul(curve, &generator, &k);
        let target = match target {
            BinaryPoint::Affine { .. } => target,
            _ => continue,
        };

        let t1 = Instant::now();
        let mut trials: u64 = 0;
        let mut found: Option<(usize, usize, usize, usize, usize, usize)> = None;
        while trials < 400_000_000 {
            trials += 1;
            let i = rng.gen_range(0..b);
            let j = rng.gen_range(0..b);
            let kk = rng.gen_range(0..b);
            let l = rng.gen_range(0..b);
            let mut res = point_add(curve, &target, &point_neg(&pts[i]));
            res = point_add(curve, &res, &point_neg(&pts[j]));
            res = point_add(curve, &res, &point_neg(&pts[kk]));
            res = point_add(curve, &res, &point_neg(&pts[l]));
            let Some(xr) = point_x_bits(&res) else { continue };
            if let Ok(pos) = table.binary_search_by_key(&xr, |e| e.0) {
                let mut p = pos;
                while p < table.len() && table[p].0 == xr {
                    let (m, n) = (table[p].1 as usize, table[p].2 as usize);
                    let s = point_add(
                        curve,
                        &point_add(
                            curve,
                            &point_add(curve, &pts[i], &pts[j]),
                            &point_add(curve, &pts[kk], &pts[l]),
                        ),
                        &point_add(curve, &pts[m], &pts[n]),
                    );
                    if s == target {
                        found = Some((i, j, kk, l, m, n));
                        break;
                    }
                    p += 1;
                }
                if found.is_some() {
                    break;
                }
            }
        }
        let wall = t1.elapsed().as_secs_f64();
        let row = serde_json::json!({
            "relation_index": rel,
            "trials": trials,
            "wall_s": wall,
            "trials_per_s": trials as f64 / wall,
            "found": found.map(|w: (usize, usize, usize, usize, usize, usize)| format!("{:?}", w)),
        });
        println!("{row}");
        rows.push(row);
        if trials >= 400_000_000 {
            println!("relation {rel}: trial cap hit without a verified 6-sum");
        }
    }

    if let Some(pos) = std::env::args().position(|a| a == "--json") {
        if let Some(path) = std::env::args().nth(pos + 1) {
            std::fs::write(
                &path,
                serde_json::to_string_pretty(&serde_json::json!({
                    "schema": "koblitz-sixsum-harvest-n83-v1",
                    "curve": "E0 over F_2^83, r=2417851639230796216685689",
                    "ell": ell,
                    "base_points": b,
                    "pair_table": tsize,
                    "trials_model": trials_model.to_string(),
                    "rows": rows,
                }))
                .unwrap(),
            )
            .unwrap();
            println!("wrote {path}");
        }
    }
}
