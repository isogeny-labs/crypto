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
            if n.is_multiple_of(d) {
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
    if args.first().map(String::as_str) == Some("wide") {
        return run_wide(&args[1..]);
    }
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

/// `nist_cover_quartic_map wide [--bits 56] [--gamma-max G] [--seed S] --out F.json`
///
/// The ladder curve `find_a3_curve(bits, 3)` (largest `p = 3 mod 4` below
/// `2^bits`, `a = -3`, smallest `b` of prime order). Search `gamma` for a
/// Smith-positive cover; check `phi^*` on a planted pair; build the trigonal
/// map; fit the canonical quartic `C` from sampled points, verify it on a
/// held-out set, and certify `C` smooth by resultants. No point enumeration;
/// `Psi` into `Pic^0(C)` is not available at this size.
fn run_wide(args: &[String]) -> ExitCode {
    use crypto_lib::cryptanalysis::nist_cover_quartic::wide;
    use crypto_lib::cryptanalysis::prime_fast::find_a3_curve;
    use crypto_lib::cryptanalysis::smith_canonical as sc;

    let mut bits = 56u32;
    let mut gamma_max = 400u64;
    let mut seed = 0x5eed_5656u64;
    let mut out = String::new();
    let mut psi = false;
    let mut i = 0;
    while i < args.len() {
        let v = args.get(i + 1).cloned().unwrap_or_default();
        match args[i].as_str() {
            "--psi" => {
                psi = true;
                i += 1;
                continue;
            }
            "--bits" => bits = v.parse().unwrap(),
            "--gamma-max" => gamma_max = v.parse().unwrap(),
            "--seed" => seed = v.parse().unwrap(),
            "--out" => out = v,
            f => {
                eprintln!("unknown flag {f}");
                return ExitCode::from(3);
            }
        }
        i += 2;
    }
    if out.is_empty() {
        eprintln!("--out required");
        return ExitCode::from(3);
    }
    let t0 = std::time::Instant::now();
    let (curve, g) = find_a3_curve(bits, 3).expect("ladder curve");
    let p = curve.f.p;
    let a = curve.f.from_mont(curve.a);
    let b = curve.f.from_mont(curve.b);
    let n = curve.n;
    let (gx, gy) = curve.canonical(g);
    println!(
        "E: p = {p} (2^{bits} - {}), a = -3, b = {b}, n = {n} (prime), G = ({gx}, {gy})",
        (1u128 << bits) - p as u128
    );

    // Independent E arithmetic: nc::Ecc (u128 affine) against prime_fast.
    let e = nc::Ecc::new(p, a, b);
    let gp: nc::EPt = Some((gx, gy));
    let mut rng = sp::SplitMix64(seed);
    let k = 1 + rng.below(n - 1);
    let q = e.mul(&gp, k as u128);
    let q_fast = curve.mul(Some(g), k).map(|pt| curve.canonical(pt));
    let e_cross = q == q_fast;
    let e_order_ok = e.mul(&gp, n as u128).is_none() && gp.is_some();
    println!("planted k = {k}; [k]G agrees across two implementations: {e_cross}; [n]G = O: {e_order_ok}");

    // gamma search.
    let mut log = Vec::new();
    let mut found = None;
    for gamma in 1..=gamma_max {
        let f = nc::cover_f(p, a, b, gamma);
        match wide::trigonal(&f, p, seed ^ gamma) {
            Ok(w) => {
                log.push(json!({"gamma": gamma, "outcome": "smith_positive"}));
                found = Some((gamma, f, w));
                break;
            }
            Err(e) => log.push(json!({"gamma": gamma, "outcome": e})),
        }
    }
    let Some((gamma, f, w)) = found else {
        eprintln!("no Smith-positive gamma in [1, {gamma_max}]");
        let rec =
            json!({"p": p, "b": b, "n": n, "status": "no_smith_positive_gamma", "gamma_log": log});
        std::fs::write(&out, serde_json::to_string_pretty(&rec).unwrap()).ok();
        return ExitCode::from(1);
    };
    let tried = log.len();
    println!("gamma = {gamma} (first Smith-positive of {tried} tried); f = {f:?}");
    println!(
        "trigonal map: n1 = {}, n0 = {}, d1 = {}, d0 = {}",
        w.tr.n1, w.tr.n0, w.tr.d1, w.tr.d0
    );

    // phi^* checks in J_H at this prime.
    let jac = sc::HJac::new(&f, p).expect("deg 7");
    let pg = nc::pullback(&jac, gamma, &gp).expect("pullback G");
    let pq = nc::pullback(&jac, gamma, &q).expect("pullback Q");
    let r_pt = e.add(&gp, &q);
    let pr = nc::pullback(&jac, gamma, &r_pt).expect("pullback G+Q");
    let pull_nonzero = !jac.is_zero(&pg);
    let pull_order_n = jac.is_zero(&jac.mul(&pg, n as u128));
    let pull_hom_k = jac.is_zero(&jac.sub(&jac.mul(&pg, k as u128), &pq));
    let pull_additive = jac.is_zero(&jac.sub(&jac.add(&pg, &pq), &pr));
    // negative control: a wrong scalar must not match
    let pull_wrong_k = jac.is_zero(&jac.sub(&jac.mul(&pg, (k + 1) as u128), &pq));
    println!("phi*: G nonzero {pull_nonzero}; [n]phi*G = 0 {pull_order_n}; phi*([k]G) = [k]phi*G {pull_hom_k}; additive {pull_additive}; wrong-k control matches (must be false) {pull_wrong_k}");

    // Canonical quartic C from sampled points; held-out verification.
    let (fit_pts, fit_draws) = match wide::sample_images(&w, 40, &mut rng) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("sampling failed: {e}");
            return ExitCode::from(1);
        }
    };
    let (quartic, used) = match sc::fit_quartic_u64(p, &fit_pts) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("fit failed: {e}");
            return ExitCode::from(1);
        }
    };
    let (hold_pts, hold_draws) = wide::sample_images(&w, 120, &mut rng).expect("held-out sample");
    let k2 = crypto_lib::cryptanalysis::smith_trigonal::F2::new(p);
    let hold_on = hold_pts
        .iter()
        .filter(|h| k2.is_zero(wide::quartic_eval_fp2(&quartic, **h, p)))
        .count();
    let hold_rational = hold_pts
        .iter()
        .filter(|h| h.iter().all(|z| z.b == 0))
        .count();
    // negative control: perturb one coefficient
    let mut bad = quartic;
    bad[1] = (bad[1] + 1) % p;
    let hold_on_bad = hold_pts
        .iter()
        .filter(|h| k2.is_zero(wide::quartic_eval_fp2(&bad, **h, p)))
        .count();
    println!("C fitted from {used} points ({fit_draws} draws): {quartic:?}");
    println!("held-out: {hold_on}/{} on C ({hold_rational} rational); perturbed-C control: {hold_on_bad}/{}", hold_pts.len(), hold_pts.len());

    let (smooth, smooth_msg) = match wide::smooth_certificate(&quartic, p, seed ^ 0xC0FFEE) {
        Ok(x) => x,
        Err(e) => (false, format!("certificate error: {e}")),
    };
    println!("smoothness certificate: {smooth} ({smooth_msg})");

    // Amendment S56b: Psi into Pic^0(C) with the 64-bit Pic^0 arithmetic.
    let (psi_ok, psi_rec) = if psi {
        let t_psi = std::time::Instant::now();
        match wide::Transfer64::new(&w, quartic, &hold_pts) {
            Err(e) => {
                println!("Psi: Transfer64 failed: {e}");
                (false, json!({"status": "transfer64_failed", "reason": e}))
            }
            Ok(t64) => {
                let pool = t64.sample_pool(48, &mut rng);
                let mut rng_a = sp::SplitMix64(seed ^ 0xA11CE);
                let mut rng_b = sp::SplitMix64(seed ^ 0xB0B);
                let mut run = || -> Result<serde_json::Value, String> {
                    let g1 = t64.transfer_divisor(&jac, &pg, &pool, &mut rng_a)?;
                    let g2 = t64.transfer_divisor(&jac, &pg, &pool, &mut rng_b)?;
                    let qq = t64.transfer_divisor(&jac, &pq, &pool, &mut rng_a)?;
                    let rr = t64.transfer_divisor(&jac, &pr, &pool, &mut rng_a)?;
                    let c = &t64.c;
                    let well_defined = c.eq(&g1, &g2);
                    let nonzero = !c.is_zero(&g1);
                    let order_n = c.is_zero(&c.mul_scalar(&g1, n as u128));
                    let log_carried = c.eq(&c.mul_scalar(&g1, k as u128), &qq);
                    let additive = c.eq(&c.add(&g1, &qq), &rr);
                    let wrong_k = c.eq(&c.mul_scalar(&g1, (k + 1) as u128), &qq);
                    Ok(json!({
                        "pool_points": pool.len(),
                        "well_defined_two_representatives": well_defined,
                        "image_nonzero": nonzero,
                        "order_n_preserved": order_n,
                        "planted_log_carried": log_carried,
                        "additive": additive,
                        "wrong_k_control_matches": wrong_k,
                        "psi_G_key": g1.key().0.to_vec(),
                        "psi_Q_key": qq.key().0.to_vec(),
                    }))
                };
                match run() {
                    Err(e) => {
                        println!("Psi: transfer failed: {e}");
                        (false, json!({"status": "transfer_failed", "reason": e}))
                    }
                    Ok(mut v) => {
                        let b = |key: &str| v[key].as_bool().unwrap_or(false);
                        let ok = b("well_defined_two_representatives")
                            && b("image_nonzero")
                            && b("order_n_preserved")
                            && b("planted_log_carried")
                            && b("additive")
                            && !b("wrong_k_control_matches");
                        println!(
                            "Psi: well-defined {}; nonzero {}; [n]Psi(phi*G) = 0 {}; Psi(phi*Q) = [k]Psi(phi*G) {}; additive {}; wrong-k control matches (must be false) {} ({} pool points, {:.2} s)",
                            b("well_defined_two_representatives"), b("image_nonzero"), b("order_n_preserved"),
                            b("planted_log_carried"), b("additive"), b("wrong_k_control_matches"),
                            pool.len(), t_psi.elapsed().as_secs_f64()
                        );
                        v["status"] = json!("ok");
                        v["seconds"] = json!(t_psi.elapsed().as_secs_f64());
                        (ok, v)
                    }
                }
            }
        }
    } else {
        (
            true,
            json!("not_run: pass --psi (amendment S56b, plane_quartic_pic64)"),
        )
    };

    let all_ok = e_cross
        && e_order_ok
        && pull_nonzero
        && pull_order_n
        && pull_hom_k
        && pull_additive
        && !pull_wrong_k
        && hold_on == hold_pts.len()
        && hold_on_bad == 0
        && smooth
        && psi_ok;
    let rec = json!({
        "bits": bits, "p": p, "two_pow_bits_minus_p": ((1u128 << bits) - p as u128) as u64,
        "a": "-3", "a_mod_p": a, "b": b, "n": n, "generator_G": [gx, gy],
        "seed": seed, "planted_k": k, "target_Q": q,
        "e_scalar_mult_two_implementations_agree": e_cross, "e_order_n_ok": e_order_ok,
        "gamma": gamma, "gamma_tried": tried, "gamma_log": log,
        "cover_f_ascending": f,
        "trigonal_map": {"n1": w.tr.n1, "n0": w.tr.n0, "d1": w.tr.d1, "d0": w.tr.d0},
        "pullback": {"nonzero": pull_nonzero, "order_divides_n_prime": pull_order_n,
                      "homomorphism_planted_k": pull_hom_k, "additive": pull_additive,
                      "wrong_k_control_matches": pull_wrong_k},
        "quartic_C_mons_deg4": quartic.to_vec(),
        "fit_points_used": used, "fit_draws": fit_draws,
        "heldout_points": hold_pts.len(), "heldout_on_C": hold_on, "heldout_rational": hold_rational,
        "heldout_draws": hold_draws, "perturbed_C_control_on": hold_on_bad,
        "smooth_certified": smooth, "smooth_certificate": smooth_msg,
        "psi_pic0_transfer": psi_rec,
        "wall_seconds": t0.elapsed().as_secs_f64(),
        "all_checks_ok": all_ok,
        "status": "ok"
    });
    std::fs::write(&out, serde_json::to_string_pretty(&rec).unwrap()).ok();
    println!("=> {}", if all_ok { "OK" } else { "FAIL" });
    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
