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
fn sum(f: &F, a: Ext, b: Ext) -> Ext {
    [f.add(a[0], b[0]), f.add(a[1], b[1])]
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
    let mut counts = Vec::new();
    let mut first = None;
    for &bits in if source {
        &[4][..]
    } else {
        &[3, 11, 19, 27][..]
    } {
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
        let initial = *first.get_or_insert(w);
        let monomials: Vec<_> = (0usize..16).filter(|v| v.count_ones() <= 2).collect();
        let witness = (0usize..2048).find(|&q| {
            (0..16).all(|i| {
                let parity = monomials
                    .iter()
                    .enumerate()
                    .filter(|(j, t)| q & (1 << j) != 0 && i & **t == **t)
                    .count()
                    % 2;
                initial[i].map(|e| if parity == 1 { f.sub(Z, e) } else { e }) == w[i]
            })
        });
        let mut even = Vec::new();
        let mut odd = 0;
        for x in 0usize..16 {
            for b in 0usize..16 {
                let t = (0..16).fold([Z; 2], |acc, i| {
                    sum(
                        f,
                        acc,
                        prod(f, z, w[i], w[i ^ x]).map(|a| sign(f, a, i & b)),
                    )
                });
                if (x & b).count_ones() % 2 == 1 {
                    assert_eq!(t, [Z; 2]);
                    odd += 1;
                } else if t == [Z; 2] {
                    even.push(x + 16 * b);
                }
            }
        }
        assert_eq!(odd, 120);
        println!("TYPE bits={bits} zero_coordinates={zero} even_zero={even:?} quadratic_witness={witness:?} w={w:?}");
        counts.push(even.len());
    }
    counts
}
fn main() {
    let raw = include_str!("../../hec_g4_spinor_marking_20261118/panel/CONTROL.txt");
    let line = raw.lines().find(|l| l.starts_with("KERNEL id=2 ")).unwrap();
    let v = array(line.split("origin_vector=").nth(1).unwrap());
    assert_eq!(v.len(), 16);
    let f = F::new(31, vec![17, 0, 30, 0, 5, 0, 6, 0, 1]);
    let squares = std::array::from_fn(|x| (0..16).fold(Z, |a, i| f.add(a, f.mul(v[i], v[i ^ x]))));
    assert_eq!(squares.iter().filter(|x| **x == Z).count(), 2);
    println!("SOURCE");
    assert_eq!(
        screen(&f, std::array::from_fn(|i| f.mul(v[i], v[i])), true),
        vec![10]
    );
    println!("QUOTIENT");
    let counts = screen(&f, squares, false);
    assert!(counts.iter().all(|a| *a == counts[0]));
    println!("CONSENSUS even_zeros={}", counts[0]);
}
