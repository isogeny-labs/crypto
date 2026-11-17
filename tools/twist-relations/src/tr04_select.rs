//! Exact finite candidate-word census for TR-04.
//!
//! The DP merges identical public selector states, while retaining the exact
//! multiplicity and summed work of the corresponding cold executions. Its own
//! elapsed time is not the runtime of an individual selector. Work is a vector
//! of declared primitives, never a scalar GAE or complete CPU price.

use crate::core::{Element, Parameters, Point};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod decimal_u128 {
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(value: &u128, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u128, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

pub fn increment(value: &mut u128, amount: u128) -> Result<(), String> {
    *value = value.checked_add(amount).ok_or("TR-04 work overflow")?;
    Ok(())
}

macro_rules! work_vector {
    ($($field:ident),+ $(,)?) => {
        #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Work {
            $(#[serde(with = "decimal_u128")] pub $field: u128,)+
        }
        impl Work {
            pub fn add_scaled(&mut self, other: &Self, scale: u128) -> Result<(), String> {
                let mut next = self.clone();
                $(next.$field = next.$field.checked_add(other.$field.checked_mul(scale)
                    .ok_or(concat!("TR-04 multiplication overflow: ", stringify!($field)))?)
                    .ok_or(concat!("TR-04 addition overflow: ", stringify!($field)))?;)+
                *self = next;
                Ok(())
            }
            pub fn accumulate(&mut self, other: &Self) -> Result<(), String> {
                self.add_scaled(other, 1)
            }
            pub fn scaled(&self, scale: u128) -> Result<Self, String> {
                let mut result = Self::default();
                result.add_scaled(self, scale)?;
                Ok(result)
            }
            pub fn add(&self, other: &Self) -> Result<Self, String> {
                let mut result = self.clone();
                result.accumulate(other)?;
                Ok(result)
            }
        }
    };
}

work_vector!(
    candidate_reads,
    random_draws,
    character_tests,
    square_roots,
    group_additions,
    field_adds,
    field_muls,
    field_reductions,
    field_inversions,
    field_exponent_bits,
    duplicate_checks,
    point_comparisons,
    coordinate_comparisons,
    table_lookups,
    table_inserts,
    table_collisions,
    linear_cells,
    linear_adds,
    linear_muls,
    linear_reductions,
    linear_inversions,
    linear_exponent_bits,
);

/// A reference arithmetic interpreter with explicit field-operation counts.
/// Point comparison, container lookup/insertion and RNG calls are abstract
/// primitives; allocator, cache, branch and RNG-internal costs remain unpriced.
pub struct Meter {
    pub params: Parameters,
    pub work: Work,
}

impl Meter {
    pub fn new(params: Parameters) -> Self {
        Self {
            params,
            work: Work::default(),
        }
    }
    fn canonical(&self, value: u64) -> Result<(), String> {
        if self.params.p < 3 || value >= self.params.p {
            return Err("TR-04 field operand is not canonical".into());
        }
        Ok(())
    }
    pub fn field_add(&mut self, a: u64, b: u64) -> Result<u64, String> {
        self.canonical(a)?;
        self.canonical(b)?;
        increment(&mut self.work.field_adds, 1)?;
        increment(&mut self.work.field_reductions, 1)?;
        Ok(((a as u128 + b as u128) % self.params.p as u128) as u64)
    }
    pub fn field_sub(&mut self, a: u64, b: u64) -> Result<u64, String> {
        self.canonical(a)?;
        self.canonical(b)?;
        increment(&mut self.work.field_adds, 1)?;
        increment(&mut self.work.field_reductions, 1)?;
        Ok(((a as u128 + self.params.p as u128 - b as u128) % self.params.p as u128) as u64)
    }
    pub fn field_mul(&mut self, a: u64, b: u64) -> Result<u64, String> {
        self.canonical(a)?;
        self.canonical(b)?;
        increment(&mut self.work.field_muls, 1)?;
        increment(&mut self.work.field_reductions, 1)?;
        Ok(((a as u128 * b as u128) % self.params.p as u128) as u64)
    }
    pub fn field_pow(&mut self, mut a: u64, mut exponent: u64) -> Result<u64, String> {
        self.canonical(a)?;
        let mut result = 1;
        while exponent != 0 {
            increment(&mut self.work.field_exponent_bits, 1)?;
            if exponent & 1 != 0 {
                result = self.field_mul(result, a)?;
            }
            exponent >>= 1;
            if exponent != 0 {
                a = self.field_mul(a, a)?;
            }
        }
        Ok(result)
    }
    pub fn field_inverse(&mut self, a: u64) -> Result<u64, String> {
        self.canonical(a)?;
        if a == 0 {
            return Err("TR-04 attempted inversion of zero".into());
        }
        increment(&mut self.work.field_inversions, 1)?;
        self.field_pow(a, self.params.p - 2)
    }
    pub fn f(&mut self, x: u64) -> Result<u64, String> {
        let x2 = self.field_mul(x, x)?;
        let xa = self.field_add(x2, self.params.a)?;
        let cubic = self.field_mul(x, xa)?;
        self.field_add(cubic, self.params.b)
    }
    pub fn character(&mut self, x: u64) -> Result<i64, String> {
        increment(&mut self.work.character_tests, 1)?;
        let value = self.f(x)?;
        let chi = self.field_pow(value, (self.params.p - 1) / 2)?;
        increment(&mut self.work.coordinate_comparisons, 1)?;
        Ok(if chi == self.params.p - 1 {
            -1
        } else {
            chi as i64
        })
    }
    pub fn point(&mut self, x: u64) -> Result<Element, String> {
        let chi = self.character(x)?;
        increment(&mut self.work.coordinate_comparisons, 1)?;
        if chi != 1 {
            return Ok(None);
        }
        self.point_known_square(x).map(Some)
    }
    pub fn point_known_square(&mut self, x: u64) -> Result<Point, String> {
        if self.params.p % 4 != 3 {
            return Err("TR-04 square root requires p = 3 mod 4".into());
        }
        increment(&mut self.work.square_roots, 1)?;
        let value = self.f(x)?;
        let y = self.field_pow(value, (self.params.p + 1) / 4)?;
        let check = self.field_mul(y, y)?;
        increment(&mut self.work.coordinate_comparisons, 1)?;
        if check != value {
            return Err("TR-04 square-root check failed".into());
        }
        let negative = self.field_sub(0, y)?;
        increment(&mut self.work.coordinate_comparisons, 1)?;
        Ok(Point {
            x,
            y: y.min(negative),
        })
    }
    pub fn neg(&mut self, point: Element) -> Result<Element, String> {
        point
            .map(|q| self.field_sub(0, q.y).map(|y| Point { x: q.x, y }))
            .transpose()
    }
    pub fn add(&mut self, left: Element, right: Element) -> Result<Element, String> {
        increment(&mut self.work.group_additions, 1)?;
        let (Some(l), Some(r)) = (left, right) else {
            return Ok(left.or(right));
        };
        increment(&mut self.work.coordinate_comparisons, 1)?;
        let (num, den) = if l.x == r.x {
            let sum = self.field_add(l.y, r.y)?;
            increment(&mut self.work.coordinate_comparisons, 1)?;
            if sum == 0 {
                return Ok(None);
            }
            let x2 = self.field_mul(l.x, l.x)?;
            let triple = self.field_mul(3, x2)?;
            (
                self.field_add(triple, self.params.a)?,
                self.field_mul(2, l.y)?,
            )
        } else {
            (self.field_sub(r.y, l.y)?, self.field_sub(r.x, l.x)?)
        };
        let inv = self.field_inverse(den)?;
        let slope = self.field_mul(num, inv)?;
        let slope2 = self.field_mul(slope, slope)?;
        let x_left = self.field_sub(slope2, l.x)?;
        let x = self.field_sub(x_left, r.x)?;
        let difference = self.field_sub(l.x, x)?;
        let product = self.field_mul(slope, difference)?;
        let y = self.field_sub(product, l.y)?;
        Ok(Some(Point { x, y }))
    }
    pub fn mul(&mut self, mut scalar: u64, mut point: Element) -> Result<Element, String> {
        let mut result = None;
        while scalar != 0 {
            if scalar & 1 != 0 {
                result = self.add(result, point)?;
            }
            scalar >>= 1;
            if scalar != 0 {
                point = self.add(point, point)?;
            }
        }
        Ok(result)
    }
    pub fn same_point(&mut self, left: Element, right: Element) -> Result<bool, String> {
        increment(&mut self.work.point_comparisons, 1)?;
        match (left, right) {
            (None, None) => Ok(true),
            (Some(l), Some(r)) => {
                increment(&mut self.work.coordinate_comparisons, 1)?;
                if l.x != r.x {
                    return Ok(false);
                }
                increment(&mut self.work.coordinate_comparisons, 1)?;
                Ok(l.y == r.y)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PairEntry {
    pub point: Element,
    pub i: usize,
    pub j: usize,
}
#[derive(Clone, Debug, Default)]
pub struct PairTable {
    pub entries: Vec<PairEntry>,
}
impl PairTable {
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn lookup(
        &self,
        meter: &mut Meter,
        point: Element,
    ) -> Result<Option<(usize, usize)>, String> {
        increment(&mut meter.work.table_lookups, 1)?;
        for entry in &self.entries {
            if meter.same_point(entry.point, point)? {
                return Ok(Some((entry.i, entry.j)));
            }
        }
        Ok(None)
    }
}

/// Canonical first i<=j representative, using an explicitly metered linear
/// table. The same helper/order is shared by the collision control and finder.
pub fn pair_table(meter: &mut Meter, base: &[Point]) -> Result<PairTable, String> {
    let mut result = PairTable::default();
    for (i, &left) in base.iter().enumerate() {
        for (j, &right) in base.iter().enumerate().skip(i) {
            let point = meter.add(Some(left), Some(right))?;
            if result.lookup(meter, point)?.is_some() {
                increment(&mut meter.work.table_collisions, 1)?;
            } else {
                increment(&mut meter.work.table_inserts, 1)?;
                result.entries.push(PairEntry { point, i, j });
            }
        }
    }
    Ok(result)
}

pub fn reuses_pair_table(method: &str) -> bool {
    method == "collision_avoid"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Selected {
    pub base: Vec<Point>,
    #[serde(with = "decimal_u128")]
    pub ways: u128,
    pub work_sum: Work,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectionAudit {
    pub method: String,
    #[serde(with = "decimal_u128")]
    pub total_words: u128,
    pub outcomes: Vec<Selected>,
}

fn threshold(method: &str) -> Result<usize, String> {
    match method {
        "random" | "progression" | "collision_avoid" => Ok(0),
        "twist_l1" => Ok(1),
        "twist_l2" => Ok(2),
        "twist_l3" => Ok(3),
        "twist_l4" => Ok(4),
        _ => Err(format!("Unknown TR-04 selector: {method}")),
    }
}

fn candidate(
    meter: &mut Meter,
    base: &[Point],
    method: &str,
    x: u64,
) -> Result<Vec<Point>, String> {
    let chi = meter.character(x)?;
    increment(&mut meter.work.coordinate_comparisons, 1)?;
    if chi != 1 {
        return Ok(base.to_vec());
    }
    for j in 1..=threshold(method)? {
        let lookahead = meter.field_add(x, j as u64)?;
        let chi = meter.character(lookahead)?;
        increment(&mut meter.work.coordinate_comparisons, 1)?;
        if chi != -1 {
            return Ok(base.to_vec());
        }
    }
    increment(&mut meter.work.duplicate_checks, 1)?;
    for point in base {
        increment(&mut meter.work.coordinate_comparisons, 1)?;
        if point.x == x {
            return Ok(base.to_vec());
        }
    }
    let point = meter.point_known_square(x)?;
    let mut proposed = base.to_vec();
    let mut index = 0;
    while index < proposed.len() {
        increment(&mut meter.work.coordinate_comparisons, 1)?;
        if proposed[index].x > x {
            break;
        }
        index += 1;
    }
    increment(&mut meter.work.table_inserts, 1)?;
    proposed.insert(index, point);
    if method == "collision_avoid" {
        let table = pair_table(meter, &proposed)?;
        if table.len() != proposed.len() * (proposed.len() + 1) / 2 {
            return Ok(base.to_vec());
        }
    }
    Ok(proposed)
}

fn mask(base: &[Point]) -> u64 {
    base.iter().fold(0, |bits, point| bits | (1u64 << point.x))
}

fn merge(states: &mut BTreeMap<u64, Selected>, item: Selected) -> Result<(), String> {
    let key = mask(&item.base);
    if let Some(old) = states.get_mut(&key) {
        if old.base != item.base {
            return Err("TR-04 canonical state mismatch".into());
        }
        increment(&mut old.ways, item.ways)?;
        old.work_sum.accumulate(&item.work_sum)?;
    } else {
        states.insert(key, item);
    }
    Ok(())
}

/// Exact all-word census. Incomplete bases remain ordinary failed outcomes;
/// no finite-field feasibility oracle is passed into the production selector.
pub fn selection(
    params: Parameters,
    b: usize,
    method: &str,
    budget: usize,
) -> Result<SelectionAudit, String> {
    threshold(method)?;
    if !crate::core::prime(params.p)
        || params.p <= 3
        || params.p >= 64
        || params.p % 4 != 3
        || params.a >= params.p
        || params.b >= params.p
        || b == 0
        || b >= 64
        || !crate::core::Curve::new(params.p, params.a, params.b).nonsingular()
    {
        return Err(
            "TR-04 requires a nonsingular canonical prime-field curve, p=3 mod 4, 3<p<64, 0<B<64"
                .into(),
        );
    }
    let mut states = BTreeMap::new();
    let total_words;
    if method == "progression" {
        total_words = params.p as u128 * (params.p - 1) as u128;
        for start in 0..params.p {
            for step in 1..params.p {
                let mut meter = Meter::new(params);
                if budget != 0 {
                    increment(&mut meter.work.random_draws, 2)?;
                }
                let mut base = Vec::new();
                let mut x = start;
                for position in 0..budget {
                    increment(&mut meter.work.candidate_reads, 1)?;
                    base = candidate(&mut meter, &base, method, x)?;
                    if base.len() == b {
                        break;
                    }
                    if position + 1 < budget {
                        x = meter.field_add(x, step)?;
                    }
                }
                merge(
                    &mut states,
                    Selected {
                        base,
                        ways: 1,
                        work_sum: meter.work,
                    },
                )?;
            }
        }
    } else {
        total_words = (params.p as u128)
            .checked_pow(u32::try_from(budget).map_err(|_| "TR-04 budget overflow")?)
            .ok_or("TR-04 word count overflow")?;
        states.insert(
            0,
            Selected {
                base: Vec::new(),
                ways: 1,
                work_sum: Work::default(),
            },
        );
        for _ in 0..budget {
            let mut next = BTreeMap::new();
            for state in states.into_values() {
                if state.base.len() == b {
                    let ways = state
                        .ways
                        .checked_mul(params.p as u128)
                        .ok_or("TR-04 tail overflow")?;
                    merge(
                        &mut next,
                        Selected {
                            base: state.base,
                            ways,
                            work_sum: state.work_sum.scaled(params.p as u128)?,
                        },
                    )?;
                } else {
                    for x in 0..params.p {
                        let mut meter = Meter::new(params);
                        increment(&mut meter.work.candidate_reads, 1)?;
                        increment(&mut meter.work.random_draws, 1)?;
                        let base = candidate(&mut meter, &state.base, method, x)?;
                        let mut work_sum = state.work_sum.clone();
                        work_sum.add_scaled(&meter.work, state.ways)?;
                        merge(
                            &mut next,
                            Selected {
                                base,
                                ways: state.ways,
                                work_sum,
                            },
                        )?;
                    }
                }
            }
            states = next;
        }
    }
    let mut observed = 0;
    for state in states.values() {
        increment(&mut observed, state.ways)?;
    }
    if observed != total_words {
        return Err("TR-04 selector word census mismatch".into());
    }
    Ok(SelectionAudit {
        method: method.into(),
        total_words,
        outcomes: states.into_values().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const P: Parameters = Parameters { p: 7, a: 0, b: 3 };

    #[test]
    fn metered_group_law_matches_reference_on_all_tiny_points() {
        let mut reference = crate::core::Curve::new(7, 0, 3);
        let mut points = vec![None];
        for x in 0..7 {
            for y in 0..7 {
                let q = Some(Point { x, y });
                if reference.on_curve(q) {
                    points.push(q);
                }
            }
        }
        assert_eq!(points.len(), 13);
        let mut meter = Meter::new(P);
        for &left in &points {
            for &right in &points {
                assert_eq!(meter.add(left, right).unwrap(), reference.add(left, right));
            }
        }
        for x in 0..7 {
            assert_eq!(meter.point(x).unwrap(), reference.point(x, false));
        }
        assert!(meter.work.field_muls > 0 && meter.work.field_reductions > 0);
    }

    #[test]
    fn absorbing_dp_counts_unused_tails_without_paid_reads() {
        let audit = selection(P, 1, "random", 2).unwrap();
        let mut total = Work::default();
        let mut failed = 0;
        for state in audit.outcomes {
            total.accumulate(&state.work_sum).unwrap();
            if state.base.is_empty() {
                failed += state.ways;
            }
        }
        assert_eq!(audit.total_words, 49);
        assert_eq!(failed, 1);
        assert_eq!(total.candidate_reads, 56);
        assert_eq!(total.random_draws, 56);
        assert_eq!(total.character_tests, 56);
        assert_eq!(total.square_roots, 48);
    }

    #[test]
    fn compressed_dp_matches_explicit_candidate_words_including_cost() {
        for method in ["random", "twist_l1", "twist_l2", "collision_avoid"] {
            let mut expected = BTreeMap::new();
            for a in 0..7 {
                for b in 0..7 {
                    for c in 0..7 {
                        let mut meter = Meter::new(P);
                        let mut base = Vec::new();
                        for x in [a, b, c] {
                            increment(&mut meter.work.candidate_reads, 1).unwrap();
                            increment(&mut meter.work.random_draws, 1).unwrap();
                            base = candidate(&mut meter, &base, method, x).unwrap();
                            if base.len() == 2 {
                                break;
                            }
                        }
                        merge(
                            &mut expected,
                            Selected {
                                base,
                                ways: 1,
                                work_sum: meter.work,
                            },
                        )
                        .unwrap();
                    }
                }
            }
            let actual = selection(P, 2, method, 3).unwrap();
            assert_eq!(
                serde_json::to_value(actual.outcomes).unwrap(),
                serde_json::to_value(expected.into_values().collect::<Vec<_>>()).unwrap(),
                "{method}"
            );
        }
    }

    #[test]
    fn collision_control_matches_two_point_support_and_retains_its_cost() {
        let random = selection(P, 2, "random", 3).unwrap();
        let collision = selection(P, 2, "collision_avoid", 3).unwrap();
        for (r, c) in random.outcomes.iter().zip(&collision.outcomes) {
            assert_eq!(r.base, c.base);
            assert_eq!(r.ways, c.ways);
            assert!(c.work_sum.group_additions >= r.work_sum.group_additions);
            if c.base.len() == 2 {
                assert_eq!(pair_table(&mut Meter::new(P), &c.base).unwrap().len(), 3);
                assert!(c.work_sum.group_additions > 0);
            }
        }
    }

    #[test]
    fn checked_work_and_large_word_counts_keep_exact_decimal_json() {
        let mut work = Work {
            candidate_reads: u128::MAX,
            ..Work::default()
        };
        let old = work.clone();
        assert!(work
            .accumulate(&Work {
                candidate_reads: 1,
                ..Work::default()
            })
            .is_err());
        assert_eq!(work, old);
        let encoded = serde_json::to_value(&work).unwrap();
        assert_eq!(encoded["candidate_reads"], u128::MAX.to_string());
        assert_eq!(serde_json::from_value::<Work>(encoded).unwrap(), old);
        assert!(selection(P, 2, "random", 128).is_err());
        assert!(selection(P, 2, "unknown", 8).is_err());
        assert!(selection(Parameters { p: 7, a: 0, b: 0 }, 2, "random", 3).is_err());
    }
}
