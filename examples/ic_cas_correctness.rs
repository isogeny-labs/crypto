//! Same-input finite Boolean-variety check; no fastest-solver claim.
use crypto_lib::cryptanalysis::koblitz_groebner::{
    f4_word_ops_thread, solve_boolean_system, sym_semaev_s4, FieldStructure, SolveOptions,
    SolverEngine, SplitRule, SymElement,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::path::Path;

#[allow(dead_code)]
#[path = "../src/bin/ecbench/structured_s4_math.rs"]
mod math;

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn poly_string(p: &F2BoolPoly) -> String {
    if p.is_zero() {
        return "0".into();
    }
    p.terms
        .iter()
        .map(|t| {
            let factors: Vec<_> = (0..p.n_vars)
                .filter(|&i| t.mask >> i & 1 != 0)
                .map(|i| format!("x{}", i + 1))
                .collect();
            if factors.is_empty() {
                "1".into()
            } else {
                factors.join("*")
            }
        })
        .collect::<Vec<_>>()
        .join("+")
}
fn parse_poly(s: &str, nv: usize) -> F2BoolPoly {
    let mut terms = vec![];
    for term in s.trim().replace(' ', "").split('+') {
        if term == "0" {
            continue;
        }
        assert!(!term.is_empty(), "empty polynomial term");
        let mut mask = 0u64;
        for factor in term.split('*') {
            if factor == "1" {
                continue;
            }
            let variable = factor.strip_prefix('x').expect("variable name");
            let (variable, exponent) = variable.split_once('^').unwrap_or((variable, "1"));
            let variable: usize = variable.parse().expect("variable index");
            assert!((1..=nv).contains(&variable));
            assert!(exponent.parse::<u32>().unwrap() > 0);
            mask |= 1 << (variable - 1);
        }
        terms.push(F2BoolMono::from_mask(mask));
    }
    F2BoolPoly::from_monos(terms, nv)
}
fn zeros(eqs: &[F2BoolPoly], nv: usize) -> Vec<u64> {
    assert!(nv <= 20);
    (0..1u64 << nv)
        .filter(|&x| eqs.iter().all(|p| p.eval(x) == 0))
        .collect()
}
fn equations(row: &Value) -> (Vec<F2BoolPoly>, usize) {
    let n = row["field_degree"].as_u64().unwrap() as u32;
    let xr = row["target"]["x"].as_u64().unwrap();
    let bs: Vec<u64> = serde_json::from_value(row["basis"].clone()).unwrap();
    let irr = find_irreducible_sparse(n).unwrap();
    assert_eq!(row["field_modulus"], Gf2::new(&irr).irr);
    let st = FieldStructure::new(n, &irr);
    if row["formulation"] == "symmetric-e" {
        let (eqs, ps) = math::symmetric_system(&Gf2::new(&irr), &bs, xr, &st).unwrap();
        return (eqs, ps.iter().map(Vec::len).sum());
    }
    assert_eq!(row["formulation"], "eliminated-s4");
    let nv = 3 * bs.len();
    let base: Vec<_> = bs.iter().map(|&x| math::element(x, n)).collect();
    let xs: Vec<_> = (0..3)
        .map(|i| SymElement::from_subspace_vars(&base, i * bs.len(), n, nv))
        .collect();
    (
        sym_semaev_s4(&xs[0], &xs[1], &xs[2], &math::element(xr, n), &st),
        nv,
    )
}
fn export(study: &Path, out: &Path) {
    assert!(!out.exists(), "export directory must be fresh");
    std::fs::create_dir_all(out).unwrap();
    let panel = study.join("runs/structural-d2-20261023-01");
    let mut selected = vec![];
    let mut dirs: Vec<_> = std::fs::read_dir(&panel)
        .unwrap()
        .map(Result::unwrap)
        .collect();
    dirs.sort_by_key(|e| e.file_name());
    for d in dirs {
        let p = d.path().join("rows.jsonl");
        if !p.exists() {
            continue;
        }
        for line in std::fs::read_to_string(&p).unwrap().lines() {
            let row: Value = serde_json::from_str(line).unwrap();
            let n = row["field_degree"].as_u64().unwrap();
            let ell = row["ell"].as_u64().unwrap();
            if [13, 19].contains(&n)
                && (ell == 2 || (ell == 3 && row["family"] == "geometric"))
                && row["base_rep"] == 0
                && row["target_index"] == 0
                && (row["formulation"] == "eliminated-s4" || row["formulation"] == "symmetric-e")
            {
                selected.push((row, p.clone(), sha(line.as_bytes()), false));
            }
        }
    }
    assert_eq!(selected.len(), 20);
    for form in ["eliminated-s4", "symmetric-e"] {
        let (mut row, path, digest, _) = selected
            .iter()
            .find(|(r, _, _, _)| {
                r["field_degree"] == 13 && r["family"] == "prefix" && r["formulation"] == form
            })
            .unwrap()
            .clone();
        row["target"]["x"] = row["basis"][0].clone();
        selected.push((row, path, digest, true));
    }
    let mut cases = vec![];
    for (index, (row, source, source_row_sha, positive)) in selected.into_iter().enumerate() {
        let name = format!("case-{index:02}");
        let (eqs, nv) = equations(&row);
        let eqhash = sha(&serde_json::to_vec(&eqs).unwrap());
        if !positive {
            assert_eq!(eqhash, row["equations_sha256"]);
            assert_eq!(row["extraction_complete"], true);
        }
        let expected = zeros(&eqs, nv);
        if positive {
            assert!(!expected.is_empty());
        } else {
            assert_eq!(
                expected.len() as u64,
                row["algebraic_roots"].as_u64().unwrap()
            );
        }
        let before = f4_word_ops_thread();
        let (mut roots, stats) = solve_boolean_system(
            &eqs,
            nv,
            &SolveOptions {
                engine: SolverEngine::InheritedF4 { max_degree: 4 },
                max_solutions: 4096,
                node_budget: 20_000,
                split_rule: SplitRule::HighestFree,
            },
        );
        let words = f4_word_ops_thread().wrapping_sub(before);
        roots.sort_unstable();
        assert!(!stats.exhausted && roots.len() < 4096);
        assert_eq!(roots, expected);
        let vars = (1..=nv)
            .map(|i| format!("x{i}"))
            .collect::<Vec<_>>()
            .join(",");
        let mut input = eqs.iter().map(poly_string).collect::<Vec<_>>();
        input.extend((1..=nv).map(|i| format!("x{i}^2+x{i}")));
        for algorithm in ["std", "slimgb"] {
            let mut script = format!("ring r=2,({vars}),dp;\noption(redSB);\nshort=0;\nideal I={};\nideal G={algorithm}(I);\nprint(\"CASE {name}\");\nprint(\"ALG {algorithm}\");\nideal N=reduce(I,G);\nprint(\"NF_OK \"+string(size(N)==0));\n",input.join(",\n"));
            writeln!(script,"for(int i=1;i<=size(G);i++){{print(\"POLY \"+string(G[i]));}}\nprint(\"END\");\nquit;").unwrap();
            std::fs::write(out.join(format!("{name}-{algorithm}.sing")), script).unwrap();
        }
        cases.push(
            json!({"case":name,"source_path":source,"source_row_sha256":source_row_sha,
            "constructed_positive_algebraic_control":positive,"source":row,"n_vars":nv,
            "equations_sha256":eqhash,"equations":eqs,"boolean_zero_set":expected,
            "native_extraction_word_ops":words,"native_stats":stats}),
        );
    }
    std::fs::write(
        out.join("manifest.json"),
        serde_json::to_vec_pretty(&json!({
        "schema":"ic-cas-correctness-inputs/v1","cases":cases,
        "scope":"Finite Boolean correctness only; no performance comparison."}))
        .unwrap(),
    )
    .unwrap();
    println!("Exported 22 cases, with native extraction equal to exhaustive Boolean enumeration.");
}
fn verify(dir: &Path) {
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(dir.join("manifest.json")).unwrap()).unwrap();
    let mut results = vec![];
    let mut failed = 0u64;
    for case in manifest["cases"].as_array().unwrap() {
        let name = case["case"].as_str().unwrap();
        let nv = case["n_vars"].as_u64().unwrap() as usize;
        let expected: Vec<u64> = serde_json::from_value(case["boolean_zero_set"].clone()).unwrap();
        let (rebuilt, rebuilt_nv) = equations(&case["source"]);
        assert_eq!(nv, rebuilt_nv);
        assert_eq!(
            sha(&serde_json::to_vec(&rebuilt).unwrap()),
            case["equations_sha256"]
        );
        assert_eq!(zeros(&rebuilt, nv), expected);
        let mut input = rebuilt.iter().map(poly_string).collect::<Vec<_>>();
        input.extend((1..=nv).map(|i| format!("x{i}^2+x{i}")));
        for algorithm in ["std", "slimgb"] {
            let prefix = dir.join(format!("{name}-{algorithm}"));
            let script = std::fs::read_to_string(prefix.with_extension("sing")).unwrap();
            assert!(script.contains(&format!("ideal I={};", input.join(",\n"))));
            let status: Value = serde_json::from_slice(
                &std::fs::read(prefix.with_extension("status.json")).unwrap(),
            )
            .unwrap();
            let stdout = std::fs::read_to_string(prefix.with_extension("stdout.txt")).unwrap();
            let markers = [
                format!("CASE {name}"),
                format!("ALG {algorithm}"),
                "NF_OK 1".into(),
                "END".into(),
            ];
            if status["exit_code"] != 0
                || markers
                    .iter()
                    .any(|m| !stdout.lines().any(|l| l.trim() == m))
            {
                failed += 1;
                results.push(json!({"case":name,"algorithm":algorithm,"status":"external_failed_or_incomplete","process":status}));
                continue;
            }
            let basis: Vec<_> = stdout
                .lines()
                .filter_map(|l| l.strip_prefix("POLY "))
                .map(|s| parse_poly(s, nv))
                .collect();
            assert!(!basis.is_empty(), "external basis missing");
            let actual = zeros(&basis, nv);
            assert_eq!(
                actual, expected,
                "external zero set differs for {name} {algorithm}"
            );
            results.push(json!({"case":name,"algorithm":algorithm,"status":"same_boolean_zero_set",
                "normal_forms_zero":true,"basis_polynomials":basis.len(),"boolean_roots":actual.len(),
                "stdout_sha256":sha(stdout.as_bytes()),"equations_sha256":case["equations_sha256"]}));
        }
    }
    assert_eq!(results.len(), 44);
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"ic-cas-correctness-replay/v1",
        "status":if failed==0{"all_external_zero_sets_equal"}else{"partial_external_results"},
        "manifest_sha256":sha(&std::fs::read(dir.join("manifest.json")).unwrap()),
        "cases":22,"planned_external_runs":44,"failed_or_incomplete":failed,"results":results,
        "scope":"Same-input finite Boolean-variety correctness only; no speed ratio or fastest-tool claim."})).unwrap());
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("export") if args.len() == 3 => export(Path::new(&args[1]), Path::new(&args[2])),
        Some("verify") if args.len() == 2 => verify(Path::new(&args[1])),
        _ => panic!("export STUDY FRESH_DIR | verify DIR"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn external_polynomial_parser_respects_boolean_reduction() {
        let p = parse_poly("x1^2+x1*x2+x1+1", 2);
        assert_eq!(zeros(&[p], 2), vec![3]);
        let p = parse_poly("x1^2+x1", 2);
        assert!(p.is_zero());
    }
    #[test]
    fn export_syntax_preserves_every_boolean_value() {
        let p = F2BoolPoly::from_monos(
            vec![
                F2BoolMono::one(),
                F2BoolMono::var(0),
                F2BoolMono::from_mask(6),
            ],
            3,
        );
        let restored = parse_poly(&poly_string(&p), 3);
        assert_eq!(
            (0..8).map(|x| p.eval(x)).collect::<Vec<_>>(),
            (0..8).map(|x| restored.eval(x)).collect::<Vec<_>>()
        );
        assert_eq!(
            zeros(&[restored], 3).into_iter().collect::<BTreeSet<_>>(),
            [1, 3, 5, 6].into_iter().collect()
        );
    }
}
