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
fn isqrt(n: i128) -> i128 {
    assert!(n >= 0);
    let (mut lo, mut hi) = (0, n + 1);
    while hi - lo > 1 {
        let m = (lo + hi) / 2;
        if m <= n / m {
            lo = m;
        } else {
            hi = m;
        }
    }
    lo
}
// Exact sign of a+b*sqrt(p); no floating point endpoints.
fn radical_sign(a: i128, b: i128, p: i128) -> i128 {
    if b == 0 {
        return a.signum();
    }
    if a == 0 || a.signum() == b.signum() {
        return b.signum();
    }
    (a * a - b * b * p).signum() * a.signum()
}
pub fn lift(f: &[u64; 8], prime: u64, subgroup: u64) -> [i64; 7] {
    let c = recurrence(f, prime);
    let (r, w) = residues(&c, prime);
    println!("FRESH_HASSE_WITT matrix={w:?} residues={r:?} coefficient_steps={} extension_counts_used=false", 3*prime-1);
    let (p, ell) = (prime as i128, subgroup as i128);
    let a1s: Vec<_> = (-6i128..=6)
        .map(|k| r[0] as i128 + k * p)
        .filter(|a| a.abs() <= isqrt(36 * p))
        .collect();
    assert_eq!(a1s.len(), 1, "ambiguous a1 is unsupported");
    let a1 = a1s[0];
    let base = p * p * p + 1 + (p * p + 1) * a1;
    let bound = isqrt(400 * p * p * p);
    let lo = base - (p + 1) * 15 * p - bound;
    let hi = base + (p + 1) * 15 * p + bound;
    assert!(lo > 0 && ell > 0, "unsupported nonpositive order enclosure");
    let first = (lo + ell - 1) / ell;
    let last = hi / ell;
    assert_eq!(first, last, "order interval ambiguous; retain ambiguity");
    let order = first * ell;
    let mut survivors = Vec::new();
    let mut counts = [0usize; 3];
    for t in -15i128..=15 {
        let b = r[1] as i128 + t * p;
        if b < -15 * p || b > 15 * p {
            continue;
        }
        counts[0] += 1;
        let c = order - base - (p + 1) * b;
        if c.abs() > bound || c.rem_euclid(p) != r[2] as i128 {
            continue;
        }
        counts[1] += 1;
        // h(+-2sqrt(p)) = c+2pa1 +-2(p+b)sqrt(p).
        if radical_sign(c + 2 * p * a1, -2 * (p + b), p) > 0
            || radical_sign(c + 2 * p * a1, 2 * (p + b), p) < 0
        {
            continue;
        }
        counts[2] += 1;
        let h = |x: i128| x * x * x + a1 * x * x + (b - 3 * p) * x + c - 2 * p * a1;
        let limit = isqrt(4 * p);
        let brackets: Vec<_> = (-limit..limit)
            .filter(|x| h(*x) * h(*x + 1) < 0)
            .map(|x| (x, x + 1))
            .collect();
        // Sufficient exact root certificate; unsupported clustered/repeated roots
        // stop the gate, rather than silently excluding a possible polynomial.
        assert_eq!(
            brackets.len(),
            3,
            "root certificate unsupported; do not reject candidate"
        );
        println!("ROOT_BRACKETS b={b} c={c} {brackets:?}");
        survivors.push((a1, b, c, order));
    }
    assert_eq!(survivors.len(), 1, "ambiguous Weil lift");
    println!(
        "LIFT_FILTER residue_lifts={} coarse_survivors={} endpoint_survivors={}",
        counts[0], counts[1], counts[2]
    );
    println!("WEIL_LIFT {:?}", survivors[0]);

    let (a, b, cc, _) = survivors[0];
    println!(
        "IMPLIED_COUNTS N1={} N2={}",
        p + 1 + a,
        p * p + 1 - a * a + 2 * b
    );
    [1, a, b, cc, p * b, p * p * a, p * p * p]
        .map(|v| i64::try_from(v).expect("Weil coefficient exceeds supported i64"))
}
