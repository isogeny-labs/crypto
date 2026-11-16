//! Subcommand tiny (correctness tier).
use crate::curves::{gen_curve, zeta_of, Family};
use crate::enumerate::{orbits_of, Orbits};
use crate::fp::Cnt;
use crate::jac::{factorize, jac_order};
use crate::la::{build_matrix, rank_and_kernel, wiedemann_count, Modulus, ELL, LAMBDA_PROXY};
use crate::lines::{n_cols_for, run_search, Arm, Row, SearchInput, SearchResult};
use crate::picrho::{check_t5, random_class, Rho};
use crate::plane_quartic_pic::{PicKey, PlaneQuartic};
use crate::{append_line, build_ctx, jb, jopt, jopt_b, verify_rows, Ctx};
use std::collections::HashMap;

fn search(ctx: &Ctx, arm: Arm, cs: u64, orb: Option<&Orbits>, rows_target: usize, cap: u64) -> SearchResult {
    let inp = SearchInput {
        arm,
        p: ctx.curve.p,
        f: &ctx.curve.f,
        pts: &ctx.pts,
        idx: &ctx.idx,
        p0: ctx.curve.p0,
        orbits: orb,
        curve_seed: cs,
        solver_seed: 1,
        rows_target,
        cap,
    };
    run_search(&inp)
}

fn dense(rows: &[Row], ncols: usize, r: u64, lam: u64) -> Vec<Vec<u64>> {
    let md = Modulus::new(r);
    let lam2 = md.mul(lam, lam);
    rows.iter()
        .map(|row| {
            let mut v = vec![0u64; ncols];
            match row {
                Row::Full(e) => {
                    for &(c, k) in e.iter() {
                        let kk = if k >= 0 { k as u64 % r } else { (r - (-k) as u64 % r) % r };
                        v[c as usize] = md.add(v[c as usize], kk);
                    }
                }
                Row::Orb(e) => {
                    for &(c, t) in e.iter() {
                        let x = md.add(
                            md.add(t[0] as u64 % r, md.mul(t[1] as u64, lam)),
                            md.mul(t[2] as u64, lam2),
                        );
                        v[c as usize] = md.add(v[c as usize], x);
                    }
                }
            }
            v
        })
        .collect()
}

/// status: 2 determined, 1 underdetermined, 0 fail. returns (status, kerdim)
fn classify(rows: &[Row], ncols: usize, r: u64, lam: u64, l: &[u64], t7a: bool) -> (u8, usize) {
    let (_rk, ker) = rank_and_kernel(&dense(rows, ncols, r, lam), ncols, r);
    let md = Modulus::new(r);
    if ker.len() == 1 {
        let v = &ker[0];
        if let Some(j) = (0..ncols).find(|&j| l[j] % r != 0) {
            let sc = md.mul(v[j], md.inv(l[j] % r));
            let prop = (0..ncols).all(|i| v[i] == md.mul(sc, l[i] % r));
            if prop {
                return (2, 1);
            }
        }
        return (0, 1);
    }
    if ker.len() > 1 && t7a {
        return (1, ker.len());
    }
    (0, ker.len())
}

pub fn run(p: u32, out: &str) {
    // Picard
    let mut accepted = 0;
    let mut rejects = 0u64;
    for cs in 1..=200u64 {
        if accepted >= 2 {
            break;
        }
        let curve = gen_curve(Family::P, p, cs);
        let (a2, a1, a0) = curve.a.unwrap();
        let (n1, _n2, _n3, jac) = jac_order(p, a2, a1, a0);
        let fac = factorize(jac as u128);
        let (r, _) = *fac.last().unwrap();
        let r2 = fac.last().unwrap().1 >= 2;
        if !(r > p as u128 && r != 3 && !r2) {
            rejects += 1;
            continue;
        }
        accepted += 1;
        let r = r as u64;
        let jac = jac as u128;
        let h = jac / r as u128;
        let ctx = build_ctx(curve.clone());
        let pq = PlaneQuartic::new(p, curve.f, curve.p0).unwrap();
        let n_pts = ctx.pts.len();
        let zeta = zeta_of(p);
        let rho = Rho::new(p, zeta);
        let mut rng = 4242 + cs;
        // T1
        let mut t1 = n1 as usize == n_pts;
        let sp = (p as f64).sqrt();
        t1 &= (jac as f64) >= (sp - 1.0).powi(6) && (jac as f64) <= (sp + 1.0).powi(6);
        for _ in 0..8 {
            let x = random_class(&pq, &ctx.pts, &mut rng);
            t1 &= pq.is_zero(&pq.mul_scalar(&x, jac));
        }
        // G
        let g = loop {
            let x = random_class(&pq, &ctx.pts, &mut rng);
            let g = pq.mul_scalar(&x, h);
            if !pq.is_zero(&g) {
                break g;
            }
        };
        // T2 table
        let mut table: HashMap<PicKey, u64> = HashMap::new();
        let mut cur = pq.zero();
        for k in 0..r {
            table.insert(cur.key(), k);
            cur = pq.add(&cur, &g);
        }
        let t2 = pq.is_zero(&cur) && table.len() as u64 == r;
        // logs
        let mut logs: Vec<u64> = vec![0; n_pts];
        let mut logs_ok = true;
        for (i, pt) in ctx.pts.iter().enumerate() {
            if i == ctx.p0idx {
                continue;
            }
            let c = pq.mul_scalar(&pq.point_class(*pt), h);
            match table.get(&c.key()) {
                Some(&k) => logs[i] = k,
                None => logs_ok = false,
            }
        }
        let t2 = t2 && logs_ok;
        // T3
        let mut own = Cnt::default();
        let lam = *table.get(&rho.rho_class(&pq, &g, &mut own).key()).unwrap_or(&u64::MAX);
        let t3 = lam != u64::MAX && ((lam as u128 * lam as u128 + lam as u128 + 1) % r as u128 == 0);
        // T4
        let mut t4 = t3;
        for (i, pt) in ctx.pts.iter().enumerate() {
            if i == ctx.p0idx {
                continue;
            }
            let fixed = pt[2] == 0 && pt[0] == 1;
            if fixed {
                t4 &= logs[i] == 0;
            } else if t3 {
                let j = ctx.idx[&rho.rho_point(&pq, *pt)];
                t4 &= logs[j] as u128 == (lam as u128 * logs[i] as u128) % r as u128;
            }
        }
        // T5
        let t5 = check_t5(&pq, &rho, &ctx.pts, &mut rng, 20, 20);
        // relations
        let orb = orbits_of(&ctx.pts, &ctx.idx, zeta, p);
        let nf = n_cols_for(Arm::PFull, n_pts, None);
        let no = orb.n_orbits;
        let rt_f = 3 * nf + 20;
        let rt_o = 3 * no + 20;
        let rf = search(&ctx, Arm::PFull, cs, None, rt_f, 200 * rt_f as u64);
        let ro = search(&ctx, Arm::POrbit, cs, Some(&orb), rt_o, 200 * rt_o as u64);
        let (_, v1) = verify_rows(&pq, &ctx.pts, &rf.rel_pts, usize::MAX, false);
        let (_, v2) = verify_rows(&pq, &ctx.pts, &ro.rel_pts, usize::MAX, false);
        let t6 = v1 && v2;
        // log vectors in column order
        let mut lfull: Vec<u64> = Vec::new();
        for i in 0..n_pts {
            if i != ctx.p0idx {
                lfull.push(logs[i]);
            }
        }
        let lrep: Vec<u64> = orb.rep.iter().map(|&i| logs[i]).collect();
        let md = Modulus::new(r);
        let dot = |row: &Row, l: &[u64], lamv: u64| -> u64 {
            let lam2 = md.mul(lamv, lamv);
            let mut s = 0u64;
            match row {
                Row::Full(e) => {
                    for &(c, k) in e.iter() {
                        let kk = if k >= 0 { k as u64 % r } else { (r - (-k) as u64 % r) % r };
                        s = md.add(s, md.mul(kk, l[c as usize] % r));
                    }
                }
                Row::Orb(e) => {
                    for &(c, t) in e.iter() {
                        let x = md.add(
                            md.add(t[0] as u64 % r, md.mul(t[1] as u64, lamv)),
                            md.mul(t[2] as u64, lam2),
                        );
                        s = md.add(s, md.mul(x, l[c as usize] % r));
                    }
                }
            }
            s
        };
        let t7a = t3
            && rf.rows.iter().all(|row| dot(row, &lfull, 1) == 0)
            && ro.rows.iter().all(|row| dot(row, &lrep, lam) == 0);
        let (sf, kf) = classify(&rf.rows, nf, r, 1, &lfull, t7a);
        let (so, ko) = classify(&ro.rows, no, r, lam, &lrep, t7a);
        let worst = std::cmp::min(sf, so);
        let t7b = match worst {
            2 => "determined",
            1 => "underdetermined",
            _ => "fail",
        };
        let mut t7c: Option<bool> = None;
        let mut t7c_kd: Option<usize> = None;
        if so == 2 {
            let lam_w = lam * lam % r;
            let (_rk, ker) = rank_and_kernel(&dense(&ro.rows, no, r, lam_w), no, r);
            t7c = Some(ker.is_empty());
            t7c_kd = Some(ker.len());
        }
        // diagnostic only (not a gate): kernel dimension of the orbit matrix at the proxy modulus
        let lp2 = Modulus::new(ELL).mul(LAMBDA_PROXY, LAMBDA_PROXY);
        let kd_ell1 = rank_and_kernel(&dense(&ro.rows, no, ELL, LAMBDA_PROXY), no, ELL).1.len();
        let kd_ell2 = rank_and_kernel(&dense(&ro.rows, no, ELL, lp2), no, ELL).1.len();
        // T8
        let take = std::cmp::min(ro.rows.len(), no);
        let a_r = build_matrix(&ro.rows[..take], no, &Modulus::new(r), lam);
        let la_r = wiedemann_count(&a_r, &Modulus::new(r), 1, p as u64);
        let a_e = build_matrix(&ro.rows[..take], no, &Modulus::new(ELL), LAMBDA_PROXY);
        let la_e = wiedemann_count(&a_e, &Modulus::new(ELL), 1, p as u64);
        let t8 = la_r.krylov_mul == la_e.krylov_mul;
        let ok = t1 && t2 && t3 && t4 && t5 && t6 && t7a && worst >= 1 && t7c.unwrap_or(true) && t8;
        let line = format!(
            "{{\"schema\":\"l10.tiny/v1\",\"family\":\"P\",\"p\":{},\"curve_seed\":{},\"coeffs\":{:?},\"n_points\":{},\"jac\":{},\"r\":{},\"h\":{},\"rejects_before\":{},\"lambda\":{},\"n_cols_full\":{},\"n_cols_orbit\":{},\"accepted_full\":{},\"accepted_orbit\":{},\"T1\":{},\"T2\":{},\"T3\":{},\"T4\":{},\"T5\":{},\"T6\":{},\"T7a\":{},\"T7b\":\"{}\",\"T7c\":{},\"T8\":{},\"ok\":{},\"T7b_full\":{},\"T7b_orbit\":{},\"kerdim_full\":{},\"kerdim_orbit\":{},\"cap_hit_full\":{},\"cap_hit_orbit\":{},\"T8_w_r\":{},\"T8_w_ell\":{},\"diag_T7c_kerdim\":{},\"diag_orbit_kerdim_ell_lambda\":{},\"diag_orbit_kerdim_ell_lambda2\":{}}}",
            p, cs, curve.f, n_pts, jac, r, h, rejects, lam, nf, no, rf.counters.accepted, ro.counters.accepted,
            jb(t1), jb(t2), jb(t3), jb(t4), jb(t5), jb(t6), jb(t7a), t7b, jopt_b(t7c), jb(t8), jb(ok),
            sf, so, kf, ko, jb(!rf.ok), jb(!ro.ok), la_r.w, la_e.w, jopt(t7c_kd), kd_ell1, kd_ell2
        );
        append_line(out, &line);
    }
    if accepted < 2 {
        append_line(
            out,
            &format!("{{\"schema\":\"l10.tiny/v1\",\"family\":\"P\",\"p\":{},\"note\":\"fewer than 2 curves accepted\",\"accepted\":{},\"ok\":false}}", p, accepted),
        );
    }
    // C and Q
    for fam in [Family::C, Family::Q] {
        for cs in 1..=2u64 {
            let curve = gen_curve(fam, p, cs);
            let ctx = build_ctx(curve.clone());
            let pq = PlaneQuartic::new(p, curve.f, curve.p0).unwrap();
            let arm = if fam == Family::C { Arm::CFull } else { Arm::QFull };
            let n_cols = n_cols_for(arm, ctx.pts.len(), None);
            let rt = n_cols + 10;
            let res = search(&ctx, arm, cs, None, rt, 60 * rt as u64);
            let (_, t6) = verify_rows(&pq, &ctx.pts, &res.rel_pts, usize::MAX, fam == Family::Q);
            let line = format!(
                "{{\"schema\":\"l10.tiny/v1\",\"family\":\"{}\",\"p\":{},\"curve_seed\":{},\"coeffs\":{:?},\"n_points\":{},\"jac\":null,\"r\":null,\"h\":null,\"rejects_before\":null,\"lambda\":null,\"n_cols_full\":{},\"n_cols_orbit\":null,\"accepted_full\":{},\"accepted_orbit\":null,\"T1\":null,\"T2\":null,\"T3\":null,\"T4\":null,\"T5\":null,\"T6\":{},\"T7a\":null,\"T7b\":null,\"T7c\":null,\"T8\":null,\"ok\":{},\"cap_hit_full\":{}}}",
                fam.name(), p, cs, curve.f, ctx.pts.len(), n_cols, res.counters.accepted, jb(t6), jb(t6), jb(!res.ok)
            );
            append_line(out, &line);
        }
    }
    let _ = jopt::<u8>;
}
