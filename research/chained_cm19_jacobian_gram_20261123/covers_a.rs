use crate::field::Field;

pub type EPoint = Option<(usize, usize)>;
pub type CPoint = [usize; 3];
pub type Fibre = Vec<(CPoint, usize)>;

pub const R: [[usize; 3]; 3] = [[1, 5, 2], [0, 5, 0], [2, 1, 1]];
pub const R_INV: [[usize; 3]; 3] = [[1, 5, 5], [0, 5, 0], [5, 6, 1]];
pub const P: [[usize; 3]; 3] = [[1, 1, 1], [5, 0, 0], [0, 1, 6]];
pub const P_INV: [[usize; 3]; 3] = [[0, 3, 0], [4, 2, 4], [4, 2, 3]];

pub struct Models<'a> {
    pub f: &'a Field,
}

impl Models<'_> {
    pub fn normalize(&self, p: CPoint) -> CPoint {
        let pivot = p.iter().copied().find(|&x| x != 0).expect("projective zero");
        p.map(|x| self.f.div(x, pivot))
    }

    pub fn apply(&self, m: [[usize; 3]; 3], p: CPoint) -> CPoint {
        self.normalize(m.map(|row| row.into_iter().zip(p).fold(0, |s, (a, b)| self.f.add(s, self.f.mul(a, b)))))
    }

    pub fn evalid(&self, p: EPoint) -> bool {
        p.is_none_or(|(x, y)| self.f.square(y) == self.f.add(self.f.add(self.f.pow(x, 3), self.f.mul(2, x)), 6))
    }

    pub fn eneg(&self, p: EPoint) -> EPoint {
        p.map(|(x, y)| (x, self.f.neg(y)))
    }

    pub fn eadd(&self, p: EPoint, q: EPoint) -> EPoint {
        assert!(self.evalid(p) && self.evalid(q));
        let (Some((x, y)), Some((u, v))) = (p, q) else { return p.or(q); };
        if x == u && self.f.add(y, v) == 0 { return None; }
        let slope = if x == u {
            self.f.div(self.f.add(self.f.mul(3, self.f.square(x)), 2), self.f.mul(2, y))
        } else { self.f.div(self.f.sub(v, y), self.f.sub(u, x)) };
        let xx = self.f.sub(self.f.sub(self.f.square(slope), x), u);
        let yy = self.f.sub(self.f.mul(slope, self.f.sub(x, xx)), y);
        let out = Some((xx, yy));
        assert!(self.evalid(out));
        out
    }

    pub fn emul(&self, p: EPoint, n: usize) -> EPoint {
        (0..n).fold(None, |s, _| self.eadd(s, p))
    }

    pub fn cvalid(&self, [x, y, z]: CPoint) -> bool {
        let g = self.f.add(self.f.add(self.f.mul(6, self.f.square(x)), self.f.mul(3, self.f.mul(x, y))), self.f.square(y));
        let left = self.f.square(self.f.add(self.f.square(z), g));
        let right = self.f.mul(y, self.f.add(self.f.add(self.f.pow(x, 3), self.f.mul(2, self.f.mul(x, self.f.square(y)))), self.f.mul(6, self.f.pow(y, 3))));
        left == right && [x, y, z] != [0, 0, 0]
    }

    fn q0(&self, [x, y, z]: CPoint) -> EPoint {
        if y == 0 { return None; }
        let g = self.f.add(self.f.add(self.f.mul(6, self.f.square(x)), self.f.mul(3, self.f.mul(x, y))), self.f.square(y));
        Some((self.f.div(x, y), self.f.div(self.f.add(self.f.square(z), g), self.f.square(y))))
    }

    pub fn q3(&self, p: CPoint) -> EPoint {
        let [x, y, z] = self.apply(P_INV, p);
        if x == 0 {
            assert_ne!(y == 0, z == 0);
            return if y == 0 { None } else { Some((1, 3)) };
        }
        let u = self.f.div(self.f.mul(y, z), self.f.square(x));
        let v = self.f.div(self.f.pow(y, 3), self.f.pow(x, 3));
        let h = self.f.add(self.f.add(1, self.f.mul(3, u)), self.f.mul(2, self.f.square(u)));
        let w = self.f.add(self.f.mul(4, v), h);
        let delta = self.f.add(w, self.f.add(self.f.add(self.f.mul(2, self.f.square(u)), self.f.mul(6, u)), 3));
        Some((self.f.add(delta, 1), self.f.add(self.f.mul(self.f.add(self.f.mul(5, u), 4), delta), 4)))
    }

    pub fn forward(&self, i: usize, p: CPoint) -> EPoint {
        assert!(self.cvalid(p));
        let out = match i { 0 => self.q0(p), 1 => self.q0(self.apply(R, p)), 2 => self.q3(p), _ => panic!("cover index") };
        assert!(self.evalid(out));
        out
    }

    fn pull0(&self, p: EPoint) -> Fibre {
        let Some((x, v)) = p else { return vec![([1, 0, 1], 1), ([1, 0, 6], 1)]; };
        let g = self.f.add(self.f.add(self.f.mul(6, self.f.square(x)), self.f.mul(3, x)), 1);
        let n = self.f.sub(v, g);
        (0..self.f.q).filter(|&z| self.f.square(z) == n).map(|z| (self.normalize([x, 1, z]), if z == 0 { 2 } else { 1 })).collect()
    }

    fn pull3(&self, p: EPoint) -> Fibre {
        if p.is_none() { return vec![(self.apply(P, [0, 0, 1]), 3)]; }
        if p == Some((1, 3)) { return vec![(self.apply(P, [0, 1, 0]), 3)]; }
        let (x, y) = p.expect("finite target");
        let delta = self.f.sub(x, 1);
        let (u, w) = if delta == 0 {
            assert_eq!(p, Some((1, 4)));
            (3, 3)
        } else {
            let u = self.f.mul(3, self.f.sub(self.f.div(self.f.sub(y, 4), delta), 4));
            let w = self.f.sub(delta, self.f.add(self.f.add(self.f.mul(2, self.f.square(u)), self.f.mul(6, u)), 3));
            (u, w)
        };
        let h = self.f.add(self.f.add(1, self.f.mul(3, u)), self.f.mul(2, self.f.square(u)));
        let v = self.f.div(self.f.sub(w, h), 4);
        if v == 0 {
            assert_eq!(u, 0);
            return (0..self.f.q).filter(|&z| self.f.pow(z, 3) == 3).map(|z| (self.apply(P, [1, 0, z]), 1)).collect();
        }
        (0..self.f.q).filter(|&t| self.f.pow(t, 3) == v).map(|t| (self.apply(P, [1, t, self.f.div(u, t)]), 1)).collect()
    }

    pub fn fibre(&self, j: usize, p: EPoint) -> Fibre {
        assert!(self.evalid(p));
        let mut out = match j {
            0 => self.pull0(p),
            1 => self.pull0(p).into_iter().map(|(q, m)| (self.apply(R_INV, q), m)).collect(),
            2 => self.pull3(p),
            _ => panic!("cover index"),
        };
        out.sort_unstable();
        assert_eq!(out.iter().map(|x| x.1).sum::<usize>(), if j == 2 { 3 } else { 2 });
        assert!(out.windows(2).all(|w| w[0].0 != w[1].0));
        for &(q, multiplicity) in &out {
            assert!(self.cvalid(q));
            assert_eq!(self.forward(j, q), p);
            let frob = self.normalize(q.map(|a| self.f.pow(a, 7)));
            assert!(out.contains(&(frob, multiplicity)));
            assert_eq!(self.apply(P, self.apply(P_INV, q)), q);
            assert_eq!(self.apply(R, self.apply(R_INV, q)), q);
        }
        out
    }

    pub fn push_sum(&self, i: usize, fibre: &Fibre) -> EPoint {
        fibre.iter().fold(None, |s, &(q, m)| self.eadd(s, self.emul(self.forward(i, q), m)))
    }
}
