//! EXPLORATORY red-team pilot (not a frozen protocol, not evidence): does the
//! factor-base line search transfer to the repository's frontier route on the
//! same weak class, the JV12 §3.2 sieving variant (`jv_sieve::run_cover_sieve_dlp_on`)?
//! Same options as research/jv_sieve_strong_rho_v2_20261020 (`--exp sieve --stop 64
//! --rho-runs 0`, JV_LIN=sparse from the environment, margin 1.25), p = 1511,
//! seeds 17..17+n-1 (that study's instance seeds).  Arm plain: the generated spec.
//! Arm line: `jv_cover::search_fb_line(spec, 65536, seed)` first, its F_p
//! multiplications ADDED to the route's total (the sieve route does not know about
//! the search).  Usage: sieve_transfer_pilot <p> <first_seed> <n_seeds> [n_lines=65536]
use crypto_lib::cryptanalysis::jv_cover::{generate_spec, search_fb_line};
use crypto_lib::cryptanalysis::jv_sieve::run_cover_sieve_dlp_on;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let p: u64 = a.get(1).map_or(1511, |s| s.parse().unwrap());
    let first: u64 = a.get(2).map_or(17, |s| s.parse().unwrap());
    let n: u64 = a.get(3).map_or(8, |s| s.parse().unwrap());
    let n_lines: u64 = a.get(4).map_or(65536, |s| s.parse().unwrap());
    for seed in first..first + n {
        let spec = generate_spec(p, seed);
        let t0 = std::time::Instant::now();
        let r0 = run_cover_sieve_dlp_on(&spec, seed, 0, 1.3, Some(64), 1.25, None, false);
        let w0 = t0.elapsed().as_secs_f64();
        let (s2, ls) = search_fb_line(&spec, n_lines, seed);
        let t1 = std::time::Instant::now();
        let r1 = run_cover_sieve_dlp_on(&s2, seed, 0, 1.3, Some(64), 1.25, None, false);
        let w1 = t1.elapsed().as_secs_f64();
        let sqrt_l = (r0.l as f64).sqrt();
        let s1 = (r1.total_muls + ls.muls) as f64 / (331.0 * sqrt_l);
        println!(
            "JSON {{\"n_lines\":{n_lines},\"p\":{p},\"seed\":{seed},\"b_plain\":{},\"b_best\":{},\"plain\":{{\"base\":{},\"m\":{},\"relations\":{},\"descent_residuals\":{},\"sieve_muls\":{},\"relation_muls\":{},\"descent_muls\":{},\"la_muls\":{},\"total_muls\":{},\"total_plus\":{},\"s\":{:.6},\"solved\":{},\"correct\":{},\"wall_s\":{:.1}}},\"line\":{{\"base\":{},\"m\":{},\"relations\":{},\"descent_residuals\":{},\"sieve_muls\":{},\"relation_muls\":{},\"descent_muls\":{},\"la_muls\":{},\"total_muls_route\":{},\"search_muls\":{},\"total_muls_charged\":{},\"total_plus_route\":{},\"s_charged\":{:.6},\"solved\":{},\"correct\":{},\"wall_s\":{:.1}}},\"ratio_total_muls\":{:.4}}}",
            ls.b_plain, ls.b_best,
            r0.base, r0.m_final, r0.relations, r0.descent_residuals, r0.sieve_muls, r0.relation_muls, r0.descent_muls, r0.la_muls, r0.total_muls, r0.total_plus, r0.s, r0.solved, r0.correct, w0,
            r1.base, r1.m_final, r1.relations, r1.descent_residuals, r1.sieve_muls, r1.relation_muls, r1.descent_muls, r1.la_muls, r1.total_muls, ls.muls, r1.total_muls + ls.muls, r1.total_plus, s1, r1.solved, r1.correct, w1,
            (r1.total_muls + ls.muls) as f64 / r0.total_muls as f64
        );
    }
}
