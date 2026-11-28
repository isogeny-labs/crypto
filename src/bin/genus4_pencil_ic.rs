//! Genus-4 non-hyperelliptic curves (smooth (3,3) curves on P1xP1): Pic^0
//! arithmetic by linear algebra, trigonal-pencil and plane-quintic relation
//! yields, a small index-calculus DLP with sparse (Wiedemann) linear algebra,
//! and a matched rho on the same arithmetic.  Local development check only.
//!
//! Subcommands: `yield P K SEED OUT`, `dlp P SEED RHO_RUNS OUT`, `bench P SEED`.
#[path = "../../research/low-gonality-genus4_20261115/src/g4.rs"]
mod g4;
#[path = "../../research/low-gonality-genus4_20261115/src/ic.rs"]
mod ic;

use g4::*;
use ic::*;
use std::fmt::Write as _;
use std::time::Instant;

fn random_curve(p: u32, rng: &mut u64) -> Curve {
    loop {
        let mut g = [0u32; 16];
        for x in g.iter_mut() {
            *x = rnd(rng, p as u64) as u32;
        }
        if let Ok(mut c) = Curve::new(p, g) {
            if c.set_base(rng).is_ok() {
                return c;
            }
        }
    }
}

fn jarr<T: std::fmt::Display>(v: &[T]) -> String {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(","))
}

fn cmd_yield(args: &[String]) {
    let p: u32 = args[0].parse().unwrap();
    let k: usize = args[1].parse().unwrap();
    let seed: u64 = args[2].parse().unwrap();
    let outf = &args[3];
    let phis = [0.3f64, 0.5, 0.7, 1.0];
    let mut out = String::new();
    for ci in 0..k {
        let mut rng = seed
            .wrapping_mul(1_000_003)
            .wrapping_add(ci as u64 * 7919 + 1)
            | 1;
        let cseed = rng;
        let c = random_curve(p, &mut rng);
        let n = c.pts.len();
        let (h1, h2, fibres) = fibre_stats(&c);
        let ev1 = ev1_table(&c);
        let mut rows = String::new();
        for (pi, &phi) in phis.iter().enumerate() {
            let m = ((phi * n as f64).ceil() as usize).clamp(4, n);
            let mut in_f = vec![false; n];
            for &d in &c.d0 {
                in_f[d] = true;
            }
            let mut cnt = 4;
            while cnt < m {
                let i = rnd(&mut rng, n as u64) as usize;
                if !in_f[i] {
                    in_f[i] = true;
                    cnt += 1;
                }
            }
            let fidx: Vec<usize> = (0..n).filter(|&i| in_f[i]).collect();
            let lay = make_layout(&c, &fidx);
            let rs = collect(&c, &lay, &ev1, true, 0, &mut rng);
            if pi > 0 {
                rows.push(',');
            }
            let _ = write!(
                rows,
                "{{\"phi\":{phi},\"m\":{m},\"pairs\":{},\"hits_a\":{},\"unique_a\":{},\"fibres_split3\":{},\"fibres_full_in_f\":{},\"native\":{}}}",
                rs.st.pairs, rs.st.hits_a, rs.st.unique_a, rs.st.fibres_split3, rs.st.fibres_full_in_f, rs.st.native
            );
        }
        let _ = writeln!(
            out,
            "{{\"p\":{p},\"curve\":{ci},\"seed\":{cseed},\"g\":{},\"npts\":{n},\"fibres\":{fibres},\"fib_hist_pencil1\":{},\"fib_hist_pencil2\":{},\"cells\":[{rows}]}}",
            jarr(&c.g),
            jarr(&h1),
            jarr(&h2)
        );
    }
    std::fs::write(outf, out).unwrap();
}

fn mulmod_ns(p: u64) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..5 {
        let t = Instant::now();
        let mut acc = 1u64;
        let mut x = 12345u64;
        let n = 20_000_000u64;
        for i in 0..n {
            x = (x * 7 + i) % p;
            acc = (acc + x * (acc | 3)) % p;
        }
        std::hint::black_box(acc);
        let dt = t.elapsed().as_nanos() as f64 / (2 * n) as f64;
        best = best.min(dt);
    }
    best
}
fn mulmod_ell_ns(ell: u64) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..5 {
        let t = Instant::now();
        let mut acc = 1u64;
        let n = 5_000_000u64;
        for i in 0..n {
            acc = mulmod(acc | 1, i + 3, ell);
        }
        std::hint::black_box(acc);
        best = best.min(t.elapsed().as_nanos() as f64 / n as f64);
    }
    best
}

fn cmd_dlp(args: &[String]) {
    let p: u32 = args[0].parse().unwrap();
    let seed: u64 = args[1].parse().unwrap();
    let rho_runs: usize = args[2].parse().unwrap();
    let outf = &args[3];
    let t_all = Instant::now();
    let mut chosen = None;
    for trial in 0..80u64 {
        let mut rng = seed
            .wrapping_mul(2_654_435_761)
            .wrapping_add(trial * 104_729 + 3)
            | 1;
        let cseed = rng;
        let c = random_curve(p, &mut rng);
        if c.pts.len() < (p as usize) / 2 {
            continue;
        }
        let Ok(nn) = order_bsgs(&c, &mut rng) else {
            continue;
        };
        let nn = nn as u64;
        let fac = factor(nn);
        let (ell, e) = *fac.iter().max_by_key(|x| x.0).unwrap();
        if e != 1 || ell.saturating_mul(16) < nn {
            eprintln!("trial {trial}: N={nn} factors={fac:?} rejected");
            continue;
        }
        chosen = Some((c, rng, cseed, trial, nn, ell));
        break;
    }
    let Some((c, mut rng, cseed, trial, nn, ell)) = chosen else {
        std::fs::write(outf, "{\"error\":\"no suitable curve\"}\n").unwrap();
        return;
    };
    let fac = factor(nn);
    let ev1 = ev1_table(&c);
    // g of order ell, h = x g
    let g = loop {
        let x0 = random_elt(&c, &mut rng);
        let g = c.mul_scalar(&x0, (nn / ell) as u128);
        if !c.is_zero(&g) {
            break g;
        }
    };
    assert!(
        c.is_zero(&c.mul_scalar(&g, ell as u128)),
        "g has order dividing ell"
    );
    let x_true = 1 + rnd(&mut rng, ell - 1);
    let h = c.mul_scalar(&g, x_true as u128);
    // calibration
    let t0 = Instant::now();
    let nadd = 60u64;
    let mut acc = g.clone();
    let ops0 = c.n_ops.get();
    let inner0 = c.inner.get();
    for _ in 0..nadd {
        acc = c.add(&acc, &h);
    }
    let t_add = t0.elapsed().as_nanos() as f64 / nadd as f64;
    let mulmods_per_add = (c.inner.get() - inner0) as f64 / (c.n_ops.get() - ops0) as f64;
    let t_mul_p = mulmod_ns(p as u64);
    let t_mul_ell = mulmod_ell_ns(ell);
    let w_cal_p = t_mul_p / t_add;
    let w_cal_ell = t_mul_ell / t_add;
    // index calculus ladder, run twice (second pass = A/A with fresh randomness)
    let mut pass_json: Vec<String> = vec![];
    let mut pass_final: Vec<String> = vec![];
    let mut pass_sums: Vec<(u64, u64, u64)> = vec![];
    let mut ic_secs = 0.0f64;
    let ladder: Vec<(f64, f64)> = vec![
        (0.4, 0.0),
        (0.5, 0.0),
        (0.6, 0.0),
        (0.7, 0.0),
        (0.8, 0.0),
        (0.9, 0.0),
        (1.0, 0.0),
        (1.0, 4.0),
    ];
    for _pass in 0..2 {
        let t_ic = Instant::now();
        let mut ic_json = vec![];
        let mut fin = "{\"ok\":false}".to_string();
        let (mut sum_group, mut sum_np, mut sum_nl) = (0u64, 0u64, 0u64);
        for &(phi, cb) in &ladder {
            let o = ic_attempt(&c, &ev1, ell, &g, &h, phi, cb, &mut rng);
            sum_group += o.group_ops;
            sum_np += o.native_p;
            sum_nl += o.native_ell;
            let js = format!(
                "{{\"phi\":{},\"c_budget_factor\":{cb},\"m\":{},\"ok\":{},\"reason\":\"{}\",\"x\":{},\"rows\":{},\"ncols_used\":{},\"unique_a\":{},\"hits_a\":{},\"pairs\":{},\"unique_c\":{},\"triples\":{},\"fibre_rows\":{},\"kernel_dim\":{},\"la_ops\":{},\"la_matvecs\":{},\"la_attempts\":{},\"group_ops\":{},\"native_p\":{},\"native_ell\":{},\"indiv_steps\":{},\"indiv_records\":{}}}",
                o.phi, o.m, o.ok, o.reason, o.x.map_or("null".to_string(), |v| v.to_string()), o.rows, o.ncols_used, o.unique_a, o.hits_a, o.pairs, o.unique_c, o.triples, o.fibre_rows, o.kernel_dim, o.la_ops, o.la_matvecs, o.la_attempts, o.group_ops, o.native_p, o.native_ell, o.indiv_steps, o.indiv_records
            );
            eprintln!("ic phi={phi} cb={cb}: ok={} {}", o.ok, o.reason);
            ic_json.push(js);
            if o.ok {
                fin = format!("{{\"ok\":true,\"phi\":{},\"c_budget_factor\":{cb},\"m\":{},\"group_ops\":{},\"native_p\":{},\"native_ell\":{}}}", o.phi, o.m, o.group_ops, o.native_p, o.native_ell);
                break;
            }
        }
        ic_secs += t_ic.elapsed().as_secs_f64();
        pass_json.push(format!("[{}]", ic_json.join(",")));
        pass_final.push(fin);
        pass_sums.push((sum_group, sum_np, sum_nl));
    }
    let enum_native = (p as u64 + 1) * (16 + 3 * p as u64 + 3);
    // rho
    let t_rho = Instant::now();
    let mut rho_json = vec![];
    let dbits = (((ell as f64).sqrt().log2() - 4.0).floor().max(2.0)) as u32;
    for r in 0..rho_runs {
        let ops_before = c.n_ops.get();
        let mut rr = seed.wrapping_mul(97).wrapping_add(r as u64 * 31 + 5) | 1;
        let walk = make_adders(&c, &g, &h, 16, ell, &mut rr);
        let setup = c.n_ops.get() - ops_before;
        let o = rho(
            &c,
            &g,
            &h,
            ell,
            dbits,
            &walk,
            &mut rr,
            60 * ((ell as f64).sqrt() as u64 + 100),
        );
        let verified =
            o.x.is_some_and(|x| c.mul_scalar(&g, x as u128).key == h.key);
        rho_json.push(format!(
            "{{\"run\":{r},\"verified\":{verified},\"setup_adds\":{setup},\"walk_adds\":{},\"steps\":{},\"dps\":{},\"native\":{},\"restarts\":{}}}",
            o.adds, o.steps, o.dps, o.native, o.restarts
        ));
    }
    let rho_secs = t_rho.elapsed().as_secs_f64();
    let fac_json = format!(
        "[{}]",
        fac.iter()
            .map(|(q, e)| format!("[{q},{e}]"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let sums_json = pass_sums
        .iter()
        .map(|(a, b, d)| format!("{{\"group_ops\":{a},\"native_p\":{b},\"native_ell\":{d}}}"))
        .collect::<Vec<_>>()
        .join(",");
    let json = format!(
        "{{\"p\":{p},\"seed\":{seed},\"curve_seed\":{cseed},\"trial\":{trial},\"g\":{},\"npts\":{},\"N\":{nn},\"factors\":{},\"ell\":{ell},\"x_true\":{x_true},\"calib\":{{\"t_add_ns\":{t_add},\"mulmods_per_add\":{mulmods_per_add},\"t_mulmod_p_ns\":{t_mul_p},\"t_mulmod_ell_ns\":{t_mul_ell},\"w_cal_p\":{w_cal_p},\"w_cal_ell\":{w_cal_ell}}},\"enum_native_p\":{enum_native},\"ic_passes\":[{}],\"ic_final\":[{}],\"ic_sums_all_rungs\":[{sums_json}],\"ic_secs\":{ic_secs},\"dbits\":{dbits},\"rho_secs\":{rho_secs},\"rho\":[{}],\"total_secs\":{},\"fallbacks\":{},\"exceptional_keys\":{},\"keyfail\":{}}}\n",
        jarr(&c.g),
        c.pts.len(),
        fac_json,
        pass_json.join(","),
        pass_final.join(","),
        rho_json.join(","),
        t_all.elapsed().as_secs_f64(),
        c.n_fallback.get(),
        c.n_exc.get(),
        c.n_keyfail.get()
    );
    std::fs::write(outf, json).unwrap();
}

fn cmd_bench(args: &[String]) {
    let p: u32 = args[0].parse().unwrap();
    let mut rng: u64 = args[1].parse::<u64>().unwrap() | 1;
    let c = random_curve(p, &mut rng);
    let g = random_elt(&c, &mut rng);
    let h = random_elt(&c, &mut rng);
    let mut acc = g.clone();
    let t = Instant::now();
    let n0 = c.n_ops.get();
    let i0 = c.inner.get();
    for _ in 0..200 {
        acc = c.add(&acc, &h);
    }
    let el = t.elapsed();
    println!(
        "p={p} pts={} add: {:.1} us, {:.0} mulmods/add, fallbacks {}",
        c.pts.len(),
        el.as_micros() as f64 / 200.0,
        (c.inner.get() - i0) as f64 / (c.n_ops.get() - n0) as f64,
        c.n_fallback.get()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("yield") => cmd_yield(&args[2..]),
        Some("dlp") => cmd_dlp(&args[2..]),
        Some("bench") => cmd_bench(&args[2..]),
        _ => eprintln!(
            "usage: genus4_pencil_ic yield P K SEED OUT | dlp P SEED RHO_RUNS OUT | bench P SEED"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::g4::*;

    use super::{ic::*, random_curve};

    #[test]
    fn ring_dims() {
        let mut rng = 12345u64;
        let c = random_curve(11, &mut rng);
        let want = [1usize, 4, 9, 15, 21, 27, 33, 39, 45];
        assert_eq!(&c.dim[..], &want[..]);
        assert_eq!(c.zero().w.len(), 5);
    }

    #[test]
    fn group_law_p11() {
        let mut rng = 777u64;
        let c = random_curve(11, &mut rng);
        let n = c.pts.len();
        let pick = |rng: &mut u64| {
            let mut idx = vec![];
            while idx.len() < 4 {
                let i = rnd(rng, n as u64) as usize;
                if !idx.contains(&i) {
                    idx.push(i);
                }
            }
            c.elt_of_points(&idx)
        };
        for _ in 0..6 {
            let (x, y, z) = (pick(&mut rng), pick(&mut rng), pick(&mut rng));
            let xy = c.add(&x, &y);
            let yx = c.add(&y, &x);
            assert_eq!(xy.key, yx.key, "commutative");
            let l = c.add(&xy, &z);
            let r = c.add(&x, &c.add(&y, &z));
            assert_eq!(l.key, r.key, "associative");
            let nx = c.neg(&x);
            assert!(c.is_zero(&c.add(&x, &nx)), "inverse");
            assert_eq!(c.add(&x, c.zero()).key, x.key, "identity");
            assert_eq!(c.sub(&xy, &y).key, x.key, "sub");
        }
        eprintln!(
            "ops {} fallbacks {} exc {} keyfail {}",
            c.n_ops.get(),
            c.n_fallback.get(),
            c.n_exc.get(),
            c.n_keyfail.get()
        );
    }

    #[test]
    fn point_classes_additive_p13() {
        let mut rng = 4242u64;
        let c = random_curve(13, &mut rng);
        let n = c.pts.len();
        let cls: Vec<Elt> = (0..n).map(|i| c.point_class(i)).collect();
        for _ in 0..8 {
            let mut idx = vec![];
            while idx.len() < 4 {
                let i = rnd(&mut rng, n as u64) as usize;
                if !idx.contains(&i) {
                    idx.push(i);
                }
            }
            let direct = c.elt_of_points(&idx);
            let mut acc = c.zero().clone();
            for &i in &idx {
                acc = c.add(&acc, &cls[i]);
            }
            for &d in &c.d0 {
                acc = c.sub(&acc, &cls[d]);
            }
            assert_eq!(acc.key, direct.key);
        }
    }

    fn zeta_n(p: u32, g: &[u32; 16], rng: &mut u64) -> i128 {
        let mut ns = vec![];
        for e in 1..=4 {
            let f = Ext::new(p as u64, e, rng);
            ns.push(count_points(g, &f) as i128);
        }
        let a = l_coeffs(p as i128, &ns);
        order_from_l(p as i128, &a)
    }

    #[test]
    fn order_kills_random_points_p7_p11() {
        for (p, seed) in [(7u32, 99u64), (11, 5)] {
            let mut rng = seed;
            let c = random_curve(p, &mut rng);
            let nn = zeta_n(p, &c.g, &mut rng);
            assert!(nn > 0);
            let n = c.pts.len();
            eprintln!("p={p} pts={n} N={nn} fallbacks={}", c.n_fallback.get());
            for _ in 0..6 {
                let mut idx = vec![];
                while idx.len() < 4 {
                    let i = rnd(&mut rng, n as u64) as usize;
                    if !idx.contains(&i) {
                        idx.push(i);
                    }
                }
                let x = c.elt_of_points(&idx);
                assert!(c.is_zero(&c.mul_scalar(&x, nn as u128)), "N x = 0");
                assert!(!c.is_zero(&c.mul_scalar(&x, nn as u128 + 1)) || c.is_zero(&x));
            }
        }
    }

    #[test]
    fn relations_are_true_in_pic_p31() {
        let mut rng = 2024u64;
        let c = random_curve(31, &mut rng);
        let n = c.pts.len();
        let cls: Vec<Elt> = (0..n).map(|i| c.point_class(i)).collect();
        let fidx: Vec<usize> = (0..n).collect();
        let lay = make_layout(&c, &fidx);
        let ev1 = ev1_table(&c);
        let rs = collect(&c, &lay, &ev1, true, 0, &mut rng);
        // classes of the columns
        let pa = c.d0[0];
        let col_class = |col: u32| -> Option<Elt> {
            let pt = fidx
                .iter()
                .copied()
                .filter(|&i| i != pa)
                .nth(col as usize)?;
            Some(cls[pt].clone())
        };
        // group relations by their special column; within a group, sums of point
        // classes must agree (all equal kappa / tau_k).
        let mut by_special: std::collections::HashMap<u32, Vec<Elt>> = Default::default();
        for r in &rs.rows {
            let sp = r.iter().find(|x| x.1 == -1 && x.0 as usize >= lay.nf);
            let Some(&(sc, _)) = sp else { continue };
            if r.len() == 3 && r.iter().all(|x| x.0 as usize >= lay.nf) {
                continue;
            }
            let mut acc = c.zero().clone();
            for &(col, v) in r {
                if v == 1 {
                    acc = c.add(&acc, &col_class(col).unwrap());
                }
            }
            by_special.entry(sc).or_default().push(acc);
        }
        let mut checked = 0;
        for v in by_special.values() {
            for w in v.windows(2) {
                assert_eq!(w[0].key, w[1].key, "two relations of one type disagree");
                checked += 1;
            }
        }
        eprintln!(
            "rows {} checked pairs {} unique_a {} hits_a {}",
            rs.rows.len(),
            checked,
            rs.st.unique_a,
            rs.st.hits_a
        );
        assert!(checked >= 3);
        // kappa = tau1 + tau2
        let kap = by_special
            .get(&lay.kappa())
            .and_then(|v| v.first())
            .cloned();
        let t1 = by_special.get(&lay.tau(0)).and_then(|v| v.first()).cloned();
        let t2 = by_special.get(&lay.tau(1)).and_then(|v| v.first()).cloned();
        if let (Some(k), Some(a), Some(b)) = (kap, t1, t2) {
            assert_eq!(k.key, c.add(&a, &b).key, "K = T1 + T2");
        }
    }

    #[test]
    fn wiedemann_matches_dense() {
        let ell = 1_000_003u64;
        let mut rng = 5u64;
        let n = 60usize;
        // every row is satisfied by the all-ones vector; the chain makes the kernel
        // exactly the all-ones line
        let mut rows: Vec<Row> = (0..n - 1)
            .map(|i| vec![(i as u32, 1), (i as u32 + 1, -1)])
            .collect();
        for _ in 0..4 * n {
            let mut v = [0usize; 4];
            for x in v.iter_mut() {
                *x = rnd(&mut rng, n as u64) as usize;
            }
            if v[0] == v[2] || v[0] == v[3] || v[1] == v[2] || v[1] == v[3] {
                continue;
            }
            rows.push(vec![
                (v[0] as u32, 1),
                (v[1] as u32, 1),
                (v[2] as u32, -1),
                (v[3] as u32, -1),
            ]);
        }
        let (kd, dv) = dense_kernel(&rows, n, ell);
        assert_eq!(kd, 1);
        let la = wiedemann_kernel(&rows, n, ell, &mut rng).expect("wiedemann");
        let sc = mulmod(la.vec[0], invmod(dv[0], ell).unwrap(), ell);
        for i in 0..n {
            assert_eq!(mulmod(sc, dv[i], ell), la.vec[i]);
            assert_eq!(la.vec[i], la.vec[0]);
        }
    }

    #[test]
    fn ic_and_rho_small_p31() {
        let mut rng = 31u64 * 77 + 1;
        let c = random_curve(31, &mut rng);
        let nn = order_bsgs(&c, &mut rng).expect("order") as u64;
        let fac = factor(nn);
        let (ell, _) = *fac.iter().max_by_key(|x| x.0).unwrap();
        eprintln!("N={nn} factors={fac:?}");
        let g = loop {
            let x0 = random_elt(&c, &mut rng);
            let g = c.mul_scalar(&x0, (nn / ell) as u128);
            if !c.is_zero(&g) {
                break g;
            }
        };
        assert!(c.is_zero(&c.mul_scalar(&g, ell as u128)));
        let x = 1 + rnd(&mut rng, ell - 1);
        let h = c.mul_scalar(&g, x as u128);
        let walk = make_adders(&c, &g, &h, 16, ell, &mut rng);
        let o = rho(&c, &g, &h, ell, 2, &walk, &mut rng, 200_000);
        let rx = o.x.expect("rho found");
        assert!(c.mul_scalar(&g, rx as u128).key == h.key);
        eprintln!("rho adds {} ell {ell}", o.adds);
    }

    #[test]
    fn exceptional_classes_have_canonical_keys_p13() {
        // four coplanar rational points give h^0(D) = 2; any other effective
        // divisor of the class must produce the same key
        let mut rng = 99u64;
        let mut found = 0;
        for _ in 0..30 {
            let c = random_curve(13, &mut rng);
            let n = c.pts.len();
            let ev1 = ev1_table(&c);
            let p = 13u64;
            let cls: Vec<Elt> = (0..n).map(|i| c.point_class(i)).collect();
            'search: for a in 0..n {
                for b in a + 1..n {
                    for d in b + 1..n {
                        // plane through a, b, d: kernel of the 3x4 evaluation matrix
                        let cols: Vec<Vec<u32>> = (0..4)
                            .map(|k| vec![ev1[a][k] as u32, ev1[b][k] as u32, ev1[d][k] as u32])
                            .collect();
                        let ker = kernel(p, cols, &c.cnt);
                        if ker.len() != 1 {
                            continue;
                        }
                        let l = &ker[0];
                        let zs: Vec<usize> = (0..n)
                            .filter(|&q| {
                                q != a
                                    && q != b
                                    && q != d
                                    && (0..4).map(|k| l[k] as u64 * ev1[q][k]).sum::<u64>() % p == 0
                            })
                            .collect();
                        if zs.is_empty() {
                            continue;
                        }
                        let quad = [a, b, d, zs[0]];
                        let direct = c.elt_of_points(&quad);
                        assert_eq!(direct.key[0], 1, "coplanar quadruple is exceptional");
                        let mut acc = c.zero().clone();
                        for &i in &quad {
                            acc = c.add(&acc, &cls[i]);
                        }
                        for &x in &c.d0 {
                            acc = c.sub(&acc, &cls[x]);
                        }
                        assert_eq!(acc.key, direct.key, "key independent of the representative");
                        // group law with an exceptional operand
                        let y = random_elt(&c, &mut rng);
                        let z = random_elt(&c, &mut rng);
                        assert_eq!(
                            c.add(&c.add(&direct, &y), &z).key,
                            c.add(&direct, &c.add(&y, &z)).key
                        );
                        assert!(c.is_zero(&c.add(&direct, &c.neg(&direct))));
                        found += 1;
                        break 'search;
                    }
                }
            }
            if found >= 3 {
                break;
            }
        }
        assert!(found >= 1, "no coplanar quadruple found");
    }
}
