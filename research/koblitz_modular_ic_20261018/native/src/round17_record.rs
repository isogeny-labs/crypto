//! Write-once Round17 summary, source pins and parsed source-bound identity checks.
use super::*;
fn statements() -> Vec<(ic::IdentityStatement, &'static str)> {
    let norm = |a: E, b: E| {
        add(
            sub(pow(a.clone(), 2), mul(a, b.clone())),
            mul(c(2), pow(b, 2)),
        )
    };
    let (a, b, cc, d) = (v("a"), v("b"), v("c"), v("d"));
    let cm = ic::IdentityStatement {
        name: "CM-norm-product".into(),
        source: "KoblitzRound17LiftProof.CM_norm_product".into(),
        variables: ["a", "b", "c", "d"].map(str::to_string).to_vec(),
        degree_bound: 4,
        lhs: norm(
            sub(
                mul(a.clone(), cc.clone()),
                mul(c(2), mul(b.clone(), d.clone())),
            ),
            sub(
                add(mul(a.clone(), d.clone()), mul(b.clone(), cc.clone())),
                mul(b.clone(), d.clone()),
            ),
        ),
        rhs: mul(norm(a, b), norm(cc, d)),
    };
    let (x, y, a) = (v("x"), v("y"), v("a"));
    let yy = add(y.clone(), c(1));
    let tr = ic::IdentityStatement {
        name: "T-translation-x".into(),
        source: "KoblitzRound17.T_translation_x_residual".into(),
        variables: ["x", "y", "a"].map(str::to_string).to_vec(),
        degree_bound: 3,
        lhs: sub(
            add(
                add(pow(yy.clone(), 2), mul(yy, x.clone())),
                mul(add(x.clone(), a.clone()), pow(x.clone(), 2)),
            ),
            x.clone(),
        ),
        rhs: add(
            add(
                add(
                    add(pow(y.clone(), 2), mul(x.clone(), y.clone())),
                    pow(x.clone(), 3),
                ),
                mul(a, pow(x, 2)),
            ),
            add(c(1), mul(c(2), y)),
        ),
    };
    vec![
        (crate::liftproof17::statement(), "KoblitzRound17LiftProof"),
        (cm, "KoblitzRound17LiftProof"),
        (tr, "KoblitzRound17"),
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
                202610230100 + i as u64,
                64,
                "native issuer;128 extra parsed checks plus source-bound kernel theorem",
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
            ic::check_with_extra(&cert, Some((202610230200 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }
    println!("Round17:3 source-bound IDC records pass128 extra parsed checks each");
}
pub fn record(round: &Path) {
    let round = std::fs::canonicalize(round).unwrap();
    let study = round.ancestors().nth(2).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    let raw = round.join("runs");
    let doc = |s: &str| read(&raw.join(s));
    let audit = std::fs::read_to_string(raw.join("lean-full-audit.log")).unwrap();
    assert!(!audit.contains("sorryAx"));
    assert!(audit.contains("0 failing, 0 trusted"));
    let audited: u64 = audit
        .lines()
        .find(|l| l.starts_with("audited "))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let q = doc("q23-survivor-source-separators.json");
    assert!(q["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["Q22_model_excluded"] == true));
    let cm = doc("cm18-integral-Frobenius-lattice-controls.json");
    let theta = doc("g17-odd-cover-kernel-and-commutators.json");
    let lifts = doc("g18-factored-odd-kernel-theta-lifts.json");
    assert_eq!(theta["records"][0]["kernel_order"], 99225);
    assert_eq!(lifts["records"].as_array().unwrap().len(), 2);
    identities(&round, true);
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    for e in std::fs::read_dir(study.join("lean")).unwrap() {
        let p = e.unwrap().path();
        if p.extension().is_some_and(|x| x == "lean") {
            sources.insert(
                p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
        }
    }
    let mut named = 0;
    for m in [
        "KoblitzRound17",
        "KoblitzRound17Integral",
        "KoblitzRound17Theta",
        "KoblitzRound17Survivors",
        "KoblitzRound17Boundary",
        "KoblitzRound17LiftProof",
    ] {
        named += std::fs::read_to_string(study.join(format!("lean/{m}.lean")))
            .unwrap()
            .lines()
            .filter(|l| l.starts_with("theorem "))
            .count();
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
    let q24 = doc("q24-after-all-ordinary-cube-repairs.json");
    let models=q24["records"].as_array().unwrap().iter().map(|r|json!({"graph":r["graph"],"visits":r["total_visits"],"zero_nodes":r["new_boundary_zero_nodes"],"nonzero_nodes":r["model"].as_array().unwrap().iter().enumerate().filter(|(_,v)|**v==1).map(|(i,_)|i).collect::<Vec<_>>()})).collect::<Vec<_>>();
    round02::save(
        &round,
        "ledger.json",
        &json!({"schema":"koblitz.round17-ledger/v1","client_date":"2026-10-22 America/Los_Angeles","UTC_date":"2026-10-23","scientific_parent":"dcc9802e717465708a3e69562f4498bd7046def4","local_base_merge":"74359952fbebe6173f8db7ae73c7f5baef1d66a0","base_parent":"6ed792e860c9c8249eb1ac6f10a3e4beec130e1a","new_named_Lean_theorems":named,"audited_project_theorems":audited,"CM18":{"eight_simple_roots_mod11_each_sign":true,"common_roots":cm["common_roots_mod11"],"split_fixed11_dimension":cm["split_fixed11_dimension"],"cyclic_fixed11_dimension":cm["cyclic_fixed11_dimension"],"integral_forward_preserves_fixed11":true,"natural_reverse_kills_fixed11":true,"prime_to11_geometric_injection_existence":"written Tate/local-lattice argument conditional on actual compact modular/Jacobian identification","actual_level77_correspondence_evaluated":false},"G17":{"CM_Smith_diagonal":[1,1,3,105],"kernel_order":99225,"kernel_generator_orders":[3,3,105,105],"torsion_field_degree":168,"split_cover_compositum_degree":840,"actual_torsion_and_pairings_checked":true},"G18":{"factored_theta_lifts_per_graph":4,"three_period_probes_per_graph":true,"commutator_probes_per_graph":18,"Pic2_square_controls_per_graph":48,"normalizing_roots_verified":true,"canonical_mixed_divisor_character_matched":false,"target243_sections_extracted":false},"Q23":{"source_bound_quartics":16,"membership_h_depth":6,"all_four_Q22_survivors_excluded":true},"Q24_new_survivors":models,"requirements":[{"track":"integral level77 map","status":"partial","verified":"actual source matrices;integral split/glued lattice controls;written prime-to11 injection existence argument","open":"actual proper model/Tate lattice,integral geometric correspondence and divisor evaluator"},{"track":"theta descent on odd cover","status":"partial","verified":"Smith kernel,actual torsion/commutators,factored normalized translation lifts and structural averaging proof","open":"canonical mixed-divisor character,F32/F2 descent compatibility,explicit invariant basis and243 target sections"},{"track":"four saturation survivors","status":"verified exclusion under source interpretation","verified":"all four Q22 points excluded by source-bound depth6 quartics;finite source binding proofs","open":"four new Q24 survivors,whole ideal equality,proper quotient embeddings"},{"track":"Lean proofs","status":"partial formalization","verified":"structural implications,actual finite matrices and boundary bindings,universal Pic2 square residual","open":"Tate/abelian geometry,native field semantics,Miller divisors and complete modular geometry"}],"failures_retained":["build-theta-01.log:Rust BigInt type inference","build-lifts-01.log:serde expression syntax","lifts-01.log:stale binary rejected unknown flag before research execution","lean-structural-01.log:numeral cast rewrite in averaging lemma","lean-lift-proof-01.log:rational denominator clearing;corrected with explicit nonzero hypotheses","first G17 output retained before inverse-certificate extension","first G18 output retained before independent Pic2 square controls","original Tate PDF web fetch timed out;Stanford proof notes read"],"source_sha256":sources,"raw_sha256":hashes,"performance_measured":false,"IC_efficiency_verdict":null}),
    );
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
        "Round17:{named} new named theorems;{audited} audited;{} source pins;{} raw files",
        sources.len(),
        hashes.len()
    );
}
