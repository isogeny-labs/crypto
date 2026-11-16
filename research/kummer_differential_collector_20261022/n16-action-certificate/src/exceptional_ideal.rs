//! Exact univariate U=u(r)=0 branch and zero/one-degree action identities.
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs};
type Mat = [[u64; 4]; 4];
#[derive(Clone, Debug, PartialEq, Eq)]
struct P {
    p: u64,
    c: Vec<u64>,
}
fn fm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn fsu(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
fn pow(mut a: u64, mut k: u64, p: u64) -> u64 {
    let mut b = 1;
    while k > 0 {
        if k & 1 != 0 {
            b = fm(b, a, p)
        }
        a = fm(a, a, p);
        k >>= 1;
    }
    b
}
impl P {
    fn z(p: u64) -> Self {
        Self { p, c: Vec::new() }
    }
    fn n(p: u64, x: u64) -> Self {
        Self::new(p, vec![x % p])
    }
    fn s(p: u64) -> Self {
        Self::new(p, vec![0, 1])
    }
    fn new(p: u64, mut c: Vec<u64>) -> Self {
        while c.last() == Some(&0) {
            c.pop();
        }
        Self { p, c }
    }
    fn add(&self, y: &Self) -> Self {
        assert_eq!(self.p, y.p);
        let mut c = vec![0; self.c.len().max(y.c.len())];
        for (i, &v) in self.c.iter().enumerate() {
            c[i] = v;
        }
        for (i, &v) in y.c.iter().enumerate() {
            c[i] = (c[i] + v) % self.p;
        }
        Self::new(self.p, c)
    }
    fn neg(&self) -> Self {
        Self::new(self.p, self.c.iter().map(|&x| fsu(0, x, self.p)).collect())
    }
    fn sub(&self, y: &Self) -> Self {
        self.add(&y.neg())
    }
    fn scale(&self, x: u64) -> Self {
        Self::new(self.p, self.c.iter().map(|&v| fm(v, x, self.p)).collect())
    }
    fn mul(&self, y: &Self) -> Self {
        if self.c.is_empty() || y.c.is_empty() {
            return Self::z(self.p);
        }
        let mut c = vec![0; self.c.len() + y.c.len() - 1];
        for (i, &a) in self.c.iter().enumerate() {
            for (j, &b) in y.c.iter().enumerate() {
                c[i + j] = (c[i + j] + fm(a, b, self.p)) % self.p;
            }
        }
        Self::new(self.p, c)
    }
    fn square(&self) -> Self {
        self.mul(self)
    }
    fn eval(&self, x: u64) -> u64 {
        self.c
            .iter()
            .rev()
            .fold(0, |a, &b| (fm(a, x, self.p) + b) % self.p)
    }
    fn is_zero(&self) -> bool {
        self.c.is_empty()
    }
}
fn n(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap()
}
fn rows(path: &str) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
fn matrix(v: &Value) -> Mat {
    std::array::from_fn(|i| std::array::from_fn(|j| v[i][j].as_u64().unwrap()))
}
fn conversion(t: &Value, p: u64) -> Mat {
    let l = n(t, "lambda");
    let mu = n(t, "mu");
    let nu = n(t, "nu");
    let d = ["a", "b", "c", "d"].map(|k| n(t, k));
    let c = [mu, fm(l, nu, p), nu, fm(l, mu, p)];
    let e = [(l + nu) % p, (1 + mu) % p, (l + mu) % p, (1 + nu) % p];
    std::array::from_fn(|i| {
        [
            fm(d[i], fm(c[i], e[i], p), p),
            fsu(0, fm(d[i], c[i], p), p),
            fm(d[i], e[i], p),
            fsu(0, d[i], p),
        ]
    })
}
fn degree_one_conversion(t: &Value, p: u64) -> Mat {
    let l = n(t, "lambda");
    let mu = n(t, "mu");
    let nu = n(t, "nu");
    let d = ["a", "b", "c", "d"].map(|k| n(t, k));
    let c = [mu, fm(l, nu, p), nu, fm(l, mu, p)];
    let prod = [(l + nu) % p, (1 + mu) % p, (l + mu) % p, (1 + nu) % p];
    let third = [(mu + 1) % p, (l + nu) % p, (nu + 1) % p, (l + mu) % p];
    std::array::from_fn(|i| {
        [
            fm(d[i], fm(c[i], prod[i], p), p),
            fsu(0, fm(d[i], c[i], p), p),
            fm(d[i], third[i], p),
            fsu(0, d[i], p),
        ]
    })
}
#[allow(clippy::needless_range_loop)] // Explicit 4x4 modular elimination indices.
fn determinant(mut a: Mat, p: u64) -> u64 {
    let mut d = 1;
    for i in 0..4 {
        let Some(k) = (i..4).find(|&k| a[k][i] != 0) else {
            return 0;
        };
        if k != i {
            a.swap(k, i);
            d = fsu(0, d, p);
        }
        let pivot = a[i][i];
        d = fm(d, pivot, p);
        let inv = pow(pivot, p - 2, p);
        for k in i + 1..4 {
            let scale = fm(a[k][i], inv, p);
            for j in i..4 {
                a[k][j] = fsu(a[k][j], fm(scale, a[i][j], p), p);
            }
        }
    }
    d
}
fn apply(m: Mat, v: [P; 4]) -> [P; 4] {
    let p = v[0].p;
    std::array::from_fn(|i| (0..4).fold(P::z(p), |a, j| a.add(&v[j].scale(m[i][j]))))
}
fn apply_numeric(m: Mat, v: [u64; 4], p: u64) -> [u64; 4] {
    std::array::from_fn(|i| (0..4).fold(0, |a, j| (a + fm(m[i][j], v[j], p)) % p))
}
fn cross(a: &[P; 4], b: &[P; 4]) -> Vec<P> {
    let mut out = Vec::new();
    for i in 0..4 {
        for j in i + 1..4 {
            out.push(a[i].mul(&b[j]).sub(&a[j].mul(&b[i])));
        }
    }
    out
}
fn cross_numeric(a: [u64; 4], b: [u64; 4], p: u64) -> Vec<u64> {
    let mut out = Vec::new();
    for i in 0..4 {
        for j in i + 1..4 {
            out.push(fsu(fm(a[i], b[j], p), fm(a[j], b[i], p), p));
        }
    }
    out
}
fn curve(t: &Value, p: u64) -> Vec<u64> {
    let mut f = vec![0; 6];
    f[1] = 1;
    for r in [1, n(t, "lambda"), n(t, "mu"), n(t, "nu")] {
        let mut g = vec![0; 6];
        for i in 0..5 {
            g[i + 1] = (g[i + 1] + f[i]) % p;
            g[i] = fsu(g[i], fm(r, f[i], p), p);
        }
        f = g;
    }
    f
}
fn h0(f: &[u64], a0: &P, a1: &P) -> P {
    let p = a0.p;
    let mut c = P::n(p, 1);
    let mut l = P::z(p);
    let mut h = P::z(p);
    for (i, &coef) in f.iter().enumerate() {
        if i >= 1 {
            h = h.sub(&l.scale(f.get(i + 1).copied().unwrap_or(0)));
        }
        let nc = a0.mul(&l).neg();
        let nl = c.sub(&a1.mul(&l));
        c = nc;
        l = nl;
        let _ = coef;
    }
    h
}
fn run(p: u64, t: &Value, selected: &Value, actions: &Value, wrong_basis: bool) -> Vec<Value> {
    let f = curve(t, p);
    let c = conversion(t, p);
    let c1 = degree_one_conversion(t, p);
    let c2_det = determinant(c, p);
    let c1_det = determinant(c1, p);
    let r0 = n(selected, "r0");
    let delta = n(selected, "delta");
    let di = pow(delta, p - 2, p);
    let s = P::s(p);
    let one = P::n(p, 1);
    let zero = P::z(p);
    let mut out = Vec::new();
    for (i, action) in actions.as_array().unwrap().iter().enumerate() {
        let r = fm(fsu(n(action, "root"), r0, p), di, p);
        let mat = matrix(&action["action"]["matrix"]);
        let rs = P::n(p, r);
        let a0 = rs.mul(&s);
        let a1 = rs.add(&s).neg();
        let h = h0(&f, &a0, &a1);
        let gap = s.sub(&rs);
        let gap2 = gap.square();
        let fs = P::new(p, f.clone());
        let q2 = [
            gap2.clone(),
            a1.neg().mul(&gap2),
            a0.neg().mul(&gap2),
            a0.mul(&a1).add(&h).mul(&gap2).add(&fs),
        ];
        let k2 = apply(c, q2.clone());
        let k1 = apply(
            if wrong_basis { c } else { c1 },
            [zero.clone(), one.clone(), s.clone(), s.square()],
        );
        let mk2 = apply(mat, k2.clone());
        let six = cross(&mk2, &k1);
        let all_zero = six.iter().all(P::is_zero);
        let theta = ["a", "b", "c", "d"].map(|k| n(t, k));
        let tor = std::array::from_fn(|j| k1[j].eval(r));
        let o_to_t = cross_numeric(apply_numeric(mat, theta, p), tor, p);
        let t_to_o = cross_numeric(apply_numeric(mat, tor, p), theta, p);
        out.push(json!({"schema":"kummer-n16-exceptional-ideal/v1","p":p,"action_index":i,"root":r,"f":f,"conversion_matrix_degree_two":c,"conversion_matrix_degree_one":c1,"degree_two_conversion_determinant":c2_det,"degree_one_conversion_determinant":c1_det,"basis_used_for_degree_one":if wrong_basis {"incorrect_c2_diagnostic"} else {"source_c1"},"action_matrix":mat,"k2_cleared":k2.map(|x|x.c),"k1":k1.map(|x|x.c),"six_cross_products":six.iter().map(|x|x.c.clone()).collect::<Vec<_>>(),"all_six_zero":all_zero,"o_to_t_cross":o_to_t,"t_to_o_cross":t_to_o,"gap_power":2,"condition":"s!=r; valid reduced degree-two at U=0","rho_ratio":null}));
        eprintln!("p={p} root={r} six_zero={all_zero}");
    }
    out
}
fn main() {
    let wrong_basis = std::env::var("N16_WRONG_BASIS").ok().as_deref() == Some("1");
    let theta: BTreeMap<u64, Value> =
        rows("research/kummer_theta_20261022/results/theta-models.jsonl")
            .into_iter()
            .map(|v| (n(&v, "p"), v))
            .collect();
    let actions: BTreeMap<u64, Value> =
        rows("research/kummer_coordinates_20261022/results/generic-models.jsonl")
            .into_iter()
            .map(|v| (n(&v, "p"), v))
            .collect();
    let mut out = Vec::new();
    for (&p, v) in &theta {
        let selected = &v["selected"];
        out.extend(run(
            p,
            &selected["constants"],
            selected,
            &actions[&p]["torsion_actions"],
            wrong_basis,
        ));
    }
    out.sort_by_key(|r| (n(r, "p"), n(r, "action_index")));
    let base = "research/kummer_differential_collector_20261022/n16-action-certificate/results";
    if wrong_basis {
        fs::write(
            format!("{base}/exceptional-ideal-wrong-basis.json"),
            format!(
                "{}\n",
                serde_json::to_string(
                    &json!({"schema":"kummer-n16-exceptional-wrong-basis-diagnostic/v1","rows":out})
                )
                .unwrap()
            ),
        )
        .unwrap();
        return;
    }
    let diagnostic: Value = serde_json::from_slice(
        &fs::read(format!("{base}/exceptional-ideal-wrong-basis.json"))
            .expect("run N16_WRONG_BASIS=1 first"),
    )
    .unwrap();
    let wrong: BTreeMap<(u64, u64), &Value> = diagnostic["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| ((n(r, "p"), n(r, "action_index")), r))
        .collect();
    assert_eq!(wrong.len(), 105);
    let mut csv = String::from(
        "schema,p,action_index,root,branch,cleared_gap_power,cross_zero_count,o_to_t_zero_count,t_to_o_zero_count,det_c2_nonzero,det_c1_nonzero,wrong_basis_cross_nonzero_count,wrong_basis_o_to_t_nonzero_count,repeated_root_valid,rho_ratio\n",
    );
    let primes: Vec<u64> = out
        .iter()
        .map(|r| n(r, "p"))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut graph = String::from(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"420\" viewBox=\"0 0 1000 420\"><rect width=\"1000\" height=\"420\" fill=\"white\"/><text x=\"70\" y=\"27\" font-family=\"sans-serif\" font-size=\"19\">Exceptional U=0 action: source degree-one chart and wrong-basis diagnostic</text><text x=\"70\" y=\"47\" font-family=\"sans-serif\" font-size=\"12\">Each model has five roots; the chart matrices differ in their third column</text><line x1=\"70\" y1=\"350\" x2=\"950\" y2=\"350\" stroke=\"black\"/><line x1=\"70\" y1=\"70\" x2=\"70\" y2=\"350\" stroke=\"black\"/>",
    );
    for count in [0, 3, 6] {
        let y = 350.0 - 280.0 * (count as f64) / 6.0;
        graph.push_str(&format!("<line x1=\"70\" y1=\"{y:.1}\" x2=\"950\" y2=\"{y:.1}\" stroke=\"#ddd\"/><text x=\"40\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"11\">{count}</text>",y+4.0));
    }
    for row in &out {
        let p = n(row, "p");
        let idx = n(row, "action_index");
        let wrong_row = wrong[&(p, idx)];
        let cross_zero = row["six_cross_products"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| x.as_array().unwrap().is_empty())
            .count();
        let wrong_cross = wrong_row["six_cross_products"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| !x.as_array().unwrap().is_empty())
            .count();
        let ot_zero = row["o_to_t_cross"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| x.as_u64() == Some(0))
            .count();
        let to_zero = row["t_to_o_cross"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| x.as_u64() == Some(0))
            .count();
        let wrong_ot = wrong_row["o_to_t_cross"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|x| x.as_u64() != Some(0))
            .count();
        csv.push_str(&format!("kummer-n16-exceptional-ideal/v1,{p},{idx},{},u_root_zero_s_ne_r,2,{cross_zero},{ot_zero},{to_zero},{},{},{wrong_cross},{wrong_ot},false,null\n",n(row,"root"),n(row,"degree_two_conversion_determinant")!=0,n(row,"degree_one_conversion_determinant")!=0));
        let model_index = primes.binary_search(&p).unwrap();
        let x = 80.0 + 860.0 * (model_index as f64) / 20.0 + (idx as f64 - 2.0) * 3.2;
        let y_good = 350.0 - 280.0 * ((6 - cross_zero) as f64) / 6.0;
        let y_bad = 350.0 - 280.0 * (wrong_cross as f64) / 6.0;
        graph.push_str(&format!("<circle cx=\"{x:.1}\" cy=\"{y_good:.1}\" r=\"3.6\" fill=\"#2171b5\"><title>p={p}, root={}, source C1, nonzero cross-products={}</title></circle><circle cx=\"{x:.1}\" cy=\"{y_bad:.1}\" r=\"3.6\" fill=\"#d95f02\"><title>p={p}, root={}, wrong C2, nonzero cross-products={wrong_cross}</title></circle>",n(row,"root"),6-cross_zero,n(row,"root")));
    }
    for (i, p) in primes.iter().enumerate() {
        let x = 80.0 + 860.0 * (i as f64) / 20.0;
        graph.push_str(&format!("<text x=\"{x:.1}\" y=\"365\" transform=\"rotate(55 {x:.1} 365)\" font-family=\"sans-serif\" font-size=\"10\">{p}</text>"));
    }
    graph.push_str("<text x=\"705\" y=\"60\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#2171b5\">source C1</text><text x=\"805\" y=\"60\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#d95f02\">wrong C2</text><text x=\"430\" y=\"415\" font-family=\"sans-serif\" font-size=\"12\">recorded field prime p</text><text x=\"12\" y=\"250\" transform=\"rotate(-90 12 250)\" font-family=\"sans-serif\" font-size=\"12\">nonzero cross-products among six</text></svg>\n");
    fs::write(format!("{base}/exceptional-ideal.csv"), csv).unwrap();
    fs::write("research/kummer_differential_collector_20261022/n16-action-certificate/exceptional-ideal-graph.svg",graph).unwrap();
    fs::write(
        format!("{base}/exceptional-ideal.json"),
        format!(
            "{}\n",
            serde_json::to_string(&json!({"schema":"kummer-n16-exceptional-ideal/v1","rows":out}))
                .unwrap()
        ),
    )
    .unwrap();
}
