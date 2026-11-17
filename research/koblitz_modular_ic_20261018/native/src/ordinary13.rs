//! Exact cyclic-121 Hecke norms in characteristic two, in native binary series.
use crate::{round02, round04::Field};
use num_bigint::BigUint;
use serde_json::json;
use std::path::Path;
type B = Vec<u64>;
const PREC: usize = 640;
#[path = "ordinary13_factor.rs"]
pub(crate) mod factor;
fn clean(mut a: B) -> B {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn one() -> B {
    vec![1]
}
fn bit(a: &[u64], i: usize) -> bool {
    a.get(i / 64).is_some_and(|x| (x >> (i % 64)) & 1 != 0)
}
fn set(a: &mut B, i: usize) {
    a.resize(a.len().max(i / 64 + 1), 0);
    a[i / 64] ^= 1 << (i % 64);
}
fn cut(mut a: B, n: usize) -> B {
    a.truncate(n.div_ceil(64));
    if n % 64 != 0 && a.len() == n.div_ceil(64) {
        *a.last_mut().unwrap() &= (1u64 << (n % 64)) - 1;
    }
    clean(a)
}
fn add(a: &[u64], b: &[u64]) -> B {
    let mut z = a.to_vec();
    z.resize(z.len().max(b.len()), 0);
    for (x, y) in z.iter_mut().zip(b) {
        *x ^= y;
    }
    clean(z)
}
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
unsafe fn clmul_hw(a: u64, b: u64) -> [u64; 2] {
    use std::arch::x86_64::*;
    std::mem::transmute(_mm_clmulepi64_si128::<0>(
        _mm_set_epi64x(0, a as i64),
        _mm_set_epi64x(0, b as i64),
    ))
}
fn clmul(a: u64, mut b: u64) -> [u64; 2] {
    #[cfg(target_arch = "x86_64")]
    if std::is_x86_feature_detected!("pclmulqdq") {
        return unsafe { clmul_hw(a, b) };
    }
    let mut z = 0u128;
    while b != 0 {
        let k = b.trailing_zeros();
        z ^= (a as u128) << k;
        b &= b - 1;
    }
    [z as u64, (z >> 64) as u64]
}
fn mul(a: &[u64], b: &[u64], n: usize) -> B {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let w = n.div_ceil(64);
    let mut z = vec![0; w];
    for (i, x) in a.iter().enumerate().take(w) {
        if *x == 0 {
            continue;
        }
        for (j, y) in b.iter().enumerate().take(w - i) {
            if *y == 0 {
                continue;
            }
            let p = clmul(*x, *y);
            z[i + j] ^= p[0];
            if i + j + 1 < w {
                z[i + j + 1] ^= p[1];
            }
        }
    }
    cut(z, n)
}
fn each(a: &[u64], mut f: impl FnMut(usize)) {
    for (i, x) in a.iter().enumerate() {
        let mut x = *x;
        while x != 0 {
            let j = x.trailing_zeros() as usize;
            f(i * 64 + j);
            x &= x - 1;
        }
    }
}
fn shift(a: &[u64], k: usize, n: usize) -> B {
    let mut z = vec![];
    each(a, |i| {
        if i + k < n {
            set(&mut z, i + k)
        }
    });
    clean(z)
}
fn dilate(a: &[u64], k: usize, n: usize) -> B {
    let mut z = vec![];
    each(a, |i| {
        if i * k < n {
            set(&mut z, i * k)
        }
    });
    clean(z)
}
fn sq(a: &[u64], n: usize) -> B {
    dilate(a, 2, n)
}
fn inv(a: &[u64], n: usize) -> B {
    assert!(bit(a, 0), "unit power-series pivot");
    let mut z = one();
    let mut k = 1;
    while k < n {
        k = (2 * k).min(n);
        z = mul(a, &sq(&z, k), k);
    }
    assert_eq!(mul(a, &z, n), one());
    z
}
fn pow(a: &[u64], mut k: usize, n: usize) -> B {
    let mut z = one();
    let mut p = cut(a.to_vec(), n);
    while k > 0 {
        if k & 1 != 0 {
            z = mul(&z, &p, n);
        }
        k >>= 1;
        if k > 0 {
            p = sq(&p, n);
        }
    }
    z
}
fn ratio(a: &[u64], b: &[u64], n: usize) -> B {
    mul(a, &inv(b, n), n)
}
fn euler(n: usize, k: usize) -> B {
    let mut z = one();
    for j in 1.. {
        let a = k * j * (3 * j - 1) / 2;
        if a >= n {
            break;
        }
        set(&mut z, a);
        let b = k * j * (3 * j + 1) / 2;
        if b < n {
            set(&mut z, b);
        }
    }
    z
}
fn hex(a: &[u64]) -> String {
    BigUint::from_bytes_le(&a.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<_>>())
        .to_str_radix(16)
}
fn degree(a: &[u64]) -> Option<usize> {
    a.last()
        .map(|v| (a.len() - 1) * 64 + 63 - v.leading_zeros() as usize)
}
fn norm11(p: &[u64], n: usize) -> B {
    let mut a = vec![vec![B::new(); 11]; 11];
    for j in 0..11 {
        each(p, |k| {
            let r = (k + j) % 11;
            let e = (k + j) / 11;
            if e < n {
                set(&mut a[r][j], e);
            }
        });
    }
    let mut det = one();
    for j in 0..11 {
        assert!(bit(&a[j][j], 0), "nonunit determinant pivot");
        let pivot = a[j][j].clone();
        let pi = inv(&pivot, n);
        det = mul(&det, &pivot, n);
        for i in j + 1..11 {
            if a[i][j].is_empty() {
                continue;
            }
            let m = mul(&a[i][j], &pi, n);
            for k in j + 1..11 {
                a[i][k] = add(&a[i][k], &mul(&m, &a[j][k], n));
            }
            a[i][j].clear();
        }
    }
    det
}
fn norm121(p: &[u64], n: usize) -> B {
    norm11(&norm11(p, 11 * n), n)
}
fn field_mul(f: Field, a: &[u32], b: &[u32], n: usize) -> Vec<u32> {
    let mut z = vec![0; n];
    for (i, x) in a.iter().enumerate().take(n) {
        if *x == 0 {
            continue;
        }
        for (j, y) in b.iter().enumerate().take(n - i) {
            if *y != 0 {
                z[i + j] ^= f.mul(*x, *y);
            }
        }
    }
    z
}
fn middle(u: &[u64], product: bool, n: usize) -> B {
    let f = Field::find(10);
    let zeta = (2..1024).map(|a| f.pow(a, 93)).find(|a| *a != 1).unwrap();
    assert_eq!(f.pow(zeta, 11), 1);
    let base: Vec<_> = (0..n).map(|i| u32::from(bit(u, i))).collect();
    let mut out = vec![0; n];
    out[0] = 1;
    for b in 1..11 {
        let other: Vec<_> = (0..n)
            .map(|k| {
                if bit(u, k) {
                    f.pow(zeta, ((b * (k + 22 - 2)) % 11) as u64)
                } else {
                    0
                }
            })
            .collect();
        let v = if product {
            let mut v = field_mul(f, &base, &other, n);
            if n > 4 {
                v[4] ^= 1;
            }
            v
        } else {
            base.iter().zip(other).map(|(a, b)| a ^ b).collect()
        };
        out = field_mul(f, &out, &v, n);
    }
    let mut out2 = vec![];
    for (i, v) in out.iter().enumerate() {
        assert!(*v <= 1, "middle norm descends to F2");
        if *v == 1 {
            set(&mut out2, i);
        }
    }
    assert!(bit(&out2, 0));
    out2
}
fn control(n: usize) -> serde_json::Value {
    let mut p = one();
    set(&mut p, 1);
    let mut expect = one();
    set(&mut expect, 1);
    assert_eq!(norm121(&p, n), expect);
    let h = euler(121 * n, 1);
    let hnorm = norm121(&h, n);
    let expected = ratio(&pow(&euler(n, 1), 133, n), &pow(&euler(n, 11), 12, n), n);
    assert_eq!(hnorm, expected);
    let a = vec![0b1011011];
    let b = vec![0b110011];
    assert_eq!(
        norm121(&mul(&a, &b, 121 * 32), 32),
        mul(&norm121(&a, 32), &norm121(&b, 32), 32)
    );
    let f = Field::find(10);
    let zeta = (2..1024).map(|a| f.pow(a, 93)).find(|a| *a != 1).unwrap();
    let mut direct = vec![1];
    for b in 0..11 {
        let v: Vec<_> = (0..8)
            .map(|k| {
                if bit(&a, k) {
                    f.pow(zeta, (b * k % 11) as u64)
                } else {
                    0
                }
            })
            .collect();
        direct = field_mul(f, &direct, &v, 96);
    }
    let mut expected = vec![];
    for (i, v) in direct.into_iter().enumerate() {
        if v != 0 {
            assert_eq!(v, 1);
            assert_eq!(i % 11, 0);
            set(&mut expected, i / 11);
        }
    }
    assert_eq!(norm11(&a, 16), expected);
    json!({"norm_one_plus_t":true,"Euler_norm_coefficients":n,"multiplicativity_precision":32,"direct_F1024_root_of_unity_norm":true,"determinant_pivots":"all units;unipotent triangular modulo the base parameter"})
}
fn xy(n: usize) -> (B, B) {
    let h = euler(n, 1);
    let h49 = euler(n, 49);
    let u = ratio(&h, &h49, n);
    let w = pow(&ratio(&euler(n, 7), &h49, n), 4, n);
    let num = add(&add(&w, &shift(&u, 5, n)), &shift(&one(), 7, n));
    let den = add(&add(&sq(&u, n), &shift(&u, 2, n)), &shift(&one(), 4, n));
    let v = ratio(&num, &den, n);
    let residual = add(
        &add(
            &add(&sq(&v, n), &shift(&mul(&u, &v, n), 1, n)),
            &pow(&u, 3, n),
        ),
        &add(&shift(&sq(&u, n), 2, n), &shift(&one(), 6, n)),
    );
    assert!(residual.is_empty(), "minimal elliptic q-expansion equation");
    (u, v)
}
fn recover(unit: &[u64], pole: usize, u: &[u64], v: &[u64], n: usize) -> serde_json::Value {
    let mut powers = vec![one()];
    for i in 1..=pole / 2 {
        powers.push(mul(&powers[i - 1], u, n));
    }
    let mut rem = unit.to_vec();
    let mut a = vec![];
    let mut b = vec![];
    for k in (2..=pole).rev() {
        if !bit(&rem, pole - k) {
            continue;
        }
        let basis = if k % 2 == 0 {
            set(&mut a, k / 2);
            powers[k / 2].clone()
        } else {
            set(&mut b, (k - 3) / 2);
            mul(v, &powers[(k - 3) / 2], n)
        };
        rem = add(&rem, &shift(&basis, pole - k, n));
    }
    assert!(!bit(&rem, pole - 1), "unrepresentable pole-one coefficient");
    if bit(&rem, pole) {
        set(&mut a, 0);
        rem = add(&rem, &shift(&one(), pole, n));
    }
    assert!(
        rem.is_empty(),
        "nonzero positive q residual after Riemann-Roch recovery: {}",
        hex(&rem)
    );
    let aa = sq(&a, 2048);
    let ab = shift(&mul(&a, &b, 2048), 1, 2048);
    let bb = mul(&sq(&b, 2048), &[0b1101], 2048);
    let norm = add(&add(&aa, &ab), &bb);
    json!({"pole_bound":pole,"A_bits":hex(&a),"B_bits":hex(&b),"A_degree":degree(&a),"B_degree":degree(&b),"norm_to_F2x_bits":hex(&norm),"norm_to_F2x_degree":degree(&norm),"q_series_normalized_bits":hex(unit),"checked_q_coefficients":n,"positive_coefficients_checked":n-pole-1,"Riemann_Roch_residual_zero":true})
}
pub fn run(dir: &Path) {
    let n = PREC;
    let input = 121 * n;
    let controls = control(n);
    println!("O13:determinant/Euler/direct-field controls passed");
    let (u, v) = xy(n);
    let h = euler(n, 1);
    let h11 = euler(n, 11);
    let h49 = euler(n, 49);
    let h539 = euler(n, 539);
    let h_t = euler(input, 1);
    let h_t49 = euler(input, 49);
    let h_t121 = euler(input, 121);
    let h_t5929 = euler(input, 5929);
    let e_sum = add(
        &mul(&h_t49, &h_t121, input),
        &shift(&mul(&h_t, &h_t5929, input), 240, input),
    );
    let e_prod = add(
        &mul(&h_t, &h_t121, input),
        &shift(&mul(&h_t49, &h_t5929, input), 244, input),
    );
    let ns = norm121(&e_sum, n);
    let np = norm121(&e_prod, n);
    println!("O13:two cyclic121 small-branch norms computed");
    let ds = ratio(
        &mul(&pow(&h49, 133, n), &pow(&h, 121, n), n),
        &pow(&h539, 12, n),
        n,
    );
    let dp = ratio(&pow(&h, 254, n), &pow(&h11, 12, n), n);
    let small_s = mul(&pow(&u, 121, n), &ratio(&ns, &ds, n), n);
    let small_p = mul(
        &ratio(&pow(&u, 254, n), &pow(&dilate(&u, 11, n), 12, n), n),
        &ratio(&np, &dp, n),
        n,
    );
    let large_s = add(&dilate(&u, 121, n), &shift(&u, 240, n));
    let large_p = add(&mul(&u, &dilate(&u, 121, n), n), &shift(&one(), 244, n));
    let sum = mul(&mul(&small_s, &middle(&u, false, n), n), &large_s, n);
    let prod = mul(&mul(&small_p, &middle(&u, true, n), n), &large_p, n);
    let records = vec![
        recover(&sum, 504, &u, &v, n),
        recover(&prod, 528, &u, &v, n),
    ];
    round02::save(
        dir,
        "o13-cyclic121-Hecke-norms.json",
        &json!({"schema":"koblitz.round13/o13-v1","precision":n,"input_precision":input,"Hecke_degree":132,"source_scalar_cover_degree":5,"controls":controls,"eta_x_unit_bits":hex(&u),"minimal_y_unit_bits":hex(&v),"small_branch_numerator_norms":[hex(&ns),hex(&np)],"functions":["x0+xinf","x0*xinf+1"],"records":records,"global_modular_Riemann_Roch_bridge":"written;not inferred solely from a finite jet","source_closed_supports_complete":false,"scope":"exact pushed-down norm functions;ordinary factorization and five-sheet full-level lift require separate certificates"}),
    );
    println!("O13:recovered norms in L(504O),L(528O),all640 coefficients match");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_product_and_unit_inverse() {
        for a in 1u64..40 {
            for b in 1u64..40 {
                let mut expected = 0u128;
                for i in 0..64 {
                    if b >> i & 1 != 0 {
                        expected ^= (a as u128) << i;
                    }
                }
                assert_eq!(clmul(a, b), [expected as u64, (expected >> 64) as u64]);
            }
            if a & 1 != 0 {
                assert_eq!(mul(&[a], &inv(&[a], 128), 128), one());
            }
        }
    }
    #[test]
    fn norm_tower_controls() {
        control(32);
    }
}
