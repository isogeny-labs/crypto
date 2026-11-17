//! The curve-free null of `research/ic_prime_residual_lp_20261026`
//! (H-PFDR-cb9606 part E).
//!
//! It runs the residual collector (`ic_framework::residual`, the large-prime
//! limit `|D| = r`) on two carriers, with the same seeds:
//! - a registered prime curve with its smallest-abscissa base of `B` points;
//! - `Z/r` with a random-bijection key and `B` random base elements.
//!
//! The collector sees only `add`, `neg` and `key`, so a gain that is the
//! birthday bound must survive the swap unchanged. Prints one JSON object:
//! the group operations per solve of every run on both carriers.
//!
//!     cargo run --release --example ic_residual_curve_free_null -- <curve-spec-json> <B> <runs> <seed>

use crypto_lib::cryptanalysis::ecbench::workload::{CurveSpec, Instance};
use crypto_lib::cryptanalysis::ic_boundary::{
    prime_factor_base, CountedGroup, FactorBase, GroupOps, IncrementalGauss, PipelineOutcome,
};
use crypto_lib::cryptanalysis::ic_framework::residual::collect_residual_and_solve;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// `Z/r`, keyed by a fixed random bijection of `u64` so that no key order
/// mirrors the group order.
struct Shuffled {
    r: u64,
    k1: u64,
    k2: u64,
}

impl Shuffled {
    fn mix(&self, mut z: u64) -> u64 {
        // A bijection of u64: xor-shift and odd multiplications are invertible.
        z ^= self.k1;
        z = z.wrapping_mul(0x9e37_79b9_7f4a_7c15 | 1);
        z ^= z >> 29;
        z = z.wrapping_mul(self.k2 | 1);
        z ^ (z >> 32)
    }
}

impl CountedGroup for Shuffled {
    type Elt = u64;
    fn identity(&self) -> u64 {
        0
    }
    fn is_identity(&self, p: &u64) -> bool {
        *p == 0
    }
    fn add(&self, ops: &mut GroupOps, p: u64, q: u64) -> u64 {
        ops.adds += 1;
        ((p as u128 + q as u128) % self.r as u128) as u64
    }
    fn double(&self, ops: &mut GroupOps, p: u64) -> u64 {
        ops.doubles += 1;
        ((2 * p as u128) % self.r as u128) as u64
    }
    fn neg(&self, p: u64) -> u64 {
        (self.r - p) % self.r
    }
    fn key(&self, p: &u64) -> u64 {
        self.mix(*p)
    }
}

/// `B` random elements and their negatives, one column each, as
/// `prime_factor_base` lays out a prime base.
fn random_base(g: &Shuffled, size: usize, rng: &mut StdRng) -> FactorBase<u64> {
    let mut fb = FactorBase::empty(format!("{size} random elements of Z/r"));
    let mut seen = std::collections::HashSet::new();
    while fb.abscissae < size {
        let e = rng.gen_range(1..g.r);
        if !seen.insert(e.min(g.r - e)) {
            continue;
        }
        let col = fb.abscissae;
        let i = fb.points.len();
        fb.points.extend([e, g.neg(e)]);
        fb.col_of.extend([col, col]);
        fb.coef_of.extend([1, g.r - 1]);
        fb.neg_index.extend([i + 1, i]);
        fb.x_index.insert(e.min(g.r - e), vec![i, i + 1]);
        fb.abscissae += 1;
    }
    fb.columns = fb.abscissae;
    for (i, p) in fb.points.iter().enumerate() {
        fb.point_index.insert(g.key(p), i);
    }
    fb.abscissa_list = fb.x_index.keys().copied().collect();
    fb.abscissa_list.sort_unstable();
    fb
}

fn ops_of(o: &PipelineOutcome) -> u64 {
    [&o.relations, &o.linear_algebra, &o.verify]
        .iter()
        .map(|p| p.group_ops.adds + p.group_ops.doubles)
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let spec: CurveSpec = serde_json::from_str(&args[1]).expect("curve spec JSON");
    let size: usize = args[2].parse().expect("B");
    let runs: u64 = args[3].parse().expect("runs");
    let seed: u64 = args[4].parse().expect("seed");
    let Instance::Prime(inst) = spec.build().expect("curve") else {
        panic!("a prime curve spec is required")
    };
    let (curve, r, h) = (&inst.curve, inst.r, inst.cofactor);
    let g0 = inst.generator_point();
    let fb = prime_factor_base(&inst, size);
    let mut rng = StdRng::seed_from_u64(seed);
    let null = Shuffled {
        r,
        k1: rng.gen(),
        k2: rng.gen(),
    };
    let null_fb = random_base(&null, size, &mut rng);
    let (mut curve_ops, mut null_ops, mut ok) = (Vec::new(), Vec::new(), (0u64, 0u64));
    for k in 0..runs {
        let d = rng.gen_range(1..r);
        let run_seed = seed.wrapping_add(k);
        let mut setup = GroupOps::default();
        let target = curve.mul(&mut setup, g0, d);
        let mut m = IncrementalGauss::new(fb.columns + 1, r);
        let o = collect_residual_and_solve(
            curve,
            g0,
            target,
            r,
            h,
            &fb,
            run_seed,
            u64::MAX / 4,
            &mut m,
        );
        ok.0 += (o.verified && o.recovered == Some(d)) as u64;
        curve_ops.push(ops_of(&o));
        let mut m = IncrementalGauss::new(null_fb.columns + 1, r);
        let o =
            collect_residual_and_solve(&null, 1, d, r, 1, &null_fb, run_seed, u64::MAX / 4, &mut m);
        ok.1 += (o.verified && o.recovered == Some(d)) as u64;
        null_ops.push(ops_of(&o));
    }
    println!(
        "{}",
        serde_json::json!({
            "schema": "ic_prime_residual_lp.null/v1",
            "curve_spec": spec, "r": r, "base": size, "runs": runs, "seed": seed,
            "curve": {"verified": ok.0, "group_ops": curve_ops},
            "curve_free": {"verified": ok.1, "group_ops": null_ops},
        })
    );
}
