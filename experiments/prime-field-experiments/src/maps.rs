use std::collections::{BTreeMap, BTreeSet};

use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};

use crate::poly::{Poly, PrimeField};
use crate::roots::roots_of;
use crate::run_jobs;

const ENUMERATION_LIMIT: u64 = 10_000;

#[derive(Deserialize)]
pub struct Config {
    pub p: u64,
    pub curve: Vec<i128>,
    pub maps: Vec<MapSpec>,
    pub targets: Option<Vec<Option<i128>>>,
}

#[derive(Deserialize)]
pub struct MapSpec {
    pub name: String,
    pub numerator: [Vec<i128>; 2],
    pub denominator: [Vec<i128>; 2],
}

#[derive(Serialize)]
pub struct Report<'a> {
    pub p: u64,
    pub curve: &'a [i128],
    pub genus: i64,
    pub rational_point_count: Option<usize>,
    pub prime_curve_order: Option<bool>,
    pub exhaustive_targets: bool,
    pub maps: Vec<MapReport<'a>>,
}

#[derive(Serialize)]
pub struct MapReport<'a> {
    pub name: &'a str,
    pub numerator: &'a [Vec<i128>; 2],
    pub denominator: &'a [Vec<i128>; 2],
    pub raw_norm_degree: i64,
    pub fixed_norm_degree: i64,
    pub map_degree: i64,
    pub separable: bool,
    pub factors_through_x: bool,
    pub infinity_image: Option<u64>,
    pub verified_by_curve_enumeration: bool,
    pub fiber_size_histogram: BTreeMap<usize, usize>,
    pub reflected_fibers: usize,
    pub fibers: Vec<FiberReport>,
}

#[derive(Serialize)]
pub struct FiberReport {
    pub target: Option<u64>,
    pub specialized_norm_degree: i64,
    pub elimination_degree: i64,
    pub reflection_extraction_degree: i64,
    pub reflection_center: Option<u64>,
    pub distinct_elimination_roots: usize,
    pub rational_points: usize,
}

type Point = Option<(u64, u64)>;
type Pair = [Poly; 2];
type PointIndex = BTreeMap<u64, Vec<Point>>;

struct Curve {
    field: PrimeField,
    f: Poly,
    derivative: Poly,
}

impl Curve {
    fn new(field: PrimeField, coeffs: &[i128]) -> Result<Self> {
        ensure!(field.modulus() > 2, "expected an odd prime field");
        let f = Poly::from_signed(field, coeffs);
        ensure!(
            f.degree() > 0 && f.degree() % 2 == 1,
            "expected a curve polynomial of positive odd degree"
        );
        let derivative = f.derivative();
        ensure!(
            f.gcd(&derivative).degree() == 0,
            "the curve polynomial must be square-free"
        );
        Ok(Self {
            field,
            f,
            derivative,
        })
    }

    fn genus(&self) -> i64 {
        (self.f.degree() - 1) / 2
    }

    fn points_above(&self, x: u64) -> impl Iterator<Item = Point> {
        let root = self.field.sqrt(self.f.eval(x));
        root.into_iter()
            .chain(root.filter(|&y| y != 0).map(|y| self.field.neg(y)))
            .map(move |y| Some((x, y)))
    }
}

struct CurveMap<'a> {
    curve: &'a Curve,
    numerator: Pair,
    denominator: Pair,
    moving_norm: [Poly; 3],
    projection: Option<Pair>,
    fixed_x: Vec<u64>,
    raw_degree: i64,
    fixed_degree: i64,
    degree: i64,
}

impl<'a> CurveMap<'a> {
    fn new(curve: &'a Curve, spec: &MapSpec) -> Result<Self> {
        let numerator = spec
            .numerator
            .each_ref()
            .map(|c| Poly::from_signed(curve.field, c));
        let denominator = spec
            .denominator
            .each_ref()
            .map(|c| Poly::from_signed(curve.field, c));
        let [a, b] = &numerator;
        let [c, d] = &denominator;
        let n0 = norm(curve, &numerator);
        let n2 = norm(curve, &denominator);
        ensure!(n2.degree() >= 0, "zero denominator");
        let n1 = a
            .mul(c)
            .sub(&curve.f.mul(&b.mul(d)))
            .scale(curve.field.neg(2));
        let content = n0.gcd(&n1).gcd(&n2);
        let raw_degree = n0.degree().max(n1.degree()).max(n2.degree());
        let fixed_degree = content.degree();
        let degree = raw_degree - fixed_degree;
        ensure!(degree > 0, "expected a nonconstant function");
        let moving_norm = [n0, n1, n2].map(|poly| poly.quotient(&content));

        let projection = if a.mul(d) == b.mul(c) {
            let (u, v) = if c.degree() < 0 { (b, d) } else { (a, c) };
            let common = u.gcd(v);
            Some([u.quotient(&common), v.quotient(&common)])
        } else {
            None
        };
        let fixed_x = if projection.is_some() {
            Vec::new()
        } else {
            content.native_roots()
        };

        Ok(Self {
            curve,
            numerator,
            denominator,
            moving_norm,
            projection,
            fixed_x,
            raw_degree,
            fixed_degree,
            degree,
        })
    }

    fn fiber_polynomial(&self, target: Option<u64>) -> Poly {
        if let Some([u, v]) = &self.projection {
            return match target {
                Some(t) => u.sub(&v.scale(t)),
                None => v.scale(1),
            };
        }
        let [n0, n1, n2] = &self.moving_norm;
        match target {
            Some(t) => n0
                .add(&n1.scale(t))
                .add(&n2.scale(self.curve.field.mul(t, t))),
            None => n2.scale(1),
        }
    }

    fn separable(&self) -> bool {
        let f = &self.curve.f;
        let differentiate = |[a, b]: &Pair| {
            [
                f.mul(&b.derivative())
                    .scale(2)
                    .add(&self.curve.derivative.mul(b)),
                a.derivative().scale(2),
            ]
        };
        let multiply = |[a, b]: &Pair, [c, d]: &Pair| {
            [a.mul(c).add(&f.mul(&b.mul(d))), a.mul(d).add(&b.mul(c))]
        };
        multiply(&differentiate(&self.numerator), &self.denominator)
            != multiply(&self.numerator, &differentiate(&self.denominator))
    }

    fn value(&self, point: Point) -> Option<u64> {
        let field = self.curve.field;
        let eval_pair = |[a, b]: &Pair, x, y| field.add(a.eval(x), field.mul(y, b.eval(x)));
        let (numerator, denominator) = match point {
            None => (
                self.at_infinity(&self.numerator),
                self.at_infinity(&self.denominator),
            ),
            Some((x, y)) => {
                if let Some([u, v]) = &self.projection {
                    let den = v.eval(x);
                    return (den != 0).then(|| field.mul(u.eval(x), field.inv(den)));
                }
                let num = eval_pair(&self.numerator, x, y);
                let den = eval_pair(&self.denominator, x, y);
                if den != 0 {
                    return Some(field.mul(num, field.inv(den)));
                }
                if num != 0 {
                    return None;
                }
                (
                    self.at_finite_point(&self.numerator, x, y),
                    self.at_finite_point(&self.denominator, x, y),
                )
            }
        };

        match numerator.0.cmp(&denominator.0) {
            std::cmp::Ordering::Less => None,
            std::cmp::Ordering::Greater => Some(0),
            std::cmp::Ordering::Equal => Some(field.mul(numerator.1, field.inv(denominator.1))),
        }
    }

    fn at_infinity(&self, [a, b]: &Pair) -> (i64, u64) {
        let terms = [(a, 0), (b, self.curve.f.degree())];
        terms
            .into_iter()
            .filter(|(poly, _)| poly.degree() >= 0)
            .map(|(poly, y_weight)| {
                let degree = poly.degree();
                (-2 * degree - y_weight, poly.coeff(degree as usize))
            })
            .min_by_key(|(order, _)| *order)
            .expect("the map numerator and denominator are nonzero")
    }

    fn at_finite_point(&self, pair: &Pair, x: u64, y: u64) -> (i64, u64) {
        let field = self.curve.field;
        let [a, b] = pair.each_ref().map(|poly| poly.shift(x));
        if y == 0 {
            // The local parameter is y, and x-x₀ starts with y²/F'(x₀).
            let inverse = field.inv(self.curve.derivative.eval(x));
            return [&a, &b]
                .into_iter()
                .enumerate()
                .filter_map(|(parity, poly)| {
                    (0..=poly.degree()).find_map(|order| {
                        let coefficient = poly.coeff(order as usize);
                        (coefficient != 0).then(|| {
                            (
                                2 * order + parity as i64,
                                field.mul(coefficient, field.pow(inverse, order as usize)),
                            )
                        })
                    })
                })
                .min_by_key(|(order, _)| *order)
                .expect("a nonzero function has a leading term");
        }

        // At an unramified point x-x₀ is a local parameter. Its order in a
        // nonzero function is at most the degree of that function's norm.
        let shifted_f = self.curve.f.shift(x);
        let bound = norm(self.curve, pair).degree() as usize;
        let inverse = field.inv(field.mul(2, y));
        let mut series = vec![y];
        for order in 1..=bound {
            let cross = (1..order).fold(0, |sum, i| {
                field.add(sum, field.mul(series[i], series[order - i]))
            });
            series.push(field.mul(field.add(shifted_f.coeff(order), field.neg(cross)), inverse));
            let leading = series
                .iter()
                .enumerate()
                .fold(a.coeff(order), |sum, (i, &term)| {
                    field.add(sum, field.mul(term, b.coeff(order - i)))
                });
            if leading != 0 {
                return (order as i64, leading);
            }
        }
        unreachable!("valuation is bounded by the degree of the norm")
    }

    fn lift_fiber(
        &self,
        target: Option<u64>,
        roots: &[u64],
        index: Option<&PointIndex>,
    ) -> BTreeSet<Point> {
        let mut points = BTreeSet::new();
        if self.value(None) == target {
            points.insert(None);
        }

        // A fixed x can still support a moving point after cancellation.
        for &x in roots.iter().chain(&self.fixed_x) {
            let lifted: Vec<_>;
            let candidates: &[Point] = match index {
                Some(index) => index.get(&x).map(Vec::as_slice).unwrap_or(&[]),
                None => {
                    lifted = self.curve.points_above(x).collect();
                    &lifted
                }
            };
            for &point in candidates {
                if self.projection.is_some() || self.value(point) == target {
                    points.insert(point);
                }
            }
        }
        points
    }
}

fn norm(curve: &Curve, [a, b]: &Pair) -> Poly {
    a.mul(a).sub(&curve.f.mul(&b.mul(b)))
}

fn measure_map<'a>(
    curve: &Curve,
    spec: &'a MapSpec,
    targets: &[Option<u64>],
    index: Option<&PointIndex>,
) -> Result<MapReport<'a>> {
    let mapping = CurveMap::new(curve, spec)?;
    let infinity_image = mapping.value(None);
    let oracle = index.map(|index| {
        let mut fibers = BTreeMap::from([(infinity_image, BTreeSet::from([None]))]);
        for &point in index.values().flatten() {
            fibers
                .entry(mapping.value(point))
                .or_default()
                .insert(point);
        }
        fibers
    });
    let mut fibers = Vec::with_capacity(targets.len());
    let mut histogram = BTreeMap::new();
    let mut reflected_fibers = 0;

    for &target in targets {
        let polynomial = mapping.fiber_polynomial(target);
        let run = roots_of(&polynomial, true, 0)?;
        if run.reflection_center.is_some() {
            let baseline = roots_of(&polynomial, false, 0)?;
            ensure!(
                run.roots == baseline.roots,
                "root method disagreement at {target:?}"
            );
            reflected_fibers += 1;
        }

        let fiber = mapping.lift_fiber(target, &run.roots, index);
        if let Some(oracle) = &oracle {
            let matches = oracle
                .get(&target)
                .map_or_else(|| fiber.is_empty(), |expected| *expected == fiber);
            ensure!(matches, "fiber mismatch at {target:?}");
        }

        *histogram.entry(fiber.len()).or_default() += 1;
        let degree = polynomial.degree();
        fibers.push(FiberReport {
            target,
            specialized_norm_degree: if mapping.projection.is_some() {
                2 * degree
            } else {
                degree
            },
            elimination_degree: degree,
            reflection_extraction_degree: run.extraction_degree,
            reflection_center: run.reflection_center,
            distinct_elimination_roots: run.roots.len(),
            rational_points: fiber.len(),
        });
    }

    Ok(MapReport {
        name: &spec.name,
        numerator: &spec.numerator,
        denominator: &spec.denominator,
        raw_norm_degree: mapping.raw_degree,
        fixed_norm_degree: mapping.fixed_degree,
        map_degree: mapping.degree,
        separable: mapping.separable(),
        factors_through_x: mapping.projection.is_some(),
        infinity_image,
        verified_by_curve_enumeration: index.is_some(),
        fiber_size_histogram: histogram,
        reflected_fibers,
        fibers,
    })
}

/// Profile the configured maps and verify measured fibers by enumeration on small fields.
pub fn measure(config: &Config, workers: usize) -> Result<Report<'_>> {
    let p = config.p;
    let field = PrimeField::new(p)?;
    let curve = Curve::new(field, &config.curve)?;
    let genus = curve.genus();
    let targets = match &config.targets {
        Some(targets) => {
            let mut seen = BTreeSet::new();
            targets
                .iter()
                .map(|target| target.map(|t| field.reduce(t)))
                .filter(|&target| seen.insert(target))
                .collect::<Vec<_>>()
        }
        None => {
            ensure!(
                p <= ENUMERATION_LIMIT,
                "specify targets when p exceeds {ENUMERATION_LIMIT}"
            );
            (0..p).map(Some).chain([None]).collect()
        }
    };

    let index = (p <= ENUMERATION_LIMIT).then(|| {
        (0..p)
            .filter_map(|x| {
                let points: Vec<_> = curve.points_above(x).collect();
                (!points.is_empty()).then_some((x, points))
            })
            .collect::<PointIndex>()
    });
    let order = index
        .as_ref()
        .map(|index| 1 + index.values().map(Vec::len).sum::<usize>());
    let prime_curve_order = order
        .filter(|_| genus == 1)
        .map(|n| PrimeField::new(n as u64).is_ok());
    let maps = run_jobs(&config.maps, workers, |spec| {
        let curve = Curve::new(field, &config.curve)?;
        measure_map(&curve, spec, &targets, index.as_ref())
            .with_context(|| format!("map {}", spec.name))
    })?;

    Ok(Report {
        p,
        curve: &config.curve,
        genus,
        rational_point_count: order,
        prime_curve_order,
        exhaustive_targets: targets.len() as u128 == u128::from(p) + 1,
        maps,
    })
}
