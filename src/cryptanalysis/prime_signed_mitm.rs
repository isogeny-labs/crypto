//! Signed four-list zero-sum search over the native-width prime-field backend.
//!
//! This is **generic decomposition**: enumerate group sums, without a
//! summation-polynomial solver. With B points per list the left table stores
//! 2B² representations. Keep all collisions: equal sums can have different
//! scalar metadata. No timing or performance comparison is made here.

use std::collections::HashMap;
use std::ops::ControlFlow;

use super::prime_fast::{inv_mod_u64, FastCurve, Pt};
use super::residual_walk::is_prime_u64;

/// A finite point R, with optional recovery metadata R = uG + vQ.
/// Coordinates use the backend's Montgomery representation (`curve.point`).
#[derive(Clone, Copy, Debug)]
pub struct Record {
    pub point: Pt,
    pub u: u64,
    pub v: u64,
}

/// One entry from each supplied list; the first sign is always +1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Relation {
    pub indices: [usize; 4],
    pub signs: [i8; 4],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub left_entries: usize,
    pub table_keys: usize,
    pub right_pair_adds: u64,
    pub lookups: u64,
    pub matches: u64,
    pub trivial_rejected: u64,
    pub witness_adds: u64,
    pub relations: u64,
}

type Lists<'a> = [&'a [Record]; 4];

fn signed(curve: &FastCurve, point: Pt, sign: i8) -> Pt {
    if sign == 1 {
        point
    } else {
        curve.neg(point)
    }
}

fn records<'a>(lists: Lists<'a>, relation: &Relation) -> Option<[&'a Record; 4]> {
    if relation.signs[0] != 1 || relation.signs.iter().any(|&s| s != 1 && s != -1) {
        return None;
    }
    Some([
        lists[0].get(relation.indices[0])?,
        lists[1].get(relation.indices[1])?,
        lists[2].get(relation.indices[2])?,
        lists[3].get(relation.indices[3])?,
    ])
}

fn nontrivial(curve: &FastCurve, points: [&Record; 4], signs: [i8; 4]) -> bool {
    let mut coefficients = HashMap::<(u64, u64), i32>::new();
    for (record, sign) in points.into_iter().zip(signs) {
        let opposite = curve.neg(record.point);
        let key = (record.point.x, record.point.y.min(opposite.y));
        let coefficient = if record.point.y <= opposite.y {
            sign
        } else {
            -sign
        };
        *coefficients.entry(key).or_default() += i32::from(coefficient);
    }
    coefficients.values().any(|&c| c != 0)
}

fn valid_point(curve: &FastCurve, point: Pt) -> bool {
    point.x < curve.f.p && point.y < curve.f.p && curve.is_on_curve(point)
}

impl Relation {
    /// Replay all four terms. Reject malformed indices/signs and formal
    /// cancellation; this also works with overlapping input lists.
    pub fn check(&self, curve: &FastCurve, lists: Lists<'_>) -> bool {
        let Some(points) = records(lists, self) else {
            return false;
        };
        if points.iter().any(|r| !valid_point(curve, r.point))
            || !nontrivial(curve, points, self.signs)
        {
            return false;
        }
        points
            .into_iter()
            .zip(self.signs)
            .fold(None, |sum, (r, sign)| {
                curve.add(sum, Some(signed(curve, r.point, sign)))
            })
            .is_none()
    }

    /// Recover k from U*G + V*Q = O in a prime-order subgroup.
    /// Check subgroup membership, each selected record's metadata, the
    /// relation, V != 0, and the final equation kG = Q before returning k.
    /// Caller establishes the field and curve parameters used by FastCurve.
    pub fn recover(&self, curve: &FastCurve, g: Pt, q: Pt, lists: Lists<'_>) -> Option<u64> {
        let n = curve.n;
        if n <= 3
            || n >= (1 << 62)
            || !is_prime_u64(n)
            || !valid_point(curve, g)
            || !valid_point(curve, q)
            || curve.mul(Some(g), n).is_some()
            || curve.mul(Some(q), n).is_some()
            || !self.check(curve, lists)
        {
            return None;
        }
        let points = records(lists, self)?;
        let (mut u, mut v) = (0u64, 0u64);
        for (r, sign) in points.into_iter().zip(self.signs) {
            if curve.combine(g, q, r.u % n, r.v % n) != Some(r.point) {
                return None;
            }
            u = add_signed_mod(u, r.u, sign, n);
            v = add_signed_mod(v, r.v, sign, n);
        }
        if v == 0 {
            return None;
        }
        let k = (((n - u) as u128 * inv_mod_u64(v, n) as u128) % n as u128) as u64;
        (curve.mul(Some(g), k) == Some(q)).then_some(k)
    }
}

fn add_signed_mod(acc: u64, term: u64, sign: i8, n: u64) -> u64 {
    let term = term % n;
    let term = if sign == 1 { term } else { (n - term) % n };
    ((acc as u128 + term as u128) % n as u128) as u64
}

/// Enumerate all nontrivial signed zero-sum relations, up to global sign.
/// The callback can stop at the first useful relation with `Break(())`.
/// Empty lists return no relations. Infinity is a valid intermediate sum.
///
/// `max_left_entries` bounds stored pair representations, not total bytes.
/// It is checked before building the table. Time is O(B² + C) for balanced
/// lists, where C is the number of matching representations to inspect;
/// memory is O(B²). Caller supplies valid prime-field FastCurve parameters.
pub fn search(
    curve: &FastCurve,
    lists: Lists<'_>,
    max_left_entries: usize,
    mut visit: impl FnMut(Relation) -> ControlFlow<()>,
) -> Result<Counts, String> {
    if lists.iter().any(|list| list.is_empty()) {
        return Ok(Counts::default());
    }
    let entries = lists[0]
        .len()
        .checked_mul(lists[1].len())
        .and_then(|n| n.checked_mul(2))
        .ok_or("pair-table size overflow")?;
    if entries > max_left_entries {
        return Err(format!(
            "left table requires {entries} entries; limit is {max_left_entries}"
        ));
    }
    if lists
        .iter()
        .flat_map(|list| list.iter())
        .any(|r| !valid_point(curve, r.point))
    {
        return Err("factor-base entry is off curve or has unreduced coordinates".into());
    }
    let mut table = HashMap::<Option<Pt>, Vec<(usize, usize, i8)>>::new();
    for (i, first) in lists[0].iter().enumerate() {
        for (j, second) in lists[1].iter().enumerate() {
            for sign in [1, -1] {
                let sum = curve.add(Some(first.point), Some(signed(curve, second.point, sign)));
                table.entry(sum).or_default().push((i, j, sign));
            }
        }
    }
    let mut counts = Counts {
        left_entries: entries,
        table_keys: table.len(),
        ..Counts::default()
    };
    for (k, third) in lists[2].iter().enumerate() {
        for (l, fourth) in lists[3].iter().enumerate() {
            for fourth_sign in [1, -1] {
                let sum = curve.add(
                    Some(third.point),
                    Some(signed(curve, fourth.point, fourth_sign)),
                );
                counts.right_pair_adds += 1;
                for right_sign in [1, -1] {
                    let key = if right_sign == 1 {
                        sum.map(|p| curve.neg(p))
                    } else {
                        sum
                    };
                    counts.lookups += 1;
                    for &(i, j, second_sign) in table.get(&key).into_iter().flatten() {
                        counts.matches += 1;
                        let relation = Relation {
                            indices: [i, j, k, l],
                            signs: [1, second_sign, right_sign, right_sign * fourth_sign],
                        };
                        let points = [&lists[0][i], &lists[1][j], third, fourth];
                        if !nontrivial(curve, points, relation.signs) {
                            counts.trivial_rejected += 1;
                            continue;
                        }
                        counts.witness_adds += 4;
                        if !relation.check(curve, lists) {
                            return Err("pair-sum relation failed group-law replay".into());
                        }
                        counts.relations += 1;
                        if visit(relation).is_break() {
                            return Ok(counts);
                        }
                    }
                }
            }
        }
    }
    Ok(counts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn fixture() -> (FastCurve, Pt) {
        let c = FastCurve::new("signed-mitm-test", 17, 2, 2, 19);
        let g = c.point(5, 1);
        (c, g)
    }

    fn record(c: &FastCurve, g: Pt, scalar: u64) -> Record {
        Record {
            point: c.mul(Some(g), scalar).unwrap(),
            u: scalar,
            v: 0,
        }
    }

    #[test]
    fn exhaustive_signed_enumeration_with_collisions() {
        let (c, g) = fixture();
        for shift in 0..10 {
            let owned: [Vec<Record>; 4] = std::array::from_fn(|list| {
                (0..3)
                    .map(|i| record(&c, g, (1 + (shift + list * 3 + i) % 18) as u64))
                    .collect()
            });
            let lists = owned.each_ref().map(|v| v.as_slice());
            let mut expected = BTreeSet::new();
            for i in 0..3 {
                for j in 0..3 {
                    for k in 0..3 {
                        for l in 0..3 {
                            for s2 in [1, -1] {
                                for s3 in [1, -1] {
                                    for s4 in [1, -1] {
                                        let r = Relation {
                                            indices: [i, j, k, l],
                                            signs: [1, s2, s3, s4],
                                        };
                                        // Independent oracle in Z/19Z, not the pair-table algorithm
                                        // or Relation::check, with formal cancellation by sign class.
                                        let scalars = std::array::from_fn::<_, 4, _>(|z| {
                                            lists[z][r.indices[z]].u
                                        });
                                        let total: i64 = scalars
                                            .iter()
                                            .zip(r.signs)
                                            .map(|(&a, s)| a as i64 * i64::from(s))
                                            .sum();
                                        let mut formal = [0i32; 10];
                                        for (&a, s) in scalars.iter().zip(r.signs) {
                                            let canonical = a.min(19 - a) as usize;
                                            formal[canonical] +=
                                                if a <= 9 { i32::from(s) } else { -i32::from(s) };
                                        }
                                        if total.rem_euclid(19) == 0
                                            && formal.iter().any(|&v| v != 0)
                                        {
                                            expected.insert(r);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            let mut actual = BTreeSet::new();
            let counts = search(&c, lists, 18, |r| {
                assert!(r.check(&c, lists));
                assert!(actual.insert(r));
                ControlFlow::Continue(())
            })
            .unwrap();
            assert_eq!(actual, expected, "shift={shift}");
            assert_eq!(counts.relations as usize, expected.len());
            assert_eq!(counts.left_entries, 18);
            assert_eq!(counts.lookups, 36);
        }
    }

    #[test]
    fn known_answer_recovery_and_early_stop() {
        // Same public known-answer input as the original standalone prototype.
        let c = FastCurve::new("signed-mitm-recovery-test", 100003, 0, 7, 99667);
        let g = c.point(2, 86328);
        let q = c.point(84377, 83230);
        let data = [
            (27235, 14003, 76414, 8108),
            (52857, 67236, 81134, 26995),
            (29422, 86121, 47393, 39291),
            (26061, 47441, 84820, 75752),
        ];
        let owned = data.map(|(x, y, u, v)| {
            vec![Record {
                point: c.point(x, y),
                u,
                v,
            }]
        });
        let lists = owned.each_ref().map(|v| v.as_slice());
        let mut recovered = None;
        let counts = search(&c, lists, 2, |r| {
            recovered = r.recover(&c, g, q, lists);
            if recovered.is_some() {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .unwrap();
        assert_eq!(recovered, Some(12345));
        assert_eq!(c.mul(Some(g), recovered.unwrap()), Some(q));
        assert_eq!(counts.relations, 1);
        assert!(counts.lookups <= 4);
        // Incorrect metadata is rejected even when the geometric relation holds.
        let mut bad = owned.clone();
        bad[0][0].v += 1;
        let r = Relation {
            indices: [0; 4],
            signs: [1, 1, 1, -1],
        };
        assert!(r.check(&c, lists));
        assert_eq!(
            r.recover(&c, g, q, bad.each_ref().map(|v| v.as_slice())),
            None
        );
    }

    #[test]
    fn continue_past_uninformative_colliding_representation() {
        let (c, g) = fixture();
        let q = c.mul(Some(g), 4).unwrap();
        let mut owned = [1, 2, 3, 6].map(|s| vec![record(&c, g, s)]);
        // Both records represent G, since 16G + Q = 20G = G.
        owned[0].push(Record {
            point: g,
            u: 16,
            v: 1,
        });
        let lists = owned.each_ref().map(|v| v.as_slice());
        let mut attempts = Vec::new();
        let counts = search(&c, lists, 4, |r| {
            let k = r.recover(&c, g, q, lists);
            attempts.push(k);
            if k.is_some() {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .unwrap();
        assert_eq!(attempts, vec![None, Some(4)]);
        assert_eq!(counts.relations, 2);
    }

    #[test]
    fn reject_trivial_empty_invalid_and_over_budget_inputs() {
        let (c, g) = fixture();
        let one = [record(&c, g, 1)];
        let lists = [&one[..]; 4];
        let counts = search(&c, lists, 2, |_| panic!("only formal cancellations")).unwrap();
        assert!(counts.trivial_rejected > 0);
        assert_eq!(counts.relations, 0);
        assert_eq!(
            search(&c, [&[]; 4], 0, |_| panic!()).unwrap(),
            Counts::default()
        );
        assert!(search(&c, lists, 1, |_| panic!()).is_err());
        let bad = [Record {
            point: c.point(1, 1),
            u: 0,
            v: 0,
        }];
        assert!(search(&c, [&bad[..]; 4], 2, |_| panic!()).is_err());
        let malformed = Relation {
            indices: [9; 4],
            signs: [1; 4],
        };
        assert!(!malformed.check(&c, lists));
        assert_eq!(malformed.recover(&c, g, g, lists), None);
    }

    #[test]
    fn uninformative_metadata_and_infinity_intermediate() {
        let (c, g) = fixture();
        let mut r = record(&c, g, 1);
        r.v = 0;
        let lists = [std::slice::from_ref(&r); 4];
        // Two inverse pairs meet at infinity, but are rejected as trivial.
        let counts = search(&c, lists, 2, |_| panic!()).unwrap();
        assert!(counts.matches > 0);
        let relation = Relation {
            indices: [0; 4],
            signs: [1, -1, 1, -1],
        };
        assert_eq!(relation.recover(&c, g, g, lists), None);
        // Nontrivial 1+2+3-6 relation with consistent metadata but V=0.
        let owned = [1, 2, 3, 6].map(|s| vec![record(&c, g, s)]);
        let lists = owned.each_ref().map(|v| v.as_slice());
        let relation = Relation {
            indices: [0; 4],
            signs: [1, 1, 1, -1],
        };
        assert!(relation.check(&c, lists));
        assert_eq!(relation.recover(&c, g, g, lists), None);
    }
}
