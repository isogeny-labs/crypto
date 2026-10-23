//! Write-once Round18 ledger and three source-bound universal identities.
use super::*;
fn prod(v: Vec<E>) -> E {
    v.into_iter().fold(c(1), mul)
}
fn sum(v: Vec<E>) -> E {
    v.into_iter().fold(c(0), add)
}
fn statements() -> Vec<(ic::IdentityStatement, &'static str)> {
    let (h, x, u, w) = (v("h"), v("x"), v("U"), v("V"));
    let sc = ic::IdentityStatement {
        name: "square-cube-source".into(),
        source: "KoblitzRound18.square_cube_residual".into(),
        variables: ["h", "x", "U", "V"].map(str::to_string).to_vec(),
        degree_bound: 6,
        lhs: mul(
            pow(h.clone(), 3),
            sub(pow(u.clone(), 3), mul(h.clone(), pow(w.clone(), 2))),
        ),
        rhs: sub(
            mul(
                sub(pow(x.clone(), 3), mul(pow(h.clone(), 2), w.clone())),
                add(pow(x.clone(), 3), mul(pow(h.clone(), 2), w)),
            ),
            mul(
                sub(pow(x.clone(), 2), mul(h.clone(), u.clone())),
                sum(vec![
                    pow(x.clone(), 4),
                    prod(vec![h.clone(), pow(x, 2), u.clone()]),
                    mul(pow(h, 2), pow(u, 2)),
                ]),
            ),
        ),
    };
    let (b, cc, s, t, x, y) = (v("b"), v("c"), v("s"), v("u"), v("X"), v("Y"));
    let aa = add(c(1), cc.clone());
    let xx = add(mul(pow(t.clone(), 2), x.clone()), b.clone());
    let yy = sum(vec![
        mul(pow(t.clone(), 3), y.clone()),
        prod(vec![s.clone(), pow(t.clone(), 2), x.clone()]),
        mul(b.clone(), cc.clone()),
    ]);
    let original = sum(vec![
        pow(yy.clone(), 2),
        prod(vec![aa.clone(), xx.clone(), yy.clone()]),
        mul(b.clone(), yy),
        pow(xx.clone(), 3),
        mul(b.clone(), pow(xx, 2)),
    ]);
    let transformed = sum(vec![
        mul(pow(t.clone(), 6), add(pow(y.clone(), 2), pow(x.clone(), 3))),
        prod(vec![aa.clone(), pow(t.clone(), 5), x.clone(), y.clone()]),
        prod(vec![b.clone(), cc.clone(), pow(t.clone(), 3), y.clone()]),
        prod(vec![
            add(pow(s.clone(), 2), mul(aa.clone(), s.clone())),
            pow(t.clone(), 4),
            pow(x.clone(), 2),
        ]),
    ]);
    let constraint = prod(vec![
        b.clone(),
        pow(t.clone(), 2),
        x.clone(),
        sum(vec![
            mul(cc.clone(), s.clone()),
            mul(aa.clone(), cc.clone()),
            b.clone(),
        ]),
    ]);
    let cofactor = sum(vec![
        prod(vec![s.clone(), pow(t.clone(), 5), x.clone(), y.clone()]),
        prod(vec![b.clone(), aa.clone(), pow(t.clone(), 3), y]),
        prod(vec![c(2), b.clone(), pow(t.clone(), 4), pow(x.clone(), 2)]),
        prod(vec![b.clone(), aa.clone(), s, pow(t.clone(), 2), x.clone()]),
        prod(vec![c(2), pow(b.clone(), 2), pow(t, 2), x]),
        prod(vec![pow(b.clone(), 2), cc, aa]),
        pow(b, 3),
    ]);
    let tate = ic::IdentityStatement {
        name: "Tate-diamond-coordinate".into(),
        source: "KoblitzRound18Tate.Tate_coordinate_residual".into(),
        variables: ["b", "c", "s", "u", "X", "Y"].map(str::to_string).to_vec(),
        degree_bound: 9,
        lhs: sub(sub(original, transformed), constraint),
        rhs: mul(c(2), cofactor),
    };
    let (x, t, a) = (v("x"), v("t"), v("a"));
    let tx = mul(t.clone(), x.clone());
    let artin = ic::IdentityStatement {
        name: "elliptic-map-cleared".into(),
        source: "KoblitzRound18.elliptic_map_cleared".into(),
        variables: ["x", "t", "a"].map(str::to_string).to_vec(),
        degree_bound: 4,
        lhs: sum(vec![
            pow(tx.clone(), 2),
            mul(x.clone(), tx),
            pow(x.clone(), 3),
            mul(a.clone(), pow(x.clone(), 2)),
            c(1),
        ]),
        rhs: add(
            mul(pow(x, 2), sum(vec![pow(t.clone(), 2), t, v("x"), a])),
            c(1),
        ),
    };
    vec![
        (sc, "KoblitzRound18"),
        (tate, "KoblitzRound18Tate"),
        (artin, "KoblitzRound18"),
    ]
}
pub fn identities(round: &Path, issue: bool) {
    let round = std::fs::canonicalize(round).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    for (i, (st, module)) in statements().into_iter().enumerate() {
        let name = format!("identity-{}.json", st.name);
        let path = format!("research/koblitz_modular_ic_20261018/lean/{module}.lean");
        if issue {
            let cert = ic::issue(
                st.clone(),
                202610230300 + i as u64,
                64,
                "native issuer;128 extra parsed checks and explicit source-bound kernel theorem",
                Some(ic::ProofRef {
                    kind: "lean4".into(),
                    path: path.clone(),
                    sha256: hash(&repo.join(&path)),
                }),
            )
            .unwrap();
            round02::save(&round, &name, &serde_json::to_value(cert).unwrap());
        }
        let cert: ic::IdentityCertificate =
            serde_json::from_value(read(&round.join(name))).unwrap();
        assert_eq!(cert.statement, st);
        assert_eq!(cert.proof.as_ref().unwrap().sha256, hash(&repo.join(path)));
        assert!(matches!(
            ic::check_with_extra(&cert, Some((202610230400 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }
    println!("Round18:3 source-bound IDC records pass128 extra parsed checks each");
}
pub fn record(round: &Path) {
    let round = std::fs::canonicalize(round).unwrap();
    let study = round.ancestors().nth(2).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    let raw = round.join("runs");
    let audit = std::fs::read_to_string(raw.join("lean-full-audit.log")).unwrap();
    assert!(!audit.contains("sorryAx"));
    assert!(audit.contains("0 failing, 0 trusted"));
    let audited: u64 = audit
        .lines()
        .find(|s| s.starts_with("audited "))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let get = |s: &str| read(&raw.join(s));
    let q = get("q25-companion-source-separators.json");
    assert!(q["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["Q24_model_excluded"] == true));
    let map = get("cm21-affine-linear-elliptic-map-search.json");
    let holds = get("cm22-independent-elliptic-map-holdouts.json");
    for row in holds["records"].as_array().unwrap() {
        assert!(row["nonconstant_survivors"].as_array().unwrap().is_empty());
    }
    for (i, (n, c)) in [(90u64, 43u64), (12, 21)].iter().enumerate() {
        assert_eq!(map["records"][i]["nonconstant_fits"], *n);
        assert_eq!(
            holds["records"][i]["rejected_finite_fits"]
                .as_array()
                .unwrap()
                .len(),
            *n as usize
        );
        assert_eq!(holds["records"][i]["constant_maps_retained"], *c);
    }
    let sq = get("q27-square-cube-source-separators.json");
    assert!(sq["records"].as_array().unwrap().iter().all(|r| r["family"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["membership_depth"] == 5)));
    let theta = get("g19-canonical-symmetric-theta-characters.json");
    let obs = get("g20-odd-cover-rotation-lift-obstruction.json");
    let refinement = get("g21-rotation-compatible-scalar-refinement.json");
    let models = get("q28-after-source-square-repairs.json");
    identities(&round, true);
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    let mut named = 0;
    for e in std::fs::read_dir(study.join("lean")).unwrap() {
        let p = e.unwrap().path();
        if p.extension().is_some_and(|e| e == "lean") {
            sources.insert(
                p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
            if p.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("KoblitzRound18")
            {
                named += std::fs::read_to_string(p)
                    .unwrap()
                    .lines()
                    .filter(|s| s.starts_with("theorem "))
                    .count();
            }
        }
    }
    for p in [
        study.join("lean/lakefile.toml"),
        study.join("lean/lake-manifest.json"),
        study.join("lean/lean-toolchain"),
        study.join("native/Cargo.toml"),
        study.join("native/Cargo.lock"),
        repo.join("src/cryptanalysis/curve_construction/gf2k.rs"),
        repo.join("src/cryptanalysis/curve_construction/weil.rs"),
        repo.join("src/cryptanalysis/curve_construction/modular.rs"),
        repo.join("src/cryptanalysis/curve_construction/lpoly.rs"),
        repo.join("src/cryptanalysis/identity_certificate.rs"),
        repo.join("src/hash/sha256.rs"),
        repo.join("tools/lean/proof-gate.sh"),
        repo.join("tools/lean/AuditAxioms.lean"),
        repo.join("tools/lean/trusted-native.txt"),
        repo.join(".github/workflows/koblitz-modular-carriers.yml"),
        round.join("PROTOCOL.md"),
        round.join("REPLAY.sh"),
        round.join("CM.md"),
        round.join("THETA.md"),
        round.join("SATURATION.md"),
        round.join("SOURCES.md"),
    ] {
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
    }
    let mut hashes = BTreeMap::new();
    walk(&raw, &raw, &mut hashes);
    let headline = json!({"schema":"koblitz.round18-ledger/v1","client_date":"2026-10-22 America/Los_Angeles","UTC_date":"2026-10-23","scientific_parent":"76e409c1e31ff5b4a51e901bcda7e44529b732a5","new_named_Lean_theorems":named,"audited_project_theorems":audited,
 "CM":{"explicit_integral_diamond2_correspondence":true,"actual_factor_inclusion_evaluated":false,"shared_rational11_F_D":[1,6],"residual_Hecke_scope":"actual CM residues and unramified system1+epsilon_minus7;arbitrary integral maps not excluded","affine_x_forms_per_sign":33554432u64,"x_valid_systems":map["records"].as_array().unwrap().iter().map(|r|r["F2_linear_systems_solved"].clone()).collect::<Vec<_>>(),"finite_nonconstant_fits":[90,12],"independent_holdouts":8,"all102_nonconstant_fits_refuted":true,"constant_controls":[43,21]},
 "theta":{"canonical_character_exponents_mod105":[35,0,50,23],"canonical_matching":"explicit unique symmetric lifts under written symmetric pullback class bridge","standard_split_Frobenius_checked":theta["records"].as_array().unwrap().iter().all(|r|r["standard_split_product_Frobenius_checked"]==true),"selected_cover_rotation_denominator":obs["lift_denominator"],"no_lift_modulus":obs["obstruction_modulus"],"stable_refinement_scalar":105,"stable_degree":refinement["stable_composite_degree"],"refinement_degree":refinement["refinement_degree"],"kernel_Frobenius_order":840,"actual_F2_invariant_sections_extracted":false},
 "saturation":{"all_four_Q24_excluded":true,"Q25_depth":6,"all_four_Q26_excluded":true,"Q27_source_residual_depth":5,"Q28_models":models["records"].as_array().unwrap().iter().map(|r|json!({"graph":r["graph"],"visits":r["total_visits"],"nonzero_nodes":r["model"].as_array().unwrap().iter().enumerate().filter(|(_,v)|**v==1).map(|(i,_)|i).collect::<Vec<_>>()})).collect::<Vec<_>>(),"full_saturation":false},
 "requirements":[{"track":"explicit integral level77 factor map","status":"partial","verified":"evaluated diamond2 curve correspondence,CM residual Hecke obstruction,complete affine-linear ansatz exclusion","open":"actual integral factor inclusion,proper model,divisor evaluator and subgroup image"},{"track":"canonical theta matching and F2 descent","status":"partial","verified":"unique symmetric lifts and characters under written class bridge;selected-cover no-lift;explicit scalar refinement and actual Galois module","open":"mixed refined action,coherent F2 invariant basis,243 target sections and proper equations"},{"track":"four Q24 survivors","status":"verified exclusion under written source bridge","verified":"source-bound depth6 quartics;also Q26 excluded by depth5 cubic residuals","open":"Q28 arcs/separators,whole saturated ideal and proper quotient embedding"},{"track":"Lean proofs","status":"partial formalization","verified":"universal identities,character uniqueness,finite source bindings,no-lift/refinement matrices,102 exact bit-polynomial counterexample residues","open":"full modular,abelian and Miller geometry;search exhaustion and field implementation semantics"}],
 "failures_retained":["build-CM-03:missing serde Value import;corrected04","lean-initial-01:unit-extensionality and under-specified separator instantiation;corrected02","affine-linear ansatz:102 finite false positives independently rejected","selected diagonal cover:no integral target rotation lift","Q24 infinity hypothesis rejected by actual source separator;Q28 not classified","LMFDB web endpoint unavailable;frozen data and primary formulae used"],"source_sha256":sources,"raw_sha256":hashes,"performance_measured":false,"IC_efficiency_verdict":null});
    round02::save(&round, "ledger.json", &headline);
    let out = round.join("runs.sha256");
    assert!(!out.exists());
    std::fs::write(
        out,
        hashes
            .iter()
            .map(|(p, h)| format!("{h}  runs/{p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!(
        "Round18:{named} new named theorems;{audited} audited;{} source pins;{} raw files",
        sources.len(),
        hashes.len()
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn universal_identity_ast_controls() {
        for (i, (st, _)) in statements().into_iter().enumerate() {
            let cert = ic::issue(
                st,
                202610230300 + i as u64,
                64,
                "test exact universal residual AST",
                None,
            )
            .unwrap();
            assert!(matches!(
                ic::check_with_extra(&cert, Some((202610230400 + i as u64, 128))).unwrap(),
                ic::Verdict::Accept { .. }
            ));
        }
    }
}
