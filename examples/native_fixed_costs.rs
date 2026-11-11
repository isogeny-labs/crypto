//! Native-size fixed costs of one table-collector solve and the per-trial lookup, on the
//! real E-222 / M-221 (Weierstrass images; research/ic_e222_m221_20261106/native/native_params.py):
//! factor-base construction (plain and cofactor-projected), pair-sum table build, one table
//! lookup as the collector makes it (key clone + hash probe), and Gaussian elimination mod n of
//! a (fb + 2) x (fb + 1) system with a planted solution.
//! Usage: native_fixed_costs NAME P A B GX GY N H FB REPS   (hex except H, FB, REPS)
use crypto_lib::cryptanalysis::ec_index_calculus::{
    build_factor_base, build_factor_base_projected, build_pair_sum_table,
    gaussian_eliminate_mod_n_particular,
};
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::point::Point;
use num_bigint::BigUint;
use num_traits::{Num, Zero};
use std::time::Instant;

fn hex(s: &str) -> BigUint {
    BigUint::from_str_radix(s, 16).expect("hex")
}

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let name: &'static str = Box::leak(v[1].clone().into_boxed_str());
    let curve = CurveParams {
        name,
        p: hex(&v[2]),
        a: hex(&v[3]),
        b: hex(&v[4]),
        gx: hex(&v[5]),
        gy: hex(&v[6]),
        n: hex(&v[7]),
        h: v[8].parse().unwrap(),
    };
    let fb_size: usize = v[9].parse().unwrap();
    let reps: usize = v[10].parse().unwrap();
    let ms = |t: Instant| t.elapsed().as_secs_f64() * 1e3;
    let avg = |xs: &[f64]| xs.iter().sum::<f64>() / xs.len() as f64;

    for (label, projected) in [("plain", false), ("projected", true)] {
        let mut t_fb = vec![];
        let mut t_tab = vec![];
        let mut entries = 0usize;
        let mut fb = vec![];
        let mut table = Default::default();
        for _ in 0..reps {
            let t = Instant::now();
            fb = if projected {
                build_factor_base_projected(&curve, fb_size)
            } else {
                build_factor_base(&curve, fb_size)
            };
            t_fb.push(ms(t));
            let t = Instant::now();
            table = build_pair_sum_table(&curve, &fb);
            t_tab.push(ms(t));
            entries = table.len();
        }
        // lookup as the collector makes it: clone two BigUint, hash, probe (a miss)
        let probe = (curve.p.clone() >> 3u32, curve.p.clone() >> 5u32);
        let n_look = 200_000usize;
        let t = Instant::now();
        let mut hit = 0usize;
        for i in 0..n_look {
            let key = (&probe.0 + BigUint::from(i as u64), probe.1.clone());
            if table.get(&key).is_some() {
                hit += 1;
            }
        }
        let look_us = t.elapsed().as_secs_f64() * 1e6 / n_look as f64;
        println!(
            "{name} {label}: factor base ({} pts) {:.1} ms | table build ({entries} entries) {:.1} ms | lookup {:.3} us (hits {hit})",
            fb.len(),
            avg(&t_fb),
            avg(&t_tab),
            look_us
        );
    }

    // elimination: (fb+2) x (fb+1) system mod n with a planted solution
    let m = fb_size;
    let mut st = 0x9e37_79b9_7f4a_7c15_u64;
    let mut next = || {
        st ^= st >> 12;
        st ^= st << 25;
        st ^= st >> 27;
        st.wrapping_mul(0x2545_f491_4f6c_dd1d)
    };
    let mut rand_mod = |n: &BigUint| {
        let mut x = BigUint::zero();
        for _ in 0..5 {
            x = (x << 64u32) + BigUint::from(next());
        }
        x % n
    };
    let sol: Vec<BigUint> = (0..=m).map(|_| rand_mod(&curve.n)).collect();
    let mut t_el = vec![];
    let mut determined = true;
    for _ in 0..reps {
        let mut matrix: Vec<Vec<BigUint>> = (0..m + 2)
            .map(|_| (0..=m).map(|_| rand_mod(&curve.n)).collect())
            .collect();
        let mut rhs: Vec<BigUint> = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .zip(&sol)
                    .fold(BigUint::zero(), |acc, (a, x)| (acc + a * x) % &curve.n)
            })
            .collect();
        let t = Instant::now();
        let r = gaussian_eliminate_mod_n_particular(&mut matrix, &mut rhs, &curve.n);
        t_el.push(ms(t));
        determined &= r
            .map(|s| s.determined[m] && s.values[m] == sol[m])
            .unwrap_or(false);
    }
    println!(
        "{name} elimination {}x{} mod n ({} bits): {:.2} ms (recovered planted last unknown in all reps: {determined})",
        m + 2,
        m + 1,
        curve.n.bits(),
        avg(&t_el)
    );
    let _ = Point::Infinity;
}
