//! `rootfind-kmitm`: meet-in-the-middle root-finding for arity `m = 3..=12`.
//!
//! Full [`KSumTable`] stops at `m = 9` (u64 packing) and costs about
//! `2^{m−1}·C(|F|, m)` build additions. This oracle stores a **left**
//! `L`-sum table and probes with every **right** `R`-sum `S` via `R − S`
//! (here `R` is the target). Overlap between left and right summands is
//! rejected.
//!
//! Default split is **probe-favoring**: `L = min(9, m−1)`, `R = m−L`.
//! That makes `R = 1` for every `m ≤ 10`, so a probe is one pass over the
//! factor base — drastic vs a balanced half-split (and the only practical
//! path for `m = 10..12`). Pass `split=balanced` for `⌈m/2⌉ / ⌊m/2⌋` when
//! the left table would be too large.
//!
//! Right-half sums with `R ≥ 2` are materialised once at prepare and reused
//! across targets. Classification: engineering (wall / build size); charged
//! group additions are the MITM's own (not equal to full `ksum`).

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
fn split_arity(m: u32, balanced: bool) -> (u32, u32) {
    if balanced {
        let right = m / 2;
        (m - right, right)
    } else {
        // Probe-favoring: biggest left table KSumTable can hold.
        let left = KSUM_MAX_K.min(m.saturating_sub(1));
        (left, m - left)
    }
}

enum LeftTable {
    Pairs(Box<PairTable>),
    KSum(Box<KSumTable>),
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

/// One precomputed right-half sum (used when `right_k ≥ 2`).
struct RightSum<P> {
    point: P,
    idxs: Vec<usize>,
}

/// Meet-in-the-middle k-sum root finding for `m = 3..=12`.
pub struct RootfindKMitmOracle<'i> {
    m: u32,
    /// Optional prime instance: enables `sib=1` on the left table when set.
    instance: Option<&'i PrimeInstance>,
    left_k: u32,
    right_k: u32,
    balanced: bool,
    table: Option<LeftTable>,
    /// Precomputed right-half sums for prime probes (`right_k ≥ 2`).
    right_prime: Vec<RightSum<PrimePoint>>,
    /// Precomputed right-half sums for binary probes (`right_k ≥ 2`).
    right_bin: Vec<RightSum<crate::cryptanalysis::koblitz_fast::FastPoint>>,
    pub mismatches: u64,
    probe_right_adds: u64,
    probe_lookups: u64,
    overlaps_rejected: u64,
}

impl<'i> RootfindKMitmOracle<'i> {
    pub fn new(m: u32) -> Self {
        let (left_k, right_k) = split_arity(m, false);
        Self {
            m,
            instance: None,
            left_k,
            right_k,
            balanced: false,
            table: None,
            right_prime: Vec::new(),
            right_bin: Vec::new(),
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

    pub fn left_k(&self) -> u32 {
        self.left_k
    }

    pub fn right_k(&self) -> u32 {
        self.right_k
    }

    pub fn right_precomputed(&self) -> u64 {
        self.right_prime.len().max(self.right_bin.len()) as u64
    }
}

fn build_left_table<G: CountedGroup>(
    g: &G,
    fb: &FactorBase<G::Elt>,
    left_k: u32,
    prune: bool,
    sib: bool,
    ops: &mut GroupOps,
) -> Result<LeftTable, String> {
    if left_k == 2 {
        let t = PairTable::build_negation_folded(g, fb);
        ops.merge(t.build_ops);
        Ok(LeftTable::Pairs(Box::new(t)))
    } else {
        if left_k > KSUM_MAX_K {
            return Err(format!(
                "rootfind-kmitm left half {left_k} exceeds KSumTable max {KSUM_MAX_K}"
            ));
        }
        let t = KSumTable::build_negation_folded_full(g, fb, left_k, false, prune, sib)?;
        ops.merge(t.build_ops);
        Ok(LeftTable::KSum(Box::new(t)))
    }
}

/// Enumerate every ordered distinct-abscissa `right_k`-sum into `out`.
fn collect_right_sums<G: CountedGroup>(
    g: &G,
    fb: &FactorBase<G::Elt>,
    right_k: u32,
    ops: &mut GroupOps,
    out: &mut Vec<RightSum<G::Elt>>,
) where
    G::Elt: Copy,
{
    out.clear();
    if right_k == 0 {
        return;
    }
    let groups: Vec<&Vec<usize>> = fb
        .abscissa_list
        .iter()
        .map(|x| fb.x_index.get(x).expect("listed abscissa indexed"))
        .collect();
    let mut idxs = vec![0usize; right_k as usize];
    collect_right_rec(
        g,
        fb,
        &groups,
        right_k as usize,
        0,
        0,
        None,
        &mut idxs,
        ops,
        out,
    );
}

#[allow(clippy::too_many_arguments)]
fn collect_right_rec<G: CountedGroup>(
    g: &G,
    fb: &FactorBase<G::Elt>,
    groups: &[&Vec<usize>],
    right_k: usize,
    depth: usize,
    from: usize,
    partial: Option<G::Elt>,
    idxs: &mut [usize],
    ops: &mut GroupOps,
    out: &mut Vec<RightSum<G::Elt>>,
) where
    G::Elt: Copy,
{
    if depth == right_k {
        let Some(point) = partial else {
            return;
        };
        if g.is_identity(&point) {
            return;
        }
        out.push(RightSum {
            point,
            idxs: idxs.to_vec(),
        });
        return;
    }
    for ai in from..groups.len() {
        for &j in groups[ai].iter() {
            let s = match partial {
                None => fb.points[j],
                Some(p) => g.add(ops, p, fb.points[j]),
            };
            if g.is_identity(&s) {
                continue;
            }
            idxs[depth] = j;
            collect_right_rec(
                g,
                fb,
                groups,
                right_k,
                depth + 1,
                ai + 1,
                Some(s),
                idxs,
                ops,
                out,
            );
        }
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
        let mode = if self.balanced {
            "balanced"
        } else {
            "probe-favoring"
        };
        format!(
            "meet-in-the-middle root finding: left={}-sum table, right={}-sum probe \
             (m={}, split={mode}; enables m=10..12; drastic vs full k-sum)",
            self.left_k, self.right_k, self.m
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[
            ("m", "summands, 3 to 12"),
            (
                "split",
                "probe (default): left=min(9,m-1); balanced: left=⌈m/2⌉",
            ),
            (
                "sib",
                "1: left table last level uses x-only siblings when the curve supports them (default 1 on primes)",
            ),
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
            if key != "m" && key != "sib" && key != "prune" && key != "split" {
                return Err(format!(
                    "rootfind-kmitm reads only `m`, `sib`, `prune`, `split`; `{key}` would change the \
                     method's identity without changing the search"
                ));
            }
        }
        let balanced = params
            .0
            .get("split")
            .map(|s| s == "balanced" || s == "half")
            .unwrap_or(false);
        let (left_k, right_k) = split_arity(self.m, balanced);
        self.balanced = balanced;
        self.left_k = left_k;
        self.right_k = right_k;
        let prune = params.0.get("prune").map(|s| s != "0").unwrap_or(true);
        let sib = params
            .0
            .get("sib")
            .map(|s| s != "0")
            .unwrap_or(ctx.group.supports_siblings());

        self.table = Some(build_left_table(ctx.group, fb, left_k, prune, sib, ops)?);
        self.right_bin.clear();
        self.right_prime.clear();
        if right_k >= 2 {
            collect_right_sums(ctx.group, fb, right_k, ops, &mut self.right_prime);
        }
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
        out.insert("kmitm_split_balanced".to_string(), u64::from(self.balanced));
        if let Some(t) = &self.table {
            out.insert("kmitm_left_entries".to_string(), t.entries());
        }
        if !self.right_prime.is_empty() {
            out.insert(
                "kmitm_right_precomputed".to_string(),
                self.right_prime.len() as u64,
            );
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
        // Existence check only — do not hold the borrow across &mut self below.
        self.table.as_ref()?;
        let g = ctx.group;
        if self.right_k == 1 {
            return self.probe_right1_prime(g, fb, ops, counters, point);
        }
        self.probe_right_precomputed_prime(g, fb, ops, counters, point)
    }
}

impl RootfindKMitmOracle<'_> {
    fn finish_hit_prime(
        &mut self,
        g: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: PrimePoint,
        left: Vec<usize>,
        right: &[usize],
    ) -> Option<Vec<usize>> {
        let overlap = left.iter().any(|&i| right.contains(&i));
        if overlap {
            self.overlaps_rejected += 1;
            return None;
        }
        let mut out = left;
        out.extend_from_slice(right);
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
        None
    }

    fn probe_right1_prime(
        &mut self,
        g: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: PrimePoint,
    ) -> Option<Vec<usize>> {
        for (j, &pj) in fb.points.iter().enumerate() {
            if g.is_identity(&pj) {
                continue;
            }
            let t = g.add(ops, target, g.neg(pj));
            self.probe_right_adds += 1;
            let mut lookups = 0u64;
            let left = {
                let table = self.table.as_ref()?;
                match table.probe_left(g, fb, ops, &mut lookups, t) {
                    Some(v) => v,
                    None => {
                        self.probe_lookups += lookups;
                        counters.lookups += lookups;
                        continue;
                    }
                }
            };
            self.probe_lookups += lookups;
            counters.lookups += lookups;
            if let Some(hit) =
                self.finish_hit_prime(g, fb, ops, counters, target, left, std::slice::from_ref(&j))
            {
                return Some(hit);
            }
        }
        None
    }

    fn probe_right_precomputed_prime(
        &mut self,
        g: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: PrimePoint,
    ) -> Option<Vec<usize>> {
        // Index into owned vec without holding &self.right_prime across &mut self.
        let n = self.right_prime.len();
        for i in 0..n {
            let (s, right_idxs) = {
                let e = &self.right_prime[i];
                (e.point, e.idxs.clone())
            };
            let t = g.add(ops, target, g.neg(s));
            self.probe_right_adds += 1;
            let mut lookups = 0u64;
            let left = {
                let table = self.table.as_ref()?;
                match table.probe_left(g, fb, ops, &mut lookups, t) {
                    Some(v) => v,
                    None => {
                        self.probe_lookups += lookups;
                        counters.lookups += lookups;
                        continue;
                    }
                }
            };
            self.probe_lookups += lookups;
            counters.lookups += lookups;
            if let Some(hit) =
                self.finish_hit_prime(g, fb, ops, counters, target, left, &right_idxs)
            {
                return Some(hit);
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
            (
                "split",
                "probe (default): left=min(9,m-1); balanced: left=⌈m/2⌉",
            ),
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
        let balanced = params
            .0
            .get("split")
            .map(|s| s == "balanced" || s == "half")
            .unwrap_or(false);
        let (left_k, right_k) = split_arity(self.m, balanced);
        self.balanced = balanced;
        self.left_k = left_k;
        self.right_k = right_k;
        let prune = params.0.get("prune").map(|s| s != "0").unwrap_or(true);
        self.table = Some(build_left_table(ctx.group, fb, left_k, prune, false, ops)?);
        self.right_prime.clear();
        self.right_bin.clear();
        if right_k >= 2 {
            collect_right_sums(ctx.group, fb, right_k, ops, &mut self.right_bin);
        }
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
        if !self.right_bin.is_empty() {
            out.insert(
                "kmitm_right_precomputed".to_string(),
                self.right_bin.len() as u64,
            );
        }
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
        self.table.as_ref()?;
        let g = ctx.group;
        if self.right_k == 1 {
            return self.probe_right1_bin(g, fb, ops, counters, point);
        }
        self.probe_right_precomputed_bin(g, fb, ops, counters, point)
    }
}

impl RootfindKMitmOracle<'_> {
    fn finish_hit_bin(
        &mut self,
        g: &crate::cryptanalysis::ic_boundary::BinaryGroup<'_>,
        fb: &FactorBase<crate::cryptanalysis::koblitz_fast::FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: crate::cryptanalysis::koblitz_fast::FastPoint,
        left: Vec<usize>,
        right: &[usize],
    ) -> Option<Vec<usize>> {
        for &i in &left {
            if right.contains(&i) {
                self.overlaps_rejected += 1;
                return None;
            }
        }
        let mut out = left;
        out.extend_from_slice(right);
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
        None
    }

    fn probe_right1_bin(
        &mut self,
        g: &crate::cryptanalysis::ic_boundary::BinaryGroup<'_>,
        fb: &FactorBase<crate::cryptanalysis::koblitz_fast::FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: crate::cryptanalysis::koblitz_fast::FastPoint,
    ) -> Option<Vec<usize>> {
        for (j, &pj) in fb.points.iter().enumerate() {
            if g.is_identity(&pj) {
                continue;
            }
            let t = g.add(ops, target, g.neg(pj));
            self.probe_right_adds += 1;
            let mut lookups = 0u64;
            let left = {
                let table = self.table.as_ref()?;
                match table.probe_left(g, fb, ops, &mut lookups, t) {
                    Some(v) => v,
                    None => {
                        self.probe_lookups += lookups;
                        counters.lookups += lookups;
                        continue;
                    }
                }
            };
            self.probe_lookups += lookups;
            counters.lookups += lookups;
            if let Some(hit) =
                self.finish_hit_bin(g, fb, ops, counters, target, left, std::slice::from_ref(&j))
            {
                return Some(hit);
            }
        }
        None
    }

    fn probe_right_precomputed_bin(
        &mut self,
        g: &crate::cryptanalysis::ic_boundary::BinaryGroup<'_>,
        fb: &FactorBase<crate::cryptanalysis::koblitz_fast::FastPoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: crate::cryptanalysis::koblitz_fast::FastPoint,
    ) -> Option<Vec<usize>> {
        let n = self.right_bin.len();
        for i in 0..n {
            let (s, right_idxs) = {
                let e = &self.right_bin[i];
                (e.point, e.idxs.clone())
            };
            let t = g.add(ops, target, g.neg(s));
            self.probe_right_adds += 1;
            let mut lookups = 0u64;
            let left = {
                let table = self.table.as_ref()?;
                match table.probe_left(g, fb, ops, &mut lookups, t) {
                    Some(v) => v,
                    None => {
                        self.probe_lookups += lookups;
                        counters.lookups += lookups;
                        continue;
                    }
                }
            };
            self.probe_lookups += lookups;
            counters.lookups += lookups;
            if let Some(hit) = self.finish_hit_bin(g, fb, ops, counters, target, left, &right_idxs)
            {
                return Some(hit);
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
    fn split_arity_probe_favoring_and_balanced() {
        assert_eq!(split_arity(3, false), (2, 1));
        assert_eq!(split_arity(4, false), (3, 1));
        assert_eq!(split_arity(6, false), (5, 1));
        assert_eq!(split_arity(10, false), (9, 1));
        assert_eq!(split_arity(11, false), (9, 2));
        assert_eq!(split_arity(12, false), (9, 3));
        assert_eq!(split_arity(4, true), (2, 2));
        assert_eq!(split_arity(12, true), (6, 6));
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
            assert_eq!(
                oracle.right_k(),
                1,
                "m={m} should be right=1 under probe split"
            );
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
    /// Probe-favoring uses left=9, so keep |F_x| small (C(A,9)·2⁸).
    #[test]
    fn kmitm_m10_builds_and_recovers_on_tiny_base() {
        let inst = prime_instance_for(20, 59297);
        let fb = prime_factor_base(&inst, 14);
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
        assert_eq!(oracle.left_k(), 9);
        assert_eq!(oracle.right_k(), 1);
        assert!(oracle.table.as_ref().unwrap().entries() > 0);
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

    /// m=12 with right=3 precomputed list recovers a planted sum.
    /// left=9 ⇒ keep |F_x| modest so the 9-sum table fits in CI.
    #[test]
    fn kmitm_m12_recovers_with_precomputed_right() {
        let inst = prime_instance_for(20, 59297);
        let fb = prime_factor_base(&inst, 14);
        let c = ctx(&inst);
        let g = &inst.curve;
        let m = 12u32;
        let mut idxs = Vec::new();
        for x in &fb.abscissa_list {
            if idxs.len() == m as usize {
                break;
            }
            idxs.push(fb.x_index[x][0]);
        }
        assert_eq!(idxs.len(), 12);
        let mut ops = GroupOps::default();
        let mut s = fb.points[idxs[0]];
        for &i in &idxs[1..] {
            s = g.add(&mut ops, s, fb.points[i]);
        }
        let (_, params) = Params::parse_spec("rootfind-kmitm:m=12").unwrap();
        let mut oracle = RootfindKMitmOracle::new_prime(m, &inst);
        let mut build = GroupOps::default();
        oracle.prepare(&c, &fb, &params, &mut build).unwrap();
        assert_eq!((oracle.left_k(), oracle.right_k()), (9, 3));
        assert!(!oracle.right_prime.is_empty());
        let mut dops = GroupOps::default();
        let mut counters = OracleCounters::default();
        let hit = oracle.decompose(&c, &fb, &mut dops, &mut counters, s);
        assert!(hit.is_some(), "m=12 should recover planted 12-sum");
    }

    /// Against full ksum at m=4: kmitm still charges fewer build adds.
    #[test]
    fn kmitm_m4_build_cheaper_than_full_ksum() {
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
        assert_eq!(mitm.right_k(), 1);
        assert!(bk.adds > bm.adds, "full ksum build should charge more adds");
        let _ = <KSumOracle as DecompositionOracle<PrimeCurve>>::setup_work_native(&ksum);
    }
}
