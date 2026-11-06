//! The symmetry check of `research/jv_sieve_prospective_20261023/`: on every weak
//! instance `y² = x(x − α)(x − σα)` over `F_{p⁶}` of `jv_cover::generate_spec(p, seed)`,
//! is `j(E)` in a proper subfield (`F_{p²}` or `F_{p³}`)? If it is, `E` is a twist of a
//! curve over that subfield and may carry a Frobenius-type endomorphism, so negation rho
//! would not be the strongest rho on it.
//!
//! `j` is computed from the Legendre parameter `λ = σα / α`:
//! `j = 256 (λ² − λ + 1)³ / (λ² (λ − 1)²)`.
//!
//! ```sh
//! cargo run --release --example jv_endo_check -- --sizes 251,503,1009,1511,1823 --seeds 16
//! ```

use crypto_lib::cryptanalysis::jv_cover::{generate_spec, Fld, Fq3, E6};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut sizes = vec![251u64, 503, 1009, 1511, 1823];
    let mut seeds = 16u64;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--sizes" => sizes = args[i + 1].split(',').map(|s| s.parse().unwrap()).collect(),
            "--seeds" => seeds = args[i + 1].parse().unwrap(),
            x => panic!("unknown argument {x}"),
        }
        i += 2;
    }
    let mut any = false;
    for &p in &sizes {
        let f = Fq3::new(p);
        for seed in 1..=seeds {
            let spec = generate_spec(p, seed);
            let a = spec.alpha;
            let lam = f.mul(&f.sigma(&a), &f.inv(&a));
            let one = f.one();
            let l2 = f.mul(&lam, &lam);
            let t = f.add(&f.sub(&l2, &lam), &one);
            let num = f.mul(&f.mul(&f.mul(&t, &t), &t), &f.from_fq(f.f.from_fp(256)));
            let lm1 = f.sub(&lam, &one);
            let den = f.mul(&l2, &f.mul(&lm1, &lm1));
            let j: E6 = f.mul(&num, &f.inv(&den));
            let p2 = (p as u128).pow(2);
            let p3 = (p as u128).pow(3);
            let in_fp2 = f.pow(&j, p2) == j;
            let in_fp3 = f.pow(&j, p3) == j;
            any |= in_fp2 || in_fp3;
            println!(
                "{{\"p\":{p},\"seed\":{seed},\"l\":{},\"j\":\"{:?}\",\"j_in_Fp2\":{in_fp2},\"j_in_Fp3\":{in_fp3}}}",
                spec.l, j
            );
        }
    }
    eprintln!("subfield j on any instance: {any}");
}
