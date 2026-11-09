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
    curve_traits::arith::is_prime,
    efd_representations::{chart_ids, check_all_charts, EfdRepresentation},
    prime_small_x::{
        self, exact_ic_probe_with_arity, measure, toy_curve, toy_curve_with_cofactor, Curve,
        MontField, Options, PrimeField, Pt,
    },
    secg_small_x::{measure_secg, Extras},
    secg_standards::{secg, SecgCurve, SecgField},
    semaev_s3_cost::{s3_cost_binary, s3_cost_prime},
};
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::field::FieldElement;
use crypto_lib::prime_ecc::efd::{catalogue, Bindings};
use num_bigint::BigUint;
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

    /// EFD model id: shortw, montgom, edwards, twisted, twistedhessian,
    /// hessian, jintersect, jquartic, 2dik or 3dik. When set, the curve is
    /// read in that model and converted to the short-Weierstrass relation
    /// engine; `--curve` is ignored.
    #[arg(long)]
    pub efd_model: Option<String>,
    /// EFD chart id (e.g. `xz`, `jacobian`, `extended`) or `affine`.
    #[arg(long, default_value = "affine")]
    pub efd_chart: String,
    /// EFD model parameter, `NAME=VALUE`; repeatable.
    #[arg(long = "efd-param", value_name = "NAME=VALUE")]
    pub efd_params: Vec<String>,
    /// Prime field modulus of the EFD curve.
    #[arg(long)]
    pub efd_p: Option<String>,
    /// Prime subgroup order of the EFD curve.
    #[arg(long)]
    pub efd_order: Option<String>,
    /// Cofactor of the EFD curve.
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u64).range(1..))]
    pub efd_cofactor: u64,
    /// Generator in `--efd-chart` coordinates, comma-separated, for the exact
    /// relation probe.
    #[arg(long)]
    pub efd_generator: Option<String>,
    /// Target in `--efd-chart` coordinates, comma-separated.
    #[arg(long)]
    pub efd_target: Option<String>,
    /// Lift index for quotient charts (x-only, y-only, y^2-only).
    #[arg(long, default_value_t = 0)]
    pub efd_lift: u32,
    /// Lift index for the generator only; defaults to --efd-lift.
    #[arg(long)]
    pub efd_generator_lift: Option<u32>,
    /// Lift index for the target only; defaults to --efd-lift.
    #[arg(long)]
    pub efd_target_lift: Option<u32>,
    /// Print the EFD models, charts and formula counts as JSON, then exit.
    #[arg(long)]
    pub efd_list: bool,
    /// Round-trip-check the selected EFD chart (or every chart with
    /// `--efd-chart all`) and exit.
    #[arg(long)]
    pub efd_check: bool,
}

fn parse_big(value: &str) -> Result<BigUint, String> {
    let value = value.trim();
    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        BigUint::parse_bytes(hex.as_bytes(), 16).ok_or_else(|| format!("bad integer {value}"))
    } else {
        value
            .parse::<BigUint>()
            .map_err(|_| format!("bad integer {value}"))
    }
}

fn parse_efd_bindings(items: &[String], p: &BigUint) -> Result<Bindings, String> {
    let mut out = Bindings::new();
    for item in items {
        let (name, value) = item
            .split_once('=')
            .ok_or_else(|| format!("--efd-param expects NAME=VALUE, got {item}"))?;
        out.insert(
            name.trim().to_owned(),
            FieldElement::new(parse_big(value)?, p.clone()),
        );
    }
    Ok(out)
}

fn parse_chart_coords(value: &str, p: &BigUint) -> Result<Vec<FieldElement>, String> {
    value
        .split(',')
        .map(|part| parse_big(part).map(|v| FieldElement::new(v, p.clone())))
        .collect()
}

fn efd_catalogue_value() -> Value {
    let models: Vec<Value> = catalogue()
        .models
        .iter()
        .map(|model| {
            let charts: Vec<Value> = chart_ids(&model.id)
                .into_iter()
                .map(|chart| {
                    let formulas = catalogue()
                        .formulas
                        .iter()
                        .filter(|formula| formula.representation == chart)
                        .count();
                    json!({"chart": chart, "formulas": formulas})
                })
                .collect();
            let equations: Vec<&str> = model
                .source
                .text
                .lines()
                .filter(|line| line.starts_with("satisfying "))
                .collect();
            json!({
                "model": model.id,
                "parameters": model.parameter_names(),
                "equations": equations,
                "charts": charts,
            })
        })
        .collect();
    json!({"models": models})
}

pub fn run(args: PrimeArgs) -> Result<Value, String> {
    let started = Instant::now();
    let opts = Options {
        size: args.size as usize,
        trials: args.trials,
        controls: args.controls as usize,
        seed: args.seed,
    };
    if args.efd_list {
        return Ok(json!({
            "schema_version": 1,
            "operation": "efd-list",
            "status": "complete",
            "efd": efd_catalogue_value(),
        }));
    }
    if args.efd_check {
        let model = args
            .efd_model
            .as_deref()
            .ok_or("--efd-check requires --efd-model")?;
        let p = parse_big(
            args.efd_p
                .as_deref()
                .ok_or("--efd-check requires --efd-p")?,
        )?;
        let params = parse_efd_bindings(&args.efd_params, &p)?;
        let check = if args.efd_chart == "all" {
            check_all_charts(model, &p, &params)?
        } else {
            EfdRepresentation::new(model, &args.efd_chart, &p, &params)?.self_check()?
        };
        let ok = check["ok"].as_bool().unwrap_or(false);
        return Ok(json!({
            "schema_version": 1,
            "operation": "efd-check",
            "status": if ok { "complete" } else { "checks_failed" },
            "check": check,
        }));
    }
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
    let mut representation = None;
    let efd_model = args.efd_model.clone();
    if (args.setup_candidates > 0 || args.target_candidates > 0)
        && efd_model.is_none()
        && !matches!(name.as_str(), "eccp-131" | "eccp131")
    {
        return Err(
            "--setup-candidates and --target-candidates require eccp-131 or --efd-model".into(),
        );
    }
    if args.relation_arity == 3 && args.setup_candidates == 0 && args.target_candidates == 0 {
        return Err("--relation-arity 3 requires setup or target candidates".into());
    }
    let report = if let Some(model_id) = efd_model.as_deref() {
        let p = parse_big(
            args.efd_p
                .as_deref()
                .ok_or("--efd-model requires --efd-p")?,
        )?;
        if !is_prime(&p) {
            return Err("--efd-p is not prime (Miller-Rabin, bases 2..53)".into());
        }
        let order = parse_big(
            args.efd_order
                .as_deref()
                .ok_or("--efd-model requires --efd-order")?,
        )?;
        if order <= BigUint::from(2u8) {
            return Err("--efd-order must exceed 2".into());
        }
        if !is_prime(&order) {
            return Err("--efd-order is not prime (Miller-Rabin, bases 2..53)".into());
        }
        let params = parse_efd_bindings(&args.efd_params, &p)?;
        let repr = EfdRepresentation::new(model_id, &args.efd_chart, &p, &params)?;
        let chart_label = repr
            .chart_id
            .split_once('/')
            .map(|(_, chart)| chart)
            .unwrap_or(repr.chart_id.as_str());
        let curve_name = format!("EFD {model_id}/{chart_label} over F_{p}");

        macro_rules! efd_prime {
            ($limbs:literal) => {{
                let field = MontField::<$limbs>::new(&p)?;
                let curve = repr.curve(
                    field,
                    curve_name.clone(),
                    order.clone(),
                    true,
                    args.efd_cofactor,
                )?;
                let mut r = measure(&curve, &opts)?;
                if raw > 0 {
                    r.raw_base = Some(prime_small_x::raw_subgroup_fraction(
                        &curve, size, raw, args.seed,
                    )?);
                }
                if s3n > 0 {
                    r.s3 = Some(s3_cost_prime(&curve, &opts, &r, s3n)?);
                }
                if let (Some(g), Some(q)) = (&args.efd_generator, &args.efd_target) {
                    let generator_lift = args.efd_generator_lift.unwrap_or(args.efd_lift);
                    let target_lift = args.efd_target_lift.unwrap_or(args.efd_lift);
                    let generator = repr.decode_point(
                        &curve.field,
                        &parse_chart_coords(g, &p)?,
                        generator_lift,
                    )?;
                    let target =
                        repr.decode_point(&curve.field, &parse_chart_coords(q, &p)?, target_lift)?;
                    let mut f = curve.field.clone();
                    if !curve.on_curve(&mut f, &generator) {
                        return Err("--efd-generator is not on the curve".into());
                    }
                    if !curve.on_curve(&mut f, &target) {
                        return Err("--efd-target is not on the curve".into());
                    }
                    if curve.mul_big(&mut f, &generator, &order).is_some() {
                        return Err(format!(
                            "--efd-generator is not in the order-{order} subgroup"
                        ));
                    }
                    if curve.mul_big(&mut f, &target, &order).is_some() {
                        return Err(format!("--efd-target is not in the order-{order} subgroup"));
                    }
                    let setup = args.setup_candidates.max(1);
                    let target_candidates = args.target_candidates.max(1);
                    ic_probe = Some(exact_ic_probe_with_arity(
                        &curve,
                        &generator,
                        &target,
                        size,
                        setup,
                        target_candidates,
                        args.seed,
                        args.relation_arity,
                    )?);
                }
                representation = Some(repr.describe());
                r
            }};
        }
        match p.bits() {
            0..=192 => efd_prime!(3),
            193..=256 => efd_prime!(4),
            257..=384 => efd_prime!(6),
            385..=576 => efd_prime!(9),
            bits => {
                return Err(format!(
                    "EFD modulus {bits} bits is beyond the 576-bit prime engine"
                ))
            }
        }
    } else if matches!(name.as_str(), "eccp-131" | "eccp131") {
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
        "representation": representation,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_efd_integers_and_coordinates() {
        assert_eq!(parse_big("0x10").unwrap(), BigUint::from(16u32));
        assert_eq!(parse_big("1019").unwrap(), BigUint::from(1019u32));
        assert!(parse_big("not-a-number").is_err());

        let p = BigUint::from(1019u32);
        let coords = parse_chart_coords("604, 1", &p).unwrap();
        assert_eq!(coords.len(), 2);
        assert_eq!(coords[0].value, BigUint::from(604u32));
        assert_eq!(coords[1].value, BigUint::from(1u32));
        assert!(parse_chart_coords("604,", &p).is_err());

        let bindings = parse_efd_bindings(&["a=131".into(), "b=1".into()], &p).unwrap();
        assert_eq!(bindings["a"].value, BigUint::from(131u32));
        assert!(parse_efd_bindings(&["a".into()], &p).is_err());
    }

    #[test]
    fn efd_catalogue_lists_every_model_and_chart() {
        let value = efd_catalogue_value();
        let models = value["models"].as_array().unwrap();
        assert_eq!(models.len(), 10);
        let non_affine: usize = models
            .iter()
            .map(|model| {
                model["charts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|chart| chart["chart"] != "affine")
                    .count()
            })
            .sum();
        assert_eq!(non_affine, 33);
    }
}
