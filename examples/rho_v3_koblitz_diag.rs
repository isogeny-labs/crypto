//! Diagnostic: counted operations of the signed-Frobenius tuned walk
//! (rho.signed_frobenius_v2's walk) against signed_frobenius_rho_v3 on
//! Koblitz curves.  Local development check only.
//! Usage: rho_v3_koblitz_diag RUNS [K] [WEIGHT]   (KOBLITZ="a:n,a:n")
use crypto_lib::cryptanalysis::ic_boundary::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let runs: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(64);
    let k: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let weight: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(3);
    let dp: Option<u32> = args.get(4).and_then(|s| s.parse().ok());
    let curves = env::var("KOBLITZ").unwrap_or_else(|_| "1:29,0:37,1:43,0:47".into());
    for c in curves.split(',') {
        let (a, n) = c.split_once(':').unwrap();
        let Some(inst) = koblitz_instance(a.parse().unwrap(), n.parse().unwrap()) else {
            println!("a:n {c}: no Koblitz instance");
            continue;
        };
        let g = BinaryGroup(&inst.fast);
        let r = inst.r;
        let sq = (r as f64).sqrt();
        let mut tot = [[0.0f64; 4]; 2];
        let mut aut = 0;
        for i in 0..runs {
            let d = 1 + (i.wrapping_mul(0x9e3779b97f4a7c15) ^ 0x1234567) % (r - 1);
            let mut o = GroupOps::default();
            let q = g.mul(&mut o, inst.generator, d);
            let cap = 64 * sq as u64 + 4096;
            let old = signed_frobenius_rho_tuned_policy(&inst, q, 1000 + i, cap, WalkPolicy::v2())
                .unwrap();
            let mut v = RhoV3::frobenius();
            v.jumps = k;
            v.weight = weight;
            v.dp_bits = dp;
            v.g_only = env::var("GONLY").is_ok();
            let new = signed_frobenius_rho_v3(&inst, q, 1000 + i, cap, v).unwrap();
            aut = new.automorphisms;
            for (j, res) in [&old, &new].iter().enumerate() {
                if res.recovered != Some(d % r) {
                    eprintln!(
                        "UNSOLVED arm {j} run {i} seed {} d {d} steps {} walks {}",
                        1000 + i,
                        res.steps,
                        res.walks
                    );
                    continue;
                }
                let su = (res.counters.get("setup_additions").copied().unwrap_or(0)
                    + res.counters.get("setup_doublings").copied().unwrap_or(0))
                    as f64;
                let w = res.counters.get("walk_operations").copied().unwrap_or(0) as f64;
                tot[j][0] += res.s;
                tot[j][1] += su / sq;
                tot[j][2] += w / sq;
                tot[j][3] += res.counters.get("jumps").copied().unwrap_or(0) as f64;
            }
        }
        let n = runs as f64;
        let floor = (std::f64::consts::PI / (2.0 * aut as f64)).sqrt();
        for (j, name) in ["v2 ", "v3 "].iter().enumerate() {
            println!(
                "a:n {c} log2r {:.1} A {aut} {name} k {:.0} S {:.3} ({:.3}x floor) setup {:.3} walk {:.3} ({:.3}x floor)",
                (r as f64).log2(),
                tot[j][3] / n,
                tot[j][0] / n,
                tot[j][0] / n / floor,
                tot[j][1] / n,
                tot[j][2] / n,
                tot[j][2] / n / floor
            );
        }
    }
}
