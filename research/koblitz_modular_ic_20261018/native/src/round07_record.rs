//! Write-once round07 ledger and Lean-bound universal identity certificates.
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
            walk(root, &p, out);
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
fn c(n: i64) -> E {
    E::Const(n)
}
fn v(s: &str) -> E {
    E::var(s)
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
    let even = read(&raw.join("n4-even-independent-forward.json"));
    let odd = read(&raw.join("odd-verified/o4-odd-reverse-divisor.json"));
    let first = read(&raw.join("m8-full-map-principal-row.json"));
    let next = read(&raw.join("m9-second-full-map-row.json"));
    let quotient = read(&raw.join("q5-mixed-translation-atlases.json"));
    let cover = read(&raw.join("q5b-expanded-cofactor-cover.json"));
    assert_eq!(even["equals_source"], true);
    assert_eq!(even["Gram_used_to_compute_norms"], false);
    assert_eq!(odd["equals_source"], true);
    assert_eq!(odd["Gram_used_for_forward"], false);
    assert_eq!(odd["terms"].as_array().unwrap().len(), 20);
    assert_eq!(odd["point_algebra_entries"], 2200);
    assert_eq!(next["evaluated_rank"], 7);
    assert_eq!(next["total_columns"], 768);
    for row in next["nonzero_image_matrices"].as_array().unwrap() {
        assert_eq!(row["rank"], 6);
    }
    assert_eq!(quotient["records"].as_array().unwrap().len(), 4);
    assert_eq!(cover["exact_curve_identity"], true);
    assert_eq!(cover["kernel_checks"], 722);
    let line = ic::IdentityStatement {
        name: "deck-line-transport".into(),
        source: "KoblitzRound07.deck_line_identity: universal coefficient identity; actual level geometry checked separately".into(),
        variables: ["a", "b", "u", "p", "q"].iter().map(|s| s.to_string()).collect(),
        degree_bound: 3,
        lhs: add(mul(mul(c(-1), v("a")), v("q")), mul(v("b"), add(mul(v("u"), v("p")), v("q")))),
        rhs: add(mul(mul(v("b"), v("u")), v("p")), mul(sub(v("b"), v("a")), v("q"))),
    };
    let source = sub(
        add(pow(v("y"), 2), v("y")),
        E::sum(vec![pow(v("x"), 3), v("x"), v("c")]),
    );
    let coordinate = sub(
        E::sum(vec![pow(v("x"), 3), v("x"), pow(v("r"), 2), v("r")]),
        add(pow(v("X"), 3), v("X")),
    );
    let lift = ic::IdentityStatement {
        name: "supersingular-coordinate-lift".into(),
        source: "KoblitzRound07.cofactor_coordinate_lift: exact integer identity; specific polynomial masks and characteristic2 implication separately kernel checked".into(),
        variables: ["x", "y", "X", "r", "c"].iter().map(|s| s.to_string()).collect(),
        degree_bound: 3,
        lhs: sub(add(pow(add(v("y"), v("r")), 2), add(v("y"), v("r"))), E::sum(vec![pow(v("X"), 3), v("X"), v("c")])),
        rhs: E::sum(vec![source, coordinate, E::product(vec![c(2), v("y"), v("r")])]),
    };
    let mut ids = vec![];
    for (i, statement) in [line, lift].into_iter().enumerate() {
        let name = format!("identity-{}.json", statement.name);
        let cert = ic::issue(
            statement,
            2026101907 + i as u64,
            64,
            "same-agent issuer and separate128 evaluation checks; direct universal Lean identities",
            Some(ic::ProofRef {
                kind: "lean4".into(),
                path: "research/koblitz_modular_ic_20261018/lean/KoblitzRound07.lean".into(),
                sha256: hash(&study.join("lean/KoblitzRound07.lean")),
            }),
        )
        .unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((2026101907001 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        round02::save(round, &name, &serde_json::to_value(&cert).unwrap());
        ids.push(json!({"file": name, "id": cert.id, "issuer_points": 64, "extra_checker_points": 128, "Lean_scope": "universal ring identity; geometric and binary-mask interpretation bridges written"}));
    }
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    for p in [
        "lean/KoblitzRound07.lean",
        "lean/KoblitzRound07Cofactor.lean",
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "native/Cargo.toml",
        "native/Cargo.lock",
        "rounds/07/PROTOCOL.md",
        "rounds/07/REPLAY.sh",
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
    let ledger = json!({
      "schema":"koblitz.round07-ledger/v1", "parent_commit":"b582a94c2b9275e538218b55c0eff67fd5f18a80", "date":"2026-10-19", "performance_measured":false,
      "even_forward":{"checked_entries":even["checked_coordinate_entries"],"equals_source":true,"Gram_used_for_computation":false},
      "odd_reverse":{"terms":20,"point_algebra_entries":2200,"positive_degree":6600,"negative_degree":6600,"split_points":55,"degree11_components":55,"F2_formal_divisor_descent":true,"independent_forward_equals_source":true,"Gram_used_for_computation":false,"Jacobian_reduction":false,"higher11_power_claim":false},
      "relations":{"after_first":{"rank":first["evaluated_rank"],"columns":first["total_evaluated_columns"],"nonzero":first["nonzero_image_matrices"]},"after_second":{"rank":next["evaluated_rank"],"columns":next["total_columns"],"nonzero":next["nonzero_image_matrices"]},"new_closed_columns":[60,120],"new_degrees":[60,660],"independent_small_field_checks":true,"complete_support_geometric_bridges":"PROOF_MAP.md; actual basis and Galois steps checked in m9c-geometric-base-check.json"},
      "quotients":{"mixed_graph_atlases":4,"actual_kernel_points":1444,"field_degree":360,"cofactor_traces":[-2,2],"expanded_cofactor_cover":true,"cover_degrees":cover["degrees"],"cover_kernel_checks":722,"pointwise_scalar_checks":64,"exact_binary_polynomial_certificate":true,"intrinsic_mixed_image_ideal":false,"Jacobian_claim":false},
      "new_named_lean_theorems":17,"audited_declarations_including_generated_equation":18,"identity_certificates":ids,
      "requirements":[
        {"track":"independent forward norms","status":"verified complete","verified":"actual target reconstruction and traces on all7200 even divisor entries return source without Gram","open":"reduced Jacobian arithmetic"},
        {"track":"corrected odd reverse divisor","status":"verified complete for chosen11-torsion fixture","verified":"all20 pullbacks,2200 entries,formal F2 descent,independent forward equality","open":"general reduced Jacobian arithmetic and higher11-power fixtures"},
        {"track":"expanded determinant19 quotient equations","status":"partial","verified":"four actual mixed quotient atlases and expanded supersingular[19] cofactor-cover equations","open":"intrinsic mixed fivefold invariant image ideal,full chart elimination,no Jacobian equation","next":"eliminate translation invariant coordinates on stable charts and verify chart gluing and F2 descent"},
        {"track":"relations beyond nonzero-image rank5","status":"verified complete for two new full-map norm rows","verified":"all-column rank7/768;nonzero-image rank6 on675/678 columns;independent finite-field checks","open":"further families,individual descent and collection costs"},
        {"track":"Lean proofs","status":"partial","verified":"17 named results,18 audited declarations,no native trust or custom axioms","open":"global modular/Picard geometry and binary-mask-to-polynomial interpretation"},
        {"track":"IC efficiency","status":"not attempted","reason":"user assesses efficiency; no timing or destination solver experiment"}
      ],
      "failed_and_partial_runs":[
        {"files":["build-04.log"],"reason":"BigInt type inference in order computation; explicit type fixed"},
        {"files":["quotients-01.log"],"reason":"affine invariant sample hit infinity; corrected sample excludes19-kernel translates"},
        {"files":["odd-screen-01.log","odd-screen-02.log"],"reason":"inherited wrong q-mod11 entry and degree5 embedding; interrupted by tool reset; no accepted fibers"},
        {"files":["odd-screen-03.log"],"reason":"corrected matrix but stale embedding; interrupted by tool reset; no accepted fibers"},
        {"files":["odd-screen-04.log","odd-stale-attempts.txt"],"reason":"Node executable absent after reset; source fix did not run; stale probe explicitly stopped"},
        {"files":["build-09.log"],"reason":"tool reset interrupted compilation"}
      ],
      "raw_sha256":rawhash,"source_sha256":sources,
      "scope":"same-agent construction,fresh byte-identical replay,separate parsers/small-field checks sharing declared primitives;no external audit;geometric bridges partly written;no IC speed verdict"
    });
    round02::save(round, "ledger.json", &ledger);
    println!("Round07 sealed:rank7/768,nonzero6/675,6/678;20 odd fibers;4 mixed atlases;17 named Lean results;2 identities64+128 checks");
}
