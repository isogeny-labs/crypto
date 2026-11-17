//! Integral CM Smith reduction and actual odd-cover kernel/torsion pairings.
use crate::binary_ecc::BinaryCurve;
use crate::{large_field as lf, round04::read};
use lf::{E, F, P};
use num_bigint::{BigInt as Z, BigUint};
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use serde_json::{json, Value};
use std::path::Path;
#[derive(Clone, Debug, PartialEq, Eq)]
struct C {
    a: Z,
    b: Z,
}
impl C {
    fn n(a: i64, b: i64) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
        }
    }
    fn add(&self, v: &Self) -> Self {
        Self {
            a: &self.a + &v.a,
            b: &self.b + &v.b,
        }
    }
    fn neg(&self) -> Self {
        Self {
            a: -&self.a,
            b: -&self.b,
        }
    }
    fn sub(&self, v: &Self) -> Self {
        self.add(&v.neg())
    }
    fn mul(&self, v: &Self) -> Self {
        Self {
            a: &self.a * &v.a - 2 * &self.b * &v.b,
            b: &self.a * &v.b + &self.b * &v.a - &self.b * &v.b,
        }
    }
    fn bar(&self) -> Self {
        Self {
            a: &self.a - &self.b,
            b: -&self.b,
        }
    }
    fn norm(&self) -> Z {
        let v = self.mul(&self.bar());
        assert!(v.b.is_zero());
        v.a
    }
    fn zero(&self) -> bool {
        self.a.is_zero() && self.b.is_zero()
    }
    fn divrem(&self, y: &Self) -> (Self, Self) {
        let d = y.norm();
        assert!(!d.is_zero());
        let v = self.mul(&y.bar());
        let a = v.a.div_floor(&d);
        let b = v.b.div_floor(&d);
        let mut best = None;
        for i in -2..=2 {
            for j in -2..=2 {
                let q = Self {
                    a: &a + i,
                    b: &b + j,
                };
                let r = self.sub(&q.mul(y));
                let n = r.norm();
                if best
                    .as_ref()
                    .is_none_or(|(nn, _, _): &(Z, Self, Self)| n < *nn)
                {
                    best = Some((n, q, r));
                }
            }
        }
        let (n, q, r) = best.unwrap();
        assert!(n < d);
        (q, r)
    }
    fn val(&self) -> Value {
        json!({"a":self.a.to_string(),"b":self.b.to_string()})
    }
}
type M = Vec<Vec<C>>;
fn id(n: usize) -> M {
    (0..n)
        .map(|i| (0..n).map(|j| C::n(i64::from(i == j), 0)).collect())
        .collect()
}
fn mm(a: &M, b: &M) -> M {
    a.iter()
        .map(|r| {
            (0..b[0].len())
                .map(|j| {
                    r.iter()
                        .enumerate()
                        .fold(C::n(0, 0), |v, (k, x)| v.add(&x.mul(&b[k][j])))
                })
                .collect()
        })
        .collect()
}
fn det(a: &M) -> C {
    if a.len() == 1 {
        return a[0][0].clone();
    }
    (0..a.len()).fold(C::n(0, 0), |s, j| {
        let m = a[1..]
            .iter()
            .map(|r| {
                r.iter()
                    .enumerate()
                    .filter(|(k, _)| *k != j)
                    .map(|(_, v)| v.clone())
                    .collect()
            })
            .collect();
        let v = a[0][j].mul(&det(&m));
        s.add(&if j % 2 == 0 { v } else { v.neg() })
    })
}
fn inverse(a: &M) -> M {
    let n = a.len();
    let d = det(a);
    assert_eq!(d.norm(), Z::one());
    let out = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    let m = (0..n)
                        .filter(|r| *r != j)
                        .map(|r| {
                            (0..n)
                                .filter(|c| *c != i)
                                .map(|c| a[r][c].clone())
                                .collect()
                        })
                        .collect();
                    let mut v = det(&m);
                    if (i + j) % 2 == 1 {
                        v = v.neg();
                    }
                    let (q, r) = v.divrem(&d);
                    assert!(r.zero());
                    q
                })
                .collect()
        })
        .collect();
    assert_eq!(mm(a, &out), id(n));
    assert_eq!(mm(&out, a), id(n));
    out
}
fn row(a: &mut M, i: usize, j: usize, q: &C) {
    for k in 0..a.len() {
        a[i][k] = a[i][k].add(&q.mul(&a[j][k]));
    }
}
fn col(a: &mut M, i: usize, j: usize, q: &C) {
    for r in a {
        r[i] = r[i].add(&q.mul(&r[j]));
    }
}
fn smith(b: &M) -> (M, M, M, usize) {
    let n = b.len();
    let (mut a, mut u, mut v) = (b.clone(), id(n), id(n));
    let mut ops = 0;
    for k in 0..n {
        let (i, j) = (k..n)
            .flat_map(|i| (k..n).map(move |j| (i, j)))
            .filter(|(i, j)| !a[*i][*j].zero())
            .min_by_key(|(i, j)| a[*i][*j].norm())
            .unwrap();
        a.swap(k, i);
        u.swap(k, i);
        for m in [&mut a, &mut v] {
            for r in m {
                r.swap(k, j);
            }
        }
        loop {
            ops += 1;
            assert!(ops <= 100000);
            assert!(a
                .iter()
                .flatten()
                .chain(u.iter().flatten())
                .chain(v.iter().flatten())
                .all(|c| c.a.bits() < 4096 && c.b.bits() < 4096));
            if let Some(i) = (k + 1..n).find(|i| !a[*i][k].zero()) {
                let (q, r) = a[i][k].divrem(&a[k][k]);
                row(&mut a, i, k, &q.neg());
                row(&mut u, i, k, &q.neg());
                if !r.zero() {
                    a.swap(i, k);
                    u.swap(i, k);
                }
                continue;
            }
            if let Some(j) = (k + 1..n).find(|j| !a[k][*j].zero()) {
                let (q, r) = a[k][j].divrem(&a[k][k]);
                col(&mut a, j, k, &q.neg());
                col(&mut v, j, k, &q.neg());
                if !r.zero() {
                    for m in [&mut a, &mut v] {
                        for r in m {
                            r.swap(k, j);
                        }
                    }
                }
                continue;
            }
            if let Some((i, _)) = (k + 1..n)
                .flat_map(|i| (k + 1..n).map(move |j| (i, j)))
                .find(|(i, j)| !a[*i][*j].divrem(&a[k][k]).1.zero())
            {
                row(&mut a, k, i, &C::n(1, 0));
                row(&mut u, k, i, &C::n(1, 0));
                continue;
            }
            break;
        }
        if a[k][k].b.is_zero() && a[k][k].a.is_negative() {
            for m in [&mut a, &mut u] {
                for x in &mut m[k] {
                    *x = x.neg();
                }
            }
        }
    }
    assert_eq!(mm(&mm(&u, b), &v), a);
    assert_eq!(det(&u).norm(), Z::one());
    assert_eq!(det(&v).norm(), Z::one());
    for i in 0..n {
        for j in 0..n {
            if i != j {
                assert!(a[i][j].zero());
            }
        }
        if i + 1 < n {
            assert!(a[i + 1][i + 1].divrem(&a[i][i]).1.zero());
        }
    }
    (u, a, v, ops)
}
fn matrixv(a: &M) -> Value {
    json!(a
        .iter()
        .map(|r| r.iter().map(C::val).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn embed(a: &M) -> Vec<Vec<Z>> {
    (0..2 * a.len())
        .map(|i| {
            (0..2 * a.len())
                .map(|j| {
                    let c = &a[i / 2][j / 2];
                    match (i % 2, j % 2) {
                        (0, 0) => c.a.clone(),
                        (0, 1) => -2 * &c.b,
                        (1, 0) => c.b.clone(),
                        _ => &c.a - &c.b,
                    }
                })
                .collect()
        })
        .collect()
}
fn lit(a: &[Vec<Z>]) -> String {
    format!(
        "![{}]",
        a.iter()
            .map(|r| format!(
                "![{}]",
                r.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn times(c: &BinaryCurve, p: &P, n: &BigUint) -> P {
    crate::binary_ecc::curve::scalar_mul(c, p, n)
}
fn ordmat(t: i64, l: i64) -> u64 {
    let a = [[0i64, -2], [1, t]];
    let mut v = [[1, 0], [0, 1]];
    for n in 1..=10000 {
        v = std::array::from_fn(|i| {
            std::array::from_fn(|j| (0..2).map(|k| v[i][k] * a[k][j]).sum::<i64>().rem_euclid(l))
        });
        if v == [[1, 0], [0, 1]] {
            return n;
        }
    }
    panic!("torsion order cap")
}
fn cardinal(t: i64, k: u32) -> BigUint {
    let (mut a, mut b) = (Z::from(2), Z::from(t));
    for _ in 1..k {
        let c = &b * t - &a * 2;
        a = b;
        b = c;
    }
    ((Z::one() << k) + Z::one() - b).to_biguint().unwrap()
}
fn basis(f: &F, c: &BinaryCurve, n: &BigUint, l: u64) -> (P, P, usize, u32) {
    let mut co = n.clone();
    let mut valuation = 0;
    while (&co % l).is_zero() {
        co /= l;
        valuation += 1;
    }
    let mut first = None;
    for x in 1..=20000 {
        let Some(p) = f.point(c, &f.e(x)) else {
            continue;
        };
        let mut p = times(c, &p, &co);
        if p == P::Infinity {
            continue;
        }
        for _ in 0..valuation {
            let q = lf::times(c, &p, l);
            if q == P::Infinity {
                break;
            }
            p = q;
        }
        assert_eq!(lf::times(c, &p, l), P::Infinity);
        if let Some(q) = &first {
            if !(0..l).any(|i| lf::times(c, q, i) == p) {
                return (q.clone(), p, x as usize, valuation);
            }
        } else {
            first = Some(p);
        }
    }
    panic!("independent torsion basis cap")
}
fn line(f: &F, c: &BinaryCurve, p: &P, q: &P, r: &P) -> Option<E> {
    if *p == P::Infinity || *q == P::Infinity {
        return Some(f.o());
    }
    let (P::Affine { x, y }, P::Affine { x: u, y: v }, P::Affine { x: rx, y: ry }) = (p, q, r)
    else {
        return None;
    };
    if (x == u && y != v) || (x.is_zero() && p == q) {
        let a = rx.add(x);
        return (!a.is_zero()).then_some(a);
    }
    let m = if p == q {
        x.add(&f.mul(y, &f.inv(x)))
    } else {
        f.mul(&y.add(v), &f.inv(&x.add(u)))
    };
    let z = ry.add(&f.mul(&m, &rx.add(x))).add(y);
    if z.is_zero() {
        return None;
    }
    match lf::plus(c, p, q) {
        P::Infinity => Some(z),
        P::Affine { x: s, .. } => {
            let d = rx.add(&s);
            (!d.is_zero()).then(|| f.mul(&z, &f.inv(&d)))
        }
    }
}
fn miller(f: &F, c: &BinaryCurve, p: &P, r: &P, l: u64) -> Option<E> {
    let mut v = p.clone();
    let mut z = f.o();
    for i in (0..63 - l.leading_zeros()).rev() {
        z = f.mul(&f.sq(&z), &line(f, c, &v, &v, r)?);
        v = lf::plus(c, &v, &v);
        if l >> i & 1 == 1 {
            z = f.mul(&z, &line(f, c, &v, p, r)?);
            v = lf::plus(c, &v, p);
        }
    }
    assert_eq!(v, P::Infinity);
    Some(z)
}
fn weil(f: &F, c: &BinaryCurve, p: &P, q: &P, l: u64) -> E {
    if *p == P::Infinity || *q == P::Infinity || p == q {
        return f.o();
    }
    for x in 1..=1000 {
        let Some(s) = f.point(c, &f.e(x)) else {
            continue;
        };
        let ns = lf::neg(&s);
        let qp = lf::plus(c, q, &s);
        let pn = lf::plus(c, p, &ns);
        if let (Some(a), Some(b), Some(d), Some(e)) = (
            miller(f, c, p, &qp, l),
            miller(f, c, p, &s, l),
            miller(f, c, q, &ns, l),
            miller(f, c, q, &pn, l),
        ) {
            let z = f.mul(&f.mul(&a, &d), &f.inv(&f.mul(&b, &e)));
            assert_eq!(f.powu(&z, l), f.o());
            return z;
        }
    }
    panic!("Weil auxiliary cap")
}
fn op(f: &F, c: &BinaryCurve, a: &C, p: &P, trace: i64) -> P {
    let mut w = f.frobp(p, 1);
    if trace == 1 {
        w = lf::neg(&w)
    }
    let x = lf::times(c, p, a.a.mod_floor(&Z::from(105)).to_u64().unwrap());
    let y = lf::times(c, &w, a.b.mod_floor(&Z::from(105)).to_u64().unwrap());
    lf::plus(c, &x, &y)
}
fn apply(f: &F, c: &BinaryCurve, a: &M, p: &[P], trace: i64) -> Vec<P> {
    a.iter()
        .map(|r| {
            r.iter().zip(p).fold(P::Infinity, |v, (a, p)| {
                lf::plus(c, &v, &op(f, c, a, p, trace))
            })
        })
        .collect()
}
fn express(c: &BinaryCurve, p: &P, q: &P, r: &P, l: u64) -> [u64; 2] {
    for i in 0..l {
        for j in 0..l {
            if lf::plus(c, &lf::times(c, p, i), &lf::times(c, q, j)) == *r {
                return [i, j];
            }
        }
    }
    panic!("torsion span")
}
pub fn run(dir: &Path) {
    let doc = read(&dir.join("g16-diagonal-cover-section-bases.json"));
    let r = &doc["records"][0];
    assert_eq!(r["B_columns"], doc["records"][2]["B_columns"]);
    let cols = r["B_columns"].as_array().unwrap();
    let b = (0..4)
        .map(|i| {
            (0..4)
                .map(|j| {
                    let v = &cols[j][i];
                    C {
                        a: v["a"].as_str().unwrap().parse().unwrap(),
                        b: v["b"].as_str().unwrap().parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect::<M>();
    let (u, d, v, ops) = smith(&b);
    let ui = inverse(&u);
    let vi = inverse(&v);
    let diagonal = (0..4).map(|i| d[i][i].clone()).collect::<Vec<_>>();
    assert_eq!(
        diagonal,
        vec![C::n(1, 0), C::n(1, 0), C::n(3, 0), C::n(105, 0)]
    );
    let orders = [3, 5, 7].map(|l| ordmat(-1, l));
    let field_degree = orders.iter().fold(1u64, |a, b| a.lcm(b)) as u32;
    assert_eq!(field_degree, 168);
    let orders_plus = [3, 5, 7].map(|l| ordmat(1, l));
    assert!(orders_plus.iter().all(|d| field_degree as u64 % d == 0));
    let f = F::find(field_degree);
    assert!(f.irreducible());
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound16Sections\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound17Theta\n");
    lean.push_str(&format!("def U : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef B : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef V : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef D : Matrix (Fin 8) (Fin 8) ℤ := {}\ntheorem actual_odd_Smith : U*B*V=D := by decide +kernel\ntheorem actual_odd_kernel_order : 3^2*105^2=99225 := by norm_num\n",lit(&embed(&u)),lit(&embed(&b)),lit(&embed(&v)),lit(&embed(&d))));
    lean.push_str(&format!("def Ui : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef Vi : Matrix (Fin 8) (Fin 8) ℤ := {}\ntheorem actual_U_inverse : U*Ui=1 ∧ Ui*U=1 := by decide +kernel\ntheorem actual_V_inverse : V*Vi=1 ∧ Vi*V=1 := by decide +kernel\n",lit(&embed(&ui)),lit(&embed(&vi))));
    for g in [0usize, 2] {
        let trace = if g == 0 { -1 } else { 1 };
        let c = f.curve(f.e(u64::from(trace == 1)), f.o());
        let n = cardinal(trace, field_degree);
        let mut bases = vec![];
        let mut data = vec![];
        for l in [3u64, 5, 7] {
            let (p, q, attempts, valuation) = basis(&f, &c, &n, l);
            let z = weil(&f, &c, &p, &q, l);
            assert_ne!(z, f.o());
            assert_eq!(weil(&f, &c, &q, &p, l), f.inv(&z));
            assert_eq!(weil(&f, &c, &lf::times(&c, &p, 2), &q, l), f.sq(&z));
            data.push(json!({"prime":l,"P":lf::pj(&p),"Q":lf::pj(&q),"point_attempts":attempts,"cardinality_prime_valuation":valuation,"primitive_Weil_pairing":lf::ej(&z)}));
            bases.push((l, p, q, z));
        }
        let p105 = bases
            .iter()
            .fold(P::Infinity, |z, (_, p, _, _)| lf::plus(&c, &z, p));
        let q105 = bases
            .iter()
            .fold(P::Infinity, |z, (_, _, q, _)| lf::plus(&c, &z, q));
        assert_eq!(lf::times(&c, &p105, 105), P::Infinity);
        let seeds = [bases[0].1.clone(), bases[0].2.clone(), p105, q105];
        let orders = [3u64, 3, 105, 105];
        let mut generators = vec![];
        for (i, p) in seeds.iter().enumerate() {
            let mut z = vec![P::Infinity; 4];
            z[if i < 2 { 2 } else { 3 }] = p.clone();
            let x = apply(&f, &c, &v, &z, trace);
            assert!(apply(&f, &c, &b, &x, trace)
                .iter()
                .all(|p| *p == P::Infinity));
            assert!(x.iter().all(|p| lf::times(&c, p, orders[i]) == P::Infinity));
            for l in [3, 5, 7] {
                if orders[i] % l == 0 {
                    assert!(x
                        .iter()
                        .any(|p| lf::times(&c, p, orders[i] / l) != P::Infinity));
                }
            }
            generators.push(x);
        }
        let dd = [3u64, 21, 105, 285];
        let mut pairing_records = vec![];
        for (l, p, q, zeta) in &bases {
            let weight = 105 / l;
            let inverse = (1..*l).find(|i| i * weight % l == 1).unwrap();
            let proj = weight * inverse;
            let xs = generators
                .iter()
                .map(|v| v.iter().map(|p| lf::times(&c, p, proj)).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let coeff = xs
                .iter()
                .map(|v| {
                    v.iter()
                        .map(|x| express(&c, p, q, x, *l))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let mut omega = vec![vec![0u64; 4]; 4];
            let mut checks = 0;
            for i in 0..4 {
                for j in 0..4 {
                    let mut value = f.o();
                    for k in 0..4 {
                        if dd[k] % l != 0 {
                            assert_eq!(xs[i][k], P::Infinity);
                            assert_eq!(xs[j][k], P::Infinity);
                            continue;
                        }
                        let e = weil(&f, &c, &xs[i][k], &xs[j][k], *l);
                        let exponent = (dd[k] / l) % l;
                        value = f.mul(&value, &f.powu(&e, exponent));
                        let a = coeff[i][k];
                        let b = coeff[j][k];
                        omega[i][j] = (omega[i][j]
                            + exponent * ((a[0] * b[1] + l * l - a[1] * b[0]) % l))
                            % l;
                        checks += 1;
                    }
                    assert_eq!(value, f.o());
                    assert_eq!(omega[i][j], 0);
                }
            }
            let km = (0..8)
                .map(|i| {
                    (0..4)
                        .map(|j| Z::from(coeff[j][i / 2][i % 2]))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let om = (0..8)
                .map(|i| {
                    (0..8)
                        .map(|j| {
                            if i / 2 == j / 2 && i % 2 != j % 2 && dd[i / 2] % l == 0 {
                                Z::from(if i % 2 == 0 {
                                    (dd[i / 2] / l) % l
                                } else {
                                    (l - (dd[i / 2] / l) % l) % l
                                })
                            } else {
                                Z::zero()
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            lean.push_str(&format!("def K{g}_{l} : Matrix (Fin 8) (Fin 4) (ZMod {l}) := {}\ndef Omega{g}_{l} : Matrix (Fin 8) (Fin 8) (ZMod {l}) := {}\ntheorem actual_kernel_isotropic{g}_{l} : (K{g}_{l}).transpose*Omega{g}_{l}*K{g}_{l}=0 := by decide +kernel\n",lit(&km),lit(&om)));
            pairing_records.push(json!({"prime":l,"kernel_generator_coefficients":coeff,"diagonal_pairing_exponents":omega,"Miller_coordinate_pairing_checks":checks,"K":km.iter().map(|r|r.iter().map(ToString::to_string).collect::<Vec<_>>()).collect::<Vec<_>>(),"Omega":om.iter().map(|r|r.iter().map(ToString::to_string).collect::<Vec<_>>()).collect::<Vec<_>>(),"both_Theta_and_3Theta_commutators_trivial":true,"pairing_root":lf::ej(zeta)}));
        }
        println!("G17:graph{g},Smith[1,1,3,105],actual odd kernel generators and all diagonal theta commutators checked");
        records.push(json!({"graph":g,"trace":trace,"curve_cardinality":n.to_string(),"torsion_bases":data,"kernel_generators":generators.iter().map(|v|v.iter().map(lf::pj).collect::<Vec<_>>()).collect::<Vec<_>>(),"generator_orders":orders,"kernel_parameterization":"V*(0,0,aP3+bQ3,cP105+dQ105);a,b in Z/3,c,d in Z/105","kernel_order":99225,"pairings":pairing_records,"full_kernel_materialized":false,"kernel_completeness":"exact unimodular Smith identity and independent full torsion bases","theta_linearization_matrices_constructed":false,"mixed_divisor_trivialization_constructed":false,"target243_sections_constructed":false}));
    }
    lean.push_str("end KoblitzRound17Theta\n#print axioms KoblitzRound17Theta.actual_odd_Smith\n#print axioms KoblitzRound17Theta.actual_odd_kernel_order\n");
    for g in [0, 2] {
        for l in [3, 5, 7] {
            lean.push_str(&format!(
                "#print axioms KoblitzRound17Theta.actual_kernel_isotropic{g}_{l}\n"
            ));
        }
    }
    lean.push_str("#print axioms KoblitzRound17Theta.actual_U_inverse\n#print axioms KoblitzRound17Theta.actual_V_inverse\n");
    let out = dir.join("KoblitzRound17Theta.lean");
    assert!(!out.exists());
    std::fs::write(out, lean).unwrap();
    crate::round02::save(
        dir,
        "g17-odd-cover-kernel-and-commutators.json",
        &json!({"schema":"koblitz.round17/g17-v1","U":matrixv(&u),"B":matrixv(&b),"V":matrixv(&v),"Smith_diagonal":matrixv(&d),"Smith_operations":ops,"U_inverse":matrixv(&ui),"V_inverse":matrixv(&vi),"U_determinant":det(&u).val(),"V_determinant":det(&v).val(),"torsion_Frobenius_orders_E0_3_5_7":orders,"torsion_Frobenius_orders_E1_3_5_7":orders_plus,"torsion_coordinate_field_degree":field_degree,"field_modulus":f.modulus().to_str_radix(16),"compositum_with_F32_degree":field_degree.lcm(&5),"F32_tracezero_identification_evaluated_in_compositum":false,"records":records,"scope":"actual split-product odd-cover kernel and diagonal commutators;section linearization/descent still open"}),
    );
}

#[path = "lifts17.rs"]
pub(crate) mod lifts;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ramified_seven_torsion_orders_differ_for_twists() {
        assert_eq!([3, 5, 7].map(|l| ordmat(-1, l)), [8, 24, 42]);
        assert_eq!([3, 5, 7].map(|l| ordmat(1, l)), [8, 24, 21]);
    }
    #[test]
    fn cm_euclidean_remainders_cover_negative_coefficients() {
        for a in -5..=5 {
            for b in -5..=5 {
                for c in -2..=2 {
                    for d in -2..=2 {
                        let x = C::n(a, b);
                        let y = C::n(c, d);
                        if y.zero() {
                            continue;
                        }
                        let (q, r) = x.divrem(&y);
                        assert_eq!(q.mul(&y).add(&r), x);
                        assert!(r.norm() < y.norm());
                    }
                }
            }
        }
    }
}
