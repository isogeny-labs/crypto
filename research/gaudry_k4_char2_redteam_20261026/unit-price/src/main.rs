use crypto_lib::cryptanalysis::gaudry_quartic_bin::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::hint::black_box;
use std::time::Instant;

fn med(mut v: Vec<f64>) -> f64 { v.sort_by(|a,b| a.partial_cmp(b).unwrap()); v[v.len()/2] }

fn main() {
    let ms: Vec<u32> = std::env::args().skip(1).map(|s| s.parse().unwrap()).collect();
    for m in ms {
        let bc = find_bin_curve(m, 1, 2).unwrap();
        let c = &bc.curve;
        let f = &c.f;
        let g = &f.f;
        let mut rng = StdRng::seed_from_u64(7);
        let q = g.q;
        // counted muls per op (as the oracle counts)
        let per_add = fq_muls_per_add(c);
        // F_q: dependent chain mul (latency), inv, sq(=mul)
        let n = 20_000_000u64;
        let mut res = vec![];
        for _ in 0..5 {
            let mut a = rng.gen_range(1..q); let b = rng.gen_range(1..q);
            let t = Instant::now();
            for _ in 0..n { a = g.mul_nc(a, b) ^ 1; a |= 1; }
            black_box(a);
            res.push(t.elapsed().as_nanos() as f64 / n as f64);
        }
        let mul_lat = med(res);
        // independent muls (throughput): 8 chains
        let mut res = vec![];
        for _ in 0..5 {
            let mut a: [u32;8] = core::array::from_fn(|_| rng.gen_range(1..q));
            let b: [u32;8] = core::array::from_fn(|_| rng.gen_range(1..q));
            let t = Instant::now();
            for _ in 0..n/8 { for i in 0..8 { a[i] = g.mul_nc(a[i], b[i]) | 1; } }
            black_box(a);
            res.push(t.elapsed().as_nanos() as f64 / n as f64);
        }
        let mul_thr = med(res);
        // random operands from precomputed list (random table access)
        let xs: Vec<u32> = (0..1<<16).map(|_| rng.gen_range(1..q)).collect();
        let mut res = vec![];
        for _ in 0..5 {
            let mut acc = 0u32;
            let t = Instant::now();
            for i in 0..n as usize { acc ^= g.mul_nc(xs[i & 0xffff], xs[(i*7+3) & 0xffff]); }
            black_box(acc);
            res.push(t.elapsed().as_nanos() as f64 / n as f64);
        }
        let mul_rand = med(res);
        // sq as mul(a,a) (what the code does)
        let mut res = vec![];
        for _ in 0..5 {
            let mut a = rng.gen_range(1..q);
            let t = Instant::now();
            for _ in 0..n { a = g.mul_nc(a, a) | 1; }
            black_box(a);
            res.push(t.elapsed().as_nanos() as f64 / n as f64);
        }
        let sq_lat = med(res);
        let mut res = vec![];
        for _ in 0..5 {
            let mut a = rng.gen_range(1..q);
            let t = Instant::now();
            for _ in 0..n { a = g.inv_nc(a) ^ 1; a |= 1; }
            black_box(a);
            res.push(t.elapsed().as_nanos() as f64 / n as f64);
        }
        let inv_lat = med(res);
        // axpy streaming, row length 4096
        let tab = &g.tab;
        let piv: Vec<u32> = (0..4096).map(|_| rng.gen_range(0..q)).collect();
        let mut row: Vec<u32> = (0..4096).map(|_| rng.gen_range(0..q)).collect();
        let reps = 200_000;
        let mut res = vec![];
        for _ in 0..5 {
            let t = Instant::now();
            for r in 0..reps { tab.axpy(&mut row, &piv, (r as u32 % (q-1)) + 1); }
            black_box(&row);
            res.push(t.elapsed().as_nanos() as f64 / (reps as f64 * 4096.0));
        }
        let axpy = med(res);
        // F_q4 mul, sq, inv
        let mut a = f.random(&mut rng); let b = f.random(&mut rng);
        let n4 = 2_000_000u64;
        let mut res = vec![];
        for _ in 0..5 { let t = Instant::now(); for _ in 0..n4 { a = f.mul(&a,&b); a.0[0] |= 1; } black_box(a); res.push(t.elapsed().as_nanos() as f64 / n4 as f64); }
        let fq4_mul = med(res);
        let mut res = vec![];
        for _ in 0..5 { let t = Instant::now(); for _ in 0..n4 { a = f.sq(&a); a.0[0] |= 1; } black_box(a); res.push(t.elapsed().as_nanos() as f64 / n4 as f64); }
        let fq4_sq = med(res);
        let mut res = vec![];
        for _ in 0..5 { let t = Instant::now(); for _ in 0..n4/4 { a = f.inv(&a); a.0[0] |= 1; } black_box(a); res.push(t.elapsed().as_nanos() as f64 / (n4/4) as f64); }
        let fq4_inv = med(res);
        // group add (distinct points) rho-style: P += J[k]; and double
        let mut jumps = vec![];
        let mut p = bc.generator;
        for _ in 0..16 { p = c.add(&p, &bc.generator); jumps.push(p); }
        let mut res = vec![]; let mut res_d = vec![];
        let na = 1_000_000u64;
        let mut p = bc.generator;
        for _ in 0..7 {
            let t = Instant::now();
            for i in 0..na { p = c.add(&p, &jumps[(i as usize) & 15]); if p.inf { p = bc.generator; } }
            black_box(p);
            res.push(t.elapsed().as_nanos() as f64 / na as f64);
            let t = Instant::now();
            for _ in 0..na/4 { p = c.add(&p, &p); if p.inf { p = bc.generator; } }
            black_box(p);
            res_d.push(t.elapsed().as_nanos() as f64 / (na/4) as f64);
        }
        let add = med(res); let dbl = med(res_d);
        println!("{{\"m\":{m},\"counted_muls_per_add\":{per_add},\"fq_mul_latency_ns\":{mul_lat:.3},\"fq_mul_throughput_ns\":{mul_thr:.3},\"fq_mul_randoperand_ns\":{mul_rand:.3},\"fq_sq_latency_ns\":{sq_lat:.3},\"fq_inv_ns\":{inv_lat:.3},\"axpy_ns_per_elem\":{axpy:.3},\"fq4_mul_ns\":{fq4_mul:.2},\"fq4_sq_ns\":{fq4_sq:.2},\"fq4_inv_ns\":{fq4_inv:.2},\"group_add_ns\":{add:.2},\"group_double_ns\":{dbl:.2}}}");
    }
}
