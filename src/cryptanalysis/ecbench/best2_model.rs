//! Counted selection-walk model, NOT the NIST CUDA kernel. Domain-separated
//! hashes replace its coordinate limbs; both candidates cost full group adds.
//! Every physical step (including seek and doubling) is charged. Existing
//! registered walks and their replay semantics remain unchanged.
use super::generic::GenericOutcome;
use crate::cryptanalysis::ic_boundary::{addmod, invmod, mulmod, CountedGroup};
use std::collections::BTreeMap;

fn hash(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}
fn draw(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    hash(*s)
}
fn count(out: &mut GenericOutcome, name: &str) {
    *out.counters.entry(name.into()).or_default() += 1;
}
#[derive(Clone, Copy)]
struct State<E> {
    p: E,
    a: u64,
    b: u64,
}
fn canon<G: CountedGroup>(
    g: &G,
    mut s: State<G::Elt>,
    r: u64,
    out: &mut GenericOutcome,
) -> State<G::Elt> {
    count(out, "canonicalisations_uncharged");
    let neg = g.neg(s.p);
    if g.key(&neg) < g.key(&s.p) {
        s.p = neg;
        s.a = (r - s.a) % r;
        s.b = (r - s.b) % r;
    }
    s
}
fn fresh<G: CountedGroup>(
    g: &G,
    gen: G::Elt,
    target: G::Elt,
    r: u64,
    rng: &mut u64,
    out: &mut GenericOutcome,
) -> State<G::Elt> {
    count(out, "coefficient_draws_uncharged");
    count(out, "coefficient_draws_uncharged");
    let a = draw(rng) % r;
    let b = draw(rng) % r;
    let p = g.mul(&mut out.search, gen, a);
    let q = g.mul(&mut out.search, target, b);
    let p = g.add(&mut out.search, p, q);
    count(out, "starts");
    canon(g, State { p, a, b }, r, out)
}

/// A bounded one-target model. The caller verifies the candidate. `budget`
/// bounds physical steps; setup, starts and all group work are still charged.
#[allow(clippy::too_many_arguments)]
pub fn solve<G: CountedGroup>(
    g: &G,
    gen: G::Elt,
    target: G::Elt,
    r: u64,
    seed: u64,
    budget: u64,
    jumps: usize,
    walks: usize,
    dp_bits: u32,
    best2: bool,
) -> GenericOutcome {
    let mut out = GenericOutcome::default();
    if r < 3 || jumps == 0 || walks == 0 || dp_bits > 24 {
        out.exhausted = true;
        return out;
    }
    let mut rng = seed;
    let score_key = draw(&mut rng);
    let dp_key = draw(&mut rng);
    let partition_key = draw(&mut rng);
    let mut table = Vec::with_capacity(jumps);
    for _ in 0..jumps {
        count(&mut out, "coefficient_draws_uncharged");
        count(&mut out, "coefficient_draws_uncharged");
        let a = draw(&mut rng) % r;
        let b = draw(&mut rng) % r;
        let p = g.mul(&mut out.setup, gen, a);
        let q = g.mul(&mut out.setup, target, b);
        let p = g.add(&mut out.setup, p, q);
        table.push(State { p, a, b });
    }
    let mut states: Vec<_> = (0..walks)
        .map(|_| fresh(g, gen, target, r, &mut rng, &mut out))
        .collect();
    let mut history: Vec<Vec<G::Elt>> = states.iter().map(|s| vec![s.p; 5]).collect();
    let mut escape = vec![0u32; walks];
    let mut sought = vec![0u64; walks];
    let mut lengths = vec![0u64; walks];
    let mut dps: BTreeMap<u64, State<G::Elt>> = BTreeMap::new();
    for step in 0..budget {
        let w = step as usize % walks;
        let old = states[w];
        count(&mut out, "physical_steps");
        let mut next;
        let mut skip = false;
        if g.is_identity(&old.p) {
            states[w] = fresh(g, gen, target, r, &mut rng, &mut out);
            history[w] = vec![states[w].p; 5];
            escape[w] = 0;
            lengths[w] = 0;
            count(&mut out, "infinity_restarts");
            continue;
        }
        if escape[w] > 0 && g.key(&old.p) == sought[w] {
            count(&mut out, "coefficient_updates_uncharged");
            let p = g.double(&mut out.search, old.p);
            next = canon(
                g,
                State {
                    p,
                    a: addmod(old.a, old.a, r),
                    b: addmod(old.b, old.b, r),
                },
                r,
                &mut out,
            );
            count(&mut out, "escape_doublings");
            escape[w] = 0;
            history[w] = vec![old.p; 5];
        } else {
            count(&mut out, "partition_uncharged");
            count(&mut out, "jump_probes_uncharged");
            count(&mut out, "coefficient_updates_uncharged");
            let j = (hash(g.key(&old.p) ^ partition_key) % jumps as u64) as usize;
            let m = table[j];
            let p = g.add(&mut out.search, old.p, m.p);
            next = State {
                p,
                a: addmod(old.a, m.a, r),
                b: addmod(old.b, m.b, r),
            };
            if best2 {
                let p = g.add(&mut out.search, old.p, g.neg(m.p));
                count(&mut out, "score_uncharged");
                count(&mut out, "score_uncharged");
                if hash(g.key(&p) ^ score_key) > hash(g.key(&next.p) ^ score_key) {
                    count(&mut out, "coefficient_updates_uncharged");
                    next = State {
                        p,
                        a: addmod(old.a, r - m.a, r),
                        b: addmod(old.b, r - m.b, r),
                    };
                    count(&mut out, "minus_taken");
                }
            }
            next = canon(g, next, r, &mut out);
            if escape[w] > 0 {
                escape[w] -= 1;
                skip = true;
                count(&mut out, "seek_steps");
                if escape[w] == 0 {
                    history[w] = vec![next.p; 5];
                }
            } else {
                for _ in 0..history[w].len() {
                    count(&mut out, "cycle_probes_uncharged");
                }
                if let Some(pos) = history[w].iter().position(|p| *p == next.p) {
                    let min = history[w][..=pos]
                        .iter()
                        .chain(std::iter::once(&old.p))
                        .map(|p| g.key(p))
                        .min()
                        .unwrap();
                    sought[w] = min;
                    escape[w] = 6;
                    skip = true;
                    count(&mut out, "cycle_detections");
                } else {
                    history[w].insert(0, old.p);
                    history[w].truncate(5);
                }
            }
        }
        states[w] = next;
        if skip {
            continue;
        }
        count(&mut out, "advanced_steps");
        lengths[w] += 1;
        count(&mut out, "dp_probes_uncharged");
        let dp = hash(g.key(&next.p) ^ dp_key) & ((1u64 << dp_bits) - 1) == 0;
        if dp && !g.is_identity(&next.p) {
            count(&mut out, "distinguished_points");
            count(&mut out, "table_probes_uncharged");
            if let Some(prior) = dps.get(&g.key(&next.p)) {
                if prior.p == next.p {
                    let den = addmod(next.b, r - prior.b, r);
                    if let Some(inv) = invmod(den, r) {
                        out.recovered = Some(mulmod(addmod(prior.a, r - next.a, r), inv, r));
                        return out;
                    }
                }
                count(&mut out, "fruitless_collisions");
            } else {
                dps.insert(g.key(&next.p), next);
                count(&mut out, "table_inserts_uncharged");
            }
        }
        if dp || g.is_identity(&next.p) || lengths[w] > 20 * (1u64 << dp_bits) {
            states[w] = fresh(g, gen, target, r, &mut rng, &mut out);
            history[w] = vec![states[w].p; 5];
            escape[w] = 0;
            lengths[w] = 0;
        }
    }
    out.exhausted = true;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::GroupOps;
    // Exact cyclic group oracle. Model bookkeeping test; not a curve benchmark.
    struct Z;
    impl CountedGroup for Z {
        type Elt = u64;
        fn identity(&self) -> u64 {
            0
        }
        fn is_identity(&self, p: &u64) -> bool {
            *p == 0
        }
        fn add(&self, o: &mut GroupOps, a: u64, b: u64) -> u64 {
            o.adds += 1;
            (a + b) % 1009
        }
        fn double(&self, o: &mut GroupOps, a: u64) -> u64 {
            o.doubles += 1;
            (2 * a) % 1009
        }
        fn neg(&self, a: u64) -> u64 {
            (1009 - a) % 1009
        }
        fn key(&self, p: &u64) -> u64 {
            *p
        }
    }
    #[test]
    fn recovers_and_replays_both_arms() {
        for best in [false, true] {
            for seed in 1..=12 {
                let a = solve(&Z, 1, 137, 1009, seed, 100000, 16, 8, 2, best);
                let b = solve(&Z, 1, 137, 1009, seed, 100000, 16, 8, 2, best);
                assert_eq!(a.recovered, Some(137));
                assert_eq!(a.counters, b.counters);
                assert_eq!(a.setup, b.setup);
                assert_eq!(a.search, b.search);
                assert!(a.search.adds > 0);
            }
        }
    }
}
