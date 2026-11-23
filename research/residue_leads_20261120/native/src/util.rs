//! RNG, statistics, FFT/Bluestein, Walsh-Hadamard, number theory helpers.

pub const PROTOCOL_SEED: u64 = 0x5015_e814_565a_dcef; // first 8 bytes of sha256(protocol.json) at freeze

pub struct SplitMix(pub u64);
impl SplitMix {
    pub fn from_label(label: &str) -> Self {
        // FNV-1a of the label mixed into the protocol seed.
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for b in label.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        SplitMix(PROTOCOL_SEED ^ h)
    }
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    pub fn unif(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn below(&mut self, n: u64) -> u64 {
        ((self.next() as u128 * n as u128) >> 64) as u64
    }
}

// ---------- number theory ----------
pub fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = ((r as u128 * b as u128) % m as u128) as u64;
        }
        b = ((b as u128 * b as u128) % m as u128) as u64;
        e >>= 1;
    }
    r
}
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}
pub fn factor(mut n: u64) -> Vec<u64> {
    let mut f = vec![];
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            f.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        f.push(n);
    }
    f
}
/// multiplicative order of a modulo prime n
pub fn mult_order(a: u64, n: u64) -> u64 {
    let mut ord = n - 1;
    for q in factor(n - 1) {
        while ord % q == 0 && powmod(a, ord / q, n) == 1 {
            ord /= q;
        }
    }
    ord
}
pub fn legendre(a: u64, p: u64) -> i64 {
    let a = a % p;
    if a == 0 {
        return 0;
    }
    if powmod(a, (p - 1) / 2, p) == 1 {
        1
    } else {
        -1
    }
}
/// Tonelli-Shanks; returns None for non-residues.
pub fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if legendre(a, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(powmod(a, (p + 1) / 4, p));
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while legendre(z, p) != -1 {
        z += 1;
    }
    let mut m = s;
    let mut c = powmod(z, q, p);
    let mut t = powmod(a, q, p);
    let mut r = powmod(a, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = tt * tt % p;
            i += 1;
        }
        let b = powmod(c, 1 << (m - i - 1), p);
        m = i;
        c = b * b % p;
        t = t * c % p;
        r = r * b % p;
    }
    Some(r)
}

// ---------- statistics ----------
/// erfc with fractional error < 1.2e-7 (Numerical Recipes erfcc), valid in the far tail.
pub fn erfc(x: f64) -> f64 {
    let z = x.abs();
    let t = 1.0 / (1.0 + 0.5 * z);
    let ans = t
        * (-z * z - 1.26551223
            + t * (1.00002368
                + t * (0.37409196
                    + t * (0.09678418
                        + t * (-0.18628806
                            + t * (0.27886807
                                + t * (-1.13520398
                                    + t * (1.48851587 + t * (-0.82215223 + t * 0.17087277)))))))))
            .exp();
    if x >= 0.0 {
        ans
    } else {
        2.0 - ans
    }
}
/// two-sided normal tail P(|Z| >= z)
pub fn norm_two_sided(z: f64) -> f64 {
    erfc(z.abs() / std::f64::consts::SQRT_2)
}
fn ln_gamma(x: f64) -> f64 {
    let c = [
        76.18009172947146,
        -86.50532032941677,
        24.01409824083091,
        -1.231739572450155,
        0.1208650973866179e-2,
        -0.5395239384953e-5,
    ];
    let mut y = x;
    let tmp = x + 5.5;
    let tmp = tmp - (x + 0.5) * tmp.ln();
    let mut ser = 1.000000000190015;
    for ci in c.iter() {
        y += 1.0;
        ser += ci / y;
    }
    -tmp + (2.5066282746310005 * ser / x).ln()
}
/// upper regularized incomplete gamma Q(a, x)
pub fn gamma_q(a: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 1.0;
    }
    if x < a + 1.0 {
        let mut ap = a;
        let mut sum = 1.0 / a;
        let mut del = sum;
        for _ in 0..1000 {
            ap += 1.0;
            del *= x / ap;
            sum += del;
            if del.abs() < sum.abs() * 1e-15 {
                break;
            }
        }
        1.0 - sum * (-x + a * x.ln() - ln_gamma(a)).exp()
    } else {
        let mut b = x + 1.0 - a;
        let mut c = 1.0 / 1e-300;
        let mut d = 1.0 / b;
        let mut h = d;
        for i in 1..1000 {
            let an = -(i as f64) * (i as f64 - a);
            b += 2.0;
            d = an * d + b;
            if d.abs() < 1e-300 {
                d = 1e-300;
            }
            c = b + an / c;
            if c.abs() < 1e-300 {
                c = 1e-300;
            }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-15 {
                break;
            }
        }
        (-x + a * x.ln() - ln_gamma(a)).exp() * h
    }
}
pub fn chi2_sf(x: f64, df: f64) -> f64 {
    gamma_q(df / 2.0, x / 2.0)
}
/// P(max of N iid tails >= ...) given single tail probability t: 1-(1-t)^N
pub fn max_upper(t: f64, n: f64) -> f64 {
    -(n * (-t).ln_1p()).exp_m1()
}
/// P(max of N iid <= ...) : (1-t)^N
pub fn max_lower(t: f64, n: f64) -> f64 {
    (n * (-t).ln_1p()).exp()
}
/// KS statistic of samples against U(0,1)
pub fn ks_uniform(mut u: Vec<f64>) -> f64 {
    u.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let k = u.len() as f64;
    let mut d: f64 = 0.0;
    for (i, &x) in u.iter().enumerate() {
        d = d.max((i as f64 + 1.0) / k - x).max(x - i as f64 / k);
    }
    d
}

// ---------- FFT ----------
#[derive(Clone, Copy, Default)]
pub struct C {
    pub re: f64,
    pub im: f64,
}
impl C {
    fn mul(self, o: C) -> C {
        C { re: self.re * o.re - self.im * o.im, im: self.re * o.im + self.im * o.re }
    }
    pub fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }
}
pub fn fft(a: &mut [C], inverse: bool) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let ang = 2.0 * std::f64::consts::PI / len as f64 * if inverse { 1.0 } else { -1.0 };
        let half = len / 2;
        let tw: Vec<C> = (0..half).map(|k| C { re: (ang * k as f64).cos(), im: (ang * k as f64).sin() }).collect();
        for s in (0..n).step_by(len) {
            for k in 0..half {
                let u = a[s + k];
                let v = a[s + k + half].mul(tw[k]);
                a[s + k] = C { re: u.re + v.re, im: u.im + v.im };
                a[s + k + half] = C { re: u.re - v.re, im: u.im - v.im };
            }
        }
        len <<= 1;
    }
    if inverse {
        let f = 1.0 / n as f64;
        for x in a.iter_mut() {
            x.re *= f;
            x.im *= f;
        }
    }
}
/// Bluestein DFT plan for arbitrary length n: X[a] = sum_k x[k] e^{-2 pi i a k / n}
pub struct Bluestein {
    n: usize,
    l: usize,
    chirp: Vec<C>,  // e^{-pi i k^2/n}
    bfft: Vec<C>,   // FFT of conj chirp, wrapped
}
impl Bluestein {
    pub fn new(n: usize) -> Self {
        let mut l = 1;
        while l < 2 * n - 1 {
            l <<= 1;
        }
        let chirp: Vec<C> = (0..n)
            .map(|k| {
                let kk = ((k as u128 * k as u128) % (2 * n as u128)) as f64;
                let ang = -std::f64::consts::PI * kk / n as f64;
                C { re: ang.cos(), im: ang.sin() }
            })
            .collect();
        let mut b = vec![C::default(); l];
        for k in 0..n {
            let c = C { re: chirp[k].re, im: -chirp[k].im };
            b[k] = c;
            if k > 0 {
                b[l - k] = c;
            }
        }
        fft(&mut b, false);
        Bluestein { n, l, chirp, bfft: b }
    }
    pub fn dft_real(&self, x: &[f64]) -> Vec<C> {
        let mut a = vec![C::default(); self.l];
        for k in 0..self.n {
            a[k] = C { re: x[k] * self.chirp[k].re, im: x[k] * self.chirp[k].im };
        }
        fft(&mut a, false);
        for i in 0..self.l {
            a[i] = a[i].mul(self.bfft[i]);
        }
        fft(&mut a, true);
        (0..self.n).map(|k| a[k].mul(self.chirp[k])).collect()
    }
}

/// in-place Walsh-Hadamard transform over i64
pub fn wht(a: &mut [i64]) {
    let n = a.len();
    let mut h = 1;
    while h < n {
        for i in (0..n).step_by(2 * h) {
            for j in i..i + h {
                let (x, y) = (a[j], a[j + h]);
                a[j] = x + y;
                a[j + h] = x - y;
            }
        }
        h <<= 1;
    }
}

// ---------- tiny JSON ----------
pub fn jnum(x: f64) -> String {
    if x.is_finite() {
        format!("{:.6e}", x)
    } else {
        "null".into()
    }
}
