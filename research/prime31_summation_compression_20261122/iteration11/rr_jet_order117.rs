//! Independent mathematical validation of the F8 genus-two order-117 fixture.
//! No repository cryptanalysis modules or Cantor group operations are used.
//! Build when an original Rust toolchain is available:
//!   rustc --edition=2021 --test rr_jet_order117.rs -o rr_jet_test && ./rr_jet_test
//! This file is source-only: it has not been compiled in the publishing runtime.

#![forbid(unsafe_code)]

const F8_MOD: u16 = 0b1011;    // z^3+z+1
const F64_MOD: u16 = 0b1000011; // w^6+w+1

fn mul(a: u8, b: u8, m: usize, irr: u16) -> u8 {
    let mut x = a as u16;
    let mut y = b as u16;
    let mut result = 0u16;
    while y != 0 {
        if (y & 1) != 0 { result ^= x; }
        y >>= 1;
        x <<= 1;
        if (x & (1u16 << m)) != 0 { x ^= irr; }
    }
    result as u8
}

fn pow(mut x: u8, mut exponent: usize, m: usize, irr: u16) -> u8 {
    let mut result = 1u8;
    while exponent != 0 {
        if (exponent & 1) != 0 { result = mul(result, x, m, irr); }
        x = mul(x, x, m, irr);
        exponent >>= 1;
    }
    result
}

fn trace(x: u8, m: usize, irr: u16) -> u8 {
    let mut result = 0u8;
    let mut term = x;
    for _ in 0..m {
        result ^= term;
        term = mul(term, term, m, irr);
    }
    assert!(result == 0 || result == 1);
    result
}

fn count_points(m: usize, irr: u16) -> usize {
    let mut trace_zero = 0usize;
    for x in 0..(1usize << m) {
        let a = pow(x as u8, 5, m, irr) ^ (x as u8) ^ 1;
        if trace(a, m, irr) == 0 { trace_zero += 1; }
    }
    1 + 2 * trace_zero // one geometric point at infinity
}

fn series_mul(a: &[u8], b: &[u8], k: usize) -> Vec<u8> {
    let mut result = vec![0u8; k];
    for i in 0..k {
        if a[i] == 0 { continue; }
        for j in 0..(k - i) {
            if b[j] != 0 { result[i + j] ^= mul(a[i], b[j], 3, F8_MOD); }
        }
    }
    result
}

/// Rank of the first k t-adic jets of L(k*infinity) at
/// P=(z,z^2+z), x=z+t, on y^2+y=x^5+x+1 over F8.
/// For k >= 5 the columns are x^j (2j <= k) and x^j*y (2j+5 <= k).
fn jet_rank(k: usize) -> (usize, usize) {
    assert!(k >= 5);
    let z = 2u8;
    let y0 = 6u8;
    assert_eq!(mul(y0, y0, 3, F8_MOD) ^ y0,
               pow(z, 5, 3, F8_MOD) ^ z ^ 1);

    let mut c = vec![0u8; k];
    // (z+t)^5 = z^5+z^4*t+z*t^4+t^5 in characteristic two.
    for i in 0..=5 {
        if i < k && ((5usize & i) == i) {
            c[i] ^= pow(z, 5-i, 3, F8_MOD);
        }
    }
    c[0] ^= z ^ 1;
    c[1] ^= 1;

    let mut y = vec![0u8; k];
    y[0] = y0;
    for i in 1..k {
        let square_term = if i % 2 == 0 {
            mul(y[i/2], y[i/2], 3, F8_MOD)
        } else { 0 };
        y[i] = c[i] ^ square_term; // y^2+y=f(z+t)
    }

    let mut x = vec![0u8; k];
    x[0] = z;
    x[1] = 1;
    let mut identity = vec![0u8; k];
    identity[0] = 1;
    let mut x_powers = vec![identity];
    for _ in 0..(k / 2) {
        let next = series_mul(x_powers.last().unwrap(), &x, k);
        x_powers.push(next);
    }
    let mut columns = x_powers.clone();
    for j in 0..=((k - 5) / 2) {
        columns.push(series_mul(&x_powers[j], &y, k));
    }
    let n = columns.len();
    assert_eq!(n, k - 1);
    let mut rows = vec![vec![0u8; n]; k];
    for (j, col) in columns.iter().enumerate() {
        for i in 0..k { rows[i][j] = col[i]; }
    }

    let mut rank = 0usize;
    for col in 0..n {
        let pivot = (rank..k).find(|&row| rows[row][col] != 0);
        if let Some(p) = pivot {
            rows.swap(rank, p);
            let inverse = pow(rows[rank][col], 6, 3, F8_MOD);
            for j in col..n { rows[rank][j] = mul(rows[rank][j], inverse, 3, F8_MOD); }
            for r in 0..k {
                if r == rank { continue; }
                let coeff = rows[r][col];
                if coeff == 0 { continue; }
                for j in col..n {
                    rows[r][j] ^= mul(coeff, rows[rank][j], 3, F8_MOD);
                }
            }
            rank += 1;
        }
    }
    (rank, n)
}

#[test]
fn irreducible_field_models() {
    for (m, irr) in [(3, F8_MOD), (6, F64_MOD)] {
        for a in 1..(1usize << m) {
            assert_eq!(pow(a as u8, (1usize << m) - 1, m, irr), 1);
        }
    }
}

#[test]
fn independent_trace_counts_and_jacobian_order() {
    let n1 = count_points(3, F8_MOD);
    let n2 = count_points(6, F64_MOD);
    assert_eq!((n1, n2), (13, 81));
    assert_eq!((n1*n1 + n2)/2 - 8, 117);
}

#[test]
fn independent_riemann_roch_jet_certificates() {
    assert_eq!(jet_rank(9), (8, 8));
    assert_eq!(jet_rank(39), (38, 38));
    assert_eq!(jet_rank(117), (115, 116));
}
