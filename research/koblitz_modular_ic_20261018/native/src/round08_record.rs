//! Native write-once round08 ledger and Lean-bound identity issuance.
use crate::{identity_certificate as ic, round02, round04::read, sha256_impl};
use ic::Expr as E;
use serde_json::json;
use std::{collections::BTreeMap, path::Path};
fn hash(p: &Path) -> String {
    hex::encode(sha256_impl::sha256(&std::fs::read(p).unwrap()))
}
fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) {
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(root, &p, out)
        } else if !p
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("record-")
        {
            out.insert(
                p.strip_prefix(root).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
        }
    }
}
fn v(s: &str) -> E {
    E::var(s)
}
fn c(n: i64) -> E {
    E::Const(n)
}
fn mul(a: E, b: E) -> E {
    E::times(a, b)
}
fn add(a: E, b: E) -> E {
    E::plus(a, b)
}
fn sub(a: E, b: E) -> E {
    add(a, mul(c(-1), b))
}
fn pow(a: E, n: usize) -> E {
    E::product((0..n).map(|_| a.clone()).collect())
}
pub fn record(round: &Path) {
    let study = round.parent().unwrap().parent().unwrap();
    let repo = study.parent().unwrap().parent().unwrap();
    let raw = round.join("runs");
    let m10 = read(&raw.join("m10-full-line-family.json"));
    let m11 = read(&raw.join("m11-fixed-infinity-row.json"));
    let q = read(&raw.join("q6-intrinsic-Kummer-charts.json"));
    let glue = read(&raw.join("q6c-global-chart-gluing.json"));
    let checks = read(&raw.join("q6d-parsed-gluing-check.json"));
    assert_eq!(m11["evaluated_rank"], 12);
    assert_eq!(m11["total_columns"], 1308);
    for s in m11["nonzero_image_matrices"].as_array().unwrap() {
        assert_eq!(s["rank"], 11)
    }
    assert_eq!(q["functions"], 40);
    assert_eq!(glue["exact_translation_identities"], 200);
    assert_eq!(checks["pointwise_translation_checks"], 400);
    let product=ic::IdentityStatement{name:"binary-function-product".into(),source:"KoblitzRound08.binary_function_product; universal residual exposes exact reduction modulo y²-hy-f".into(),variables:["a","b","c","d","y","h","f"].iter().map(|s|s.to_string()).collect(),degree_bound:4,
 lhs:sub(mul(add(v("a"),mul(v("b"),v("y"))),add(v("c"),mul(v("d"),v("y")))),add(add(mul(v("a"),v("c")),E::product(vec![v("b"),v("d"),v("f")])),mul(E::sum(vec![mul(v("a"),v("d")),mul(v("b"),v("c")),E::product(vec![v("b"),v("d"),v("h")])]),v("y")))),rhs:E::product(vec![v("b"),v("d"),sub(sub(pow(v("y"),2),mul(v("h"),v("y"))),v("f"))])};
    let jac=ic::IdentityStatement{name:"kummer-jacobian-inverse".into(),source:"KoblitzRound08.kummer_jacobian_identity; actual inverse follows only when z^19*d=n and n*e=1".into(),variables:["z","d","n","e"].iter().map(|s|s.to_string()).collect(),degree_bound:21,lhs:sub(mul(mul(pow(v("z"),18),v("d")),mul(v("z"),v("e"))),c(1)),rhs:add(mul(v("e"),sub(mul(pow(v("z"),19),v("d")),v("n"))),sub(mul(v("n"),v("e")),c(1)))};
    let mut ids = vec![];
    for (i, statement) in [product, jac].into_iter().enumerate() {
        let name = format!("identity-{}.json", statement.name);
        let cert = ic::issue(
            statement,
            2026101908 + i as u64,
            64,
            "same-agent native issuer and separate128 evaluation checks; direct Lean ring identity",
            Some(ic::ProofRef {
                kind: "lean4".into(),
                path: "research/koblitz_modular_ic_20261018/lean/KoblitzRound08.lean".into(),
                sha256: hash(&study.join("lean/KoblitzRound08.lean")),
            }),
        )
        .unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((2026101908001 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        round02::save(round, &name, &serde_json::to_value(&cert).unwrap());
        ids.push(json!({"file":name,"id":cert.id,"issuer_points":64,"extra_checker_points":128,"scope":"universal identity;finite fields and global geometry separately checked/written"}));
    }
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    for p in [
        "lean/KoblitzRound08.lean",
        "lean/KoblitzRound07.lean",
        "lean/KoblitzRound07Cofactor.lean",
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "native/Cargo.toml",
        "native/Cargo.lock",
        "rounds/08/PROTOCOL.md",
        "rounds/08/REPLAY.sh",
    ] {
        let p = study.join(p);
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
    }
    for p in [
        "src/cryptanalysis/curve_construction/gf2k.rs",
        "src/cryptanalysis/curve_construction/modular.rs",
        "src/cryptanalysis/curve_construction/weil.rs",
        "src/cryptanalysis/curve_construction/lpoly.rs",
        "src/cryptanalysis/identity_certificate.rs",
        "src/hash/sha256.rs",
    ] {
        sources.insert(p.into(), hash(&repo.join(p)));
    }
    let mut rawhash = BTreeMap::new();
    walk(&raw, &raw, &mut rawhash);
    let ledger = json!({"schema":"koblitz.round08-ledger/v1","parent_commit":"244f1b850ce0d7bc9070b8b07d35e0e0ae3c5023","date":"2026-10-19 UTC","performance_measured":false,
 "relations":{"M10":{"rank":m10["evaluated_rank"],"columns":m10["total_columns"],"nonzero":m10["nonzero_image_matrices"]},"M11":{"rank":m11["evaluated_rank"],"columns":m11["total_columns"],"nonzero":m11["nonzero_image_matrices"]},"additional_columns_after_round07":540,"geometric_joins":0,"new_family_functions":6,"norm_zero_degree_sum":72600,"separate_small_field_and_modular_rank_checks":true},
 "quotients":{"graphs":4,"intrinsic_equations_each_chart":2,"dimension":5,"field_degree":360,"Miller_functions":40,"finite_valuations":720,"pointwise_Miller_checks":640,"covering_charts_each_graph":6,"exact_translation_identities":200,"parsed_translation_identities":200,"exact_curve_translation_identities":100,"pointwise_translation_checks":400,"global_K_atlas":true,"F2_coordinate_descent":false,"closed_projective_embedding":false,"Jacobian_identification":false,"dual_quotient_identification":"written using retained polarized graph certificate and rigidified Kummer/Picard theory"},
 "new_named_lean_theorems":16,"audited_declarations_including_generated_equation":17,"identity_certificates":ids,
 "requirements":[{"track":"more direct modular relations","status":"verified complete for full line-norm family","verified":"rank12/1308;nonzero ranks11 on1165/1160;all supports,joins,images and sums checked","open":"coupled function families,collection costs,individual descent"},{"track":"intrinsic determinant19 quotient equations","status":"partial","verified":"two intrinsic Kummer equations and six covering charts per graph over F2^360;200 exact transitions;written identification with actual polarized quotient","open":"effective F2 coordinate descent and closed projective embedding;Jacobian question remains open"},{"track":"Lean proofs","status":"partial","verified":"16 named results and17 audited declarations;no custom axioms or native trust","open":"complete native finite-field semantics and global modular/Picard geometry"},{"track":"IC efficiency","status":"not attempted","reason":"user assesses efficiency;no timing or solver run"}],
 "failed_and_partial_runs":[{"files":["build-01.log"],"reason":"missing explicit Vec collection type;fixed in build02"},{"files":["m11-metadata-incomplete-attempt.json","infinity-01.log"],"reason":"first M11 function metadata omitted infinity;mathematical output retained and rerun with corrected metadata"}],"raw_sha256":rawhash,"source_sha256":sources,"scope":"same-agent exact construction,fresh replay and separate parsers/small-field checks;shared primitives;no external audit;global theory written;no IC speed verdict"});
    round02::save(round, "ledger.json", &ledger);
    println!("Round08:rank12/1308,nonzero11both;four global six-chart Kummer atlases;16 named Lean results;two identities64+128 checks");
}
