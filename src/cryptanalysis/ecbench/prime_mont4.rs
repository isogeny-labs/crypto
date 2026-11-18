//! Four-limb public-input prime curves for counted generic ECDLP methods.
//!
//! The caller supplies a certified prime modulus and group order, as for
//! `PrimeExplicit`. This adapter checks representation bounds, nonsingularity,
//! Hasse eligibility, prime subgroup order, membership and `[r]G = O`; it does
//! not issue a primality or point-count certificate. Field coordinates stay
//! four words throughout the group law. Scalars for these generic arms fit u64.
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

use crate::cryptanalysis::curve_id::{self, CurveId};
use crate::cryptanalysis::ic_boundary::{CountedGroup, FieldOps, GroupOps};
use crate::cryptanalysis::residual_walk::is_prime_u64;
use crate::ecc::mont_vt::Mont;

type Fp = [u64; 4];

/// Canonical Montgomery residues; infinity has a unique representation.
/// Coordinates are private so an external point must pass `decode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: Fp,
    y: Fp,
    infinity: bool,
}

impl Point {
    const IDENTITY: Self = Self {
        x: [0; 4],
        y: [0; 4],
        infinity: true,
    };
}

#[derive(Clone)]
pub struct PrimeMont4 {
    pub p: BigUint,
    pub a: BigUint,
    pub b: BigUint,
    pub group_order: BigUint,
    pub r: u64,
    pub generator: Point,
    pub id: CurveId,
    field: Mont<4>,
    a_mont: Fp,
    b_mont: Fp,
    inv_exponent: BigUint,
}

fn decimal(s: &str) -> Result<BigUint, String> {
    // Unique decimal text; no signs, leading zeroes, exponents or whitespace.
    if s.is_empty() || (s.len() > 1 && s.starts_with('0')) || !s.bytes().all(|c| c.is_ascii_digit())
    {
        return Err("prime_mont4 requires canonical unsigned decimal strings".into());
    }
    BigUint::parse_bytes(s.as_bytes(), 10).ok_or_else(|| "invalid decimal integer".into())
}

impl PrimeMont4 {
    /// `p,a,b,N,gx,gy` are decimal strings. Prime `p` and exact `N` need
    /// external certificates; the representation and subgroup checks here
    /// cannot establish either. No coefficient or coordinate is reduced silently.
    pub fn build(values: [&str; 6], r: u64) -> Result<Self, String> {
        let [p, a, b, group_order, gx, gy] = values.map(decimal);
        let (p, a, b, group_order, gx, gy) = (p?, a?, b?, group_order?, gx?, gy?);
        if group_order.bits() > 256 || a.is_zero() || b.is_zero() {
            return Err("prime_mont4 currently requires an order below 2^256 and a,b nonzero; special automorphism models need a separate adapter".into());
        }
        if p < BigUint::from(5u8) || p.bits() > 256 || !p.bit(0) || a >= p || b >= p {
            return Err("prime_mont4 needs odd 5 <= p < 2^256 and canonical a,b < p".into());
        }
        if r < 5 || !is_prime_u64(r) || (&group_order % r) != BigUint::zero() {
            return Err("prime_mont4 needs prime r >= 5 dividing the supplied order".into());
        }
        // Also reject composite word-size moduli; wider primality is external.
        if p.to_u64().is_some_and(|p| !is_prime_u64(p)) {
            return Err("composite word-size field modulus".into());
        }
        let id = curve_id::prime(&p, &a, &b, &group_order)
            .ok_or("singular curve or supplied order outside Hasse interval")?;
        let field = Mont::<4>::new(&p).ok_or("four-limb field context")?;
        let a_mont = field.enter(&a).ok_or("noncanonical a")?;
        let b_mont = field.enter(&b).ok_or("noncanonical b")?;
        let inv_exponent = &p - 2u8;
        let mut curve = Self {
            p,
            a,
            b,
            group_order,
            r,
            generator: Point::IDENTITY,
            id,
            field,
            a_mont,
            b_mont,
            inv_exponent,
        };
        let generator = curve.point(&gx, &gy)?;
        let mut scratch = GroupOps::default();
        if !curve.mul(&mut scratch, generator, r).infinity {
            return Err("prime_mont4 generator does not have the supplied prime order".into());
        }
        curve.generator = generator;
        Ok(curve)
    }

    pub fn point(&self, x: &BigUint, y: &BigUint) -> Result<Point, String> {
        let point = Point {
            x: self.field.enter(x).ok_or("noncanonical x")?,
            y: self.field.enter(y).ok_or("noncanonical y")?,
            infinity: false,
        };
        if !self.on_curve(point) {
            return Err("point is not on prime_mont4 curve".into());
        }
        Ok(point)
    }

    pub fn decode(&self, coordinates: &[String; 2]) -> Result<Point, String> {
        let parse = |s: &str| {
            let digits = s
                .strip_prefix("0x")
                .ok_or("hex coordinate requires 0x prefix")?;
            if digits.is_empty()
                || (digits.len() > 1 && digits.starts_with('0'))
                || !digits
                    .bytes()
                    .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
            {
                return Err("noncanonical lower-case hex coordinate");
            }
            BigUint::parse_bytes(digits.as_bytes(), 16).ok_or("invalid hex coordinate")
        };
        self.point(&parse(&coordinates[0])?, &parse(&coordinates[1])?)
    }

    pub fn encode(&self, point: Point) -> Option<[String; 2]> {
        (!point.infinity).then(|| {
            [
                format!("0x{:x}", self.field.leave(&point.x)),
                format!("0x{:x}", self.field.leave(&point.y)),
            ]
        })
    }

    pub fn on_curve(&self, point: Point) -> bool {
        if point.infinity {
            return point == Point::IDENTITY;
        }
        let f = &self.field;
        f.mul(&point.y, &point.y)
            == f.add(
                &f.add(
                    &f.mul(&f.mul(&point.x, &point.x), &point.x),
                    &f.mul(&self.a_mont, &point.x),
                ),
                &self.b_mont,
            )
    }

    fn mul_field(&self, ops: &mut FieldOps, x: &Fp, y: &Fp) -> Fp {
        ops.muls += 1;
        self.field.mul(x, y)
    }
    fn square(&self, ops: &mut FieldOps, x: &Fp) -> Fp {
        ops.sqrs += 1;
        self.field.mul(x, x)
    }
    fn inverse(&self, ops: &mut FieldOps, x: &Fp) -> Fp {
        assert_ne!(*x, [0; 4], "nonzero field inverse");
        ops.invs += 1;
        // One semantic field inversion; the implementation's exponentiation
        // is not additionally reported as curve-formula multiplications.
        self.field.pow(x, &self.inv_exponent)
    }
    fn finish(&self, ops: &mut FieldOps, p: Point, q: Point, slope: Fp) -> Point {
        let x = self
            .field
            .sub(&self.field.sub(&self.square(ops, &slope), &p.x), &q.x);
        let y = self.field.sub(
            &self.mul_field(ops, &slope, &self.field.sub(&p.x, &x)),
            &p.y,
        );
        Point {
            x,
            y,
            infinity: false,
        }
    }
    fn double_raw(&self, ops: &mut FieldOps, p: Point) -> Point {
        if p.infinity || p.y == [0; 4] {
            return Point::IDENTITY;
        }
        let xx = self.square(ops, &p.x);
        let numerator = self.field.add(
            &self.field.add(&self.field.add(&xx, &xx), &xx),
            &self.a_mont,
        );
        let inverse = self.inverse(ops, &self.field.add(&p.y, &p.y));
        let slope = self.mul_field(ops, &numerator, &inverse);
        self.finish(ops, p, p, slope)
    }
}

impl CountedGroup for PrimeMont4 {
    type Elt = Point;
    fn identity(&self) -> Point {
        Point::IDENTITY
    }
    fn is_identity(&self, point: &Point) -> bool {
        point.infinity
    }
    fn add(&self, ops: &mut GroupOps, p: Point, q: Point) -> Point {
        ops.adds += 1;
        if p.infinity {
            return q;
        }
        if q.infinity {
            return p;
        }
        if p.x == q.x {
            return if p.y == q.y {
                self.double_raw(&mut ops.field_ops, p)
            } else {
                Point::IDENTITY
            };
        }
        let inverse = self.inverse(&mut ops.field_ops, &self.field.sub(&q.x, &p.x));
        let slope = self.mul_field(&mut ops.field_ops, &self.field.sub(&q.y, &p.y), &inverse);
        self.finish(&mut ops.field_ops, p, q, slope)
    }
    fn double(&self, ops: &mut GroupOps, p: Point) -> Point {
        ops.doubles += 1;
        self.double_raw(&mut ops.field_ops, p)
    }
    fn neg(&self, point: Point) -> Point {
        if point.infinity {
            point
        } else {
            Point {
                y: self.field.sub(&[0; 4], &point.y),
                ..point
            }
        }
    }
    fn key(&self, point: &Point) -> u64 {
        if point.infinity {
            return 0;
        }
        // Walk partition/hash only, never a full-width coordinate encoding.
        let mut key = 0x243f_6a88_85a3_08d3u64;
        for word in point.x.into_iter().chain(point.y) {
            key ^= word;
            key = key.wrapping_mul(0x9e37_79b9_7f4a_7c15);
            key ^= key >> 29;
        }
        key
    }
    fn counts_field_ops(&self) -> bool {
        true
    }
}

/// BSGS with complete Montgomery point keys in a deterministic B-tree.
/// This is a separate method: the older textbook method stores only u64 keys.
/// Tree allocation/comparison work is not priced as field/group operations.
pub fn bsgs(curve: &PrimeMont4, target: Point) -> super::generic::GenericOutcome {
    use std::collections::BTreeMap;
    let mut out = super::generic::GenericOutcome::default();
    let m = super::generic::ceil_sqrt(curve.r);
    let mut table = BTreeMap::new();
    let mut baby = curve.identity();
    for j in 0..m {
        table.entry(baby).or_insert(j);
        if j + 1 < m {
            baby = curve.add(&mut out.setup, baby, curve.generator);
        }
    }
    let stride = curve.mul(&mut out.setup, curve.generator, m);
    let back = curve.neg(stride);
    let mut giant = target;
    let mut queries = 0;
    for i in 0..=m {
        queries += 1;
        if let Some(&j) = table.get(&giant) {
            out.recovered = Some(
                ((u128::from(i) * u128::from(m) + u128::from(j)) % u128::from(curve.r)) as u64,
            );
            break;
        }
        if i < m {
            giant = curve.add(&mut out.search, giant, back);
        }
    }
    out.exhausted = out.recovered.is_none();
    out.counters.insert("inserts_uncharged".into(), m);
    out.counters.insert("lookups_uncharged".into(), queries);
    out.counters.insert(
        "full_point_key_bytes_stored_uncharged".into(),
        m * std::mem::size_of::<Point>() as u64,
    );
    // The caller/runner verifies the recovered scalar. B-tree node overhead,
    // allocation and exact comparison counts remain unpriced/unmeasured.
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecc::{field::FieldElement, point::Point as ReferencePoint};

    fn small() -> PrimeMont4 {
        PrimeMont4::build(["223", "74", "122", "205", "119", "198"], 41).unwrap()
    }

    fn reference_mul(curve: &PrimeMont4, mut scalar: u64) -> Option<[String; 2]> {
        let coordinates = curve.encode(curve.generator).unwrap();
        let coordinate = |s: &str| {
            FieldElement::new(
                BigUint::parse_bytes(s[2..].as_bytes(), 16).unwrap(),
                curve.p.clone(),
            )
        };
        let mut point = ReferencePoint::Affine {
            x: coordinate(&coordinates[0]),
            y: coordinate(&coordinates[1]),
        };
        let a = FieldElement::new(curve.a.clone(), curve.p.clone());
        let mut result = ReferencePoint::Infinity;
        // Explicit BigUint affine formulas, avoiding the reference's optional
        // Montgomery scalar-multiplication fast path.
        while scalar != 0 {
            if scalar & 1 != 0 {
                result = result.add(&point, &a);
            }
            scalar >>= 1;
            if scalar != 0 {
                point = point.double(&a);
            }
        }
        match result {
            ReferencePoint::Infinity => None,
            ReferencePoint::Affine { x, y } => {
                Some([format!("0x{:x}", x.value), format!("0x{:x}", y.value)])
            }
        }
    }

    #[test]
    fn small_group_law_and_exact_bsgs() {
        let curve = small();
        let mut ops = GroupOps::default();
        for scalar in 0..=curve.r {
            let point = curve.mul(&mut ops, curve.generator, scalar);
            assert_eq!(curve.encode(point), reference_mul(&curve, scalar));
            assert_eq!(
                curve.add(&mut ops, point, curve.neg(point)),
                curve.identity()
            );
            assert_eq!(curve.add(&mut ops, point, curve.identity()), point);
            if scalar > 0 && scalar < curve.r {
                let outcome = bsgs(&curve, point);
                assert_eq!(outcome.recovered, Some(scalar));
                assert!(outcome.setup.field_ops.invs > 0);
            }
        }
        assert!(ops.adds > 0 && ops.doubles > 0 && ops.scalar_mults > 0);
    }

    #[test]
    fn frozen_202_bit_source_scalars_match_biguint_affine() {
        let curve = PrimeMont4::build(
            [
                "4794168233664648458988367110514316682522236917173341621535751",
                "4794168233664648458988367110514316682522236917173341621535611",
                "784",
                "4794168233664648458988367110509937564696162391583477974939208",
                "603209182794240622510892080705207481693427956682909708617837",
                "1682738350748939452891832813322046124324576766734042502997852",
            ],
            1125023250121,
        )
        .unwrap();
        assert_eq!(curve.p.bits(), 202);
        let mut ops = GroupOps::default();
        for scalar in [0, 1, 2, 3, 17, 31, 65537, curve.r - 1, curve.r, curve.r + 1] {
            assert_eq!(
                curve.encode(curve.mul(&mut ops, curve.generator, scalar)),
                reference_mul(&curve, scalar)
            );
        }
    }

    #[test]
    fn canonical_and_order_guards() {
        let valid = ["223", "74", "122", "205", "119", "198"];
        for (slot, bad) in [
            (0, "0223"),
            (0, "224"),
            (0, "221"),
            (1, "223"),
            (1, "0"),
            (2, "0"),
            (3, "206"),
            (4, "223"),
            (4, "1"),
            (5, "1"),
        ] {
            let mut input = valid;
            input[slot] = bad;
            assert!(PrimeMont4::build(input, 41).is_err(), "slot {slot}: {bad}");
        }
        assert!(PrimeMont4::build(valid, 40).is_err());
        let curve = small();
        for value in [
            ["0x077", "0xc6"],
            ["0x77", "0xC6"],
            ["119", "198"],
            ["0xdf", "0xc6"],
        ] {
            assert!(curve.decode(&value.map(String::from)).is_err());
        }
        let encoded = curve.encode(curve.generator).unwrap();
        assert_eq!(curve.decode(&encoded).unwrap(), curve.generator);
    }

    #[test]
    fn frozen_source_vectors_match_pari() {
        let cases: serde_json::Value = serde_json::from_str(include_str!(
            "../../../research/prime_conductor_gap80_20261031/wide-harness-20261118/source-vectors.json"
        )).unwrap();
        for case in cases.as_array().unwrap() {
            let text = |name: &str| case[name].as_str().unwrap();
            let generator = case["generator"].as_array().unwrap();
            let curve = PrimeMont4::build(
                [
                    text("p"),
                    text("a"),
                    text("b"),
                    text("group_order"),
                    generator[0].as_str().unwrap(),
                    generator[1].as_str().unwrap(),
                ],
                text("r").parse().unwrap(),
            )
            .unwrap();
            let mut ops = GroupOps::default();
            for row in case["targets"].as_array().unwrap() {
                let scalar = row["scalar"].as_str().unwrap().parse().unwrap();
                let actual = curve
                    .encode(curve.mul(&mut ops, curve.generator, scalar))
                    .unwrap();
                for (actual, expected) in actual.iter().zip(row["point"].as_array().unwrap()) {
                    let value = BigUint::parse_bytes(&actual.as_bytes()[2..], 16).unwrap();
                    assert_eq!(value.to_string(), expected.as_str().unwrap());
                }
            }
        }
    }
}
