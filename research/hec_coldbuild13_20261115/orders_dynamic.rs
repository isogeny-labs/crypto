use num_bigint::{BigInt, BigUint};
type Matrix = Vec<Vec<BigInt>>;
fn eye() -> Matrix {
    (0..6)
        .map(|i| (0..6).map(|j| BigInt::from((i == j) as i64)).collect())
        .collect()
}
fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    (0..6)
        .map(|i| {
            (0..6)
                .map(|j| (0..6).fold(BigInt::from(0), |s, k| s + &a[i][k] * &b[k][j]))
                .collect()
        })
        .collect()
}
fn det(mut a: Matrix) -> BigInt {
    let mut prev = BigInt::from(1);
    let mut sign = 1;
    for k in 0..5 {
        let i = (k..6).find(|&i| a[i][k] != BigInt::from(0)).unwrap();
        if i != k {
            a.swap(i, k);
            sign = -sign
        }
        let pivot = a[k][k].clone();
        for i in k + 1..6 {
            for j in k + 1..6 {
                let n = &a[i][j] * &pivot - &a[i][k] * &a[k][j];
                assert_eq!(&n % &prev, BigInt::from(0));
                a[i][j] = n / &prev;
            }
            a[i][k] = BigInt::from(0);
        }
        prev = pivot;
    }
    a[5][5].clone() * sign
}
#[allow(non_snake_case)]
pub fn extension_order(L: [i64; 7], ell: u64) -> (BigUint, BigUint, u32) {
    let zero = BigInt::from(0);
    let mut s = vec![zero.clone(); 79];
    for n in 1..=78 {
        let mut z = zero.clone();
        if n <= 6 {
            z += BigInt::from(n as i64) * L[n];
        }
        for i in 1..=6.min(n - 1) {
            z += BigInt::from(L[i]) * &s[n - i];
        }
        s[n] = -z;
    }
    let mut b = vec![BigInt::from(1)];
    for k in 1..=6 {
        let z = (1..=k).fold(zero.clone(), |s0, i| s0 + &b[k - i] * &s[13 * i]);
        assert_eq!(&z % k, zero);
        b.push(-z / k);
    }
    let q = BigInt::from(127).pow(13);
    for j in 0..=3 {
        assert_eq!(b[6 - j], q.pow((3 - j) as u32) * &b[j]);
    }
    let n = b.iter().fold(zero.clone(), |s, x| s + x);
    let mut m = vec![vec![zero.clone(); 6]; 6];
    for i in 0..5 {
        m[i + 1][i] = BigInt::from(1);
    }
    for i in 0..6 {
        m[i][5] = BigInt::from(-L[6 - i]);
    }
    let mut power = eye();
    let mut e = 13;
    while e != 0 {
        if e & 1 != 0 {
            power = mul(&power, &m)
        }
        e >>= 1;
        if e != 0 {
            m = mul(&m, &m)
        }
    }
    let id = eye();
    let diff = (0..6)
        .map(|i| (0..6).map(|j| &id[i][j] - &power[i][j]).collect())
        .collect();
    assert_eq!(n, det(diff));
    let n = n.to_biguint().unwrap();
    assert_eq!(&n % ell, BigUint::from(0u32));
    let mut h = n.clone();
    let mut v = 0;
    while &h % 3u32 == BigUint::from(0u32) {
        h /= 3u32;
        v += 1;
    }
    assert!(v >= 6);
    (n, h, v)
}
