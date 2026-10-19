//! Actual F2 coefficient-block polynomial presentations, not Boolean point equations.
use super::*;
use num_bigint::BigUint;
use num_traits::One;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "op", content = "args")]
enum Op {
    Input(usize),
    Constant(String),
    Add(usize, usize),
    Mul(usize, usize),
    CoefficientFrobenius(usize),
    Inverse(usize),
}
#[derive(Serialize, Deserialize)]
struct Program {
    nodes: Vec<Op>,
    residuals: Vec<usize>,
    inputs: Vec<usize>,
}
struct Builder<'a> {
    f: &'a F,
    p: Program,
    memo: BTreeMap<String, usize>,
}
impl<'a> Builder<'a> {
    fn new(f: &'a F) -> Self {
        Self {
            f,
            p: Program {
                nodes: vec![],
                residuals: vec![],
                inputs: vec![],
            },
            memo: BTreeMap::new(),
        }
    }
    fn node(&mut self, op: Op) -> usize {
        let key = serde_json::to_string(&op).unwrap();
        if let Some(i) = self.memo.get(&key) {
            return *i;
        }
        let i = self.p.nodes.len();
        self.memo.insert(key, i);
        self.p.nodes.push(op);
        assert!(self.p.nodes.len() <= 10000);
        i
    }
    fn input(&mut self, k: usize) -> usize {
        let i = self.node(Op::Input(k));
        self.p.inputs.push(i);
        i
    }
    fn constant(&mut self, x: &E) -> usize {
        self.node(Op::Constant(x.to_biguint().to_str_radix(16)))
    }
    fn c(&mut self, n: u64) -> usize {
        self.constant(&self.f.e(n))
    }
    fn add(&mut self, a: usize, b: usize) -> usize {
        if a == b {
            return self.c(0);
        }
        self.node(Op::Add(a.min(b), a.max(b)))
    }
    fn mul(&mut self, a: usize, b: usize) -> usize {
        self.node(Op::Mul(a.min(b), a.max(b)))
    }
    fn inv(&mut self, a: usize) -> usize {
        self.node(Op::Inverse(a))
    }
    fn div(&mut self, a: usize, b: usize) -> usize {
        let i = self.inv(b);
        self.mul(a, i)
    }
    fn sigma(&mut self, a: usize) -> usize {
        self.node(Op::CoefficientFrobenius(a))
    }
    fn pow(&mut self, a: usize, n: u64) -> usize {
        let mut r = self.c(1);
        for j in (0..64 - n.leading_zeros()).rev() {
            r = self.mul(r, r);
            if n >> j & 1 == 1 {
                r = self.mul(r, a)
            }
        }
        r
    }
    fn poly(&mut self, a: &Poly, x: usize) -> usize {
        let mut r = self.c(0);
        for c in a.iter().rev() {
            r = self.mul(r, x);
            let v = self.constant(c);
            r = self.add(r, v)
        }
        r
    }
    fn fun(&mut self, a: &Fn, p: (usize, usize)) -> usize {
        let x = self.poly(&a.a, p.0);
        let y = self.poly(&a.b, p.0);
        let y = self.mul(y, p.1);
        let n = self.add(x, y);
        let d = self.poly(&a.d, p.0);
        self.div(n, d)
    }
    fn neg(&mut self, p: (usize, usize)) -> (usize, usize) {
        (p.0, self.add(p.0, p.1))
    }
    fn point(&mut self, p: &P) -> (usize, usize) {
        let P::Affine { x, y } = p else {
            panic!("affine constant")
        };
        (self.constant(x), self.constant(y))
    }
    fn plus(&mut self, p: (usize, usize), q: (usize, usize), a: u64) -> (usize, usize) {
        let n = self.add(p.1, q.1);
        let d = self.add(p.0, q.0);
        let l = self.div(n, d);
        let ll = self.mul(l, l);
        let x = self.add(ll, l);
        let pq = self.add(p.0, q.0);
        let x = self.add(x, pq);
        let aa = self.c(a);
        let x = self.add(x, aa);
        let v = self.add(p.0, x);
        let y = self.mul(l, v);
        let y = self.add(y, p.1);
        let y = self.add(y, x);
        (x, y)
    }
    fn h(&mut self, t: &P, x: (usize, usize), y: (usize, usize), a: u64) -> usize {
        let tp = self.point(t);
        let ny = self.neg(y);
        let p = self.plus(tp, ny, a);
        let n = self.add(p.1, y.1);
        let d = self.add(p.0, y.0);
        let l = self.div(n, d);
        let dx = self.add(x.0, p.0);
        let ld = self.mul(l, dx);
        let n = self.add(x.1, p.1);
        let n = self.add(n, ld);
        let d1 = self.add(x.0, tp.0);
        let d2 = self.add(x.0, y.0);
        let d = self.mul(d1, d2);
        self.div(n, d)
    }
    fn zero(&mut self, x: usize) {
        self.p.residuals.push(x)
    }
}
fn build(f: &F, l: &Lift) -> Program {
    let mut b = Builder::new(f);
    let inputs: Vec<_> = (0..12).map(|k| b.input(k)).collect();
    let p: Vec<_> = (0..5).map(|i| (inputs[2 * i], inputs[2 * i + 1])).collect();
    let a = u64::from(l.trace == 1);
    for j in 0..5 {
        let (x, y) = p[j];
        let y2 = b.mul(y, y);
        let h = if j == 4 { b.c(1) } else { x };
        let hy = b.mul(h, y);
        let lhs = b.add(y2, hy);
        let x3 = b.pow(x, 3);
        let rhs = if j == 4 {
            let c = b.c(a);
            let v = b.add(x3, x);
            b.add(v, c)
        } else {
            let x2 = b.mul(x, x);
            let c = b.c(a);
            let v = b.mul(x2, c);
            let v = b.add(x3, v);
            let one = b.c(1);
            b.add(v, one)
        };
        let eq = b.add(lhs, rhs);
        b.zero(eq);
    }
    for i in 0..2 {
        let mut r = b.c(1);
        for j in 0..5 {
            let v = b.fun(&l.functions[i][j], p[j]);
            r = b.mul(r, v)
        }
        let z = b.pow(inputs[10 + i], 19);
        let eq = b.add(z, r);
        b.zero(eq);
        let _ = b.inv(inputs[10 + i]);
    }
    let sp: Vec<_> = p.iter().map(|(x, y)| (b.sigma(*x), b.sigma(*y))).collect();
    let mut dest = vec![b.neg(sp[3])];
    for j in 0..3 {
        let q = b.neg(sp[3]);
        dest.push(b.plus(sp[j], q, a))
    }
    dest.push(sp[4]);
    let mut hs = vec![];
    for i in 0..2 {
        let mut h = b.fun(&l.sums[i], sp[3]);
        for j in 1..4 {
            let ny = b.neg(sp[3]);
            let z = b.h(&l.t[i][j], sp[j - 1], ny, a);
            h = b.mul(h, z)
        }
        for j in 0..5 {
            let z = b.fun(&l.classes[i][j], sp[j]);
            h = b.div(h, z)
        }
        hs.push(h)
    }
    for j in 0..5 {
        let ex = b.add(p[j].0, dest[j].0);
        let ey = b.add(p[j].1, dest[j].1);
        b.zero(ex);
        b.zero(ey)
    }
    let z0 = b.sigma(inputs[10]);
    let z1 = b.sigma(inputs[11]);
    let w0 = b.mul(z1, hs[0]);
    let p9 = b.pow(z0, 9);
    let signed = if l.trace < 0 { b.inv(z1) } else { z1 };
    let w1 = b.mul(p9, signed);
    let w1 = b.mul(w1, hs[1]);
    let e0 = b.add(inputs[10], w0);
    let e1 = b.add(inputs[11], w1);
    b.zero(e0);
    b.zero(e1);
    b.p
}
fn reduce(mut x: BigUint, modulus: &BigUint) -> BigUint {
    while x.bits() >= modulus.bits() {
        let shift = x.bits() - modulus.bits();
        x ^= modulus << shift;
    }
    x
}
fn templates(dir: &Path) -> Value {
    let f = F::new(360, 45);
    assert!(f.irreducible());
    let m = f.modulus();
    let reductions: Vec<_> = (0..719).map(|k| reduce(BigUint::one() << k, &m)).collect();
    for i in 0..360 {
        for j in 0..360 {
            let a = E::from_biguint(&(BigUint::one() << i), 360);
            let b = E::from_biguint(&(BigUint::one() << j), 360);
            assert_eq!(f.mul(&a, &b).to_biguint(), reductions[i + j]);
        }
    }
    for j in 0..360 {
        let a = E::from_biguint(&(BigUint::one() << j), 360);
        assert_eq!(f.sq(&a).to_biguint(), reductions[2 * j]);
        assert_eq!(f.frob(&a, 360), a);
    }
    // Test the multiplication tensor over F2[e]/e²; coefficient sigma fixes e.
    let mut dual_controls = 0;
    for k in 0..64 {
        let a = f.e(3 + 17 * k);
        let d = f.e(9 + 23 * k);
        let b = f.e(11 + 31 * k);
        let e = f.e(5 + 19 * k);
        let pr = (f.mul(&a, &b), f.mul(&a, &e).add(&f.mul(&d, &b)));
        let sigpr = (f.sq(&pr.0), f.sq(&pr.1));
        let prod = (
            f.mul(&f.sq(&a), &f.sq(&b)),
            f.mul(&f.sq(&a), &f.sq(&e))
                .add(&f.mul(&f.sq(&d), &f.sq(&b))),
        );
        assert_eq!(sigpr, prod);
        assert!(!f.sq(&d).is_zero());
        dual_controls += 1;
    }
    let data = json!({"schema":"koblitz.round10/F2-polynomial-templates-v1","field_degree":360,"modulus":"z^360+z^5+z^3+z^2+1","reductions_zk":reductions.iter().map(|x|x.to_str_radix(16)).collect::<Vec<_>>(),"multiplication_equation":"out_i + sum_(0<=j,k<360;bit(reductions_zk[j+k],i)=1) left_j*right_k = 0","coefficient_Frobenius_equation":"out_i + sum_(0<=j<360;bit(reductions_zk[2*j],i)=1) input_j = 0","addition_equation":"out_i+left_i+right_i=0","constant_equation":"out_i+bit(constant,i)=0","inverse_equation":"sum_(0<=j,k<360;bit(reductions_zk[j+k],i)=1) input_j*out_k+delta_(i,0)=0","residual_equation":"node_i=0","basis_products_checked":129600,"basis_sigma_checks":360,"dual_number_controls":dual_controls,"coordinate_variables_are_indeterminates":true,"Boolean_relations":false});
    round02::save(dir, "q8-F2-polynomial-templates.json", &data);
    data
}
fn orbit(f: &F, l: &Lift, fixture: &Value) -> Vec<Vec<E>> {
    let c = curves(f, l.trace);
    let mut p: Vec<_> = fixture["point"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| dp(f, p))
        .collect();
    let z: Vec<_> = fixture["root_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| crate::reverse06::decode(f, x))
        .collect();
    let mut z = [z[0].clone(), z[1].clone()];
    let start = (p.clone(), z.clone());
    let mut out = vec![];
    for _ in 0..360 {
        let mut row = vec![];
        for p in &p {
            let P::Affine { x, y } = p else {
                panic!("affine orbit")
            };
            row.extend([x.clone(), y.clone()])
        }
        row.extend(z.clone());
        out.push(row);
        let sp: Vec<_> = p.iter().map(|p| f.frobp(p, 1)).collect();
        let sz0 = f.sq(&z[0]);
        let sz1 = f.sq(&z[1]);
        z = [
            f.mul(&sz1, &h_value(&c, l, 0, &sp)),
            f.mul(
                &f.mul(
                    &f.powu(&sz0, 9),
                    &if l.trace < 0 { f.inv(&sz1) } else { sz1 },
                ),
                &h_value(&c, l, 1, &sp),
            ),
        ];
        p = beta(&c[0], &sp);
    }
    assert_eq!((p, z), start);
    out
}
fn split_input(f: &F, orb: &[Vec<E>]) -> Vec<Vec<E>> {
    (0..12)
        .map(|k| {
            (0..360)
                .map(|j| f.frob(&orb[(360 - j) % 360][k], j as u32))
                .collect()
        })
        .collect()
}
fn evaluate(f: &F, p: &Program, input: &[Vec<E>]) -> (usize, usize, Vec<Vec<E>>) {
    let mut values: Vec<Vec<E>> = vec![];
    let mut inverses = 0;
    for (i, op) in p.nodes.iter().enumerate() {
        let v = match op {
            Op::Input(k) => input[*k].clone(),
            Op::Constant(s) => {
                let mut x = E::from_biguint(&BigUint::parse_bytes(s.as_bytes(), 16).unwrap(), 360);
                (0..360)
                    .map(|_| {
                        let v = x.clone();
                        x = f.sq(&x);
                        v
                    })
                    .collect()
            }
            Op::Add(a, b) => {
                assert!(*a < i && *b < i);
                (0..360)
                    .map(|j| values[*a][j].add(&values[*b][j]))
                    .collect()
            }
            Op::Mul(a, b) => {
                assert!(*a < i && *b < i);
                (0..360)
                    .map(|j| f.mul(&values[*a][j], &values[*b][j]))
                    .collect()
            }
            Op::CoefficientFrobenius(a) => {
                assert!(*a < i);
                (0..360)
                    .map(|j| values[*a][(j + 1) % 360].clone())
                    .collect()
            }
            Op::Inverse(a) => {
                assert!(*a < i);
                inverses += 1;
                (0..360)
                    .map(|j| {
                        assert!(!values[*a][j].is_zero());
                        let v = f.inv(&values[*a][j]);
                        assert_eq!(f.mul(&v, &values[*a][j]), f.o());
                        v
                    })
                    .collect()
            }
        };
        values.push(v);
    }
    for r in &p.residuals {
        assert!(
            values[*r].iter().all(E::is_zero),
            "descent residual node{r}"
        );
    }
    let checks = (p.nodes.len() - p.inputs.len() + p.residuals.len()) * 360;
    (checks, inverses * 360, values)
}
fn recover(f: &F, input: &[Vec<E>], dual: &[E]) -> Vec<Vec<E>> {
    let embedding = embeddings(f);
    let mut coeff = vec![vec![f.z(); 360]; 12];
    let mut dj = dual.to_vec();
    for j in 0..360 {
        for k in 0..12 {
            for i in 0..360 {
                coeff[k][i] = coeff[k][i].add(&f.mul(&dj[i], &input[k][j]));
            }
        }
        for d in &mut dj {
            *d = f.sq(d)
        }
    }
    for k in 0..12 {
        for j in 0..360 {
            let value = (0..360).fold(f.z(), |a, i| a.add(&f.mul(&embedding[j][i], &coeff[k][i])));
            assert_eq!(value, input[k][j]);
        }
    }
    coeff
}

fn embeddings(f: &F) -> Vec<Vec<E>> {
    let mut row: Vec<E> = (0..360)
        .map(|i| E::from_biguint(&(BigUint::one() << i), 360))
        .collect();
    let mut out = Vec::with_capacity(360);
    for _ in 0..360 {
        out.push(row.clone());
        row = row.iter().map(|x| f.sq(x)).collect();
    }
    out
}
pub fn run(dir: &Path) {
    let _ = templates(dir);
    let f = F::new(360, 45);
    let descent = round04::read(&dir.join("q7-Frobenius-coordinate-circuits.json"));
    let base = round04::read(&dir.join("q6-intrinsic-Kummer-charts.json"));
    let tr = round04::read(&dir.join("q7c-invariant-rational-generators.json"));
    let dual: Vec<_> = tr["dual_basis"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| crate::reverse06::decode(&f, x))
        .collect();
    let mut records = vec![];
    for (ri, row) in descent["records"].as_array().unwrap().iter().enumerate() {
        let l = parsed_lift(&f, row, &base["records"][ri]);
        let p = build(&f, &l);
        let equations = (p.nodes.len() - p.inputs.len() + p.residuals.len()) * 360;
        let variables = p.nodes.len() * 360;
        let mut fixtures = vec![];
        let mut total = 0;
        let mut units = 0;
        for fixture in row["fixtures"].as_array().unwrap() {
            let orb = orbit(&f, &l, fixture);
            let input = split_input(&f, &orb);
            let (checks, inverses, _) = evaluate(&f, &p, &input);
            let coeff = recover(&f, &input, &dual);
            total += checks;
            units += inverses;
            fixtures.push(json!({"split_input_values":input.iter().map(|r|r.iter().map(ej).collect::<Vec<_>>()).collect::<Vec<_>>(),"F2_coordinate_blocks_at_K_point":coeff.iter().map(|r|r.iter().map(ej).collect::<Vec<_>>()).collect::<Vec<_>>(),"recovered_all_360_components":true}));
        }
        let filename = format!("q8-graph-{ri}-F2-relations.json");
        round02::save(
            dir,
            &filename,
            &json!({"schema":"koblitz.round10/q8-graph-v1","graph":ri,"program":p,"variables":variables,"individual_F2_equations":equations,"maximum_equation_degree":2,"closed_affine_embedding":true,"open_scope":"intersection of360 conjugate rational-lift domains in the retained quotient;global cocycle supplies cycle closure","saturation_element":"product of Norm_K/F2(input_to_inverse_node);norm polynomial is product of its360 coefficient-Frobenius conjugates","inverse_witness_saturated":true,"homogenization":"constant:out_i+c_i*h;add:out_i+left_i+right_i;mul:out_i*h+tensor(left,right)_i;inverse:tensor(input,out)_i+delta_i0*h^2;coefficient-Frobenius/residual:unchanged linear equations","projective_h_saturation_computed":false,"whole_proper_quotient_embedded":false,"fixtures":fixtures,"equation_component_checks":total,"unit_component_checks":units}),
        );
        records.push(json!({"graph":ri,"file":filename,"variables":variables,"F2_equations":equations,"equation_component_checks":total,"unit_component_checks":units}));
        println!("Q8 graph{ri}:{} gates,{variables} variables,{equations} F2 equations;two split fixtures pass",p.nodes.len());
    }
    round02::save(
        dir,
        "q8-F2-presentations-summary.json",
        &json!({"records":records,"scope":"explicit quadratic F2 relation systems in lossless coefficient-block format;denominator saturation and closed affine open embeddings;whole projective embedding remains open"}),
    );
}

pub fn verify(dir: &Path) {
    let t = round04::read(&dir.join("q8-F2-polynomial-templates.json"));
    let f = F::new(360, 45);
    let embedding = embeddings(&f);
    let reductions: Vec<_> = t["reductions_zk"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| BigUint::parse_bytes(s.as_str().unwrap().as_bytes(), 16).unwrap())
        .collect();
    for k in 0..719 {
        assert_eq!(reductions[k], reduce(BigUint::one() << k, &f.modulus()));
    }
    let mut records = vec![];
    for ri in 0..4 {
        let data = round04::read(&dir.join(format!("q8-graph-{ri}-F2-relations.json")));
        let p: Program = serde_json::from_value(data["program"].clone()).unwrap();
        let mut checks = 0;
        let mut units = 0;
        for fixture in data["fixtures"].as_array().unwrap() {
            let input: Vec<Vec<E>> = fixture["split_input_values"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| {
                    r.as_array()
                        .unwrap()
                        .iter()
                        .map(|x| crate::reverse06::decode(&f, x))
                        .collect()
                })
                .collect();
            let (a, b, _) = evaluate(&f, &p, &input);
            checks += a;
            units += b;
            let coeff: Vec<Vec<E>> = fixture["F2_coordinate_blocks_at_K_point"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| {
                    r.as_array()
                        .unwrap()
                        .iter()
                        .map(|x| crate::reverse06::decode(&f, x))
                        .collect()
                })
                .collect();
            for k in 0..12 {
                for j in 0..360 {
                    let v =
                        (0..360).fold(f.z(), |s, i| s.add(&f.mul(&embedding[j][i], &coeff[k][i])));
                    assert_eq!(v, input[k][j]);
                }
            }
        }
        assert_eq!(data["equation_component_checks"], checks);
        assert_eq!(data["unit_component_checks"], units);
        records.push(json!({"graph":ri,"parsed_equation_component_checks":checks,"parsed_unit_component_checks":units,"original_input_coefficient_values_checked":8640}));
    }
    round02::save(
        dir,
        "q8b-parsed-F2-presentation-check.json",
        &json!({"records":records,"coefficient_variables_indeterminate":true,"scope":"separate parser,shared ring/field primitives;checks every emitted block constraint in all split components and recovers every input coefficient;global affine-descent bridge written"}),
    );
    println!("Q8b:all four parsed systems and all coefficient recoveries pass");
}

/// Expand representative scalar equations to literal sparse F2 monomials.
pub fn expand(dir: &Path) {
    use std::collections::BTreeSet;
    let template = round04::read(&dir.join("q8-F2-polynomial-templates.json"));
    let f = F::new(360, 45);
    let reductions: Vec<_> = template["reductions_zk"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| BigUint::parse_bytes(s.as_str().unwrap().as_bytes(), 16).unwrap())
        .collect();
    let mut records = vec![];
    let mut checks = 0;
    for ri in 0..4 {
        let data = round04::read(&dir.join(format!("q8-graph-{ri}-F2-relations.json")));
        let p: Program = serde_json::from_value(data["program"].clone()).unwrap();
        let selectors: [fn(&Op) -> bool; 6] = [
            |o| matches!(o, Op::Constant(_)),
            |o| matches!(o, Op::Add(..)),
            |o| matches!(o,Op::Mul(a,b)if a==b),
            |o| matches!(o,Op::Mul(a,b)if a!=b),
            |o| matches!(o, Op::CoefficientFrobenius(_)),
            |o| matches!(o, Op::Inverse(_)),
        ];
        for select in selectors {
            let (node, op) = p.nodes.iter().enumerate().find(|(_, o)| select(o)).unwrap();
            for i in [0usize, 1, 359] {
                let mut monomials = BTreeSet::<Vec<usize>>::new();
                let mut add = |mut v: Vec<usize>| {
                    v.sort();
                    if !monomials.insert(v.clone()) {
                        monomials.remove(&v);
                    }
                };
                match op {
                    Op::Constant(s) => {
                        add(vec![node * 360 + i]);
                        let c = BigUint::parse_bytes(s.as_bytes(), 16).unwrap();
                        if c.bit(i as u64) {
                            add(vec![])
                        }
                    }
                    Op::Add(a, b) => {
                        add(vec![node * 360 + i]);
                        add(vec![a * 360 + i]);
                        add(vec![b * 360 + i]);
                    }
                    Op::CoefficientFrobenius(a) => {
                        add(vec![node * 360 + i]);
                        for j in 0..360 {
                            if reductions[2 * j].bit(i as u64) {
                                add(vec![a * 360 + j])
                            }
                        }
                    }
                    Op::Mul(a, b) => {
                        add(vec![node * 360 + i]);
                        for j in 0..360 {
                            for k in 0..360 {
                                if reductions[j + k].bit(i as u64) {
                                    add(vec![a * 360 + j, b * 360 + k])
                                }
                            }
                        }
                    }
                    Op::Inverse(a) => {
                        if i == 0 {
                            add(vec![])
                        }
                        for j in 0..360 {
                            for k in 0..360 {
                                if reductions[j + k].bit(i as u64) {
                                    add(vec![a * 360 + j, node * 360 + k])
                                }
                            }
                        }
                    }
                    Op::Input(_) => unreachable!(),
                }
                for seed in 1..=64 {
                    let a = f.powu(&f.e(2), ((17 * seed + 11) % 359 + 1) as u64);
                    let b = f.powu(&f.e(3), ((31 * seed + 9) % 359 + 1) as u64);
                    let mut blocks = BTreeMap::new();
                    match op {
                        Op::Constant(s) => {
                            blocks.insert(
                                node,
                                E::from_biguint(
                                    &BigUint::parse_bytes(s.as_bytes(), 16).unwrap(),
                                    360,
                                ),
                            );
                        }
                        Op::Add(x, y) => {
                            blocks.insert(*x, a.clone());
                            blocks.insert(*y, b.clone());
                            blocks.insert(node, a.add(&b));
                        }
                        Op::CoefficientFrobenius(x) => {
                            blocks.insert(*x, a.clone());
                            blocks.insert(node, f.sq(&a));
                        }
                        Op::Mul(x, y) => {
                            let b = if x == y { a.clone() } else { b };
                            blocks.insert(*x, a.clone());
                            blocks.insert(*y, b.clone());
                            blocks.insert(node, f.mul(&a, &b));
                        }
                        Op::Inverse(x) => {
                            blocks.insert(*x, a.clone());
                            blocks.insert(node, f.inv(&a));
                        }
                        Op::Input(_) => unreachable!(),
                    }
                    let parity = monomials
                        .iter()
                        .filter(|m| {
                            m.iter().all(|var| {
                                blocks[&(var / 360)].to_biguint().bit((var % 360) as u64)
                            })
                        })
                        .count()
                        % 2;
                    assert_eq!(parity, 0);
                    checks += 1;
                }
                records.push(json!({"graph":ri,"node":node,"coordinate":i,"operation":op,"variable_index":"360*node+coefficient","all_coefficients":1,"monomials":monomials}));
            }
        }
    }
    round02::save(
        dir,
        "q8c-expanded-F2-equations.json",
        &json!({"schema":"koblitz.round10/q8c-v1","polynomials":records,"local_gate_evaluation_controls":checks,"scope":"literal sparse scalar F2 equations for six representative gate types at three coefficient positions,all four graphs;full systems are losslessly specified by the same shared template and program"}),
    );
    println!("Q8c:72 scalar polynomials expanded and{checks} local gate controls pass");
}
