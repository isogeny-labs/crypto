use crate::fp::Fp;
use crate::poly::Poly;

#[derive(Clone, Debug)]
pub struct Crit {
    pub chi_mod_l: Vec<u32>,
    pub factor_degrees: Vec<usize>,
    pub fire: bool,
    pub fire_fac: bool,
    pub class: &'static str,
    pub squarefree: bool,
    pub n_h: usize,
    pub n_pairs: usize,
    pub ext_deg: Option<u32>,
}

impl Crit {
    /// Gate G4 for this sample.
    pub fn g4(&self) -> bool {
        self.fire == self.fire_fac && (!self.fire || self.n_h == (1usize << self.n_pairs))
    }
}

/// h^*[i] = h[d-i] q^(d-i) / h[0]
pub fn recip(fl: &Fp, h: &Poly, q: u64) -> Poly {
    let d = h.n - 1;
    let h0 = h.c[0] as u64;
    assert!(h0 != 0);
    let i0 = fl.inv(h0);
    let mut out = Poly::zero();
    for i in 0..=d {
        let k = d - i;
        let qk = fl.pow(q, k as u64);
        out.c[i] = (h.c[k] as u64 * qk % fl.p * i0 % fl.p) as u32;
    }
    out.n = d + 1;
    out.trim();
    out
}

fn digit_poly(l: u64, d: usize, idx: u64) -> Poly {
    // monic degree d; g[0] is the most significant digit
    let mut g = Poly::zero();
    let mut rest = idx;
    for i in (0..d).rev() {
        g.c[i] = (rest % l) as u32;
        rest /= l;
    }
    g.c[d] = 1;
    g.n = d + 1;
    g
}

/// c: monic polynomial of even degree 2g over F_l, q = multiplier mod l.
pub fn crit(fl: &Fp, c: &Poly, q: u64) -> Crit {
    let l = fl.p;
    let deg = c.n - 1;
    assert!(deg % 2 == 0 && c.c[deg] == 1);
    let gg = deg / 2;
    // constant term must equal q^g (det of the similitude with multiplier q)
    assert!(c.get(0) == fl.pow(q, gg as u64), "c[0] != q^g");
    // factorisation
    let mut rem = *c;
    let mut facs: Vec<(Poly, usize)> = vec![];
    for d in 1..=3usize {
        if rem.n < d + 1 {
            break;
        }
        let total = l.pow(d as u32);
        for idx in 0..total {
            if rem.n < d + 1 {
                break;
            }
            let g = digit_poly(l, d, idx);
            if d >= 2 {
                let mut root = false;
                for a in 0..l {
                    if fl.peval(&g, a) == 0 {
                        root = true;
                        break;
                    }
                }
                if root {
                    continue;
                }
            }
            let mut mult = 0;
            loop {
                if rem.n < d + 1 {
                    break;
                }
                let (qq, r) = fl.pdivrem(&rem, &g);
                if r.is_zero() {
                    rem = qq;
                    mult += 1;
                } else {
                    break;
                }
            }
            if mult > 0 {
                facs.push((g, mult));
            }
        }
    }
    if rem.n >= 2 {
        facs.push((rem, 1));
    }
    let mut fd: Vec<usize> = vec![];
    for (g, m) in &facs {
        for _ in 0..*m {
            fd.push(g.n - 1);
        }
    }
    fd.sort_unstable_by(|a, b| b.cmp(a));
    let mut self_recip = false;
    let mut nonself = 0usize;
    for (g, _) in &facs {
        let gs = recip(fl, g, q);
        if gs == *g {
            self_recip = true;
        } else {
            nonself += 1;
        }
    }
    assert!(nonself % 2 == 0, "non-self-reciprocal factors must pair up");
    let n_pairs = nonself / 2;
    let fire_fac = !self_recip;
    // brute force
    let total = l.pow(gg as u32);
    let mut n_h = 0usize;
    let mut ext: Option<u32> = None;
    for idx in 0..total {
        let h = digit_poly(l, gg, idx);
        if h.c[0] == 0 {
            continue;
        }
        let hs = recip(fl, &h, q);
        if fl.pmul(&h, &hs) == *c && fl.pgcd(&h, &hs).n == 1 {
            n_h += 1;
            // multiplicative order of T mod h
            let tpoly = Poly::x();
            let mut t = fl.pmod(&tpoly, &h);
            let one = Poly::one();
            let mut k = 1u32;
            loop {
                let t1 = fl.pmod(&t, &h);
                if t1 == fl.pmod(&one, &h) {
                    break;
                }
                t = fl.pmod(&fl.pmul(&t, &tpoly), &h);
                k += 1;
                assert!(k <= 20000, "ext_deg cap exceeded");
            }
            ext = Some(match ext {
                Some(e) => e.min(k),
                None => k,
            });
        }
    }
    let fire = n_h > 0;
    let squarefree = facs.iter().all(|(_, m)| *m == 1);
    let class = if fire {
        "fire"
    } else if squarefree {
        "none"
    } else {
        "undetermined"
    };
    Crit {
        chi_mod_l: (0..=deg).map(|i| c.c[i]).collect(),
        factor_degrees: fd,
        fire,
        fire_fac,
        class,
        squarefree,
        n_h,
        n_pairs,
        ext_deg: ext,
    }
}

/// Reduce integer coefficients mod l and run the criterion.
pub fn crit_int(fl: &Fp, chi: &[i128], p: i128) -> Crit {
    let l = fl.p as i128;
    let v: Vec<u64> = chi.iter().map(|&a| a.rem_euclid(l) as u64).collect();
    let c = Poly::from(&v, fl.p);
    let q = p.rem_euclid(l) as u64;
    crit(fl, &c, q)
}
