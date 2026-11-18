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
fn sum(f: &F, a: Ext, b: Ext) -> Ext {
    [f.add(a[0], b[0]), f.add(a[1], b[1])]
}
fn inverse(f: &F, z: E, x: Ext) -> Ext {
    let n = f.inv(f.sub(f.mul(x[0], x[0]), f.mul(z, f.mul(x[1], x[1]))));
    [f.mul(x[0], n), f.sub(Z, f.mul(x[1], n))]
}
fn descent(f: &F, z: E, values: &[Ext]) -> (bool, Ext) {
    let mut base = values.to_vec();
    base.sort();
    let z15 = f.pow(z, 15);
    let mut image: Vec<_> = base
        .iter()
        .map(|x| [f.pow(x[0], 31), f.mul(f.pow(x[1], 31), z15)])
        .collect();
    image.sort();
    let first = *base.iter().find(|v| **v != [Z; 2]).unwrap();
    let inv = inverse(f, z, first);
    for rhs in image
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
    {
        if rhs == [Z; 2] {
            continue;
        }
        let scale = prod(f, z, rhs, inv);
        // A preliminary membership test avoids sorting for most trial scales.
        if base
            .iter()
            .step_by(13)
            .any(|&x| image.binary_search(&prod(f, z, scale, x)).is_err())
        {
            continue;
        }
        let mut scaled: Vec<_> = base.iter().map(|&x| prod(f, z, scale, x)).collect();
        scaled.sort();
        if scaled == image {
            return (true, scale);
        }
    }
    (false, [Z; 2])
}
fn screen(f: &F, squares: [E; 16], source: bool) -> Vec<usize> {
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
        let mut values = Vec::new();
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
                values.push(t4);
                s8 = sum(&f, s8, t4);
                s16 = sum(&f, s16, prod(&f, z, t4, t4));
            }
        }
        let a = prod(&f, z, s8, s8);
        let j = [0, 1].map(|i| f.sub(a[i], f.mul(scalar(16), s16[i])));
        let (stable, scale) = descent(f, z, &values);
        println!(
            "COSET bits={bits} schottky_zero={} frobenius={stable} scale={scale:?}",
            j == [Z; 2]
        );
        if stable && (!source || j == [Z; 2]) {
            possible.push(bits);
        }
    }
    println!("SCREEN zero_coordinates={zero} sign_cosets=32 passing_cosets={possible:?}");
    possible
}
fn main() {
    let raw = include_str!("../hec_g4_spinor_marking_20261118/panel/CONTROL.txt");
    let line = raw.lines().find(|l| l.starts_with("KERNEL id=2 ")).unwrap();
    let v = array(line.split("origin_vector=").nth(1).unwrap());
    assert_eq!(v.len(), 16);
    let f = F::new(31, vec![17, 0, 30, 0, 5, 0, 6, 0, 1]);
    let squares = std::array::from_fn(|x| (0..16).fold(Z, |a, i| f.add(a, f.mul(v[i], v[i ^ x]))));
    assert_eq!(squares.iter().filter(|x| **x == Z).count(), 2);
    println!("SOURCE");
    assert!(!screen(&f, std::array::from_fn(|i| f.mul(v[i], v[i])), true).is_empty());
    println!("QUOTIENT");
    assert!(!screen(&f, squares, false).is_empty());
}
