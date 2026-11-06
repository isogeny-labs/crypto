//! Frozen round-04 identities, source/input bindings and requirement ledger.
use crate::{identity_certificate as ic, round02, round04::read, sha256_impl};
use ic::Expr as E;
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::Path};
fn c(x: i64) -> E {
    E::Const(x)
}
fn p(x: E, n: usize) -> E {
    E::product((0..n).map(|_| x.clone()).collect())
}
fn a(x: E, y: E) -> E {
    E::plus(x, y)
}
fn s(x: E, y: E) -> E {
    a(x, E::minus(y))
}
fn m(x: E, y: E) -> E {
    E::times(x, y)
}
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
fn certs(round: &Path, repo: &Path) -> Vec<Value> {
    let mut out = vec![];
    let mut issue = |name: &str, vars: Vec<&str>, degree: u32, lhs: E, rhs: E, lean: bool| {
        let statement=ic::IdentityStatement{name:name.into(),source:"round04 PROOF_MAP.md, exact universal integer identity; characteristic-two bridges stated separately".into(),variables:vars.into_iter().map(str::to_string).collect(),degree_bound:degree,lhs,rhs};
        let proof = if lean {
            Some(ic::ProofRef {
                kind: "lean4".into(),
                path: "research/koblitz_modular_ic_20261018/lean/KoblitzRound04.lean".into(),
                sha256: hash(
                    &repo.join("research/koblitz_modular_ic_20261018/lean/KoblitzRound04.lean"),
                ),
            })
        } else {
            None
        };
        let cert = ic::issue(
            statement,
            2026101804,
            64,
            "native issuance and extra checking; exact Lean coverage separated",
            proof,
        )
        .unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((2026101804001, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        let file = format!("identity-{name}.json");
        round02::save(round, &file, &serde_json::to_value(&cert).unwrap());
        out.push(json!({"file":file,"id":cert.id,"degree":degree,"direct_Lean_statement":lean}));
    };
    let x0 = E::var("X");
    let yy = E::var("Y");
    let x = s(x0.clone(), c(2));
    let d = E::sum(vec![p(x.clone(), 2), m(c(7), x.clone()), c(7)]);
    let y = E::sum(vec![
        m(d.clone(), yy.clone()),
        m(c(4), p(x.clone(), 3)),
        m(c(22), p(x.clone(), 2)),
        m(c(35), x.clone()),
        c(7),
    ]);
    let poly = |coeff: &[i64]| {
        E::sum(
            coeff
                .iter()
                .enumerate()
                .map(|(i, v)| m(c(*v), p(x.clone(), i)))
                .collect(),
        )
    };
    let lhs = s(
        s(
            p(y.clone(), 2),
            E::product(vec![c(7), x.clone(), y, poly(&[7, 5, 1])]),
        ),
        m(x.clone(), poly(&[343, 343, 147, 49, 21, 7, 1])),
    );
    let rhs = m(
        p(d, 2),
        E::sum(vec![
            p(yy.clone(), 2),
            m(x0.clone(), yy),
            E::minus(p(x0.clone(), 3)),
            p(x0.clone(), 2),
            m(c(2), x0),
            c(1),
        ]),
    );
    issue("modular49-change", vec!["X", "Y"], 7, lhs, rhs, true);
    let av: Vec<_> = ["a", "b", "c", "d"].iter().map(|n| E::var(*n)).collect();
    let bv: Vec<_> = ["e", "f", "g", "h"].iter().map(|n| E::var(*n)).collect();
    let q = E::var("q");
    let r = E::var("r");
    let sa = E::sum(av.clone());
    let sb = E::sum(bv.clone());
    let mut lhs: Vec<_> = av
        .iter()
        .zip(&bv)
        .map(|(x, y)| m(s(x.clone(), q.clone()), s(y.clone(), r.clone())))
        .collect();
    lhs.push(m(
        s(E::minus(sa.clone()), q.clone()),
        s(E::minus(sb.clone()), r.clone()),
    ));
    let mut rhs: Vec<_> = av
        .iter()
        .zip(&bv)
        .map(|(x, y)| m(x.clone(), y.clone()))
        .collect();
    rhs.push(m(sa, sb));
    rhs.push(E::product(vec![c(5), q, r]));
    issue(
        "polarization-pullback",
        vec!["a", "b", "c", "d", "e", "f", "g", "h", "q", "r"],
        2,
        E::sum(lhs),
        E::sum(rhs),
        true,
    );
    let (y, t, x, b, aa) = (
        E::var("y"),
        E::var("t"),
        E::var("x"),
        E::var("b"),
        E::var("a"),
    );
    let lhs = E::sum(vec![
        p(a(y.clone(), t.clone()), 2),
        m(x.clone(), a(y.clone(), t.clone())),
        p(x.clone(), 3),
        m(aa.clone(), p(x.clone(), 2)),
        b.clone(),
        t.clone(),
        p(t.clone(), 2),
    ]);
    let rhs = E::sum(vec![
        p(y.clone(), 2),
        m(x.clone(), y.clone()),
        p(x.clone(), 3),
        m(aa, p(x.clone(), 2)),
        m(t.clone(), x),
        b,
        t.clone(),
        m(c(2), a(m(y, t.clone()), p(t, 2))),
    ]);
    issue(
        "velu-integer-lift",
        vec!["y", "t", "x", "b", "a"],
        3,
        lhs,
        rhs,
        false,
    );
    out
}
pub fn record(round: &Path) {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let raw = round.join("runs");
    let ids = certs(round, repo);
    let q1 = read(&raw.join("q1-quotient-lattices.json"));
    let q2 = read(&raw.join("q2-natural-quotient.json"));
    let m1 = read(&raw.join("m1-modular-base.json"));
    let m3 = read(&raw.join("m3b-paired-transport.json"));
    let m4 = read(&raw.join("m4-direct-modular-relations.json"));
    let m5 = read(&raw.join("m5-interior-function.json"));
    let mut rawhash = BTreeMap::new();
    walk(&raw, &raw, &mut rawhash);
    let mut sources = BTreeMap::new();
    let study = repo.join("research/koblitz_modular_ic_20261018");
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    for p in [
        "native/Cargo.toml",
        "native/Cargo.lock",
        "lean/KoblitzRelations.lean",
        "lean/KoblitzRound03.lean",
        "lean/KoblitzRound04.lean",
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "rounds/04/PROTOCOL.md",
        "rounds/04/REPLAY.sh",
    ] {
        let path = study.join(p);
        sources.insert(
            path.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&path),
        );
    }
    for name in [
        "src/cryptanalysis/curve_construction/gf2k.rs",
        "src/cryptanalysis/curve_construction/lpoly.rs",
        "src/cryptanalysis/curve_construction/weil.rs",
        "src/cryptanalysis/curve_construction/modular.rs",
        "src/cryptanalysis/identity_certificate.rs",
        "src/hash/sha256.rs",
    ] {
        sources.insert(name.into(), hash(&repo.join(name)));
    }
    let ledger = json!({"schema":"koblitz.round04-ledger/v1","parent_commit":"f5b90fb286dde2e8b0fa15b127a37514a8d64258","performance_measured":false,"identity_certificates":ids,"new_named_lean_theorems":11,"quotients":{"integral_modules":q1["quotients"].as_array().unwrap().len(),"global_minimum3_forms":q1["short_vectors"].as_array().unwrap().len(),"explicit_natural_quotient":q2["quotient_map"],"rational_point_checks":q2["rational_checks"].as_array().unwrap().iter().map(|r|json!({"a":r["a"],"points":r["rational_points"],"addition_checks":r["addition_checks"]})).collect::<Vec<_>>(),"geometric_kernel_order_each_sign":25,"all_other_quotient_coordinate_equations":false},"modular_base":{"extensions":m1["records"].as_array().unwrap().len(),"points":m1["records"].as_array().unwrap().iter().map(|r|r["points"].as_u64().unwrap()).sum::<u64>(),"primary_polynomial_input":"phi_j_11.txt"},"transport":{"supported_sources":m3["records"].as_array().unwrap().iter().filter(|r|r["status"]=="PASS").count(),"unsupported_nonzero_j_sources":m3["records"].as_array().unwrap().iter().filter(|r|r["status"]!="PASS").count(),"actual_Hecke_targets_per_source":12,"a11":m3["a11"],"fixed_Weil_pairing":true},"relations":{"geometric_points":m4["geometric_points"],"columns":m4["columns"],"closed_degrees":m4["closed_degrees"],"nonzero_images_by_sign":m4["nonzero_images_by_sign"],"principal_rows":m4["principal_relations"].as_array().unwrap().len(),"rank_mod11":m4["rank_mod11"],"missing_rank":m4["missing_rank_to_unique_scale"],"recoveries_attempted":0},"interior_function_attempt":m5,
 "requirements":[{"track":"explicit polarized quotient geometry","status":"partial","verified":"natural determinant-five quotient equations, rational inverse, all group additions and two geometric order25 kernels; fourteen explicit integral quotient modules","open":"equations and polarization decomposition for the other graph quotients, especially determinant19","next":"construct a characteristic-two quotient presentation for a determinant19 graph, or prove its polarized decomposition"},{"track":"n5 F2 modular-factor map","status":"partial","verified":"actual 11-isogeny transport of both7 stages, paired bases, canonical modular Frobenius, nonzero closed-divisor projections to both trace-zero groups","open":"global scheme/character correspondence bridge not formalized; no reverse subgroup-to-Jacobian map or optimal saturation certificate","next":"certify the induced polarization/composition on the order11 subgroup, including projector denominators"},{"track":"direct modular relations","status":"partial","verified":"102 actual full-fiber principal norm rows on544 closed points, rank4; cusp elimination exposed","open":"539 ranks to unique scale; individual descent; level-dependent zero/pole support","next":"factor the degree15 b-polynomial for x(P)+1, construct its missing level49/11 closed orbits, resolve poles, then seek rows outside the four-fiber span"},{"track":"Lean proofs","status":"partial","verified":"11 new named structural theorems; actual kernel build and axiom audit","open":"full modular moduli/descent, Weil/Miller correctness, actual principal-divisor and polarization bridges"},{"track":"IC efficiency assessment","status":"not attempted","reason":"user assesses significance; no timing protocol"}],
 "failed_and_partial_runs":[{"file":"record-build-01.log","cause":"ledger builder moved a borrowed symbolic expression; corrected with an explicit clone"},{"file":"replay-01.log","cause":"new pairing metadata changed the historical unpaired serialization; restored the original format without changing its frozen file"},{"file":"q1-run-01.log","cause":"used nonnegative residue representatives for the elliptic Frobenius as an integer lift; corrected to the signed companion matrix"},{"file":"m1-run-01.log","cause":"degree-one Rabin special case initialized X outside the field; corrected before successful modular enumeration"},{"file":"m3-run-01.log","cause":"prime-field point count omitted the projective point in a11 initialization; corrected a11=4 and all Hecke sums pass"},{"file":"m3-modular-transport.json","cause":"unpaired level bases are not used for cross-point relations; replaced by the separately frozen paired experiment"},{"file":"m4-direct-modular-relations.json","cause":"rank4/544; structural four-fiber obstruction, not a solver"},{"file":"m5-interior-function.json","cause":"no complete zero support; no row admitted"},{"file":"lean-build-01.log and lean-build-02.log","cause":"binary identity residual, natural-smul simplification and matrix multiplication scope; final build succeeds"}],
 "raw_sha256":rawhash,"source_sha256":sources,"proof_map":"PROOF_MAP.md","scope":"same-agent construction/replay, separate maintained-library arithmetic checker; no independent external audit; no IC timing claim"});
    round02::save(round, "ledger.json", &ledger);
    println!("Round04 ledger:14 quotient modules; explicit natural quotient;24 modular inputs;544 closed columns;102 rows rank4;11 Lean lemmas");
}
