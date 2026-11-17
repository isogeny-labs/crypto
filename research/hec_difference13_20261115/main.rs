#![allow(dead_code)]
mod target {
    include!("../hec_transport13_20261115/reused.rs");
    fn splitmix64(mut x: u64) -> u64 {
        x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = x;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn is_prime_u64(p: u64) -> bool {
        p >= 2 && !(2..).take_while(|d| d * d <= p).any(|d| p % d == 0)
    }
}
const P: u64 = 127;
type Pt = [u64; 3];
fn sub(a: u64, b: u64) -> u64 {
    (a + P - b) % P
}
fn inv(a: u64) -> u64 {
    assert_ne!(a, 0);
    (1..P).find(|b| a * b % P == 1).unwrap()
}
fn dot(a: Pt, b: Pt) -> u64 {
    (0..3).map(|i| a[i] * b[i]).sum::<u64>() % P
}
fn cross(a: Pt, b: Pt) -> Pt {
    [
        sub(a[1] * b[2] % P, a[2] * b[1] % P),
        sub(a[2] * b[0] % P, a[0] * b[2] % P),
        sub(a[0] * b[1] % P, a[1] * b[0] % P),
    ]
}
fn pow(a: u64, n: u8) -> u64 {
    (0..n).fold(1, |v, _| v * a % P)
}
fn gradient(f: &[u64; 15], x: Pt) -> Pt {
    let es = target::monomials(4);
    std::array::from_fn(|j| {
        es.iter()
            .zip(f)
            .map(|(&(a, b, c), &v)| {
                let mut e = [a, b, c];
                if e[j] == 0 {
                    return 0;
                }
                let k = e[j] as u64;
                e[j] -= 1;
                v * k % P * pow(x[0], e[0]) % P * pow(x[1], e[1]) % P * pow(x[2], e[2]) % P
            })
            .sum::<u64>()
            % P
    })
}
fn main() {
    let f = [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111];
    let pts = target::rational_points(&f, P);
    assert_eq!(pts.len(), 129);
    let mut excluded = [0usize; 3];
    let mut tested = 0;
    let mut examples = 0;
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            let (a, b) = (pts[i], pts[j]);
            if a[2] != 1 || b[2] != 1 || a[0] == b[0] {
                excluded[0] += 1;
                continue;
            }
            let ga = gradient(&f, a);
            let gb = gradient(&f, b);
            if ga[1] == 0 || gb[1] == 0 {
                excluded[1] += 1;
                continue;
            }
            let n = cross(a, b);
            let hits: Vec<_> = pts
                .iter()
                .enumerate()
                .filter(|(_, q)| dot(n, **q) == 0)
                .map(|(k, q)| (k, *q))
                .collect();
            if hits.len() != 4 {
                excluded[2] += 1;
                continue;
            }
            let da = sub(n[0] * ga[1] % P, n[1] * ga[0] % P);
            let db = sub(n[0] * gb[1] % P, n[1] * gb[0] % P);
            assert_ne!(da, 0);
            assert_ne!(db, 0);
            // P,Q basis: z=1 and x coordinates yield a left inverse on the chord.
            let d = inv(sub(b[0], a[0]));
            let coords = |q: Pt| {
                let beta = sub(q[0], q[2] * a[0] % P) * d % P;
                [sub(q[2], beta), beta]
            };
            let mut difference = Vec::new();
            let mut sum = Vec::new();
            let mut zero = 0;
            for u in 0..4 {
                for v in u + 1..4 {
                    let x = coords(hits[u].1);
                    let y = coords(hits[v].1);
                    let first = sub(0, da) * x[0] % P * y[0] % P;
                    let second = db * x[1] % P * y[1] % P;
                    if (first + second) % P == 0 {
                        difference.push((hits[u].0, hits[v].0))
                    }
                    if sub(first, second) == 0 {
                        sum.push((hits[u].0, hits[v].0))
                    }
                    zero += 1;
                }
            }
            assert_eq!(difference, vec![(i, j)]);
            assert_eq!(sum.len(), 2);
            assert!(sum.contains(&(i, j)));
            assert_eq!(zero, 6);
            tested += 1;
            if examples < 8 {
                println!("CASE pair={i},{j} P={a:?} Q={b:?} normal={n:?} diagonals={:?} hits={hits:?} difference={difference:?} sum={sum:?} zero_pairs={zero}",[sub(0,da),db]);
                examples += 1
            }
        }
    }
    assert!(tested > 0);
    println!("COUNTS rational_points={} tested_pairs={} excluded_chart={} excluded_Fy={} excluded_nonsplit_chord={} difference_unique={} sum_ambiguous={} zero_rejected={}",pts.len(),tested,excluded[0],excluded[1],excluded[2],tested,tested,tested);
    println!("PASS destination_second_jet_decoder=true source_jet_decoder=false");
}
