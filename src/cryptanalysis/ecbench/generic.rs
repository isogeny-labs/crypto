//! Generic square-root algorithms on a [`CountedGroup`]: baby-step
//! giant-step in three forms and the van Oorschot–Wiener kangaroo.
//!
//! The repository's other BSGS implementations each own a group type
//! (`ecdlp_variants::EcGroup` on `BigUint` points, `bsgs_fast` on
//! Montgomery words, `koblitz_isogeny_cost::bsgs` on one binary curve),
//! so none of them is charged in the ledger rho and index calculus are.
//! These are, operation for operation: every addition and doubling goes
//! through [`CountedGroup`], and the table work the unit does not charge
//! is counted beside it (`lookups_uncharged`, `inserts_uncharged`, the
//! repository's suffix for counted-but-not-charged work), never dropped.
//!
//! None of these functions sees the planted logarithm.  Each returns the
//! candidate it derived from a collision; the caller verifies it.

use std::collections::BTreeMap;

use crate::cryptanalysis::ic_boundary::{
    addmod, fast_map, mulmod, CountedGroup, FastMap, GroupOps,
};

/// What one generic solve did.
#[derive(Clone, Debug, Default)]
pub struct GenericOutcome {
    pub recovered: Option<u64>,
    pub setup: GroupOps,
    pub search: GroupOps,
    /// Kangaroo re-placements after a same-kind collision.  What they cost
    /// is charged one of two ways, chosen by [`kangaroo_with`]: in `search`
    /// (frozen `kangaroo.vow`; this field is then only the part of `search`
    /// spent re-placing, informational and not added again) or in this
    /// ledger alone (`restarts_apart`, `kangaroo.vow_v2`, reported as its own
    /// `restarts` phase).  Zero for every other method.
    pub restarts: GroupOps,
    /// The final `[d]G = Q` check of [`verify_candidate`]; zero for the
    /// ids that do not run it.
    pub verification: GroupOps,
    /// Native work the group-operation unit does not charge, and the
    /// shape of the run (`baby_steps`, `giant_steps`, `restarts`, …).
    pub counters: BTreeMap<String, u64>,
    /// The step budget ran out before a collision.
    pub exhausted: bool,
}

impl GenericOutcome {
    /// Called on every baby, giant or kangaroo step: the key is only
    /// allocated the first time a counter is seen, not per step.
    fn count(&mut self, name: &str, by: u64) {
        if let Some(c) = self.counters.get_mut(name) {
            *c += by;
        } else {
            self.counters.insert(name.to_string(), by);
        }
    }
}

/// Charge the check `[d]G = Q` of the candidate a solve returned to
/// `out.verification`, one scalar multiplication (`≈ 1.5·log₂ r` additions,
/// counted exactly), and book it in `verification_checks`; a candidate that
/// fails is booked in `failed_candidates` and kept for the runner to
/// grade.  A solve with no candidate (exhausted) pays nothing.  This is the
/// rule of every `_v2` method: rho and index calculus have always paid it
/// for the answer they return, BSGS, the kangaroo and the claw had not.  The
/// final point equality is classified by `verification_checks` as outcome
/// shape under the repository's verification convention; the scalar replay
/// carries the charged group work.
pub fn verify_candidate<G: CountedGroup>(
    g: &G,
    gen: G::Elt,
    target: G::Elt,
    out: &mut GenericOutcome,
) {
    let Some(d) = out.recovered else { return };
    out.count("verification_checks", 1);
    if g.mul(&mut out.verification, gen, d) != target {
        out.count("failed_candidates", 1);
    }
}

fn submod(a: u64, b: u64, m: u64) -> u64 {
    addmod(a % m, m - b % m, m)
}

/// `⌈√r⌉`.
pub fn ceil_sqrt(r: u64) -> u64 {
    let mut s = (r as f64).sqrt() as u64;
    while s.saturating_mul(s) < r {
        s += 1;
    }
    while s > 0 && (s - 1).saturating_mul(s - 1) >= r {
        s -= 1;
    }
    s.max(1)
}

/// The per-step counters of the three BSGS forms, kept in words during the
/// search and written to [`GenericOutcome::counters`] once at the end.
/// Every per-step call adds 1, so a counter is in the map exactly when it
/// is non-zero, and [`StepCounters::flush`] writes only those: the map
/// ends with the keys and values the per-step `count` calls produced,
/// without a string-keyed map lookup per step.
#[derive(Default)]
struct StepCounters {
    inserts: u64,
    baby: u64,
    lookups: u64,
    giant: u64,
}

impl StepCounters {
    fn flush(self, out: &mut GenericOutcome) {
        for (name, v) in [
            ("inserts_uncharged", self.inserts),
            ("baby_steps", self.baby),
            ("lookups_uncharged", self.lookups),
            ("giant_steps", self.giant),
        ] {
            if v > 0 {
                out.count(name, v);
            }
        }
    }
}

/// **Textbook BSGS.**  Baby steps `jG`, `0 ≤ j < m = ⌈√r⌉`, all stored
/// first; giant steps `Q − i·mG` until one lands in the table.  Worst
/// case `2√r` operations, average `1.5√r`.
pub fn bsgs_textbook<G: CountedGroup>(
    g: &G,
    gen: G::Elt,
    target: G::Elt,
    r: u64,
) -> GenericOutcome {
    let mut out = GenericOutcome::default();
    let mut c = StepCounters::default();
    let m = ceil_sqrt(r);
    let mut table: FastMap<u64> = fast_map(m as usize);
    let mut p = g.identity();
    for j in 0..m {
        table.entry(g.key(&p)).or_insert(j);
        c.inserts += 1;
        c.baby += 1;
        if j + 1 < m {
            p = g.add(&mut out.setup, p, gen);
        }
    }
    let stride = g.mul(&mut out.setup, gen, m);
    let back = g.neg(stride);
    let mut y = target;
    let found = 'search: {
        for i in 0..=m {
            c.lookups += 1;
            if let Some(&j) = table.get(&g.key(&y)) {
                break 'search Some(addmod(mulmod(i, m, r), j, r));
            }
            c.giant += 1;
            y = g.add(&mut out.search, y, back);
        }
        None
    };
    c.flush(&mut out);
    out.recovered = found;
    out.exhausted = found.is_none();
    out
}

/// **Interleaved BSGS** (Pollard): one baby step and one giant step at a
/// time, each looked up in the other's table, so the search stops at
/// `max(i, j)` rather than after the whole baby table.  Average
/// `(4/3)√r` operations.
pub fn bsgs_interleaved<G: CountedGroup>(
    g: &G,
    gen: G::Elt,
    target: G::Elt,
    r: u64,
) -> GenericOutcome {
    let mut out = GenericOutcome::default();
    let m = ceil_sqrt(r);
    let stride = g.mul(&mut out.setup, gen, m);
    let back = g.neg(stride);
    // Each table holds at most the steps taken before the collision, on
    // average about √r · 2/3; sized for the expectation, grown past it.
    let mut baby: FastMap<u64> = fast_map((m as usize).div_ceil(3) * 2);
    let mut giant: FastMap<u64> = fast_map((m as usize).div_ceil(3) * 2);
    let mut b = g.identity();
    let mut y = target;
    let mut c = StepCounters::default();
    let found = 'search: {
        for t in 0..m {
            // Baby step t: tG against the giant table.
            c.lookups += 1;
            if let Some(&i) = giant.get(&g.key(&b)) {
                break 'search Some(addmod(mulmod(i, m, r), t, r));
            }
            baby.entry(g.key(&b)).or_insert(t);
            c.inserts += 1;
            // Giant step t: Q − t·mG against the baby table.
            c.lookups += 1;
            if let Some(&j) = baby.get(&g.key(&y)) {
                break 'search Some(addmod(mulmod(t, m, r), j, r));
            }
            giant.entry(g.key(&y)).or_insert(t);
            c.inserts += 1;
            c.baby += 1;
            c.giant += 1;
            b = g.add(&mut out.search, b, gen);
            y = g.add(&mut out.search, y, back);
        }
        None
    };
    c.flush(&mut out);
    out.recovered = found;
    out.exhausted = found.is_none();
    out
}

/// The class key of `{P, −P}`: the smaller of the two keys.
fn class_key<G: CountedGroup>(g: &G, p: &G::Elt) -> u64 {
    g.key(p).min(g.key(&g.neg(*p)))
}

/// **BSGS on the classes `{P, −P}`.**  The baby table holds `jG` for
/// `0 ≤ j ≤ m`, keyed by class, so it covers `±j`; the giant stride is
/// `2m + 1`.  With `m = ⌈√r / 2⌉` the worst case is about `1.5√r` and
/// the average about `√r`, the negation map's `√2`-class saving on BSGS.
/// The sign is read off the stored key: no extra operation.
pub fn bsgs_negation<G: CountedGroup>(
    g: &G,
    gen: G::Elt,
    target: G::Elt,
    r: u64,
) -> GenericOutcome {
    bsgs_negation_table(g, gen, target, r, ceil_sqrt(r).div_ceil(2).max(1))
}

/// [`bsgs_negation`] with the baby table's half-width `m` given: the table
/// holds `jG` for `0 ≤ j ≤ m` (so `m + 1` classes covering `±j`) and the
/// giant stride is `2m + 1`.  `m = ⌈√r / 2⌉` is `bsgs_negation`; a smaller
/// `m` is the equal-memory comparator for a known-log base.
pub fn bsgs_negation_table<G: CountedGroup>(
    g: &G,
    gen: G::Elt,
    target: G::Elt,
    r: u64,
    m: u64,
) -> GenericOutcome {
    let mut out = GenericOutcome::default();
    let mut c = StepCounters::default();
    let m = m.max(1);
    let mut table: FastMap<(u64, u64)> = fast_map(m as usize + 1);
    let mut p = g.identity();
    for j in 0..=m {
        table.entry(class_key(g, &p)).or_insert((j, g.key(&p)));
        c.inserts += 1;
        c.baby += 1;
        if j < m {
            p = g.add(&mut out.setup, p, gen);
        }
    }
    let s = 2 * m + 1;
    let stride = g.mul(&mut out.setup, gen, s);
    let back = g.neg(stride);
    let mut y = target;
    let giants = r / s + 1;
    let found = 'search: {
        for i in 0..=giants {
            c.lookups += 1;
            if let Some(&(j, key)) = table.get(&class_key(g, &y)) {
                let is = mulmod(i, s, r);
                break 'search Some(if g.key(&y) == key {
                    addmod(is, j, r)
                } else {
                    submod(is, j, r)
                });
            }
            c.giant += 1;
            y = g.add(&mut out.search, y, back);
        }
        None
    };
    c.flush(&mut out);
    out.recovered = found;
    out.exhausted = found.is_none();
    out
}

/// Target-independent advice for [`bsgs_negation_table`].
///
/// Building this value performs every baby step and constructs the giant
/// stride from `G` alone.  [`NegationBabyTable::solve`] is consequently the
/// exact online boundary for a cached-table query: its first operation reads
/// `target`, while the cold one-target cost remains the sum of `setup` and
/// `search` in the returned [`GenericOutcome`].
pub struct NegationBabyTable<E> {
    table: FastMap<(u64, u64)>,
    back: E,
    giant_stride: u64,
    order: u64,
    setup: GroupOps,
    inserts: u64,
    baby_steps: u64,
    point_negations: u64,
}

/// Build the target-independent half-width-`m` negation table used by
/// [`bsgs_negation_table`].  The table is reusable only for the same group,
/// generator, order and `m`; callers comparing a cold one-target solve must
/// still charge this complete build once.
pub fn build_negation_table<G: CountedGroup>(
    g: &G,
    gen: G::Elt,
    r: u64,
    m: u64,
) -> NegationBabyTable<G::Elt> {
    let mut setup = GroupOps::default();
    let m = m.max(1);
    let mut table: FastMap<(u64, u64)> = fast_map(m as usize + 1);
    let mut p = g.identity();
    for j in 0..=m {
        table.entry(class_key(g, &p)).or_insert((j, g.key(&p)));
        if j < m {
            p = g.add(&mut setup, p, gen);
        }
    }
    let s = 2 * m + 1;
    let stride = g.mul(&mut setup, gen, s);
    let back = g.neg(stride);

    NegationBabyTable {
        table,
        back,
        giant_stride: s,
        order: r,
        setup,
        inserts: m + 1,
        baby_steps: m + 1,
        // `back = -stride` is target-independent native work outside the
        // counted group-operation ledger.  Per-entry class negations are
        // carried by `inserts_uncharged` instead.
        point_negations: 1,
    }
}

impl<E: Copy> NegationBabyTable<E> {
    /// Query one target against this table.  The returned outcome includes
    /// the table's complete setup ledger and insert count, so it is a cold
    /// one-target record even when the caller clocks this method alone as
    /// the cached online interval.
    pub fn solve<G: CountedGroup<Elt = E>>(&self, g: &G, target: E) -> GenericOutcome {
        let mut out = GenericOutcome {
            setup: self.setup,
            ..Default::default()
        };
        let mut c = StepCounters {
            inserts: self.inserts,
            baby: self.baby_steps,
            ..Default::default()
        };
        out.count("point_negations_uncharged", self.point_negations);
        let mut y = target;
        let giants = self.order / self.giant_stride + 1;
        let found = 'search: {
            for i in 0..=giants {
                c.lookups += 1;
                if let Some(&(j, key)) = self.table.get(&class_key(g, &y)) {
                    // One key equality chooses the sign, then one multiply
                    // and one add/subtract modulo the subgroup order recover
                    // the scalar.  These happen only on a table hit.
                    out.count("recovery_key_comparisons_uncharged", 1);
                    out.count("recovery_modr_muls_uncharged", 1);
                    out.count("recovery_modr_adds_uncharged", 1);
                    let is = mulmod(i, self.giant_stride, self.order);
                    break 'search Some(if g.key(&y) == key {
                        addmod(is, j, self.order)
                    } else {
                        submod(is, j, self.order)
                    });
                }
                c.giant += 1;
                y = g.add(&mut out.search, y, self.back);
            }
            None
        };
        c.flush(&mut out);
        out.recovered = found;
        out.exhausted = found.is_none();
        out
    }
}

/// Kangaroo parameters chosen from `r`.
#[derive(Clone, Copy, Debug)]
pub struct KangarooShape {
    /// Jumps are `2^0 … 2^{k−1}`, mean about `√r / 2`.
    pub jumps: u32,
    /// Distinguished when the key's low `dp_bits` bits of its mix are zero.
    pub dp_bits: u32,
}

impl KangarooShape {
    pub fn of(r: u64) -> Self {
        let target_mean = ((r as f64).sqrt() / 2.0).max(1.0);
        // Mean of {2^0, …, 2^{k−1}} is (2^k − 1)/k.
        let mut k = 1u32;
        while k < 62 && (((1u64 << (k + 1)) - 1) as f64 / (k + 1) as f64) <= target_mean {
            k += 1;
        }
        let bits = 64 - r.leading_zeros();
        Self {
            jumps: k,
            dp_bits: (bits / 4).saturating_sub(1),
        }
    }
}

/// `h mod d` for a fixed `d ≥ 1` without a division instruction: Lemire,
/// Kaser and Kurz, "Faster remainder by direct computation" (2019), the
/// 64-bit case with the 128-bit constant `M = ⌈2^128 / d⌉`, which is exact
/// for every 64-bit `h` and every `d < 2^64` (their Theorem 1 with
/// `F = 128 ≥ N + log₂ d`).  `d = 1` wraps `M` to 0 and returns 0.  The
/// kangaroo's jump index was a 64-bit `div` per step; the value is the
/// same remainder.
#[derive(Clone, Copy)]
struct FastMod {
    m: u128,
    d: u64,
}

impl FastMod {
    fn new(d: u64) -> Self {
        assert!(d > 0, "FastMod of a zero modulus");
        Self {
            m: (u128::MAX / d as u128).wrapping_add(1),
            d,
        }
    }

    #[inline]
    fn rem(self, h: u64) -> u64 {
        let low = self.m.wrapping_mul(h as u128);
        let bottom = ((low as u64) as u128 * self.d as u128) >> 64;
        let top = (low >> 64) * self.d as u128;
        ((bottom + top) >> 64) as u64
    }
}

fn mix(v: u64) -> u64 {
    crate::cryptanalysis::ecbench::canonical::splitmix64(v)
}

/// **Van Oorschot–Wiener kangaroo**, one tame and one wild, on the whole
/// group (`[0, r)` as the interval).  Jumps are powers of two chosen by
/// the key's hash, mean about `√r / 2`; distinguished points go into one
/// table with their distance.  A tame–wild meeting gives the logarithm;
/// a same-kind meeting means the later kangaroo is retracing the other's
/// path, and it is restarted from a fresh offset (counted).
///
/// Expected cost about `2√r`: the kangaroo is a method for intervals, and
/// it is here so that a short-interval workload has its natural
/// reference.  `max_steps` caps the total jumps.
pub fn kangaroo<G: CountedGroup>(
    g: &G,
    gen: G::Elt,
    target: G::Elt,
    r: u64,
    seed: u64,
    max_steps: u64,
) -> GenericOutcome {
    kangaroo_with(g, gen, target, r, seed, max_steps, false)
}

/// [`kangaroo`], with the scalar multiplications of a restart charged to
/// `out.restarts` alone when `restarts_apart` (the `_v2` phase attribution:
/// a restart is neither jump-table set-up nor a jump, so it has its own
/// ledger).  Without it a restart is charged to `out.search`, as frozen
/// `kangaroo.vow` has done since 2026-10-18, and `out.restarts` only tallies
/// the part of `search` it was.  The walk is the same either way.
pub fn kangaroo_with<G: CountedGroup>(
    g: &G,
    gen: G::Elt,
    target: G::Elt,
    r: u64,
    seed: u64,
    max_steps: u64,
    restarts_apart: bool,
) -> GenericOutcome {
    let mut out = GenericOutcome::default();
    let shape = KangarooShape::of(r);
    out.count("jump_count", shape.jumps as u64);
    out.count("dp_bits", shape.dp_bits as u64);
    let jump_of = FastMod::new(shape.jumps as u64);
    let mut jumps: Vec<(G::Elt, u64)> = Vec::with_capacity(shape.jumps as usize);
    let mut p = gen;
    for i in 0..shape.jumps {
        jumps.push((p, (1u64 << i) % r));
        if i + 1 < shape.jumps {
            p = g.double(&mut out.setup, p);
        }
    }
    let dp_mask = if shape.dp_bits == 0 {
        0
    } else {
        (1u64 << shape.dp_bits) - 1
    };
    // kind 0 = tame (position = distance), kind 1 = wild (position = k + distance).
    let mut table: FastMap<(u8, u64)> = fast_map(64);
    let mut rng = seed;
    let mut next = || {
        rng = mix(rng);
        rng
    };
    // The two initial placements are set-up.  A placement after a same-kind
    // collision is part of the search that produced the collision: frozen
    // `kangaroo.vow` charges it to `search` (before 2026-10-18 to `setup`;
    // the total is the same) and tallies it in `out.restarts` for `detail`;
    // `restarts_apart` (`kangaroo.vow_v2`) charges it to `out.restarts`
    // alone, which that id reports as its own phase.
    let start = |kind: u8, out: &mut GenericOutcome, off: u64, restart: bool| -> (G::Elt, u64) {
        out.count("starts", 1);
        let mut ops = GroupOps::default();
        let point = if kind == 0 {
            let d = addmod(r / 2, off, r);
            (g.mul(&mut ops, gen, d), d)
        } else {
            let z = g.mul(&mut ops, gen, off);
            (g.add(&mut ops, target, z), off)
        };
        if !restart {
            out.setup.merge(ops);
        } else if restarts_apart {
            out.restarts.merge(ops);
        } else {
            out.search.merge(ops);
            out.restarts.adds += ops.adds;
            out.restarts.doubles += ops.doubles;
            out.restarts.scalar_mults += ops.scalar_mults;
        }
        point
    };
    // Each kangaroo carries the mixed key of its point (the jump index's
    // input), computed once when the point is reached rather than again at
    // the start of its next step.
    let place = |(p, d): (G::Elt, u64)| (p, d, mix(g.key(&p)));
    let mut roo = [
        place(start(0, &mut out, 0, false)),
        place(start(1, &mut out, 0, false)),
    ];
    // The per-distinguished-point counters, kept in words and written to
    // the map once at each exit: a counter is in the map exactly when the
    // per-event `count` calls would have created it (it is non-zero), so
    // the map ends with the same keys and values.  `steps` is always
    // written, as before.
    let (mut dps, mut inserts, mut collisions) = (0u64, 0u64, 0u64);
    let flush = |out: &mut GenericOutcome, steps: u64, dps: u64, inserts: u64, collisions: u64| {
        for (name, v) in [
            ("distinguished_points", dps),
            ("lookups_uncharged", dps),
            ("same_kind_collisions", collisions),
            ("inserts_uncharged", inserts),
        ] {
            if v > 0 {
                out.count(name, v);
            }
        }
        out.count("steps", steps);
    };
    let mut steps = 0u64;
    while steps < max_steps {
        for kind in 0..2u8 {
            let (pos, dist, h) = roo[kind as usize];
            let (jp, jd) = jumps[jump_of.rem(h) as usize];
            let np = g.add(&mut out.search, pos, jp);
            let nd = addmod(dist, jd, r);
            steps += 1;
            let nk = g.key(&np);
            roo[kind as usize] = (np, nd, mix(nk));
            if mix(nk ^ 0xD15C) & dp_mask != 0 {
                continue;
            }
            dps += 1;
            match table.get(&nk) {
                Some(&(other, od)) if other != kind => {
                    // tame: P = [dt]G; wild: P = Q + [dw]G  ⇒  k = dt − dw.
                    let (dt, dw) = if kind == 0 { (nd, od) } else { (od, nd) };
                    flush(&mut out, steps, dps, inserts, collisions);
                    out.recovered = Some(submod(dt, dw, r));
                    return out;
                }
                Some(_) => {
                    collisions += 1;
                    let off = next() % r.max(2);
                    roo[kind as usize] = place(start(kind, &mut out, off, true));
                }
                None => {
                    table.insert(nk, (kind, nd));
                    inserts += 1;
                }
            }
        }
    }
    flush(&mut out, steps, dps, inserts, collisions);
    out.exhausted = true;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::{
        find_prime_order_curve, koblitz_instance, BinaryGroup,
    };

    #[test]
    fn fast_mod_is_the_remainder() {
        let mut s = 0x9E37_79B9_7F4A_7C15u64;
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s
        };
        let edges = [
            0u64,
            1,
            2,
            3,
            u64::MAX,
            u64::MAX - 1,
            1 << 63,
            (1 << 63) - 1,
        ];
        let mut moduli: Vec<u64> = (1..=64).collect();
        moduli.extend([u64::MAX, u64::MAX - 1, 1 << 63, (1 << 63) + 1, 1_000_003]);
        for d in moduli {
            let f = FastMod::new(d);
            for &h in &edges {
                assert_eq!(f.rem(h), h % d, "d={d} h={h}");
            }
            for _ in 0..4096 {
                let h = next();
                assert_eq!(f.rem(h), h % d, "d={d} h={h}");
            }
        }
    }

    fn check_all_prime(bits: u32, seed: u64) {
        let inst = find_prime_order_curve(bits, seed);
        let g = &inst.curve;
        let gen = inst.generator_point();
        for k in [1u64, 2, inst.r / 3, inst.r / 2 + 7, inst.r - 1] {
            let mut ops = GroupOps::default();
            let q = g.mul(&mut ops, gen, k);
            for (name, out) in [
                ("textbook", bsgs_textbook(g, gen, q, inst.r)),
                ("interleaved", bsgs_interleaved(g, gen, q, inst.r)),
                ("negation", bsgs_negation(g, gen, q, inst.r)),
                ("kangaroo", kangaroo(g, gen, q, inst.r, k ^ 5, 64 * inst.r)),
            ] {
                assert_eq!(out.recovered, Some(k), "{name} k={k} r={}", inst.r);
            }
        }
    }

    #[test]
    fn negation_table_is_target_independent_reusable_advice() {
        let inst = find_prime_order_curve(14, 59297);
        let g = &inst.curve;
        let gen = inst.generator_point();
        let m = ceil_sqrt(inst.r).div_ceil(2);
        let table = build_negation_table(g, gen, inst.r, m);
        let mut setup = None;
        for k in [1, inst.r / 3, inst.r / 2 + 7, inst.r - 1] {
            let q = g.mul(&mut GroupOps::default(), gen, k);
            let cached = table.solve(g, q);
            let direct = bsgs_negation_table(g, gen, q, inst.r, m);
            assert_eq!(cached.recovered, Some(k));
            assert_eq!(cached.recovered, direct.recovered);
            assert_eq!(cached.setup, direct.setup);
            assert_eq!(cached.search, direct.search);
            for name in [
                "inserts_uncharged",
                "baby_steps",
                "lookups_uncharged",
                "giant_steps",
            ] {
                assert_eq!(cached.counters.get(name), direct.counters.get(name), "{name}");
            }
            assert_eq!(cached.counters["point_negations_uncharged"], 1);
            assert_eq!(cached.counters["recovery_key_comparisons_uncharged"], 1);
            assert_eq!(cached.counters["recovery_modr_muls_uncharged"], 1);
            assert_eq!(cached.counters["recovery_modr_adds_uncharged"], 1);
            assert_eq!(
                cached.counters["inserts_uncharged"],
                m + 1,
                "one insert attempt per baby-table entry"
            );
            match setup {
                Some(want) => assert_eq!(want, cached.setup),
                None => setup = Some(cached.setup),
            }
        }
    }

    #[test]
    fn cached_negation_table_counts_recovery_only_on_a_hit() {
        struct Cyclic(u64);
        impl CountedGroup for Cyclic {
            type Elt = u64;
            fn identity(&self) -> u64 {
                0
            }
            fn is_identity(&self, p: &u64) -> bool {
                *p == 0
            }
            fn add(&self, ops: &mut GroupOps, p: u64, q: u64) -> u64 {
                ops.adds += 1;
                (p + q) % self.0
            }
            fn double(&self, ops: &mut GroupOps, p: u64) -> u64 {
                ops.doubles += 1;
                (2 * p) % self.0
            }
            fn neg(&self, p: u64) -> u64 {
                (self.0 - p) % self.0
            }
            fn key(&self, p: &u64) -> u64 {
                *p
            }
        }

        // The declared order deliberately bounds the search before target 50
        // in Z/101 can reach the table.  This exercises the exhausted path,
        // not a valid DLP instance under the declared order.
        let group = Cyclic(101);
        let out = build_negation_table(&group, 1, 5, 1).solve(&group, 50);
        assert!(out.exhausted);
        assert_eq!(out.recovered, None);
        assert_eq!(out.counters["point_negations_uncharged"], 1);
        for name in [
            "recovery_key_comparisons_uncharged",
            "recovery_modr_muls_uncharged",
            "recovery_modr_adds_uncharged",
        ] {
            assert!(!out.counters.contains_key(name), "{name}");
        }
    }

    /// Kangaroo's set-up is the jump table and the two initial placements,
    /// a function of `r` alone; any re-placement after a same-kind
    /// collision is charged to `search`.  So the set-up charge is the same
    /// on every seed.  (A same-kind collision needs a kangaroo to land on
    /// its own trail; none occurs on these seeds.  The restart itself is
    /// exercised by `a_kangaroo_restart_is_charged_to_search`.)
    #[test]
    fn kangaroo_setup_is_a_function_of_r_alone() {
        let inst = find_prime_order_curve(14, 59297);
        let g = &inst.curve;
        let gen = inst.generator_point();
        let mut setups = Vec::new();
        for seed in 0..200u64 {
            let k = 1 + (seed * 7919 + 13) % (inst.r - 1);
            let q = g.mul(&mut GroupOps::default(), gen, k);
            let out = kangaroo(g, gen, q, inst.r, seed, 64 * inst.r);
            assert_eq!(out.recovered, Some(k), "seed {seed}");
            assert_eq!(
                out.setup.gae() + out.search.gae(),
                (out.setup.adds + out.setup.doubles + out.search.adds + out.search.doubles) as f64
            );
            setups.push(out.setup.gae());
        }
        assert!(
            setups.iter().all(|&s| s == setups[0]),
            "set-up varies with the seed: {setups:?}"
        );
    }

    /// Positive control for the re-placement accounting.  On this 11-bit
    /// curve (r = 1553) the walk from Q = G is periodic before it meets the
    /// tame kangaroo's trail, so the wild kangaroo lands on its own
    /// distinguished point and is re-placed; this happens on every seed
    /// (the walk does not depend on it, only the re-placement offset does).
    /// The re-placement must be in `search`, `setup` must equal a run with
    /// no restart on the same group, and the logarithm must still be right.
    #[test]
    fn a_kangaroo_restart_is_charged_to_search() {
        let inst = find_prime_order_curve(11, 0);
        assert_eq!(inst.r, 1553, "the pinned control curve moved");
        let g = &inst.curve;
        let gen = inst.generator_point();
        let plain = (2..inst.r)
            .map(|k| {
                let q = g.mul(&mut GroupOps::default(), gen, k);
                (k, kangaroo(g, gen, q, inst.r, 0, 64 * inst.r))
            })
            .find(|(_, o)| !o.counters.contains_key("same_kind_collisions"))
            .expect("some target has no restart");
        assert_eq!(plain.1.recovered, Some(plain.0));
        assert_eq!(plain.1.restarts, GroupOps::default());
        for seed in 0..32u64 {
            let out = kangaroo(g, gen, gen, inst.r, seed, 64 * inst.r);
            assert_eq!(out.recovered, Some(1), "seed {seed}");
            assert!(out.counters["same_kind_collisions"] >= 1, "seed {seed}");
            assert_eq!(
                out.counters["starts"],
                2 + out.counters["same_kind_collisions"]
            );
            // A re-placement is one scalar multiplication (plus the wild
            // kangaroo's addition of Q), all of it in `search`.
            assert!(out.restarts.scalar_mults >= 1, "seed {seed}");
            assert!(out.search.adds >= out.restarts.adds);
            assert!(out.search.doubles >= out.restarts.doubles);
            assert!(out.search.scalar_mults >= out.restarts.scalar_mults);
            assert_eq!(
                out.setup, plain.1.setup,
                "seed {seed}: a restart leaked into setup"
            );
        }
    }

    /// The `_v2` placement: the same walk, a re-placement charged to
    /// `restarts` alone, so `search` holds jumps only, and `search +
    /// restarts` is the frozen id's `search`.
    #[test]
    fn a_kangaroo_restart_is_its_own_ledger_when_kept_apart() {
        let inst = find_prime_order_curve(11, 0);
        assert_eq!(inst.r, 1553, "the pinned control curve moved");
        let g = &inst.curve;
        let gen = inst.generator_point();
        for seed in 0..32u64 {
            let cap = 64 * inst.r;
            let frozen = kangaroo(g, gen, gen, inst.r, seed, cap);
            let apart = kangaroo_with(g, gen, gen, inst.r, seed, cap, true);
            assert_eq!(apart.recovered, Some(1), "seed {seed}");
            assert_eq!(apart.counters, frozen.counters, "the same walk");
            assert!(apart.counters["same_kind_collisions"] >= 1, "seed {seed}");
            assert!(apart.restarts.scalar_mults >= 1, "seed {seed}");
            assert_eq!(apart.setup, frozen.setup, "seed {seed}");
            assert_eq!(
                apart.search.gae() + apart.restarts.gae(),
                frozen.search.gae(),
                "seed {seed}"
            );
            assert_eq!(
                apart.setup.gae() + apart.search.gae() + apart.restarts.gae(),
                frozen.setup.gae() + frozen.search.gae(),
                "the total does not move"
            );
        }
    }

    #[test]
    fn generic_methods_recover_on_prime_curves() {
        check_all_prime(14, 59297);
        check_all_prime(18, 59297);
    }

    #[test]
    fn generic_methods_recover_on_a_koblitz_curve() {
        // icv1-f2m13-t181-515ee569 (KoblitzCurve::new(0, 13)): #E = 4 · 2003.
        // KoblitzCurve::new(0, 17) has no usable prime-order subgroup.
        let inst = koblitz_instance(0, 13).expect("KoblitzCurve::new(0, 13)");
        let g = BinaryGroup(&inst.fast);
        for k in [3u64, inst.r - 2, inst.r / 5] {
            let mut ops = GroupOps::default();
            let q = g.mul(&mut ops, inst.generator, k);
            assert_eq!(
                bsgs_textbook(&g, inst.generator, q, inst.r).recovered,
                Some(k)
            );
            assert_eq!(
                bsgs_interleaved(&g, inst.generator, q, inst.r).recovered,
                Some(k)
            );
            assert_eq!(
                bsgs_negation(&g, inst.generator, q, inst.r).recovered,
                Some(k)
            );
            assert_eq!(
                kangaroo(&g, inst.generator, q, inst.r, 11, 64 * inst.r).recovered,
                Some(k)
            );
        }
    }

    #[test]
    fn bsgs_costs_sit_where_theory_puts_them() {
        // Worst case of the textbook form is m − 1 + bits(m)·2-ish setup
        // plus at most m giant steps; never more than 2.2√r here.
        let inst = find_prime_order_curve(20, 59297);
        let g = &inst.curve;
        let gen = inst.generator_point();
        let mut ops = GroupOps::default();
        let q = g.mul(&mut ops, gen, inst.r - 1);
        let out = bsgs_textbook(g, gen, q, inst.r);
        let total = out.setup.gae() + out.search.gae();
        assert!(total <= 2.2 * (inst.r as f64).sqrt(), "{total}");
    }
}
