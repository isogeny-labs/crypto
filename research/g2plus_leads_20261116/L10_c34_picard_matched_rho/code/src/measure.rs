//! Subcommand measure.
use crate::curves::{gen_curve, zeta_of, Family};
use crate::enumerate::{orbits_of, points_picard_counted, Orbits};
use crate::fp::Cnt;
use crate::la::{build_matrix, wiedemann_count, Modulus, ELL, LAMBDA_PROXY};
use crate::lines::{n_cols_for, run_search, Arm, SearchInput};
use crate::plane_quartic_pic::PlaneQuartic;
use crate::{append_line, build_ctx, jb, jopt, jopt_b, verify_rows, Ctx};
use std::collections::HashMap;

pub fn run(p: u32, fams: &[Family], out: &str) {
    let mut ctxs: HashMap<(u8, u64), Ctx> = HashMap::new();
    for cs in 1..=3u64 {
        for ss in 1..=2u64 {
            for &fam in fams.iter() {
                let key = (fam as u8, cs);
                if !ctxs.contains_key(&key) {
                    ctxs.insert(key, build_ctx(gen_curve(fam, p, cs)));
                }
                let ctx = &ctxs[&key];
                let arms: Vec<Arm> = match fam {
                    Family::Q => vec![Arm::QFull],
                    Family::C => vec![Arm::CFull],
                    Family::P => vec![Arm::PFull, Arm::POrbit],
                };
                for arm in arms {
                    let line = one_run(ctx, arm, p, cs, ss);
                    append_line(out, &line);
                }
            }
        }
    }
}

pub fn one_run(ctx: &Ctx, arm: Arm, p: u32, cs: u64, ss: u64) -> String {
    let curve = &ctx.curve;
    let n_pts = ctx.pts.len();
    let m3 = ((n_pts as f64) - (p as f64 + 1.0)).abs() <= 6.0 * (p as f64).sqrt();
    let is_p = arm == Arm::PFull || arm == Arm::POrbit;
    let mut zeta: Option<u64> = None;
    let mut orbits: Option<Orbits> = None;
    let (mut enum_mul, mut enum_inv) = (0u64, 0u64);
    let mut m1: Option<bool> = None;
    if is_p {
        let z = zeta_of(p);
        zeta = Some(z);
        let (a2, a1, a0) = curve.a.unwrap();
        let mut c = Cnt::default();
        let pts_c = points_picard_counted(a2, a1, a0, p, &mut c);
        let mut s1 = pts_c.clone();
        s1.sort();
        let mut s2 = ctx.pts.clone();
        s2.sort();
        m1 = Some(s1 == s2);
        enum_mul = c.mul;
        enum_inv = c.inv;
        let orb = orbits_of(&ctx.pts, &ctx.idx, z, p);
        if arm == Arm::POrbit {
            enum_mul += 2 * orb.n_orbits as u64;
        }
        orbits = Some(orb);
    }
    let orb_ref = if arm == Arm::POrbit { orbits.as_ref() } else { None };
    let n_cols = n_cols_for(arm, n_pts, orb_ref);
    let rows_target = n_cols + 10;
    let cap = 60 * rows_target as u64;
    let inp = SearchInput {
        arm,
        p,
        f: &curve.f,
        pts: &ctx.pts,
        idx: &ctx.idx,
        p0: curve.p0,
        orbits: orb_ref,
        curve_seed: cs,
        solver_seed: ss,
        rows_target,
        cap,
    };
    let res = run_search(&inp);
    let ct = &res.counters;
    // M2
    let pq = PlaneQuartic::new(p, curve.f, curve.p0).expect("verify curve");
    let (nver, allok) = verify_rows(&pq, &ctx.pts, &res.rel_pts, 50, arm == Arm::QFull);
    let m2_verified = if allok { nver } else { 0 };
    let m2_ok = allok && nver == std::cmp::min(50, ct.accepted as usize);
    // LA
    let md = Modulus::new(ELL);
    let lam = if arm == Arm::POrbit { LAMBDA_PROXY } else { 1 };
    let take = std::cmp::min(res.rows.len(), n_cols);
    let a = build_matrix(&res.rows[..take], n_cols, &md, lam);
    let la = wiedemann_count(&a, &md, ss, p as u64);
    let m4 = la.krylov_mul == 2 * (la.n as u64) * (la.n as u64 + la.w as u64);
    let m5 = res.ok;
    let fpm_enum = enum_mul + 10 * enum_inv;
    let fpm_line = ct.line_mul + 10 * ct.line_inv;
    let fpm_total = fpm_enum + fpm_line + la.fpm_w();
    let ok = m1.unwrap_or(true) && m2_ok && m3 && m4 && m5;
    format!(
        "{{\"schema\":\"l10.measure/v1\",\"family\":\"{}\",\"arm\":\"{}\",\"p\":{},\"curve_seed\":{},\"solver_seed\":{},\"coeffs\":{:?},\"zeta\":{},\"n_points\":{},\"n_fixed\":{},\"n_orbits\":{},\"n_cols\":{},\"rows_target\":{},\"trials\":{},\"accepted\":{},\"dup\":{},\"trivial\":{},\"nonsquare\":{},\"tangent\":{},\"enum_mul\":{},\"enum_inv\":{},\"line_mul\":{},\"line_inv\":{},\"line_setup_mul\":{},\"la_n\":{},\"la_w\":{},\"krylov_mul\":{},\"bm_mul\":{},\"bm_inv\":{},\"L_bm\":{},\"recon_mul\":{},\"recon_inv\":{},\"la_mul\":{},\"la_inv\":{},\"fpm_enum\":{},\"fpm_line\":{},\"fpm_la_unw\":{},\"fpm_la_w\":{},\"fpm_total\":{},\"M1\":{},\"M2_verified\":{},\"M3\":{},\"M4\":{},\"M5\":{},\"ok\":{}}}",
        arm.family().name(), arm.name(), p, cs, ss, curve.f,
        jopt(zeta), n_pts,
        jopt(orbits.as_ref().map(|o| o.n_fixed)), jopt(orbits.as_ref().map(|o| o.n_orbits)),
        n_cols, rows_target, ct.trials, ct.accepted, ct.dup, ct.trivial, ct.nonsquare, ct.tangent,
        enum_mul, enum_inv, ct.line_mul, ct.line_inv, ct.line_setup_mul,
        la.n, la.w, la.krylov_mul, la.bm_mul, la.bm_inv, la.l_bm, la.recon_mul, la.recon_inv,
        la.la_mul(), la.la_inv(), fpm_enum, fpm_line, la.fpm_unw(), la.fpm_w(), fpm_total,
        jopt_b(m1), m2_verified, jb(m3), jb(m4), jb(m5), jb(ok)
    )
}
