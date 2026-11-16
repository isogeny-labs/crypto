//! Write-once round06 evidence and exact, Lean-bound identity certificates.
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
fn pow(x: E, n: usize) -> E {
    E::product((0..n).map(|_| x.clone()).collect())
}
fn mul(a: E, b: E) -> E {
    E::times(a, b)
}
fn c(n: i64) -> E {
    E::Const(n)
}
fn sub(a: E, b: E) -> E {
    E::plus(a, mul(c(-1), b))
}
pub fn record(round: &Path) {
    let study = round.parent().unwrap().parent().unwrap();
    let repo = study.parent().unwrap().parent().unwrap();
    let raw = round.join("runs");
    let images = read(&raw.join("i1-degree210-images.json"));
    let matrix = read(&raw.join("i2-zero-column-matrix-check.json"));
    let quotients = read(&raw.join("q4-split-quotient-equations.json"));
    let reverse = read(&raw.join("r3d-even-reverse-divisor.json"));
    assert_eq!(images["nonzero_by_sign"], json!([0, 0]));
    assert_eq!(quotients["graphs"].as_array().unwrap().len(), 10);
    assert_eq!(reverse["terms"].as_array().unwrap().len(), 20);
    let vars: Vec<E> = [
        "a", "b", "c", "d", "e", "f", "g", "h", "Q", "R", "p", "q", "alpha",
    ]
    .iter()
    .map(|v| E::var(*v))
    .collect();
    let sa = E::sum(vars[..4].to_vec());
    let sb = E::sum(vars[4..8].to_vec());
    let sy = mul(c(5), sa.clone());
    let sw = mul(c(5), sb.clone());
    let z = sub(
        mul(c(-1), mul(vars[11].clone(), sa.clone())),
        vars[8].clone(),
    );
    let zz = sub(
        mul(c(-1), mul(vars[10].clone(), sb.clone())),
        vars[9].clone(),
    );
    let lhs = E::sum(vec![
        E::sum(
            (0..4)
                .map(|i| {
                    mul(
                        E::plus(vars[i].clone(), sa.clone()),
                        E::plus(vars[i + 4].clone(), sb.clone()),
                    )
                })
                .collect(),
        ),
        E::product(vec![vars[12].clone(), sy.clone(), sw.clone()]),
        E::product(vec![vars[11].clone(), sy, zz.clone()]),
        E::product(vec![vars[10].clone(), z.clone(), sw]),
        mul(c(5), mul(z, zz)),
    ]);
    let rhs = E::sum(vec![
        E::sum(
            (0..4)
                .map(|i| mul(vars[i].clone(), vars[i + 4].clone()))
                .collect(),
        ),
        mul(sa.clone(), sb.clone()),
        E::product(vec![c(5), vars[8].clone(), vars[9].clone()]),
        E::product(vec![
            c(5),
            sub(
                E::plus(c(1), mul(c(5), vars[12].clone())),
                mul(vars[10].clone(), vars[11].clone()),
            ),
            sa,
            sb,
        ]),
    ]);
    let first=ic::IdentityStatement{name:"split-polarization-lift".into(),source:"KoblitzRound06.polarization_lift: universal bilinear residual; geometry and norm hypothesis separated in PROOF_MAP.md".into(),variables:["a","b","c","d","e","f","g","h","Q","R","p","q","alpha"].iter().map(|v|v.to_string()).collect(),degree_bound:4,lhs,rhs};
    let x = E::var("x");
    let t = E::var("t");
    let series = E::sum((0..=6).map(|i| pow(x.clone(), i)).collect());
    let lhs = E::sum(vec![
        pow(pow(x.clone(), 4), 2),
        E::product(vec![
            x.clone(),
            pow(x.clone(), 4),
            t.clone(),
            E::sum(vec![pow(x.clone(), 2), x.clone(), c(1)]),
        ]),
        E::product(vec![x.clone(), series.clone(), pow(t.clone(), 2)]),
    ]);
    let rhs = mul(
        x.clone(),
        E::sum(vec![
            pow(x.clone(), 7),
            mul(
                t.clone(),
                E::sum((4..=6).map(|i| pow(x.clone(), i)).collect()),
            ),
            mul(pow(t.clone(), 2), series),
        ]),
    );
    let second=ic::IdentityStatement{name:"degree-seven-equation".into(),source:"KoblitzRound06.degree_seven_equation: exact cleared equation only; local valuation and fiber geometry written separately".into(),variables:vec!["x".into(),"t".into()],degree_bound:9,lhs,rhs};
    let mut ids = vec![];
    for (i, statement) in [first, second].into_iter().enumerate() {
        let name = format!("identity-{}.json", statement.name);
        let cert=ic::issue(statement,2026101906+i as u64,64,"same-agent native issuer plus separate128 evaluation checks; direct universal Lean identities",Some(ic::ProofRef{kind:"lean4".into(),path:"research/koblitz_modular_ic_20261018/lean/KoblitzRound06.lean".into(),sha256:hash(&study.join("lean/KoblitzRound06.lean"))})).unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((2026101906001 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        round02::save(round, &name, &serde_json::to_value(&cert).unwrap());
        ids.push(json!({"file":name,"id":cert.id,"Lean_scope":"direct universal ring identity; geometric interpretation not formalized","issuer_points":64,"extra_checker_points":128}));
    }
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    for p in [
        "lean/KoblitzRound06.lean",
        "lean/KoblitzRound06Matrices.lean",
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "native/Cargo.toml",
        "native/Cargo.lock",
        "rounds/06/PROTOCOL.md",
        "rounds/06/REPLAY.sh",
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
    let ledger = json!({"schema":"koblitz.round06-ledger/v1","parent_commit":"ec0d839d4200eb72eafe250aab254fa2b2f90ebb","base_merge_commit":"1d7811e45","host_date":"2026-10-19 UTC","client_date":"2026-10-18 America/Los_Angeles","performance_measured":false,
        "images":{"closed_columns":44,"degree":210,"geometric_tuples_nonzero_each_sign":616,"nonzero_closed_images_each_sign":0,"relative14_traces_nonzero_each_sign":0,"first7_fibers":8,"target_checks":96,"fiber_sum":"T=(0,1), order2","matrix":matrix},
        "quotients":{"frozen_determinant5_graphs_realized":10,"geometric_kernel_points_checked":250,"split_models_k":[0,4,8],"equations":"five E_a projective equations over F32 with explicit order5 polarized F2 descent","expanded_F2_ideal":false,"determinant19_equations":"OPEN","Jacobian_claim":false},
        "reverse":{"sign":"even","source":"one nonzero order11 generator of A5(E1)(F2)","pullback_fibers":20,"positive_degree":6600,"negative_degree":6600,"point_algebra_entries":7200,"point_algebra_degrees":[1,11],"split_seed_points":660,"negative_seed_components":60,"F2_descent_checked":true,"forward_composition":"proven Gram correspondence;32000P=P; not independent pointwise norm evaluation","Jacobian_reduction":false,"odd_reverse_divisor":false},
        "new_named_lean_theorems":14,"audited_theorems_including_generated_equation":15,"identity_certificates":ids,
        "requirements":[
          {"track":"44 degree210 images","status":"verified complete","verified":"all44 images zero both signs; two-stage trace, Frobenius equivariance and degree7 fiber cancellation checked","open":"new functions with nonzero column images and constraints beyond rank4","next":"use functions distinguishing second7 choices, rather than whole degree7 fibers"},
          {"track":"reverse divisor evaluation","status":"partial","verified":"all20 even reverse pullbacks in explicit split/etale coordinates; degree6600 each sign; F2 descent; correspondence-based forward check","open":"independent pointwise forward norm, reduced genus2841 Jacobian arithmetic and odd reverse divisor","next":"norm-evaluate target functions on degree11 components and compare with the Gram result"},
          {"track":"remaining quotient equations","status":"partial","verified":"all10 frozen determinant5 graphs realized with declared actual torsion basis binding; split equations, effective F2 descent and positive unimodular polarization","open":"determinant19 mixed quotients and expanded F2 projective ideals; no Jacobian identification","next":"construct an explicit determinant19 mixed quotient or a verified obstruction"},
          {"track":"Lean proofs","status":"partial","verified":"14 new named ring, group, descent and explicit matrix proofs;15 declarations transitively audited","open":"modular schemes, Picard maps, global divisor geometry and full coordinate-algebra formalization"},
          {"track":"IC efficiency","status":"not attempted","reason":"user assesses IC implications; no timing or solver experiment"}],
        "failed_and_partial_runs":[{"file":"build-01.log","reason":"unsigned point-count expression overflow; removed unused expression"},{"file":"build-06.log","reason":"BigInt type inference; fixed explicit type"},{"file":"reverse-screen-01.log","reason":"positive seed has all60 rational11 x roots, contradicting predicted degree11 complements"},{"file":"split-fiber-negative-01.log","reason":"negative seed has only5 rational11 x roots; finite-etale coordinates required"}],
        "raw_sha256":rawhash,"source_sha256":sources,"scope":"same-agent construction, fresh byte-identical replay and separate parse/library/BigInt checks; shared arithmetic primitives; no external audit; geometry partly written; no IC speed verdict"});
    round02::save(round, "ledger.json", &ledger);
    println!("Round06:44 zero columns both signs;10 determinant5 graphs;20 reverse fibers;14 named Lean proofs;2 identities checked64+128 points");
}
