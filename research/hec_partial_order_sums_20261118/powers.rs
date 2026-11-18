use hec_native_ic::jac::{Curve, Div};

pub struct Powers {
    pub partial: Vec<(u64,Div)>,
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
    pub fn multiples(&self) -> &[Div] { &self.points }
    pub fn checked<const KEEP:bool>(
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
            let next = crate::lift::double(c, p, mode, audit);
            if audit {
                assert_eq!(next, c.add(p, p));
            }
            points.push(next);
        }
        let mut table = Self { partial:Vec::new(),
            points,
            ell,
            mode,
            audit,
        };
        let (positive, negative) = naf(ell);
        let extra = positive >> bits != 0;
        let signed_cost = (positive | negative).count_ones() + u32::from(extra);
        let (plus, minus) = if signed_cost < ell.count_ones() {
            (positive, negative)
        } else {
            (ell, 0)
        };
        // Compare the top power to the negation of the remaining signed sum.
        // Keep any temporary double out of the relation and verification tables.
        let top = 63 - plus.leading_zeros() as usize;
        let high = if top == bits {
            crate::lift::double(c, &table.points[bits - 1], mode, audit)
        } else {
            table.points[top]
        };
        let lower_minus = plus ^ (1u64 << top);
        let mut partial = Vec::new();
        let mask = minus | lower_minus;
        let lower = table.sum(c, minus, lower_minus, |remaining, point| {
            if !KEEP || remaining == 0 {return;}
            let used = mask ^ remaining;
            if used.count_ones() < 2 {return;}
            let scalar = ((used & minus) as i128 - (used & lower_minus) as i128)
                .rem_euclid(ell as i128) as u64;
            if scalar!=0 && !scalar.is_power_of_two() && !(ell-scalar).is_power_of_two() {
                if audit {assert_eq!(*point,c.mul(d,scalar));}
                partial.push((scalar,*point));
            }
        });
        table.partial = partial;

        if lower != high {
            return None;
        }
        let mut root = 1u128;
        while root * root < c.fp.p as u128 {
            root += 1;
        }
        let upper = (root + 1).pow((2 * c.g) as u32);
        ((ell as u128) * (ell as u128) > upper).then_some(table)
    }

    fn sum(&self, c: &Curve, plus: u64, minus: u64, mut record: impl FnMut(u64,&Div)) -> Div {
        let mut r = c.zero();
        let mut used = plus | minus;
        while used != 0 {
            let i = used.trailing_zeros() as usize;
            let p = &self.points[i];
            let q = if (minus >> i) & 1 != 0 { c.neg(p) } else { *p };
            let next = crate::lift::add(c, &r, &q, self.mode, self.audit);
            if self.audit {
                assert_eq!(next, c.add(&r, &q));
            }
            r = next;
            used &= used - 1;
            record(used,&r);
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
        self.sum(c, best.0, best.1, |_,_|{})
    }
}
