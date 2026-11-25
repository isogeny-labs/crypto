#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct K(pub u16);
pub const Z: K = K(0);
pub const I: K = K(1);

impl K {
    pub fn encoded(n: u16) -> Self {
        assert!(n < 2401);
        Self(n)
    }
    pub fn prime(n: i64) -> Self {
        Self(n.rem_euclid(7) as u16)
    }
    fn coefficients(self) -> [i64; 4] {
        let mut n = self.0;
        std::array::from_fn(|_| {
            let digit = n % 7;
            n /= 7;
            i64::from(digit)
        })
    }
    fn pack(c: [i64; 4]) -> Self {
        let mut n = 0;
        for digit in c.into_iter().rev() {
            n = n * 7 + digit.rem_euclid(7) as u16;
        }
        Self(n)
    }
    pub fn sum(self, rhs: Self) -> Self {
        let a = self.coefficients();
        let b = rhs.coefficients();
        Self::pack(std::array::from_fn(|i| a[i] + b[i]))
    }
    pub fn opposite(self) -> Self {
        Self::pack(self.coefficients().map(|c| -c))
    }
    pub fn difference(self, rhs: Self) -> Self {
        self.sum(rhs.opposite())
    }
    pub fn product(self, rhs: Self) -> Self {
        let mut c = [0_i64; 7];
        for (i, a) in self.coefficients().into_iter().enumerate() {
            for (j, b) in rhs.coefficients().into_iter().enumerate() {
                c[i + j] += a * b;
            }
        }
        for degree in (4..7).rev() {
            let leading = c[degree].rem_euclid(7);
            c[degree] = 0;
            c[degree - 2] -= 5 * leading;
            c[degree - 4] -= 5 * leading;
        }
        Self::pack(c[..4].try_into().expect("four coefficients"))
    }
    pub fn power(self, mut exponent: u32) -> Self {
        let mut value = I;
        let mut base = self;
        while exponent != 0 {
            if exponent & 1 != 0 {
                value = value.product(base);
            }
            base = base.product(base);
            exponent >>= 1;
        }
        value
    }
    pub fn inverse(self) -> Self {
        assert_ne!(self, Z);
        let value = self.power(2399);
        assert_eq!(value.product(self), I);
        value
    }
    pub fn quotient(self, rhs: Self) -> Self {
        self.product(rhs.inverse())
    }
}

pub fn field_controls() {
    let a = K::encoded(7);
    let b = a.product(a).difference(I);
    assert_eq!(b.product(b), K::prime(3));
    assert_eq!(I.sum(b).product(I.difference(b)), K::prime(5));
    assert_eq!(K::prime(3).power(3), K::prime(-1));
    assert_eq!(K::prime(5).power(3), K::prime(-1));
    assert_eq!(a.power(2401), a);
    println!("AUDIT_FIELD\ttower_nonsquares=3,5\tcoefficient_long_reduction\tgenerator_Frobenius\tno_field_population_search\tPASS");
}

// Bareiss cross-products with previous-pivot division; no normalized RREF.
pub fn determinant(mut a: Vec<Vec<K>>) -> K {
    let n = a.len();
    assert!(a.iter().all(|row| row.len() == n));
    if n == 0 {
        return I;
    }
    let mut sign = I;
    let mut previous = I;
    for k in 0..n - 1 {
        let Some(row) = (k..n).find(|&row| a[row][k] != Z) else {
            return Z;
        };
        if row != k {
            a.swap(k, row);
            sign = sign.opposite();
        }
        let pivot = a[k][k];
        let pivot_row = a[k].clone();
        for row in a.iter_mut().skip(k + 1) {
            let entry = row[k];
            for (column, value) in row.iter_mut().enumerate().skip(k + 1) {
                *value = pivot
                    .product(*value)
                    .difference(entry.product(pivot_row[column]))
                    .quotient(previous);
            }
            row[k] = Z;
        }
        previous = pivot;
    }
    sign.product(a[n - 1][n - 1])
}

pub fn dot(a: &[K], b: &[K]) -> K {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).fold(Z, |s, (&a, &b)| s.sum(a.product(b)))
}
