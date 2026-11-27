//! Explicit, bounded single-instance pipeline; never a campaign launcher.
use m31_projective_preclosure::{f4, Poly, Ring, M31};
use m31_projective_preclosure::f4::projection::{self, ProjectionLimits, ProjectionStatus};
use m31_projective_preclosure::systems::{self, Arm, WitnessMode};
use std::{collections::BTreeSet, fs, path::Path};

#[derive(Debug)]
struct Config<'a> {
    out: &'a Path,
    summands: usize,
    case: usize,
    arm: Arm,
    mode: WitnessMode,
}

fn parse(args: &[String]) -> Result<Config<'_>, String> {
    super::validate_options(args, &["--out", "--summands", "--case", "--arm", "--mode"])?;
    let summands = super::number(args, "--summands", 3)?;
    if !(3..=4).contains(&summands) { return Err("solve-one supports m=3 or m=4".into()); }
    let case = super::number(args, "--case", 0)?;
    if case >= 500 { return Err("solve-one requires case in 0..500".into()); }
    let arm_name = super::required(args, "--arm")?;
    let arm = Arm::ALL.into_iter().find(|a| a.name() == arm_name)
        .ok_or_else(|| format!("unknown arm {arm_name}"))?;
    let mode = match super::required(args, "--mode")? {
        "geometric" => WitnessMode::Geometric,
        "fp2" => WitnessMode::ExplicitFp2,
        _ => return Err("--mode must be geometric or fp2".into()),
    };
    Ok(Config { out: Path::new(super::required(args, "--out")?), summands, case, arm, mode })
}

fn producer_limits() -> f4::Limits {
    f4::Limits { max_pairs: 2_000, max_rows: 512, max_columns: 512,
        max_nonzeros: 25_000, max_basis: 64, max_degree: 8,
        max_steps: 32, max_reductions: 20_000 }
}

fn verifier_limits() -> f4::Limits {
    f4::Limits { max_pairs: 10_000, max_steps: 10_000,
        max_reductions: 1_000_000, max_basis: 128, max_degree: 12,
        ..producer_limits() }
}

fn json_string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"), '\r' => out.push_str("\\r"), '\t' => out.push_str("\\t"),
            c if c < '\u{20}' => out.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"'); out
}

fn polynomial(p: &Poly) -> String {
    let terms = p.terms.iter().map(|(m, c)| format!("{{\"exponents\":{m:?},\"coefficient\":{c}}}"))
        .collect::<Vec<_>>().join(",");
    format!("{{\"variables\":{},\"terms\":[{terms}]}}", p.variables)
}
fn polynomials(ps: &[Poly]) -> String {
    format!("[{}]", ps.iter().map(polynomial).collect::<Vec<_>>().join(","))
}
fn points(ps: &BTreeSet<[u64; 2]>) -> String {
    format!("[{}]", ps.iter().map(|p| format!("{p:?}")).collect::<Vec<_>>().join(","))
}
fn limits(l: f4::Limits) -> String {
    format!("{{\"max_pairs\":{},\"max_rows\":{},\"max_columns\":{},\"max_nonzeros\":{},\"max_basis\":{},\"max_degree\":{},\"max_steps\":{},\"max_reductions\":{}}}",
        l.max_pairs, l.max_rows, l.max_columns, l.max_nonzeros, l.max_basis,
        l.max_degree, l.max_steps, l.max_reductions)
}

/// The seven defining relations of an explicit Fp2 chart are retained as
/// ordinary equations. Only the geometric four-relation chart uses NF_J.
fn presentation(model: &systems::System) -> Result<(Ring, Vec<Poly>, Vec<String>, [usize; 2]), String> {
    if model.arm != Arm::Monolithic && model.witness_mode == WitnessMode::Geometric {
        let (ring, equations, names) = model.quotient_presentation().map_err(|e| e.to_string())?;
        let parameters = [names.iter().position(|n| n == "t").ok_or("missing t")?,
                          names.iter().position(|n| n == "u").ok_or("missing u")?];
        Ok((ring, equations, names, parameters))
    } else {
        Ok((Ring::new(0, model.variables.len()).map_err(|e| e.to_string())?,
            model.equations.clone(), model.variables.clone(), model.parameters))
    }
}

fn producer_json(report: &f4::Report) -> String {
    let (status, reason) = match &report.status {
        f4::Status::Complete => ("complete", "null".into()),
        f4::Status::Incomplete { reason } => ("incomplete", json_string(reason)),
    };
    let s = &report.statistics;
    format!("{{\"status\":\"{status}\",\"reason\":{reason},\"fixed_basis_len\":{},\"nonfixed_basis\":{},\"raw_generator_degrees\":{:?},\"prepared_generator_degrees\":{:?},\"scheduled_pairs\":{},\"processed_pairs\":{},\"pending_pairs\":{},\"matrices\":{},\"max_raw_product_degree\":{},\"max_post_nf_matrix_degree\":{},\"polynomial_reductions\":{},\"field_multiplications\":{}}}",
        report.fixed_basis_len, polynomials(&report.basis), report.raw_generator_degrees,
        report.prepared_generator_degrees, s.scheduled_pairs, s.processed_pairs,
        s.pending_pairs, report.matrices.len(), s.max_raw_product_degree,
        s.max_post_nf_matrix_degree, s.polynomial_reductions, s.field_multiplications)
}

fn projection_json(report: &projection::ProjectionReport, expected: &BTreeSet<[u64; 2]>) -> (i32, String) {
    let (status, reason, phase, code) = match &report.status {
        ProjectionStatus::Complete => ("complete", "null".into(), "null".into(), 0),
        ProjectionStatus::Rejected(reason) => ("rejected", json_string(reason), "null".into(), 3),
        ProjectionStatus::Incomplete { phase, reason } =>
            ("incomplete", json_string(reason), json_string(&format!("{phase:?}")), 2),
    };
    let mut code = code;
    let solution = match report.solution() {
        Some(solution) => {
            let agrees = solution.points() == expected;
            if !agrees { code = 4; }
            format!("{{\"points\":{},\"parameter_indices\":{:?},\"indicator_remainders\":{},\"elimination_variables\":[\"t\",\"u\"],\"elimination_generators\":{},\"matches_signed_point_oracle\":{agrees}}}",
                points(solution.points()), solution.parameters(), polynomials(solution.indicator_remainders()),
                polynomials(solution.elimination_generators()))
        },
        None => "null".into(),
    };
    let v = report.verification.as_ref().map_or("null".into(), |v| {
        format!("{{\"status\":{},\"candidate_pairs_checked\":{},\"oracle_pairs_processed\":{},\"original_generators_checked\":{},\"candidate_generators_checked\":{},\"oracle_basis_len\":{}}}",
            json_string(&format!("{:?}", v.status)), v.candidate_pairs_checked, v.oracle_pairs_processed,
            v.original_generators_checked, v.candidate_generators_checked, v.oracle_basis_len)
    });
    let m = &report.metrics;
    let json = format!("{{\"schema\":\"m31-checked-section-projection/v1\",\"field\":{M31},\"status\":\"{status}\",\"reason\":{reason},\"incomplete_phase\":{phase},\"verification\":{v},\"projection_work\":{{\"queries_started\":{},\"queries_completed\":{},\"section_queries_completed\":{},\"indicator_queries_completed\":{},\"leading_term_iterations\":{},\"polynomial_reductions\":{},\"max_degree\":{}}},\"solution\":{solution},\"expected_projection\":{},\"process_exit_code\":{code},\"standing\":\"producer-local-development-output\"}}\n",
        m.queries_started, m.queries_completed, m.section_queries_completed,
        m.indicator_queries_completed, m.leading_term_iterations, m.polynomial_reductions,
        m.max_degree, points(expected));
    (code, json)
}

pub(super) fn run(args: &[String]) -> Result<i32, String> {
    let config = parse(args)?;
    // create_dir, not create_dir_all: no existing output directory is reused.
    fs::create_dir(config.out).map_err(|e| format!("{}: {e}", config.out.display()))?;
    super::write_new(&config.out.join("STARTED"), "Single bounded native development solve. Admission and immutable executable custody are the caller's responsibility. No completion is asserted until result.json exists.\n")?;
    let result = (|| -> Result<i32, String> {
        let plane = systems::fixture_plane(config.summands, config.case)?;
        let model = systems::build(&plane, config.arm, config.mode).map_err(|e| e.to_string())?;
        let (ring, inputs, names, parameters) = presentation(&model)?;
        let meta = format!("{{\"schema\":\"m31-native-solve-one-input/v1\",\"budget_profile\":\"m3-development-v1\",\"field\":{M31},\"summands\":{},\"case\":{},\"arm\":{},\"mode\":{},\"fixture_seed\":{},\"fixture_family\":{},\"target_x\":{},\"forms\":{:?},\"chart_blocks\":{},\"variables\":[{}],\"parameters\":{parameters:?},\"equations\":{},\"producer_limits\":{},\"verifier_limits\":{},\"projection_limits\":{{\"max_iterations\":1000,\"max_reductions\":1000,\"max_terms\":1000,\"max_degree\":2}},\"timing_and_rss\":\"not-measured\"}}\n",
            config.summands, config.case, json_string(config.arm.name()),
            json_string(if config.mode == WitnessMode::Geometric { "geometric" } else { "fp2" }),
            plane.seed, json_string(&plane.family), plane.target_x, plane.forms, ring.block_count(),
            names.iter().map(|n| json_string(n)).collect::<Vec<_>>().join(","),
            polynomials(&inputs), limits(producer_limits()), limits(verifier_limits()));
        super::write_new(&config.out.join("input.json"), meta)?;
        super::write_new(&config.out.join("input.ms"), model.msolve_input())?;
        let basis = f4::compute(&ring, &inputs, producer_limits()).map_err(|e| e.to_string())?;
        super::write_new(&config.out.join("basis.json"), producer_json(&basis))?;
        let report = projection::verify_and_project(&ring, &inputs, &basis, parameters,
            verifier_limits(), ProjectionLimits::default()).map_err(|e| e.to_string())?;
        // This oracle is called AFTER native projection and is never its input.
        let expected = systems::expected_projection(&plane)?;
        let (code, json) = projection_json(&report, &expected);
        super::write_new(&config.out.join("result.json"), json)?;
        println!("M31 m={} case={} arm={} status={:?}; output={}; exit={code}",
            config.summands, config.case, config.arm.name(), report.status, config.out.display());
        Ok(code)
    })();
    if let Err(error) = &result {
        super::write_new(&config.out.join("error.json"), format!("{{\"status\":\"error\",\"solution\":null,\"reason\":{}}}\n", json_string(error)))?;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn args(extra: &[&str]) -> Vec<String> {
        [vec!["--out", "unused", "--arm", "preclosed_sequential", "--mode", "geometric"],
         extra.to_vec()].concat().into_iter().map(str::to_owned).collect()
    }
    #[test]
    fn strict_config_and_json_escaping() {
        assert_eq!(parse(&args(&[])).unwrap().summands, 3);
        for extra in [vec!["--summands", "2"], vec!["--case", "500"], vec!["--arm", "monolithic"],
                      vec!["--mode", "fp2"], vec!["--unknown", "1"], vec!["--case"]] {
            assert!(parse(&args(&extra)).is_err(), "{extra:?}");
        }
        assert_eq!(json_string("\"\\\n\r\t\u{1}"), "\"\\\"\\\\\\n\\r\\t\\u0001\"");
    }
    #[test]
    fn representation_adapter_keeps_fp2_equations_and_chart_permutation() {
        let plane = systems::fixture_plane(3, 0).unwrap();
        for arm in Arm::ALL { for mode in [WitnessMode::Geometric, WitnessMode::ExplicitFp2] {
            let model = systems::build(&plane, arm, mode).unwrap();
            let (ring, equations, names, indices) = presentation(&model).unwrap();
            assert_eq!(equations.len(), model.equations.len());
            assert_eq!(names[indices[0]], "t"); assert_eq!(names[indices[1]], "u");
            if arm == Arm::Monolithic || mode == WitnessMode::ExplicitFp2 {
                assert_eq!(ring.block_count(), 0);
                assert_eq!(equations, model.equations);
                assert_eq!(indices, model.parameters);
            } else {
                assert_eq!(ring.block_count(), 1);
                assert_eq!(&names[..3], ["z0", "w0", "s0"]);
            }
        }}
    }
    #[test]
    fn serialized_limit_is_null_and_completed_empty_support_is_an_empty_list() {
        let ring = Ring::new(0, 2).unwrap();
        let t = Poly::variable(2, 0); let u = Poly::variable(2, 1);
        let section = vec![t.pow(2).unwrap().sub(&t).unwrap(),
                           u.pow(2).unwrap().sub(&u).unwrap()];
        let incomplete = f4::compute(&ring, &section,
            f4::Limits { max_pairs: 0, ..producer_limits() }).unwrap();
        let report = projection::verify_and_project(&ring, &section, &incomplete,
            [0, 1], verifier_limits(), ProjectionLimits::default()).unwrap();
        let (code, json) = projection_json(&report, &BTreeSet::new());
        assert_eq!(code, 2);
        assert!(json.contains("\"status\":\"incomplete\""));
        assert!(json.contains("\"solution\":null"));
        assert!(!json.contains("\"points\":[]"));

        let mut inconsistent = section;
        inconsistent.push(Poly::constant(2, 1));
        let basis = f4::compute(&ring, &inconsistent, producer_limits()).unwrap();
        let report = projection::verify_and_project(&ring, &inconsistent, &basis,
            [0, 1], verifier_limits(), ProjectionLimits::default()).unwrap();
        let (code, json) = projection_json(&report, &BTreeSet::new());
        assert_eq!(code, 0);
        assert!(json.contains("\"status\":\"complete\""));
        assert!(json.contains("\"points\":[]"));
        assert!(json.contains("\"matches_signed_point_oracle\":true"));
        let (code, json) = projection_json(&report, &BTreeSet::from([[0, 0]]));
        assert_eq!(code, 4);
        assert!(json.contains("\"matches_signed_point_oracle\":false"));
        let rejected = projection::verify_and_project(&ring, &inconsistent[..2],
            &basis, [0, 1], verifier_limits(), ProjectionLimits::default()).unwrap();
        let (code, json) = projection_json(&rejected, &BTreeSet::new());
        assert_eq!(code, 3);
        assert!(json.contains("\"status\":\"rejected\""));
        assert!(json.contains("\"solution\":null"));
    }
}
