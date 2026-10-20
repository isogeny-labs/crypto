//! New method IDs for the five-leads follow-up. Frozen engines are unchanged.
use super::three_leads::*;
use serde_json::json;
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// CRT composition in homogeneous numerator coordinates. The ordinary
/// degree-two chart is normalized once at its boundary; exceptional divisors
/// use the complete Cantor law. Every field operation uses the counted field.
struct ProjectiveChart<'a> {
    j: &'a Jacobian,
    ordinary: Cell<u64>,
    fallback: Cell<u64>,
}
impl<'a> ProjectiveChart<'a> {
    fn new(j: &'a Jacobian) -> Self {
        Self {
            j,
            ordinary: Cell::new(0),
            fallback: Cell::new(0),
        }
    }
    fn add(&self, a: Divisor, b: Divisor) -> Divisor {
        if a.degree == 0 {
            return b;
        }
        if b.degree == 0 {
            return a;
        }
        if a.degree != 2 || b.degree != 2 {
            self.fallback.set(self.fallback.get() + 1);
            return self.j.add(a, b);
        }
        let f = &self.j.f;
        let (a0, a1, b0, b1) = (a.u[0], a.u[1], b.u[0], b.u[1]);
        let aa = f.sub(a1, b1);
        let bb = f.sub(a0, b0);
        let cc = f.sub(b.v[1], a.v[1]);
        let dd = f.sub(b.v[0], a.v[0]);
        let norm = f.add(
            f.sub(f.sq(bb), f.mul(f.mul(aa, bb), b1)),
            f.mul(f.sq(aa), b0),
        );
        let s1 = f.sub(f.mul(cc, bb), f.mul(aa, dd));
        if norm == 0 || s1 == 0 {
            self.fallback.set(self.fallback.get() + 1);
            return self.j.add(a, b);
        }
        let s0 = f.add(
            f.mul(dd, f.sub(bb, f.mul(aa, b1))),
            f.mul(f.mul(aa, cc), b0),
        );
        // W = norm*v_a + (s1*x+s0)*u_a. Its coefficients are integral
        // projective coordinates for the CRT interpolation polynomial.
        let w0 = f.add(f.mul(norm, a.v[0]), f.mul(s0, a0));
        let w1 = f.add(f.add(f.mul(norm, a.v[1]), f.mul(s0, a1)), f.mul(s1, a0));
        let w2 = f.add(s0, f.mul(s1, a1));
        let w3 = s1;
        let norm2 = f.sq(norm);
        let u2 = f.add(f.add(a0, b0), f.mul(a1, b1));
        let u3 = f.add(a1, b1);
        // (norm^2*f-W^2)/(u_a*u_b) = h0+h1*x-s1^2*x^2.
        let h2 = f.neg(f.sq(w3));
        let h5 = f.sub(
            f.mul(norm2, self.j.polynomial[5]),
            f.add(f.mul(w2, w3), f.mul(w2, w3)),
        );
        let h4 = f.sub(
            f.mul(norm2, self.j.polynomial[4]),
            f.add(f.sq(w2), f.add(f.mul(w1, w3), f.mul(w1, w3))),
        );
        let h1 = f.sub(h5, f.mul(h2, u3));
        let h0 = f.sub(f.sub(h4, f.mul(h2, u2)), f.mul(h1, u3));
        // One inversion normalizes both the outgoing u and v coordinates.
        let pair_inv = f.inv(f.mul(norm, s1));
        let norm_inv = f.mul(pair_inv, s1);
        let s1_inv = f.mul(pair_inv, norm);
        let scale = f.neg(f.sq(s1_inv));
        let out_u0 = f.mul(h0, scale);
        let out_u1 = f.mul(h1, scale);
        let r0 = f.add(
            f.sub(w0, f.mul(w2, out_u0)),
            f.mul(w3, f.mul(out_u1, out_u0)),
        );
        let r1 = f.add(
            f.sub(w1, f.mul(w2, out_u1)),
            f.mul(w3, f.sub(f.sq(out_u1), out_u0)),
        );
        let out = Divisor {
            degree: 2,
            u: [out_u0, out_u1, 1],
            v: [f.neg(f.mul(r0, norm_inv)), f.neg(f.mul(r1, norm_inv))],
        };
        self.ordinary.set(self.ordinary.get() + 1);
        out
    }
    fn mul(&self, mut a: Divisor, mut k: u64) -> Divisor {
        let mut r = Divisor::O;
        while k > 0 {
            if k & 1 != 0 {
                r = self.add(r, a);
            }
            k >>= 1;
            if k > 0 {
                a = self.add(a, a);
            }
        }
        r
    }
}

#[derive(Default)]
struct Components {
    parent: Vec<usize>,
    size: Vec<usize>,
    probes: u64,
}
impl Components {
    fn node(&mut self) {
        self.parent.push(self.parent.len());
        self.size.push(1);
    }
    fn root(&mut self, x: usize) -> usize {
        self.probes += 1;
        let p = self.parent[x];
        if p == x {
            x
        } else {
            let r = self.root(p);
            self.parent[x] = r;
            r
        }
    }
    fn score(&mut self, u: usize, v: usize) -> (bool, usize) {
        let a = self.root(u);
        let b = self.root(v);
        (a == b, self.size[a] + if a == b { 0 } else { self.size[b] })
    }
    fn join(&mut self, u: usize, v: usize) {
        let mut a = self.root(u);
        let mut b = self.root(v);
        if a == b {
            return;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
    }
}
fn uniform(rng: &mut Rng, r: u64, draws: &mut u64) -> u64 {
    let upper = u64::MAX - u64::MAX % r;
    loop {
        *draws += 1;
        let a = rng.next();
        if a < upper {
            return a % r;
        }
    }
}
fn evaluate(f: &Field, poly: &[u64], x: u64) -> u64 {
    poly.iter().rev().fold(0, |v, &a| f.add(f.mul(v, x), a))
}

/// Two prospective additions per continuation. All valid new relations from
/// both are inserted immediately. Component connectivity is a rank heuristic,
/// never a claim that a graph cycle alone recovers a scalar. The incumbent
/// signed elimination remains the authority for extracting a target relation.
pub fn solve_genus2_choice2(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    candidate_cap: u64,
) -> Outcome {
    solve_genus2_with_chart(inst, target, seed, candidate_cap, false)
}

pub fn solve_genus2_projective(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    candidate_cap: u64,
) -> Outcome {
    solve_genus2_with_chart(inst, target, seed, candidate_cap, true)
}

fn solve_genus2_with_chart(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    candidate_cap: u64,
    projective: bool,
) -> Outcome {
    assert_eq!(inst.family, "scholten");
    assert!(inst.curve.valid(target));
    let s = Scholten::new(inst.curve.f.p);
    assert_eq!(s.short_curve().c, inst.curve.c);
    let counts = || s.e.f.counts() + s.j.f.counts();
    let mut phases = vec![NativePhase {
        name: "destination_construction".into(),
        counts: counts(),
    }];
    let mut previous = counts();
    let p = s.tau(s.from_short(inst.generator));
    let q = s.tau(s.from_short(target));
    assert_ne!(p, Divisor::O);
    assert_ne!(q, Divisor::O);
    phases.push(NativePhase {
        name: "transfer".into(),
        counts: counts() - previous,
    });
    previous = counts();
    let ring = Ring::new(inst.r);
    let chart = ProjectiveChart::new(&s.j);
    let add = |a, b| {
        if projective {
            chart.add(a, b)
        } else {
            s.j.add(a, b)
        }
    };
    let mul = |a, k| {
        if projective {
            chart.mul(a, k)
        } else {
            s.j.mul(a, k)
        }
    };
    let mut rng = Rng(seed);
    let mut draws = 0;
    let mut jumps = vec![];
    for _ in 0..16 {
        let a = uniform(&mut rng, inst.r.min(4096), &mut draws);
        let b = uniform(&mut rng, inst.r.min(4096), &mut draws);
        jumps.push((add(mul(p, a), mul(q, b)), a, b));
    }
    let mut a = uniform(&mut rng, inst.r, &mut draws);
    let mut b = uniform(&mut rng, inst.r, &mut draws);
    let mut d = add(mul(p, a), mul(q, b));
    phases.push(NativePhase {
        name: "walk_initialization".into(),
        counts: counts() - previous,
    });
    previous = counts();
    let mut graph = SignedGraph::new(ring.clone());
    let mut components = Components::default();
    components.node();
    let mut nodes = BTreeMap::new();
    let mut seen = BTreeSet::new();
    let mut walk_seen = HashSet::new();
    let (mut attempts, mut continuations, mut split, mut duplicates, mut probes, mut restarts) =
        (0, 0, 0, 0, 0, 0);
    let mut first_cycle = None;
    let mut recovered = None;
    let mut max_walk_capacity = 0;
    'walk: while attempts < candidate_cap {
        continuations += 1;
        probes += 1;
        if !walk_seen.insert(d) {
            restarts += 1;
            walk_seen.clear();
            a = uniform(&mut rng, inst.r, &mut draws);
            b = uniform(&mut rng, inst.r, &mut draws);
            d = add(mul(p, a), mul(q, b));
        }
        max_walk_capacity = max_walk_capacity.max(walk_seen.capacity());
        let key =
            mix(d.u[0] ^ d.u[1].rotate_left(19) ^ d.v[0].rotate_left(37) ^ d.v[1].rotate_left(7));
        let indices = [
            key as usize % 16,
            ((key as usize % 16) + 1 + (rng.next() as usize % 15)) % 16,
        ];
        draws += 1;
        let mut best = None;
        for index in indices {
            if attempts == candidate_cap {
                break;
            }
            attempts += 1;
            let jump = &jumps[index];
            let nd = add(d, jump.0);
            let na = ring.add(a, jump.1);
            let nb = ring.add(b, jump.2);
            let mut priority = (false, 0usize);
            let mut repeated = false;
            if nd.degree == 0 {
                if nb != 0 {
                    recovered = Some(ring.mul(ring.neg(na), ring.inv(nb)));
                    break 'walk;
                }
            } else {
                let f = &s.j.f;
                let roots = if nd.degree == 1 {
                    Some(vec![f.neg(nd.u[0])])
                } else {
                    let disc = f.sub(f.sq(nd.u[1]), f.mul(4, nd.u[0]));
                    f.sqrt(disc).map(|root| {
                        let half = f.p.div_ceil(2);
                        vec![
                            f.mul(f.sub(root, nd.u[1]), half),
                            f.mul(f.sub(f.neg(root), nd.u[1]), half),
                        ]
                    })
                };
                if let Some(roots) = roots {
                    probes += 1;
                    let canonical = nd.min(s.j.neg(nd));
                    if !seen.insert(canonical) {
                        duplicates += 1;
                        repeated = true;
                    } else {
                        let mut edge = vec![];
                        for x in roots {
                            let y = evaluate(f, &nd.v[..nd.degree], x);
                            assert_eq!(f.sq(y), evaluate(f, &s.j.polynomial, x));
                            probes += 1;
                            let node = *nodes.entry(x).or_insert_with(|| {
                                components.node();
                                graph.node()
                            });
                            edge.push((node, if y <= f.p / 2 { 1 } else { -1 }));
                        }
                        if edge.len() == 1 {
                            edge.push((0, 1));
                        }
                        split += 1;
                        priority = components.score(edge[0].0, edge[1].0);
                        let cycles = graph.cycles;
                        recovered = graph.edge(edge[0].0, edge[0].1, edge[1].0, edge[1].1, na, nb);
                        components.join(edge[0].0, edge[1].0);
                        if graph.cycles > cycles && first_cycle.is_none() {
                            first_cycle = Some(attempts);
                        }
                        if recovered.is_some() {
                            break 'walk;
                        }
                    }
                }
            }
            if best
                .as_ref()
                .is_none_or(|(score, _, _, _, _)| priority > *score)
            {
                best = Some((priority, nd, na, nb, repeated));
            }
        }
        if let Some((_, next, na, nb, repeated)) = best {
            d = next;
            a = na;
            b = nb;
            if repeated {
                restarts += 1;
                walk_seen.clear();
                a = uniform(&mut rng, inst.r, &mut draws);
                b = uniform(&mut rng, inst.r, &mut draws);
                d = add(mul(p, a), mul(q, b));
            }
        }
    }
    phases.push(NativePhase {
        name: "relations_and_signed_elimination".into(),
        counts: counts() - previous,
    });
    let scalar = ring.counts.get();
    Outcome {
        recovered,
        exhausted: recovered.is_none(),
        phases,
        counts: counts(),
        native: BTreeMap::from([
            ("rng_draws_uncharged".into(), draws),
            ("hash_probes_uncharged".into(), probes),
            ("graph_operations_uncharged".into(), graph.operations),
            (
                "component_policy_probes_uncharged".into(),
                components.probes,
            ),
            ("scalar_adds_uncharged".into(), scalar.add),
            ("scalar_muls_uncharged".into(), scalar.mul),
            ("scalar_inversions_uncharged".into(), scalar.inv),
            ("processed_divisors".into(), attempts),
            ("walk_continuations".into(), continuations),
            ("split_relations".into(), split),
            ("graph_cycles".into(), graph.cycles),
            ("negative_cycles".into(), graph.negative_cycles),
            ("zero_target_coefficients".into(), graph.zero_coefficient),
            ("duplicate_relation_divisors".into(), duplicates),
            ("first_graph_cycle_trial".into(), first_cycle.unwrap_or(0)),
            ("large_prime_nodes".into(), graph.nodes() as u64),
            ("walk_cycle_restarts".into(), restarts),
            ("projective_chart_additions".into(), chart.ordinary.get()),
            (
                "projective_exceptional_fallbacks".into(),
                chart.fallback.get(),
            ),
            (
                "logical_component_bytes".into(),
                ((components.parent.capacity() + components.size.capacity())
                    * std::mem::size_of::<usize>()) as u64,
            ),
            (
                "logical_walk_seen_bytes".into(),
                (max_walk_capacity * std::mem::size_of::<Divisor>()) as u64,
            ),
            (
                "logical_graph_bytes".into(),
                (graph.nodes() * 64 + nodes.len() * 24) as u64,
            ),
        ]),
        detail: json!({"policy":"two_prospective_jumps_admit_both_relations_choose_connected_or_larger_component","candidate_cap":candidate_cap,"candidate_trials":attempts,"continuations":continuations,"first_cycle_trial":first_cycle,"p":inst.curve.f.p,"r":inst.r,"h":inst.h,"subgroup_map":"tau_pullback_plus_conjugate","dual_composition":"[2]","score_is_rank_heuristic":true,"unique_relations":true,"addition_backend":if projective {"crt_projective_numerator_chart_with_complete_exceptional_fallback"} else {"affine_cantor"}}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn projective_chart_agrees_with_complete_cantor_on_random_and_exceptional_inputs() {
        for p in [19, 31, 43, 59, 103, 739] {
            let s = Scholten::new(p);
            let chart = ProjectiveChart::new(&s.j);
            let mut rng = Rng(0x2b19 + p);
            let mut samples = vec![Divisor::O];
            for x in 0..p {
                if let Some(y) = s.j.f.sqrt(evaluate(&s.j.f, &s.j.polynomial, x)) {
                    samples.push(s.j.point(x, y));
                    if samples.len() == 12 {
                        break;
                    }
                }
            }
            let base = s.j.add(samples[1], samples[2]);
            for _ in 0..500 {
                let ka = 1 + rng.next() % (4 * p);
                let kb = 1 + rng.next() % (4 * p);
                let a = s.j.mul(base, ka);
                let b = s.j.mul(base, kb);
                let expected = s.j.add(a, b);
                let actual = chart.add(a, b);
                assert_eq!(actual, expected, "p={p}, ka={ka}, kb={kb}");
                assert!(s.j.valid(actual));
            }
            for &a in &samples {
                for &b in &samples {
                    assert_eq!(chart.add(a, b), s.j.add(a, b));
                    assert_eq!(chart.add(a, s.j.neg(a)), Divisor::O);
                }
            }
            assert!(chart.ordinary.get() > 0);
            assert!(chart.fallback.get() > 0);
        }
    }
    #[test]
    fn projective_collector_recovers_source_scalars() {
        for p in [19, 31, 43, 59] {
            let Some(inst) = scholten_instance(p, 20261019 + p) else {
                continue;
            };
            for k in 1..8 {
                let target = inst.curve.mul(inst.generator, k);
                if target.inf {
                    continue;
                }
                let out = solve_genus2_projective(&inst, target, 20261020 + k, 100 * p + 10000);
                assert_eq!(out.recovered, Some(k % inst.r));
                assert_eq!(
                    inst.curve.mul(inst.generator, out.recovered.unwrap()),
                    target
                );
                assert_eq!(
                    out.phases
                        .iter()
                        .fold(Counts::default(), |c, p| c + p.counts),
                    out.counts
                );
            }
        }
    }
    #[test]
    fn choice2_recovers_from_relations_and_counts_both_choices() {
        for p in [19, 31, 43, 59] {
            let Some(inst) = scholten_instance(p, 20261018 + p) else {
                continue;
            };
            for k in 1..8 {
                let target = inst.curve.mul(inst.generator, k);
                if target.inf {
                    continue;
                }
                let out = solve_genus2_choice2(&inst, target, 20261019 + k, 100 * p + 10000);
                assert_eq!(out.recovered, Some(k % inst.r));
                assert_eq!(
                    inst.curve.mul(inst.generator, out.recovered.unwrap()),
                    target
                );
                assert!(out.native["processed_divisors"] <= 100 * p + 10000);
                assert!(out.native["processed_divisors"] <= 2 * out.native["walk_continuations"]);
                assert_eq!(
                    out.phases
                        .iter()
                        .fold(Counts::default(), |c, p| c + p.counts),
                    out.counts
                );
            }
        }
    }
    #[test]
    fn components_score_does_not_claim_an_algebraic_solution() {
        let mut c = Components::default();
        for _ in 0..4 {
            c.node();
        }
        assert_eq!(c.score(1, 2), (false, 2));
        c.join(1, 2);
        assert_eq!(c.score(1, 2), (true, 2));
        assert_eq!(c.score(1, 3), (false, 3));
        let ring = Ring::new(101);
        let mut graph = SignedGraph::new(ring);
        let u = graph.node();
        assert_eq!(graph.edge(u, 1, u, 1, 20, 2), None);
        assert_eq!(graph.negative_cycles, 1);
    }
}
