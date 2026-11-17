//! Validate the machine-readable inventory of the five Lean scaling results.
//! The Lean compiler supplies the proof; this checks that the exported report
//! names those exact results and does not silently change their scope.

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.to_owned()) }
}

fn emit(root: &Path) -> Result<(), String> {
    let packet = serde_json::json!({
        "schema": "projective-law-cross-scale22-weights/v1",
        "hypotheses": "q and r have common weights Z^2 and Z^4; s and t have common weights Z^3 W and Z^5 W; nonzero branch equivalence requires nonzero Z and W",
        "rows": [
            {"coefficient": "B", "z_power": 8, "w_power": 0,
             "theorem": "CrossScale22.scale_B", "nonzero_theorem": "CrossScale22.scale_B_ne_zero_iff"},
            {"coefficient": "A", "z_power": 9, "w_power": 1,
             "theorem": "CrossScale22.scale_A", "nonzero_theorem": null},
            {"coefficient": "C", "z_power": 7, "w_power": 1,
             "theorem": "CrossScale22.scale_C", "nonzero_theorem": "CrossScale22.scale_C_ne_zero_iff"}
        ],
        "runtime_semantic_link": "open",
        "cost_ratio_change": null
    });
    let json = serde_json::to_string_pretty(&packet).map_err(|e| e.to_string())? + "\n";
    let csv = concat!(
        "coefficient,z_power,w_power,theorem,nonzero_theorem\n",
        "B,8,0,CrossScale22.scale_B,CrossScale22.scale_B_ne_zero_iff\n",
        "A,9,1,CrossScale22.scale_A,\n",
        "C,7,1,CrossScale22.scale_C,CrossScale22.scale_C_ne_zero_iff\n"
    );
    fs::write(root.join("cross-scale22-weights.json"), json).map_err(|e| e.to_string())?;
    fs::write(root.join("cross-scale22-weights.csv"), csv).map_err(|e| e.to_string())?;
    Ok(())
}

fn validate(root: &Path) -> Result<(), String> {
    let source = read(&root.join("CrossScale22.lean"))?;
    let log = read(&root.join("cross-scale22-build.log"))?;
    let weights: Value = serde_json::from_str(&read(&root.join("cross-scale22-weights.json"))?)
        .map_err(|e| format!("weights JSON: {e}"))?;
    require(weights["schema"] == "projective-law-cross-scale22-weights/v1", "schema mismatch")?;
    require(weights["runtime_semantic_link"] == "open", "runtime link must remain open")?;
    require(weights["cost_ratio_change"].is_null(), "cost change must remain null")?;
    let rows = weights["rows"].as_array().ok_or("rows must be an array")?;
    let expected = [
        ("B", 8, 0, "scale_B", Some("scale_B_ne_zero_iff")),
        ("A", 9, 1, "scale_A", None),
        ("C", 7, 1, "scale_C", Some("scale_C_ne_zero_iff")),
    ];
    require(rows.len() == expected.len(), "expected three weight rows")?;
    let mut csv = String::from("coefficient,z_power,w_power,theorem,nonzero_theorem\n");
    for (row, (coefficient, z_power, w_power, theorem, nonzero)) in rows.iter().zip(expected) {
        let theorem_name = format!("CrossScale22.{theorem}");
        let nonzero_name = nonzero.map(|name| format!("CrossScale22.{name}"));
        require(row["coefficient"] == coefficient, "coefficient mismatch")?;
        require(row["z_power"] == z_power, "Z exponent mismatch")?;
        require(row["w_power"] == w_power, "W exponent mismatch")?;
        require(row["theorem"] == theorem_name, "theorem name mismatch")?;
        require(row["nonzero_theorem"].as_str() == nonzero_name.as_deref(), "nonzero theorem mismatch")?;
        csv.push_str(&format!("{coefficient},{z_power},{w_power},{theorem_name},{}\n",
            nonzero_name.unwrap_or_default()));
    }
    require(read(&root.join("cross-scale22-weights.csv"))? == csv, "CSV/JSON mismatch")?;
    require(!source.contains("sorry") && !source.contains("admit") && !source.contains("axiom "),
        "forbidden proof token")?;
    for theorem in ["scale_B", "scale_A", "scale_C", "scale_B_ne_zero_iff", "scale_C_ne_zero_iff"] {
        require(source.lines().any(|line| line.starts_with(&format!("theorem {theorem} "))),
            &format!("missing Lean declaration: {theorem}"))?;
        let axioms = if theorem.ends_with("ne_zero_iff") {
            "[propext, Classical.choice, Quot.sound]"
        } else { "[propext, Quot.sound]" };
        require(log.contains(&format!("'CrossScale22.{theorem}' depends on axioms: {axioms}")),
            &format!("missing kernel receipt: {theorem}"))?;
    }
    for artifact in ["CROSS_SCALE22.md", "CROSS_SCALE22.pdf", "cross-scale22.csv",
        "cross-scale22.svg", "cross-scale22.png", "cross-scale22-weights.csv",
        "cross-scale22-weights.json"] {
        require(fs::metadata(root.join(artifact)).map(|m| m.len() > 0).unwrap_or(false),
            &format!("missing or empty artifact: {artifact}"))?;
    }
    let pdf = fs::read(root.join("CROSS_SCALE22.pdf")).map_err(|e| e.to_string())?;
    require(pdf.starts_with(b"%PDF-"), "invalid PDF header")?;
    Ok(())
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let first = args.next();
    let emit_requested = first.as_deref().is_some_and(|value| value == "--emit");
    let root = (if emit_requested { args.next() } else { first }).map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("formal"));
    if emit_requested {
        if let Err(error) = emit(&root) {
            eprintln!("cross-scale weight emission failed: {error}");
            std::process::exit(1);
        }
    }
    if let Err(error) = validate(&root) {
        eprintln!("cross-scale weight validation failed: {error}");
        std::process::exit(1);
    }
    println!("cross-scale weight validation: 3 rows, 5 kernel receipts, report artifacts pass");
}
