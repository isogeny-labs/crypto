#![allow(dead_code)]
//! Toy reproduction of the quartic-extension genus-five cover family.
//!
//!   humbert classify                         binary-pattern table (lead 2)
//!   humbert verify  <p> <twist> <seed>       zeta / L-polynomial check of C (lead 1)
//!   humbert attack  <p> <twist> <seed> <arm> end-to-end IC on C vs matched rho (leads 1, 3, 4)
//!                                            arm = full | iota
mod classify;
mod cover;
mod curve;
mod field;
mod la;
mod lpsummary;
mod poly;

use cover::*;
use curve::{invr, mulr, pollard_rho, random_point, Rng};
use field::FieldOps;
use std::time::Instant;

fn peak_rss_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| s.lines().find(|l| l.starts_with("VmHWM:")).map(|l| l.split_whitespace().nth(1).unwrap().parse().unwrap()))
        .unwrap_or(0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("classify") => {
            for d in [2usize, 3, 4] {
                for g0 in [1usize, 2, 3] {
                    for row in classify::enumerate(d, g0) {
                        println!("{}", classify::row_json(&row));
                    }
                }
            }
        }
        Some("verify") => verify(args[2].parse().unwrap(), args[3].parse().unwrap(), args[4].parse().unwrap()),
        Some("summarize-lp") => {
            if let Err(e) = lpsummary::run(&args[2]) {
                eprintln!("summarize-lp: {e}");
                std::process::exit(1);
            }
        }
        Some("attack") => attack(args[2].parse().unwrap(), args[3].parse().unwrap(), args[4].parse().unwrap(), &args[5]),
        _ => eprintln!("usage: humbert classify | verify p twist seed | attack p twist seed full|iota"),
    }
}

fn poly_mul_i(a: &[i128], b: &[i128]) -> Vec<i128> {
    let mut c = vec![0i128; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] += x * y;
        }
    }
    c
}

/// Lead 1: point counts N_1..N_5 of the explicit model -> L-polynomial, compared
/// with L_E(T^4) L_D(T) (E counted by BSGS on E, D counted on its own equation).
/// Negative control: the same counts compared with a wrong-twist prediction.
fn verify(p: u64, twist: i32, seed: u64) {
    let mut rng = Rng(seed);
    let t0 = Instant::now();
    let s = Setup::new(p, twist, &mut rng);
    let counts = count_c(&s, 5, &mut rng);
    let lc = lpoly_from_counts(p, &counts, 5);
    let pi = p as i128;
    let a_e = pi.pow(4) + 1 - s.order as i128;
    let a_d = pi + 1 - s.d_count as i128;
    let le4 = {
        let mut v = vec![0i128; 9];
        v[0] = 1;
        v[4] = -a_e;
        v[8] = pi.pow(4);
        v
    };
    let ld = vec![1, -a_d, pi];
    let pred = poly_mul_i(&le4, &ld);
    let ok = pred == lc;
    // negative control: quadratic twist of E (a_E -> -a_E) with the same D
    let mut le4t = le4.clone();
    le4t[4] = a_e;
    let pred_twist = poly_mul_i(&le4t, &ld);
    let control_distinguished = pred_twist != lc;
    // h1 of degree one: E_b: y^2 = eta (x - l0)(x - l1)(x - l2)(x - b), b in F_p, is the
    // Moebius image x -> 1/(x - b) of the cubic form with roots 1/(l_j - b) and constant
    // eta prod(b - l_j).  Both orders are counted by brute force over F_{p^4}.
    let k = s.k();
    let b = [rng.below(p), 0, 0, 0];
    let quartic_count = {
        let mut n: u64 = (1 + k.chi(&s.eta)) as u64;
        for idx in 0..(p as u64).pow(4) {
            let x = [idx % p, (idx / p) % p, (idx / p / p) % p, idx / p / p / p];
            let mut f = s.eta;
            for j in 0..3 {
                f = k.mul(&f, &k.sub(&x, &s.lams[j]));
            }
            f = k.mul(&f, &k.sub(&x, &b));
            n += (1 + k.chi(&f)) as u64;
        }
        n
    };
    let cubic_image_count = {
        let mut c = s.eta;
        let mut roots = Vec::new();
        for j in 0..3 {
            c = k.mul(&c, &k.sub(&b, &s.lams[j]));
            roots.push(k.inv(&k.sub(&s.lams[j], &b)));
        }
        let mut n: u64 = 1;
        for idx in 0..(p as u64).pow(4) {
            let x = [idx % p, (idx / p) % p, (idx / p / p) % p, idx / p / p / p];
            let mut f = c;
            for rt in &roots {
                f = k.mul(&f, &k.sub(&x, rt));
            }
            n += (1 + k.chi(&f)) as u64;
        }
        n
    };
    let jac: i128 = lc.iter().sum();
    let jac_pred = s.order as i128 * s.d_count as i128;
    println!(
        "{{\"kind\":\"verify\",\"p\":{},\"twist\":{},\"seed\":{},\"counts\":{:?},\"lpoly_from_counts\":{:?},\"lpoly_predicted\":{:?},\"match\":{},\"negative_control_twisted_E_distinguished\":{},\"E_order\":{},\"D_order\":{},\"jac_order_from_counts\":{},\"jac_order_predicted\":{},\"r\":{},\"quartic_form_order\":{},\"moebius_cubic_order\":{},\"quartic_form_equivalent\":{},\"seconds\":{:.3}}}",
        p, twist, seed, counts, lc, pred, ok, control_distinguished, s.order, s.d_count, jac, jac_pred, s.r, quartic_count, cubic_image_count, quartic_count == cubic_image_count, t0.elapsed().as_secs_f64()
    );
}

const DENSE_MAX: usize = 1200;

fn attack(p: u64, twist: i32, seed: u64, arm: &str) {
    // "full" | "iota" | "lp:<alpha>": iota-folded, small factor base of a fraction
    // alpha of the x-values, single large prime relations combined in pairs
    let (iota, lp_alpha): (bool, Option<f64>) = match arm {
        "full" => (false, None),
        "iota" => (true, None),
        a if a.starts_with("lp:") => (true, Some(a[3..].parse().expect("lp:<alpha>"))),
        _ => panic!("arm must be full, iota or lp:<alpha>"),
    };
    let mut rng = Rng(seed);
    let t_all = Instant::now();
    // ---- cover construction ------------------------------------------------
    let t = Instant::now();
    let s = Setup::new(p, twist, &mut rng);
    let ce = s.curve();
    let r = s.r;
    let cof = s.order / r as u128;
    let sq = |a: &field::E4| s.sqrt_k4(a);
    let gen = loop {
        let pt = ce.mul(&random_point(&ce, &sq, &mut rng), cof);
        if pt.is_some() {
            break pt;
        }
    };
    assert!(ce.mul(&gen, r as u128).is_none());
    let x_true = 1 + rng.below(r - 1);
    let q = ce.mul(&gen, x_true as u128);
    eprintln!("[setup] r={} #E={} {:.2}s", r, s.order, t.elapsed().as_secs_f64());
    let mut fb = FactorBase::build(&s);
    if let Some(alpha) = lp_alpha {
        // small base: a seeded random subset of the x-values (its own RNG stream)
        let mut idx: Vec<usize> = (0..fb.xs.len()).collect();
        let mut prng = Rng(seed ^ 0x5EED_1A2B);
        for i in (1..idx.len()).rev() {
            idx.swap(i, prng.below(i as u64 + 1) as usize);
        }
        let k = ((alpha * idx.len() as f64).ceil() as usize).clamp(12, idx.len());
        idx.truncate(k);
        idx.sort();
        fb.pick = idx;
    }
    let fb = fb;
    let nodes = Nodes::new(&s);
    eprintln!("[fb] points={}", fb.n_points());
    let t_setup = t.elapsed().as_secs_f64();
    let k4_after_setup = s.k().muls.get();
    let nx = fb.xs.len();
    let nfb = fb.n_points();
    // columns
    let nsmall = fb.pick.len();
    let mut small_col: Vec<Option<usize>> = vec![None; nx]; // compact column of a small x (pinned: None)
    let mut is_small = vec![false; nx];
    for (k, &j) in fb.pick.iter().enumerate() {
        is_small[j] = true;
        if k > 0 {
            small_col[j] = Some(k - 1);
        }
    }
    let pinned = fb.pick[0];
    let (ncols, inf_col, c_col) = if iota { (nsmall + 1, nsmall, nsmall - 1) } else { (nfb + 1 - 1, nfb - 1, usize::MAX) };
    // big coordinates: key j < nx is Y_j, BIG_C is c, BIG_INF is D_oo
    let big_c = nx;
    let big_inf = nx + 1;
    let merge = |mut row: Vec<(usize, u64)>| -> Vec<(usize, u64)> {
        row.sort();
        let mut merged: Vec<(usize, u64)> = Vec::new();
        for (c, v) in row {
            match merged.last_mut() {
                Some(last) if last.0 == c => last.1 = (last.1 + v) % r,
                _ => merged.push((c, v)),
            }
        }
        merged.retain(|e| e.1 != 0);
        merged
    };
    let to_big = |rel: &Relation| -> Vec<(usize, u64)> {
        let mut row = Vec::new();
        for &i in &rel.pts {
            let j = i / 2;
            if i % 2 == 0 {
                row.push((j, 1));
            } else {
                row.push((j, r - 1));
                row.push((big_c, 1));
            }
        }
        let inf = if rel.inf >= 0 { rel.inf as u64 % r } else { r - ((-rel.inf) as u64 % r) };
        row.push((big_inf, inf));
        merge(row)
    };
    let compact = |big: &[(usize, u64)]| -> Vec<(usize, u64)> {
        let mut row = Vec::new();
        for &(k, v) in big {
            if k == big_c {
                row.push((c_col, v));
            } else if k == big_inf {
                row.push((inf_col, v));
            } else if k == pinned {
                // Y_pinned = 0
            } else {
                row.push((small_col[k].expect("large prime left in a full relation"), v));
            }
        }
        merge(row)
    };
    let to_row = |rel: &Relation| -> Vec<(usize, u64)> {
        let mut row: Vec<(usize, u64)> = Vec::new();
        let mut push = |c: Option<usize>, v: u64| {
            if let Some(c) = c {
                row.push((c, v));
            }
        };
        for &i in &rel.pts {
            if iota {
                let j = i / 2;
                let yc = if j == 0 { None } else { Some(j - 1) };
                if i % 2 == 0 {
                    push(yc, 1);
                } else {
                    push(yc, r - 1);
                    push(Some(c_col), 1);
                }
            } else {
                push(if i == 0 { None } else { Some(i - 1) }, 1);
            }
        }
        let inf = if rel.inf >= 0 { rel.inf as u64 % r } else { r - ((-rel.inf) as u64 % r) };
        push(Some(inf_col), inf);
        // merge repeated columns, drop zeros: the canonical form used for de-duplication
        row.sort();
        let mut merged: Vec<(usize, u64)> = Vec::new();
        for (c, v) in row {
            match merged.last_mut() {
                Some(last) if last.0 == c => last.1 = (last.1 + v) % r,
                _ => merged.push((c, v)),
            }
        }
        merged.retain(|e| e.1 != 0);
        merged
    };
    // ---- relation collection + linear algebra -------------------------------
    let t = Instant::now();
    let mut st = Stats::default();
    let mut qst = Stats::default();
    let mut sym_rows = 0u64;
    let mut quad_rows = 0u64;
    let mut dup_found = 0u64;
    let mut seen: std::collections::HashSet<Vec<(usize, u64)>> = std::collections::HashSet::new();
    let mut window: Vec<bool> = Vec::new();
    let mut use_quadrics = false;
    let mut switch_at_rows = usize::MAX;
    let mut rows: Vec<Vec<(usize, u64)>> = Vec::new();
    let mut partials: std::collections::HashMap<usize, Vec<(usize, u64)>> = std::collections::HashMap::new();
    let mut lp_combined = 0u64;
    let mut lp_dropped = 0u64;
    let mut target = ncols + 5;
    // HUMBERT_LA_PRECHECK=1: never start a Wiedemann solve while a column is empty
    let la_precheck = std::env::var("HUMBERT_LA_PRECHECK").map(|v| v == "1").unwrap_or(false);
    let mut precheck_extensions = 0u64;
    let mut t_rel = 0.0;
    let mut t_la = 0.0;
    let mut la_mulmods = 0u64;
    let mut la_rounds = 0;
    let sol;
    loop {
        let tr = Instant::now();
        while rows.len() < target {
            let rel = if use_quadrics {
                quadric_relation(&s, &fb, &nodes, &mut rng, &mut qst)
            } else {
                hyperplane_relation(&s, &fb, &nodes, &mut rng, &mut st)
            };
            let Some(rel) = rel else { continue };
            let big = if iota { to_big(&rel) } else { Vec::new() };
            let row = if iota { big.clone() } else { to_row(&rel) };
            let fresh = seen.insert(row.clone());
            if !use_quadrics {
                window.push(fresh);
                if window.len() >= 200 {
                    let dup = window.iter().filter(|f| !**f).count();
                    if dup * 2 > window.len() {
                        use_quadrics = true;
                        switch_at_rows = rows.len();
                    }
                    window.clear();
                }
            }
            if !fresh {
                dup_found += 1;
                continue;
            }
            let row = if iota {
                let lps: Vec<(usize, u64)> = big.iter().cloned().filter(|&(k, _)| k < nx && !is_small[k]).collect();
                match lps.len() {
                    0 => compact(&big),
                    1 => {
                        let (j, a) = lps[0];
                        match partials.get(&j) {
                            None => {
                                partials.insert(j, big.clone());
                                continue;
                            }
                            Some(first) => {
                                // new * b - first * a cancels Y_j
                                let b = first.iter().find(|e| e.0 == j).unwrap().1;
                                let mut comb: Vec<(usize, u64)> = big.iter().map(|&(k, v)| (k, mulr(v, b, r))).collect();
                                comb.extend(first.iter().map(|&(k, v)| (k, (r - mulr(v, a, r)) % r)));
                                let comb = merge(comb);
                                debug_assert!(comb.iter().all(|e| e.0 >= nx || is_small[e.0]));
                                lp_combined += 1;
                                let c = compact(&comb);
                                if c.is_empty() {
                                    continue;
                                }
                                c
                            }
                        }
                    }
                    _ => {
                        lp_dropped += 1;
                        continue;
                    }
                }
            } else {
                row
            };
            sym_rows += rel.sym as u64;
            if use_quadrics {
                quad_rows += 1;
            }
            rows.push(row);
        }
        t_rel += tr.elapsed().as_secs_f64();
        if la_precheck && ncols > DENSE_MAX {
            // columns in no relation are certain extra corank: collect more relations
            // before paying for a Wiedemann solve that is bound to be rejected
            let mut used = vec![false; ncols];
            for row in &rows {
                for &(c, _) in row {
                    used[c] = true;
                }
            }
            let empty = used.iter().filter(|u| !**u).count();
            if empty > 0 {
                precheck_extensions += 1;
                target += (2 * empty).max(5);
                continue;
            }
        }
        let tl = Instant::now();

        let solve = if ncols <= DENSE_MAX {
            la::kernel_mod_r(&rows, ncols, r)
        } else {
            // two independent Wiedemann solves; a corank > 1 shows up as disagreement
            let a = la::kernel_wiedemann(&rows, ncols, r, seed ^ 0xA5A5 ^ la_rounds as u64);
            let b = la::kernel_wiedemann(&rows, ncols, r, seed ^ 0x5A5A ^ (la_rounds as u64) << 8);
            let same = match (&a.x, &b.x) {
                (Some(xa), Some(xb)) => {
                    let ia = invr(xa[inf_col], r);
                    let ib = invr(xb[inf_col], r);
                    xa.iter().zip(xb).all(|(u, v)| mulr(*u, ia, r) == mulr(*v, ib, r))
                }
                _ => false,
            };
            // a column in no relation is an undetermined unknown that Krylov methods
            // silently set to 0 under every scaling: count it as extra corank
            let mut used = vec![false; ncols];
            for row in &rows {
                for &(c, _) in row {
                    used[c] = true;
                }
            }
            let empty = used.iter().filter(|u| !**u).count();
            la::Solve { kernel_dim: if same && empty == 0 { 1 } else { 1 + empty.max(1) }, x: a.x, rank: a.rank, mulmods: a.mulmods + b.mulmods }
        };

        t_la += tl.elapsed().as_secs_f64();
        la_mulmods += solve.mulmods;
        la_rounds += 1;
        if solve.kernel_dim == 1 {
            sol = solve;
            break;
        }
        target += (ncols / 20).max(5);
    }
    let k4_after_rel = s.k().muls.get();
    let _ = t;
    let xv = sol.x.unwrap();
    // Y_j for every x with a known log (iota/lp); None where undetermined
    let mut yv: Vec<Option<u64>> = vec![None; nx];
    if iota {
        for j in 0..nx {
            if j == pinned {
                yv[j] = Some(0);
            } else if let Some(c) = small_col[j] {
                yv[j] = Some(xv[c]);
            }
        }
        let big_val = |k: usize, yv: &Vec<Option<u64>>| -> u64 {
            if k == big_c {
                xv[c_col]
            } else if k == big_inf {
                xv[inf_col]
            } else {
                yv[k].unwrap()
            }
        };
        for (&j, row) in &partials {
            let mut acc = 0u64;
            let mut coef = 0u64;
            for &(k, v) in row {
                if k == j {
                    coef = v;
                } else {
                    acc = (acc + mulr(v, big_val(k, &yv), r)) % r;
                }
            }
            yv[j] = Some(mulr((r - acc) % r, invr(coef, r), r));
        }
    }
    let lp_known = (0..nx).filter(|&j| !is_small[j] && yv[j].is_some()).count();
    let val = |i: usize| -> u64 {
        if iota {
            let y = yv[i / 2].expect("log of a point outside the known set");
            if i % 2 == 0 {
                y
            } else {
                (xv[c_col] + r - y) % r
            }
        } else if i == 0 {
            0
        } else {
            xv[i - 1]
        }
    };
    let known = |i: usize| -> bool { !iota || yv[i / 2].is_some() };
    let x_inf = xv[inf_col];
    // ---- lead 4 checks: iota pairs, X_oo = 4c ------------------------------
    // c from the pinned point: always in the small base, so its log is known
    let c_val = (val(2 * pinned) + val(2 * pinned + 1)) % r;
    let mut iota_pairs_const = true;
    for j in (0..nx).filter(|&j| known(2 * j)) {
        if (val(2 * j) + val(2 * j + 1)) % r != c_val {
            iota_pairs_const = false;
        }
    }
    let xinf_eq_4c = x_inf == mulr(4, c_val, r);
    // ---- transfer + individual logs ----------------------------------------
    let t = Instant::now();
    let mut dst = Stats::default();
    let mut place_degs = Vec::new();
    let mut push_ok = true;
    let mut logs = Vec::new();
    let mut multipliers = Vec::new();
    for base in [gen, q] {
        // smallest m with every F_q-place of T(mP) of degree 4 (degree-8 places leave
        // only 3 free points in the quadric descent: too few candidates at toy q)
        let mut m = 1u64;
        let (pt, places) = loop {
            let mp = ce.mul(&base, m as u128).unwrap();
            let pl = transfer_places(&s, mp);
            if pl.iter().all(|z| z.len() == 4) || m == 64 {
                break (mp, pl);
            }
            m += 1;
        };
        multipliers.push(m);
        push_ok &= pushforward_check(&s, &places, pt);
        let mut lg = (r - mulr(4, x_inf, r)) % r;
        for z in &places {
            place_degs.push(z.len());
            let pts = descend_place(&s, &fb, &nodes, z, &known, &mut rng, &mut dst);
            let mut lz = mulr(2, x_inf, r);
            for &i in &pts {
                lz = (lz + r - val(i)) % r;
            }
            lg = (lg + lz) % r;
        }
        logs.push(mulr(lg, invr(m % r, r), r));
    }
    let t_desc = t.elapsed().as_secs_f64();
    let k4_after_desc = s.k().muls.get();
    let (lp, lq) = (logs[0], logs[1]);
    let tp_nonzero = lp != 0;
    let x_rec = if tp_nonzero { mulr(lq, invr(lp, r), r) } else { 0 };
    let solved = x_rec == x_true;
    let t_ic = t_all.elapsed().as_secs_f64();
    let rss_ic = peak_rss_kb();
    // ---- matched rho on E(F_{q^4})[r] --------------------------------------
    // HUMBERT_SKIP_RHO=1: no rho run (the instance's rho is measured once, in its baseline arm)
    let skip_rho = std::env::var("HUMBERT_SKIP_RHO").map(|v| v == "1").unwrap_or(false);
    let rho = if skip_rho {
        curve::RhoResult { log: None, steps: 1, seconds: 0.0, k4_muls: 0 }
    } else {
        pollard_rho(&ce, &gen, &q, r, &mut rng)
    };
    let rho_ok = rho.log == Some(x_true);
    let sec_per_step = rho.seconds / rho.steps as f64;
    let expected_rho_steps = (std::f64::consts::PI * r as f64 / 2.0).sqrt();
    println!(
        "{{\"kind\":\"attack\",\"p\":{p},\"twist\":{twist},\"seed\":{seed},\"arm\":\"{arm}\",\"E_order\":{},\"r\":{r},\"D_order\":{},\
\"fb_x\":{nx},\"fb_points\":{nfb},\"small_x\":{nsmall},\"lp_alpha\":{},\"lp_partials\":{},\"lp_combined\":{lp_combined},\"lp_dropped\":{lp_dropped},\"lp_known\":{lp_known},\"unknowns\":{ncols},\
\"rel_candidates\":{},\"rel_degenerate\":{},\"rel_symmetric_found\":{},\"rel_symmetric_rows\":{sym_rows},\"rel_nondeg_found\":{},\"residual_root_hist\":{:?},\"relations\":{},\"duplicate_relations\":{dup_found},\"quadric_switch_at_rows\":{},\"quad_candidates\":{},\"quad_found\":{},\"quad_rows\":{quad_rows},\"la_rounds\":{la_rounds},\"la_precheck\":{la_precheck},\"precheck_extensions\":{precheck_extensions},\"la_method\":\"{}\",\"rank\":{},\"la_mulmods\":{la_mulmods},\
\"target_multipliers\":{:?},\"place_degrees\":{:?},\"descent_candidates\":{},\"descent_relations\":{},\
\"pushforward_8P\":{push_ok},\"T_P_nonzero\":{tp_nonzero},\"iota_pairs_constant\":{iota_pairs_const},\"x_inf_eq_4c\":{xinf_eq_4c},\
\"x_true\":{x_true},\"x_recovered\":{x_rec},\"solved\":{solved},\
\"t_setup\":{t_setup:.4},\"t_relations\":{t_rel:.4},\"t_linear_algebra\":{t_la:.4},\"t_descent\":{t_desc:.4},\"t_ic_total\":{t_ic:.4},\
\"k4_muls_relations\":{},\"k4_muls_descent\":{},\"peak_rss_kb\":{rss_ic},\
\"rho_skipped\":{skip_rho},\"rho_solved\":{rho_ok},\"rho_steps\":{},\"rho_seconds\":{:.4},\"rho_expected_steps\":{expected_rho_steps:.1},\"rho_expected_steps_negation\":{:.1},\"rho_k4_muls\":{},\
\"ic_over_rho_time\":{:.4},\"ic_time_in_rho_steps\":{:.1}}}",
        s.order, s.d_count, lp_alpha.unwrap_or(1.0), partials.len(), st.candidates, st.degenerate, st.symmetric, st.found, st.root_hist, rows.len(), if switch_at_rows == usize::MAX { -1 } else { switch_at_rows as i64 }, qst.candidates, qst.found, if ncols <= DENSE_MAX { "dense" } else { "wiedemann" }, sol.rank, multipliers, place_degs, dst.candidates, dst.found,
        k4_after_rel - k4_after_setup, k4_after_desc - k4_after_rel,
        rho.steps, rho.seconds, expected_rho_steps / std::f64::consts::SQRT_2, rho.k4_muls, if skip_rho { 0.0 } else { t_ic / rho.seconds }, if skip_rho { 0.0 } else { t_ic / sec_per_step }
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldOps;

    #[test]
    fn sqrt_in_l_of_every_k4_element_sample() {
        let mut rng = Rng(5);
        let s = Setup::new(17, -1, &mut rng);
        for _ in 0..200 {
            let a = [rng.below(17), rng.below(17), rng.below(17), rng.below(17)];
            let r = s.l.sqrt(&s.l.emb(&a)).expect("every K4 element is a square in L");
            assert_eq!(s.l.mul(&r, &r), s.l.emb(&a));
        }
    }

    #[test]
    fn lpoly_of_model_equals_res_e_times_d_p17_both_twists() {
        for tw in [1, -1] {
            let mut rng = Rng(9);
            let s = Setup::new(17, tw, &mut rng);
            let lc = lpoly_from_counts(17, &count_c(&s, 5, &mut rng), 5);
            let pi = 17i128;
            let a_e = pi.pow(4) + 1 - s.order as i128;
            let a_d = pi + 1 - s.d_count as i128;
            let mut le4 = vec![0i128; 9];
            le4[0] = 1;
            le4[4] = -a_e;
            le4[8] = pi.pow(4);
            assert_eq!(poly_mul_i(&le4, &[1, -a_d, pi]), lc);
        }
    }

    #[test]
    fn wiedemann_matches_dense_kernel() {
        // random corank-1 system: rows orthogonal to a hidden vector
        let r = 1_000_003u64;
        let mut rng = Rng(3);
        let n = 30;
        let hidden: Vec<u64> = (0..n).map(|_| 1 + rng.below(r - 1)).collect();
        let mut rows = Vec::new();
        for _ in 0..(n + 5) {
            let mut row: Vec<(usize, u64)> = Vec::new();
            let mut cols: Vec<usize> = Vec::new();
            while cols.len() < 5 {
                let c = rng.below(n as u64 - 1) as usize;
                if !cols.contains(&c) {
                    cols.push(c);
                }
            }
            let mut acc = 0u64;
            for &c in &cols {
                let a = 1 + rng.below(r - 1);
                acc = (acc + mulr(a, hidden[c], r)) % r;
                row.push((c, a));
            }
            // last column balances the row
            row.push((n - 1, mulr((r - acc) % r, invr(hidden[n - 1], r), r)));
            rows.push(row);
        }
        let d = la::kernel_mod_r(&rows, n, r);
        assert_eq!(d.kernel_dim, 1);
        let w = la::kernel_wiedemann(&rows, n, r, 11);
        let (xd, xw) = (d.x.unwrap(), w.x.unwrap());
        let (id, iw) = (invr(xd[0], r), invr(xw[0], r));
        for c in 0..n {
            assert_eq!(mulr(xd[c], id, r), mulr(xw[c], iw, r));
            assert_eq!(mulr(xd[c], id, r), mulr(hidden[c], invr(hidden[0], r), r));
        }
    }

    #[test]
    fn summarize_lp_parses_records_and_rejects_duplicates() {
        let r = lpsummary::Rec::parse(r#"{"tag":"pre","order":2,"p":4001,"place_degrees":[4, 4],"solved":true,"t_ic_total":5.25}"#).unwrap();
        assert_eq!((r.s("tag"), r.u("p"), r.b("solved"), r.f("t_ic_total")), ("pre".to_string(), 4001, true, 5.25));
        let dir = std::env::temp_dir().join(format!("humbert-dup-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("results")).unwrap();
        std::fs::write(dir.join("results/v2.jsonl"), "{\"tag\":\"pre\",\"p\":1,\"seed\":1}\n{\"tag\":\"pre\",\"p\":1,\"seed\":1}\n").unwrap();
        let e = lpsummary::run(dir.to_str().unwrap()).unwrap_err();
        assert!(e.contains("duplicate record"), "{e}");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn classifier_contains_target_and_kani_rosen_agrees() {
        let rows = classify::enumerate(4, 1);
        assert!(rows.iter().all(|r| r.g == r.g_kani_rosen));
        let t: Vec<_> = rows.iter().filter(|r| r.target).collect();
        assert_eq!(t.len(), 2);
        for r in t {
            assert_eq!((r.m, r.s, r.g, r.e, r.hyperelliptic), (4, 5, 5, 1, false));
        }
        assert!(rows.iter().filter(|r| r.conj_distinct).all(|r| r.e >= 1), "no e = 0 family for d = 4, g0 = 1");
    }
}
