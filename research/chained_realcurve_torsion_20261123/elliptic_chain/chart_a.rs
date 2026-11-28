use crate::field::{Field, Q};
use crate::inherited::{padd, peval, pmod, pmonic, pmul, pscale, ptrim, pxgcd, Div, Fld, Hyp};

pub type Divisor = Div<u32>;

pub fn key(d: &Divisor) -> Vec<u32> {
    let mut result = vec![d.u.len() as u32];
    result.extend_from_slice(&d.u);
    result.push(d.v.len() as u32);
    result.extend_from_slice(&d.v);
    result
}

pub fn homogeneous(f: &Field, p: &[u32], weight: usize, r: u32) -> Vec<u32> {
    assert!(p.len() <= weight + 1);
    let mut result = vec![0; weight + 1];
    for (i, c) in p.iter().enumerate() {
        let mut choose = 1;
        for j in 0..=i {
            let coefficient = f.mul(c, &f.mul(&(choose % 3), &f.pow(r, (i - j) as u32)));
            result[weight - j] = f.add(&result[weight - j], &coefficient);
            if j < i { choose = choose * (i - j) as u32 / (j + 1) as u32; }
        }
    }
    ptrim(f, &mut result);
    result
}

pub struct Chart<'a> {
    pub h: Hyp<'a, Field>,
    pub model: Vec<u32>,
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
    pub fn new(f: &'a Field, model: &[u32]) -> Self {
        assert_eq!(model.len(), 7);
        assert!(model.iter().all(|&c| c < 3));
        let roots: Vec<_> = (0..Q).filter(|x| peval(f, model, x) == 0).collect();
        assert_eq!(roots.len(), 6);
        assert!(roots.iter().all(|&r| f.pow(r, 3) != r && f.pow(r, 9) != r));
        let r = roots[0];
        let g = homogeneous(f, model, 6, r);
        assert_eq!(g.len(), 6);
        let leading = g[5];
        let scale = f.sqrt(leading).expect("frozen F3^6 monic-chart square root absent");
        assert_eq!(f.mul(&scale, &scale), leading);
        let monic = pscale(f, &g, &f.inv(&leading));
        assert_eq!(monic[5], 1);
        let mut derivative: Vec<_> = monic.iter().enumerate().skip(1)
            .map(|(i, c)| f.mul(c, &(i as u32 % 3))).collect();
        ptrim(f, &mut derivative);
        assert_eq!(pxgcd(f, &monic, &derivative).0.len(), 1);
        let h = Hyp { f, h: monic, g: 2 };
        let delta = f.sub(&f.pow(r, 3), &r);
        let pole = f.inv(&f.sub(&f.pow(r, 243), &r));
        let pre = Self::point_chart(&h, pole, 0);
        let post = Self::point_chart(&h, f.inv(&delta), 0);
        assert_eq!(h.add(&pre, &pre), h.identity());
        assert_eq!(h.add(&post, &post), h.identity());
        Self { h, model: model.to_vec(), roots, r, scale, leading, delta, pole, pre, post }
    }

    pub fn point_homogeneous(&self, a: u32, b: u32, ordinate: u32) -> Divisor {
        assert!(a != 0 || b != 0);
        let f = self.h.f;
        let mut equation = 0;
        for (i, c) in self.model.iter().enumerate() {
            let term = f.mul(c, &f.mul(&f.pow(a, i as u32), &f.pow(b, (6 - i) as u32)));
            equation = f.add(&equation, &term);
        }
        assert_eq!(f.mul(&ordinate, &ordinate), equation);
        let denominator = f.sub(&a, &f.mul(&self.r, &b));
        if denominator == 0 {
            assert_eq!(ordinate, 0);
            return self.h.identity();
        }
        let t = f.mul(&b, &f.inv(&denominator));
        let z = f.mul(&ordinate, &f.inv(&f.mul(&f.pow(denominator, 3), &self.scale)));
        Self::point_chart(&self.h, t, z)
    }

    pub fn controls(&self) {
        assert_eq!(self.frobenius(&self.pre), self.post);
        let naive = self.push_regular(&self.post);
        let correct = self.frobenius(&self.post);
        assert_ne!(naive, correct);
        let root9 = self.h.f.pow(self.r, 9);
        let expected = self.h.sub(&self.point_homogeneous(root9, 1, 0), &self.post);
        assert_eq!(correct, expected);
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
