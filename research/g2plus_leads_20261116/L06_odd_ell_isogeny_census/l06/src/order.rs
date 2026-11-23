use crate::cantor::{key, Curve, Div};
use crate::fp::Fp;
use crate::poly::Poly;
use crate::rng::{curve_seed, SplitMix64};

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

/// f = x^7 + c6 x^6 + ... + c0 from the curve seed (SPEC 2).
pub fn curve_f(p: u64, i: u64) -> Poly {
    let mut rng = SplitMix64::new(curve_seed(p, i));
    let mut c = vec![0u64; 8];
    for j in 0..7 {
        c[j] = rng.uniform(p);
    }
    c[7] = 1;
    Poly::from(&c, p)
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

pub struct ChiRes {
    pub status: &'static str,
    pub ncj: usize,
    pub ncp: usize,
    pub s1: i128,
    pub s2: i128,
    pub s3: i128,
    pub nj: i64,
    pub njtw: i64,
    pub gate_weil: bool,
    pub gate_ann: bool,
}

fn filtered(c: &Curve, cand: Vec<i64>, ds: &[Div]) -> Vec<i64> {
    cand.into_iter()
        .filter(|&n| ds.iter().all(|d| c.is_id(&c.mul(n, d))))
        .collect()
}

pub fn chi_search(fp: &Fp, f: &Poly, cseed: u64) -> ChiRes {
    let p = fp.p as i128;
    let mut res = ChiRes {
        status: "ambiguous",
        ncj: 0,
        ncp: 0,
        s1: 0,
        s2: 0,
        s3: 0,
        nj: 0,
        njtw: 0,
        gate_weil: false,
        gate_ann: false,
    };
    let n1 = count_n1(fp, f) as i128;
    let s1 = p + 1 - n1;
    let cc = 1 - s1 * (1 + p * p) + p * p * p;
    let ct = 1 + s1 * (1 + p * p) + p * p * p;
    let b = 15 * p * (p + 1) + 20 * isqrt((p * p * p) as u128) as i128 + 20;
    let cj = Curve::new(fp, *f);
    let ft = fp.pscale(f, fp.nu);
    let ctw = Curve::new(fp, ft);
    let mut rng = SplitMix64::new(cseed ^ 0xD1B5_4A32_D192_ED03);
    let (l, r) = ((cc - b) as i64, (cc + b) as i64);
    assert!(l > 0);
    // step 1
    let mut cand;
    let mut tries = 0;
    loop {
        let d1 = cj.random_class(&mut rng);
        let (cd, over) = bsgs_all(&cj, &d1, l, r, 1, 4096);
        cand = cd;
        if !over {
            break;
        }
        tries += 1;
        if tries > 3 {
            res.ncj = cand.len();
            return res;
        }
    }
    // step 2
    let ds: Vec<Div> = (0..7).map(|_| cj.random_class(&mut rng)).collect();
    let cand = filtered(&cj, cand, &ds);
    res.ncj = cand.len();
    if cand.is_empty() {
        res.status = "error";
        return res;
    }
    if cand.len() > 16 {
        return res;
    }
    // twist
    let dt0 = ctw.random_class(&mut rng);
    let dts: Vec<Div> = (0..7).map(|_| ctw.random_class(&mut rng)).collect();
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
        let (ct_c, over) = bsgs_all(&ctw, &dt0, lp, rp, modulus, 4096);
        if over {
            res.ncp = 17;
            return res;
        }
        for n2 in filtered(&ctw, ct_c, &dts) {
            pairs.push((n, n2));
        }
    }
    res.ncp = pairs.len();
    if pairs.len() != 1 {
        return res;
    }
    let (nj, njtw) = pairs[0];
    let num2 = (nj as i128 + njtw as i128) / 2 - 1 - p * p * p;
    assert!(num2 % (1 + p) == 0);
    let s2 = num2 / (1 + p);
    let num3 = (njtw as i128 - nj as i128) / 2 - s1 * (1 + p * p);
    let s3 = num3;
    assert!(cc + s2 * (1 + p) - s3 == nj as i128);
    assert!(ct + s2 * (1 + p) + s3 == njtw as i128);
    res.status = "ok";
    res.s1 = s1;
    res.s2 = s2;
    res.s3 = s3;
    res.nj = nj;
    res.njtw = njtw;
    res.gate_weil = s1 * s1 <= 36 * p && s2.abs() <= 15 * p && s3 * s3 <= 400 * p * p * p;
    let mut g2 = SplitMix64::new(cseed ^ 0x8CB9_2BA7_2F3D_8DD7);
    let mut ok = true;
    for _ in 0..3 {
        let d = cj.random_class(&mut g2);
        ok &= cj.is_id(&cj.mul(nj, &d));
    }
    for _ in 0..3 {
        let d = ctw.random_class(&mut g2);
        ok &= ctw.is_id(&ctw.mul(njtw, &d));
    }
    res.gate_ann = ok;
    res
}

pub fn chi_vec(p: i128, s1: i128, s2: i128, s3: i128) -> [i128; 7] {
    [p * p * p, -p * p * s1, p * s2, -s3, s2, -s1, 1]
}
