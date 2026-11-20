//! `rootfind-floor`: lever 1 of
//! `research/rootfind_baseline_100x_20261117/PLAN.md` as a production
//! oracle beside [`super::plugins::RootfindSubtractOracle`].
//!
//! Same search as `rootfind-subtract` (every `R ∓ F_i`, same order, same
//! charged group additions) on three cheaper field/probe steps:
//!
//! 1. **Shared denominators** — already in `rootfind-subtract`; kept.
//! 2. **Abscissa only, Montgomery-resident** — form `x(R ∓ F_i)` in
//!    Montgomery form and leave `y` unfinished until a filter hit.
//! 3. **Word-blocked Bloom filter** — 16 bits a key in front of the
//!    exact abscissa map, since nearly every probe misses. For `m = 3`
//!    the Bloom sits on Montgomery images of the PairTable's folded keys;
//!    a pass looks up the PairTable directly (no second FastMap of every
//!    pair — that doubled prepare wall and erased the e2e gain).
//!
//! New id: never change what `rootfind-subtract` counts. Classification
//! is constant-factor engineering (wall / field work); charged `S` matches
//! `rootfind-subtract` by design. Fail-closed: every hit re-adds in the
//! group on the ledger.

use std::collections::BTreeMap;

use super::plugins::table_counted;
use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    fast_map, CountedGroup, FactorBase, FastMap, GroupOps, OracleCounters, PairTable, PrimeCurve,
    PrimeInstance, PrimePoint,
};
use crate::cryptanalysis::prime_fast::{Fp64, MAX_BITS as PRIME_FAST_MAX_BITS};

/// One representative an abscissa, Montgomery form, with both signs'
/// factor-base indices.
#[derive(Clone, Copy)]
struct FloorRep {
    x: u64,
    y: u64,
    plus: usize,
    minus: usize,
}

/// Word-blocked Bloom filter: one 64-bit word a key, five bits in it
/// (matches `examples/rootfind_floor_probe.rs`).
struct WordBloom {
    words: Vec<u64>,
    mask: usize,
}

impl WordBloom {
    fn new(keys: usize) -> Self {
        let words = (keys / 4).next_power_of_two().max(64);
        Self {
            words: vec![0; words],
            mask: words - 1,
        }
    }

    #[inline(always)]
    fn slot(&self, key: u64) -> (usize, u64) {
        let h = key.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let h2 = (h ^ (h >> 29)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        let mut bits = 0u64;
        let mut g = h2;
        for _ in 0..5 {
            bits |= 1u64 << (g & 63);
            g >>= 6;
        }
        ((h >> 34) as usize & self.mask, bits)
    }

    fn insert(&mut self, key: u64) {
        let (w, bits) = self.slot(key);
        self.words[w] |= bits;
    }

    #[inline(always)]
    fn maybe(&self, key: u64) -> bool {
        let (w, bits) = self.slot(key);
        self.words[w] & bits == bits
    }
}

/// Direct-subtraction root finding with x-only Montgomery candidates and
/// a Bloom filter in front of the exact abscissa map.
pub struct RootfindFloorOracle<'i> {
    m: u32,
    instance: &'i PrimeInstance,
    field: Option<Fp64>,
    reps: Vec<FloorRep>,
    /// `m = 2`: Montgomery abscissa → (plus, minus) base indices.
    x_map: FastMap<(u32, u32)>,
    bloom: Option<WordBloom>,
    /// `m = 3`: negation-folded pair table; bloom-pass probes look up here
    /// (no parallel Montgomery witness map — that doubled prepare wall).
    table: Option<PairTable>,
    diffs: Vec<u64>,
    degenerate: Vec<bool>,
    scratch: Vec<u64>,
    pub mismatches: u64,
    sweep_field_muls: u64,
    sweep_field_adds: u64,
    /// Exact-map probes that passed the Bloom filter.
    bloom_passes: u64,
    bloom_false_positives: u64,
}

impl<'i> RootfindFloorOracle<'i> {
    pub fn new(m: u32, instance: &'i PrimeInstance) -> Self {
        Self {
            m,
            instance,
            field: None,
            reps: Vec::new(),
            x_map: fast_map(0),
            bloom: None,
            table: None,
            diffs: Vec::new(),
            degenerate: Vec::new(),
            scratch: Vec::new(),
            mismatches: 0,
            sweep_field_muls: 0,
            sweep_field_adds: 0,
            bloom_passes: 0,
            bloom_false_positives: 0,
        }
    }
}

impl DecompositionOracle<PrimeCurve> for RootfindFloorOracle<'_> {
    fn name(&self) -> &str {
        "rootfind-floor"
    }

    fn summands(&self) -> u32 {
        self.m
    }

    fn describe(&self, _params: &Params) -> String {
        match self.m {
            2 => "x-only Montgomery R∓F_i from one batched inversion, Bloom-filtered \
                  lookup in the base (lever-1 floor probe as a production oracle)"
                .into(),
            _ => "x-only Montgomery R∓F_i from one batched inversion, Bloom-filtered \
                  probe of the negation-folded pair table (lever-1 floor probe)"
                .into(),
        }
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[(
            "m",
            "summands, 2 or 3; 3 builds the negation-folded pair table",
        )]
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        params: &Params,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        if !(2..=3).contains(&self.m) {
            return Err(format!("rootfind-floor takes m = 2 or 3, got {}", self.m));
        }
        for key in params.0.keys() {
            if key != "m" {
                return Err(format!(
                    "rootfind-floor reads only `m`; `{key}` would change the method's identity \
                     without changing the search"
                ));
            }
        }
        let p = self.instance.curve.p;
        if p != ctx.group.p {
            return Err("rootfind-floor was built for another curve".into());
        }
        if p < 5 || p & 1 == 0 || p >= (1u64 << PRIME_FAST_MAX_BITS) {
            return Err(format!(
                "rootfind-floor needs an odd prime 3 < p < 2^{PRIME_FAST_MAX_BITS}"
            ));
        }
        let f = Fp64::new(p);
        let mut reps = Vec::with_capacity(fb.abscissa_list.len());
        let mut x_map: FastMap<(u32, u32)> = fast_map(fb.abscissa_list.len() + 1);
        for x in &fb.abscissa_list {
            let idx = fb
                .x_index
                .get(x)
                .ok_or("every listed abscissa is indexed")?;
            let plus = *idx.first().ok_or("an abscissa with no point")?;
            let minus = idx.get(1).copied().unwrap_or(plus);
            let pt = fb.points[plus];
            if pt.infinity || pt.x != *x {
                return Err("the base's abscissa index does not match its points".into());
            }
            let xm = f.to_mont(pt.x);
            let ym = f.to_mont(pt.y);
            x_map.insert(xm, (plus as u32, minus as u32));
            reps.push(FloorRep {
                x: xm,
                y: ym,
                plus,
                minus,
            });
        }
        self.diffs = vec![0; reps.len()];
        self.degenerate = vec![false; reps.len()];
        self.reps = reps;
        self.x_map = x_map;
        self.field = Some(f);

        let mut bloom = WordBloom::new(self.x_map.len().max(1));
        if self.m == 2 {
            for &k in self.x_map.keys() {
                bloom.insert(k);
            }
        } else {
            let table = PairTable::build_negation_folded(ctx.group, fb);
            ops.merge(table.build_ops);
            // Bloom over Montgomery abscissae of the table's folded keys
            // (x+1 → to_mont(x)). Misses skip from_mont; hits look up the
            // PairTable directly — no second FastMap of every entry.
            let n = table.entries.max(1) as usize;
            bloom = WordBloom::new(n);
            table.for_each_folded_key(|folded| {
                bloom.insert(f.to_mont(folded.wrapping_sub(1)));
            });
            self.table = Some(table);
        }
        self.bloom = Some(bloom);
        self.bloom_passes = 0;
        self.bloom_false_positives = 0;
        Ok(())
    }

    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        let mut out = self
            .table
            .as_ref()
            .map(PairTable::work_counters)
            .unwrap_or_default();
        let k = self.reps.len() as u64;
        if k > 0 {
            crate::cryptanalysis::ic_boundary::bump_counter(&mut out, "lookups", k);
            out.insert("montgomery_conversions".to_string(), 2 * k);
        }
        if let Some(b) = &self.bloom {
            out.insert("bloom_words".to_string(), b.words.len() as u64);
        }
        if self.m == 2 && !self.x_map.is_empty() {
            out.insert("floor_map_entries".to_string(), self.x_map.len() as u64);
        }
        if let Some(t) = &self.table {
            out.insert("floor_table_entries".to_string(), t.entries);
        }
        table_counted(out)
    }

    fn relation_work_native(&self) -> BTreeMap<String, u64> {
        let mut out = BTreeMap::new();
        if self.sweep_field_muls > 0 {
            out.insert("sweep_field_muls".to_string(), self.sweep_field_muls);
        }
        if self.sweep_field_adds > 0 {
            out.insert("sweep_field_adds".to_string(), self.sweep_field_adds);
        }
        if self.bloom_passes > 0 {
            out.insert("bloom_passes".to_string(), self.bloom_passes);
        }
        if self.bloom_false_positives > 0 {
            out.insert(
                "bloom_false_positives".to_string(),
                self.bloom_false_positives,
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
        let f = self.field?;
        // Existence check only — do not hold the borrow across &mut self below.
        self.bloom.as_ref()?;
        if point.infinity {
            return None;
        }
        let g = ctx.group;
        let xr = f.to_mont(point.x);
        let yr = f.to_mont(point.y);
        ops.field_ops.muls += 2;
        let k = self.reps.len();
        for (i, rep) in self.reps.iter().enumerate() {
            let d = f.sub(xr, rep.x);
            self.degenerate[i] = d == 0;
            self.diffs[i] = if d == 0 { f.one } else { d };
        }
        f.batch_inv(&mut self.diffs, &mut self.scratch);
        counters.inversions += 1;
        ops.field_ops.invs += 1;
        ops.field_ops.muls += 3 * k as u64;
        self.sweep_field_muls += 2 + 3 * k as u64;
        self.sweep_field_adds += k as u64;

        for i in 0..k {
            let rep = self.reps[i];
            if self.degenerate[i] {
                for summand in [rep.plus, rep.minus] {
                    let t = g.add(ops, point, g.neg(fb.points[summand]));
                    if let Some(found) = self.lookup_full(ctx, fb, ops, counters, point, summand, t)
                    {
                        return Some(found);
                    }
                }
                continue;
            }
            let inv = self.diffs[i];
            let lam_minus = f.mul(f.add(yr, rep.y), inv);
            let lam_plus = f.mul(f.sub(yr, rep.y), inv);
            let base = f.add(xr, rep.x);
            // x-only: two multiplications (λ·λ) and no from_mont / y.
            let x_minus = f.sub(f.mul(lam_minus, lam_minus), base);
            let x_plus = f.sub(f.mul(lam_plus, lam_plus), base);
            // Charge one group addition per candidate, matching
            // rootfind-subtract's ledger (same search, same S).
            for (summand, x3, lam) in [
                (rep.plus, x_minus, lam_minus),
                (rep.minus, x_plus, lam_plus),
            ] {
                ops.adds += 1;
                // Honest field work of the x-only path: 2 muls for λ
                // (shared across the pair above — counted once each here
                // as 1 mul for λ and 1 for λ²) plus no ordinate / mont-out.
                ops.field_ops.muls += 2;
                self.sweep_field_muls += 2;
                counters.lookups += 1;
                let maybe = self.bloom.as_ref().map(|b| b.maybe(x3)).unwrap_or(false);
                if !maybe {
                    continue;
                }
                self.bloom_passes += 1;
                if let Some(found) =
                    self.lookup_x(ctx, fb, ops, counters, point, summand, x3, lam, yr, f)
                {
                    return Some(found);
                }
                self.bloom_false_positives += 1;
            }
        }
        None
    }
}

impl RootfindFloorOracle<'_> {
    #[allow(clippy::too_many_arguments)]
    fn lookup_full(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PrimePoint,
        summand: usize,
        t: PrimePoint,
    ) -> Option<Vec<usize>> {
        let g = ctx.group;
        if g.is_identity(&t) {
            counters.lookups += 1;
            return None;
        }
        match self.m {
            2 => {
                counters.lookups += 1;
                let j = fb.index_of_key(g.key(&t))?;
                if g.add(ops, fb.points[summand], fb.points[j]) == point {
                    return Some(vec![summand, j]);
                }
                self.mismatches += 1;
                counters.frobfold_mismatches += 1;
                None
            }
            _ => {
                let table = self.table.as_ref()?;
                let (j, k) = table.probe(g, fb, ops, &mut counters.lookups, t)?;
                Some(vec![summand, j, k])
            }
        }
    }

    /// Bloom passed: finish the ordinate only when the exact map hits.
    #[allow(clippy::too_many_arguments)]
    fn lookup_x(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PrimePoint,
        summand: usize,
        x3: u64,
        lam: u64,
        yr: u64,
        f: Fp64,
    ) -> Option<Vec<usize>> {
        let g = ctx.group;
        match self.m {
            2 => {
                let &(jp, jm) = self.x_map.get(&x3)?;
                for j in [jp as usize, jm as usize] {
                    if g.add(ops, fb.points[summand], fb.points[j]) == point {
                        return Some(vec![summand, j]);
                    }
                }
                // Map hit but neither sign re-added: should not happen for
                // a correct abscissa; count as a mismatch if we got here
                // with a key that was in the map.
                self.mismatches += 1;
                counters.frobfold_mismatches += 1;
                None
            }
            _ => {
                // Bloom already passed: one from_mont → folded key → table.
                let x_aff = f.from_mont(x3);
                ops.field_ops.muls += 1;
                let table = self.table.as_ref()?;
                let (i, j) = table.lookup_folded(x_aff.wrapping_add(1))?;
                // Finish y only to resolve the sign against ±T.
                let y3 = f.sub(f.mul(lam, f.sub(f.to_mont(point.x), x3)), yr);
                let y_aff = f.from_mont(y3);
                ops.field_ops.muls += 2; // λ·Δx + from_mont(y)
                let t = PrimePoint::affine(x_aff, y_aff);
                let s = g.add(ops, fb.points[i], fb.points[j]);
                if s == t {
                    Some(vec![summand, i, j])
                } else if s == g.neg(t) {
                    Some(vec![summand, fb.neg_index[i], fb.neg_index[j]])
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::{prime_factor_base, prime_instance_for, GroupOps};
    use crate::cryptanalysis::ic_framework::plugins::RootfindSubtractOracle;
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

    /// Same verdicts and same charged additions as `rootfind-subtract`
    /// on a small prime curve (PLAN.md lever-1 landing criterion).
    #[test]
    fn floor_matches_subtract_verdicts_and_charged_adds_m2() {
        let inst = prime_instance_for(20, 59297);
        let fb = prime_factor_base(&inst, 48);
        let c = ctx(&inst);
        let (_, params) = Params::parse_spec("rootfind-floor:m=2").unwrap();
        let mut floor = RootfindFloorOracle::new(2, &inst);
        let mut sub = RootfindSubtractOracle::new(2, &inst);
        let mut build_f = GroupOps::default();
        let mut build_s = GroupOps::default();
        floor.prepare(&c, &fb, &params, &mut build_f).unwrap();
        sub.prepare(&c, &fb, &params, &mut build_s).unwrap();

        let mut state = 0xC0FFEE_u64;
        let mut agree = 0u64;
        let mut both_miss = 0u64;
        for _ in 0..256 {
            state = state.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
            let mut scratch = GroupOps::default();
            let t = inst.curve.mul(
                &mut scratch,
                inst.generator_point(),
                1 + state % (inst.r - 1),
            );
            let mut ops_f = GroupOps::default();
            let mut ops_s = GroupOps::default();
            let mut cf = OracleCounters::default();
            let mut cs = OracleCounters::default();
            let hit_f = floor.decompose(&c, &fb, &mut ops_f, &mut cf, t);
            let hit_s = sub.decompose(&c, &fb, &mut ops_s, &mut cs, t);
            assert_eq!(ops_f.adds, ops_s.adds, "charged additions must match");
            match (hit_f, hit_s) {
                (None, None) => both_miss += 1,
                (Some(a), Some(b)) => {
                    assert_eq!(a, b, "witnesses must match");
                    agree += 1;
                }
                (a, b) => panic!("verdict mismatch: floor={a:?} subtract={b:?}"),
            }
        }
        assert_eq!(floor.mismatches, 0);
        assert_eq!(sub.mismatches, 0);
        assert!(
            agree + both_miss == 256,
            "every target classified; hits={agree} misses={both_miss}"
        );
    }

    #[test]
    fn floor_matches_subtract_verdicts_m3() {
        let inst = prime_instance_for(18, 1);
        let fb = prime_factor_base(&inst, 24);
        let c = ctx(&inst);
        let (_, params) = Params::parse_spec("rootfind-floor:m=3").unwrap();
        let mut floor = RootfindFloorOracle::new(3, &inst);
        let mut sub = RootfindSubtractOracle::new(3, &inst);
        let mut build_f = GroupOps::default();
        let mut build_s = GroupOps::default();
        floor.prepare(&c, &fb, &params, &mut build_f).unwrap();
        sub.prepare(&c, &fb, &params, &mut build_s).unwrap();

        let mut state = 0xBADC0DE_u64;
        for _ in 0..128 {
            state = state.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(3);
            let mut scratch = GroupOps::default();
            let t = inst.curve.mul(
                &mut scratch,
                inst.generator_point(),
                1 + state % (inst.r - 1),
            );
            let mut ops_f = GroupOps::default();
            let mut ops_s = GroupOps::default();
            let mut cf = OracleCounters::default();
            let mut cs = OracleCounters::default();
            let hit_f = floor.decompose(&c, &fb, &mut ops_f, &mut cf, t);
            let hit_s = sub.decompose(&c, &fb, &mut ops_s, &mut cs, t);
            assert_eq!(ops_f.adds, ops_s.adds);
            assert_eq!(hit_f, hit_s);
        }
        assert_eq!(floor.mismatches, 0);
    }
}
