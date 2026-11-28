//! Same-root-pool policy comparison. Full references never enter the solver.
use super::{jv_forced_divisor::Family, jv_forced_graph as graph};
use crypto_lib::cryptanalysis::{
    f4_fp::{self, F4Options},
    jv_bitops,
    jv_cover::{self as cover, BaseEl, Div, Fld, Fq, LinMode, ZeroDimStats, E2},
    jv_sieve::FpRing,
};
use rand::{rngs::StdRng, SeedableRng};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::Path,
    time::{Duration, Instant},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const FIXTURE_SHA: &str = "d09c03c2734e32b36c229f1fc52997c05e9dfc1b12ba7f8c57c330364d6c26e9";
fn ensure(b: bool, s: &str) -> Result<()> {
    if b {
        Ok(())
    } else {
        Err(s.into())
    }
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn encode(v: &[E2]) -> Vec<[u64; 2]> {
    v.iter().map(|e| e.0).collect()
}
fn native(v: &[[u64; 2]]) -> Vec<E2> {
    v.iter().copied().map(E2).collect()
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Small {
    original_index: usize,
    x: u64,
    y: [u64; 2],
    sign: i8,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Large {
    key: [u64; 6],
    coefficient: i8,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Seed {
    schema: String,
    raw_sha256: String,
    p: u64,
    source_id: usize,
    h: Vec<[u64; 2]>,
    a: Vec<[u64; 2]>,
    c: Vec<[u64; 2]>,
    norm: Vec<u64>,
    small: Vec<Small>,
    remainders: Vec<Large>,
    normalized_coordinates: Vec<u64>,
}
fn family(seed: &Seed, f: &Fq) -> Result<Family> {
    ensure(
        seed.p == 1009
            && seed.source_id == 0
            && seed.schema == "graph-ic.anchored-forced-divisor.fixture/v1"
            && seed.raw_sha256
                == "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d"
            && seed.remainders.len() == 1
            && seed.small.len() == 7,
        "frozen public anchor",
    )?;
    let r = &seed.remainders[0];
    ensure([-1, 1].contains(&r.coefficient), "quadratic sign")?;
    let k = r.key;
    let mut v = vec![E2([k[2], k[3]]), E2([k[4], k[5]])];
    if r.coefficient == -1 {
        for x in &mut v {
            *x = f.neg(x);
        }
    }
    Ok(Family::new(
        f,
        &native(&seed.h),
        &[f.from_fp(k[0]), f.from_fp(k[1]), E2::ONE],
        &v,
    )?)
}
fn base(f: &Fq, h: &[E2]) -> Vec<BaseEl> {
    let mut rng = StdRng::seed_from_u64(13);
    let mut out = vec![];
    for x in 0..f.p {
        let rhs = h
            .iter()
            .rev()
            .fold(E2::ZERO, |v, c| f.add(&f.scale(&v, x), c));
        let Some(y) = f.sqrt(&rhs, &mut rng) else {
            continue;
        };
        if y == E2::ZERO {
            continue;
        }
        let neg = f.neg(&y);
        let y = if y.0 <= neg.0 { y } else { neg };
        out.push(BaseEl {
            x,
            y,
            d: Div {
                u: vec![f.neg(&f.from_fp(x)), E2::ONE],
                v: vec![y],
            },
        });
    }
    out
}
fn anchor(
    seed: &Seed,
    f: &Fq,
    family: &Family,
    domain: &graph::Domain<'_>,
    base: &[BaseEl],
) -> Result<graph::PrincipalProof> {
    for s in &seed.small {
        let b = base
            .get(s.original_index)
            .ok_or("anchor original base index")?;
        ensure(
            b.x == s.x && b.y.0 == s.y && [-1, 1].contains(&s.sign),
            "anchor original canonical base binding",
        )?;
    }
    let point = family.recover_parameters(f, &native(&seed.a), &native(&seed.c))?;
    ensure(
        point == seed.normalized_coordinates && point[0] == 369,
        "known positive coordinates",
    )?;
    let function = family.instantiate(f, &point)?;
    let scale = f.inv(&f.sq(&native(&seed.c)[1]));
    let expected: Vec<_> = seed
        .norm
        .iter()
        .map(|&x| f.mul(&f.from_fp(x), &scale))
        .collect();
    ensure(function.norm == expected, "original positive norm")?;
    let roots: Vec<_> = seed.small.iter().map(|s| s.x).collect();
    let proof = graph::PrincipalProof::check(domain, family, &function, &roots, &[])?;
    ensure(proof.large_terms().len() == 1, "checked one-LP anchor")?;
    let expected: Vec<_> = seed
        .small
        .iter()
        .map(|s| (s.original_index, s.sign))
        .collect();
    ensure(
        serde_json::to_value(&proof)?["small"] == json!(expected),
        "original anchor signed terms",
    )?;
    Ok(proof)
}
#[derive(Clone, Copy)]
enum Mode {
    Full,
    Traced,
}
impl Mode {
    fn name(self) -> &'static str {
        match self {
            Self::Full => "full-reference",
            Self::Traced => "traced",
        }
    }
}
fn options(mode: Mode) -> F4Options {
    cover::opts_for_mode(
        24,
        10.0,
        match mode {
            Mode::Full => cover::StopMode::Full,
            Mode::Traced => cover::StopMode::Staircase(128),
        },
    )
}
fn solve(
    input: &[f4_fp::Poly],
    p: u64,
    mode: Mode,
    opts: &F4Options,
    index: usize,
) -> (Vec<Vec<u64>>, ZeroDimStats) {
    if opts.deadline.is_some_and(|d| Instant::now() >= d) {
        return (
            vec![],
            ZeroDimStats {
                timed_out: true,
                ..Default::default()
            },
        );
    }
    let (mut roots, mut stats) = cover::solve_zero_dim_mode(
        input,
        7,
        p,
        opts,
        &mut StdRng::seed_from_u64(0x9470_0000 ^ index as u64),
        matches!(mode, Mode::Traced),
        LinMode::Fast,
    );
    roots.sort();
    roots.dedup();
    if opts.deadline.is_some_and(|d| Instant::now() >= d) {
        stats.timed_out = true;
    }
    (roots, stats)
}
struct Policy {
    graph: graph::Graph,
    edges: Vec<graph::CompositeEdge>,
    projections: Vec<Value>,
}
impl Policy {
    fn new() -> Result<Self> {
        Ok(Self {
            graph: graph::Graph::new(graph::Options::default())?,
            edges: vec![],
            projections: vec![],
        })
    }
    fn admit(
        &mut self,
        domain: &graph::Domain<'_>,
        candidate: graph::PrincipalProof,
        anchor: &graph::PrincipalProof,
    ) -> Result<Value> {
        let edge = graph::CompositeEdge::cancel(domain, candidate, anchor.clone())?;
        if edge.is_trivial() {
            return Ok(json!({"trivial":true,"admitted":false}));
        }
        let id = self.edges.len();
        let endpoints = edge.large_terms().len();
        let events = edge.insert(domain, &mut self.graph, id)?;
        self.edges.push(edge);
        let mut projections = vec![];
        for event in &events {
            if let graph::Event::Full { source, witness } = event {
                let terms = graph::project_integer(domain, &self.edges, witness)?;
                let row =
                    json!({"activation_source":source,"witness":witness,"integer_small":terms});
                projections.push(row.clone());
                self.projections.push(row);
            }
        }
        Ok(
            json!({"admitted":true,"trivial":false,"source":id,"endpoints":endpoints,"proof":self.edges[id],"events":events,"projections":projections}),
        )
    }
}
fn remove_root(poly: &[u64], r: u64, p: u64) -> Result<Vec<u64>> {
    ensure(poly.len() >= 2, "positive-degree root division")?;
    let n = poly.len() - 1;
    let mut q = vec![0; n];
    q[n - 1] = poly[n];
    for i in (1..n).rev() {
        q[i - 1] = (poly[i] + r * q[i]) % p;
    }
    ensure(
        (poly[0] + r * q[0]).is_multiple_of(p),
        "exact root division",
    )?;
    Ok(q)
}
fn case(
    f: &Fq,
    family: &Family,
    domain: &graph::Domain<'_>,
    anchor: &graph::PrincipalProof,
    policies: &mut [Policy; 2],
    case_spec: (u64, usize),
    mode: Mode,
) -> Result<Value> {
    let (parameter, index) = case_spec;
    let opts = options(mode);
    let input = family.specialize(0, parameter)?;
    let (roots, mut stats) = solve(&input, f.p, mode, &opts, index);
    let before = f.muls();
    let ring = FpRing::new(f.p);
    let mut functions = vec![];
    let mut invalid = false;
    for (root_index, root) in roots.iter().enumerate() {
        ensure(
            root.len() == 7 && root.iter().all(|&x| x < f.p),
            "canonical root",
        )?;
        if input.iter().any(|q| f4_fp::eval(q, root, f.p) != 0) {
            invalid = true;
        }
        let mut point = root.clone();
        point.insert(0, parameter);
        let mut row = json!({"point":point});
        match family.instantiate(f, &point) {
            Err(error) => {
                row["function_refusal"] = json!(error);
                invalid |= error != "C is not invertible on forced divisor";
            }
            Ok(value) => {
                let polynomial: Vec<_> = value.quotient.iter().map(|e| e.0[0]).collect();
                let small = ring.roots_by_table(&polynomial);
                row["normalized_a"] = json!(encode(&value.a));
                row["normalized_c"] = json!(encode(&value.c));
                row["norm"] = json!(encode(&value.norm));
                row["small_roots"] = json!(small);
                if [5, 7].contains(&small.len()) {
                    let proof = graph::PrincipalProof::check(domain, family, &value, &small, &[])?;
                    for (i, name) in ["old", "expanded"].iter().enumerate() {
                        let start = f.muls();
                        row[*name] = policies[i].admit(domain, proof.clone(), anchor)?;
                        row[format!("{name}_legacy_fq_products")] = json!(f.muls() - start);
                    }
                } else if small.len() == 3 {
                    row["old"] = json!({"admitted":false,"policy_refusal":"three-large-prime support excluded"});
                    let start = f.muls();
                    let mut tail = polynomial;
                    for &r in &small {
                        tail = remove_root(&tail, r, f.p)?;
                    }
                    let report = graph::split_quartic(
                        &ring,
                        &tail,
                        graph::SplitOptions { max_attempts: 8 },
                        &mut StdRng::seed_from_u64(
                            0xabc0_0000 ^ ((index as u64) << 16) ^ root_index as u64,
                        ),
                    )?;
                    if let Some(factors) = &report.factors {
                        match graph::PrincipalProof::check(domain, family, &value, &small, factors)
                        {
                            Ok(proof) => {
                                row["expanded"] = policies[1].admit(domain, proof, anchor)?
                            }
                            Err(error) => {
                                row["expanded"] = json!({"admitted":false,"support_refusal":error})
                            }
                        }
                    } else {
                        row["expanded"] = json!({"admitted":false,"split_refusal":report.refusal});
                    }
                    row["expanded_legacy_fq_products"] = json!(f.muls() - start);
                    row["split_report"] = json!(report);
                } else {
                    row["old"] =
                        json!({"admitted":false,"policy_refusal":"unsupported small-root count"});
                    row["expanded"] = row["old"].clone();
                }
            }
        }
        functions.push(row);
    }
    if opts.deadline.is_some_and(|d| Instant::now() >= d) {
        stats.timed_out = true;
    }
    let resolved = !stats.timed_out && !stats.incomplete && stats.nonseparating == 0;
    let mut policy_products = [0u64; 2];
    for row in &functions {
        for (i, name) in ["old", "expanded"].iter().enumerate() {
            policy_products[i] += row[format!("{name}_legacy_fq_products")]
                .as_u64()
                .unwrap_or(0);
        }
    }
    let post = f.muls() - before;
    ensure(
        post >= policy_products.iter().sum(),
        "nonoverlapping legacy components",
    )?;
    Ok(
        json!({"case":index,"parameter":parameter,"mode":mode.name(),"roots":roots,"root_stats":stats,"resolved":resolved,"invalid_original_replay":invalid,"functions":functions,"post_legacy_fq_products":post,"policy_legacy_fq_products":policy_products,"common_original_legacy_fq_products":post-policy_products.iter().sum::<u64>(),"old_graph":policies[0].graph.stats(),"expanded_graph":policies[1].graph.stats()}),
    )
}
fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    writeln!(file)?;
    file.sync_all()?;
    Ok(())
}
fn event(log: &mut fs::File, value: &Value) -> Result<()> {
    writeln!(log, "{value}")?;
    log.flush()?;
    log.sync_data()?;
    Ok(())
}
fn core(seed: &Seed, out: &Path) -> Result<()> {
    jv_bitops::enable(false);
    let parameters: Vec<_> = std::iter::once(369u64).chain(0..128).collect();
    let mut log = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("events.jsonl"))?;
    let mut samples = vec![];
    let mut reference = Vec::<Value>::new();
    let mut setups = vec![];
    let mut graphs = vec![];
    let mut stopped = None;
    let mut complete = true;
    'arms: for (arm, mode) in [Mode::Full, Mode::Traced].into_iter().enumerate() {
        cover::clear_traces();
        cover::set_trace_scope(0x9040_0000 + arm as u64);
        let f = Fq::new(seed.p);
        let h = native(&seed.h);
        let family = family(seed, &f)?;
        let base = base(&f, &h);
        let domain = graph::Domain::new(&f, &h, &base)?;
        let anchor = anchor(seed, &f, &family, &domain, &base)?;
        setups.push(json!({"mode":mode.name(),"base_columns":base.len(),"family_base_domain_anchor_legacy_fq_products":f.muls()}));
        let mut policies = [Policy::new()?, Policy::new()?];
        for (index, &parameter) in parameters.iter().enumerate() {
            event(
                &mut log,
                &json!({"event":"start","mode":mode.name(),"case":index,"parameter":parameter}),
            )?;
            let mut sample = match case(
                &f,
                &family,
                &domain,
                &anchor,
                &mut policies,
                (parameter, index),
                mode,
            ) {
                Ok(v) => v,
                Err(e) => {
                    json!({"case":index,"parameter":parameter,"mode":mode.name(),"error":e.to_string(),"resolved":false,"invalid_original_replay":true})
                }
            };
            if arm == 0 {
                if index == 0 {
                    let known = sample["functions"].as_array().is_some_and(|rows| {
                        rows.iter().any(|r| {
                            r["point"] == json!(seed.normalized_coordinates)
                                && r["old"]["trivial"] == true
                                && r["expanded"]["trivial"] == true
                        })
                    });
                    let positive = known
                        && sample["resolved"] == true
                        && sample["invalid_original_replay"] == false;
                    sample["retained_positive_control_pass"] = json!(positive);
                    if !positive {
                        stopped = Some("full positive anchor root/control missing or unresolved");
                    }
                }
                reference.push(sample.clone());
            } else {
                let r = &reference[index];
                let same_flags = ["timed_out", "incomplete", "inconsistent", "nonseparating"]
                    .iter()
                    .all(|k| sample["root_stats"][k] == r["root_stats"][k]);
                let same = sample["roots"] == r["roots"]
                    && sample["functions"] == r["functions"]
                    && sample["old_graph"] == r["old_graph"]
                    && sample["expanded_graph"] == r["expanded_graph"]
                    && sample["resolved"] == true
                    && r["resolved"] == true
                    && sample["invalid_original_replay"] == false
                    && r["invalid_original_replay"] == false
                    && same_flags;
                sample["matches_full_reference"] = json!(same);
                sample["resolution_flags_match_full_reference"] = json!(same_flags);
                complete &= same;
                if index == 0 && !same {
                    stopped = Some("traced positive full-reference mismatch");
                }
            }
            complete &= sample["resolved"] == true && sample["invalid_original_replay"] == false;
            event(&mut log, &sample)?;
            samples.push(sample);
            if stopped.is_some() {
                complete = false;
                break;
            }
        }
        graphs.push(json!({"mode":mode.name(),"old":{"stats":policies[0].graph.stats(),"edges":policies[0].edges,"projections":policies[0].projections},"expanded":{"stats":policies[1].graph.stats(),"edges":policies[1].edges,"projections":policies[1].projections}}));
        if stopped.is_some() {
            break 'arms;
        }
    }
    cover::clear_traces();
    cover::set_trace_scope(0);
    let attempted = samples.len();
    write_new(
        &out.join("summary.json"),
        &json!({"schema":"graph-ic.anchored-policy-pilot/v1","fixture_sha256":FIXTURE_SHA,"parameters":parameters,"planned_calls":258,"attempted_calls":attempted,"not_run_calls":258-attempted,"stopped":stopped,"complete_reference_comparison":complete,"samples":samples,"setups":setups,"graphs":graphs,"shared_finder_cost_charged_in_full_to_either_policy":true,"original_anchor_collection_excluded":true,"independent_rank_available":false,"full_native_cost_available":false,"wall_comparison":false,"signed_verification":false}),
    )
}
fn run(path: &str, out: &str) -> Result<()> {
    let out = Path::new(out);
    ensure(!out.exists(), "exclusive pilot output")?;
    let bytes = fs::read(path)?;
    ensure(sha(&bytes) == FIXTURE_SHA, "frozen public anchor hash")?;
    let seed: Seed = serde_json::from_slice(&bytes)?;
    fs::create_dir(out)?;
    write_new(
        &out.join("started.json"),
        &json!({"schema":"graph-ic.anchored-policy-started/v1","fixture_sha256":FIXTURE_SHA,"planned_calls":258,"watchdog_seconds":360,"virtual_memory_bytes":8589934592u64}),
    )?;
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(360));
        eprintln!("policy pilot watchdog expired");
        std::process::exit(124);
    });
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build()?;
    pool.install(|| core(&seed, out).map_err(|e| e.to_string()))
        .map_err(Into::into)
}
pub fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    ensure(
        a.len() == 4 && a[1] == "run",
        "run PUBLIC_ANCHOR NEW_OUTPUT; requires frozen protocol",
    )?;
    run(&a[2], &a[3])
}
#[cfg(test)]
mod controls {
    use super::*;
    fn shape() -> Vec<f4_fp::Poly> {
        let mut out = vec![];
        for i in 0..6 {
            let mut a = vec![0; 7];
            a[i] = 1;
            let mut b = vec![0; 7];
            b[6] = 1;
            out.push(vec![(a, 1), (b, 100)]);
        }
        let mut a = vec![0; 7];
        a[6] = 2;
        out.push(vec![(a, 1), (vec![0; 7], 100)]);
        out
    }
    #[test]
    fn original_seven_variable_roots_and_trace_replay_match() {
        cover::clear_traces();
        cover::set_trace_scope(0x904f_ffff);
        for mode in [Mode::Full, Mode::Traced, Mode::Traced] {
            let (roots, st) = solve(&shape(), 101, mode, &options(mode), 0);
            assert_eq!(roots, vec![vec![1; 7], vec![100; 7]]);
            assert!(!st.timed_out && !st.incomplete && st.nonseparating == 0);
        }
        cover::clear_traces();
        cover::set_trace_scope(0);
    }
    #[test]
    fn expired_solver_does_not_start_fresh_work() {
        let mut opts = options(Mode::Traced);
        opts.deadline = Some(Instant::now() - Duration::from_secs(1));
        let (roots, st) = solve(&shape(), 101, Mode::Traced, &opts, 0);
        assert!(roots.is_empty() && st.timed_out);
        assert_eq!(st.f4_muls, 0);
    }
    #[test]
    fn retained_anchor_and_canonical_factor_base_admit_without_f4() {
        let seed: Seed = serde_json::from_str(include_str!("../results/fixture.json")).unwrap();
        let f = Fq::new(seed.p);
        let h = native(&seed.h);
        let family = family(&seed, &f).unwrap();
        let base = base(&f, &h);
        assert_eq!(base.len(), 509);
        let domain = graph::Domain::new(&f, &h, &base).unwrap();
        let proof = anchor(&seed, &f, &family, &domain, &base).unwrap();
        let mut policy = Policy::new().unwrap();
        let row = policy.admit(&domain, proof.clone(), &proof).unwrap();
        assert_eq!(row["trivial"], true);
        assert!(policy.edges.is_empty());
    }
}
