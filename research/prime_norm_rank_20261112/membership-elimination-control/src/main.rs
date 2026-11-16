//! Fixed symbolic projection of one public known-answer input; no solving or search.
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs::OpenOptions, io::Write};

const P: u64 = 14_680_063;
const TERMS: usize = 500_000;
const PAIR_PRODUCTS: u64 = 100_000_000;
const ORIGINAL: &str = include_str!("../../membership-elimination-evidence/fixtures/rr_s2.ms");
const PLANT: &str =
    include_str!("../../membership-elimination-evidence/fixtures/rr_s2.ms.plant.json");
const EMIT: &str = include_str!("../../membership-elimination-evidence/fixtures/rr_s2.emit.json");
const CORE_NAMES: [&str; 4] = ["a1", "a2", "c0", "c1"];
type Exponent = [u16; 4];

fn add(a: u64, b: u64) -> u64 {
    (a + b) % P
}
fn sub(a: u64, b: u64) -> u64 {
    (a + P - b) % P
}
fn mul(a: u64, b: u64) -> u64 {
    a * b % P
}
fn power(mut a: u64, mut e: u16) -> u64 {
    let mut out = 1;
    while e != 0 {
        if e & 1 != 0 {
            out = mul(out, a);
        }
        a = mul(a, a);
        e >>= 1;
    }
    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Poly(BTreeMap<Exponent, u64>);
#[derive(Default)]
struct Guard {
    products: u64,
    peak_terms: usize,
}
impl Guard {
    fn observe(&mut self, p: &Poly) {
        self.peak_terms = self.peak_terms.max(p.0.len());
        assert!(p.0.len() <= TERMS, "intermediate term cap");
    }
}
impl Poly {
    fn zero() -> Self {
        Self(BTreeMap::new())
    }
    fn constant(c: u64) -> Self {
        let mut out = Self::zero();
        if !c.is_multiple_of(P) {
            out.0.insert([0; 4], c % P);
        }
        out
    }
    fn variable(i: usize) -> Self {
        let mut e = [0; 4];
        e[i] = 1;
        Self(BTreeMap::from([(e, 1)]))
    }
    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }
    fn plus(&self, other: &Self) -> Self {
        let mut out = self.clone();
        for (e, c) in &other.0 {
            let v = add(*out.0.get(e).unwrap_or(&0), *c);
            if v == 0 {
                out.0.remove(e);
            } else {
                out.0.insert(*e, v);
            }
        }
        assert!(out.0.len() <= TERMS, "addition term cap");
        out
    }
    fn scale(&self, c: u64) -> Self {
        Self(
            self.0
                .iter()
                .filter_map(|(e, x)| {
                    let v = mul(*x, c % P);
                    (v != 0).then_some((*e, v))
                })
                .collect(),
        )
    }
    fn minus(&self, other: &Self) -> Self {
        self.plus(&other.scale(P - 1))
    }
    fn times(&self, other: &Self, g: &mut Guard) -> Self {
        let pairs = (self.0.len() as u64) * (other.0.len() as u64);
        g.products = g.products.checked_add(pairs).unwrap();
        assert!(g.products <= PAIR_PRODUCTS, "sparse-pair-product cap");
        let mut out = Self::zero();
        for (e, a) in &self.0 {
            for (f, b) in &other.0 {
                let mut ef = [0; 4];
                for j in 0..4 {
                    ef[j] = e[j].checked_add(f[j]).unwrap();
                }
                let v = add(*out.0.get(&ef).unwrap_or(&0), mul(*a, *b));
                if v == 0 {
                    out.0.remove(&ef);
                } else {
                    out.0.insert(ef, v);
                }
            }
        }
        g.observe(&out);
        out
    }
    fn pow(&self, mut n: u16, g: &mut Guard) -> Self {
        let mut base = self.clone();
        let mut out = Self::constant(1);
        while n != 0 {
            if n & 1 != 0 {
                out = out.times(&base, g);
            }
            n >>= 1;
            if n != 0 {
                base = base.times(&base, g);
            }
        }
        out
    }
    fn eval(&self, x: &[u64; 4]) -> u64 {
        self.0.iter().fold(0, |out, (e, c)| {
            let term = (0..4).fold(*c, |a, j| mul(a, power(x[j], e[j])));
            add(out, term)
        })
    }
    fn degree(&self) -> u32 {
        self.0
            .keys()
            .map(|e| e.iter().map(|x| u32::from(*x)).sum())
            .max()
            .unwrap_or(0)
    }
    fn expression(&self) -> String {
        if self.is_zero() {
            return "0".into();
        }
        self.0
            .iter()
            .map(|(e, c)| {
                let mut s = c.to_string();
                for j in 0..4 {
                    if e[j] != 0 {
                        s.push('*');
                        s.push_str(CORE_NAMES[j]);
                        if e[j] != 1 {
                            s.push('^');
                            s.push_str(&e[j].to_string());
                        }
                    }
                }
                s
            })
            .collect::<Vec<_>>()
            .join("+")
    }
}
type XPoly = Vec<Poly>;
fn trim(mut x: XPoly) -> XPoly {
    while x.last().is_some_and(Poly::is_zero) {
        x.pop();
    }
    x
}
fn coefficient(x: &XPoly, j: usize) -> Poly {
    x.get(j).cloned().unwrap_or_else(Poly::zero)
}
fn xsub(a: &XPoly, b: &XPoly) -> XPoly {
    trim(
        (0..a.len().max(b.len()))
            .map(|j| coefficient(a, j).minus(&coefficient(b, j)))
            .collect(),
    )
}
fn xmul(a: &XPoly, b: &XPoly, g: &mut Guard) -> XPoly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = vec![Poly::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] = out[i + j].plus(&x.times(y, g));
            g.observe(&out[i + j]);
        }
    }
    trim(out)
}
fn divrem(p: &XPoly, u: &XPoly, g: &mut Guard) -> (XPoly, XPoly) {
    assert!(!u.is_empty());
    assert_eq!(u.last().unwrap(), &Poly::constant(1));
    let mut r = trim(p.clone());
    let mut q = vec![Poly::zero(); r.len().saturating_sub(u.len()) + 1];
    while r.len() >= u.len() {
        let shift = r.len() - u.len();
        let c = r.last().unwrap().clone();
        q[shift] = c.clone();
        for (j, uj) in u.iter().enumerate() {
            r[shift + j] = r[shift + j].minus(&c.times(uj, g));
            g.observe(&r[shift + j]);
        }
        r = trim(r);
    }
    (trim(q), r)
}
fn subtract_constant(p: &XPoly, c: u64) -> XPoly {
    xsub(p, &vec![Poly::constant(c)])
}

struct Input {
    names: Vec<String>,
    equations: Vec<String>,
}
fn input() -> Input {
    let mut lines = ORIGINAL.lines();
    let names = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert_eq!(lines.next().unwrap().parse::<u64>().unwrap(), P);
    let equations = lines
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().trim_end_matches(',').to_owned())
        .collect::<Vec<_>>();
    assert_eq!(names.len(), 23);
    assert_eq!(equations.len(), 25);
    Input { names, equations }
}
fn substituted(expr: &str, lift: &BTreeMap<String, Poly>, g: &mut Guard) -> Poly {
    let mut sum = Poly::zero();
    for term in expr.split('+') {
        let mut value = Poly::constant(1);
        for factor in term.split('*') {
            let p = if let Ok(c) = factor.parse::<u64>() {
                Poly::constant(c)
            } else {
                let mut parts = factor.split('^');
                let name = parts.next().unwrap();
                let exponent = parts.next().map(|x| x.parse::<u16>().unwrap()).unwrap_or(1);
                assert!(parts.next().is_none());
                lift.get(name).expect("original variable").pow(exponent, g)
            };
            value = value.times(&p, g);
        }
        sum = sum.plus(&value);
        g.observe(&sum);
    }
    sum
}
fn original_value(expr: &str, values: &BTreeMap<String, u64>) -> u64 {
    expr.split('+').fold(0, |total, term| {
        let term = term.split('*').fold(1, |product, factor| {
            let v = if let Ok(c) = factor.parse::<u64>() {
                c % P
            } else {
                let mut parts = factor.split('^');
                let name = parts.next().unwrap();
                let e = parts.next().map(|x| x.parse::<u16>().unwrap()).unwrap_or(1);
                assert!(parts.next().is_none());
                power(*values.get(name).unwrap(), e)
            };
            mul(product, v)
        });
        add(total, term)
    })
}
struct Construction {
    lift: BTreeMap<String, Poly>,
    reduced: XPoly,
    guard: Guard,
}
fn construct() -> Construction {
    let meta: Value = serde_json::from_str(EMIT).unwrap();
    let i = &meta["instance"];
    assert_eq!(i["p"].as_u64().unwrap(), P);
    assert_eq!(i["m"], 5);
    assert_eq!(i["t"], 4);
    assert_eq!(i["scale_g"], 1);
    assert_eq!(i["kind"], "dickson");
    let q = i["R"][0].as_u64().unwrap();
    let z = i["R"][1].as_u64().unwrap();
    let ca = i["a"].as_u64().unwrap();
    let cb = i["b"].as_u64().unwrap();
    let terminal = i["terminal_c"].as_u64().unwrap();
    assert_eq!(mul(z, z), add(add(power(q, 3), mul(ca, q)), cb));
    let core = (0..4).map(Poly::variable).collect::<Vec<_>>();
    let a0 = core[2]
        .plus(&core[3].scale(q))
        .scale(z)
        .minus(&Poly::constant(power(q, 3)))
        .minus(&core[0].scale(q))
        .minus(&core[1].scale(power(q, 2)));
    let a = vec![
        a0.clone(),
        core[0].clone(),
        core[1].clone(),
        Poly::constant(1),
    ];
    let b = vec![core[2].clone(), core[3].clone()];
    let f = vec![
        Poly::constant(cb),
        Poly::constant(ca),
        Poly::zero(),
        Poly::constant(1),
    ];
    let mut guard = Guard::default();
    let aa = xmul(&a, &a, &mut guard);
    let bb = xmul(&b, &b, &mut guard);
    let fbb = xmul(&f, &bb, &mut guard);
    let n = xsub(&aa, &fbb);
    assert_eq!(n.len(), 7);
    assert_eq!(n.last(), Some(&Poly::constant(1)));
    let (u, tail) = divrem(
        &n,
        &vec![Poly::constant(sub(0, q)), Poly::constant(1)],
        &mut guard,
    );
    assert!(tail.is_empty());
    assert_eq!(u.len(), 6);
    let mut master = vec![Poly::zero(), Poly::constant(1)];
    for _ in 0..3 {
        master = subtract_constant(&xmul(&master, &master, &mut guard), 2);
    }
    let (q3, r3) = divrem(&master, &u, &mut guard);
    assert_eq!(q3.len(), 4);
    assert_eq!(r3.len(), 5);
    let r3sq = xmul(&r3, &r3, &mut guard);
    let (q4, reduced) = divrem(&subtract_constant(&r3sq, add(2, terminal)), &u, &mut guard);
    assert_eq!(q4.len(), 4);
    assert_eq!(reduced.len(), 5);
    let master4 = xmul(&master, &master, &mut guard);
    let (_, direct) = divrem(
        &subtract_constant(&master4, add(2, terminal)),
        &u,
        &mut guard,
    );
    assert_eq!(
        reduced, direct,
        "direct master and modular tower differ symbolically"
    );
    let mut lift = BTreeMap::new();
    lift.insert("a0".into(), a0);
    for (name, value) in CORE_NAMES.iter().zip(core) {
        lift.insert((*name).into(), value);
    }
    for (prefix, coeffs, count) in [
        ("u", &u, 5),
        ("q3_", &q3, 4),
        ("r3_", &r3, 5),
        ("q4_", &q4, 4),
    ] {
        for j in 0..count {
            lift.insert(format!("{prefix}{j}"), coefficient(coeffs, j));
        }
    }
    assert_eq!(lift.len(), 23);
    for p in lift.values() {
        guard.observe(p);
    }
    Construction {
        lift,
        reduced,
        guard,
    }
}
fn control() -> (Value, String) {
    let input = input();
    let mut c = construct();
    for (j, eq) in input.equations.iter().enumerate() {
        let actual = substituted(eq, &c.lift, &mut c.guard);
        let expected = if j < 20 {
            Poly::zero()
        } else {
            coefficient(&c.reduced, 24 - j)
        };
        assert_eq!(
            actual, expected,
            "original equation {j} symbolic residual mismatch"
        );
    }
    let plant: Value = serde_json::from_str(PLANT).unwrap();
    let mut original = [0; 4];
    for (j, name) in CORE_NAMES.iter().enumerate() {
        original[j] = plant["planted"][name].as_u64().unwrap();
    }
    let mut cases = vec![];
    for case in 0..5 {
        let mut core = original;
        if case != 0 {
            core[case - 1] = add(core[case - 1], 1);
        }
        let values = c
            .lift
            .iter()
            .map(|(name, p)| (name.clone(), p.eval(&core)))
            .collect::<BTreeMap<_, _>>();
        let old = input
            .equations
            .iter()
            .map(|eq| original_value(eq, &values))
            .collect::<Vec<_>>();
        let reduced = (0..5)
            .rev()
            .map(|j| coefficient(&c.reduced, j).eval(&core))
            .collect::<Vec<_>>();
        assert!(old[..20].iter().all(|x| *x == 0));
        assert_eq!(old[20..], reduced);
        if case == 0 {
            for name in &input.names {
                assert_eq!(
                    values[name],
                    plant["planted"][name].as_u64().unwrap(),
                    "plant lift {name}"
                );
            }
            assert!(old.iter().all(|x| *x == 0));
        }
        cases.push(json!({"case":case,"core":core,"original_residuals":old,
            "reduced_residuals_descending":reduced,"all_original_zero":old.iter().all(|x|*x==0)}));
    }
    let mut bad = c
        .lift
        .iter()
        .map(|(name, p)| (name.clone(), p.eval(&original)))
        .collect::<BTreeMap<_, _>>();
    *bad.get_mut("q3_3").unwrap() = add(bad["q3_3"], 1);
    let bad_residuals = input
        .equations
        .iter()
        .map(|eq| original_value(eq, &bad))
        .collect::<Vec<_>>();
    assert!(bad_residuals.iter().any(|x| *x != 0));
    let stats = (0..5)
        .rev()
        .map(|j| {
            let p = coefficient(&c.reduced, j);
            assert!(p.degree() <= 24);
            json!({"X_coefficient":j,"total_degree":p.degree(),"terms":p.0.len()})
        })
        .collect::<Vec<_>>();
    let expressions = (0..5)
        .rev()
        .map(|j| coefficient(&c.reduced, j).expression())
        .collect::<Vec<_>>();
    let emitted = format!("a1,a2,c0,c1\n{P}\n{}\n", expressions.join(",\n"));
    let artifact = json!({"schema":"norm.membership-elimination-control/v1","p":P,
        "original_variables":23,"original_equations":25,"retained_variables":CORE_NAMES,
        "eliminated_variables":19,"reduced_equations":5,"symbolic_zero_original_rows":20,
        "symbolic_remaining_rows_equal_reduced":true,"direct_master_equals_reduced_tower":true,
        "reduced_polynomial_statistics":stats,"core_cases":cases,
        "corrupted_auxiliary":"q3_3 plus1 at supplied plant","corrupted_original_residuals":bad_residuals,
        "guard":{"sparse_pair_products":c.guard.products,"limit":PAIR_PRODUCTS,
            "peak_observed_terms":c.guard.peak_terms,"terms_limit":TERMS},
        "physical_artifact_invocations":1,"search_executed":false,"groebner_solver_executed":false,
        "timing_measured":false,"scope":"one exact frozen symbolic input transformation and five fixed assignment controls; no solver-cost or ECDLP bound",
        "standing":"unsigned producer construction/evidence"});
    (artifact, emitted)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err("usage: norm-membership-elimination-control NEW_OUTPUT_DIRECTORY".into());
    }
    let root = std::path::Path::new(&args[1]);
    std::fs::create_dir(root)?;
    let (artifact, emitted) = control();
    for (name, bytes) in [
        ("native-control.json", serde_json::to_vec_pretty(&artifact)?),
        ("reduced.ms", emitted.into_bytes()),
    ] {
        let mut out = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join(name))?;
        out.write_all(&bytes)?;
    }
    println!("23 variables ->4; all25 original symbolic residuals checked; no solver invoked");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_original_symbolic_projection_and_fixed_assignments() {
        control();
    }
    #[test]
    fn polynomial_arithmetic_rejects_coefficient_corruption() {
        let mut g = Guard::default();
        let x = Poly::variable(0);
        let y = Poly::variable(1);
        let square = x.plus(&y).pow(2, &mut g);
        let xx = x.pow(2, &mut g);
        let yy = y.pow(2, &mut g);
        let xy = x.times(&y, &mut g);
        assert_eq!(square, xx.plus(&xy.scale(2)).plus(&yy));
        assert_ne!(square, xx.plus(&xy).plus(&yy));
    }
    #[test]
    fn original_expression_parser_retains_powers_and_coefficients() {
        let mut g = Guard::default();
        let env = BTreeMap::from([("a1".into(), Poly::variable(0))]);
        let symbolic = substituted("2*a1^2+3", &env, &mut g);
        for x in [0, 1, 17, P - 1] {
            let values = BTreeMap::from([("a1".into(), x)]);
            assert_eq!(
                symbolic.eval(&[x, 0, 0, 0]),
                original_value("2*a1^2+3", &values)
            );
        }
    }
}
