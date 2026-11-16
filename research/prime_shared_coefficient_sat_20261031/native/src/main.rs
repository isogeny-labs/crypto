use prime_shared_coeff_ir::{
    boolean::{BooleanEncoding, EncodingStatistics},
    build_ir, e0_f17_panel,
    e1::run_e1_sat_panel,
    e1_msolve::run_e1_msolve_panel,
    InstanceSpec,
};
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

fn parse_u64(value: &str, name: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|_| format!("invalid {name}: expected an unsigned decimal integer"))
}

fn parse_spec(args: &[String]) -> Result<InstanceSpec, String> {
    if args.len() != 8 {
        return Err("expected P A B M T C TARGET_X TARGET_Y".to_owned());
    }
    Ok(InstanceSpec {
        modulus: parse_u64(&args[0], "P")?,
        curve_a: parse_u64(&args[1], "A")?,
        curve_b: parse_u64(&args[2], "B")?,
        arity: parse_u64(&args[3], "M")?
            .try_into()
            .map_err(|_| "M does not fit usize".to_owned())?,
        depth: parse_u64(&args[4], "T")?
            .try_into()
            .map_err(|_| "T does not fit usize".to_owned())?,
        terminal_c: parse_u64(&args[5], "C")?,
        target_x: parse_u64(&args[6], "TARGET_X")?,
        target_y: parse_u64(&args[7], "TARGET_Y")?,
    })
}

fn emit(args: &[String]) -> Result<(), String> {
    if args.len() != 9 {
        return Err(
            "usage: prime-shared-coeff-ir emit P A B M T C TARGET_X TARGET_Y json|msolve|qf-ff|cnf|cnf-xor"
                .to_owned(),
        );
    }
    let ir = build_ir(parse_spec(&args[..8])?)?;
    match args[8].as_str() {
        "json" => println!("{}", ir.canonical_json_pretty()?),
        "msolve" => print!("{}", ir.to_msolve()?),
        "qf-ff" => print!("{}", ir.to_qf_ff()?),
        "cnf" => print!("{}", BooleanEncoding::from_ir(&ir)?.to_cnf()),
        "cnf-xor" => print!("{}", BooleanEncoding::from_ir(&ir)?.to_cnf_xor()),
        _ => return Err("format must be json, msolve, qf-ff, cnf, or cnf-xor".to_owned()),
    }
    Ok(())
}

fn statistics(args: &[String]) -> Result<(), String> {
    let spec = parse_spec(args)?;
    let ir = build_ir(spec)?;
    let encoding = BooleanEncoding::from_ir(&ir)?;
    let polynomial_terms: usize = ir
        .equations
        .iter()
        .map(|equation| equation.terms.len())
        .sum();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "schema": "prime-shared-coefficient.encoding-statistics/v1",
            "spec": {
                "modulus": spec.modulus,
                "curve_a": spec.curve_a,
                "curve_b": spec.curve_b,
                "arity": spec.arity,
                "depth": spec.depth,
                "terminal_c": spec.terminal_c,
                "target_x": spec.target_x,
                "target_y": spec.target_y
            },
            "ir_sha256": ir.sha256()?,
            "ir_variables": ir.variables.len(),
            "ir_projected_variables": ir.projected_variables.len(),
            "ir_equations": ir.equations.len(),
            "ir_polynomial_terms": polynomial_terms,
            "boolean": encoding.statistics(),
            "claim_scope": "exact deterministic representation counts; no solver-work or timing claim"
        }))
        .map_err(|error| error.to_string())?
    );
    Ok(())
}

#[derive(Clone, Debug, Serialize)]
struct Rank5Cell {
    bits: usize,
    modulus_family: &'static str,
    curve_class: &'static str,
    modulus: u64,
    curve_a: u64,
    curve_b: u64,
    target: [u64; 2],
    ir_sha256: String,
    ir_variables: usize,
    ir_projected_variables: usize,
    ir_equations: usize,
    ir_polynomial_terms: usize,
    boolean: EncodingStatistics,
    repeated_construction_equal: bool,
}

fn rank5_cell(
    bits: usize,
    modulus_family: &'static str,
    curve_class: &'static str,
    modulus: u64,
    curve_a: u64,
    curve_b: u64,
    target: [u64; 2],
) -> Result<Rank5Cell, String> {
    let spec = InstanceSpec {
        modulus,
        curve_a,
        curve_b,
        arity: 3,
        depth: 3,
        terminal_c: 1,
        target_x: target[0],
        target_y: target[1],
    };
    let first_ir = build_ir(spec)?;
    let first_hash = first_ir.sha256()?;
    let first_statistics = BooleanEncoding::from_ir(&first_ir)?.statistics();
    let second_ir = build_ir(spec)?;
    let second_hash = second_ir.sha256()?;
    let second_statistics = BooleanEncoding::from_ir(&second_ir)?.statistics();
    let repeated_construction_equal =
        first_hash == second_hash && first_statistics == second_statistics;
    if !repeated_construction_equal {
        return Err(format!(
            "rank-5 repeated construction mismatch for {bits}/{modulus_family}/{curve_class}"
        ));
    }
    let ir_polynomial_terms = first_ir
        .equations
        .iter()
        .map(|equation| equation.terms.len())
        .sum();
    Ok(Rank5Cell {
        bits,
        modulus_family,
        curve_class,
        modulus,
        curve_a,
        curve_b,
        target,
        ir_sha256: first_hash,
        ir_variables: first_ir.variables.len(),
        ir_projected_variables: first_ir.projected_variables.len(),
        ir_equations: first_ir.equations.len(),
        ir_polynomial_terms,
        boolean: first_statistics,
        repeated_construction_equal,
    })
}

fn fitted_slope(points: &[(f64, f64)]) -> Result<f64, String> {
    if points.len() < 2 {
        return Err("rank-5 slope needs at least two points".to_owned());
    }
    let count = f64::from(
        u32::try_from(points.len())
            .map_err(|_| "rank-5 slope point count exceeds u32".to_owned())?,
    );
    let mean_x = points.iter().map(|point| point.0).sum::<f64>() / count;
    let mean_y = points.iter().map(|point| point.1).sum::<f64>() / count;
    let numerator = points
        .iter()
        .map(|point| (point.0 - mean_x) * (point.1 - mean_y))
        .sum::<f64>();
    let denominator = points
        .iter()
        .map(|point| (point.0 - mean_x).powi(2))
        .sum::<f64>();
    if denominator == 0.0 {
        return Err("rank-5 slope has zero size variance".to_owned());
    }
    Ok(numerator / denominator)
}

fn metric(cell: &Rank5Cell, name: &str) -> Result<u32, String> {
    match name {
        "boolean_variables" => Ok(cell.boolean.boolean_variables),
        "ordinary_cnf_clauses" => u32::try_from(cell.boolean.ordinary_cnf_clauses)
            .map_err(|_| "ordinary clause count exceeds u32".to_owned()),
        "cnf_xor_total_constraints" => u32::try_from(cell.boolean.cnf_xor_total_constraints)
            .map_err(|_| "CNF-XOR constraint count exceeds u32".to_owned()),
        _ => Err(format!("unknown rank-5 metric {name}")),
    }
}

fn rank5_comparisons(
    cells: &[Rank5Cell],
    metric_names: &[&str],
) -> Result<Vec<serde_json::Value>, String> {
    let mut comparisons = Vec::new();
    for special in ["j0", "j1728"] {
        for family in ["solinas", "generic"] {
            for &metric_name in metric_names {
                let slope = |curve_class: &str| -> Result<f64, String> {
                    let points = cells
                        .iter()
                        .filter(|cell| {
                            cell.modulus_family == family && cell.curve_class == curve_class
                        })
                        .map(|cell| {
                            let bits = u32::try_from(cell.bits)
                                .map_err(|_| "rank-5 field width exceeds u32".to_owned())?;
                            let value = metric(cell, metric_name)?;
                            Ok((f64::from(bits), f64::from(value).log2()))
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    fitted_slope(&points)
                };
                let special_slope = slope(special)?;
                let generic_slope = slope("generic")?;
                comparisons.push(serde_json::json!({
                    "special_curve": special,
                    "modulus_family": family,
                    "metric": metric_name,
                    "special_slope": special_slope,
                    "generic_curve_slope": generic_slope,
                    "difference": special_slope - generic_slope
                }));
            }
        }
    }
    Ok(comparisons)
}

fn rank5_panel(output: Option<&Path>) -> Result<(), String> {
    let moduli = [
        (20, "solinas", 1_032_193),
        (20, "generic", 991_961),
        (24, "solinas", 16_760_833),
        (24, "generic", 9_117_497),
        (26, "solinas", 67_104_769),
        (26, "generic", 51_442_009),
        (28, "solinas", 268_369_921),
        (28, "generic", 252_089_969),
    ];
    let curves = [
        ("j0", 0, 1, [0, 1]),
        ("j1728", 3, 0, [1, 2]),
        ("generic", 2, 1, [1, 2]),
    ];
    let mut cells = Vec::new();
    for (bits, family, modulus) in moduli {
        for (class, curve_a, curve_b, target) in curves {
            cells.push(rank5_cell(
                bits, family, class, modulus, curve_a, curve_b, target,
            )?);
        }
    }

    let metric_names = [
        "boolean_variables",
        "ordinary_cnf_clauses",
        "cnf_xor_total_constraints",
    ];
    let comparisons = rank5_comparisons(&cells, &metric_names)?;

    let difference = |special: &str, family: &str, metric_name: &str| -> Result<f64, String> {
        comparisons
            .iter()
            .find(|entry| {
                entry["special_curve"] == special
                    && entry["modulus_family"] == family
                    && entry["metric"] == metric_name
            })
            .and_then(|entry| entry["difference"].as_f64())
            .ok_or_else(|| "missing rank-5 comparison".to_owned())
    };
    let advances = ["j0", "j1728"].into_iter().any(|special| {
        ["solinas", "generic"].into_iter().all(|family| {
            metric_names.into_iter().all(|metric_name| {
                difference(special, family, metric_name).is_ok_and(|d| d <= -0.05)
            })
        })
    });
    let rejects = ["j0", "j1728"].into_iter().all(|special| {
        metric_names.into_iter().all(|metric_name| {
            let solinas = difference(special, "solinas", metric_name);
            let generic = difference(special, "generic", metric_name);
            match (solinas, generic) {
                (Ok(left), Ok(right)) => {
                    (left.abs() < 0.05 && right.abs() < 0.05) || left * right <= 0.0
                }
                _ => false,
            }
        })
    });
    let verdict = if advances {
        "advance_as_representation_lead"
    } else if rejects {
        "reject_at_representation_size_scope"
    } else {
        "inconclusive"
    };
    let rendered = serde_json::to_string_pretty(&serde_json::json!({
            "schema": "prime-ic.rank5-representation-panel/v1",
            "protocol": "research/prime_ic_elimination_20261101/RANK5_REPRESENTATION_PROTOCOL.md",
            "cell_count": cells.len(),
            "repeated_construction_matches": cells.iter().all(|cell| cell.repeated_construction_equal),
            "cells": cells,
            "slope_comparisons": comparisons,
            "effect_size_gate": 0.05,
            "verdict": verdict,
            "timing_collected": false,
            "claim_scope": "exact deterministic representation counts; no solver-work, subgroup, relation-yield, or asymptotic claim"
        }))
        .map_err(|error| error.to_string())?;
    if let Some(path) = output {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|error| format!("could not create {}: {error}", path.display()))?;
        file.write_all(rendered.as_bytes())
            .and_then(|()| file.write_all(b"\n"))
            .and_then(|()| file.sync_all())
            .map_err(|error| format!("could not write {}: {error}", path.display()))?;
        println!("wrote {}", path.display());
    } else {
        println!("{rendered}");
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("emit") => emit(&args[1..]),
        Some("stats") if args.len() == 9 => statistics(&args[1..]),
        Some("rank5-panel") if args.len() == 1 => rank5_panel(None),
        Some("rank5-panel") if args.len() == 2 => rank5_panel(Some(Path::new(&args[1]))),
        Some("e0-panel") if args.len() == 1 => {
            println!(
                "{}",
                serde_json::to_string_pretty(&e0_f17_panel()?).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        Some("e1-sat-panel") if args.len() == 7 => {
            let report = run_e1_sat_panel(
                Path::new(&args[1]),
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                Path::new(&args[5]),
                &args[6],
            )?;
            println!(
                "{}: {}/{} exact target/backend matches; {} checked UNSAT proofs",
                report.verdict,
                report.exact_target_backend_matches,
                report.expected_target_backend_matches,
                report.checked_terminal_unsat_proofs
            );
            Ok(())
        }
        Some("e1-msolve-panel") if args.len() == 4 => {
            let report = run_e1_msolve_panel(
                Path::new(&args[1]),
                Path::new(&args[2]),
                &args[3],
            )?;
            println!(
                "{}: {}/{} exact target matches; {}/{} fixed controls",
                report.verdict,
                report.exact_target_matches,
                report.expected_target_matches,
                report.fixed_control_matches,
                report.expected_fixed_control_matches
            );
            Ok(())
        }
        _ => {
            Err(
                "usage: prime-shared-coeff-ir emit ... | prime-shared-coeff-ir stats P A B M T C TARGET_X TARGET_Y | prime-shared-coeff-ir rank5-panel [OUTPUT] | prime-shared-coeff-ir e0-panel | prime-shared-coeff-ir e1-sat-panel CADICAL LRAT_TRIM CRYPTOMINISAT CAKE_XLRUP OUTPUT_DIR SOURCE_REVISION | prime-shared-coeff-ir e1-msolve-panel MSOLVE OUTPUT_DIR SOURCE_REVISION"
                    .to_owned(),
            )
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
