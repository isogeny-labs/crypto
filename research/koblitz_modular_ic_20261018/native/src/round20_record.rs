//! Write-once round20 ledger and source-bound universal identities.
use super::*;
fn prod(xs: Vec<E>) -> E {
    xs.into_iter().fold(c(1), mul)
}
fn statements() -> Vec<ic::IdentityStatement> {
    let (d, p, e) = (v("d"), v("p"), v("e"));
    let cancel = ic::IdentityStatement {
        name: "integral-lift-step".into(),
        source: "KoblitzRound20.lift_step_cancel".into(),
        variables: ["d", "p", "e"].map(str::to_string).to_vec(),
        degree_bound: 3,
        lhs: add(
            mul(d.clone(), add(c(1), mul(p.clone(), e.clone()))),
            prod(vec![c(-1), e, p, d.clone()]),
        ),
        rhs: d,
    };
    let (u, z) = (v("U"), v("V"));
    let top = ic::IdentityStatement {
        name: "power19-factorization".into(),
        source: "KoblitzRound20.power19_factorization".into(),
        variables: ["U", "V"].map(str::to_string).to_vec(),
        degree_bound: 19,
        lhs: pow(add(u.clone(), z.clone()), 19),
        rhs: prod(vec![
            pow(add(u.clone(), z.clone()), 16),
            pow(add(u.clone(), z.clone()), 2),
            add(u, z),
        ]),
    };
    let (x, y, q) = (v("x"), v("y"), v("q"));
    let ratio = ic::IdentityStatement {
        name: "infinity-coordinate-clearing".into(),
        source: "KoblitzRound20.coordinate_ratio_clearing".into(),
        variables: ["x", "y", "q"].map(str::to_string).to_vec(),
        degree_bound: 3,
        lhs: prod(vec![x.clone(), q.clone(), y.clone()]),
        rhs: prod(vec![x, y, q]),
    };
    vec![cancel, top, ratio]
}
pub fn identities(round: &Path, issue: bool) {
    let round = std::fs::canonicalize(round).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    let path = "research/koblitz_modular_ic_20261018/lean/KoblitzRound20.lean";
    for (i, st) in statements().into_iter().enumerate() {
        let name = format!("identity-{}.json", st.name);
        if issue {
            let cert = ic::issue(
                st.clone(),
                202610240100 + i as u64,
                64,
                "native issuer;128 additional parsed checks;source-bound kernel theorem",
                Some(ic::ProofRef {
                    kind: "lean4".into(),
                    path: path.into(),
                    sha256: hash(&repo.join(path)),
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
            ic::check_with_extra(&cert, Some((202610240200 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }
    println!("Round20:3 source-bound IDC records pass128 additional parsed checks each");
}
pub fn record(round: &Path) {
    let round = std::fs::canonicalize(round).unwrap();
    let study = round.ancestors().nth(2).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    let raw = round.join("runs");
    let audit = std::fs::read_to_string(raw.join("lean-full-audit.log")).unwrap();
    assert!(audit.contains("0 failing, 0 trusted"));
    assert!(!audit.contains("sorryAx"));
    let audited: u64 = audit
        .lines()
        .find(|s| s.starts_with("audited "))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let g26 = read(&raw.join("g26-canonical-orbit-Pic2-matching.json"));
    let g27 = read(&raw.join("g27-common-orbit-map-kernel.json"));
    let g28 = read(&raw.join("g28-integral-orbit-retraction.json"));
    let mut g29 = read(&raw.join("g29-full-weight-orbit-sections.json"));
    let g30 = read(&raw.join("g30-proper-orbit-image-equations.json"));
    assert!(g26["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["difference_bits"] == 0));
    assert!(g27["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["primary"]
            .as_array()
            .unwrap()
            .iter()
            .all(|t| t["rank"] == 8
                && t["dimension"] == 0
                && t["actual_column_map_checks"] == 160)));
    assert_eq!(g28["point_round_trips"].as_array().unwrap().len(), 8);
    assert_eq!(g29["ordinary_sparse_sections"], 2952);
    assert!(g29["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["binary_evaluation_rank"] == 2688));
    assert_eq!(g30["Galois_stable_group_equations"], 100);
    let maps = read(&raw.join("cm26-quadratic-map-summary.json"));
    assert_eq!(maps["x_forms_examined"], 524288);
    assert_eq!(maps["finite_fits"], 10);
    assert_eq!(maps["verified_constant_maps"], 10);
    assert_eq!(maps["unexpanded_affine_spaces"], 0);
    let q33 = read(&raw.join("q33-coupled-quotient-resultant.json"));
    assert!(q33["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["Q32_excluded"] == false && r["top_is_sum_power19"] == true));
    let q34 = read(&raw.join("q34-after-coupled-quotient-repair.json"));
    let supports = q34["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            assert_eq!(r["capped"], false);
            let v = r["model"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .filter(|(_, z)| **z == 1)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            assert_eq!(v, vec![123, 124, 126, 127]);
            json!({"graph":r["graph"],"nodes":v,"visits":r["total_visits"]})
        })
        .collect::<Vec<_>>();
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    let mut named = 0;
    let mut modules = BTreeMap::new();
    for entry in std::fs::read_dir(study.join("lean")).unwrap() {
        let p = entry.unwrap().path();
        if p.extension().is_some_and(|e| e == "lean") {
            sources.insert(
                p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
            if p.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("KoblitzRound20")
            {
                let module = p.file_stem().unwrap().to_str().unwrap();
                let s = std::fs::read_to_string(&p).unwrap();
                let mut n = 0;
                for line in s.lines().filter(|s| s.starts_with("theorem ")) {
                    let name = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .split(['{', '(', ':'])
                        .next()
                        .unwrap();
                    assert!(
                        audit.contains(&format!("{module}.{name}")),
                        "new theorem missing from full audit:{module}.{name}"
                    );
                    n += 1;
                }
                modules.insert(module.to_string(), n);
                named += n;
            }
        }
    }
    assert_eq!(named, 76);
    identities(&round, false);
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
        round.join("THETA.md"),
        round.join("SATURATION.md"),
        round.join("CM.md"),
        round.join("SOURCES.md"),
        round.join("PROOF_MAP.md"),
    ] {
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
    }
    let mut hashes = BTreeMap::new();
    walk(&raw, &raw, &mut hashes);
    // Full point vectors remain in the hash-bound raw evidence, not duplicated in the ledger.
    for r in g29["records"].as_array_mut().unwrap() {
        r.as_object_mut().unwrap().remove("probes");
    }
    let ledger = json!({"schema":"koblitz.round20-ledger/v1","client_date":"2026-10-24 America/Los_Angeles","UTC_date":"2026-10-24","scientific_parent":"11b2f23f97195b5e3eda5f5764d46a1dd7818f92","operational_parent":"ef4f01db0d7539a2f954fe9fd633039660bbae59","Conductor_task":"T-291","new_named_Lean_theorems":named,"module_named_counts":modules,"audited_project_theorems":audited,"canonical_Pic2":g26,"common_kernel":g27,"integral_retraction":g28,"weighted_sections":g29,"proper_source_equations":g30,"controlled_poles":maps,"saturation":{"Q33_excludes_Q32":false,"Q33_top":"(U+V)^19","Q34_models":supports,"full_saturation":false},"requirements":[{"track":"canonical mixed theta class","status":"verified under written geometric bridges","verified":"actual axis class match and no nonzero Frobenius-fixed Pic0[2] difference","open":"explicit rational trivialization and complete actual theta/kernel action matrices"},{"track":"proper polarized embedding","status":"source-cover construction verified under written morphism/descent bridges","verified":"integral CM retraction,full elliptic factor bases and factorized Segre embedding;20 proper group equations/100 Galois-stable conjugates","open":"small projective equations and the final105/19 quotient embedding/243 sections"},{"track":"mixed sections","status":"partial","verified":"2952 ordinary F2 orbit sections,2688 binary evaluation rank on16 tuples per sign,48 noninvariance witnesses;no forced ordinary fixed divisor under pairwise-independence bridge","open":"basepoint-free sparse basis,complete source H0 and final invariant243-space"},{"track":"integral level77 map","status":"partial","verified":"complementary524288 quadratic numerator cases yield10 verified constant-map identities only","open":"proper normalization,explicit pole divisors,complete prescribed Riemann--Roch spaces,integral factor/divisor map"},{"track":"full saturation","status":"partial","verified":"186-term exact coupled quotient relations and radical U=V;four complete Q34 models retained","open":"complete simultaneous source arc or actual rejecting separator,and whole saturated-ideal equality"},{"track":"Lean proofs","status":"partial formalization","verified":"actual CM matrices,universal evaluated recovery,Pic2 and finite gate/model/basis certificates,structural lift/boundary/coordinate implications","open":"full elliptic/group/divisor/descent and scheme semantics,native field identities/exhaustion/evaluation ranks"}],"retained_failures":["first Q33 hopeful rejection export is false;never built or claimed;actual zero boundary residue retained","formal degree1 denominator is actually constant1;degree corrected with initial artifact retained","first G27 nullspace-basis export is vacuous for zero-dimensional spaces;replaced by left-inverse and trivial-kernel proofs","first augmented native build has BigInt scalar type inference errors;source and diagnostics retained","initial Lean launch used wrong working directory;receipt retained","recorder compile failed on u32/usize exponent mismatch;source and diagnostics retained","initial IDC test correctly rejects a characteristic2-only expansion over its fixed odd prime;no certificate issued;replaced by universally valid factorization and matching Lean theorem","manual proof compile diagnoses missing Matrix annotations,rectangular rewrite names,ratio normalization and binder spacing;source/logs retained","entrywise proof controls initially leave unsolved goals;combined Field instantiations are interrupted;final stronger CommRing proof passes all13 finite-kernel names","root suite first F4 cheaper-count threshold fails18;isolated control and full rerun retained"],"source_sha256":sources,"raw_sha256":hashes,"performance_measured":false,"IC_efficiency_verdict":null});
    round02::save(&round, "ledger.json", &ledger);
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
        "Round20:{named} new named Lean results;{audited} audited;{} source pins;{} raw files",
        sources.len(),
        hashes.len()
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_new_identity_asts_check_independently() {
        for (i, st) in statements().into_iter().enumerate() {
            let cert = ic::issue(
                st,
                202610240100 + i as u64,
                64,
                "universal AST control",
                None,
            )
            .unwrap();
            assert!(matches!(
                ic::check_with_extra(&cert, Some((202610240200 + i as u64, 128))).unwrap(),
                ic::Verdict::Accept { .. }
            ));
        }
    }
}
