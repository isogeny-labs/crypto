use std::collections::{BTreeMap, HashMap};

const MODULUS: u64 = 2_147_483_647;
const CURVE_A: u64 = 2_147_483_644;
const CURVE_B: u64 = 21;
const TARGET_G1_X: u64 = 1_391_443_431;
const TARGET_G17_X: u64 = 1_960_828_035;
const SLOT_CAP: usize = 262_144;
const NODE_CAP: usize = 262_144;
const VARS: usize = 7;
const SA: usize = 0;
const PA: usize = 1;
const SB: usize = 2;
const PB: usize = 3;
const TARGET_R: usize = 4;
const AUX_Z: usize = 5;
const AUX_W: usize = 6;

fn fadd(a: u64, b: u64) -> u64 {
    let x = a + b;
    if x >= MODULUS {
        x - MODULUS
    } else {
        x
    }
}

fn fneg(a: u64) -> u64 {
    if a == 0 {
        0
    } else {
        MODULUS - a
    }
}

fn fmul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % MODULUS as u128) as u64
}

#[derive(Clone, Copy, Debug, Default)]
struct Counts {
    add: u64,
    mul: u64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Exp([u16; VARS]);

#[derive(Clone, Debug)]
struct Poly {
    terms: BTreeMap<Exp, u64>,
}

#[derive(Clone, Copy, Debug)]
enum PolyStop {
    Cap,
    Exponent,
    Odd,
    VariableCollision,
}

type PResult<T> = Result<T, PolyStop>;

impl Poly {
    fn zero() -> Self {
        Self {
            terms: BTreeMap::new(),
        }
    }

    fn constant(c: u64) -> Self {
        if c == 0 {
            Self::zero()
        } else {
            let mut terms = BTreeMap::new();
            terms.insert(Exp([0; VARS]), c);
            Self { terms }
        }
    }

    fn variable(index: usize) -> Self {
        let mut exponent = [0; VARS];
        exponent[index] = 1;
        let mut terms = BTreeMap::new();
        terms.insert(Exp(exponent), 1);
        Self { terms }
    }

    fn neg(&self) -> Self {
        Self {
            terms: self.terms.iter().map(|(&e, &c)| (e, fneg(c))).collect(),
        }
    }

    fn add(&self, other: &Self, counts: &mut Counts) -> PResult<Self> {
        let mut out = self.terms.clone();
        for (&e, &c) in &other.terms {
            counts.add += 1;
            let value = fadd(out.get(&e).copied().unwrap_or(0), c);
            if value == 0 {
                out.remove(&e);
            } else {
                if !out.contains_key(&e) && out.len() >= SLOT_CAP {
                    return Err(PolyStop::Cap);
                }
                out.insert(e, value);
            }
        }
        Ok(Self { terms: out })
    }

    fn sub(&self, other: &Self, counts: &mut Counts) -> PResult<Self> {
        self.add(&other.neg(), counts)
    }

    fn scale(&self, scalar: u64, counts: &mut Counts) -> Self {
        if scalar == 0 {
            return Self::zero();
        }
        if scalar == 1 {
            return self.clone();
        }
        Self {
            terms: self
                .terms
                .iter()
                .filter_map(|(&e, &c)| {
                    counts.mul += 1;
                    let value = fmul(c, scalar);
                    (value != 0).then_some((e, value))
                })
                .collect(),
        }
    }

    fn mul(&self, other: &Self, counts: &mut Counts) -> PResult<Self> {
        if self.terms.is_empty() || other.terms.is_empty() {
            return Ok(Self::zero());
        }
        let mut out = BTreeMap::<Exp, u64>::new();
        for (&Exp(ae), &ac) in &self.terms {
            for (&Exp(be), &bc) in &other.terms {
                let mut exponent = [0; VARS];
                for i in 0..VARS {
                    exponent[i] = ae[i].checked_add(be[i]).ok_or(PolyStop::Exponent)?;
                }
                counts.mul += 1;
                let product = fmul(ac, bc);
                let key = Exp(exponent);
                counts.add += u64::from(out.contains_key(&key));
                let value = fadd(out.get(&key).copied().unwrap_or(0), product);
                if value == 0 {
                    out.remove(&key);
                } else {
                    if !out.contains_key(&key) && out.len() >= SLOT_CAP {
                        return Err(PolyStop::Cap);
                    }
                    out.insert(key, value);
                }
            }
        }
        Ok(Self { terms: out })
    }

    fn sign_flip(&self, variable: usize) -> Self {
        Self {
            terms: self
                .terms
                .iter()
                .map(|(&Exp(e), &c)| {
                    let value = if e[variable] & 1 == 1 { fneg(c) } else { c };
                    (Exp(e), value)
                })
                .collect(),
        }
    }

    fn rename(&self, from: usize, to: usize) -> PResult<Self> {
        let mut out = BTreeMap::new();
        for (&Exp(mut e), &c) in &self.terms {
            if e[to] != 0 {
                return Err(PolyStop::VariableCollision);
            }
            e[to] = e[from];
            e[from] = 0;
            out.insert(Exp(e), c);
        }
        Ok(Self { terms: out })
    }

    fn halve_even(&self, variable: usize) -> PResult<Self> {
        let mut out = BTreeMap::new();
        for (&Exp(mut e), &c) in &self.terms {
            if e[variable] & 1 == 1 {
                return Err(PolyStop::Odd);
            }
            e[variable] /= 2;
            out.insert(Exp(e), c);
        }
        Ok(Self { terms: out })
    }

    fn substitute(
        &self,
        variable: usize,
        replacement: &Poly,
        counts: &mut Counts,
    ) -> PResult<Self> {
        let max_power = self
            .terms
            .keys()
            .map(|Exp(e)| e[variable])
            .max()
            .unwrap_or(0);
        let mut powers = Vec::with_capacity(max_power as usize + 1);
        powers.push(Poly::constant(1));
        for power in 1..=max_power {
            let next = powers[(power - 1) as usize].mul(replacement, counts)?;
            powers.push(next);
        }
        let mut out = Poly::zero();
        for (&Exp(mut e), &c) in &self.terms {
            let power = e[variable];
            e[variable] = 0;
            let mut base = Poly::zero();
            base.terms.insert(Exp(e), c);
            let term = base.mul(&powers[power as usize], counts)?;
            out = out.add(&term, counts)?;
        }
        Ok(out)
    }

    fn digest(&self) -> u128 {
        let mut h = 0x6a09_e667_f3bc_c908_bb67_ae85_84ca_a73bu128;
        for (Exp(exponents), coefficient) in &self.terms {
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

fn pconst(value: i64) -> Poly {
    let reduced = if value >= 0 {
        value as u64 % MODULUS
    } else {
        fneg((-value) as u64 % MODULUS)
    };
    Poly::constant(reduced)
}

fn poly_sum(parts: &[Poly], counts: &mut Counts) -> PResult<Poly> {
    let mut out = Poly::zero();
    for part in parts {
        out = out.add(part, counts)?;
    }
    Ok(out)
}

fn pair_c(s: &Poly, pair: &Poly, counts: &mut Counts) -> PResult<[Poly; 3]> {
    let c2 = s.mul(s, counts)?.sub(&pair.scale(4, counts), counts)?;
    let pair_plus_a = pair.add(&Poly::constant(CURVE_A), counts)?;
    let inner = s
        .mul(&pair_plus_a, counts)?
        .add(&Poly::constant(fmul(2, CURVE_B)), counts)?;
    let c1 = inner.scale(fneg(2), counts);
    let pair_minus_a = pair.sub(&Poly::constant(CURVE_A), counts)?;
    let c0 = pair_minus_a
        .mul(&pair_minus_a, counts)?
        .sub(&s.scale(fmul(4, CURVE_B), counts), counts)?;
    Ok([c0, c1, c2])
}

fn array_add(a: &[Poly], b: &[Poly], counts: &mut Counts) -> PResult<Vec<Poly>> {
    let n = a.len().max(b.len());
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(
            a.get(i)
                .cloned()
                .unwrap_or_else(Poly::zero)
                .add(b.get(i).unwrap_or(&Poly::zero()), counts)?,
        );
    }
    Ok(out)
}

fn array_sub(a: &[Poly], b: &[Poly], counts: &mut Counts) -> PResult<Vec<Poly>> {
    let negative: Vec<_> = b.iter().map(Poly::neg).collect();
    array_add(a, &negative, counts)
}

fn array_scale(a: &[Poly], scalar: &Poly, counts: &mut Counts) -> PResult<Vec<Poly>> {
    a.iter().map(|x| x.mul(scalar, counts)).collect()
}

fn array_mul(a: &[Poly], b: &[Poly], counts: &mut Counts) -> PResult<Vec<Poly>> {
    let mut out = vec![Poly::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            let product = x.mul(y, counts)?;
            out[i + j] = out[i + j].add(&product, counts)?;
        }
    }
    Ok(out)
}

fn s4_coefficients(pair: &[Poly; 3], r: &Poly, counts: &mut Counts) -> PResult<Vec<Poly>> {
    let r2 = r.mul(r, counts)?;
    let b2 = vec![r2.clone(), r.scale(fneg(2), counts), Poly::constant(1)];
    let b1 = vec![
        r.scale(CURVE_A, counts)
            .add(&Poly::constant(fmul(2, CURVE_B)), counts)?
            .scale(fneg(2), counts),
        r2.add(&Poly::constant(CURVE_A), counts)?
            .scale(fneg(2), counts),
        r.scale(fneg(2), counts),
    ];
    let b0 = vec![
        Poly::constant(fmul(CURVE_A, CURVE_A)).sub(&r.scale(fmul(4, CURVE_B), counts), counts)?,
        r.scale(fmul(2, CURVE_A), counts)
            .add(&Poly::constant(fmul(4, CURVE_B)), counts)?
            .neg(),
        r2,
    ];
    let at = array_sub(
        &array_scale(&b0, &pair[2], counts)?,
        &array_scale(&b2, &pair[0], counts)?,
        counts,
    )?;
    let bt = array_sub(
        &array_scale(&b1, &pair[2], counts)?,
        &array_scale(&b2, &pair[1], counts)?,
        counts,
    )?;
    let ct = array_sub(
        &array_scale(&b0, &pair[1], counts)?,
        &array_scale(&b1, &pair[0], counts)?,
        counts,
    )?;
    array_sub(
        &array_mul(&at, &at, counts)?,
        &array_mul(&bt, &ct, counts)?,
        counts,
    )
}

fn sylvester_4_2(quartic: &[Poly], quadratic: &[Poly]) -> Vec<Vec<Poly>> {
    let mut matrix = vec![vec![Poly::zero(); 6]; 6];
    for shift in 0..2 {
        for i in 0..5 {
            matrix[shift][shift + i] = quartic[4 - i].clone();
        }
    }
    for shift in 0..4 {
        for i in 0..3 {
            matrix[2 + shift][shift + i] = quadratic[2 - i].clone();
        }
    }
    matrix
}

fn determinant_subset(matrix: &[Vec<Poly>], counts: &mut Counts) -> PResult<Poly> {
    let n = matrix.len();
    let mut dp = vec![Poly::zero(); 1 << n];
    dp[0] = Poly::constant(1);
    for mask in 0usize..(1usize << n) {
        let row = mask.count_ones() as usize;
        if row >= n || dp[mask].terms.is_empty() {
            continue;
        }
        for (column, entry) in matrix[row].iter().enumerate() {
            if mask & (1 << column) != 0 {
                continue;
            }
            let selected_after = (mask >> (column + 1)).count_ones();
            let mut term = dp[mask].mul(entry, counts)?;
            if selected_after & 1 == 1 {
                term = term.neg();
            }
            let next = mask | (1 << column);
            dp[next] = dp[next].add(&term, counts)?;
        }
    }
    Ok(dp[(1 << n) - 1].clone())
}

fn build_h_poly() -> PResult<(Poly, Counts)> {
    let mut counts = Counts::default();
    let ca = pair_c(&Poly::variable(SA), &Poly::variable(PA), &mut counts)?;
    let cb = pair_c(&Poly::variable(SB), &Poly::variable(PB), &mut counts)?;
    let s4 = s4_coefficients(&cb, &Poly::variable(TARGET_R), &mut counts)?;
    let h = determinant_subset(&sylvester_4_2(&s4, &ca), &mut counts)?;
    Ok((h, counts))
}

fn pair_norm_poly(input: &Poly, s: usize, pair: usize, counts: &mut Counts) -> PResult<Poly> {
    if input
        .terms
        .keys()
        .any(|Exp(e)| e[AUX_Z] != 0 || e[AUX_W] != 0)
    {
        return Err(PolyStop::VariableCollision);
    }
    let renamed = input.rename(pair, AUX_W)?.rename(s, AUX_Z)?;
    let conjugate_z = renamed.sign_flip(AUX_Z);
    let even = renamed.mul(&conjugate_z, counts)?.halve_even(AUX_Z)?;

    let lower_s = Poly::variable(s);
    let w = Poly::variable(AUX_W);
    let plus_q = poly_sum(&[lower_s.clone(), w.scale(2, counts), pconst(4)], counts)?;
    let minus_q = poly_sum(
        &[lower_s.clone(), w.scale(fneg(2), counts), pconst(4)],
        counts,
    )?;
    let plus = even.substitute(AUX_Z, &plus_q, counts)?;
    let minus = even.sign_flip(AUX_W).substitute(AUX_Z, &minus_q, counts)?;
    let product = plus.mul(&minus, counts)?.halve_even(AUX_W)?;
    let w2 = poly_sum(
        &[Poly::variable(pair), lower_s.scale(2, counts), pconst(4)],
        counts,
    )?;
    product.substitute(AUX_W, &w2, counts)
}

fn emit_poly(label: &str, depth: usize, poly: &Poly, counts: Counts) {
    println!(
        "POLY_BEGIN label={label} depth={depth} slots={} add={} mul={} digest={:032x}",
        poly.terms.len(),
        counts.add,
        counts.mul,
        poly.digest()
    );
    for (Exp(e), coefficient) in &poly.terms {
        println!(
            "P {} {} {} {} {} {} {} {}",
            e[0], e[1], e[2], e[3], e[4], e[5], e[6], coefficient
        );
    }
    println!("POLY_END label={label} depth={depth}");
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Node {
    Input(u8),
    Const(u64),
    Add(usize, usize),
    Mul(usize, usize),
    Neg(usize),
}

#[derive(Clone, Debug)]
struct Dag {
    nodes: Vec<Node>,
    index: HashMap<Node, usize>,
    stopped: bool,
}

impl Dag {
    fn new() -> Self {
        let mut dag = Self {
            nodes: Vec::new(),
            index: HashMap::new(),
            stopped: false,
        };
        dag.intern_raw(Node::Const(0));
        dag.intern_raw(Node::Const(1));
        dag
    }

    fn intern_raw(&mut self, node: Node) -> usize {
        if let Some(&id) = self.index.get(&node) {
            return id;
        }
        if self.nodes.len() >= NODE_CAP {
            self.stopped = true;
            return 0;
        }
        let id = self.nodes.len();
        self.nodes.push(node.clone());
        self.index.insert(node, id);
        id
    }

    fn constant(&mut self, value: u64) -> usize {
        self.intern_raw(Node::Const(value))
    }

    fn input(&mut self, index: usize) -> usize {
        self.intern_raw(Node::Input(index as u8))
    }

    fn neg(&mut self, value: usize) -> usize {
        match self.nodes[value].clone() {
            Node::Const(c) => self.constant(fneg(c)),
            Node::Neg(inner) => inner,
            _ => self.intern_raw(Node::Neg(value)),
        }
    }

    fn add(&mut self, left: usize, right: usize) -> usize {
        if left == 0 {
            return right;
        }
        if right == 0 {
            return left;
        }
        match (self.nodes[left].clone(), self.nodes[right].clone()) {
            (Node::Const(a), Node::Const(b)) => self.constant(fadd(a, b)),
            _ => {
                let (a, b) = if left <= right {
                    (left, right)
                } else {
                    (right, left)
                };
                self.intern_raw(Node::Add(a, b))
            }
        }
    }

    fn sub(&mut self, left: usize, right: usize) -> usize {
        let negative = self.neg(right);
        self.add(left, negative)
    }

    fn mul(&mut self, left: usize, right: usize) -> usize {
        if left == 0 || right == 0 {
            return 0;
        }
        if left == 1 {
            return right;
        }
        if right == 1 {
            return left;
        }
        match (self.nodes[left].clone(), self.nodes[right].clone()) {
            (Node::Const(a), Node::Const(b)) => self.constant(fmul(a, b)),
            _ => {
                let (a, b) = if left <= right {
                    (left, right)
                } else {
                    (right, left)
                };
                self.intern_raw(Node::Mul(a, b))
            }
        }
    }

    fn scale(&mut self, value: usize, scalar: u64) -> usize {
        let c = self.constant(scalar);
        self.mul(value, c)
    }

    fn compact(&self, root: usize) -> (Dag, usize) {
        fn clone_node(
            source: &Dag,
            old: usize,
            dest: &mut Dag,
            map: &mut HashMap<usize, usize>,
        ) -> usize {
            if let Some(&id) = map.get(&old) {
                return id;
            }
            let id = match source.nodes[old].clone() {
                Node::Input(i) => dest.input(i as usize),
                Node::Const(c) => dest.constant(c),
                Node::Add(a, b) => {
                    let x = clone_node(source, a, dest, map);
                    let y = clone_node(source, b, dest, map);
                    dest.add(x, y)
                }
                Node::Mul(a, b) => {
                    let x = clone_node(source, a, dest, map);
                    let y = clone_node(source, b, dest, map);
                    dest.mul(x, y)
                }
                Node::Neg(a) => {
                    let x = clone_node(source, a, dest, map);
                    dest.neg(x)
                }
            };
            map.insert(old, id);
            id
        }
        let mut dest = Dag::new();
        let new_root = clone_node(self, root, &mut dest, &mut HashMap::new());
        (dest, new_root)
    }

    fn operation_counts(&self) -> (usize, usize, usize, usize) {
        let mut adds = 0;
        let mut muls = 0;
        let mut negs = 0;
        let mut edges = 0;
        for node in &self.nodes {
            match node {
                Node::Add(_, _) => {
                    adds += 1;
                    edges += 2;
                }
                Node::Mul(_, _) => {
                    muls += 1;
                    edges += 2;
                }
                Node::Neg(_) => {
                    negs += 1;
                    edges += 1;
                }
                _ => {}
            }
        }
        (adds, muls, negs, edges)
    }
}

type DPoly = Vec<usize>;

fn darray_add(dag: &mut Dag, a: &[usize], b: &[usize]) -> DPoly {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| dag.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
        .collect()
}

fn darray_sub(dag: &mut Dag, a: &[usize], b: &[usize]) -> DPoly {
    let negative: Vec<_> = b.iter().map(|&x| dag.neg(x)).collect();
    darray_add(dag, a, &negative)
}

fn darray_scale(dag: &mut Dag, a: &[usize], scalar: usize) -> DPoly {
    a.iter().map(|&x| dag.mul(x, scalar)).collect()
}

fn darray_mul(dag: &mut Dag, a: &[usize], b: &[usize]) -> DPoly {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            let product = dag.mul(x, y);
            out[i + j] = dag.add(out[i + j], product);
        }
    }
    out
}

fn pair_c_dag(dag: &mut Dag, s: usize, pair: usize) -> [usize; 3] {
    let s2 = dag.mul(s, s);
    let four_pair = dag.scale(pair, 4);
    let c2 = dag.sub(s2, four_pair);
    let a = dag.constant(CURVE_A);
    let pair_a = dag.add(pair, a);
    let s_pair_a = dag.mul(s, pair_a);
    let two_b = dag.constant(fmul(2, CURVE_B));
    let inner = dag.add(s_pair_a, two_b);
    let c1 = dag.scale(inner, fneg(2));
    let pair_minus_a = dag.sub(pair, a);
    let square = dag.mul(pair_minus_a, pair_minus_a);
    let four_b_s = dag.scale(s, fmul(4, CURVE_B));
    let c0 = dag.sub(square, four_b_s);
    [c0, c1, c2]
}

fn s4_dag(dag: &mut Dag, pair: &[usize; 3], r: usize) -> DPoly {
    let r2 = dag.mul(r, r);
    let minus_two_r = dag.scale(r, fneg(2));
    let one = dag.constant(1);
    let b2 = vec![r2, minus_two_r, one];

    let a = dag.constant(CURVE_A);
    let ar = dag.mul(a, r);
    let two_b = dag.constant(fmul(2, CURVE_B));
    let ar_two_b = dag.add(ar, two_b);
    let b10 = dag.scale(ar_two_b, fneg(2));
    let r2_a = dag.add(r2, a);
    let b11 = dag.scale(r2_a, fneg(2));
    let b12 = dag.scale(r, fneg(2));
    let b1 = vec![b10, b11, b12];

    let a2 = dag.constant(fmul(CURVE_A, CURVE_A));
    let four_b_r = dag.scale(r, fmul(4, CURVE_B));
    let b00 = dag.sub(a2, four_b_r);
    let two_a_r = dag.scale(r, fmul(2, CURVE_A));
    let four_b = dag.constant(fmul(4, CURVE_B));
    let b01_sum = dag.add(two_a_r, four_b);
    let b01 = dag.neg(b01_sum);
    let b0 = vec![b00, b01, r2];

    let a2b0 = darray_scale(dag, &b0, pair[2]);
    let a0b2 = darray_scale(dag, &b2, pair[0]);
    let at = darray_sub(dag, &a2b0, &a0b2);
    let a2b1 = darray_scale(dag, &b1, pair[2]);
    let a1b2 = darray_scale(dag, &b2, pair[1]);
    let bt = darray_sub(dag, &a2b1, &a1b2);
    let a1b0 = darray_scale(dag, &b0, pair[1]);
    let a0b1 = darray_scale(dag, &b1, pair[0]);
    let ct = darray_sub(dag, &a1b0, &a0b1);
    let at2 = darray_mul(dag, &at, &at);
    let btct = darray_mul(dag, &bt, &ct);
    darray_sub(dag, &at2, &btct)
}

fn determinant_dag(dag: &mut Dag, matrix: &[Vec<usize>]) -> usize {
    let n = matrix.len();
    let mut dp = vec![0; 1 << n];
    dp[0] = 1;
    for mask in 0usize..(1usize << n) {
        let row = mask.count_ones() as usize;
        if row >= n || dp[mask] == 0 {
            continue;
        }
        for (column, &entry) in matrix[row].iter().enumerate() {
            if mask & (1 << column) != 0 {
                continue;
            }
            let mut term = dag.mul(dp[mask], entry);
            if (mask >> (column + 1)).count_ones() & 1 == 1 {
                term = dag.neg(term);
            }
            let next = mask | (1 << column);
            dp[next] = dag.add(dp[next], term);
        }
    }
    dp[(1 << n) - 1]
}

fn build_h_dag() -> (Dag, usize) {
    let mut dag = Dag::new();
    let sa = dag.input(SA);
    let pa = dag.input(PA);
    let sb = dag.input(SB);
    let pb = dag.input(PB);
    let r = dag.input(TARGET_R);
    let ca = pair_c_dag(&mut dag, sa, pa);
    let cb = pair_c_dag(&mut dag, sb, pb);
    let quartic = s4_dag(&mut dag, &cb, r);
    let mut matrix = vec![vec![0; 6]; 6];
    for shift in 0..2 {
        for i in 0..5 {
            matrix[shift][shift + i] = quartic[4 - i];
        }
    }
    for shift in 0..4 {
        for i in 0..3 {
            matrix[2 + shift][shift + i] = ca[2 - i];
        }
    }
    let root = determinant_dag(&mut dag, &matrix);
    dag.compact(root)
}

fn specialize_dag(source: &Dag, root: usize, input: usize, value: u64) -> (Dag, usize, usize) {
    let mut destination = Dag::new();
    let mut map = vec![0; source.nodes.len()];
    for (id, node) in source.nodes.iter().enumerate() {
        map[id] = match *node {
            Node::Input(index) if index as usize == input => destination.constant(value),
            Node::Input(index) => destination.input(index as usize),
            Node::Const(c) => destination.constant(c),
            Node::Add(a, b) => destination.add(map[a], map[b]),
            Node::Mul(a, b) => destination.mul(map[a], map[b]),
            Node::Neg(a) => destination.neg(map[a]),
        };
    }
    let peak = source.nodes.len() + destination.nodes.len();
    let (compact, compact_root) = destination.compact(map[root]);
    (compact, compact_root, peak)
}

type Rw = [usize; 2];
type Elem4 = [usize; 4];

fn rw_add(dag: &mut Dag, a: Rw, b: Rw) -> Rw {
    [dag.add(a[0], b[0]), dag.add(a[1], b[1])]
}

fn rw_sub(dag: &mut Dag, a: Rw, b: Rw) -> Rw {
    [dag.sub(a[0], b[0]), dag.sub(a[1], b[1])]
}

fn rw_mul(dag: &mut Dag, a: Rw, b: Rw, k: usize) -> Rw {
    let u = dag.mul(a[0], b[0]);
    let v = dag.mul(a[1], b[1]);
    let a_sum = dag.add(a[0], a[1]);
    let b_sum = dag.add(b[0], b[1]);
    let sum_product = dag.mul(a_sum, b_sum);
    let cross_without_u = dag.sub(sum_product, u);
    let cross = dag.sub(cross_without_u, v);
    let kv = dag.mul(k, v);
    [dag.add(u, kv), cross]
}

fn q_mul(dag: &mut Dag, value: Rw, s4: usize, k: usize) -> Rw {
    let s4_v0 = dag.mul(s4, value[0]);
    let k_v1 = dag.mul(k, value[1]);
    let two_k_v1 = dag.scale(k_v1, 2);
    let first = dag.add(s4_v0, two_k_v1);
    let two_v0 = dag.scale(value[0], 2);
    let s4_v1 = dag.mul(s4, value[1]);
    [first, dag.add(two_v0, s4_v1)]
}

fn elem_add(dag: &mut Dag, a: Elem4, b: Elem4) -> Elem4 {
    [
        dag.add(a[0], b[0]),
        dag.add(a[1], b[1]),
        dag.add(a[2], b[2]),
        dag.add(a[3], b[3]),
    ]
}

fn elem_neg(dag: &mut Dag, a: Elem4) -> Elem4 {
    [dag.neg(a[0]), dag.neg(a[1]), dag.neg(a[2]), dag.neg(a[3])]
}

fn elem_mul(dag: &mut Dag, a: Elem4, b: Elem4, s4: usize, k: usize) -> Elem4 {
    let a0 = [a[0], a[1]];
    let a1 = [a[2], a[3]];
    let b0 = [b[0], b[1]];
    let b1 = [b[2], b[3]];
    let u = rw_mul(dag, a0, b0, k);
    let v = rw_mul(dag, a1, b1, k);
    let a_sum = rw_add(dag, a0, a1);
    let b_sum = rw_add(dag, b0, b1);
    let sum_product = rw_mul(dag, a_sum, b_sum, k);
    let cross_without_u = rw_sub(dag, sum_product, u);
    let cross = rw_sub(dag, cross_without_u, v);
    let qv = q_mul(dag, v, s4, k);
    let constant = rw_add(dag, u, qv);
    [constant[0], constant[1], cross[0], cross[1]]
}

fn tower_norm(source: &Dag, root: usize, s_index: usize, pair_index: usize) -> (Dag, usize, usize) {
    let mut dag = Dag::new();
    let s = dag.input(s_index);
    let pair = dag.input(pair_index);
    let two_s = dag.scale(s, 2);
    let pair_two_s = dag.add(pair, two_s);
    let four = dag.constant(4);
    let k = dag.add(pair_two_s, four);
    let s4 = dag.add(s, four);
    let mut values = vec![[0; 4]; source.nodes.len()];
    for (id, node) in source.nodes.iter().enumerate() {
        values[id] = match *node {
            Node::Input(index) if index as usize == s_index => [0, 0, 1, 0],
            Node::Input(index) if index as usize == pair_index => [0, 1, 0, 0],
            Node::Input(index) => [dag.input(index as usize), 0, 0, 0],
            Node::Const(c) => [dag.constant(c), 0, 0, 0],
            Node::Add(a, b) => elem_add(&mut dag, values[a], values[b]),
            Node::Mul(a, b) => elem_mul(&mut dag, values[a], values[b], s4, k),
            Node::Neg(a) => elem_neg(&mut dag, values[a]),
        };
        if dag.stopped {
            return (dag, 0, source.nodes.len() + NODE_CAP);
        }
    }
    let value = values[root];
    let a = [value[0], value[1]];
    let b = [value[2], value[3]];
    let a2 = rw_mul(&mut dag, a, a, k);
    let b2 = rw_mul(&mut dag, b, b, k);
    let qb2 = q_mul(&mut dag, b2, s4, k);
    let g = rw_sub(&mut dag, a2, qb2);
    let g0_sq = dag.mul(g[0], g[0]);
    let g1_sq = dag.mul(g[1], g[1]);
    let k_g1_sq = dag.mul(k, g1_sq);
    let norm = dag.sub(g0_sq, k_g1_sq);
    let peak = source.nodes.len() + dag.nodes.len();
    let (compact, compact_root) = dag.compact(norm);
    (compact, compact_root, peak)
}

fn emit_dag(label: &str, depth: usize, dag: &Dag, root: usize, peak: usize) {
    let (adds, muls, negs, edges) = dag.operation_counts();
    println!(
        "DAG_BEGIN label={label} depth={depth} nodes={} edges={edges} add={adds} mul={muls} neg={negs} root={root} peak={peak}",
        dag.nodes.len()
    );
    for (id, node) in dag.nodes.iter().enumerate() {
        match node {
            Node::Input(i) => println!("D {id} I {i}"),
            Node::Const(c) => println!("D {id} C {c}"),
            Node::Add(a, b) => println!("D {id} A {a} {b}"),
            Node::Mul(a, b) => println!("D {id} M {a} {b}"),
            Node::Neg(a) => println!("D {id} N {a}"),
        }
    }
    println!("DAG_END label={label} depth={depth}");
}

fn run_poly_series(prefix: &str, target: Option<u64>) {
    let h_label = if prefix.is_empty() {
        "h".to_owned()
    } else {
        format!("{prefix}_h")
    };
    let norm_label = if prefix.is_empty() {
        "second_pair_norm".to_owned()
    } else {
        format!("{prefix}_second_pair_norm")
    };
    match build_h_poly() {
        Ok((formal, mut counts)) => {
            let mut poly = if let Some(value) = target {
                match formal.substitute(TARGET_R, &Poly::constant(value), &mut counts) {
                    Ok(specialized) => specialized,
                    Err(stop) => {
                        println!(
                            "POLY_STOP label={h_label} depth=0 reason={stop:?} prior_slots={}",
                            formal.terms.len()
                        );
                        return;
                    }
                }
            } else {
                formal
            };
            emit_poly(&h_label, 0, &poly, counts);
            for depth in 1..=6 {
                let mut norm_counts = Counts::default();
                match pair_norm_poly(&poly, SB, PB, &mut norm_counts) {
                    Ok(next) => {
                        poly = next;
                        emit_poly(&norm_label, depth, &poly, norm_counts);
                    }
                    Err(stop) => {
                        println!(
                            "POLY_STOP label={norm_label} depth={depth} reason={stop:?} prior_slots={}",
                            poly.terms.len()
                        );
                        break;
                    }
                }
            }
        }
        Err(stop) => println!("POLY_STOP label={h_label} depth=0 reason={stop:?} prior_slots=0"),
    }
}

fn run_dag_series(prefix: &str, target: Option<u64>) {
    let h_label = if prefix.is_empty() {
        "h".to_owned()
    } else {
        format!("{prefix}_h")
    };
    let norm_label = if prefix.is_empty() {
        "second_pair_norm".to_owned()
    } else {
        format!("{prefix}_second_pair_norm")
    };
    let (formal, formal_root) = build_h_dag();
    let (mut dag, mut root, initial_peak) = if let Some(value) = target {
        specialize_dag(&formal, formal_root, TARGET_R, value)
    } else {
        let nodes = formal.nodes.len();
        (formal, formal_root, nodes)
    };
    emit_dag(&h_label, 0, &dag, root, initial_peak);
    for depth in 1..=6 {
        let (next, next_root, peak) = tower_norm(&dag, root, SB, PB);
        if next.stopped {
            println!(
                "DAG_STOP label={norm_label} depth={depth} reason=CAP prior_nodes={} attempted_peak={peak}",
                dag.nodes.len()
            );
            break;
        }
        dag = next;
        root = next_root;
        emit_dag(&norm_label, depth, &dag, root, peak);
    }
}

fn main() {
    println!("HEADER schema=pplus-pair-divisor-transfer-candidate-v3 prime={MODULUS} seed_manifest=null slot_cap={SLOT_CAP} node_cap={NODE_CAP} cross_target_amortization=0");
    run_poly_series("", None);
    run_poly_series("g1", Some(TARGET_G1_X));
    run_poly_series("g17", Some(TARGET_G17_X));
    run_dag_series("", None);
    run_dag_series("g1", Some(TARGET_G1_X));
    run_dag_series("g17", Some(TARGET_G17_X));
    println!("FOOTER status=PRODUCER_COMPLETE");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_arithmetic_wraps() {
        assert_eq!(fadd(MODULUS - 1, 2), 1);
        assert_eq!(fmul(MODULUS - 1, MODULUS - 1), 1);
        assert_eq!(fadd(0, fneg(1)), MODULUS - 1);
    }

    #[test]
    fn polynomial_evenization() {
        let mut counts = Counts::default();
        let z = Poly::variable(AUX_Z);
        let f = z.add(&Poly::constant(3), &mut counts).unwrap();
        let product = f.mul(&f.sign_flip(AUX_Z), &mut counts).unwrap();
        let even = product.halve_even(AUX_Z).unwrap();
        assert_eq!(even.terms.len(), 2);
    }

    #[test]
    fn tower_norm_of_linear_form() {
        let mut source = Dag::new();
        let s = source.input(SB);
        let pair = source.input(PB);
        let root = source.add(s, pair);
        let (source, root) = source.compact(root);
        let (result, _, _) = tower_norm(&source, root, SB, PB);
        assert!(!result.stopped);
        assert!(result.nodes.len() > source.nodes.len());
    }
}
