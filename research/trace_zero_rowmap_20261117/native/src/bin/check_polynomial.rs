//! Separate schoolbook polynomial/group arithmetic. Does not import production
//! ECC, field, SAT or linearized-solver modules. This is a producer-written
//! cross-check, not an independently signed verifier.
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};
type Point = Option<(BigUint, BigUint)>;
#[derive(Clone)]
struct Field {
    n: usize,
    modulus: BigUint,
}
fn zero() -> BigUint {
    BigUint::default()
}
fn one() -> BigUint {
    1u32.into()
}
impl Field {
    fn new(n: usize) -> Self {
        let terms: &[usize] = match n {
            5 => &[0, 2],
            7 => &[0, 1],
            9 => &[0, 4],
            83 => &[0, 1, 2, 45],
            131 => &[0, 1, 2, 13],
            _ => panic!("unsupported field"),
        };
        let mut modulus = one() << n;
        for i in terms {
            modulus ^= one() << i;
        }
        Self { n, modulus }
    }
    fn reduce(&self, mut a: BigUint) -> BigUint {
        while a.bits() > self.n as u64 {
            let shift = a.bits() as usize - 1 - self.n;
            a ^= &self.modulus << shift;
        }
        a
    }
    fn mul(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let mut r = zero();
        for i in 0..b.bits() {
            if b.bit(i) {
                r ^= a << i as usize;
            }
        }
        self.reduce(r)
    }
    fn square(&self, a: &BigUint) -> BigUint {
        let mut r = zero();
        for i in 0..a.bits() {
            if a.bit(i) {
                r ^= one() << (2 * i as usize);
            }
        }
        self.reduce(r)
    }
    fn inv(&self, a: &BigUint) -> BigUint {
        assert!(*a != zero());
        let (mut u, mut v, mut g, mut h) = (a.clone(), self.modulus.clone(), one(), zero());
        while u != one() {
            if u.bits() < v.bits() {
                std::mem::swap(&mut u, &mut v);
                std::mem::swap(&mut g, &mut h);
            }
            assert!(u != zero());
            let j = (u.bits() - v.bits()) as usize;
            u ^= &v << j;
            g ^= &h << j;
        }
        self.reduce(g)
    }
    fn point(&self, v: &Value) -> Point {
        if v.is_null() {
            return None;
        }
        let x = BigUint::parse_bytes(v[0].as_str().unwrap().as_bytes(), 16).unwrap();
        let y = BigUint::parse_bytes(v[1].as_str().unwrap().as_bytes(), 16).unwrap();
        assert!(x.bits() <= self.n as u64 && y.bits() <= self.n as u64);
        Some((x, y))
    }
    fn on_curve(&self, p: &Point) -> bool {
        match p {
            None => true,
            Some((x, y)) => self.square(y) ^ self.mul(x, y) == self.mul(&self.square(x), x) ^ one(),
        }
    }
    fn neg(&self, p: &Point) -> Point {
        p.as_ref().map(|(x, y)| (x.clone(), x ^ y))
    }
    fn add(&self, p: &Point, q: &Point) -> Point {
        let (Some((x, y)), Some((u, v))) = (p, q) else {
            return if p.is_none() { q.clone() } else { p.clone() };
        };
        if x == u && y ^ v == *x {
            return None;
        }
        let lambda = if x == u {
            assert_eq!(y, v);
            x ^ self.mul(y, &self.inv(x))
        } else {
            self.mul(&(y ^ v), &self.inv(&(x ^ u)))
        };
        let z = self.square(&lambda) ^ &lambda ^ x ^ u;
        let w = self.mul(&lambda, &(x ^ &z)) ^ &z ^ y;
        Some((z, w))
    }
    fn trace_tables(&self) -> (Vec<BigUint>, Vec<BigUint>) {
        let mut traces = Vec::new();
        let mut halves = Vec::new();
        for i in 0..self.n {
            let a = one() << i;
            let mut term = a.clone();
            let mut t = zero();
            for _ in 0..self.n {
                t ^= &term;
                term = self.square(&term);
            }
            assert!(t == zero() || t == one());
            traces.push(t);
            let mut h = zero();
            term = a;
            for _ in 0..=(self.n - 1) / 2 {
                h ^= &term;
                term = self.square(&self.square(&term));
            }
            halves.push(h);
        }
        (traces, halves)
    }
    fn linear(&self, a: &BigUint, columns: &[BigUint]) -> BigUint {
        let mut r = zero();
        for (i, c) in columns.iter().enumerate() {
            if a.bit(i as u64) {
                r ^= c;
            }
        }
        r
    }
    fn kernel_member(&self, p: &Point, t: &[BigUint], h: &[BigUint]) -> bool {
        match p {
            None => true,
            Some((x, y)) => {
                self.on_curve(p)
                    && self.linear(x, t) == zero()
                    && self.linear(&(y ^ self.mul(x, &self.linear(x, h))), t) == zero()
            }
        }
    }
}
fn span(values: impl IntoIterator<Item = BigUint>, n: usize) -> Vec<BigUint> {
    let mut rows = vec![zero(); n];
    for mut x in values {
        while x != zero() {
            let bit = x.bits() as usize - 1;
            if rows[bit] == zero() {
                rows[bit] = x;
                break;
            }
            x ^= &rows[bit];
        }
    }
    rows.into_iter().filter(|x| *x != zero()).collect()
}
fn contains(columns: &[BigUint], mut target: BigUint, n: usize) -> bool {
    let rows = span(columns.iter().cloned(), n);
    for r in rows.iter().rev() {
        if target.bit(r.bits() - 1) {
            target ^= r;
        }
    }
    target == zero()
}
fn verify_false(f: &Field, b: &[Point], t: &Point) -> Value {
    if b.len() <= 32 {
        let mut pairs = BTreeSet::new();
        for p in b {
            for q in b {
                pairs.insert(f.add(p, q));
            }
        }
        assert!(b.iter().all(|r| !pairs.contains(&f.add(t, &f.neg(r)))));
        return json!({"method":"complete independent pair table","base_size":b.len()});
    }
    let xs = span(b.iter().map(|p| p.as_ref().unwrap().0.clone()), f.n);
    let mut products = Vec::new();
    for x in &xs {
        for y in &xs {
            products.push(f.mul(x, y));
        }
    }
    let ws = span(products, f.n);
    let x2: Vec<_> = xs.iter().map(|x| f.square(x)).collect();
    let w2: Vec<_> = ws.iter().map(|w| f.square(w)).collect();
    let mut systems = 0;
    for r in b {
        let residual = f.add(t, &f.neg(r));
        let (z, _) = residual.expect("identity residual cannot be refuted with a signed base");
        let z2 = f.square(&z);
        let mut columns: Vec<_> = x2.iter().map(|x| f.mul(&z2, x)).collect();
        columns.extend(ws.iter().zip(&w2).map(|(w, s)| s ^ f.mul(&z, w)));
        assert!(
            !contains(&columns, one(), f.n),
            "unproven refutation: necessary system consistent"
        );
        systems += 1;
    }
    json!({"method":"every necessary symmetric system inconsistent in independent polynomial arithmetic","base_size":b.len(),"systems":systems,"columns":xs.len()+ws.len()})
}
fn main() {
    let a: Vec<_> = env::args().skip(1).collect();
    let input: Value = serde_json::from_slice(&fs::read(&a[0]).unwrap()).unwrap();
    let cases = input["cases"].as_array().unwrap();
    let mut fields = BTreeMap::new();
    for c in cases {
        let n = c["n"].as_u64().unwrap() as usize;
        fields.entry(n).or_insert_with(|| {
            let f = Field::new(n);
            let (t, h) = f.trace_tables();
            (f, t, h)
        });
    }
    let mut admitted = 0;
    let mut negatives = Vec::new();
    let mut known_witnesses = 0;
    for c in cases {
        let (f, tr, h) = &fields[&(c["n"].as_u64().unwrap() as usize)];
        let b: Vec<_> = c["base"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| f.point(p))
            .collect();
        let target = f.point(&c["target"]);
        assert!(f.kernel_member(&target, tr, h));
        for p in &b {
            assert!(p.is_some() && f.kernel_member(p, tr, h));
            admitted += 1;
        }
        if c["truth"] == true {
            let ids = c["truth_witness"].as_array().unwrap();
            let sum = ids
                .iter()
                .fold(None, |s, i| f.add(&s, &b[i.as_u64().unwrap() as usize]));
            assert_eq!(sum, target);
            known_witnesses += 1;
        } else {
            let mut result = verify_false(f, &b, &target);
            result["instance"] = c["id"].clone();
            negatives.push(result);
        }
    }
    let mut outputs = Vec::new();
    for path in &a[1..] {
        let mut replayed = 0;
        let mut rows = 0;
        let mut seen = BTreeSet::new();
        for line in fs::read_to_string(path).unwrap().lines() {
            let r: Value = serde_json::from_str(line).unwrap();
            let c = cases.iter().find(|c| c["id"] == r["instance"]).unwrap();
            let (f, _, _) = &fields[&(c["n"].as_u64().unwrap() as usize)];
            let z = &r["result"];
            assert!(seen.insert((
                r["instance"].to_string(),
                r["arm"].to_string(),
                r["replicate"].to_string()
            )));
            match z["outcome"].as_str().unwrap() {
                "Found" => {
                    assert_eq!(c["truth"], true);
                    let ps = z["points"].as_array().unwrap();
                    assert_eq!(ps.len(), 3);
                    for p in ps {
                        assert!(c["base"].as_array().unwrap().contains(p));
                        assert!(f.on_curve(&f.point(p)));
                    }
                    assert_eq!(
                        ps.iter().fold(None, |s, p| f.add(&s, &f.point(p))),
                        f.point(&c["target"])
                    );
                    replayed += 1;
                }
                "Refuted" => assert_eq!(c["truth"], false),
                "Inconclusive" => (),
                _ => panic!("unknown disposition"),
            };
            rows += 1;
        }
        outputs.push(
            json!({"path":path,"rows":rows,"saved_relations_replayed":replayed,"unique_grid":true}),
        );
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"trace-zero-independent-polynomial-check/v1","arithmetic":"schoolbook polynomial multiply/reduce and Euclid inversion; separate from production code","standing":"producer-written cross-check, no independently signed verdict","cases":cases.len(),"base_admissions_with_repeats":admitted,"known_witnesses":known_witnesses,"negative_proofs":negatives,"row_files":outputs})).unwrap());
}
