//! Write-once Round19 evidence and source-bound universal identities.
use super::*;
fn ps(vs: Vec<E>) -> E {
    vs.into_iter().fold(c(0), add)
}
fn pm(vs: Vec<E>) -> E {
    vs.into_iter().fold(c(1), mul)
}
fn statements() -> Vec<ic::IdentityStatement> {
    let (x, y, s, a) = (v("x"), v("y"), v("S"), v("a"));
    let norm = ic::IdentityStatement {
        name: "elliptic-coupled-norm".into(),
        source: "KoblitzRound19.elliptic_norm_identity".into(),
        variables: ["x", "y", "S", "a"].map(str::to_string).to_vec(),
        degree_bound: 6,
        lhs: ps(vec![
            pow(mul(y.clone(), s.clone()), 2),
            pm(vec![x.clone(), s.clone(), y.clone(), s.clone()]),
            mul(
                pow(s.clone(), 2),
                ps(vec![
                    pow(x.clone(), 3),
                    mul(a.clone(), pow(x.clone(), 2)),
                    c(1),
                ]),
            ),
        ]),
        rhs: mul(
            pow(s, 2),
            ps(vec![
                pow(y.clone(), 2),
                mul(x.clone(), y),
                pow(x.clone(), 3),
                mul(a, pow(x, 2)),
                c(1),
            ]),
        ),
    };
    let (q, x, y, a) = (v("q"), v("x"), v("y"), v("a"));
    let nn = mul(pow(q.clone(), 2), x.clone());
    let mm = mul(pow(q.clone(), 3), y.clone());
    let weighted = ic::IdentityStatement {
        name: "controlled-pole-weighted-map".into(),
        source: "KoblitzRound19.weighted_map_identity".into(),
        variables: ["q", "x", "y", "a"].map(str::to_string).to_vec(),
        degree_bound: 10,
        lhs: ps(vec![
            pow(mm.clone(), 2),
            pm(vec![nn.clone(), mm, q.clone()]),
            pow(nn.clone(), 3),
            pm(vec![a.clone(), pow(nn, 2), pow(q.clone(), 2)]),
            pow(q.clone(), 6),
        ]),
        rhs: mul(
            pow(q, 6),
            ps(vec![
                pow(y.clone(), 2),
                mul(x.clone(), y),
                pow(x.clone(), 3),
                mul(a, pow(x, 2)),
                c(1),
            ]),
        ),
    };
    let factors = ["a", "b", "c", "d", "e"].map(|s| add(c(1), mul(v(s), v("t"))));
    let orbit = ic::IdentityStatement {
        name: "mixed-orbit-rotation".into(),
        source: "KoblitzRound19.orbit_product_rotation".into(),
        variables: ["a", "b", "c", "d", "e", "t"].map(str::to_string).to_vec(),
        degree_bound: 10,
        lhs: pm([4, 0, 1, 2, 3]
            .iter()
            .map(|i| factors[*i].clone())
            .collect()),
        rhs: pm(factors.to_vec()),
    };
    vec![norm, weighted, orbit]
}
pub fn identities(round: &Path, issue: bool) {
    let round = std::fs::canonicalize(round).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    let path = "research/koblitz_modular_ic_20261018/lean/KoblitzRound19.lean";
    for (i, st) in statements().into_iter().enumerate() {
        let name = format!("identity-{}.json", st.name);
        if issue {
            let cert = ic::issue(
                st.clone(),
                202610230500 + i as u64,
                64,
                "native issuer;128 extra parsed checks;explicit source-bound kernel theorem",
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
            ic::check_with_extra(&cert, Some((202610230600 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }
    println!("Round19:3 source-bound IDC records pass128 additional parsed checks each");
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
    let maps = read(&raw.join("cm24-controlled-pole-summary.json"));
    assert_eq!(maps["x_forms_examined"], 335544320u64);
    assert_eq!(maps["nonconstant_finite_fits"], 0);
    let theta = read(&raw.join("g22-mixed-F2-orbit-sections.json"));
    assert_eq!(theta["section_count_constructed"], 6);
    let orbit_search = read(&raw.join("g23-integral-orbit-polarization-search.json"));
    let orbit_obstruction = read(&raw.join("g24-orbit-transfer-mod5-obstruction.json"));
    let orbit_cover = read(&raw.join("g25-correct-polarization-orbit-sections.json"));
    assert_eq!(orbit_cover["constructed_sections"], 41);
    assert_eq!(orbit_cover["target_polarization_scalar"], 33075);
    assert!(orbit_search["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["matches"].as_array().unwrap().is_empty()));
    let q29 = read(&raw.join("q29-coupled-elliptic-resultants.json"));
    let q31 = read(&raw.join("q31-pure-source-resultant-family.json"));
    let q32 = read(&raw.join("q32-after-pure-source-repairs.json"));
    assert!(q29["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["Q28_excluded_under_source_bridge"] == true));
    assert!(q31["records"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["Q30_excluded"] == true));
    let support = q32["records"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            assert_eq!(r["capped"], false);
            let nodes = r["model"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .filter(|(_, v)| **v == 1)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            assert_eq!(nodes, vec![123, 124, 126, 127]);
            json!({"graph":r["graph"],"visits":r["total_visits"],"nodes":nodes})
        })
        .collect::<Vec<_>>();
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    let mut named = 0;
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
                .starts_with("KoblitzRound19")
            {
                let module = p.file_stem().unwrap().to_str().unwrap();
                let source = std::fs::read_to_string(&p).unwrap();
                for line in source.lines().filter(|s| s.starts_with("theorem ")) {
                    let name = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .trim_end_matches(':');
                    assert!(
                        audit.contains(&format!("{module}.{name}")),
                        "new theorem missing from final transitive audit: {module}.{name}"
                    );
                    named += 1;
                }
            }
        }
    }
    assert_eq!(named, 79);
    identities(&round, true);
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
    let ledger = json!({"schema":"koblitz.round19-ledger/v1","client_date":"2026-10-23 America/Los_Angeles","UTC_date":"2026-10-23","scientific_parent":"a2b8f27be5fb3faac9362430ef43245b3bce819d","new_named_Lean_theorems":named,"audited_project_theorems":audited,"controlled_poles":maps,"mixed_sections":{"constructed":6,"independence_asserted":false,"tuples_per_sign":16,"actual_torsion_translations":768,"auxiliary_h0":80,"G23_integral_rows_per_form":390625,"G23_matches":0,"G24_integral_orbit_sums_excluded":true,"G24_certificate":orbit_obstruction,"scalar_pullback_h0":theta["h0_after_scalar_pullback"],"full_auxiliary_product_h0":theta["full_product_h0"],"G25_correct_polarization_sections":41,"G25_certificate":orbit_cover,"final_19_kernel_descent_for_auxiliary_bundle":false,"canonical_243_target_sections_extracted":false},"saturation":{"Q28_excluded":4,"Q29_terms_per_graph":191,"Q30_excluded":4,"Q31_distinct_resultants":q31["records"].as_array().unwrap().iter().map(|r|r["family"].as_array().unwrap().len()).collect::<Vec<_>>(),"Q31_source_aliases":q31["records"].as_array().unwrap().iter().map(|r|r["radical_boundary_zero_nodes"].as_array().unwrap().len()).collect::<Vec<_>>(),"Q32_models":support,"full_saturation":false},"requirements":[{"track":"explicit integral level77 map using controlled poles","status":"partial","verified":"complete registered five-denominator coefficient search;only64 constant maps survive18 actual incidences;pole bound and clearing identities","open":"proper normalization,explicit pole divisors and complete Riemann--Roch spaces,integral factor inclusion and divisor evaluator"},{"track":"mixed F2 invariant sections","status":"partial","verified":"six auxiliary mixed sections;primitive transfer obstruction;41 explicit F2 sections with correct scalar-cover polarization and48 kernel noninvariance witnesses","open":"canonical3L trivializations,canonical Pic2 matching,full weighted mixed sections and actual odd/19-kernel invariant basis,243 target sections and proper equations"},{"track":"four Q28 survivors","status":"verified exclusion under written source bridge","verified":"coupled elliptic resultants alsoexcludeQ30;266 pure-source relations","open":"Q32 complete arc or source separator,whole quotient saturation and proper embedding"},{"track":"Lean proofs","status":"partial formalization","verified":"universal identities,determinant and monic-boundary implications,orbit invariance,finite source/leading-term/model certificates","open":"full field/group/modular/divisor/descent geometry,actual norm-matrix evaluation bridge and native search exhaustion"}],"retained_failures":["univariate higher-degree trigger fails for Q28;coupled elliptic norm succeeds","first poles Rust build:JSON macro indexing syntax","first coupled resultant:graph2 cubic gate is not constant;actual gate extraction fixed","initial Lean unit orientation/noncomputable definition and matrix notation errors","G24 Matrix.mulVec_mulVec rewrite direction failure;fixed with retained source","expanded Lean separator elaboration limit;list representation retains identical polynomials","duplicate long expanded proof interrupted;passing prior expanded proof and logs retained","root suite initial F4 cheaper-count19 failure;isolated and full rechecks retained","auxiliary80-dimensional ordinary factor with actual degree57 companion cannot descend through degree361 quotient","G23 bounded null upgraded by actual G24 modulo5 transfer obstruction","G25 correctly polarized41 sections have forced fixed components and48 tested kernel noninvariance witnesses"],"source_sha256":sources,"raw_sha256":hashes,"performance_measured":false,"IC_efficiency_verdict":null});
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
        "Round19:{named} new named theorems;{audited} audited;{} source pins;{} raw files",
        sources.len(),
        hashes.len()
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_identity_ast_controls() {
        for (i, st) in statements().into_iter().enumerate() {
            let cert = ic::issue(
                st,
                202610230500 + i as u64,
                64,
                "test universal identity AST",
                None,
            )
            .unwrap();
            assert!(matches!(
                ic::check_with_extra(&cert, Some((202610230600 + i as u64, 128))).unwrap(),
                ic::Verdict::Accept { .. }
            ));
        }
    }
}
