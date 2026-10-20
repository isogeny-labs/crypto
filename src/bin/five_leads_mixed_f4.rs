//! F4 all-roots control for the published five-leads mixed-fiber systems.
//! Builds Semaev S4 and three fiber equations symbolically over F_p.
use crypto_lib::cryptanalysis::f4_fp::{solve, F4Options, Ordering, Poly, Verdict};
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::time::Duration;

#[derive(Clone)]
struct Q {
    p: u64,
    terms: BTreeMap<[u32; 3], u64>,
}
impl Q {
    fn constant(p: u64, c: u64) -> Self {
        let mut terms = BTreeMap::new();
        if c % p != 0 {
            terms.insert([0; 3], c % p);
        }
        Self { p, terms }
    }
    fn var(p: u64, j: usize) -> Self {
        let mut m = [0; 3];
        m[j] = 1;
        Self {
            p,
            terms: BTreeMap::from([(m, 1)]),
        }
    }
    fn add(&self, b: &Self) -> Self {
        assert_eq!(self.p, b.p);
        let mut out = self.clone();
        for (&m, &c) in &b.terms {
            let v = (out.terms.get(&m).copied().unwrap_or(0) + c) % self.p;
            if v == 0 {
                out.terms.remove(&m);
            } else {
                out.terms.insert(m, v);
            }
        }
        out
    }
    fn neg(&self) -> Self {
        Self {
            p: self.p,
            terms: self.terms.iter().map(|(&m, &c)| (m, self.p - c)).collect(),
        }
    }
    fn sub(&self, b: &Self) -> Self {
        self.add(&b.neg())
    }
    fn mul(&self, b: &Self) -> Self {
        assert_eq!(self.p, b.p);
        let mut out = Self::constant(self.p, 0);
        for (&a, &c) in &self.terms {
            for (&d, &e) in &b.terms {
                let mut m = [0; 3];
                for j in 0..3 {
                    m[j] = a[j] + d[j];
                }
                let v = (out.terms.get(&m).copied().unwrap_or(0) + c * e) % self.p;
                if v == 0 {
                    out.terms.remove(&m);
                } else {
                    out.terms.insert(m, v);
                }
            }
        }
        out
    }
    fn sq(&self) -> Self {
        self.mul(self)
    }
    fn eval(&self, x: &[u64; 3]) -> u64 {
        let mut out = 0;
        for (&m, &c) in &self.terms {
            let mut v = c;
            for j in 0..3 {
                for _ in 0..m[j] {
                    v = v * x[j] % self.p;
                }
            }
            out = (out + v) % self.p;
        }
        out
    }
    fn f4(&self) -> Poly {
        self.terms.iter().map(|(m, &c)| (m.to_vec(), c)).collect()
    }
}
fn s3(x: &Q, y: &Q, a: &Q, b: &Q) -> [Q; 3] {
    let p = x.p;
    let sum = x.add(y);
    let product = x.mul(y);
    let leading = x.sub(y).sq();
    let middle = Q::constant(p, 2)
        .mul(&sum.mul(&product.add(a)).add(&Q::constant(p, 2).mul(b)))
        .neg();
    let constant = product
        .sub(a)
        .sq()
        .sub(&Q::constant(p, 4).mul(&b.mul(&sum)));
    [constant, middle, leading]
}
fn system(p: u64, a: u64, b: u64, xs: &[Vec<u64>; 3], xr: u64) -> Vec<Poly> {
    let v: [Q; 3] = std::array::from_fn(|j| Q::var(p, j));
    let aa = Q::constant(p, a);
    let bb = Q::constant(p, b);
    let xr = Q::constant(p, xr);
    let [c, b1, a1] = s3(&v[0], &v[1], &aa, &bb);
    let [g, h, d] = s3(&v[2], &xr, &aa, &bb);
    let res = a1
        .mul(&g)
        .sub(&c.mul(&d))
        .sq()
        .sub(&a1.mul(&h).sub(&b1.mul(&d)).mul(&b1.mul(&g).sub(&c.mul(&h))));
    let mut polys = vec![res.f4()];
    for j in 0..3 {
        let mut vanishing = Q::constant(p, 1);
        for &x in &xs[j] {
            vanishing = vanishing.mul(&v[j].sub(&Q::constant(p, x)));
        }
        polys.push(vanishing.f4());
    }
    polys
}
fn run(
    p: u64,
    a: u64,
    b: u64,
    xs: [Vec<u64>; 3],
    xr: u64,
    budget_seconds: u64,
) -> (serde_json::Value, Vec<Vec<u64>>) {
    let input = system(p, a, b, &xs, xr);
    let mut exhaustive = Vec::new();
    for &x in &xs[0] {
        for &y in &xs[1] {
            for &z in &xs[2] {
                let point = [x, y, z];
                if input.iter().all(|poly| {
                    let q = Q {
                        p,
                        terms: poly.iter().map(|(m, c)| ([m[0], m[1], m[2]], *c)).collect(),
                    };
                    q.eval(&point) == 0
                }) {
                    exhaustive.push(point.to_vec());
                }
            }
        }
    }
    let opts =
        F4Options::new(Ordering::Grevlex, 24).with_budget(Duration::from_secs(budget_seconds));
    let report = solve(&input, 3, p, &opts);
    let mut roots = match &report.verdict {
        Verdict::Solutions(roots) => roots.clone(),
        Verdict::Inconsistent => Vec::new(),
        Verdict::Undetermined => Vec::new(),
    };
    roots.sort();
    exhaustive.sort();
    let decided = report.verdict != Verdict::Undetermined;
    if decided {
        assert_eq!(roots, exhaustive, "F4 roots differ from exhaustive oracle");
    }
    let receipt = json!({"schema":"five-leads-mixed-f4-gate/v2","p":p,"a":a,"b":b,"xs":xs,"target_x":xr,
        "solutions":if decided {Some(roots.len())} else {None},"exhaustive_solutions":exhaustive.len(),
        "complete_and_matching":decided,"solving_degree":report.solving_degree,"degree_reached":report.degree_reached,
        "f4_runs":report.f4_runs,"field_ops":report.field_ops,"max_rows":report.max_rows,"max_cols":report.max_cols,
        "ms_unqualified":report.ms,"timed_out":report.timed_out,"candidates_rejected":report.candidates_rejected});
    (receipt, roots)
}
fn main() {
    if std::env::args().any(|arg| arg == "--dlp-cohort") {
        let input = fs::read_to_string(
            "research/five_leads_e2e_20261019/correctness/mixed-fiber-polynomial-dlp-v1.jsonl",
        )
        .unwrap();
        for line in input.lines() {
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            let p = v["p"].as_u64().unwrap();
            let a = v["a"].as_u64().unwrap();
            let b = v["b"].as_u64().unwrap();
            let xs: [Vec<u64>; 3] = std::array::from_fn(|j| {
                v["fibers"][j]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_u64().unwrap())
                    .collect()
            });
            let mut query_results = Vec::new();
            let mut field_ops = 0u64;
            let mut complete = true;
            for query in v["candidate"]["query_xs"].as_array().unwrap() {
                if let Some(xr) = query.as_u64() {
                    let (receipt, roots) = run(p, a, b, xs.clone(), xr, 2);
                    field_ops += receipt["field_ops"].as_u64().unwrap();
                    complete &= receipt["complete_and_matching"] == true;
                    query_results.push(json!({"x":xr,"roots":roots,"field_ops":receipt["field_ops"],
                        "f4_runs":receipt["f4_runs"],"ms_unqualified":receipt["ms_unqualified"],
                        "complete_and_matching":receipt["complete_and_matching"],"timed_out":receipt["timed_out"]}));
                } else {
                    query_results.push(
                        json!({"x":null,"roots":[],"field_ops":0,"complete_and_matching":true}),
                    );
                }
            }
            assert_eq!(
                query_results.len(),
                v["candidate"]["trials"].as_u64().unwrap() as usize
            );
            println!(
                "{}",
                json!({"schema":"five-leads-mixed-f4-dlp-oracle/v1","p":p,"a":a,"b":b,
                "arm":v["arm"],"target_index":v["target_index"],"target":v["target"],
                "recovered":v["candidate"]["recovered"],"source_verified":v["candidate"]["verified"],
                "complete_and_matching":complete,"queries":query_results,"total_f4_field_ops":field_ops})
            );
        }
        return;
    }
    if std::env::args().any(|arg| arg == "--cohort") {
        let input = fs::read_to_string(
            "research/five_leads_e2e_20261019/correctness/mixed-fiber-polynomial-dlp-v1.jsonl",
        )
        .unwrap();
        for line in input.lines() {
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            let xs: [Vec<u64>; 3] = std::array::from_fn(|j| {
                v["fibers"][j]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_u64().unwrap())
                    .collect()
            });
            let (mut out, _) = run(
                v["p"].as_u64().unwrap(),
                v["a"].as_u64().unwrap(),
                v["b"].as_u64().unwrap(),
                xs,
                v["target"][0].as_u64().unwrap(),
                2,
            );
            out["arm"] = v["arm"].clone();
            out["target_index"] = v["target_index"].clone();
            println!("{out}");
        }
    } else {
        let xs: [Vec<u64>; 3] = std::array::from_fn(|_| vec![16, 24, 36, 43, 54, 61, 73, 81]);
        println!("{}", run(97, 1, 1, xs, 50, 30).0);
    }
}
