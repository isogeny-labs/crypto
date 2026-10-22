//! `ic prime`: the exact pair-sum yield of a small-x factor base on a
//! prime-field curve.
//!
//! Nothing else in `ic` selects a factor base over a prime field: `search`,
//! `run` and `workflow` are binary Koblitz pipelines.  This command measures
//! the one number a prime-field index calculus with two-point sums turns on,
//! the probability that a random group element is a signed sum of two base
//! points, exactly (from every pair sum) and by Monte Carlo where the yield is
//! large enough to see.  It runs on the exact Certicom ECCp-131 group,
//! NIST P-192 and small generated primes. ECCp-131 can also run a bounded
//! scalar-derived relation, rank, and published-target descent probe.
use clap::{Args, ValueEnum};
use crypto_lib::cryptanalysis::{
    binary_small_x,
    prime_small_x::{
        self, exact_ic_probe_with_arity, measure, toy_curve, toy_curve_with_cofactor, Curve,
        MontField, Options, PrimeField, Pt,
    },
    secg_small_x::{measure_secg, Extras},
    secg_standards::{secg, SecgCurve, SecgField},
    semaev_s3_cost::{s3_cost_binary, s3_cost_prime},
};
use crypto_lib::ecc::curve::CurveParams;
use serde::Serialize;
use serde_json::{json, Value};
use std::time::Instant;

use super::params;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrimeBase {
    /// The points with the smallest abscissae `x = 0, 1, 2, …` on the curve.
    SmallX,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct PrimeArgs {
    /// ECCp-131 or any SECG curve of the standards database that `ic list` names
    /// (`secp192r1` also as `p192`, `nist-p192`; `sect113r1`, `sect163r2`,
    /// `secp160k1`, … up to `secp521r1` and `sect571r1`), or a toy curve:
    /// `toy:<bits>[,<seed>[,<cofactor>]]` over a prime just below `2^bits`
    /// (8 to 28 bits; cofactor 2, 4 or 8, else a root-free curve of odd
    /// order), `toy-binary:<degree>[,<seed>[,<cofactor>]]` over
    /// `F_2^degree` (11, 13, 17 or 19; cofactor 2 or 4).
    #[arg(long, default_value = "secp192r1")]
    pub curve: String,
    /// Factor-base family; the small-x base is the only one so far.
    #[arg(long, value_enum, default_value_t = PrimeBase::SmallX)]
    pub factor_base: PrimeBase,
    /// Abscissae in the base (the base has twice as many points). The pair
    /// sums are about `size²` and are held in memory.
    #[arg(long, default_value_t = 2048, value_parser = clap::value_parser!(u32).range(2..=8192))]
    pub size: u32,
    /// Random group elements tested by Monte Carlo.
    #[arg(long, default_value_t = 1_000_000, value_parser = clap::value_parser!(u64).range(0..=1_000_000_000))]
    pub trials: u64,
    /// Planted decompositions that must be detected and recovered.
    #[arg(long, default_value_t = 32, value_parser = clap::value_parser!(u32).range(0..=4096))]
    pub controls: u32,
    #[arg(long, default_value_t = 1)]
    pub seed: u64,
    /// Sample this many signed pair sums of *raw* small-x points (no `[h]`)
    /// and report the share that lands in the prime-order subgroup, which is
    /// the factor by which that variant loses against the base used here.
    #[arg(long, default_value_t = 0, value_parser = clap::value_parser!(u64).range(0..=1_000_000))]
    pub raw_samples: u64,
    /// Decide this many random targets by a Semaev S₃ solve over the base
    /// (one quadratic per base abscissa), check each against the pair-sum
    /// table and time it: the cost of a trial without the table, and the time
    /// to collect `B + 1` relations against Pollard rho on the same machine.
    #[arg(long, default_value_t = 0, value_parser = clap::value_parser!(u64).range(0..=100_000))]
    pub s3_trials: u64,
    /// Exact ECCp-131 setup candidates `[a]G` tested against the pair-sum base.
    #[arg(long, default_value_t = 0, value_parser = clap::value_parser!(u64).range(0..=1_000_000))]
    pub setup_candidates: u64,
    /// Exact ECCp-131 descent candidates `Q+[a]G`, starting with the published Q.
    #[arg(long, default_value_t = 0, value_parser = clap::value_parser!(u64).range(0..=1_000_000))]
    pub target_candidates: u64,
    /// Signed base points per exact ECCp-131 relation: 2 or 3. The three-point
    /// path probes the pair table after subtracting one base point.
    #[arg(long, default_value_t = 2, value_parser = clap::value_parser!(u8).range(2..=3))]
    pub relation_arity: u8,
}

pub fn run(args: PrimeArgs) -> Result<Value, String> {
    let started = Instant::now();
    let opts = Options {
        size: args.size as usize,
        trials: args.trials,
        controls: args.controls as usize,
        seed: args.seed,
    };
    let name = args.curve.to_ascii_lowercase();
    let toy_args = |spec: &str, what: &str| -> Result<(u32, u64, Option<u64>), String> {
        let mut parts = spec.split(',');
        let n: u32 = parts.next().and_then(|s| s.parse().ok()).ok_or_else(|| {
            format!("{what} curves are written {what}:<size>[,<seed>[,<cofactor>]]")
        })?;
        let seed: u64 = parts.next().map_or(Ok(args.seed), |s| {
            s.parse().map_err(|_| "toy curve seed must be an integer")
        })?;
        let h: Option<u64> = parts
            .next()
            .map(|s| s.parse().map_err(|_| "toy cofactor must be an integer"))
            .transpose()?;
        Ok((n, seed, h))
    };
    let raw = args.raw_samples;
    let s3n = args.s3_trials;
    let size = args.size as usize;
    let mut ic_probe = None;
    if (args.setup_candidates > 0 || args.target_candidates > 0)
        && !matches!(name.as_str(), "eccp-131" | "eccp131")
    {
        return Err("--setup-candidates and --target-candidates currently require eccp-131".into());
    }
    if args.relation_arity == 3 && args.setup_candidates == 0 && args.target_candidates == 0 {
        return Err("--relation-arity 3 requires setup or target candidates".into());
    }
    let report = if matches!(name.as_str(), "eccp-131" | "eccp131") {
        let params = CurveParams::eccp131();
        let record = SecgCurve {
            name: params.name.into(),
            oid: String::new(),
            field: SecgField::Prime {
                p: params.p.clone(),
            },
            a: params.a,
            b: params.b,
            gx: params.gx,
            gy: params.gy,
            order: params.n,
            cofactor: params.h as u64,
        };
        let c = Curve::<MontField<3>>::from_secg(&record)?;
        let r = measure(&c, &opts)?;
        if args.setup_candidates > 0 || args.target_candidates > 0 {
            let g = c.secg_generator(&record)?;
            let published = params::named("eccp-131")?
                .point
                .ok_or("ECCp-131 profile has no published point")?;
            let q = Pt {
                x: c.field.embed_big(&params::number(&published.x)?)?,
                y: c.field.embed_big(&params::number(&published.y)?)?,
            };
            ic_probe = Some(exact_ic_probe_with_arity(
                &c,
                &g,
                &q,
                size,
                args.setup_candidates,
                args.target_candidates,
                args.seed,
                args.relation_arity,
            )?);
        }
        r
    } else if matches!(name.as_str(), "secp192r1" | "p192" | "nist-p192" | "p-192") {
        let c = Curve::p192();
        let mut r = measure(&c, &opts)?;
        if raw > 0 {
            r.raw_base = Some(prime_small_x::raw_subgroup_fraction(
                &c, size, raw, args.seed,
            )?);
        }
        if s3n > 0 {
            r.s3 = Some(s3_cost_prime(&c, &opts, &r, s3n)?);
        }
        r
    } else if let Some(spec) = name.strip_prefix("toy-binary:") {
        let (degree, seed, h) = toy_args(spec, "toy-binary")?;
        let c = binary_small_x::toy_curve_with_cofactor::<1>(degree, seed, h.unwrap_or(2))?;
        let mut r = binary_small_x::measure(&c, &opts)?;
        if raw > 0 {
            r.raw_base = Some(binary_small_x::raw_subgroup_fraction(
                &c, size, raw, args.seed,
            )?);
        }
        if s3n > 0 {
            r.s3 = Some(s3_cost_binary(&c, &opts, &r, s3n)?);
        }
        r
    } else if let Some(spec) = name.strip_prefix("toy:") {
        let (bits, seed, h) = toy_args(spec, "toy")?;
        let c = match h {
            Some(h) => toy_curve_with_cofactor(bits, seed, h)?,
            None => toy_curve(bits, seed)?,
        };
        let mut r = measure(&c, &opts)?;
        if raw > 0 {
            r.raw_base = Some(prime_small_x::raw_subgroup_fraction(
                &c, size, raw, args.seed,
            )?);
        }
        if s3n > 0 {
            r.s3 = Some(s3_cost_prime(&c, &opts, &r, s3n)?);
        }
        r
    } else if let Some(c) = secg(&name) {
        measure_secg(
            c,
            &opts,
            Extras {
                raw_samples: raw,
                s3_trials: s3n,
            },
        )?
    } else {
        return Err(format!(
            "unknown curve {:?}; use a SECG name from `ic list`, secp192r1, toy:<bits> or toy-binary:<degree>",
            args.curve
        ));
    };
    Ok(json!({
        "schema_version": 1,
        "operation": "prime",
        "status": "complete",
        "evidence_scope": if ic_probe.is_some() {
            "exact_pair_sum_yield_and_bounded_relation_rank_descent"
        } else { "exact_pair_sum_yield_with_monte_carlo" },
        "arguments": args,
        "report": report,
        "ic_probe": ic_probe,
        "elapsed_seconds": started.elapsed().as_secs_f64(),
        "limitations": [
            "The yield report measures two-point sums of the small-x base. --relation-arity 3 tests signed three-point relations separately; the two-point yield is not its success probability.",
            "The yield is exact for the base built. The Monte Carlo rate agrees with it only where the yield is large enough to produce hits; at P-192 it is a null result and bounds the rate from above.",
            "The cost of deciding a trial by a Semaev S₃ solve is measured only with --s3-trials. On ECCp-131, --setup-candidates and --target-candidates separately run and account for a bounded relation, rank, and descent probe; a yield alone is not a discrete logarithm.",
            "The base is capped at 8192 abscissae, and lower where the pair-sum keys would not fit in memory (wide fields).",
            "With a cofactor h > 1 (every SECG binary curve, secp112r2, secp128r2) each small-x point enters the base as [h]P and targets are [h] times a uniform point, so base and targets lie in the subgroup. --raw-samples measures, by sampling, the share of raw small-x pair sums that lie in the subgroup.",
            "The bounded ECCp-131 probe records natural scalar-derived relations and modular rank. Zero relations or an undetermined target logarithm apply only to its enumerated base, seed, and candidate budget."
        ]
    }))
}
