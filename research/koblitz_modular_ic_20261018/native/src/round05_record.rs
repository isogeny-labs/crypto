//! Write-once round-05 record and symbolic quadratic certificates.
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
        } else {
            if p.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("record-")
            {
                continue;
            }
            out.insert(
                p.strip_prefix(root).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
        }
    }
}
pub fn record(round: &Path) {
    let study = round.parent().unwrap().parent().unwrap();
    let repo = study.parent().unwrap().parent().unwrap();
    let raw = round.join("runs");
    let s = read(&raw.join("s1-reverse-gram.json"));
    let q = read(&raw.join("q3-equivalent-quotients.json"));
    let qb = read(&raw.join("q3b-first-graph-check.json"));
    let m = read(&raw.join("m6-interior-factors.json"));
    let mb = read(&raw.join("m6b-closed-zero-orbits.json"));
    let m7 = read(&raw.join("m7-poles-and-symbolic-row.json"));
    let mut ids = vec![];
    for (i, sign) in s["signs"].as_array().unwrap().iter().enumerate() {
        let rev: Vec<Vec<i64>> =
            serde_json::from_value(sign["reverse_coefficients_FtH"].clone()).unwrap();
        let vars: Vec<_> = ["a", "b", "c", "d"].iter().map(|v| E::var(*v)).collect();
        let w: Vec<_> = rev
            .iter()
            .map(|r| {
                E::sum(
                    r.iter()
                        .zip(&vars)
                        .filter(|(c, _)| **c != 0)
                        .map(|(c, x)| E::times(E::Const(*c), x.clone()))
                        .collect(),
                )
            })
            .collect();
        let sw = E::sum(w.clone());
        let lhs = E::plus(
            E::times(
                E::Const(640),
                E::sum(w.iter().map(|x| E::times(x.clone(), x.clone())).collect()),
            ),
            E::times(E::Const(20), E::times(sw.clone(), sw)),
        );
        let total = E::sum(vars.clone());
        let rhs = E::times(
            E::Const(32000),
            E::plus(
                E::sum(
                    vars.iter()
                        .map(|x| E::times(x.clone(), x.clone()))
                        .collect(),
                ),
                E::times(total.clone(), total),
            ),
        );
        let statement=ic::IdentityStatement{name:format!("polarized-adjoint-norm-{i}"),source:"round05 PROOF_MAP.md: w=F^t H v, w^t G w=32000 v^t H v; follows from kernel-checked matrix composition".into(),variables:["a","b","c","d"].iter().map(|v|v.to_string()).collect(),degree_bound:2,lhs,rhs};
        let cert=ic::issue(statement,2026101905,64,"same-agent native issuer and extra checker, no geometry assumptions hidden in the polynomial",Some(ic::ProofRef{kind:"lean4".into(),path:"research/koblitz_modular_ic_20261018/lean/KoblitzRound05.lean".into(),sha256:hash(&study.join("lean/KoblitzRound05.lean"))})).unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((2026101905001, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        let name = format!("identity-polarized-adjoint-norm-{i}.json");
        round02::save(round, &name, &serde_json::to_value(&cert).unwrap());
        ids.push(json!({"file":name,"id":cert.id,"degree":2,"Lean_scope":"matrix identity; quadratic-form consequence is written"}));
    }
    let (ss, xx, yy) = (E::var("s"), E::var("X"), E::var("Y"));
    let pow = |x: E, n: usize| E::product((0..n).map(|_| x.clone()).collect());
    let sp = |n| pow(ss.clone(), n);
    let xn = E::plus(E::times(E::times(sp(1), sp(2)), xx.clone()), E::Const(1));
    let yn = E::plus(E::times(sp(6), yy.clone()), E::Const(1));
    let lhs = E::sum(vec![
        pow(yn.clone(), 2),
        E::product(vec![sp(3), xn.clone(), yn.clone()]),
        E::times(sp(3), yn),
        E::times(sp(3), pow(xn, 3)),
    ]);
    let rhs = E::sum(vec![
        E::times(
            sp(12),
            E::sum(vec![
                pow(yy.clone(), 2),
                E::times(xx.clone(), yy.clone()),
                pow(xx.clone(), 3),
            ]),
        ),
        E::times(sp(9), pow(xx.clone(), 2)),
        sp(3),
        E::Const(1),
        E::times(
            E::Const(2),
            E::sum(vec![
                E::times(sp(9), pow(xx.clone(), 2)),
                E::product(vec![E::Const(2), sp(6), xx]),
                E::times(sp(9), yy.clone()),
                E::times(sp(6), yy),
                sp(3),
            ]),
        ),
    ]);
    let cert=ic::issue(ic::IdentityStatement{name:"supersingular-deformation-lift".into(),source:"KoblitzRound05.deformation_integral_lift; characteristic-two specialization and valuation argument separated in PROOF_MAP.md".into(),variables:vec!["s".into(),"X".into(),"Y".into()],degree_bound:15,lhs,rhs},2026101905,64,"native issue plus128 extra checker points; direct integral Lean ring identity",Some(ic::ProofRef{kind:"lean4".into(),path:"research/koblitz_modular_ic_20261018/lean/KoblitzRound05.lean".into(),sha256:hash(&study.join("lean/KoblitzRound05.lean"))})).unwrap();
    assert!(matches!(
        ic::check_with_extra(&cert, Some((2026101905002, 128))).unwrap(),
        ic::Verdict::Accept { .. }
    ));
    round02::save(
        round,
        "identity-supersingular-deformation-lift.json",
        &serde_json::to_value(&cert).unwrap(),
    );
    ids.push(json!({"file":"identity-supersingular-deformation-lift.json","id":cert.id,"degree":15,"Lean_scope":"direct universal integer identity"}));
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    for p in [
        "lean/KoblitzRound05.lean",
        "lean/KoblitzRound05Quotients.lean",
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "native/Cargo.toml",
        "native/Cargo.lock",
        "rounds/05/PROTOCOL.md",
        "rounds/05/REPLAY.sh",
    ] {
        let f = study.join(p);
        sources.insert(
            f.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&f),
        );
    }
    let mut rawhash = BTreeMap::new();
    walk(&raw, &raw, &mut rawhash);
    let ledger = json!({"schema":"koblitz.round05-ledger/v1","parent_commit":"75b902661f8e0047281d847d9f1b6ec35827f22b","performance_measured":false,"reverse":{"deck_group_order":660,"ordered_pair_orbit":132,"map_degree":660,"gram_diagonal":660,"gram_off_diagonal":20,"composition_scalar":32000,"scalar_mod11":1,"target_polarization":"H=I4+ones4; degree25","status":"written geometric reverse correspondence and 11-primary splitting; Lean matrix and conditional group proofs","split_coordinate_divisor_evaluated":false,"positive_reverse_degree_bound":6600,"negative_reverse_degree_bound":6600,"certified_inverse_powers":8},"quotients":{"forms_tested":q["forms"].as_array().unwrap().len(),"coefficient_vectors":q["coefficient_vectors"],"unimodular_cyclic_matrices":q["unimodular_cyclic_matrices"],"equivalent_forms":q["forms"].as_array().unwrap().iter().filter(|f|!f["first"].is_null()).count(),"canonical_graph_equations_both_signs":true,"frozen_graph_pm_identity_matches":qb["graphs"].as_array().unwrap().iter().filter(|g|g["direct_pm_identity_match"]==true).count(),"frozen_graphs_tested":10,"det19_equations":"OPEN"},"interior":{"b_polynomial":m["polynomial_bits"],"irreducible_factor_degrees":m["factors"].as_array().unwrap().iter().map(|f|f["degree"].clone()).collect::<Vec<_>>(),"trace_after_point_twist":231,"cyclic49_orbits":{"14":4},"closed_zero_orbits":{"210":44},"ordinary_zero_degree":9240,"closed_zero_orbit_records":mb["records"],"poles_resolved":false,"new_principal_rows":0,"rank_before":4,"rank_after":4,"columns_of_old_matrix":544},"new_named_lean_theorems":15,"identity_certificates":ids,
    "requirements":[{"track":"remaining quotient equations","status":"partial","verified":"all five determinant5 forms have explicit integral automorphisms, inverse and pairing identities; quotient equations for their canonical transported diagonal graphs","open":"nine unmatched frozen first graphs, trace+1 basis alignment and determinant19 mixed quotient equations","next":"construct quotient coordinates for a specified nonidentity common-block graph rather than replacing its graph"},{"track":"n5 reverse transfer and saturation","status":"partial","verified":"written integral polarized adjoint, exact composition32000 and prime-to11 splitting; no11 denominator; general conditional torsion Lean proofs","open":"evaluate reverse divisor into a concrete Jacobian representation; full moduli/adjunction proof not formalized; global optimal quotient and non11 kernel not computed","next":"construct the ten pullback fibers of a nonzero trace-zero point and reduce their divisor class"},{"track":"relations beyond rank4","status":"partial","verified":"irreducible degree15 zero polynomial, maximal-order cyclic49 Frobenius and44 degree210 closed zero labels","open":"pole divisor, explicit new closed-point coordinates and projections, first complete level-dependent principal row; rank remains4/544","next":"resolve supersingular/cusp poles of x(P)+1 and evaluate degree210 support before admitting a row"},{"track":"Lean proofs","status":"partial","verified":"10 new named structural theorems plus5 explicit inverse/pairing certificates","open":"full modular scheme, Picard, adjunction, point-count/Frobenius classification and divisor formalization"},{"track":"IC efficiency","status":"not attempted","reason":"user assesses IC implications; no timing protocol"}],
    "failed_and_partial_runs":[{"file":"run-01.log","reason":"x-coordinate division root does not imply a rational y on the selected a=0 twist; corrected by an explicit a=1 point twist"},{"file":"m6-interior-factors.json","reason":"first orbit screen only admitted valuation-one discriminants; amended separate M6b also handles maximal unramified orders"},{"file":"q3b-first-graph-check.json","reason":"nine of ten frozen first graphs fail the +/-I natural-kernel test; not replaced by canonical graphs"},{"file":"lean-build-02.log","reason":"generated quotient module needed its own Lake library declaration"},{"file":"m6b-closed-zero-orbits.json","reason":"zeros known as orbit labels, poles and actual level coordinates still incomplete; zero new rows"}],"raw_sha256":rawhash,"source_sha256":sources,"scope":"same-agent construction and fresh replay, separate BigInt/factor/partition checks; standard geometric bridges written explicitly; no external audit, no speed or n131 transfer claim"});
    let mut ledger = ledger;
    ledger["failed_and_partial_runs"].as_array_mut().unwrap().push(json!({"file":"assembly-attempt-01/ledger.json and record-raw-hash-check.log","reason":"assembly started before its native rebuild finished; prior binary hashed its own changing log and a live build log; retained failed assembly, then reissued after successful build with assembly receipts excluded"}));
    ledger["new_named_lean_theorems"] = json!(18);
    ledger["interior"]["poles_resolved"]=json!("written complete supersingular stratum and multiplicity; split pole coordinates not enumerated");
    ledger["interior"]["new_principal_rows"] = json!(1);
    ledger["interior"]["rank_after"] = json!(5);
    ledger["interior"]["columns_after"] = json!(588);
    ledger["interior"]["row_status"] = m7["row_status"].clone();
    ledger["interior"]["new_column_images_evaluated"] = json!(false);
    ledger["interior"]["evaluated_matrix_rank"] = json!(4);
    ledger["interior"]["evaluated_matrix_columns"] = json!(544);
    ledger["interior"]["supersingular_geometric_poles"] = json!(3080);
    ledger["interior"]["pole_multiplicity"] = json!(3);
    ledger["requirements"][2] = json!({"track":"relations beyond rank4","status":"partial","verified":"complete written divisor for x(P)+1:44 degree210 ordinary zeros and3080 supersingular poles of multiplicity3; poles project to zero; symbolic projected matrix rank5/588","open":"new closed-point coordinates and their individual projected images are unevaluated; evaluated matrix remains rank4/544; no individual descent","next":"evaluate the44 degree210 column images and independently check the new row in the target group"});
    ledger["requirements"][3]["verified"] =
        json!("13 new named structural theorems plus5 explicit inverse/pairing certificates");
    ledger["failed_and_partial_runs"].as_array_mut().unwrap().push(json!({"file":"lean-build-04.log","reason":"simp reduced the Bezout scalar before applying torsion hypotheses; replaced by an explicit zsmul calculation"}));
    round02::save(round, "ledger.json", &ledger);
    println!("Round05 ledger: reverse scalar32000; five canonical quotient equivalences; symbolic rank5/588; evaluated rank4/544");
}
