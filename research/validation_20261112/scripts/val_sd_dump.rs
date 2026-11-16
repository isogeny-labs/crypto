// VALIDATOR TOOL (not producer code): dumps small_de instances from the committed
// construct() and the run's target stream (StdRng(p ^ 0x5EED_7A26), t in 1..r).
use crypto_lib::cryptanalysis::chained_transfer::small_de::construct;
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

fn main() {
    let sizes = [12u32, 16, 20, 24, 28];
    let ds = [-3i64, -4, -7, -8, -11, 0];
    for &bits in &sizes {
        for &d in &ds {
            let Some(inst) = construct(d, bits, 20261112) else {
                println!("{}", json!({"bits": bits, "d": d, "none": true}));
                continue;
            };
            let mut tr = StdRng::seed_from_u64(inst.curve.p ^ 0x5EED_7A26);
            let mut ops = GroupOps::default();
            let mut ts = vec![];
            for _ in 0..16 {
                let t = tr.gen_range(1..inst.r);
                let tp = inst.curve.mul(&mut ops, inst.g, t);
                ts.push(json!({"t": t, "x": tp.x, "y": tp.y, "inf": tp.infinity}));
            }
            println!(
                "{}",
                json!({"bits": bits, "d": d, "p": inst.curve.p, "a": inst.curve.a, "b": inst.curve.b,
                       "n": inst.n, "h": inst.h, "r": inst.r,
                       "g": [inst.g.x, inst.g.y, inst.g.infinity], "targets": ts})
            );
        }
    }
}
