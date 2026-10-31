//! Draft, exact operation-price arithmetic. No file importer or completeness
//! inference: absent observations remain absent. This cannot qualify an experiment.
use std::collections::BTreeMap;

pub const MODEL: &str = "atomic-scalar-disjoint-logical/v1-draft";
pub mod disjoint;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Overflow,
    InconsistentInverseExpansion,
    InvalidModulus,
    Missing(&'static str),
    Duplicate(String),
    Unknown(String),
    ZeroUnit,
    InvalidCell,
}
fn add(a: u128, b: u128) -> Result<u128, Error> {
    a.checked_add(b).ok_or(Error::Overflow)
}
fn mul(a: u128, b: u128) -> Result<u128, Error> {
    a.checked_mul(b).ok_or(Error::Overflow)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub square_twice: u128,
    pub inversion: u128,
    pub native_tenths: u128,
}
pub fn grid() -> Vec<Cell> {
    let mut out = Vec::new();
    for square_twice in [0, 1, 2] {
        for inversion in [1, 10, 100, 1000] {
            for native_tenths in [0, 1, 10] {
                out.push(Cell {
                    square_twice,
                    inversion,
                    native_tenths,
                });
            }
        }
    }
    out
}
impl Cell {
    fn validate(self) -> Result<(), Error> {
        if [0, 1, 2].contains(&self.square_twice)
            && [1, 10, 100, 1000].contains(&self.inversion)
            && [0, 1, 10].contains(&self.native_tenths)
        {
            Ok(())
        } else {
            Err(Error::InvalidCell)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction {
    numerator: u128,
    denominator: u128,
}
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
impl Fraction {
    pub fn numerator(self) -> u128 {
        self.numerator
    }
    pub fn denominator(self) -> u128 {
        self.denominator
    }
    pub fn new(n: u128, d: u128) -> Result<Self, Error> {
        if d == 0 {
            return Err(Error::ZeroUnit);
        }
        let g = gcd(n, d);
        Ok(Self {
            numerator: n / g,
            denominator: d / g,
        })
    }
    pub fn plus(self, other: Self) -> Result<Self, Error> {
        let g = gcd(self.denominator, other.denominator);
        let a = other.denominator / g;
        let b = self.denominator / g;
        Self::new(
            add(mul(self.numerator, a)?, mul(other.numerator, b)?)?,
            mul(self.denominator, a)?,
        )
    }
    pub fn divided_by(self, other: Self) -> Result<Self, Error> {
        if other.numerator == 0 {
            return Err(Error::ZeroUnit);
        }
        let g = gcd(self.numerator, other.numerator);
        let h = gcd(self.denominator, other.denominator);
        Self::new(
            mul(self.numerator / g, other.denominator / h)?,
            mul(self.denominator / h, other.numerator / g)?,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    pub m: u128,
    pub s: u128,
    pub i: u128,
}
impl Field {
    fn twice_price(self, c: Cell) -> Result<u128, Error> {
        add(
            add(mul(2, self.m)?, mul(c.square_twice, self.s)?)?,
            mul(mul(2, c.inversion)?, self.i)?,
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InverseModel {
    ExpandedPencil,
    AtomicRho,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scalar {
    pub adds: u128,
    pub negations: u128,
    pub muls: u128,
    pub inversions: u128,
    pub model: InverseModel,
}
fn expanded_inverse_muls(modulus: u64, inversions: u128) -> Result<u128, Error> {
    if modulus <= 2 {
        return Err(Error::InvalidModulus);
    }
    let e = modulus - 2;
    mul(
        u128::from(64 - e.leading_zeros() + e.count_ones()),
        inversions,
    )
}
impl Scalar {
    pub fn atomic_events(self, r: u64) -> Result<u128, Error> {
        let m = match self.model {
            InverseModel::AtomicRho => self.muls,
            InverseModel::ExpandedPencil => self
                .muls
                .checked_sub(expanded_inverse_muls(r, self.inversions)?)
                .ok_or(Error::InconsistentInverseExpansion)?,
        };
        add(add(self.adds, self.negations)?, add(m, self.inversions)?)
    }
}
/// Explicit disjoint logical events. API calls, row-update summaries and allocator
/// bytes are excluded: adding them on top would double count or invent a price.
pub const LOGICAL_KEYS: [&str; 8] = [
    "field_adds",
    "field_bit_ops",
    "base_adds",
    "key_comparisons",
    "entry_visits",
    "elements_copied",
    "elements_initialized",
    "standalone_modular_reductions",
];
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Logical(BTreeMap<String, u128>);
impl Logical {
    pub fn from_named(entries: impl IntoIterator<Item = (String, u128)>) -> Result<Self, Error> {
        let mut m = BTreeMap::new();
        for (k, v) in entries {
            if !LOGICAL_KEYS.contains(&k.as_str()) {
                return Err(Error::Unknown(k));
            }
            if m.insert(k.clone(), v).is_some() {
                return Err(Error::Duplicate(k));
            }
        }
        Ok(Self(m))
    }
    pub fn total(&self) -> Result<u128, Error> {
        let mut total = 0;
        for k in LOGICAL_KEYS {
            total = add(total, *self.0.get(k).ok_or(Error::Missing(k))?)?;
        }
        Ok(total)
    }
}
/// These are ledger inclusion checks, not a replacement for independent review.
/// A false flag prevents total pricing even when the corresponding counter is zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coverage {
    pub outer_validation: bool,
    pub setup_transfer: bool,
    pub collection: bool,
    pub elimination: bool,
    pub source_verification: bool,
    pub adapter_overhead: bool,
    pub candidate_reference_semantics_matched: bool,
}
impl Coverage {
    fn check(self) -> Result<(), Error> {
        for (ok, name) in [
            (self.outer_validation, "outer_validation"),
            (self.setup_transfer, "setup_transfer"),
            (self.collection, "collection"),
            (self.elimination, "elimination"),
            (self.source_verification, "source_verification"),
            (self.adapter_overhead, "adapter_overhead"),
            (
                self.candidate_reference_semantics_matched,
                "candidate_reference_semantics_matched",
            ),
        ] {
            if !ok {
                return Err(Error::Missing(name));
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Counts {
    pub group: Field,
    pub extra_base_expanded_muls: u128,
    pub extra_base_inverses: u128,
    pub scalar: Scalar,
    pub logical: Logical,
    pub coverage: Coverage,
}
/// Caller must bind p/r to independently validated prime-domain metadata.
/// Result is an observed price under a DRAFT model, never a qualified run ratio.
pub fn price(
    counts: &Counts,
    p: u64,
    r: u64,
    source_add: Field,
    cell: Cell,
) -> Result<Fraction, Error> {
    cell.validate()?;
    counts.coverage.check()?;
    if p <= 2 || r <= 2 {
        return Err(Error::InvalidModulus);
    }
    let extra = counts
        .extra_base_expanded_muls
        .checked_sub(expanded_inverse_muls(p, counts.extra_base_inverses)?)
        .ok_or(Error::InconsistentInverseExpansion)?;
    let field = Field {
        m: add(counts.group.m, extra)?,
        s: counts.group.s,
        i: add(counts.group.i, counts.extra_base_inverses)?,
    };
    let unit = source_add.twice_price(cell)?;
    if unit == 0 {
        return Err(Error::ZeroUnit);
    }
    let logical = add(counts.logical.total()?, counts.scalar.atomic_events(r)?)?;
    Fraction::new(
        add(
            mul(10, field.twice_price(cell)?)?,
            mul(mul(cell.native_tenths, logical)?, unit)?,
        )?,
        mul(10, unit)?,
    )
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Verified,
    Capped,
    Failed,
    Missing,
}
#[derive(Debug, PartialEq, Eq)]
pub struct PairPrice {
    pub candidate: Option<Fraction>,
    pub reference: Option<Fraction>,
    pub ratio: Option<Fraction>,
}
/// Capped candidate cost and ratio stay null; retain its attempted raw ledgers
/// outside this value. Reference observations may remain independently available.
pub fn pair(
    candidate: (&Counts, Outcome),
    reference: (&Counts, Outcome),
    p: u64,
    r: u64,
    unit: Field,
    cell: Cell,
) -> Result<PairPrice, Error> {
    let c = if candidate.1 == Outcome::Verified {
        Some(price(candidate.0, p, r, unit, cell)?)
    } else {
        None
    };
    let q = if reference.1 == Outcome::Verified {
        Some(price(reference.0, p, r, unit, cell)?)
    } else {
        None
    };
    let ratio = match (c, q) {
        (Some(a), Some(b)) => Some(a.divided_by(b)?),
        _ => None,
    };
    Ok(PairPrice {
        candidate: c,
        reference: q,
        ratio,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn counts() -> Counts {
        Counts {
            group: Field { m: 12, s: 4, i: 2 },
            extra_base_expanded_muls: 0,
            extra_base_inverses: 0,
            scalar: Scalar {
                adds: 2,
                negations: 1,
                muls: 3,
                inversions: 1,
                model: InverseModel::AtomicRho,
            },
            logical: Logical::from_named(LOGICAL_KEYS.map(|x| (x.into(), 1))).unwrap(),
            coverage: Coverage {
                outer_validation: true,
                setup_transfer: true,
                collection: true,
                elimination: true,
                source_verification: true,
                adapter_overhead: true,
                candidate_reference_semantics_matched: true,
            },
        }
    }
    #[test]
    fn fixed_grid_and_hand_price() {
        let g = grid();
        assert_eq!(g.len(), 36);
        let c = Cell {
            square_twice: 1,
            inversion: 10,
            native_tenths: 1,
        };
        let q = price(&counts(), 101, 101, Field { m: 10, s: 0, i: 0 }, c).unwrap(); /* (12+2+20)/10 + (8+7)/10 = 49/10 */
        assert_eq!(
            q,
            Fraction {
                numerator: 49,
                denominator: 10
            }
        );
    }
    #[test]
    fn no_silent_missing_or_unknown_categories() {
        let c = counts();
        let mut missing = c.clone();
        missing.coverage.outer_validation = false;
        assert_eq!(
            price(&missing, 101, 101, c.group, grid()[0]),
            Err(Error::Missing("outer_validation"))
        );
        let mut entries: Vec<_> = LOGICAL_KEYS.map(|x| (x.to_string(), 1)).into();
        entries.pop();
        let l = Logical::from_named(entries).unwrap();
        assert_eq!(
            l.total(),
            Err(Error::Missing("standalone_modular_reductions"))
        );
        assert!(matches!(
            Logical::from_named([("map_calls".into(), 1)]),
            Err(Error::Unknown(_))
        ));
        assert!(matches!(
            Logical::from_named([("field_adds".into(), 1), ("field_adds".into(), 2)]),
            Err(Error::Duplicate(_))
        ));
    }
    #[test]
    fn inverse_models_match_but_reject_impossible_counts() {
        for r in [3, 5, 101, 34147, 18446744073709551557] {
            let mut e = r - 2;
            let mut expanded = 0;
            while e != 0 {
                if e & 1 != 0 {
                    expanded += 1
                }
                expanded += 1;
                e >>= 1;
            }
            let a = Scalar {
                adds: 2,
                negations: 1,
                muls: expanded + 3,
                inversions: 1,
                model: InverseModel::ExpandedPencil,
            };
            let b = Scalar {
                muls: 3,
                model: InverseModel::AtomicRho,
                ..a
            };
            assert_eq!(a.atomic_events(r), b.atomic_events(r));
            assert_eq!(
                Scalar {
                    muls: expanded - 1,
                    ..a
                }
                .atomic_events(r),
                Err(Error::InconsistentInverseExpansion)
            );
        }
    }
    #[test]
    fn failure_is_not_zero_cost_or_success_ratio() {
        let c = counts();
        let q = pair(
            (&c, Outcome::Capped),
            (&c, Outcome::Verified),
            101,
            101,
            c.group,
            grid()[0],
        )
        .unwrap();
        assert_eq!(q.candidate, None);
        assert_eq!(q.ratio, None);
        assert!(q.reference.is_some());
    }
    #[test]
    fn overflow_and_ratio_of_sums() {
        assert_eq!(Fraction::new(0, 0), Err(Error::ZeroUnit));
        assert_eq!(Fraction::new(1, 0), Err(Error::ZeroUnit));
        assert_eq!(
            Fraction::new(0, 1)
                .unwrap()
                .divided_by(Fraction::new(0, 1).unwrap()),
            Err(Error::ZeroUnit)
        );
        assert_eq!(
            Fraction::new(u128::MAX, 1)
                .unwrap()
                .plus(Fraction::new(1, 1).unwrap()),
            Err(Error::Overflow)
        );
        let c = Fraction::new(2, 1)
            .unwrap()
            .plus(Fraction::new(8, 1).unwrap())
            .unwrap();
        let r = Fraction::new(1, 1)
            .unwrap()
            .plus(Fraction::new(4, 1).unwrap())
            .unwrap();
        assert_eq!(
            c.divided_by(r).unwrap(),
            Fraction {
                numerator: 2,
                denominator: 1
            }
        );
        let mut x = counts();
        x.group.m = u128::MAX;
        assert_eq!(
            price(&x, 101, 101, Field { m: 1, s: 0, i: 0 }, grid()[0]),
            Err(Error::Overflow)
        );
    }
}

pub mod aggregate;
pub mod analyzer;
pub mod importer;
