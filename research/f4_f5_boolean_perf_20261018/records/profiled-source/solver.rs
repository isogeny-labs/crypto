//! Bounded complete finite-field system solving. No Gröbner computation.
#[path = "field.rs"]
pub mod field;
#[path = "poly.rs"]
pub mod poly;
use field::{Field, Result, N};
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct Limits {
    pub max_work: u64,
    pub max_table: usize,
    pub max_solutions: usize,
    pub max_domain: usize,
    pub max_degree: usize,
    pub max_trials: u64,
    pub seed: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_work: 1_000_000,
            max_table: 100_000,
            max_solutions: 10_000,
            max_domain: 4096,
            max_degree: 256,
            max_trials: 2048,
            seed: 1,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Term {
    pub coefficient: N,
    pub powers: Vec<u32>,
}
pub type Equation = Vec<Term>;
#[derive(Clone, Debug)]
pub struct Search {
    pub solutions: Vec<Vec<N>>,
    pub count: Option<u64>,
    pub lower_bound: u64,
    pub complete: bool,
    pub work: u64,
    pub method: String,
}

pub fn evaluate(f: &Field, e: &Equation, x: &[N]) -> N {
    e.iter().fold(N::zero(), |sum, t| {
        f.add(
            &sum,
            &t.powers
                .iter()
                .zip(x)
                .fold(t.coefficient.clone(), |a, (&p, x)| {
                    if p == 0 {
                        a
                    } else {
                        f.mul(&a, &f.pow(x, &N::from(p)))
                    }
                }),
        )
    })
}
fn add_solution(out: &mut Search, x: Vec<N>, limit: usize) -> Result<()> {
    out.lower_bound = out
        .lower_bound
        .checked_add(1)
        .ok_or("solution count overflow")?;
    if out.solutions.len() < limit {
        out.solutions.push(x);
    }
    Ok(())
}
pub fn normalize(f: &Field, raw: &Value, n: usize) -> Result<Vec<Equation>> {
    let raw = raw
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or("at least one equation required")?;
    raw.iter()
        .map(|e| {
            let mut merged = BTreeMap::<Vec<u32>, N>::new();
            for t in e.as_array().ok_or("equation must be sparse terms")? {
                let powers = t["powers"]
                    .as_array()
                    .filter(|a| a.len() == n)
                    .ok_or("powers must match variables")?
                    .iter()
                    .map(|p| {
                        p.as_u64()
                            .filter(|p| *p <= 1_000_000)
                            .map(|p| p as u32)
                            .ok_or("invalid exponent".to_string())
                    })
                    .collect::<Result<Vec<_>>>()?;
                let c = f.element(&t["coefficient"])?;
                let v = merged.entry(powers).or_default();
                *v = f.add(v, &c);
            }
            Ok(merged
                .into_iter()
                .filter(|(_, c)| !c.is_zero())
                .map(|(powers, coefficient)| Term {
                    powers,
                    coefficient,
                })
                .collect())
        })
        .collect()
}

pub fn search(
    f: &Field,
    equations: &[Equation],
    n: usize,
    domain: &[N],
    limits: &Limits,
    method: &str,
) -> Result<Search> {
    if n == 0 {
        return Err("at least one variable required".into());
    }
    if domain.iter().collect::<BTreeSet<_>>().len() != domain.len() {
        return Err("duplicate domain element".into());
    }
    for x in domain {
        f.validate(x)?;
    }
    for e in equations {
        for t in e {
            f.validate(&t.coefficient)?;
            if t.powers.len() != n {
                return Err("term arity mismatch".into());
            }
        }
    }
    let mut out = Search {
        solutions: vec![],
        count: None,
        lower_bound: 0,
        complete: true,
        work: 0,
        method: method.into(),
    };
    if domain.is_empty() {
        out.count = Some(0);
        return Ok(out);
    }
    if !["auto", "enumerate", "mitm", "native_boolean"].contains(&method) {
        return Err("unknown search method".into());
    }
    let boolean = domain.len() == 2
        && domain.contains(&N::zero())
        && domain.contains(&N::one())
        && f.p == N::from(2u8)
        && equations
            .iter()
            .flatten()
            .all(|t| t.powers.iter().filter(|&&p| p > 0).count() <= 2);
    if method == "native_boolean" && !boolean {
        return Err(
            "native Boolean route requires quadratic Boolean support in characteristic two".into(),
        );
    }
    // Enumerate one packed 64-equation necessary subsystem; verify every
    // resulting assignment against all original coefficient equations.
    // The subsystem is a superset, so no original root can be omitted.
    if boolean
        && n <= 32
        && (method == "auto" || method == "native_boolean")
        && (1u64 << n) <= limits.max_work
    {
        use crypto_lib::cryptanalysis::{
            mq_fes::{gray_incremental_find_all, QuadraticForm},
            pq_groebner_f2::{F2BoolMono, F2BoolPoly},
        };
        let mut binary = Vec::new();
        for e in equations {
            let mut merged = BTreeMap::<u64, N>::new();
            for t in e {
                let mask = t
                    .powers
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| **p > 0)
                    .fold(0, |v, (i, _)| v | (1u64 << i));
                *merged.entry(mask).or_default() ^= &t.coefficient;
            }
            let bits = merged.values().map(N::bits).max().unwrap_or(0);
            for bit in 0..bits {
                let terms = merged
                    .iter()
                    .filter(|(_, c)| c.bit(bit))
                    .map(|(&m, _)| F2BoolMono::from_mask(m))
                    .collect::<Vec<_>>();
                if !terms.is_empty() {
                    binary.push(F2BoolPoly::from_monos(terms, n));
                }
                if binary.len() == 64 {
                    break;
                }
            }
            if binary.len() == 64 {
                break;
            }
        }
        out.method = "native_boolean_gray".into();
        out.work = 1u64 << n;
        if binary.is_empty() {
            out.method = "constant_boolean".into();
            out.work = 0;
            out.lower_bound = 1u64 << n;
            out.count = Some(out.lower_bound);
            for bits in 0..out.lower_bound.min(limits.max_solutions as u64) {
                out.solutions
                    .push((0..n).map(|i| N::from((bits >> i) & 1)).collect());
            }
            out.solutions.sort();
            return Ok(out);
        }
        let candidates = {
            let forms = binary
                .iter()
                .map(|p| QuadraticForm::from_poly(p).unwrap())
                .collect::<Vec<_>>();
            gray_incremental_find_all(&forms, usize::MAX)
                .ok_or("native Boolean backend refused input")?
        };
        for bits in candidates {
            let x = (0..n).map(|i| N::from((bits >> i) & 1)).collect::<Vec<_>>();
            if equations.iter().all(|e| evaluate(f, e, &x).is_zero()) {
                add_solution(&mut out, x, limits.max_solutions)?;
            }
        }
        out.solutions.sort();
        out.count = Some(out.lower_bound);
        return Ok(out);
    }
    let mut parent = (0..n).collect::<Vec<_>>();
    fn root(parent: &[usize], mut i: usize) -> usize {
        while parent[i] != i {
            i = parent[i];
        }
        i
    }
    for t in equations.iter().flatten() {
        let support = t
            .powers
            .iter()
            .enumerate()
            .filter(|(_, p)| **p > 0)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if let Some(&first) = support.first() {
            for &i in &support[1..] {
                let (a, b) = (root(&parent, first), root(&parent, i));
                parent[b] = a;
            }
        }
    }
    let mut components = BTreeMap::<usize, Vec<usize>>::new();
    for i in 0..n {
        components.entry(root(&parent, i)).or_default().push(i);
    }
    if method == "mitm" && components.len() < 2 {
        return Err("MITM requires at least two monomial-interaction components".into());
    }
    if components.len() >= 2 && (method == "mitm" || method == "auto") {
        let mut components = components.into_values().collect::<Vec<_>>();
        components.sort_by_key(|c| (std::cmp::Reverse(c.len()), c[0]));
        let (mut left, mut right) = (Vec::new(), Vec::new());
        for c in components {
            if left.len() <= right.len() {
                left.extend(c);
            } else {
                right.extend(c);
            }
        }
        left.sort();
        right.sort();
        let mut table = BTreeMap::<Vec<N>, Vec<Vec<N>>>::new();
        let (left_eq, right_eq): (Vec<Equation>, Vec<Equation>) = equations
            .iter()
            .map(|e| {
                e.iter().cloned().partition(|t| {
                    t.powers
                        .iter()
                        .enumerate()
                        .all(|(i, &p)| p == 0 || left.contains(&i))
                })
            })
            .unzip();
        out.method = "component_mitm".into();
        let left_count = domain
            .len()
            .checked_pow(left.len() as u32)
            .filter(|&c| c <= limits.max_table)
            .ok_or("MITM table preflight cap")?;
        if left_count as u64 > limits.max_work {
            out.complete = false;
            return Ok(out);
        }
        fn tuple(mut code: usize, vars: &[usize], domain: &[N], n: usize) -> Vec<N> {
            let mut x = vec![N::zero(); n];
            for &i in vars {
                x[i] = domain[code % domain.len()].clone();
                code /= domain.len();
            }
            x
        }
        for code in 0..left_count {
            let x = tuple(code, &left, domain, n);
            let key = left_eq
                .iter()
                .map(|e| evaluate(f, e, &x))
                .collect::<Vec<_>>();
            table.entry(key).or_default().push(x);
            out.work += 1;
        }
        let right_count = domain
            .len()
            .checked_pow(right.len() as u32)
            .ok_or("MITM right size overflow")?;
        for code in 0..right_count {
            if out.work >= limits.max_work {
                out.complete = false;
                break;
            }
            out.work += 1;
            let x = tuple(code, &right, domain, n);
            let key = right_eq
                .iter()
                .map(|e| f.neg(&evaluate(f, e, &x)))
                .collect::<Vec<_>>();
            if let Some(matches) = table.get(&key) {
                let keep = matches
                    .len()
                    .min(limits.max_solutions.saturating_sub(out.solutions.len()));
                out.lower_bound = out
                    .lower_bound
                    .checked_add((matches.len() - keep) as u64)
                    .ok_or("solution count overflow")?;
                for old in matches.iter().take(keep) {
                    let mut joined = old.clone();
                    for &i in &right {
                        joined[i] = x[i].clone();
                    }
                    add_solution(&mut out, joined, limits.max_solutions)?;
                }
            }
        }
    } else {
        out.method = "enumerate".into();
        let mut x = vec![domain[0].clone(); n];
        let mut digits = vec![0usize; n];
        loop {
            if out.work >= limits.max_work {
                out.complete = false;
                break;
            }
            out.work += 1;
            if equations.iter().all(|e| evaluate(f, e, &x).is_zero()) {
                add_solution(&mut out, x.clone(), limits.max_solutions)?;
            }
            let mut i = 0;
            while i < n {
                digits[i] += 1;
                if digits[i] < domain.len() {
                    x[i] = domain[digits[i]].clone();
                    break;
                }
                digits[i] = 0;
                x[i] = domain[0].clone();
                i += 1;
            }
            if i == n {
                break;
            }
        }
    }
    out.solutions.sort();
    if out.complete {
        out.count = Some(out.lower_bound);
    }
    Ok(out)
}

pub fn linearized_domain(
    f: &Field,
    terms: &[(usize, N)],
    constant: &N,
    max_domain: usize,
) -> Result<Vec<N>> {
    f.validate(constant)?;
    for (_, a) in terms {
        f.validate(a)?;
    }
    // Linearized Frobenius polynomials are F_p-linear maps. Solve their
    // affine system over the certified prime field, never modulo p^degree.
    let base = f.prime_subfield();
    let image = |x: &N| {
        terms.iter().fold(N::zero(), |v, (power, c)| {
            f.add(
                &v,
                &f.mul(c, &f.pow(x, &f.p.pow((power % f.degree) as u32))),
            )
        })
    };
    let d = f.degree;
    let mut rows = vec![vec![N::zero(); d + 1]; d];
    let rhs = f.digits(&f.neg(constant));
    for i in 0..d {
        rows[i][d] = rhs[i].clone();
    }
    let mut unit = N::one();
    for j in 0..d {
        let col = f.digits(&image(&unit));
        for i in 0..d {
            rows[i][j] = col[i].clone();
        }
        unit *= &f.p;
    }
    let mut pivots = Vec::new();
    for col in 0..d {
        let rank = pivots.len();
        let Some(pivot) = (rank..d).find(|&i| !rows[i][col].is_zero()) else {
            continue;
        };
        rows.swap(rank, pivot);
        let inv = base.inv(&rows[rank][col]);
        for a in &mut rows[rank] {
            *a = base.mul(a, &inv);
        }
        for i in 0..d {
            if i != rank && !rows[i][col].is_zero() {
                let scale = rows[i][col].clone();
                for j in col..=d {
                    rows[i][j] = base.sub(&rows[i][j], &base.mul(&scale, &rows[rank][j]));
                }
            }
        }
        pivots.push(col);
    }
    if rows
        .iter()
        .any(|r| r[..d].iter().all(N::is_zero) && !r[d].is_zero())
    {
        return Ok(vec![]);
    }
    let free = (0..d).filter(|c| !pivots.contains(c)).collect::<Vec<_>>();
    let count =
        f.p.pow(free.len() as u32)
            .to_usize()
            .filter(|&c| c <= max_domain)
            .ok_or("affine domain exceeds cap")?;
    let mut origin = vec![N::zero(); d];
    for (r, &c) in pivots.iter().enumerate() {
        origin[c] = rows[r][d].clone();
    }
    let mut out = Vec::with_capacity(count);
    for code in 0..count {
        let mut digits = origin.clone();
        let mut code = N::from(code);
        for &c in &free {
            let value = &code % &f.p;
            code /= &f.p;
            digits[c] = value.clone();
            for (r, &pivot) in pivots.iter().enumerate() {
                digits[pivot] = base.sub(&digits[pivot], &base.mul(&rows[r][c], &value));
            }
        }
        let x = f.encode(&digits);
        if f.add(&image(&x), constant) != N::zero() {
            return Err("affine-domain replay failed".into());
        }
        out.push(x);
    }
    out.sort();
    Ok(out)
}
pub fn domain(f: &Field, l: &Value, limits: &Limits) -> Result<Vec<N>> {
    let mut budget = poly::SplitBudget {
        remaining: limits.max_trials,
        state: limits.seed.max(1),
    };
    let coefficients = |v: &Value| -> Result<Vec<N>> {
        v.as_array()
            .filter(|a| !a.is_empty())
            .ok_or("nonempty coefficients required")?
            .iter()
            .map(|v| f.element(v))
            .collect()
    };
    let variants = ["coefficients", "linearized", "composition", "elements"]
        .iter()
        .filter(|k| l.get(**k).is_some())
        .count();
    if variants != 1 {
        return Err("domain requires exactly one representation".into());
    }
    if let Some(v) = l.get("elements") {
        let mut out = coefficients(v)?;
        out.sort();
        out.dedup();
        if out.len() > limits.max_domain {
            return Err("explicit domain exceeds cap".into());
        }
        return Ok(out);
    }
    if let Some(v) = l.get("linearized") {
        if l.get("denominator").is_some() {
            return Err("linearized domain cannot have a denominator".into());
        }
        let terms = v["terms"]
            .as_array()
            .ok_or("linearized terms required")?
            .iter()
            .map(|t| {
                Ok((
                    t["power"].as_u64().ok_or("Frobenius power required")? as usize,
                    f.element(&t["coefficient"])?,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let constant = f.element(v.get("constant").unwrap_or(&json!(0)))?;
        return linearized_domain(f, &terms, &constant, limits.max_domain);
    }
    if let Some(maps) = l.get("composition") {
        if l.get("denominator").is_some() {
            return Err("composition denominator belongs to each map".into());
        }
        let maps = maps
            .as_array()
            .filter(|a| !a.is_empty())
            .ok_or("composition needs maps")?
            .iter()
            .map(|m| {
                Ok((
                    coefficients(&m["coefficients"])?,
                    coefficients(m.get("denominator").unwrap_or(&json!([1])))?,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        if l.get("semantics").is_some() && l["semantics"] != "partial_circuit" {
            return Err("unsupported composition semantics".into());
        }
        if maps.iter().any(|(n, d)| {
            n.len() > limits.max_degree.saturating_add(1)
                || d.len() > limits.max_degree.saturating_add(1)
        }) {
            return Err("composition map degree cap".into());
        }
        if maps.iter().any(|(_, d)| d.len() > 1) && l["semantics"] != "partial_circuit" {
            return Err("rational composition requires partial_circuit semantics".into());
        }
        let mut targets = vec![N::zero()];
        let mut work = 0;
        for (num, den) in maps.iter().rev() {
            if poly::trim(den.clone()).is_empty() {
                return Err("zero denominator".into());
            }
            let mut next = BTreeSet::new();
            for target in targets {
                if work >= limits.max_work {
                    return Err("composition inverse-work cap".into());
                }
                work += 1;
                let scaled = den.iter().map(|d| f.mul(d, &target)).collect::<Vec<_>>();
                for x in poly::roots(
                    f,
                    poly::sub(f, num, &scaled),
                    limits.max_domain,
                    limits.max_degree,
                    &mut budget,
                )? {
                    if !poly::eval(f, den, &x).is_zero() {
                        next.insert(x);
                    }
                }
                if next.len() > limits.max_domain {
                    return Err("composition domain exceeds cap".into());
                }
            }
            targets = next.into_iter().collect();
        }
        return Ok(targets);
    }
    let numerator = coefficients(&l["coefficients"])?;
    let denominator = coefficients(l.get("denominator").unwrap_or(&json!([1])))?;
    if denominator.len() > limits.max_degree.saturating_add(1) {
        return Err("domain denominator degree cap".into());
    }
    if poly::trim(denominator.clone()).is_empty() {
        return Err("zero denominator".into());
    }
    Ok(poly::roots(
        f,
        numerator,
        limits.max_domain,
        limits.max_degree,
        &mut budget,
    )?
    .into_iter()
    .filter(|x| !poly::eval(f, &denominator, x).is_zero())
    .collect())
}

pub fn solve_json(problem: &Value, limits: &Limits) -> Result<Value> {
    if problem["method"] == "separator" {
        return separator_json(problem, limits);
    }
    let f = Field::from_json(problem.get("field").unwrap_or(&json!({})))?;
    let variables = problem["variables"]
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or("distinct variables required")?;
    let names = variables
        .iter()
        .map(|v| {
            v.as_str()
                .filter(|s| !s.is_empty())
                .ok_or("variable must be named")
        })
        .collect::<std::result::Result<BTreeSet<_>, _>>()?;
    if names.len() != variables.len() {
        return Err("duplicate variable".into());
    }
    let n = variables.len();
    let raw = problem
        .get("equations")
        .cloned()
        .unwrap_or_else(|| json!([problem["f"]]));
    let equations = normalize(&f, &raw, n)?;
    let domain = match domain(&f, &problem["L"], limits) {
        Ok(d) => d,
        Err(reason) => {
            return Ok(
                json!({"status":"budget_exceeded","reason":reason,"solutions":[],"solution_count_exact":null,
        "certificate":{"arithmetic":"exact","domain_complete":false,"search_complete":false,"all_solutions_returned":false,"returned_tuples_verified":true}}),
            )
        }
    };
    let out = search(
        &f,
        &equations,
        n,
        &domain,
        limits,
        problem["method"].as_str().unwrap_or("auto"),
    )?;
    let verified = out.solutions.iter().all(|x| {
        x.iter().all(|a| domain.contains(a))
            && equations.iter().all(|e| evaluate(&f, e, x).is_zero())
    });
    if !verified {
        return Err("original-system verification failed".into());
    }
    let all = out.complete && out.lower_bound == out.solutions.len() as u64;
    Ok(
        json!({"schema":"generalized-rootfinder/v1","status":if all{"complete"}else if out.complete{"output_truncated"}else{"budget_exceeded"},
        "field":{"characteristic":f.p.to_str_radix(10),"degree":f.degree,"order":f.order.to_str_radix(10),"field_verified":true},
        "variables":variables,"domain":{"status":"complete","roots":domain.iter().map(|x|format!("0x{}",x.to_str_radix(16))).collect::<Vec<_>>(),"domain_size":domain.len()},
        "method":out.method,"work":out.work,"solution_count_exact":out.count,"solution_count_lower_bound":out.lower_bound,
        "solutions":out.solutions.iter().map(|x|x.iter().map(|a|format!("0x{}",a.to_str_radix(16))).collect::<Vec<_>>()).collect::<Vec<_>>(),
        "certificate":{"arithmetic":"exact","domain_complete":true,"search_complete":out.complete,"all_solutions_returned":all,"returned_tuples_verified":verified}}),
    )
}

/// Exact S3 separator preserving all eight original coordinates. Source
/// variables lie in D; auxiliary fibers range over the whole field.
pub struct Separator<'a> {
    pub field: &'a Field,
    pub domain: Vec<N>,
    curve: [N; 3],
    binary: bool,
    left: BTreeMap<N, Vec<[N; 4]>>,
    cache: BTreeMap<(N, N), Vec<N>>,
    budget: poly::SplitBudget,
    pub calls: u64,
    pub left_records: usize,
    limits: Limits,
}
impl<'a> Separator<'a> {
    pub fn new(field: &'a Field, domain: Vec<N>, curve: &Value, limits: &Limits) -> Result<Self> {
        let f = field;
        let scalar = |n: u8| N::from(n) % &f.p;
        let binary = f.p == N::from(2u8);
        let get = |name: &str, default: u8| f.element(curve.get(name).unwrap_or(&json!(default)));
        let parameters = if binary {
            [N::zero(), N::zero(), get("b", 1)?]
        } else {
            [get("a2", 0)?, get("a4", 0)?, get("a6", 1)?]
        };
        if binary {
            if parameters[2].is_zero() {
                return Err("singular binary curve".into());
            }
        } else {
            let cubic = vec![
                parameters[2].clone(),
                parameters[1].clone(),
                parameters[0].clone(),
                N::one(),
            ];
            let derivative = vec![
                parameters[1].clone(),
                f.mul(&scalar(2), &parameters[0]),
                scalar(3),
            ];
            if poly::gcd(f, cubic, poly::trim(derivative)).len() != 1 {
                return Err("singular odd-characteristic cubic".into());
            }
        }
        if domain.len() > limits.max_domain
            || domain.iter().collect::<BTreeSet<_>>().len() != domain.len()
        {
            return Err("invalid source domain".into());
        }
        for x in &domain {
            f.validate(x)?;
        }
        if domain
            .len()
            .checked_pow(3)
            .and_then(|n| n.checked_mul(4))
            .is_none_or(|n| n > limits.max_table)
        {
            return Err("complete separator left-table preflight cap".into());
        }
        let mut s = Self {
            field,
            domain: domain.clone(),
            curve: parameters,
            binary,
            left: BTreeMap::new(),
            cache: BTreeMap::new(),
            budget: poly::SplitBudget {
                remaining: limits.max_trials,
                state: limits.seed.max(1),
            },
            calls: 0,
            left_records: 0,
            limits: limits.clone(),
        };
        for x1 in &domain {
            for x2 in &domain {
                for u1 in s.fiber(x1, x2)? {
                    for x3 in &domain {
                        for u2 in s.fiber(&u1, x3)? {
                            s.left.entry(u2).or_default().push([
                                x1.clone(),
                                x2.clone(),
                                x3.clone(),
                                u1.clone(),
                            ]);
                            s.left_records += 1;
                        }
                    }
                }
            }
        }
        Ok(s)
    }
    pub fn evaluate(&self, x: &N, y: &N, z: &N) -> N {
        let f = self.field;
        let xy = f.mul(x, y);
        if self.binary {
            let pairs = f.add(&xy, &f.add(&f.mul(x, z), &f.mul(y, z)));
            return f.add(
                &f.add(&f.mul(&pairs, &pairs), &f.mul(&xy, z)),
                &self.curve[2],
            );
        }
        let pair = f.add(&xy, &f.add(&f.mul(x, z), &f.mul(y, z)));
        let total = f.add(&f.add(&f.add(x, y), z), &self.curve[0]);
        let product = f.add(&f.mul(&xy, z), &self.curve[2]);
        let difference = f.sub(&self.curve[1], &pair);
        f.sub(
            &f.mul(&difference, &difference),
            &f.mul(&(N::from(4u8) % &f.p), &f.mul(&total, &product)),
        )
    }
    fn fiber(&mut self, x: &N, y: &N) -> Result<Vec<N>> {
        let key = if x <= y {
            (x.clone(), y.clone())
        } else {
            (y.clone(), x.clone())
        };
        if let Some(v) = self.cache.get(&key) {
            return Ok(v.clone());
        }
        if self.calls >= self.limits.max_work {
            return Err("separator fiber-work cap".into());
        }
        self.calls += 1;
        let f = self.field;
        let xy = f.mul(x, y);
        let sum = f.add(x, y);
        let coefficients = if self.binary {
            vec![
                f.add(&f.mul(&xy, &xy), &self.curve[2]),
                xy,
                f.mul(&sum, &sum),
            ]
        } else {
            let two = N::from(2u8) % &f.p;
            let four = N::from(4u8) % &f.p;
            let inside = f.add(
                &f.mul(&sum, &f.add(&xy, &self.curve[1])),
                &f.add(
                    &f.mul(&f.mul(&two, &self.curve[0]), &xy),
                    &f.mul(&two, &self.curve[2]),
                ),
            );
            let difference = f.sub(&self.curve[1], &xy);
            let diff = f.sub(x, y);
            vec![
                f.sub(
                    &f.mul(&difference, &difference),
                    &f.mul(&f.mul(&four, &self.curve[2]), &f.add(&sum, &self.curve[0])),
                ),
                f.neg(&f.mul(&two, &inside)),
                f.mul(&diff, &diff),
            ]
        };
        if coefficients.iter().all(N::is_zero) {
            return Err("unrestricted auxiliary fiber; completeness not established".into());
        }
        let roots = poly::roots(f, coefficients, 2, 2, &mut self.budget)?;
        if roots.len() > 2 || roots.iter().any(|z| !self.evaluate(x, y, z).is_zero()) {
            return Err("original S3 fiber replay failed".into());
        }
        if self.cache.len() < self.limits.max_table {
            self.cache.insert(key, roots.clone());
        }
        Ok(roots)
    }
    pub fn solve(&mut self, target: &N) -> Result<Search> {
        self.field.validate(target)?;
        let mut out = Search {
            solutions: vec![],
            count: None,
            lower_bound: 0,
            complete: true,
            work: 0,
            method: "complete_s3_separator".into(),
        };
        let domain = self.domain.clone();
        for x5 in &domain {
            for u3 in self.fiber(x5, target)? {
                for x4 in &domain {
                    for u2 in self.fiber(&u3, x4)? {
                        if let Some(matches) = self.left.get(&u2) {
                            let keep = matches.len().min(
                                self.limits
                                    .max_solutions
                                    .saturating_sub(out.solutions.len()),
                            );
                            out.lower_bound = out
                                .lower_bound
                                .checked_add((matches.len() - keep) as u64)
                                .ok_or("solution count overflow")?;
                            for [x1, x2, x3, u1] in matches.iter().take(keep) {
                                let row = vec![
                                    x1.clone(),
                                    x2.clone(),
                                    x3.clone(),
                                    x4.clone(),
                                    x5.clone(),
                                    u1.clone(),
                                    u2.clone(),
                                    u3.clone(),
                                ];
                                if !self.evaluate(x1, x2, u1).is_zero()
                                    || !self.evaluate(u1, x3, &u2).is_zero()
                                    || !self.evaluate(&u2, x4, &u3).is_zero()
                                    || !self.evaluate(&u3, x5, target).is_zero()
                                {
                                    return Err("complete original tuple replay failed".into());
                                }
                                add_solution(&mut out, row, self.limits.max_solutions)?;
                            }
                        }
                    }
                }
            }
        }
        out.solutions.sort();
        out.count = Some(out.lower_bound);
        out.work = self.calls;
        Ok(out)
    }
}
pub fn separator_json(problem: &Value, limits: &Limits) -> Result<Value> {
    let f = Field::from_json(&problem["field"])?;
    let domain = domain(&f, &problem["L"], limits)?;
    let target = f.element(&problem["target"])?;
    let mut separator = match Separator::new(
        &f,
        domain,
        problem.get("curve").unwrap_or(&json!({})),
        limits,
    ) {
        Ok(s) => s,
        Err(reason) => {
            return Ok(
                json!({"status":"budget_exceeded","reason":reason,"search_complete":false,"output_complete":false,"solution_count_exact":null,"solutions":[]}),
            )
        }
    };
    let out = match separator.solve(&target) {
        Ok(s) => s,
        Err(reason) => {
            return Ok(
                json!({"status":"budget_exceeded","reason":reason,"search_complete":false,"output_complete":false,"solution_count_exact":null,"solutions":[]}),
            )
        }
    };
    let all = out.solutions.len() as u64 == out.lower_bound;
    Ok(
        json!({"schema":"generalized-rootfinder-separator/v1","status":if all{"complete"}else{"output_truncated"},
        "coordinates":["x1","x2","x3","x4","x5","u1","u2","u3"],"search_complete":true,"output_complete":all,"verified":true,
        "solution_count_exact":out.count,"left_records":separator.left_records,"fiber_calls":separator.calls,
        "solutions":out.solutions.iter().map(|x|x.iter().map(|a|format!("0x{}",a.to_str_radix(16))).collect::<Vec<_>>()).collect::<Vec<_>>()}),
    )
}
