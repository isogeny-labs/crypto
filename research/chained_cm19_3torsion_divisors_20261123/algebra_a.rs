use crate::field::{F, ONE, ZERO};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ring {
    pub degree: usize,
    pub constant: F,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct A {
    pub ring: Ring,
    pub values: [F; 3],
}

impl Ring {
    pub fn new(degree: usize, constant: F) -> Self {
        assert!(degree == 2 || degree == 3);
        Self { degree, constant }
    }

    pub fn scalar(self, value: F) -> A {
        A {
            ring: self,
            values: [value, ZERO, ZERO],
        }
    }

    pub fn variable(self) -> A {
        A {
            ring: self,
            values: [ZERO, ONE, ZERO],
        }
    }

    pub fn modulus(self) -> Vec<F> {
        let mut out = vec![ZERO; self.degree + 1];
        out[0] = self.constant.negative();
        out[self.degree] = ONE;
        out
    }
}

impl A {
    pub fn plus(self, rhs: Self) -> Self {
        assert_eq!(self.ring, rhs.ring);
        Self {
            ring: self.ring,
            values: std::array::from_fn(|i| self.values[i].plus(rhs.values[i])),
        }
    }

    pub fn negative(self) -> Self {
        Self {
            ring: self.ring,
            values: self.values.map(F::negative),
        }
    }

    pub fn minus(self, rhs: Self) -> Self {
        self.plus(rhs.negative())
    }

    pub fn times(self, rhs: Self) -> Self {
        assert_eq!(self.ring, rhs.ring);
        let degree = self.ring.degree;
        let mut product = [ZERO; 5];
        for (i, &x) in self.values[..degree].iter().enumerate() {
            for (j, &y) in rhs.values[..degree].iter().enumerate() {
                product[i + j] = product[i + j].plus(x.times(y));
            }
        }
        for k in (degree..5).rev() {
            product[k - degree] = product[k - degree].plus(product[k].times(self.ring.constant));
            product[k] = ZERO;
        }
        Self {
            ring: self.ring,
            values: std::array::from_fn(|i| product[i]),
        }
    }

    pub fn power(self, n: u8) -> Self {
        (0..n).fold(self.ring.scalar(ONE), |s, _| s.times(self))
    }

    pub fn scale(self, value: F) -> Self {
        Self {
            ring: self.ring,
            values: self.values.map(|x| x.times(value)),
        }
    }
}

pub fn remainder(mut p: Vec<F>, q: &[F]) -> Vec<F> {
    trim(&mut p);
    assert!(!q.is_empty() && q.last() != Some(&ZERO));
    while p.len() >= q.len() {
        let shift = p.len() - q.len();
        let factor = p[p.len() - 1].over(q[q.len() - 1]);
        for (i, &x) in q.iter().enumerate() {
            p[i + shift] = p[i + shift].minus(factor.times(x));
        }
        trim(&mut p);
    }
    p
}

fn trim(p: &mut Vec<F>) {
    while p.last() == Some(&ZERO) {
        p.pop();
    }
}

pub fn unit(value: A) -> bool {
    let mut a = value.ring.modulus();
    let mut b = value.values[..value.ring.degree].to_vec();
    trim(&mut b);
    while !b.is_empty() {
        let r = remainder(a, &b);
        a = b;
        b = r;
    }
    a.len() == 1
}
