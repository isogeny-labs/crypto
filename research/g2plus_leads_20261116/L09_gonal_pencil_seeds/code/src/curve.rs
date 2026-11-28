//! Curve generation, points, fibres, line trial (SPEC sections 5-7).
use crate::field::Fp;
use crate::poly::*;
use crate::rng::{derive, SplitMix64};
use std::collections::HashMap;

pub type Coef = [[u64; 4]; 4];

pub struct Curve {
    pub p: u64,
    pub a: Coef,
    pub pts: Vec<(u64, u64)>,
    pub map: HashMap<(u64, u64), u32>,
    pub redraws: u64,
}

/// Uncounted evaluation of F(x,y) with plain arithmetic.
pub fn raw_eval_f(a: &Coef, x: u64, y: u64, p: u64) -> u64 {
    let mut s = 0u64;
    let mut xi = 1u64;
    for i in 0..4 {
        let mut yj = 1u64;
        for j in 0..4 {
            s = (s + a[i][j] * xi % p * yj) % p;
            yj = yj * y % p;
        }
        xi = xi * x % p;
    }
    s
}

fn raw_fx(a: &Coef, x: u64, y: u64, p: u64) -> u64 {
    let mut s = 0u64;
    for i in 1..4 {
        let mut xi = 1u64;
        for _ in 0..(i - 1) {
            xi = xi * x % p;
        }
        for j in 0..4 {
            let mut yj = 1u64;
            for _ in 0..j {
                yj = yj * y % p;
            }
            s = (s + (i as u64) * a[i][j] % p * xi % p * yj) % p;
        }
    }
    s
}

fn raw_fy(a: &Coef, x: u64, y: u64, p: u64) -> u64 {
    let mut s = 0u64;
    for i in 0..4 {
        let mut xi = 1u64;
        for _ in 0..i {
            xi = xi * x % p;
        }
        for j in 1..4 {
            let mut yj = 1u64;
            for _ in 0..(j - 1) {
                yj = yj * y % p;
            }
            s = (s + (j as u64) * a[i][j] % p * xi % p * yj) % p;
        }
    }
    s
}

fn cubic_has_root(c: [u64; 4], p: u64) -> bool {
    for t in 0..p {
        let mut acc = c[3];
        for k in (0..3).rev() {
            acc = (acc * t + c[k]) % p;
        }
        if acc == 0 {
            return true;
        }
    }
    false
}

/// Monic fibre cubic. kind 1: x-fibre at x=t (cubic in y); kind 2: y-fibre at y=t (cubic in x).
pub fn fibre_cubic(fp: &Fp, a: &Coef, kind: u8, t: u64) -> Poly {
    let mut c = vec![0u64; 4];
    for j in 0..4 {
        let coefs: [u64; 4] = if kind == 1 {
            [a[0][j], a[1][j], a[2][j], a[3][j]]
        } else {
            [a[j][0], a[j][1], a[j][2], a[j][3]]
        };
        let mut acc = coefs[3];
        for k in (0..3).rev() {
            acc = fp.add(fp.mul(acc, t), coefs[k]);
        }
        c[j] = acc;
    }
    make_monic(fp, &c)
}

impl Curve {
    /// Acceptance (i)-(v) for given coefficients; enumerates points with `fp` (phase setup).
    pub fn from_coeffs(fp: &Fp, a: Coef, redraws: u64) -> Result<Curve, &'static str> {
        let p = fp.p;
        if a[3][3] == 0 {
            return Err("i");
        }
        let ax = [a[3][0], a[3][1], a[3][2], a[3][3]];
        let ay = [a[0][3], a[1][3], a[2][3], a[3][3]];
        if cubic_has_root(ax, p) || cubic_has_root(ay, p) {
            return Err("ii");
        }
        let mut pts: Vec<(u64, u64)> = vec![];
        for x in 0..p {
            let c = fibre_cubic(fp, &a, 1, x);
            for y in roots_all(fp, &c) {
                pts.push((x, y));
            }
        }
        pts.sort();
        for &(x, y) in &pts {
            if raw_fx(&a, x, y, p) == 0 && raw_fy(&a, x, y, p) == 0 {
                return Err("iii");
            }
        }
        let n1 = pts.len() as i64;
        let d = n1 - (p as i64 + 1);
        if d * d > 64 * p as i64 {
            return Err("iv");
        }
        if n1 < 16 {
            return Err("v");
        }
        let mut map = HashMap::new();
        for (i, &pt) in pts.iter().enumerate() {
            map.insert(pt, i as u32);
        }
        Ok(Curve { p, a, pts, map, redraws })
    }

    pub fn generate(fp: &Fp, seed: u64) -> Curve {
        let p = fp.p;
        let mut rng = SplitMix64::new(derive(p, seed, 255, 1));
        let mut redraws = 0u64;
        loop {
            let mut a: Coef = [[0; 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    a[i][j] = rng.below(p);
                }
            }
            match Curve::from_coeffs(fp, a, redraws) {
                Ok(c) => return c,
                Err(_) => redraws += 1,
            }
        }
    }
    pub fn n1(&self) -> usize {
        self.pts.len()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Outcome {
    Degenerate,
    NoSplit,
    Tangent,
    Ok,
}

pub struct LineRes {
    pub outcome: Outcome,
    pub b: [u64; 4],
    pub g: Poly,
    pub r: Poly,
    pub pts: Vec<u32>, // O, P1, P2, residuals when Ok
}

fn det3(fp: &Fp, m: [[u64; 3]; 3]) -> u64 {
    let (a, b, c) = (m[0][0], m[0][1], m[0][2]);
    let (d, e, f) = (m[1][0], m[1][1], m[1][2]);
    let (g, h, i) = (m[2][0], m[2][1], m[2][2]);
    let t1 = fp.mul(a, fp.sub(fp.mul(e, i), fp.mul(f, h)));
    let t2 = fp.mul(b, fp.sub(fp.mul(d, i), fp.mul(f, g)));
    let t3 = fp.mul(c, fp.sub(fp.mul(d, h), fp.mul(e, g)));
    fp.add(fp.sub(t1, t2), t3)
}

pub fn line_trial(fp: &Fp, cv: &Curve, o: u32, p1: u32, p2: u32) -> LineRes {
    let ids = [o, p1, p2];
    let mut rows = [[0u64; 4]; 3];
    for (k, &id) in ids.iter().enumerate() {
        let (x, y) = cv.pts[id as usize];
        let w = fp.mul(x, y);
        rows[k] = [1, x, y, w];
    }
    let mut b = [0u64; 4];
    for m in 0..4 {
        let mut mat = [[0u64; 3]; 3];
        for k in 0..3 {
            let mut cidx = 0;
            for c in 0..4 {
                if c == m {
                    continue;
                }
                mat[k][cidx] = rows[k][c];
                cidx += 1;
            }
        }
        let d = det3(fp, mat);
        b[m] = if m % 2 == 1 { fp.neg(d) } else { d };
    }
    let (b00, b10, b01, b11) = (b[0], b[1], b[2], b[3]);
    let dd = fp.sub(fp.mul(b00, b11), fp.mul(b10, b01));
    if dd == 0 {
        return LineRes { outcome: Outcome::Degenerate, b, g: vec![], r: vec![], pts: vec![] };
    }
    let mut u = vec![fp.neg(b00), fp.neg(b10)];
    trim(&mut u);
    let mut v = vec![b01, b11];
    trim(&mut v);
    let mut up: Vec<Poly> = vec![vec![1]];
    let mut vp: Vec<Poly> = vec![vec![1]];
    for j in 1..4 {
        let n = pmul(fp, &up[j - 1], &u);
        up.push(n);
        let n = pmul(fp, &vp[j - 1], &v);
        vp.push(n);
    }
    let mut g: Poly = vec![];
    for j in 0..4 {
        let mut aj = vec![cv.a[0][j], cv.a[1][j], cv.a[2][j], cv.a[3][j]];
        trim(&mut aj);
        let t = pmul(fp, &up[j], &vp[3 - j]);
        let t = pmul(fp, &aj, &t);
        g = padd(fp, &g, &t);
    }
    assert!(g.len() == 7, "deg G != 6");
    let xs3: Vec<u64> = ids.iter().map(|&i| cv.pts[i as usize].0).collect();
    let mut cur = g.clone();
    for &xr in &xs3 {
        let (q, rem) = synth_div(fp, &cur, xr);
        assert!(rem == 0, "line trial: non-zero remainder");
        cur = q;
    }
    let r = make_monic(fp, &cur);
    if !split_test(fp, &r) {
        return LineRes { outcome: Outcome::NoSplit, b, g, r, pts: vec![] };
    }
    let xs = roots_split(fp, &r);
    if xs.iter().any(|x| xs3.contains(x)) {
        return LineRes { outcome: Outcome::Tangent, b, g, r, pts: vec![] };
    }
    let mut num = [0u64; 3];
    let mut den = [0u64; 3];
    for k in 0..3 {
        num[k] = fp.add(b00, fp.mul(b10, xs[k]));
        den[k] = fp.add(b01, fp.mul(b11, xs[k]));
        assert!(den[k] != 0, "den zero");
    }
    let c1 = den[0];
    let c2 = fp.mul(c1, den[1]);
    let c3 = fp.mul(c2, den[2]);
    let i3 = fp.inv(c3);
    let inv2 = fp.mul(i3, c2);
    let t = fp.mul(i3, den[2]);
    let inv1 = fp.mul(t, c1);
    let inv0 = fp.mul(t, den[1]);
    let invs = [inv0, inv1, inv2];
    let mut pts = vec![o, p1, p2];
    for k in 0..3 {
        let y = fp.neg(fp.mul(num[k], invs[k]));
        let id = *cv.map.get(&(xs[k], y)).expect("residual point not on curve map");
        pts.push(id);
    }
    LineRes { outcome: Outcome::Ok, b, g, r, pts }
}
