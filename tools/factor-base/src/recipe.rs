use crate::arithmetic::*;
use anyhow::{bail, ensure, Result};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Shape {
    Coordinate,
    Geometric,
    RandomLinear,
    TraceZero,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Recipe {
    BinarySubspace {
        shape: Shape,
        dimension: usize,
        seed: u64,
        offsets: Vec<String>,
    },
    BinaryExplicit {
        basis: Vec<String>,
        offsets: Vec<String>,
    },
    FrobeniusOrbits {
        seeds: usize,
        seed: u64,
    },
    PrimeProgression {
        start: String,
        step: String,
        count: usize,
    },
    /// A coset of a supplied, exact-order subgroup of F_p^* (PKM section 3.3).
    PrimeMultiplicativeCoset {
        order: usize,
        generator: String,
        coset: String,
    },
    /// Traces of a subgroup in the norm-one torus of F_p[s]/(s^2-nonsquare).
    PrimeNormOneTrace {
        order: usize,
        generator_real: String,
        generator_imag: String,
        nonsquare: String,
        scale: String,
        shift: String,
    },
    /// Image of an explicit progression under D_degree(x, parameter).
    /// An image recipe does not assert a cheap membership or PDP solver.
    PrimeDicksonImage {
        degree: usize,
        parameter: String,
        start: String,
        step: String,
        count: usize,
    },
    RandomX {
        count: usize,
        seed: u64,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub curve: CurveSpec,
    pub recipe: Recipe,
    pub max_candidates: usize,
    pub shard_points: usize,
}
pub struct Generated {
    pub curve: Curve,
    pub recipe: Recipe,
    pub points: Vec<Point>,
    pub basis: Option<Vec<N>>,
    pub structure: Value,
}
pub struct Stream {
    domain: Vec<u8>,
    counter: u64,
}
impl Stream {
    pub fn new(domain: &str, seed: u64) -> Self {
        Self {
            domain: format!("factor-base/v1/{domain}/{seed}").into_bytes(),
            counter: 0,
        }
    }
    pub fn below(&mut self, bound: &N) -> N {
        let bytes = (bound.bits() as usize).div_ceil(8);
        loop {
            let mut data = Vec::new();
            while data.len() < bytes {
                let mut h = Sha256::new();
                h.update(&self.domain);
                h.update(self.counter.to_be_bytes());
                data.extend_from_slice(&h.finalize());
                self.counter += 1;
            }
            data.truncate(bytes);
            let excess = bytes * 8 - bound.bits() as usize;
            if excess > 0 {
                *data.last_mut().unwrap() &= 255u8 >> excess;
            }
            let n = N::from_bytes_le(&data);
            if &n < bound {
                return n;
            }
        }
    }
}
fn canonical_recipe(recipe: &Recipe) -> Result<Recipe> {
    let mut r = recipe.clone();
    match &mut r {
        Recipe::BinarySubspace { offsets, .. } | Recipe::BinaryExplicit { offsets, .. } => {
            for x in offsets {
                *x = enc(&parse(x)?);
            }
        }
        Recipe::PrimeProgression { start, step, .. } => {
            *start = enc(&parse(start)?);
            *step = enc(&parse(step)?);
        }
        Recipe::PrimeMultiplicativeCoset {
            generator, coset, ..
        } => {
            *generator = enc(&parse(generator)?);
            *coset = enc(&parse(coset)?);
        }
        Recipe::PrimeNormOneTrace {
            generator_real,
            generator_imag,
            nonsquare,
            scale,
            shift,
            ..
        } => {
            for value in [generator_real, generator_imag, nonsquare, scale, shift] {
                *value = enc(&parse(value)?);
            }
        }
        Recipe::PrimeDicksonImage {
            parameter,
            start,
            step,
            ..
        } => {
            for value in [parameter, start, step] {
                *value = enc(&parse(value)?);
            }
        }
        _ => {}
    }
    if let Recipe::BinaryExplicit { basis, .. } = &mut r {
        for x in basis {
            *x = enc(&parse(x)?);
        }
    }
    Ok(r)
}
pub fn offsets(recipe: &Recipe) -> Option<&[String]> {
    match recipe {
        Recipe::BinarySubspace { offsets, .. } | Recipe::BinaryExplicit { offsets, .. } => {
            Some(offsets)
        }
        _ => None,
    }
}

// Exact-order tests need only the distinct prime divisors of the bounded order.
fn order_primes(mut order: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut q = 2;
    while q <= order / q {
        if order.is_multiple_of(q) {
            primes.push(q);
            while order.is_multiple_of(q) {
                order /= q;
            }
        }
        q += 1;
    }
    if order > 1 {
        primes.push(order);
    }
    primes
}

type Quadratic = (N, N);
fn torus_mul(f: &Field, nu: &N, a: &Quadratic, b: &Quadratic) -> Quadratic {
    (
        f.add(&f.mul(&a.0, &b.0), &f.mul(nu, &f.mul(&a.1, &b.1))),
        f.add(&f.mul(&a.0, &b.1), &f.mul(&a.1, &b.0)),
    )
}
fn torus_pow(f: &Field, nu: &N, base: &Quadratic, mut exponent: usize) -> Quadratic {
    let mut result = (N::one(), N::zero());
    let mut power = base.clone();
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = torus_mul(f, nu, &result, &power);
        }
        power = torus_mul(f, nu, &power, &power);
        exponent >>= 1;
    }
    result
}
fn dickson(f: &Field, x: &N, a: &N, degree: usize) -> N {
    let (mut previous, mut value) = (n(2), x.clone());
    for _ in 1..degree {
        let next = f.sub(&f.mul(x, &value), &f.mul(a, &previous));
        previous = value;
        value = next;
    }
    value
}

pub fn generate(config: &Config) -> Result<Generated> {
    ensure!(
        (1..=1_000_000).contains(&config.max_candidates),
        "max_candidates must be in 1..=1000000"
    );
    ensure!(
        (1..=65536).contains(&config.shard_points),
        "shard_points must be in 1..=65536"
    );
    let curve = Curve::new(config.curve.clone())?;
    let recipe = canonical_recipe(&config.recipe)?;
    let f = &curve.f;
    let cap = config.max_candidates;
    let mut basis = None;
    let mut xs = BTreeSet::new();
    let mut orbit_points = BTreeSet::new();
    match &recipe {
        Recipe::BinarySubspace {
            shape,
            dimension,
            seed,
            offsets,
        } => {
            let degree = f
                .degree()
                .ok_or_else(|| anyhow::anyhow!("binary recipe requires a binary field"))?;
            ensure!(
                *dimension > 0 && *dimension <= degree && *dimension < usize::BITS as usize,
                "invalid dimension"
            );
            ensure!(
                (1usize << dimension)
                    .checked_mul(offsets.len())
                    .is_some_and(|x| x <= cap)
                    && !offsets.is_empty(),
                "subspace exceeds candidate cap"
            );
            ensure!(
                !matches!(shape, Shape::TraceZero) || *dimension < degree,
                "trace-zero dimension exceeds n-1"
            );
            let mut stream = Stream::new("basis", *seed);
            let mut rows = Vec::new();
            let mut attempts = 0;
            let g = loop {
                let g = stream.below(&f.size());
                if g > n(1) {
                    break g;
                }
            };
            let mut power = N::one();
            while rows.len() < *dimension {
                attempts += 1;
                ensure!(
                    attempts < 100000,
                    "basis construction exhausted attempt cap"
                );
                let x = match shape {
                    Shape::Coordinate => N::one() << rows.len(),
                    Shape::Geometric => {
                        let x = power.clone();
                        power = f.mul(&power, &g);
                        x
                    }
                    _ => stream.below(&f.size()),
                };
                if matches!(shape, Shape::TraceZero) && !f.trace(&x).is_zero() {
                    continue;
                }
                let mut next = rows.clone();
                next.push(x);
                if rank(&next) > rows.len() {
                    rows = next;
                }
            }
            expand(&rows, offsets, &curve, &mut xs)?;
            basis = Some(rows);
        }
        Recipe::BinaryExplicit {
            basis: strings,
            offsets,
        } => {
            let degree = f
                .degree()
                .ok_or_else(|| anyhow::anyhow!("binary recipe requires a binary field"))?;
            ensure!(
                !strings.is_empty()
                    && strings.len() <= degree
                    && strings.len() < usize::BITS as usize,
                "invalid basis length"
            );
            ensure!(
                (1usize << strings.len())
                    .checked_mul(offsets.len())
                    .is_some_and(|x| x <= cap)
                    && !offsets.is_empty(),
                "subspace exceeds candidate cap"
            );
            let rows = strings
                .iter()
                .map(|s| parse(s))
                .collect::<Result<Vec<_>>>()?;
            ensure!(
                rows.iter().all(|x| x < &f.size()) && rank(&rows) == rows.len(),
                "basis must be canonical and independent"
            );
            expand(&rows, offsets, &curve, &mut xs)?;
            basis = Some(rows);
        }
        Recipe::PrimeProgression { start, step, count } => {
            ensure!(
                f.degree().is_none(),
                "prime progression requires a prime field"
            );
            let start = parse(start)?;
            let step = parse(step)?;
            ensure!(
                start < f.size() && !step.is_zero() && step < f.size(),
                "invalid progression start/step"
            );
            ensure!(
                *count > 0 && *count <= cap && n(*count as u64) <= f.size(),
                "progression exceeds cap or field"
            );
            let mut x = start;
            for _ in 0..*count {
                xs.insert(x.clone());
                x = f.add(&x, &step);
            }
        }
        Recipe::PrimeMultiplicativeCoset {
            order,
            generator,
            coset,
        } => {
            ensure!(
                f.degree().is_none(),
                "multiplicative coset requires a prime field"
            );
            ensure!(
                *order > 0 && *order <= cap,
                "subgroup order exceeds candidate cap"
            );
            let group_order = f.size() - N::one();
            ensure!(
                &group_order % n(*order as u64) == N::zero(),
                "order must divide p-1"
            );
            let generator = parse(generator)?;
            let coset = parse(coset)?;
            ensure!(
                !generator.is_zero()
                    && generator < f.size()
                    && !coset.is_zero()
                    && coset < f.size(),
                "noncanonical or zero coset parameters"
            );
            ensure!(
                generator.modpow(&n(*order as u64), &f.size()) == N::one()
                    && order_primes(*order)
                        .iter()
                        .all(|q| generator.modpow(&n((order / q) as u64), &f.size()) != N::one()),
                "generator does not have the declared exact order"
            );
            let mut x = coset;
            for _ in 0..*order {
                xs.insert(x.clone());
                x = f.mul(&x, &generator);
            }
        }
        Recipe::PrimeNormOneTrace {
            order,
            generator_real,
            generator_imag,
            nonsquare,
            scale,
            shift,
        } => {
            ensure!(
                f.degree().is_none(),
                "norm-one torus requires a prime field"
            );
            ensure!(
                *order > 0 && *order <= cap,
                "torus order exceeds candidate cap"
            );
            ensure!(
                (f.size() + N::one()) % n(*order as u64) == N::zero(),
                "torus order must divide p+1"
            );
            let nu = parse(nonsquare)?;
            let scale = parse(scale)?;
            let shift = parse(shift)?;
            let g = (parse(generator_real)?, parse(generator_imag)?);
            ensure!(
                nu < f.size()
                    && !nu.is_zero()
                    && scale < f.size()
                    && !scale.is_zero()
                    && shift < f.size()
                    && g.0 < f.size()
                    && g.1 < f.size(),
                "noncanonical torus parameters or zero scale"
            );
            ensure!(
                nu.modpow(&((f.size() - N::one()) >> 1), &f.size()) == f.size() - N::one(),
                "quadratic parameter must be a nonsquare"
            );
            ensure!(
                f.sub(&f.sq(&g.0), &f.mul(&nu, &f.sq(&g.1))) == N::one(),
                "generator is not norm one"
            );
            let identity = (N::one(), N::zero());
            ensure!(
                torus_pow(f, &nu, &g, *order) == identity
                    && order_primes(*order)
                        .iter()
                        .all(|q| torus_pow(f, &nu, &g, order / q) != identity),
                "torus generator does not have the declared exact order"
            );
            let mut point = identity;
            for _ in 0..*order {
                // Tr(z)=z+z^p=2*Re(z). Inverse elements have equal traces.
                xs.insert(f.add(&f.mul(&scale, &f.add(&point.0, &point.0)), &shift));
                point = torus_mul(f, &nu, &point, &g);
            }
        }
        Recipe::PrimeDicksonImage {
            degree,
            parameter,
            start,
            step,
            count,
        } => {
            ensure!(f.degree().is_none(), "Dickson image requires a prime field");
            ensure!(
                *degree > 0
                    && *count > 0
                    && degree.checked_mul(*count).is_some_and(|work| work <= cap)
                    && n(*count as u64) <= f.size(),
                "Dickson degree times count exceeds cap or field"
            );
            let a = parse(parameter)?;
            let mut x = parse(start)?;
            let step = parse(step)?;
            ensure!(
                a < f.size() && x < f.size() && !step.is_zero() && step < f.size(),
                "noncanonical Dickson parameters or zero step"
            );
            for _ in 0..*count {
                xs.insert(dickson(f, &x, &a, *degree));
                x = f.add(&x, &step);
            }
        }
        Recipe::RandomX { count, seed } => {
            ensure!(
                *count > 0 && *count <= cap && n(*count as u64) <= f.size(),
                "random subset exceeds cap or field"
            );
            let mut stream = Stream::new("random-x", *seed);
            for _ in 0..cap * 64 {
                if xs.len() == *count {
                    break;
                }
                xs.insert(stream.below(&f.size()));
            }
            ensure!(xs.len() == *count, "random subset attempt cap");
        }
        Recipe::FrobeniusOrbits { seeds, seed } => {
            ensure!(
                curve.is_koblitz(),
                "Frobenius orbit construction requires coefficients in F2"
            );
            let degree = f.degree().unwrap();
            ensure!(
                *seeds > 0 && seeds.checked_mul(degree).is_some_and(|x| x <= cap),
                "orbit recipe exceeds cap"
            );
            let mut stream = Stream::new("frobenius-seeds", *seed);
            let mut taken = 0;
            for _ in 0..cap * 64 {
                if taken == *seeds {
                    break;
                }
                let x = stream.below(&f.size());
                let Some(p) = curve.lift(&x).into_iter().find(|p| curve.subgroup(p)) else {
                    continue;
                };
                if orbit_points.contains(&p) {
                    continue;
                }
                taken += 1;
                let mut p = p;
                for _ in 0..degree {
                    xs.insert(p.x.clone());
                    orbit_points.insert(p.clone());
                    orbit_points.insert(curve.neg(&p));
                    p = Point {
                        x: f.sq(&p.x),
                        y: f.sq(&p.y),
                    };
                }
            }
            ensure!(
                taken == *seeds,
                "orbit seed attempt cap; requested distinct orbits unavailable within budget"
            );
        }
    }
    let mut points = BTreeSet::new();
    let mut rational = 0;
    for x in &xs {
        let ps = curve.lift(x);
        rational += ps.len();
        for p in ps {
            ensure!(curve.on_curve(&p), "lifting failed curve check");
            if curve.subgroup(&p) {
                points.insert(p);
            }
        }
    }
    ensure!(!points.is_empty(), "empty subgroup factor base");
    let points = points.into_iter().collect::<Vec<_>>();
    let frobenius_closed = if curve.is_koblitz() {
        let set = points.iter().cloned().collect::<BTreeSet<_>>();
        Some(points.iter().all(|p| {
            set.contains(&Point {
                x: f.sq(&p.x),
                y: f.sq(&p.y),
            })
        }))
    } else {
        None
    };
    let structure = if let Some(v) = &basis {
        let vv = v
            .iter()
            .flat_map(|a| v.iter().map(move |b| f.mul(a, b)))
            .collect::<Vec<_>>();
        let frob = v.iter().map(|a| f.sq(a)).collect::<Vec<_>>();
        let joined = [v.clone(), frob].concat();
        let vvv = if v.len() <= 16 {
            Some(rank(
                &vv.iter()
                    .flat_map(|a| v.iter().map(move |b| f.mul(a, b)))
                    .collect::<Vec<_>>(),
            ))
        } else {
            None
        };
        json!({"dimension":v.len(),"basis":v.iter().map(enc).collect::<Vec<_>>(),"dim_vv":rank(&vv),"dim_vvv":vvv,"dim_v_intersect_frobenius_v":2*v.len()-rank(&joined),"basis_trace_zero":v.iter().all(|x|f.trace(x).is_zero()),"interpretation":"structural_proxies_not_measured_regularity"})
    } else {
        json!({"dimension":null,"interpretation":"point_set_without_linear_subspace_encoding"})
    };
    let structure = json!({"linear":structure,"x_candidates":xs.len(),"rational_points_before_subgroup_filter":rational,"subgroup_points":points.len(),"frobenius_closed":frobenius_closed,"signed_points":"both_signs_materialized","known_logs_generated":false});
    Ok(Generated {
        curve,
        recipe,
        points,
        basis,
        structure,
    })
}
fn expand(v: &[N], offsets: &[String], curve: &Curve, out: &mut BTreeSet<N>) -> Result<()> {
    for offset in offsets {
        let offset = parse(offset)?;
        ensure!(offset < curve.f.size(), "offset outside field");
        for mask in 0..1usize << v.len() {
            let mut x = offset.clone();
            for (i, b) in v.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    x ^= b;
                }
            }
            out.insert(x);
        }
    }
    Ok(())
}
pub fn sample_targets(curve: &Curve, count: usize, seed: u64, split: &str) -> Result<Vec<Point>> {
    ensure!((1..=1024).contains(&count), "target count must be 1..=1024");
    ensure!(
        split == "discovery" || split == "holdout",
        "split must be discovery or holdout"
    );
    let mut stream = Stream::new(&format!("query/{split}"), seed);
    let mut result = Vec::new();
    for _ in 0..count * 10000 {
        if result.len() == count {
            break;
        }
        let x = stream.below(&curve.f.size());
        let ps = curve.lift(&x);
        let sign = stream.below(&n(2));
        // Choosing either root slot before subgroup filtering keeps every finite point equally likely.
        if let Some(p) = ps.get(if sign.is_zero() { 0 } else { 1 }) {
            if curve.subgroup(p) {
                result.push(p.clone());
            }
        }
    }
    if result.len() != count {
        bail!("query sampling cap reached; large cofactor requires supplied public targets");
    }
    Ok(result)
}
