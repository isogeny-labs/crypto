//! Write-once round13 ledger, native hashes and Lean-source-bound certificates.
use crate::{identity_certificate as ic, round02, round04::read, sha256_impl};
use ic::Expr as E;
use serde_json::json;
use std::{collections::BTreeMap, path::Path};
#[path = "round14_record.rs"]
pub(crate) mod round14;
#[path = "round15_record.rs"]
pub(crate) mod round15;
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
fn add(a: E, b: E) -> E {
    E::plus(a, b)
}
fn mul(a: E, b: E) -> E {
    E::times(a, b)
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
    let readraw = |s: &str| read(&raw.join(s));
    let f = readraw("o16-ordinary-source-fibers.json");
    let images = readraw("o17-ordinary-projected-images.json");
    let parsed = readraw("o18-parsed-function-and-fiber-check.json");
    let q = readraw("q13-inverse-associative-saturation.json");
    let pair = readraw("q14-two-product-colon.json");
    let sq = readraw("q15-linear-square-saturation.json");
    assert_eq!(f["records"].as_array().unwrap().len(), 6);
    assert_eq!(images["new_nonzero_11_primary_columns"], 0);
    assert_eq!(parsed["ordinary_source_degrees"], json!([500, 580]));
    for g in 0..4 {
        assert!(!pair["records"][g]["counterpoint"].is_null());
        assert!(!sq["records"][g]["counterpoint_after_all_Q14"].is_null());
    }
    let norm = ic::IdentityStatement {
        name: "quadratic-norm-residual".into(),
        source: "KoblitzRound13.quadratic_norm_residual".into(),
        variables: ["a", "b", "x", "y", "d"].map(str::to_string).to_vec(),
        degree_bound: 4,
        lhs: sub(
            mul(
                add(v("a"), mul(v("b"), v("y"))),
                add(v("a"), mul(v("b"), sub(mul(c(-1), v("x")), v("y")))),
            ),
            sub(
                sub(pow(v("a"), 2), mul(mul(v("a"), v("b")), v("x"))),
                mul(pow(v("b"), 2), v("d")),
            ),
        ),
        rhs: mul(
            mul(c(-1), pow(v("b"), 2)),
            sub(add(pow(v("y"), 2), mul(v("x"), v("y"))), v("d")),
        ),
    };
    let inverse = ic::IdentityStatement {
        name: "inverse-cancellation-saturation".into(),
        source: "KoblitzRound13.inverse_cancellation_saturation".into(),
        variables: ["h", "out", "a", "b", "z"].map(str::to_string).to_vec(),
        degree_bound: 3,
        lhs: mul(v("h"), sub(mul(v("out"), v("z")), mul(v("b"), v("h")))),
        rhs: add(
            mul(sub(mul(v("h"), v("out")), mul(v("a"), v("b"))), v("z")),
            mul(v("b"), sub(mul(v("a"), v("z")), pow(v("h"), 2))),
        ),
    };
    let shared = ic::IdentityStatement {
        name: "shared-factor-saturation".into(),
        source: "KoblitzRound13.shared_factor_saturation".into(),
        variables: ["h", "out", "other", "a", "b", "c"]
            .map(str::to_string)
            .to_vec(),
        degree_bound: 3,
        lhs: mul(v("h"), sub(mul(v("out"), v("c")), mul(v("other"), v("b")))),
        rhs: sub(
            mul(sub(mul(v("h"), v("out")), mul(v("a"), v("b"))), v("c")),
            mul(sub(mul(v("h"), v("other")), mul(v("a"), v("c"))), v("b")),
        ),
    };
    let mut ids = vec![];
    for (i, statement) in [norm, inverse, shared].into_iter().enumerate() {
        let name = format!("identity-{}.json", statement.name);
        let cert = ic::issue(
            statement,
            202610201300 + i as u64,
            64,
            "native issuer plus128 extra checks;source-bound universal Lean identity",
            Some(ic::ProofRef {
                kind: "lean4".into(),
                path: "research/koblitz_modular_ic_20261018/lean/KoblitzRound13.lean".into(),
                sha256: hash(&study.join("lean/KoblitzRound13.lean")),
            }),
        )
        .unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((20261020130001 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        round02::save(round, &name, &serde_json::to_value(&cert).unwrap());
        ids.push(json!({"file":name,"id":cert.id,"issuer_points":64,"extra_checker_points":128}));
    }
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    for e in std::fs::read_dir(study.join("lean")).unwrap() {
        let p = e.unwrap().path();
        if p.extension().and_then(|x| x.to_str()) == Some("lean") {
            sources.insert(
                p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
        }
    }
    for p in [
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "native/Cargo.toml",
        "native/Cargo.lock",
        "rounds/13/PROTOCOL.md",
        "rounds/13/REPLAY.sh",
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
    let audit = std::fs::read_to_string(raw.join("lean-full-audit.log")).unwrap();
    let audited: u64 = audit
        .lines()
        .find(|l| l.starts_with("audited "))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    assert!(audited >= 269);
    let mut hashes = BTreeMap::new();
    walk(&raw, &raw, &mut hashes);
    let ledger = json!({"schema":"koblitz.round13-ledger/v1","UTC_date":"2026-10-20","client_date":"2026-10-20 America/Los_Angeles","parent_commit":"7fde2b9498ec30ea2e111f1b178ae000e465331d","performance_measured":false,"ordinary":{"norm_poles":[504,528],"q_precision":640,"input_precision":77440,"pushed_degrees":[100,116],"pushed_closed_degrees":[[50,50],[29,29,29,29]],"source_closed_degrees":[[250,250],[145,145,145,145]],"source_ordinary_degrees":[500,580],"all_multiplicities":1,"fiber_extension_degree":5,"cyclic121_Hecke_degree":132,"commuting_squares":24,"O16_binary_reductions":321809763,"O18":parsed,"projected_images":images,"complete_under_written_moduli_norm_pairing_bridge":true,"pointwise_projection_coordinates_computed":false,"whole_principal_rows_assembled":false,"other_ordinary_degrees_open":[1760,1880,2760,1370,2180]},"saturation":{"inverse_quadrics":q["records"].as_array().unwrap().iter().map(|r|r["inverse_relations"].as_array().unwrap().len()).collect::<Vec<_>>(),"shared_factor_quadrics":q["records"].as_array().unwrap().iter().map(|r|r["shared_factor_relations"].as_array().unwrap().len()).collect::<Vec<_>>(),"three_leaf_linear_family_empty":true,"Q14_relations_per_graph":4,"Q15_relations_per_graph":5,"Q15_maximum_h_power":5,"counterpoint_after_all_Q14_per_graph":true,"nonzero_boundary_models_after_all_Q15_per_graph":true,"last_models_prove_incomplete_saturation":false,"full_h_saturation":false,"proper_quotient_embedding":false},"retained_rank":{"evaluated":12,"columns":1308,"nonzero_image_rank":11,"nonzero_columns":[1165,1160],"promoted":false},"new_named_lean_theorems":63,"actual_circuit_finite_certificates":32,"audited_project_theorems":audited,"identity_certificates":ids,"requirements":[{"track":"ordinary antidiagonal500","status":"verified complete under written bridge","evidence":"O13 through O18;two simple degree250 points,both projected images zero","open":"global modular and native semantics fully formalized"},{"track":"ordinary product-plus-one580","status":"verified complete under written bridge","evidence":"O13 through O18;four simple degree145 points,both projected images zero","open":"global modular and native semantics fully formalized"},{"track":"full quotient saturation","status":"partial","evidence":"four stages of actual counterpoints and certified inverse/shared/four-leaf/square relations","open":"whole h-saturated ideals,boundary gluing and proper polarized embeddings"},{"track":"complete coupled relations","status":"partial","evidence":"two requested ordinary components exhausted;zero projected images","open":"assemble whole principal rows and remaining five ordinary supports;rank unchanged"},{"track":"Lean proofs","status":"partial","evidence":"63 named checked structural,actual circuit and bit-checker results","open":"Rabin checker semantics and theorem,full modular geometry,finite algebra/pairing identification"},{"track":"IC efficiency","status":"not attempted","reason":"user assesses;no timing or efficiency verdict"}],"failed_and_partial_runs":[{"files":["support-01.log"],"reason":"field inverse initialized with a rather than1;exact multiplication check rejected;fixed and exhaustive F16 inverse/AS controls added"},{"files":["fibers-01.log"],"reason":"100m reduction cap after first degree250 fiber;AmendmentB froze1b O16 cap before repeat"},{"files":["build-02.log","build-10.log"],"reason":"Rust syntax/import errors;retained and fixed"},{"files":["lean-build-01.log","lean-build-02.log","lean-data-build-01.log","lean-data-build-02.log","lean-square-build-01.log","lean-square-build-02.log"],"reason":"elaboration,generated nested array indexing,and characteristic2 normalization failures;only final passing builds counted"},{"files":["lean-boundary-build-01.log","lean-boundary-build-02.log","lean-boundary-attempts.txt"],"reason":"first build incomplete after runtime continuation;second explicitly interrupted above28GB observed RSS;List representation preserves all data and completes kernel proofs"}],"raw_sha256":hashes,"source_sha256":sources,"scope":"native producer and parsed verifier share arithmetic primitives;deterministic replay and partial kernel proofs;written global modular bridges;no external implementation audit or IC efficiency claim"});
    round02::save(round, "ledger.json", &ledger);
    println!("Round13:ordinary500/580 complete under written bridges,all6 projected images zero;four saturation stages,full saturation open;63 new named Lean proofs,{audited} audited");
}

#[path = "round16_record.rs"]
pub(crate) mod round16;

#[path = "round17_record.rs"]
pub(crate) mod round17;

#[path = "round18_record.rs"]
pub(crate) mod round18;
