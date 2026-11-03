//! Write-once round10 ledger, native hashes and Lean-source-bound certificates.
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
    let q = read(&raw.join("q8-F2-presentations-summary.json"));
    let check = read(&raw.join("q8b-parsed-F2-presentation-check.json"));
    let expanded = read(&raw.join("q8c-expanded-F2-equations.json"));
    let proto = read(&raw.join("q8p-projective-saturation-prototype.json"));
    let m = read(&raw.join("m13-coupled-cusp-valuations.json"));
    let product = read(&raw.join("m13b-complete-product-divisor.json"));
    let collision = read(&raw.join("m14-Phi11-collision-resultant.json"));
    let diagonal = read(&raw.join("m15-supersingular-diagonal-component.json"));
    assert_eq!(q["records"].as_array().unwrap().len(), 4);
    let mut equations = 0u64;
    let mut variables = 0u64;
    let mut checks = 0u64;
    let mut units = 0u64;
    for r in q["records"].as_array().unwrap() {
        equations += r["F2_equations"].as_u64().unwrap();
        variables += r["variables"].as_u64().unwrap();
        checks += r["equation_component_checks"].as_u64().unwrap();
        units += r["unit_component_checks"].as_u64().unwrap();
    }
    assert_eq!(equations, 1679760);
    assert_eq!(checks, 3359520);
    assert_eq!(units, 144000);
    assert_eq!(expanded["local_gate_evaluation_controls"], 4608);
    assert_eq!(m["geometric_cusps"], 480);
    assert_eq!(product["closed_divisor"].as_array().unwrap().len(), 22);
    assert_eq!(product["zero_degree"], 2640);
    assert_eq!(collision["resultant_degree"], 96);
    assert_eq!(
        diagonal["geometric_equal_map_labels"]
            .as_array()
            .unwrap()
            .len(),
        400
    );
    assert_eq!(diagonal["degree_gap"], 460);
    let sat=ic::IdentityStatement{name:"homogeneous-saturation".into(),source:"KoblitzRound10.homogeneous_saturation_certificate; exact missing relation on the localized projective prototype".into(),variables:["x","y","z","u","h"].iter().map(|s|s.to_string()).collect(),degree_bound:3,lhs:mul(v("h"),sub(mul(v("z"),v("u")),mul(v("y"),v("h")))),rhs:sub(mul(v("y"),sub(mul(v("x"),v("u")),pow(v("h"),2))),mul(v("u"),sub(mul(v("x"),v("y")),mul(v("z"),v("h")))))};
    let x = v("x");
    let y = v("y");
    let d = E::sum(vec![
        mul(pow(x.clone(), 9), pow(y.clone(), 8)),
        mul(pow(x.clone(), 11), pow(y.clone(), 2)),
        mul(
            add(pow(x.clone(), 3), pow(x.clone(), 11)),
            pow(y.clone(), 10),
        ),
    ]);
    let g = E::sum(vec![
        mul(add(c(1), pow(x.clone(), 4)), pow(y.clone(), 4)),
        mul(pow(x.clone(), 3), pow(y.clone(), 3)),
        pow(x.clone(), 4),
    ]);
    let error = E::sum(vec![
        mul(pow(x.clone(), 7), pow(y.clone(), 10)),
        E::product(vec![
            pow(x.clone(), 6),
            add(c(1), pow(x.clone(), 4)),
            pow(y.clone(), 9),
        ]),
        E::product(vec![
            pow(x.clone(), 7),
            add(c(1), pow(x.clone(), 4)),
            pow(y.clone(), 6),
        ]),
        mul(pow(x, 10), pow(y, 5)),
    ]);
    let derivative=ic::IdentityStatement{name:"collision-derivative-residual".into(),source:"KoblitzRound10.collision_derivative_residual; universal integer residual specializes to the derivative factorization in characteristic2;resultant computation separate".into(),variables:vec!["x".into(),"y".into()],degree_bound:21,lhs:sub(d,mul(E::product(vec![pow(v("x"),3),pow(v("y"),2)]),pow(g,2))),rhs:mul(c(-2),error)};
    let mut ids = vec![];
    for (i, s) in [sat, derivative].into_iter().enumerate() {
        let name = format!("identity-{}.json", s.name);
        let cert=ic::issue(s,2026101910+i as u64,64,"same-agent native issuance and separate128 extra checks;exact universal Lean identities",Some(ic::ProofRef{kind:"lean4".into(),path:"research/koblitz_modular_ic_20261018/lean/KoblitzRound10.lean".into(),sha256:hash(&study.join("lean/KoblitzRound10.lean"))})).unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((2026101910001 + i as u64, 128))).unwrap(),
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
    for p in [
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "native/Cargo.toml",
        "native/Cargo.lock",
        "rounds/10/PROTOCOL.md",
        "rounds/10/REPLAY.sh",
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
    let ledger = json!({"schema":"koblitz.round10-ledger/v1","parent_commit":"f75e3ddaf54148ffe874a5f927020aae107bc92e","date":"2026-10-19 UTC","performance_measured":false,"quotients":{"graphs":q["records"],"total_variables":variables,"total_F2_equations":equations,"maximum_degree":2,"equation_component_checks":checks,"parsed_checks":check["records"],"unit_component_checks":units,"basis_products_checked":129600,"dual_number_controls":64,"expanded_scalar_polynomials":72,"expanded_evaluation_controls":4608,"closed_affine_open_embeddings":4,"inverse_witness_norm_saturation":true,"full_projective_h_saturation":false,"proper_quotient_closed_embedding":false,"Jacobian_identification":false},"projective_control":{"naive_false_point":proto["false_projective_F2_point"],"saturated_prototype_generators":3,"Groebner_S_pairs":3,"scope":"prototype saturation complete;whole quotient projective boundary remains open"},"coupled_divisors":{"cusps":480,"functions":m["functions"],"complete_functions":1,"complete_function":"x0*xinf","product_closed_points":22,"product_zero_and_pole_degree":2640,"new_noncuspidal_columns":0,"rank_promoted":false,"retained_nonzero_image_rank":11},"collision":{"resultant":"X^88*(X+1)^8","degree":96,"finite_field_gcd_controls":2046,"operations":collision["polynomial_operations"],"ordinary_j1_equal_transport_on49":0},"diagonal":{"ss_modular_classes":3080,"actual_automorphism_point_controls":225,"norm11_elements":288,"matching_geometric_labels":400,"closed_labels":40,"closed_degree_each":10,"multiplicity_lower_sum":400,"required_residual_degree":860,"unresolved_multiplicity":460,"complete":false,"antidiagonal_complete":false,"explicit_torsion_coordinates":false},"new_named_lean_theorems":23,"identity_certificates":ids,"requirements":[{"track":"F2 polynomial relations","status":"verified complete for specified affine opens","verified":"all four quadratic relation systems in lossless coefficient-block form,with72 literal scalar equations and split-fixture checks","open":"smaller invariant-coordinate ideals and all global chart boundaries"},{"track":"saturation","status":"partial","verified":"norm-denominator saturation by explicit inverse witnesses;complete projective prototype Groebner certificate","open":"homogenizing-variable saturation of the full quotient ideal"},{"track":"closed embeddings","status":"partial","verified":"four explicit closed affine embeddings of descended dense opens","open":"closed projective embeddings of the whole proper quotients,all six charts and retained polarization on the model"},{"track":"complete coupled divisors","status":"partial","verified":"x0*xinf complete on22 closed cusps;all eight pole divisors and all cuspidal zeros","open":"seven residual noncuspidal supports;diagonal400 labels lack460 total multiplicity;antidiagonal and point coordinates"},{"track":"Lean proofs","status":"partial","verified":"23 new named proofs,including inverse ideals,homogenization,coefficient algebra,ordinary49 exclusion and collision factorization","open":"native field semantics,affine Galois-descent scheme equivalence,Tate charts,Hurwitz torsion modules and intersection multiplicities"},{"track":"IC efficiency","status":"not attempted","reason":"user assesses efficiency;no timings or solver run"}],"failed_and_partial_runs":[{"files":["build-01.log"],"reason":"nonexistent field-element hex method;fixed using canonical native BigUint encoding"},{"files":["build-05.log"],"reason":"missing BigUint type annotation and borrowed polynomial slices"},{"files":["collision-01.log"],"reason":"specialized polynomial had trailing zero at j=1;small-field gcd received untrimmed coefficients;corrected before accepted resultant"},{"files":["lean-build-01.log"],"reason":"inverse cancellation rewrite needed explicit associativity;final proof corrected"},{"files":["q8b-original-counter-label-attempt.json","polynomial-check-01.log"],"reason":"counter labeled8640 coefficient values as blocks;mathematical checks unchanged;canonical record reissued with precise name"},{"files":["q8p-projective-saturation-prototype.json"],"reason":"naive homogenization rejected by explicit false boundary point;prototype repaired,full projective quotient not claimed"},{"files":["m15-supersingular-diagonal-component.json"],"reason":"degree-exhaustion attempt incomplete:lower sum400 versus860,leaving460 multiplicity"}],"raw_sha256":rawhash,"source_sha256":sources,"scope":"same-agent exact native checks,fresh replay and parser;shared primitives;global bridges written,partial Lean;no external audit or IC-efficiency claim"});
    round02::save(round, "ledger.json", &ledger);
    println!("Round10:four closed affine F2 models,1679760 quadratic equations;complete product cusp divisor;400 diagonal labels with460 unresolved multiplicity;23 Lean lemmas;two identities64+128 checks");
}
