use std::collections::BTreeMap;
use std::env;
use std::fs;

const P: u64 = 2_147_483_647;
const A: u64 = 2_147_483_644;
const B: u64 = 21;
const NV: usize = 7;
const SA: usize = 0;
const PA: usize = 1;
const SB: usize = 2;
const PB: usize = 3;
const R: usize = 4;
const Z: usize = 5;
const W: usize = 6;
const LIMIT: usize = 262_144;

fn add(a: u64, b: u64) -> u64 {
    let value = a + b;
    if value >= P {
        value - P
    } else {
        value
    }
}

fn neg(a: u64) -> u64 {
    if a == 0 {
        0
    } else {
        P - a
    }
}

fn sub(a: u64, b: u64) -> u64 {
    add(a, neg(b))
}

fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}

fn power(mut base: u64, mut exponent: u16) -> u64 {
    let mut result = 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = mul(result, base);
        }
        exponent >>= 1;
        if exponent > 0 {
            base = mul(base, base);
        }
    }
    result
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Monomial([u16; NV]);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Polynomial(BTreeMap<Monomial, u64>);

impl Polynomial {
    fn zero() -> Self {
        Self(BTreeMap::new())
    }

    fn constant(value: u64) -> Self {
        let mut terms = BTreeMap::new();
        if value != 0 {
            terms.insert(Monomial([0; NV]), value);
        }
        Self(terms)
    }

    fn variable(index: usize) -> Self {
        let mut exponents = [0; NV];
        exponents[index] = 1;
        let mut terms = BTreeMap::new();
        terms.insert(Monomial(exponents), 1);
        Self(terms)
    }

    fn negative(&self) -> Self {
        Self(self.0.iter().map(|(&m, &c)| (m, neg(c))).collect())
    }

    fn plus(&self, other: &Self) -> Result<Self, String> {
        let mut terms = self.0.clone();
        for (&monomial, &coefficient) in &other.0 {
            let value = add(terms.get(&monomial).copied().unwrap_or(0), coefficient);
            if value == 0 {
                terms.remove(&monomial);
            } else {
                if !terms.contains_key(&monomial) && terms.len() >= LIMIT {
                    return Err("verifier polynomial cap".into());
                }
                terms.insert(monomial, value);
            }
        }
        Ok(Self(terms))
    }

    fn minus(&self, other: &Self) -> Result<Self, String> {
        self.plus(&other.negative())
    }

    fn times(&self, other: &Self) -> Result<Self, String> {
        let mut terms = BTreeMap::new();
        for (&Monomial(x), &cx) in &self.0 {
            for (&Monomial(y), &cy) in &other.0 {
                let mut exponents = [0; NV];
                for i in 0..NV {
                    exponents[i] = x[i]
                        .checked_add(y[i])
                        .ok_or_else(|| "verifier exponent overflow".to_owned())?;
                }
                let monomial = Monomial(exponents);
                let value = add(terms.get(&monomial).copied().unwrap_or(0), mul(cx, cy));
                if value == 0 {
                    terms.remove(&monomial);
                } else {
                    if !terms.contains_key(&monomial) && terms.len() >= LIMIT {
                        return Err("verifier polynomial cap".into());
                    }
                    terms.insert(monomial, value);
                }
            }
        }
        Ok(Self(terms))
    }

    fn scaled(&self, scalar: u64) -> Self {
        if scalar == 0 {
            return Self::zero();
        }
        Self(
            self.0
                .iter()
                .filter_map(|(&m, &c)| {
                    let value = mul(c, scalar);
                    (value != 0).then_some((m, value))
                })
                .collect(),
        )
    }

    fn flipped(&self, variable: usize) -> Self {
        Self(
            self.0
                .iter()
                .map(|(&Monomial(e), &c)| {
                    (Monomial(e), if e[variable] & 1 == 1 { neg(c) } else { c })
                })
                .collect(),
        )
    }

    fn renamed(&self, from: usize, to: usize) -> Result<Self, String> {
        let mut terms = BTreeMap::new();
        for (&Monomial(mut e), &c) in &self.0 {
            if e[to] != 0 {
                return Err("verifier variable collision".into());
            }
            e[to] = e[from];
            e[from] = 0;
            terms.insert(Monomial(e), c);
        }
        Ok(Self(terms))
    }

    fn even_quotient(&self, variable: usize) -> Result<Self, String> {
        let mut terms = BTreeMap::new();
        for (&Monomial(mut e), &c) in &self.0 {
            if e[variable] & 1 == 1 {
                return Err("verifier odd remainder".into());
            }
            e[variable] /= 2;
            terms.insert(Monomial(e), c);
        }
        Ok(Self(terms))
    }

    fn compose(&self, variable: usize, replacement: &Self) -> Result<Self, String> {
        let maximum = self
            .0
            .keys()
            .map(|Monomial(e)| e[variable])
            .max()
            .unwrap_or(0);
        let mut powers = vec![Self::constant(1)];
        for i in 1..=maximum {
            powers.push(powers[(i - 1) as usize].times(replacement)?);
        }
        let mut result = Self::zero();
        for (&Monomial(mut e), &c) in &self.0 {
            let exponent = e[variable];
            e[variable] = 0;
            let mut coefficient_monomial = BTreeMap::new();
            coefficient_monomial.insert(Monomial(e), c);
            result = result.plus(&Self(coefficient_monomial).times(&powers[exponent as usize])?)?;
        }
        Ok(result)
    }

    fn evaluate(&self, values: &[u64; NV]) -> u64 {
        self.0.iter().fold(0, |acc, (Monomial(e), c)| {
            let term = (0..NV).fold(*c, |value, i| mul(value, power(values[i], e[i])));
            add(acc, term)
        })
    }

    fn digest(&self) -> u128 {
        let mut h = 0x6a09_e667_f3bc_c908_bb67_ae85_84ca_a73bu128;
        for (Monomial(exponents), coefficient) in &self.0 {
            h ^= *coefficient as u128;
            h = h.rotate_left(17).wrapping_mul(0x100000001b3);
            for exponent in exponents {
                h ^= *exponent as u128;
                h = h.rotate_left(11).wrapping_mul(0x9e3779b185ebca87);
            }
        }
        h
    }
}

fn sum(parts: &[Polynomial]) -> Result<Polynomial, String> {
    parts.iter().try_fold(Polynomial::zero(), |a, b| a.plus(b))
}

fn pair_coefficients(s: &Polynomial, p: &Polynomial) -> Result<[Polynomial; 3], String> {
    let c2 = s.times(s)?.minus(&p.scaled(4))?;
    let c1 = s
        .times(&p.plus(&Polynomial::constant(A))?)?
        .plus(&Polynomial::constant(mul(2, B)))?
        .scaled(neg(2));
    let p_minus_a = p.minus(&Polynomial::constant(A))?;
    let c0 = p_minus_a.times(&p_minus_a)?.minus(&s.scaled(mul(4, B)))?;
    Ok([c0, c1, c2])
}

fn vector_scale(values: &[Polynomial], scalar: &Polynomial) -> Result<Vec<Polynomial>, String> {
    values.iter().map(|x| x.times(scalar)).collect()
}

fn vector_sub(a: &[Polynomial], b: &[Polynomial]) -> Result<Vec<Polynomial>, String> {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| {
            a.get(i)
                .cloned()
                .unwrap_or_else(Polynomial::zero)
                .minus(b.get(i).unwrap_or(&Polynomial::zero()))
        })
        .collect()
}

fn convolution(a: &[Polynomial], b: &[Polynomial]) -> Result<Vec<Polynomial>, String> {
    let mut result = vec![Polynomial::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            result[i + j] = result[i + j].plus(&x.times(y)?)?;
        }
    }
    Ok(result)
}

fn s4(pair: &[Polynomial; 3], r: &Polynomial) -> Result<Vec<Polynomial>, String> {
    let r2 = r.times(r)?;
    let b2 = vec![r2.clone(), r.scaled(neg(2)), Polynomial::constant(1)];
    let b1 = vec![
        r.scaled(A)
            .plus(&Polynomial::constant(mul(2, B)))?
            .scaled(neg(2)),
        r2.plus(&Polynomial::constant(A))?.scaled(neg(2)),
        r.scaled(neg(2)),
    ];
    let b0 = vec![
        Polynomial::constant(mul(A, A)).minus(&r.scaled(mul(4, B)))?,
        r.scaled(mul(2, A))
            .plus(&Polynomial::constant(mul(4, B)))?
            .negative(),
        r2,
    ];
    let at = vector_sub(&vector_scale(&b0, &pair[2])?, &vector_scale(&b2, &pair[0])?)?;
    let bt = vector_sub(&vector_scale(&b1, &pair[2])?, &vector_scale(&b2, &pair[1])?)?;
    let ct = vector_sub(&vector_scale(&b0, &pair[1])?, &vector_scale(&b1, &pair[0])?)?;
    vector_sub(&convolution(&at, &at)?, &convolution(&bt, &ct)?)
}

fn permutation_sign(permutation: &[usize]) -> bool {
    let mut odd = false;
    for i in 0..permutation.len() {
        for j in i + 1..permutation.len() {
            odd ^= permutation[i] > permutation[j];
        }
    }
    odd
}

fn enumerate_permutations<F: FnMut(&[usize])>(values: &mut [usize], at: usize, f: &mut F) {
    if at == values.len() {
        f(values);
        return;
    }
    for i in at..values.len() {
        values.swap(at, i);
        enumerate_permutations(values, at + 1, f);
        values.swap(at, i);
    }
}

fn determinant_permutations(matrix: &[Vec<Polynomial>]) -> Result<Polynomial, String> {
    let mut permutation: Vec<_> = (0..matrix.len()).collect();
    let mut terms = Vec::with_capacity(720);
    enumerate_permutations(&mut permutation, 0, &mut |columns| {
        let mut product = Polynomial::constant(1);
        for (row, &column) in columns.iter().enumerate() {
            product = product
                .times(&matrix[row][column])
                .expect("determinant term within cap");
        }
        if permutation_sign(columns) {
            product = product.negative();
        }
        terms.push(product);
    });
    terms.iter().try_fold(Polynomial::zero(), |a, b| a.plus(b))
}

fn expected_h() -> Result<Polynomial, String> {
    let left = pair_coefficients(&Polynomial::variable(SA), &Polynomial::variable(PA))?;
    let right = pair_coefficients(&Polynomial::variable(SB), &Polynomial::variable(PB))?;
    let quartic = s4(&right, &Polynomial::variable(R))?;
    let mut matrix = vec![vec![Polynomial::zero(); 6]; 6];
    for shift in 0..2 {
        for i in 0..5 {
            matrix[shift][shift + i] = quartic[4 - i].clone();
        }
    }
    for shift in 0..4 {
        for i in 0..3 {
            matrix[2 + shift][shift + i] = left[2 - i].clone();
        }
    }
    determinant_permutations(&matrix)
}

fn expected_norm(input: &Polynomial) -> Result<Polynomial, String> {
    let renamed = input.renamed(PB, W)?.renamed(SB, Z)?;
    let even = renamed.times(&renamed.flipped(Z))?.even_quotient(Z)?;
    let lower_s = Polynomial::variable(SB);
    let w = Polynomial::variable(W);
    let plus = even.compose(
        Z,
        &sum(&[lower_s.clone(), w.scaled(2), Polynomial::constant(4)])?,
    )?;
    let minus = even.flipped(W).compose(
        Z,
        &sum(&[lower_s.clone(), w.scaled(neg(2)), Polynomial::constant(4)])?,
    )?;
    plus.times(&minus)?.even_quotient(W)?.compose(
        W,
        &sum(&[
            Polynomial::variable(PB),
            lower_s.scaled(2),
            Polynomial::constant(4),
        ])?,
    )
}

#[derive(Clone, Debug)]
enum ParsedNode {
    Input(usize),
    Constant(u64),
    Add(usize, usize),
    Multiply(usize, usize),
    Negative(usize),
}

#[derive(Clone, Debug)]
struct DagCell {
    label: String,
    depth: usize,
    declared_nodes: usize,
    declared_edges: usize,
    declared_add: usize,
    declared_mul: usize,
    declared_neg: usize,
    root: usize,
    nodes: Vec<ParsedNode>,
}

type ParsedCandidate = (BTreeMap<(String, usize), Polynomial>, Vec<DagCell>);

fn field<'a>(line: &'a str, key: &str) -> Result<&'a str, String> {
    line.split_whitespace()
        .find_map(|part| part.strip_prefix(&format!("{key}=")))
        .ok_or_else(|| format!("missing field {key}: {line}"))
}

fn parse_candidate(text: &str) -> Result<ParsedCandidate, String> {
    let lines: Vec<_> = text.lines().collect();
    if !lines
        .first()
        .copied()
        .unwrap_or("")
        .starts_with("HEADER schema=pplus-pair-divisor-transfer-candidate-v1")
    {
        return Err("candidate header missing".into());
    }
    if !lines
        .last()
        .copied()
        .unwrap_or("")
        .contains("status=PRODUCER_COMPLETE")
    {
        return Err("candidate footer missing".into());
    }
    let mut polynomials = BTreeMap::new();
    let mut dags = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("POLY_BEGIN ") {
            let label = field(line, "label")?.to_owned();
            let depth = field(line, "depth")?
                .parse()
                .map_err(|_| "bad poly depth")?;
            let declared_slots: usize = field(line, "slots")?.parse().map_err(|_| "bad slots")?;
            let declared_digest =
                u128::from_str_radix(field(line, "digest")?, 16).map_err(|_| "bad digest")?;
            let mut terms = BTreeMap::new();
            i += 1;
            while i < lines.len() && lines[i].starts_with("P ") {
                let fields: Vec<_> = lines[i].split_whitespace().collect();
                if fields.len() != 9 {
                    return Err("bad polynomial row".into());
                }
                let mut exponents = [0; NV];
                for j in 0..NV {
                    exponents[j] = fields[j + 1].parse().map_err(|_| "bad exponent")?;
                }
                let coefficient = fields[8].parse().map_err(|_| "bad coefficient")?;
                if coefficient == 0 || coefficient >= P {
                    return Err("noncanonical coefficient".into());
                }
                if terms.insert(Monomial(exponents), coefficient).is_some() {
                    return Err("duplicate polynomial monomial".into());
                }
                i += 1;
            }
            let polynomial = Polynomial(terms);
            if polynomial.0.len() != declared_slots || polynomial.digest() != declared_digest {
                return Err(format!("polynomial metadata mismatch {label}/{depth}"));
            }
            if i >= lines.len() || lines[i] != format!("POLY_END label={label} depth={depth}") {
                return Err("polynomial end mismatch".into());
            }
            polynomials.insert((label, depth), polynomial);
        } else if line.starts_with("DAG_BEGIN ") {
            let label = field(line, "label")?.to_owned();
            let depth = field(line, "depth")?.parse().map_err(|_| "bad dag depth")?;
            let mut cell = DagCell {
                label: label.clone(),
                depth,
                declared_nodes: field(line, "nodes")?.parse().map_err(|_| "bad nodes")?,
                declared_edges: field(line, "edges")?.parse().map_err(|_| "bad edges")?,
                declared_add: field(line, "add")?.parse().map_err(|_| "bad adds")?,
                declared_mul: field(line, "mul")?.parse().map_err(|_| "bad muls")?,
                declared_neg: field(line, "neg")?.parse().map_err(|_| "bad negs")?,
                root: field(line, "root")?.parse().map_err(|_| "bad root")?,
                nodes: Vec::new(),
            };
            i += 1;
            while i < lines.len() && lines[i].starts_with("D ") {
                let f: Vec<_> = lines[i].split_whitespace().collect();
                let id: usize = f
                    .get(1)
                    .ok_or("node id missing")?
                    .parse()
                    .map_err(|_| "bad id")?;
                if id != cell.nodes.len() {
                    return Err("nonsequential node id".into());
                }
                let node = match *f.get(2).ok_or("node op missing")? {
                    "I" => ParsedNode::Input(f[3].parse().map_err(|_| "bad input")?),
                    "C" => ParsedNode::Constant(f[3].parse().map_err(|_| "bad constant")?),
                    "A" => ParsedNode::Add(
                        f[3].parse().map_err(|_| "bad add")?,
                        f[4].parse().map_err(|_| "bad add")?,
                    ),
                    "M" => ParsedNode::Multiply(
                        f[3].parse().map_err(|_| "bad mul")?,
                        f[4].parse().map_err(|_| "bad mul")?,
                    ),
                    "N" => ParsedNode::Negative(f[3].parse().map_err(|_| "bad neg")?),
                    _ => return Err("unknown node operation".into()),
                };
                cell.nodes.push(node);
                i += 1;
            }
            if i >= lines.len() || lines[i] != format!("DAG_END label={label} depth={depth}") {
                return Err("dag end mismatch".into());
            }
            dags.push(cell);
        }
        i += 1;
    }
    Ok((polynomials, dags))
}

fn check_dag(cell: &DagCell) -> Result<(), String> {
    if cell.nodes.len() != cell.declared_nodes || cell.root >= cell.nodes.len() {
        return Err(format!(
            "dag size/root mismatch {}/{}",
            cell.label, cell.depth
        ));
    }
    let mut adds = 0;
    let mut muls = 0;
    let mut negs = 0;
    let mut edges = 0;
    for (id, node) in cell.nodes.iter().enumerate() {
        match *node {
            ParsedNode::Input(index) => {
                if index >= NV {
                    return Err("dag input out of range".into());
                }
            }
            ParsedNode::Constant(c) => {
                if c >= P {
                    return Err("dag constant out of range".into());
                }
            }
            ParsedNode::Add(a, b) => {
                if a >= id || b >= id || a > b {
                    return Err("noncanonical dag add".into());
                }
                adds += 1;
                edges += 2;
            }
            ParsedNode::Multiply(a, b) => {
                if a >= id || b >= id || a > b {
                    return Err("noncanonical dag multiply".into());
                }
                muls += 1;
                edges += 2;
            }
            ParsedNode::Negative(a) => {
                if a >= id {
                    return Err("non-topological dag neg".into());
                }
                negs += 1;
                edges += 1;
            }
        }
    }
    if (adds, muls, negs, edges)
        != (
            cell.declared_add,
            cell.declared_mul,
            cell.declared_neg,
            cell.declared_edges,
        )
    {
        return Err(format!(
            "dag metadata mismatch {}/{}",
            cell.label, cell.depth
        ));
    }
    Ok(())
}

fn eval_dag(cell: &DagCell, inputs: &[u64; NV]) -> u64 {
    let mut values = Vec::with_capacity(cell.nodes.len());
    for node in &cell.nodes {
        let value = match *node {
            ParsedNode::Input(i) => inputs[i],
            ParsedNode::Constant(c) => c,
            ParsedNode::Add(a, b) => add(values[a], values[b]),
            ParsedNode::Multiply(a, b) => mul(values[a], values[b]),
            ParsedNode::Negative(a) => neg(values[a]),
        };
        values.push(value);
    }
    values[cell.root]
}

type NumericRw = [u64; 2];
type NumericElem = [u64; 4];

fn numeric_rw_add(a: NumericRw, b: NumericRw) -> NumericRw {
    [add(a[0], b[0]), add(a[1], b[1])]
}

fn numeric_rw_sub(a: NumericRw, b: NumericRw) -> NumericRw {
    [add(a[0], neg(b[0])), add(a[1], neg(b[1]))]
}

fn numeric_rw_mul(a: NumericRw, b: NumericRw, k: u64) -> NumericRw {
    let u = mul(a[0], b[0]);
    let v = mul(a[1], b[1]);
    let cross = sub(sub(mul(add(a[0], a[1]), add(b[0], b[1])), u), v);
    [add(u, mul(k, v)), cross]
}

fn numeric_q_mul(value: NumericRw, s4: u64, k: u64) -> NumericRw {
    [
        add(mul(s4, value[0]), mul(mul(2, k), value[1])),
        add(mul(2, value[0]), mul(s4, value[1])),
    ]
}

fn numeric_elem_add(a: NumericElem, b: NumericElem) -> NumericElem {
    [
        add(a[0], b[0]),
        add(a[1], b[1]),
        add(a[2], b[2]),
        add(a[3], b[3]),
    ]
}

fn numeric_elem_neg(a: NumericElem) -> NumericElem {
    [neg(a[0]), neg(a[1]), neg(a[2]), neg(a[3])]
}

fn numeric_elem_mul(a: NumericElem, b: NumericElem, s4: u64, k: u64) -> NumericElem {
    let a0 = [a[0], a[1]];
    let a1 = [a[2], a[3]];
    let b0 = [b[0], b[1]];
    let b1 = [b[2], b[3]];
    let u = numeric_rw_mul(a0, b0, k);
    let v = numeric_rw_mul(a1, b1, k);
    let cross = numeric_rw_sub(
        numeric_rw_sub(
            numeric_rw_mul(numeric_rw_add(a0, a1), numeric_rw_add(b0, b1), k),
            u,
        ),
        v,
    );
    let constant = numeric_rw_add(u, numeric_q_mul(v, s4, k));
    [constant[0], constant[1], cross[0], cross[1]]
}

fn independently_norm_dag(
    cell: &DagCell,
    inputs: &[u64; NV],
    s_index: usize,
    pair_index: usize,
) -> u64 {
    let s = inputs[s_index];
    let pair = inputs[pair_index];
    let k = add(add(pair, mul(2, s)), 4);
    let s4 = add(s, 4);
    let mut values = Vec::with_capacity(cell.nodes.len());
    for node in &cell.nodes {
        let value = match *node {
            ParsedNode::Input(index) if index == s_index => [0, 0, 1, 0],
            ParsedNode::Input(index) if index == pair_index => [0, 1, 0, 0],
            ParsedNode::Input(index) => [inputs[index], 0, 0, 0],
            ParsedNode::Constant(c) => [c, 0, 0, 0],
            ParsedNode::Add(a, b) => numeric_elem_add(values[a], values[b]),
            ParsedNode::Multiply(a, b) => numeric_elem_mul(values[a], values[b], s4, k),
            ParsedNode::Negative(a) => numeric_elem_neg(values[a]),
        };
        values.push(value);
    }
    let value = values[cell.root];
    let a = [value[0], value[1]];
    let b = [value[2], value[3]];
    let g = numeric_rw_sub(
        numeric_rw_mul(a, a, k),
        numeric_q_mul(numeric_rw_mul(b, b, k), s4, k),
    );
    sub(mul(g[0], g[0]), mul(k, mul(g[1], g[1])))
}

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: pair-transfer-verify CANDIDATE.txt");
        std::process::exit(2);
    });
    let text = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("cannot read {path}: {error}");
        std::process::exit(2);
    });
    let mut errors = Vec::new();
    let (candidate_polynomials, dags) = match parse_candidate(&text) {
        Ok(value) => value,
        Err(error) => {
            println!("VERIFY schema=pplus-pair-divisor-transfer-verify-v1 exactness_ok=0 errors=1 detail={error:?}");
            std::process::exit(1);
        }
    };

    let mut expected_cells = BTreeMap::new();
    match expected_h() {
        Ok(mut expected) => {
            expected_cells.insert(("h".to_owned(), 0), expected.clone());
            for depth in 1..=6 {
                if !candidate_polynomials.contains_key(&("second_pair_norm".to_owned(), depth)) {
                    break;
                }
                match expected_norm(&expected) {
                    Ok(next) => {
                        expected = next;
                        expected_cells
                            .insert(("second_pair_norm".to_owned(), depth), expected.clone());
                    }
                    Err(error) => {
                        errors.push(format!("independent norm depth {depth}: {error}"));
                        break;
                    }
                }
            }
        }
        Err(error) => errors.push(format!("independent H construction: {error}")),
    }

    for (key, expected) in &expected_cells {
        match candidate_polynomials.get(key) {
            Some(actual) if actual == expected => {}
            Some(_) => errors.push(format!("polynomial mismatch {key:?}")),
            None => errors.push(format!("polynomial missing {key:?}")),
        }
    }

    let assignments: Vec<[u64; NV]> = (0..9)
        .map(|i| {
            let base = 3 + i as u64 * 17;
            [base, base + 1, base + 2, base + 3, base + 5, 0, 0]
        })
        .collect();
    let mut dag_eval_checks = 0usize;
    for cell in &dags {
        if let Err(error) = check_dag(cell) {
            errors.push(error);
            continue;
        }
        if let Some(polynomial) = expected_cells.get(&(cell.label.clone(), cell.depth)) {
            for assignment in &assignments {
                let left = eval_dag(cell, assignment);
                let right = polynomial.evaluate(assignment);
                dag_eval_checks += 1;
                if left != right {
                    errors.push(format!(
                        "dag evaluation mismatch {}/{}",
                        cell.label, cell.depth
                    ));
                    break;
                }
            }
        }
    }

    let dag_index: BTreeMap<_, _> = dags
        .iter()
        .map(|cell| ((cell.label.clone(), cell.depth), cell))
        .collect();
    let mut dag_norm_checks = 0usize;
    for current in dags.iter().filter(|cell| cell.label == "second_pair_norm") {
        let previous_key = if current.depth == 1 {
            ("h".to_owned(), 0)
        } else {
            ("second_pair_norm".to_owned(), current.depth - 1)
        };
        let Some(previous) = dag_index.get(&previous_key) else {
            errors.push(format!(
                "missing predecessor DAG for depth {}",
                current.depth
            ));
            continue;
        };
        for assignment in &assignments {
            let expected = independently_norm_dag(previous, assignment, SB, PB);
            let actual = eval_dag(current, assignment);
            dag_norm_checks += 1;
            if actual != expected {
                errors.push(format!(
                    "independent DAG norm mismatch depth {}",
                    current.depth
                ));
                break;
            }
        }
    }

    let exactness_ok = usize::from(errors.is_empty());
    println!(
        "VERIFY schema=pplus-pair-divisor-transfer-verify-v2 exactness_ok={exactness_ok} poly_cells={} dag_cells={} dag_eval_checks={} dag_norm_checks={} errors={}",
        expected_cells.len(), dags.len(), dag_eval_checks, dag_norm_checks, errors.len()
    );
    for error in &errors {
        println!("ERROR {error}");
    }
    if !errors.is_empty() {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_parity_is_correct() {
        assert!(!permutation_sign(&[0, 1, 2]));
        assert!(permutation_sign(&[1, 0, 2]));
        assert!(!permutation_sign(&[2, 0, 1]));
    }

    #[test]
    fn independent_even_quotient_rejects_odd() {
        assert!(Polynomial::variable(Z).even_quotient(Z).is_err());
    }

    #[test]
    fn subtraction_wraps() {
        assert_eq!(add(0, neg(1)), P - 1);
    }
}
