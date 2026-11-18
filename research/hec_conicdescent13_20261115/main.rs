#[allow(dead_code)]
mod pol {
    include!("../hec_inverse13_20261115/verify_poly.rs");
    pub fn marker() {}
}
include!("../hec_inverse13_20261115/verify_poly.rs");
#[allow(dead_code)]
mod target {
    include!("../hec_alignment13_20261115/atlas/target.rs");
    include!("../hec_alignment13_20261115/atlas/target_api.rs");
}
const RAW: [u64; 15] = [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111];
const ANCHORS: [[u64; 3]; 5] = [
    [18, 52, 1],
    [28, 86, 1],
    [43, 125, 1],
    [101, 43, 1],
    [34, 113, 1],
];
fn ev(f: &[u64], x: u64) -> u64 {
    f.iter().rev().fold(0, |s, a| (s * x + a) % P)
}
fn fp() -> Vec<(usize, usize, u64)> {
    let mut out = vec![];
    let mut k = 0;
    for i in (0..=4).rev() {
        for j in (0..=4 - i).rev() {
            out.push((i, j, RAW[k]));
            k += 1
        }
    }
    out
}
fn fy(x: u64) -> U {
    let mut f = vec![0; 5];
    for (i, j, c) in fp() {
        f[j] = (f[j] + c * power(x, i as u64)) % P
    }
    trim(f)
}
fn subst(u: &[u64], v: &[u64]) -> U {
    let mut r = vec![];
    for (i, j, c) in fp() {
        let mut s = vec![c];
        for _ in 0..i {
            s = mm(&s, &[0, 1], u)
        }
        for _ in 0..j {
            s = mm(&s, v, u)
        }
        r = add(&r, &s, false)
    }
    r
}
// c=[1,x,y,x²,xy,y²].
fn cmons(u: &[u64], v: &[u64]) -> Vec<U> {
    vec![
        vec![1],
        vec![0, 1],
        v.to_vec(),
        rem(&[0, 0, 1], u),
        mm(&[0, 1], v, u),
        mm(v, v, u),
    ]
}
fn row(p: [u64; 3]) -> Vec<u64> {
    let [x, y, _] = p;
    vec![1, x, y, x * x % P, x * y % P, y * y % P]
}
fn null(mut a: Vec<Vec<u64>>) -> Option<U> {
    let mut piv = vec![];
    let mut r = 0;
    for c in 0..6 {
        if let Some(i) = (r..a.len()).find(|i| a[*i][c] != 0) {
            a.swap(r, i);
            let z = power(a[r][c], 125);
            for k in c..6 {
                a[r][k] = a[r][k] * z % P
            }
            for i in 0..a.len() {
                if i != r {
                    let z = a[i][c];
                    for k in c..6 {
                        a[i][k] = (a[i][k] + P - z * a[r][k] % P) % P
                    }
                }
            }
            piv.push(c);
            r += 1;
            if r == a.len() {
                break;
            }
        }
    }
    if r != 5 {
        return None;
    }
    let free = (0..6).find(|c| !piv.contains(c))?;
    let mut x = vec![0; 6];
    x[free] = 1;
    for (i, c) in piv.iter().enumerate() {
        x[*c] = (P - a[i][free]) % P
    }
    Some(x)
}
fn det(mut a: Vec<Vec<u64>>) -> u64 {
    let n = a.len();
    let mut z = 1;
    for c in 0..n {
        let Some(i) = (c..n).find(|i| a[*i][c] != 0) else {
            return 0;
        };
        if i != c {
            a.swap(i, c);
            z = (P - z) % P
        }
        let t = a[c][c];
        z = z * t % P;
        let ti = power(t, 125);
        for i in c + 1..n {
            let w = a[i][c] * ti % P;
            for k in c..n {
                a[i][k] = (a[i][k] + P - w * a[c][k] % P) % P
            }
        }
    }
    z
}
fn syl(f: &[u64], g: &[u64]) -> u64 {
    assert_eq!(f.len(), 5);
    assert_eq!(g.len(), 3);
    let mut a = vec![vec![0; 6]; 6];
    for i in 0..2 {
        for k in 0..5 {
            a[i][i + k] = f[4 - k]
        }
    }
    for i in 0..4 {
        for k in 0..3 {
            a[2 + i][i + k] = g[2 - k]
        }
    }
    det(a)
}
fn cy(c: &[u64], x: u64) -> U {
    vec![
        (c[0] + c[1] * x + c[3] * x % P * x) % P,
        (c[2] + c[4] * x) % P,
        c[5],
    ]
}
fn resultant(c: &[u64]) -> U {
    let mut r = vec![];
    for x in 0..9 {
        let mut b = vec![1];
        let mut d = 1;
        for y in 0..9 {
            if x != y {
                b = mul(&b, &[(P - y) % P, 1]);
                d = d * (x + P - y) % P
            }
        }
        r = add(
            &r,
            &scale(&b, syl(&fy(x), &cy(c, x)) * power(d, 125) % P),
            false,
        )
    }
    r
}
fn random(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
fn split(f: U, s: &mut u64, out: &mut Vec<u64>) -> bool {
    if f.len() == 2 {
        out.push((P - f[0]) * power(f[1], 125) % P);
        return true;
    }
    for _ in 0..256 {
        let a = (0..f.len() - 1).map(|_| random(s) % P).collect();
        let h = add(&mpow(a, 63, &f), &[1], true);
        let g = gcd(f.clone(), h);
        if g.len() > 1 && g.len() < f.len() {
            let (q, r) = div(&f, &g);
            assert!(r.is_empty());
            return split(g, s, out) && split(q, s, out);
        }
    }
    false
}
fn derivative(f: &[u64]) -> U {
    trim((1..f.len()).map(|i| i as u64 * f[i] % P).collect())
}
fn class(j: &target::QuarticJacobian, u: &[u64], v: &[u64]) -> target::QClass {
    let mons = target::monomials(3);
    let cols: Vec<U> = mons
        .iter()
        .map(|(a, b, _)| {
            let mut r = vec![1];
            for _ in 0..*a {
                r = mm(&r, &[0, 1], u)
            }
            for _ in 0..*b {
                r = mm(&r, v, u)
            }
            r
        })
        .collect();
    let rows = (0..3)
        .map(|i| {
            cols.iter()
                .map(|c| c.get(i).copied().unwrap_or(0))
                .collect()
        })
        .collect();
    j.class_from_conditions(rows).unwrap()
}
fn main() {
    let j = target::QuarticJacobian::with_points(P, RAW, ANCHORS.to_vec()).unwrap();
    let o = ANCHORS[0];
    let mut ds = j.zero();
    for p in j.d_inf {
        ds = j.add_raw(&ds, &target::difference(&j, p, o))
    }
    let ks = j.add_raw(
        &target::difference(&j, ANCHORS[3], o),
        &target::difference(&j, ANCHORS[4], o),
    );
    let rhs = j.add_raw(&ks, &j.neg_raw(&ds));
    let rhs = j.add_raw(&rhs, &rhs);
    let mut counts = [0; 2];
    for (case, (u, v)) in [
        (vec![9, 31, 82, 1], vec![50, 30, 116]),
        (vec![80, 89, 51, 1], vec![97, 30, 21]),
    ]
    .into_iter()
    .enumerate()
    {
        assert!(subst(&u, &v).is_empty());
        assert_eq!(gcd(u.clone(), derivative(&u)), vec![1]);
        let mut bad = v.clone();
        bad[0] = (bad[0] + 1) % P;
        assert!(!subst(&u, &bad).is_empty());
        let e = class(&j, &u, &v);
        let mons = cmons(&u, &v);
        for a in 0..5 {
            for b in a + 1..5 {
                let mut rows: Vec<Vec<u64>> = (0..3)
                    .map(|i| {
                        mons.iter()
                            .map(|m| m.get(i).copied().unwrap_or(0))
                            .collect()
                    })
                    .collect();
                rows.push(row(ANCHORS[a]));
                rows.push(row(ANCHORS[b]));
                let Some(c) = null(rows) else {
                    println!("EXCLUDE case={case} pair={a},{b} reason=rank");
                    continue;
                };
                if c[5] == 0 {
                    println!("EXCLUDE case={case} pair={a},{b} reason=y_degree");
                    continue;
                }
                let r = resultant(&c);
                for x in 0..P {
                    assert_eq!(ev(&r, x), syl(&fy(x), &cy(&c, x)), "resultant_full_grid")
                }
                if r.len() != 9 {
                    println!(
                        "EXCLUDE case={case} pair={a},{b} reason=infinity_or_projection_degree"
                    );
                    continue;
                }
                let known = mul(
                    &u,
                    &mul(&[(P - ANCHORS[a][0]) % P, 1], &[(P - ANCHORS[b][0]) % P, 1]),
                );
                let (res, remn) = div(&r, &known);
                assert!(remn.is_empty(), "known_divisor_exact");
                assert_eq!(res.len(), 4);
                if gcd(res.clone(), derivative(&res)).len() != 1 {
                    println!("EXCLUDE case={case} pair={a},{b} reason=nonreduced");
                    continue;
                }
                if gcd(res.clone(), known.clone()).len() != 1 {
                    println!("EXCLUDE case={case} pair={a},{b} reason=overlap");
                    continue;
                }
                if !add(&mpow(vec![0, 1], P, &res), &[0, 1], true).is_empty() {
                    println!("EXCLUDE case={case} pair={a},{b} reason=nonsplit");
                    continue;
                }
                let mut roots = vec![];
                assert!(split(res.clone(), &mut 20261115, &mut roots));
                roots.sort();
                assert_eq!(roots.len(), 3);
                let pts: Vec<[u64; 3]> = roots
                    .iter()
                    .map(|x| {
                        let h = gcd(fy(*x), trim(cy(&c, *x)));
                        assert_eq!(h.len(), 2);
                        let y = (P - h[0]) * power(h[1], 125) % P;
                        assert_eq!(ev(&fy(*x), y), 0);
                        assert_eq!(ev(&cy(&c, *x), y), 0);
                        [*x, y, 1]
                    })
                    .collect();
                let rc = j.from_points(pts.clone().try_into().unwrap()).unwrap();
                let mut lhs = j.add_raw(&e, &rc);
                lhs = j.add_raw(&lhs, &target::difference(&j, ANCHORS[a], o));
                lhs = j.add_raw(&lhs, &target::difference(&j, ANCHORS[b], o));
                assert_eq!(lhs, rhs, "independent_divisor_identity");
                counts[case] += 1;
                println!("ACCEPT case={case} pair={a},{b} conic={c:?} residual={res:?} points={pts:?} independent_group_identity=true candidate_group_adds=0");
            }
        }
    }
    println!("COUNTS accepted={counts:?} requested_pairs=20");
    assert!(counts.iter().all(|c| *c > 0), "no_supported_descent");
    println!("PASS conic_descent=true exact_divisor_identities=true corrupted_graph_rejected=true timing_claim=false");
}
