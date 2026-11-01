//! Native matched-input panel for the unchanged msolve-neogb F4 backend.
//! Execution needs an explicit admission record; tests use artificial ideals only.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
type Mono = Vec<u32>;
type Poly = BTreeMap<Mono, u64>;
type Roots = BTreeSet<Vec<u64>>;
type Relations = BTreeSet<([u64; 2], [u64; 2])>;
type BaseBinding = (Option<Vec<u64>>, BTreeSet<[u64; 2]>);
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Term {
    exponents: Vec<u32>,
    coefficient: u64,
}
#[derive(Clone, Debug, Deserialize)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
}
#[derive(Clone, Debug, Deserialize)]
struct System {
    id: String,
    p: u64,
    variables: Vec<String>,
    polynomials: Vec<Vec<Term>>,
    curve: Curve,
    target: [u64; 2],
    base_x: Vec<u64>,
    base_points: Vec<[u64; 2]>,
    arm: String,
    #[serde(default)]
    signs: Option<[i8; 2]>,
    #[serde(default)]
    expected_roots: Option<Vec<Vec<u64>>>,
    #[serde(default)]
    h_low_to_high: Option<Vec<u64>>,
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn write(path: &Path, v: &Value) -> Result<(), String> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(&serde_json::to_vec_pretty(v).unwrap())
        .map_err(|e| e.to_string())?;
    f.write_all(b"\n").map_err(|e| e.to_string())
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    a * b % p
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 != 0 {
            r = mul(r, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    r
}
fn prime(p: u64) -> bool {
    p >= 2
        && (2..)
            .take_while(|d| d * d <= p)
            .all(|d| !p.is_multiple_of(d))
}
fn degree(m: &[u32]) -> u32 {
    m.iter().sum()
}
fn cmp(a: &[u32], b: &[u32]) -> Ordering {
    degree(a).cmp(&degree(b)).then_with(|| {
        a.iter()
            .rev()
            .zip(b.iter().rev())
            .find_map(|(x, y)| (x != y).then(|| y.cmp(x)))
            .unwrap_or(Ordering::Equal)
    })
}
fn insert(poly: &mut Poly, m: Mono, c: u64, p: u64) {
    let v = (poly.get(&m).copied().unwrap_or(0) + c) % p;
    if v == 0 {
        poly.remove(&m);
    } else {
        poly.insert(m, v);
    }
}
fn parse_poly(ts: &[Term], n: usize, p: u64) -> Result<Poly, String> {
    let mut poly = Poly::new();
    for t in ts {
        if t.exponents.len() != n
            || t.exponents.iter().any(|e| *e > 128)
            || t.coefficient == 0
            || t.coefficient >= p
            || poly.contains_key(&t.exponents)
        {
            return Err("noncanonical polynomial".into());
        }
        poly.insert(t.exponents.clone(), t.coefficient);
    }
    Ok(poly)
}
fn leading(poly: &Poly) -> Option<(&Mono, &u64)> {
    poly.iter().max_by(|(a, _), (b, _)| cmp(a, b))
}
fn divides(a: &[u32], b: &[u32]) -> bool {
    a.iter().zip(b).all(|(a, b)| a <= b)
}
fn shifted(poly: &Poly, m: &[u32], c: u64, p: u64) -> Poly {
    poly.iter()
        .map(|(k, v)| (k.iter().zip(m).map(|(a, b)| a + b).collect(), mul(*v, c, p)))
        .filter(|(_, v)| *v != 0)
        .collect()
}
fn subtract(a: &mut Poly, b: &Poly, p: u64) {
    for (m, c) in b {
        insert(a, m.clone(), p - *c, p);
    }
}
fn remainder(mut f: Poly, g: &[Poly], p: u64) -> Result<Poly, String> {
    let mut r = Poly::new();
    let mut steps = 0;
    while let Some((m, c)) = leading(&f).map(|(m, c)| (m.clone(), *c)) {
        steps += 1;
        if steps > 1_000_000 {
            return Err("normal form verification cap".into());
        }
        let reducer = g.iter().find_map(|q| {
            leading(q)
                .filter(|(k, _)| divides(k, &m))
                .map(|(k, b)| (q, k, b))
        });
        if let Some((q, k, b)) = reducer {
            let shift: Vec<_> = m.iter().zip(k).map(|(i, j)| i - j).collect();
            subtract(
                &mut f,
                &shifted(q, &shift, mul(c, pow(*b, p - 2, p), p), p),
                p,
            );
        } else {
            f.remove(&m);
            r.insert(m, c);
        }
    }
    Ok(r)
}
fn check_gb(input: &[Poly], basis: &[Poly], p: u64) -> Result<Value, String> {
    if basis.is_empty() {
        return Err("empty exported basis unsupported".into());
    }
    for f in input {
        if !remainder(f.clone(), basis, p)?.is_empty() {
            return Err("input generator has nonzero GB remainder".into());
        }
    }
    let mut pairs = 0;
    for i in 0..basis.len() {
        for j in 0..i {
            let Some((a, c)) = leading(&basis[i]) else {
                return Err("zero polynomial in basis".into());
            };
            let Some((b, d)) = leading(&basis[j]) else {
                return Err("zero polynomial in basis".into());
            };
            let l: Vec<_> = a.iter().zip(b).map(|(x, y)| (*x).max(*y)).collect();
            let sa: Vec<_> = l.iter().zip(a).map(|(x, y)| x - y).collect();
            let sb: Vec<_> = l.iter().zip(b).map(|(x, y)| x - y).collect();
            let mut sp = shifted(&basis[i], &sa, pow(*c, p - 2, p), p);
            subtract(&mut sp, &shifted(&basis[j], &sb, pow(*d, p - 2, p), p), p);
            if !remainder(sp, basis, p)?.is_empty() {
                return Err("Buchberger criterion failed".into());
            }
            pairs += 1;
        }
    }
    Ok(
        json!({"input_generator_remainders_zero":true,"buchberger_pairs_checked":pairs,"buchberger_criterion":true,"ideal_membership_of_exported_basis_in_input":"not_certified","scheme_multiplicity_equality":"not_certified"}),
    )
}
fn eval(f: &Poly, point: &[u64], p: u64) -> u64 {
    f.iter().fold(0, |out, (m, c)| {
        (out + m
            .iter()
            .zip(point)
            .fold(*c, |v, (e, x)| mul(v, pow(*x, *e as u64, p), p)))
            % p
    })
}
fn add_poly(a: &Poly, b: &Poly, p: u64) -> Poly {
    let mut out = a.clone();
    for (m, c) in b {
        insert(&mut out, m.clone(), *c, p);
    }
    out
}
fn scale(a: &Poly, c: u64, p: u64) -> Poly {
    a.iter()
        .map(|(m, v)| (m.clone(), mul(*v, c, p)))
        .filter(|(_, v)| *v != 0)
        .collect()
}
fn product_poly(a: &Poly, b: &Poly, p: u64) -> Poly {
    let mut out = Poly::new();
    for (m, c) in a {
        for (k, d) in b {
            insert(
                &mut out,
                m.iter().zip(k).map(|(i, j)| i + j).collect(),
                mul(*c, *d, p),
                p,
            );
        }
    }
    out
}
fn expected_model(s: &System) -> Vec<Poly> {
    let n = s.variables.len();
    let p = s.p;
    let constant = |c: u64| {
        if c.is_multiple_of(p) {
            Poly::new()
        } else {
            Poly::from([(vec![0; n], c % p)])
        }
    };
    let variable = |i: usize| {
        let mut m = vec![0; n];
        m[i] = 1;
        Poly::from([(m, 1)])
    };
    let (x, z) = (variable(0), variable(1));
    let sum = |a: &Poly, b: &Poly| add_poly(a, b, p);
    let neg = |a: &Poly| scale(a, p - 1, p);
    let times = |a: &Poly, b: &Poly| product_poly(a, b, p);
    let membership = |v: &Poly| {
        s.base_x.iter().fold(constant(1), |out, r| {
            times(&out, &sum(v, &constant((p - r) % p)))
        })
    };
    let r = s.target[0];
    let t = s.target[1];
    let a = s.curve.a;
    let b = s.curve.b;
    if n == 2 {
        let xz = times(&x, &z);
        let plus = sum(&x, &z);
        let difference = sum(&x, &neg(&z));
        let xza = sum(&xz, &constant((p - a) % p));
        let first = scale(&times(&difference, &difference), mul(r, r, p), p);
        let second = scale(
            &sum(&times(&plus, &sum(&xz, &constant(a))), &constant(2 * b % p)),
            (p - 2 * r % p) % p,
            p,
        );
        let third = times(&xza, &xza);
        let fourth = scale(&plus, (p - 4 * b % p) % p, p);
        vec![
            membership(&x),
            membership(&z),
            sum(&sum(&first, &second), &sum(&third, &fourth)),
        ]
    } else {
        let m = variable(2);
        let m2 = times(&m, &m);
        let g1 = sum(&sum(&m2, &neg(&x)), &sum(&neg(&z), &constant((p - r) % p)));
        let g2 = sum(
            &sum(&times(&x, &z), &scale(&m2, (p - r) % p, p)),
            &sum(
                &scale(&m, (p - 2 * t % p) % p, p),
                &constant((2 * p - mul(r, r, p) - a) % p),
            ),
        );
        let h = s.h_low_to_high.as_ref().unwrap();
        let h_at = |v: &Poly| {
            h.iter()
                .rev()
                .fold(Poly::new(), |out, c| sum(&times(&out, v), &constant(*c)))
        };
        let signs = s.signs.unwrap();
        let q = |v: &Poly, e: i8| {
            sum(
                &sum(
                    &scale(&h_at(v), if e == 1 { 1 } else { p - 1 }, p),
                    &neg(&times(&m, &sum(v, &constant((p - r) % p)))),
                ),
                &constant(t),
            )
        };
        let mut out = vec![g1, g2, q(&x, signs[0]), q(&z, signs[1])];
        if s.arm == "control-secant" {
            out.extend([membership(&x), membership(&z)]);
        }
        out
    }
}
fn specialized(f: &Poly, values: &[Option<u64>], p: u64) -> Poly {
    let mut out = Poly::new();
    for (m, c) in f {
        let mut k = m.clone();
        let mut v = *c;
        for (i, x) in values.iter().enumerate() {
            if let Some(x) = x {
                v = mul(v, pow(*x, k[i] as u64, p), p);
                k[i] = 0;
            }
        }
        insert(&mut out, k, v, p);
    }
    out
}
fn roots(basis: &[Poly], n: usize, p: u64) -> Result<(BTreeSet<Vec<u64>>, u64), String> {
    fn visit(
        g: &[Poly],
        v: &mut [Option<u64>],
        p: u64,
        out: &mut BTreeSet<Vec<u64>>,
        tries: &mut u64,
    ) -> Result<(), String> {
        let gs: Vec<_> = g
            .iter()
            .map(|f| specialized(f, v, p))
            .filter(|f| !f.is_empty())
            .collect();
        if gs
            .iter()
            .any(|f| f.len() == 1 && f.keys().next().unwrap().iter().all(|e| *e == 0))
        {
            return Ok(());
        }
        if v.iter().all(Option::is_some) {
            out.insert(v.iter().map(|x| x.unwrap()).collect());
            return Ok(());
        }
        let univariate = gs
            .iter()
            .filter_map(|f| {
                let used: BTreeSet<_> = f
                    .keys()
                    .flat_map(|m| {
                        m.iter()
                            .enumerate()
                            .filter_map(|(i, e)| (*e != 0).then_some(i))
                    })
                    .collect();
                (used.len() == 1).then(|| (*used.iter().next().unwrap(), f))
            })
            .min_by_key(|(_, f)| f.keys().map(|m| degree(m)).max().unwrap_or(0));
        let index = univariate
            .map(|(i, _)| i)
            .unwrap_or_else(|| v.iter().position(Option::is_none).unwrap());
        for x in 0..p {
            *tries += 1;
            if *tries > 2_000_000 {
                return Err("root extraction assignment cap".into());
            }
            v[index] = Some(x);
            if let Some((_, f)) = univariate {
                let z: Vec<_> = v.iter().map(|x| x.unwrap_or(0)).collect();
                if eval(f, &z, p) != 0 {
                    continue;
                }
            }
            visit(&gs, v, p, out, tries)?;
        }
        v[index] = None;
        Ok(())
    }
    let mut out = BTreeSet::new();
    let mut tries = 0;
    visit(basis, &mut vec![None; n], p, &mut out, &mut tries)?;
    Ok((out, tries))
}
fn point_add(c: &Curve, a: [u64; 2], b: [u64; 2]) -> Option<[u64; 2]> {
    let p = c.p;
    let [x, y] = a;
    let [z, w] = b;
    if x == z && (y + w) % p == 0 {
        return None;
    }
    let m = if x == z {
        mul(
            (mul(3, mul(x, x, p), p) + c.a) % p,
            pow(2 * y % p, p - 2, p),
            p,
        )
    } else {
        mul((w + p - y) % p, pow((z + p - x) % p, p - 2, p), p)
    };
    let r = (mul(m, m, p) + 2 * p - x - z) % p;
    Some([r, (mul(m, (x + p - r) % p, p) + p - y) % p])
}
fn references(s: &System) -> (Roots, Relations) {
    let mut assignments = BTreeSet::new();
    let mut relations = BTreeSet::new();
    for a in &s.base_points {
        for b in &s.base_points {
            if let Some(signs) = s.signs {
                let h = s
                    .h_low_to_high
                    .as_ref()
                    .expect("validated intended orientation");
                let hvalue = |x| {
                    h.iter()
                        .rev()
                        .fold(0, |out, c| (mul(out, x, s.p) + c) % s.p)
                };
                let signed = |v: u64, e: i8| if e == 1 { v } else { (s.p - v) % s.p };
                if a[1] != signed(hvalue(a[0]), signs[0]) || b[1] != signed(hvalue(b[0]), signs[1])
                {
                    continue;
                }
            }
            if point_add(&s.curve, *a, *b) != Some(s.target) {
                continue;
            }
            let mut v = vec![a[0], b[0]];
            if s.variables.len() == 3 {
                let p = s.p;
                let m = if a[0] == b[0] {
                    mul(
                        (mul(3, mul(a[0], a[0], p), p) + s.curve.a) % p,
                        pow(2 * a[1] % p, p - 2, p),
                        p,
                    )
                } else {
                    mul(
                        (b[1] + p - a[1]) % p,
                        pow((b[0] + p - a[0]) % p, p - 2, p),
                        p,
                    )
                };
                v.push(m);
            }
            assignments.insert(v);
            relations.insert((*a, *b));
        }
    }
    (assignments, relations)
}
fn validate(s: &System) -> Result<Vec<Poly>, String> {
    let n = s.variables.len();
    if !(5..=65521).contains(&s.p)
        || !prime(s.p)
        || !(2..=3).contains(&n)
        || s.polynomials.is_empty()
        || s.polynomials.len() > 256
        || s.curve.p != s.p
        || s.curve.a >= s.p
        || s.curve.b >= s.p
        || s.target.iter().any(|v| *v >= s.p)
    {
        return Err("invalid system scope".into());
    }
    if s.variables
        != if n == 2 {
            vec!["x", "z"]
        } else {
            vec!["x", "z", "m"]
        }
        || !matches!(
            (s.arm.as_str(), n),
            ("candidate-dense" | "control-dense", 2) | ("candidate-secant" | "control-secant", 3)
        )
    {
        return Err("invalid arm/variable-order binding".into());
    }
    if n == 3
        && (s.signs.is_none()
            || s.h_low_to_high
                .as_ref()
                .is_none_or(|h| h.is_empty() || h.len() > 32 || h.iter().any(|v| *v >= s.p)))
    {
        return Err("secant sign branch missing".into());
    }
    if s.signs
        .is_some_and(|v| v.iter().any(|s| ![-1, 1].contains(s)))
        || n == 2 && s.signs.is_some()
    {
        return Err("invalid sign/presentation metadata".into());
    }
    if (mul(4, mul(mul(s.curve.a, s.curve.a, s.p), s.curve.a, s.p), s.p)
        + mul(27, mul(s.curve.b, s.curve.b, s.p), s.p))
    .is_multiple_of(s.p)
    {
        return Err("singular curve".into());
    }
    if s.base_points.len() > 64
        || s.base_x.len() > 32
        || s.base_x.is_empty()
        || s.base_x.windows(2).any(|v| v[0] >= v[1])
    {
        return Err("base exceeds frozen limits".into());
    }
    let rhs = |x| (mul(mul(x, x, s.p), x, s.p) + mul(s.curve.a, x, s.p) + s.curve.b) % s.p;
    if mul(s.target[1], s.target[1], s.p) != rhs(s.target[0]) {
        return Err("target off curve".into());
    }
    let mut points = BTreeSet::new();
    for a in &s.base_points {
        if a.iter().any(|v| *v >= s.p)
            || mul(a[1], a[1], s.p) != rhs(a[0])
            || !s.base_x.contains(&a[0])
            || !points.insert(*a)
        {
            return Err("invalid base inventory".into());
        }
    }
    for x in &s.base_x {
        let lifts: Vec<_> = s
            .base_points
            .iter()
            .filter(|point| point[0] == *x)
            .collect();
        if lifts.is_empty()
            || lifts
                .iter()
                .any(|point| !points.contains(&[point[0], (s.p - point[1]) % s.p]))
        {
            return Err("base inventory omits a sign".into());
        }
        if let Some(h) = &s.h_low_to_high {
            let y = h
                .iter()
                .rev()
                .fold(0, |out, c| (mul(out, *x, s.p) + c) % s.p);
            if !points.contains(&[*x, y]) {
                return Err("intended graph orientation is not in base".into());
            }
        }
    }
    if s.arm == "candidate-secant" {
        let h = s.h_low_to_high.as_ref().unwrap();
        if h.len() < 3 || h.last() == Some(&0) || s.base_x.len() != 2 * (h.len() - 1) {
            return Err("candidate does not bind a complete degree-d graph".into());
        }
        let hpoly: Poly = h
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != 0)
            .map(|(i, c)| (vec![i as u32], *c))
            .collect();
        let f = Poly::from([(vec![3], 1), (vec![1], s.curve.a), (vec![0], s.curve.b)])
            .into_iter()
            .filter(|(_, c)| *c != 0)
            .collect();
        let mut lhs = product_poly(&hpoly, &hpoly, s.p);
        subtract(&mut lhs, &f, s.p);
        let mut rhs = Poly::from([(vec![0], mul(*h.last().unwrap(), *h.last().unwrap(), s.p))]);
        for x in &s.base_x {
            rhs = product_poly(
                &rhs,
                &Poly::from([(vec![1], 1), (vec![0], (s.p - x) % s.p)])
                    .into_iter()
                    .filter(|(_, c)| *c != 0)
                    .collect(),
                s.p,
            );
        }
        if lhs != rhs {
            return Err("candidate split-graph factorization fails".into());
        }
    }
    let input = s
        .polynomials
        .iter()
        .map(|f| parse_poly(f, n, s.p))
        .collect::<Result<Vec<_>, _>>()?;
    let mut expected = expected_model(s);
    let mut canonical = input.clone();
    expected.sort();
    canonical.sort();
    if canonical != expected {
        return Err("input does not equal the independently reconstructed complete model".into());
    }
    Ok(input)
}
fn export_tokens(s: &System) -> String {
    let mut out = format!("{} {} {}\n", s.p, s.variables.len(), s.polynomials.len());
    for f in &s.polynomials {
        out.push_str(&format!("{}\n", f.len()));
        for t in f {
            out.push_str(&t.coefficient.to_string());
            for e in &t.exponents {
                out.push_str(&format!(" {e}"));
            }
            out.push('\n');
        }
    }
    out
}
fn export_msolve(s: &System) -> String {
    let mut out = format!("{}\n{}\n", s.variables.join(","), s.p);
    let polys: Vec<_> = s
        .polynomials
        .iter()
        .map(|f| {
            if f.is_empty() {
                return "0".into();
            }
            f.iter()
                .map(|t| {
                    let mut m = t.coefficient.to_string();
                    for (v, e) in s.variables.iter().zip(&t.exponents) {
                        if *e != 0 {
                            m.push('*');
                            m.push_str(v);
                            if *e != 1 {
                                m.push_str(&format!("^{e}"));
                            }
                        }
                    }
                    m
                })
                .collect::<Vec<_>>()
                .join("+")
        })
        .collect();
    out.push_str(&polys.join(",\n"));
    out.push('\n');
    out
}
fn selected_degrees(trace: &str) -> Vec<u32> {
    trace
        .lines()
        .filter_map(|line| {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.len() >= 7
                && w.get(4) == Some(&"x")
                && w[0].parse::<u32>().is_ok()
                && w[1].parse::<u64>().is_ok()
                && w[2].parse::<u64>().is_ok()
            {
                w[0].parse().ok()
            } else {
                None
            }
        })
        .collect()
}
fn trace_matrix_shapes(trace: &str) -> Vec<[u64; 2]> {
    trace
        .lines()
        .filter_map(|line| {
            let w: Vec<_> = line.split_whitespace().collect();
            w.windows(3).find_map(|v| {
                (v[1] == "x")
                    .then(|| Some([v[0].parse().ok()?, v[2].parse().ok()?]))
                    .flatten()
            })
        })
        .collect()
}
fn trace_statistics(trace: &str) -> Value {
    let scalar = |prefix: &str| {
        trace.lines().find_map(|l| {
            l.trim()
                .strip_prefix(prefix)
                .and_then(|t| t.split_whitespace().next()?.parse::<u64>().ok())
        })
    };
    let area = trace
        .lines()
        .find(|l| l.trim().starts_with("max. matrix data"))
        .and_then(|l| trace_matrix_shapes(l).into_iter().next());
    json!({"pairs_reduced":scalar("#pairs reduced"),"rows_reduced":scalar("#rows reduced"),"zero_reductions":scalar("#zero reductions"),"largest_area_matrix_rows":area.map(|s|s[0]),"largest_area_matrix_columns":area.map(|s|s[1]),"counter_source":"first_core_computational_data_block_in_raw_stderr"})
}
fn admission(path: &Path) -> Result<Value, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    let a: Value = serde_json::from_slice(&data).map_err(|e| e.to_string())?;
    if a["backend"] != "local"
        || a["checkpoint_override"] != true
        || a["backend_override"] != true
        || a["user_authorization_received"] != true
        || a["max_seconds"].as_u64().unwrap_or(0) > 1800
        || a["max_seconds"].as_u64().unwrap_or(0) == 0
        || a["address_space_bytes"] != 2147483648u64
    {
        return Err("explicit matching local admission required".into());
    }
    let source = a["source_revision"]
        .as_str()
        .ok_or("source_revision required")?;
    let actual_source = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| e.to_string())?;
    if !actual_source.status.success()
        || String::from_utf8_lossy(&actual_source.stdout).trim() != source
    {
        return Err("admission source_revision mismatch".into());
    }
    let protocol_path = a["protocol_path"]
        .as_str()
        .ok_or("protocol_path required")?;
    if hash(&fs::read(protocol_path).map_err(|e| e.to_string())?)
        != a["protocol_sha256"]
            .as_str()
            .ok_or("protocol_sha256 required")?
    {
        return Err("admission protocol hash mismatch".into());
    }
    let limits = fs::read_to_string("/proc/self/limits").map_err(|e| e.to_string())?;
    let actual = limits
        .lines()
        .find(|l| l.starts_with("Max address space"))
        .and_then(|l| l.split_whitespace().nth(3))
        .and_then(|s| s.parse::<u64>().ok());
    if actual.is_none_or(|n| n > 2147483648) {
        return Err("run under ulimit -v 2097152 (2 GiB)".into());
    }
    Ok(a)
}
fn panel_coverage(systems: &[Value]) -> Result<(), String> {
    let mut groups: BTreeMap<String, (usize, BTreeSet<[i8; 2]>)> = BTreeMap::new();
    let mut populations: BTreeMap<String, (Curve, BTreeSet<[u64; 2]>)> = BTreeMap::new();
    let mut roles: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut bindings: BTreeMap<String, BaseBinding> = BTreeMap::new();
    for value in systems {
        let s: System = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
        validate(&s)?;
        if s.p > 257 {
            return Err("this complete toy panel is frozen to p<=257".into());
        }
        let role = if s.arm.starts_with("candidate-") {
            "candidate"
        } else {
            "control"
        };
        roles
            .entry(
                serde_json::to_string(
                    &json!({"p":s.p,"a":s.curve.a,"b":s.curve.b,"target":s.target}),
                )
                .unwrap(),
            )
            .or_default()
            .insert(role.to_string());
        let base = json!({"p":s.p,"a":s.curve.a,"b":s.curve.b,"base_x":s.base_x,"role":role,"control_name":value.get("control_name")});
        let base_key = serde_json::to_string(&base).unwrap();
        let inventory: BTreeSet<_> = s.base_points.iter().copied().collect();
        let binding = bindings
            .entry(base_key.clone())
            .or_insert_with(|| (s.h_low_to_high.clone(), inventory.clone()));
        if binding.1 != inventory {
            return Err("signed inventory changes within a base".into());
        }
        if let Some(h) = &s.h_low_to_high {
            if binding.0.as_ref().is_some_and(|g| g != h) {
                return Err(
                    "intended graph orientation changes across target/sign branches".into(),
                );
            }
            binding.0 = Some(h.clone());
        }
        let key = serde_json::to_string(&json!({"base":base,"target":s.target})).unwrap();
        let group = groups.entry(key).or_default();
        if s.variables.len() == 2 {
            group.0 += 1;
        } else if !group.1.insert(s.signs.ok_or("missing sign branch")?) {
            return Err("duplicate sign branch".into());
        }
        populations
            .entry(base_key)
            .or_insert_with(|| (s.curve.clone(), BTreeSet::new()))
            .1
            .insert(s.target);
    }
    for (dense, signs) in groups.values() {
        if *dense != 1 || *signs != BTreeSet::from([[1, 1], [1, -1], [-1, 1], [-1, -1]]) {
            return Err(
                "panel must include one dense system and all four sign branches per base/target"
                    .into(),
            );
        }
    }
    if roles
        .values()
        .any(|s| *s != BTreeSet::from(["candidate".to_string(), "control".to_string()]))
    {
        return Err("candidate or matched control missing at a target".into());
    }
    let mut curve_targets: BTreeMap<(u64, u64, u64), BTreeSet<[u64; 2]>> = BTreeMap::new();
    for (curve, targets) in populations.values() {
        let all = curve_targets
            .entry((curve.p, curve.a, curve.b))
            .or_insert_with(|| {
                let mut all = BTreeSet::new();
                for x in 0..curve.p {
                    let rhs =
                        (mul(mul(x, x, curve.p), x, curve.p) + mul(curve.a, x, curve.p) + curve.b)
                            % curve.p;
                    for y in 0..curve.p {
                        if mul(y, y, curve.p) == rhs {
                            all.insert([x, y]);
                        }
                    }
                }
                all
            });
        if targets != all {
            return Err(
                "panel target population is incomplete or includes off-curve points".into(),
            );
        }
    }
    Ok(())
}
fn run(
    bundle_path: &Path,
    driver: &Path,
    outdir: &Path,
    admission_path: &Path,
) -> Result<(), String> {
    let a = admission(admission_path)?;
    if outdir.exists() {
        return Err("refuse output directory overwrite".into());
    }
    fs::create_dir_all(outdir).map_err(|e| e.to_string())?;
    let bytes = fs::read(bundle_path).map_err(|e| e.to_string())?;
    if bytes.len() > 128 * 1024 * 1024 {
        return Err("bundle exceeds 128MiB".into());
    }
    let bundle: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let systems = bundle["systems"]
        .as_array()
        .ok_or("expected systems array")?;
    if systems.len() > 20000 {
        return Err("system count cap".into());
    }
    panel_coverage(systems)?;
    let driver_sha = hash(&fs::read(driver).map_err(|e| e.to_string())?);
    let start = Instant::now();
    let mut results = Vec::new();
    let mut failed = 0usize;
    for (index, value) in systems.iter().enumerate() {
        if start.elapsed() > Duration::from_secs(a["max_seconds"].as_u64().unwrap()) {
            results.push(json!({"status":"panel_deadline","first_unattempted_index":index,"unattempted_systems":systems.len()-index}));
            failed += 1;
            break;
        }
        let s: System =
            serde_json::from_value(value.clone()).map_err(|e| format!("system {index}: {e}"))?;
        let input = validate(&s)?;
        let dir = outdir.join(format!("cell-{index:05}"));
        fs::create_dir(&dir).map_err(|e| e.to_string())?;
        fs::write(dir.join("input.tokens"), export_tokens(&s)).map_err(|e| e.to_string())?;
        fs::write(dir.join("input.ms"), export_msolve(&s)).map_err(|e| e.to_string())?;
        write(&dir.join("input.json"), value)?;
        let stdout = File::create(dir.join("solver.json")).map_err(|e| e.to_string())?;
        let stderr = File::create(dir.join("solver.stderr")).map_err(|e| e.to_string())?;
        let wall = Instant::now();
        let status = Command::new("timeout")
            .args(["--signal=TERM", "--kill-after=1s", "10s"])
            .arg(driver)
            .arg(dir.join("input.tokens"))
            .env("OMP_NUM_THREADS", "1")
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .status()
            .map_err(|e| e.to_string())?;
        let host_ns = wall.elapsed().as_nanos();
        let raw = fs::read(dir.join("solver.json")).map_err(|e| e.to_string())?;
        let trace = fs::read(dir.join("solver.stderr")).map_err(|e| e.to_string())?;
        let mut result = json!({"id":s.id,"source_system_id":s.id,"arm":s.arm,"control_name":value.get("control_name"),"d":value.get("d"),"field_p":s.p,"curve":value["curve"],"target":s.target,"base_x":s.base_x,"base_columns":s.base_x.len(),"signed_base_points":s.base_points.len(),"fb_id":value.get("fb_id"),"signs":s.signs,"status":"solver_failed","exit_code":status.code(),"input_sha256":hash(&serde_json::to_vec(value).unwrap()),"solver_stdout_sha256":hash(&raw),"solver_stderr_sha256":hash(&trace),"process_wall_ns":host_ns.to_string(),"timing_qualification":"L0_shared_host_diagnostic_only","maximum_input_degree":input.iter().flat_map(|f|f.keys()).map(|m|degree(m)).max().unwrap_or(0)});
        if status.success() {
            let checked = (|| -> Result<Value, String> {
                let mut solver: Value = serde_json::from_slice(&raw).map_err(|e| e.to_string())?;
                for (k, v) in trace_statistics(&String::from_utf8_lossy(&trace))
                    .as_object()
                    .unwrap()
                {
                    solver[k] = v.clone();
                }
                let ts: Vec<Vec<Term>> = serde_json::from_value(solver["reduced_basis"].clone())
                    .map_err(|e| e.to_string())?;
                let basis = ts
                    .iter()
                    .map(|t| parse_poly(t, s.variables.len(), s.p))
                    .collect::<Result<Vec<_>, _>>()?;
                let criterion = check_gb(&input, &basis, s.p)?;
                let (root_set, tries) = roots(&basis, s.variables.len(), s.p)?;
                let (expected, relations) = references(&s);
                let excluded_reference = expected
                    .iter()
                    .filter(|v| input.iter().any(|f| eval(f, v, s.p) != 0))
                    .count();
                if excluded_reference != 0 {
                    return Ok(
                        json!({"status":"verification_failed","error":"input equations exclude independently valid reference relations","false_negatives":excluded_reference,"false_positives":null,"expected_roots":expected,"roots":root_set}),
                    );
                }
                if let Some(producer) = &s.expected_roots {
                    let producer: BTreeSet<_> = producer.iter().cloned().collect();
                    if producer != expected {
                        return Err(
                            "producer expected roots disagree with independent group reference"
                                .into(),
                        );
                    }
                }
                let fp = root_set.difference(&expected).count();
                let nf = expected.difference(&root_set).count();
                let selected = selected_degrees(&String::from_utf8_lossy(&trace));
                let shapes = trace_matrix_shapes(&String::from_utf8_lossy(&trace));
                let input_degree = result["maximum_input_degree"].as_u64().unwrap();
                let max_pair = selected.iter().copied().max();
                let trace_complete = solver["pairs_reduced"] == 0 || !selected.is_empty();
                let observed =
                    trace_complete.then(|| max_pair.map(u64::from).unwrap_or(0).max(input_degree));
                let basis_degree = basis
                    .iter()
                    .flat_map(|f| f.keys())
                    .map(|m| degree(m))
                    .max()
                    .unwrap_or(0);
                Ok(
                    json!({"status":if fp==0&&nf==0{"verified"}else{"verification_failed"},"solver":solver,"gb_checks":criterion,"maximum_f4_selected_pair_degree":max_pair,"selected_pair_degree_trace":selected,"selected_degree_trace_complete":trace_complete,"observed_f4_degree":observed,"observed_f4_degree_definition":"max(maximum_input_degree,maximum_selected_pair_degree); not homogeneous regularity or sugar degree","trace_matrix_shapes":shapes,"peak_matrix_rows_in_trace":shapes.iter().map(|s|s[0]).max(),"peak_matrix_columns_in_trace":shapes.iter().map(|s|s[1]).max(),"maximum_exported_basis_degree":basis_degree,"roots":root_set,"expected_roots":expected,"sat_class":if relations.is_empty(){"UNSAT"}else{"SAT"},"ordered_signed_relation_count":relations.len(),"group_replayed_relations":relations,"every_reported_relation_group_replayed":true,"false_positives":fp,"false_negatives":nf,"root_extraction_variable_values_tested":tries,"full_ideal_equality":"not_certified; exact rational-root set equality verified"}),
                )
            })();
            match checked {
                Ok(fields) => {
                    for (k, v) in fields.as_object().unwrap() {
                        result[k] = v.clone();
                    }
                }
                Err(e) => {
                    result["status"] = json!("verification_failed");
                    result["error"] = json!(e);
                }
            }
        }
        if result["status"] != "verified" {
            failed += 1;
        }
        result["input_json_sha256"] = json!(hash(
            &fs::read(dir.join("input.json")).map_err(|e| e.to_string())?
        ));
        result["input_tokens_sha256"] = json!(hash(
            &fs::read(dir.join("input.tokens")).map_err(|e| e.to_string())?
        ));
        result["input_msolve_sha256"] = json!(hash(
            &fs::read(dir.join("input.ms")).map_err(|e| e.to_string())?
        ));
        write(&dir.join("result.json"), &result)?;
        results.push(json!({"id":s.id,"arm":s.arm,"target":s.target,"signs":s.signs,"status":result["status"],"result_path":format!("cell-{index:05}/result.json"),"result_sha256":hash(&fs::read(dir.join("result.json")).map_err(|e|e.to_string())?)}));
    }
    let attempted = results
        .iter()
        .filter(|x| x.get("result_path").is_some())
        .count();
    let verified = results.iter().filter(|x| x["status"] == "verified").count();
    let summary = json!({"schema":"secant-f4-panel/v1","input_bundle_sha256":hash(&bytes),"driver_sha256":driver_sha,"source_revision":"3672113ee2c696a2ac9c637636e0f7214f500ba1","algorithm":"stock_msolve_neogb_f4","admission_record_sha256":hash(&fs::read(admission_path).map_err(|e|e.to_string())?),"requested_systems":systems.len(),"attempted_cells":attempted,"completed_cells":verified,"failed_cells":attempted-verified,"unattempted_cells":systems.len()-attempted,"failed_or_unfinished":systems.len()-verified,"maximum_threads_per_cas":1,"per_cas_timeout_seconds":10,"timing_qualification":"L0_diagnostic; no performance win claimed","results":results});
    write(&outdir.join("summary.json"), &summary)?;
    if failed != 0 {
        return Err("panel has failed/unfinished cells; raw artifacts retained".into());
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() == 7 && a[1] == "run" && a[5] == "--admission" {
        run(
            Path::new(&a[2]),
            Path::new(&a[3]),
            Path::new(&a[4]),
            Path::new(&a[6]),
        )?;
    } else {
        return Err("usage: secant-f4-panel run SYSTEMS.json DRIVER FRESH_OUTPUT --admission ADMISSION.json".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn poly(ts: &[(Vec<u32>, u64)]) -> Poly {
        ts.iter().cloned().collect()
    }
    #[test]
    fn drl_uses_total_degree_then_reverse_lex() {
        assert_eq!(cmp(&[2, 0], &[1, 1]), Ordering::Greater);
        assert_eq!(cmp(&[0, 3], &[2, 0]), Ordering::Greater);
    }
    #[test]
    fn normal_form_and_buchberger_checks() {
        let g = vec![
            poly(&[(vec![1, 0], 1), (vec![0, 0], 16)]),
            poly(&[(vec![0, 1], 1), (vec![0, 0], 15)]),
        ];
        check_gb(&g, &g, 17).unwrap();
        let f = poly(&[(vec![2, 0], 1), (vec![0, 0], 16)]);
        assert!(remainder(f, &g, 17).unwrap().is_empty());
    }
    #[test]
    fn triangular_roots_are_complete_for_artificial_ideal() {
        let g = vec![
            poly(&[(vec![2, 0], 1), (vec![0, 0], 16)]),
            poly(&[(vec![0, 1], 1), (vec![1, 0], 16)]),
        ];
        let (r, _) = roots(&g, 2, 17).unwrap();
        assert_eq!(r, BTreeSet::from([vec![1, 1], vec![16, 16]]));
    }
    #[test]
    fn unit_ideal_has_no_roots() {
        let g = vec![poly(&[(vec![0, 0], 1)])];
        assert!(roots(&g, 2, 17).unwrap().0.is_empty());
    }
    #[test]
    fn external_adapter_artificial_smoke_has_exact_roots() {
        let raw: Value =
            serde_json::from_str(include_str!("development/artificial-final-solver.json")).unwrap();
        assert_eq!(raw["algorithm"], "stock_msolve_neogb_f4");
        assert_eq!(raw["random_seed"], 315);
        let ts: Vec<Vec<Term>> = serde_json::from_value(raw["reduced_basis"].clone()).unwrap();
        let gb = ts
            .iter()
            .map(|t| parse_poly(t, 2, 17).unwrap())
            .collect::<Vec<_>>();
        let original = vec![
            poly(&[(vec![2, 0], 1), (vec![0, 0], 16)]),
            poly(&[(vec![0, 1], 1), (vec![1, 0], 16)]),
        ];
        check_gb(&original, &gb, 17).unwrap();
        assert_eq!(
            roots(&gb, 2, 17).unwrap().0,
            BTreeSet::from([vec![1, 1], vec![16, 16]])
        );
    }
    #[test]
    fn parser_never_credits_nontrace_statistics_as_degree() {
        let s="deg     sel   pairs        mat density\n  3  2  8  4 x 7 25.0%\nmax. matrix data 4 x 7\n";
        assert_eq!(selected_degrees(s), vec![3]);
        let full =
            format!("{s}reduce final basis 9 x 31 15.0%\nreduce intermediate basis 22 x 6 2.0%\n");
        let shapes = trace_matrix_shapes(&full);
        assert!(shapes.contains(&[9, 31]));
        assert!(shapes.contains(&[22, 6]));
        let real = include_str!("development/artificial-pairs-solver.stderr");
        assert_eq!(selected_degrees(real), vec![3, 4]);
        assert_eq!(trace_statistics(real)["pairs_reduced"], 2);
        assert_eq!(trace_statistics(real)["rows_reduced"], 7);
        assert_eq!(trace_statistics(real)["largest_area_matrix_columns"], 7);
    }
    #[test]
    fn noncanonical_input_is_rejected() {
        assert!(parse_poly(
            &[Term {
                exponents: vec![1, 0],
                coefficient: 0
            }],
            2,
            17
        )
        .is_err());
        assert!(parse_poly(&[], 2, 17).is_ok());
    }
    #[test]
    fn malformed_equations_cannot_erase_valid_group_reference() {
        let s:System=serde_json::from_value(json!({"id":"oracle-regression","p":17,"variables":["x","z","m"],"polynomials":[[{"coefficient":1,"exponents":[0,0,0]}]],"curve":{"p":17,"a":2,"b":2},"target":[6,3],"base_x":[5],"base_points":[[5,1],[5,16]],"arm":"candidate-secant","signs":[1,1],"h_low_to_high":[1]})).unwrap();
        assert!(validate(&s).is_err());
        let input = vec![parse_poly(&s.polynomials[0], 3, 17).unwrap()];
        let (reference, _) = references(&s);
        assert_eq!(reference, BTreeSet::from([vec![5, 5, 13]]));
        assert!(reference
            .iter()
            .any(|v| input.iter().any(|f| eval(f, v, s.p) != 0)));
    }
    #[test]
    fn independent_model_matches_committed_complete_secant_fixture() {
        let cases: Value =
            serde_json::from_str(include_str!("../../secant/target-10-6.json")).unwrap();
        let fixture: Value =
            serde_json::from_str(include_str!("../../secant/fixture.json")).unwrap();
        for (i, name) in ["candidate", "control"].iter().enumerate() {
            let points: Vec<[u64; 2]> = fixture[*name]["points"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| {
                    [
                        u64::from_str_radix(p["x"].as_str().unwrap().trim_start_matches("0x"), 16)
                            .unwrap(),
                        u64::from_str_radix(p["y"].as_str().unwrap().trim_start_matches("0x"), 16)
                            .unwrap(),
                    ]
                })
                .collect();
            let xs: BTreeSet<_> = points.iter().map(|p| p[0]).collect();
            for branch in cases["arms"][i]["branches"].as_array().unwrap() {
                let s:System=serde_json::from_value(json!({"id":"committed-model","p":17,"variables":["x","z","m"],"polynomials":branch["polynomials"],"curve":{"p":17,"a":2,"b":2},"target":[10,6],"base_x":xs,"base_points":points,"arm":format!("{name}-secant"),"signs":branch["signs"],"h_low_to_high":fixture[format!("{name}_h_low_to_high")]})).unwrap();
                validate(&s).unwrap();
            }
        }
    }
}
