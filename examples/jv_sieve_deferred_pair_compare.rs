//! Public-input workers for original IC and adapted two-LP scalar recovery.
//! Fixture generation is separate; workers reject fields carrying an answer.
pub use crypto_lib::cryptanalysis;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_deferred_pair_graph.rs"]
mod deferred_pair_graph;
// shared source file; this example does not call every entry point
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_extension_deferred_pair_dlp.rs"]
mod extension_deferred_pair_dlp;
#[path = "../src/cryptanalysis/jv_extension_quadratic_pair_dlp.rs"]
mod extension_quadratic_pair_dlp;
#[path = "../src/cryptanalysis/jv_extension_sparse_dlp.rs"]
#[allow(clippy::type_complexity)] // Unchanged graph key from the existing collector.
mod extension_sparse_dlp;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_homogeneous_quotient.rs"]
mod homogeneous_quotient;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_principal_relation.rs"]
#[allow(clippy::too_many_arguments)] // Existing equation verifier's explicit inputs.
mod principal_relation;
#[path = "../src/cryptanalysis/jv_quadratic_pair_graph.rs"]
mod quadratic_pair_graph;
#[path = "../src/cryptanalysis/jv_quadratic_pair_relation.rs"]
mod quadratic_pair_relation;
#[path = "../src/cryptanalysis/jv_quartic_discriminant.rs"]
mod quartic_discriminant;
#[path = "../src/cryptanalysis/jv_sparse_collection_rank.rs"]
mod sparse_collection_rank;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_sparse_kernel.rs"]
mod sparse_kernel;
use clap::Parser;
use crypto_lib::cryptanalysis::{
    jv_bitops::{self as work, Phase},
    jv_cover::{self as cover, EllE, Fq3, PtE6, Spec, E2, E6},
    jv_sieve::{
        self as sieve,
        two_large_prime::{
            run_extension_remainder_dlp_on, run_two_large_prime_dlp_on,
            run_two_large_prime_dlp_on_with_quotient_reuse,
            run_two_large_prime_dlp_on_with_structural_gate, ExtensionRemainderDlpOptions,
            TwoLargePrimeDlpOptions,
        },
    },
    residual_walk::is_prime_u64,
};
use extension_deferred_pair_dlp::{run_extension_deferred_pair_dlp_on, DeferredPairDlpOptions};
use extension_quadratic_pair_dlp::{
    run_extension_discriminant_pair_dlp_on, run_extension_quadratic_pair_dlp_on,
    DiscriminantPairDlpOptions, QuadraticPairDlpOptions,
};
use extension_sparse_dlp::{
    run_extension_sparse_dlp_on, run_extension_sparse_table_dlp_on, ExtensionSparseDlpOptions,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
    time::Instant,
};

#[allow(dead_code)]
#[path = "jv_sieve_graph_compare.rs"]
mod shared_comparison;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    generate: bool,
    #[arg(long)]
    root_table: bool,
    /// Explicit separately registered pair history cap; unused methods reject it.
    #[arg(long)]
    pair_split_cap: Option<usize>,
    /// Lifetime deferred-edge history bound, accepted only by deferred-pairs.
    #[arg(long)]
    deferred_edge_cap: Option<usize>,
    /// Mathematical replay of retained norms only; starts no DLP worker.
    #[arg(long)]
    replay_norms: Option<PathBuf>,
    #[arg(long)]
    p: Option<u64>,
    #[arg(long)]
    seed: u64,
    #[arg(long)]
    method: Option<String>,
    #[arg(long)]
    fixture: Option<PathBuf>,
    #[arg(long)]
    out: PathBuf,
    #[arg(long)]
    source_revision: String,
    #[arg(long, default_value_t = 75)]
    small_percent: usize,
    #[arg(long)]
    b_count: Option<u128>,
    #[arg(long)]
    collection_rank_stride: Option<usize>,
    #[arg(long)]
    structural_rank_gate: bool,
    #[arg(long)]
    reuse_homogeneous_basis: bool,
    #[arg(long, default_value_t = 8192)]
    max_descent_residuals: u64,
    #[arg(long, default_value_t = 32)]
    max_accepted_descents: usize,
    #[arg(long, default_value_t = 1)]
    rho_batch: usize,
    #[arg(long)]
    rho_plain: bool,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PublicPoint {
    x: [[u64; 2]; 3],
    y: [[u64; 2]; 3],
    inf: bool,
}
impl PublicPoint {
    fn new(p: PtE6) -> Self {
        Self {
            x: p.x.0.map(|e| e.0),
            y: p.y.0.map(|e| e.0),
            inf: p.inf,
        }
    }
    fn native(&self) -> PtE6 {
        PtE6 {
            x: E6(self.x.map(E2)),
            y: E6(self.y.map(E2)),
            inf: self.inf,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PublicCurve {
    p: u64,
    l: u64,
    alpha: [[u64; 2]; 3],
    g: PublicPoint,
    q: PublicPoint,
}
impl PublicCurve {
    fn from_spec(s: &Spec) -> Self {
        Self {
            p: s.p,
            l: s.l,
            alpha: s.alpha.0.map(|e| e.0),
            g: PublicPoint::new(s.g),
            q: PublicPoint::new(s.q),
        }
    }
    fn validate(&self) -> Result<(), String> {
        if !(101..=1511).contains(&self.p)
            || !is_prime_u64(self.p)
            || !(3..1u64 << 63).contains(&self.l)
            || !is_prime_u64(self.l)
            || self.g.inf
            || self.q.inf
            || self
                .alpha
                .iter()
                .flatten()
                .chain(self.g.x.iter().flatten())
                .chain(self.g.y.iter().flatten())
                .chain(self.q.x.iter().flatten())
                .chain(self.q.y.iter().flatten())
                .any(|&v| v >= self.p)
        {
            return Err("invalid public domain".into());
        }
        let f = Fq3::new(self.p);
        let ec = EllE::new(&f, &E6(self.alpha.map(E2)));
        let (g, q) = (self.g.native(), self.q.native());
        if !ec.on_curve(&g)
            || !ec.on_curve(&q)
            || !ec.mul(&g, self.l).inf
            || !ec.mul(&q, self.l).inf
        {
            return Err("public points or subgroup invalid".into());
        }
        Ok(())
    }
}

fn worker(curve: &PublicCurve, args: &Args, method: &str) -> Result<Value, String> {
    if args.root_table && method != "extension-sparse-remainders" {
        return Err("table roots require the quadratic sparse method".into());
    }
    if args.pair_split_cap.is_some()
        && !["quadratic-pairs", "discriminant-pairs", "deferred-pairs"].contains(&method)
    {
        return Err("pair split cap requires a quadratic-pair method".into());
    }
    if args.deferred_edge_cap.is_some() && method != "deferred-pairs" {
        return Err("deferred edge cap requires deferred-pairs".into());
    }
    if matches!(
        method,
        "extension-full-smooth"
            | "extension-remainders"
            | "extension-sparse-full"
            | "extension-sparse-remainders"
            | "quadratic-pairs"
            | "discriminant-pairs"
            | "deferred-pairs"
    ) && args.small_percent != 100
    {
        return Err(
            "extension-remainder methods require the full factor base (--small-percent 100)".into(),
        );
    }
    if args.reuse_homogeneous_basis
        && (method != "two-lp"
            || !args.structural_rank_gate
            || args.collection_rank_stride.is_none())
    {
        return Err("homogeneous reuse requires two-lp, structural gate and a rank stride".into());
    }
    if args.structural_rank_gate && method != "two-lp" {
        return Err("structural rank gate is only defined for two-lp".into());
    }
    curve.validate()?;
    if method == "rho" {
        use crypto_lib::cryptanalysis::jv_rho_strong::StrongOpts;
        let options = StrongOpts {
            negation: !args.rho_plain,
            batch: args.rho_batch,
            r: 256,
            dp_bits: 16.min((curve.l.ilog2() / 2).saturating_sub(4)),
        };
        let public = serde_json::to_value(curve).map_err(|e| e.to_string())?;
        let reference =
            shared_comparison::solve_configured_native_rho(&public, args.seed, options)?;
        return Ok(
            json!({"schema":"cover.two-large-prime.dlp-worker/v1","source_revision":args.source_revision,"method":"rho","seed":args.seed,"public":public,"solver_answer_placeholder":0,"actual_scalar":reference["recovered_scalar"],"recovered_scalar":reference["recovered_scalar"],"verified":reference["verified"],"capped":reference["capped"],"inner_wall_ns":reference["inner_wall_ns"],"work":reference["work"],"work_native":reference["work_native"],"signature":reference["signature"],"rho_options":options}),
        );
    }
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .map_err(|e| e.to_string())?;
    pool.install(||{
        work::claim();work::set_moduli(curve.p,curve.l);work::enable(true);
        let before=work::snapshot();let uniforms_before=work::uniform_snapshot();let other_before=work::snapshot_other();
        let started=Instant::now();
        let model=work::enter(Phase::Model);
        let(alpha,g,q)=(E6(curve.alpha.map(E2)),curve.g.native(),curve.q.native());
        // The worker has no planted answer. Transfer and its checks are cold work.
        let spec=cover::spec_from_curve(curve.p,alpha,curve.l,g,q,0).ok_or("public transfer failed")?;
        let (report,scalar)=match method{
            "ic"=>{let(r,d)=sieve::run_cover_sieve_dlp_on_with_filter(&spec,args.seed,0,1.0,Some(64),1.25,None,false,None);(serde_json::to_value(r).map_err(|e|e.to_string())?,d)},
            "two-lp"=>{
                let options=TwoLargePrimeDlpOptions{small_percent:args.small_percent,b_count:args.b_count,collection_rank_stride:args.collection_rank_stride,max_descent_residuals:args.max_descent_residuals,max_accepted_descents:args.max_accepted_descents};
                if args.reuse_homogeneous_basis {run_two_large_prime_dlp_on_with_quotient_reuse(&spec,args.seed,options)?}
                else if args.structural_rank_gate {run_two_large_prime_dlp_on_with_structural_gate(&spec,args.seed,options)?}
                else {run_two_large_prime_dlp_on(&spec,args.seed,options)?}
            },
            "extension-full-smooth"|"extension-remainders"=>{
                let options=ExtensionRemainderDlpOptions{quadratic_remainders:method=="extension-remainders",b_count:args.b_count,max_descent_residuals:args.max_descent_residuals,max_accepted_descents:args.max_accepted_descents};
                run_extension_remainder_dlp_on(&spec,args.seed,options)?
            },
            "quadratic-pairs"=>{
                let options=QuadraticPairDlpOptions{b_count:args.b_count,max_descent_residuals:args.max_descent_residuals,max_accepted_descents:args.max_accepted_descents,max_split_records:args.pair_split_cap.unwrap_or(262_144),..Default::default()};
                run_extension_quadratic_pair_dlp_on(&spec,args.seed,options)?
            }
            "discriminant-pairs"=>{
                let collection=QuadraticPairDlpOptions{b_count:args.b_count,max_descent_residuals:args.max_descent_residuals,max_accepted_descents:args.max_accepted_descents,max_split_records:args.pair_split_cap.unwrap_or(262_144),..Default::default()};
                run_extension_discriminant_pair_dlp_on(&spec,args.seed,DiscriminantPairDlpOptions{collection,..Default::default()})?
            }
            "deferred-pairs"=>{
                let mut options=DeferredPairDlpOptions::default();
                options.collection.collection.b_count=args.b_count;
                options.collection.collection.max_descent_residuals=args.max_descent_residuals;
                options.collection.collection.max_accepted_descents=args.max_accepted_descents;
                if let Some(cap)=args.pair_split_cap {options.collection.collection.max_split_records=cap;}
                if let Some(cap)=args.deferred_edge_cap {options.max_deferred_edges=cap;}
                run_extension_deferred_pair_dlp_on(&spec,args.seed,options)?
            }
            "extension-sparse-full"|"extension-sparse-remainders"=>{
                let options=ExtensionSparseDlpOptions{quadratic_remainders:method=="extension-sparse-remainders",b_count:args.b_count,max_descent_residuals:args.max_descent_residuals,max_accepted_descents:args.max_accepted_descents,..Default::default()};
                if args.root_table {run_extension_sparse_table_dlp_on(&spec,args.seed,options)?}
                else {run_extension_sparse_dlp_on(&spec,args.seed,options)?}
            },
            _=>return Err("method must be ic, two-lp, extension-full-smooth, extension-remainders or rho".into()),
        };
        drop(model);
        let verified={let _v=work::enter(Phase::Verification);let f=Fq3::new(curve.p);let ec=EllE::new(&f,&alpha);scalar.is_some_and(|d|d>0&&d<curve.l&&ec.mul(&g,d)==q)};
        let elapsed=started.elapsed().as_nanos();
        let tally=work::snapshot().since(&before);let widths=work::widths(curve.p,curve.l);
        let phase_rows:Vec<_>=Phase::IC.iter().map(|&phase|{let t=tally.phase(phase);json!({"phase":phase.name(),"primitives":work::prims_json(t),"diagnostic_priced_bitops":work::price(t,&widths).expect("known widths").to_string()})}).collect();
        let other=work::snapshot_other();let other_delta:Vec<_>=other.iter().zip(other_before).map(|(&a,b)|a.checked_sub(b)).collect();
        let uniforms=work::uniform_snapshot().since(&uniforms_before).ok_or("uniform counter regression")?;
        let mut native=json!({"schema":"cover.two-large-prime.dlp-work/v1","complete":false,"total_bitops":null,"comparative_eligibility":false,
            "phases":phase_rows,"observed_primitives":work::prims_json(&tally.sum(&Phase::IC)),"uniform_sampling":work::uniform_json(&uniforms.sum(&Phase::IC).ok_or("uniform overflow")?),
            "foreign_hook_calls":work::foreign(),"other_modulus_ops":other_delta,
            "known_uncounted":["native containers/normalization/scheduling","PRNG internals and other sampling paths","incremental RREF scans/fixed-factor internals","cohort hashing/serialization"],
            "excluded_harness_primitives":work::prims_json(tally.phase(Phase::Harness)),
            "excluded_harness_uniform_events":work::uniform_json(&uniforms.phases[Phase::Harness as usize]),
            "excluded":"public instance generation/domain checks and legacy report unit calibration; calibration is excluded from IC primitive subtotal but included in process wall; no reusable attack precomputation supplied"});
        if args.structural_rank_gate {
            native["known_uncounted"].as_array_mut().expect("gap array").push(json!("structural rank gate native bookkeeping beyond counted state/parent events"));
        }
        if args.reuse_homogeneous_basis {
            native["known_uncounted"].as_array_mut().expect("gap array").push(json!("homogeneous kernel/projection native control beyond recorded state events"));
        }
        work::enable(false);
        Ok(json!({"schema":"cover.two-large-prime.dlp-worker/v1","source_revision":args.source_revision,"method":method,"seed":args.seed,
            "public":curve,"solver_answer_placeholder":0,"actual_scalar":scalar,"verified":verified,"capped":scalar.is_none(),"inner_wall_ns":elapsed,"work":report,"work_native":native}))
    })
}
fn independent_eval(norm: &[u64], x: u64, p: u64) -> u64 {
    norm.iter()
        .rev()
        .fold(0u128, |v, &c| (v * x as u128 + c as u128) % p as u128) as u64
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if let Some(path) = &args.replay_norms {
        if args.generate || args.root_table || args.method.is_some() {
            return Err("norm replay is separate from scientific execution".into());
        }
        let raw: Value = serde_json::from_slice(&std::fs::read(path)?)?;
        let p = raw["public"]["p"].as_u64().ok_or("p")?;
        let sources = raw["work"]["collection"]["sources"]
            .as_array()
            .ok_or("sources")?;
        if p != 1009 || sources.len() != 11822 {
            return Err("recorded46 norm corpus".into());
        }
        let ring = sieve::FpRing::new(p);
        for source in sources {
            let norm: Vec<u64> = serde_json::from_value(source["certificate"]["norm"].clone())?;
            if norm.len() != 10 || norm.iter().any(|&x| x >= p) {
                return Err("norm domain".into());
            }
            let table = ring.roots_by_table(&norm);
            let direct: Vec<u64> = (0..p)
                .filter(|&x| independent_eval(&norm, x, p) == 0)
                .collect();
            if table != direct {
                return Err("retained norm root mismatch".into());
            }
        }
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args.out)?;
        serde_json::to_writer_pretty(
            &mut f,
            &json!({"schema":"cover.table-roots.recorded-replay/v1","source_file":path,"source_sha256":format!("{:x}",Sha256::digest(std::fs::read(path)?)),"p":p,"norms":sources.len(),"all_fp_abscissae_checked":true,"independent_u128_horner":true,"scientific_worker_invoked":false,"new_yield_or_timing_measurement":false}),
        )?;
        f.write_all(b"\n")?;
        return Ok(());
    }
    if args.source_revision.len() != 40
        || !args.source_revision.bytes().all(|c| c.is_ascii_hexdigit())
    {
        return Err("source SHA required".into());
    }
    // Pin the same incidence/shared baseline recipe and clear unrelated
    // experimental JV overrides before the worker creates any threads.
    let overrides: Vec<_> = std::env::vars_os()
        .map(|(k, _)| k)
        .filter(|k| k.to_str().is_some_and(|s| s.starts_with("JV_")))
        .collect();
    for key in overrides {
        std::env::remove_var(key);
    }
    std::env::set_var("JV_ADDS_INC", "1");
    std::env::set_var("JV_ADDS_SHARE", "1");
    std::env::set_var("JV_SIEVE_PROGRESS", "1");
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args.out)?;
    let result = if args.generate {
        if args.method.is_some() || args.fixture.is_some() {
            return Err("generator cannot be a worker".into());
        }
        let p = args.p.ok_or("generator needs --p")?;
        if !(101..=1511).contains(&p) || !is_prime_u64(p) {
            return Err("invalid fixture prime".into());
        }
        let spec = cover::generate_spec(p, args.seed);
        let public = PublicCurve::from_spec(&spec);
        json!({"schema":"cover.two-large-prime.dlp-fixture/v1","source_revision":args.source_revision,"seed":args.seed,"public":public,"expected_scalar":spec.d,"workers_receive":"public only"})
    } else {
        let fixture = args
            .fixture
            .as_ref()
            .ok_or("worker needs public --fixture")?;
        let curve: PublicCurve = serde_json::from_reader(File::open(fixture)?)?;
        worker(
            &curve,
            &args,
            args.method.as_deref().ok_or("worker needs --method")?,
        )?
    };
    // Buffer capture output without changing the measured primitive interval.
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &result)?;
    writer.flush()?;
    println!(
        "{}",
        serde_json::to_string(
            &json!({"schema":result["schema"],"method":result["method"],"actual_scalar":result["actual_scalar"],"verified":result["verified"],"out":args.out})
        )?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn existing_difference_roots_match_independent_evaluation_exhaustively() {
        for p in [5u64, 7] {
            let ring = sieve::FpRing::new(p);
            for code in 1..p.pow(4) {
                let mut x = code;
                let mut norm: Vec<_> = (0..4)
                    .map(|_| {
                        let c = x % p;
                        x /= p;
                        c
                    })
                    .collect();
                while norm.last() == Some(&0) {
                    norm.pop();
                }
                let expected: Vec<_> = (0..p)
                    .filter(|&x| independent_eval(&norm, x, p) == 0)
                    .collect();
                assert_eq!(ring.roots_by_table(&norm), expected);
            }
        }
    }
    #[test]
    fn degree_nine_repeated_zero_and_nonbase_roots_keep_original_base_order() {
        for p in [11u64, 101, 1009] {
            let ring = sieve::FpRing::new(p);
            for roots in [
                [0, 1, 1, 2, 3, 4, 5, 6, 7],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 2, 3, 4, 5, 6, 7, 8, 9],
            ] {
                let mut norm = vec![1u64];
                for r in roots {
                    let mut out = vec![0; norm.len() + 1];
                    for (i, &c) in norm.iter().enumerate() {
                        out[i] =
                            ((out[i] as u128 + c as u128 * (p - r) as u128) % p as u128) as u64;
                        out[i + 1] = (out[i + 1] + c) % p;
                    }
                    norm = out;
                }
                let table = ring.roots_by_table(&norm);
                let direct: Vec<_> = (0..p)
                    .filter(|&x| independent_eval(&norm, x, p) == 0)
                    .collect();
                assert_eq!(table, direct);
                let base = [9, 0, 2, 5]; // Deliberately unsorted and excludes some roots.
                let original: Vec<_> = base
                    .iter()
                    .copied()
                    .filter(|&x| independent_eval(&norm, x, p) == 0)
                    .collect();
                let reused: Vec<_> = base.iter().copied().filter(|x| table.contains(x)).collect();
                assert_eq!(reused, original);
            }
        }
    }
    #[test]
    fn table_entrypoint_refuses_unrelated_full_smooth_mode() {
        let spec = cover::generate_spec(101, 19);
        assert!(run_extension_sparse_table_dlp_on(
            &spec,
            331,
            ExtensionSparseDlpOptions {
                quadratic_remainders: false,
                ..Default::default()
            }
        )
        .is_err());
    }
    #[test]
    fn combined_public_worker_recovers_with_each_rho_portfolio_configuration() {
        work::claim();
        let spec = cover::generate_spec(101, 19);
        let public = PublicCurve::from_spec(&spec);
        let mut args = Args {
            generate: false,
            root_table: false,
            pair_split_cap: None,
            deferred_edge_cap: None,
            replay_norms: None,
            p: None,
            seed: 331,
            method: Some("rho".into()),
            fixture: None,
            out: PathBuf::from("unused"),
            source_revision: "a".repeat(40),
            small_percent: 75,
            b_count: None,
            collection_rank_stride: None,
            structural_rank_gate: false,
            reuse_homogeneous_basis: false,
            max_descent_residuals: 8192,
            max_accepted_descents: 32,
            rho_batch: 1,
            rho_plain: false,
        };
        for (negation, batch) in [(false, 1), (true, 1), (false, 16), (true, 16)] {
            args.rho_batch = batch;
            args.rho_plain = !negation;
            let got = worker(&public, &args, "rho").unwrap();
            assert_eq!(got["actual_scalar"], spec.d);
            assert_eq!(got["verified"], true);
            assert_eq!(got["solver_answer_placeholder"], 0);
            assert_eq!(got["work"]["run"]["correct"], false);
            assert_eq!(got["work"]["run"]["batch"], batch);
            assert_eq!(got["work"]["run"]["negation"], negation);
            assert_eq!(got["rho_options"]["batch"], batch);
            assert_eq!(
                got["work_native"]["native_tower_calibration"]["inverse"]["field_mul"],
                80
            );
            assert_eq!(got["work_native"]["complete"], false);
        }
        args.rho_batch = 0;
        assert!(worker(&public, &args, "rho").is_err());
    }
    #[test]
    fn public_fixture_rejects_answer_fields_including_nested_fields() {
        work::claim();
        let spec = cover::generate_spec(101, 17);
        let public = PublicCurve::from_spec(&spec);
        let clean = serde_json::to_value(&public).unwrap();
        assert!(clean.get("d").is_none() && clean.get("expected_scalar").is_none());
        let parsed: PublicCurve = serde_json::from_value(clean.clone()).unwrap();
        parsed.validate().unwrap();
        let mut reuse_args = Args::try_parse_from([
            "test",
            "--seed",
            "17",
            "--out",
            "unwritten",
            "--source-revision",
            &"a".repeat(40),
            "--reuse-homogeneous-basis",
        ])
        .unwrap();
        assert!(worker(&parsed, &reuse_args, "ic").is_err());
        assert!(worker(&parsed, &reuse_args, "two-lp").is_err());
        reuse_args.structural_rank_gate = true;
        assert!(worker(&parsed, &reuse_args, "two-lp").is_err());
        reuse_args.collection_rank_stride = Some(32);
        assert!(worker(&parsed, &reuse_args, "rho").is_err());
        let mut bad = clean.clone();
        bad["d"] = json!(spec.d);
        assert!(serde_json::from_value::<PublicCurve>(bad).is_err());
        let mut nested = clean;
        nested["g"]["expected_scalar"] = json!(spec.d);
        assert!(serde_json::from_value::<PublicCurve>(nested).is_err());
    }
}
