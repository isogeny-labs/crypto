//! Write-once round12 ledger, native hashes and Lean-source-bound certificates.
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
    let s = read(&raw.join("s19-complete-supersingular-divisor-strata.json"));
    let q = read(&raw.join("q10-full-homogenized-boundary-counterpoints.json"));
    let sat = read(&raw.join("q10c-constant-gate-saturation-step.json"));
    let parsed = read(&raw.join("s20-parsed-local-and-divisor-check.json"));
    assert_eq!(s["source_geometric_points"].as_array().unwrap().len(), 3080);
    assert_eq!(s["closed_points"].as_array().unwrap().len(), 308);
    assert_eq!(q["naive_full_systems_rejected"], 4);
    let generated = sat["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["new_scalar_linear_equations"].as_u64().unwrap())
        .sum::<u64>();
    assert_eq!(generated, 213120);
    let membership = ic::IdentityStatement {
        name: "constant-gate-saturation".into(),
        source: "KoblitzRound12.constant_multiplication_saturation".into(),
        variables: ["h", "out", "a", "c", "x"].map(str::to_string).to_vec(),
        degree_bound: 3,
        lhs: mul(v("h"), sub(v("out"), mul(v("c"), v("x")))),
        rhs: add(
            sub(mul(v("h"), v("out")), mul(v("a"), v("x"))),
            mul(v("x"), sub(v("a"), mul(v("c"), v("h")))),
        ),
    };
    let ss = v("s");
    let cc = v("c");
    let a = add(ss.clone(), mul(pow(ss.clone(), 2), cc.clone()));
    let b = E::sum(vec![
        c(1),
        mul(ss.clone(), cc.clone()),
        pow(ss.clone(), 3),
        mul(pow(ss.clone(), 4), cc.clone()),
    ]);
    let delta = ic::IdentityStatement {
        name: "velu-discriminant-residual".into(),
        source: "KoblitzRound12.velu_discriminant_residual".into(),
        variables: vec!["s".into(), "c".into()],
        degree_bound: 7,
        lhs: sub(
            E::sum(vec![
                mul(pow(ss.clone(), 2), b),
                mul(ss.clone(), a.clone()),
                pow(a, 2),
            ]),
            E::sum(vec![
                pow(ss.clone(), 2),
                mul(pow(ss.clone(), 4), pow(cc.clone(), 2)),
                pow(ss.clone(), 5),
                mul(pow(ss.clone(), 6), cc.clone()),
            ]),
        ),
        rhs: mul(
            c(2),
            add(pow(ss.clone(), 2), mul(c(2), mul(pow(ss, 3), cc))),
        ),
    };
    let mut ids = vec![];
    for (i, id) in [membership, delta].into_iter().enumerate() {
        let name = format!("identity-{}.json", id.name);
        let cert = ic::issue(
            id,
            202610201200 + i as u64,
            64,
            "native issuer plus128 additional checks;universal Lean identity",
            Some(ic::ProofRef {
                kind: "lean4".into(),
                path: "research/koblitz_modular_ic_20261018/lean/KoblitzRound12.lean".into(),
                sha256: hash(&study.join("lean/KoblitzRound12.lean")),
            }),
        )
        .unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((20261020120001 + i as u64, 128))).unwrap(),
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
        "rounds/12/PROTOCOL.md",
        "rounds/12/REPLAY.sh",
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
    assert!(audited >= 192);
    let mut hashes = BTreeMap::new();
    walk(&raw, &raw, &mut hashes);
    let ledger = json!({"schema":"koblitz.round12-ledger/v1","UTC_date":"2026-10-20","client_date":"2026-10-19 America/Los_Angeles","parent_commit":"539325b5e66719f4cea7ee1315659808103d5bcb","performance_measured":false,"supersingular":{"actual7_points":49,"norm7_elements":192,"actual7_isogeny_bindings":8,"quaternion_Velu_point_checks":31696,"level49_lines":56,"target49_orbit_sizes":[12,12,12,12,4,4],"torsion_precision":128,"Hauptmodul_precision":64,"target_coordinate_precision":16,"source_points":3080,"closed_points":308,"closed_degrees":10,"zero_degrees":s["supersingular_zero_degrees"],"ordinary_zero_degrees_remaining":s["ordinary_zero_degrees_remaining"],"closed_divisor_counts":s["eight_supersingular_divisors"].as_array().unwrap().iter().map(|x|x.as_array().unwrap().len()).collect::<Vec<_>>(),"antidiagonal_SS_degree":740,"ordinary_antidiagonal_degree":500,"contact_histograms":s["contact_histograms"],"round11_contacts_reproduced":400,"parser":parsed,"all_target_points_killed_by8":true,"new_11_primary_nonzero_images":0,"explicit49_torsion_coordinates":false,"complete_SS_under_written_moduli_bridge":true,"seven_whole_divisors_complete":false},"saturation":{"full_system_counterpoints":4,"full_equations_checked":1679760,"new_K_linear_relation_blocks":[150,150,146,146],"generated_scalar_linear_relations":generated,"maximum_h_power_membership_depth":5,"independence_claimed":false,"full_h_saturation":false,"proper_quotient_embedding":false,"actual_boundary_data_kernel_theorems":8},"retained_rank":{"evaluated":12,"columns":1308,"nonzero_image_rank":11,"nonzero_columns":[1165,1160],"promoted":false},"new_named_lean_theorems":24,"audited_project_theorems":audited,"identity_certificates":ids,"requirements":[{"track":"projective saturation","status":"partial","verified":"counterpoints on all four full homogenized systems,all scalar equations checked;592 generated K-linear saturation relations with membership depth<=5","open":"complete saturated ideals and all proper quotient boundaries"},{"track":"closed embeddings","status":"partial","verified":"four affine open embeddings retained;new actual linear relations identify removable auxiliary boundary artifacts","open":"proper polarized closed projective quotient models"},{"track":"seven noncuspidal divisors","status":"partial","verified":"complete supersingular closed supports and exact local multiplicities of all eight functions;ordinary degrees split exactly","open":"ordinary supports of degrees500,1760,1880,2760,580,1370,2180;no whole new principal row"},{"track":"antidiagonal","status":"partial","verified":"complete SS contribution740 on52 degree10 closed points,contacts1,2,4;old diagonal860 reproduced","open":"ordinary contribution500"},{"track":"460 diagonal gap","status":"verified complete under written moduli bridge","verified":"all400 round11 local contacts independently reconstructed using local49 coordinates","open":"full global moduli formalization and explicit49 torsion points"},{"track":"Lean proofs","status":"partial","verified":"24 new named proofs,including8 actual full-circuit finite certificates and structural saturation/order/primary-image implications","open":"native field semantics and complete modular/local deformation geometry remain written"},{"track":"IC efficiency","status":"not attempted","reason":"user assesses;no timings,solver run or rank promotion"}],"corrections":["Round11 proof map said three independent AS characters;there are two independent characters and their nonzero sum,the three nontrivial characters of a rank2 group;genus3 unchanged","Previous status misread expected TODO negative-test output as the backlog failure;actual failure is concurrent_claims_resolve_to_one_holder_after_sync","Initial S18 antidiagonal lower bound used only one branch's modular stabilizer;correct minimum gives520 before exact local contact computation740","Initial Q10c max depth omitted scaling relations;final full maximum is5,with prior export retained"],"failed_and_partial_runs":[{"files":["lean-build-02.log"],"reason":"nested-power normalization was missing in the parameter factorization;explicit pow_mul rewrite fixes the elaboration"},{"files":["root-tests.log","root-timer-isolated.log","root-tests-serial.log"],"reason":"parallel root run observed shared profiling counter calls2 instead of1;isolated test passes;full serial result retained without weakening any test"},{"files":["s18-initial-lower-bound-attempt.json","ss49-01.log"],"reason":"one-branch stabilizer not a valid lower bound under non-modular elliptic negation"},{"files":["s18-eight-jet-attempt.json","ss49-02.log"],"reason":"20 diagonal order14 points censored at8;expanded to16 under frozen amendment"},{"files":["lean-boundary-build-01.log","lean-build-01.log","Boundary-before-minimal-import.lean"],"reason":"duplicate dependent builds/import overhead interrupted;regenerated minimal boundary module and built sequentially"},{"files":["q10c-before-depth-crosscheck.json"],"reason":"aggregate depth initially counted constant nodes only;corrected to include all linear scaling relations"},{"reason":"runtime migration lacks CLI GitHub credentials;publication uses the authorized GitHub connector,without reading or copying secrets"}],"raw_sha256":hashes,"source_sha256":sources,"scope":"same-agent native construction and parsed substitutions share primitives;fresh replay and partial Lean formalization;global modular/etale bridges written;no external audit or IC efficiency claim"});
    round02::save(round, "ledger.json", &ledger);
    println!("Round12:complete SS strata of8 functions;antidiagonal740 SS+500 ordinary;four actual boundary counterpoints;213120 generated scalar linear saturation relations;24 new Lean proofs;{audited} audited");
}
