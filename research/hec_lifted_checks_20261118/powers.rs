use hec_native_ic::jac::{Curve, Div};

pub struct Powers {
    points: Vec<Div>,
    ell: u64,
    mode: usize,
    audit: bool,
}

fn naf(mut k: u64) -> (u64, u64) {
    let (mut plus, mut minus, mut bit) = (0, 0, 1);
    while k != 0 {
        if k & 1 != 0 {
            if k & 3 == 3 {
                minus |= bit;
                k += 1;
            } else {
                plus |= bit;
            }
        }
        k >>= 1;
        bit <<= 1;
    }
    (plus, minus)
}

impl Powers {
    pub fn checked(
        c: &Curve,
        d: &Div,
        t: &Div,
        ell: u64,
        mode: usize,
        audit: bool,
    ) -> Option<Self> {
        if !(3..1u64 << 62).contains(&ell)
            || !(2..).take_while(|q| q * q <= ell).all(|q| ell % q != 0)
        {
            return None;
        }
        if !c.is_valid(d) || !c.is_valid(t) || c.is_zero(d) {
            return None;
        }
        let bits = (u64::BITS - ell.leading_zeros()) as usize;
        let mut points = Vec::with_capacity(bits);
        points.push(*d);
        for i in 1..bits {
            let p = &points[i - 1];
            let next = if mode >= 2 {
                crate::lift::double(c, p)
            } else {
                c.add(p, p)
            };
            if audit {
                assert_eq!(next, c.add(p, p));
            }
            points.push(next);
        }
        let table = Self {
            points,
            ell,
            mode,
            audit,
        };
        if !c.is_zero(&table.sum(c, ell, 0)) {
            return None;
        }
        let mut root = 1u128;
        while root * root < c.fp.p as u128 {
            root += 1;
        }
        let upper = (root + 1).pow((2 * c.g) as u32);
        ((ell as u128) * (ell as u128) > upper).then_some(table)
    }

    fn sum(&self, c: &Curve, plus: u64, minus: u64) -> Div {
        let mut r = c.zero();
        let mut used = plus | minus;
        while used != 0 {
            let i = used.trailing_zeros() as usize;
            let p = &self.points[i];
            let q = if (minus >> i) & 1 != 0 { c.neg(p) } else { *p };
            let next = if self.mode == 3 {
                crate::lift::add(c, &r, &q)
            } else {
                c.add(&r, &q)
            };
            if self.audit {
                assert_eq!(next, c.add(&r, &q));
            }
            r = next;
            used &= used - 1;
        }
        r
    }

    pub fn product(&self, c: &Curve, k: u64, signed: bool) -> Div {
        assert!(k < self.ell);
        let mut best = (k, 0u64);
        if signed {
            let complement = self.ell - k;
            let (p, n) = naf(k);
            let (cp, cn) = naf(complement);
            for (a, b) in [(0, complement), (p, n), (cn, cp)] {
                if (a | b) >> self.points.len() == 0
                    && (a | b).count_ones() < (best.0 | best.1).count_ones()
                {
                    best = (a, b);
                }
            }
        }
        self.sum(c, best.0, best.1)
    }
}
