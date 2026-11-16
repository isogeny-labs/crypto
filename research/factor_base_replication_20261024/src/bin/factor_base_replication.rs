//! Independently seeded replication of the frozen factor-base/graph experiment.
#[path = "cost.rs"]
#[allow(dead_code)]
mod cost;
use cost::Counted;
use divisor_arithmetic::g2::ramified::not_char2::{
    add as ext_add, double as ext_double, CurveConstants,
};
use five_leads_study::three_leads::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::process::Command;

const REPLICATION_ID: &str = "factor-base-graph-replication/20261024/v1";
const PROTOCOL_SHA256: &str = "ace04952556bfe1cb9b1a0c8cb5dec6715040f55dd376d1e3ca3d7b659738997";

fn replication_digest(domain: &str, bits: u64, workload: &Value) -> [u8; 32] {
    let mut h = Sha256::new();
    let bits = bits.to_string();
    let target_index = workload["target_index"].as_u64().unwrap().to_string();
    for part in [
        REPLICATION_ID,
        domain,
        &bits,
        workload["curve"]["icv1"].as_str().unwrap(),
        &target_index,
    ] {
        h.update(part);
        h.update([0]);
    }
    h.finalize().into()
}

fn digest_u64(digest: &[u8; 32]) -> u64 {
    u64::from_be_bytes(digest[..8].try_into().unwrap())
}

fn hex_digest(digest: &[u8]) -> String {
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

fn workload_digest(bits: u64, workload: &Value, planted: u64, target: Point) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(replication_digest("workload/v1", bits, workload));
    h.update(planted.to_be_bytes());
    h.update(target.x.to_be_bytes());
    h.update(target.y.to_be_bytes());
    h.finalize().into()
}

#[derive(Clone, Copy)]
struct Policy {
    base_denominator: u64,
    jump_count: usize,
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
fn choose(n: usize, k: usize) -> u64 {
    let mut x = 1;
    for i in 0..k {
        x = x * (n - i) as u64 / (i + 1) as u64;
    }
    x
}
fn transform(poly: &[u64], alpha: u64, beta: u64, scale: u64, f: &Field) -> Vec<u64> {
    let mut out = vec![0; poly.len()];
    let inv = f.inv(scale);
    for (k, &coefficient) in poly.iter().enumerate() {
        for (j, slot) in out.iter_mut().enumerate().take(k + 1) {
            let term = f.mul(
                coefficient,
                f.mul(
                    choose(k, j) % f.p,
                    f.mul(f.pow(alpha, j as u64), f.pow(beta, (k - j) as u64)),
                ),
            );
            *slot = f.add(*slot, f.mul(term, inv));
        }
    }
    while out.last() == Some(&0) {
        out.pop();
    }
    out
}
fn map_divisor(d: Divisor, alpha: u64, beta: u64, f: &Field) -> Divisor {
    if d.degree == 0 {
        return d;
    }
    let u = transform(
        &d.u[..=d.degree],
        alpha,
        beta,
        f.pow(alpha, d.degree as u64),
        f,
    );
    let v = transform(&d.v[..d.degree], alpha, beta, f.pow(alpha, 3), f);
    let mut out = Divisor {
        degree: d.degree,
        u: [0; 3],
        v: [0; 2],
    };
    out.u[..u.len()].copy_from_slice(&u);
    out.v[..v.len()].copy_from_slice(&v);
    out
}

fn solve<const P: u64>(
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    candidate_cap: u64,
    policy: Policy,
) -> Outcome {
    let rank_policy = false;
    assert_eq!(inst.family, "scholten");
    assert_eq!(inst.curve.f.p, P);
    assert!(inst.curve.valid(target));
    cost::reset();
    let s = Scholten::new(inst.curve.f.p);
    assert_eq!(s.short_curve().c, inst.curve.c);
    let old = &s.j.polynomial;
    let lead = old[5];
    let shift = s.j.f.neg(s.j.f.div(old[4], s.j.f.mul(5, lead)));
    let normalized = transform(old, lead, shift, s.j.f.pow(lead, 6), &s.j.f);
    assert_eq!(normalized.len(), 6);
    assert_eq!(normalized[5], 1);
    assert_eq!(normalized[4], 0);
    let j = Jacobian {
        f: Field::new(P, 1, 0),
        polynomial: normalized,
    };
    let cc = CurveConstants {
        f3: Counted::<P>::new(j.polynomial[3]),
        f2: Counted::<P>::new(j.polynomial[2]),
        f1: Counted::<P>::new(j.polynomial[1]),
        f0: Counted::<P>::new(j.polynomial[0]),
    };
    let counts = || {
        let e = cost::counts();
        s.e.f.counts()
            + s.j.f.counts()
            + j.f.counts()
            + Counts {
                m: e.m,
                s: e.s,
                i: e.i,
                field_adds: e.a,
                ..Default::default()
            }
    };
    let mut phases = vec![NativePhase {
        name: "destination_construction".into(),
        counts: counts(),
    }];
    let mut previous = counts();
    let p = map_divisor(s.tau(s.from_short(inst.generator)), lead, shift, &j.f);
    let q = map_divisor(s.tau(s.from_short(target)), lead, shift, &j.f);
    assert_ne!(p, Divisor::O);
    assert_ne!(q, Divisor::O);
    assert!(j.valid(p));
    assert!(j.valid(q));
    phases.push(NativePhase {
        name: "transfer".into(),
        counts: counts() - previous,
    });
    previous = counts();
    let ring = Ring::new(inst.r);
    let equality_dispatches = Cell::new(0u64);
    let external_additions = Cell::new(0u64);
    let add = |a, b| {
        external_additions.set(external_additions.get() + 1);
        let ea = cost::external::<P>(a);
        let eb = cost::external::<P>(b);
        let out = if a == b {
            equality_dispatches.set(equality_dispatches.get() + 1);
            ext_double(&ea, &cc)
        } else {
            ext_add(&ea, &eb, &cc)
        };
        cost::native::<P>(out)
    };
    let mul = |mut a, mut k| {
        let mut r = Divisor::O;
        while k > 0 {
            if k & 1 != 0 {
                r = add(r, a);
            }
            k >>= 1;
            if k > 0 {
                a = add(a, a);
            }
        }
        r
    };
    let mut rng = Rng(seed);
    let mut draws = 0;
    let mut jumps = vec![];
    for _ in 0..policy.jump_count {
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
    let mut rank_policy_probes = 0;
    let mut rank_policy_scores = [0u64; 4];
    let mut base_tests = 0u64;
    let mut rejected_large_large = 0u64;
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
            key as usize % policy.jump_count,
            ((key as usize % policy.jump_count)
                + 1
                + (rng.next() as usize % (policy.jump_count - 1)))
                % policy.jump_count,
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
                let f = &j.f;
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
                    // The full-base arm admits every split relation. A smaller
                    // base admits only base/base or base/large-prime edges.
                    // The subset is target-blind and fixed by p and x.
                    let eligible = if policy.base_denominator == 1 {
                        true
                    } else {
                        roots.iter().any(|&x| {
                            base_tests += 1;
                            mix(x ^ P).is_multiple_of(policy.base_denominator)
                        })
                    };
                    if !eligible {
                        rejected_large_large += 1;
                    } else {
                        let canonical = nd.min(j.neg(nd));
                        if !seen.insert(canonical) {
                            duplicates += 1;
                            repeated = true;
                        } else {
                            let mut edge = vec![];
                            for x in roots {
                                let y = evaluate(f, &nd.v[..nd.degree], x);
                                assert_eq!(f.sq(y), evaluate(f, &j.polynomial, x));
                                probes += 1;
                                let node = *nodes.entry(x).or_insert_with(|| {
                                    if !rank_policy {
                                        components.node();
                                    }
                                    graph.node()
                                });
                                edge.push((node, if y <= f.p / 2 { 1 } else { -1 }));
                            }
                            if edge.len() == 1 {
                                edge.push((0, 1));
                            }
                            split += 1;
                            if rank_policy {
                                rank_policy_probes += 1;
                                let score = graph.target_rank_score(
                                    edge[0].0, edge[0].1, edge[1].0, edge[1].1, nb,
                                );
                                rank_policy_scores[score as usize] += 1;
                                priority = (score > 0, score as usize);
                            } else {
                                priority = components.score(edge[0].0, edge[1].0);
                            }
                            let cycles = graph.cycles;
                            recovered =
                                graph.edge(edge[0].0, edge[0].1, edge[1].0, edge[1].1, na, nb);
                            if !rank_policy {
                                components.join(edge[0].0, edge[1].0);
                            }
                            if graph.cycles > cycles && first_cycle.is_none() {
                                first_cycle = Some(attempts);
                            }
                            if recovered.is_some() {
                                break 'walk;
                            }
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
    let mut native = BTreeMap::from([
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
        ("factor_base_membership_tests_uncharged".into(), base_tests),
        (
            "rejected_large_large_relations".into(),
            rejected_large_large,
        ),
        ("walk_continuations".into(), continuations),
        ("split_relations".into(), split),
        ("graph_cycles".into(), graph.cycles),
        ("negative_cycles".into(), graph.negative_cycles),
        ("zero_target_coefficients".into(), graph.zero_coefficient),
        ("duplicate_relation_divisors".into(), duplicates),
        ("first_graph_cycle_trial".into(), first_cycle.unwrap_or(0)),
        ("large_prime_nodes".into(), graph.nodes() as u64),
        ("walk_cycle_restarts".into(), restarts),
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
    ]);
    native.insert(
        "external_additions_uncharged".into(),
        external_additions.get(),
    );
    native.insert(
        "external_equality_dispatches_uncharged".into(),
        equality_dispatches.get(),
    );
    let mut detail = json!({"policy":"target_blind_small_base_at_least_one_endpoint_with_signed_large_prime_graph","base_denominator":policy.base_denominator,"base_rule":"mix(x xor p) mod base_denominator == 0","jump_count":policy.jump_count,"candidate_cap":candidate_cap,"candidate_trials":attempts,"continuations":continuations,"first_cycle_trial":first_cycle,"p":inst.curve.f.p,"r":inst.r,"h":inst.h,"subgroup_map":"tau_pullback_plus_conjugate","dual_composition":"[2]","score_is_rank_heuristic":true,"unique_relations":true,"addition_backend":"wrapped_divisor_arithmetic_commit_87e2ebc1_normalized_quintic"});
    if rank_policy {
        native.insert("rank_policy_probes_uncharged".into(), rank_policy_probes);
        for (score, count) in rank_policy_scores.iter().enumerate() {
            native.insert(format!("rank_policy_score_{score}"), *count);
        }
        detail["policy"] = json!("two_prospective_jumps_admit_both_relations_choose_signed_target_coefficient_rank_effect");
        detail["score_uses_exact_target_coefficient"] = json!(true);
        detail["rank_score_definition"] =
            json!("0=target_free,1=target_bearing_merge,2=target_bearing_anchor,3=target_equation");
    }
    Outcome {
        recovered,
        exhausted: recovered.is_none(),
        phases,
        counts: counts(),
        native,
        detail,
    }
}

fn dispatch(
    p: u64,
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    cap: u64,
    policy: Policy,
) -> Outcome {
    match p {
        739 => solve::<739>(inst, target, seed, cap, policy),
        1151 => solve::<1151>(inst, target, seed, cap, policy),
        1187 => solve::<1187>(inst, target, seed, cap, policy),
        3371 => solve::<3371>(inst, target, seed, cap, policy),
        3527 => solve::<3527>(inst, target, seed, cap, policy),
        3931 => solve::<3931>(inst, target, seed, cap, policy),
        11731 => solve::<11731>(inst, target, seed, cap, policy),
        11887 => solve::<11887>(inst, target, seed, cap, policy),
        12143 => solve::<12143>(inst, target, seed, cap, policy),
        48383 => solve::<48383>(inst, target, seed, cap, policy),
        48823 => solve::<48823>(inst, target, seed, cap, policy),
        49451 => solve::<49451>(inst, target, seed, cap, policy),
        186379 => solve::<186379>(inst, target, seed, cap, policy),
        186451 => solve::<186451>(inst, target, seed, cap, policy),
        186583 => solve::<186583>(inst, target, seed, cap, policy),
        741967 => solve::<741967>(inst, target, seed, cap, policy),
        748807 => solve::<748807>(inst, target, seed, cap, policy),
        749299 => solve::<749299>(inst, target, seed, cap, policy),
        2967343 => solve::<2967343>(inst, target, seed, cap, policy),
        2968367 => solve::<2968367>(inst, target, seed, cap, policy),
        2972219 => solve::<2972219>(inst, target, seed, cap, policy),
        _ => panic!("unexpected p={p}"),
    }
}
fn run_row(row: Value, bits: u64, policy: Policy) -> Value {
    let w = &row["workload"];
    let spec = &w["curve_spec"];
    let p = spec["p"].as_u64().unwrap();
    let r = spec["r"].as_u64().unwrap();
    let order = spec["group_order"].as_u64().unwrap();
    let generator = Point::affine(spec["gx"].as_u64().unwrap(), spec["gy"].as_u64().unwrap());
    let curve = Elliptic {
        f: Field::new(p, 2, spec["modulus"].as_u64().unwrap()),
        c: [
            spec["b"].as_u64().unwrap(),
            spec["a"].as_u64().unwrap(),
            0,
            1,
        ],
        binary: false,
    };
    let inst = StudyInstance {
        curve,
        order,
        r,
        h: order / r,
        generator,
        family: "scholten".into(),
        parameter: 0,
        evidence: Value::Null,
    };
    assert_eq!(order % r, 0);
    assert!(inst.curve.valid(generator));
    let target_seed_digest = replication_digest("target-scalar/v1", bits, w);
    let planted = 1 + digest_u64(&target_seed_digest) % (r - 1);
    let target = inst.curve.mul(inst.generator, planted);
    assert!(!target.inf);
    assert!(inst.curve.valid(target));
    let algorithm_seed_digest = replication_digest("algorithm-seed/v1", bits, w);
    let seed = digest_u64(&algorithm_seed_digest);
    let workload_digest = workload_digest(bits, w, planted, target);
    let workload_sha256 = hex_digest(&workload_digest);
    let workload_id = format!("W{}", &workload_sha256[..12]);
    let cap: u64 = row["method"]["params"]["trial_cap"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let mut out = dispatch(p, &inst, target, seed, cap, policy);
    let verified = if let Some(k) = out.recovered {
        let verifier = inst.fresh();
        let got = verifier.curve.mul(verifier.generator, k);
        let c = verifier.curve.f.counts();
        out.counts = out.counts + c;
        out.phases.push(NativePhase {
            name: "source_verification".into(),
            counts: c,
        });
        got == target && k == planted
    } else {
        false
    };
    // Preserve an incorrect recovery or cap exhaustion as an explicit row;
    // one bad target must not erase the rest of the cohort.
    let instrument = inst.fresh();
    let twice = instrument.curve.mul(instrument.generator, 2);
    instrument.curve.f.reset();
    let _ = instrument.curve.add(instrument.generator, twice);
    let source_add = instrument.curve.f.counts();
    assert!(source_add.m > 0);
    // Run the protocol's strongest source reference in this same executable
    // on the same target. Keep its own independent source verification charge.
    let mut rho = solve_rho(&inst, target, seed, cap, true, false);
    let rho_verified = if let Some(k) = rho.recovered {
        let verifier = inst.fresh();
        let got = verifier.curve.mul(verifier.generator, k);
        let c = verifier.curve.f.counts();
        rho.counts = rho.counts + c;
        rho.phases.push(NativePhase {
            name: "source_verification".into(),
            counts: c,
        });
        got == target && k == planted
    } else {
        false
    };
    json!({"schema":"factor-base-graph-replication-record/v1","replication_id":REPLICATION_ID,"protocol_sha256":PROTOCOL_SHA256,
    "source_workload_id":w["workload_id"],"source_workload_sha256":w["workload_sha256"],
    "workload_id":workload_id,"workload_sha256":workload_sha256,"bits":bits,"p":p,"target_index":w["target_index"],
    "curve_icv1":w["curve"]["icv1"],"target":[format!("0x{:x}",target.x),format!("0x{:x}",target.y)],
    "target_derivation":{"schema":"sha256-domain-separated/v1","target_domain":"target-scalar/v1","algorithm_domain":"algorithm-seed/v1","target_digest":hex_digest(&target_seed_digest),"algorithm_digest":hex_digest(&algorithm_seed_digest)},
    "algorithm_seed":seed.to_string(),"cap":cap,"planted":planted,"recovered":out.recovered,"verified":verified,"exhausted":out.exhausted,"counts":out.counts,"source_add":source_add,"native":out.native,"phases":out.phases,"detail":out.detail,
    "rho":{"recovered":rho.recovered,"verified":rho_verified,"exhausted":rho.exhausted,"counts":rho.counts,"source_add":source_add,"native":rho.native,"phases":rho.phases,"detail":rho.detail}})
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() == 4,
        "usage: factor-base-replication BITS BASE_DENOMINATOR JUMP_COUNT"
    );
    let bits: u64 = args[1].parse().unwrap();
    let policy = Policy {
        base_denominator: args[2].parse().unwrap(),
        jump_count: args[3].parse().unwrap(),
    };
    assert!([1, 2, 4].contains(&policy.base_denominator));
    assert!([8, 16, 32].contains(&policy.jump_count));
    assert!([16, 20, 24, 28, 32, 36, 40].contains(&bits));
    let path=format!("research/five_leads_e2e_20261019/sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let extracted = if std::env::var_os("FOUR_FOLLOWUPS_ARCHIVE_INPUT").is_some() {
        None
    } else {
        fs::read_to_string(&path).ok()
    };
    let input = extracted.unwrap_or_else(|| {
        let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
        let out = Command::new("tar")
            .args([
                "--zstd",
                "-xOf",
                "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst",
                &member,
            ])
            .output()
            .expect("read committed input archive");
        assert!(
            out.status.success(),
            "tar failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).expect("UTF-8 records")
    });
    let rows: Vec<Value> = input
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .filter(|r: &Value| r["arm"] == "genus_projective")
        .collect();
    assert_eq!(rows.len(), 180);
    for row in rows {
        println!("{}", run_row(row, bits, policy));
    }
}
