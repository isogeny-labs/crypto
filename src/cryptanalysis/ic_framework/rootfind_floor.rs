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
//!    pair — that doubled prepare wall and erased the e2e gain). The m=3
//!    PairTable is built with [`PairTable::build_negation_folded_prime_batch`]
//!    (PLAN lever 5) so prepare matches `rootfind-subtract:m=3` and scales.
//!
//! New id: never change what `rootfind-subtract` counts. Classification
//! is constant-factor engineering (wall / field work); charged `S` matches
//! `rootfind-subtract` by design. Fail-closed: every hit re-adds in the
//! group on the ledger.
//!
//! PLAN lever 2: [`RootfindFloorOracle::decompose_lanes`] probes up to eight
//! targets behind one shared batch-inv; on AVX-512 IFMA hosts with `p < 2^52`
//! it prefers an IFMA lockstep (canonical Bloom) for the miss path.

use std::collections::BTreeMap;
use std::sync::Arc;

use super::plugins::table_counted;
use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    fast_map, CountedGroup, FactorBase, FastMap, GroupOps, OracleCounters, PairTable, PrimeCurve,
    PrimeInstance, PrimePoint,
};
use crate::cryptanalysis::prime_fast::{Fp64, MAX_BITS as PRIME_FAST_MAX_BITS};

#[inline(always)]
fn prefetch_t0(p: *const u8) {
    #[cfg(target_arch = "x86_64")]
    // SAFETY: `_mm_prefetch` only hints; `p` need not be dereferenceable.
    unsafe {
        std::arch::x86_64::_mm_prefetch::<{ std::arch::x86_64::_MM_HINT_T0 }>(p as *const i8);
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = p;
}

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
    /// Canonical affine-x Bloom for the IFMA lane path (`m = 3`).
    canon_bloom: Option<WordBloom>,
    /// `m = 3`: negation-folded pair table; bloom-pass probes look up here
    /// (no parallel Montgomery witness map — that doubled prepare wall).
    table: Option<PairTable>,
    diffs: Vec<u64>,
    degenerate: Vec<bool>,
    scratch: Vec<u64>,
    #[cfg(target_arch = "x86_64")]
    ifma: Option<Arc<super::rootfind_kmitm_ifma::IfmaRight1>>,
    ifma_enabled: bool,
    #[cfg(target_arch = "x86_64")]
    ifma_prefix: Vec<[u64; 8]>,
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
            canon_bloom: None,
            table: None,
            diffs: Vec::new(),
            degenerate: Vec::new(),
            scratch: Vec::new(),
            #[cfg(target_arch = "x86_64")]
            ifma: None,
            ifma_enabled: true,
            #[cfg(target_arch = "x86_64")]
            ifma_prefix: Vec::new(),
            mismatches: 0,
            sweep_field_muls: 0,
            sweep_field_adds: 0,
            bloom_passes: 0,
            bloom_false_positives: 0,
        }
    }

    pub fn ifma_prepared(&self) -> bool {
        #[cfg(target_arch = "x86_64")]
        {
            self.ifma.is_some()
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            false
        }
    }

    pub fn set_ifma_enabled(&mut self, on: bool) {
        self.ifma_enabled = on;
    }

    pub fn ifma_enabled(&self) -> bool {
        self.ifma_enabled
    }

    /// PLAN lever 2: probe up to eight targets per chunk. Prefers IFMA when
    /// prepared and enabled; otherwise scalar shared batch-inv.
    pub fn decompose_lanes(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        targets: &[PrimePoint],
        out: &mut [Option<Vec<usize>>],
    ) {
        let n = targets.len().min(out.len());
        for slot in out.iter_mut().take(n) {
            *slot = None;
        }
        if n == 0 {
            return;
        }
        if n == 1 || self.field.is_none() || self.bloom.is_none() {
            for i in 0..n {
                out[i] = self.decompose(ctx, fb, ops, counters, targets[i]);
            }
            return;
        }
        const LANE_CAP: usize = 8;
        let mut base = 0;
        while base < n {
            let end = (base + LANE_CAP).min(n);
            #[cfg(target_arch = "x86_64")]
            if self.ifma.is_some() && self.ifma_enabled && self.m == 3 {
                self.probe_lanes_ifma(
                    ctx,
                    fb,
                    ops,
                    counters,
                    &targets[base..end],
                    &mut out[base..end],
                );
                base = end;
                continue;
            }
            self.probe_lanes_scalar(
                ctx,
                fb,
                ops,
                counters,
                &targets[base..end],
                &mut out[base..end],
            );
            base = end;
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
        self.canon_bloom = None;
        #[cfg(target_arch = "x86_64")]
        {
            self.ifma = None;
        }
        if self.m == 2 {
            for &k in self.x_map.keys() {
                bloom.insert(k);
            }
        } else {
            // PLAN lever 5: batched prime PairTable (one inv per abscissa
            // row) instead of serial `build_negation_folded`. Same keys /
            // witnesses as `rootfind-subtract:m=3` once that arm uses the
            // same builder; prepare wall drops sharply at large |Fₓ|.
            let table = PairTable::build_negation_folded_prime_batch(ctx.group, fb)?;
            ops.merge(table.build_ops);
            // Bloom over Montgomery abscissae of the table's folded keys
            // (x+1 → to_mont(x)). Misses skip from_mont; hits look up the
            // PairTable directly — no second FastMap of every entry.
            let n = table.entries.max(1) as usize;
            bloom = WordBloom::new(n);
            #[cfg(target_arch = "x86_64")]
            let want_ifma = (5..1u64 << 52).contains(&p)
                && super::rootfind_kmitm_ifma::has_ifma();
            #[cfg(not(target_arch = "x86_64"))]
            let want_ifma = false;
            let mut canon_bloom = if want_ifma {
                Some(WordBloom::new(n))
            } else {
                None
            };
            table.for_each_folded_key(|folded| {
                let x_aff = folded.wrapping_sub(1);
                bloom.insert(f.to_mont(x_aff));
                if let Some(cb) = canon_bloom.as_mut() {
                    cb.insert(x_aff);
                }
            });
            self.table = Some(table);
            self.canon_bloom = canon_bloom;
            #[cfg(target_arch = "x86_64")]
            if want_ifma {
                let xs: Vec<u64> = self
                    .reps
                    .iter()
                    .map(|r| f.from_mont(r.x))
                    .collect();
                let ys: Vec<u64> = self
                    .reps
                    .iter()
                    .map(|r| f.from_mont(r.y))
                    .collect();
                self.ifma =
                    super::rootfind_kmitm_ifma::IfmaRight1::new(p, &xs, &ys).map(Arc::new);
                self.ifma_prefix.clear();
            }
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
    /// Shared batch-inv across ≤8 independent floor probes (scalar lever 2).
    fn probe_lanes_scalar(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        targets: &[PrimePoint],
        out: &mut [Option<Vec<usize>>],
    ) {
        let Some(f) = self.field else {
            for (i, &t) in targets.iter().enumerate() {
                out[i] = self.decompose(ctx, fb, ops, counters, t);
            }
            return;
        };
        let k = self.reps.len();
        let t_n = targets.len();
        let total = k * t_n;
        if self.diffs.len() < total {
            self.diffs.resize(total, 0);
            self.scratch.resize(total, 0);
            self.degenerate.resize(total, false);
        }
        let mut xr = [0u64; 8];
        let mut yr = [0u64; 8];
        for (t, &target) in targets.iter().enumerate() {
            if target.infinity {
                continue;
            }
            xr[t] = f.to_mont(target.x);
            yr[t] = f.to_mont(target.y);
        }
        ops.field_ops.muls += 2 * t_n as u64;
        for t in 0..t_n {
            if targets[t].infinity {
                continue;
            }
            for i in 0..k {
                let slot = t * k + i;
                let d = f.sub(xr[t], self.reps[i].x);
                self.degenerate[slot] = d == 0;
                self.diffs[slot] = if d == 0 { f.one } else { d };
            }
        }
        f.batch_inv(&mut self.diffs[..total], &mut self.scratch);
        counters.inversions += 1;
        ops.field_ops.invs += 1;
        ops.field_ops.muls += 3 * total as u64;
        self.sweep_field_muls += 2 * t_n as u64 + 3 * total as u64;
        self.sweep_field_adds += total as u64;
        for i in 0..k {
            if i + 1 < k {
                prefetch_t0(&self.reps[i + 1] as *const _ as *const u8);
            }
            let rep = self.reps[i];
            for t in 0..t_n {
                if out[t].is_some() || targets[t].infinity {
                    continue;
                }
                let slot = t * k + i;
                if self.degenerate[slot] {
                    let g = ctx.group;
                    for summand in [rep.plus, rep.minus] {
                        let tt = g.add(ops, targets[t], g.neg(fb.points[summand]));
                        if let Some(found) =
                            self.lookup_full(ctx, fb, ops, counters, targets[t], summand, tt)
                        {
                            out[t] = Some(found);
                            break;
                        }
                    }
                    continue;
                }
                let inv = self.diffs[slot];
                let lam_minus = f.mul(f.add(yr[t], rep.y), inv);
                let lam_plus = f.mul(f.sub(yr[t], rep.y), inv);
                let base = f.add(xr[t], rep.x);
                let x_minus = f.sub(f.mul(lam_minus, lam_minus), base);
                let x_plus = f.sub(f.mul(lam_plus, lam_plus), base);
                for (summand, x3, lam) in [
                    (rep.plus, x_minus, lam_minus),
                    (rep.minus, x_plus, lam_plus),
                ] {
                    ops.adds += 1;
                    ops.field_ops.muls += 2;
                    self.sweep_field_muls += 2;
                    counters.lookups += 1;
                    let maybe = self.bloom.as_ref().map(|b| b.maybe(x3)).unwrap_or(false);
                    if !maybe {
                        continue;
                    }
                    self.bloom_passes += 1;
                    if let Some(found) =
                        self.lookup_x(ctx, fb, ops, counters, targets[t], summand, x3, lam, yr[t], f)
                    {
                        out[t] = Some(found);
                        break;
                    }
                    self.bloom_false_positives += 1;
                }
            }
        }
    }

    /// IFMA lockstep for m=3: canonical Bloom, PairTable lookup without
    /// from_mont on the miss path.
    #[cfg(target_arch = "x86_64")]
    fn probe_lanes_ifma(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        targets: &[PrimePoint],
        out: &mut [Option<Vec<usize>>],
    ) {
        let Some(ifma) = self.ifma.clone() else {
            self.probe_lanes_scalar(ctx, fb, ops, counters, targets, out);
            return;
        };
        let Some(f) = self.field else {
            self.probe_lanes_scalar(ctx, fb, ops, counters, targets, out);
            return;
        };
        let t_n = targets.len();
        let k = self.reps.len();
        // Degenerate same-abscissa rows: scalar fallback.
        for (t, &target) in targets.iter().enumerate() {
            if target.infinity {
                continue;
            }
            let tx = f.to_mont(target.x);
            ops.field_ops.muls += 1;
            let degenerates: Vec<(usize, usize)> = self
                .reps
                .iter()
                .filter(|r| r.x == tx)
                .map(|r| (r.plus, r.minus))
                .collect();
            let g = ctx.group;
            for (plus, minus) in degenerates {
                for summand in [plus, minus] {
                    let tt = g.add(ops, target, g.neg(fb.points[summand]));
                    self.sweep_field_adds += 1;
                    if let Some(found) =
                        self.lookup_full(ctx, fb, ops, counters, target, summand, tt)
                    {
                        out[t] = Some(found);
                        break;
                    }
                }
                if out[t].is_some() {
                    break;
                }
            }
        }
        let skip: [bool; 8] = std::array::from_fn(|i| i < t_n && (out[i].is_some() || targets[i].infinity));
        let bloom = self.canon_bloom.as_ref();
        let mut cands: Vec<(usize, usize, u64)> = Vec::new();
        let mut adds = 0u64;
        let mut bloom_rejects = 0u64;
        let mut bloom_passes = 0u64;
        let plus: Vec<usize> = self.reps.iter().map(|r| r.plus).collect();
        let minus: Vec<usize> = self.reps.iter().map(|r| r.minus).collect();
        let muls = {
            let prefix = &mut self.ifma_prefix;
            ifma.for_each_floor_pair(targets, prefix, |lane, j, x_minus, x_plus| {
                if skip[lane] {
                    return;
                }
                adds += 1;
                for (x, summand) in [(x_minus, plus[j]), (x_plus, minus[j])] {
                    let pass = bloom.map(|b| b.maybe(x)).unwrap_or(true);
                    if pass {
                        bloom_passes += 1;
                        cands.push((lane, summand, x));
                    } else {
                        bloom_rejects += 1;
                    }
                }
            })
        };
        counters.inversions += 1;
        ops.field_ops.invs += 1;
        ops.field_ops.muls += muls;
        ops.adds += adds;
        self.sweep_field_muls += muls;
        self.sweep_field_adds += adds;
        self.bloom_passes += bloom_passes;
        let _ = bloom_rejects;
        for &(lane, summand, x_aff) in &cands {
            if out[lane].is_some() {
                continue;
            }
            counters.lookups += 1;
            if let Some(found) =
                self.lookup_x_canon(ctx, fb, ops, counters, targets[lane], summand, x_aff, f)
            {
                out[lane] = Some(found);
            } else {
                self.bloom_false_positives += 1;
            }
        }
        let _ = k;
    }

    /// Canonical-x bloom pass (`m = 3`): PairTable lookup without `from_mont`.
    #[allow(clippy::too_many_arguments)]
    fn lookup_x_canon(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PrimePoint,
        summand: usize,
        x_aff: u64,
        f: Fp64,
    ) -> Option<Vec<usize>> {
        let g = ctx.group;
        let table = self.table.as_ref()?;
        let (i, j) = table.lookup_folded(x_aff.wrapping_add(1))?;
        // Recompute λ on the rare hit path (Montgomery).
        let xr = f.to_mont(point.x);
        let yr = f.to_mont(point.y);
        let rep = self.reps.iter().find(|r| r.plus == summand || r.minus == summand)?;
        let dx = f.sub(xr, rep.x);
        if dx == 0 {
            return None;
        }
        let inv = f.inv(dx);
        ops.field_ops.invs += 1;
        ops.field_ops.muls += 3; // to_mont×2 counted below + inv internals approx
        ops.field_ops.muls += 2;
        let lam = if summand == rep.plus {
            f.mul(f.add(yr, rep.y), inv)
        } else {
            f.mul(f.sub(yr, rep.y), inv)
        };
        ops.field_ops.muls += 1;
        let x3 = f.to_mont(x_aff);
        let y3 = f.sub(f.mul(lam, f.sub(xr, x3)), yr);
        let y_aff = f.from_mont(y3);
        ops.field_ops.muls += 3; // to_mont(x) + λ·Δx + from_mont(y)
        let t = PrimePoint::affine(x_aff, y_aff);
        let s = g.add(ops, fb.points[i], fb.points[j]);
        if s == t {
            Some(vec![summand, i, j])
        } else if s == g.neg(t) {
            Some(vec![summand, fb.neg_index[i], fb.neg_index[j]])
        } else {
            let _ = counters;
            None
        }
    }

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

    /// IFMA / scalar lanes recover the same m=3 verdicts as single decompose.
    #[test]
    fn floor_lanes_match_single_m3() {
        let inst = prime_instance_for(20, 59297);
        let fb = prime_factor_base(&inst, 32);
        let c = ctx(&inst);
        let (_, params) = Params::parse_spec("rootfind-floor:m=3").unwrap();
        let mut floor = RootfindFloorOracle::new(3, &inst);
        floor
            .prepare(&c, &fb, &params, &mut GroupOps::default())
            .unwrap();
        let mut state = 0xF100_00A5_u64;
        let mut targets = Vec::new();
        for _ in 0..8 {
            state = state.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
            let mut scratch = GroupOps::default();
            targets.push(inst.curve.mul(
                &mut scratch,
                inst.generator_point(),
                1 + state % (inst.r - 1),
            ));
        }
        let mut single = Vec::new();
        for &t in &targets {
            let mut o = GroupOps::default();
            let mut cters = OracleCounters::default();
            single.push(floor.decompose(&c, &fb, &mut o, &mut cters, t));
        }
        for enable in [false, true] {
            if enable && !floor.ifma_prepared() {
                continue;
            }
            floor.set_ifma_enabled(enable);
            let mut out = vec![None; 8];
            let mut o = GroupOps::default();
            let mut cters = OracleCounters::default();
            floor.decompose_lanes(&c, &fb, &mut o, &mut cters, &targets, &mut out);
            for i in 0..8 {
                assert_eq!(out[i], single[i], "mode ifma={enable} lane {i}");
            }
        }
    }
}
