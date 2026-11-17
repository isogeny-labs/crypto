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
fn raw() -> [u64;15] { crate::setup::qf() }
fn anchors() -> [[u64;3];5] { crate::config::get().points[..5].try_into().unwrap() }
fn ev(f: &[u64], x: u64) -> u64 {
    f.iter().rev().fold(0, |s, a| (s * x + a) % P)
}
fn fp() -> Vec<(usize, usize, u64)> {
    let mut out = vec![];
    let mut k = 0;
    for i in (0..=4).rev() {
        for j in (0..=4 - i).rev() {
            out.push((i, j, raw()[k]));
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
