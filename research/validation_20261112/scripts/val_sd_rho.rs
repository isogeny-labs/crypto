// VALIDATOR TOOL (not producer code): replays the matched-rho arm of small_de_ic
// exactly as src/bin/small_de_ic.rs run_cell + small_de::rho_cost call it (same
// instance, target stream and seeds) and prints the recovered logarithm of every
// rho run, which small_de.json does not record, so that it can be certified
// independently ([k]G == T in the validator's Python).  It also prints the cost
// fields so they can be compared with small_de.json.
use crypto_lib::cryptanalysis::chained_transfer::small_de::construct;
use crypto_lib::cryptanalysis::ic_boundary::{
    rho_walk_with, CountedGroup, CubicClasses, GroupOps, NegationClasses, PointClasses, QuarticClasses, RhoWalk,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

fn splitmix(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn main() {
    let seed0: u64 = 20261112;
    for &bits in &[12u32, 16, 20, 24, 28] {
        for &d in &[-3i64, -4, -7, -8, -11, 0] {
            let Some(inst) = construct(d, bits, seed0) else { continue };
            for &m in &[2usize, 3] {
                let seed = splitmix(seed0 ^ ((bits as u64) << 32) ^ ((m as u64) << 48) ^ (inst.d as u64));
                let mut tr = StdRng::seed_from_u64(inst.curve.p ^ 0x5EED_7A26);
                let mut ops = GroupOps::default();
                let tps: Vec<_> = (0..16)
                    .map(|_| {
                        let t = tr.gen_range(1..inst.r);
                        (t, inst.curve.mul(&mut ops, inst.g, t))
                    })
                    .collect();
                let mut kinds = vec!["plain", "negation"];
                if d == -3 || d == -4 {
                    kinds.push("aut");
                }
                for kind in kinds {
                    for (k, (t, tp)) in tps.iter().enumerate() {
                        let c = &inst.curve;
                        let (g, r) = (inst.g, inst.r);
                        let max = 400 * r;
                        let s = splitmix(seed ^ 0xB0 ^ (k as u64));
                        let mut setup = GroupOps::default();
                        let res = match kind {
                            "plain" => rho_walk_with(c, &PointClasses, g, *tp, r, s, max, RhoWalk::plain()),
                            "negation" => rho_walk_with(c, &NegationClasses { r }, g, *tp, r, s, max, RhoWalk::negation()),
                            _ => {
                                if d == -3 {
                                    let cl = CubicClasses::new(c, g, r, &mut setup).unwrap();
                                    rho_walk_with(c, &cl, g, *tp, r, s, max, RhoWalk::negation())
                                } else {
                                    let cl = QuarticClasses::new(c, g, r, &mut setup).unwrap();
                                    rho_walk_with(c, &cl, g, *tp, r, s, max, RhoWalk::negation())
                                }
                            }
                        };
                        let native = res.counters.get("canonicalisations_uncharged").copied().unwrap_or(0);
                        println!(
                            "{}",
                            json!({"bits": bits, "m": m, "d": d, "kind": kind, "k": k, "t": t,
                                   "tx": tp.x, "ty": tp.y, "tinf": tp.infinity,
                                   "recovered": res.recovered, "gae": res.gae + setup.gae(), "native": native,
                                   "counters": res.counters})
                        );
                    }
                }
            }
        }
    }
}
