use prime31_selector_sat::circuit::{validate_dimacs, BooleanEncoding};
use prime31_selector_sat::panel::{prime31_weighted_relation, PanelTarget};
use prime31_selector_sat::rr9::{build_frozen_tower, polynomial_from_roots};
use prime31_selector_sat::rr9_panel::{prime31_rr9_case, Rr9Target, SharedRr9Case};
use prime31_selector_sat::{RelationSpec, PRIME31_A, PRIME31_P};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;

#[derive(Serialize)]
struct ArtifactManifest {
    schema: &'static str,
    source_case: String,
    rendering: String,
    artifact_bytes: usize,
    artifact_sha256: String,
    statistics: prime31_selector_sat::EncodingStatistics,
    pcc: &'static str,
}

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn read_relation(path: &str) -> Result<RelationSpec, String> {
    let bytes = fs::read(path).map_err(|error| format!("read {path}: {error}"))?;
    let relation = serde_json::from_slice::<RelationSpec>(&bytes)
        .map_err(|error| format!("parse {path}: {error}"))?;
    relation.validate().map_err(|error| error.to_string())?;
    Ok(relation)
}

fn write_json(path: &str, value: &impl Serialize) -> Result<(), String> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    fs::write(path, bytes).map_err(|error| format!("write {path}: {error}"))
}

fn usage() -> String {
    [
        "usage:",
        "  prime31-selector-sat generate <B> <natural|planted> <index> <relation.json>",
        "  prime31-selector-sat stats <relation.json>",
        "  prime31-selector-sat encode <relation.json> <cnf|xnf> <formula> <manifest.json>",
        "  prime31-selector-sat verify <relation.json> <solver-output>",
        "  prime31-selector-sat verify-prefix <relation.json> <external-record.json>",
        "  prime31-selector-sat rr9-generate <B> <known-chart|natural-N> <case.json>",
        "  prime31-selector-sat rr9-stats <case.json>",
        "  prime31-selector-sat tower-shapes",
    ]
    .join("\n")
}

fn parse_usize(name: &str, value: &str) -> Result<usize, String> {
    value
        .parse::<usize>()
        .map_err(|_| format!("invalid {name}: {value}"))
}

fn main() -> Result<(), String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let Some(command) = arguments.first().map(String::as_str) else {
        return Err(usage());
    };
    match command {
        "generate" if arguments.len() == 5 => {
            let base_size = parse_usize("base size", &arguments[1])?;
            let index = parse_usize("panel index", &arguments[3])?;
            let target = match arguments[2].as_str() {
                "natural" => PanelTarget::Natural(index),
                "planted" => PanelTarget::Planted(index),
                value => return Err(format!("unknown panel class {value:?}\n{}", usage())),
            };
            let relation = prime31_weighted_relation(base_size, target)?;
            write_json(&arguments[4], &relation)?;
        }
        "stats" if arguments.len() == 2 => {
            let relation = read_relation(&arguments[1])?;
            let encoding = BooleanEncoding::from_relation(&relation)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&encoding.statistics())
                    .map_err(|error| error.to_string())?
            );
        }
        "encode" if arguments.len() == 5 => {
            let relation = read_relation(&arguments[1])?;
            let encoding = BooleanEncoding::from_relation(&relation)?;
            let (rendering, formula, allow_xor) = match arguments[2].as_str() {
                "cnf" => ("ordinary_cnf", encoding.to_dimacs(), false),
                "xnf" => ("cnf_xor", encoding.to_cnf_xor(), true),
                value => return Err(format!("unknown rendering {value:?}")),
            };
            validate_dimacs(&formula, allow_xor)?;
            fs::write(&arguments[3], formula.as_bytes())
                .map_err(|error| format!("write {}: {error}", arguments[3]))?;
            let manifest = ArtifactManifest {
                schema: "prime31.selector-artifact-manifest/v1",
                source_case: relation.case_id,
                rendering: rendering.into(),
                artifact_bytes: formula.len(),
                artifact_sha256: digest(formula.as_bytes()),
                statistics: encoding.statistics(),
                pcc: "unverified",
            };
            write_json(&arguments[4], &manifest)?;
        }
        "verify" if arguments.len() == 3 => {
            let relation = read_relation(&arguments[1])?;
            let encoding = BooleanEncoding::from_relation(&relation)?;
            let output = fs::read_to_string(&arguments[2])
                .map_err(|error| format!("read {}: {error}", arguments[2]))?;
            let witness = encoding.decode_solver_model(&relation, &output)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&witness).map_err(|error| error.to_string())?
            );
        }
        "verify-prefix" if arguments.len() == 3 => {
            let relation = read_relation(&arguments[1])?;
            let encoding = BooleanEncoding::from_relation(&relation)?;
            let bytes = fs::read(&arguments[2])
                .map_err(|error| format!("read {}: {error}", arguments[2]))?;
            let record = serde_json::from_slice::<serde_json::Value>(&bytes)
                .map_err(|error| format!("parse {}: {error}", arguments[2]))?;
            if record.get("status").and_then(serde_json::Value::as_str) != Some("sat") {
                return Err("external record does not declare SAT".into());
            }
            let prefix = record
                .get("model")
                .and_then(serde_json::Value::as_str)
                .ok_or("external SAT record has no semantic model prefix")?;
            let witness = encoding.decode_semantic_prefix(&relation, prefix)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&witness).map_err(|error| error.to_string())?
            );
        }
        "rr9-generate" if arguments.len() == 4 => {
            let base_size = parse_usize("base size", &arguments[1])?;
            let target = if arguments[2] == "known-chart" {
                Rr9Target::KnownChart
            } else if let Some(index) = arguments[2].strip_prefix("natural-") {
                Rr9Target::Natural(parse_usize("RR9 natural panel index", index)?)
            } else {
                return Err(format!(
                    "unknown RR9 target {:?}\n{}",
                    arguments[2],
                    usage()
                ));
            };
            let case = prime31_rr9_case(base_size, target)?;
            write_json(&arguments[3], &case)?;
        }
        "rr9-stats" if arguments.len() == 2 => {
            let bytes = fs::read(&arguments[1])
                .map_err(|error| format!("read {}: {error}", arguments[1]))?;
            let case = serde_json::from_slice::<SharedRr9Case>(&bytes)
                .map_err(|error| format!("parse {}: {error}", arguments[1]))?;
            case.validate()?;
            let tower = build_frozen_tower(
                case.curve.p,
                case.curve.a,
                (case.residual.x, case.residual.y),
                &case.support_polynomial,
            )?;
            println!(
                "{}",
                serde_json::to_string_pretty(&tower.spec).map_err(|error| error.to_string())?
            );
        }
        "tower-shapes" if arguments.len() == 1 => {
            let mut specifications = Vec::new();
            for base_size in [4usize, 8, 12, 16, 24, 31] {
                let roots = (0..base_size as u64).collect::<Vec<_>>();
                let h_b = polynomial_from_roots(&roots, PRIME31_P);
                let tower =
                    build_frozen_tower(PRIME31_P, PRIME31_A, (1_911_579_686, 548_820_096), &h_b)?;
                specifications.push(tower.spec);
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&specifications).map_err(|error| error.to_string())?
            );
        }
        _ => return Err(usage()),
    }
    Ok(())
}
