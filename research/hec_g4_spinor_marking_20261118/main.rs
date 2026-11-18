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
fn main() {
    let data = include_str!("../hec_g4_four_torsion_20261118/CONTROL.txt")
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
    let recorded = include_str!("../hec_g4_spinor_20261118/CONTROL.txt");
    let v = array(recorded.lines().next().unwrap());
    let scale = array(recorded.split("scale=").nth(1).unwrap())[0];
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
    assert_eq!((roots.len(), v.len(), basis.len()), (9, 16, 8));
    let mask = |c: usize| {
        basis
            .iter()
            .enumerate()
            .filter(|(i, _)| c & (1 << i) != 0)
            .fold(0, |a, (_, b)| a ^ b)
    };
    let shifts: Vec<_> = (0usize..256)
        .filter(|&o| {
            (0usize..256)
                .all(|c| dimension(o ^ mask(c)) % 2 == ((c & (c >> 4)).count_ones() as usize % 2))
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
            expected = f.mul(scale, f.mul(p, p));
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
        println!("COS​ET bits={bits} schottky={j:?}");
        if j == [Z; 2] {
            possible.push(bits);
        }
    }
    println!("SCREEN zero_coordinates={zero} sign_cosets=32 vanishing_cosets={possible:?}");
}
