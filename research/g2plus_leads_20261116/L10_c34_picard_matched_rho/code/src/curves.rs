//! Curve generation (families Q, C, P), zeta, F-evaluation cost rule.
use crate::fp::{Cnt, Fp};
use crate::plane_quartic_pic::{
    mons_of_degree, next_smooth_quartic_with_point, smoothness_certificate, xorshift_next,
    PlaneQuartic, Pt,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Q,
    C,
    P,
}
impl Family {
    pub fn name(&self) -> &'static str {
        match self {
            Family::Q => "Q",
            Family::C => "C",
            Family::P => "P",
        }
    }
    pub fn parse(s: &str) -> Family {
        match s {
            "Q" => Family::Q,
            "C" => Family::C,
            "P" => Family::P,
            _ => panic!("bad family {}", s),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Curve {
    pub family: Family,
    pub p: u32,
    pub f: [u32; 15],
    pub p0: Pt,
    /// Picard only: (a2, a1, a0)
    pub a: Option<(u32, u32, u32)>,
}

pub fn draw(s: &mut u64, p: u32) -> u32 {
    (xorshift_next(s) % p as u64) as u32
}

pub fn picard_from(p: u32, a2: u32, a1: u32, a0: u32) -> [u32; 15] {
    let mut f = [0u32; 15];
    f[0] = (p - a0) % p;
    f[1] = (p - a1) % p;
    f[3] = (p - a2) % p;
    f[9] = 1;
    f[10] = p - 1;
    f
}

pub fn gen_curve(family: Family, p: u32, curve_seed: u64) -> Curve {
    match family {
        Family::Q => {
            let mut s = 1000 + curve_seed;
            let (f, p0) = next_smooth_quartic_with_point(p, &mut s);
            let pq0 = PlaneQuartic::new(p, f, p0).expect("Q curve");
            let p0 = pq0.normalize_point(p0);
            Curve { family, p, f, p0, a: None }
        }
        Family::P => {
            let mut s = 2000 + curve_seed;
            loop {
                let a2 = draw(&mut s, p);
                let a1 = draw(&mut s, p);
                let a0 = draw(&mut s, p);
                let f = picard_from(p, a2, a1, a0);
                if smoothness_certificate(p, &f) {
                    let p0 = [0, 0, 1];
                    PlaneQuartic::new(p, f, p0).expect("P curve");
                    return Curve { family, p, f, p0, a: Some((a2, a1, a0)) };
                }
            }
        }
        Family::C => {
            let mut s = 3000 + curve_seed;
            loop {
                let order = [0usize, 1, 3, 6, 2, 4, 7, 5, 8];
                let mut f = [0u32; 15];
                for &i in order.iter() {
                    f[i] = draw(&mut s, p);
                }
                f[9] = 1;
                f[10] = p - 1;
                if smoothness_certificate(p, &f) {
                    let p0 = [0, 0, 1];
                    PlaneQuartic::new(p, f, p0).expect("C curve");
                    return Curve { family, p, f, p0, a: None };
                }
            }
        }
    }
}

/// smallest g>=2 with g^((p-1)/3) != 1; zeta = that power.
pub fn zeta_of(p: u32) -> u64 {
    let fp = Fp::new(p as u64);
    assert!(p % 3 == 1);
    let mut g = 2u64;
    loop {
        let t = fp.pow_u(g, (p as u64 - 1) / 3);
        if t != 1 {
            return t;
        }
        g += 1;
    }
}

/// F-evaluation with the frozen cost rule.
pub fn eval_f(f: &[u32; 15], pt: [u64; 3], fp: &Fp, c: &mut Cnt) -> u64 {
    let mons = mons_of_degree(4);
    let mut pw = [[0u64; 5]; 3];
    for i in 0..3 {
        let x = pt[i] % fp.p;
        let x2 = fp.mul(x, x, c);
        let x3 = fp.mul(x2, x, c);
        let x4 = fp.mul(x2, x2, c);
        pw[i] = [1, x, x2, x3, x4];
    }
    let mut acc = 0u64;
    for (m, &(a, b, cc)) in mons.iter().enumerate() {
        if f[m] == 0 {
            continue;
        }
        let exps = [a as usize, b as usize, cc as usize];
        let mut prod: Option<u64> = None;
        for i in 0..3 {
            if exps[i] > 0 {
                let v = pw[i][exps[i]];
                prod = Some(match prod {
                    None => v,
                    Some(q) => fp.mul(q, v, c),
                });
            }
        }
        let mut t = prod.unwrap();
        let fm = f[m] as u64;
        if fm == 1 {
        } else if fm == fp.p - 1 {
            t = fp.neg(t);
        } else {
            t = fp.mul(t, fm, c);
        }
        acc = fp.add(acc, t);
    }
    acc
}
