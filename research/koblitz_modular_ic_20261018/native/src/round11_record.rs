//! Write-once round11 ledger, native hashes and Lean-source-bound certificates.
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
    let local = read(&raw.join("m16-local-diagonal-contacts.json"));
    let parsed = read(&raw.join("m16b-parsed-local-check.json"));
    let sat = read(&raw.join("q9-coupled-projective-saturation.json"));
    let rel = read(&raw.join("m17-residual-correspondences.json"));
    assert_eq!(local["weighted_contact_sum"], 860);
    assert_eq!(parsed["weighted_degree"], 860);
    assert_eq!(sat["saturated_Groebner_basis"].as_array().unwrap().len(), 5);
    let sat_id = ic::IdentityStatement {
        name: "coupled-saturation-other-inverse".into(),
        source: "KoblitzRound11.saturation_other_inverse".into(),
        variables: ["x", "z", "y", "v", "h"].map(str::to_string).to_vec(),
        degree_bound: 3,
        lhs: mul(v("h"), sub(mul(v("z"), v("v")), mul(v("x"), v("h")))),
        rhs: sub(
            mul(v("x"), sub(mul(v("y"), v("v")), pow(v("h"), 2))),
            mul(v("v"), sub(mul(v("x"), v("y")), mul(v("z"), v("h")))),
        ),
    };
    let curve = |x: E| {
        sub(
            sub(
                sub(
                    add(pow(v("y"), 2), mul(x.clone(), v("y"))),
                    pow(x.clone(), 3),
                ),
                pow(x, 2),
            ),
            c(1),
        )
    };
    let ey = ic::IdentityStatement {
        name: "equal-y-correspondence".into(),
        source: "KoblitzRound11.equal_y_factorization".into(),
        variables: ["x", "u", "y"].map(str::to_string).to_vec(),
        degree_bound: 3,
        lhs: sub(curve(v("u")), curve(v("x"))),
        rhs: mul(
            sub(v("u"), v("x")),
            sub(
                sub(
                    sub(
                        sub(sub(v("y"), pow(v("u"), 2)), mul(v("u"), v("x"))),
                        pow(v("x"), 2),
                    ),
                    v("u"),
                ),
                v("x"),
            ),
        ),
    };
    let mut ids = vec![];
    for (i, s) in [sat_id, ey].into_iter().enumerate() {
        let name = format!("identity-{}.json", s.name);
        let cert = ic::issue(
            s,
            202610191100 + i as u64,
            64,
            "same-agent native issuance plus128 extra checks;universal Lean identity",
            Some(ic::ProofRef {
                kind: "lean4".into(),
                path: "research/koblitz_modular_ic_20261018/lean/KoblitzRound11.lean".into(),
                sha256: hash(&study.join("lean/KoblitzRound11.lean")),
            }),
        )
        .unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((20261019110001 + i as u64, 128))).unwrap(),
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
        if p.extension().and_then(|s| s.to_str()) == Some("lean") {
            sources.insert(
                p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
        }
    }
    for s in [
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "native/Cargo.toml",
        "native/Cargo.lock",
        "rounds/11/PROTOCOL.md",
        "rounds/11/REPLAY.sh",
    ] {
        let p = study.join(s);
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
    let ledger = json!({"schema":"koblitz.round11-ledger/v1","date":"2026-10-19 UTC","parent_commit":"33db29ba4bcf5f72f5fe8d908b0fb80c0f44d7bc","performance_measured":false,"diagonal":{"geometric_labels":400,"closed_points":40,"closed_degrees":10,"contact_histogram":local["contact_histogram"],"trace_histogram":local["trace_histogram"],"old_lower_sum":400,"weighted_degree":860,"old_gap":460,"remaining_gap":0,"precision":32,"actual_11_torsion_points":121,"simple_root_series":60,"quaternion_Velu_point_checks":12936,"unit_binding_point_checks":26136,"parsed_root_coefficients":1920,"parsed_transformation_coefficients":51200,"complete_under_written_moduli_bridge":true,"explicit49_torsion_coordinates":false,"antidiagonal_remaining_degree":1240,"full_xsum_divisor_complete":false},"saturation":{"generators":5,"S_pairs":10,"charts":6,"F2_boundary_points":10,"naive_false_points":5,"regular_initial_h":true,"scope":"coupled monomial multiplication and two-inverse graph only","full_quotient_saturation":false,"proper_quotient_embedding":false},"residual_correspondences":{"point_controls":rel["exact_point_controls"],"xsum_plus_one_ambient_genus":3,"translated_product_fiber_degrees":[1280,1360],"equal_y_residual_degree":2680,"full_noncuspidal_divisors_complete":0,"remaining_noncuspidal_divisors":7,"rank_promoted":false,"retained_rank":12,"retained_columns":1308,"retained_nonzero_image_rank":11},"new_named_lean_theorems":22,"identity_certificates":ids,"requirements":[{"track":"projective saturation","status":"partial","verified":"five-quadric saturated coupled graph closure with10 S-pairs,exact membership and six affine charts","open":"full four quotient homogeneous ideals and global boundaries"},{"track":"closed embeddings","status":"partial","verified":"six-chart projective embedding of the controlled coupled graph;four previously proved quotient affine open embeddings retained","open":"whole proper polarized quotient models"},{"track":"seven noncuspidal divisors","status":"partial","verified":"complete degree860 diagonal sub-divisor;translated product fibers,genus3 ambient correspondence,equal-y residual curve","open":"antidiagonal degree1240 and all seven full zero supports;no new evaluated relation row"},{"track":"460 diagonal gap","status":"verified complete under written moduli bridge","verified":"260 contacts1,100 contacts2,20 contacts6,20 contacts14;weighted sum860;actual11-torsion and quaternion/Velu binding;Frobenius-stable40 closed points","open":"explicit49 torsion point coordinates and full Lean moduli/deformation construction"},{"track":"Lean proofs","status":"partial","verified":"22 named universal algebra,chart,Weierstrass,AS and degree results;no custom axioms/native trust","open":"native arithmetic semantics,universal deformation/level-rigidity and global geometry remain written"},{"track":"IC efficiency","status":"not attempted","reason":"user assesses;no timings or solver run"}],"failures":[{"files":["build-01.log"],"reason":"unbalanced residual expression delimiter;corrected"},{"files":["saturation-01.log","saturation-02.log"],"reason":"lex order did not produce an h-free leading ideal;recomputed in grevlex"},{"files":["local-03.log","m16-local-contacts-attempt-01.json"],"reason":"write-once save rejected overwriting preliminary export;retained original and reran full final export"},{"files":["lean-build-01.log"],"reason":"lake invoked before Lean source was created after wrong working-directory paths;corrected"},{"files":["lean-build-02.log"],"reason":"field_simp already solved chart transition;removed extra tactic"},{"reason":"temporary filesystem filled by concurrent workspace activity;preserved this task's older temporary artifacts on workspace disk,then resumed"}],"source_sha256":sources,"raw_sha256":rawhash,"scope":"same-agent exact native runs and fresh replay;parsed substitution shares primitives;global bridges written with partially formalized algebra;no external audit or IC-efficiency claim"});
    round02::save(round, "ledger.json", &ledger);
    println!("Round11:diagonal degree860 complete under written bridge;gap0;five saturated quadrics;seven whole supports open;22 Lean theorems");
}
