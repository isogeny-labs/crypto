//! Small, dependency-free verification pieces for E/F_p: y²=x³−3x+40.
//! This crate does not generate relations or solve discrete logs by itself.

pub const P: u64 = (1_u64 << 61) - 1;
pub const R: u64 = 329_406_144_237_565_159;
pub const COFACTOR: u64 = 7;
pub const ROOT_ORDER: u64 = 4_950;
pub const ROOT_GENERATOR: u64 = 636_328_216_386_749_903;

pub mod sparse_wiedemann;

#[derive(Clone, Copy, Debug)]
pub struct Curve {
    p: u64,
    a: u64,
    b: u64,
    subgroup_order: u64,
    cofactor: u64,
}

/// Arithmetic and subgroup parameters for the selected M61 b=40 curve.
pub const SELECTED: Curve = Curve {
    p: P,
    a: P - 3,
    b: 40,
    subgroup_order: R,
    cofactor: COFACTOR,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Point {
    Infinity,
    Affine { x: u64, y: u64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidPoint,
    InvalidTerm,
    InvalidRelation,
    InvalidDecomposition,
    WrongSubgroup,
    WrongRank,
    ZeroDenominator,
    VerificationFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Term {
    pub index: usize,
    /// Must be +1 or -1. Repeated indices represent larger coefficients.
    pub sign: i8,
}

fn add_mod(a: u64, b: u64, modulus: u64) -> u64 {
    debug_assert!(a < modulus && b < modulus && modulus <= P);
    let sum = a + b; // sum < 2p < 2^62 for every supported curve
    if sum >= modulus {
        sum - modulus
    } else {
        sum
    }
}

fn sub_mod(a: u64, b: u64, modulus: u64) -> u64 {
    debug_assert!(a < modulus && b < modulus && modulus <= P);
    if a >= b {
        a - b
    } else {
        modulus - (b - a)
    }
}

fn mul_mod(a: u64, b: u64, modulus: u64) -> u64 {
    debug_assert!(a < modulus && b < modulus && modulus <= P);
    ((a as u128 * b as u128) % modulus as u128) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1_u64;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    result
}

// P and R are prime constants. Fermat inversion is used only on nonzero inputs.
fn inv_mod(value: u64, modulus: u64) -> Option<u64> {
    if value == 0 {
        None
    } else {
        Some(pow_mod(value, modulus - 2, modulus))
    }
}

impl Curve {
    /// Parameters are trusted to describe a nonsingular curve over a prime
    /// field, with prime subgroup order. Both fixed fixtures satisfy this.
    pub fn is_on_curve(self, point: Point) -> bool {
        match point {
            Point::Infinity => true,
            Point::Affine { x, y } if x < self.p && y < self.p => {
                let x2 = mul_mod(x, x, self.p);
                let x3 = mul_mod(x2, x, self.p);
                let rhs = add_mod(
                    add_mod(x3, mul_mod(self.a, x, self.p), self.p),
                    self.b,
                    self.p,
                );
                mul_mod(y, y, self.p) == rhs
            }
            _ => false,
        }
    }

    pub fn neg(self, point: Point) -> Point {
        match point {
            Point::Infinity => Point::Infinity,
            Point::Affine { x, y } if y < self.p => Point::Affine {
                x,
                y: if y == 0 { 0 } else { self.p - y },
            },
            // Preserve an invalid point so a later checked group operation
            // can reject it, without underflowing in this helper.
            invalid => invalid,
        }
    }

    pub fn add(self, left: Point, right: Point) -> Result<Point, Error> {
        if !self.is_on_curve(left) || !self.is_on_curve(right) {
            return Err(Error::InvalidPoint);
        }
        let (x1, y1, x2, y2) = match (left, right) {
            (Point::Infinity, q) => return Ok(q),
            (q, Point::Infinity) => return Ok(q),
            (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => (x1, y1, x2, y2),
        };
        if x1 == x2 && add_mod(y1, y2, self.p) == 0 {
            return Ok(Point::Infinity);
        }
        let (numerator, denominator) = if x1 == x2 {
            if y1 != y2 {
                return Err(Error::InvalidPoint);
            }
            (
                add_mod(mul_mod(3, mul_mod(x1, x1, self.p), self.p), self.a, self.p),
                add_mod(y1, y1, self.p),
            )
        } else {
            (sub_mod(y2, y1, self.p), sub_mod(x2, x1, self.p))
        };
        let inverse = inv_mod(denominator, self.p).ok_or(Error::InvalidPoint)?;
        let slope = mul_mod(numerator, inverse, self.p);
        let x3 = sub_mod(
            sub_mod(mul_mod(slope, slope, self.p), x1, self.p),
            x2,
            self.p,
        );
        let y3 = sub_mod(mul_mod(slope, sub_mod(x1, x3, self.p), self.p), y1, self.p);
        let result = Point::Affine { x: x3, y: y3 };
        debug_assert!(self.is_on_curve(result));
        Ok(result)
    }

    pub fn mul(self, mut scalar: u64, point: Point) -> Result<Point, Error> {
        if !self.is_on_curve(point) {
            return Err(Error::InvalidPoint);
        }
        let mut result = Point::Infinity;
        let mut power = point;
        while scalar != 0 {
            if scalar & 1 != 0 {
                result = self.add(result, power)?;
            }
            scalar >>= 1;
            if scalar != 0 {
                power = self.add(power, power)?;
            }
        }
        Ok(result)
    }

    pub fn sum_terms(self, base: &[Point], terms: &[Term]) -> Result<Point, Error> {
        let mut sum = Point::Infinity;
        for term in terms {
            let point = *base.get(term.index).ok_or(Error::InvalidTerm)?;
            if !self.is_on_curve(point) || (term.sign != 1 && term.sign != -1) {
                return Err(Error::InvalidTerm);
            }
            sum = self.add(
                sum,
                if term.sign == 1 {
                    point
                } else {
                    self.neg(point)
                },
            )?;
        }
        Ok(sum)
    }
}

pub fn is_on_curve(point: Point) -> bool {
    SELECTED.is_on_curve(point)
}
pub fn neg(point: Point) -> Point {
    SELECTED.neg(point)
}
pub fn add(left: Point, right: Point) -> Result<Point, Error> {
    SELECTED.add(left, right)
}
pub fn mul(scalar: u64, point: Point) -> Result<Point, Error> {
    SELECTED.mul(scalar, point)
}
pub fn sum_terms(base: &[Point], terms: &[Term]) -> Result<Point, Error> {
    SELECTED.sum_terms(base, terms)
}

/// Canonical lifts of the 4950th-root x-coordinate base on the selected curve.
/// Both signs may be used in relations; only the smaller y is stored here.
pub fn selected_root_factor_base() -> Vec<Point> {
    let mut x = 1_u64;
    let mut base = Vec::new();
    for _ in 0..ROOT_ORDER {
        let x2 = mul_mod(x, x, P);
        let x3 = mul_mod(x2, x, P);
        let rhs = add_mod(sub_mod(x3, mul_mod(3, x, P), P), 40, P);
        if rhs == 0 {
            base.push(Point::Affine { x, y: 0 });
        } else if pow_mod(rhs, (P - 1) / 2, P) == 1 {
            let y = pow_mod(rhs, (P + 1) / 4, P); // P ≡ 3 (mod 4)
            base.push(Point::Affine { x, y: y.min(P - y) });
        }
        x = mul_mod(x, ROOT_GENERATOR, P);
    }
    debug_assert_eq!(x, 1);
    base
}

fn coefficients(base_len: usize, terms: &[Term], modulus: u64) -> Result<Vec<u64>, Error> {
    let mut row = vec![0_u64; base_len];
    for term in terms {
        let entry = row.get_mut(term.index).ok_or(Error::InvalidTerm)?;
        *entry = match term.sign {
            1 => add_mod(*entry, 1, modulus),
            -1 => sub_mod(*entry, 1, modulus),
            _ => return Err(Error::InvalidTerm),
        };
    }
    Ok(row)
}

fn dot(left: &[u64], right: &[u64], modulus: u64) -> u64 {
    debug_assert_eq!(left.len(), right.len());
    left.iter().zip(right).fold(0, |sum, (&a, &b)| {
        add_mod(sum, mul_mod(a, b, modulus), modulus)
    })
}

/// Return a nonzero null vector only when the row rank is exactly B−1.
fn one_dimensional_nullspace(
    mut matrix: Vec<Vec<u64>>,
    columns: usize,
    modulus: u64,
) -> Result<Vec<u64>, Error> {
    if columns == 0 || matrix.iter().any(|row| row.len() != columns) {
        return Err(Error::WrongRank);
    }
    let mut pivots = Vec::new();
    let mut pivot_row = 0;
    for column in 0..columns {
        let found = (pivot_row..matrix.len()).find(|&row| matrix[row][column] != 0);
        let Some(found) = found else { continue };
        matrix.swap(pivot_row, found);
        let inverse = inv_mod(matrix[pivot_row][column], modulus).ok_or(Error::WrongRank)?;
        for value in matrix[pivot_row].iter_mut().skip(column) {
            *value = mul_mod(*value, inverse, modulus);
        }
        for i in 0..matrix.len() {
            if i == pivot_row {
                continue;
            }
            let factor = matrix[i][column];
            if factor == 0 {
                continue;
            }
            for j in column..columns {
                matrix[i][j] = sub_mod(
                    matrix[i][j],
                    mul_mod(factor, matrix[pivot_row][j], modulus),
                    modulus,
                );
            }
        }
        pivots.push(column);
        pivot_row += 1;
        if pivot_row == matrix.len() {
            break;
        }
    }
    if pivots.len() != columns - 1 {
        return Err(Error::WrongRank);
    }
    let free = (0..columns)
        .find(|column| !pivots.contains(column))
        .ok_or(Error::WrongRank)?;
    let mut vector = vec![0_u64; columns];
    vector[free] = 1;
    for (row, &column) in pivots.iter().enumerate() {
        vector[column] = sub_mod(0, matrix[row][free], modulus);
    }
    Ok(vector)
}

/// Recover d with Q=[d]G from already-found projected relations and descents.
///
/// Every relation must sum to a point killed by [7]. The generator and target
/// terms need only equal G and Q after multiplication by [7]. The function
/// verifies these claims, enforces rank B−1 modulo R, and checks [d]G=Q.
impl Curve {
    pub fn recover_log(
        self,
        base: &[Point],
        relations: &[Vec<Term>],
        generator_terms: &[Term],
        target_terms: &[Term],
        generator: Point,
        target: Point,
    ) -> Result<u64, Error> {
        if base.is_empty() || base.iter().any(|&point| !self.is_on_curve(point)) {
            return Err(Error::InvalidPoint);
        }
        if generator == Point::Infinity
            || !self.is_on_curve(generator)
            || !self.is_on_curve(target)
            || self.mul(self.subgroup_order, generator)? != Point::Infinity
            || self.mul(self.subgroup_order, target)? != Point::Infinity
        {
            return Err(Error::WrongSubgroup);
        }
        let mut rows = Vec::with_capacity(relations.len());
        for relation in relations {
            let sum = self.sum_terms(base, relation)?;
            if self.mul(self.cofactor, sum)? != Point::Infinity {
                return Err(Error::InvalidRelation);
            }
            rows.push(coefficients(base.len(), relation, self.subgroup_order)?);
        }
        let generator_sum = self.sum_terms(base, generator_terms)?;
        let target_sum = self.sum_terms(base, target_terms)?;
        if self.mul(self.cofactor, self.add(generator_sum, self.neg(generator))?)?
            != Point::Infinity
            || self.mul(self.cofactor, self.add(target_sum, self.neg(target))?)? != Point::Infinity
        {
            return Err(Error::InvalidDecomposition);
        }
        let vector = one_dimensional_nullspace(rows, base.len(), self.subgroup_order)?;
        let generator_row = coefficients(base.len(), generator_terms, self.subgroup_order)?;
        let target_row = coefficients(base.len(), target_terms, self.subgroup_order)?;
        let denominator = dot(&generator_row, &vector, self.subgroup_order);
        let inverse = inv_mod(denominator, self.subgroup_order).ok_or(Error::ZeroDenominator)?;
        let d = mul_mod(
            dot(&target_row, &vector, self.subgroup_order),
            inverse,
            self.subgroup_order,
        );
        if self.mul(d, generator)? != target {
            return Err(Error::VerificationFailed);
        }
        Ok(d)
    }
}

pub fn recover_log(
    base: &[Point],
    relations: &[Vec<Term>],
    generator_terms: &[Term],
    target_terms: &[Term],
    generator: Point,
    target: Point,
) -> Result<u64, Error> {
    SELECTED.recover_log(
        base,
        relations,
        generator_terms,
        target_terms,
        generator,
        target,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_GENERATOR: Point = Point::Affine {
        x: 0,
        y: 313_662_643_971_843_112,
    };

    #[test]
    fn selected_curve_group_arithmetic() {
        assert!(is_on_curve(FULL_GENERATOR));
        assert_eq!(mul(7 * R, FULL_GENERATOR).unwrap(), Point::Infinity);
        assert_ne!(mul(R, FULL_GENERATOR).unwrap(), Point::Infinity);
        assert_eq!(
            mul(7, FULL_GENERATOR).unwrap(),
            Point::Affine {
                x: 241_376_504_258_976_030,
                y: 2_288_991_884_717_741_184,
            }
        );
        let g = mul(7, FULL_GENERATOR).unwrap();
        assert_eq!(add(g, neg(g)).unwrap(), Point::Infinity);
    }

    #[test]
    fn selected_root_base_matches_expected_count() {
        use std::collections::HashSet;

        assert_eq!(pow_mod(ROOT_GENERATOR, ROOT_ORDER, P), 1);
        for prime in [2, 3, 5, 11] {
            assert_ne!(pow_mod(ROOT_GENERATOR, ROOT_ORDER / prime, P), 1);
        }
        let base = selected_root_factor_base();
        assert_eq!(base.len(), 2_447);
        assert_eq!(
            base[0],
            Point::Affine {
                x: 1,
                y: 1_013_433_362_578_841_545
            }
        );
        let mut xs = HashSet::new();
        for point in base {
            assert!(is_on_curve(point));
            let Point::Affine { x, .. } = point else {
                unreachable!()
            };
            assert!(xs.insert(x));
            assert_eq!(pow_mod(x, ROOT_ORDER, P), 1);
        }
    }

    #[test]
    fn cofactor_projected_relation_recovers_log() {
        let g = mul(7, FULL_GENERATOR).unwrap();
        let torsion = mul(R, FULL_GENERATOR).unwrap();
        let with_cofactor = add(g, torsion).unwrap();
        let q = mul(2, g).unwrap();
        let base = [g, with_cofactor];
        let plus = |index| Term { index, sign: 1 };
        let minus = |index| Term { index, sign: -1 };
        let relation = vec![plus(1), minus(0)]; // sums to nonzero 7-torsion
        assert_ne!(sum_terms(&base, &relation).unwrap(), Point::Infinity);
        let d = recover_log(
            &base,
            &[relation],
            &[plus(0)],
            &[plus(0), plus(1)], // sums to Q plus a 7-torsion point
            g,
            q,
        )
        .unwrap();
        assert_eq!(d, 2);
    }

    #[test]
    fn fixed_toy_fixture_recovers_log_and_rejects_corruption() {
        let toy = Curve {
            p: 8191,
            a: 8188,
            b: 40,
            subgroup_order: 59,
            cofactor: 136,
        };
        let fixture = include_str!("../fixtures/toy_8191.txt");
        let mut raw = Vec::new();
        let mut projected = Vec::new();
        let mut relations: Vec<Vec<Term>> = Vec::new();
        for line in fixture.lines() {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() == 5 && parts[0].parse::<usize>().is_ok() {
                assert_eq!(parts[0].parse::<usize>().unwrap(), raw.len());
                let coordinates: Vec<u64> = parts[1..]
                    .iter()
                    .map(|item| item.parse::<u64>().unwrap())
                    .collect();
                raw.push(Point::Affine {
                    x: coordinates[0],
                    y: coordinates[1],
                });
                projected.push(Point::Affine {
                    x: coordinates[2],
                    y: coordinates[3],
                });
            } else if parts.len() == 8
                && parts[0].starts_with('R')
                && parts[0].as_bytes().get(1).is_some_and(u8::is_ascii_digit)
            {
                let row = parts[1..]
                    .iter()
                    .map(|item| {
                        let (index, sign) = item.split_once(':').unwrap();
                        Term {
                            index: index.parse().unwrap(),
                            sign: sign.parse().unwrap(),
                        }
                    })
                    .collect();
                relations.push(row);
            }
        }
        assert_eq!(raw.len(), 20);
        assert_eq!(relations.len(), 19);
        for (&point, &image) in raw.iter().zip(&projected) {
            assert!(toy.is_on_curve(point));
            assert_eq!(toy.mul(136, point).unwrap(), image);
        }
        for row in &relations {
            assert_eq!(toy.sum_terms(&raw, row).unwrap(), Point::Infinity);
        }

        let g = Point::Affine { x: 2228, y: 404 };
        let q = Point::Affine { x: 7742, y: 7694 };
        assert_eq!(toy.mul(17, g).unwrap(), q);
        // The fixture's descents are over [136]P_i and equal G and Q.
        // Since 136 mod 59 = 18 and 18^-1 mod 59 = 23, the same raw-point
        // coefficient lists are projected descents of [23]G and [23]Q.
        let raw_g = toy.mul(23, g).unwrap();
        let raw_q = toy.mul(23, q).unwrap();
        let generator_terms = [Term { index: 0, sign: -1 }, Term { index: 9, sign: -1 }];
        let target_terms = [Term { index: 1, sign: -1 }, Term { index: 7, sign: 1 }];
        assert_eq!(
            toy.recover_log(
                &raw,
                &relations,
                &generator_terms,
                &target_terms,
                raw_g,
                raw_q
            )
            .unwrap(),
            17
        );

        let mut projection_only = vec![Term { index: 0, sign: -1 }; 37];
        projection_only.push(Term { index: 1, sign: 1 });
        let sum = toy.sum_terms(&raw, &projection_only).unwrap();
        assert_ne!(sum, Point::Infinity);
        assert_eq!(toy.mul(136, sum).unwrap(), Point::Infinity);
        let mut with_projection_only = relations.clone();
        with_projection_only.push(projection_only);
        assert_eq!(
            toy.recover_log(
                &raw,
                &with_projection_only,
                &generator_terms,
                &target_terms,
                raw_g,
                raw_q
            )
            .unwrap(),
            17
        );

        let mut corrupt = relations;
        let term = corrupt[0].iter_mut().find(|term| term.index == 10).unwrap();
        term.sign = -1;
        assert_eq!(
            toy.recover_log(
                &raw,
                &corrupt,
                &generator_terms,
                &target_terms,
                raw_g,
                raw_q
            ),
            Err(Error::InvalidRelation)
        );
    }
}
