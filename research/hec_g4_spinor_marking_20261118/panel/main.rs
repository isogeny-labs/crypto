#[allow(dead_code)]
#[path = "../../hec_g4_four_torsion_20261118/field.rs"]
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
fn sign(f: &F, a: E, m: usize) -> E {
    if m.count_ones() % 2 == 1 {
        f.sub(Z, a)
    } else {
        a
    }
}
fn array(s: &str) -> Vec<E> {
    nums(s)
        .chunks_exact(9)
        .map(|v| v.try_into().unwrap())
        .collect()
}
fn representative(mask: usize) -> usize {
    if mask.count_ones() > 4 {
        511 ^ mask
    } else {
        mask
    }
}
fn dimension(mask: usize) -> usize {
    match representative(mask).count_ones() {
        0 | 1 => 2,
        2 | 3 => 1,
        _ => 0,
    }
}
type Ext = [E; 2];
fn prod(f: &F, z: E, a: Ext, b: Ext) -> Ext {
    [
        f.add(f.mul(a[0], b[0]), f.mul(z, f.mul(a[1], b[1]))),
        f.add(f.mul(a[0], b[1]), f.mul(a[1], b[0])),
    ]
}
fn sum(f: &F, a: Ext, b: Ext) -> Ext {
    [f.add(a[0], b[0]), f.add(a[1], b[1])]
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
fn pairing(a: usize, b: usize) -> usize {
    (((a & b).count_ones() + a.count_ones() * b.count_ones()) % 2) as usize
}
fn adapted(points: &[usize]) -> Vec<usize> {
    let mut span = vec![0];
    let mut basis = Vec::new();
    for &p in points {
        if !span.contains(&p) {
            basis.push(p);
            span.extend(span.clone().iter().map(|a| a ^ p));
        }
    }
    span.sort();
    assert_eq!(span, points);
    assert_eq!(basis.len(), 4);
    for i in 0..4 {
        let dual = (1..256)
            .find(|&p| {
                basis
                    .iter()
                    .enumerate()
                    .all(|(j, &q)| pairing(p, q) == usize::from(j == i))
            })
            .unwrap();
        basis.push(dual);
    }
    for i in 0..8 {
        for j in 0..8 {
            assert_eq!(pairing(basis[i], basis[j]), usize::from((i ^ j) == 4));
        }
    }
    basis
}
fn spinor(f: &F, roots: &[E], basis: &[usize]) -> [E; 16] {
    let mut z = Z;
    z[1] = 1;
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
    kernel(f, rows)
}
fn screen(f: &F, squares: [E; 16]) -> Vec<usize> {
    let mut z = Z;
    z[1] = 1;
    assert_eq!(f.pow(z, (f.q - 1) / 2), scalar(30));
    let mut root = [[Z; 2]; 16];
    let mut zero = 0;
    for (i, &s) in squares.iter().enumerate() {
        if s == Z {
            zero += 1;
            continue;
        }
        root[i] = if f.pow(s, (f.q - 1) / 2) == scalar(1) {
            [sqroot(&f, s, z), Z]
        } else {
            [Z, sqroot(&f, f.mul(s, f.inv(z)), z)]
        };
        assert_eq!(prod(&f, z, root[i], root[i]), [s, Z]);
    }
    let terms: Vec<_> = (0usize..16).filter(|i| i.count_ones() >= 3).collect();
    let mut possible = Vec::new();
    for bits in 0..32 {
        let mut w = root;
        for (i, val) in w.iter_mut().enumerate() {
            let n = terms
                .iter()
                .enumerate()
                .filter(|(j, t)| bits & (1 << j) != 0 && i & **t == **t)
                .count();
            if n % 2 == 1 {
                *val = val.map(|s| f.sub(Z, s));
            }
        }
        let mut s8 = [Z; 2];
        let mut s16 = [Z; 2];
        for x in 0usize..16 {
            let products: Vec<_> = (0..16).map(|i| prod(&f, z, w[i], w[i ^ x])).collect();
            for b in 0..16 {
                let t = products.iter().enumerate().fold([Z; 2], |a, (i, v)| {
                    sum(&f, a, v.map(|e| sign(&f, e, b & i)))
                });
                let t2 = prod(&f, z, t, t);
                let t4 = prod(&f, z, t2, t2);
                s8 = sum(&f, s8, t4);
                s16 = sum(&f, s16, prod(&f, z, t4, t4));
            }
        }
        let a = prod(&f, z, s8, s8);
        let j = [0, 1].map(|i| f.sub(a[i], f.mul(scalar(16), s16[i])));
        println!("COSET bits={bits} schottky={j:?}");
        if j == [Z; 2] {
            possible.push(bits);
        }
    }
    println!("SCREEN zero_coordinates={zero} sign_cosets=32 vanishing_cosets={possible:?}");
    possible
}
fn main() {
    let data = include_str!("../../hec_g4_four_torsion_20261118/CONTROL.txt")
        .lines()
        .next()
        .unwrap();
    let f = F::new(
        31,
        nums(
            data.split("modulus=")
                .nth(1)
                .unwrap()
                .split(']')
                .next()
                .unwrap(),
        ),
    );
    let roots = array(data.split("roots=").nth(1).unwrap());
    let kernels = include_str!("../../hec_g4_theta_admission_20261118/CONTROL.txt");
    let cases: Vec<_> = kernels
        .lines()
        .filter(|s| s.starts_with("KERNEL id="))
        .collect();
    assert_eq!(cases.len(), 11);
    for (id, line) in cases.iter().enumerate() {
        assert_eq!(nums(line)[0], id as u64);
        let points: Vec<_> = nums(line.split("abstract=").nth(1).unwrap())
            .into_iter()
            .map(|x| x as usize)
            .collect();
        let basis = adapted(&points);
        let v = spinor(&f, &roots, &basis);
        println!("KERNEL id={id} basis={basis:?} origin_vector={v:?}");
        let mut scale = None;
        let mask = |c: usize| {
            basis
                .iter()
                .enumerate()
                .filter(|(i, _)| c & (1 << i) != 0)
                .fold(0, |a, (_, b)| a ^ b)
        };
        let shifts: Vec<_> = (0usize..256)
            .filter(|&o| {
                (0usize..256).all(|c| {
                    dimension(o ^ mask(c)) % 2 == ((c & (c >> 4)).count_ones() as usize % 2)
                })
            })
            .collect();
        assert_eq!(shifts.len(), 1);
        let origin = shifts[0];
        println!("parity_origin={origin}");
        let mut squares = [Z; 16];
        let mut nonzero = 0;
        for c in 0usize..256 {
            let (x, b) = (c & 15, c >> 4);
            let t = (0..16).fold(Z, |a, i| f.add(a, sign(&f, f.mul(v[i], v[i ^ x]), i & b)));
            let part = representative(origin ^ mask(c));
            let mut expected = Z;
            if part.count_ones() == 4 {
                let mut p = scalar(1);
                for i in 0..9 {
                    for j in 0..i {
                        if ((part >> i) ^ (part >> j)) & 1 == 0 {
                            p = f.mul(p, f.sub(roots[i], roots[j]));
                        }
                    }
                }
                let p2 = f.mul(p, p);
                let common = *scale.get_or_insert_with(|| f.mul(f.pow(t, 4), f.inv(p2)));
                assert_ne!(common, Z);
                expected = f.mul(common, p2);
                nonzero += 1;
            }
            assert_eq!(f.pow(t, 4), expected, "characteristic {c} partition {part}");
            if b == 0 {
                squares[x] = t;
            }
            println!("LABEL c={c} partition={part} eighth={expected:?}");
        }
        assert_eq!(nonzero, 126);
        println!("MARKED checked=256 nonzero=126");
        println!("QUOTIENT id={id}");
        let possible = screen(&f, squares);
        println!("STATUS id={id} candidates={possible:?}");
        if id == 0 {
            assert!(possible.is_empty());
            println!("POSITIVE source_sign_recovery");
            assert!(!screen(&f, v.map(|a| f.mul(a, a))).is_empty());
        }
    }
}
