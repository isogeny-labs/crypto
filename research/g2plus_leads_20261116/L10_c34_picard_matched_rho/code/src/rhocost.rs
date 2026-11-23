//! Subcommand rhocost.
use crate::curves::{gen_curve, zeta_of, Family};
use crate::fp::Cnt;
use crate::picrho::{check_t5, Rho};
use crate::plane_quartic_pic::{find_rational_point, FqCounter, PicClass, PicStats, PlaneQuartic, Pt};
use crate::{append_line, jb, jopt_b};

fn stats_add(a: &mut [u64; 4], before: &PicStats, after: &PicStats) {
    a[0] += after.exc_results - before.exc_results;
    a[1] += after.neg_pencil - before.neg_pencil;
    a[2] += after.lifts - before.lifts;
    a[3] += after.p0_in_support - before.p0_in_support;
}

pub fn run(fam: Family, p: u32, cs: u64, out: &str) {
    let curve = gen_curve(fam, p, cs);
    let pq = PlaneQuartic::new(p, curve.f, curve.p0).expect("curve");
    let mut rng: u64 = 1_000_003 * cs + p as u64;
    let mut pts: Vec<Pt> = Vec::new();
    while pts.len() < 300 {
        if let Some(pt) = find_rational_point(p, &curve.f, &mut rng, 256) {
            pts.push(pq.normalize_point(pt));
        }
    }
    let mut xs: Vec<PicClass> = Vec::new();
    for i in 0..100 {
        let a = pq.add(&pq.point_class(pts[3 * i]), &pq.point_class(pts[3 * i + 1]));
        xs.push(pq.add(&a, &pq.point_class(pts[3 * i + 2])));
    }
    let rho = if fam == Family::P { Some(Rho::new(p, zeta_of(p))) } else { None };
    let mut st = [0u64; 4];
    let (mut n_add, mut add_c) = (0u64, FqCounter::default());
    let (mut n_neg, mut neg_c) = (0u64, FqCounter::default());
    let (mut n_dbl, mut dbl_c) = (0u64, FqCounter::default());
    let (mut n_rho, mut rho_c, mut rho_own) = (0u64, FqCounter::default(), Cnt::default());
    let mut r1 = true;
    let mut ss: Vec<Option<PicClass>> = Vec::new();
    for i in 0..99 {
        if xs[i] == xs[i + 1] {
            ss.push(None);
            continue;
        }
        let (c0, s0) = (pq.counter(), pq.stats());
        let s = pq.add(&xs[i], &xs[i + 1]);
        add_c = sum(add_c, pq.counter().since(&c0));
        stats_add(&mut st, &s0, &pq.stats());
        n_add += 1;
        let (c0, s0) = (pq.counter(), pq.stats());
        let n = pq.neg(&s);
        neg_c = sum(neg_c, pq.counter().since(&c0));
        stats_add(&mut st, &s0, &pq.stats());
        n_neg += 1;
        r1 &= pq.is_zero(&pq.add(&s, &n));
        ss.push(Some(s));
    }
    for i in 0..50 {
        if let Some(s) = &ss[i] {
            let (c0, s0) = (pq.counter(), pq.stats());
            let _d = pq.double(s);
            dbl_c = sum(dbl_c, pq.counter().since(&c0));
            stats_add(&mut st, &s0, &pq.stats());
            n_dbl += 1;
        }
    }
    let mut t5: Option<bool> = None;
    if let Some(rh) = &rho {
        for i in 0..99 {
            if let Some(s) = &ss[i] {
                let (c0, s0) = (pq.counter(), pq.stats());
                let _r = rh.rho_class(&pq, s, &mut rho_own);
                rho_c = sum(rho_c, pq.counter().since(&c0));
                stats_add(&mut st, &s0, &pq.stats());
                n_rho += 1;
            }
        }
        // T5 on first 20 classes and 20 points
        let mut own = Cnt::default();
        let mut ok = true;
        for i in 0..20 {
            let x = &xs[i];
            let y = &xs[i + 1];
            let rx = rh.rho_class(&pq, x, &mut own);
            let r2x = rh.rho_class(&pq, &rx, &mut own);
            ok &= rh.rho_class(&pq, &r2x, &mut own) == *x;
            ok &= pq.is_zero(&pq.add(&pq.add(x, &rx), &r2x));
            let ry = rh.rho_class(&pq, y, &mut own);
            ok &= rh.rho_class(&pq, &pq.add(x, y), &mut own) == pq.add(&rx, &ry);
            let pt = pts[i];
            ok &= rh.rho_class(&pq, &pq.point_class(pt), &mut own)
                == pq.point_class(rh.rho_point(&pq, pt));
        }
        let _ = check_t5;
        t5 = Some(ok);
    }
    for i in 0..10 {
        r1 &= pq.add(&xs[i], &xs[i + 1]) == pq.add(&xs[i + 1], &xs[i]);
    }
    let ok = r1 && t5.unwrap_or(true);
    let line = format!(
        "{{\"schema\":\"l10.rhocost/v1\",\"family\":\"{}\",\"p\":{},\"curve_seed\":{},\"coeffs\":{:?},\"p0\":{:?},\"n_add\":{},\"add_mul\":{},\"add_inv\":{},\"n_neg\":{},\"neg_mul\":{},\"neg_inv\":{},\"n_dbl\":{},\"dbl_mul\":{},\"dbl_inv\":{},\"n_rho\":{},\"rho_mul_lab\":{},\"rho_inv_lab\":{},\"rho_mul_own\":{},\"stats\":{{\"exc_results\":{},\"neg_pencil\":{},\"lifts\":{},\"p0_in_support\":{}}},\"R1\":{},\"T5\":{},\"ok\":{}}}",
        fam.name(), p, cs, curve.f, curve.p0, n_add, add_c.mul, add_c.inv, n_neg, neg_c.mul, neg_c.inv,
        n_dbl, dbl_c.mul, dbl_c.inv, n_rho, rho_c.mul, rho_c.inv, rho_own.mul,
        st[0], st[1], st[2], st[3], jb(r1), jopt_b(t5), jb(ok)
    );
    append_line(out, &line);
}

fn sum(a: FqCounter, b: FqCounter) -> FqCounter {
    FqCounter { mul: a.mul + b.mul, inv: a.inv + b.inv, add: a.add + b.add }
}
