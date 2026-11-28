//! Counted known-kernel Vélu cost vs Pollard-rho scale for prime degrees `ℓ`.
//!
//! Experimental half of `PROTOCOL.md`: once a degree-`ℓ` kernel is known,
//! classical Vélu evaluation costs `Θ(ℓ)` field multiplications. Compare
//! that bridge cost to `√N` (rho scale) on the certified `-19` conductor-gap
//! pairs from `research/conductor_gap_cm_pairs_20261010/results/`.
//!
//! Kernel search / `Φ_ℓ` is out of scope. CM instances use `j = −3375`
//! (`D = −7`) only to build a rational order-`ℓ` kernel for counting.

use crypto_lib::cryptanalysis::isogeny_eval::{
    counted_fp::{is_prime_u64, PrimeField},
    montgomery::{is_kernel_generator, ladder, MontgomeryCurve, XZ},
    velu,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
struct PairRow {
    ell: u64,
    n: u128,
    q_bits: u32,
}

fn parse_pairs(path: &str) -> Option<PairRow> {
    let text = fs::read_to_string(path).ok()?;
    let line = text.lines().next()?;
    let toks: Vec<&str> = line.split_whitespace().collect();
    let mut ell = None;
    let mut n = None;
    let mut q = None;
    for i in 0..toks.len() {
        if toks[i] == "ell" && toks.get(i + 1) == Some(&"=") {
            ell = toks.get(i + 2).and_then(|s| s.parse().ok());
        }
        if toks[i] == "N" && toks.get(i + 1) == Some(&"=") {
            if toks.get(i + 2) == Some(&"#E") && toks.get(i + 3) == Some(&"=") {
                n = toks.get(i + 4).and_then(|s| s.trim_end_matches('(').parse().ok());
            }
        }
        if toks[i] == "q" && toks.get(i + 1) == Some(&"=") {
            q = toks.get(i + 2).and_then(|s| s.parse::<u128>().ok());
        }
    }
    let ell = ell?;
    let n = n?;
    let q = q?;
    let q_bits = (128 - q.leading_zeros()) as u32;
    Some(PairRow { ell, n, q_bits })
}

struct Inst {
    p: u64,
    curve: MontgomeryCurve,
    n: u128,
    kernel_x: u64,
}

fn valuation(mut n: u128, l: u128) -> (u32, u128) {
    let mut e = 0;
    while n.is_multiple_of(l) {
        n /= l;
        e += 1;
    }
    (e, n)
}

fn cm_instance(ell: u64, bits: u32, rng: &mut StdRng) -> Inst {
    let lo: u128 = 1 << (bits + 1);
    let hi: u128 = 1 << (bits + 2);
    let l = ell as i128;
    'outer: loop {
        let wmax = (((hi - 1) / 7) as f64).sqrt() as u128 / ell as u128;
        let w = rng.gen_range(1..=wmax.max(1)) as u64;
        let v = ell * w;
        let sv = 7u128 * (v as u128) * (v as u128);
        if sv >= hi {
            continue;
        }
        let tmax = ((hi - 1 - sv) as f64).sqrt() as i128;
        let tmin = if sv >= lo {
            0
        } else {
            ((lo - sv) as f64).sqrt().ceil() as i128
        };
        let r = 2 % l;
        let span = (tmax - tmin) / (2 * l);
        if span < 1 {
            continue;
        }
        let k = rng.gen_range(0..=span);
        let mut t = tmin - tmin.rem_euclid(2 * l) + r + 2 * l * k;
        if t < tmin {
            t += 2 * l;
        }
        let four_p = (t * t) as u128 + sv;
        if t > tmax || !four_p.is_multiple_of(4) || four_p < lo || four_p >= hi {
            continue;
        }
        let p = (four_p / 4) as u64;
        if !is_prime_u64(p) || p < 1 << (bits - 1) {
            continue;
        }
        let f = PrimeField::new(p).unwrap();
        let n = (p as i128 + 1 - t) as u128;
        let mut roots = vec![7u64];
        if let Some(sq) = f.sqrt(f.from_i64(-7)) {
            let inv2 = f.inv(2).unwrap();
            roots.push(f.mul(f.sub(sq, 7), inv2));
            roots.push(f.mul(f.sub(f.neg(sq), 7), inv2));
        }
        for alpha in roots {
            let wpoly = f.add(f.mul_small(3, f.sqr(alpha)), f.from_i64(-35));
            let Some(s) = f.sqrt(wpoly).filter(|&s| s != 0) else {
                continue;
            };
            let a = f.div(f.mul_small(3, alpha), s).unwrap();
            let Ok(curve) = MontgomeryCurve::new(&f, a) else {
                continue;
            };
            let (e, m) = valuation(n, ell as u128);
            if e < 2 {
                continue;
            }
            for _ in 0..128 {
                let x = rng.gen_range(2..p);
                let mut q = ladder(&f, &XZ::affine(x), m, &curve.a24);
                if q.is_infinity() {
                    continue;
                }
                for _ in 0..e {
                    let r = ladder(&f, &q, ell as u128, &curve.a24);
                    if r.is_infinity() {
                        let kernel_x = q.to_affine(&f).unwrap();
                        if is_kernel_generator(&f, &curve, ell, kernel_x) {
                            return Inst {
                                p,
                                curve,
                                n,
                                kernel_x,
                            };
                        }
                        break;
                    }
                    q = r;
                }
            }
        }
        continue 'outer;
    }
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pairs_dir = root.join("research/conductor_gap_cm_pairs_20261010/results");
    let pair_files = ["ell_263_q19.txt", "ell_8209_q29.txt", "ell_131101_q37.txt"];
    let mut pairs = Vec::new();
    for name in pair_files {
        let p = pairs_dir.join(name);
        match parse_pairs(p.to_str().unwrap()) {
            Some(row) => pairs.push(row),
            None => eprintln!("warn: could not parse {}", p.display()),
        }
    }

    let mut rng = StdRng::seed_from_u64(20261120);
    let mut rows = Vec::new();

    println!("ell\tpair_N\tsqrt_N\tvelu_M\tvelu_I\tbridge/rho\tcm_p_bits");
    for pair in &pairs {
        let bits = (pair.q_bits + 8).max(40).min(61);
        eprintln!("building CM instance ell={} bits={} ...", pair.ell, bits);
        let inst = cm_instance(pair.ell, bits, &mut rng);
        let f = PrimeField::counted(inst.p).unwrap();
        let x = rng.gen_range(2..inst.p);
        let _ = velu(&f, &inst.curve, pair.ell, inst.kernel_x, &[x, inst.kernel_x])
            .expect("velu");
        let snap = f.tally();
        let sqrt_n = (pair.n as f64).sqrt();
        let bridge_over_rho = snap.mul as f64 / sqrt_n;
        println!(
            "{}\t{}\t{:.1}\t{}\t{}\t{:.3}\t{}",
            pair.ell,
            pair.n,
            sqrt_n,
            snap.mul,
            snap.inv,
            bridge_over_rho,
            64 - inst.p.leading_zeros()
        );
        rows.push(serde_json::json!({
            "ell": pair.ell,
            "pair_N": pair.n,
            "sqrt_N": sqrt_n,
            "velu_M": snap.mul,
            "velu_I": snap.inv,
            "bridge_over_rho": bridge_over_rho,
            "cm_p": inst.p,
            "cm_n": inst.n,
            "note": "known-kernel Vélu M-count; kernel search not included"
        }));
    }

    let out_dir = root.join("research/floor_crater_ecdlp_gap_20261120/results");
    fs::create_dir_all(&out_dir).unwrap();
    let json_path = out_dir.join("bridge_cost.json");
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&serde_json::json!({
            "protocol": "research/floor_crater_ecdlp_gap_20261120/PROTOCOL.md",
            "seed": 20261120,
            "rows": rows,
        }))
        .unwrap(),
    )
    .unwrap();
    println!("wrote {}", json_path.display());
}
