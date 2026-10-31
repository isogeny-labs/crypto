//! Exact, sparse polynomial reductions for the generic torsion action.
//! Variable order: a0, a1, b0, b1. All coefficients lie in the recorded F_p.
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

type Exp = [u8; 4];
type Mat = [[u64; 4]; 4];
#[derive(Clone, Debug, Default)]
struct Poly {
    p: u64,
    terms: BTreeMap<Exp, u64>,
}
fn fm(x: u64, y: u64, p: u64) -> u64 {
    ((x as u128 * y as u128) % p as u128) as u64
}
fn fsu(x: u64, y: u64, p: u64) -> u64 {
    (x + p - y) % p
}
fn fpow(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut y = 1;
    while n > 0 {
        if n & 1 != 0 {
            y = fm(y, x, p)
        }
        x = fm(x, x, p);
        n >>= 1;
    }
    y
}
impl Poly {
    fn zero(p: u64) -> Self {
        Self {
            p,
            terms: BTreeMap::new(),
        }
    }
    fn c(p: u64, v: u64) -> Self {
        let mut y = Self::zero(p);
        y.add_term([0; 4], v % p);
        y
    }
    fn var(p: u64, i: usize) -> Self {
        let mut y = Self::zero(p);
        let mut e = [0; 4];
        e[i] = 1;
        y.add_term(e, 1);
        y
    }
    fn add_term(&mut self, e: Exp, v: u64) {
        let w = (self.terms.get(&e).copied().unwrap_or(0) + v) % self.p;
        if w == 0 {
            self.terms.remove(&e);
        } else {
            self.terms.insert(e, w);
        }
    }
    fn add(&self, y: &Self) -> Self {
        assert_eq!(self.p, y.p);
        let mut z = self.clone();
        for (&e, &v) in &y.terms {
            z.add_term(e, v)
        }
        z
    }
    fn neg(&self) -> Self {
        let mut z = Self::zero(self.p);
        for (&e, &v) in &self.terms {
            z.add_term(e, fsu(0, v, self.p))
        }
        z
    }
    fn sub(&self, y: &Self) -> Self {
        self.add(&y.neg())
    }
    fn scale(&self, v: u64) -> Self {
        let mut z = Self::zero(self.p);
        for (&e, &c) in &self.terms {
            z.add_term(e, fm(c, v, self.p))
        }
        z
    }
    fn mul(&self, y: &Self) -> Self {
        assert_eq!(self.p, y.p);
        let mut z = Self::zero(self.p);
        for (&e, &a) in &self.terms {
            for (&f, &b) in &y.terms {
                let mut g = [0; 4];
                for i in 0..4 {
                    g[i] = e[i].checked_add(f[i]).expect("exponent overflow")
                }
                z.add_term(g, fm(a, b, self.p))
            }
        }
        z
    }
    fn pow(&self, n: u32) -> Self {
        let mut z = Self::c(self.p, 1);
        for _ in 0..n {
            z = z.mul(self)
        }
        z
    }
    fn mul_monom(&self, e: Exp, c: u64) -> Self {
        let mut z = Self::zero(self.p);
        for (&f, &v) in &self.terms {
            let mut g = [0; 4];
            for i in 0..4 {
                g[i] = e[i].checked_add(f[i]).unwrap()
            }
            z.add_term(g, fm(v, c, self.p))
        }
        z
    }
    fn count(&self) -> usize {
        self.terms.len()
    }
    fn raw(&self) -> Vec<[u64; 5]> {
        self.terms
            .iter()
            .map(|(e, c)| {
                [
                    u64::from(e[0]),
                    u64::from(e[1]),
                    u64::from(e[2]),
                    u64::from(e[3]),
                    *c,
                ]
            })
            .collect()
    }
}
fn xmul(x: &[Poly], y: &[Poly], p: u64) -> Vec<Poly> {
    let mut z = vec![Poly::zero(p); x.len() + y.len() - 1];
    for (i, a) in x.iter().enumerate() {
        for (j, b) in y.iter().enumerate() {
            z[i + j] = z[i + j].add(&a.mul(b))
        }
    }
    z
}
fn n(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap()
}
fn matrix(v: &Value) -> Mat {
    std::array::from_fn(|i| std::array::from_fn(|j| v[i][j].as_u64().unwrap()))
}
#[allow(clippy::needless_range_loop)] // Indexing is the explicit 4x8 Gaussian elimination schedule.
fn inverse(m: Mat, p: u64) -> Mat {
    let mut a = [[0; 8]; 4];
    for i in 0..4 {
        for j in 0..4 {
            a[i][j] = m[i][j]
        }
        a[i][i + 4] = 1;
    }
    for c in 0..4 {
        let r = (c..4).find(|&i| a[i][c] != 0).unwrap();
        a.swap(c, r);
        let d = fpow(a[c][c], p - 2, p);
        for j in 0..8 {
            a[c][j] = fm(a[c][j], d, p)
        }
        for i in 0..4 {
            if i != c {
                let t = a[i][c];
                for j in 0..8 {
                    a[i][j] = fsu(a[i][j], fm(t, a[c][j], p), p)
                }
            }
        }
    }
    std::array::from_fn(|i| std::array::from_fn(|j| a[i][j + 4]))
}
fn mm(x: Mat, y: Mat, p: u64) -> Mat {
    std::array::from_fn(|i| {
        std::array::from_fn(|j| (0..4).fold(0, |s, k| (s + fm(x[i][k], y[k][j], p)) % p))
    })
}
fn conversion(t: &Value, p: u64) -> Mat {
    let (l, mu, nu) = (n(t, "lambda"), n(t, "mu"), n(t, "nu"));
    let d = ["a", "b", "c", "d"].map(|k| n(t, k));
    let c = [mu, fm(nu, l, p), nu, fm(mu, l, p)];
    let e = [(l + nu) % p, (1 + mu) % p, (l + mu) % p, (1 + nu) % p];
    std::array::from_fn(|i| {
        [
            fm(d[i], fm(c[i], e[i], p), p),
            fm(d[i], fsu(0, c[i], p), p),
            fm(d[i], e[i], p),
            fsu(0, d[i], p),
        ]
    })
}
fn rem_f(f: &[u64], a0: &Poly, a1: &Poly, p: u64) -> (Poly, Poly, Poly) {
    let mut c = Poly::c(p, 1);
    let mut l = Poly::zero(p);
    let mut r0 = Poly::zero(p);
    let mut r1 = Poly::zero(p);
    let mut h0 = Poly::zero(p);
    for (i, &v) in f.iter().enumerate() {
        r0 = r0.add(&c.scale(v));
        r1 = r1.add(&l.scale(v));
        if i >= 2 {
            h0 = h0.sub(&l.scale(v))
        }
        let nc = a0.mul(&l).neg();
        let nl = c.sub(&a1.mul(&l));
        c = nc;
        l = nl;
    } // h0 is corrected below from the recurrence directly.
    let mut lin = Poly::zero(p);
    let mut prev = Poly::zero(p);
    let mut con = Poly::c(p, 1);
    h0 = Poly::zero(p);
    for (i, &v) in f.iter().enumerate() {
        if i >= 2 {
            h0 = h0.sub(&prev.scale(v))
        }
        let nc = a0.mul(&lin).neg();
        let nl = con.sub(&a1.mul(&lin));
        prev = lin;
        lin = nl;
        con = nc;
    }
    (r0, r1, h0)
}
fn reduce(mut v: Poly, e0: &Poly, e1: &Poly) -> (Poly, Poly, Poly, u64) {
    let p = v.p;
    let mut q0 = Poly::zero(p);
    let mut q1 = Poly::zero(p);
    let mut steps = 0;
    loop {
        let candidate = v
            .terms
            .iter()
            .find(|(e, _)| e[2] >= 2 || (e[2] >= 1 && e[3] >= 1))
            .map(|(&e, &c)| (e, c));
        let Some((e, c)) = candidate else { break };
        let mut q = e;
        let (generator, quotient) = if e[2] >= 2 {
            q[2] -= 2;
            (e0, &mut q0)
        } else {
            q[2] -= 1;
            q[3] -= 1;
            (e1, &mut q1)
        };
        let c = if e[2] >= 2 {
            c
        } else {
            fm(c, fpow(2, p - 2, p), p)
        };
        v = v.sub(&generator.mul_monom(q, c));
        quotient.add_term(q, c);
        steps += 1;
        assert!(steps < 10_000_000, "reduction budget exceeded");
    }
    (v, q0, q1, steps)
}
fn f_poly(t: &Value, p: u64) -> Vec<u64> {
    let mut f = vec![0; 6];
    f[1] = 1;
    for r in [1, n(t, "lambda"), n(t, "mu"), n(t, "nu")] {
        let mut g = vec![0; 6];
        for i in 0..5 {
            g[i + 1] = (g[i + 1] + f[i]) % p;
            g[i] = fsu(g[i], fm(r, f[i], p), p)
        }
        f = g
    }
    f
}
fn q3_numerator(
    q0: &Poly,
    q1: &Poly,
    u: &Poly,
    vr: &Poly,
    a1: &Poly,
    b1: &Poly,
    h0: &Poly,
) -> (Poly, u32) {
    let p = u.p;
    let d = h0
        .terms
        .keys()
        .map(|e| u32::from(e[0]) + u32::from(e[1]))
        .max()
        .unwrap_or(0);
    let power = (2 * d).max(6);
    let u2 = u.pow(2);
    let mut hnum = Poly::zero(p);
    for (&e, &c) in &h0.terms {
        assert_eq!(e[2], 0);
        assert_eq!(e[3], 0);
        let degree = u32::from(e[0]) + u32::from(e[1]);
        let t = q0
            .pow(u32::from(e[0]))
            .mul(&q1.pow(u32::from(e[1])))
            .mul(&u2.pow(d - degree))
            .scale(c);
        hnum = hnum.add(&t);
    }
    let bnum = b1.neg().mul(&u.pow(3)).add(&vr.mul(&a1.mul(&u2).sub(q1)));
    let q3num = q0
        .mul(q1)
        .mul(&u.pow(power - 4))
        .add(&bnum.pow(2).mul(&u.pow(power - 6)))
        .add(&hnum.mul(&u.pow(power - 2 * d)));
    (q3num, power)
}
fn run_model(p: u64, t: &Value, actions: &Value, r0: u64, delta: u64) -> Vec<Value> {
    let a0 = Poly::var(p, 0);
    let a1 = Poly::var(p, 1);
    let b0 = Poly::var(p, 2);
    let b1 = Poly::var(p, 3);
    let one = Poly::c(p, 1);
    let zero = Poly::zero(p);
    let f = f_poly(t, p);
    let (r0p, r1p, h0) = rem_f(&f, &a0, &a1, p);
    let e0 = b0.pow(2).sub(&a0.mul(&b1.pow(2))).sub(&r0p);
    let e1 = b0.mul(&b1).scale(2).sub(&a1.mul(&b1.pow(2))).sub(&r1p);
    let q = [
        one.clone(),
        a1.neg(),
        a0.neg(),
        a0.mul(&a1).add(&b1.pow(2)).add(&h0),
    ];
    let c = conversion(t, p);
    let ci = inverse(c, p);
    let di = fpow(delta, p - 2, p);
    let mut out = Vec::new();
    for (idx, action) in actions.as_array().unwrap().iter().enumerate() {
        let root = fm(fsu(n(action, "root"), r0, p), di, p);
        let m = matrix(&action["action"]["matrix"]);
        let trans = mm(mm(ci, m, p), c, p);
        let w: Vec<Poly> = (0..4)
            .map(|i| (0..4).fold(zero.clone(), |s, j| s.add(&q[j].scale(trans[i][j]))))
            .collect();
        let r = Poly::c(p, root);
        let u = a0.add(&a1.mul(&r)).add(&r.pow(2));
        let vr = b0.add(&b1.mul(&r));
        let lnum = vec![
            u.mul(&b0).sub(&vr.mul(&a0)),
            u.mul(&b1).sub(&vr.mul(&a1)),
            vr.neg(),
        ];
        let mut numerator = vec![zero.clone(); 6];
        let u2 = u.pow(2);
        for (i, &v) in f.iter().enumerate() {
            numerator[i] = u2.scale(v)
        }
        let l2 = xmul(&lnum, &lnum, p);
        for i in 0..l2.len() {
            numerator[i] = numerator[i].sub(&l2[i])
        }
        let denom = [
            a0.mul(&r).neg(),
            a0.sub(&a1.mul(&r)),
            a1.sub(&r),
            one.clone(),
        ];
        let mut rem = numerator.clone();
        let mut quo = vec![zero.clone(); 3];
        for k in (0..3).rev() {
            quo[k] = rem[k + 3].clone();
            for j in 0..4 {
                rem[j + k] = rem[j + k].sub(&quo[k].mul(&denom[j]))
            }
        }
        let i1 = quo[1].mul(&w[0]).add(&u2.mul(&w[1]));
        let i2 = quo[0].mul(&w[0]).add(&u2.mul(&w[2]));
        let (q3num, denominator_power) = q3_numerator(&quo[0], &quo[1], &u, &vr, &a1, &b1, &h0);
        let i3 = q3num.mul(&w[0]).sub(&w[3].mul(&u.pow(denominator_power)));
        let translated_num = [
            u.pow(denominator_power),
            quo[1].neg().mul(&u.pow(denominator_power - 2)),
            quo[0].neg().mul(&u.pow(denominator_power - 2)),
            q3num,
        ];
        let source_k: Vec<Poly> = (0..4)
            .map(|i| (0..4).fold(zero.clone(), |s, j| s.add(&w[j].scale(c[i][j]))))
            .collect();
        let translated_k: Vec<Poly> = (0..4)
            .map(|i| {
                (0..4).fold(zero.clone(), |s, j| {
                    s.add(&translated_num[j].scale(c[i][j]))
                })
            })
            .collect();
        let cross_products: Vec<Poly> = (0..4)
            .flat_map(|i| {
                ((i + 1)..4).map({
                    let source_k = &source_k;
                    let translated_k = &translated_k;
                    move |j| {
                        translated_k[i]
                            .mul(&source_k[j])
                            .sub(&translated_k[j].mul(&source_k[i]))
                    }
                })
            })
            .collect();
        let cross_certificates: Vec<Value> = cross_products.iter().map(|v| { let (remainder, e0_quotient, e1_quotient, steps)=reduce(v.clone(),&e0,&e1);json!({"cross_product":v.raw(),"e0_quotient":e0_quotient.raw(),"e1_quotient":e1_quotient.raw(),"remainder":remainder.raw(),"steps":steps}) }).collect();
        let cross_remainders: Vec<usize> = cross_certificates
            .iter()
            .map(|v| v["remainder"].as_array().unwrap().len())
            .collect();
        let (z1, q10, q11, s1) = reduce(i1.clone(), &e0, &e1);
        let (z2, q20, q21, s2) = reduce(i2.clone(), &e0, &e1);
        let (z3, q30, q31, s3) = reduce(i3.clone(), &e0, &e1);
        let quotient_remainder_certificates: Vec<Value> = rem[..3].iter().map(|v| {let (residue,q0,q1,steps)=reduce(v.clone(),&e0,&e1);json!({"polynomial":v.raw(),"e0_quotient":q0.raw(),"e1_quotient":q1.raw(),"remainder":residue.raw(),"steps":steps})}).collect();
        let rem_counts: Vec<usize> = quotient_remainder_certificates
            .iter()
            .map(|v| v["remainder"].as_array().unwrap().len())
            .collect();
        let w0_factor = w[0].terms.get(&[1, 0, 0, 0]).copied().unwrap_or(0);
        let w0_equals_factor_u = w[0].sub(&u.scale(w0_factor)).count() == 0;
        out.push(json!({"p":p,"action_index":idx,"root":root,"generic_branch":"u(root)!=0","denominator_power":denominator_power,"w0_factor":w0_factor,"w0_equals_factor_u":w0_equals_factor_u,"curve_coefficients":f,"conversion_matrix":c,"action_matrix":m,"mumford_generators":[e0.raw(),e1.raw()],"translated_num":translated_num.map(|v|v.raw()),"source_w":w.iter().map(Poly::raw).collect::<Vec<_>>(),"six_cross_product_certificates":cross_certificates,"quotient_remainder_certificates":quotient_remainder_certificates,"quotient_identity_remainders":rem_counts,"six_cross_product_remainders":cross_remainders,"coefficient_identity":[{"label":"a1","input_terms":i1.count(),"remainder_terms":z1.count(),"e0_quotient_terms":q10.count(),"e1_quotient_terms":q11.count(),"steps":s1},{"label":"a0","input_terms":i2.count(),"remainder_terms":z2.count(),"e0_quotient_terms":q20.count(),"e1_quotient_terms":q21.count(),"steps":s2},{"label":"q3","input_terms":i3.count(),"remainder_terms":z3.count(),"e0_quotient_terms":q30.count(),"e1_quotient_terms":q31.count(),"steps":s3}],"complete_kummer_identity":null}));
        eprintln!(
            "p={p} root={root} quotient={rem_counts:?} a1={} a0={} q3={}",
            z1.count(),
            z2.count(),
            z3.count()
        );
    }
    out
}
fn main() {
    let theta_path = "research/kummer_theta_20261022/results/theta-models.jsonl";
    let actions_path = "research/kummer_coordinates_20261022/results/generic-models.jsonl";
    let theta: HashMap<u64, Value> = fs::read_to_string(theta_path)
        .unwrap()
        .lines()
        .map(|s| {
            let v: Value = serde_json::from_str(s).unwrap();
            (n(&v, "p"), v)
        })
        .collect();
    let actions: HashMap<u64, Value> = fs::read_to_string(actions_path)
        .unwrap()
        .lines()
        .map(|s| {
            let v: Value = serde_json::from_str(s).unwrap();
            (n(&v, "p"), v)
        })
        .collect();
    let mut output = Vec::new();
    for (&p, v) in &theta {
        let s = &v["selected"];
        output.extend(run_model(
            p,
            &s["constants"],
            &actions[&p]["torsion_actions"],
            n(s, "r0"),
            n(s, "delta"),
        ));
    }
    output.sort_by_key(|v| (n(v, "p"), n(v, "action_index")));
    let base = "research/kummer_differential_collector_20261022/n16-action-certificate/results";
    let mut csv = String::from(
        "schema,p,p_bits,action_index,root,generic_scope,denominator_power,w0_factor,quotient_zero_count,cross_zero_count,max_cross_input_terms,max_cross_steps,exceptional_u_zero,degree_one_zero_action,rho_ratio\n",
    );
    let mut svg = String::from(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"470\" viewBox=\"0 0 1000 470\"><rect width=\"1000\" height=\"470\" fill=\"white\"/><text x=\"70\" y=\"30\" font-family=\"sans-serif\" font-size=\"19\">Generic torsion-action ideal certificates: reduction steps</text><text x=\"70\" y=\"49\" font-family=\"sans-serif\" font-size=\"12\">Five roots per model; each point represents six zero projective remainders, generic U ≠ 0 only</text><line x1=\"70\" y1=\"400\" x2=\"950\" y2=\"400\" stroke=\"black\"/><line x1=\"70\" y1=\"70\" x2=\"70\" y2=\"400\" stroke=\"black\"/>",
    );
    let primes: Vec<u64> = output
        .iter()
        .map(|r| n(r, "p"))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    for tick in [0_u64, 50, 100, 150] {
        let y = 400.0 - 330.0 * (tick as f64) / 150.0;
        svg.push_str(&format!("<line x1=\"70\" y1=\"{y:.1}\" x2=\"950\" y2=\"{y:.1}\" stroke=\"#ddd\"/><text x=\"28\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"11\">{tick}</text>",y+4.0));
    }
    let colors = ["#1f77b4", "#d95f02", "#2ca02c", "#9467bd", "#b8860b"];
    for row in &output {
        let p = n(row, "p");
        let i = n(row, "action_index") as usize;
        let index = primes.binary_search(&p).unwrap();
        let certs = row["six_cross_product_certificates"].as_array().unwrap();
        let max_steps = certs.iter().map(|c| n(c, "steps")).max().unwrap();
        let max_terms = certs
            .iter()
            .map(|c| c["cross_product"].as_array().unwrap().len())
            .max()
            .unwrap();
        csv.push_str(&format!("kummer-n16-generic-ideal/v1,{p},{},{i},{},u_root_nonzero,{},{},3,6,{max_terms},{max_steps},null,null,null\n",64-p.leading_zeros(),n(row,"root"),n(row,"denominator_power"),n(row,"w0_factor")));
        let x = 80.0 + 860.0 * (index as f64) / 20.0 + (i as f64 - 2.0) * 3.5;
        let y = 400.0 - 330.0 * (max_steps as f64) / 150.0;
        svg.push_str(&format!("<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"4\" fill=\"{}\"><title>p={p}, root={}, max steps={max_steps}</title></circle>",colors[i],n(row,"root")));
    }
    for (i, p) in primes.iter().enumerate() {
        let x = 80.0 + 860.0 * (i as f64) / 20.0;
        svg.push_str(&format!("<text x=\"{x:.1}\" y=\"416\" transform=\"rotate(55 {x:.1} 416)\" font-family=\"sans-serif\" font-size=\"10\">{p}</text>"));
    }
    svg.push_str("<text x=\"470\" y=\"465\" font-family=\"sans-serif\" font-size=\"12\">recorded field prime p</text><text x=\"10\" y=\"215\" transform=\"rotate(-90 10 215)\" font-family=\"sans-serif\" font-size=\"12\">maximum ideal-reduction steps among six cross-products</text></svg>\n");
    fs::write(format!("{base}/generic-ideal.csv"), csv).unwrap();
    fs::write("research/kummer_differential_collector_20261022/n16-action-certificate/generic-ideal-graph.svg",svg).unwrap();
    fs::write(
        format!("{base}/generic-ideal.json"),
        format!(
            "{}\n",
            serde_json::to_string(&json!({"schema":"kummer-n16-generic-ideal/v1","rows":output}))
                .unwrap()
        ),
    )
    .unwrap();
}
