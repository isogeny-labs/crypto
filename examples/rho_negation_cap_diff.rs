//! Capped-step `ic_wide::rho_negation` (u128) on the m = 17 claim curve (review 1, issue 6):
//! build at the base commit and at HEAD, run `rho_negation_cap_diff 4096 200000 3000000 100000000`,
//! and `cmp` the two stdout files.
use crypto_lib::cryptanalysis::gaudry_quartic_bin::{find_bin_curve, CurveB, PtB};
use crypto_lib::cryptanalysis::ic_wide::rho_negation;
use std::time::Instant;
fn h(s: &str) -> u128 {
    u128::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
}
fn main() {
    let caps: Vec<u64> = std::env::args()
        .skip(1)
        .map(|a| a.parse().unwrap())
        .collect();
    let t = Instant::now();
    let inst = find_bin_curve(17, 1, 2).unwrap();
    eprintln!("curve {:?} r={} ", t.elapsed(), inst.r);
    let f = &inst.curve.f;
    let targets = [
        (
            "0x8e1cddcd9dbc11e1e",
            "0x7da10ba85a823b63a",
            17619941755832365511u64,
        ),
        (
            "0x559a15baab4e756a5",
            "0x6fa6daf28979d7a1f",
            3778716907206883881u64,
        ),
    ];
    for (tx, ty, seed) in targets {
        for &cap in &caps {
            let target = PtB {
                x: f.unpack(h(tx)),
                y: f.unpack(h(ty)),
                inf: false,
            };
            let t = Instant::now();
            let (r, d) = rho_negation::<CurveB, u128>(
                &inst.curve,
                inst.generator,
                target,
                inst.r,
                seed,
                cap,
            );
            eprintln!("  wall {:?}", t.elapsed());
            println!("seed={seed} cap={cap} ops={:?} steps={} walks={} dps={} gae={:016x} s={:016x} swalk={:016x} exp={:016x} ratio={:016x} rec={:?} ver={} counters={:?} d={:?}",
                r.group_ops, r.steps, r.walks, r.distinguished_points, r.gae.to_bits(), r.s.to_bits(), r.s_walk.to_bits(), r.expected_steps.to_bits(), r.steps_over_expected.to_bits(), r.recovered, r.verified, r.counters, d);
        }
    }
}
