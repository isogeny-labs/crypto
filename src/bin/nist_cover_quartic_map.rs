//! `nist_cover_quartic_map --primes P,... [--gamma-max G] [--k K] --out F.jsonl`
//!
//! For each prime `p`, take the first `a=-3` nonsingular curve `E` of prime
//! order (NIST cofactor-1 shape), build a novel genus-3 hyperelliptic cover
//! `H_gamma` of `E` (search `gamma`), run Smith's trigonal method to the
//! explicit same-field plane quartic `C`, and check the full transfer chain
//!
//! ```text
//! E(F_p) --phi^*--> J_{H_gamma}(F_p) --Psi--> Pic^0(C)
//! ```
//!
//! carrying a planted logarithm `k` (generator `P`, target `Q=[k]P`) onto `C`,
//! recovered there by BSGS. Emits one JSON record per prime with the explicit
//! cover `f`, quartic `C`, counts, and every check.
//!
//! Correctness study; no timing or cost. Protocol:
//! `research/nist_cover_quartic_map_20261116/PROTOCOL.md`.

use std::io::Write;
use std::process::ExitCode;

use crypto_lib::cryptanalysis::nist_cover_quartic as nc;
use crypto_lib::cryptanalysis::smith_prop24 as sp;
use serde_json::json;

fn find_prime_e(p: u64) -> Option<(u64, u128)> {
    let a = p - 3;
    for b in 1..p {
        let e = nc::Ecc::new(p, a, b);
        let disc = (4 * (a as u128).pow(3) % p as u128 + 27 * (b as u128).pow(2)) % p as u128;
        if disc == 0 {
            continue;
        }
        let n = nc::ecc_order(&e);
        // prime test via trial factor
        let mut is_prime = n >= 2;
        let mut d = 2u128;
        while d * d <= n {
            if n % d == 0 {
                is_prime = false;
                break;
            }
            d += 1;
        }
        if is_prime {
            return Some((b, n));
        }
    }
    None
}

fn generator(e: &nc::Ecc, r: u128) -> Option<nc::EPt> {
    for x in 0..e.p {
        let rhs = ((x as u128).pow(3) + e.a as u128 * x as u128 + e.b as u128) % e.p as u128;
        if let Some(y) = sp::sqrt_mod(rhs as u64, e.p) {
            let pt = Some((x, y));
            if e.order(&pt, r) == r {
                return Some(pt);
            }
        }
    }
    None
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut primes: Vec<u64> = vec![];
    let mut gamma_max = 500u64;
    let mut k_plant = 0u128; // 0 => choose r/3 + 1
    let mut out = String::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--primes" => {
                i += 1;
                primes = args[i].split(',').map(|s| s.parse().unwrap()).collect();
            }
            "--gamma-max" => {
                i += 1;
                gamma_max = args[i].parse().unwrap();
            }
            "--k" => {
                i += 1;
                k_plant = args[i].parse().unwrap();
            }
            "--out" => {
                i += 1;
                out = args[i].clone();
            }
            f => {
                eprintln!("unknown flag {f}");
                return ExitCode::from(3);
            }
        }
        i += 1;
    }
    if primes.is_empty() || out.is_empty() {
        eprintln!("usage: nist_cover_quartic_map --primes P,... --out F [--gamma-max G] [--k K]");
        return ExitCode::from(3);
    }

    let mut file = match std::fs::File::create(&out) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("cannot create {out}: {e}");
            return ExitCode::from(1);
        }
    };

    let mut all_ok = true;
    for &p in &primes {
        let a = p - 3;
        let (b, n) = match find_prime_e(p) {
            Some(x) => x,
            None => {
                eprintln!("p={p}: no prime-order a=-3 curve");
                let rec = json!({"p": p, "status": "no_prime_e"});
                writeln!(file, "{rec}").ok();
                all_ok = false;
                continue;
            }
        };
        let chain = match nc::Chain::build(p, a, b, gamma_max, true) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("p={p} b={b}: {e}");
                let rec = json!({"p": p, "a": a, "b": b, "e_order": n as u64,
                    "status": "no_smith_positive_cover", "reason": e});
                writeln!(file, "{rec}").ok();
                all_ok = false;
                continue;
            }
        };
        let e = chain.ecc();
        let r = chain.e_prime_subgroup;
        let gp = generator(&e, r).expect("generator");
        let k = if k_plant == 0 {
            (r / 3) + 1
        } else {
            k_plant % r
        };
        let q = e.mul(&gp, k);

        let mut rng = sp::SplitMix64(0x5eed_0000_0000_0001u64 ^ p);

        // Pullback and order checks in J_H.
        let pg = nc::pullback(&chain.jac, chain.gamma, &gp).expect("pullback P");
        let pq = nc::pullback(&chain.jac, chain.gamma, &q).expect("pullback Q");
        let ord_pg = chain.jac.order(&pg, n);
        let hom_ok = {
            let kpg = chain.jac.mul(&pg, k);
            chain.jac.is_zero(&chain.jac.sub(&kpg, &pq))
        };

        // Transfer to Pic^0(C) and recover k there.
        let psi_g = chain.transfer_point(&gp, &mut rng).expect("Psi phi* P");
        let psi_q = chain.transfer_point(&q, &mut rng).expect("Psi phi* Q");
        let c = &chain.transfer.c;
        let img_nonzero = !c.is_zero(&psi_g);
        let recovered = nc::bsgs_quartic(c, &psi_g, &psi_q, r);
        let dlp_ok = recovered == Some(k % r);

        let nc_fp =
            crypto_lib::cryptanalysis::smith_canonical::count_quartic_points(p, &chain.quartic, 1);
        let nh_fp = crypto_lib::cryptanalysis::smith_canonical::count_h_points(&chain.f, p, 1);

        let ok = ord_pg == r && hom_ok && img_nonzero && dlp_ok && nc_fp == nh_fp;
        all_ok &= ok;

        let rec = json!({
            "p": p, "a": a, "b": b, "gamma": chain.gamma,
            "e_order": n as u64, "e_prime": r as u64, "e_cofactor": (n / r) as u64,
            "cover_f_ascending": chain.f,
            "quartic_C_mons_deg4": chain.quartic.to_vec(),
            "count_C_Fp": nc_fp, "count_H_Fp": nh_fp, "counts_match": nc_fp == nh_fp,
            "usable_points_over_U": chain.pts_over_u.len(),
            "generator_P": gp, "planted_k": k as u64, "target_Q": q,
            "ord_pullback_P": ord_pg as u64, "pullback_injective_on_r": ord_pg == r,
            "pullback_homomorphism": hom_ok,
            "image_generator_nonzero_on_C": img_nonzero,
            "recovered_log_on_C": recovered.map(|x| x as u64),
            "dlp_transfer_ok": dlp_ok,
            "all_checks_ok": ok,
            "status": "ok"
        });
        writeln!(file, "{rec}").ok();
        println!(
            "p={p} a=-3 b={b} #E={n}(prime) gamma={} | ord(phi*P)={ord_pg} hom={hom_ok} #C=#H={} dlp_on_C={dlp_ok} k={k} rec={:?} => {}",
            chain.gamma,
            nc_fp == nh_fp,
            recovered,
            if ok { "OK" } else { "FAIL" }
        );
    }

    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
