// Exact coefficient gate; not a timing benchmark or production replacement.
fn inv(a: u128, m: u128) -> u128 {
    let (mut r, mut s, mut x, mut y) = (m as i128, a as i128, 0i128, 1i128);
    while s != 0 {
        let q = r / s;
        (r, s) = (s, r - q * s);
        (x, y) = (y, x - q * y);
    }
    assert_eq!(r, 1);
    x.rem_euclid(m as i128) as u128
}
fn pow(mut a: u128, mut n: u64, m: u128) -> u128 {
    let mut b = 1;
    while n != 0 {
        if n & 1 != 0 {
            b = b * a % m;
        }
        a = a * a % m;
        n >>= 1;
    }
    b
}
// k*f0*c[k] = sum_i (((n+1)*i-k)*fi*c[k-i]).
// Start at p^3; only k=p and k=2p lose one p-adic digit.
// All coefficients stored before each step remain correct at current modulus.
fn recurrence(f: &[u64; 8], p: u64) -> Vec<u64> {
    assert!((5..=65537).contains(&p) && p % 2 == 1);
    assert!(f.iter().all(|x| *x < p) && f[0] != 0);
    let n = (p - 1) / 2;
    let mut modulus = (p as u128).pow(3);
    let mut c = vec![0u128; 3 * p as usize];
    c[0] = pow(f[0] as u128, n, modulus);
    for k in 1..c.len() {
        let mut rhs = 0;
        for i in 1..=7.min(k) {
            let z = (((n + 1) as i128 * i as i128) - k as i128).rem_euclid(modulus as i128) as u128;
            rhs = (rhs + z * f[i] as u128 % modulus * (c[k - i] % modulus)) % modulus;
        }
        let mut unit = k as u128;
        if k as u64 % p == 0 {
            assert_eq!(rhs % p as u128, 0);
            rhs /= p as u128;
            unit /= p as u128;
            modulus /= p as u128;
        }
        c[k] = rhs * inv(unit * f[0] as u128 % modulus, modulus) % modulus;
    }
    c.into_iter().map(|x| (x % p as u128) as u64).collect()
}
// Independent coefficient oracle: repeated polynomial multiplication over Fp.
fn reference(f: &[u64; 8], p: u64) -> Vec<u64> {
    let mut c = vec![1u64];
    for _ in 0..(p - 1) / 2 {
        let mut d = vec![0; c.len() + 7];
        for (i, a) in c.iter().enumerate() {
            for (j, b) in f.iter().enumerate() {
                d[i + j] = (d[i + j] + a * b) % p;
            }
        }
        c = d;
    }
    c.resize(3 * p as usize, 0);
    c.truncate(3 * p as usize);
    c
}
fn residues(c: &[u64], p: u64) -> ([u64; 3], [[u64; 3]; 3]) {
    let w: [[u64; 3]; 3] =
        std::array::from_fn(|i| std::array::from_fn(|j| c[(i + 1) * p as usize - (j + 1)]));
    let a1 = -(w[0][0] as i128 + w[1][1] as i128 + w[2][2] as i128);
    let mut a2 = 0i128;
    for i in 0..3 {
        for j in i + 1..3 {
            a2 += w[i][i] as i128 * w[j][j] as i128 - w[i][j] as i128 * w[j][i] as i128;
        }
    }
    let mut det = 0i128;
    for i in 0..3 {
        det += w[0][i] as i128
            * (w[1][(i + 1) % 3] as i128 * w[2][(i + 2) % 3] as i128
                - w[1][(i + 2) % 3] as i128 * w[2][(i + 1) % 3] as i128);
    }
    ([a1, a2, -det].map(|a| a.rem_euclid(p as i128) as u64), w)
}
fn main() {
    let mut cases = 0;
    for p in [5, 7, 11, 13, 17, 31, 127, 257, 499] {
        for seed in 1..=12u64 {
            let f: [u64; 8] = std::array::from_fn(|i| {
                if i == 0 {
                    1 + seed % (p - 1)
                } else {
                    (seed * seed + 17 * i as u64 + 3 * seed * i as u64) % p
                }
            });
            let got = recurrence(&f, p);
            assert_eq!(got, reference(&f, p), "p={p} seed={seed}");
            cases += 1;
        }
    }
    let f = [31, 89, 73, 48, 91, 41, 123, 1];
    let c = recurrence(&f, 127);
    assert_eq!(c, reference(&f, 127));
    let (r, w) = residues(&c, 127);
    println!("HASSE_WITT {w:?}");
    println!("RESIDUES {r:?}");
    assert_eq!(r, [1, 84, 46]);
    println!(
        "COEFFICIENT_GATE cases={} fixture=true p_adic_losses=2",
        cases
    );
}
