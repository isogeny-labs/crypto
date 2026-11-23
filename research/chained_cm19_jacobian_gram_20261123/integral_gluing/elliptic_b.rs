use crate::field::Field;
pub type EPoint = Option<(usize, usize)>;
pub struct Models<'a> {
    pub f: &'a Field,
}
impl Models<'_> {
    pub fn evalid(&self, p: EPoint) -> bool {
        p.is_none_or(|(x, y)| {
            self.f.square(y)
                == self
                    .f
                    .add(self.f.add(self.f.pow(x, 3), self.f.mul(2, x)), 6)
        })
    }

    pub fn eneg(&self, p: EPoint) -> EPoint {
        p.map(|(x, y)| (x, self.f.neg(y)))
    }

    pub fn eadd(&self, p: EPoint, q: EPoint) -> EPoint {
        assert!(self.evalid(p) && self.evalid(q));
        let (Some((x, y)), Some((u, v))) = (p, q) else {
            return p.or(q);
        };
        if x == u && self.f.add(y, v) == 0 {
            return None;
        }
        let slope = if x == u {
            self.f.div(
                self.f.add(self.f.mul(3, self.f.square(x)), 2),
                self.f.mul(2, y),
            )
        } else {
            self.f.div(self.f.sub(v, y), self.f.sub(u, x))
        };
        let xx = self.f.sub(self.f.sub(self.f.square(slope), x), u);
        let yy = self.f.sub(self.f.mul(slope, self.f.sub(x, xx)), y);
        let out = Some((xx, yy));
        assert!(self.evalid(out));
        out
    }

    pub fn emul(&self, p: EPoint, n: usize) -> EPoint {
        (0..n).fold(None, |s, _| self.eadd(s, p))
    }
}
