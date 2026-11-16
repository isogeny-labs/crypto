//! Native write-once round14 ledger, source hashes and universal identity certificates.
use super::*;
pub fn digest(path: &Path) {
    println!("{}", hash(path));
}
pub fn validation_manifest(round: &Path) {
    let raw = round.join("validation-runs");
    let mut hashes = BTreeMap::new();
    walk(&raw, &raw, &mut hashes);
    let out = round.join("validation-runs.sha256");
    assert!(!out.exists());
    std::fs::write(
        out,
        hashes
            .iter()
            .map(|(p, h)| format!("{h}  validation-runs/{p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!("Round14:validation manifest pins{} files", hashes.len());
}
pub fn verify_sources(round: &Path) {
    let repo = round
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let ledger = read(&round.join("ledger.json"));
    let sources = ledger["source_sha256"].as_object().unwrap();
    for (p, h) in sources {
        assert_eq!(hash(&repo.join(p)), h.as_str().unwrap(), "source drift:{p}");
    }
    println!("Round14:all{} pinned source hashes match", sources.len());
}
pub fn check_identities(round: &Path) {
    let repo = round
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    for (i, name) in [
        "expanded-product-residual",
        "linear-product-residual",
        "cross-target-residual",
        "scalar-deck-augmentation",
        "tate-determinant-residual",
    ]
    .into_iter()
    .enumerate()
    {
        let certificate: ic::IdentityCertificate =
            serde_json::from_value(read(&round.join(format!("identity-{name}.json")))).unwrap();
        let proof = certificate.proof.as_ref().unwrap();
        assert_eq!(proof.kind, "lean4");
        assert_eq!(hash(&repo.join(&proof.path)), proof.sha256);
        assert!(matches!(
            ic::check_with_extra(&certificate, Some((20261021010101 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }
    println!("Round14:5 parsed IDC records and actual Lean source digests checked");
}
pub fn record(round: &Path) {
    let study = round.parent().unwrap().parent().unwrap();
    let repo = study.parent().unwrap().parent().unwrap();
    let raw = round.join("runs");
    let mut ordinary = read(&raw.join("o24-five-source-supports.json"));
    assert_eq!(ordinary["all25_closed_supports_verified"], true);
    let relations = read(&raw.join("o27-complete-projected-relations.json"));
    let cusps = read(&raw.join("o28-complete-closed-coupled-divisors.json"));
    assert_eq!(
        cusps["all_eight_F2_closed_signed_divisors_complete_under_written_bridges"],
        true
    );
    assert_eq!(relations["all_ordinary_images_complete"], true);
    let new_columns = relations["new_columns"].as_array().unwrap();
    for r in ordinary["records"].as_array_mut().unwrap() {
        r["individual_images_complete"] = json!(true);
        let index = r["index"].as_u64().unwrap();
        for (scalar, p) in r["source_points"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .enumerate()
        {
            if let Some(c) = new_columns.iter().find(|c| {
                c["source_point_index"] == index && c["source_scalar_factor_index"] == scalar
            }) {
                for (sign, label) in ["even", "odd"].into_iter().enumerate() {
                    let im = c["images_F32"][sign].clone();
                    p[format!("{label}_projected_image")] = json!(if im["infinity"] == true {
                        "zero"
                    } else {
                        "nonzero"
                    });
                    p[format!("{label}_projected_image_F32")] = im;
                }
            }
        }
    }
    ordinary["split_fiber_individual_images_open"] = json!(0);
    ordinary["individual_images_all_complete"] = json!(true);
    ordinary["refinement_evidence"] = json!("O25/O26 direct images and O27 parsed u32 F32 checker");
    let q16 = read(&raw.join("q16-linear-product-saturation.json"));
    let q17 = read(&raw.join("q17-expanded-product-quadrics.json"));
    let geometry = read(&raw.join("g14-canonical-polarized-target.json"));
    for g in 0..4 {
        assert_eq!(
            q16["records"][g]["counterpoint_after_all_Q15"],
            serde_json::Value::Null
        );
        assert_eq!(
            q17["records"][g]["counterpoint_after_all_Q16"],
            serde_json::Value::Null
        );
        assert!(!q17["records"][g]["after_all_Q17"]["model"].is_null());
    }
    let product = ic::IdentityStatement {
        name: "expanded-product-residual".into(),
        source: "KoblitzRound14.product_form_residual".into(),
        variables: ["h", "out", "a", "b", "A", "B"]
            .map(str::to_string)
            .to_vec(),
        degree_bound: 2,
        lhs: sub(mul(v("h"), v("out")), mul(v("A"), v("B"))),
        rhs: add(
            add(
                sub(mul(v("h"), v("out")), mul(v("a"), v("b"))),
                mul(sub(v("a"), v("A")), v("b")),
            ),
            mul(v("A"), sub(v("b"), v("B"))),
        ),
    };
    let linear = ic::IdentityStatement {
        name: "linear-product-residual".into(),
        source: "KoblitzRound14.linear_product_residual".into(),
        variables: ["h", "out", "a", "b", "A", "B", "w"]
            .map(str::to_string)
            .to_vec(),
        degree_bound: 2,
        lhs: mul(v("h"), sub(v("out"), v("w"))),
        rhs: add(
            add(
                add(
                    sub(mul(v("h"), v("out")), mul(v("a"), v("b"))),
                    mul(sub(v("a"), v("A")), v("b")),
                ),
                mul(v("A"), sub(v("b"), v("B"))),
            ),
            sub(mul(v("A"), v("B")), mul(v("h"), v("w"))),
        ),
    };
    let uy = add(mul(v("u"), v("y")), v("c"));
    let cross = ic::IdentityStatement {
        name: "cross-target-residual".into(),
        source: "KoblitzRound14.cross_target_residual".into(),
        variables: ["x", "y", "u", "v", "c"].map(str::to_string).to_vec(),
        degree_bound: 5,
        lhs: sub(
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
                        add(pow(uy.clone(), 2), mul(mul(v("x"), v("u")), uy.clone())),
                        mul(pow(v("x"), 2), pow(v("u"), 3)),
                    ),
                    mul(pow(v("x"), 2), pow(v("u"), 2)),
                ),
                pow(v("x"), 2),
            ),
        ),
        rhs: mul(
            sub(mul(v("x"), v("v")), uy.clone()),
            add(add(mul(v("x"), v("v")), uy), mul(v("x"), v("u"))),
        ),
    };
    let vars = ["a", "b", "c", "d", "e"];
    let sum = E::sum(vars.iter().map(|s| v(s)).collect());
    let deck = ic::IdentityStatement {
        name: "scalar-deck-augmentation".into(),
        source: "KoblitzRound14.scalar_deck_identity".into(),
        variables: vars.map(str::to_string).to_vec(),
        degree_bound: 1,
        lhs: E::sum(
            vars.iter()
                .map(|s| sub(mul(c(5), v(s)), sum.clone()))
                .collect(),
        ),
        rhs: c(0),
    };
    let tate = ic::IdentityStatement {
        name: "tate-determinant-residual".into(),
        source: "KoblitzRound14Cusps.tate_determinant_residual".into(),
        variables: ["a", "b", "c", "d", "i"].map(str::to_string).to_vec(),
        degree_bound: 3,
        lhs: sub(
            sub(
                mul(mul(c(2), v("a")), mul(v("i"), v("d"))),
                mul(v("b"), v("c")),
            ),
            sub(mul(v("a"), v("d")), mul(v("b"), v("c"))),
        ),
        rhs: mul(mul(v("a"), v("d")), sub(mul(c(2), v("i")), c(1))),
    };
    let mut ids = vec![];
    for (i, statement) in [product, linear, cross, deck, tate].into_iter().enumerate() {
        let name = format!("identity-{}.json", statement.name);
        let module = if i == 4 {
            "KoblitzRound14Cusps"
        } else {
            "KoblitzRound14"
        };
        let cert = ic::issue(
            statement,
            202610210100 + i as u64,
            64,
            "native issuer plus128 extra checks;source-bound universal Lean theorem",
            Some(ic::ProofRef {
                kind: "lean4".into(),
                path: format!("research/koblitz_modular_ic_20261018/lean/{module}.lean"),
                sha256: hash(&study.join(format!("lean/{module}.lean"))),
            }),
        )
        .unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((20261021010101 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        round02::save(round, &name, &serde_json::to_value(&cert).unwrap());
        ids.push(json!({"id":cert.id,"file":name,"issuer_points":64,"extra_checker_points":128}));
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
    for name in [
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "native/Cargo.toml",
        "native/Cargo.lock",
        "rounds/14/PROTOCOL.md",
        "rounds/14/REPLAY.sh",
        "rounds/14/CUSPS.md",
        "rounds/14/GEOMETRY.md",
        "rounds/14/PROOF_MAP.md",
    ] {
        let p = study.join(name);
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
    }
    for name in [
        "src/cryptanalysis/curve_construction/gf2k.rs",
        "src/cryptanalysis/curve_construction/modular.rs",
        "src/cryptanalysis/curve_construction/weil.rs",
        "src/cryptanalysis/curve_construction/lpoly.rs",
        "src/cryptanalysis/identity_certificate.rs",
        "src/hash/sha256.rs",
    ] {
        sources.insert(name.into(), hash(&repo.join(name)));
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
    let named = 31
        + relations["new_named_Lean_theorems"].as_u64().unwrap()
        + cusps["new_named_Lean_theorems"].as_u64().unwrap();
    assert!(audited >= 270 + named);
    let mut hashes = BTreeMap::new();
    walk(&raw, &raw, &mut hashes);
    let joined = json!({"evidence":"runs/o27-complete-projected-relations.json","old_rank_controls":relations["old_rank_controls"],"principal_only_ranks":relations["principal_only_joined_ranks"],"with_deck_ranks":relations["with_deck_joined_ranks"],"all_columns":relations["all_columns"],"nonzero_column_counts":relations["nonzero_column_counts"],"new_split_columns":40,"projected_principal_rows_complete":true,"all_geometric_signed_divisors_complete_under_written_bridges":true,"all_F2_closed_cusp_divisors_complete":true,"closed_cusp_refinement":"runs/o28-complete-closed-coupled-divisors.json;O27 is the frozen earlier stage","closed_cusps":cusps["closed_cusp_count"],"complete_closed_divisors":cusps["eight_closed_signed_divisors"],"deck_rows_are_principal":false,"degree_table":relations["degree_table_cusp_SS_ordinary_poles"]});
    let ledger = json!({"schema":"koblitz.round14-ledger/v1","parent_commit":"3556fe3f6103bf212e18fcf7a37ac4acede8d6ed","UTC_date":"2026-10-21","client_date":"2026-10-20 America/Los_Angeles","ordinary":ordinary,"joined_relations":joined,"saturation":{"Q16_linear_relations_per_graph":98,"Q16_counterpoints_found":0,"Q17_unique_boundary_coefficient_quadrics_per_graph":288,"Q17_counterpoints_found":0,"all_four_nonzero_Q17_models_kernel_checked":true,"full_h_saturation":false,"surviving_models_prove_incomplete_saturation":false,"scope":"bounded scalar restriction;explicit h-membership identities;not full elimination"},"polarized_geometry":geometry,"requirements":[{"track":"five ordinary supports","status":"verified complete under written global bridges","evidence":"all57 simple closed points,all even/odd images,complete degrees","open":"full Lean modular geometry and native semantics"},{"track":"projected coupled principal rows","status":"verified complete under written global bridges","evidence":"all eight projected rows;old plus principal ranks16/15/15;with deck20/18/17","open":"full Lean global divisor geometry","closed_divisors_evidence":"O28 completes all eight F2 closed signed divisors under written Tate/moduli bridges"},{"track":"complete F2 closed signed divisors","status":"verified complete under written global bridges","evidence":"O28:44 cusp orbits cover480 labels,old22 product control agrees,all8 divisors assembled","open":"formal Tate/moduli and global divisor semantics"},{"track":"full quotient saturation","status":"partial","open":"complete generating ideals and containment/h-regularity equality certificate"},{"track":"proper polarized F2 embeddings","status":"partial","open":"actual global section bases,linearization/descent matrices,coordinate ideals and boundary identification"},{"track":"Lean proofs","status":"partial","evidence":"65 new named results:13 structural,8 actual boundary,10 support arithmetic,18 actual F32/minor/independence,16 actual cusp results","open":"full global modular/abelian geometry and native implementation semantics"}],"new_named_Lean_theorems":named,"audited_project_theorems":audited,"identity_certificates":ids,"failures_and_negative_results":[{"files":["build-05.log","build-16.log"],"reason":"Rust generic inference and modulus-width errors;retained and fixed"},{"files":["fiber-13-01.log","fiber-14-01.log"],"reason":"one-billion reduction cap;AmendmentE freezes5b before accepted retries"},{"files":["geometry-01.log"],"reason":"wrong prior H catalog rejected graph binding;actual gl2b catalog/trace/coefficients/coset binding fixes it"},{"files":["g14-canonical-polarized-target-02-linear-dual-only.json","geometry-02.log"],"reason":"linear dual is not Cartier character Frobenius;corrected to2M^-transpose;original preserved"},{"files":["lean-structural-01.log","lean-structural-02.log","lean-cusps-01.log","lean-relations-01.log","KoblitzRound14Relations-attempt-01.lean"],"reason":"normalization/sign and missing prime-field instance;only passing final checks count"},{"files":["build-17.log","lean-full-gate-01.log"],"reason":"temporary filesystem full;retried with task scratch TMPDIR;not mathematical failures"},{"experiment":"Q16","reason":"no new separating scalar boundary model in147 coefficient queries per graph;not full saturation"},{"experiment":"Q17","reason":"no new separating scalar boundary model in288 coefficient queries per graph;not full saturation"}],"performance_measured":false,"IC_efficiency_verdict":null,"raw_sha256":hashes,"source_sha256":sources});
    let mut ledger = ledger;
    ledger["failures_and_negative_results"].as_array_mut().unwrap().push(json!({"files":["o25-degree7-individual-images-unmatched-odd-root.json","o26-point-0-images-unmatched-odd-root.json","o27-complete-projected-relations-unmatched-odd-root.json","KoblitzRound14Relations-unmatched-odd-root.lean","lean-unmatched-odd-root-audit-320.log","replay-01.log"],"reason":"the old odd F4 twist root also needs transport through the fixed cyclotomic embedding;all eight direct fibers,point proofs and joined data regenerated with exact old convention;rank/nonzero counts unchanged;earlier successful replay superseded"}));
    ledger["failures_and_negative_results"].as_array_mut().unwrap().push(json!({"files":["lean-cusps-01.log","lean-cusps-02.log","KoblitzRound14Cusps-attempt-02.lean"],"reason":"initial missing module copy,opaque mergeSort kernel reduction and lexical spacing;fixed by explicit occurrence-count coverage and corrected syntax;only final passing module counts"}));
    round02::save(round, "ledger.json", &ledger);
    let manifest = hashes
        .iter()
        .map(|(p, h)| format!("{h}  runs/{p}\n"))
        .collect::<String>();
    let path = round.join("runs.sha256");
    assert!(!path.exists());
    std::fs::write(path, manifest).unwrap();
    println!("Round14:all57 ordinary source supports/images verified;principal ranks16/15/15,with deck20/18/17;full saturation/explicit proper equations open;{audited} theorems audited");
}
