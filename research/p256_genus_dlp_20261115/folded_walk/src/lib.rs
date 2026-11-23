//! Correctness-first signed-orbit walk. Timing requires separate admitted execution.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use p256_genus_check::{hash::sha256::sha256, inv, sub, transfer, Divisor, Jacobian, Point, Poly};
use p256_transfer_search::Curve;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

pub trait Group {
    type Element: Clone + PartialEq;
    fn zero(&self) -> Self::Element;
    fn add(&self, a: &Self::Element, b: &Self::Element) -> Self::Element;
    fn neg(&self, a: &Self::Element) -> Self::Element;
    fn encode(&self, a: &Self::Element) -> Vec<u8>;
    fn rotate(&self, a: &Self::Element) -> Self::Element;
    fn lambda(&self) -> BigUint;
    fn rotations(&self) -> usize;
    fn valid(&self, a: &Self::Element) -> bool;
    fn mul(&self, n: &BigUint, p: &Self::Element) -> Self::Element {
        let (mut k, mut a, mut r) = (n.clone(), p.clone(), self.zero());
        while !k.is_zero() {
            if k.bit(0) {
                r = self.add(&r, &a);
            }
            a = self.add(&a, &a);
            k >>= 1;
        }
        r
    }
    fn canonical(&self, x: &Self::Element, n: &BigUint) -> (Self::Element, Vec<u8>, BigUint) {
        let mut current = x.clone();
        let mut multiplier = BigUint::one();
        let mut best = (x.clone(), self.encode(x), BigUint::one());
        for _ in 0..self.rotations() {
            for (candidate, m) in [
                (current.clone(), multiplier.clone()),
                (self.neg(&current), sub(&BigUint::zero(), &multiplier, n)),
            ] {
                let key = self.encode(&candidate);
                if key < best.1 {
                    best = (candidate, key, m);
                }
            }
            current = self.rotate(&current);
            multiplier = multiplier * self.lambda() % n;
        }
        best
    }
}

fn fixed_bytes(x: &BigUint, width: usize) -> Vec<u8> {
    let raw = x.to_bytes_be();
    assert!(raw.len() <= width);
    let mut result = vec![0; width - raw.len()];
    result.extend(raw);
    result
}

pub struct EllipticGroup {
    pub curve: Curve,
}
impl Group for EllipticGroup {
    type Element = Point;
    fn zero(&self) -> Point {
        None
    }
    fn add(&self, a: &Point, b: &Point) -> Point {
        self.curve.add(a, b)
    }
    fn neg(&self, a: &Point) -> Point {
        a.as_ref()
            .map(|(x, y)| (x.clone(), sub(&BigUint::zero(), y, &self.curve.p)))
    }
    fn encode(&self, a: &Point) -> Vec<u8> {
        let Some((x, y)) = a else {
            return vec![0];
        };
        let width = self.curve.p.bits().div_ceil(8) as usize;
        let mut key = vec![1];
        key.extend(fixed_bytes(x, width));
        key.extend(fixed_bytes(y, width));
        key
    }
    fn rotate(&self, a: &Point) -> Point {
        a.clone()
    }
    fn lambda(&self) -> BigUint {
        BigUint::one()
    }
    fn rotations(&self) -> usize {
        1
    }
    fn valid(&self, a: &Point) -> bool {
        self.curve.contains(a)
    }
}

pub struct MixedGroup {
    pub curve: Curve,
    pub jacobian: Jacobian,
    pub zeta: BigUint,
    pub eigenvalue: BigUint,
    powers: [BigUint; 3],
}
impl MixedGroup {
    pub fn new(curve: Curve, order: &BigUint) -> Self {
        let zeta = cube_root(&curve.p);
        let eigenvalue = cube_root(order);
        let powers = [BigUint::one(), zeta.clone(), &zeta * &zeta % &curve.p];
        let jacobian = Jacobian::new(3, &curve.b, &curve.p);
        Self {
            curve,
            jacobian,
            zeta,
            eigenvalue,
            powers,
        }
    }
    pub fn image(&self, p: &Point) -> Divisor {
        let left = transfer(3, p, &self.curve.p);
        let right = self.rotate(&transfer(
            3,
            &self.curve.mul(&self.eigenvalue, p),
            &self.curve.p,
        ));
        self.jacobian.add(&left, &self.jacobian.neg(&right))
    }
}
impl Group for MixedGroup {
    type Element = Divisor;
    fn zero(&self) -> Divisor {
        self.jacobian.zero()
    }
    fn add(&self, a: &Divisor, b: &Divisor) -> Divisor {
        self.jacobian.add(a, b)
    }
    fn neg(&self, a: &Divisor) -> Divisor {
        self.jacobian.neg(a)
    }
    fn encode(&self, a: &Divisor) -> Vec<u8> {
        let d = a.0.degree().unwrap();
        let width = self.curve.p.bits().div_ceil(8) as usize;
        let mut key = vec![d as u8];
        for i in 0..d {
            key.extend(fixed_bytes(&a.0.coeff(i), width));
        }
        for i in 0..d {
            key.extend(fixed_bytes(&a.1.coeff(i), width));
        }
        key
    }
    fn rotate(&self, a: &Divisor) -> Divisor {
        let d = a.0.degree().unwrap();
        let p = &self.curve.p;
        (
            Poly::new(
                a.0 .0
                    .iter()
                    .enumerate()
                    .map(|(i, c)| c * &self.powers[(i + 2 * d) % 3] % p)
                    .collect(),
                p,
            ),
            Poly::new(
                a.1 .0
                    .iter()
                    .enumerate()
                    .map(|(i, c)| c * &self.powers[(i + 1) % 3] % p)
                    .collect(),
                p,
            ),
        )
    }
    fn lambda(&self) -> BigUint {
        self.eigenvalue.clone()
    }
    fn rotations(&self) -> usize {
        3
    }
    fn valid(&self, a: &Divisor) -> bool {
        self.jacobian.valid(a)
    }
}

/// Fixed-width external source control. Its timing is admitted only on a
/// matched isolated runner; this is not a global fastest-implementation claim.
pub struct FastP256;
impl Group for FastP256 {
    type Element = p256::ProjectivePoint;
    fn zero(&self) -> Self::Element {
        p256::ProjectivePoint::IDENTITY
    }
    fn add(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        *a + *b
    }
    fn neg(&self, a: &Self::Element) -> Self::Element {
        -*a
    }
    fn encode(&self, a: &Self::Element) -> Vec<u8> {
        use p256::elliptic_curve::sec1::ToEncodedPoint;
        p256::AffinePoint::from(*a)
            .to_encoded_point(true)
            .as_bytes()
            .to_vec()
    }
    fn rotate(&self, a: &Self::Element) -> Self::Element {
        *a
    }
    fn lambda(&self) -> BigUint {
        BigUint::one()
    }
    fn rotations(&self) -> usize {
        1
    }
    fn valid(&self, _: &Self::Element) -> bool {
        true
    }
    fn mul(&self, n: &BigUint, p: &Self::Element) -> Self::Element {
        use p256::elliptic_curve::PrimeField;
        let bytes: [u8; 32] = fixed_bytes(n, 32).try_into().unwrap();
        let scalar = Option::<p256::Scalar>::from(p256::Scalar::from_repr(bytes.into())).unwrap();
        *p * scalar
    }
    fn canonical(&self, x: &Self::Element, n: &BigUint) -> (Self::Element, Vec<u8>, BigUint) {
        let mut key = self.encode(x);
        if key[0] == 3 {
            key[0] = 2;
            (-*x, key, n - 1u32)
        } else {
            (*x, key, BigUint::one())
        }
    }
}
impl FastP256 {
    pub fn to_point(p: &p256::ProjectivePoint) -> Point {
        use p256::elliptic_curve::sec1::ToEncodedPoint;
        let e = p256::AffinePoint::from(*p).to_encoded_point(false);
        if e.as_bytes() == [0] {
            None
        } else {
            Some((
                BigUint::from_bytes_be(&e.as_bytes()[1..33]),
                BigUint::from_bytes_be(&e.as_bytes()[33..65]),
            ))
        }
    }
}

pub fn cube_root(p: &BigUint) -> BigUint {
    assert_eq!(p % 3u32, BigUint::one());
    (2u32..)
        .map(|a| BigUint::from(a).modpow(&((p - 1u32) / 3u32), p))
        .find(|v| *v != BigUint::one())
        .unwrap()
}

#[derive(Clone)]
pub struct State<E> {
    pub element: E,
    pub key: Vec<u8>,
    pub a: BigUint,
    pub b: BigUint,
}
fn normalize<G: Group>(
    g: &G,
    n: &BigUint,
    x: G::Element,
    a: BigUint,
    b: BigUint,
) -> State<G::Element> {
    let (element, key, m) = g.canonical(&x, n);
    State {
        element,
        key,
        a: a * m.clone() % n,
        b: b * m % n,
    }
}

fn advance<G: Group>(
    g: &G,
    n: &BigUint,
    current: &State<G::Element>,
    jump: &(G::Element, BigUint, BigUint),
) -> State<G::Element> {
    normalize(
        g,
        n,
        g.add(&current.element, &jump.0),
        (&current.a + &jump.1) % n,
        (&current.b + &jump.2) % n,
    )
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Counters {
    pub setup_additions: u64,
    pub walk_additions: u64,
    pub escape_doublings: u64,
    pub escape_jumps: u64,
    pub canonicalizations: u64,
    pub distinguished_hits: u64,
    pub fruitless_collisions: u64,
    pub short_cycles: u64,
    pub restarts: u64,
    pub max_table_entries: usize,
    pub setup_scalar_multiplications: u64,
    pub answer_scalar_multiplications: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub jumps: usize,
    pub history: usize,
    pub dp_bits: u32,
    pub step_cap: u64,
    pub restart_cap: u32,
    pub seed: u64,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            jumps: 32,
            history: 16,
            dp_bits: 2,
            step_cap: 20000,
            restart_cap: 8,
            seed: 20261115,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Outcome {
    pub status: String,
    pub scalar: Option<String>,
    pub counters: Counters,
    pub scalar_replay_matches: bool,
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    fn scalar(&mut self, n: &BigUint) -> BigUint {
        let mut bytes = vec![];
        for _ in 0..n.bits().div_ceil(64) {
            bytes.extend(self.next().to_le_bytes());
        }
        BigUint::from_bytes_le(&bytes) % n
    }
}
fn hash(key: &[u8]) -> u64 {
    u64::from_le_bytes(sha256(key)[..8].try_into().unwrap())
}
fn recover<E>(x: &State<E>, y: &State<E>, n: &BigUint) -> Option<BigUint> {
    let denominator = sub(&y.b, &x.b, n);
    if denominator.is_zero() {
        None
    } else {
        Some(sub(&x.a, &y.a, n) * inv(&denominator, n) % n)
    }
}
fn check_answer<G: Group>(g: &G, p: &G::Element, q: &G::Element, k: &BigUint) -> bool {
    g.mul(k, p) == *q
}

pub fn solve<G: Group>(
    g: &G,
    n: &BigUint,
    p: &G::Element,
    q: &G::Element,
    cfg: &Config,
) -> Outcome {
    assert!(cfg.jumps >= 2 && cfg.history >= 2 && cfg.dp_bits < 63);
    let mut counts = Counters::default();
    let mut rng = Rng(cfg.seed);
    if !g.valid(p) || !g.valid(q) || *p == g.zero() {
        return Outcome {
            status: "invalid_input".into(),
            scalar: None,
            counters: counts,
            scalar_replay_matches: false,
        };
    }
    if *q == g.zero() {
        return Outcome {
            status: "solved".into(),
            scalar: Some("0".into()),
            counters: counts,
            scalar_replay_matches: true,
        };
    }
    let mut table: BTreeMap<Vec<u8>, (BigUint, BigUint)> = BTreeMap::new();
    for epoch in 0..=cfg.restart_cap {
        if epoch > 0 {
            counts.restarts += 1;
        }
        let mut jumps = vec![];
        for _ in 0..cfg.jumps {
            let a = rng.scalar(n);
            let b = rng.scalar(n);
            let x = g.add(&g.mul(&a, p), &g.mul(&b, q));
            // Scalar-multiplication work is not included in this addition tally;
            // this development counter is explicitly not a complete GAE price.
            counts.setup_additions += 1;
            counts.setup_scalar_multiplications += 2;
            jumps.push((x, a, b));
        }
        let a = rng.scalar(n);
        let b = rng.scalar(n);
        let mut current = normalize(g, n, g.add(&g.mul(&a, p), &g.mul(&b, q)), a, b);
        counts.setup_additions += 1;
        counts.setup_scalar_multiplications += 2;
        counts.canonicalizations += 1;
        let mut history: VecDeque<State<G::Element>> = VecDeque::new();
        let mut restart = false;
        while counts.walk_additions < cfg.step_cap {
            if current.element == g.zero() && !current.b.is_zero() {
                let k = sub(&BigUint::zero(), &current.a, n) * inv(&current.b, n) % n;
                counts.answer_scalar_multiplications += 1;
                if check_answer(g, p, q, &k) {
                    return Outcome {
                        status: "solved".into(),
                        scalar: Some(k.to_string()),
                        counters: counts,
                        scalar_replay_matches: true,
                    };
                }
                return Outcome {
                    status: "invalid_recovery".into(),
                    scalar: None,
                    counters: counts,
                    scalar_replay_matches: false,
                };
            }
            if let Some(pos) = history.iter().position(|s| s.key == current.key) {
                let old = &history[pos];
                if let Some(k) = recover(old, &current, n) {
                    counts.answer_scalar_multiplications += 1;
                    let ok = check_answer(g, p, q, &k);
                    return Outcome {
                        status: if ok { "solved" } else { "invalid_recovery" }.into(),
                        scalar: Some(k.to_string()),
                        counters: counts,
                        scalar_replay_matches: ok,
                    };
                }
                counts.fruitless_collisions += 1;
                counts.short_cycles += 1;
                let cycle: Vec<_> = history.iter().skip(pos).cloned().collect();
                let base = cycle.iter().min_by_key(|s| &s.key).unwrap().clone();
                let keys: Vec<_> = cycle.iter().map(|s| s.key.clone()).collect();
                let mut exit = base.clone();
                let mut escaped = false;
                for _ in 0..64 {
                    exit = normalize(
                        g,
                        n,
                        g.add(&exit.element, &exit.element),
                        2u32 * &exit.a % n,
                        2u32 * &exit.b % n,
                    );
                    counts.escape_doublings += 1;
                    counts.canonicalizations += 1;
                    if !keys.contains(&exit.key) {
                        escaped = true;
                        break;
                    }
                }
                if !escaped {
                    let offset = hash(&base.key) as usize % jumps.len();
                    for i in 0..jumps.len() {
                        let (jump, a, b) = &jumps[(offset + i) % jumps.len()];
                        exit = normalize(
                            g,
                            n,
                            g.add(&base.element, jump),
                            (&base.a + a) % n,
                            (&base.b + b) % n,
                        );
                        counts.escape_jumps += 1;
                        counts.canonicalizations += 1;
                        if !keys.contains(&exit.key) {
                            escaped = true;
                            break;
                        }
                    }
                }
                if !escaped {
                    restart = true;
                    break;
                }
                current = exit;
                history.clear();
                continue;
            }
            if hash(&current.key) & ((1u64 << cfg.dp_bits) - 1) == 0 {
                counts.distinguished_hits += 1;
                if let Some((a, b)) = table.get(&current.key) {
                    let old = State {
                        element: current.element.clone(),
                        key: current.key.clone(),
                        a: a.clone(),
                        b: b.clone(),
                    };
                    if let Some(k) = recover(&old, &current, n) {
                        counts.answer_scalar_multiplications += 1;
                        let ok = check_answer(g, p, q, &k);
                        return Outcome {
                            status: if ok { "solved" } else { "invalid_recovery" }.into(),
                            scalar: Some(k.to_string()),
                            counters: counts,
                            scalar_replay_matches: ok,
                        };
                    }
                    counts.fruitless_collisions += 1;
                } else {
                    table.insert(current.key.clone(), (current.a.clone(), current.b.clone()));
                    counts.max_table_entries = counts.max_table_entries.max(table.len());
                }
            }
            history.push_back(current.clone());
            if history.len() > cfg.history {
                history.pop_front();
            }
            let jump = &jumps[hash(&current.key) as usize % jumps.len()];
            current = advance(g, n, &current, jump);
            counts.walk_additions += 1;
            counts.canonicalizations += 1;
        }
        if !restart {
            break;
        }
    }
    Outcome {
        status: "budget_exhausted".into(),
        scalar: None,
        counters: counts,
        scalar_replay_matches: false,
    }
}

pub fn toy_fixture(prime: u64) -> Option<(Curve, BigUint, Point, u64)> {
    let p = BigUint::from(prime);
    let b = p256_genus_check::parameters().1 % &p;
    let curve = Curve {
        p: p.clone(),
        a: &p - 3u32,
        b,
    };
    let mut size = 1;
    for x in 0..prime {
        let r = curve.rhs(&x.into());
        size += if r.is_zero() {
            1
        } else if r.modpow(&((prime - 1) / 2).into(), &p) == BigUint::one() {
            2
        } else {
            0
        };
    }
    let mut remaining = size;
    let mut factor = 2;
    let mut selected = 0;
    while factor * factor <= remaining {
        if remaining % factor == 0 {
            if factor >= 31 && factor % 3 == 1 {
                selected = factor;
            }
            while remaining % factor == 0 {
                remaining /= factor;
            }
        }
        factor += 1;
    }
    if remaining >= 31 && remaining % 3 == 1 {
        selected = remaining;
    }
    if selected == 0 {
        return None;
    }
    let n = BigUint::from(selected);
    for x in 0..prime {
        let xx = BigUint::from(x);
        let rhs = curve.rhs(&xx);
        for y in 0..prime {
            if BigUint::from(y) * y % &p == rhs {
                let g = curve.mul(&(size / selected).into(), &Some((xx.clone(), y.into())));
                if g.is_some() {
                    assert_eq!(curve.mul(&n, &g), None);
                    return Some((curve, n, g, size));
                }
                break;
            }
        }
    }
    None
}

pub fn correctness_report() -> serde_json::Value {
    use serde_json::json;
    let mut rows = vec![];
    for prime in [1009u64, 4093, 16381] {
        let Some((curve, n, g, size)) = toy_fixture(prime) else {
            rows.push(json!({"p":prime,"status":"no_eligible_subgroup"}));
            continue;
        };
        let source = EllipticGroup {
            curve: curve.clone(),
        };
        let mixed = MixedGroup::new(curve, &n);
        let gg = mixed.image(&g);
        for scalar in [0u64, 1, 2, 17, 37, 43] {
            let k = BigUint::from(scalar) % &n;
            let q = source.mul(&k, &g);
            let qq = mixed.image(&q);
            for seed in [7u64, 19, 20261115] {
                let cfg = Config {
                    seed,
                    ..Config::default()
                };
                let a = solve(&source, &n, &g, &q, &cfg);
                let b = solve(&mixed, &n, &gg, &qq, &cfg);
                assert_eq!(
                    a.scalar,
                    Some(k.to_string()),
                    "source p={prime} k={k} seed={seed}: {a:?}"
                );
                assert_eq!(
                    b.scalar,
                    Some(k.to_string()),
                    "mixed p={prime} k={k} seed={seed}: {b:?}"
                );
                assert!(a.scalar_replay_matches && b.scalar_replay_matches);
                rows.push(json!({"p":prime,"b":source.curve.b.to_string(),"curve_order":size,"subgroup_order":n.to_string(),"public_scalar":k.to_string(),"seed":seed,"source":a,"destination":b}));
            }
        }
    }
    let solves = rows.iter().filter(|r| r.get("source").is_some()).count();
    let applicable: Vec<_> = [1009u64, 4093, 16381]
        .iter()
        .filter(|p| {
            rows.iter()
                .any(|r| r["p"] == **p && r.get("source").is_some())
        })
        .copied()
        .collect();
    json!({"schema":"p256-folded-walk-correctness/v1","rows":rows,"complete_paired_toy_cases":solves,
        "applicable_fields":applicable,"standing":"producer toy correctness checks, not timed performance or signed verification",
        "counters_are_complete_cost":false,"complete_cost_measured":false})
}

/// Actual-P256 transition correctness: no full-order logarithm or timing.
pub fn full_field_trace() -> serde_json::Value {
    use serde_json::json;
    let (p, b, n, g) = p256_genus_check::parameters();
    let curve = Curve {
        p: p.clone(),
        a: &p - 3u32,
        b,
    };
    let mixed = MixedGroup::new(curve.clone(), &n);
    let fast = FastP256;
    let fg = p256::ProjectivePoint::GENERATOR;
    let fq = fast.mul(&37u32.into(), &fg);
    let q = FastP256::to_point(&fq);
    let mut rng = Rng(2026111503);
    let mut source_jumps = vec![];
    let mut mixed_jumps = vec![];
    for _ in 0..32 {
        let a = rng.scalar(&n);
        let b = rng.scalar(&n);
        let v = fast.add(&fast.mul(&a, &fg), &fast.mul(&b, &fq));
        mixed_jumps.push((mixed.image(&FastP256::to_point(&v)), a.clone(), b.clone()));
        source_jumps.push((v, a, b));
    }
    let a = rng.scalar(&n);
    let bb = rng.scalar(&n);
    let point = fast.add(&fast.mul(&a, &fg), &fast.mul(&bb, &fq));
    let mut source = normalize(&fast, &n, point, a.clone(), bb.clone());
    let mut image = normalize(&mixed, &n, mixed.image(&FastP256::to_point(&point)), a, bb);
    let mut digest_data = vec![];
    for _ in 0..32 {
        source = advance(
            &fast,
            &n,
            &source,
            &source_jumps[hash(&source.key) as usize % 32],
        );
        image = advance(
            &mixed,
            &n,
            &image,
            &mixed_jumps[hash(&image.key) as usize % 32],
        );
        assert_eq!(
            source.element,
            fast.add(&fast.mul(&source.a, &fg), &fast.mul(&source.b, &fq))
        );
        let preimage = curve.add(&curve.mul(&image.a, &g), &curve.mul(&image.b, &q));
        assert_eq!(image.element, mixed.image(&preimage));
        assert!(mixed.valid(&image.element));
        assert_eq!(
            mixed.canonical(&mixed.rotate(&image.element), &n).1,
            image.key
        );
        digest_data.extend(&source.key);
        digest_data.extend(&image.key);
    }
    json!({"schema":"p256-folded-transition-trace/v1","source_backend":"RustCrypto p256 0.13.2 projective arithmetic",
        "subgroup_order":n.to_string(),"jump_count":32,"transitions_each":32,
        "trace_sha256":sha256(&digest_data).iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "source_final_coefficients":[source.a.to_string(),source.b.to_string()],
        "destination_final_coefficients":[image.a.to_string(),image.b.to_string()],
        "standing":"producer transition invariants only; no P-256 DLP recovery and no timing", "complete_cost_measured":false})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn toy_solves() {
        let r = correctness_report();
        assert_eq!(r["complete_paired_toy_cases"], 54);
        // The frozen public panel must actually exercise the escape path.
        assert!(r["rows"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["destination"]["counters"]["escape_doublings"]
                .as_u64()
                .unwrap_or(0)
                > 0));
    }
    #[test]
    fn coefficient_and_canonical_controls() {
        let (c, n, g, _) = toy_fixture(1009).unwrap();
        let mixed = MixedGroup::new(c.clone(), &n);
        let image = mixed.image(&g);
        let (normal, key, m) = mixed.canonical(&image, &n);
        assert_eq!(normal, mixed.mul(&m, &image));
        let rotated = mixed.rotate(&image);
        assert_eq!(mixed.canonical(&rotated, &n).1, key);
        assert_ne!(normal, mixed.mul(&((&m + 1u32) % &n), &image));
        let x = normalize(&mixed, &n, image.clone(), 1u32.into(), 0u32.into());
        assert!(recover(&x, &x, &n).is_none());
        let q = mixed.mul(&37u32.into(), &image);
        let cfg = Config {
            step_cap: 0,
            ..Config::default()
        };
        assert_eq!(
            solve(&mixed, &n, &image, &q, &cfg).status,
            "budget_exhausted"
        );
        let bad = Some((c.p.clone(), BigUint::one()));
        let source = EllipticGroup { curve: c };
        assert_eq!(
            solve(&source, &n, &g, &bad, &Config::default()).status,
            "invalid_input"
        );
    }
    #[test]
    fn external_source_control() {
        let (p, b, n, g) = p256_genus_check::parameters();
        let fast = FastP256;
        let fg = p256::ProjectivePoint::GENERATOR;
        assert_eq!(FastP256::to_point(&fg), g);
        let c = Curve {
            p: p.clone(),
            a: &p - 3u32,
            b,
        };
        for k in [1u32, 2, 37, 109] {
            let q = fast.mul(&k.into(), &fg);
            assert_eq!(FastP256::to_point(&q), c.mul(&k.into(), &g));
            let (canon, key, m) = fast.canonical(&q, &n);
            assert_eq!(canon, fast.mul(&m, &q));
            assert_eq!(fast.canonical(&fast.neg(&q), &n).1, key);
        }
    }
    #[test]
    fn full_field_coefficient_trace() {
        let r = full_field_trace();
        assert_eq!(r["transitions_each"], 32);
    }
}
