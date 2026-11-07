//! Diagnostic: counted operations of the legacy tuned rho walks against
//! rho_walk_v3 on the prime toy ladder.  Local development check only; the
//! measurement of record is an ecbench session.
use crypto_lib::cryptanalysis::ic_boundary::*;
use std::env;
const PARTS: [&str; 6] = [
    "steps",
    "look_ahead_additions",
    "cycle_escape_doublings",
    "start_additions",
    "verification_operations",
    "distinguished_points",
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let runs: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(64);
    let k_override: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    let weight: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(3);
    let cont: bool = args.get(4).is_none_or(|s| s != "0");
    let dp: Option<u32> = args.get(5).and_then(|s| s.parse().ok());
    let ladder: u32 = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(0);
    let sizes: Vec<u32> = env::var("BITS")
        .ok()
        .map(|v| v.split(',').filter_map(|x| x.parse().ok()).collect())
        .unwrap_or_else(|| vec![18, 20, 22, 24, 26, 28]);
    for bits in sizes {
        let inst = find_prime_order_curve(bits, 59297);
        let g = inst.curve.clone();
        let gen = inst.generator_point();
        let r = inst.r;
        let neg = NegationClasses { r };
        let floor = (std::f64::consts::PI / 4.0).sqrt();
        let (mut s_old, mut s_new, mut set_old, mut set_new) = (0.0, 0.0, 0.0, 0.0);
        let (mut ok_old, mut ok_new) = (0, 0);
        let mut kk = 0;
        let mut s2_new = 0.0;
        let (mut t_old, mut t_new) = (0.0f64, 0.0f64);
        let mut parts_old = [0.0f64; 6];
        let mut parts_new = [0.0f64; 6];
        for i in 0..runs {
            let d = 1 + (i.wrapping_mul(0x9e3779b97f4a7c15) ^ 0x1234567) % (r - 1);
            let mut o = GroupOps::default();
            let q = g.mul(&mut o, gen, d);
            let seed = 1000 + i;
            let t0 = std::time::Instant::now();
            let a = rho_walk_with(
                &g,
                &neg,
                gen,
                q,
                r,
                seed,
                64 * (r as f64).sqrt() as u64 + 4096,
                RhoWalk::negation(),
            );
            t_old += t0.elapsed().as_nanos() as f64 / a.gae;
            let mut v = RhoV3::negation();
            v.jumps = k_override;
            v.weight = weight;
            v.continue_walks = cont;
            v.dp_bits = dp;
            v.ladder_bits = ladder;
            v.dp_from_floor = env::var("DPF").is_ok();
            v.ladder_verify = env::var("LV").is_ok();
            let t1 = std::time::Instant::now();
            let b = rho_walk_v3(
                &g,
                &neg,
                gen,
                q,
                r,
                seed,
                64 * (r as f64).sqrt() as u64 + 4096,
                v,
            );
            t_new += t1.elapsed().as_nanos() as f64 / b.gae;
            ok_old += (a.recovered == Some(d)) as u32;
            ok_new += (b.recovered == Some(d)) as u32;
            let sq = (r as f64).sqrt();
            s_old += a.s;
            s_new += b.s;
            s2_new += b.s * b.s;
            let su = |res: &RhoResult| {
                (res.counters.get("setup_additions").copied().unwrap_or(0)
                    + res.counters.get("setup_doublings").copied().unwrap_or(0))
                    as f64
                    / sq
            };
            set_old += su(&a);
            set_new += su(&b);
            kk = b.counters.get("jumps").copied().unwrap_or(0);
            for (res, acc) in [(&a, &mut parts_old), (&b, &mut parts_new)] {
                for (j, key) in PARTS.iter().enumerate() {
                    acc[j] += res.counters.get(*key).copied().unwrap_or(0) as f64 / sq;
                }
            }
        }
        let n = runs as f64;
        let sd = (s2_new / n - (s_new / n).powi(2)).sqrt() / n.sqrt();
        println!(
            "   v3 S standard error {sd:.4}; ns/GAE legacy {:.1} v3 {:.1}",
            t_old / n,
            t_new / n
        );
        println!(
            "bits {bits} log2r {:.1} | legacy S {:.3} ({:.3}x floor, setup {:.3}) ok {ok_old} | v3 k={kk} S {:.3} ({:.3}x floor, setup {:.3}) ok {ok_new}",
            (r as f64).log2(),
            s_old / n,
            s_old / n / floor,
            set_old / n,
            s_new / n,
            s_new / n / floor,
            set_new / n
        );
        let f = |p: &[f64; 6]| {
            p.iter()
                .map(|x| format!("{:.4}", x / n))
                .collect::<Vec<_>>()
                .join(" ")
        };
        println!(
            "   parts [steps lookahead escapes starts verify dps]/sqrt(r): legacy {} | v3 {}",
            f(&parts_old),
            f(&parts_new)
        );
    }
}
