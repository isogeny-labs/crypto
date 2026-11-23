use hec_native_ic::{fp::Fp, poly::Poly};
pub fn weil(p: u64, counts: [u64; 4]) -> [i128; 9] {
    let mut c = [0i128; 5];
    c[0] = 1;
    let mut sums = [0i128; 5];
    for k in 1..=4 {
        sums[k] = p.pow(k as u32) as i128 + 1 - counts[k - 1] as i128;
        assert!(sums[k] * sums[k] <= 64 * p.pow(k as u32) as i128);
        let sum: i128 = (1..=k).map(|i| c[k - i] * sums[i]).sum();
        assert_eq!(sum % k as i128, 0);
        c[k] = -sum / k as i128
    }
    let q = p as i128;
    [
        q.pow(4),
        q.pow(3) * c[1],
        q * q * c[2],
        q * c[3],
        c[4],
        c[3],
        c[2],
        c[1],
        1,
    ]
}
fn divides(f: &[i128], g: &[i128]) -> bool {
    let mut v = f.to_vec();
    let d = g.len() - 1;
    assert_eq!(g[d], 1);
    for k in (d..v.len()).rev() {
        let a = v[k];
        for j in 0..=d {
            v[k - d + j] = v[k - d + j]
                .checked_sub(a.checked_mul(g[j]).unwrap())
                .unwrap()
        }
    }
    v.iter().all(|&x| x == 0)
}
pub fn proper_factor(p: u64, f: &[i128; 9]) -> Option<Vec<i128>> {
    let mut s = 1i128;
    while s * s < (p as i128) {
        s += 1
    }
    for a in -2 * s..=2 * s {
        if a * a > 4 * p as i128 {
            continue;
        }
        let g = vec![p as i128, a, 1];
        if divides(f, &g) {
            return Some(g);
        }
    }
    for a in -4 * s..=4 * s {
        if a * a > 16 * p as i128 {
            continue;
        }
        for b in -6 * (p as i128)..=6 * (p as i128) {
            let g = vec![(p as i128).pow(2), p as i128 * a, b, a, 1];
            if divides(f, &g) {
                return Some(g);
            }
        }
    }
    None
}
type M = [[u64; 8]; 8];
fn identity() -> M {
    let mut a = [[0; 8]; 8];
    for i in 0..8 {
        a[i][i] = 1
    }
    a
}
fn mul(a: &M, b: &M, r: u64) -> M {
    let mut c = [[0; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            c[i][j] = (0..8).map(|k| a[i][k] * b[k][j]).sum::<u64>() % r
        }
    }
    c
}
pub fn power_characteristic(f: &[i128; 9], mut exponent: u64, r: u64) -> Poly {
    let fp = Fp::new(r);
    let mut a = [[0; 8]; 8];
    for j in 0..7 {
        a[j + 1][j] = 1
    }
    for j in 0..8 {
        a[j][7] = (-f[j]).rem_euclid(r as i128) as u64
    }
    let mut b = identity();
    while exponent > 0 {
        if exponent & 1 != 0 {
            b = mul(&b, &a, r)
        }
        a = mul(&a, &a, r);
        exponent >>= 1
    }
    let mut power = identity();
    let mut sums = [0; 9];
    for k in 1..=8 {
        power = mul(&power, &b, r);
        sums[k] = (0..8).map(|i| power[i][i]).sum::<u64>() % r
    }
    let mut c = [0; 9];
    c[0] = 1;
    for k in 1..=8 {
        let s = (1..=k).map(|j| c[k - j] * sums[j]).sum::<u64>() % r;
        c[k] = fp.neg(fp.mul(s, fp.inv(k as u64)))
    }
    let mut out = [0; 9];
    for j in 0..=8 {
        out[j] = c[8 - j]
    }
    Poly::from_coeffs(&out)
}
pub fn witness(f: &[i128; 9], m: u64, r: u64) -> bool {
    let p = power_characteristic(f, m, r);
    Poly::gcd(&p, &p.deriv(&Fp::new(r)), &Fp::new(r)).deg() == 0
}
pub fn phi(mut n: u64) -> u64 {
    let mut out = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            out = out / p * (p - 1);
            while n % p == 0 {
                n /= p
            }
        }
        p += 1
    }
    if n > 1 {
        out = out / n * (n - 1)
    }
    out
}
pub fn primes() -> [u64; 8] {
    [10007, 10009, 10037, 10039, 10061, 10067, 10069, 10079]
}
pub fn controls() {
    for r in primes() {
        assert!((2..).take_while(|d| d * d <= r).all(|d| r % d != 0));
        let negative = [-1, 0, 0, 0, 0, 0, 0, 0, 1];
        assert!(!witness(&negative, 2, r));
        let mut f = vec![1i128];
        let mut expected = Poly::one();
        let fp = Fp::new(r);
        for j in 1..=8i128 {
            let mut v = vec![0; f.len() + 1];
            for k in 0..f.len() {
                v[k] -= j * f[k];
                v[k + 1] += f[k]
            }
            f = v;
            expected = expected.mul(&Poly::x_minus((j * j) as u64, &fp), &fp)
        }
        let f: [i128; 9] = f.try_into().unwrap();
        assert_eq!(power_characteristic(&f, 2, r), expected);
        assert!(witness(&f, 2, r));
    }
}
