//! Public-input workers for original IC and adapted two-LP scalar recovery.
//! Fixture generation is separate; workers reject fields carrying an answer.
use clap::Parser;
use crypto_lib::cryptanalysis::{
    jv_bitops::{self as work, Phase},
    jv_cover::{self as cover, EllE, Fq3, PtE6, Spec, E2, E6},
    jv_sieve::{
        self as sieve,
        two_large_prime::{run_two_large_prime_dlp_on, TwoLargePrimeDlpOptions},
    },
    residual_walk::is_prime_u64,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    time::Instant,
};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    generate: bool,
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
    #[arg(long, default_value_t = 8192)]
    max_descent_residuals: u64,
    #[arg(long, default_value_t = 32)]
    max_accepted_descents: usize,
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
    curve.validate()?;
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
            "two-lp"=>run_two_large_prime_dlp_on(&spec,args.seed,TwoLargePrimeDlpOptions{small_percent:args.small_percent,b_count:args.b_count,collection_rank_stride:args.collection_rank_stride,max_descent_residuals:args.max_descent_residuals,max_accepted_descents:args.max_accepted_descents})?,
            _=>return Err("method must be ic or two-lp".into()),
        };
        drop(model);
        let verified={let _v=work::enter(Phase::Verification);let f=Fq3::new(curve.p);let ec=EllE::new(&f,&alpha);scalar.is_some_and(|d|d>0&&d<curve.l&&ec.mul(&g,d)==q)};
        let elapsed=started.elapsed().as_nanos();
        let tally=work::snapshot().since(&before);let widths=work::widths(curve.p,curve.l);
        let phase_rows:Vec<_>=Phase::IC.iter().map(|&phase|{let t=tally.phase(phase);json!({"phase":phase.name(),"primitives":work::prims_json(t),"diagnostic_priced_bitops":work::price(t,&widths).expect("known widths").to_string()})}).collect();
        let other=work::snapshot_other();let other_delta:Vec<_>=other.iter().zip(other_before).map(|(&a,b)|a.checked_sub(b)).collect();
        let uniforms=work::uniform_snapshot().since(&uniforms_before).ok_or("uniform counter regression")?;
        let native=json!({"schema":"cover.two-large-prime.dlp-work/v1","complete":false,"total_bitops":null,"comparative_eligibility":false,
            "phases":phase_rows,"observed_primitives":work::prims_json(&tally.sum(&Phase::IC)),"uniform_sampling":work::uniform_json(&uniforms.sum(&Phase::IC).ok_or("uniform overflow")?),
            "foreign_hook_calls":work::foreign(),"other_modulus_ops":other_delta,
            "known_uncounted":["native containers/normalization/scheduling","PRNG internals and other sampling paths","incremental RREF scans/fixed-factor internals","cohort hashing/serialization"],
            "excluded_harness_primitives":work::prims_json(tally.phase(Phase::Harness)),
            "excluded_harness_uniform_events":work::uniform_json(&uniforms.phases[Phase::Harness as usize]),
            "excluded":"public instance generation/domain checks and legacy report unit calibration; calibration is excluded from IC primitive subtotal but included in process wall; no reusable attack precomputation supplied"});
        work::enable(false);
        Ok(json!({"schema":"cover.two-large-prime.dlp-worker/v1","source_revision":args.source_revision,"method":method,"seed":args.seed,
            "public":curve,"solver_answer_placeholder":0,"actual_scalar":scalar,"verified":verified,"capped":scalar.is_none(),"inner_wall_ns":elapsed,"work":report,"work_native":native}))
    })
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
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
    serde_json::to_writer(file, &result)?;
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
    fn public_fixture_rejects_answer_fields_including_nested_fields() {
        let spec = cover::generate_spec(101, 17);
        let public = PublicCurve::from_spec(&spec);
        let clean = serde_json::to_value(&public).unwrap();
        assert!(clean.get("d").is_none() && clean.get("expected_scalar").is_none());
        let parsed: PublicCurve = serde_json::from_value(clean.clone()).unwrap();
        parsed.validate().unwrap();
        let mut bad = clean.clone();
        bad["d"] = json!(spec.d);
        assert!(serde_json::from_value::<PublicCurve>(bad).is_err());
        let mut nested = clean;
        nested["g"]["expected_scalar"] = json!(spec.d);
        assert!(serde_json::from_value::<PublicCurve>(nested).is_err());
    }
}
