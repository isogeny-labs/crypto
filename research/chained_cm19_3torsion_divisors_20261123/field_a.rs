#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct F(pub u16);

pub const ZERO: F = F(0);
pub const ONE: F = F(1);
pub const Q: usize = 2401;

impl F {
    pub fn c(value: i32) -> Self {
        Self(value.rem_euclid(7) as u16)
    }

    fn digits(self) -> [u32; 4] {
        let mut n = self.0 as u32;
        let mut out = [0; 4];
        for x in &mut out {
            *x = n % 7;
            n /= 7;
        }
        assert_eq!(n, 0);
        out
    }

    fn encoded(values: [u32; 4]) -> Self {
        Self(values.into_iter().rev().fold(0, |s, x| 7 * s + x) as u16)
    }

    pub fn plus(self, rhs: Self) -> Self {
        let a = self.digits();
        let b = rhs.digits();
        Self::encoded(std::array::from_fn(|i| (a[i] + b[i]) % 7))
    }

    pub fn minus(self, rhs: Self) -> Self {
        self.plus(rhs.negative())
    }

    pub fn negative(self) -> Self {
        Self::encoded(self.digits().map(|x| (7 - x) % 7))
    }

    pub fn times(self, rhs: Self) -> Self {
        let a = self.digits();
        let b = rhs.digits();
        let mut product = [0_u32; 7];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                product[i + j] += x * y;
            }
        }
        for k in (4..7).rev() {
            let coefficient = product[k] % 7;
            product[k - 4] = (product[k - 4] + 2 * coefficient) % 7;
            product[k - 2] = (product[k - 2] + 2 * coefficient) % 7;
            product[k] = 0;
        }
        Self::encoded(std::array::from_fn(|i| product[i] % 7))
    }

    pub fn power(self, mut n: usize) -> Self {
        let mut out = ONE;
        let mut base = self;
        while n != 0 {
            if n & 1 == 1 {
                out = out.times(base);
            }
            base = base.times(base);
            n >>= 1;
        }
        out
    }

    pub fn inverse(self) -> Self {
        assert_ne!(self, ZERO);
        self.power(Q - 2)
    }

    pub fn over(self, rhs: Self) -> Self {
        self.times(rhs.inverse())
    }
}

pub fn qualify() {
    let a = F(7);
    let b = a.power(2).minus(ONE);
    assert_eq!(a.power(4), F::c(2).times(a.power(2)).plus(F::c(2)));
    assert_eq!(b.power(2), F::c(3));
    assert_eq!(ONE.plus(b).power(8), F::c(5));
    assert_eq!(F::c(5).power(3), F::c(6));
    for encoded in 0..Q {
        let x = F(encoded as u16);
        assert_eq!(x.plus(x.negative()), ZERO);
        assert_eq!(x.power(Q), x);
        if x != ZERO {
            assert_eq!(x.times(x.inverse()), ONE);
        }
    }
    for x in 0..7 {
        for y in 0..7 {
            assert_eq!(F::c(x).plus(F::c(y)), F::c(x + y));
            assert_eq!(F::c(x).times(F::c(y)), F::c(x * y));
        }
    }
    println!("FIELD\tq=2401\tmodulus=a4+5a2+5\tall2401_Frobenius_inverse_controls\tno_q_squared_table\tPASS");
}
