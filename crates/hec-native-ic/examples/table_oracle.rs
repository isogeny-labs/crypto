#[path = "../src/table_factor.rs"]
mod table_factor;
use hec_native_ic::{
    instance::{factor_base, pick_instance},
    oracle::{split, split_reference, OracleStats, Roots},
    poly::Poly,
    rng::Rng,
};
use std::{hint::black_box, time::Instant};
fn checksum(r: Option<Roots>) -> u64 {
    r.map_or(0, |r| {
        r.as_slice()
            .iter()
            .fold(1, |a, &(x, m)| a.wrapping_add(x.wrapping_mul(m as u64 + 1)))
    })
}
fn main() {
    let repeats: u64 = std::env::var("HEC_REPEATS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    for (g, p) in [(2, 503), (2, 4001), (3, 61), (3, 401), (4, 61), (4, 127)] {
        let i = pick_instance(p, g).unwrap();
        let fb = factor_base(&i.curve);
        let k = i.ell / 3 + 7;
        let target = i.curve.mul(&i.d1, k);
        let factorial = (1..=g).product::<usize>();
        let count = fb.len() * factorial;
        for rep in 0..repeats {
            // Shared fixture: real reduced walk divisors. Its generation is excluded
            // from every oracle arm, so this is explicitly a component experiment.
            let mut rng = Rng::new(20261114 + rep);
            let mut r = i.curve.add(&i.d1, &target);
            let mut polys: Vec<Poly> = Vec::with_capacity(count);
            for _ in 0..count {
                r = match rng.below(4) {
                    0 => i.curve.add(&r, &i.d1),
                    1 => i.curve.add(&r, &target),
                    _ => {
                        let (x, y) = fb[rng.below(fb.len() as u64) as usize];
                        i.curve.add_place(&r, x, y)
                    }
                };
                polys.push(r.u);
            }
            let clock = Instant::now();
            let tables = table_factor::Tables::new(p);
            let setup = clock.elapsed().as_nanos();
            for u in &polys {
                assert_eq!(
                    tables.split(u),
                    split_reference(u, &i.curve.fp, &mut OracleStats::default()),
                    "production walk oracle gate"
                );
            }
            let mut sums = Vec::new();
            let order = if rep % 2 == 0 {
                ["reference", "direct", "table"]
            } else {
                ["table", "direct", "reference"]
            };
            for arm in order {
                let now = Instant::now();
                let mut sum = 0u64;
                for u in &polys {
                    let roots = match arm {
                        "reference" => {
                            split_reference(black_box(u), &i.curve.fp, &mut OracleStats::default())
                        }
                        "direct" => split(black_box(u), &i.curve.fp, &mut OracleStats::default()),
                        _ => tables.split(black_box(u)),
                    };
                    sum = sum.wrapping_add(checksum(black_box(roots)));
                }
                let scan = now.elapsed().as_nanos();
                sums.push(sum);
                println!("JSON {{\"g\":{g},\"p\":{p},\"repeat\":{rep},\"arm\":\"{arm}\",\"n\":{count},\"scan_ns\":{scan},\"setup_ns\":{},\"cold_ns\":{},\"bytes\":{},\"checksum\":{sum},\"verified\":true}}",if arm=="table"{setup}else{0},scan+if arm=="table"{setup}else{0},if arm=="table"{tables.bytes()}else{0});
            }
            assert!(sums.iter().all(|s| s == &sums[0]));
        }
    }
}
