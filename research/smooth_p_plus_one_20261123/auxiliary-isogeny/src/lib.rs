//! Public toy auxiliary curves and standard two-isogeny factor-base maps.
//! No relation solver, discrete-log recovery, or timing harness is included.

use std::collections::BTreeSet;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Field(u64);

impl Field {
    fn new(p: u64) -> Result<Self> {
        match p {
            7 | 31 | 127 | 8191 => Ok(Self(p)),
            _ => Err("unsupported public fixture prime".into()),
        }
    }
    fn n(self, x: i128) -> u64 {
        x.rem_euclid(self.0 as i128) as u64
    }
    fn add(self, x: u64, y: u64) -> u64 {
        self.n(x as i128 + y as i128)
    }
    fn sub(self, x: u64, y: u64) -> u64 {
        self.n(x as i128 - y as i128)
    }
    fn mul(self, x: u64, y: u64) -> u64 {
        (x as u128 * y as u128 % self.0 as u128) as u64
    }
    fn pow(self, mut x: u64, mut n: u64) -> u64 {
        let mut out = 1;
        while n != 0 {
            if n & 1 != 0 {
                out = self.mul(out, x);
            }
            x = self.mul(x, x);
            n >>= 1;
        }
        out
    }
    fn inverse(self, x: u64) -> Option<u64> {
        (x % self.0 != 0).then(|| self.pow(x, self.0 - 2))
    }
    fn sqrt(self, x: u64) -> Option<u64> {
        let y = self.pow(x, (self.0 + 1) / 4);
        (self.mul(y, y) == x).then(|| y.min(self.sub(0, y)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(Option<(u64, u64)>);

impl Point {
    pub fn infinity() -> Self {
        Self(None)
    }
    pub fn coordinates(self) -> Option<(u64, u64)> {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Curve {
    field: Field,
    a2: u64,
    a4: u64,
    a6: u64,
}

impl Curve {
    pub fn new(p: u64, a2: i64, a4: i64, a6: i64) -> Result<Self> {
        let f = Field::new(p)?;
        let (a, b, c) = (f.n(a2 as i128), f.n(a4 as i128), f.n(a6 as i128));
        let (aa, bb, cc) = (a as i128, b as i128, c as i128);
        if f.n(
            aa * aa * bb * bb - 4 * bb * bb * bb - 4 * aa * aa * aa * cc - 27 * cc * cc
                + 18 * aa * bb * cc,
        ) == 0
        {
            return Err("singular curve".into());
        }
        Ok(Self {
            field: f,
            a2: a,
            a4: b,
            a6: c,
        })
    }
    pub fn auxiliary(p: u64) -> Result<Self> {
        Self::new(p, 0, 1, 0)
    }
    pub fn prime(self) -> u64 {
        self.field.0
    }
    pub fn coefficients(self) -> [u64; 3] {
        [self.a2, self.a4, self.a6]
    }
    fn rhs(self, x: u64) -> u64 {
        let f = self.field;
        f.add(
            f.mul(f.add(f.mul(f.add(x, self.a2), x), self.a4), x),
            self.a6,
        )
    }
    pub fn contains(self, point: Point) -> bool {
        point.0.map_or(true, |(x, y)| {
            x < self.prime() && y < self.prime() && self.field.mul(y, y) == self.rhs(x)
        })
    }
    pub fn point(self, x: u64, y: u64) -> Result<Point> {
        let point = Point(Some((x, y)));
        self.contains(point)
            .then_some(point)
            .ok_or_else(|| "point outside curve".into())
    }
    pub fn add(self, left: Point, right: Point) -> Result<Point> {
        if !self.contains(left) || !self.contains(right) {
            return Err("point outside curve".into());
        }
        let (Some((x, y)), Some((u, v))) = (left.0, right.0) else {
            return Ok(if left.0.is_none() { right } else { left });
        };
        let f = self.field;
        if x == u && f.add(y, v) == 0 {
            return Ok(Point::infinity());
        }
        let (num, den) = if x == u {
            (
                f.n(3 * x as i128 * x as i128 + 2 * self.a2 as i128 * x as i128 + self.a4 as i128),
                f.add(y, y),
            )
        } else {
            (f.sub(v, y), f.sub(u, x))
        };
        let slope = f.mul(num, f.inverse(den).ok_or("nonunit addition denominator")?);
        let nx = f.n(slope as i128 * slope as i128 - self.a2 as i128 - x as i128 - u as i128);
        let ny = f.sub(f.mul(slope, f.sub(x, nx)), y);
        self.point(nx, ny)
    }
    pub fn multiply(self, mut n: u64, mut point: Point) -> Result<Point> {
        if !self.contains(point) {
            return Err("point outside curve".into());
        }
        let mut out = Point::infinity();
        while n != 0 {
            if n & 1 != 0 {
                out = self.add(out, point)?;
            }
            point = self.add(point, point)?;
            n >>= 1;
        }
        Ok(out)
    }
}

/// Polynomial coefficients are canonical and ordered from constant term up.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial {
    field: Field,
    coefficients: Vec<u64>,
}

impl Polynomial {
    fn new(f: Field, mut coefficients: Vec<u64>) -> Self {
        for x in &mut coefficients {
            *x %= f.0;
        }
        while coefficients.last() == Some(&0) {
            coefficients.pop();
        }
        Self {
            field: f,
            coefficients,
        }
    }
    pub fn coefficients(&self) -> &[u64] {
        &self.coefficients
    }
    pub fn degree(&self) -> Option<usize> {
        self.coefficients.len().checked_sub(1)
    }
    pub fn evaluate(&self, x: u64) -> u64 {
        self.coefficients
            .iter()
            .rev()
            .fold(0, |out, &a| self.field.add(self.field.mul(out, x), a))
    }
    fn plus(&self, other: &Self) -> Self {
        assert_eq!(self.field, other.field);
        let mut out = vec![0; self.coefficients.len().max(other.coefficients.len())];
        for (i, &a) in self.coefficients.iter().enumerate() {
            out[i] = a;
        }
        for (i, &a) in other.coefficients.iter().enumerate() {
            out[i] = self.field.add(out[i], a);
        }
        Self::new(self.field, out)
    }
    fn scale(&self, scalar: u64) -> Self {
        Self::new(
            self.field,
            self.coefficients
                .iter()
                .map(|&a| self.field.mul(a, scalar))
                .collect(),
        )
    }
    fn times(&self, other: &Self) -> Self {
        assert_eq!(self.field, other.field);
        if self.coefficients.is_empty() || other.coefficients.is_empty() {
            return Self::new(self.field, vec![]);
        }
        let mut out = vec![0; self.coefficients.len() + other.coefficients.len() - 1];
        for (i, &a) in self.coefficients.iter().enumerate() {
            for (j, &b) in other.coefficients.iter().enumerate() {
                out[i + j] = self.field.add(out[i + j], self.field.mul(a, b));
            }
        }
        Self::new(self.field, out)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RationalMap {
    numerator: Polynomial,
    denominator: Polynomial,
}

impl RationalMap {
    pub fn numerator(&self) -> &Polynomial {
        &self.numerator
    }
    pub fn denominator(&self) -> &Polynomial {
        &self.denominator
    }
    pub fn evaluate(&self, x: u64) -> Option<u64> {
        let f = self.numerator.field;
        f.inverse(self.denominator.evaluate(x))
            .map(|d| f.mul(self.numerator.evaluate(x), d))
    }
    fn compose_step(&self, step: &TwoIsogeny) -> Self {
        let f = self.numerator.field;
        let z = self
            .numerator
            .plus(&self.denominator.scale(f.sub(0, step.r)));
        Self {
            numerator: z
                .times(&z)
                .plus(&z.times(&self.denominator).scale(step.a))
                .plus(&self.denominator.times(&self.denominator).scale(step.b)),
            denominator: z.times(&self.denominator),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TwoIsogeny {
    source: Curve,
    target: Curve,
    r: u64,
    a: u64,
    b: u64,
}

impl TwoIsogeny {
    pub fn new(source: Curve, kernel_x: u64) -> Result<Self> {
        source.point(kernel_x, 0)?;
        let f = source.field;
        let r = kernel_x;
        let a = f.n(3 * r as i128 + source.a2 as i128);
        let b =
            f.n(3 * r as i128 * r as i128 + 2 * source.a2 as i128 * r as i128 + source.a4 as i128);
        if b == 0 {
            return Err("singular kernel chart".into());
        }
        let target = Curve::new(
            source.prime(),
            -2 * a as i64,
            a as i64 * a as i64 - 4 * b as i64,
            0,
        )?;
        Ok(Self {
            source,
            target,
            r,
            a,
            b,
        })
    }
    pub fn source(self) -> Curve {
        self.source
    }
    pub fn target(self) -> Curve {
        self.target
    }
    pub fn kernel_x(self) -> u64 {
        self.r
    }
    pub fn map_x(self, x: u64) -> Option<u64> {
        let f = self.source.field;
        let z = f.sub(x % f.0, self.r);
        f.inverse(z)
            .map(|inverse| f.add(f.add(z, self.a), f.mul(self.b, inverse)))
    }
    pub fn map_point(self, point: Point) -> Result<Point> {
        if !self.source.contains(point) {
            return Err("point outside isogeny domain".into());
        }
        let Some((x, y)) = point.0 else {
            return Ok(point);
        };
        let f = self.source.field;
        let Some(inverse) = f.inverse(f.sub(x, self.r)) else {
            return Ok(Point::infinity());
        };
        let u = self.map_x(x).ok_or("unexpected isogeny pole")?;
        let v = f.mul(y, f.sub(1, f.mul(self.b, f.mul(inverse, inverse))));
        self.target.point(u, v)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeneratorSearch {
    pub point: Point,
    pub x_values_examined: u64,
    pub order_checks: u64,
}

/// Deterministic setup search. These counters are not CPU-work measurements.
pub fn find_generator(prime: u64) -> Result<GeneratorSearch> {
    let curve = Curve::auxiliary(prime)?;
    let mut order_checks = 0;
    for x in 0..prime {
        if let Some(y) = curve.field.sqrt(curve.rhs(x)).filter(|&y| y != 0) {
            order_checks += 1;
            let point = curve.point(x, y)?;
            if curve.multiply((prime + 1) / 2, point)? != Point::infinity()
                && curve.multiply(prime + 1, point)? == Point::infinity()
            {
                return Ok(GeneratorSearch {
                    point,
                    x_values_examined: x + 1,
                    order_checks,
                });
            }
        }
    }
    Err("no full-order generator found".into())
}

#[derive(Clone, Debug)]
pub struct Chain {
    source: Curve,
    generator: Point,
    steps: Vec<TwoIsogeny>,
    rational_map: RationalMap,
    kernel_generator: Point,
}

impl Chain {
    pub fn new(prime: u64, generator: Point, depth: usize) -> Result<Self> {
        let source = Curve::auxiliary(prime)?;
        let n = prime + 1;
        if !(1..=6).contains(&depth) || (n >> depth) < 4 {
            return Err("depth must be 1..6 and leave image order at least four".into());
        }
        if !source.contains(generator)
            || source.multiply(n, generator)? != Point::infinity()
            || source.multiply(n / 2, generator)? == Point::infinity()
        {
            return Err("generator does not have full auxiliary order".into());
        }
        let f = source.field;
        let mut rational_map = RationalMap {
            numerator: Polynomial::new(f, vec![0, 1]),
            denominator: Polynomial::new(f, vec![1]),
        };
        let mut curve = source;
        let mut image = generator;
        let mut order = n;
        let mut steps = Vec::new();
        for _ in 0..depth {
            let kernel = curve.multiply(order / 2, image)?;
            let Some((r, 0)) = kernel.0 else {
                return Err("expected nonzero rational two-torsion".into());
            };
            let step = TwoIsogeny::new(curve, r)?;
            image = step.map_point(image)?;
            curve = step.target;
            order /= 2;
            if curve.multiply(order, image)? != Point::infinity()
                || curve.multiply(order / 2, image)? == Point::infinity()
            {
                return Err("unexpected image order".into());
            }
            rational_map = rational_map.compose_step(&step);
            steps.push(step);
        }
        let kernel_generator = source.multiply(n >> depth, generator)?;
        Ok(Self {
            source,
            generator,
            steps,
            rational_map,
            kernel_generator,
        })
    }
    pub fn source(&self) -> Curve {
        self.source
    }
    pub fn target(&self) -> Curve {
        self.steps.last().expect("nonempty chain").target
    }
    pub fn generator(&self) -> Point {
        self.generator
    }
    pub fn steps(&self) -> &[TwoIsogeny] {
        &self.steps
    }
    pub fn rational_map(&self) -> &RationalMap {
        &self.rational_map
    }
    pub fn degree(&self) -> usize {
        1 << self.steps.len()
    }
    pub fn map_point(&self, mut point: Point) -> Result<Point> {
        for step in &self.steps {
            point = step.map_point(point)?;
        }
        Ok(point)
    }
    pub fn map_x(&self, mut x: u64) -> Option<u64> {
        for step in &self.steps {
            x = step.map_x(x)?;
        }
        Some(x)
    }
    pub fn fiber(&self, anchor: Point) -> Result<Fiber> {
        let image = self.map_point(anchor)?;
        let Some((chi, y)) = image.0 else {
            return Err("fiber image is infinity".into());
        };
        if y == 0 {
            return Err("fiber image is two-torsion".into());
        }
        let f = self.source.field;
        let polynomial = self
            .rational_map
            .numerator
            .plus(&self.rational_map.denominator.scale(f.sub(0, chi)));
        let mut roots = BTreeSet::new();
        let mut point = anchor;
        for _ in 0..self.degree() {
            let Some((x, _)) = point.0 else {
                return Err("fiber contains infinity".into());
            };
            if !roots.insert(x) || polynomial.evaluate(x) != 0 || self.map_point(point)? != image {
                return Err("inconsistent fiber coset".into());
            }
            point = self.source.add(point, self.kernel_generator)?;
        }
        if point != anchor || polynomial.degree() != Some(self.degree()) {
            return Err("incorrect fiber degree or orbit".into());
        }
        Ok(Fiber {
            image,
            polynomial,
            x_coordinates: roots.into_iter().collect(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Fiber {
    image: Point,
    polynomial: Polynomial,
    x_coordinates: Vec<u64>,
}

impl Fiber {
    pub fn image(&self) -> Point {
        self.image
    }
    pub fn polynomial(&self) -> &Polynomial {
        &self.polynomial
    }
    pub fn x_coordinates(&self) -> &[u64] {
        &self.x_coordinates
    }
    /// One canonical sign, with target-curve two-torsion excluded.
    pub fn lift_to(&self, target: Curve) -> Result<Vec<Point>> {
        if target.field != self.polynomial.field {
            return Err("target field differs".into());
        }
        self.x_coordinates
            .iter()
            .filter_map(|&x| {
                target
                    .field
                    .sqrt(target.rhs(x))
                    .filter(|&y| y != 0)
                    .map(|y| target.point(x, y))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g7() -> (Curve, Point) {
        let e = Curve::auxiliary(7).unwrap();
        (e, e.point(3, 3).unwrap())
    }
    fn g31() -> (Curve, Point) {
        let e = Curve::auxiliary(31).unwrap();
        (e, e.point(6, 6).unwrap())
    }
    #[test]
    fn known_group_vectors_and_orders() {
        let (e, g) = g31();
        assert_eq!(e.multiply(2, g).unwrap(), e.point(7, 3).unwrap());
        assert_eq!(e.multiply(4, g).unwrap(), e.point(2, 17).unwrap());
        assert_eq!(e.multiply(16, g).unwrap(), e.point(0, 0).unwrap());
        assert_eq!(e.multiply(32, g).unwrap(), Point::infinity());
        assert_eq!(
            e.add(g, e.point(6, 25).unwrap()).unwrap(),
            Point::infinity()
        );
        assert_eq!(e.add(g, Point::infinity()).unwrap(), g);
    }
    #[test]
    fn deterministic_generator_search_has_explicit_counts() {
        let (_, g) = g7();
        let found = find_generator(7).unwrap();
        assert_eq!(found.point, g);
        assert_eq!(found.x_values_examined, 4);
        assert_eq!(found.order_checks, 2);
        let (_, g) = g31();
        let found = find_generator(31).unwrap();
        assert_eq!(found.point, g);
        assert_eq!(found.x_values_examined, 7);
        assert_eq!(found.order_checks, 3);
    }
    #[test]
    fn known_two_isogeny_vector_and_exceptional_points() {
        let (e, g) = g7();
        let step = TwoIsogeny::new(e, 0).unwrap();
        assert_eq!(step.target().coefficients(), [0, 3, 0]);
        assert_eq!(
            step.map_point(g).unwrap(),
            step.target().point(1, 5).unwrap()
        );
        assert_eq!(
            step.map_point(e.point(0, 0).unwrap()).unwrap(),
            Point::infinity()
        );
        assert_eq!(
            step.map_point(Point::infinity()).unwrap(),
            Point::infinity()
        );
        assert_eq!(step.map_x(0), None);
    }
    #[test]
    fn two_isogeny_respects_the_full_f7_group_law() {
        let (e, g) = g7();
        let step = TwoIsogeny::new(e, 0).unwrap();
        let points = (0..8)
            .map(|i| e.multiply(i, g).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(points.iter().copied().collect::<BTreeSet<_>>().len(), 8);
        for &p in &points {
            for &q in &points {
                assert_eq!(
                    step.map_point(e.add(p, q).unwrap()).unwrap(),
                    step.target()
                        .add(step.map_point(p).unwrap(), step.map_point(q).unwrap())
                        .unwrap()
                );
            }
        }
    }
    #[test]
    fn chain_uses_translated_kernel_and_dense_poles_agree() {
        let (_, g) = g31();
        let chain = Chain::new(31, g, 3).unwrap();
        assert_eq!(chain.steps()[1].kernel_x(), 2);
        assert_eq!(chain.rational_map().numerator().degree(), Some(8));
        assert_eq!(chain.rational_map().denominator().degree(), Some(7));
        let mut poles = 0;
        for x in 0..31 {
            assert_eq!(chain.map_x(x), chain.rational_map().evaluate(x));
            if chain.map_x(x).is_none() {
                poles += 1;
                assert_ne!(chain.rational_map().numerator().evaluate(x), 0);
            }
        }
        assert!(poles > 0);
        assert_eq!(
            chain
                .target()
                .multiply(4, chain.map_point(g).unwrap())
                .unwrap(),
            Point::infinity()
        );
    }
    #[test]
    fn coset_polynomial_equals_independent_root_product() {
        let (_, g) = g31();
        let chain = Chain::new(31, g, 3).unwrap();
        let fiber = chain.fiber(g).unwrap();
        let f = Field(31);
        let mut product = Polynomial::new(f, vec![1]);
        for &x in fiber.x_coordinates() {
            product = product.times(&Polynomial::new(f, vec![f.sub(0, x), 1]));
        }
        assert_eq!(fiber.x_coordinates().len(), 8);
        assert_eq!(&product, fiber.polynomial());
        for x in 0..31 {
            assert_eq!(
                fiber.polynomial().evaluate(x) == 0,
                fiber.x_coordinates().contains(&x)
            );
        }
        let mut corrupted = product.clone();
        corrupted.coefficients[0] = f.add(corrupted.coefficients[0], 1);
        assert!(fiber
            .x_coordinates()
            .iter()
            .all(|&x| corrupted.evaluate(x) != 0));
    }
    #[test]
    fn target_lifting_filters_nonresidues_and_uses_canonical_sign() {
        let (_, g) = g31();
        let fiber = Chain::new(31, g, 3).unwrap().fiber(g).unwrap();
        let target = Curve::new(31, 0, 1, 5).unwrap();
        let base = fiber.lift_to(target).unwrap();
        let enumerated = fiber
            .x_coordinates()
            .iter()
            .filter_map(|&x| {
                (1..=15)
                    .find(|&y| target.point(x, y).is_ok())
                    .map(|y| target.point(x, y).unwrap())
            })
            .collect::<Vec<_>>();
        assert_eq!(base, enumerated);
        assert!(base.len() < fiber.x_coordinates().len());
        let r = fiber.x_coordinates()[0];
        let with_two_torsion = Curve::new(31, 0, 0, -(r as i64).pow(3)).unwrap();
        assert!(with_two_torsion.point(r, 0).is_ok());
        assert!(fiber.lift_to(with_two_torsion).unwrap().iter().all(|p| p
            .coordinates()
            .unwrap()
            .0
            != r));
        assert!(fiber.lift_to(Curve::auxiliary(7).unwrap()).is_err());
    }
    #[test]
    fn rejects_invalid_curve_point_kernel_generator_and_depth() {
        assert!(Curve::auxiliary(13).is_err());
        assert!(Curve::new(7, 0, 0, 0).is_err());
        let (e, g) = g31();
        assert!(e.point(31, 0).is_err());
        assert!(e.point(1, 1).is_err());
        assert!(TwoIsogeny::new(e, 6).is_err());
        assert!(Chain::new(31, Point::infinity(), 1).is_err());
        assert!(Chain::new(31, e.multiply(2, g).unwrap(), 1).is_err());
        for depth in [0, 4, 7, usize::MAX] {
            assert!(Chain::new(31, g, depth).is_err());
        }
    }
    #[test]
    fn rejects_infinite_or_two_torsion_fiber_images() {
        let (e, g) = g31();
        let chain = Chain::new(31, g, 3).unwrap();
        assert!(chain.fiber(Point::infinity()).is_err());
        assert!(chain.fiber(e.multiply(4, g).unwrap()).is_err());
        assert!(chain.fiber(e.multiply(2, g).unwrap()).is_err());
        assert!(chain.fiber(Point(Some((1, 1)))).is_err());
    }
}
