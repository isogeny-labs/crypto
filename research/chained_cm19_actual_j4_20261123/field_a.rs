#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct K(pub u32);
pub const Z: K = K(0);
pub const I: K = K(1);
pub const Q: u32 = 117649;

impl K {
    pub fn encoded(n: u32) -> Self {
        assert!(n < Q);
        Self(n)
    }
    pub fn prime(n: i64) -> Self {
        Self(n.rem_euclid(7) as u32)
    }
    fn coefficients(self) -> [i64; 6] {
        let mut value = self.0;
        std::array::from_fn(|_| {
            let digit = value % 7;
            value /= 7;
            i64::from(digit)
        })
    }
    fn pack(coefficients: [i64; 6]) -> Self {
        Self(
            coefficients
                .into_iter()
                .rev()
                .fold(0, |s, c| s * 7 + c.rem_euclid(7) as u32),
        )
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
        fn cubic(a: &[i64], b: &[i64]) -> [i64; 3] {
            let mut c = [0_i64; 5];
            for (i, &x) in a.iter().enumerate() {
                for (j, &y) in b.iter().enumerate() {
                    c[i + j] += x * y;
                }
            }
            for d in (3..5).rev() {
                let t = c[d].rem_euclid(7);
                c[d] = 0;
                c[d - 3] += t;
                c[d - 2] += 5 * t;
            }
            [c[0].rem_euclid(7), c[1].rem_euclid(7), c[2].rem_euclid(7)]
        }
        let a = self.coefficients();
        let b = rhs.coefficients();
        let ac = cubic(&a[..3], &b[..3]);
        let bd = cubic(&a[3..], &b[3..]);
        let ab: [i64; 3] = std::array::from_fn(|j| (a[j] + a[j + 3]).rem_euclid(7));
        let cd: [i64; 3] = std::array::from_fn(|j| (b[j] + b[j + 3]).rem_euclid(7));
        let cross = cubic(&ab, &cd);
        Self::pack(std::array::from_fn(|j| {
            if j < 3 {
                ac[j] + 3 * bd[j]
            } else {
                cross[j - 3] - ac[j - 3] - bd[j - 3]
            }
        }))
    }
    pub fn power(self, mut exponent: u32) -> Self {
        let mut a = self;
        let mut result = I;
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = result.product(a);
            }
            a = a.product(a);
            exponent >>= 1;
        }
        result
    }
    pub fn inverse(self) -> Self {
        assert_ne!(self, Z);
        let value = self.power(Q - 2);
        assert_eq!(self.product(value), I);
        value
    }
    pub fn quotient(self, rhs: Self) -> Self {
        self.product(rhs.inverse())
    }
    pub fn sqrt(self) -> Option<Self> {
        if self == Z {
            return Some(Z);
        }
        let eta = I.sum(K::encoded(343)).power(7353);
        let base = self.power(3677);
        let mut phase = I;
        for _ in 0..8 {
            let root = base.product(phase);
            if root.product(root) == self {
                return Some(root.min(root.opposite()));
            }
            phase = phase.product(eta);
        }
        None
    }
}

pub fn determinant(a: Vec<Vec<K>>) -> K {
    let n = a.len();
    assert!(a.iter().all(|row| row.len() == n));
    match n {
        0 => I,
        1 => a[0][0],
        2 => a[0][0]
            .product(a[1][1])
            .difference(a[0][1].product(a[1][0])),
        3 => {
            let mut positive = Z;
            let mut negative = Z;
            for j in 0..3 {
                positive = positive.sum(
                    a[0][j]
                        .product(a[1][(j + 1) % 3])
                        .product(a[2][(j + 2) % 3]),
                );
                negative = negative.sum(
                    a[0][j]
                        .product(a[1][(j + 2) % 3])
                        .product(a[2][(j + 1) % 3]),
                );
            }
            positive.difference(negative)
        }
        _ => panic!("only degree2/3 multiplication norms"),
    }
}

pub fn qualify() {
    let a = K::encoded(7);
    let b = K::encoded(343);
    assert!((0..7).all(|x| K::prime(x)
        .power(3)
        .sum(K::prime(2).product(K::prime(x)))
        .sum(K::prime(6))
        != Z));
    assert_eq!(a.power(3).sum(K::prime(2).product(a)).sum(K::prime(6)), Z);
    assert_eq!(b.power(2), K::prime(3));
    assert_eq!(K::prime(3).power(171), K::prime(-1));
    assert_eq!(I.sum(b).product(I.difference(b)), K::prime(5));
    assert_eq!(K::prime(5).power(171), K::prime(-1));
    assert_eq!(a.power(343), a);
    assert_eq!(b.power(343), b.opposite());
    assert_eq!(a.power(Q), a);
    assert_eq!(b.power(Q), b);
    let eta = I.sum(b).power(7353);
    assert_eq!(eta.power(16), I);
    assert_eq!(eta.power(8), K::prime(-1));
    let i = b.product(K::prime(3));
    assert_eq!(i.power(2), K::prime(-1));
    assert_eq!(I.sqrt(), Some(I));
    assert_eq!(Z.sqrt(), Some(Z));
    assert_eq!(I.sum(b).sqrt(), None);
    for value in [a, b, a.sum(b), eta, i] {
        let square = value.power(2);
        assert_eq!(square.sqrt().expect("actual square").power(2), square);
        assert_eq!(value.product(value.inverse()), I);
    }
    println!("FIELD\tq={}\ta3+2a+6\tb2=3\teta={:?}\ti={:?}\tfull_tower_nonsquare_Frobenius_and_fixed_root_phase_controls\tPASS",Q,eta,i);
}
