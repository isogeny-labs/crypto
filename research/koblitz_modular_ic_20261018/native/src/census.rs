//! Native scan of pinned, published genus-five point-count data.
use crate::{lpoly, round02, sha256_impl, weil};
use serde_json::json;
use std::path::Path;
fn records(text: &str) -> Vec<&str> {
    let mut depth = 0;
    let mut start = 0;
    let mut out = Vec::new();
    for (i, ch) in text.char_indices() {
        if ch == '[' {
            depth += 1;
            if depth == 2 {
                start = i;
            }
        } else if ch == ']' {
            if depth == 2 {
                out.push(&text[start..=i]);
            }
            depth -= 1;
        }
    }
    out
}
pub fn run(dir: &Path) {
    let source = dir.join("census");
    let mut tables = Vec::new();
    let mut hits = Vec::new();
    for (kind, file, equations) in [
        (
            "hyperelliptic",
            "NumPts_Hyp.txt",
            "HyperellipticCurvesData.txt",
        ),
        (
            "trigonal",
            "NumPts_Trig.txt",
            "TrigonalCurvesWithAutomorphisms.txt",
        ),
        (
            "complete_intersection",
            "Pts_Count_Complete_Intersections.txt",
            "Complete_Intersections.txt",
        ),
    ] {
        let bytes = std::fs::read(source.join(file)).unwrap();
        let rows: Vec<Vec<i64>> = serde_json::from_slice(&bytes).unwrap();
        let eq_text = std::fs::read_to_string(source.join(equations)).unwrap();
        let eqs = records(&eq_text);
        let mut bad = Vec::new();
        let mut matched = 0;
        for (index, counts) in rows.iter().enumerate() {
            assert_eq!(counts.len(), 5);
            let Some(poly) = lpoly::char_poly_from_counts(counts, 2) else {
                bad.push(index);
                continue;
            };
            for trace in [-1, 1] {
                if let Some(cofactor) =
                    lpoly::divide_exact(&poly, &weil::to_i128(&weil::trace_zero_poly(trace, 5)))
                {
                    matched += 1;
                    hits.push(json!({"kind":kind,"index_zero_based":index,"trace":trace,"counts":counts,"polynomial":poly,"cofactor":cofactor,"equation_file":equations,"equation_record":eqs.get(index),"independent_curve_recount":"pending"}));
                }
            }
        }
        println!(
            "Census {kind}: {} records, {matched} A5 matches, {} nonintegral",
            rows.len(),
            bad.len()
        );
        tables.push(json!({"kind":kind,"file":file,"sha256":hex::encode(sha256_impl::sha256(&bytes)),"rows":rows.len(),"equation_records":eqs.len(),"newton_nonintegral_indices":bad,"matches":matched}));
    }
    round02::save(
        dir,
        "gl3-genus5-census.json",
        &json!({"schema":"koblitz.round03/gl3-v1","source_commit":std::fs::read_to_string(dir.join("census-commit.txt")).unwrap().trim(),"tables":tables,"hits":hits,"scope":"all vectors in pinned published tables; completeness and curve smoothness rely on the census until independently checked"}),
    );
}

fn decode_label(label: &str) -> Vec<i128> {
    let fields: Vec<_> = label.split('.').collect();
    assert_eq!(&fields[..2], ["6", "2"]);
    let mut half = vec![1i128];
    for token in fields[2].split('_') {
        let value = token.bytes().fold(0i128, |a, c| {
            assert!(c.is_ascii_lowercase());
            26 * a + (c - b'a') as i128
        });
        half.push(if token.starts_with('a') {
            -value
        } else {
            value
        });
    }
    assert_eq!(half.len(), 7);
    let mut poly = vec![0; 13];
    for i in 0..=6 {
        poly[12 - i] = half[i];
        poly[i] = (1i128 << (6 - i)) * half[i];
    }
    poly
}
fn encode_label(poly: &[i128]) -> String {
    let tokens: Vec<_> = (1..=6)
        .map(|i| {
            let value = poly[12 - i];
            let mut n = value.abs();
            let mut token = Vec::new();
            while n > 0 {
                token.push((b'a' + (n % 26) as u8) as char);
                n /= 26;
            }
            token.reverse();
            let digits: String = token.into_iter().collect();
            if value <= 0 {
                format!("a{digits}")
            } else {
                digits
            }
        })
        .collect();
    format!("6.2.{}", tokens.join("_"))
}
pub fn run_six(dir: &Path) {
    let bytes = std::fs::read(dir.join("census6/lmfdbgenus6_curves_F2.txt")).unwrap();
    let text = std::str::from_utf8(&bytes).unwrap();
    let mut hits = Vec::new();
    let mut rows = 0;
    let mut curves = 0;
    for (index, line) in text.lines().enumerate() {
        let fields: Vec<_> = line.split('|').collect();
        assert_eq!(fields.len(), 5);
        let label = fields[0];
        let poly = decode_label(label);
        assert_eq!(encode_label(&poly), label);
        let number = fields[2].parse::<usize>().unwrap();
        curves += number;
        rows += 1;
        let counts = lpoly::counts_from_char_poly(&poly, 2, 6);
        let counts64: Vec<_> = counts.iter().map(|v| *v as i64).collect();
        assert_eq!(lpoly::char_poly_from_counts(&counts64, 2).unwrap(), poly);
        for trace in [-1, 1] {
            if let Some(cofactor) =
                lpoly::divide_exact(&poly, &weil::to_i128(&weil::trace_zero_poly(trace, 5)))
            {
                hits.push(json!({"label":label,"index_zero_based":index,"trace":trace,"polynomial":poly,"cofactor":cofactor,"counts":counts,"curve_count":number,"hyperelliptic_count":fields[3].parse::<usize>().unwrap(),"equations_latex":fields[4],"independent_recount":"pending"}));
            }
        }
    }
    assert_eq!((rows, curves), (38327, 72227));
    println!(
        "Census genus6: {rows} polynomials, {curves} curves, {} A5 matching classes",
        hits.len()
    );
    round02::save(
        dir,
        "dp4-genus6-census.json",
        &json!({"schema":"koblitz.round03/dp4-v1","source_commit":std::fs::read_to_string(dir.join("census6-commit.txt")).unwrap().trim(),"input_sha256":hex::encode(sha256_impl::sha256(&bytes)),"polynomials":rows,"curves":curves,"label_roundtrips_verified":rows,"newton_roundtrips_verified":rows,"hits":hits,"scope":"all exported genus6 curve classes; completeness relies on the published census; no Sage/Python code executed"}),
    );
}

pub fn genus_four_bounds(dir: &Path) {
    let mut rows = Vec::new();
    for trace in [-1, 1] {
        let p = weil::to_i128(&weil::trace_zero_poly(trace, 5));
        let counts = lpoly::counts_from_char_poly(&p, 2, 5);
        let monotone = counts[1] >= counts[0];
        let bound = counts[4] <= 71;
        if trace == 1 {
            assert_eq!((counts[0], counts[1]), (4, 2));
            assert!(!monotone);
        } else {
            assert_eq!(counts[4], 77);
            assert!(!bound);
        }
        rows.push(json!({"trace":trace,"polynomial":p,"counts_k1_to5":counts,"passes_N2_ge_N1":monotone,"passes_published_F2_defined_genus4_N5_bound71":bound,"source_bound":"Xarles, arXiv:2007.07822v1, Example16, PDF page10; externally supplied census maximum"}));
    }
    round02::save(
        dir,
        "g4-carrier-bounds.json",
        &json!({"schema":"koblitz.round03/g4-v1","rows":rows,"scope":"dimension-four point-count obstructions; the negative sign uses an external genus4 census bound, not a new independent classification"}),
    );
    println!("G4: t=+1 has N1=4>N2=2; t=-1 has N5=77>published bound71");
}
