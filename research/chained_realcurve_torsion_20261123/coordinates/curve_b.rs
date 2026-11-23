use crate::field::Field;
use crate::inherited::{padd, peval, pmod, pmonic, pmul, pscale, ptrim, pxgcd, Div, Fld, Hyp};

pub const ORIGINAL: [u32; 7] = [1, 0, 1, 0, 0, 1, 1];
pub type Divisor = Div<u32>;

pub fn key(d: &Divisor) -> Vec<u32> {
    let mut k = vec![d.u.len() as u32];
    k.extend_from_slice(&d.u);
    k.push(d.v.len() as u32);
    k.extend_from_slice(&d.v);
    k
}

pub struct Chart<'a> {
    pub h: Hyp<'a, Field>,
    pub roots: Vec<u32>,
    pub r: u32,
    pub scale: u32,
    pub leading: u32,
    pub delta: u32,
    pub pole: u32,
    pub pre: Divisor,
    pub post: Divisor,
}

impl<'a> Chart<'a> {
    pub fn new(field: &'a Field) -> Self {
        let roots: Vec<_> = field
            .subfield27()
            .into_iter()
            .filter(|x| peval(field, &ORIGINAL, x) == 0)
            .collect();
        assert_eq!(roots.len(), 6);
        assert!(roots.iter().all(|&x| field.pow(x, 3) != x));
        let r = roots[0];
        let mut g = vec![0; 7];
        for (i, c) in ORIGINAL.iter().enumerate() {
            let mut choose = 1;
            for j in 0..=i {
                let term = field.mul(c, &field.mul(&(choose % 3), &field.pow(r, (i - j) as u32)));
                g[6 - j] = field.add(&g[6 - j], &term);
                if j < i {
                    choose = choose * (i - j) as u32 / (j + 1) as u32;
                }
            }
        }
        ptrim(field, &mut g);
        assert_eq!(g.len(), 6);
        let leading = g[5];
        let scale = field.sqrt(leading).unwrap();
        let monic = pscale(field, &g, &field.inv(&leading));
        assert_eq!(monic[5], 1);
        let derivative: Vec<_> = monic
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| field.mul(c, &(i as u32 % 3)))
            .collect();
        assert_eq!(pxgcd(field, &monic, &derivative).0.len(), 1);
        let h = Hyp {
            f: field,
            h: monic,
            g: 2,
        };
        let delta = field.sub(&field.pow(r, 3), &r);
        let pole = field.inv(&field.sub(&field.pow(r, 9), &r));
        let pre = Self::point_chart(&h, pole, 0);
        let post = Self::point_chart(&h, field.inv(&delta), 0);
        assert_eq!(field.mul(&scale, &scale), leading);
        assert_eq!(h.add(&pre, &pre), h.identity());
        assert_eq!(h.add(&post, &post), h.identity());
        assert_ne!(pre, post);
        Self {
            h,
            roots,
            r,
            scale,
            leading,
            delta,
            pole,
            pre,
            post,
        }
    }

    fn point_chart(h: &Hyp<'a, Field>, x: u32, y: u32) -> Divisor {
        let mut v = vec![y];
        ptrim(h.f, &mut v);
        let d = Div {
            u: vec![h.f.neg(&x), 1],
            v,
        };
        assert!(h.is_valid(&d));
        d
    }

    pub fn point(&self, x: u32, y: u32) -> Divisor {
        let f = self.h.f;
        assert_eq!(f.mul(&y, &y), peval(f, &ORIGINAL, &x));
        if x == self.r {
            return self.h.identity();
        }
        let t = f.inv(&f.sub(&x, &self.r));
        let z = f.mul(&f.mul(&y, &f.pow(t, 3)), &f.inv(&self.scale));
        Self::point_chart(&self.h, t, z)
    }

    pub fn pair(&self, i: usize, j: usize) -> Divisor {
        assert_ne!(i, j);
        self.h
            .add(&self.point(self.roots[i], 0), &self.point(self.roots[j], 0))
    }

    pub fn push_regular(&self, d: &Divisor) -> Divisor {
        assert_ne!(peval(self.h.f, &d.u, &self.pole), 0);
        let f = self.h.f;
        let degree = d.u.len() - 1;
        let denominator = vec![1, f.neg(&self.delta)];
        let mut powers = vec![vec![1]];
        for _ in 0..3 {
            powers.push(pmul(f, powers.last().unwrap(), &denominator));
        }
        let substitute = |coefficients: &[u32], weight: usize| {
            let mut result = Vec::new();
            for (i, &c) in coefficients.iter().enumerate() {
                let scaled = pscale(f, &powers[weight - i], &f.pow(c, 3));
                let mut shifted = vec![0; i];
                shifted.extend(scaled);
                result = padd(f, &result, &shifted);
            }
            result
        };
        let u = pmonic(f, &substitute(&d.u, degree));
        assert_eq!(u.len(), d.u.len());
        let v = pmod(f, &pscale(f, &substitute(&d.v, 3), &self.leading), &u);
        let image = Div { u, v };
        assert!(self.h.is_valid(&image));
        image
    }

    pub fn frobenius(&self, d: &Divisor) -> Divisor {
        assert!(self.h.is_valid(d));
        if self.h.is_identity(d) {
            return self.h.identity();
        }
        if peval(self.h.f, &d.u, &self.pole) == 0 {
            let reduced = self.h.add(d, &self.pre);
            assert!(reduced.u.len() < d.u.len());
            return self.h.add(&self.frobenius(&reduced), &self.post);
        }
        let image = self.push_regular(d);
        if (d.u.len() - 1) & 1 == 1 {
            self.h.sub(&image, &self.post)
        } else {
            image
        }
    }
}
