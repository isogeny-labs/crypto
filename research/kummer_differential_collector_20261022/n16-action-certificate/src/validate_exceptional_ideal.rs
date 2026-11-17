//! Independent reconstruction of the U=0 and O/T_r certificate rows.
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashSet},
    fs,
};
type M = [[u64; 4]; 4];
#[derive(Clone, Debug, PartialEq, Eq)]
struct V {
    p: u64,
    a: Vec<u64>,
}
fn fm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn neg(a: u64, p: u64) -> u64 {
    if a == 0 { 0 } else { p - a }
}
fn modpow(mut x: u64, mut e: u64, p: u64) -> u64 {
    let mut y = 1;
    while e > 0 {
        if e & 1 != 0 {
            y = fm(y, x, p)
        }
        x = fm(x, x, p);
        e >>= 1;
    }
    y
}
fn n(x: &Value, key: &str) -> u64 {
    x[key].as_u64().unwrap()
}
impl V {
    fn poly(p: u64, mut a: Vec<u64>) -> Self {
        while a.last() == Some(&0) {
            a.pop();
        }
        Self { p, a }
    }
    fn con(p: u64, a: u64) -> Self {
        Self::poly(p, vec![a % p])
    }
    fn s(p: u64) -> Self {
        Self::poly(p, vec![0, 1])
    }
    fn raw(x: &Value, p: u64) -> Self {
        let v = x
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a.as_u64().unwrap())
            .collect::<Vec<_>>();
        assert!(v.iter().all(|&a| a < p));
        let z = Self::poly(p, v.clone());
        assert_eq!(z.a, v);
        z
    }
    fn plus(&self, b: &Self) -> Self {
        let mut v = vec![0; self.a.len().max(b.a.len())];
        for (i, &x) in self.a.iter().enumerate() {
            v[i] = x
        }
        for (i, &x) in b.a.iter().enumerate() {
            v[i] = (v[i] + x) % self.p
        }
        Self::poly(self.p, v)
    }
    fn by(&self, k: u64) -> Self {
        Self::poly(self.p, self.a.iter().map(|&x| fm(x, k, self.p)).collect())
    }
    fn minus(&self, b: &Self) -> Self {
        self.plus(&b.by(self.p - 1))
    }
    fn product(&self, b: &Self) -> Self {
        if self.a.is_empty() || b.a.is_empty() {
            return Self::poly(self.p, vec![]);
        }
        let mut v = vec![0; self.a.len() + b.a.len() - 1];
        for (i, &x) in self.a.iter().enumerate() {
            for (j, &y) in b.a.iter().enumerate() {
                v[i + j] = (v[i + j] + fm(x, y, self.p)) % self.p;
            }
        }
        Self::poly(self.p, v)
    }
    fn at(&self, x: u64) -> u64 {
        self.a
            .iter()
            .rev()
            .fold(0, |a, &b| (fm(a, x, self.p) + b) % self.p)
    }
    fn zero(&self) -> bool {
        self.a.is_empty()
    }
}
fn mat(x: &Value) -> M {
    std::array::from_fn(|i| std::array::from_fn(|j| x[i][j].as_u64().unwrap()))
}
fn action(m: &M, q: &[V; 4]) -> [V; 4] {
    let p = q[0].p;
    std::array::from_fn(|i| (0..4).fold(V::con(p, 0), |a, j| a.plus(&q[j].by(m[i][j]))))
}
fn numeric(m: &M, q: [u64; 4], p: u64) -> [u64; 4] {
    std::array::from_fn(|i| (0..4).fold(0, |a, j| (a + fm(m[i][j], q[j], p)) % p))
}
fn cross(a: &[V; 4], b: &[V; 4]) -> Vec<V> {
    let mut z = Vec::new();
    for i in 0..4 {
        for j in i + 1..4 {
            z.push(a[i].product(&b[j]).minus(&a[j].product(&b[i])));
        }
    }
    z
}
fn cross_num(a: [u64; 4], b: [u64; 4], p: u64) -> Vec<u64> {
    let mut z = Vec::new();
    for i in 0..4 {
        for j in i + 1..4 {
            z.push((fm(a[i], b[j], p) + neg(fm(a[j], b[i], p), p)) % p);
        }
    }
    z
}
#[allow(clippy::needless_range_loop)] // Independent explicit modular elimination.
fn determinant(mut a: M, p: u64) -> u64 {
    let mut d = 1;
    for c in 0..4 {
        let Some(k) = (c..4).find(|&i| a[i][c] != 0) else {
            return 0;
        };
        if k != c {
            a.swap(k, c);
            d = neg(d, p)
        }
        let pivot = a[c][c];
        d = fm(d, pivot, p);
        let reciprocal = modpow(pivot, p - 2, p);
        for i in c + 1..4 {
            let scale = fm(a[i][c], reciprocal, p);
            for j in c..4 {
                a[i][j] = (a[i][j] + neg(fm(scale, a[c][j], p), p)) % p;
            }
        }
    }
    d
}
fn curve(t: &Value, p: u64) -> Vec<u64> {
    let mut f = [0; 6];
    f[1] = 1;
    for root in [1, n(t, "lambda"), n(t, "mu"), n(t, "nu")] {
        let mut g = [0; 6];
        for i in 0..5 {
            g[i + 1] = (g[i + 1] + f[i]) % p;
            g[i] = (g[i] + neg(fm(root, f[i], p), p)) % p;
        }
        f = g;
    }
    f.to_vec()
}
fn matrices(t: &Value, p: u64) -> (M, M) {
    let l = n(t, "lambda");
    let mu = n(t, "mu");
    let nu = n(t, "nu");
    let d = ["a", "b", "c", "d"].map(|key| n(t, key));
    let c = [mu, fm(l, nu, p), nu, fm(l, mu, p)];
    let product = [(l + nu) % p, (1 + mu) % p, (l + mu) % p, (1 + nu) % p];
    let third = [(mu + 1) % p, (l + nu) % p, (nu + 1) % p, (l + mu) % p];
    let m2 = std::array::from_fn(|i| {
        [
            fm(d[i], fm(c[i], product[i], p), p),
            neg(fm(d[i], c[i], p), p),
            fm(d[i], product[i], p),
            neg(d[i], p),
        ]
    });
    let m1 = std::array::from_fn(|i| {
        [
            fm(d[i], fm(c[i], product[i], p), p),
            neg(fm(d[i], c[i], p), p),
            fm(d[i], third[i], p),
            neg(d[i], p),
        ]
    });
    (m2, m1)
}
fn chart_h(f: &[u64], a0: &V, a1: &V) -> V {
    let p = a0.p;
    let mut cc = V::con(p, 1);
    let mut ll = V::con(p, 0);
    let mut h = V::con(p, 0);
    for i in 0..f.len() {
        if i + 1 < f.len() {
            h = h.minus(&ll.by(f[i + 1]));
        }
        let nc = a0.product(&ll).by(p - 1);
        let nl = cc.minus(&a1.product(&ll));
        cc = nc;
        ll = nl;
    }
    h
}
fn verify(row: &Value, theta: &Value, action_row: &Value) {
    let p = n(row, "p");
    let idx = n(row, "action_index") as usize;
    assert!(idx < 5);
    let selected = &theta["selected"];
    let t = &selected["constants"];
    let root = n(row, "root");
    assert_eq!(
        (n(selected, "r0") + fm(root, n(selected, "delta"), p)) % p,
        n(action_row, "root")
    );
    let f = curve(t, p);
    let field = V::poly(p, f.clone());
    assert_eq!(
        f,
        row["f"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect::<Vec<_>>()
    );
    assert_eq!(field.at(root), 0);
    let mut derivative = 0;
    for (i, &v) in f.iter().enumerate().skip(1) {
        derivative = (derivative + fm(fm(i as u64, v, p), modpow(root, (i - 1) as u64, p), p)) % p;
    }
    assert_ne!(derivative, 0, "branch root must be simple");
    let (c2, c1) = matrices(t, p);
    let action_mat = mat(&action_row["action"]["matrix"]);
    assert_eq!(c2, mat(&row["conversion_matrix_degree_two"]));
    assert_eq!(c1, mat(&row["conversion_matrix_degree_one"]));
    assert_eq!(action_mat, mat(&row["action_matrix"]));
    assert_eq!(
        n(row, "degree_two_conversion_determinant"),
        determinant(c2, p)
    );
    assert_eq!(
        n(row, "degree_one_conversion_determinant"),
        determinant(c1, p)
    );
    assert_ne!(determinant(c2, p), 0);
    assert_ne!(determinant(c1, p), 0);
    let s = V::s(p);
    let rr = V::con(p, root);
    let gap = s.minus(&rr);
    let gap2 = gap.product(&gap);
    let a0 = rr.product(&s);
    let a1 = rr.plus(&s).by(p - 1);
    let h = chart_h(&f, &a0, &a1);
    let q = [
        gap2.clone(),
        a1.by(p - 1).product(&gap2),
        a0.by(p - 1).product(&gap2),
        a0.product(&a1).plus(&h).product(&gap2).plus(&field),
    ];
    let k2 = action(&c2, &q);
    let k1 = action(&c1, &[V::con(p, 0), V::con(p, 1), s.clone(), s.product(&s)]);
    for i in 0..4 {
        assert_eq!(k2[i], V::raw(&row["k2_cleared"][i], p));
        assert_eq!(k1[i], V::raw(&row["k1"][i], p));
    }
    let cross_products = cross(&action(&action_mat, &k2), &k1);
    assert_eq!(cross_products.len(), 6);
    for (i, v) in cross_products.iter().enumerate() {
        assert_eq!(*v, V::raw(&row["six_cross_products"][i], p));
        assert!(v.zero(), "nonzero exceptional cross product p={p} i={idx}");
    }
    assert_eq!(row["all_six_zero"], true);
    assert_eq!(n(row, "gap_power"), 2);
    let theta_point = ["a", "b", "c", "d"].map(|key| n(t, key));
    let torsion_point = std::array::from_fn(|i| k1[i].at(root));
    let ot = cross_num(numeric(&action_mat, theta_point, p), torsion_point, p);
    let to = cross_num(numeric(&action_mat, torsion_point, p), theta_point, p);
    assert!(ot.iter().all(|&x| x == 0));
    assert!(to.iter().all(|&x| x == 0));
    assert_eq!(
        ot,
        row["o_to_t_cross"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        to,
        row["t_to_o_cross"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect::<Vec<_>>()
    );
}
fn rows(path: &str) -> BTreeMap<u64, Value> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let v: Value = serde_json::from_str(line).unwrap();
            (n(&v, "p"), v)
        })
        .collect()
}
fn main() {
    let base = "research/kummer_differential_collector_20261022/n16-action-certificate/results";
    let data: Value =
        serde_json::from_slice(&fs::read(format!("{base}/exceptional-ideal.json")).unwrap())
            .unwrap();
    assert_eq!(data["schema"], "kummer-n16-exceptional-ideal/v1");
    let theta = rows("research/kummer_theta_20261022/results/theta-models.jsonl");
    let actions = rows("research/kummer_coordinates_20261022/results/generic-models.jsonl");
    let input = data["rows"].as_array().unwrap();
    assert_eq!(input.len(), 105);
    let mut seen = HashSet::new();
    for row in input {
        let p = n(row, "p");
        let i = n(row, "action_index") as usize;
        assert!(seen.insert((p, i)));
        verify(row, &theta[&p], &actions[&p]["torsion_actions"][i]);
    }
    assert_eq!(seen.len(), 105);
    println!(
        "verified 105 U=0 branch identities (630 exact zero cross-products), 105 O↔T_r cases, and 21 C1/C2 determinants"
    );
}
