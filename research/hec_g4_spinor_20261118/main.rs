#[allow(dead_code)]
#[path = "../hec_g4_four_torsion_20261118/field.rs"]
mod field;
use field::{scalar, E, F, Z};
fn nums(s: &str) -> Vec<u64> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
fn sqroot(f: &F, a: E, z: E) -> E {
    if a == Z {
        return Z;
    }
    assert_eq!(f.pow(a, (f.q - 1) / 2), scalar(1));
    let mut q = f.q - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1
    }
    let (mut c, mut x, mut t, mut m) = (f.pow(z, q), f.pow(a, (q + 1) / 2), f.pow(a, q), s);
    while t != scalar(1) {
        let mut i = 0;
        let mut tt = t;
        while tt != scalar(1) {
            tt = f.mul(tt, tt);
            i += 1;
            assert!(i < m)
        }
        let b = f.pow(c, 1u64 << (m - i - 1));
        x = f.mul(x, b);
        c = f.mul(b, b);
        t = f.mul(t, c);
        m = i
    }
    assert_eq!(f.mul(x, x), a);
    x.min(f.sub(Z, x))
}
fn sign(f: &F, x: E, mask: usize) -> E {
    if mask.count_ones() % 2 == 1 {
        f.sub(Z, x)
    } else {
        x
    }
}
fn kernel(f: &F, mut rows: Vec<[E; 16]>) -> [E; 16] {
    let original = rows.clone();
    let mut pivots = Vec::new();
    for j in 0..16 {
        let n = pivots.len();
        let Some(i) = (n..rows.len()).find(|&i| rows[i][j] != Z) else {
            continue;
        };
        rows.swap(n, i);
        let inverse = f.inv(rows[n][j]);
        let pivot = rows[n].map(|x| f.mul(x, inverse));
        rows[n] = pivot;
        for (i, row) in rows.iter_mut().enumerate() {
            if i == n {
                continue;
            }
            let coeff = row[j];
            for (x, p) in row.iter_mut().zip(pivot) {
                *x = f.sub(*x, f.mul(coeff, p));
            }
        }
        pivots.push(j);
    }
    assert_eq!(pivots.len(), 15);
    let free = (0..16).find(|j| !pivots.contains(j)).unwrap();
    let mut v = [Z; 16];
    v[free] = scalar(1);
    for (i, j) in pivots.into_iter().enumerate() {
        v[j] = f.sub(Z, rows[i][free]);
    }
    for row in original {
        assert_eq!(
            row.iter()
                .zip(v)
                .fold(Z, |a, (&b, c)| f.add(a, f.mul(b, c))),
            Z
        );
    }
    v
}
fn main() {
    let line = include_str!("../hec_g4_four_torsion_20261118/CONTROL.txt")
        .lines()
        .next()
        .unwrap();
    let f = F::new(
        31,
        nums(
            line.split("modulus=")
                .nth(1)
                .unwrap()
                .split(']')
                .next()
                .unwrap(),
        ),
    );
    let raw = nums(line.split("roots=").nth(1).unwrap());
    let roots: Vec<E> = raw.chunks_exact(9).map(|a| a.try_into().unwrap()).collect();
    assert_eq!(roots.len(), 9);
    let mut z = Z;
    z[1] = 1;
    let basis: Vec<usize> = include_str!("../hec_g4_weil_lifts_20261118/CONTROL.txt")
        .lines()
        .filter(|s| s.starts_with("SYMPLECTIC index="))
        .map(|s| {
            nums(
                s.split("coefficients=")
                    .nth(1)
                    .unwrap()
                    .split(']')
                    .next()
                    .unwrap(),
            )
            .iter()
            .enumerate()
            .map(|(i, a)| ((*a as usize) % 2) << i)
            .sum()
        })
        .collect();
    assert_eq!(basis.len(), 8);
    let mut rows = vec![[Z; 16]; 64];
    for (i, &r) in roots.iter().enumerate() {
        let wi = if i == 8 { 255 } else { 1 << i };
        let coord = (0usize..256)
            .find(|&s| {
                basis
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| s & (1 << j) != 0)
                    .fold(0, |a, (_, b)| a ^ b)
                    == wi
            })
            .unwrap();
        let (x, b) = (coord & 15, coord >> 4);
        let derivative = roots
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .fold(scalar(1), |a, (_, &s)| f.mul(a, f.sub(r, s)));
        let mut weight = sqroot(&f, sign(&f, f.inv(derivative), x & b), z);
        for k in 0..4 {
            for col in 0..16 {
                let entry = &mut rows[k * 16 + (col ^ x)][col];
                *entry = f.add(*entry, sign(&f, weight, b & col));
            }
            weight = f.mul(weight, r);
        }
    }
    let v = kernel(&f, rows);
    println!("origin={v:?}");
    let mut spectrum = Vec::new();
    for x in 0..16 {
        for b in 0..16 {
            let t = (0..16).fold(Z, |a, i| f.add(a, sign(&f, f.mul(v[i], v[i ^ x]), b & i)));
            if (x & b).count_ones() % 2 == 1 {
                assert_eq!(t, Z);
            }
            if t != Z {
                spectrum.push(f.pow(t, 4));
            }
        }
    }
    assert_eq!(spectrum.len(), 126);
    let mut thomae = Vec::new();
    for mask in 0usize..512 {
        if mask.count_ones() != 4 {
            continue;
        }
        let mut p = scalar(1);
        for i in 0..9 {
            for j in 0..i {
                if (mask >> i ^ mask >> j) & 1 == 0 {
                    p = f.mul(p, f.sub(roots[i], roots[j]));
                }
            }
        }
        thomae.push(f.mul(p, p));
    }
    spectrum.sort();
    thomae.sort();
    let inv = f.inv(thomae[0]);
    let factor = spectrum
        .iter()
        .map(|&s| f.mul(s, inv))
        .find(|&a| {
            let mut scaled = thomae.iter().map(|&p| f.mul(a, p)).collect::<Vec<_>>();
            scaled.sort();
            scaled == spectrum
        })
        .expect("full projective Thomae spectrum mismatch");
    println!("PASS rank=15 residuals=64 odd_zeros=120 even_zeros=10 thomae_multiset=126 scale={factor:?}");
}
