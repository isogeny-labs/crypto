//! Write-once round09 records; hashes and identity certificates use native Rust.
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
    let q = read(&raw.join("q7-Frobenius-coordinate-circuits.json"));
    let qb = read(&raw.join("q7b-parsed-descent-chart-check.json"));
    let tr = read(&raw.join("q7c-invariant-rational-generators.json"));
    let l = read(&raw.join("l7-diagonal-twist-reflection-controls.json"));
    let m = read(&raw.join("m12-coupled-function-restrictions.json"));
    assert_eq!(q["records"].as_array().unwrap().len(), 4);
    for (k, n) in [
        ("univariate_class_identities", 48),
        ("exact_slice_identities", 80),
        ("sigma_Miller_identities", 40),
        ("semilinear_iterations", 2880),
        ("old_transition_sigma_checks", 400),
    ] {
        assert_eq!(q[k], n);
    }
    assert_eq!(qb["parsed_univariate_identities"], 48);
    assert_eq!(qb["chart_pair_checks"], 576);
    assert_eq!(qb["parsed_semilinear_iterations"], 2880);
    assert_eq!(tr["trace_pairing_entries_checked"], 129600);
    assert_eq!(tr["invariant_rational_generators"], 4320);
    assert_eq!(tr["coordinate_recoveries"], 16);
    assert_eq!(tr["equivariance_checks"], 128);
    assert_eq!(l["records"].as_array().unwrap().len(), 50);
    assert_eq!(l["simultaneous_stable_and_retained"], 0);
    assert_eq!(m["known_closed_columns_tested"], 720);
    assert_eq!(m["component_function_evaluations"], 40320);
    for f in m["functions"].as_array().unwrap() {
        assert!(f["zero_columns"].as_array().unwrap().is_empty());
    }
    let p = |i: usize| v(&format!("p{i}"));
    let r = |i: usize| v(&format!("q{i}"));
    let pairing = ic::IdentityStatement {
        name: "dual-pairing".into(),
        source:
            "KoblitzRound09.dual_pairing_identity; universal inverse-transpose pairing identity"
                .into(),
        variables: ["p0", "p1", "p2", "p3", "q0", "q1", "q2", "q3"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        degree_bound: 2,
        lhs: E::sum(vec![
            mul(
                mul(c(-1), E::sum((0..4).map(p).collect())),
                mul(c(-1), r(3)),
            ),
            mul(p(0), sub(r(0), r(3))),
            mul(p(1), sub(r(1), r(3))),
            mul(p(2), sub(r(2), r(3))),
        ]),
        rhs: E::sum((0..4).map(|i| mul(p(i), r(i))).collect()),
    };
    let x = v("x");
    let weil=ic::IdentityStatement{name:"weil-five-plus".into(),source:"KoblitzRound09.weil_five_plus; polynomial factorization only, geometric identification separately written".into(),variables:vec!["x".into()],degree_bound:10,
      lhs:mul(E::sum(vec![pow(x.clone(),8),pow(x.clone(),7),mul(c(-1),pow(x.clone(),6)),mul(c(-3),pow(x.clone(),5)),mul(c(-1),pow(x.clone(),4)),mul(c(-6),pow(x.clone(),3)),mul(c(-4),pow(x.clone(),2)),mul(c(8),x.clone()),c(16)]),add(sub(pow(x.clone(),2),x.clone()),c(2))),rhs:add(sub(pow(x.clone(),10),mul(c(11),pow(x,5))),c(32))};
    let mut ids = vec![];
    for (i, statement) in [pairing, weil].into_iter().enumerate() {
        let name = format!("identity-{}.json", statement.name);
        let cert = ic::issue(
            statement,
            2026101909 + i as u64,
            64,
            "same-agent native issuer;separate128 checks;direct Lean universal identity",
            Some(ic::ProofRef {
                kind: "lean4".into(),
                path: "research/koblitz_modular_ic_20261018/lean/KoblitzRound09.lean".into(),
                sha256: hash(&study.join("lean/KoblitzRound09.lean")),
            }),
        )
        .unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((2026101909001 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        round02::save(round, &name, &serde_json::to_value(&cert).unwrap());
        ids.push(json!({"file":name,"id":cert.id,"issuer_points":64,"extra_checker_points":128}));
    }
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    for entry in std::fs::read_dir(study.join("lean")).unwrap() {
        let p = entry.unwrap().path();
        if p.extension().and_then(|s| s.to_str()) == Some("lean") {
            sources.insert(
                p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
        }
    }
    for p in [
        "lean/KoblitzRound09.lean",
        "lean/KoblitzRound08.lean",
        "lean/KoblitzRound07.lean",
        "lean/KoblitzRound07Cofactor.lean",
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "native/Cargo.toml",
        "native/Cargo.lock",
        "rounds/09/PROTOCOL.md",
        "rounds/09/REPLAY.sh",
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
    let ledger = json!({"schema":"koblitz.round09-ledger/v1","parent_commit":"92688745f0ffadf854367ad1d2f33fa05bf7ce2a","date":"2026-10-19 UTC","performance_measured":false,
      "quotients":{"graphs":4,"field_degree":360,"univariate_class_identities":48,"sigma_Miller_identities":40,"exact_slice_identities":80,"semilinear_iterations":2880,"parsed_semilinear_iterations":2880,"chart_pair_checks":576,"old_transition_sigma_checks":400,"explicit_semilinear_coordinate_circuits":true,"cocycle_argument":"written constant-deck argument with two nonzero-root fixtures per graph","invariant_rational_generators_per_graph":4320,"trace_duality_entries":129600,"separability_idempotent_sums":360,"direct_trace_controls":64,"sampled_invariant_values":5760,"coordinate_recoveries":16,"equivariance_checks":128,"additional_point_steps":1440,"F2_polynomial_relations":false,"saturation":false,"closed_projective_embedding":false,"Jacobian_identification":false},
      "low_genus":{"signed_controls":50,"stable_and_known_component_retained":0,"scope":"known augmentation component only;not full Jacobian-factor exclusion","conditional_reflection_genus_upper_bound":1421,"actual_quotient_equations":false},
      "relations":{"coupled_functions":8,"known_columns_tested":720,"component_function_evaluations":40320,"known_zero_columns_each":0,"rank_promoted":false,"retained_evaluated_rank":12,"retained_columns":1308,"retained_nonzero_image_ranks":[11,11],"retained_nonzero_image_columns":[1165,1160],"complete_new_principal_divisors":false},
      "new_named_lean_theorems":17,"audited_declarations":17,"identity_certificates":ids,
      "requirements":[{"track":"remaining determinant19 quotient equations","status":"partial","verified":"explicit dual-base Frobenius circuits,chart compatibility,cocycle and4320 invariant rational generating circuits per graph","open":"F2 polynomial relation ideal,saturation,closed embedding and polarization verification on that model;Jacobian identification"},{"track":"lower-genus carriers","status":"partial","verified":"all50 signed diagonal-twist/reflection controls have no stable quotient retaining the known A5 component","open":"actual reflection quotient equations/genus;other Jacobian factors and other carrier families"},{"track":"direct relations beyond rank11","status":"partial","verified":"eight coupled functions are nonzero on720 known norm columns","open":"outside zero support,multiplicities,poles;no new complete relation or rank"},{"track":"Lean proofs","status":"partial","verified":"17 named kernel proofs and transitive audit;no custom axioms/native trust","open":"native finite-field semantics and global modular/Picard/Galois geometry"},{"track":"reverse transfer and individual descent","status":"partial","verified":"prior corrected odd divisor and independent forward norms remain frozen","open":"general Jacobian reduction and individual descent"},{"track":"IC efficiency","status":"not attempted","reason":"user assesses efficiency;no timings or solver run"}],
      "failed_and_partial_runs":[{"files":["build-01.log","controls-01.log"],"reason":"json array index parsing error;stale binary then rejected output path;no accepted arithmetic output"},{"files":["lean-build-03.log"],"reason":"missing DecidableEq J in generic trace-recovery statement;fixed before final build/audit"},{"files":["lean-build-02.log"],"reason":"preceding append used wrong relative path;this successful build checks previous15-lemma source only"}],
      "raw_sha256":rawhash,"source_sha256":sources,"scope":"same-agent native exact checks,fresh replay and separate parser with shared primitives;written global theory not fully Lean;no external audit or IC speed verdict"});
    round02::save(round, "ledger.json", &ledger);
    println!("Round09:four explicit Frobenius lifts;4320 invariant circuits/graph;50 bounded negative controls;no rank promotion;17 Lean lemmas;two identities64+128 checks");
}
