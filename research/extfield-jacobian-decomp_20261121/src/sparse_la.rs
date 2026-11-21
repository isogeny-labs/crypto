//! Sequential Wiedemann over F_ell (ell prime, < 2^63) on a sparse, overdetermined system M x = b,
//! solved through the normal equations A x = c, A = M^T M, c = M^T b.  Every multiply-add is counted.
//! Written for this study because `pq_wiedemann` uses BigUint and crate-internal row types.

pub struct Counter(pub u64);

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn powmod(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, b, p);
        }
        b = mulmod(b, b, p);
        e >>= 1;
    }
    r
}

fn inv(a: u64, p: u64, ctr: &mut Counter) -> u64 {
    ctr.0 += 64;
    powmod(a, p - 2, p)
}

/// out[r] = sum_j coef * v[col] over each row.
fn mat_vec(rows: &[Vec<(u32, u64)>], v: &[u64], p: u64, out: &mut [u64], ctr: &mut Counter) {
    for (r, row) in rows.iter().enumerate() {
        let mut acc = 0u128;
        for &(c, k) in row {
            acc += k as u128 * v[c as usize] as u128 % p as u128;
        }
        ctr.0 += row.len() as u64;
        out[r] = (acc % p as u128) as u64;
    }
}

/// out = M^T y.
fn mat_t_vec(rows: &[Vec<(u32, u64)>], y: &[u64], p: u64, out: &mut [u64], ctr: &mut Counter) {
    out.iter_mut().for_each(|x| *x = 0);
    for (r, row) in rows.iter().enumerate() {
        if y[r] == 0 {
            continue;
        }
        for &(c, k) in row {
            let t = mulmod(k, y[r], p);
            let o = &mut out[c as usize];
            *o = (*o + t) % p;
        }
        ctr.0 += row.len() as u64;
    }
}

fn apply_a(
    rows: &[Vec<(u32, u64)>],
    v: &[u64],
    p: u64,
    tmp: &mut [u64],
    out: &mut [u64],
    ctr: &mut Counter,
) {
    mat_vec(rows, v, p, tmp, ctr);
    mat_t_vec(rows, tmp, p, out, ctr);
}

/// Berlekamp-Massey: returns the connection polynomial C (C[0] = 1) with
/// s_n + sum_{j>=1} C[j] s_{n-j} = 0 for the whole sequence.
pub fn berlekamp_massey(s: &[u64], p: u64, ctr: &mut Counter) -> Vec<u64> {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let (mut l, mut m, mut bd) = (0usize, 1usize, 1u64);
    for n in 0..s.len() {
        let mut d = s[n];
        for i in 1..=l.min(c.len() - 1) {
            d = (d + mulmod(c[i], s[n - i], p)) % p;
        }
        ctr.0 += l as u64;
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = mulmod(d, inv(bd, p, ctr), p);
        let t = c.clone();
        if c.len() < b.len() + m {
            c.resize(b.len() + m, 0);
        }
        for (i, &bi) in b.iter().enumerate() {
            let sub = mulmod(coef, bi, p);
            c[i + m] = (c[i + m] + p - sub) % p;
        }
        ctr.0 += b.len() as u64;
        if 2 * l <= n {
            l = n + 1 - l;
            b = t;
            bd = d;
            m = 1;
        } else {
            m += 1;
        }
    }
    c.resize(l + 1, 0);
    c
}

pub struct LaOut {
    pub x: Option<Vec<u64>>,
    pub mulmods: u64,
    pub tries: u32,
}

/// Solve M x = b (consistent, overdetermined) mod p.  `seed` drives the random projection.
/// Returns x (one solution; unique only if M has full column rank) or None.
pub fn solve(rows: &[Vec<(u32, u64)>], b: &[u64], ncols: usize, p: u64, seed: u64) -> LaOut {
    let mut ctr = Counter(0);
    let mut st = seed | 1;
    let mut next = move || {
        st ^= st << 13;
        st ^= st >> 7;
        st ^= st << 17;
        st
    };
    let mut c = vec![0u64; ncols];
    mat_t_vec(rows, b, p, &mut c, &mut ctr);
    let mut tmp = vec![0u64; rows.len()];
    let seq_len = 2 * ncols + 2;
    for tries in 1..=3u32 {
        let u: Vec<u64> = (0..ncols).map(|_| next() % p).collect();
        let mut v = c.clone();
        let mut w = vec![0u64; ncols];
        let mut s = Vec::with_capacity(seq_len);
        for _ in 0..seq_len {
            let mut d = 0u64;
            for i in 0..ncols {
                d = (d + mulmod(u[i], v[i], p)) % p;
            }
            ctr.0 += ncols as u64;
            s.push(d);
            apply_a(rows, &v, p, &mut tmp, &mut w, &mut ctr);
            std::mem::swap(&mut v, &mut w);
        }
        let conn = berlekamp_massey(&s, p, &mut ctr);
        let l = conn.len() - 1;
        // p(lambda) coefficients: p_i = conn[l - i]; require p_0 = conn[l] != 0.
        if l == 0 || conn[l] == 0 {
            continue;
        }
        let p0inv = inv(conn[l], p, &mut ctr);
        let mut acc = vec![0u64; ncols];
        let mut vv = c.clone();
        for i in 1..=l {
            let pi = conn[l - i];
            for j in 0..ncols {
                acc[j] = (acc[j] + mulmod(pi, vv[j], p)) % p;
            }
            ctr.0 += ncols as u64;
            if i < l {
                apply_a(rows, &vv, p, &mut tmp, &mut w, &mut ctr);
                std::mem::swap(&mut vv, &mut w);
            }
        }
        let scale = (p - p0inv) % p;
        let x: Vec<u64> = acc.iter().map(|&a| mulmod(a, scale, p)).collect();
        ctr.0 += ncols as u64;
        // residual check: M x == b
        mat_vec(rows, &x, p, &mut tmp, &mut ctr);
        if tmp.iter().zip(b).all(|(&a, &bb)| a == bb % p) {
            return LaOut {
                x: Some(x),
                mulmods: ctr.0,
                tries,
            };
        }
    }
    LaOut {
        x: None,
        mulmods: ctr.0,
        tries: 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rng(st: &mut u64) -> u64 {
        *st ^= *st << 13;
        *st ^= *st >> 7;
        *st ^= *st << 17;
        *st
    }

    #[test]
    fn bm_recovers_linear_recurrence() {
        let p = 1_000_003u64;
        // s_n = 3 s_{n-1} + 5 s_{n-2}
        let mut s = vec![2u64, 7];
        for n in 2..40 {
            s.push((3 * s[n - 1] + 5 * s[n - 2]) % p);
        }
        let mut ctr = Counter(0);
        let c = berlekamp_massey(&s, p, &mut ctr);
        assert_eq!(c.len(), 3);
        assert_eq!(c[1], p - 3);
        assert_eq!(c[2], p - 5);
    }

    #[test]
    fn solves_random_sparse_overdetermined_system() {
        let p = 1_000_000_007u64;
        let mut st = 0x1234_5678_9abc_def1u64;
        let n = 60usize;
        let m = 90usize;
        let xs: Vec<u64> = (0..n).map(|_| rng(&mut st) % p).collect();
        let mut rows: Vec<Vec<(u32, u64)>> = Vec::new();
        let mut b = Vec::new();
        for r in 0..m {
            let mut row: Vec<(u32, u64)> = Vec::new();
            for t in 0..4 {
                let col = ((rng(&mut st) % n as u64) as usize + if r < n { 0 } else { t }) % n;
                if !row.iter().any(|&(c, _)| c as usize == col) {
                    row.push((col as u32, 1 + rng(&mut st) % (p - 1)));
                }
            }
            if r < n && !row.iter().any(|&(c, _)| c as usize == r) {
                row.push((r as u32, 1 + rng(&mut st) % (p - 1)));
            }
            let rhs = row
                .iter()
                .fold(0u64, |a, &(c, k)| (a + mulmod(k, xs[c as usize], p)) % p);
            rows.push(row);
            b.push(rhs);
        }
        let out = solve(&rows, &b, n, p, 77);
        let x = out.x.expect("solved");
        assert_eq!(x, xs);
        assert!(out.mulmods > 0);
    }
}
