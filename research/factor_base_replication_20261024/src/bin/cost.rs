#![allow(clippy::suspicious_arithmetic_impl)]
//! Operation-count gate for the wrapped external law on the exact 21-model cohort.
use divisor_arithmetic::field::{Field as ExtField, PrimeField};
use divisor_arithmetic::g2::ramified::not_char2::{add, double, CurveConstants, DivisorCoords};
use five_leads_study::three_leads::{Divisor, Field, Jacobian};
use serde_json::{json, Value};
use std::cell::Cell;
use std::fs;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct OpCounts {
    pub m: u64,
    pub s: u64,
    pub i: u64,
    pub a: u64,
}
thread_local! { static OPS: Cell<OpCounts> = Cell::new(OpCounts::default()); }
fn bump(f: impl FnOnce(&mut OpCounts)) {
    OPS.with(|c| {
        let mut v = c.get();
        f(&mut v);
        c.set(v);
    });
}
pub(crate) fn reset() {
    OPS.with(|c| c.set(OpCounts::default()));
}
pub(crate) fn counts() -> OpCounts {
    OPS.with(Cell::get)
}
impl AddAssign for OpCounts {
    fn add_assign(&mut self, rhs: Self) {
        self.m += rhs.m;
        self.s += rhs.s;
        self.i += rhs.i;
        self.a += rhs.a;
    }
}
fn as_json(c: OpCounts) -> Value {
    json!({"m":c.m,"s":c.s,"i":c.i,"field_adds":c.a})
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Counted<const P: u64>(PrimeField<P>);
impl<const P: u64> Counted<P> {
    pub(crate) fn new(v: u64) -> Self {
        Self(PrimeField::new(v))
    }
    fn value(self) -> u64 {
        self.0.value()
    }
}
impl<const P: u64> Add for Counted<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        bump(|c| c.a += 1);
        Self(self.0 + rhs.0)
    }
}
impl<const P: u64> Sub for Counted<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        bump(|c| c.a += 2);
        Self(self.0 - rhs.0)
    }
}
impl<const P: u64> Neg for Counted<P> {
    type Output = Self;
    fn neg(self) -> Self {
        bump(|c| c.a += 1);
        Self(-self.0)
    }
}
impl<const P: u64> Mul for Counted<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        bump(|c| c.m += 1);
        Self(self.0 * rhs.0)
    }
}
impl<const P: u64> Div for Counted<P> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}
impl<const P: u64> AddAssign for Counted<P> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<const P: u64> SubAssign for Counted<P> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<const P: u64> MulAssign for Counted<P> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<const P: u64> DivAssign for Counted<P> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl<const P: u64> ExtField for Counted<P> {
    fn zero() -> Self {
        Self(PrimeField::new(0))
    }
    fn one() -> Self {
        Self(PrimeField::new(1))
    }
    fn is_zero(&self) -> bool {
        self.0.value() == 0
    }
    fn inv(&self) -> Self {
        bump(|c| c.i += 1);
        Self(self.0.inv())
    }
    fn square(&self) -> Self {
        bump(|c| c.s += 1);
        Self(self.0 * self.0)
    }
    fn random<R: rand::Rng>(rng: &mut R) -> Self {
        Self(PrimeField::random(rng))
    }
}
pub(crate) fn external<const P: u64>(d: Divisor) -> DivisorCoords<Counted<P>> {
    match d.degree {
        0 => DivisorCoords::identity(),
        1 => DivisorCoords::deg1(Counted::new(d.u[0]), Counted::new(d.v[0])),
        2 => DivisorCoords::deg2(
            Counted::new(d.u[1]),
            Counted::new(d.u[0]),
            Counted::new(d.v[1]),
            Counted::new(d.v[0]),
        ),
        _ => unreachable!(),
    }
}
pub(crate) fn native<const P: u64>(d: DivisorCoords<Counted<P>>) -> Divisor {
    match d.degree() {
        0 => Divisor::O,
        1 => Divisor {
            degree: 1,
            u: [d.u0.value(), 1, 0],
            v: [d.v0.value(), 0],
        },
        2 => Divisor {
            degree: 2,
            u: [d.u0.value(), d.u1.value(), 1],
            v: [d.v0.value(), d.v1.value()],
        },
        _ => unreachable!(),
    }
}
fn value(f: &Field, poly: &[u64], x: u64) -> u64 {
    poly.iter().rev().fold(0, |z, &a| f.add(f.mul(z, x), a))
}
fn run<const P: u64>(poly: Vec<u64>, bits: u64) -> Value {
    assert_eq!(poly.len(), 6);
    assert_eq!(poly[5], 1);
    assert_eq!(poly[4], 0);
    let j = Jacobian {
        f: Field::new(P, 1, 0),
        polynomial: poly.clone(),
    };
    let cc = CurveConstants {
        f3: Counted::new(poly[3]),
        f2: Counted::new(poly[2]),
        f1: Counted::new(poly[1]),
        f0: Counted::new(poly[0]),
    };
    let mut points = Vec::new();
    for x in 0..P {
        if let Some(y) = j.f.sqrt(value(&j.f, &poly, x)) {
            points.push(j.point(x, y));
            if points.len() == 9 {
                break;
            }
        }
    }
    assert_eq!(points.len(), 9);
    let mut pool = vec![Divisor::O];
    for &d in &points {
        pool.push(d);
        pool.push(j.neg(d));
    }
    for i in 0..9 {
        for k in i..9 {
            pool.push(j.add(points[i], points[k]));
        }
    }
    pool.sort();
    pool.dedup();
    assert!(pool.iter().all(|&d| j.valid(d)));
    let (mut ec, mut nc) = (OpCounts::default(), OpCounts::default());
    let (mut eq, mut neq, mut errors) = (0u64, 0u64, 0u64);
    for &a in &pool {
        for &b in &pool {
            j.f.reset();
            let expected = j.add(a, b);
            let n = j.f.counts();
            nc += OpCounts {
                m: n.m,
                s: n.s,
                i: n.i,
                a: n.field_adds,
            };
            let ea = external::<P>(a);
            let eb = external::<P>(b);
            reset();
            let out = if a == b {
                eq += 1;
                double(&ea, &cc)
            } else {
                neq += 1;
                add(&ea, &eb, &cc)
            };
            ec += counts();
            if native::<P>(out) != expected {
                errors += 1;
            }
        }
    }
    assert_eq!(errors, 0);
    json!({"p":P,"bits":bits,"pool":pool.len(),"calls":eq+neq,"equal_dispatches":eq,"non_equal_dispatches":neq,"wrapped_external":as_json(ec),"native_cantor":as_json(nc),"mismatches":errors})
}
fn dispatch(p: u64, poly: Vec<u64>, bits: u64) -> Value {
    match p {
        739 => run::<739>(poly, bits),
        1151 => run::<1151>(poly, bits),
        1187 => run::<1187>(poly, bits),
        3371 => run::<3371>(poly, bits),
        3527 => run::<3527>(poly, bits),
        3931 => run::<3931>(poly, bits),
        11731 => run::<11731>(poly, bits),
        11887 => run::<11887>(poly, bits),
        12143 => run::<12143>(poly, bits),
        48383 => run::<48383>(poly, bits),
        48823 => run::<48823>(poly, bits),
        49451 => run::<49451>(poly, bits),
        186379 => run::<186379>(poly, bits),
        186451 => run::<186451>(poly, bits),
        186583 => run::<186583>(poly, bits),
        741967 => run::<741967>(poly, bits),
        748807 => run::<748807>(poly, bits),
        749299 => run::<749299>(poly, bits),
        2967343 => run::<2967343>(poly, bits),
        2968367 => run::<2968367>(poly, bits),
        2972219 => run::<2972219>(poly, bits),
        _ => panic!("unexpected p={p}"),
    }
}
fn main() {
    let path = "research/five_leads_e2e_20261019/correctness/genus2-model-gate-v1.jsonl";
    let rows: Vec<Value> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| serde_json::from_str(x).unwrap())
        .filter(|r: &Value| r["cohort"] == "genus-order-fixtures-v1")
        .collect();
    assert_eq!(rows.len(), 21);
    let results: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let p = r["p"].as_u64().unwrap();
            let bits = r["bits"].as_u64().unwrap();
            let poly = r["normalized_polynomial"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap())
                .collect();
            dispatch(p, poly, bits)
        })
        .collect();
    println!(
        "{}",
        json!({"schema":"genus2-external-21-curve-add-cost/v1","upstream_commit":"87e2ebc1b59e5313b67bf81d11e42ec094251518","field_adds_convention":"subtraction counts as two adds, matching native Field::sub","results":results})
    );
}
