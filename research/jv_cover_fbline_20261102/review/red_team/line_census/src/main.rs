//! Red-team line census for research/jv_cover_fbline_20261102.
//!
//! For each of the study's nine instances (`jv_cover::generate_spec(p, seed)`):
//!
//! * **Automorphism check.** `j(E)` of `E : y² = x(x−α)(x−σα)` over `F_{p⁶}` from
//!   `λ = σα/α`; is `j ∈ F_{p²}` or `F_{p³}`?  (If not, `E` is not a twist of a curve
//!   over a proper subfield, has no Frobenius-type endomorphism of small order on
//!   `E(F_{p⁶})`, and `Aut(E) = {±1}`: the negation map is the full automorphism
//!   group available to rho.)
//! * **Replication.** The square-character table `χ(h(x))` on all of `F_q`, the
//!   plain line's `B`, and the study's own 65,536-line search with its RNG stream
//!   (`seed ^ 0xFB_11AE`), compared with the record (`b_plain`, `b_best`, `best_index`).
//! * **Census.** `B` of every line `a·F_p + b` with `a` a square (all `p(p+1)/2`
//!   lines): its exact distribution, the plain line's and the chosen line's
//!   percentiles, the exact expected maximum of 65,536 draws, the fair
//!   ("typical plain instance") speed-up, and the ceiling of an exhaustive search.
//!
//! Output: one block of text per instance and a JSON-lines summary on stdout.
//! Build: linked by `rustc` against the repository's compiled `crypto_lib` (see
//! `build.sh`).
use crypto_lib::cryptanalysis::jv_cover::{generate_spec, Cover, Fld, Fq, Fq3, E2};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::sync::Arc;

const N_SEARCH: u64 = 65536;

// (p, seed, b_plain, b_best, best_index) from results/line_p*_s*.json (fb_line)
const RECORD: [(u64, u64, usize, usize, u64); 9] = [
    (1301, 1, 634, 728, 37845),
    (1301, 2, 651, 725, 36013),
    (1301, 3, 662, 730, 8085),
    (1451, 1, 735, 811, 23659),
    (1451, 2, 729, 800, 17465),
    (1451, 3, 703, 804, 49965),
    (1601, 1, 764, 884, 60117),
    (1601, 2, 820, 883, 9758),
    (1601, 3, 824, 883, 55916),
];

fn rate_ff(b: f64, p: f64) -> f64 {
    let mut prod = 1.0;
    for k in 0..6 {
        prod *= (b - k as f64) / p;
    }
    64.0 * prod / 720.0
}
/// Residuals needed: (B+1) relations at the falling-factorial rate.
fn resid_model(b: f64, p: f64) -> f64 {
    (b + 1.0) / rate_ff(b, p)
}

fn main() {
    let only: Option<u64> = std::env::args().nth(1).map(|s| s.parse().unwrap());
    for &(p, seed, rec_plain, rec_best, rec_idx) in RECORD.iter() {
        if only.is_some_and(|q| q != p) {
            continue;
        }
        let t0 = std::time::Instant::now();
        let spec = generate_spec(p, seed);
        let f3 = Fq3::new(p);
        // ── automorphism check
        let a6 = spec.alpha;
        let lam = f3.mul(&f3.sigma(&a6), &f3.inv(&a6));
        let one = f3.one();
        let l2 = f3.mul(&lam, &lam);
        let t = f3.add(&f3.sub(&l2, &lam), &one);
        let num = f3.mul(&f3.mul(&f3.mul(&t, &t), &t), &f3.from_fq(f3.f.from_fp(256)));
        let lm1 = f3.sub(&lam, &one);
        let den = f3.mul(&l2, &f3.mul(&lm1, &lm1));
        let j = f3.mul(&num, &f3.inv(&den));
        let p2 = (p as u128).pow(2);
        let p3 = (p as u128).pow(3);
        let j_fp2 = f3.pow(&j, p2) == j;
        let j_fp3 = f3.pow(&j, p3) == j;
        let j0 = j == f3.from_fq(E2::ZERO);
        let j1728 = j == f3.from_fq(f3.f.from_fp(1728));
        // ── cover polynomial
        let cov = Cover::new(&f3, &a6).expect("cover");
        let h: Vec<E2> = cov.hx.clone();
        let deg = h.len() - 1;
        let w = f3.f.w;
        let pu = p as usize;
        // squares of F_p
        let mut qr = vec![false; pu];
        for x in 1..p {
            qr[((x * x) % p) as usize] = true;
        }
        // χ table on F_q: chi[x0 + p·x1] = 1 iff h(x) is a non-zero square
        let mut chi = vec![0u8; pu * pu];
        for x1 in 0..p {
            for x0 in 0..p {
                let (mut r0, mut r1) = (h[deg].0[0], h[deg].0[1]);
                for k in (0..deg).rev() {
                    let n0 = (r0 * x0 + w * ((r1 * x1) % p)) % p;
                    let n1 = (r0 * x1 + r1 * x0) % p;
                    r0 = (n0 + h[k].0[0]) % p;
                    r1 = (n1 + h[k].0[1]) % p;
                }
                let n = (r0 * r0 % p + p - (w * (r1 * r1 % p)) % p) % p;
                if n != 0 && qr[n as usize] {
                    chi[(x0 + p * x1) as usize] = 1;
                }
            }
        }
        let chi = Arc::new(chi);
        let line_b = |a: (u64, u64), b: (u64, u64), chi: &[u8]| -> usize {
            let (mut x0, mut x1) = b;
            let mut c = 0usize;
            for _ in 0..p {
                c += chi[(x0 + p * x1) as usize] as usize;
                x0 += a.0;
                if x0 >= p {
                    x0 -= p;
                }
                x1 += a.1;
                if x1 >= p {
                    x1 -= p;
                }
            }
            c
        };
        let b_plain = line_b((1, 0), (0, 0), &chi);
        // ── replicate the study's search (same RNG stream as search_fb_line)
        let fq = Fq::new(p);
        let mut rng = StdRng::seed_from_u64(seed ^ 0xFB_11AE);
        let mut best = (0usize, 0u64);
        let mut samples: Vec<usize> = Vec::with_capacity(N_SEARCH as usize);
        for i in 0..N_SEARCH {
            let (tt, bb) = if i == 0 {
                (E2::ONE, E2::ZERO)
            } else {
                loop {
                    let tt = fq.random(&mut rng);
                    if tt != E2::ZERO {
                        break (tt, fq.random(&mut rng));
                    }
                }
            };
            let a = fq.mul(&tt, &tt);
            let c = line_b((a.0[0], a.0[1]), (bb.0[0], bb.0[1]), &chi);
            samples.push(c);
            if c > best.0 {
                best = (c, i);
            }
        }
        // ── census of every square-direction line, 4 threads over directions
        let mut dirs: Vec<(u64, u64)> = vec![(1, 0)];
        for c in 0..p {
            // N(c + i) = c² − w, a square of F_p?
            let n = (c * c % p + p - w % p) % p;
            if n != 0 && qr[n as usize] {
                dirs.push((c, 1));
            }
        }
        let n_dirs = dirs.len();
        let dirs = Arc::new(dirs);
        let mut handles = Vec::new();
        for th in 0..4usize {
            let dirs = Arc::clone(&dirs);
            let chi = Arc::clone(&chi);
            handles.push(std::thread::spawn(move || {
                let mut hist = vec![0u64; pu + 1];
                let mut k = th;
                while k < dirs.len() {
                    let a = dirs[k];
                    for u in 0..p {
                        // b = a·(u·i) = (w·a1·u, a0·u)
                        let b = ((w * a.1 % p) * u % p, a.0 * u % p);
                        let (mut x0, mut x1) = b;
                        let mut c = 0usize;
                        for _ in 0..p {
                            c += chi[(x0 + p * x1) as usize] as usize;
                            x0 += a.0;
                            if x0 >= p {
                                x0 -= p;
                            }
                            x1 += a.1;
                            if x1 >= p {
                                x1 -= p;
                            }
                        }
                        hist[c] += 1;
                    }
                    k += 4;
                }
                hist
            }));
        }
        let mut hist = vec![0u64; pu + 1];
        for hd in handles {
            for (i, v) in hd.join().unwrap().iter().enumerate() {
                hist[i] += v;
            }
        }
        let n_lines: u64 = hist.iter().sum();
        let nf = n_lines as f64;
        let pf = p as f64;
        let mean = hist.iter().enumerate().map(|(b, &c)| b as f64 * c as f64).sum::<f64>() / nf;
        let var = hist
            .iter()
            .enumerate()
            .map(|(b, &c)| (b as f64 - mean).powi(2) * c as f64)
            .sum::<f64>()
            / nf;
        let sdv = var.sqrt();
        let skew = hist
            .iter()
            .enumerate()
            .map(|(b, &c)| ((b as f64 - mean) / sdv).powi(3) * c as f64)
            .sum::<f64>()
            / nf;
        let kurt = hist
            .iter()
            .enumerate()
            .map(|(b, &c)| ((b as f64 - mean) / sdv).powi(4) * c as f64)
            .sum::<f64>()
            / nf
            - 3.0;
        let bmax = hist.iter().rposition(|&c| c > 0).unwrap();
        let n_at_max = hist[bmax];
        let below = |x: usize| hist[..x].iter().sum::<u64>() as f64 / nf;
        // exact distribution of the max of n draws (with replacement) from the census
        let mut cdf = vec![0f64; pu + 1];
        let mut acc = 0u64;
        for b in 0..=pu {
            acc += hist[b];
            cdf[b] = acc as f64 / nf;
        }
        let nn = N_SEARCH as f64;
        let mut e_max = 0.0;
        let mut e_gmax = 0.0;
        let mut p_ge_best = 0.0;
        for b in 0..=pu {
            let lo = if b == 0 { 0.0 } else { cdf[b - 1] };
            let pm = cdf[b].powf(nn) - lo.powf(nn);
            if pm > 0.0 {
                e_max += pm * b as f64;
                if b >= 6 {
                    e_gmax += pm * resid_model(b as f64, pf);
                }
                if b >= rec_best {
                    p_ge_best += pm;
                }
            }
        }
        let e_gpop = hist
            .iter()
            .enumerate()
            .filter(|(b, _)| *b >= 6)
            .map(|(b, &c)| resid_model(b as f64, pf) * c as f64)
            .sum::<f64>()
            / nf;
        let e_b5pop = hist
            .iter()
            .enumerate()
            .filter(|(b, _)| *b >= 6)
            .map(|(b, &c)| (b as f64).powi(-5) * c as f64)
            .sum::<f64>()
            / nf;
        let fair = e_gmax / e_gpop; // expected line-arm residuals / expected plain residuals
        let realised = resid_model(best.0 as f64, pf) / resid_model(b_plain as f64, pf);
        let ceiling = resid_model(bmax as f64, pf) / resid_model(best.0 as f64, pf);
        let s_mean = samples.iter().sum::<usize>() as f64 / nn;
        let s_sd = (samples.iter().map(|&c| (c as f64 - s_mean).powi(2)).sum::<f64>() / (nn - 1.0)).sqrt();
        let rep_ok = b_plain == rec_plain && best.0 == rec_best && best.1 == rec_idx;
        println!(
            "p={p} s={seed}: j in F_p2 {j_fp2} j in F_p3 {j_fp3} j=0 {j0} j=1728 {j1728} | deg h {deg} | \
             replication: B_plain {b_plain} (record {rec_plain}) B_best {} idx {} (record {rec_best}, {rec_idx}) -> {} | \
             search sample mean {:.2} sd {:.2} | census: {n_lines} lines ({n_dirs} square directions x {p}), mean {:.2} (model {:.1}), sd {:.2} (model sqrt(p)/2 = {:.2}), skew {:+.3}, ex.kurt {:+.3}, max {bmax} (x{n_at_max}) | \
             percentile B_plain {:.3} B_best {:.6} | E[max of 65536] {:.1}, P(max >= B_best) {:.3} | \
             fair residual ratio E[g(max)]/E[g(B)] {:.4} (x{:.3}); E[B^-5]^(-1/5) {:.1}; realised model ratio {:.4}; exhaustive ceiling extra x{:.3} | {:.0} s",
            best.0,
            best.1,
            if rep_ok { "MATCH" } else { "MISMATCH" },
            s_mean,
            s_sd,
            mean,
            (pf - 1.0) / 2.0,
            sdv,
            pf.sqrt() / 2.0,
            skew,
            kurt,
            below(b_plain) + 0.5 * hist[b_plain] as f64 / nf,
            below(best.0) + 0.5 * hist[best.0] as f64 / nf,
            e_max,
            p_ge_best,
            fair,
            1.0 / fair,
            e_b5pop.powf(-0.2),
            realised,
            1.0 / ceiling,
            t0.elapsed().as_secs_f64()
        );
        println!(
            "JSON {{\"p\":{p},\"seed\":{seed},\"j_in_fp2\":{j_fp2},\"j_in_fp3\":{j_fp3},\"replication_match\":{rep_ok},\"b_plain\":{b_plain},\"b_best\":{},\"best_index\":{},\"n_lines\":{n_lines},\"mean\":{mean:.4},\"sd\":{sdv:.4},\"skew\":{skew:.4},\"exkurt\":{kurt:.4},\"b_max_all\":{bmax},\"n_at_max\":{n_at_max},\"pct_plain\":{:.5},\"e_max_65536\":{e_max:.3},\"p_max_ge_best\":{p_ge_best:.4},\"fair_ratio\":{fair:.5},\"realised_model_ratio\":{realised:.5},\"exhaustive_extra_speedup\":{:.5}}}",
            best.0,
            best.1,
            below(b_plain) + 0.5 * hist[b_plain] as f64 / nf,
            1.0 / ceiling
        );
    }
}
