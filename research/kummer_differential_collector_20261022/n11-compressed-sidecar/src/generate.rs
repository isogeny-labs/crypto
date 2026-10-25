use kummer_n11_sidecar::{SCHEMA, gcd, prime, read_json, root, sha256, u64_field};
use serde_json::{Value, json};
use std::fmt::Write as _;
use std::fs;

const HEADER: &str = "schema,bits,p,r,jump_index,jump_scalar,fixed_offset_classes,fixed_offset_explicit_bytes,pair_classes,pair_bytes,pair_plus_label_bytes_external_step,pair_plus_label_and_step_bytes,minimum_label_bits_for_all_histories,regular_pair_step_xadds,regular_pair_step_field_m,regular_pair_step_field_a,pair_sign_ambiguities,coordinate_xadd_exception_bound,cohort_eligible\n";

fn main() {
    let here = root();
    let input = here.join("../orbit-proof/frozen-inputs.json");
    let walk_source = here.join("../src/walk_impl.rs");
    let frozen = read_json(&input);
    assert_eq!(frozen["schema"], "kummer-fixed-offset-orbit-input/v1");
    let models = frozen["models"].as_array().expect("models");
    assert_eq!(models.len(), 21);
    let mut rows: Vec<Value> = Vec::new();
    let mut csv = String::from(HEADER);
    let mut first_by_model: Vec<(u64, u64)> = Vec::new();
    for model in models {
        let bits = u64_field(model, "bits");
        let p = u64_field(model, "p");
        let r = u64_field(model, "r");
        assert!(prime(r) && r > 2);
        let jumps = model["jumps"].as_array().expect("jumps");
        assert_eq!(jumps.len(), 16);
        first_by_model.push((bits, 32 * r));
        for jump in jumps {
            let jump_index = u64_field(jump, "jump_index");
            let jump_scalar = u64_field(jump, "jump_scalar");
            assert!(jump_scalar > 0 && jump_scalar < r && gcd(jump_scalar, r) == 1);
            let bytes = 32 * r;
            let label_bits = 64 - (r - 1).leading_zeros();
            let row = json!([
                SCHEMA,
                bits,
                p,
                r,
                jump_index,
                jump_scalar,
                r,
                bytes,
                2,
                64,
                72,
                80,
                label_bits,
                3,
                72,
                108,
                0,
                null,
                false
            ]);
            writeln!(csv, "{SCHEMA},{bits},{p},{r},{jump_index},{jump_scalar},{r},{bytes},2,64,72,80,{label_bits},3,72,108,0,,False").unwrap();
            rows.push(row);
        }
    }
    assert_eq!(rows.len(), 336);
    let results = here.join("results");
    fs::create_dir_all(&results).expect("results directory");
    fs::write(results.join("bounds.csv"), csv).expect("write CSV");
    let bundle = json!({
        "schema": SCHEMA,
        "study_id": "KUMMER-N11-COMPRESSED-SIDECAR-20261025",
        "input_sha256": sha256(&input),
        "walk_source_sha256": sha256(&walk_source),
        "proof_scope": "odd-prime cyclic group and exact Kummer quotient",
        "columns": HEADER.trim_end().split(',').collect::<Vec<_>>(),
        "row_format": "array-in-columns-order",
        "coordinate_xadd_exception_bound": null,
        "cohort_eligible": false,
        "rows": rows
    });
    fs::write(
        results.join("bounds.json"),
        format!("{}\n", serde_json::to_string(&bundle).unwrap()),
    )
    .expect("write JSON");
    fs::write(here.join("state-graph.svg"), graph(&first_by_model)).expect("write graph");
    fs::write(here.join("REPORT.pdf"), pdf(&first_by_model)).expect("write deterministic PDF");
    let files = [
        "../orbit-proof/frozen-inputs.json",
        "../src/walk_impl.rs",
        "Cargo.toml",
        "Cargo.lock",
        ".gitignore",
        "src/lib.rs",
        "src/generate.rs",
        "src/validate.rs",
        "REPORT.md",
        "PROTOCOL.md",
        "state-graph.svg",
        "REPORT.pdf",
        "results/bounds.csv",
        "results/bounds.json",
    ];
    let sums = files
        .iter()
        .map(|name| format!("{}  {name}\n", sha256(&here.join(name))))
        .collect::<String>();
    fs::write(here.join("SHA256SUMS"), sums).expect("write checksums");
    println!("generated {} rows from {} models", rows.len(), models.len());
}

fn pdf(models: &[(u64, u64)]) -> Vec<u8> {
    // Base-14 Helvetica needs no embedded font file or generated timestamp.
    let lines = [
        "N11: compressed sign-safe Kummer sidecar",
        "25 October 2026  |  Proof and derivation packet  |  No cohort run",
        "",
        "State: (K(D), K(D+G), a, n), with D = aG + 2^n T in an odd-prime group.",
        "A branch c in {0,1} sends D to 2D+cG and a to 2a+c modulo r.",
        "The target coefficient is 2^(n+1); induction preserves the invariant.",
        "",
        "Sign recovery: if K(X)=K(Y), then Y=X or Y=-X.",
        "If Y=-X and second classes match, then 2X=0 or 2G=0.",
        "The first makes Y=X; the second is impossible for nonzero odd-order G.",
        "Thus (K(X), K(X+G)) is injective, including at identity.",
        "A signed endpoint is checked against aG + (2^n mod r)T.",
        "",
        "Across 21 frozen models and 16 jumps: 336 raw rows, 21 group orders.",
        "Explicit pair plus one label: 72 bytes with an external step count.",
        "With a stored counter: 80 bytes. Existing pair plus two labels: 80 bytes.",
        "Regular source path: three xADD calls, 72 field M and 108 field A.",
        "N7 fixed-offset reference: r classes, 32r bytes; different transition.",
        "",
        "Mathematical pair-sign ambiguity: zero in the stated group model.",
        "Concrete theta xADD exception bound and signed-lift cost: unknown.",
        "Cohort eligibility: false. No timing or rho-speed claim is made.",
    ];
    let mut stream = String::from("BT /F1 10 Tf 45 755 Td 15 TL\n");
    for line in lines {
        let escaped = line
            .replace('\\', "\\\\")
            .replace('(', "\\(")
            .replace(')', "\\)");
        writeln!(stream, "({escaped}) Tj T*").unwrap();
    }
    stream.push_str("ET\n");
    let chart = pdf_chart(models);
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
        "<< /Type /Pages /Kids [3 0 R 6 0 R] /Count 2 >>".to_owned(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>".to_owned(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_owned(),
        format!("<< /Length {} >>\nstream\n{}endstream", stream.len(), stream),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 7 0 R >>".to_owned(),
        format!("<< /Length {} >>\nstream\n{}endstream", chart.len(), chart),
    ];
    let mut out = String::from("%PDF-1.4\n");
    let mut offsets = vec![0];
    for (i, object) in objects.iter().enumerate() {
        offsets.push(out.len());
        writeln!(out, "{} 0 obj\n{}\nendobj", i + 1, object).unwrap();
    }
    let xref = out.len();
    out.push_str("xref\n0 8\n0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        writeln!(out, "{offset:010} 00000 n ").unwrap();
    }
    write!(
        out,
        "trailer\n<< /Size 8 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n"
    )
    .unwrap();
    out.into_bytes()
}

fn pdf_chart(models: &[(u64, u64)]) -> String {
    assert_eq!(models.len(), 21);
    let (x0, x1, y0, y1) = (80.0, 550.0, 145.0, 610.0);
    let top = models.iter().map(|(_, bytes)| *bytes).max().unwrap() as f64;
    let xx = |bits: u64| x0 + (bits as f64 - 16.0) * (x1 - x0) / 24.0;
    let yy = |bytes: f64| y0 + (bytes.log10() - 1.0) / (top.log10() - 1.0) * (y1 - y0);
    let mut out = String::from("0 0 0 RG 0 0 0 rg\n");
    out.push_str("BT /F1 16 Tf 48 743 Td (Explicit state by subgroup size) Tj ET\n");
    out.push_str("BT /F1 10 Tf 48 720 Td (KUMMER-N11-COMPRESSED-SIDECAR-20261025 | 21 models, 336 model/jump rows) Tj ET\n");
    out.push_str(
        "BT /F1 10 Tf 48 701 Td (Derived bytes on a log10 axis; no measured timing) Tj ET\n",
    );
    out.push_str("0.85 0.85 0.85 RG 0.5 w\n");
    for power in [2, 4, 6, 8, 10, 12, 14] {
        let y = yy(10_f64.powi(power));
        if (y0..=y1).contains(&y) {
            writeln!(out, "{x0:.1} {y:.1} m {x1:.1} {y:.1} l S").unwrap();
            writeln!(out, "BT /F1 9 Tf 39 {:.1} Td (10^{power}) Tj ET", y - 3.0).unwrap();
        }
    }
    out.push_str("0 0 0 RG 1 w\n");
    writeln!(out, "{x0:.1} {y0:.1} m {x0:.1} {y1:.1} l S").unwrap();
    writeln!(out, "{x0:.1} {y0:.1} m {x1:.1} {y0:.1} l S").unwrap();
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        writeln!(
            out,
            "BT /F1 9 Tf {:.1} 130 Td ({bits}) Tj ET",
            xx(bits) - 5.0
        )
        .unwrap();
    }
    out.push_str("BT /F1 10 Tf 260 108 Td (subgroup bits) Tj ET\n");
    out.push_str("BT /F1 10 Tf 39 631 Td (bytes, log10 scale) Tj ET\n");
    out.push_str("0.10 0.40 0.64 RG 2 w\n");
    writeln!(out, "{x0:.1} {:.1} m {x1:.1} {:.1} l S", yy(72.0), yy(72.0)).unwrap();
    out.push_str("0.70 0.29 0.14 rg\n");
    for &(bits, bytes) in models {
        writeln!(
            out,
            "{:.1} {:.1} 7 7 re f",
            xx(bits) - 3.5,
            yy(bytes as f64) - 3.5
        )
        .unwrap();
    }
    out.push_str("0.70 0.29 0.14 rg 55 82 8 8 re f 0 0 0 rg\n");
    out.push_str("BT /F1 9 Tf 70 82 Td (N7 fixed-offset explicit state: 32r bytes) Tj ET\n");
    out.push_str("0.10 0.40 0.64 RG 2 w 55 67 m 65 67 l S 0 0 0 RG\n");
    out.push_str("BT /F1 9 Tf 70 64 Td (differential pair + one label: 72 bytes, external step count) Tj ET\n");
    out.push_str("BT /F1 8 Tf 48 39 Td (N7 frozen inputs; markers overlap at shared bit sizes. Different transitions.) Tj ET\n");
    out
}

fn graph(models: &[(u64, u64)]) -> String {
    let (x0, x1, y0, y1) = (88.0, 720.0, 44.0, 400.0);
    let pmax = models.iter().map(|(_, b)| *b).max().unwrap() as f64;
    let xx = |bits: u64| x0 + (bits as f64 - 16.0) * (x1 - x0) / 24.0;
    let yy = |bytes: f64| y1 - (bytes / 72.0).log10() / (pmax / 72.0).log10() * (y1 - y0);
    let mut out = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"800\" height=\"460\" viewBox=\"0 0 800 460\">\n<rect width=\"800\" height=\"460\" fill=\"white\"/>\n<text x=\"55\" y=\"22\" font-size=\"17\">Explicit state by subgroup size (log byte axis)</text>\n<path d=\"M{x0} {y0}V{y1}H{x1}\" fill=\"none\" stroke=\"black\"/>\n"
    );
    for power in [2, 4, 6, 8, 10, 12, 14] {
        let y = yy(10_f64.powi(power));
        if (y0..=y1).contains(&y) {
            writeln!(out, "<path d=\"M{} {y:.1}H{x1}\" stroke=\"#ddd\"/><text x=\"20\" y=\"{:.1}\" font-size=\"12\">10^{power}</text>", x0 - 5.0, y + 4.0).unwrap();
        }
    }
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        writeln!(
            out,
            "<text x=\"{:.1}\" y=\"420\" font-size=\"12\">{bits}</text>",
            xx(bits) - 9.0
        )
        .unwrap();
    }
    writeln!(
        out,
        "<text x=\"362\" y=\"444\" font-size=\"12\">subgroup bits</text>"
    )
    .unwrap();
    writeln!(
        out,
        "<path d=\"M{x0} {:.1}H{x1}\" stroke=\"#1965a3\" stroke-width=\"2\"/>",
        yy(72.0)
    )
    .unwrap();
    for &(bits, bytes) in models {
        writeln!(
            out,
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"#b34b27\"/>",
            xx(bits),
            yy(bytes as f64)
        )
        .unwrap();
    }
    out.push_str("<text x=\"525\" y=\"60\" fill=\"#b34b27\" font-size=\"12\">N7 fixed-offset explicit state</text>\n<text x=\"525\" y=\"389\" fill=\"#1965a3\" font-size=\"12\">pair + label, external step: 72 B</text>\n</svg>\n");
    out
}
