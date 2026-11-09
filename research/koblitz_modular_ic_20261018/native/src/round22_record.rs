//! Source-bound Round22 evidence, exact identities and explicit open obligations.
use super::*;
fn statements() -> Vec<ic::IdentityStatement> {
    let (a, b, y) = (v("a"), v("b"), v("y"));
    let norm = ic::IdentityStatement {
        name: "quadratic-norm-integer-lift".into(),
        source: "KoblitzRound22Base.quadratic_norm_integer_lift".into(),
        variables: ["a", "b", "y"].map(str::to_owned).to_vec(),
        degree_bound: 4,
        lhs: mul(
            add(a.clone(), mul(b.clone(), y.clone())),
            add(a.clone(), mul(b.clone(), add(y.clone(), c(1)))),
        ),
        rhs: add(
            add(
                add(pow(a.clone(), 2), mul(a.clone(), b.clone())),
                mul(pow(b.clone(), 2), add(pow(y.clone(), 2), y.clone())),
            ),
            mul(c(2), mul(mul(a, b), y)),
        ),
    };
    let (u, y) = (v("u"), v("y"));
    let r = add(mul(add(u.clone(), c(1)), y.clone()), c(1));
    let cof = add(
        r.clone(),
        (1..=5).fold(c(0), |s, i| add(s, pow(u.clone(), i))),
    );
    let base = ic::IdentityStatement {
        name: "base11-integer-lift".into(),
        source: "KoblitzRound22Base.base11_integer_lift".into(),
        variables: ["u", "y"].map(str::to_owned).to_vec(),
        degree_bound: 5,
        lhs: add(
            add(pow(r.clone(), 2), mul(r, add(u.clone(), c(1)))),
            pow(u.clone(), 5),
        ),
        rhs: add(
            mul(
                pow(add(u.clone(), c(1)), 2),
                sub(add(pow(y.clone(), 2), y), add(pow(u.clone(), 3), u)),
            ),
            mul(c(2), cof),
        ),
    };
    let (t, w) = (v("t"), v("w"));
    let m = add(
        add(add(c(2), w.clone()), mul(c(7), t.clone())),
        mul(add(c(6), mul(c(2), w.clone())), pow(t.clone(), 3)),
    );
    let b = add(
        add(
            add(
                add(c(1), mul(c(3), w.clone())),
                mul(add(c(2), mul(c(5), w.clone())), t.clone()),
            ),
            mul(c(3), pow(t.clone(), 2)),
        ),
        mul(add(c(1), mul(c(2), w.clone())), pow(t.clone(), 3)),
    );
    let cc = add(
        sub(
            add(c(-2), mul(c(4), w.clone())),
            mul(c(3), pow(t.clone(), 2)),
        ),
        mul(add(c(-2), mul(c(3), w.clone())), pow(t.clone(), 3)),
    );
    let qw = add(
        add(
            add(
                add(c(3), mul(c(5), t.clone())),
                mul(c(8), pow(t.clone(), 3)),
            ),
            mul(c(10), pow(t.clone(), 4)),
        ),
        mul(c(4), pow(t.clone(), 6)),
    );
    let qt = add(
        add(
            add(c(-19), mul(c(32), w.clone())),
            mul(sub(c(20), mul(c(4), w.clone())), t.clone()),
        ),
        mul(add(c(-2), mul(c(10), w.clone())), pow(t.clone(), 2)),
    );
    let phi = (1..=4).fold(c(1), |s, i| add(s, pow(t.clone(), i)));
    let two = ic::IdentityStatement {
        name: "small-two-map-integer-lift".into(),
        source: "KoblitzRound22TwoMap.two_map_integer_lift".into(),
        variables: ["t", "w"].map(str::to_owned).to_vec(),
        degree_bound: 8,
        lhs: sub(add(mul(b, m), mul(c(7), cc)), c(1)),
        rhs: add(mul(qw, add(add(pow(w.clone(), 2), w), c(2))), mul(qt, phi)),
    };
    vec![norm, base, two]
}
pub fn identities(round: &Path, issue: bool) {
    let round = std::fs::canonicalize(round).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    for (i, st) in statements().into_iter().enumerate() {
        let path = if i == 2 {
            "research/koblitz_modular_ic_20261018/lean/KoblitzRound22TwoMap.lean"
        } else {
            "research/koblitz_modular_ic_20261018/lean/KoblitzRound22Base.lean"
        };
        let name = format!("identity-{}.json", st.name);
        if issue {
            let cert = ic::issue(
                st.clone(),
                202610260300 + i as u64,
                64,
                "native issuer;separate kernel proof;128 additional parsed checks",
                Some(ic::ProofRef {
                    kind: "lean4".into(),
                    path: path.into(),
                    sha256: hash(&repo.join(path)),
                }),
            )
            .unwrap();
            assert!(!round.join(&name).exists());
            round02::save(&round, &name, &serde_json::to_value(cert).unwrap());
        }
        let cert: ic::IdentityCertificate =
            serde_json::from_value(read(&round.join(&name))).unwrap();
        assert_eq!(cert.statement, st);
        assert_eq!(cert.proof.as_ref().unwrap().sha256, hash(&repo.join(path)));
        assert!(matches!(
            ic::check_with_extra(&cert, Some((202610260400 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }
    println!("Round22:3 source-bound universal integer lifts;128 additional parsed checks each");
}
fn summary_value(raw: &Path) -> serde_json::Value {
    let g = read(&raw.join("g32-normzero-integral-recovery.json"));
    let gi = read(&raw.join("g32-independent-integer-replay.json"));
    assert_eq!(g["least_positive_recovery_scalar"], "1");
    assert_eq!(g["Smith_rank"], 8);
    assert_eq!(g["F32_order11_fixtures"].as_array().unwrap().len(), 22);
    assert_eq!(gi["all_Smith_factors_one"], true);
    assert_eq!(gi["kernel_rank"], 24);
    assert_eq!(
        gi["augmentation_metric_Smith_diagonal"],
        json!([1, 1, 1, 5])
    );
    assert_eq!(gi["replacing_induced_metric_by_identity_rejected"], true);
    let small = read(&raw.join("g33-small-two-map-recovery.json"));
    assert_eq!(small["both_sign_order11_fixture_checks"], 22);
    assert_eq!(small["commutation_verified"], true);
    assert_eq!(small["B0_and_B2_zero"], true);
    let q = read(&raw.join("q41-node187-coupled-valuations.json"));
    let qs = q["records"].as_array().unwrap();
    assert_eq!(qs.len(), 4);
    assert!(qs.iter().all(
        |r| r["old_scalar_model_excluded_under_written_DVR_bridge"] == true
            && r["explicit_polynomial_separator_extracted"] == false
    ));
    let b = read(&raw.join("cm22-base-divisors.json"));
    assert_eq!(b["Tate11_substitution_zero"], true);
    assert_eq!(b["q5_hex"], "3d");
    let t = read(&raw.join("cm22-toric-model.json"));
    assert_eq!(t["degree_after"], 72);
    assert_eq!(t["tested_unimodular_matrices"], 1384);
    let mut out = json!({"schema":"koblitz.round22-summary/v1","recovery":{"regular_target_R21_least_scalar":5,"augmentation_target_least_scalar":1,"Smith_invariants":[1,1,1,1,1,1,1,1],"integer_system_rows":8,"integer_system_columns":32,"Smith_operations":g["Smith_operations"],"both_sign_order11_fixtures":22,"M_polynomial_coefficients":gi["M_polynomial_coefficients"],"B_polynomial_coefficients":gi["B_polynomial_coefficients"]},"coupled":{"old_node187_models":4,"excluded_under_written_proper_DVR_bridge":4,"ordinary_function_conjugates_checked":80,"valuation_controls":q["finite_integer_controls"],"explicit_polynomial_separator":false,"full_saturation":false},"geometry":{"base_genus":1,"base_divisors":b["complete_divisors_on_base"],"residual_closed_support_degree":5,"residual_polynomial_hex":"3d","generic_QR_degree":16,"plane_projection_degree":32,"plane_model_degree_initial":96,"plane_model_degree_toric":72,"toric_matrices_checked":1384,"proper_level77_divisors_complete":false,"complete_prescribed_Riemann_Roch_space":false,"modular_factor_map":false},"reverse_evaluator_arbitrary_modular_divisors":false,"ordinary_460_367_images_resolved":false,"new_IC_relation_rank":null,"performance_measured":false,"IC_efficiency_verdict":null});
    out["recovery"]["small_two_map_certificate"] = small;
    out["recovery"]["kernel_rank"] = json!(24);
    out["recovery"]["induced_metric_Smith_diagonal"] =
        gi["augmentation_metric_Smith_diagonal"].clone();
    let exceptional = read(&raw.join("cm22-exceptional-supports.json"));
    assert_eq!(exceptional["qSS_hex"], "29");
    assert_eq!(exceptional["qSS_irreducible"], true);
    out["geometry"]["exceptional_base_divisors"] = exceptional;
    let ss = std::fs::read_to_string(raw.join("cm22-ss5-fiber-02.log")).unwrap();
    assert!(ss.contains("SS5_FIBER_ALL_CHECKS_PASS"));
    assert!(!ss.contains("error") && !ss.contains("?") && !ss.contains("failed"));
    assert!(ss.contains("FROBENIUS_MINUS_I_RANK\n12\n"));
    assert!(ss.contains("FROBENIUS_SQUARED_MINUS_I_RANK\n8\n"));
    let input = read(&raw.join("cm22-input-circuit.json"));
    assert_eq!(input["residuals"], json!([48, 52, 238, 240, 242]));
    assert_eq!(input["units"], json!([0, 11, 1, 1, 19, 36, 64, 74]));
    out["geometry"]["SS5_fiber"] = json!({
        "source":"original unchanged incidence residuals and all registered units",
        "coefficient_field":"F2[a]/(a^5+a^3+1)","b":"a+1","c":1,
        "ideal_generators":["lambda^4+b*lambda+b^2","nu^4+b^3*nu+b^4*(lambda+1)"],
        "both_lift_residuals_zero":true,"dimension":16,"all_registered_units_invertible":true,
        "lambda_and_nu_units":true,"relative_Jacobian":"b^4",
        "Frobenius_minus_I_rank":12,"Frobenius_squared_minus_I_rank":8,
        "Frobenius_fourth_is_identity":true,"residue_degrees_over_F32":[4,4,4,4],
        "closed_point_degrees_over_F2":[20,20,20,20],
        "proper_completion_branch_table_certified":false
    });
    out
}
pub fn summary(raw: &Path) {
    let s = summary_value(raw);
    assert!(!raw.join("round22-native-summary.json").exists());
    round02::save(raw, "round22-native-summary.json", &s);
    println!("Round22:G32 integral recovery d=1;Q41 coupled valuative exclusion;complete base divisors;proper level77 and modular reverse evaluator remain open");
}
fn test_counts(text: &str) -> serde_json::Value {
    let line = text
        .lines()
        .rev()
        .find(|l| l.starts_with("test result:"))
        .expect("missing final Cargo test summary");
    assert!(
        line.starts_with("test result: ok."),
        "final Cargo test summary did not succeed: {line}"
    );
    let fields = line.split_whitespace().collect::<Vec<_>>();
    let count = |unit: &str| {
        let i = fields
            .iter()
            .position(|f| f.trim_end_matches(';') == unit)
            .unwrap();
        fields[i - 1].parse::<usize>().unwrap()
    };
    assert_eq!(count("failed"), 0);
    json!({"passed":count("passed"),"failed":0,"ignored":count("ignored")})
}

pub fn record(round: &Path) {
    let round = std::fs::canonicalize(round).unwrap();
    let study = round.ancestors().nth(2).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    let raw = round.join("runs");
    assert_eq!(
        read(&raw.join("round22-native-summary.json")),
        summary_value(&raw)
    );
    let audit = std::fs::read_to_string(raw.join("lean-full-audit.log")).unwrap();
    assert!(audit.contains("0 failing, 0 trusted"));
    assert!(!audit.contains("sorryAx") && !audit.contains("FAIL"));
    let audited: usize = audit
        .lines()
        .find(|s| s.starts_with("audited "))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let build = std::fs::read_to_string(raw.join("lean-full-build-final.log")).unwrap();
    assert!(build.contains("Build completed successfully"));
    let root = std::fs::read_to_string(raw.join("root-tests-integrated.log")).unwrap();
    let native = std::fs::read_to_string(raw.join("native-tests-final.log")).unwrap();
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    let mut modules = BTreeMap::new();
    let mut named = 0;
    for e in std::fs::read_dir(study.join("lean")).unwrap() {
        let p = e.unwrap().path();
        if p.extension().is_some_and(|e| e == "lean") {
            sources.insert(
                p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
            let module = p.file_stem().unwrap().to_str().unwrap();
            if module.starts_with("KoblitzRound22") {
                let text = std::fs::read_to_string(&p).unwrap();
                let mut n = 0;
                for l in text.lines().filter(|s| s.starts_with("theorem ")) {
                    let name = l
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .split(['{', '(', ':'])
                        .next()
                        .unwrap();
                    assert!(audit.contains(&format!("{module}.{name}")));
                    n += 1;
                }
                modules.insert(module.to_owned(), n);
                named += n;
            }
        }
    }
    assert_eq!(named, 51);
    for p in [
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "native/Cargo.toml",
        "native/Cargo.lock",
    ] {
        let p = study.join(p);
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
    }
    for p in [
        "src/cryptanalysis/curve_construction/gf2k.rs",
        "src/cryptanalysis/curve_construction/weil.rs",
        "src/cryptanalysis/curve_construction/modular.rs",
        "src/cryptanalysis/curve_construction/lpoly.rs",
        "src/cryptanalysis/identity_certificate.rs",
        "src/hash/sha256.rs",
        "src/cryptanalysis/sat.rs",
        "AGENTS.md",
        "tools/progress_metrics.py",
        "tools/lean/proof-gate.sh",
        "tools/lean/AuditAxioms.lean",
        "tools/lean/trusted-native.txt",
        ".github/workflows/koblitz-modular-carriers.yml",
    ] {
        sources.insert(p.into(), hash(&repo.join(p)));
    }
    for p in [
        "PROTOCOL.md",
        "REPLAY.sh",
        "REPORT.md",
        "RECOVERY.md",
        "CM.md",
        "SATURATION.md",
        "SOURCES.md",
        "PROOF_MAP.md",
        "VALIDATION.md",
        "SS5.sing",
        "REPLAY_SS5.sh",
    ] {
        let p = round.join(p);
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
    }
    identities(&round, false);
    let incidence = std::fs::read_to_string(raw.join("cm22-incidence.sing")).unwrap();
    let prefix = incidence.split("poly unitprod=1;").next().unwrap();
    let fragment = std::fs::read_to_string(round.join("SS5.sing")).unwrap();
    assert_eq!(
        std::fs::read_to_string(raw.join("cm22-ss5-fiber-02.sing")).unwrap(),
        format!("{prefix}{fragment}")
    );
    let cache = read(&raw.join("proof-cache-manifest.json"));
    let sig = read(&raw.join("proof-cache-ir-signatures.json"));
    assert_eq!(cache["all_source_destination_hashes_equal"], true);
    assert_eq!(sig["all_source_destination_hashes_equal"], true);
    assert!(sig["combined_bytes"].as_u64().unwrap() <= 8 * 1024 * 1024 * 1024);
    let eks = read(&raw.join("eks-overview-final.json"));
    let jobs = eks["jobs"].as_array().unwrap();
    let active = jobs
        .iter()
        .filter(|j| j["state"] != "succeeded" && j["state"] != "failed")
        .count();
    let matching = jobs
        .iter()
        .filter(|j| {
            let n = j["job_name"].as_str().unwrap();
            n.contains("koblitz") || n.contains("modular")
        })
        .count();
    assert_eq!((active, matching), (0, 0));
    let mut cas = vec![];
    for name in [
        "plane-places",
        "toric-places",
        "open-smooth",
        "integral-basis",
        "generic-certificate",
        "projection-certificate",
    ] {
        let status = std::fs::read_to_string(raw.join(format!("cm22-{name}-01.exit"))).unwrap();
        let log = std::fs::read_to_string(raw.join(format!("cm22-{name}-01.log"))).unwrap();
        cas.push(json!({"name":name,"exit":status.trim().parse::<i32>().unwrap(),"reported_error":log.contains("error")||log.contains("halt ")}));
    }
    let corrected = std::fs::read_to_string(raw.join("cm22-generic-certificate-03.log")).unwrap();
    assert!(!corrected.contains("error") && !corrected.contains("failed"));
    assert!(corrected.contains("forwardResidual[1,1]=0") && corrected.contains("bres[1,1]=0"));
    for (name, marker) in [
        ("trace-tower-02", "TRACE_TOWER_RESIDUAL"),
        ("ss5-fiber-02", "SS5_FIBER_ALL_CHECKS_PASS"),
        (
            "rational-gate-check-01",
            "PASS_ALL_245_RATIONAL_GATE_IDENTITIES",
        ),
    ] {
        let log = std::fs::read_to_string(raw.join(format!("cm22-{name}.log"))).unwrap();
        assert!(!log.contains("error") && !log.contains("FAIL") && log.contains(marker));
    }
    if raw.join("cm22-intermediate-01.exit").exists() {
        let status = std::fs::read_to_string(raw.join("cm22-intermediate-01.exit")).unwrap();
        let log = std::fs::read_to_string(raw.join("cm22-intermediate-01.log")).unwrap();
        cas.push(json!({"name":"intermediate","exit":status.trim().parse::<i32>().unwrap(),"reported_error":log.contains("error")||log.contains("halt "),"complete":log.contains("END_TRACE_ADJUNCTION")}));
    }
    let regression = raw.join("regression");
    for name in [
        "klein-wide-generate",
        "klein-wide-verify",
        "n5-generate",
        "n5-verify",
    ] {
        assert_eq!(
            std::fs::read_to_string(regression.join(format!("{name}.exit")))
                .unwrap()
                .trim(),
            "0"
        );
    }
    let n5 = read(&regression.join("n5d-geometric-relations.json"));
    assert!(n5["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["recovered"] == 11));
    let revision = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo)
        .output()
        .unwrap();
    assert!(revision.status.success());
    let source_revision = String::from_utf8(revision.stdout)
        .unwrap()
        .trim()
        .to_owned();
    for (path, wanted) in &sources {
        let blob = std::process::Command::new("git")
            .args(["show", &format!("{source_revision}:{path}")])
            .current_dir(repo)
            .output()
            .unwrap();
        assert!(
            blob.status.success(),
            "source must be committed before sealing:{path}"
        );
        assert_eq!(
            &hex::encode(crate::sha256_impl::sha256(&blob.stdout)),
            wanted,
            "uncommitted source:{path}"
        );
    }
    let root_counts = test_counts(&root);
    let native_counts = test_counts(&native);
    let mut hashes = BTreeMap::new();
    walk(&raw, &raw, &mut hashes);
    let ledger = json!({"schema":"koblitz.round22-ledger/v1","date":"2026-10-26","timezone":"America/Los_Angeles","experiment_parent":"69f67409f5fe6b525c23b000edea047dd38fcf83","publication_parent":"3c87193e7c754e9abefa401b7cd0974d0708b3b1","integration_parent":"8eef1252714097e0b8279c0bf55b82ed9e49943d","source_revision":source_revision,"Conductor_task":"T-409","native":summary_value(&raw),"new_named_Lean_theorems":named,"module_named_counts":modules,"audited_project_theorems":audited,"root_library_tests":root_counts,"native_tests":native_counts,"source_sha256":sources,"raw_sha256":hashes,"CAS_attempts":cas,"EKS_inventory":{"jobs":jobs.len(),"active":active,"matching_study":matching,"mutations":0},"requirements":[{"track":"exact norm-zero recovery","status":"verified complete in the CM equivariant augmentation category","open":"geometric final quotient and modular factor interpretation remain separate"},{"track":"proper level77 geometry","status":"partial","verified":"genus-one base,complete base divisors,generic QR tower,birational plane model,smooth registered open;original SS5 incidence fiber with both ideal lifts and Frobenius residue degrees","open":"proper normalized charts,all lifted coordinate and denominator divisors,complete prescribed RR basis,map with nonzero order11 action"},{"track":"node187 coupled divisors","status":"partial","verified":"all four models excluded by actual coupled valuation/descent under written DVR and pole bridges","open":"explicit source-bound polynomial separator/membership;full saturation;formal schemes/DVR bridge"},{"track":"direct relations","status":"prerequisite open","open":"arbitrary modular divisor reverse evaluator and complete460/367 fibers/images"}],"retained_failures":["generated Lean sum scope/free variable corrected;all failed files retained","missing Lean imports and Rust string-reference compilation errors corrected","original Singular saturation exponent print used wrong API;no exponent claim","affine closure normalization exits124","degree96 and degree72 plane place computations exit14 at4GiB","generic ideal-subtraction display syntax failed;lift matrices retained and corrected replay required","F8-coordinate Klein fixture fails the wide split-fiber verifier precondition;matching F512-coordinate fixture separately replayed","Conductor repeated claim requests used the four-attempt coordination budget;no fifth claim or duplicate session launched"],"performance_measured":false,"IC_efficiency_verdict":null});
    assert!(!round.join("ledger.json").exists());
    round02::save(&round, "ledger.json", &ledger);
    assert!(!round.join("runs.sha256").exists());
    std::fs::write(
        round.join("runs.sha256"),
        hashes
            .iter()
            .map(|(p, h)| format!("{h}  runs/{p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!("Round22 sealed:{named} named Lean statements;{audited} audited;{} source pins;{} raw files",sources.len(),hashes.len());
}

#[cfg(test)]
mod seal_tests {
    #[test]
    #[should_panic(expected = "final Cargo test summary did not succeed")]
    fn failing_final_summary_rejects_successful_nested_process() {
        super::test_counts("test result: ok. 1 passed; 0 failed; 0 ignored;\ntest result: FAILED. 4895 passed; 1 failed; 121 ignored;\n");
    }

    #[test]
    #[should_panic(expected = "missing final Cargo test summary")]
    fn missing_final_summary_is_rejected() {
        super::test_counts("running 5017 tests\ninterrupted\n");
    }

    #[test]
    #[should_panic]
    fn success_label_with_nonzero_failures_is_rejected() {
        super::test_counts("test result: ok. 1 passed; 1 failed; 0 ignored;\n");
    }
    #[test]
    fn final_cargo_summary_excludes_spawned_test_processes() {
        let log = "running 5017 tests\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5016 filtered out;\nchild complete\ntest result: ok. 4896 passed; 0 failed; 121 ignored; 0 measured; 0 filtered out;\n";
        assert_eq!(
            super::test_counts(log),
            serde_json::json!({"passed":4896,"failed":0,"ignored":121})
        );
    }
}
