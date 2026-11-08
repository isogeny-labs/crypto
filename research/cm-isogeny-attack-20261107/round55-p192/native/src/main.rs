//! Native exact replay of the saved Round55 records. No Python subprocesses.
use serde_json::{json, Value};
use std::{fs, path::Path};
mod p192;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct K(i32, i32);
fn md(x: i32) -> i32 {
    x.rem_euclid(11)
}
fn digits(x: i32) -> [i32; 3] {
    [x % 11, x / 11 % 11, x / 121]
}
fn pack(v: [i32; 3]) -> i32 {
    md(v[0]) + 11 * md(v[1]) + 121 * md(v[2])
}
fn fa(x: i32, y: i32) -> i32 {
    let a = digits(x);
    let b = digits(y);
    pack([a[0] + b[0], a[1] + b[1], a[2] + b[2]])
}
fn fn_(x: i32) -> i32 {
    let a = digits(x);
    pack([-a[0], -a[1], -a[2]])
}
fn fm(x: i32, y: i32) -> i32 {
    let a = digits(x);
    let b = digits(y);
    let mut v = [0; 5];
    for i in 0..3 {
        for j in 0..3 {
            v[i + j] += a[i] * b[j]
        }
    }
    for j in (3..5).rev() {
        let z = v[j];
        v[j - 3] -= 4 * z;
        v[j - 2] -= z;
    }
    pack([v[0], v[1], v[2]])
}
fn fp(mut x: i32, mut n: u64) -> i32 {
    let mut z = 1;
    while n > 0 {
        if n & 1 == 1 {
            z = fm(z, x)
        }
        x = fm(x, x);
        n >>= 1;
    }
    z
}
impl K {
    fn c(x: i32) -> Self {
        Self(md(x), 0)
    }
    fn zero() -> Self {
        Self(0, 0)
    }
    fn pow(mut self, mut n: u64) -> Self {
        let mut z = Self::c(1);
        while n > 0 {
            if n & 1 == 1 {
                z = z * self
            }
            self = self * self;
            n >>= 1;
        }
        z
    }
    fn inverse(self) -> Self {
        assert!(self != Self::zero());
        let z = fp(fa(fm(self.0, self.0), fn_(fm(2, fm(self.1, self.1)))), 1329);
        Self(fm(self.0, z), fn_(fm(self.1, z)))
    }
    fn rational(self) -> bool {
        self.1 == 0 && self.0 < 11
    }
    fn quadratic(self) -> bool {
        self.0 < 11 && self.1 < 11
    }
}
impl std::ops::Add for K {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self(fa(self.0, b.0), fa(self.1, b.1))
    }
}
impl std::ops::Neg for K {
    type Output = Self;
    fn neg(self) -> Self {
        Self(fn_(self.0), fn_(self.1))
    }
}
impl std::ops::Sub for K {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        self + -b
    }
}
impl std::ops::Mul for K {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        Self(
            fa(fm(self.0, b.0), fm(2, fm(self.1, b.1))),
            fa(fm(self.0, b.1), fm(self.1, b.0)),
        )
    }
}
impl std::ops::Div for K {
    type Output = Self;
    fn div(self, b: Self) -> Self {
        self * b.inverse()
    }
}
type P = Option<(K, K)>;
fn neg(p: P) -> P {
    p.map(|(x, y)| (x, -y))
}
fn add(p: P, q: P) -> P {
    match (p, q) {
        (None, _) => q,
        (_, None) => p,
        (Some((x, y)), Some((u, v))) => {
            if x == u && y + v == K::zero() {
                return None;
            }
            let s = if p == q {
                (K::c(3) * x * x + K::c(2)) / (K::c(2) * y)
            } else {
                (v - y) / (u - x)
            };
            let z = s * s - x - u;
            Some((z, s * (x - z) - y))
        }
    }
}
fn mul(mut n: u64, mut p: P) -> P {
    let mut q = None;
    while n > 0 {
        if n & 1 == 1 {
            q = add(q, p)
        }
        p = add(p, p);
        n >>= 1;
    }
    q
}
fn read(v: &Value) -> K {
    let a = v.as_array().unwrap();
    assert_eq!(a.len(), 6);
    K(
        pack([
            a[0].as_i64().unwrap() as i32,
            a[1].as_i64().unwrap() as i32,
            a[2].as_i64().unwrap() as i32,
        ]),
        pack([
            a[3].as_i64().unwrap() as i32,
            a[4].as_i64().unwrap() as i32,
            a[5].as_i64().unwrap() as i32,
        ]),
    )
}
fn point(v: &Value) -> P {
    if v.is_null() {
        None
    } else {
        Some((read(&v[0]), read(&v[1])))
    }
}
fn poly(v: &Value) -> Vec<K> {
    v.as_array().unwrap().iter().map(read).collect()
}
fn eval(a: &[K], x: K) -> K {
    a.iter().rev().fold(K::zero(), |z, &c| z * x + c)
}
fn derivative(a: &[K]) -> Vec<K> {
    (1..a.len()).map(|i| K::c(i as i32) * a[i]).collect()
}
fn pmul(a: &[K], b: &[K]) -> Vec<K> {
    let mut c = vec![K::zero(); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = c[i + j] + x * y;
        }
    }
    c
}
fn rootdivide(a: &[K], z: K) -> Vec<K> {
    let mut q = vec![K::zero(); a.len() - 1];
    q[a.len() - 2] = *a.last().unwrap();
    for i in (1..a.len() - 1).rev() {
        q[i - 1] = a[i] + z * q[i];
    }
    assert_eq!(a[0] + z * q[0], K::zero());
    q
}
fn accumulate(a: &mut Vec<K>, b: &[K], c: K) {
    a.resize(a.len().max(b.len()), K::zero());
    for (i, &v) in b.iter().enumerate() {
        a[i] = a[i] + v * c;
    }
}
// Takes public graph pairs, never the rational target logarithm.
fn reconstruct(er: &[P], image: &[P], shift: P, den: &[K]) -> (Vec<K>, Vec<K>) {
    let mut a = vec![K::zero()];
    a.extend(den.iter().map(|&v| v * shift.unwrap().0));
    let mut b = vec![K::zero()];
    let mut used = vec![];
    for (i, &u) in er.iter().enumerate() {
        if let Some((x, y)) = u {
            if used.contains(&x) {
                continue;
            }
            used.push(x);
            let j = er.iter().position(|&v| v == neg(u)).unwrap();
            let up = add(shift, image[i]).unwrap().0;
            let um = add(shift, image[j]).unwrap().0;
            let q = rootdivide(&rootdivide(den, x), x);
            let part = pmul(&q, &[K::c(2) * x + K::c(14), K::c(2) + x * x, x]);
            accumulate(&mut a, &part, up + um);
            accumulate(&mut b, &q, (um - up) * K::c(2) * y);
        }
    }
    (a, b)
}
fn degree(a: &[K]) -> usize {
    a.iter().rposition(|&x| x != K::zero()).unwrap()
}
fn oncurve(p: P) -> bool {
    p.map(|(x, y)| y * y == x * x * x + K::c(2) * x + K::c(7))
        .unwrap_or(true)
}
fn extract(u: P, a: &[K], b: &[K], psi: &[K], shift: P) -> P {
    let (x, y) = u.unwrap();
    let dp = eval(&derivative(psi), x);
    let k = K::c(4) * y * y * dp * dp;
    assert_ne!(k, K::zero());
    let v = (eval(a, x) - y * eval(b, x)) / k;
    let rhs = v * v * v + K::c(2) * v + K::c(7);
    let mut candidates = vec![];
    for c in 0..11 {
        for d in 0..11 {
            let root = K(c, d);
            if root * root == rhs {
                let p = add(Some((v, root)), neg(shift));
                if let Some((px, py)) = p {
                    if px.rational() && py.rational() && mul(7, p).is_none() {
                        candidates.push(p)
                    }
                }
            }
        }
    }
    assert_eq!(candidates.len(), 1);
    candidates[0]
}
fn pow7(mut x: u64, mut n: u64) -> u64 {
    let mut y = 1;
    while n > 0 {
        if n & 1 == 1 {
            y = y * x % 7
        }
        x = x * x % 7;
        n >>= 1;
    }
    y
}
fn inv7(x: u64) -> u64 {
    assert!(x > 0);
    pow7(x, 5)
}
fn bsgs(g: P, q: P, z: u64, n: u64) -> Option<(u64, u64)> {
    let m = (1..=n).find(|&m| m * m >= n).unwrap();
    let mut baby = std::collections::HashMap::new();
    for j in 0..m {
        baby.insert(mul(pow7(z, j), g), j);
    }
    let mut p = q;
    let f = inv7(pow7(z, m));
    let mut calls = m;
    for i in 0..=m {
        if let Some(&j) = baby.get(&p) {
            let k = i * m + j;
            calls += 1;
            if k < n && mul(pow7(z, k), g) == q {
                return Some((k, calls));
            }
        }
        p = mul(f, p);
        calls += 1;
    }
    None
}
fn solve(g: P, q: P, pd: P, d: u64) -> Option<(u64, u64)> {
    let l = 6 / d;
    let (k2, c1) = bsgs(g, pd, pow7(3, d), l)?;
    let u = mul(inv7(pow7(3, k2)), q);
    let (k1, c2) = bsgs(g, u, pow7(3, l), d)?;
    let x = pow7(3, k2 + l * k1);
    if mul(x, g) != q || mul(pow7(x, d), g) != pd {
        None
    } else {
        Some((x, c1 + c2 + 3))
    }
}
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_str(&fs::read_to_string(p).unwrap()).unwrap()
}
fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("usage: p192-trace-action-check ROUND55_DIRECTORY");
    let root = Path::new(&arg);
    let c = load(root.join("certificate.json"));
    let prev = load(root.join("../round54-p192/certificate.json"));
    let g = point(&c["G"]);
    assert_eq!(c["r"], 7);
    assert_eq!(g, point(&prev["curve"]["G"]));
    assert_eq!(c["targets"].as_array().unwrap().len(), 6);
    let t = point(&prev["curve"]["T"]);
    let s = point(&prev["shift_generator"]);
    let shift = point(&c["R0"]);
    assert_eq!(shift, mul(6, s));
    assert!(oncurve(g) && oncurve(t) && oncurve(s));
    assert!(mul(7, g).is_none() && mul(7, t).is_none());
    assert_eq!(mul(17, s), None);
    assert_ne!(mul(7, s), None);
    let er: Vec<P> = (0..7)
        .flat_map(|i| (0..7).map(move |j| add(mul(i, g), mul(j, t))))
        .collect();
    assert_eq!(
        er.iter().collect::<std::collections::HashSet<_>>().len(),
        49
    );
    let psi = poly(&c["psi"]);
    assert_eq!(degree(&psi), 24);
    assert_eq!(psi[24], K::c(7));
    let mut roots = vec![];
    for u in &er {
        if let Some((x, _)) = u {
            if !roots.contains(x) {
                roots.push(*x)
            }
        }
    }
    assert_eq!(roots.len(), 24);
    let mut prod = vec![K::c(7)];
    for &x in &roots {
        prod = pmul(&prod, &[-x, K::c(1)]);
    }
    assert_eq!(prod, psi);
    let den = pmul(&psi, &psi);
    let dp = derivative(&psi);
    let mut actions = 0;
    let mut traces = 0;
    let mut aux = 0;
    let mut rejected = 0;
    let mut rows = vec![];
    for target in c["targets"].as_array().unwrap() {
        let q = point(&target["Q"]);
        let x = (1..7).find(|&x| mul(x, g) == q).unwrap();
        let a = poly(&target["A"]);
        let b = poly(&target["B"]);
        assert_eq!(degree(&a), 49);
        assert_eq!(degree(&b), 46);
        assert!(a.iter().chain(b.iter()).all(|v| v.quadratic()));
        // Recover the graph map on the basis from the previous explicit permutation, independently of the numerator producer.
        let prior = prev["targets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| point(&v["Q"]) == q)
            .unwrap();
        let perm = prior["psi_permutation"].as_array().unwrap();
        let image: Vec<P> = perm
            .iter()
            .map(|v| er[v.as_u64().unwrap() as usize])
            .collect();
        assert_eq!(image[7], q);
        let (native_a, native_b) = reconstruct(&er, &image, shift, &den);
        assert_eq!(native_a, a);
        assert_eq!(native_b, b);
        for i in 0..7 {
            for j in 0..7 {
                assert_eq!(
                    image[(i * 7 + j) as usize],
                    add(mul(i, q), mul((7 - inv7(x)) % 7 * j % 7, t))
                );
            }
        }
        for n in 1..=3 {
            let p = mul(n, s);
            let (px, py) = p.unwrap();
            assert_ne!(eval(&den, px), K::zero());
            let trace = er.iter().zip(image.iter()).fold(K::zero(), |z, (&u, &v)| {
                z + add(p, u).unwrap().0 * add(shift, v).unwrap().0
            });
            assert_eq!(eval(&a, px) + py * eval(&b, px), eval(&den, px) * trace);
            traces += 1;
        }
        for rec in target["actions"].as_array().unwrap() {
            let u = point(&rec["input"]);
            let out = extract(u, &a, &b, &psi, shift);
            assert_eq!(out, point(&rec["output"]));
            assert_eq!(out, mul(x, u));
            let (ux, uy) = u.unwrap();
            let k = K::c(4) * uy * uy * eval(&dp, ux).pow(2);
            assert_eq!(k, read(&rec["receipt"]["kappa"]));
            assert_eq!(
                (eval(&a, ux) - uy * eval(&b, ux)) / k,
                add(shift, out).unwrap().0
            );
            actions += 1;
        }
        let mut costs = vec![];
        for rec in target["scalar_recoveries"].as_array().unwrap() {
            let d = rec["divisor"].as_u64().unwrap();
            let mut pd = q;
            for _ in 1..d {
                pd = extract(pd, &a, &b, &psi, shift)
            }
            assert_eq!(pd, point(&rec["auxiliary"]));
            assert_eq!(pd, mul(pow7(x, d), g));
            let (result, calls) = solve(g, q, pd, d).unwrap();
            assert_eq!(result, rec["recovered"].as_u64().unwrap());
            assert_eq!(mul(result, g), q);
            assert!(solve(g, q, add(pd, g), d).is_none());
            aux += 1;
            rejected += 1;
            costs.push(json!({"d":d,"action_extractions":d-1,"scalar_multiplication_calls":calls,"recovered":result}));
        }
        rows.push(json!({"target_scalar_checked_only_in_verifier":x,"costs":costs}));
    }
    let big = p192::verify(root);
    let out = json!({"all_checks_pass":true,"backend":"native Rust; independent field and original division-polynomial jet recurrence","dense_numerator_reconstructions":6,"direct_trace_checks":traces,"marked_actions":actions,"auxiliary_recoveries":aux,"wrong_auxiliary_rejections":rejected,"targets":rows,"P192":big,"scientific_standing":"unsigned producer evidence; no trusted signed verdict"});
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
