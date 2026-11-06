//! Write-once evidence and explicit historical/current source verification.
use super::*;
pub fn verify_sources(round: &Path, historical: bool) {
    let round = std::fs::canonicalize(round).unwrap();
    let study = round.parent().unwrap().parent().unwrap();
    let repo = study.parent().unwrap().parent().unwrap();
    let ledger = read(&round.join("ledger.json"));
    let sources = ledger["source_sha256"].as_object().unwrap();
    for (p, h) in sources {
        let actual = if historical {
            let arg = format!("1f733a2963736514f644aa8cff202d6e507c9dad:{p}");
            let output = std::process::Command::new("git")
                .args(["show", &arg])
                .current_dir(repo)
                .output()
                .unwrap();
            assert!(output.status.success(), "missing historical object:{arg}");
            hex::encode(sha256_impl::sha256(&output.stdout))
        } else {
            hash(&repo.join(p))
        };
        assert_eq!(actual, h.as_str().unwrap(), "source drift:{p}");
    }
    println!(
        "PASS:{} pinned sources;historical={historical}",
        sources.len()
    );
}
fn statements() -> Vec<ic::IdentityStatement> {
    let vars = |a: &[&str]| a.iter().map(|s| s.to_string()).collect();
    let st = |name: &str, theorem: &str, vs: Vec<String>, degree, lhs, rhs| ic::IdentityStatement {
        name: name.into(),
        source: format!("KoblitzRound15.{theorem}"),
        variables: vs,
        degree_bound: degree,
        lhs,
        rhs,
    };
    let theta = st(
        "theta-bias",
        "theta_bias_residual",
        vars(&["a", "b", "c", "d"]),
        2,
        sub(
            sub(
                mul(add(v("a"), v("c")), add(v("b"), v("d"))),
                mul(v("a"), v("b")),
            ),
            mul(v("c"), v("d")),
        ),
        add(mul(v("a"), v("d")), mul(v("c"), v("b"))),
    );
    let jet = st(
        "product-first-jet",
        "product_first_jet",
        vars(&["t", "x0", "x1", "y0", "y1"]),
        4,
        sub(
            sub(
                mul(
                    add(v("x0"), mul(v("t"), v("x1"))),
                    add(v("y0"), mul(v("t"), v("y1"))),
                ),
                mul(v("x0"), v("y0")),
            ),
            mul(v("t"), add(mul(v("x0"), v("y1")), mul(v("x1"), v("y0")))),
        ),
        mul(pow(v("t"), 2), mul(v("x1"), v("y1"))),
    );
    let f = add(v("x"), v("u"));
    let g = add(v("y"), v("v"));
    let s = mul(v("x"), v("u"));
    let r = mul(v("y"), v("v"));
    let cross = add(mul(v("x"), v("v")), mul(v("u"), v("y")));
    let coupled = st(
        "coupled-invariant",
        "coupled_invariant_residual",
        vars(&["x", "y", "u", "v"]),
        4,
        add(
            add(
                sub(pow(cross.clone(), 2), mul(mul(f.clone(), g.clone()), cross)),
                mul(pow(f, 2), r.clone()),
            ),
            mul(pow(g, 2), s.clone()),
        ),
        mul(c(4), mul(s, r)),
    );
    let curve = add(add(pow(v("x"), 3), pow(v("x"), 2)), c(1));
    let product = st(
        "coordinate-product",
        "coordinate_product_residual",
        vars(&["x", "y", "A", "B", "C", "D"]),
        5,
        sub(
            mul(
                add(v("A"), mul(v("B"), v("y"))),
                add(v("C"), mul(v("D"), v("y"))),
            ),
            add(
                add(mul(v("A"), v("C")), mul(mul(v("B"), v("D")), curve.clone())),
                mul(
                    add(
                        add(mul(v("A"), v("D")), mul(v("B"), v("C"))),
                        mul(mul(v("B"), v("D")), v("x")),
                    ),
                    v("y"),
                ),
            ),
        ),
        mul(
            mul(v("B"), v("D")),
            sub(sub(pow(v("y"), 2), mul(v("x"), v("y"))), curve),
        ),
    );
    let asymmetric = st(
        "asymmetric-target",
        "asymmetric_target_residual",
        vars(&["x", "u", "v"]),
        5,
        sub(
            mul(
                pow(v("x"), 2),
                sub(
                    sub(
                        sub(add(pow(v("v"), 2), mul(v("u"), v("v"))), pow(v("u"), 3)),
                        pow(v("u"), 2),
                    ),
                    c(1),
                ),
            ),
            sub(
                sub(
                    sub(
                        add(c(1), mul(v("x"), v("u"))),
                        mul(pow(v("x"), 2), pow(v("u"), 3)),
                    ),
                    mul(pow(v("x"), 2), pow(v("u"), 2)),
                ),
                pow(v("x"), 2),
            ),
        ),
        mul(
            sub(mul(v("x"), v("v")), c(1)),
            add(add(mul(v("x"), v("v")), c(1)), mul(v("x"), v("u"))),
        ),
    );
    vec![theta, jet, coupled, product, asymmetric]
}
pub fn identities(round: &Path, issue: bool) {
    let repo = round
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let path = "research/koblitz_modular_ic_20261018/lean/KoblitzRound15.lean";
    for (i, st) in statements().into_iter().enumerate() {
        let file = format!("identity-{}.json", st.name);
        if issue {
            let cert = ic::issue(
                st.clone(),
                202610211500 + i as u64,
                64,
                "native issuer;128 extra points and source-bound Lean theorem",
                Some(ic::ProofRef {
                    kind: "lean4".into(),
                    path: path.into(),
                    sha256: hash(&repo.join(path)),
                }),
            )
            .unwrap();
            round02::save(round, &file, &serde_json::to_value(cert).unwrap());
        }
        let cert: ic::IdentityCertificate =
            serde_json::from_value(read(&round.join(file))).unwrap();
        assert_eq!(cert.statement, st);
        assert_eq!(cert.proof.as_ref().unwrap().sha256, hash(&repo.join(path)));
        assert!(matches!(
            ic::check_with_extra(&cert, Some((202610211600 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }
    println!("Round15:5 parsed source-bound IDC certificates pass128 extra points each");
}
pub fn record(round: &Path) {
    let study = round.parent().unwrap().parent().unwrap();
    let repo = study.parent().unwrap().parent().unwrap();
    let raw = round.join("runs");
    let theta = read(&raw.join("g15-theta-class-descent.json"));
    let cm = read(&raw.join("cm15-one-prime-carriers.json"));
    let jets = read(&raw.join("q18-original-boundary-first-jets.json"));
    let supports = read(&raw.join("m16-new-pushed-supports.json"));
    assert_eq!(theta["sections_3lambda"], 243);
    assert_eq!(cm["LMFDB_level77_and539_polynomials_match"], true);
    assert_eq!(jets["all_four_unramified_first_jets_obstructed"], true);
    identities(round, true);
    let audit = std::fs::read_to_string(raw.join("lean-full-audit.log")).unwrap();
    assert!(!audit.contains("sorryAx"));
    let audited: u64 = audit
        .lines()
        .find(|l| l.starts_with("audited "))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    let mut named = 0;
    for name in [
        "KoblitzRound15",
        "KoblitzRound15Jets",
        "KoblitzRound15Theta",
        "KoblitzRound15CM",
    ] {
        let p = study.join(format!("lean/{name}.lean"));
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
        named += std::fs::read_to_string(p)
            .unwrap()
            .lines()
            .filter(|l| l.starts_with("theorem "))
            .count();
    }
    for p in [
        study.join("lean/lakefile.toml"),
        study.join("lean/lake-manifest.json"),
        study.join("native/Cargo.toml"),
        study.join("native/Cargo.lock"),
        repo.join("src/cryptanalysis/identity_certificate.rs"),
        repo.join("src/hash/sha256.rs"),
        repo.join(".github/workflows/koblitz-modular-carriers.yml"),
        round.join("REPLAY.sh"),
        round.join("CM.md"),
        round.join("THETA.md"),
        round.join("PROTOCOL.md"),
    ] {
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
    }
    let mut hashes = BTreeMap::new();
    walk(&raw, &raw, &mut hashes);
    round02::save(
        round,
        "ledger.json",
        &json!({"schema":"koblitz.round15-ledger/v1","client_date":"2026-10-21 America/Los_Angeles","execution_parent":"15c22cc670384c8e18506c7255667c8af4267e35","scientific_parent":"1f733a2963736514f644aa8cff202d6e507c9dad","theta":theta,"CM":cm,"first_jets":jets,"new_pushed_supports":supports,"new_named_Lean_theorems":named,"audited_project_theorems":audited,"requirements":[{"track":"new low-genus carriers","status":"verified finite data and written construction","open":"explicit Gamma_H equations,diamond twist maps,subgroup injection and divisor arithmetic"},{"track":"proper polarized embeddings","status":"partial","evidence":"actual rational symmetric theta class and unique rational19-lift;3lambda target P242","open":"section bases,theta functions,linearization matrices,saturated equations"},{"track":"full quotient saturation","status":"partial","evidence":"all four actual Q17 models obstructed at h=t","open":"ramified arcs,full ideal equality and projective boundary identification"},{"track":"new direct modular relations","status":"partial","evidence":"two exact new norms and complete pushed cusp/SS/ordinary partitions","open":"new source fibers/images,complete closed signed divisors and rank joins"},{"track":"Lean proofs","status":"partial","evidence":"universal residuals,actual gate/model binding,Picard/Cartier inverses,CM numeric certificates and n5 Euler identity","open":"global modular/abelian geometry and implementation semantics"}],"failures_and_negative_results":[{"file":"build-theta-01.log","reason":"Rust serde macro expression;corrected"},{"file":"supports-01.log","reason":"rejected false cusp-unit hypothesis;AmendmentB uses exact norm multiplicativity and F8 zero comparison"},{"file":"lean-structural-01.log","reason":"wrong working-directory invocation;no mathematical result"},{"file":"lean-new-03.log","reason":"unopened scoped matrix-vector notation in generated certificate;fixed explicit Matrix.mulVec"},{"file":"historical-sources-01.log","reason":"relative repository path produced an empty subprocess working directory;fixed canonical round path"},{"file":"build-record-07.log","reason":"incorrect proof pointer type name;fixed to ProofRef"},{"file":"lean-structural-02.log","reason":"affine uniqueness rewrite also rewrote c(y);fixed explicit subtraction congruence"},{"file":"KoblitzRound15Jets-attempt-01.lean","reason":"finite residual alone insufficient source binding;final module binds actual gate12 and original Q17 model"},{"experiment":"CM15 scan","reason":"all bounded exclusions retained in CM.scan;not universal nonexistence"},{"experiment":"Q18","reason":"h=t obstruction does not exclude h=t2 or ramified boundary arcs"}],"source_sha256":sources,"raw_sha256":hashes,"performance_measured":false,"IC_efficiency_verdict":null}),
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
        "Round15:{named} named Lean results,{audited} audited;{} sources,{} raw records",
        sources.len(),
        hashes.len()
    );
}
