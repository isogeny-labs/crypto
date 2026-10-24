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
    /// The part of `search` spent re-placing a kangaroo after a same-kind
    /// collision (zero for every other method).
    pub restarts: GroupOps,
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
    let m = ceil_sqrt(r);
    let mut table: FastMap<u64> = fast_map(m as usize);
    let mut p = g.identity();
    for j in 0..m {
        table.entry(g.key(&p)).or_insert(j);
        out.count("inserts_uncharged", 1);
        out.count("baby_steps", 1);
        if j + 1 < m {
            p = g.add(&mut out.setup, p, gen);
        }
    }
    let stride = g.mul(&mut out.setup, gen, m);
    let back = g.neg(stride);
    let mut y = target;
    for i in 0..=m {
        out.count("lookups_uncharged", 1);
        if let Some(&j) = table.get(&g.key(&y)) {
            out.recovered = Some(addmod(mulmod(i, m, r), j, r));
            return out;
        }
        out.count("giant_steps", 1);
        y = g.add(&mut out.search, y, back);
    }
    out.exhausted = true;
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
    for t in 0..m {
        // Baby step t: tG against the giant table.
        out.count("lookups_uncharged", 1);
        if let Some(&i) = giant.get(&g.key(&b)) {
            out.recovered = Some(addmod(mulmod(i, m, r), t, r));
            return out;
        }
        baby.entry(g.key(&b)).or_insert(t);
        out.count("inserts_uncharged", 1);
        // Giant step t: Q − t·mG against the baby table.
        out.count("lookups_uncharged", 1);
        if let Some(&j) = baby.get(&g.key(&y)) {
            out.recovered = Some(addmod(mulmod(t, m, r), j, r));
            return out;
        }
        giant.entry(g.key(&y)).or_insert(t);
        out.count("inserts_uncharged", 1);
        out.count("baby_steps", 1);
        out.count("giant_steps", 1);
        b = g.add(&mut out.search, b, gen);
        y = g.add(&mut out.search, y, back);
    }
    out.exhausted = true;
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
    let mut out = GenericOutcome::default();
    let m = ceil_sqrt(r).div_ceil(2).max(1);
    let mut table: FastMap<(u64, u64)> = fast_map(m as usize + 1);
    let mut p = g.identity();
    for j in 0..=m {
        table.entry(class_key(g, &p)).or_insert((j, g.key(&p)));
        out.count("inserts_uncharged", 1);
        out.count("baby_steps", 1);
        if j < m {
            p = g.add(&mut out.setup, p, gen);
        }
    }
    let s = 2 * m + 1;
    let stride = g.mul(&mut out.setup, gen, s);
    let back = g.neg(stride);
    let mut y = target;
    let giants = r / s + 1;
    for i in 0..=giants {
        out.count("lookups_uncharged", 1);
        if let Some(&(j, key)) = table.get(&class_key(g, &y)) {
            let is = mulmod(i, s, r);
            out.recovered = Some(if g.key(&y) == key {
                addmod(is, j, r)
            } else {
                submod(is, j, r)
            });
            return out;
        }
        out.count("giant_steps", 1);
        y = g.add(&mut out.search, y, back);
    }
    out.exhausted = true;
    out
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
    let mut out = GenericOutcome::default();
    let shape = KangarooShape::of(r);
    out.count("jump_count", shape.jumps as u64);
    out.count("dp_bits", shape.dp_bits as u64);
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
    // The two initial placements are set-up; a placement after a same-kind
    // collision is part of the search that produced the collision, and is
    // charged there, so the `setup` / `search` split reflects where the
    // work happened (the total is the same either way).
    let start = |kind: u8, out: &mut GenericOutcome, off: u64, restart: bool| -> (G::Elt, u64) {
        out.count("starts", 1);
        let ledger = if restart {
            &mut out.search
        } else {
            &mut out.setup
        };
        if kind == 0 {
            let d = addmod(r / 2, off, r);
            (g.mul(ledger, gen, d), d)
        } else {
            let z = g.mul(ledger, gen, off);
            (g.add(ledger, target, z), off)
        }
    };
    let mut roo = [start(0, &mut out, 0, false), start(1, &mut out, 0, false)];
    let mut steps = 0u64;
    while steps < max_steps {
        for kind in 0..2u8 {
            let (pos, dist) = roo[kind as usize];
            let h = mix(g.key(&pos));
            let (jp, jd) = jumps[(h % shape.jumps as u64) as usize];
            let np = g.add(&mut out.search, pos, jp);
            let nd = addmod(dist, jd, r);
            steps += 1;
            roo[kind as usize] = (np, nd);
            if mix(g.key(&np) ^ 0xD15C) & dp_mask != 0 {
                continue;
            }
            out.count("distinguished_points", 1);
            out.count("lookups_uncharged", 1);
            match table.get(&g.key(&np)) {
                Some(&(other, od)) if other != kind => {
                    // tame: P = [dt]G; wild: P = Q + [dw]G  ⇒  k = dt − dw.
                    let (dt, dw) = if kind == 0 { (nd, od) } else { (od, nd) };
                    out.count("steps", steps);
                    out.recovered = Some(submod(dt, dw, r));
                    return out;
                }
                Some(_) => {
                    out.count("same_kind_collisions", 1);
                    let off = next() % r.max(2);
                    let before = out.search;
                    roo[kind as usize] = start(kind, &mut out, off, true);
                    out.restarts.adds += out.search.adds - before.adds;
                    out.restarts.doubles += out.search.doubles - before.doubles;
                    out.restarts.scalar_mults += out.search.scalar_mults - before.scalar_mults;
                }
                None => {
                    table.insert(g.key(&np), (kind, nd));
                    out.count("inserts_uncharged", 1);
                }
            }
        }
    }
    out.count("steps", steps);
    out.exhausted = true;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::{
        find_prime_order_curve, koblitz_instance, BinaryGroup,
    };

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
