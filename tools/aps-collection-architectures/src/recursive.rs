//! Two-level recursive decomposition with complete intermediate witnesses.
use crate::{key, valid_points, Affine, Fixture, FixtureId, PointKey, Work};
use std::collections::{BTreeMap, BTreeSet};

pub type Tuple = [usize; 4];

#[derive(Debug)]
pub struct PairTable {
    fixture: FixtureId,
    base: Vec<Affine>,
    pairs: BTreeMap<PointKey, Vec<[usize; 2]>>,
}
impl PairTable {
    pub fn build(f: Fixture, base: &[Affine], w: &mut Work) -> Result<Self, String> {
        if !valid_points(f, base) {
            return Err("invalid base point".into());
        }
        let mut pairs: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for i in 0..base.len() {
            for j in i + 1..base.len() {
                let sum = w.add(f, base[i], base[j]);
                pairs.entry(key(sum)).or_default().push([i, j]);
                w.pair_entries_built += 1;
            }
        }
        Ok(Self {
            fixture: f.id,
            base: base.to_vec(),
            pairs,
        })
    }
    pub fn query(
        &self,
        f: Fixture,
        base: &[Affine],
        target: Affine,
        w: &mut Work,
    ) -> Result<BTreeSet<Tuple>, String> {
        if self.fixture != f.id || self.base != base || !valid_points(f, &[target]) {
            return Err("replay fixture/base/target mismatch".into());
        }
        let mut out = BTreeSet::new();
        // All right pairs, not only those with a successful previous target.
        for k in 0..base.len() {
            for l in k + 1..base.len() {
                let sum = w.add(f, base[k], base[l]);
                let need = w.add(f, target, f.neg(sum));
                w.lookup_calls += 1;
                if let Some(lefts) = self.pairs.get(&key(need)) {
                    for &[i, j] in lefts {
                        w.join_candidates += 1;
                        if j < k {
                            let t = [i, j, k, l];
                            w.witness_checks += 1;
                            if sum_tuple(f, base, t) != target {
                                return Err("bad intermediate witness".into());
                            }
                            out.insert(t);
                        }
                    }
                }
            }
        }
        Ok(out)
    }
}

pub fn sum_tuple(f: Fixture, base: &[Affine], tuple: Tuple) -> Affine {
    tuple.iter().fold(None, |s, &i| f.curve.add(s, base[i]))
}

/// Exhaustive four-loop oracle, with no intermediate table or target lookup.
pub fn direct(f: Fixture, base: &[Affine], target: Affine, w: &mut Work) -> BTreeSet<Tuple> {
    let mut out = BTreeSet::new();
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            for k in j + 1..base.len() {
                for l in k + 1..base.len() {
                    w.direct_tuples += 1;
                    let mut sum = None;
                    for ix in [i, j, k, l] {
                        sum = w.add(f, sum, base[ix]);
                    }
                    if sum == target {
                        out.insert([i, j, k, l]);
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_cold_shared_matches_all_targets_in_both_groups() {
        for id in [FixtureId::P17, FixtureId::P101] {
            let f = Fixture::new(id);
            // Includes infinity, opposite pairs and repeated coordinates/sums.
            let base: Vec<_> = [0, 1, 2, 3, 4, 5, 6, f.order - 1]
                .map(|k| f.point(k))
                .into();
            let mut setup = Work::default();
            let shared = PairTable::build(f, &base, &mut setup).unwrap();
            assert_eq!(setup.pair_entries_built, 28);
            for k in 0..f.order {
                let q = f.point(k);
                let expected = direct(f, &base, q, &mut Work::default());
                let cold = PairTable::build(f, &base, &mut Work::default()).unwrap();
                assert_eq!(
                    cold.query(f, &base, q, &mut Work::default()).unwrap(),
                    expected
                );
                assert_eq!(
                    shared.query(f, &base, q, &mut Work::default()).unwrap(),
                    expected
                );
            }
        }
    }
    #[test]
    fn replay_rejects_changed_base_and_curve() {
        let f = Fixture::new(FixtureId::P17);
        let base: Vec<_> = (1..9).map(|k| f.point(k)).collect();
        let table = PairTable::build(f, &base, &mut Work::default()).unwrap();
        let mut changed = base.clone();
        changed.swap(0, 1);
        assert!(table
            .query(f, &changed, None, &mut Work::default())
            .is_err());
        assert!(table
            .query(
                Fixture::new(FixtureId::P101),
                &base,
                None,
                &mut Work::default()
            )
            .is_err());
    }
    #[test]
    fn duplicate_points_preserve_index_witnesses() {
        let f = Fixture::new(FixtureId::P17);
        let base = vec![f.point(1); 5];
        let q = f.point(4);
        let got = PairTable::build(f, &base, &mut Work::default())
            .unwrap()
            .query(f, &base, q, &mut Work::default())
            .unwrap();
        assert_eq!(got.len(), 5);
        assert_eq!(got, direct(f, &base, q, &mut Work::default()));
    }
}
