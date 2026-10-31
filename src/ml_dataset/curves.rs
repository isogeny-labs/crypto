//! Synthetic, reconstructible curves with checked order certificates.
//!
//! Native arithmetic comes from `binary_ecc`, `koblitz_fast_arith`, and
//! `prime_fast`. Hasse interval point counting follows the standard BSGS
//! construction (see Sage's `cardinality_bsgs` documentation), but acceptance
//! here requires an independently replayable exact-point-order certificate.
//! Koblitz counts use the library's Frobenius trace recurrence.
use super::types::{CaseConfig, CaseData};
use crate::binary_ecc::curve::{point_add, scalar_mul};
use crate::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use crate::cryptanalysis::koblitz_fast_arith::FastBinaryCurve;
use crate::cryptanalysis::koblitz_index_calculus::{
    find_irreducible_sparse, is_irreducible_f2, koblitz_point_count,
};
use crate::cryptanalysis::p256_structural::deep_factor;
use crate::cryptanalysis::prime_fast::FastCurve;
use crate::cryptanalysis::residual_walk::is_prime_u64;
use crate::ecc::field::FieldElement;
use crate::ecc::point::Point;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

pub(super) type WPoint = Option<(u64, u64)>;

/// Exact dataset domain metadata. The legacy public `CurveParams` uses a u32
/// cofactor, while these word-field research populations can have larger ones.
/// Keep the selected domain intact and reuse the existing textbook point API.
#[derive(Clone, Debug)]
pub struct ExactPrimeCurve {
    pub p: BigUint,
    pub a: BigUint,
    pub b: BigUint,
    pub gx: BigUint,
    pub gy: BigUint,
    pub n: BigUint,
    pub cofactor: u64,
}

impl ExactPrimeCurve {
    pub fn fe(&self, value: BigUint) -> FieldElement {
        FieldElement::new(value, self.p.clone())
    }
    pub fn a_fe(&self) -> FieldElement {
        self.fe(self.a.clone())
    }
    pub fn generator(&self) -> Point {
        Point::Affine {
            x: self.fe(self.gx.clone()),
            y: self.fe(self.gy.clone()),
        }
    }
    pub fn group_order(&self) -> BigUint {
        &self.n * BigUint::from(self.cofactor)
    }
    pub fn is_on_curve(&self, point: &Point) -> bool {
        match point {
            Point::Infinity => true,
            Point::Affine { x, y } => {
                x.modulus == self.p
                    && y.modulus == self.p
                    && x.value < self.p
                    && y.value < self.p
                    && y.mul(y)
                        == x.mul(&x.mul(x))
                            .add(&self.a_fe().mul(x))
                            .add(&self.fe(self.b.clone()))
            }
        }
    }
    fn fast(&self) -> Result<FastCurve, String> {
        let p = self.p.to_u64().ok_or("prime modulus exceeds u64")?;
        let n = self.n.to_u64().ok_or("prime subgroup exceeds u64")?;
        if p >= 1u64 << 62 || n == 0 || n >= 1u64 << 62 {
            return Err("unsupported prime arithmetic parameters".into());
        }
        Ok(FastCurve::new(
            "ml-dataset synthetic",
            p,
            self.a.to_u64().ok_or("prime a exceeds u64")?,
            self.b.to_u64().ok_or("prime b exceeds u64")?,
            n,
        ))
    }
}

#[derive(Clone)]
pub(super) enum Arithmetic {
    Binary { fast: FastBinaryCurve, b: u64 },
    Prime(FastCurve),
}

impl Arithmetic {
    pub(super) fn q(&self) -> u64 {
        match self {
            Self::Binary { fast, .. } => 1u64 << fast.n,
            Self::Prime(c) => c.f.p,
        }
    }
    pub(super) fn add(&self, p: WPoint, q: WPoint) -> WPoint {
        match self {
            Self::Binary { fast, .. } => fast.add(p, q),
            Self::Prime(c) => c
                .add(p.map(|(x, y)| c.point(x, y)), q.map(|(x, y)| c.point(x, y)))
                .map(|p| c.canonical(p)),
        }
    }
    pub(super) fn neg(&self, p: WPoint) -> WPoint {
        match self {
            Self::Binary { .. } => FastBinaryCurve::neg(p),
            Self::Prime(c) => p.map(|(x, y)| (x, if y == 0 { 0 } else { c.f.p - y })),
        }
    }
    pub(super) fn mul(&self, p: WPoint, k: u64) -> WPoint {
        match self {
            Self::Binary { fast, .. } => fast.scalar_mul(p, &BigUint::from(k)),
            Self::Prime(c) => c
                .mul(p.map(|(x, y)| c.point(x, y)), k)
                .map(|p| c.canonical(p)),
        }
    }
    pub(super) fn lift(&self, x: u64) -> Vec<WPoint> {
        match self {
            Self::Binary { fast, b } if !fast.n.is_multiple_of(2) => fast.points_with_x(*b, x),
            Self::Binary { fast, b } => {
                // The optimized lift returns None for even degrees above20.
                // The library's general lift solves the Artin-Schreier linear
                // map by Gaussian elimination and supports all n<=63.
                let c = BinaryCurve {
                    m: fast.n,
                    irreducible: IrreduciblePoly {
                        degree: fast.n,
                        low_terms: (0..fast.n)
                            .filter(|i| fast.gf.irr & (1u64 << i) != 0)
                            .collect(),
                    },
                    a: fast.element(fast.a),
                    b: fast.element(*b),
                    generator: BinaryPoint::Infinity,
                    order: 1u64.into(),
                    cofactor: 1u64.into(),
                };
                crate::cryptanalysis::koblitz_index_calculus::points_with_x(&c, &fast.element(x))
                    .into_iter()
                    .map(|p| match p {
                        BinaryPoint::Infinity => None,
                        BinaryPoint::Affine { x, y } => Some((fast.word(&x), fast.word(&y))),
                    })
                    .collect()
            }
            Self::Prime(c) => match c.lift_x(x) {
                None => Vec::new(),
                Some(p) => {
                    let p = Some(c.canonical(p));
                    let neg = self.neg(p);
                    if p == neg {
                        vec![p]
                    } else {
                        vec![p, neg]
                    }
                }
            },
        }
    }
    pub(super) fn on_curve(&self, p: WPoint) -> bool {
        let Some((x, y)) = p else { return true };
        if x >= self.q() || y >= self.q() {
            return false;
        }
        match self {
            Self::Prime(c) => c.is_on_curve(c.point(x, y)),
            Self::Binary { fast, b } => {
                let f = &fast.gf;
                (f.sqr(y) ^ f.mul(x, y)) == (f.mul(f.sqr(x), x) ^ f.mul(fast.a, f.sqr(x)) ^ b)
            }
        }
    }
    fn frobenius(&self, p: WPoint, power: u32) -> WPoint {
        match self {
            Self::Binary { fast, .. } => {
                p.map(|(x, y)| (fast.gf.sqr_k(x, power), fast.gf.sqr_k(y, power)))
            }
            Self::Prime(_) => p,
        }
    }
    pub(super) fn reference_mul(&self, p: WPoint, k: u64) -> WPoint {
        match self {
            Self::Prime(c) => {
                let generic = c.to_textbook(p.map(|(x, y)| c.point(x, y)));
                let a =
                    crate::ecc::field::FieldElement::new(c.f.from_mont(c.a).into(), c.f.p.into());
                let result = generic.scalar_mul_vartime(&k.into(), &a);
                match result {
                    Point::Infinity => None,
                    Point::Affine { x, y } => Some((
                        x.value.to_u64().expect("word field"),
                        y.value.to_u64().expect("word field"),
                    )),
                }
            }
            Self::Binary { fast, b } => {
                let generic = BinaryCurve {
                    m: fast.n,
                    irreducible: IrreduciblePoly {
                        degree: fast.n,
                        low_terms: (0..fast.n)
                            .filter(|i| fast.gf.irr & (1u64 << i) != 0)
                            .collect(),
                    },
                    a: fast.element(fast.a),
                    b: fast.element(*b),
                    generator: BinaryPoint::Infinity,
                    order: 1u64.into(),
                    cofactor: 1u64.into(),
                };
                let p = p.map_or(BinaryPoint::Infinity, |(x, y)| BinaryPoint::Affine {
                    x: fast.element(x),
                    y: fast.element(y),
                });
                match scalar_mul(&generic, &p, &k.into()) {
                    BinaryPoint::Infinity => None,
                    BinaryPoint::Affine { x, y } => Some((fast.word(&x), fast.word(&y))),
                }
            }
        }
    }
    pub(super) fn reference_add(&self, p: WPoint, q: WPoint) -> WPoint {
        match self {
            Self::Prime(c) => {
                let a =
                    crate::ecc::field::FieldElement::new(c.f.from_mont(c.a).into(), c.f.p.into());
                let left = c.to_textbook(p.map(|(x, y)| c.point(x, y)));
                let right = c.to_textbook(q.map(|(x, y)| c.point(x, y)));
                match left.add_vartime(&right, &a) {
                    Point::Infinity => None,
                    Point::Affine { x, y } => Some((
                        x.value.to_u64().expect("word field"),
                        y.value.to_u64().expect("word field"),
                    )),
                }
            }
            Self::Binary { fast, b } => {
                let c = BinaryCurve {
                    m: fast.n,
                    irreducible: IrreduciblePoly {
                        degree: fast.n,
                        low_terms: (0..fast.n)
                            .filter(|i| fast.gf.irr & (1u64 << i) != 0)
                            .collect(),
                    },
                    a: fast.element(fast.a),
                    b: fast.element(*b),
                    generator: BinaryPoint::Infinity,
                    order: 1u64.into(),
                    cofactor: 1u64.into(),
                };
                let lift = |p: WPoint| {
                    p.map_or(BinaryPoint::Infinity, |(x, y)| BinaryPoint::Affine {
                        x: fast.element(x),
                        y: fast.element(y),
                    })
                };
                match point_add(&c, &lift(p), &lift(q)) {
                    BinaryPoint::Infinity => None,
                    BinaryPoint::Affine { x, y } => Some((fast.word(&x), fast.word(&y))),
                }
            }
        }
    }
}

fn u(v: u64) -> Value {
    Value::String(v.to_string())
}
pub(super) fn point_json(p: WPoint) -> Value {
    p.map_or(Value::Null, |(x, y)| json!({"x": u(x), "y": u(y)}))
}
pub(super) fn number(v: &Value, name: &str) -> Result<u64, String> {
    let s = v
        .as_str()
        .ok_or_else(|| format!("{name} must be a canonical decimal string"))?;
    let n = s.parse::<u64>().map_err(|_| format!("invalid {name}"))?;
    if s != n.to_string() {
        return Err(format!("noncanonical {name}"));
    }
    Ok(n)
}
pub(super) fn point_words(v: &Value) -> Result<WPoint, String> {
    if v.is_null() {
        return Ok(None);
    }
    Ok(Some((
        number(&v["x"], "point x")?,
        number(&v["y"], "point y")?,
    )))
}

fn sqrt_floor(v: u64) -> u64 {
    let mut x = (v as f64).sqrt() as u64;
    while (x as u128 + 1) * (x as u128 + 1) <= v as u128 {
        x += 1;
    }
    while x as u128 * x as u128 > v as u128 {
        x -= 1;
    }
    x
}
fn hasse(q: u64) -> (u64, u64) {
    // sqrt(4*q), not 2*floor(sqrt(q)); the latter loses valid boundary orders.
    let radius = sqrt_floor(4 * q);
    (q + 1 - radius, q + 1 + radius)
}
fn factor(n: u64) -> Result<Vec<(u64, u32)>, String> {
    let raw = deep_factor(&BigUint::from(n), 4096, 300_000);
    let mut factors = Vec::new();
    let mut product = 1u128;
    for (p, e) in raw {
        let p = p.to_u64().ok_or("factor does not fit u64")?;
        if e == 0 || !is_prime_u64(p) {
            return Err(format!(
                "budget_exhausted: exact factorization of {n}; unresolved {p}"
            ));
        }
        product = product
            .checked_mul((p as u128).pow(e))
            .ok_or("factorization product overflow")?;
        factors.push((p, e));
    }
    if product != n as u128 {
        return Err("factorization product mismatch".into());
    }
    factors.sort_unstable_by_key(|x| x.0);
    Ok(factors)
}

fn prime_projection(
    c: &Arithmetic,
    p: WPoint,
    order: u64,
    r: u64,
    reference: bool,
) -> (WPoint, u64) {
    let mut clearing = order;
    let mut multiplicity = 0;
    while clearing > 0 && clearing.is_multiple_of(r) {
        clearing /= r;
        multiplicity += 1;
    }
    let mul = |p, k| {
        if reference {
            c.reference_mul(p, k)
        } else {
            c.mul(p, k)
        }
    };
    let mut result = mul(p, clearing);
    let mut multiplier = clearing;
    // Public transcript replay must remain bounded even when its claimed
    // group order is false. Callers reject a surviving point not killed by r.
    for _ in 1..multiplicity {
        if result.is_none() {
            break;
        }
        let next = mul(result, r);
        if next.is_none() {
            break;
        }
        result = next;
        multiplier *= r;
    }
    (result, multiplier)
}

struct CyclicMembership {
    babies: HashMap<WPoint, u64>,
    stride: u64,
    negative_stride: WPoint,
    order: u64,
}
impl CyclicMembership {
    fn new(c: &Arithmetic, g: WPoint, r: u64) -> Self {
        let stride = sqrt_floor(r) + 1;
        let mut babies = HashMap::new();
        let mut current = None;
        for j in 0..stride {
            babies.entry(current).or_insert(j);
            current = c.add(current, g);
        }
        Self {
            babies,
            stride,
            negative_stride: c.neg(current),
            order: r,
        }
    }
    fn logarithm(&self, c: &Arithmetic, p: WPoint) -> Option<u64> {
        self.logarithm_counted(c, p).0
    }
    fn logarithm_counted(&self, c: &Arithmetic, p: WPoint) -> (Option<u64>, u64) {
        let mut current = p;
        let mut additions = 0;
        for i in 0..=self.order / self.stride {
            if let Some(j) = self.babies.get(&current) {
                let k = i * self.stride + j;
                if k < self.order {
                    return (Some(k), additions);
                }
            }
            current = c.add(current, self.negative_stride);
            additions += 1;
        }
        (None, additions)
    }
}

/// Shared exact projection for public-input relation setup. In noncyclic
/// r-torsion, membership is an explicitly charged diagnostic DLP. Its logs are
/// returned separately for restricted provenance and never fed to elimination.
pub(super) struct SubgroupProjector {
    arithmetic: Arithmetic,
    order: u64,
    r: u64,
    generator: WPoint,
    membership: Option<CyclicMembership>,
    checks: u64,
    giant_additions: u64,
    reference_log_checks: u64,
}
impl SubgroupProjector {
    pub(super) fn new(public: &Value) -> Result<Self, String> {
        let arithmetic = arithmetic(public)?;
        let order = number(&public["curve"]["group_order"], "group order")?;
        let r = number(&public["curve"]["subgroup_order"], "subgroup order")?;
        let cofactor = number(&public["curve"]["cofactor"], "cofactor")?;
        let generator = point_words(&public["generator"])?;
        if r <= 2
            || !is_prime_u64(r)
            || order == 0
            || !order.is_multiple_of(r)
            || order / r != cofactor
            || generator.is_none()
            || !arithmetic.on_curve(generator)
            || arithmetic.reference_mul(generator, r).is_some()
        {
            return Err("invalid cyclic subgroup projection parameters".into());
        }
        let factors = read_factors(&public["curve"]["group_order_factorization"], order)?;
        if factors.last().map(|&(p, _)| p) != Some(r) {
            return Err("projection subgroup differs from largest prime factor".into());
        }
        let membership = cofactor
            .is_multiple_of(r)
            .then(|| CyclicMembership::new(&arithmetic, generator, r));
        Ok(Self {
            arithmetic,
            order,
            r,
            generator,
            membership,
            checks: 0,
            giant_additions: 0,
            reference_log_checks: 0,
        })
    }
    pub(super) fn project(
        &mut self,
        input: &Value,
    ) -> Result<(Value, u64, bool, Option<u64>), String> {
        let p = point_words(input)?;
        if !self.arithmetic.on_curve(p) {
            return Err("projection input is off curve".into());
        }
        let (p, multiplier) = prime_projection(&self.arithmetic, p, self.order, self.r, true);
        if self.arithmetic.reference_mul(p, self.r).is_some() {
            return Err("claimed group order does not project into r-torsion".into());
        }
        self.checks += 1;
        let (member, log) = if let Some(test) = &self.membership {
            let (log, additions) = test.logarithm_counted(&self.arithmetic, p);
            self.giant_additions += additions;
            if let Some(k) = log {
                self.reference_log_checks += 1;
                if self.arithmetic.reference_mul(self.generator, k) != p {
                    return Err("independent cyclic membership replay failed".into());
                }
            }
            (p.is_some() && log.is_some(), log)
        } else {
            (p.is_some(), None)
        };
        Ok((point_json(p), multiplier, member, log))
    }
    pub(super) fn counters(&self) -> Value {
        json!({"method":if self.membership.is_some(){"bounded-bsgs-diagnostic"}else{"unique-prime-subgroup"},
            "projection_candidates":self.checks,
            "membership_baby_add_calls":self.membership.as_ref().map_or(0,|m|m.stride),
            "membership_giant_add_calls":self.giant_additions,
            "independent_membership_scalar_replays":self.reference_log_checks,
            "membership_logs_restricted":true,"membership_logs_used_in_relation_elimination":false})
    }
}
fn factors_json(f: &[(u64, u32)]) -> Value {
    Value::Array(
        f.iter()
            .map(|(p, e)| json!({"prime":u(*p),"exponent":e}))
            .collect(),
    )
}
fn read_factors(v: &Value, n: u64) -> Result<Vec<(u64, u32)>, String> {
    let mut factors = Vec::new();
    let mut product = 1u128;
    let mut previous = 1;
    for row in v.as_array().ok_or("factorization is not an array")? {
        let p = number(&row["prime"], "factor prime")?;
        let e = row["exponent"].as_u64().ok_or("invalid factor exponent")?;
        if e == 0 || e > 64 || p <= previous || !is_prime_u64(p) {
            return Err("invalid prime factorization".into());
        }
        let power = (p as u128).checked_pow(e as u32).ok_or("factor overflow")?;
        product = product
            .checked_mul(power)
            .ok_or("factor product overflow")?;
        if product > n as u128 {
            return Err("factor product exceeds order".into());
        }
        factors.push((p, e as u32));
        previous = p;
    }
    if product != n as u128 {
        return Err("factor product differs from order".into());
    }
    Ok(factors)
}

/// Find an annihilating integer in the complete, exact Hasse interval.
/// This alone is NOT an order certificate: the next function proves uniqueness.
fn hasse_candidate(c: &Arithmetic, p: WPoint) -> Option<u64> {
    let (lo, hi) = hasse(c.q());
    let width = hi - lo + 1;
    let stride = sqrt_floor(width) + 1;
    let mut babies = HashMap::with_capacity(stride as usize);
    let mut cur = None;
    for j in 0..stride {
        babies.entry(cur).or_insert(j);
        cur = c.add(cur, p);
    }
    let step = c.neg(c.mul(p, stride));
    let mut giant = c.neg(c.mul(p, lo));
    for i in 0..=width / stride {
        if let Some(j) = babies.get(&giant) {
            let candidate = lo + i * stride + j;
            if candidate <= hi && c.mul(p, candidate).is_none() {
                return Some(candidate);
            }
        }
        giant = c.add(giant, step);
    }
    None
}

fn certify_count(c: &Arithmetic, rng: &mut StdRng) -> Result<(u64, Value), String> {
    if c.q() <= 4096 {
        let counts: Vec<_> = (0..c.q()).map(|x| c.lift(x).len()).collect();
        let order = 1 + counts.iter().map(|x| *x as u64).sum::<u64>();
        return Ok((
            order,
            json!({"method":"exact-small-field-exhaustive-v1","field_cardinality":u(c.q()),"x_point_counts":counts}),
        ));
    }
    let (lo, hi) = hasse(c.q());
    let mut attempts = Vec::new();
    for attempt in 0..16 {
        let p =
            (0..1024).find_map(|_| c.lift(rng.gen_range(1..c.q())).into_iter().next().flatten());
        let Some(p) = p else {
            return Err(format!("budget_exhausted: point lift sampling; construction_trace_partial=true; completed_attempts={}",json!(attempts)));
        };
        let p = Some(p);
        let Some(candidate) = hasse_candidate(c, p) else {
            attempts.push(json!({"point":point_json(p),"status":"no_hasse_candidate"}));
            continue;
        };
        let factors = factor(candidate).map_err(|e|format!("{e}; construction_trace_partial=true; current_point={}; candidate={candidate}; completed_attempts={}",point_json(p),json!(attempts)))?;
        let mut order = candidate;
        for &(prime, _) in &factors {
            while order.is_multiple_of(prime) && c.mul(p, order / prime).is_none() {
                order /= prime;
            }
        }
        let first = lo.div_ceil(order) * order;
        if first <= hi && first.checked_add(order).is_none_or(|next| next > hi) {
            let order_factors: Vec<_> = factors
                .iter()
                .filter_map(|&(prime, _)| {
                    let mut tmp = order;
                    let mut e = 0;
                    while tmp.is_multiple_of(prime) {
                        tmp /= prime;
                        e += 1;
                    }
                    (e > 0).then_some((prime, e))
                })
                .collect();
            return Ok((
                first,
                json!({
                    "method":"exact-point-order-unique-hasse-multiple-v1",
                    "point":point_json(p),"point_order":u(order),
                    "point_order_factorization":factors_json(&order_factors),
                    "hasse_lower":u(lo),"hasse_upper":u(hi),
                    "attempt_index":attempt,"rejected_attempts":attempts,
                    "construction_counts":{"hasse_bsgs_calls":attempt+1},
                    "literature":"https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/cardinality.html"
                }),
            ));
        }
        attempts.push(json!({"point":point_json(p),"candidate":u(candidate),"point_order":u(order),"status":"multiple_hasse_multiples"}));
    }
    Err(format!("budget_exhausted: 16 point orders did not certify a unique Hasse multiple; this is not a point-count impossibility; construction_trace_partial=true; completed_attempts={}",json!(attempts)))
}

fn construct(config: &CaseConfig) -> Result<(Arithmetic, Value, Value, u64, Value, u32), String> {
    let n = config.n;
    let mut rng =
        StdRng::seed_from_u64(config.curve_seed ^ (n as u64).wrapping_mul(0x9e3779b97f4a7c15));
    if config.family == "prime" {
        // A fixed prime per size avoids an unrecorded prime-selection bias.
        // n=61 selects the explicitly requested Mersenne prime 2^61-1.
        let mut p = (1u64 << n) - 1;
        while !is_prime_u64(p) {
            p -= 2;
        }
        let mut rejected = Vec::new();
        let (a, b) = loop {
            let a = rng.gen_range(0..p);
            let b = rng.gen_range(0..p);
            let mul = |x: u64, y: u64| ((x as u128 * y as u128) % p as u128) as u64;
            if !(4u128 * mul(mul(a, a), a) as u128 + 27u128 * mul(b, b) as u128)
                .is_multiple_of(p as u128)
            {
                break (a, b);
            }
            rejected.push(json!({"a":u(a),"b":u(b),"reason":"singular"}));
        };
        let base = Arithmetic::Prime(FastCurve::new("ml-dataset synthetic", p, a, b, 1));
        let (base_order, base_certificate) = certify_count(&base, &mut rng)?;
        let f = match &base {
            Arithmetic::Prime(c) => c.f,
            _ => unreachable!(),
        };
        let mut d = 2;
        while f.pow(f.to_mont(d), (p - 1) / 2) == f.one {
            d += 1;
        }
        let mul = |x: u64, y: u64| ((x as u128 * y as u128) % p as u128) as u64;
        let (actual_a, actual_b) = if config.twist {
            (mul(a, mul(d, d)), mul(b, mul(d, mul(d, d))))
        } else {
            (a, b)
        };
        let actual = Arithmetic::Prime(FastCurve::new(
            "ml-dataset synthetic",
            p,
            actual_a,
            actual_b,
            1,
        ));
        let order = if config.twist {
            2 * (p + 1) - base_order
        } else {
            base_order
        };
        let certificate = if config.twist {
            json!({"method":"quadratic-twist-order-v1","base_order":u(base_order),"base_certificate":base_certificate})
        } else {
            base_certificate
        };
        let field =
            json!({"characteristic":u(p),"modulus":u(p),"degree":1,"basis":"canonical-residue"});
        let model = json!({"model":"short-weierstrass","a":u(actual_a),"b":u(actual_b),"base_a":u(a),"base_b":u(b),"twist_parameter":u(d),"rejected_curve_samples":rejected});
        Ok((actual, field, model, order, certificate, 1))
    } else {
        let irr = find_irreducible_sparse(n)
            .ok_or("unsupported: native irreducible polynomial search failed")?;
        let q = 1u64 << n;
        let (a, b) = if config.family == "koblitz" {
            (config.curve_seed & 1, 1)
        } else {
            (rng.gen_range(0..q), rng.gen_range(2..q))
        };
        let fast = FastBinaryCurve::new(&irr, a).ok_or("unsupported binary degree")?;
        let mut shift = None;
        for i in 0..n {
            let candidate = F2mElement::from_biguint(&BigUint::from(1u64 << i), n);
            if candidate.trace(&irr) == 1 {
                shift = Some(1u64 << i);
                break;
            }
        }
        let shift = shift.ok_or("trace map unexpectedly has no trace-one basis vector")?;
        let base = Arithmetic::Binary {
            fast: fast.clone(),
            b,
        };
        let (base_order, base_certificate) = if config.family == "koblitz" {
            (
                koblitz_point_count(a as u8, n)
                    .to_u64()
                    .ok_or("Koblitz count overflow")?,
                json!({"method":"koblitz-frobenius-trace-recurrence-v1","a":a,"n":n,"base_trace":if a==0 {-1} else {1}}),
            )
        } else {
            certify_count(&base, &mut rng)?
        };
        let actual_a = if config.twist { a ^ shift } else { a };
        let actual = Arithmetic::Binary {
            fast: FastBinaryCurve::new(&irr, actual_a).ok_or("invalid field")?,
            b,
        };
        let order = if config.twist {
            2 * (q + 1) - base_order
        } else {
            base_order
        };
        let certificate = if config.twist {
            json!({"method":"quadratic-twist-order-v1","base_order":u(base_order),"base_certificate":base_certificate})
        } else {
            base_certificate
        };
        let k = (1..=n)
            .find(|&k| {
                n.is_multiple_of(k)
                    && fast.gf.sqr_k(actual_a, k) == actual_a
                    && fast.gf.sqr_k(b, k) == b
            })
            .ok_or("coefficient field not found")?;
        let field = json!({"characteristic":"2","degree":n,"irreducible_low_terms":irr.low_terms,"basis":"polynomial-lsb-first","polynomial_irreducibility":"native-rabin-test"});
        let model = json!({"model":"binary-ordinary","a":u(actual_a),"b":u(b),"base_a":u(a),"base_b":u(b),"twist_parameter":u(shift),"twist_parameter_absolute_trace":1});
        Ok((actual, field, model, order, certificate, k))
    }
}

/// Construct independent synthetic public inputs, and keep scalar provenance
/// only in the separate witness object. No solver reads that object.
pub fn generate(config: &CaseConfig) -> Result<CaseData, String> {
    if !["koblitz", "binary", "prime"].contains(&config.family.as_str()) {
        return Err("unsupported family".into());
    }
    if !(3..=61).contains(&config.n) {
        return Err("unsupported: curve constructor supports n=3..61".into());
    }
    if !(2..=15).contains(&config.m) {
        return Err("m must lie in 2..15".into());
    }
    if config.factor_base_size == 0 || config.factor_base_size > 4096 {
        return Err("factor base size must lie in 1..4096".into());
    }
    let (arithmetic, field, mut model, order, certificate, k) = construct(config)?;
    let factors = factor(order)?;
    let subgroup = factors.last().ok_or("empty group order factorization")?.0;
    if subgroup <= 2 {
        return Err("unsupported: no odd prime subgroup on selected curve".into());
    }
    let cofactor = order / subgroup;
    let mut setup_rng = StdRng::seed_from_u64(
        config.curve_seed
            ^ (config.n as u64).rotate_left(17)
            ^ if config.twist { 0x71af } else { 0xbace },
    );
    let mut base_generator = None;
    let mut generator_attempts = Vec::new();
    for _ in 0..1024 {
        let x = setup_rng.gen_range(1..arithmetic.q());
        let lifted = arithmetic.lift(x).into_iter().next().flatten();
        let (projected, projection_multiplier) =
            prime_projection(&arithmetic, lifted, order, subgroup, false);
        generator_attempts
            .push(json!({"x":u(x),"lifted":point_json(lifted),"projected":point_json(projected),"projection_multiplier":u(projection_multiplier)}));
        if projected.is_some() && arithmetic.mul(projected, subgroup).is_none() {
            base_generator = projected;
            break;
        }
    }
    if base_generator.is_none() {
        return Err(format!("budget_exhausted: 1024 cofactor projection samples found no nonzero prime-subgroup point; construction_trace_partial=true; attempts={}",json!(generator_attempts)));
    }
    let mut generator_rng = StdRng::seed_from_u64(
        config.seed
            ^ config.curve_seed.rotate_left(13)
            ^ config.generator_index.wrapping_mul(0xd6e8feb86659fd93)
            ^ (config.n as u64).rotate_left(39)
            ^ if config.twist { 0xa753 } else { 0x817b },
    );
    let multiplier = generator_rng.gen_range(1..subgroup);
    let generator = arithmetic.mul(base_generator, multiplier);
    let membership = cofactor
        .is_multiple_of(subgroup)
        .then(|| CyclicMembership::new(&arithmetic, generator, subgroup));
    let mut key_rng = StdRng::seed_from_u64(
        config.seed.rotate_left(7)
            ^ config.curve_seed
            ^ config.key_index.wrapping_mul(0xa0761d6478bd642f)
            ^ config.generator_index.rotate_left(29)
            ^ (config.n as u64).rotate_left(47)
            ^ if config.twist { 0xd1a5 } else { 0xc4e7 },
    );
    let scalar = key_rng.gen_range(1..subgroup);
    let target = arithmetic.mul(generator, scalar);
    model["group_order"] = u(order);
    model["subgroup_order"] = u(subgroup);
    model["cofactor"] = u(cofactor);
    model["group_order_factorization"] = factors_json(&factors);
    model["order_certificate"] = certificate;
    let mut points = Vec::new();
    let mut seen = HashSet::new();
    let mut candidates = Vec::new();
    let mut private_membership_logs = Vec::new();
    let limit = (config.factor_base_size as u64 * 32 + 128).min(arithmetic.q());
    for x in 1..limit {
        let lifts = arithmetic.lift(x);
        let mut row = Vec::new();
        for p in lifts {
            let (projected, projection_multiplier) =
                prime_projection(&arithmetic, p, order, subgroup, false);
            let membership_log = membership
                .as_ref()
                .and_then(|test| test.logarithm(&arithmetic, projected));
            let subgroup_member = projected.is_some()
                && arithmetic.mul(projected, subgroup).is_none()
                && (membership.is_none() || membership_log.is_some());
            let accepted = subgroup_member && seen.insert(projected);
            row.push(json!({"lifted":point_json(p),"projected":point_json(projected),"projection_multiplier":u(projection_multiplier),"cyclic_subgroup_member":subgroup_member,"accepted":accepted}));
            if accepted {
                points.push(projected);
                private_membership_logs.push(membership_log.map(|k| k.to_string()));
            }
        }
        candidates.push(json!({"x":u(x),"lifts":row}));
        if points.len() >= config.factor_base_size || points.len() as u64 == subgroup - 1 {
            break;
        }
    }
    let lookup: HashMap<_, _> = points
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();
    let negation: Vec<_> = points
        .iter()
        .map(|&p| lookup.get(&arithmetic.neg(p)).copied())
        .collect();
    let frobenius: Vec<_> = points
        .iter()
        .map(|&p| lookup.get(&arithmetic.frobenius(p, k)).copied())
        .collect();
    let orbits: Vec<_> = points
        .iter()
        .map(|&p| {
            let mut orbit = vec![point_json(p)];
            let mut cur = arithmetic.frobenius(p, k);
            while cur != p && orbit.len() <= config.n as usize {
                orbit.push(point_json(cur));
                cur = arithmetic.frobenius(cur, k);
            }
            let action_order = if config.family == "prime" {
                1
            } else {
                config.n as usize / k as usize
            };
            json!({"points":orbit,"closed":cur==p,"stabilizer_size":action_order / orbit.len()})
        })
        .collect();
    let factor_base = json!({
        "schema":"ml-ecdlp-factor-base/v1","rule":"small-x-prime-primary-projection-with-cyclic-membership",
        "projection_rule":"multiply by N/r^v_r(N), then trim by r until exact order r; multiplier retained for every candidate",
        "cyclic_membership_method":if membership.is_some(){"bounded-bsgs-diagnostic; computed membership logs restricted to private provenance; setup cost must be charged"}else{"unique subgroup because r divides N once"},
        "points":points.into_iter().map(point_json).collect::<Vec<_>>(),
        "requested_size":config.factor_base_size,"negation_closed":negation.iter().all(Option::is_some),
        "size_policy":"retain-both-signs; may exceed requested size by one; capped by subgroup cardinality",
        "construction_candidates":candidates,"negation_action":negation,"frobenius_action":frobenius,
        "frobenius_power":k,"orbits":orbits,
        "construction_status":if lookup.len()>=config.factor_base_size || lookup.len() as u64==subgroup-1 {"verified"} else {"budget_exhausted"},
        "known_log_features":false
    });
    let public_instance = json!({
        "schema":"ml-ecdlp-instance/v1","family":config.family,"n":config.n,"m":config.m,"twist":config.twist,
        "field":field,"curve":model,"generator":point_json(generator),"target":point_json(target),
        "population":"unconditioned-uniform-nonzero-subgroup-scalar",
        "symmetry":{"coefficient_field_degree":k,"frobenius_power":k,"frobenius_nontrivial_on_model":config.family!="prime" && k<config.n,"negation":true},
        "sampling":{"algorithm":"rand-StdRng-rejection-gen_range","scalar_domain":"1..r","generator_domain":"1..r","curve_selection":"one nonsingular seeded model; point-count failures are not resampled"},
        "construction_trace":{"status":"partial","retained":"order certificates, point-count attempts, factor-base candidates, subgroup checks and orbit witnesses","not_exported":["Hasse BSGS baby table and giant probes","integer factorization internal states","individual field operation traces"]},
        "known_answer_verified":true,"scalar_recovered_without_witness":false,
        "split_component":format!("field-size-n{}",config.n),
        "split_policy":"conservative whole-size holdout: all seeds, families, isomorphic models, generators, keys and twists at the same n stay together; cross-size algebraic transfer links require a future merged-component index"
    });
    let private_witness = json!({"schema":"ml-ecdlp-private-witness/v1","scalar":u(scalar),"generator_multiplier":u(multiplier),"base_generator":point_json(base_generator),"generator_construction_attempts":generator_attempts,"factor_base_membership_logs":private_membership_logs,"sampling_provenance":{"seed":config.seed,"curve_seed":config.curve_seed,"generator_index":config.generator_index,"key_index":config.key_index},"feature_access":"forbidden; replay and supervised-label store only"});
    verify(&public_instance, &private_witness)?;
    verify_factor_base(&public_instance, &factor_base)?;
    Ok(CaseData {
        public_instance,
        private_witness,
        factor_base,
    })
}

/// Isolated positive-control constructor. The native planner does not route
/// cells here yet; callers must retain the private witness as a restricted
/// artifact and charge the ordinary constructor work performed before this
/// control replaces its target and factor base.
pub fn generate_planted(config: &CaseConfig) -> Result<CaseData, String> {
    let mut case = generate(config)?;
    let c = arithmetic(&case.public_instance)?;
    let generator = point_words(&case.public_instance["generator"])?;
    let r = number(&case.public_instance["curve"]["subgroup_order"], "subgroup")?;
    let mut base_rng = planted_rng(config, "base-v1")?;
    let mut base_points = Vec::new();
    let mut base_logs = Vec::new();
    let mut base_draws = Vec::new();
    let mut seen = HashSet::new();
    let base_cap = config
        .factor_base_size
        .saturating_mul(32)
        .saturating_add(128);
    for _ in 0..base_cap {
        if base_points.len() == config.factor_base_size {
            break;
        }
        let b = base_rng.gen_range(1..r);
        let p = c.mul(generator, b);
        let accepted = p.is_some() && !seen.contains(&p) && !seen.contains(&c.neg(p));
        base_draws.push(json!({"scalar":u(b),"point":point_json(p),"accepted":accepted}));
        if accepted {
            seen.insert(p);
            base_points.push(p);
            base_logs.push(b);
        }
    }
    if base_points.len() != config.factor_base_size {
        return Err(format!("budget_exhausted: planted factor-base construction retained {} of {} after {base_cap} draws", base_points.len(), config.factor_base_size));
    }
    let mut target_rng = planted_rng(config, "target-v1")?;
    let mut target_draws = Vec::new();
    let mut selected = None;
    for _ in 0..128 {
        let mut terms = Vec::with_capacity(config.m);
        let mut q = None;
        let mut scalar = 0u64;
        for _ in 0..config.m {
            let index = target_rng.gen_range(0..base_points.len());
            let positive = target_rng.gen_bool(0.5);
            let sign = if positive { 1 } else { -1 };
            let p = if positive {
                base_points[index]
            } else {
                c.neg(base_points[index])
            };
            q = c.add(q, p);
            let contribution = if positive {
                base_logs[index]
            } else {
                r - base_logs[index]
            };
            scalar = ((scalar as u128 + contribution as u128) % r as u128) as u64;
            terms.push(json!({"index":index,"sign":sign}));
        }
        let accepted = scalar != 0 && q.is_some();
        target_draws.push(
            json!({"terms":terms,"scalar":u(scalar),"target":point_json(q),"accepted":accepted}),
        );
        if accepted {
            if c.mul(generator, scalar) != q {
                return Err("planted target scalar construction disagrees with group sum".into());
            }
            selected = Some((q, scalar));
            break;
        }
    }
    let (target, scalar) =
        selected.ok_or("budget_exhausted: 128 planted target draws were zero")?;
    let k = case.public_instance["symmetry"]["frobenius_power"]
        .as_u64()
        .ok_or("missing Frobenius power")? as u32;
    let lookup: HashMap<_, _> = base_points
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();
    let negation: Vec<_> = base_points
        .iter()
        .map(|&p| lookup.get(&c.neg(p)).copied())
        .collect();
    let frobenius: Vec<_> = base_points
        .iter()
        .map(|&p| lookup.get(&c.frobenius(p, k)).copied())
        .collect();
    let orbits: Vec<_> = base_points
        .iter()
        .map(|&p| {
            let mut orbit = vec![point_json(p)];
            let mut cur = c.frobenius(p, k);
            while cur != p && orbit.len() <= config.n as usize {
                orbit.push(point_json(cur));
                cur = c.frobenius(cur, k);
            }
            let action_order = if config.family == "prime" {
                1
            } else {
                config.n as usize / k as usize
            };
            json!({"points":orbit,"closed":cur==p,"stabilizer_size":action_order/orbit.len()})
        })
        .collect();
    case.public_instance["target"] = point_json(target);
    // Keep construction/population provenance out of the public instance.
    // The natural constructor's sampling and factor-base trace describe the
    // discarded target and point set, so they must not survive this override.
    let public_object = case
        .public_instance
        .as_object_mut()
        .ok_or("invalid public instance")?;
    public_object.remove("population");
    public_object.remove("sampling");
    public_object.remove("construction_trace");
    case.factor_base = json!({"schema":"ml-ecdlp-factor-base/v1",
        "rule":"planted-known-log-subgroup/v1",
        "points":base_points.into_iter().map(point_json).collect::<Vec<_>>(),
        "requested_size":config.factor_base_size,"construction_status":"verified",
        "known_log_features":false,"negation_closed":negation.iter().all(Option::is_some),
        "negation_action":negation,
        "frobenius_action":frobenius,"frobenius_power":k,"orbits":orbits});
    case.private_witness["scalar"] = u(scalar);
    case.private_witness
        .as_object_mut()
        .ok_or("invalid private witness")?
        .remove("factor_base_membership_logs");
    let accepted_target_attempt = target_draws.len() - 1;
    case.private_witness["planted_construction"] = json!({
        "schema":"ml-dataset.planted-construction/v1",
        "population":"planted-pdp-v1",
        "preliminary_natural_constructor_run":true,
        "preliminary_natural_constructor_cost":{"operation_counts":"unavailable","charge_required":true,
            "scope":"curve/order/generator, discarded natural scalar and factor-base construction, and verification"},
        "config_sha256":super::digest(&serde_json::to_vec(config).map_err(|e| e.to_string())?),
        "base_draw_cap":base_cap,"target_draw_cap":128,
        "base_draws":base_draws,"target_draws":target_draws,
        "base_logs":base_logs.into_iter().map(u).collect::<Vec<_>>(),
        "accepted_target_attempt":accepted_target_attempt,
        "raw_base_draw_probability":"1/(r-1)",
        "raw_signed_term_probability":format!("1/{}", 2*config.factor_base_size),
        "feature_access":"forbidden; restricted replay and label store only"});
    verify_planted(config, &case)?;
    Ok(case)
}

fn planted_rng(config: &CaseConfig, domain: &str) -> Result<StdRng, String> {
    let binding = serde_json::to_vec(config).map_err(|e| e.to_string())?;
    let mut seed = b"ml-dataset.planted-pdp/v1\0".to_vec();
    seed.extend_from_slice(domain.as_bytes());
    seed.push(0);
    seed.extend_from_slice(&binding);
    Ok(StdRng::from_seed(crate::hash::sha256(&seed)))
}

/// Recompute every private draw and both target identities using independent
/// reference group arithmetic. A constructed scalar is a known answer, never
/// evidence that an ECDLP solver recovered it.
pub fn verify_planted(config: &CaseConfig, case: &CaseData) -> Result<(), String> {
    if config.factor_base_size == 0 || config.factor_base_size > 4096 {
        return Err("invalid planted factor-base size".into());
    }
    let public = &case.public_instance;
    let private = &case.private_witness;
    let base = &case.factor_base;
    if public["family"].as_str() != Some(config.family.as_str())
        || public["n"].as_u64() != Some(config.n as u64)
        || public["m"].as_u64() != Some(config.m as u64)
        || public["twist"].as_bool() != Some(config.twist)
        || public.get("population").is_some()
        || public.get("sampling").is_some()
        || public.get("construction_trace").is_some()
    {
        return Err("planted public axes differ from config".into());
    }
    verify(public, private)?;
    verify_planted_factor_base(public, base)?;
    let record = &private["planted_construction"];
    let binding = super::digest(&serde_json::to_vec(config).map_err(|e| e.to_string())?);
    if record["schema"] != "ml-dataset.planted-construction/v1"
        || record["population"] != "planted-pdp-v1"
        || record["preliminary_natural_constructor_run"] != true
        || record["preliminary_natural_constructor_cost"]
            != json!({"operation_counts":"unavailable",
            "charge_required":true,
            "scope":"curve/order/generator, discarded natural scalar and factor-base construction, and verification"})
        || record["config_sha256"] != binding
        || record["base_draw_cap"].as_u64()
            != Some(
                config
                    .factor_base_size
                    .saturating_mul(32)
                    .saturating_add(128) as u64,
            )
        || record["target_draw_cap"].as_u64() != Some(128)
        || record["raw_base_draw_probability"] != "1/(r-1)"
        || record["raw_signed_term_probability"] != format!("1/{}", 2 * config.factor_base_size)
        || record["feature_access"] != "forbidden; restricted replay and label store only"
    {
        return Err("planted construction binding or caps differ".into());
    }
    let c = arithmetic(public)?;
    let generator = point_words(&public["generator"])?;
    let r = number(&public["curve"]["subgroup_order"], "subgroup")?;
    let points = base["points"].as_array().ok_or("missing planted points")?;
    let logs = record["base_logs"]
        .as_array()
        .ok_or("missing planted logs")?;
    let draws = record["base_draws"]
        .as_array()
        .ok_or("missing planted base draws")?;
    if points.len() != config.factor_base_size
        || logs.len() != points.len()
        || draws.is_empty()
        || draws.len()
            > config
                .factor_base_size
                .saturating_mul(32)
                .saturating_add(128)
    {
        return Err("planted base cardinality or draw count differs".into());
    }
    let mut rng = planted_rng(config, "base-v1")?;
    let mut reconstructed = Vec::new();
    let mut reconstructed_logs = Vec::new();
    let mut seen = HashSet::new();
    for draw in draws {
        if reconstructed.len() == config.factor_base_size {
            return Err("planted base contains draws after completion".into());
        }
        let b = rng.gen_range(1..r);
        let p = c.reference_mul(generator, b);
        let accepted = p.is_some() && !seen.contains(&p) && !seen.contains(&c.neg(p));
        if draw["scalar"] != u(b)
            || point_words(&draw["point"])? != p
            || draw["accepted"].as_bool() != Some(accepted)
        {
            return Err("planted base RNG or point draw differs".into());
        }
        if accepted {
            seen.insert(p);
            reconstructed.push(point_json(p));
            reconstructed_logs.push(u(b));
        }
    }
    if reconstructed != *points || reconstructed_logs != *logs {
        return Err("planted factor base or restricted logs differ".into());
    }
    let target_draws = record["target_draws"]
        .as_array()
        .ok_or("missing planted target draws")?;
    if target_draws.is_empty()
        || target_draws.len() > 128
        || record["accepted_target_attempt"].as_u64() != Some((target_draws.len() - 1) as u64)
    {
        return Err("planted target attempt count differs".into());
    }
    let mut rng = planted_rng(config, "target-v1")?;
    for (attempt, draw) in target_draws.iter().enumerate() {
        let terms = draw["terms"].as_array().ok_or("missing planted terms")?;
        if terms.len() != config.m {
            return Err("planted term arity differs".into());
        }
        let mut q = None;
        let mut scalar = 0u64;
        for term in terms {
            let index = rng.gen_range(0..points.len());
            let positive = rng.gen_bool(0.5);
            let sign = if positive { 1 } else { -1 };
            if term["index"].as_u64() != Some(index as u64) || term["sign"].as_i64() != Some(sign) {
                return Err("planted term draw differs".into());
            }
            let p = point_words(&points[index])?;
            q = c.add(q, if positive { p } else { c.neg(p) });
            let b = number(&logs[index], "base log")?;
            let contribution = if positive { b } else { r - b };
            scalar = ((scalar as u128 + contribution as u128) % r as u128) as u64;
        }
        let accepted = scalar != 0 && q.is_some();
        if draw["scalar"] != u(scalar)
            || point_words(&draw["target"])? != q
            || draw["accepted"].as_bool() != Some(accepted)
            || accepted != (attempt + 1 == target_draws.len())
        {
            return Err("planted target draw, rejection or acceptance differs".into());
        }
        if accepted
            && (point_words(&public["target"])? != q
                || private["scalar"] != u(scalar)
                || c.reference_mul(generator, scalar) != q)
        {
            return Err("planted target and scalar replay failed".into());
        }
    }
    Ok(())
}

/// Reconstruct binary arithmetic after strict field and coordinate validation.
pub fn binary_curve(v: &Value) -> Result<BinaryCurve, String> {
    let n = v["n"].as_u64().ok_or("missing n")?;
    if !(3..=61).contains(&n)
        || v["field"]["degree"].as_u64() != Some(n)
        || v["field"]["characteristic"] != "2"
    {
        return Err("invalid binary field degree/characteristic".into());
    }
    let terms = v["field"]["irreducible_low_terms"]
        .as_array()
        .ok_or("missing field polynomial")?;
    let mut low_terms = Vec::new();
    let mut mask = 1u64 << n;
    for x in terms {
        let x = x.as_u64().ok_or("invalid polynomial exponent")?;
        if x >= n || low_terms.contains(&(x as u32)) {
            return Err("invalid or duplicate polynomial exponent".into());
        }
        low_terms.push(x as u32);
        mask |= 1u64 << x;
    }
    if !is_irreducible_f2(mask) {
        return Err("reducible field polynomial".into());
    }
    let a = number(&v["curve"]["a"], "a")?;
    let b = number(&v["curve"]["b"], "b")?;
    if a >= 1u64 << n || b == 0 || b >= 1u64 << n {
        return Err("invalid binary coefficients".into());
    }
    Ok(BinaryCurve {
        m: n as u32,
        irreducible: IrreduciblePoly {
            degree: n as u32,
            low_terms,
        },
        a: F2mElement::from_biguint(&a.into(), n as u32),
        b: F2mElement::from_biguint(&b.into(), n as u32),
        generator: binary_point(&v["generator"], n as u32)?,
        order: number(&v["curve"]["subgroup_order"], "subgroup order")?.into(),
        cofactor: number(&v["curve"]["cofactor"], "cofactor")?.into(),
    })
}
pub fn binary_point(v: &Value, n: u32) -> Result<BinaryPoint, String> {
    if !(1..=63).contains(&n) {
        return Err("invalid binary point degree".into());
    }
    match point_words(v)? {
        None => Ok(BinaryPoint::Infinity),
        Some((x, y)) if x < (1u64 << n) && y < (1u64 << n) => Ok(BinaryPoint::Affine {
            x: F2mElement::from_biguint(&x.into(), n),
            y: F2mElement::from_biguint(&y.into(), n),
        }),
        _ => Err("noncanonical binary point coordinates".into()),
    }
}
pub fn prime_curve(v: &Value) -> Result<ExactPrimeCurve, String> {
    let p = number(&v["field"]["modulus"], "modulus")?;
    if p <= 3 || p >= 1u64 << 62 || !is_prime_u64(p) || v["field"]["characteristic"] != u(p) {
        return Err("invalid prime field".into());
    }
    if v["n"].as_u64() != Some((64 - p.leading_zeros()) as u64)
        || v["field"]["degree"].as_u64() != Some(1)
    {
        return Err("prime field size metadata differs from modulus".into());
    }
    let a = number(&v["curve"]["a"], "a")?;
    let b = number(&v["curve"]["b"], "b")?;
    if a >= p || b >= p {
        return Err("noncanonical prime coefficients".into());
    }
    let mul = |x: u64, y: u64| ((x as u128 * y as u128) % p as u128) as u64;
    if (4u128 * mul(mul(a, a), a) as u128 + 27u128 * mul(b, b) as u128).is_multiple_of(p as u128) {
        return Err("singular prime curve".into());
    }
    let (gx, gy) = point_words(&v["generator"])?.ok_or("infinite generator")?;
    if gx >= p || gy >= p {
        return Err("noncanonical generator".into());
    }
    let subgroup = number(&v["curve"]["subgroup_order"], "subgroup order")?;
    let cofactor = number(&v["curve"]["cofactor"], "cofactor")?;
    let group_order = number(&v["curve"]["group_order"], "group order")?;
    if subgroup == 0 || cofactor == 0 {
        return Err("invalid zero subgroup or cofactor".into());
    }
    let curve = ExactPrimeCurve {
        p: p.into(),
        a: a.into(),
        b: b.into(),
        gx: gx.into(),
        gy: gy.into(),
        n: subgroup.into(),
        cofactor,
    };
    if curve.group_order() != BigUint::from(group_order) {
        return Err("prime subgroup and cofactor do not match group order".into());
    }
    Ok(curve)
}
pub fn prime_point(v: &Value, c: &ExactPrimeCurve) -> Result<Point, String> {
    match point_words(v)? {
        None => Ok(Point::Infinity),
        Some((x, y)) if BigUint::from(x) < c.p && BigUint::from(y) < c.p => Ok(Point::Affine {
            x: c.fe(x.into()),
            y: c.fe(y.into()),
        }),
        _ => Err("noncanonical prime point coordinates".into()),
    }
}

pub(super) fn arithmetic(v: &Value) -> Result<Arithmetic, String> {
    match v["family"].as_str() {
        Some("prime") => Ok(Arithmetic::Prime(prime_curve(v)?.fast()?)),
        Some("koblitz" | "binary") => {
            let c = binary_curve(v)?;
            Ok(Arithmetic::Binary {
                fast: FastBinaryCurve::new(
                    &c.irreducible,
                    c.a.to_biguint().to_u64().ok_or("a overflow")?,
                )
                .ok_or("invalid binary parameters")?,
                b: c.b.to_biguint().to_u64().ok_or("b overflow")?,
            })
        }
        _ => Err("invalid family".into()),
    }
}

fn verify_certificate(
    c: &Arithmetic,
    certificate: &Value,
    order: u64,
    base_a: u64,
    base_b: u64,
    twist: bool,
) -> Result<(), String> {
    let (lo, hi) = hasse(c.q());
    if !(lo..=hi).contains(&order) {
        return Err("order outside Hasse interval".into());
    }
    match certificate["method"].as_str() {
        Some("exact-small-field-exhaustive-v1") => {
            if c.q() > 4096
                || number(
                    &certificate["field_cardinality"],
                    "exhaustive field cardinality",
                )? != c.q()
            {
                return Err("invalid exhaustive cardinality certificate bounds".into());
            }
            let counts: Vec<_> = (0..c.q()).map(|x| c.lift(x).len()).collect();
            if certificate["x_point_counts"] != json!(counts)
                || 1 + counts.iter().map(|x| *x as u64).sum::<u64>() != order
            {
                return Err("exhaustive cardinality replay failed".into());
            }
        }
        Some("koblitz-frobenius-trace-recurrence-v1") => {
            let Arithmetic::Binary { fast, b } = c else {
                return Err("Koblitz certificate on prime curve".into());
            };
            if twist
                || fast.a > 1
                || *b != 1
                || certificate["a"].as_u64() != Some(fast.a)
                || certificate["n"].as_u64() != Some(fast.n as u64)
                || koblitz_point_count(fast.a as u8, fast.n) != BigUint::from(order)
            {
                return Err("invalid Koblitz order certificate".into());
            }
        }
        Some("exact-point-order-unique-hasse-multiple-v1") => {
            let p = point_words(&certificate["point"])?;
            let r = number(&certificate["point_order"], "certificate point order")?;
            if r == 0
                || r > order
                || !order.is_multiple_of(r)
                || p.is_none()
                || !c.on_curve(p)
                || c.reference_mul(p, r).is_some()
            {
                return Err("invalid certificate point or annihilator".into());
            }
            let factors = read_factors(&certificate["point_order_factorization"], r)?;
            if factors
                .iter()
                .any(|&(pdiv, _)| c.reference_mul(p, r / pdiv).is_none())
            {
                return Err("point order not exact".into());
            }
            let first = lo.div_ceil(r) * r;
            if first != order
                || first + r <= hi
                || number(&certificate["hasse_lower"], "Hasse lower")? != lo
                || number(&certificate["hasse_upper"], "Hasse upper")? != hi
            {
                return Err("point order does not isolate cardinality".into());
            }
        }
        Some("quadratic-twist-order-v1") => {
            if !twist {
                return Err("twist certificate attached to untwisted model".into());
            }
            let base = match c {
                Arithmetic::Prime(c) => {
                    Arithmetic::Prime(FastCurve::new("certificate base", c.f.p, base_a, base_b, 1))
                }
                Arithmetic::Binary { fast, .. } => {
                    let mut base = fast.clone();
                    base.a = base_a;
                    Arithmetic::Binary {
                        fast: base,
                        b: base_b,
                    }
                }
            };
            let base_order = number(&certificate["base_order"], "base order")?;
            if order.checked_add(base_order) != Some(2 * (c.q() + 1)) {
                return Err("twist order identity failed".into());
            }
            verify_certificate(
                &base,
                &certificate["base_certificate"],
                base_order,
                base_a,
                base_b,
                false,
            )?;
        }
        _ => return Err("unknown cardinality certificate".into()),
    }
    Ok(())
}

/// Verify public curve, twist, cardinality and subgroup facts without witness
/// access. Exact target membership in <G> follows only from a scalar replay.
pub fn verify_public(public: &Value) -> Result<(), String> {
    if public["schema"] != "ml-ecdlp-instance/v1"
        || !public["m"].as_u64().is_some_and(|m| (2..=15).contains(&m))
    {
        return Err("invalid instance/witness schema or arity".into());
    }
    let c = arithmetic(public)?;
    let model = &public["curve"];
    let order = number(&model["group_order"], "group order")?;
    let subgroup = number(&model["subgroup_order"], "subgroup order")?;
    let cofactor = number(&model["cofactor"], "cofactor")?;
    if subgroup <= 2
        || !is_prime_u64(subgroup)
        || subgroup as u128 * cofactor as u128 != order as u128
    {
        return Err("invalid prime subgroup/cofactor".into());
    }
    let factors = read_factors(&model["group_order_factorization"], order)?;
    if factors.last().map(|x| x.0) != Some(subgroup) {
        return Err("subgroup is not declared largest prime factor".into());
    }
    let base_a = number(&model["base_a"], "base a")?;
    let base_b = number(&model["base_b"], "base b")?;
    let d = number(&model["twist_parameter"], "twist parameter")?;
    let a = number(&model["a"], "a")?;
    let b = number(&model["b"], "b")?;
    let twist = public["twist"].as_bool().ok_or("missing twist boolean")?;
    if base_a >= c.q() || base_b >= c.q() || d >= c.q() {
        return Err("noncanonical twist data".into());
    }
    match &c {
        Arithmetic::Prime(c) => {
            let mul = |x: u64, y: u64| ((x as u128 * y as u128) % c.f.p as u128) as u64;
            if d == 0 || c.f.pow(c.f.to_mont(d), (c.f.p - 1) / 2) != c.f.neg(c.f.one) {
                return Err("twist parameter is not a nonsquare".into());
            }
            let expected = if twist {
                (mul(base_a, mul(d, d)), mul(base_b, mul(d, mul(d, d))))
            } else {
                (base_a, base_b)
            };
            if (a, b) != expected {
                return Err("wrong prime twist coefficients".into());
            }
        }
        Arithmetic::Binary { fast, .. } => {
            let generic = binary_curve(public)?;
            if F2mElement::from_biguint(&d.into(), fast.n).trace(&generic.irreducible) != 1 {
                return Err("binary twist parameter does not have trace one".into());
            }
            if b != base_b || a != if twist { base_a ^ d } else { base_a } {
                return Err("wrong binary twist coefficients".into());
            }
            if public["family"] == "koblitz" && (base_a > 1 || base_b != 1) {
                return Err("family mislabeled as Koblitz".into());
            }
        }
    }
    verify_certificate(
        &c,
        &model["order_certificate"],
        order,
        base_a,
        base_b,
        twist,
    )?;
    let degree = match &c {
        Arithmetic::Prime(_) => 1,
        Arithmetic::Binary { fast, b } => (1..=fast.n)
            .find(|&k| {
                fast.n.is_multiple_of(k)
                    && fast.gf.sqr_k(fast.a, k) == fast.a
                    && fast.gf.sqr_k(*b, k) == *b
            })
            .ok_or("no coefficient field")?,
    };
    if public["symmetry"]["coefficient_field_degree"].as_u64() != Some(degree as u64)
        || public["symmetry"]["frobenius_power"].as_u64() != Some(degree as u64)
    {
        return Err("incorrect coefficient field or Frobenius action".into());
    }
    for name in ["generator", "target"] {
        let p = point_words(&public[name])?;
        if p.is_none() || !c.on_curve(p) || c.reference_mul(p, subgroup).is_some() {
            return Err(format!("invalid public subgroup point {name}"));
        }
    }
    Ok(())
}

/// Verify the algebra and scalar with the library's separate general point
/// arithmetic, rather than trusting the fast arithmetic used for construction.
pub fn verify(public: &Value, private: &Value) -> Result<(), String> {
    verify_public(public)?;
    if private["schema"] != "ml-ecdlp-private-witness/v1" {
        return Err("invalid private witness schema".into());
    }
    let subgroup = number(&public["curve"]["subgroup_order"], "subgroup order")?;
    let scalar = number(&private["scalar"], "private scalar")?;
    let multiplier = number(&private["generator_multiplier"], "generator multiplier")?;
    if scalar == 0 || scalar >= subgroup || multiplier == 0 || multiplier >= subgroup {
        return Err("scalar/multiplier outside nonzero subgroup".into());
    }
    if public["family"] == "prime" {
        let c = prime_curve(public)?;
        let g = c.generator();
        let q = prime_point(&public["target"], &c)?;
        let base = prime_point(&private["base_generator"], &c)?;
        if matches!(g, Point::Infinity)
            || matches!(q, Point::Infinity)
            || matches!(base, Point::Infinity)
            || !c.is_on_curve(&g)
            || !c.is_on_curve(&q)
            || !c.is_on_curve(&base)
        {
            return Err("invalid prime point".into());
        }
        if g.scalar_mul_vartime(&subgroup.into(), &c.a_fe()) != Point::Infinity
            || base.scalar_mul_vartime(&subgroup.into(), &c.a_fe()) != Point::Infinity
            || base.scalar_mul_vartime(&multiplier.into(), &c.a_fe()) != g
            || g.scalar_mul_vartime(&scalar.into(), &c.a_fe()) != q
        {
            return Err("independent prime scalar replay failed".into());
        }
    } else {
        let c = binary_curve(public)?;
        let g = &c.generator;
        let q = binary_point(&public["target"], c.m)?;
        let base = binary_point(&private["base_generator"], c.m)?;
        if *g == BinaryPoint::Infinity
            || q == BinaryPoint::Infinity
            || base == BinaryPoint::Infinity
            || !c.is_on_curve(g)
            || !c.is_on_curve(&q)
            || !c.is_on_curve(&base)
        {
            return Err("invalid binary point".into());
        }
        // Reverse-bit reference multiplication gives a different schedule from
        // the native fast constructor and compares the library group formulas.
        let reference_mul = |p: &BinaryPoint, k: u64| {
            let mut result = BinaryPoint::Infinity;
            let mut addend = p.clone();
            let mut k = k;
            while k > 0 {
                if k & 1 == 1 {
                    result = point_add(&c, &result, &addend);
                }
                addend = point_add(&c, &addend, &addend);
                k >>= 1;
            }
            result
        };
        if scalar_mul(&c, g, &subgroup.into()) != BinaryPoint::Infinity
            || scalar_mul(&c, &base, &subgroup.into()) != BinaryPoint::Infinity
            || reference_mul(&base, multiplier) != *g
            || reference_mul(g, scalar) != q
        {
            return Err("independent binary scalar replay failed".into());
        }
    }
    Ok(())
}

pub fn verify_factor_base(public: &Value, base: &Value) -> Result<(), String> {
    if base["rule"] == "planted-known-log-subgroup/v1" {
        return Err(
            "planted factor base requires restricted witness replay via verify_planted".into(),
        );
    }
    verify_natural_factor_base(public, base)
}

fn verify_planted_factor_base(public: &Value, base: &Value) -> Result<(), String> {
    if base["schema"] == "ml-ecdlp-factor-base/v1"
        && base["rule"] == "planted-known-log-subgroup/v1"
    {
        let c = arithmetic(public)?;
        let r = number(&public["curve"]["subgroup_order"], "subgroup")?;
        let points = base["points"].as_array().ok_or("missing planted points")?;
        if points.is_empty()
            || points.len() > 4096
            || base["requested_size"].as_u64() != Some(points.len() as u64)
            || base["construction_status"] != "verified"
            || base["known_log_features"] != false
        {
            return Err("invalid planted factor-base size or public policy".into());
        }
        let mut seen = HashSet::new();
        for point in points {
            let p = point_words(point)?;
            if p.is_none()
                || !c.on_curve(p)
                || c.reference_mul(p, r).is_some()
                || seen.contains(&p)
                || seen.contains(&c.neg(p))
            {
                return Err("invalid, repeated or opposite planted base point".into());
            }
            seen.insert(p);
        }
        let k = base["frobenius_power"]
            .as_u64()
            .ok_or("missing planted Frobenius power")?;
        if k == 0 || k > 61 || public["symmetry"]["frobenius_power"].as_u64() != Some(k) {
            return Err("invalid planted Frobenius power".into());
        }
        for (name, frobenius) in [("negation_action", false), ("frobenius_action", true)] {
            let action = base[name]
                .as_array()
                .ok_or("missing planted symmetry action")?;
            if action.len() != points.len() {
                return Err("planted symmetry action length differs".into());
            }
            for (i, destination) in action.iter().enumerate() {
                let p = point_words(&points[i])?;
                let mapped = if frobenius {
                    c.frobenius(p, k as u32)
                } else {
                    c.neg(p)
                };
                match destination.as_u64() {
                    Some(j)
                        if j < points.len() as u64
                            && point_words(&points[j as usize])? == mapped => {}
                    None if destination.is_null() && !seen.contains(&mapped) => {}
                    _ => return Err("invalid planted symmetry action".into()),
                }
            }
        }
        if base["negation_closed"].as_bool()
            != Some(
                base["negation_action"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|x| !x.is_null()),
            )
        {
            return Err("incorrect planted negation closure".into());
        }
        let orbits = base["orbits"].as_array().ok_or("missing planted orbits")?;
        if orbits.len() != points.len() {
            return Err("planted orbit count differs".into());
        }
        let order = match &c {
            Arithmetic::Binary { fast, .. } => fast.n / k as u32,
            Arithmetic::Prime(_) => 1,
        };
        for (point, orbit) in points.iter().zip(orbits) {
            let saved = orbit["points"]
                .as_array()
                .ok_or("missing planted orbit points")?;
            if saved.is_empty() || saved.len() > order as usize {
                return Err("invalid planted orbit length".into());
            }
            let start = point_words(point)?;
            let mut next = start;
            let mut orbit_seen = HashSet::new();
            for p in saved {
                if point_words(p)? != next || !orbit_seen.insert(next) || !c.on_curve(next) {
                    return Err("invalid planted orbit witness".into());
                }
                next = c.frobenius(next, k as u32);
            }
            if next != start
                || orbit["closed"].as_bool() != Some(true)
                || orbit["stabilizer_size"].as_u64() != Some(order as u64 / saved.len() as u64)
            {
                return Err("incorrect planted orbit closure or stabilizer".into());
            }
        }
        return Ok(());
    }
    Err("invalid planted factor-base schema or policy".into())
}

fn verify_natural_factor_base(public: &Value, base: &Value) -> Result<(), String> {
    if base["schema"] != "ml-ecdlp-factor-base/v1"
        || base["rule"] != "small-x-prime-primary-projection-with-cyclic-membership"
    {
        return Err("invalid factor-base schema or membership rule".into());
    }
    let c = arithmetic(public)?;
    let r = number(&public["curve"]["subgroup_order"], "subgroup order")?;
    let order = number(&public["curve"]["group_order"], "group order")?;
    let cofactor = number(&public["curve"]["cofactor"], "cofactor")?;
    let generator = point_words(&public["generator"])?;
    if r <= 2
        || !is_prime_u64(r)
        || order == 0
        || !order.is_multiple_of(r)
        || order / r != cofactor
        || generator.is_none()
        || !c.on_curve(generator)
        || c.reference_mul(generator, r).is_some()
    {
        return Err("invalid factor-base subgroup parameters".into());
    }
    let factors = read_factors(&public["curve"]["group_order_factorization"], order)?;
    if factors.last().map(|&(p, _)| p) != Some(r) {
        return Err("factor-base subgroup differs from largest prime factor".into());
    }
    let membership = cofactor
        .is_multiple_of(r)
        .then(|| CyclicMembership::new(&c, generator, r));
    let points = base["points"]
        .as_array()
        .ok_or("missing factor base points")?;
    let mut seen = HashSet::new();
    for point in points {
        let p = point_words(point)?;
        if p.is_none()
            || !c.on_curve(p)
            || !seen.insert(p)
            || c.reference_mul(p, r).is_some()
            || membership
                .as_ref()
                .is_some_and(|test| test.logarithm(&c, p).is_none())
        {
            return Err("invalid, duplicated, or wrong-subgroup factor-base point".into());
        }
    }
    let k = base["frobenius_power"]
        .as_u64()
        .ok_or("missing Frobenius power")?;
    if k == 0 || k > 61 || public["symmetry"]["frobenius_power"].as_u64() != Some(k) {
        return Err("invalid Frobenius power".into());
    }
    let candidates = base["construction_candidates"]
        .as_array()
        .ok_or("missing factor-base construction candidates")?;
    let mut reconstructed = Vec::new();
    let mut reconstructed_seen = HashSet::new();
    for (index, row) in candidates.iter().enumerate() {
        let x = number(&row["x"], "factor base source x")?;
        if x != index as u64 + 1 || x >= c.q() {
            return Err("factor-base candidate sequence differs from rule".into());
        }
        let lifts = c.lift(x);
        let saved = row["lifts"].as_array().ok_or("missing candidate lifts")?;
        if lifts.len() != saved.len() {
            return Err("incomplete factor-base lifts".into());
        }
        for (p, saved) in lifts.into_iter().zip(saved) {
            let (projected, projection_multiplier) = prime_projection(&c, p, order, r, true);
            if c.reference_mul(projected, r).is_some() {
                return Err(
                    "claimed group order does not project factor-base candidates into r-torsion"
                        .into(),
                );
            }
            let subgroup_member = projected.is_some()
                && membership
                    .as_ref()
                    .is_none_or(|test| test.logarithm(&c, projected).is_some());
            let accepted = subgroup_member && reconstructed_seen.insert(projected);
            if point_words(&saved["lifted"])? != p
                || point_words(&saved["projected"])? != projected
                || saved["projection_multiplier"] != u(projection_multiplier)
                || saved["cyclic_subgroup_member"].as_bool() != Some(subgroup_member)
                || saved["accepted"].as_bool() != Some(accepted)
            {
                return Err("invalid factor-base construction witness".into());
            }
            if accepted {
                reconstructed.push(point_json(projected));
            }
        }
    }
    if reconstructed != *points {
        return Err("factor base differs from projected construction".into());
    }
    for (name, action) in [("negation_action", false), ("frobenius_action", true)] {
        let mapping = base[name].as_array().ok_or("missing symmetry action")?;
        if mapping.len() != points.len() {
            return Err("symmetry map length differs".into());
        }
        for (i, destination) in mapping.iter().enumerate() {
            let p = point_words(&points[i])?;
            let mapped = if action {
                c.frobenius(p, k as u32)
            } else {
                c.neg(p)
            };
            match destination.as_u64() {
                Some(j)
                    if j < points.len() as u64 && point_words(&points[j as usize])? == mapped => {}
                None if destination.is_null() && !seen.contains(&mapped) => {}
                _ => return Err("invalid factor-base symmetry map".into()),
            }
        }
    }
    let closed = base["negation_action"]
        .as_array()
        .ok_or("missing negation map")?
        .iter()
        .all(|x| !x.is_null());
    if base["negation_closed"].as_bool() != Some(closed) {
        return Err("incorrect negation closure label".into());
    }
    let orbits = base["orbits"]
        .as_array()
        .ok_or("missing Frobenius orbits")?;
    if orbits.len() != points.len() {
        return Err("orbit count differs from factor base".into());
    }
    let automorphism_order = match &c {
        Arithmetic::Binary { fast, .. } => fast.n / k as u32,
        Arithmetic::Prime(_) => 1,
    };
    for (point, orbit) in points.iter().zip(orbits) {
        let orbit_points = orbit["points"].as_array().ok_or("missing orbit points")?;
        if orbit_points.is_empty() || orbit_points.len() > automorphism_order as usize {
            return Err("invalid orbit length".into());
        }
        let start = point_words(point)?;
        let mut next = start;
        let mut orbit_seen = HashSet::new();
        for saved in orbit_points {
            if point_words(saved)? != next || !orbit_seen.insert(next) || !c.on_curve(next) {
                return Err("invalid Frobenius orbit witness".into());
            }
            next = c.frobenius(next, k as u32);
        }
        if next != start
            || orbit["closed"].as_bool() != Some(true)
            || orbit["stabilizer_size"].as_u64()
                != Some(automorphism_order as u64 / orbit_points.len() as u64)
        {
            return Err("incorrect orbit closure/stabilizer".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn config(family: &str, n: u32, twist: bool) -> CaseConfig {
        CaseConfig {
            family: family.into(),
            n,
            m: 2,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 8,
            degree_bound: 4,
            work_limit: 1000,
            solver: "enumerate".into(),
        }
    }
    #[test]
    fn planted_control_is_deterministic_and_independently_replayed() {
        let mut cfg = config("koblitz", 23, false);
        cfg.factor_base_size = 4;
        cfg.m = 15; // Terms are sampled with replacement, even above base size.
        let natural = generate(&cfg).unwrap();
        let planted = generate_planted(&cfg).unwrap();
        assert!(planted.public_instance.get("population").is_none());
        assert!(planted.public_instance.get("sampling").is_none());
        assert!(planted.public_instance.get("construction_trace").is_none());
        assert_eq!(
            planted.private_witness["planted_construction"]["population"],
            "planted-pdp-v1"
        );
        assert_eq!(
            natural.public_instance["population"],
            "unconditioned-uniform-nonzero-subgroup-scalar"
        );
        assert_eq!(planted.factor_base["rule"], "planted-known-log-subgroup/v1");
        assert_eq!(planted.factor_base["points"].as_array().unwrap().len(), 4);
        assert_eq!(
            planted.private_witness["planted_construction"]["target_draws"][0]["terms"]
                .as_array()
                .unwrap()
                .len(),
            15
        );
        assert_eq!(
            serde_json::to_value(&planted).unwrap(),
            serde_json::to_value(generate_planted(&cfg).unwrap()).unwrap()
        );
        verify_planted(&cfg, &planted).unwrap();
        assert!(verify_factor_base(&planted.public_instance, &planted.factor_base).is_err());
        let mut leaked = planted.clone();
        leaked.public_instance["population"] = json!("planted-pdp-v1");
        assert!(verify_planted(&cfg, &leaked).is_err());
        let mut wrong_sign = planted.clone();
        wrong_sign.private_witness["planted_construction"]["target_draws"][0]["terms"][0]["sign"] =
            json!(0);
        assert!(verify_planted(&cfg, &wrong_sign).is_err());
        let mut wrong_log = planted.clone();
        wrong_log.private_witness["planted_construction"]["base_logs"][0] = json!("0");
        assert!(verify_planted(&cfg, &wrong_log).is_err());
        let mut wrong_target = planted.clone();
        wrong_target.public_instance["target"] = natural.public_instance["target"].clone();
        assert!(verify_planted(&cfg, &wrong_target).is_err());
    }
    #[test]
    fn exact_prime_context_matches_legacy_small_cofactor_arithmetic() {
        use crate::ecc::curve::CurveParams;
        for twist in [false, true] {
            let data = generate(&config("prime", 11, twist)).unwrap();
            let exact = prime_curve(&data.public_instance).unwrap();
            let legacy = CurveParams {
                name: "compatibility fixture",
                p: exact.p.clone(),
                a: exact.a.clone(),
                b: exact.b.clone(),
                gx: exact.gx.clone(),
                gy: exact.gy.clone(),
                n: exact.n.clone(),
                h: u32::try_from(exact.cofactor).unwrap(),
            };
            assert_eq!(exact.generator(), legacy.generator());
            let old_fast = FastCurve::from_params(&legacy).unwrap();
            let new_fast = exact.fast().unwrap();
            assert_eq!(
                (old_fast.f.p, old_fast.a, old_fast.b, old_fast.n),
                (new_fast.f.p, new_fast.a, new_fast.b, new_fast.n)
            );
            for scalar in [0u64, 1, 2, 7, 31] {
                let old = legacy
                    .generator()
                    .scalar_mul_vartime(&scalar.into(), &legacy.a_fe());
                let new = exact
                    .generator()
                    .scalar_mul_vartime(&scalar.into(), &exact.a_fe());
                assert_eq!(old, new);
                assert_eq!(legacy.is_on_curve(&old), exact.is_on_curve(&new));
            }
            verify(&data.public_instance, &data.private_witness).unwrap();
        }
    }

    #[test]
    fn exact_prime_cofactor_above_u32_replays_and_solves_without_witness() {
        // A deterministic, bounded software fixture, not campaign sampling.
        // Certify the actual order instead of attaching a made-up cofactor.
        let h = 1u64 << 33;
        // Frozen from the first prime r in the declared odd 3..=1023 search.
        let r = 31;
        let p = 266_287_972_351;
        assert!(is_prime_u64(r) && is_prime_u64(p));
        assert_eq!(p + 1, r * h);
        let c = Arithmetic::Prime(FastCurve::new("large-cofactor fixture", p, p - 1, 0, r));
        let (order, certificate) =
            certify_count(&c, &mut StdRng::seed_from_u64(0xeca0fac7)).unwrap();
        assert_eq!(order, p + 1);
        verify_certificate(&c, &certificate, order, p - 1, 0, false).unwrap();
        let factors = factor(order).unwrap();
        assert_eq!(factors, vec![(2, 33), (r, 1)]);
        let generator = (1..=1024)
            .flat_map(|x| c.lift(x))
            .find_map(|point| {
                let (projected, multiplier) = prime_projection(&c, point, order, r, false);
                assert_eq!(projected, c.reference_mul(point, multiplier));
                projected
            })
            .expect("bounded nonzero subgroup generator");
        let generator = Some(generator);
        assert_eq!(c.reference_mul(generator, r), None);
        let public = json!({
            "schema":"ml-ecdlp-instance/v1", "family":"prime",
            "n":64-p.leading_zeros(), "m":2, "twist":false,
            "field":{"characteristic":u(p),"degree":1,"modulus":u(p)},
            "curve":{"a":u(p-1),"b":"0","base_a":u(p-1),"base_b":"0",
                "twist_parameter":u(p-1),"group_order":u(order),
                "subgroup_order":u(r),"cofactor":u(h),
                "group_order_factorization":factors_json(&factors),"order_certificate":certificate},
            "generator":point_json(generator),"target":point_json(c.reference_mul(generator,2)),
            "symmetry":{"coefficient_field_degree":1,"frobenius_power":1}
        });
        let witness = json!({"schema":"ml-ecdlp-private-witness/v1", "scalar":"2",
            "generator_multiplier":"1","base_generator":point_json(generator)});
        verify(&public, &witness).unwrap();
        let exact = prime_curve(&public).unwrap();
        assert!(exact.cofactor > u32::MAX as u64);
        assert_eq!(exact.cofactor, h);
        assert_eq!(exact.group_order(), BigUint::from(order));
        assert_eq!(exact.fast().unwrap().n, r);
        let case = CaseData {
            public_instance: public,
            private_witness: Value::Null,
            // BSGS needs only a public domain. This minimal adapter fixture is
            // not a claimed factor-base construction transcript or export.
            factor_base: json!({"points":[point_json(generator)]}),
        };
        let mut cfg = config("prime", 64 - p.leading_zeros(), false);
        cfg.solver = "bsgs".into();
        let solved = super::super::solvers::solve(&cfg, &case).unwrap();
        assert_eq!(solved.outcome["scalar_recovered_without_witness"], true);
        assert_eq!(solved.outcome["scalar_replay_verified"], true);
        let label = solved
            .artifacts
            .iter()
            .find(|a| a.role == "restricted-independent-scalar-label")
            .unwrap();
        let mut value: Value = serde_json::from_slice(&label.bytes).unwrap();
        assert_eq!(value["scalar"], "2");
        super::super::solvers::verify_solution_artifact(
            &case.public_instance,
            &case.factor_base,
            &label.role,
            &value,
        )
        .unwrap();
        value["scalar"] = json!("3");
        assert!(super::super::solvers::verify_solution_artifact(
            &case.public_instance,
            &case.factor_base,
            &label.role,
            &value
        )
        .is_err());
        // Bridge the exact arithmetic context into the bounded native root
        // adapter on this same frozen public fixture; no target resampling.
        cfg.solver = "root".into();
        assert!(case.private_witness.is_null());
        let rooted = super::super::solvers::solve(&cfg, &case).unwrap();
        let certificates: Vec<_> = rooted
            .artifacts
            .iter()
            .filter(|a| a.role == "prime-root-certificates/v1")
            .collect();
        assert_eq!(certificates.len(), 1);
        let certificate: Value = serde_json::from_slice(&certificates[0].bytes).unwrap();
        assert!(!certificate["queries"].as_array().unwrap().is_empty());
        super::super::solvers::verify_prime_root_artifact(
            &case.public_instance,
            &case.factor_base,
            &cfg,
            &certificate,
        )
        .unwrap();
        super::super::verify_prime_root_outcome(&rooted.outcome, &certificate).unwrap();
        assert_eq!(rooted.outcome["scalar_recovered_without_witness"], false);
        assert_eq!(certificate["execution_trace_complete_verified"], false);
        assert_eq!(certificate["total_operation_count_verified"], false);
        if certificate["censored"] == true {
            assert_eq!(rooted.outcome["status"], "budget_exhausted");
            assert_eq!(rooted.outcome["solver_completed"], false);
            assert_eq!(certificate["root_queries_completed"], false);
        }
        for solver in ["f4", "pkm"] {
            cfg.solver = solver.into();
            let result = super::super::solvers::solve(&cfg, &case).unwrap();
            assert_eq!(
                result.outcome["status"], "unsupported",
                "{solver}: {}",
                result.outcome
            );
        }
        eprintln!(
            "exact-cofactor fixture p={p} r={r} h={h} public={}",
            case.public_instance
        );
    }

    #[test]
    fn exact_prime_cofactor_rejects_invalid_metadata_without_overflow() {
        let data = generate(&config("prime", 11, false)).unwrap();
        for bad in ["0", "01", "-1", "18446744073709551616"] {
            let mut public = data.public_instance.clone();
            public["curve"]["cofactor"] = json!(bad);
            assert!(prime_curve(&public).is_err(), "accepted {bad}");
        }
        let mut public = data.public_instance.clone();
        public["curve"]["group_order"] = u(1);
        assert!(prime_curve(&public).is_err());
        public["curve"]["group_order"] = u(u64::MAX);
        public["curve"]["cofactor"] = u(u64::MAX);
        public["curve"]["subgroup_order"] = u(3);
        assert!(prime_curve(&public).is_err());
        public["curve"]["subgroup_order"] = u(0);
        assert!(prime_curve(&public).is_err());
    }

    #[test]
    fn independent_replay_rejects_tampering() {
        for family in ["koblitz", "binary", "prime"] {
            for twist in [false, true] {
                let data = generate(&config(family, 11, twist)).unwrap();
                verify(&data.public_instance, &data.private_witness).unwrap();
                verify_factor_base(&data.public_instance, &data.factor_base).unwrap();
                let mut witness = data.private_witness.clone();
                witness["scalar"] = json!("0");
                assert!(verify(&data.public_instance, &witness).is_err());
                let mut public = data.public_instance.clone();
                public["twist"] = json!(!twist);
                assert!(verify(&public, &data.private_witness).is_err());
                let mut public = data.public_instance.clone();
                public["curve"]["group_order"] = json!("123");
                assert!(verify(&public, &data.private_witness).is_err());
                let mut public = data.public_instance.clone();
                public["generator"] = json!({"x":"invalid","y":"0"});
                assert!(verify_factor_base(&public, &data.factor_base).is_err());
                let mut public = data.public_instance.clone();
                public["curve"]["subgroup_order"] = json!("0");
                assert!(verify_factor_base(&public, &data.factor_base).is_err());
                let mut public = data.public_instance.clone();
                let r = number(&public["curve"]["subgroup_order"], "r").unwrap();
                let h = number(&public["curve"]["cofactor"], "h").unwrap();
                let fake_h = if h == 3 { 5 } else { 3 };
                public["curve"]["group_order"] = u(fake_h * r);
                public["curve"]["cofactor"] = u(fake_h);
                assert!(verify_factor_base(&public, &data.factor_base).is_err());
                public["curve"]["group_order"] = u(0);
                public["curve"]["cofactor"] = u(0);
                assert!(verify_factor_base(&public, &data.factor_base).is_err());
            }
        }
    }
    #[test]
    fn repeated_prime_projection_checks_actual_cyclic_subgroup() {
        // Seed7 selects a=1; its odd-degree quadratic twist is a=0 and has
        // #E=2116=4*23^2 with rank-two rational23-torsion.
        let data = generate(&config("koblitz", 11, true)).unwrap();
        let public = &data.public_instance;
        assert_eq!(public["curve"]["a"], "0");
        assert_eq!(public["curve"]["group_order"], "2116");
        assert_eq!(public["curve"]["subgroup_order"], "23");
        let c = arithmetic(public).unwrap();
        let order = number(&public["curve"]["group_order"], "order").unwrap();
        let r = number(&public["curve"]["subgroup_order"], "r").unwrap();
        assert!(order.is_multiple_of(r * r));
        let generator = point_words(&public["generator"]).unwrap();
        let mut projector = SubgroupProjector::new(public).unwrap();
        let mut admitted = HashSet::new();
        let mut rejected_nonzero = 0;
        for x in 1..c.q() {
            for input in c.lift(x) {
                let (p, multiplier, member, log) = projector.project(&point_json(input)).unwrap();
                let p = point_words(&p).unwrap();
                assert_eq!(c.reference_mul(input, multiplier), p);
                if member {
                    admitted.insert(p);
                    assert_eq!(c.reference_mul(generator, log.unwrap()), p);
                } else if p.is_some() {
                    rejected_nonzero += 1;
                    assert!(log.is_none());
                }
            }
        }
        assert_eq!(admitted.len() as u64, r - 1);
        assert!(rejected_nonzero > 0);
        assert_eq!(projector.counters()["method"], "bounded-bsgs-diagnostic");
    }
    #[test]
    fn even_binary_twist_is_nontrivial_and_orders_pair() {
        let base = generate(&config("koblitz", 12, false)).unwrap();
        let twist = generate(&config("koblitz", 12, true)).unwrap();
        let c = binary_curve(&twist.public_instance).unwrap();
        let d = number(&twist.public_instance["curve"]["twist_parameter"], "d").unwrap();
        assert_ne!(d, 1);
        assert_eq!(
            F2mElement::from_biguint(&d.into(), 12).trace(&c.irreducible),
            1
        );
        assert_eq!(
            number(&base.public_instance["curve"]["group_order"], "N").unwrap()
                + number(&twist.public_instance["curve"]["group_order"], "N").unwrap(),
            2 * ((1 << 12) + 1)
        );
        let mut bad = twist.public_instance.clone();
        bad["curve"]["twist_parameter"] = json!("1");
        assert!(verify(&bad, &twist.private_witness).is_err());
    }
    #[test]
    fn hasse_counter_matches_exhaustive_count() {
        let mut rng = StdRng::seed_from_u64(3);
        let c = Arithmetic::Prime(FastCurve::new("unit", 2039, 2036, 17, 1));
        let exhaustive = 1 + (0..c.q()).map(|x| c.lift(x).len() as u64).sum::<u64>();
        let (count, cert) = certify_count(&c, &mut rng).unwrap();
        assert_eq!(count, exhaustive);
        verify_certificate(&c, &cert, count, 2036, 17, false).unwrap();
    }
    #[test]
    fn requested_boundaries_all_families_and_twists_replay() {
        // Correctness checks at the requested field boundaries, not timing or
        // comparative experiments. Constructors have fixed 16 Hasse samples,
        // 300000 factoring iterations per factor, and 1024 generator samples.
        for n in [23, 24, 31, 47, 61] {
            for family in ["koblitz", "binary", "prime"] {
                for twist in [false, true] {
                    let mut cfg = config(family, n, twist);
                    cfg.factor_base_size = 2;
                    let data = generate(&cfg)
                        .unwrap_or_else(|e| panic!("{family} n={n} twist={twist}: {e}"));
                    verify(&data.public_instance, &data.private_witness).unwrap();
                    verify_factor_base(&data.public_instance, &data.factor_base).unwrap();
                    assert!(data.public_instance.get("curve_seed").is_none());
                    assert!(data.public_instance.get("generator_index").is_none());
                    assert!(data.public_instance.get("key_index").is_none());
                    if family == "prime" && n == 61 {
                        assert_eq!(
                            number(&data.public_instance["field"]["modulus"], "p").unwrap(),
                            (1u64 << 61) - 1
                        );
                    }
                }
            }
        }
    }
}
