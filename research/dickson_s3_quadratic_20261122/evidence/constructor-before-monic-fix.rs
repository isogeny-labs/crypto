//! Matched Dickson-selector systems with quartic and quadratic S3 blocks.
//!
//! This is an export and correctness harness, not a comparison experiment.
//! `--check` includes one bounded, complete, tiny-field solve_visit control.
//! `--export --m 4 --depth 3` emits all three arms on a public planted fixture.
//! No Prime31 solve, sweep, elapsed time, or speed claim is produced here.
//!
//! Every arm uses D_(2^depth)(x, 1) = 0 and the same finite intermediate-x
//! chain. The input-degree reduction does not imply a solving-degree reduction.
//! Invariant Q preserves the quartic zero set over the coefficient field;
//! line L additionally requires an F_p-rational line and can exclude twist-only
//! triples. All accepted relations still require signed, exact point replay.

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly, VisitReport};
use crypto_lib::cryptanalysis::fp_backend::{Fp32, PrimeField};
use crypto_lib::cryptanalysis::ic_boundary::{
    lift_abscissae, CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::dickson_abscissae;
use crypto_lib::cryptanalysis::symmetrized_semaev::eval_s3_u64;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

const M31: u64 = 2_147_483_647;
const TINY_MAX_DEGREE: u32 = 8;
const TINY_DEADLINE_SECONDS: u64 = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    Quartic,
    Invariant,
    Line,
}

impl Arm {
    const ALL: [Self; 3] = [Self::Quartic, Self::Invariant, Self::Line];

    fn name(self) -> &'static str {
        match self {
            Self::Quartic => "quartic_s3",
            Self::Invariant => "invariant_q",
            Self::Line => "line_l",
        }
    }

    fn degree_bound(self) -> u32 {
        if self == Self::Quartic {
            4
        } else {
            2
        }
    }
}

/// Thin canonical-u64 adapters around the existing production Fp32 backend.
/// No alternative modular arithmetic implementation is used by this example.
struct Field(Fp32);

impl Field {
    fn new(p: u64) -> Self {
        Self(Fp32::new_scalar(p))
    }
    fn canonical(&self, x: u64) -> u64 {
        self.0.to_u64(self.0.from_u64(x)).unwrap()
    }
    fn add(&self, x: u64, y: u64) -> u64 {
        self.0
            .to_u64(self.0.add(self.0.from_u64(x), self.0.from_u64(y)))
            .unwrap()
    }
    fn sub(&self, x: u64, y: u64) -> u64 {
        self.0
            .to_u64(self.0.sub(self.0.from_u64(x), self.0.from_u64(y)))
            .unwrap()
    }
    fn mul(&self, x: u64, y: u64) -> u64 {
        self.0
            .to_u64(self.0.mul(self.0.from_u64(x), self.0.from_u64(y)))
            .unwrap()
    }
    fn pow(&self, x: u64, e: u64) -> u64 {
        self.0
            .to_u64(self.0.pow_u64(self.0.from_u64(x), e))
            .unwrap()
    }
    fn inv(&self, x: u64) -> Option<u64> {
        self.0
            .inv(self.0.from_u64(x))
            .map(|v| self.0.to_u64(v).unwrap())
    }
}

struct Polys {
    field: Field,
    p: u64,
    n: usize,
}

impl Polys {
    fn new(p: u64, n: usize) -> Self {
        Self {
            field: Field::new(p),
            p,
            n,
        }
    }
    fn constant(&self, c: u64) -> Poly {
        let c = self.field.canonical(c);
        if c == 0 {
            vec![]
        } else {
            vec![(vec![0; self.n], c)]
        }
    }
    fn variable(&self, i: usize) -> Poly {
        let mut e = vec![0; self.n];
        e[i] = 1;
        vec![(e, 1)]
    }
    fn add(&self, a: &Poly, b: &Poly) -> Poly {
        let mut terms = a.clone();
        terms.extend_from_slice(b);
        f4_fp::normalise(&terms, self.p, Ordering::Grevlex)
    }
    fn scale(&self, a: &Poly, c: u64) -> Poly {
        let terms: Poly = a
            .iter()
            .map(|(e, v)| (e.clone(), self.field.mul(*v, c)))
            .collect();
        f4_fp::normalise(&terms, self.p, Ordering::Grevlex)
    }
    fn sub(&self, a: &Poly, b: &Poly) -> Poly {
        self.add(a, &self.scale(b, self.field.sub(0, 1)))
    }
    fn mul(&self, a: &Poly, b: &Poly) -> Poly {
        let mut terms = Vec::with_capacity(a.len() * b.len());
        for (ea, ca) in a {
            for (eb, cb) in b {
                let e = ea.iter().zip(eb).map(|(u, v)| u + v).collect();
                terms.push((e, self.field.mul(*ca, *cb)));
            }
        }
        f4_fp::normalise(&terms, self.p, Ordering::Grevlex)
    }
    fn square(&self, a: &Poly) -> Poly {
        self.mul(a, a)
    }
}

#[derive(Clone, Copy)]
enum Coordinate {
    Variable(usize),
    Constant(u64),
}

impl Coordinate {
    fn polynomial(self, ps: &Polys) -> Poly {
        match self {
            Self::Variable(i) => ps.variable(i),
            Self::Constant(c) => ps.constant(c),
        }
    }
    fn evaluate(self, assignment: &[u64]) -> u64 {
        match self {
            Self::Variable(i) => assignment[i],
            Self::Constant(c) => c,
        }
    }
}

struct Block {
    coordinates: [Coordinate; 3],
    auxiliaries: Vec<usize>,
}

struct System {
    curve: PrimeCurve,
    target: PrimePoint,
    arm: Arm,
    m: usize,
    depth: u32,
    names: Vec<String>,
    ladders: Vec<Vec<usize>>,
    intermediates: Vec<usize>,
    blocks: Vec<Block>,
    labels: Vec<String>,
    equations: Vec<Poly>,
}

fn total_degree(poly: &Poly) -> u32 {
    poly.iter().map(|(e, _)| e.iter().sum()).max().unwrap_or(0)
}

fn evaluate(poly: &Poly, values: &[u64], field: &Field) -> u64 {
    poly.iter().fold(0, |sum, (exponents, coefficient)| {
        let term = exponents
            .iter()
            .zip(values)
            .fold(*coefficient, |a, (e, x)| {
                field.mul(a, field.pow(*x, u64::from(*e)))
            });
        field.add(sum, term)
    })
}

/// The quartic arm is built from the ordinary S3 formula, independently of
/// the elementary-symmetric-variable form used in the quadratic Q arm.
fn block_equations(ps: &Polys, curve: &PrimeCurve, arm: Arm, block: &Block) -> Vec<Poly> {
    let [x, y, z] = block.coordinates.map(|c| c.polynomial(ps));
    let xy = ps.mul(&x, &y);
    let sum_xy = ps.add(&x, &y);
    let a = ps.constant(curve.a);
    let b = ps.constant(curve.b);
    match arm {
        Arm::Quartic => {
            let first = ps.mul(&ps.square(&ps.sub(&x, &y)), &ps.square(&z));
            let middle = ps.scale(
                &ps.mul(
                    &ps.add(&ps.mul(&sum_xy, &ps.add(&xy, &a)), &ps.scale(&b, 2)),
                    &z,
                ),
                2,
            );
            let last = ps.sub(
                &ps.square(&ps.sub(&xy, &a)),
                &ps.scale(&sum_xy, ps.field.mul(4, curve.b)),
            );
            vec![ps.add(&ps.sub(&first, &middle), &last)]
        }
        Arm::Invariant => {
            let r = ps.variable(block.auxiliaries[0]);
            let e2 = ps.variable(block.auxiliaries[1]);
            let e3 = ps.variable(block.auxiliaries[2]);
            vec![
                ps.sub(&r, &xy),
                ps.sub(&ps.sub(&e2, &r), &ps.mul(&z, &sum_xy)),
                ps.sub(&e3, &ps.mul(&r, &z)),
                ps.sub(
                    &ps.square(&ps.sub(&e2, &a)),
                    &ps.scale(&ps.mul(&ps.add(&sum_xy, &z), &ps.add(&e3, &b)), 4),
                ),
            ]
        }
        Arm::Line => {
            let r = ps.variable(block.auxiliaries[0]);
            let lambda = ps.variable(block.auxiliaries[1]);
            let nu = ps.variable(block.auxiliaries[2]);
            vec![
                ps.sub(&r, &xy),
                ps.sub(&ps.square(&lambda), &ps.add(&sum_xy, &z)),
                ps.add(
                    &ps.sub(&ps.add(&r, &ps.mul(&z, &sum_xy)), &a),
                    &ps.scale(&ps.mul(&lambda, &nu), 2),
                ),
                ps.sub(&ps.add(&ps.mul(&r, &z), &b), &ps.square(&nu)),
            ]
        }
    }
}

fn append_variable(names: &mut Vec<String>, name: String) -> usize {
    let i = names.len();
    names.push(name);
    i
}

impl System {
    fn new(
        curve: PrimeCurve,
        target: PrimePoint,
        arm: Arm,
        m: usize,
        depth: u32,
    ) -> Result<Self, String> {
        if !(2..=4).contains(&m) || !(1..=29).contains(&depth) {
            return Err("supported m is 2..=4 and zero-selector depth is 1..=29".into());
        }
        if curve.p <= 3 || curve.p >= (1u64 << 32) || curve.a >= curve.p || curve.b >= curve.p {
            return Err("the production Fp32 path requires canonical coefficients and a known prime 3 < p < 2^32".into());
        }
        if target.infinity
            || target.x >= curve.p
            || target.y >= curve.p
            || !curve.is_on_curve(target)
        {
            return Err("target must be a canonical finite point on the curve".into());
        }
        let f = Field::new(curve.p);
        if f.add(f.mul(4, f.pow(curve.a, 3)), f.mul(27, f.pow(curve.b, 2))) == 0 {
            return Err("singular curve".into());
        }
        let mut names = Vec::new();
        let mut ladders = Vec::new();
        for i in 0..m {
            let mut ladder = Vec::new();
            for j in 0..depth {
                let name = if j == 0 {
                    format!("x_{i}")
                } else {
                    format!("d_{i}_{j}")
                };
                ladder.push(append_variable(&mut names, name));
            }
            ladders.push(ladder);
        }
        let intermediates: Vec<usize> = (0..m - 2)
            .map(|j| append_variable(&mut names, format!("t_{j}")))
            .collect();
        let mut blocks = Vec::new();
        for j in 0..m - 1 {
            let left = if j == 0 {
                ladders[0][0]
            } else {
                intermediates[j - 1]
            };
            let third = if j == m - 2 {
                Coordinate::Constant(target.x)
            } else {
                Coordinate::Variable(intermediates[j])
            };
            let auxiliaries = match arm {
                Arm::Quartic => Vec::new(),
                Arm::Invariant => ["r", "e2", "e3"]
                    .iter()
                    .map(|s| append_variable(&mut names, format!("{s}_{j}")))
                    .collect(),
                Arm::Line => ["r", "lambda", "nu"]
                    .iter()
                    .map(|s| append_variable(&mut names, format!("{s}_{j}")))
                    .collect(),
            };
            blocks.push(Block {
                coordinates: [
                    Coordinate::Variable(left),
                    Coordinate::Variable(ladders[j + 1][0]),
                    third,
                ],
                auxiliaries,
            });
        }
        let ps = Polys::new(curve.p, names.len());
        let mut labels = Vec::new();
        let mut equations = Vec::new();
        for (i, ladder) in ladders.iter().enumerate() {
            for (j, index) in ladder.iter().enumerate() {
                let rhs = if j + 1 == ladder.len() {
                    ps.constant(0)
                } else {
                    ps.variable(ladder[j + 1])
                };
                equations.push(ps.sub(
                    &ps.sub(&ps.square(&ps.variable(*index)), &rhs),
                    &ps.constant(2),
                ));
                labels.push(format!("selector_{i}_{j}"));
            }
        }
        for (j, block) in blocks.iter().enumerate() {
            for (k, equation) in block_equations(&ps, &curve, arm, block)
                .into_iter()
                .enumerate()
            {
                labels.push(format!("{}_block_{j}_{k}", arm.name()));
                equations.push(equation);
            }
        }
        if equations
            .iter()
            .any(|p| total_degree(p) > arm.degree_bound())
        {
            return Err("input polynomial exceeded its advertised total-degree bound".into());
        }
        Ok(Self {
            curve,
            target,
            arm,
            m,
            depth,
            names,
            ladders,
            intermediates,
            blocks,
            labels,
            equations,
        })
    }

    /// Every variable has the full coefficient-field domain. In particular,
    /// intermediate x, line, invariant, and ladder variables are never given
    /// a factor-base domain. Only the leaf ladder equations select the base.
    fn domains(&self) -> Vec<Option<Vec<u64>>> {
        vec![None; self.names.len()]
    }

    fn check_assignment(&self, values: &[u64]) -> Result<(), String> {
        if values.len() != self.names.len() || values.iter().any(|v| *v >= self.curve.p) {
            return Err("assignment length or canonical field range is invalid".into());
        }
        let f = Field::new(self.curve.p);
        for (label, poly) in self.labels.iter().zip(&self.equations) {
            if evaluate(poly, values, &f) != 0 {
                return Err(format!("nonzero equation: {label}"));
            }
        }
        Ok(())
    }

    fn leaf_xs(&self, values: &[u64]) -> Vec<u64> {
        self.ladders
            .iter()
            .map(|ladder| values[ladder[0]])
            .collect()
    }

    fn assignment_for_points(&self, points: &[PrimePoint]) -> Result<Vec<u64>, String> {
        if points.len() != self.m
            || points.iter().any(|p| {
                p.infinity
                    || p.x >= self.curve.p
                    || p.y >= self.curve.p
                    || !self.curve.is_on_curve(*p)
            })
        {
            return Err(
                "the assignment constructor requires m canonical finite curve points".into(),
            );
        }
        let f = Field::new(self.curve.p);
        let mut values = vec![0; self.names.len()];
        for (point, ladder) in points.iter().zip(&self.ladders) {
            let mut d = point.x;
            for index in ladder {
                values[*index] = d;
                d = f.sub(f.mul(d, d), 2);
            }
            if d != 0 {
                return Err("a leaf is outside the zero selector".into());
            }
        }
        let mut ops = GroupOps::default();
        let mut prefix = points[0];
        for j in 0..self.m - 1 {
            prefix = self.curve.add(&mut ops, prefix, points[j + 1]);
            if prefix.infinity {
                return Err(format!("finite chain chart excludes infinity at block {j}"));
            }
            if j < self.intermediates.len() {
                values[self.intermediates[j]] = prefix.x;
            }
        }
        if prefix != self.target {
            return Err("signed point sum differs from the full target point".into());
        }
        for block in &self.blocks {
            let xs = block.coordinates.map(|c| c.evaluate(&values));
            let aux = block_assignment(&self.curve, self.arm, xs)
                .ok_or("no coefficient-field lift for a block")?;
            for (index, value) in block.auxiliaries.iter().zip(aux) {
                values[*index] = value;
            }
        }
        self.check_assignment(&values)?;
        Ok(values)
    }

    fn export(&self) -> Value {
        let equations: Vec<Value> = self.labels.iter().zip(&self.equations).map(|(label, poly)| {
            json!({"label": label, "total_degree": total_degree(poly), "terms": poly})
        }).collect();
        json!({
            "arm": self.arm.name(), "curve": self.curve,
            "target": point_json(self.target), "m": self.m, "depth": self.depth,
            "selector": "D_(2^depth)(x,1)=0; d_next=d^2-2; final d^2-2=0",
            "ordering": "grevlex; variables listed in descending precedence",
            "variables": self.names, "variable_domains": self.domains(),
            "domain_semantics": "null means the full coefficient field for every variable",
            "leaf_indices": self.ladders.iter().map(|l| l[0]).collect::<Vec<_>>(),
            "intermediate_indices": self.intermediates,
            "input_maximum_total_degree": self.equations.iter().map(total_degree).max(),
            "term_encoding": "[exponent_vector, canonical_coefficient]; equations equal zero",
            "equations": equations,
            "chart": "finite affine chain; partial sums at infinity require a separate chart",
            "signed_validation": "lift_abscissae followed by literal full-point target replay",
        })
    }
}

fn invariants(f: &Field, [x, y, z]: [u64; 3]) -> (u64, u64, u64, u64) {
    let r = f.mul(x, y);
    (
        r,
        f.add(f.add(x, y), z),
        f.add(r, f.mul(z, f.add(x, y))),
        f.mul(r, z),
    )
}

fn block_assignment(curve: &PrimeCurve, arm: Arm, xs: [u64; 3]) -> Option<Vec<u64>> {
    let f = Field::new(curve.p);
    let (r, e1, e2, e3) = invariants(&f, xs);
    match arm {
        Arm::Quartic => Some(vec![]),
        Arm::Invariant => Some(vec![r, e2, e3]),
        Arm::Line => {
            let lambda = curve.sqrt(e1)?;
            let nu = if lambda == 0 {
                if e2 != curve.a {
                    return None;
                }
                curve.sqrt(f.add(e3, curve.b))?
            } else {
                f.mul(f.sub(curve.a, e2), f.inv(f.mul(2, lambda))?)
            };
            if f.mul(nu, nu) != f.add(e3, curve.b) {
                return None;
            }
            Some(vec![r, lambda, nu])
        }
    }
}

fn point_json(point: PrimePoint) -> Value {
    json!({"x": point.x, "y": point.y, "infinity": point.infinity})
}

/// This minimal factor-base view supplies only the points and x_index consumed
/// by lift_abscissae. It is never used for relation coefficients or linear
/// algebra, so no unknown subgroup order is invented for the tiny fixture.
fn recover_signed(curve: &PrimeCurve, xs: &[u64], target: PrimePoint) -> Option<Vec<PrimePoint>> {
    if xs.is_empty() || target.infinity || !curve.is_on_curve(target) {
        return None;
    }
    let mut view = FactorBase::empty("signed-lift-only view; no relation-column metadata".into());
    for x in xs {
        if *x >= curve.p {
            return None;
        }
        if view.x_index.contains_key(x) {
            continue;
        }
        let y = curve.sqrt(curve.rhs(*x))?;
        let y = y.min(if y == 0 { 0 } else { curve.p - y });
        let point = PrimePoint::affine(*x, y);
        let negative = curve.neg(point);
        let mut indices = vec![view.points.len()];
        view.points.push(point);
        if negative != point {
            indices.push(view.points.len());
            view.points.push(negative);
        }
        view.x_index.insert(*x, indices);
    }
    let mut ops = GroupOps::default();
    let indices = lift_abscissae(curve, &view, &mut ops, xs, target)?;
    let points: Vec<_> = indices.into_iter().map(|i| view.points[i]).collect();
    let sum = points.iter().fold(curve.identity(), |acc, point| {
        curve.add(&mut ops, acc, *point)
    });
    if sum != target || points.iter().any(|p| !curve.is_on_curve(*p)) {
        return None;
    }
    Some(points)
}

fn prime31_curve() -> PrimeCurve {
    // Existing prime31_wz_rho_frontier fixture; no new group-order claim.
    PrimeCurve {
        p: M31,
        a: 847_514_846,
        b: 2_087_444_724,
    }
}

/// A transparent known-answer fixture, never an unknown-target attack cohort.
/// Repeating the first suitable selector point makes the constructor bounded
/// and checks the tangent case as well as shared intermediate consistency.
fn planted_fixture(
    curve: &PrimeCurve,
    m: usize,
    depth: u32,
) -> Result<(Vec<PrimePoint>, PrimePoint), String> {
    if !(2..=4).contains(&m) || !(1..=12).contains(&depth) {
        return Err("the bounded fixture exporter supports m=2..4 and depth=1..12".into());
    }
    let mut roots = dickson_abscissae(curve.p, depth, 0, 0)?;
    roots.sort_unstable();
    for x in roots {
        let Some(y) = curve.sqrt(curve.rhs(x)) else {
            continue;
        };
        let point = PrimePoint::affine(x, y.min(if y == 0 { 0 } else { curve.p - y }));
        let mut target = point;
        let mut ops = GroupOps::default();
        let mut finite = true;
        for _ in 1..m {
            target = curve.add(&mut ops, target, point);
            if target.infinity {
                finite = false;
                break;
            }
        }
        if finite {
            return Ok((vec![point; m], target));
        }
    }
    Err("no finite planted fixture among the selector roots".into())
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn check_block_identities() -> Result<Value, String> {
    let mut cases = 0usize;
    for p in [5, 7] {
        let curve = PrimeCurve { p, a: 2, b: 1 };
        let ps = Polys::new(p, 6);
        let f = Field::new(p);
        let block = Block {
            coordinates: [
                Coordinate::Variable(0),
                Coordinate::Variable(1),
                Coordinate::Variable(2),
            ],
            auxiliaries: vec![3, 4, 5],
        };
        let quartic = block_equations(&ps, &curve, Arm::Quartic, &block);
        let q = block_equations(&ps, &curve, Arm::Invariant, &block);
        let l = block_equations(&ps, &curve, Arm::Line, &block);
        require(
            total_degree(&quartic[0]) == 4,
            "quartic control lost degree four",
        )?;
        require(
            q.iter().chain(&l).all(|poly| total_degree(poly) <= 2),
            "a lifted block is not quadratic",
        )?;
        for x in 0..p {
            for y in 0..p {
                for z in 0..p {
                    let xs = [x, y, z];
                    let expected = eval_s3_u64(x, y, z, curve.a, curve.b, p);
                    let mut assignment = xs.to_vec();
                    assignment.extend(block_assignment(&curve, Arm::Invariant, xs).unwrap());
                    require(
                        evaluate(&quartic[0], &assignment, &f) == expected,
                        "quartic differs from production eval_s3_u64",
                    )?;
                    require(
                        q[..3]
                            .iter()
                            .all(|poly| evaluate(poly, &assignment, &f) == 0),
                        "invariant definitions failed",
                    )?;
                    require(
                        evaluate(&q[3], &assignment, &f) == expected,
                        "invariant residual differs from S3",
                    )?;
                    if let Some(aux) = block_assignment(&curve, Arm::Line, xs) {
                        let mut line_assignment = xs.to_vec();
                        line_assignment.extend(aux);
                        require(
                            expected == 0
                                && l.iter()
                                    .all(|poly| evaluate(poly, &line_assignment, &f) == 0),
                            "line lift did not satisfy S3 and its equations",
                        )?;
                    }
                    cases += 1;
                }
            }
        }
    }
    Ok(json!({"exhaustive_triples": cases, "fields": [5, 7], "input_degree_checked": true}))
}

fn check_exceptional_charts() -> Result<Value, String> {
    let horizontal = PrimeCurve {
        p: M31,
        a: M31 - 1,
        b: 1,
    };
    let xs = [0, 1, M31 - 1];
    let f = Field::new(M31);
    let (_, e1, e2, e3) = invariants(&f, xs);
    require(
        e1 == 0 && e2 == horizontal.a && f.add(e3, horizontal.b) == 1,
        "horizontal invariants differ",
    )?;
    require(
        eval_s3_u64(xs[0], xs[1], xs[2], horizontal.a, horizontal.b, M31) == 0,
        "horizontal S3 failed",
    )?;
    let aux = block_assignment(&horizontal, Arm::Line, xs)
        .ok_or("horizontal line was lost by division")?;
    require(aux[1] == 0, "horizontal lambda was not zero")?;
    let block = Block {
        coordinates: [
            Coordinate::Variable(0),
            Coordinate::Variable(1),
            Coordinate::Variable(2),
        ],
        auxiliaries: vec![3, 4, 5],
    };
    let ps = Polys::new(M31, 6);
    for arm in [Arm::Invariant, Arm::Line] {
        let mut assignment = xs.to_vec();
        assignment
            .extend(block_assignment(&horizontal, arm, xs).ok_or("horizontal block lift missing")?);
        require(
            block_equations(&ps, &horizontal, arm, &block)
                .iter()
                .all(|poly| evaluate(poly, &assignment, &f) == 0),
            "a quadratic block lost the horizontal witness",
        )?;
    }
    let points = xs.map(|x| PrimePoint::affine(x, 1));
    let mut ops = GroupOps::default();
    let sum = points.iter().fold(horizontal.identity(), |acc, point| {
        horizontal.add(&mut ops, acc, *point)
    });
    require(
        sum.infinity && points.iter().all(|p| horizontal.is_on_curve(*p)),
        "horizontal curve witness failed",
    )?;
    // e1*h=1 cannot hold here: a denominator-saturated rational chart loses
    // this valid triple, whereas the denominator-free Q and L blocks retain it.
    require(
        f.inv(e1).is_none(),
        "zero denominator unexpectedly invertible",
    )?;

    let twist = PrimeCurve { p: 5, a: 2, b: 1 };
    let twist_xs = [2, 2, 4];
    require(
        eval_s3_u64(2, 2, 4, 2, 1, 5) == 0,
        "twist-only S3 fixture failed",
    )?;
    require(
        block_assignment(&twist, Arm::Invariant, twist_xs).is_some(),
        "Q lost twist-only triple",
    )?;
    require(
        block_assignment(&twist, Arm::Line, twist_xs).is_none(),
        "L admitted the nonsquare-e1 twist-only triple",
    )?;
    require(
        twist_xs.iter().all(|x| twist.sqrt(twist.rhs(*x)).is_none()),
        "twist-only abscissa lifted to the original curve",
    )?;
    let mut twist_assignment = twist_xs.to_vec();
    twist_assignment.extend(block_assignment(&twist, Arm::Invariant, twist_xs).unwrap());
    let twist_ps = Polys::new(5, 6);
    let twist_field = Field::new(5);
    require(
        block_equations(&twist_ps, &twist, Arm::Invariant, &block)
            .iter()
            .all(|poly| evaluate(poly, &twist_assignment, &twist_field) == 0),
        "Q equations rejected the twist-only witness",
    )?;
    Ok(json!({
        "horizontal": {"p": M31, "x": xs, "e1": e1, "retained_by_q_and_l": true, "excluded_by_e1_inverse_chart": true},
        "twist_only": {"p": 5, "a": 2, "b": 1, "x": twist_xs, "quartic_and_q": true, "line_l": false},
    }))
}

fn check_all_m31_roots() -> Result<Value, String> {
    let roots = dickson_abscissae(M31, 12, 0, 0)?;
    let distinct: BTreeSet<u64> = roots.iter().copied().collect();
    require(
        roots.len() == 4096 && distinct.len() == 4096,
        "selector did not return 4096 distinct roots",
    )?;
    let f = Field::new(M31);
    for x in &roots {
        let mut value = *x;
        for _ in 0..12 {
            value = f.sub(f.mul(value, value), 2);
        }
        require(
            value == 0,
            "a reported selector root failed its forward ladder",
        )?;
    }
    Ok(
        json!({"p": M31, "depth": 12, "distinct_roots_checked": roots.len(), "degree": 4096, "completeness_reason": "4096 distinct roots of a monic polynomial of degree 4096"}),
    )
}

fn check_m31_assignments() -> Result<Value, String> {
    let curve = prime31_curve();
    let mut checked = Vec::new();
    for m in 2..=4 {
        let (points, target) = planted_fixture(&curve, m, 3)?;
        let mut reference_selector: Option<Vec<Poly>> = None;
        for arm in Arm::ALL {
            let system = System::new(curve.clone(), target, arm, m, 3)?;
            let assignment = system.assignment_for_points(&points)?;
            require(
                system.domains().iter().all(Option::is_none),
                "an auxiliary received a restricted domain",
            )?;
            let selector: Vec<Poly> = system.equations[..m * 3]
                .iter()
                .map(|poly| {
                    poly.iter()
                        .map(|(e, c)| (e[..m * 3].to_vec(), *c))
                        .collect()
                })
                .collect();
            if let Some(reference) = &reference_selector {
                require(
                    selector == *reference,
                    "matched arms have different leaf selectors",
                )?;
            } else {
                reference_selector = Some(selector);
            }
            let recovered = recover_signed(&curve, &system.leaf_xs(&assignment), target)
                .ok_or("signed recovery failed")?;
            require(
                recovered.len() == m,
                "signed recovery returned the wrong length",
            )?;
            let mut altered = assignment.clone();
            let i = *system.ladders[0].last().unwrap();
            altered[i] = Field::new(curve.p).add(altered[i], 1);
            require(
                system.check_assignment(&altered).is_err(),
                "tampered assignment was accepted",
            )?;
            if let Some(index) = system.blocks[0].auxiliaries.first() {
                let mut altered_auxiliary = assignment.clone();
                altered_auxiliary[*index] = Field::new(curve.p).add(altered_auxiliary[*index], 1);
                require(
                    system.check_assignment(&altered_auxiliary).is_err(),
                    "tampered product auxiliary was accepted",
                )?;
            }
            checked.push(json!({"m": m, "arm": arm.name(), "variables": system.names.len(), "equations": system.equations.len(), "signed_target_replay": true}));
        }
        let negative_target = curve.neg(target);
        let system = System::new(curve.clone(), negative_target, Arm::Invariant, m, 3)?;
        require(
            system.assignment_for_points(&points).is_err(),
            "assignment constructor compared only target x",
        )?;
    }
    let (points, _) = planted_fixture(&curve, 4, 3)?;
    let mut ops = GroupOps::default();
    let target = curve.add(&mut ops, points[2], points[3]);
    let with_infinity = [points[0], curve.neg(points[0]), points[2], points[3]];
    let system = System::new(curve, target, Arm::Invariant, 4, 3)?;
    require(
        system.assignment_for_points(&with_infinity).is_err(),
        "finite chart silently accepted an infinity prefix",
    )?;
    Ok(
        json!({"planted_correctness_only": true, "cells": checked, "wrong_target_sign_rejected": true, "infinity_prefix_explicitly_rejected": true}),
    )
}

fn fallback_classification(report: &VisitReport) -> &'static str {
    if report.full_field_branches > 0 {
        "enumerative_fallback_observed"
    } else {
        "no_full_field_fallback_observed; this alone is not an algebraic-extractor proof"
    }
}

fn check_tiny_solve() -> Result<Value, String> {
    let curve = PrimeCurve { p: 31, a: 2, b: 3 };
    let points = [PrimePoint::affine(8, 2), PrimePoint::affine(23, 8)];
    let target = PrimePoint::affine(20, 18);
    let mut ops = GroupOps::default();
    require(
        points.iter().all(|p| curve.is_on_curve(*p))
            && curve.add(&mut ops, points[0], points[1]) == target,
        "tiny known-answer fixture failed",
    )?;
    let roots = dickson_abscissae(curve.p, 1, 0, 0)?;
    let mut expected = BTreeSet::new();
    // This 2^2 control is explicitly enumerative ground truth, kept separate
    // from the unseeded solve_visit call and never counted as solver work.
    for x in &roots {
        for y in &roots {
            let xs = vec![*x, *y];
            if recover_signed(&curve, &xs, target).is_some() {
                expected.insert(xs);
            }
        }
    }
    require(
        expected == BTreeSet::from([vec![8, 23], vec![23, 8]]),
        "tiny independent signed control changed",
    )?;
    let mut reports = Vec::new();
    for arm in Arm::ALL {
        let system = System::new(curve.clone(), target, arm, 2, 1)?;
        system.assignment_for_points(&points)?;
        let domains = system.domains();
        require(
            domains.iter().all(Option::is_none),
            "tiny solver received an enumerated domain",
        )?;
        let opts = F4Options::new(Ordering::Grevlex, TINY_MAX_DEGREE)
            .with_budget(Duration::from_secs(TINY_DEADLINE_SECONDS));
        let mut projected = BTreeSet::new();
        let mut callback_error: Option<String> = None;
        let report = f4_fp::solve_visit(
            &system.equations,
            system.names.len(),
            curve.p,
            &opts,
            &domains,
            |values| {
                if let Err(error) = system.check_assignment(values) {
                    callback_error = Some(error);
                    return true;
                }
                let xs = system.leaf_xs(values);
                if recover_signed(&curve, &xs, target).is_none() {
                    callback_error = Some(
                        "equation solution did not recover an exact signed target witness".into(),
                    );
                    return true;
                }
                projected.insert(xs);
                false
            },
        )?;
        if let Some(error) = callback_error {
            return Err(error);
        }
        require(
            report.complete && !report.stopped_early && !report.timed_out && !report.trace_stopped,
            "tiny solve did not complete within the correctness guard",
        )?;
        require(
            projected == expected,
            "tiny solver projected set differs from exhaustive signed control",
        )?;
        reports.push(json!({
            "arm": arm.name(), "variables": system.names.len(),
            "distinct_ordered_leaf_witnesses": projected,
            "fallback_classification": fallback_classification(&report),
            "audit_counters": report,
        }));
    }
    Ok(json!({
        "fixture": {"p": 31, "a": 2, "b": 3, "m": 2, "depth": 1, "target": point_json(target)},
        "purpose": "bounded complete correctness control; no performance conclusion",
        "domain_policy": "all variables full field; selector roots are not supplied to solve_visit",
        "ground_truth": "exhaustive 2^2 selector tuples with production signed point lifting",
        "max_degree_guard": TINY_MAX_DEGREE,
        "deadline_seconds_guard": TINY_DEADLINE_SECONDS,
        "arms": reports,
    }))
}

fn correctness() -> Result<Value, String> {
    Ok(json!({
        "schema": "dickson-s3-quadratic-correctness/v1",
        "block_identities": check_block_identities()?,
        "exceptional_charts": check_exceptional_charts()?,
        "selector": check_all_m31_roots()?,
        "prime31_assignments": check_m31_assignments()?,
        "tiny_complete_solve": check_tiny_solve()?,
        "prime31_solver_measurements": null,
        "performance_claim": null,
    }))
}

fn plan() -> Value {
    json!({
        "schema": "dickson-s3-quadratic-plan/v1",
        "status": "correctness and export harness; comparison experiment not admitted here",
        "field": M31,
        "selector": {"a": 1, "lo": 0, "c_top": 0, "root_test_depth": 12, "root_count": 4096},
        "arms": Arm::ALL.map(Arm::name),
        "m": [2, 3, 4],
        "variable_count": {"quartic": "m*depth + m - 2", "invariant_q_or_line_l": "m*depth + m - 2 + 3*(m-1)"},
        "equation_count": {"quartic": "m*depth + m - 1", "invariant_q_or_line_l": "m*depth + 4*(m-1)"},
        "domain_policy": "full field for every variable; identical zero-selector equations on leaves",
        "finite_chart_limit": "prefix sums at infinity are rejected and require a separate chart",
        "q_projection": "exact quartic S3 zero set, including twist-only triples",
        "l_projection": "rational-line locus; signed point recovery and exact target replay still required",
        "not_yet_measured": ["Prime31 coupled extraction", "matched solving degree and peak matrices", "complete useful-relation cost", "independent target cohort", "full DLP recovery"],
        "commands": ["--check", "--export --m 4 --depth 3", "--plan"],
    })
}

fn run() -> Result<Value, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args == ["--plan"] {
        return Ok(plan());
    }
    if args == ["--check"] {
        return correctness();
    }
    if args.first().map(String::as_str) != Some("--export") {
        return Err("usage: --plan | --check | --export [--m 2|3|4] [--depth 1..12]".into());
    }
    let mut parameters = BTreeMap::from([("--m", 4usize), ("--depth", 3usize)]);
    let mut seen = BTreeSet::new();
    let mut i = 1;
    while i < args.len() {
        let key = args[i].as_str();
        if !parameters.contains_key(key) || !seen.insert(key) {
            return Err(format!("unknown or repeated parameter: {key}"));
        }
        let value = args
            .get(i + 1)
            .ok_or("missing parameter value")?
            .parse::<usize>()
            .map_err(|e| e.to_string())?;
        parameters.insert(key, value);
        i += 2;
    }
    let m = parameters["--m"];
    let depth = u32::try_from(parameters["--depth"]).map_err(|e| e.to_string())?;
    let curve = prime31_curve();
    let (points, target) = planted_fixture(&curve, m, depth)?;
    let mut systems = Vec::new();
    for arm in Arm::ALL {
        let system = System::new(curve.clone(), target, arm, m, depth)?;
        system.assignment_for_points(&points)?;
        systems.push(system.export());
    }
    Ok(json!({
        "schema": "dickson-s3-quadratic-systems/v1",
        "fixture_kind": "public planted repeated-point correctness fixture; not an unknown-target measurement",
        "public_fixture_points": points.into_iter().map(point_json).collect::<Vec<_>>(),
        "systems": systems,
        "performance_claim": null,
    }))
}

fn main() {
    match run() {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).unwrap()),
        Err(error) => {
            eprintln!("dickson_s3_quadratic: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_quadratic_blocks_match_s3() {
        check_block_identities().unwrap();
    }

    #[test]
    fn horizontal_and_twist_only_charts_are_explicit() {
        check_exceptional_charts().unwrap();
    }

    #[test]
    fn every_m31_selector_root_satisfies_the_ladder() {
        check_all_m31_roots().unwrap();
    }

    #[test]
    fn m2_m3_m4_m31_assignments_and_signed_targets() {
        check_m31_assignments().unwrap();
    }

    #[test]
    fn matched_tiny_systems_are_actually_solved() {
        check_tiny_solve().unwrap();
    }
}
