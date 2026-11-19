//! `rootfind-kmitm`: meet-in-the-middle root-finding for arity `m = 3..=12`.
//!
//! Full [`KSumTable`] stops at `m = 9` (u64 packing) and costs about
//! `2^{m−1}·C(|F|, m)` build additions. This oracle stores a table of
//! **left** = `⌈m/2⌉` sums and probes with every **right** = `⌊m/2⌋` sum
//! `S`, looking up `R − S`. Overlap between left and right summands is
//! rejected. Cost is about `2^{L−1}·C(A, L)` build plus
//! `2^{R−1}·C(A, R)` probe additions a target — drastic vs a full
//! `m`-sum table once `m ≥ 4`, and the only practical path for `m = 10..12`.
//!
//! At `m = 3` this is the same shape as `rootfind-subtract:m=3` (pair
//! table + one subtract). New id: do not change `ksum` or
//! `rootfind-subtract`. Classification: engineering (wall / build size);
//! charged group additions are the MITM's own (not equal to full `ksum`).

use std::collections::BTreeMap;

use super::plugins::table_counted;
use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    CountedGroup, FactorBase, GroupOps, KSumTable, OracleCounters, PairTable, PrimeCurve,
    PrimeInstance, PrimePoint, KSUM_MAX_K,
};

/// Largest arity this oracle accepts.
pub const KMITM_MAX_M: u32 = 12;

#[inline]
fn split_arity(m: u32) -> (u32, u32) {
    let right = m / 2;
    (m - right, right)
}

enum LeftTable {
    Pairs(PairTable),
    KSum(KSumTable),
}

impl LeftTable {
    fn entries(&self) -> u64 {
        match self {
            Self::Pairs(t) => t.entries,
            Self::KSum(t) => t.entries,
        }
    }

    fn work_counters(&self) -> BTreeMap<String, u64> {
        match self {
            Self::Pairs(t) => t.work_counters(),
            Self::KSum(t) => t.work_counters(),
        }
    }

    fn probe_left<G: CountedGroup>(
        &self,
        g: &G,
        fb: &FactorBase<G::Elt>,
        ops: &mut GroupOps,
        lookups: &mut u64,
        target: G::Elt,
    ) -> Option<Vec<usize>> {
        match self {
            Self::Pairs(t) => {
                let (i, j) = t.probe(g, fb, ops, lookups, target)?;
                Some(vec![i, j])
            }
            Self::KSum(t) => t.probe(g, fb, ops, lookups, target),
        }
    }
}

/// Meet-in-the-middle k-sum root finding for `m = 3..=12`.
pub struct RootfindKMitmOracle<'i> {
    m: u32,
    /// Optional prime instance: enables `sib=1` on the left table when set.
    instance: Option<&'i PrimeInstance>,
    left_k: u32,
    right_k: u32,
    table: Option<LeftTable>,
    /// Scratch for the right-hand combination walk.
    right_idx: Vec<usize>,
    pub mismatches: u64,
    probe_right_adds: u64,
    probe_lookups: u64,
    overlaps_rejected: u64,
}

impl<'i> RootfindKMitmOracle<'i> {
    pub fn new(m: u32) -> Self {
        let (left_k, right_k) = split_arity(m);
        Self {
            m,
            instance: None,
            left_k,
            right_k,
            table: None,
            right_idx: Vec::new(),
            mismatches: 0,
            probe_right_adds: 0,
            probe_lookups: 0,
            overlaps_rejected: 0,
        }
    }

    /// Prefer sibling last-level on prime curves when building the left table.
    pub fn new_prime(m: u32, instance: &'i PrimeInstance) -> Self {
        let mut s = Self::new(m);
        s.instance = Some(instance);
        s
    }

    /// Left-half table entry count after [`DecompositionOracle::prepare`].
    pub fn left_entries(&self) -> u64 {
        self.table.as_ref().map(LeftTable::entries).unwrap_or(0)
    }
}

impl DecompositionOracle<PrimeCurve> for RootfindKMitmOracle<'_> {
    fn name(&self) -> &str {
        "rootfind-kmitm"
    }

    fn summands(&self) -> u32 {
        self.m
    }

    fn describe(&self, _params: &Params) -> String {
        format!(
            "meet-in-the-middle root finding: table of left={} sums, probe with every right={} sum \
             (m={}, drastic vs full k-sum for m≥4; enables m=10..12)",
            self.left_k, self.right_k, self.m
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[
            ("m", "summands, 3 to 12; left=⌈m/2⌉ table, right=⌊m/2⌋ probe"),
            ("sib", "1: left table last level uses x-only siblings when the curve supports them (default 1 on primes)"),
            ("prune", "1: skip dead-end partials in the left k-sum build (default 1)"),
        ]
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        params: &Params,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        if !(3..=KMITM_MAX_M).contains(&self.m) {
            return Err(format!(
                "rootfind-kmitm takes m = 3 to {KMITM_MAX_M}, got {}",
                self.m
            ));
        }
        for key in params.0.keys() {
            if key != "m" && key != "sib" && key != "prune" {
                return Err(format!(
                    "rootfind-kmitm reads only `m`, `sib`, `prune`; `{key}` would change the \
                     method's identity without changing the search"
                ));
            }
        }
        let (left_k, right_k) = split_arity(self.m);
        self.left_k = left_k;
        self.right_k = right_k;
        self.right_idx = vec![0; right_k as usize];
        let prune = params.0.get("prune").map(|s| s != "0").unwrap_or(true);
        let sib = params
            .0
            .get("sib")
            .map(|s| s != "0")
            .unwrap_or(ctx.group.supports_siblings());

        let table = if left_k == 2 {
            let t = PairTable::build_negation_folded(ctx.group, fb);
            ops.merge(t.build_ops);
            LeftTable::Pairs(t)
        } else {
            if left_k > KSUM_MAX_K {
                return Err(format!(
                    "rootfind-kmitm left half {left_k} exceeds KSumTable max {KSUM_MAX_K}"
                ));
            }
            let t =
                KSumTable::build_negation_folded_full(ctx.group, fb, left_k, false, prune, sib)?;
            ops.merge(t.build_ops);
            LeftTable::KSum(t)
        };
        self.table = Some(table);
        self.mismatches = 0;
        self.probe_right_adds = 0;
        self.probe_lookups = 0;
        self.overlaps_rejected = 0;
        Ok(())
    }

    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        let mut out = self
            .table
            .as_ref()
            .map(LeftTable::work_counters)
            .unwrap_or_default();
        out.insert("kmitm_left_k".to_string(), u64::from(self.left_k));
        out.insert("kmitm_right_k".to_string(), u64::from(self.right_k));
        if let Some(t) = &self.table {
            out.insert("kmitm_left_entries".to_string(), t.entries());
        }
        table_counted(out)
    }

    fn relation_work_native(&self) -> BTreeMap<String, u64> {
        let mut out = BTreeMap::new();
        if self.probe_right_adds > 0 {
            out.insert("kmitm_right_adds".to_string(), self.probe_right_adds);
        }
        if self.probe_lookups > 0 {
            out.insert("kmitm_probe_lookups".to_string(), self.probe_lookups);
        }
        if self.overlaps_rejected > 0 {
            out.insert(
                "kmitm_overlaps_rejected".to_string(),
                self.overlaps_rejected,
            );
        }
        out
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PrimePoint,
    ) -> Option<Vec<usize>> {
        if point.infinity {
            return None;
        }
        // Take the table pointer pattern that avoids holding &mut self and
        // &table together: clone isn't available; use index-free reborrow
        // via split of fields after checking table exists.
        if self.table.is_none() {
            return None;
        }
        let g = ctx.group;
        let groups: Vec<&Vec<usize>> = fb
            .abscissa_list
            .iter()
            .map(|x| fb.x_index.get(x).expect("listed abscissa indexed"))
            .collect();
        self.enum_right(g, fb, ops, counters, point, &groups, 0, 0, None)
    }
}

impl RootfindKMitmOracle<'_> {
    #[allow(clippy::too_many_arguments)]
    fn enum_right(
        &mut self,
        g: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: PrimePoint,
        groups: &[&Vec<usize>],
        depth: usize,
        from: usize,
        partial: Option<PrimePoint>,
    ) -> Option<Vec<usize>> {
        let right_k = self.right_k as usize;
        if depth == right_k {
            let s = partial?;
            let t = g.add(ops, target, g.neg(s));
            self.probe_right_adds += 1;
            let mut lookups = 0u64;
            let left = {
                let table = self.table.as_ref()?;
                table.probe_left(g, fb, ops, &mut lookups, t)?
            };
            self.probe_lookups += lookups;
            counters.lookups += lookups;
            // Disjointness: no shared base index between left and right.
            let overlap = left.iter().any(|&i| self.right_idx[..right_k].contains(&i));
            if overlap {
                self.overlaps_rejected += 1;
                return None;
            }
            let mut out = left;
            out.extend_from_slice(&self.right_idx[..right_k]);
            // Fail-closed: re-sum on the ledger.
            let mut acc = fb.points[out[0]];
            for &i in &out[1..] {
                acc = g.add(ops, acc, fb.points[i]);
            }
            if acc == target {
                return Some(out);
            }
            if acc == g.neg(target) {
                for i in &mut out {
                    *i = fb.neg_index[*i];
                }
                return Some(out);
            }
            self.mismatches += 1;
            counters.frobfold_mismatches += 1;
            return None;
        }

        let end = groups.len();
        for ai in from..end {
            let group = groups[ai];
            // All signs on the right (full search); left table used the
            // negation fold so ± collisions are resolved at probe time.
            for &j in group.iter() {
                let s = match partial {
                    None => fb.points[j],
                    Some(p) => {
                        let q = g.add(ops, p, fb.points[j]);
                        self.probe_right_adds += 1;
                        q
                    }
                };
                if g.is_identity(&s) {
                    continue;
                }
                self.right_idx[depth] = j;
                if let Some(hit) = self.enum_right(
                    g,
                    fb,
                    ops,
                    counters,
                    target,
                    groups,
                    depth + 1,
                    ai + 1,
                    Some(s),
                ) {
                    return Some(hit);
                }
            }
        }
        None
    }
}

// Generic group impl for binary curves: same MITM, no sib default.
impl<'a> DecompositionOracle<crate::cryptanalysis::ic_boundary::BinaryGroup<'a>>
    for RootfindKMitmOracle<'_>
{
    fn name(&self) -> &str {
        "rootfind-kmitm"
    }
    fn summands(&self) -> u32 {
        self.m
    }
    fn describe(&self, _params: &Params) -> String {
        format!(
            "meet-in-the-middle root finding: left={} / right={} (m={})",
            self.left_k, self.right_k, self.m
        )
    }
    fn parameters(&self) -> &[(&str, &str)] {
        &[
            ("m", "summands, 3 to 12"),
            ("prune", "1: prune left k-sum dead ends (default 1)"),
        ]
    }
    fn prepare(
        &mut self,
        ctx: &InstanceCtx<crate::cryptanalysis::ic_boundary::BinaryGroup<'a>>,
        fb: &FactorBase<crate::cryptanalysis::koblitz_fast::FastPoint>,
        params: &Params,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        if !(3..=KMITM_MAX_M).contains(&self.m) {
            return Err(format!(
                "rootfind-kmitm takes m = 3 to {KMITM_MAX_M}, got {}",
                self.m
            ));
        }
        let (left_k, right_k) = split_arity(self.m);
        self.left_k = left_k;
        self.right_k = right_k;
        self.right_idx = vec![0; right_k as usize];
        let prune = params.0.get("prune").map(|s| s != "0").unwrap_or(true);
        let table = if left_k == 2 {
            let t = PairTable::build_negation_folded(ctx.group, fb);
            ops.merge(t.build_ops);
            LeftTable::Pairs(t)
        } else {
            let t =
                KSumTable::build_negation_folded_full(ctx.group, fb, left_k, false, prune, false)?;
            ops.merge(t.build_ops);
            LeftTable::KSum(t)
        };
        self.table = Some(table);
        Ok(())
    }
    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        let mut out = self
            .table
            .as_ref()
            .map(LeftTable::work_counters)
            .unwrap_or_default();
        out.insert("kmitm_left_k".to_string(), u64::from(self.left_k));
        out.insert("kmitm_right_k".to_string(), u64::from(self.right_k));
        table_counted(out)
    }
    fn decompose(
        &mut self,
        ctx: &InstanceCtx<crate::cryptanalysis::ic_boundary::BinaryGroup<'a>>,
        fb: &FactorBase<crate::cryptanalysis::koblitz_fast::FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: crate::cryptanalysis::koblitz_fast::FastPoint,
    ) -> Option<Vec<usize>> {
        if ctx.group.is_identity(&point) {
            return None;
        }
        if self.table.is_none() {
            return None;
        }
        let g = ctx.group;
        let groups: Vec<&Vec<usize>> = fb
            .abscissa_list
            .iter()
            .map(|x| fb.x_index.get(x).expect("listed"))
            .collect();
        self.enum_right_bin(g, fb, ops, counters, point, &groups, 0, 0, None)
    }
}

impl RootfindKMitmOracle<'_> {
    #[allow(clippy::too_many_arguments)]
    fn enum_right_bin(
        &mut self,
        g: &crate::cryptanalysis::ic_boundary::BinaryGroup<'_>,
        fb: &FactorBase<crate::cryptanalysis::koblitz_fast::FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: crate::cryptanalysis::koblitz_fast::FastPoint,
        groups: &[&Vec<usize>],
        depth: usize,
        from: usize,
        partial: Option<crate::cryptanalysis::koblitz_fast::FastPoint>,
    ) -> Option<Vec<usize>> {
        let right_k = self.right_k as usize;
        if depth == right_k {
            let s = partial?;
            let t = g.add(ops, target, g.neg(s));
            self.probe_right_adds += 1;
            let mut lookups = 0u64;
            let left = {
                let table = self.table.as_ref()?;
                table.probe_left(g, fb, ops, &mut lookups, t)?
            };
            self.probe_lookups += lookups;
            counters.lookups += lookups;
            for &i in &left {
                if self.right_idx[..right_k].contains(&i) {
                    self.overlaps_rejected += 1;
                    return None;
                }
            }
            let mut out = left;
            out.extend_from_slice(&self.right_idx[..right_k]);
            let mut acc = fb.points[out[0]];
            for &i in &out[1..] {
                acc = g.add(ops, acc, fb.points[i]);
            }
            if acc == target || acc == g.neg(target) {
                if acc == g.neg(target) {
                    for i in &mut out {
                        *i = fb.neg_index[*i];
                    }
                }
                return Some(out);
            }
            self.mismatches += 1;
            counters.frobfold_mismatches += 1;
            return None;
        }
        for ai in from..groups.len() {
            for &j in groups[ai].iter() {
                let s = match partial {
                    None => fb.points[j],
                    Some(p) => {
                        let q = g.add(ops, p, fb.points[j]);
                        self.probe_right_adds += 1;
                        q
                    }
                };
                if g.is_identity(&s) {
                    continue;
                }
                self.right_idx[depth] = j;
                if let Some(hit) = self.enum_right_bin(
                    g,
                    fb,
                    ops,
                    counters,
                    target,
                    groups,
                    depth + 1,
                    ai + 1,
                    Some(s),
                ) {
                    return Some(hit);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::{prime_factor_base, prime_instance_for};
    use crate::cryptanalysis::ic_framework::plugins::KSumOracle;
    use crate::cryptanalysis::ic_framework::stages::Params;

    fn ctx(inst: &PrimeInstance) -> InstanceCtx<'_, PrimeCurve> {
        InstanceCtx {
            group: &inst.curve,
            generator: inst.generator_point(),
            target: inst.generator_point(),
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        }
    }

    #[test]
    fn split_arity_balances() {
        assert_eq!(split_arity(3), (2, 1));
        assert_eq!(split_arity(4), (2, 2));
        assert_eq!(split_arity(5), (3, 2));
        assert_eq!(split_arity(12), (6, 6));
    }

    /// Planted m-sum is recovered by kmitm for several arities.
    #[test]
    fn kmitm_recovers_planted_sums_m3_to_m6() {
        let inst = prime_instance_for(22, 59297);
        let fb = prime_factor_base(&inst, 16);
        let c = ctx(&inst);
        let g = &inst.curve;
        assert!(fb.points.len() >= 12);
        for m in [3u32, 4, 5, 6] {
            // Distinct abscissa representatives.
            let mut idxs = Vec::new();
            for x in &fb.abscissa_list {
                if idxs.len() == m as usize {
                    break;
                }
                idxs.push(fb.x_index[x][0]);
            }
            assert_eq!(idxs.len(), m as usize);
            let mut ops = GroupOps::default();
            let mut s = fb.points[idxs[0]];
            for &i in &idxs[1..] {
                s = g.add(&mut ops, s, fb.points[i]);
            }
            let (_, params) = Params::parse_spec(&format!("rootfind-kmitm:m={m}")).unwrap();
            let mut oracle = RootfindKMitmOracle::new_prime(m, &inst);
            let mut build = GroupOps::default();
            oracle.prepare(&c, &fb, &params, &mut build).unwrap();
            let mut dops = GroupOps::default();
            let mut counters = OracleCounters::default();
            let hit = oracle
                .decompose(&c, &fb, &mut dops, &mut counters, s)
                .unwrap_or_else(|| panic!("m={m} missed planted sum"));
            assert_eq!(hit.len(), m as usize, "m={m}");
            let mut acc = fb.points[hit[0]];
            for &i in &hit[1..] {
                acc = g.add(&mut dops, acc, fb.points[i]);
            }
            assert!(acc == s || acc == g.neg(s), "m={m} re-sum failed");
            assert_eq!(oracle.mismatches, 0);
        }
    }

    /// m=10 builds (full ksum cannot); planted recovery on a tiny base.
    #[test]
    fn kmitm_m10_builds_and_recovers_on_tiny_base() {
        let inst = prime_instance_for(20, 59297);
        // Need ≥10 abscissae; size 24 is safe.
        let fb = prime_factor_base(&inst, 24);
        let c = ctx(&inst);
        let g = &inst.curve;
        let m = 10u32;
        let mut idxs = Vec::new();
        for x in &fb.abscissa_list {
            if idxs.len() == m as usize {
                break;
            }
            idxs.push(fb.x_index[x][0]);
        }
        let mut ops = GroupOps::default();
        let mut s = fb.points[idxs[0]];
        for &i in &idxs[1..] {
            s = g.add(&mut ops, s, fb.points[i]);
        }
        let (_, params) = Params::parse_spec("rootfind-kmitm:m=10").unwrap();
        let mut oracle = RootfindKMitmOracle::new_prime(m, &inst);
        let mut build = GroupOps::default();
        oracle.prepare(&c, &fb, &params, &mut build).unwrap();
        assert!(oracle.table.as_ref().unwrap().entries() > 0);
        // Full ksum must refuse m=10.
        let err = match KSumTable::build_negation_folded(g, &fb, 10) {
            Err(e) => e,
            Ok(_) => panic!("full ksum should refuse m=10"),
        };
        assert!(err.contains("3 to 9") || err.contains("not 10"), "{err}");
        let mut dops = GroupOps::default();
        let mut counters = OracleCounters::default();
        let hit = oracle.decompose(&c, &fb, &mut dops, &mut counters, s);
        assert!(hit.is_some(), "m=10 should recover planted 10-sum");
    }

    /// Against full ksum at m=4 on a small base: kmitm build is far smaller.
    #[test]
    fn kmitm_m4_build_much_smaller_than_full_ksum() {
        let inst = prime_instance_for(20, 1);
        let fb = prime_factor_base(&inst, 12);
        let c = ctx(&inst);
        let (_, pk) = Params::parse_spec("ksum:m=4,prune=1").unwrap();
        let (_, pm) = Params::parse_spec("rootfind-kmitm:m=4,prune=1").unwrap();
        let mut ksum = KSumOracle::new(4);
        let mut mitm = RootfindKMitmOracle::new_prime(4, &inst);
        let mut bk = GroupOps::default();
        let mut bm = GroupOps::default();
        ksum.prepare(&c, &fb, &pk, &mut bk).unwrap();
        mitm.prepare(&c, &fb, &pm, &mut bm).unwrap();
        let m_entries = mitm.table.as_ref().unwrap().entries();
        // Pair table (~A²/4) vs 4-sum (~8·C(A,4)).
        assert!(
            m_entries < 500,
            "mitm left entries should be pair-table sized, got {m_entries}"
        );
        assert!(bk.adds > bm.adds, "full ksum build should charge more adds");
        let _ = <KSumOracle as DecompositionOracle<PrimeCurve>>::setup_work_native(&ksum);
    }
}
