//! `rootfind-kmitm`: meet-in-the-middle root-finding for arity `m = 3..=12`.
//!
//! Full [`KSumTable`] stops at `m = 9` (u64 packing) and costs about
//! `2^{m−1}·C(|F|, m)` build additions. This oracle stores a **left**
//! `L`-sum table and probes with every **right** `R`-sum `S` via `R − S`
//! (here `R` is the target). Overlap between left and right summands is
//! rejected.
//!
//! Default split is **probe-favoring**: `L = min(9, m−1)`, `R = m−L`.
//! That makes `R = 1` for every `m ≤ 10`. Pass `split=balanced` for
//! `⌈m/2⌉ / ⌊m/2⌋` when the left table would be too large.
//!
//! On primes with siblings, probes use a Montgomery **batch inversion**
//! plus x-only left lookups (lever-1 style from `rootfind-floor`):
//! `R = 1` batches over the factor base; `R ≥ 2` batches over the
//! precomputed right-half sums. A word-blocked **Bloom filter** over
//! Montgomery abscissae sits in front of a mont→witness map (no
//! `from_mont` on the miss path; lever-3 style). Prepared tables are
//! `Arc`-shared so [`RootfindKMitmOracle::fork_worker`] /
//! [`RootfindKMitmOracle::decompose_threaded`] can run PLAN lever 4
//! (threads over targets) without rebuilding. Classification:
//! engineering (wall / build size); charged group additions are the
//! MITM's own (not equal to full `ksum`).

use std::collections::BTreeMap;
use std::sync::Arc;

use super::plugins::table_counted;
use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    fast_map, CountedGroup, FactorBase, FastMap, GroupOps, KSumTable, OracleCounters, PairTable,
    PrimeCurve, PrimeInstance, PrimePoint, KSUM_MAX_K,
};
use crate::cryptanalysis::prime_fast::{Fp64, MAX_BITS as PRIME_FAST_MAX_BITS};

/// Word-blocked Bloom filter: one 64-bit word a key, five bits in it
/// (same layout as `rootfind-floor`).
#[derive(Clone)]
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

    /// Lookup by folded abscissa key (`x + 1`), no group ops.
    fn lookup_folded(&self, folded: u64) -> Option<Vec<usize>> {
        match self {
            Self::Pairs(t) => {
                let (i, j) = t.lookup_folded(folded)?;
                Some(vec![i, j])
            }
            Self::KSum(t) => t.lookup_folded(folded),
        }
    }

    fn for_each_folded_key(&self, f: impl FnMut(u64)) {
        match self {
            Self::Pairs(t) => t.for_each_folded_key(f),
            Self::KSum(t) => t.for_each_folded_key(f),
        }
    }
}

/// Cap on right-half summand count (`m ≤ 12` ⇒ `right_k ≤ 6` under either
/// split). Inline storage avoids a heap clone on every bloom-pass hit.
const RIGHT_IDX_CAP: usize = 8;

/// One precomputed right-half sum (used when `right_k ≥ 2`).
struct RightSum<P> {
    point: P,
    idxs: [usize; RIGHT_IDX_CAP],
    n_idxs: u8,
    /// Montgomery abscissa / ordinate when the prime batch-inv probe is on.
    mont_x: u64,
    mont_y: u64,
}

impl<P> RightSum<P> {
    #[inline]
    fn idxs_slice(&self) -> &[usize] {
        &self.idxs[..usize::from(self.n_idxs)]
    }
}

/// Meet-in-the-middle k-sum root finding for `m = 3..=12`.
pub struct RootfindKMitmOracle<'i> {
    m: u32,
    /// Optional prime instance: enables `sib=1` on the left table when set.
    instance: Option<&'i PrimeInstance>,
    left_k: u32,
    right_k: u32,
    balanced: bool,
    table: Option<Arc<LeftTable>>,
    /// Montgomery field for the prime batch-inv probe (lever-1 style).
    field: Option<Fp64>,
    /// Bloom filter over left-table folded keys (lever-3 style).
    bloom: Option<Arc<WordBloom>>,
    /// Montgomery-abscissa → packed left witness (pairs: `i<<32|j`; k-sum:
    /// native packed). Lets the probe skip `from_mont` on every bloom pass.
    mont_left: Arc<FastMap<u64>>,
    /// When true, `mont_left` values are pair witnesses (`i<<32|j`).
    mont_left_pairs: bool,
    /// Bits per summand for unpacking k-sum `mont_left` values.
    mont_left_bits: u32,
    /// Summand count encoded in each k-sum `mont_left` value.
    mont_left_k: u32,
    /// Precomputed right-half sums for prime probes (`right_k ≥ 2`).
    right_prime: Arc<Vec<RightSum<PrimePoint>>>,
    /// Precomputed right-half sums for binary probes (`right_k ≥ 2`).
    right_bin: Arc<Vec<RightSum<crate::cryptanalysis::koblitz_fast::FastPoint>>>,
    /// Factor-base Montgomery (x, y) for the right=1 batch probe.
    fb_mont: Arc<Vec<(u64, u64)>>,
    /// Scratch for the batch-inv probe.
    diffs: Vec<u64>,
    scratch: Vec<u64>,
    /// Skip mask for same-abscissa / identity slots in the batch-inv pass.
    degenerate: Vec<bool>,
    /// Reused left-witness buffer (avoids a Vec alloc on every bloom pass).
    left_scratch: Vec<usize>,
    pub mismatches: u64,
    probe_right_adds: u64,
    probe_lookups: u64,
    overlaps_rejected: u64,
    /// Probes that used the batch-inv / x-only key path.
    probe_batch_inv: u64,
    bloom_rejects: u64,
    bloom_passes: u64,
    bloom_false_positives: u64,
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
            field: None,
            bloom: None,
            mont_left: Arc::new(fast_map(0)),
            mont_left_pairs: false,
            mont_left_bits: 0,
            mont_left_k: 0,
            right_prime: Arc::new(Vec::new()),
            right_bin: Arc::new(Vec::new()),
            fb_mont: Arc::new(Vec::new()),
            diffs: Vec::new(),
            scratch: Vec::new(),
            degenerate: Vec::new(),
            left_scratch: Vec::new(),
            mismatches: 0,
            probe_right_adds: 0,
            probe_lookups: 0,
            overlaps_rejected: 0,
            probe_batch_inv: 0,
            bloom_rejects: 0,
            bloom_passes: 0,
            bloom_false_positives: 0,
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
        self.table.as_deref().map(LeftTable::entries).unwrap_or(0)
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

    /// PLAN lever 2 (scalar): probe up to eight targets behind **one** shared
    /// Montgomery batch-inversion per chunk of ≤8. `out[i]` receives the
    /// witness for `targets[i]` (or `None`). Falls back to one-at-a-time when
    /// the batch-inv path is off or `targets.len() ≤ 1`. Larger slices are
    /// processed in successive 8-wide chunks (AVX/IFMA lockstep still open).
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
        if n == 1 || self.field.is_none() || self.table.is_none() {
            for i in 0..n {
                out[i] = self.decompose(ctx, fb, ops, counters, targets[i]);
            }
            return;
        }
        let g = ctx.group;
        const LANE_CAP: usize = 8;
        let mut base = 0;
        while base < n {
            let end = (base + LANE_CAP).min(n);
            if self.right_k == 1 {
                self.probe_right1_prime_lanes(
                    g,
                    fb,
                    ops,
                    counters,
                    &targets[base..end],
                    &mut out[base..end],
                );
            } else {
                self.probe_right_precomputed_prime_lanes(
                    g,
                    fb,
                    ops,
                    counters,
                    &targets[base..end],
                    &mut out[base..end],
                );
            }
            base = end;
        }
    }

    /// PLAN lever 4: independent worker sharing this oracle's prepared
    /// tables (`Arc`). Scratch buffers and counters start empty.
    pub fn fork_worker(&self) -> Self {
        let n = self.diffs.len().max(1);
        Self {
            m: self.m,
            instance: self.instance,
            left_k: self.left_k,
            right_k: self.right_k,
            balanced: self.balanced,
            table: self.table.clone(),
            field: self.field,
            bloom: self.bloom.clone(),
            mont_left: Arc::clone(&self.mont_left),
            mont_left_pairs: self.mont_left_pairs,
            mont_left_bits: self.mont_left_bits,
            mont_left_k: self.mont_left_k,
            right_prime: Arc::clone(&self.right_prime),
            right_bin: Arc::clone(&self.right_bin),
            fb_mont: Arc::clone(&self.fb_mont),
            diffs: vec![0; n],
            scratch: vec![0; n],
            degenerate: vec![false; n],
            left_scratch: Vec::with_capacity(self.left_k as usize),
            mismatches: 0,
            probe_right_adds: 0,
            probe_lookups: 0,
            overlaps_rejected: 0,
            probe_batch_inv: 0,
            bloom_rejects: 0,
            bloom_passes: 0,
            bloom_false_positives: 0,
        }
    }

    /// PLAN lever 4: probe every target across `threads` workers that share
    /// the prepared tables (`Arc`). Uses `std::thread::scope` (no rayon pool
    /// build). Return order matches `targets`. Engineering throughput only —
    /// charged `S` is unchanged.
    pub fn decompose_threaded(
        &self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        targets: &[PrimePoint],
        threads: usize,
    ) -> Vec<Option<Vec<usize>>> {
        let threads = threads.max(1).min(targets.len().max(1));
        let mut out = vec![None; targets.len()];
        if threads <= 1 || targets.len() <= 1 {
            let mut worker = self.fork_worker();
            for (slot, &t) in out.iter_mut().zip(targets) {
                let mut ops = GroupOps::default();
                let mut c = OracleCounters::default();
                *slot = worker.decompose(ctx, fb, &mut ops, &mut c, t);
            }
            return out;
        }
        let chunk = targets.len().div_ceil(threads);
        std::thread::scope(|scope| {
            let mut rest_out = out.as_mut_slice();
            let mut rest_tgt = targets;
            while !rest_tgt.is_empty() {
                let n = chunk.min(rest_tgt.len());
                let (out_chunk, out_next) = rest_out.split_at_mut(n);
                let (tgt_chunk, tgt_next) = rest_tgt.split_at(n);
                rest_out = out_next;
                rest_tgt = tgt_next;
                scope.spawn(move || {
                    let mut worker = self.fork_worker();
                    for (slot, &t) in out_chunk.iter_mut().zip(tgt_chunk) {
                        let mut ops = GroupOps::default();
                        let mut c = OracleCounters::default();
                        *slot = worker.decompose(ctx, fb, &mut ops, &mut c, t);
                    }
                });
            }
        });
        out
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
        debug_assert!(right_k <= RIGHT_IDX_CAP);
        let mut buf = [0usize; RIGHT_IDX_CAP];
        buf[..right_k].copy_from_slice(&idxs[..right_k]);
        out.push(RightSum {
            point,
            idxs: buf,
            n_idxs: right_k as u8,
            mont_x: 0,
            mont_y: 0,
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
             (m={}, split={mode}; Bloom + batch-inv on primes; enables m=10..12)",
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

        self.table = Some(Arc::new(build_left_table(
            ctx.group, fb, left_k, prune, sib, ops,
        )?));
        self.right_bin = Arc::new(Vec::new());
        self.right_prime = Arc::new(Vec::new());
        self.fb_mont = Arc::new(Vec::new());
        let batch_ok = ctx.group.supports_siblings()
            && (3..1u64 << PRIME_FAST_MAX_BITS).contains(&ctx.group.p);
        // Lever-3: Bloom over Montgomery abscissae of every left folded key
        // (`folded = x+1` affine → insert `to_mont(x)`). Probe checks the
        // mont x₃ before any table lookup, so miss-dominated paths skip
        // `from_mont`. A parallel mont→witness map makes bloom-pass hits
        // avoid the affine conversion entirely.
        let entries = self.table.as_deref().map(LeftTable::entries).unwrap_or(1) as usize;
        let mut bloom = WordBloom::new(entries.max(1));
        self.mont_left = Arc::new(fast_map(0));
        self.mont_left_pairs = left_k == 2;
        self.mont_left_k = left_k;
        self.mont_left_bits = if left_k >= 3 {
            crate::cryptanalysis::ic_boundary::ksum_index_bits(left_k)
        } else {
            0
        };
        if batch_ok {
            let f = Fp64::new(ctx.group.p);
            let mut mont_left = fast_map(entries.max(1));
            if let Some(t) = self.table.as_deref() {
                let pairs = left_k == 2;
                let bits = self.mont_left_bits;
                let k = left_k as usize;
                t.for_each_folded_key(|folded| {
                    let xm = f.to_mont(folded.wrapping_sub(1));
                    bloom.insert(xm);
                    let Some(left) = t.lookup_folded(folded) else {
                        return;
                    };
                    let packed = if pairs {
                        ((left[0] as u64) << 32) | (left[1] as u64)
                    } else {
                        left.iter()
                            .take(k)
                            .rev()
                            .fold(0u64, |w, &i| (w << bits) | (i as u64))
                    };
                    mont_left.entry(xm).or_insert(packed);
                });
            }
            self.mont_left = Arc::new(mont_left);
        } else if let Some(t) = self.table.as_deref() {
            t.for_each_folded_key(|k| bloom.insert(k));
        }
        self.bloom = Some(Arc::new(bloom));
        if right_k >= 2 {
            let mut right = Vec::new();
            collect_right_sums(ctx.group, fb, right_k, ops, &mut right);
            if batch_ok {
                let f = Fp64::new(ctx.group.p);
                for e in &mut right {
                    e.mont_x = f.to_mont(e.point.x);
                    e.mont_y = f.to_mont(e.point.y);
                }
            }
            self.right_prime = Arc::new(right);
        }
        // Montgomery batch-inv for right=1 (over F) and right≥2 (over S).
        self.field = if batch_ok {
            let f = Fp64::new(ctx.group.p);
            let n = if right_k == 1 {
                let mut fb_mont = Vec::with_capacity(fb.points.len());
                for &pj in &fb.points {
                    fb_mont.push((f.to_mont(pj.x), f.to_mont(pj.y)));
                }
                let n = fb_mont.len();
                self.fb_mont = Arc::new(fb_mont);
                n
            } else {
                self.right_prime.len()
            };
            self.diffs.resize(n.max(1), 0);
            self.scratch.resize(n.max(1), 0);
            self.degenerate.resize(n.max(1), false);
            Some(f)
        } else {
            self.diffs.clear();
            self.scratch.clear();
            self.degenerate.clear();
            self.fb_mont = Arc::new(Vec::new());
            None
        };
        self.mismatches = 0;
        self.probe_right_adds = 0;
        self.probe_lookups = 0;
        self.overlaps_rejected = 0;
        self.probe_batch_inv = 0;
        self.bloom_rejects = 0;
        self.bloom_passes = 0;
        self.bloom_false_positives = 0;
        Ok(())
    }

    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        let mut out = self
            .table
            .as_deref()
            .map(LeftTable::work_counters)
            .unwrap_or_default();
        out.insert("kmitm_left_k".to_string(), u64::from(self.left_k));
        out.insert("kmitm_right_k".to_string(), u64::from(self.right_k));
        out.insert("kmitm_split_balanced".to_string(), u64::from(self.balanced));
        if let Some(t) = self.table.as_deref() {
            out.insert("kmitm_left_entries".to_string(), t.entries());
        }
        if !self.right_prime.is_empty() {
            out.insert(
                "kmitm_right_precomputed".to_string(),
                self.right_prime.len() as u64,
            );
        }
        if let Some(b) = self.bloom.as_deref() {
            out.insert("kmitm_bloom_words".to_string(), b.words.len() as u64);
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
        if self.probe_batch_inv > 0 {
            out.insert("kmitm_probe_batch_inv".to_string(), self.probe_batch_inv);
        }
        if self.bloom_rejects > 0 {
            out.insert("kmitm_bloom_rejects".to_string(), self.bloom_rejects);
        }
        if self.bloom_passes > 0 {
            out.insert("kmitm_bloom_passes".to_string(), self.bloom_passes);
        }
        if self.bloom_false_positives > 0 {
            out.insert(
                "kmitm_bloom_false_positives".to_string(),
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
        // Lever-1 style: one batched inversion for every R ∓ F_j abscissa,
        // then folded-key lookup into the left table (no full point build on
        // misses). Falls back to group-add probes when siblings are unavailable.
        if self.field.is_some() {
            if let Some(hit) = self.probe_right1_prime_batch(g, fb, ops, counters, target) {
                return Some(hit);
            }
            // Batch path enumerates every candidate; a miss is final.
            return None;
        }
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

    /// Batch-inv + x-only folded keys for every factor-base point (right=1).
    fn probe_right1_prime_batch(
        &mut self,
        g: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: PrimePoint,
    ) -> Option<Vec<usize>> {
        let f = self.field?;
        let n = fb.points.len();
        if self.diffs.len() < n {
            self.diffs.resize(n, 0);
            self.scratch.resize(n, 0);
            self.degenerate.resize(n, false);
        }
        debug_assert_eq!(self.fb_mont.len(), n);
        let xr = f.to_mont(target.x);
        let yr = f.to_mont(target.y);
        ops.field_ops.muls += 2;
        let mut any_active = false;
        for j in 0..n {
            let pj = fb.points[j];
            if g.is_identity(&pj) {
                self.degenerate[j] = true;
                self.diffs[j] = f.one;
                continue;
            }
            if pj.x == target.x {
                // Same abscissa: sibling path refuses; try group add.
                self.degenerate[j] = true;
                self.diffs[j] = f.one;
                let t = g.add(ops, target, g.neg(pj));
                self.probe_right_adds += 1;
                let mut lookups = 0u64;
                if let Some(left) = self.table.as_ref()?.probe_left(g, fb, ops, &mut lookups, t) {
                    self.probe_lookups += lookups;
                    counters.lookups += lookups;
                    if let Some(hit) = self.finish_hit_prime(
                        g,
                        fb,
                        ops,
                        counters,
                        target,
                        left,
                        std::slice::from_ref(&j),
                    ) {
                        return Some(hit);
                    }
                } else {
                    self.probe_lookups += lookups;
                    counters.lookups += lookups;
                }
                continue;
            }
            self.degenerate[j] = false;
            // dx = x_J − x_R (same as CountedGroup::sibling_pair).
            self.diffs[j] = f.sub(self.fb_mont[j].0, xr);
            any_active = true;
        }
        if !any_active {
            return None;
        }
        // In-place batch-inv over the full slot array (degenerate slots hold 1).
        f.batch_inv(&mut self.diffs[..n], &mut self.scratch);
        counters.inversions += 1;
        ops.field_ops.invs += 1;
        ops.field_ops.muls += 3 * n as u64;
        self.probe_batch_inv += 1;
        for j in 0..n {
            if self.degenerate[j] {
                continue;
            }
            let inv = self.diffs[j];
            let (xj, yj) = self.fb_mont[j];
            let dy = f.sub(yj, yr);
            let sy = f.add(yj, yr);
            let lp = f.mul(dy, inv);
            let lm = f.mul(f.sub(0, sy), inv);
            let base = f.add(xr, xj);
            // Keep x₃ in Montgomery form for the Bloom check; from_mont only
            // on a pass (folded key for the left table is still affine x+1).
            let xp_m = f.sub(f.mul(lp, lp), base);
            let xm_m = f.sub(f.mul(lm, lm), base);
            // One sibling pair = one charged add (matches sibling_pair callers).
            ops.adds += 1;
            self.probe_right_adds += 1;
            // key_plus = x(R+F_j) → R = left + (−F_j); key_minus → R = left + F_j.
            let candidates = [(xp_m, fb.neg_index[j]), (xm_m, j)];
            for &(x3_m, right_j) in &candidates {
                if !self.bloom_maybe(x3_m) {
                    continue;
                }
                self.probe_lookups += 1;
                counters.lookups += 1;
                if !self.lookup_mont_into(x3_m) {
                    self.bloom_false_positives += 1;
                    continue;
                }
                let left = std::mem::take(&mut self.left_scratch);
                let hit = self.finish_hit_prime(
                    g,
                    fb,
                    ops,
                    counters,
                    target,
                    left,
                    std::slice::from_ref(&right_j),
                );
                // Restore scratch capacity even on a miss.
                if self.left_scratch.capacity() == 0 {
                    self.left_scratch = Vec::with_capacity(self.left_k as usize);
                }
                if let Some(hit) = hit {
                    return Some(hit);
                }
            }
        }
        None
    }

    /// Unpack a Montgomery-abscissa left witness into [`Self::left_scratch`].
    #[inline]
    fn lookup_mont_into(&mut self, xm: u64) -> bool {
        self.left_scratch.clear();
        let Some(&packed) = self.mont_left.get(&xm) else {
            return false;
        };
        if self.mont_left_pairs {
            self.left_scratch.push((packed >> 32) as usize);
            self.left_scratch.push((packed & 0xffff_ffff) as usize);
        } else {
            let bits = self.mont_left_bits;
            let mask = (1u64 << bits) - 1;
            let k = self.mont_left_k as usize;
            self.left_scratch.reserve(k);
            for d in 0..k {
                self.left_scratch
                    .push(((packed >> (d as u32 * bits)) & mask) as usize);
            }
        }
        true
    }

    #[inline]
    fn bloom_maybe(&mut self, folded: u64) -> bool {
        match &self.bloom {
            Some(b) if b.maybe(folded) => {
                self.bloom_passes += 1;
                true
            }
            Some(_) => {
                self.bloom_rejects += 1;
                false
            }
            None => true,
        }
    }

    fn probe_right_precomputed_prime(
        &mut self,
        g: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: PrimePoint,
    ) -> Option<Vec<usize>> {
        if self.field.is_some() {
            if let Some(hit) =
                self.probe_right_precomputed_prime_batch(g, fb, ops, counters, target)
            {
                return Some(hit);
            }
            // Batch path enumerates every right sum; a miss is final.
            return None;
        }
        let n = self.right_prime.len();
        for i in 0..n {
            let s = self.right_prime[i].point;
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
            // Copy idxs before the &mut self call (inline, no heap).
            let mut right_buf = [0usize; RIGHT_IDX_CAP];
            let n_r = usize::from(self.right_prime[i].n_idxs);
            right_buf[..n_r].copy_from_slice(self.right_prime[i].idxs_slice());
            if let Some(hit) =
                self.finish_hit_prime(g, fb, ops, counters, target, left, &right_buf[..n_r])
            {
                return Some(hit);
            }
        }
        None
    }

    /// Batch-inv + x-only folded keys over every precomputed right sum.
    fn probe_right_precomputed_prime_batch(
        &mut self,
        g: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: PrimePoint,
    ) -> Option<Vec<usize>> {
        let f = self.field?;
        let n = self.right_prime.len();
        if n == 0 {
            return None;
        }
        if self.diffs.len() < n {
            self.diffs.resize(n, 0);
            self.scratch.resize(n, 0);
            self.degenerate.resize(n, false);
        }
        let xr = f.to_mont(target.x);
        let yr = f.to_mont(target.y);
        ops.field_ops.muls += 2;
        let mut any_active = false;
        for i in 0..n {
            let e = &self.right_prime[i];
            if e.point.x == target.x {
                // Same abscissa: fall back to a full group subtract.
                self.degenerate[i] = true;
                self.diffs[i] = f.one;
                let t = g.add(ops, target, g.neg(e.point));
                self.probe_right_adds += 1;
                let mut right_buf = [0usize; RIGHT_IDX_CAP];
                let n_r = usize::from(e.n_idxs);
                right_buf[..n_r].copy_from_slice(e.idxs_slice());
                let mut lookups = 0u64;
                if let Some(left) = self.table.as_ref()?.probe_left(g, fb, ops, &mut lookups, t) {
                    self.probe_lookups += lookups;
                    counters.lookups += lookups;
                    if let Some(hit) =
                        self.finish_hit_prime(g, fb, ops, counters, target, left, &right_buf[..n_r])
                    {
                        return Some(hit);
                    }
                } else {
                    self.probe_lookups += lookups;
                    counters.lookups += lookups;
                }
                continue;
            }
            self.degenerate[i] = false;
            self.diffs[i] = f.sub(e.mont_x, xr);
            any_active = true;
        }
        if !any_active {
            return None;
        }
        f.batch_inv(&mut self.diffs[..n], &mut self.scratch);
        counters.inversions += 1;
        ops.field_ops.invs += 1;
        ops.field_ops.muls += 3 * n as u64;
        self.probe_batch_inv += 1;
        for i in 0..n {
            if self.degenerate[i] {
                continue;
            }
            let inv = self.diffs[i];
            let (xs, ys) = {
                let e = &self.right_prime[i];
                (e.mont_x, e.mont_y)
            };
            // Only R − S = R + (−S): lambda = (−y_S − y_R) / (x_S − x_R).
            let sy = f.add(ys, yr);
            let lm = f.mul(f.sub(0, sy), inv);
            let base = f.add(xr, xs);
            let xm_m = f.sub(f.mul(lm, lm), base);
            ops.adds += 1;
            self.probe_right_adds += 1;
            if !self.bloom_maybe(xm_m) {
                continue;
            }
            self.probe_lookups += 1;
            counters.lookups += 1;
            if !self.lookup_mont_into(xm_m) {
                self.bloom_false_positives += 1;
                continue;
            }
            let left = std::mem::take(&mut self.left_scratch);
            let mut right_buf = [0usize; RIGHT_IDX_CAP];
            let n_r = usize::from(self.right_prime[i].n_idxs);
            right_buf[..n_r].copy_from_slice(self.right_prime[i].idxs_slice());
            let hit = self.finish_hit_prime(g, fb, ops, counters, target, left, &right_buf[..n_r]);
            if self.left_scratch.capacity() == 0 {
                self.left_scratch = Vec::with_capacity(self.left_k as usize);
            }
            if let Some(hit) = hit {
                return Some(hit);
            }
        }
        None
    }

    /// Shared batch-inv across `targets.len()` independent right=1 probes.
    fn probe_right1_prime_lanes(
        &mut self,
        g: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        targets: &[PrimePoint],
        out: &mut [Option<Vec<usize>>],
    ) {
        let Some(f) = self.field else {
            for (i, &t) in targets.iter().enumerate() {
                out[i] = self.probe_right1_prime(g, fb, ops, counters, t);
            }
            return;
        };
        let n = fb.points.len();
        let t_n = targets.len();
        debug_assert_eq!(self.fb_mont.len(), n);
        let total = n * t_n;
        if self.diffs.len() < total {
            self.diffs.resize(total, 0);
            self.scratch.resize(total, 0);
            self.degenerate.resize(total, false);
        }
        debug_assert!(t_n <= 8, "lane chunk cap is 8");
        let mut xr = [0u64; 8];
        let mut yr = [0u64; 8];
        for (t, &target) in targets.iter().enumerate() {
            xr[t] = f.to_mont(target.x);
            yr[t] = f.to_mont(target.y);
        }
        ops.field_ops.muls += 2 * t_n as u64;
        let mut any_active = false;
        for t in 0..t_n {
            let target = targets[t];
            for j in 0..n {
                let slot = t * n + j;
                let pj = fb.points[j];
                if g.is_identity(&pj) {
                    self.degenerate[slot] = true;
                    self.diffs[slot] = f.one;
                    continue;
                }
                if pj.x == target.x {
                    self.degenerate[slot] = true;
                    self.diffs[slot] = f.one;
                    let tt = g.add(ops, target, g.neg(pj));
                    self.probe_right_adds += 1;
                    let mut lookups = 0u64;
                    if let Some(left) = self
                        .table
                        .as_ref()
                        .and_then(|tb| tb.probe_left(g, fb, ops, &mut lookups, tt))
                    {
                        self.probe_lookups += lookups;
                        counters.lookups += lookups;
                        if out[t].is_none() {
                            if let Some(hit) = self.finish_hit_prime(
                                g,
                                fb,
                                ops,
                                counters,
                                target,
                                left,
                                std::slice::from_ref(&j),
                            ) {
                                out[t] = Some(hit);
                            }
                        }
                    } else {
                        self.probe_lookups += lookups;
                        counters.lookups += lookups;
                    }
                    continue;
                }
                self.degenerate[slot] = false;
                self.diffs[slot] = f.sub(self.fb_mont[j].0, xr[t]);
                any_active = true;
            }
        }
        if !any_active {
            return;
        }
        f.batch_inv(&mut self.diffs[..total], &mut self.scratch);
        counters.inversions += 1;
        ops.field_ops.invs += 1;
        ops.field_ops.muls += 3 * total as u64;
        self.probe_batch_inv += 1;
        // Interleave targets inside the right-sum loop so multiply latency
        // overlaps across independent probes.
        for j in 0..n {
            let (xj, yj) = self.fb_mont[j];
            for t in 0..t_n {
                if out[t].is_some() {
                    continue;
                }
                let slot = t * n + j;
                if self.degenerate[slot] {
                    continue;
                }
                let inv = self.diffs[slot];
                let dy = f.sub(yj, yr[t]);
                let sy = f.add(yj, yr[t]);
                let lp = f.mul(dy, inv);
                let lm = f.mul(f.sub(0, sy), inv);
                let base = f.add(xr[t], xj);
                let xp_m = f.sub(f.mul(lp, lp), base);
                let xm_m = f.sub(f.mul(lm, lm), base);
                ops.adds += 1;
                self.probe_right_adds += 1;
                let candidates = [(xp_m, fb.neg_index[j]), (xm_m, j)];
                for &(x3_m, right_j) in &candidates {
                    if !self.bloom_maybe(x3_m) {
                        continue;
                    }
                    self.probe_lookups += 1;
                    counters.lookups += 1;
                    if !self.lookup_mont_into(x3_m) {
                        self.bloom_false_positives += 1;
                        continue;
                    }
                    let left = std::mem::take(&mut self.left_scratch);
                    let hit = self.finish_hit_prime(
                        g,
                        fb,
                        ops,
                        counters,
                        targets[t],
                        left,
                        std::slice::from_ref(&right_j),
                    );
                    if self.left_scratch.capacity() == 0 {
                        self.left_scratch = Vec::with_capacity(self.left_k as usize);
                    }
                    if let Some(hit) = hit {
                        out[t] = Some(hit);
                        break;
                    }
                }
            }
        }
    }

    /// Shared batch-inv across `targets.len()` independent right≥2 probes.
    fn probe_right_precomputed_prime_lanes(
        &mut self,
        g: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        targets: &[PrimePoint],
        out: &mut [Option<Vec<usize>>],
    ) {
        let Some(f) = self.field else {
            for (i, &t) in targets.iter().enumerate() {
                out[i] = self.probe_right_precomputed_prime(g, fb, ops, counters, t);
            }
            return;
        };
        let n = self.right_prime.len();
        if n == 0 {
            return;
        }
        let t_n = targets.len();
        let total = n * t_n;
        if self.diffs.len() < total {
            self.diffs.resize(total, 0);
            self.scratch.resize(total, 0);
            self.degenerate.resize(total, false);
        }
        debug_assert!(t_n <= 8, "lane chunk cap is 8");
        let mut xr = [0u64; 8];
        let mut yr = [0u64; 8];
        for (t, &target) in targets.iter().enumerate() {
            xr[t] = f.to_mont(target.x);
            yr[t] = f.to_mont(target.y);
        }
        ops.field_ops.muls += 2 * t_n as u64;
        let mut any_active = false;
        for t in 0..t_n {
            let target = targets[t];
            for i in 0..n {
                let slot = t * n + i;
                let e = &self.right_prime[i];
                if e.point.x == target.x {
                    self.degenerate[slot] = true;
                    self.diffs[slot] = f.one;
                    let tt = g.add(ops, target, g.neg(e.point));
                    self.probe_right_adds += 1;
                    let mut right_buf = [0usize; RIGHT_IDX_CAP];
                    let n_r = usize::from(e.n_idxs);
                    right_buf[..n_r].copy_from_slice(e.idxs_slice());
                    let mut lookups = 0u64;
                    if let Some(left) = self
                        .table
                        .as_ref()
                        .and_then(|tb| tb.probe_left(g, fb, ops, &mut lookups, tt))
                    {
                        self.probe_lookups += lookups;
                        counters.lookups += lookups;
                        if out[t].is_none() {
                            if let Some(hit) = self.finish_hit_prime(
                                g,
                                fb,
                                ops,
                                counters,
                                target,
                                left,
                                &right_buf[..n_r],
                            ) {
                                out[t] = Some(hit);
                            }
                        }
                    } else {
                        self.probe_lookups += lookups;
                        counters.lookups += lookups;
                    }
                    continue;
                }
                self.degenerate[slot] = false;
                self.diffs[slot] = f.sub(e.mont_x, xr[t]);
                any_active = true;
            }
        }
        if !any_active {
            return;
        }
        f.batch_inv(&mut self.diffs[..total], &mut self.scratch);
        counters.inversions += 1;
        ops.field_ops.invs += 1;
        ops.field_ops.muls += 3 * total as u64;
        self.probe_batch_inv += 1;
        for i in 0..n {
            let (xs, ys) = {
                let e = &self.right_prime[i];
                (e.mont_x, e.mont_y)
            };
            let mut right_buf = [0usize; RIGHT_IDX_CAP];
            let n_r = usize::from(self.right_prime[i].n_idxs);
            right_buf[..n_r].copy_from_slice(self.right_prime[i].idxs_slice());
            for t in 0..t_n {
                if out[t].is_some() {
                    continue;
                }
                let slot = t * n + i;
                if self.degenerate[slot] {
                    continue;
                }
                let inv = self.diffs[slot];
                let sy = f.add(ys, yr[t]);
                let lm = f.mul(f.sub(0, sy), inv);
                let base = f.add(xr[t], xs);
                let xm_m = f.sub(f.mul(lm, lm), base);
                ops.adds += 1;
                self.probe_right_adds += 1;
                if !self.bloom_maybe(xm_m) {
                    continue;
                }
                self.probe_lookups += 1;
                counters.lookups += 1;
                if !self.lookup_mont_into(xm_m) {
                    self.bloom_false_positives += 1;
                    continue;
                }
                let left = std::mem::take(&mut self.left_scratch);
                let hit = self.finish_hit_prime(
                    g,
                    fb,
                    ops,
                    counters,
                    targets[t],
                    left,
                    &right_buf[..n_r],
                );
                if self.left_scratch.capacity() == 0 {
                    self.left_scratch = Vec::with_capacity(self.left_k as usize);
                }
                if let Some(hit) = hit {
                    out[t] = Some(hit);
                }
            }
        }
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
        self.table = Some(Arc::new(build_left_table(
            ctx.group, fb, left_k, prune, false, ops,
        )?));
        self.right_prime = Arc::new(Vec::new());
        self.right_bin = Arc::new(Vec::new());
        if right_k >= 2 {
            let mut right = Vec::new();
            collect_right_sums(ctx.group, fb, right_k, ops, &mut right);
            self.right_bin = Arc::new(right);
        }
        Ok(())
    }
    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        let mut out = self
            .table
            .as_deref()
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
            let s = self.right_bin[i].point;
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
            let mut right_buf = [0usize; RIGHT_IDX_CAP];
            let n_r = usize::from(self.right_bin[i].n_idxs);
            right_buf[..n_r].copy_from_slice(self.right_bin[i].idxs_slice());
            if let Some(hit) =
                self.finish_hit_bin(g, fb, ops, counters, target, left, &right_buf[..n_r])
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

    /// Eight-lane shared batch-inv recovers the same planted m=4 sum.
    #[test]
    fn kmitm_lanes_recover_planted_m4() {
        let inst = prime_instance_for(22, 59297);
        let fb = prime_factor_base(&inst, 16);
        let c = ctx(&inst);
        let g = &inst.curve;
        let m = 4u32;
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
        // Seven decoys + the planted sum (8-wide lever-2 chunk).
        let mut state = 0xC0FF_EE00_u64;
        let mut targets = vec![s];
        for _ in 0..7 {
            state = state.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
            targets.push(g.mul(&mut ops, inst.generator_point(), 1 + state % (inst.r - 1)));
        }
        let (_, params) = Params::parse_spec("rootfind-kmitm:m=4").unwrap();
        let mut oracle = RootfindKMitmOracle::new_prime(m, &inst);
        oracle
            .prepare(&c, &fb, &params, &mut GroupOps::default())
            .unwrap();
        let mut out = vec![None; 8];
        let mut dops = GroupOps::default();
        let mut counters = OracleCounters::default();
        oracle.decompose_lanes(&c, &fb, &mut dops, &mut counters, &targets, &mut out);
        assert!(out[0].is_some(), "lanes should recover planted sum");
        assert_eq!(out[0].as_ref().unwrap().len(), 4);
    }

    /// PLAN lever 4: two-thread probe recovers a planted m=4 sum among decoys.
    #[test]
    fn kmitm_threaded_recovers_planted_m4() {
        let inst = prime_instance_for(22, 59297);
        let fb = prime_factor_base(&inst, 16);
        let c = ctx(&inst);
        let g = &inst.curve;
        let m = 4u32;
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
        let mut state = 0xA11C_EEE5_u64;
        let mut targets = vec![s];
        for _ in 0..7 {
            state = state.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
            targets.push(g.mul(&mut ops, inst.generator_point(), 1 + state % (inst.r - 1)));
        }
        let (_, params) = Params::parse_spec("rootfind-kmitm:m=4").unwrap();
        let mut oracle = RootfindKMitmOracle::new_prime(m, &inst);
        oracle
            .prepare(&c, &fb, &params, &mut GroupOps::default())
            .unwrap();
        let out = oracle.decompose_threaded(&c, &fb, &targets, 2);
        assert_eq!(out.len(), targets.len());
        assert!(out[0].is_some(), "threaded probe should recover planted sum");
        assert_eq!(out[0].as_ref().unwrap().len(), 4);
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
