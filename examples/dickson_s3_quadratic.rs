//! Matched Dickson-selector systems with quartic and quadratic S3 blocks.
//!
//! This is an export and correctness harness, not a comparison experiment.
//! `--check` includes one bounded, complete, tiny-field solve_visit control.
//! `--export --m 4 --depth 3` emits all four arms on a public planted fixture.
//! `--check-m31-accounting` attempts one fixed public Prime31 control.
//! Accounting timings are diagnostic development observations, not speed claims.
//!
//! Every arm uses D_(2^depth)(x, 1) = 0 and the same finite intermediate-x
//! chain. The input-degree reduction does not imply a solving-degree reduction.
//! Invariant Q preserves the quartic zero set over the coefficient field;
//! line L additionally requires an F_p-rational line and can exclude twist-only
//! triples. All accepted relations still require signed, exact point replay.
//! The parent-oriented O arm reuses the all-node formulation from PR #4928.
//! Its slopes and shared internal ordinates retain the actual root orientation.

#[path = "dickson_s3_quadratic/accounting.rs"]
mod accounting;

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly, VisitReport};
use crypto_lib::cryptanalysis::fp_backend::{Fp32, PrimeField};
use crypto_lib::cryptanalysis::groebner_f4::cmp_monomial;
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
    Oriented,
}

impl Arm {
    const ALL: [Self; 4] = [Self::Quartic, Self::Invariant, Self::Line, Self::Oriented];

    fn name(self) -> &'static str {
        match self {
            Self::Quartic => "quartic_s3",
            Self::Invariant => "invariant_q",
            Self::Line => "line_l",
            Self::Oriented => "oriented_o",
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
    n: usize,
}

impl Polys {
    fn new(p: u64, n: usize) -> Self {
        Self {
            field: Field::new(p),
            n,
        }
    }
    /// Merge equal monomials without rescaling the expression. The solver's
    /// `normalise` makes its result monic, which is valid for a completed
    /// zero equation but changes intermediate sums/products and constants.
    fn collect_terms(&self, terms: Poly) -> Poly {
        let mut coefficients = BTreeMap::<Vec<u32>, u64>::new();
        for (exponents, coefficient) in terms {
            let entry = coefficients.entry(exponents).or_insert(0);
            *entry = self.field.add(*entry, coefficient);
        }
        let mut result: Poly = coefficients.into_iter().filter(|(_, c)| *c != 0).collect();
        result.sort_unstable_by(|(a, _), (b, _)| cmp_monomial(b, a, Ordering::Grevlex));
        result
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
        self.collect_terms(terms)
    }
    fn scale(&self, a: &Poly, c: u64) -> Poly {
        let terms: Poly = a
            .iter()
            .map(|(e, v)| (e.clone(), self.field.mul(*v, c)))
            .collect();
        self.collect_terms(terms)
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
        self.collect_terms(terms)
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
    /// The oriented parent's ordinate; absent in the three x-only arms.
    parent_y: Option<Coordinate>,
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
    /// Shared ordinates of the finite nonroot prefix points, for O only.
    intermediate_ys: Vec<usize>,
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
        Arm::Oriented => {
            let lambda = ps.variable(block.auxiliaries[0]);
            let parent_y = block
                .parent_y
                .expect("oriented block requires its parent ordinate")
                .polynomial(ps);
            let numerator = ps.sub(
                &ps.mul(&ps.sub(&x, &z), &ps.sub(&y, &z)),
                &ps.add(&ps.scale(&ps.square(&z), 3), &a),
            );
            vec![
                ps.sub(&ps.square(&lambda), &ps.add(&sum_xy, &z)),
                ps.sub(&ps.scale(&ps.mul(&parent_y, &lambda), 2), &numerator),
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
        let intermediate_ys: Vec<usize> = if arm == Arm::Oriented {
            (0..m - 2)
                .map(|j| append_variable(&mut names, format!("y_t_{j}")))
                .collect()
        } else {
            Vec::new()
        };
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
                Arm::Oriented => vec![append_variable(&mut names, format!("lambda_{j}"))],
            };
            let parent_y = if arm == Arm::Oriented {
                Some(if j == m - 2 {
                    Coordinate::Constant(target.y)
                } else {
                    Coordinate::Variable(intermediate_ys[j])
                })
            } else {
                None
            };
            blocks.push(Block {
                coordinates: [
                    Coordinate::Variable(left),
                    Coordinate::Variable(ladders[j + 1][0]),
                    third,
                ],
                auxiliaries,
                parent_y,
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
            if arm == Arm::Oriented && j > 0 {
                let lambda = ps.variable(block.auxiliaries[0]);
                let parent_x = block.coordinates[2].polynomial(&ps);
                let parent_y = block.parent_y.unwrap().polynomial(&ps);
                let left_x = block.coordinates[0].polynomial(&ps);
                let oriented_left_y =
                    ps.sub(&ps.mul(&lambda, &ps.sub(&left_x, &parent_x)), &parent_y);
                equations.push(ps.sub(&ps.variable(intermediate_ys[j - 1]), &oriented_left_y));
                labels.push(format!("oriented_o_propagate_{}_from_{j}", j - 1));
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
            intermediate_ys,
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
        let mut prefixes = Vec::with_capacity(self.m - 1);
        for j in 0..self.m - 1 {
            prefix = self.curve.add(&mut ops, prefix, points[j + 1]);
            if prefix.infinity {
                return Err(format!("finite chain chart excludes infinity at block {j}"));
            }
            if j < self.intermediates.len() {
                values[self.intermediates[j]] = prefix.x;
                if self.arm == Arm::Oriented {
                    values[self.intermediate_ys[j]] = prefix.y;
                }
            }
            prefixes.push(prefix);
        }
        if prefix != self.target {
            return Err("signed point sum differs from the full target point".into());
        }
        for (j, block) in self.blocks.iter().enumerate() {
            let aux = if self.arm == Arm::Oriented {
                let left = if j == 0 { points[0] } else { prefixes[j - 1] };
                vec![signed_chord_slope(&self.curve, left, points[j + 1])?]
            } else {
                let xs = block.coordinates.map(|c| c.evaluate(&values));
                block_assignment(&self.curve, self.arm, xs)
                    .ok_or("no coefficient-field lift for a block")?
            };
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
            "intermediate_y_indices": self.intermediate_ys,
            "oriented_slope_indices": if self.arm == Arm::Oriented {
                self.blocks.iter().map(|block| block.auxiliaries[0]).collect::<Vec<_>>()
            } else {
                Vec::new()
            },
            "input_maximum_total_degree": self.equations.iter().map(total_degree).max(),
            "term_encoding": "[exponent_vector, canonical_coefficient]; equations equal zero",
            "equations": equations,
            "chart": "finite affine chain; partial sums at infinity require a separate chart",
            "signed_validation": "lift_abscissae followed by literal full-point target replay",
            "oriented_semantics": if self.arm == Arm::Oriented {
                Some("shared signed prefix ordinates and chord slopes; y_child=lambda*(x_child-x_parent)-y_parent; no division by parent y")
            } else {
                None
            },
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
        // Abscissae alone do not select an oriented lift at a 2-torsion
        // parent. The signed assignment constructor supplies actual children.
        Arm::Oriented => None,
    }
}

/// The slope of the true signed chord/tangent. A finite 2-torsion parent is
/// permitted: the denominator is a child abscissa difference or a tangent's
/// nonzero child ordinate, never the parent's ordinate. A vertical chord or
/// the tangent at a 2-torsion child has sum O and is outside this finite chart.
fn signed_chord_slope(
    curve: &PrimeCurve,
    left: PrimePoint,
    right: PrimePoint,
) -> Result<u64, String> {
    if left.infinity
        || right.infinity
        || left.x >= curve.p
        || left.y >= curve.p
        || right.x >= curve.p
        || right.y >= curve.p
        || !curve.is_on_curve(left)
        || !curve.is_on_curve(right)
    {
        return Err("chord children must be canonical finite curve points".into());
    }
    let f = Field::new(curve.p);
    let (numerator, denominator) = if left.x != right.x {
        (f.sub(right.y, left.y), f.sub(right.x, left.x))
    } else if left.y == right.y && left.y != 0 {
        (
            f.add(f.mul(3, f.mul(left.x, left.x)), curve.a),
            f.mul(2, left.y),
        )
    } else {
        return Err("vertical chord or 2-torsion tangent has an infinity parent".into());
    };
    let inverse = f
        .inv(denominator)
        .ok_or("noninvertible chord denominator")?;
    Ok(f.mul(numerator, inverse))
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
            parent_y: None,
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
                    let actual = evaluate(&quartic[0], &assignment, &f);
                    if actual != expected {
                        return Err(format!("quartic differs from production eval_s3_u64: p={p}, xs={xs:?}, actual={actual}, expected={expected}"));
                    }
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
        parent_y: None,
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

/// Constructor and exceptional-case controls only. The separate accounting
/// module owns complete target extraction and signed-output enumeration.
fn check_oriented_charts() -> Result<Value, String> {
    let curve = PrimeCurve { p: 7, a: 0, b: 3 };
    let a = PrimePoint::affine(3, 3);
    let b = PrimePoint::affine(4, 2);
    let target = PrimePoint::affine(1, 2);
    let minus_b = curve.neg(b);
    let cases = [
        ("pair", vec![a, b], vec![6]),
        ("repeated_tangent", vec![b, b, b], vec![5, 6]),
        ("horizontal_root", vec![a, a, minus_b], vec![1, 0]),
        ("same_leaf_x_first", vec![a, b, b, minus_b], vec![6, 0, 0]),
        ("same_leaf_x_second", vec![a, b, minus_b, b], vec![6, 1, 6]),
        (
            "child_x_equals_parent",
            vec![a, minus_b, b, b],
            vec![2, 5, 6],
        ),
    ];
    let mut checked = Vec::new();
    let mut same_leaf_x_assignments = BTreeSet::new();
    for (label, points, slopes) in cases {
        let m = points.len();
        let system = System::new(curve.clone(), target, Arm::Oriented, m, 1)?;
        let assignment = system.assignment_for_points(&points)?;
        require(
            system.names.len() == 4 * m - 5 && system.equations.len() == 4 * m - 4,
            "oriented variable/equation counts differ from the lean construction",
        )?;
        require(
            system.intermediate_ys.len() == m - 2
                && system.domains().iter().all(Option::is_none)
                && system.equations.iter().all(|p| total_degree(p) <= 2),
            "oriented shared-y, degree, or full-field domain contract failed",
        )?;
        let f = Field::new(curve.p);
        for (j, (block, expected_slope)) in system.blocks.iter().zip(slopes).enumerate() {
            let lambda = assignment[block.auxiliaries[0]];
            require(
                lambda == expected_slope,
                "signed chord slope differs from the fixed fixture",
            )?;
            let parent_x = block.coordinates[2].evaluate(&assignment);
            let parent_y = block.parent_y.unwrap().evaluate(&assignment);
            let left = if j == 0 {
                points[0]
            } else {
                PrimePoint::affine(
                    assignment[system.intermediates[j - 1]],
                    assignment[system.intermediate_ys[j - 1]],
                )
            };
            for child in [left, points[j + 1]] {
                require(
                    f.sub(f.mul(lambda, f.sub(child.x, parent_x)), parent_y) == child.y,
                    "downward orientation differs from the supplied signed child",
                )?;
            }
        }
        if let Some(index) = system.intermediate_ys.first() {
            let mut altered = assignment.clone();
            altered[*index] = f.add(altered[*index], 1);
            require(
                system.check_assignment(&altered).is_err(),
                "altered shared ordinate accepted",
            )?;
        }
        if m == 4 {
            require(
                system.leaf_xs(&assignment) == vec![3, 4, 4, 4],
                "fixed leaf x tuple changed",
            )?;
            same_leaf_x_assignments.insert(assignment);
        }
        checked.push(json!({"case": label, "m": m, "variables": system.names.len(), "equations": system.equations.len()}));
    }
    require(
        same_leaf_x_assignments.len() == 3,
        "distinct signed chains with the same leaf abscissae were collapsed",
    )?;

    let positive = System::new(curve.clone(), target, Arm::Oriented, 2, 1)?;
    let negative = System::new(curve.clone(), curve.neg(target), Arm::Oriented, 2, 1)?;
    let positive_assignment = positive.assignment_for_points(&[a, b])?;
    let negative_assignment = negative.assignment_for_points(&[curve.neg(a), minus_b])?;
    require(
        positive.leaf_xs(&positive_assignment) == negative.leaf_xs(&negative_assignment)
            && positive.check_assignment(&negative_assignment).is_err()
            && negative.assignment_for_points(&[a, b]).is_err(),
        "signed-root flip was confused with an x-only target check",
    )?;
    let finite = System::new(curve.clone(), target, Arm::Oriented, 4, 1)?;
    require(
        finite
            .assignment_for_points(&[a, curve.neg(a), a, b])
            .is_err()
            && System::new(curve.clone(), curve.identity(), Arm::Oriented, 2, 1).is_err(),
        "an infinity prefix or root was silently admitted",
    )?;

    // On this separate even-order curve, the finite target has ordinate zero.
    // Both sign choices are valid oriented assignments; no inversion of 2s
    // or odd-order uniqueness assumption is permitted here.
    let torsion_curve = PrimeCurve { p: 7, a: 3, b: 3 };
    let torsion_target = PrimePoint::affine(1, 0);
    let p = PrimePoint::affine(3, 2);
    let q = PrimePoint::affine(4, 3);
    let torsion = System::new(torsion_curve.clone(), torsion_target, Arm::Oriented, 2, 1)?;
    let first = torsion.assignment_for_points(&[p, q])?;
    let second = torsion.assignment_for_points(&[torsion_curve.neg(p), torsion_curve.neg(q)])?;
    let slope_index = torsion.blocks[0].auxiliaries[0];
    require(
        torsion.leaf_xs(&first) == torsion.leaf_xs(&second)
            && first[slope_index] == 1
            && second[slope_index] == 6
            && first != second,
        "finite 2-torsion parent lost one of its two signed lifts",
    )?;
    let internal_torsion = System::new(
        torsion_curve.clone(),
        torsion_curve.neg(p),
        Arm::Oriented,
        3,
        1,
    )?;
    let internal_first = internal_torsion.assignment_for_points(&[p, q, q])?;
    let internal_second =
        internal_torsion.assignment_for_points(&[torsion_curve.neg(p), torsion_curve.neg(q), q])?;
    require(
        internal_first[internal_torsion.intermediates[0]] == torsion_target.x
            && internal_first[internal_torsion.intermediate_ys[0]] == 0
            && internal_second[internal_torsion.intermediate_ys[0]] == 0
            && internal_first[internal_torsion.blocks[0].auxiliaries[0]] == 1
            && internal_second[internal_torsion.blocks[0].auxiliaries[0]] == 6,
        "shared zero ordinate lost a valid internal 2-torsion branch",
    )?;
    require(
        signed_chord_slope(&torsion_curve, torsion_target, torsion_target).is_err(),
        "2-torsion doubling admitted a finite tangent",
    )?;

    // A rational 2-torsion parent can also support a quartic ghost. Here the
    // numerator of Q2 is zero but Q1 would require lambda^2=3 in F7.
    let ghost_block = Block {
        coordinates: [
            Coordinate::Variable(0),
            Coordinate::Variable(1),
            Coordinate::Variable(2),
        ],
        auxiliaries: vec![3],
        parent_y: Some(Coordinate::Constant(0)),
    };
    let ps = Polys::new(7, 4);
    let equations = block_equations(&ps, &torsion_curve, Arm::Oriented, &ghost_block);
    require(
        eval_s3_u64(2, 0, 1, 3, 3, 7) == 0
            && torsion_curve.sqrt(torsion_curve.rhs(2)).is_none()
            && torsion_curve.sqrt(torsion_curve.rhs(0)).is_none(),
        "2-torsion-parent ghost fixture changed",
    )?;
    for lambda in 0..7 {
        require(
            equations
                .iter()
                .any(|eq| evaluate(eq, &[2, 0, 1, lambda], &ps.field) != 0),
            "oriented equations admitted a nonrational 2-torsion-parent ghost",
        )?;
    }
    Ok(json!({
        "purpose": "oriented constructors and exceptional cases; no extraction or performance result",
        "finite_odd_order_fixture": {"p": 7, "a": 0, "b": 3, "target": point_json(target), "cases": checked},
        "same_leaf_x_distinct_signed_chains": same_leaf_x_assignments.len(),
        "root_sign_flip_checked": true,
        "infinity_prefix_and_root_rejected": true,
        "finite_two_torsion_parent": {"p": 7, "a": 3, "b": 3, "target": point_json(torsion_target), "distinct_slopes": [1, 6]},
        "finite_two_torsion_intermediate_checked": true,
        "two_torsion_parent_quartic_ghost_rejected": true,
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
        "oriented_charts": check_oriented_charts()?,
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
        "variable_count": {"quartic": "m*depth + m - 2", "invariant_q_or_line_l": "m*depth + m - 2 + 3*(m-1)", "oriented_o": "m*depth + 3*m - 5"},
        "equation_count": {"quartic": "m*depth + m - 1", "invariant_q_or_line_l": "m*depth + 4*(m-1)", "oriented_o": "m*depth + 3*m - 4"},
        "domain_policy": "full field for every variable; identical zero-selector equations on leaves",
        "finite_chart_limit": "prefix sums at infinity are rejected and require a separate chart",
        "q_projection": "exact quartic S3 zero set, including twist-only triples",
        "l_projection": "rational-line locus; signed point recovery and exact target replay still required",
        "o_projection": "root-oriented finite rational chain; shared internal y; 2-torsion parents may have distinct signed slope lifts",
        "not_yet_measured": ["Prime31 coupled extraction", "matched solving degree and peak matrices", "complete useful-relation cost", "independent target cohort", "full DLP recovery"],
        "commands": ["--check", "--check-accounting", "--export --m 4 --depth 3", "--plan"],
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
    if args == ["--check-accounting"] {
        return accounting::check();
    }
    if args == ["--check-m31-accounting"] {
        return accounting::check_m31();
    }
    if args.len() == 2 && args[0] == "--extract-accounting" {
        return accounting::extract_file(&args[1]);
    }
    if args.first().map(String::as_str) != Some("--export") {
        return Err("usage: --plan | --check | --check-accounting | --check-m31-accounting | --extract-accounting REQUEST.json | --export [--m 2|3|4] [--depth 1..12]".into());
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
    fn oriented_signs_shared_ordinates_and_two_torsion_are_explicit() {
        check_oriented_charts().unwrap();
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
