use crate::cantor::{key, Curve, Div};
use crate::fp::Fp;
use crate::poly::Poly;
use crate::enumnorm::{n2_norm, n3_norm, s2_from, s3_from};
use crate::rng::{curve_seed_t, SplitMix64, X_G2, X_ORDER, X_RES};

pub fn isqrt(n: u128) -> u128 {
    let mut x = (n as f64).sqrt() as u128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

/// f = x^7 + c6 x^6 + ... + c0 from the curve seed (SPEC 2), and the number of next()
/// calls made on the coefficient stream.
pub fn curve_f_t(tag: u64, p: u64, i: u64) -> (Poly, u64) {
    let mut rng = SplitMix64::new(curve_seed_t(tag, p, i));
    let mut c = vec![0u64; 8];
    for j in 0..7 {
        c[j] = rng.uniform(p);
    }
    c[7] = 1;
    (Poly::from(&c, p), rng.count)
}

/// yesterday's curves (tag 0x4C06), used by the copied unit tests
pub fn curve_f(p: u64, i: u64) -> Poly {
    curve_f_t(crate::rng::TAG_L06, p, i).0
}

pub fn count_n1(fp: &Fp, f: &Poly) -> i64 {
    let mut n = 1i64;
    for x in 0..fp.p {
        n += 1 + fp.legendre(fp.peval(f, x)) as i64;
    }
    n
}

/// All n in [l, r], n = l mod s, with n D = 0. Collection stops once more than `cap`
/// are found; the returned flag is then true.
pub fn bsgs_all(c: &Curve, d: &Div, l: i64, r: i64, s: i64, cap: usize) -> (Vec<i64>, bool) {
    assert!(r >= l && s >= 1);
    let t = ((r - l) / s) as u128;
    let e = c.mul(s, d);
    let mut m = isqrt(t + 1) as i64;
    if (m as u128) * (m as u128) < t + 1 {
        m += 1;
    }
    let mut table: Vec<(u128, u32)> = Vec::with_capacity(m as usize);
    let mut cur = c.identity();
    for j in 0..m {
        table.push((key(&cur), j as u32));
        cur = c.add(&cur, &e);
    }
    table.sort_unstable();
    let mut g = c.mul(l, d);
    let step = c.mul(m, &e);
    let steps = (t + 1 + m as u128 - 1) / m as u128;
    let mut out: Vec<i64> = vec![];
    for i in 0..steps {
        let k = key(&c.neg(&g));
        let mut idx = table.partition_point(|x| x.0 < k);
        while idx < table.len() && table[idx].0 == k {
            let tt = i * m as u128 + table[idx].1 as u128;
            if tt <= t {
                out.push(l + s * tt as i64);
                if out.len() > cap {
                    out.sort_unstable();
                    out.dedup();
                    return (out, true);
                }
            }
            idx += 1;
        }
        g = c.add(&g, &step);
    }
    out.sort_unstable();
    out.dedup();
    (out, false)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Force {
    /// normal cascade: stage A, then B, then C
    Auto,
    /// skip stage A (gate G1 only): stage B, then C
    B,
    /// skip stages A and B (gate G1 only): stage C
    C,
}

/// Stage C (N3 enumeration) is allowed only for p <= STAGE_C_MAX_P (SPEC.md 4.6).
pub const STAGE_C_MAX_P: u64 = 1031;

pub struct ChiRes {
    /// "ok", "unresolved" or "error"
    pub status: &'static str,
    /// "bsgs" (stage A), "n2" (stage B), "n3" (stage C), or "" if not ok
    pub resolved_by: &'static str,
    pub ncj: usize,
    pub ncp: usize,
    /// candidates after stage B filtering; -1 if stage B not run; 4097 if stage B over cap
    pub ncb: i64,
    pub s1: i128,
    pub s2: i128,
    pub s3: i128,
    pub nj: i64,
    pub njtw: i64,
    pub gate_weil: bool,
    pub gate_ann: bool,
    /// next() calls made on the order, resolution and G2 streams
    pub draws_order: u64,
    pub draws_res: u64,
    pub draws_g2: u64,
}

fn filtered(c: &Curve, cand: Vec<i64>, ds: &[Div]) -> Vec<i64> {
    cand.into_iter()
        .filter(|&n| ds.iter().all(|d| c.is_id(&c.mul(n, d))))
        .collect()
}

/// Stage A (yesterday's method, SPEC.md 4.3-4.4). Returns Some((nJ, nJtw)) iff exactly one pair.
fn stage_a(
    cj: &Curve, ctw: &Curve, p: i128, cc: i128, ct: i128, b: i128, rng: &mut SplitMix64,
    res: &mut ChiRes,
) -> Option<(i64, i64)> {
    let (l, r) = ((cc - b) as i64, (cc + b) as i64);
    assert!(l > 0);
    let mut cand;
    let mut tries = 0;
    loop {
        let d1 = cj.random_class(rng);
        let (cd, over) = bsgs_all(cj, &d1, l, r, 1, 4096);
        cand = cd;
        if !over {
            break;
        }
        tries += 1;
        if tries > 3 {
            res.ncj = cand.len();
            return None;
        }
    }
    let ds: Vec<Div> = (0..7).map(|_| cj.random_class(rng)).collect();
    let cand = filtered(cj, cand, &ds);
    res.ncj = cand.len();
    if cand.is_empty() {
        // the true order always survives: an empty set is an instrument failure
        res.status = "error";
        return None;
    }
    if cand.len() > 16 {
        return None;
    }
    let dt0 = ctw.random_class(rng);
    let dts: Vec<Div> = (0..7).map(|_| ctw.random_class(rng)).collect();
    let modulus = 2 * (p as i64 + 1);
    let mut pairs: Vec<(i64, i64)> = vec![];
    let lo = (ct - b) as i64;
    for &n in &cand {
        let rr = ((2 + 2 * p * p * p - n as i128).rem_euclid(modulus as i128)) as i64;
        let mut lp = lo - lo.rem_euclid(modulus) + rr;
        if lp < lo {
            lp += modulus;
        }
        let rp = (ct + b) as i64;
        if lp > rp {
            continue;
        }
        let (ct_c, over) = bsgs_all(ctw, &dt0, lp, rp, modulus, 4096);
        if over {
            res.ncp = 17;
            return None;
        }
        for n2 in filtered(ctw, ct_c, &dts) {
            pairs.push((n, n2));
        }
    }
    res.ncp = pairs.len();
    if pairs.is_empty() {
        // the true pair always survives: instrument failure
        res.status = "error";
        return None;
    }
    if pairs.len() != 1 {
        return None;
    }
    Some(pairs[0])
}

pub fn chi_search(fp: &Fp, f: &Poly, cseed: u64, force: Force) -> ChiRes {
    let p = fp.p as i128;
    let mut res = ChiRes {
        status: "unresolved",
        resolved_by: "",
        ncj: 0,
        ncp: 0,
        ncb: -1,
        s1: 0,
        s2: 0,
        s3: 0,
        nj: 0,
        njtw: 0,
        gate_weil: false,
        gate_ann: false,
        draws_order: 0,
        draws_res: 0,
        draws_g2: 0,
    };
    let n1 = count_n1(fp, f) as i128;
    let s1 = p + 1 - n1;
    let cc = 1 - s1 * (1 + p * p) + p * p * p;
    let ct = 1 + s1 * (1 + p * p) + p * p * p;
    let b = 15 * p * (p + 1) + 20 * isqrt((p * p * p) as u128) as i128 + 20;
    let cj = Curve::new(fp, *f);
    let ft = fp.pscale(f, fp.nu);
    let ctw = Curve::new(fp, ft);
    let mut found: Option<(i64, i64, i128, i128, &'static str)> = None; // (nJ, nJtw, s2, s3, by)
    // ---- stage A ----
    if force == Force::Auto {
        let mut rng = SplitMix64::new(cseed ^ X_ORDER);
        let a = stage_a(&cj, &ctw, p, cc, ct, b, &mut rng, &mut res);
        res.draws_order = rng.count;
        if res.status == "error" {
            return res;
        }
        if let Some((nj, njtw)) = a {
            let num2 = (nj as i128 + njtw as i128) / 2 - 1 - p * p * p;
            assert!(num2 % (1 + p) == 0);
            let s2 = num2 / (1 + p);
            let s3 = (njtw as i128 - nj as i128) / 2 - s1 * (1 + p * p);
            found = Some((nj, njtw, s2, s3, "bsgs"));
        }
    }
    // ---- stage B: s2 exactly from N2, then a 1-dimensional search in s3 ----
    let mut cand_b: Option<Vec<i64>> = None;
    let mut s2b: Option<i128> = None;
    if found.is_none() && force != Force::C {
        let n2 = n2_norm(fp, f);
        let s2 = s2_from(p, s1, n2);
        s2b = Some(s2);
        let center = cc + s2 * (1 + p);
        let s3max = isqrt((400 * p * p * p) as u128) as i128;
        let (l, r) = ((center - s3max) as i64, (center + s3max) as i64);
        assert!(l > 0);
        let mut rng = SplitMix64::new(cseed ^ X_RES);
        let mut cand: Option<Vec<i64>> = None;
        for _try in 0..4 {
            let d = cj.random_class(&mut rng);
            let (cd, over) = bsgs_all(&cj, &d, l, r, 1, 4096);
            if !over {
                cand = Some(cd);
                break;
            }
        }
        let dj: Vec<Div> = (0..8).map(|_| cj.random_class(&mut rng)).collect();
        let dt: Vec<Div> = (0..8).map(|_| ctw.random_class(&mut rng)).collect();
        res.draws_res = rng.count;
        if let Some(cd) = cand {
            let total = cc + ct + 2 * s2 * (1 + p);
            let kept: Vec<i64> = cd
                .into_iter()
                .filter(|&n| dj.iter().all(|d| cj.is_id(&cj.mul(n, d))))
                .filter(|&n| {
                    let ntw = (total - n as i128) as i64;
                    ntw > 0 && dt.iter().all(|d| ctw.is_id(&ctw.mul(ntw, d)))
                })
                .collect();
            res.ncb = kept.len() as i64;
            if kept.len() == 1 {
                let nj = kept[0];
                let s3 = center - nj as i128;
                let njtw = (total - nj as i128) as i64;
                found = Some((nj, njtw, s2, s3, "n2"));
            } else if kept.is_empty() {
                res.status = "error";
                return res;
            } else {
                cand_b = Some(kept);
            }
        } else {
            res.ncb = 4097;
        }
    }
    // ---- stage C: s3 exactly from N3 ----
    if found.is_none() {
        if fp.p > STAGE_C_MAX_P {
            res.status = "unresolved";
            return res;
        }
        let n2 = n2_norm(fp, f);
        let s2 = s2_from(p, s1, n2);
        if let Some(x) = s2b {
            assert_eq!(x, s2);
        }
        let n3 = n3_norm(fp, f);
        let s3 = s3_from(p, s1, s2, n2, n3);
        let nj = (cc + s2 * (1 + p) - s3) as i64;
        let njtw = (ct + s2 * (1 + p) + s3) as i64;
        if let Some(cb) = &cand_b {
            if !cb.contains(&nj) {
                res.status = "error";
                return res;
            }
        }
        found = Some((nj, njtw, s2, s3, "n3"));
    }
    let (nj, njtw, s2, s3, by) = found.unwrap();
    assert!(cc + s2 * (1 + p) - s3 == nj as i128);
    assert!(ct + s2 * (1 + p) + s3 == njtw as i128);
    res.status = "ok";
    res.resolved_by = by;
    res.s1 = s1;
    res.s2 = s2;
    res.s3 = s3;
    res.nj = nj;
    res.njtw = njtw;
    res.gate_weil = s1 * s1 <= 36 * p && s2.abs() <= 15 * p && s3 * s3 <= 400 * p * p * p;
    // ---- gate G2: its own stream (X_G2), never used for any search ----
    let mut g2 = SplitMix64::new(cseed ^ X_G2);
    let mut ok = true;
    for _ in 0..3 {
        let d = cj.random_class(&mut g2);
        ok &= !cj.is_id(&d) && cj.is_id(&cj.mul(nj, &d));
    }
    for _ in 0..3 {
        let d = ctw.random_class(&mut g2);
        ok &= !ctw.is_id(&d) && ctw.is_id(&ctw.mul(njtw, &d));
    }
    res.draws_g2 = g2.count;
    res.gate_ann = ok;
    res
}

pub fn chi_vec(p: i128, s1: i128, s2: i128, s3: i128) -> [i128; 7] {
    [p * p * p, -p * p * s1, p * s2, -s3, s2, -s1, 1]
}
