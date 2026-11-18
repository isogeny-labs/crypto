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
type Ext = [E; 2];
fn prod(f: &F, z: E, a: Ext, b: Ext) -> Ext {
    [
        f.add(f.mul(a[0], b[0]), f.mul(z, f.mul(a[1], b[1]))),
        f.add(f.mul(a[0], b[1]), f.mul(a[1], b[0])),
    ]
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
fn spinor(f: &F, roots: &[E], basis: &[usize], plane: &[field::P]) -> [E; 16] {
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
        let weight = sqroot(&f, sign(&f, f.inv(derivative), x & b), z);
        for k in 0..4 {
            for col in 0..16 {
                let entry = &mut rows[k * 16 + (col ^ x)][col];
                *entry = f.add(
                    *entry,
                    sign(&f, f.mul(weight, f.eval(&plane[k], r)), b & col),
                );
            }
        }
    }
    kernel(f, rows)
}
#[allow(dead_code)]
#[path = "../../hec_g4_four_torsion_20261118/jac.rs"]
mod jac;
use jac::{C, D};
fn plane(c: &C, d: &D) -> Vec<field::P> {
    let f = c.f;
    let m = d.u.len() - 1;
    let v = c.exact(&f.ps(&c.ff, &f.pm(&d.v, &d.v)), &d.u);
    let mut rows = Vec::new();
    for i in 0..4 - m {
        let mut p = vec![Z; i];
        p.extend(&d.u);
        rows.push(p);
    }
    for i in 0..m {
        let mut xv = vec![Z; i];
        xv.extend(&v);
        let mut xr = vec![Z; i];
        xr.extend(&d.v);
        let (_, a) = f.div(&xv, &d.u);
        let (q, _) = f.div(&c.scale(&xr, scalar(29)), &d.u);
        rows.push(f.pa(&f.pa(&c.scale(&xv, scalar(29)), &a), &f.pm(&q, &d.v)));
    }
    assert_eq!(rows.len(), 4);
    rows
}
fn corr(f: &F, v: [E; 16]) -> [E; 16] {
    std::array::from_fn(|x| (0..16).fold(Z, |a, i| f.add(a, f.mul(v[i], v[i ^ x]))))
}
fn match_image(f: &F, z: E, w: &[Ext; 16], y: &[E; 16]) -> bool {
    let j = y.iter().position(|&v| v != Z).unwrap();
    for shift in 0..16 {
        let p: Vec<_> = (0..16).map(|i| prod(f, z, w[i], w[i ^ shift])).collect();
        if (0..16).any(|i| (p[i] == [Z; 2]) != (y[i] == Z)) {
            continue;
        }
        let left: Vec<_> = y.iter().map(|&a| p[j].map(|e| f.mul(e, a))).collect();
        let right: Vec<_> = p.iter().map(|a| a.map(|e| f.mul(e, y[j]))).collect();
        for b in 0..16 {
            if (0..16).all(|i| left[i] == right[i].map(|e| sign(f, e, b & (i ^ j)))) {
                return true;
            }
        }
    }
    false
}
fn screen(f: &F, squares: [E; 16], images: &[[E; 16]]) -> Vec<usize> {
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
        let matches: Vec<_> = images.iter().map(|y| match_image(f, z, &w, y)).collect();
        println!("COSET bits={bits} matched={matches:?}");
        if matches.iter().all(|&v| v) {
            possible.push(bits);
        }
    }
    println!("PASS zero={zero} compatible_candidates={possible:?}");
    assert!(!possible.is_empty());
    possible
}
fn main() {
    let raw = include_str!("../../hec_g4_four_torsion_20261118/CONTROL.txt");
    let first = raw.lines().next().unwrap();
    let roots = array(first.split("roots=").nth(1).unwrap());
    let f = F::new(
        31,
        nums(
            first
                .split("modulus=")
                .nth(1)
                .unwrap()
                .split(']')
                .next()
                .unwrap(),
        ),
    );
    let c = C {
        f: &f,
        ff: field::poly(&[1, 16, 0, 1, 0, 0, 0, 0, 0, 1]),
        g: 4,
    };
    let line = include_str!("../../hec_g4_spinor_marking_20261118/panel/CONTROL.txt")
        .lines()
        .find(|s| s.starts_with("KERNEL id=2 "))
        .unwrap();
    let basis: Vec<_> = nums(
        line.split("basis=")
            .nth(1)
            .unwrap()
            .split(']')
            .next()
            .unwrap(),
    )
    .into_iter()
    .map(|u| u as usize)
    .collect();
    let eval = |d: &D| spinor(&f, &roots, &basis, &plane(&c, d));
    let origin = eval(&c.zero());
    assert_eq!(
        origin.to_vec(),
        array(line.split("origin_vector=").nth(1).unwrap())
    );
    let halves: Vec<_> = raw
        .lines()
        .filter(|s| s.starts_with("SELECTED "))
        .take(8)
        .map(|s| {
            let (u, v) = s.split(" u=").nth(1).unwrap().split_once(" v=").unwrap();
            D {
                u: array(u),
                v: array(v),
            }
        })
        .collect();
    let mut images = Vec::new();
    for mask in 1usize..16 {
        let coeff = (0..4)
            .filter(|i| mask & (1 << i) != 0)
            .fold(0, |a, i| a ^ basis[i]);
        let mut p = c.zero();
        let mut expected = c.zero();
        for i in 0..8 {
            if coeff & (1 << i) != 0 {
                p = c.add(&p, &halves[i]);
                expected = c.add(&expected, &c.branch(roots[i]));
            }
        }
        assert!(c.valid(&p));
        assert_eq!(c.add(&p, &p), expected);
        let sp = eval(&p);
        let duplicate = eval(&expected);
        let shifted: [E; 16] = std::array::from_fn(|i| origin[i ^ mask]);
        let j = duplicate.iter().position(|&a| a != Z).unwrap();
        assert_ne!(shifted[j], Z);
        for i in 0..16 {
            assert_eq!(
                f.mul(duplicate[i], shifted[j]),
                f.mul(duplicate[j], shifted[i])
            );
        }
        let y = corr(&f, sp);
        assert!(y.iter().any(|&a| a != Z));
        println!(
            "HALF mask={mask} coeff={coeff} degree={} image_products={y:?}",
            p.u.len() - 1
        );
        images.push(y);
    }
    screen(&f, corr(&f, origin), &images);
}
