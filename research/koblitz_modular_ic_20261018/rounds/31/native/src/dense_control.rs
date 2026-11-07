use num_bigint::BigInt;
use num_traits::{One, Zero};
fn p_mul(a: &[BigInt], b: &[BigInt]) -> Vec<BigInt> {
    let mut c = vec![BigInt::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] += x * y;
        }
    }
    c
}
pub(crate) fn an(n: usize, t: i64) -> Vec<BigInt> {
    let (mut a, mut b) = (BigInt::from(2), BigInt::from(t));
    for _ in 1..n {
        let c = t * &b - 2 * &a;
        a = b;
        b = c;
    }
    let mut p = vec![BigInt::zero(); 2 * n + 1];
    p[0] = BigInt::one() << n;
    p[n] = -b;
    p[2 * n] = BigInt::one();
    let d = [BigInt::from(2), BigInt::from(-t), BigInt::one()];
    let mut q = vec![BigInt::zero(); 2 * n - 1];
    for k in (2..=2 * n).rev() {
        let x = p[k].clone();
        q[k - 2] = x.clone();
        for j in 0..3 {
            p[k - 2 + j] -= &x * &d[j];
        }
    }
    assert!(p.iter().all(Zero::is_zero));
    q
}
type Cyc = Vec<[BigInt; 2]>;
fn zero(n: usize) -> Cyc {
    vec![[BigInt::zero(), BigInt::zero()]; n - 1]
}
fn shift(a: &Cyc, e: usize, kind: usize, k: i64, n: usize) -> Cyc {
    let mut out = vec![[BigInt::zero(), BigInt::zero()]; n];
    for (i, v) in a.iter().enumerate() {
        let w = match kind {
            0 => v.clone(),
            1 => [-2 * &v[1], &v[0] + &v[1]],
            2 => [&v[0] + 2 * &v[1], -v[0].clone()],
            _ => unreachable!(),
        };
        for j in 0..2 {
            out[(i + e) % n][j] += k * &w[j];
        }
    }
    let last = out.pop().unwrap();
    for v in &mut out {
        for j in 0..2 {
            v[j] -= &last[j];
        }
    }
    out
}
fn add(a: &mut Cyc, b: &Cyc) {
    for (i, v) in a.iter_mut().enumerate() {
        for j in 0..2 {
            v[j] += &b[i][j];
        }
    }
}
fn norm_euler(n: usize, i: usize, j: usize) -> Vec<BigInt> {
    let mut p = vec![zero(n)];
    p[0][0][0] = BigInt::one();
    let ii = (n + 1) / 2;
    for l in (1..2 * n).filter(|l| l % 2 == 1 && *l != n) {
        for conjugate in 0..2 {
            let a = i * l % (2 * n);
            let b = j * l % (2 * n);
            let mut next = vec![zero(n); p.len() + 2];
            for (k, c) in p.iter().enumerate() {
                add(&mut next[k + 2], c);
                add(
                    &mut next[k + 1],
                    &shift(
                        c,
                        a * ii % n,
                        if conjugate == 0 { 1 } else { 2 },
                        if a % 2 == 0 { -1 } else { 1 },
                        n,
                    ),
                );
                add(
                    &mut next[k + 1],
                    &shift(
                        c,
                        b * ii % n,
                        if conjugate == 0 { 2 } else { 1 },
                        if b % 2 == 0 { -1 } else { 1 },
                        n,
                    ),
                );
                add(
                    &mut next[k],
                    &shift(
                        c,
                        (a + b) * ii % n,
                        0,
                        if (a + b) % 2 == 0 { 2 } else { -2 },
                        n,
                    ),
                );
            }
            p = next;
        }
    }
    p.into_iter()
        .map(|c| {
            assert!(c.iter().skip(1).all(|v| v.iter().all(Zero::is_zero)));
            assert!(c[0][1].is_zero());
            c[0][0].clone()
        })
        .collect()
}

pub fn dense_norm(n: usize, i: usize, j: usize) -> Vec<BigInt> {
    norm_euler(n, i, j)
}
pub fn trace_zero(n: usize, t: i64) -> Vec<BigInt> {
    an(n, t)
}
