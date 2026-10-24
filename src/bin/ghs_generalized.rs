//! Bounded exact generalized Hess parameter search on a supplied binary curve.
use clap::Parser;
use crypto_lib::cryptanalysis::{ghs_generalized::GeneralizedContext, ghs_screen::GhsCurveInput};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Parser)]
#[command(about = "Screen every proper subfield and a declared range of nonzero Hess parameters")]
struct Args {
    #[arg(long)]
    degree: u32,
    #[arg(long)]
    modulus: String,
    #[arg(long)]
    a: String,
    #[arg(long)]
    b: String,
    #[arg(long, default_value_t = 65535)]
    c_limit: u64,
    #[arg(long, default_value_t = 16)]
    genus_limit: u64,
    #[arg(long, default_value_t = 100)]
    max_candidates: usize,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    input: BTreeMap<&'static str, String>,
    c_range: [u64; 2],
    c_fraction_numerator: u64,
    c_fraction_denominator: String,
    base_rows: Vec<BaseRow>,
    candidates: Vec<crypto_lib::cryptanalysis::ghs_generalized::GeneralizedProfile>,
    candidates_total: u64,
    candidates_truncated: bool,
}

#[derive(Serialize)]
struct BaseRow {
    base_degree: u32,
    relative_degree: u32,
    profiles: u64,
    descendable: u64,
    genus_histogram: BTreeMap<String, u64>,
    under_bound: u64,
}

fn number(text: &str) -> Result<BigUint, String> {
    let cleaned = text.replace('_', "");
    let (digits, radix) = cleaned
        .strip_prefix("0x")
        .or_else(|| cleaned.strip_prefix("0X"))
        .map_or((cleaned.as_str(), 10), |v| (v, 16));
    BigUint::parse_bytes(digits.as_bytes(), radix).ok_or(format!("invalid integer: {text}"))
}

fn run() -> Result<(), String> {
    let args = Args::parse();
    let input = GhsCurveInput {
        absolute_degree: args.degree,
        modulus: number(&args.modulus)?,
        a: number(&args.a)?,
        b: number(&args.b)?,
    };
    let ctx = GeneralizedContext::new(&input)?;
    let possible = (BigUint::from(1u8) << args.degree as usize) - BigUint::from(1u8);
    let limit = args.c_limit.min(possible.to_u64().unwrap_or(u64::MAX));
    if limit == 0 {
        return Err("c-limit must cover at least c=1".into());
    }
    let mut candidates = Vec::new();
    let mut candidates_total = 0u64;
    let mut base_rows = Vec::new();
    for base_degree in 1..args.degree {
        if !args.degree.is_multiple_of(base_degree) {
            continue;
        }
        let mut row = BaseRow {
            base_degree,
            relative_degree: args.degree / base_degree,
            profiles: 0,
            descendable: 0,
            genus_histogram: BTreeMap::new(),
            under_bound: 0,
        };
        for c in 1..=limit {
            let p = ctx.profile(base_degree, &BigUint::from(c))?;
            row.profiles += 1;
            if p.descends {
                row.descendable += 1;
            }
            *row.genus_histogram.entry(p.genus.clone()).or_default() += 1;
            if p.descends
                && p.genus
                    .parse::<u64>()
                    .map(|g| g <= args.genus_limit)
                    .unwrap_or(false)
            {
                row.under_bound += 1;
                candidates_total += 1;
                if candidates.len() < args.max_candidates {
                    candidates.push(p);
                }
            }
        }
        base_rows.push(row);
    }
    let mut fields = BTreeMap::new();
    fields.insert("absolute_degree", args.degree.to_string());
    fields.insert("modulus", format!("0x{:x}", input.modulus));
    fields.insert("a", format!("0x{:x}", input.a));
    fields.insert("b", format!("0x{:x}", input.b));
    let report = Report {
        schema: "ghs-generalized-search/v1",
        input: fields,
        c_range: [1, limit],
        c_fraction_numerator: limit,
        c_fraction_denominator: possible.to_string(),
        base_rows,
        candidates,
        candidates_total,
        candidates_truncated: candidates_total > args.max_candidates as u64,
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("ghs_generalized: {e}");
        std::process::exit(2)
    }
}
