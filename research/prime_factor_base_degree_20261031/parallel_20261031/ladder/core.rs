use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ops {
    pub field_adds: u64,
    pub field_muls: u64,
    pub field_inversions: u64,
    pub hash_calls: u64,
    pub primality_trial_divisions: u64,
    pub group_calls: u64,
    pub group_doublings: u64,
}
impl Ops {
    pub fn delta(&self, before: &Self) -> Self {
        Self {
            field_adds: self.field_adds - before.field_adds,
            field_muls: self.field_muls - before.field_muls,
            field_inversions: self.field_inversions - before.field_inversions,
            hash_calls: self.hash_calls - before.hash_calls,
            primality_trial_divisions: self.primality_trial_divisions
                - before.primality_trial_divisions,
            group_calls: self.group_calls - before.group_calls,
            group_doublings: self.group_doublings - before.group_doublings,
        }
    }
}
pub struct Field {
    pub p: u64,
    pub ops: Ops,
}
impl Field {
    pub fn new(p: u64) -> Self {
        Self {
            p,
            ops: Ops::default(),
        }
    }
    pub fn add(&mut self, a: u64, b: u64) -> u64 {
        self.ops.field_adds += 1;
        (a + b) % self.p
    }
    pub fn sub(&mut self, a: u64, b: u64) -> u64 {
        self.ops.field_adds += 1;
        (a + self.p - b) % self.p
    }
    pub fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.ops.field_muls += 1;
        a * b % self.p
    }
    pub fn pow(&mut self, mut a: u64, mut n: u64) -> u64 {
        let mut out = 1;
        while n > 0 {
            if n & 1 == 1 {
                out = self.mul(out, a);
            }
            n >>= 1;
            if n > 0 {
                a = self.mul(a, a);
            }
        }
        out
    }
    pub fn inv(&mut self, a: u64) -> u64 {
        assert_ne!(a, 0);
        self.ops.field_inversions += 1;
        self.pow(a, self.p - 2)
    }
    pub fn eval(&mut self, h: &[u64], x: u64) -> u64 {
        h.iter().rev().fold(0, |a, c| {
            let a = self.mul(a, x);
            self.add(a, *c)
        })
    }
    pub fn prime(&mut self, n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let mut d = 2;
        while d * d <= n {
            self.ops.primality_trial_divisions += 1;
            if n.is_multiple_of(d) {
                return false;
            }
            d += 1;
        }
        true
    }
    pub fn hash(&mut self, domain: &str, words: &[u64]) -> [u8; 32] {
        self.ops.hash_calls += 1;
        hash_words(domain, words)
    }
}
pub fn hash_words(domain: &str, words: &[u64]) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(domain.as_bytes());
    hash.update([0]);
    for word in words {
        hash.update(word.to_be_bytes());
    }
    hash.finalize().into()
}
pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub type Point = (u64, u64);
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub group_order: u64,
    pub generator: Point,
    pub slug: String,
    pub icv1: String,
    pub model: Value,
    pub points: Vec<Point>,
}
pub fn rhs(field: &mut Field, a: u64, b: u64, x: u64) -> u64 {
    let xx = field.mul(x, x);
    let xxx = field.mul(xx, x);
    let ax = field.mul(a, x);
    let sum = field.add(xxx, ax);
    field.add(sum, b)
}
pub fn curve_points(field: &mut Field, a: u64, b: u64) -> Vec<Point> {
    let mut roots = vec![Vec::new(); field.p as usize];
    for y in 0..field.p {
        let square = field.mul(y, y);
        roots[square as usize].push(y);
    }
    let mut points = Vec::new();
    for x in 0..field.p {
        let value = rhs(field, a, b, x);
        for y in &roots[value as usize] {
            points.push((x, *y));
        }
    }
    points
}
pub fn identity(field: &mut Field, a: u64, b: u64, n: u64, points: Vec<Point>) -> Curve {
    let p = field.p;
    let model = json!({"a":a.to_string(),"b":b.to_string(),"field":format!("fp-{p}"),"form":"y^2=x^3+a*x+b","p":p.to_string(),"v":"1"});
    field.ops.hash_calls += 1;
    let hash = digest(&serde_json::to_vec(&model).unwrap());
    let trace = p as i64 + 1 - n as i64;
    let t = if trace < 0 {
        format!("tm{}", -trace)
    } else {
        format!("t{trace}")
    };
    let bits = 64 - p.leading_zeros();
    let aa = field.mul(a, a);
    let aaa = field.mul(aa, a);
    let a4 = field.mul(4 % p, aaa);
    let bb = field.mul(b, b);
    let b27 = field.mul(27 % p, bb);
    let denom = field.add(a4, b27);
    let inv = field.inv(denom);
    let j = field.mul(1728 % p, a4);
    let j = field.mul(j, inv);
    Curve {
        p,
        a,
        b,
        group_order: n,
        generator: points[0],
        slug: format!("icv1-fp{bits}-{t}-{}", &hash[..8]),
        icv1: format!("ICV1:fp-{p}:{trace}:{n}:{j}:unk:unk:r:{}", &hash[..12]),
        model,
        points,
    }
}
pub fn point_add(
    field: &mut Field,
    curve: &Curve,
    p: Option<Point>,
    q: Option<Point>,
) -> Option<Point> {
    field.ops.group_calls += 1;
    let (Some((x, y)), Some((z, w))) = (p, q) else {
        return p.or(q);
    };
    if x == z && field.add(y, w) == 0 {
        return None;
    }
    let (num, den) = if x == z {
        field.ops.group_doublings += 1;
        let xx = field.mul(x, x);
        let triple = field.mul(3, xx);
        (field.add(triple, curve.a), field.mul(2, y))
    } else {
        (field.sub(w, y), field.sub(z, x))
    };
    let inverse = field.inv(den);
    let m = field.mul(num, inverse);
    let square = field.mul(m, m);
    let rx = field.sub(square, x);
    let rx = field.sub(rx, z);
    let delta = field.sub(x, rx);
    let yy = field.mul(m, delta);
    let ry = field.sub(yy, y);
    Some((rx, ry))
}
pub fn trim(mut p: Vec<u64>) -> Vec<u64> {
    while p.last() == Some(&0) {
        p.pop();
    }
    p
}
pub fn univariate_product(field: &mut Field, a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, c) in a.iter().enumerate() {
        for (j, d) in b.iter().enumerate() {
            let t = field.mul(*c, *d);
            out[i + j] = field.add(out[i + j], t);
        }
    }
    trim(out)
}
pub fn membership(field: &mut Field, roots: &[u64]) -> Vec<u64> {
    roots.iter().fold(vec![1], |p, x| {
        let c = field.sub(0, *x);
        univariate_product(field, &p, &[c, 1])
    })
}
pub fn interpolate(field: &mut Field, points: &[Point]) -> Result<Vec<u64>, String> {
    if points.is_empty() {
        return Err("empty interpolation".into());
    }
    let mut xs = BTreeSet::new();
    if points.iter().any(|(x, _)| !xs.insert(*x)) {
        return Err("duplicate interpolation abscissa".into());
    }
    let mut out = vec![0; points.len()];
    for (i, (xi, yi)) in points.iter().enumerate() {
        let mut basis = vec![1];
        let mut denominator = 1;
        for (j, (xj, _)) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            let neg = field.sub(0, *xj);
            basis = univariate_product(field, &basis, &[neg, 1]);
            let diff = field.sub(*xi, *xj);
            denominator = field.mul(denominator, diff);
        }
        let inv = field.inv(denominator);
        let scale = field.mul(*yi, inv);
        for (j, c) in basis.into_iter().enumerate() {
            let value = field.mul(c, scale);
            out[j] = field.add(out[j], value);
        }
    }
    Ok(trim(out))
}
pub fn signed_points(field: &mut Field, roots: &[u64], h: &[u64]) -> Vec<Point> {
    let mut points = Vec::new();
    for x in roots {
        let y = field.eval(h, *x);
        let neg = field.sub(0, y);
        let low = y.min(neg);
        let high = y.max(neg);
        points.push((*x, low));
        if low != high {
            points.push((*x, high));
        }
    }
    points
}
pub fn dump(curve: &Curve, roots: &[u64], points: &[Point]) -> Value {
    let mut keys: Vec<u64> = points
        .iter()
        .map(|(x, y)| ((x + 1) << 1) | u64::from(*y > curve.p - y))
        .collect();
    keys.sort_unstable();
    let bytes: Vec<u8> = keys.into_iter().flat_map(u64::to_be_bytes).collect();
    let point_hash = digest(&bytes);
    let params = json!({"xs":roots.iter().map(u64::to_string).collect::<Vec<_>>().join(";")});
    let preimage = json!({"schema":"ecbench.factor_base/v1","curve":curve.slug,"family":"explicit-x","params":params,"columns":roots.len(),"signed_points":points.len(),"points_sha256":point_hash});
    let fb_hash = digest(&serde_json::to_vec(&preimage).unwrap());
    let point_records:Vec<_>=points.iter().map(|(x,y)|{let col=roots.iter().position(|u|u==x).unwrap();json!({"x":format!("0x{x:x}"),"y":format!("0x{y:x}"),"col":col,"coef":if *y<=curve.p-y{1}else{curve.group_order-1}})}).collect();
    json!({"schema":"ecbench.factor_base_dump/v1","curve":{"slug":curve.slug,"icv1":curve.icv1,"family":"prime","construction":"fixed_hash_selected_graph_ladder","field_degree":null,"field_bits":64-curve.p.leading_zeros(),"group_order":curve.group_order,"r":curve.group_order,"cofactor":1,"generator":[format!("0x{:x}",curve.generator.0),format!("0x{:x}",curve.generator.1)],"automorphisms_available":2,"registered":false,"ec1":null,"curve_uid":null},"factor_base":{"fb_id":format!("FB1h{}",&fb_hash[..12]),"fb_sha256":fb_hash,"family":"explicit-x","params":params,"description":"explicit x coordinates; both signs, one column per supplied x","signed_points":points.len(),"abscissae":roots.len(),"columns":roots.len(),"dimension":null,"points_sha256":point_hash},"build_adds":0,"build_doubles":0,"points":point_records})
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Term {
    pub exponents: Vec<u32>,
    pub coefficient: u64,
}
pub type Poly = BTreeMap<Vec<u32>, u64>;
fn term(poly: &mut Poly, e: &[u32], c: i128, p: u64) {
    let c = c.rem_euclid(p as i128) as u64;
    let value = (poly.get(e).copied().unwrap_or(0) + c) % p;
    if value == 0 {
        poly.remove(e);
    } else {
        poly.insert(e.to_vec(), value);
    }
}
fn terms(poly: Poly) -> Vec<Term> {
    poly.into_iter()
        .map(|(exponents, coefficient)| Term {
            exponents,
            coefficient,
        })
        .collect()
}
fn member_poly(coefficients: &[u64], slot: usize, n: usize) -> Vec<Term> {
    coefficients
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if *c == 0 {
                None
            } else {
                let mut e = vec![0; n];
                e[slot] = i as u32;
                Some(Term {
                    exponents: e,
                    coefficient: *c,
                })
            }
        })
        .collect()
}
pub fn dense(curve: &Curve, member: &[u64], r: u64) -> Vec<Vec<Term>> {
    let mut s = Poly::new();
    let p = curve.p;
    let a = curve.a as i128;
    let b = curve.b as i128;
    let r = r as i128;
    for (e, c) in [
        ([2, 2], 1),
        ([1, 1], -2 * a - 2 * r * r),
        ([0, 0], a * a - 4 * b * r),
        ([2, 1], -2 * r),
        ([1, 2], -2 * r),
        ([2, 0], r * r),
        ([0, 2], r * r),
        ([1, 0], -2 * a * r - 4 * b),
        ([0, 1], -2 * a * r - 4 * b),
    ] {
        term(&mut s, &e, c, p);
    }
    vec![
        member_poly(member, 0, 2),
        member_poly(member, 1, 2),
        terms(s),
    ]
}
pub fn secant(
    curve: &Curve,
    h: &[u64],
    member: &[u64],
    target: Point,
    signs: [i8; 2],
    complete_graph: bool,
) -> Vec<Vec<Term>> {
    let p = curve.p;
    let (r, s) = (target.0 as i128, target.1 as i128);
    let a = curve.a as i128;
    let mut g1 = Poly::new();
    for (e, c) in [
        ([0, 0, 2], 1),
        ([1, 0, 0], -1),
        ([0, 1, 0], -1),
        ([0, 0, 0], -r),
    ] {
        term(&mut g1, &e, c, p);
    }
    let mut g2 = Poly::new();
    for (e, c) in [
        ([1, 1, 0], 1),
        ([0, 0, 2], -r),
        ([0, 0, 1], -2 * s),
        ([0, 0, 0], -r * r - a),
    ] {
        term(&mut g2, &e, c, p);
    }
    let mut out = vec![terms(g1), terms(g2)];
    for (slot, sign) in signs.iter().enumerate() {
        let mut q = Poly::new();
        for (i, c) in h.iter().enumerate() {
            let mut e = [0, 0, 0];
            e[slot] = i as u32;
            term(&mut q, &e, (*sign as i128) * (*c as i128), p);
        }
        let mut mx = [0, 0, 1];
        mx[slot] = 1;
        term(&mut q, &mx, -1, p);
        term(&mut q, &[0, 0, 1], r, p);
        term(&mut q, &[0, 0, 0], s, p);
        out.push(terms(q));
        if !complete_graph {
            out.push(member_poly(member, slot, 3));
        }
    }
    out
}
pub fn evaluate(poly: &[Term], point: &[u64], p: u64) -> u64 {
    poly.iter().fold(0, |s, t| {
        let value = point
            .iter()
            .zip(&t.exponents)
            .fold(t.coefficient, |c, (x, n)| (0..*n).fold(c, |v, _| v * x % p));
        (s + value) % p
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interpolation_reproduces_supplied_quadratic() {
        let mut f = Field::new(17);
        let h = interpolate(&mut f, &[(5, 1), (6, 3), (10, 6)]).unwrap();
        assert_eq!(h, vec![9, 9, 4]);
    }
    #[test]
    fn duplicate_abscissae_rejected() {
        let mut f = Field::new(17);
        assert!(interpolate(&mut f, &[(1, 2), (1, 3)]).is_err());
    }
    #[test]
    fn known_group_and_secant_fixture() {
        let mut f = Field::new(17);
        let points = curve_points(&mut f, 2, 2);
        assert_eq!(points.len(), 18);
        let c = identity(&mut f, 2, 2, 19, points);
        assert_eq!(
            point_add(&mut f, &c, Some((5, 1)), Some((6, 3))),
            Some((10, 6))
        );
        let mem = membership(&mut f, &[5, 6, 10, 16]);
        for poly in secant(&c, &[9, 9, 4], &mem, (10, 6), [1, 1], true) {
            assert_eq!(evaluate(&poly, &[5, 6, 2], 17), 0);
        }
        for poly in dense(&c, &mem, 10) {
            assert_eq!(evaluate(&poly, &[5, 6], 17), 0);
        }
    }
    #[test]
    fn canonical_ids_match_committed_fixed_fixture() {
        let mut f = Field::new(17);
        let pts = curve_points(&mut f, 2, 2);
        let c = identity(&mut f, 2, 2, 19, pts);
        assert_eq!(c.slug, "icv1-fp5-tm1-a77cc6af");
        let pts = signed_points(&mut f, &[5, 6, 10, 16], &[9, 9, 4]);
        let d = dump(&c, &[5, 6, 10, 16], &pts);
        assert_eq!(d["factor_base"]["fb_id"], "FB1hed5b19904cc8");
    }
}
