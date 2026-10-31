//! Source-bound write-once Round21 records; no research arithmetic in layout tools.
use super::*;

fn statements() -> Vec<ic::IdentityStatement> {
    let z = v("z");
    let sum = (1..=18).fold(c(1), |s, i| add(s, pow(z.clone(), i)));
    let geom = ic::IdentityStatement {
        name: "geometric-series19".into(),
        source: "KoblitzRound21.geometric19".into(),
        variables: vec!["z".into()],
        degree_bound: 19,
        lhs: mul(sub(z.clone(), c(1)), sum),
        rhs: sub(pow(z, 19), c(1)),
    };
    let (t, z, a) = (v("t"), v("z"), v("a"));
    let chart = ic::IdentityStatement {
        name: "ordinary-infinity-chart".into(),
        source: "KoblitzRound21.ordinary_chart_factorization".into(),
        variables: ["t", "z", "a"].map(str::to_string).to_vec(),
        degree_bound: 4,
        lhs: sub(
            sub(
                sub(add(z.clone(), mul(t.clone(), z.clone())), pow(t.clone(), 3)),
                mul(a.clone(), mul(pow(t.clone(), 2), z.clone())),
            ),
            pow(z.clone(), 3),
        ),
        rhs: sub(
            mul(
                z.clone(),
                sub(
                    sub(add(c(1), t.clone()), mul(a, pow(t.clone(), 2))),
                    pow(z, 2),
                ),
            ),
            pow(t, 3),
        ),
    };
    vec![geom, chart]
}
pub fn identities(round: &Path, issue: bool) {
    let round = std::fs::canonicalize(round).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    let path = "research/koblitz_modular_ic_20261018/lean/KoblitzRound21.lean";
    for (i, st) in statements().into_iter().enumerate() {
        let name = format!("identity-{}.json", st.name);
        if issue {
            let cert = ic::issue(
                st.clone(),
                202610240310 + i as u64,
                64,
                "universal identity;source-bound kernel theorem;128 additional parsed checks",
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
            serde_json::from_value(read(&round.join(name))).unwrap();
        assert_eq!(cert.statement, st);
        assert_eq!(cert.proof.as_ref().unwrap().sha256, hash(&repo.join(path)));
        assert!(matches!(
            ic::check_with_extra(&cert, Some((202610240410 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }
    println!("Round21:2 source-bound universal IDC records;128 additional parsed checks each;characteristic-specific implications remain separate Lean proofs");
}
fn summary_value(raw: &Path) -> serde_json::Value {
    let q35 = read(&raw.join("q35-trace-zero-tangent-obstruction.json"));
    let g31 = read(&raw.join("g31-equivariant-recovery-denominator.json"));
    let q37 = read(&raw.join("q37-all-ordinary-coupled-resultants.json"));
    let q39 = read(&raw.join("q39-all-elliptic-pure-source-families.json"));
    assert_eq!(q35["four_prefixes_exhausted"], 130321);
    assert_eq!(q35["compatible_five_tuples"], json!([]));
    let coupled = q37["records"].as_array().unwrap();
    assert_eq!(coupled.len(), 32);
    assert!(coupled.iter().all(|r| r["top_is_sum_power19"] == true
        && r["exact_cleared_elliptic_substitution_zero"] == true));
    let rejecting = coupled.iter().filter(|r| r["Q36_excluded"] == true).count();
    assert_eq!(rejecting, 4);
    let families = q39["records"].as_array().unwrap();
    assert_eq!(families.len(), 20);
    let count = families
        .iter()
        .map(|r| r["family"].as_array().unwrap().len())
        .sum::<usize>();
    assert_eq!(count, 1330);
    let mut stages = vec![];
    for (name, file, expected) in [
        (
            "Q36",
            "q36-after-trace-zero-tangent-repair.json",
            vec![155, 157, 184, 186],
        ),
        (
            "Q38",
            "q38-after-all-ordinary-norm-repairs.json",
            vec![180, 182],
        ),
        (
            "Q40",
            "q40-after-all-elliptic-source-families.json",
            vec![187],
        ),
    ] {
        let d = read(&raw.join(file));
        let mut rows = vec![];
        for r in d["records"].as_array().unwrap() {
            assert_eq!(r["capped"], false);
            assert_eq!(r["all_prior_original_Q13_Q17_constraints_checked"], true);
            let nodes = r["model"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .filter_map(|(i, v)| (v == 1).then_some(i))
                .collect::<Vec<_>>();
            assert_eq!(nodes, expected);
            rows.push(json!({"graph":r["graph"],"support":nodes,"visits":r["total_visits"]}));
        }
        stages.push(json!({"stage":name,"records":rows,"models":4,"capped":false}));
    }
    json!({"schema":"koblitz.round21-native-summary/v1","Q35":q35,"G31":g31,"ordinary_coupled_relations":coupled.len(),"Q36_rejecting_relations":rejecting,"pure_source_input_families":families.len(),"pure_source_relations":count,"stages":stages,"full_saturation":false,"complete_arcs":false,"integral_level77_map":false,"final_quotient_embedding":false,"IC_efficiency_verdict":null})
}
pub fn summary(raw: &Path) {
    assert!(!raw.join("round21-native-summary.json").exists());
    let s = summary_value(raw);
    round02::save(raw, "round21-native-summary.json", &s);
    println!("Round21:32 exact coupled norms;1330 exact pure-source relations;four Q40 survivors on187;no IC assessment");
}
pub fn record(round: &Path) {
    let round = std::fs::canonicalize(round).unwrap();
    let study = round.ancestors().nth(2).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    let raw = round.join("runs");
    let summary = summary_value(&raw);
    assert_eq!(read(&raw.join("round21-native-summary.json")), summary);
    let status = std::fs::read_to_string(raw.join("lean-full-status.txt")).unwrap();
    assert_eq!(
        status, "build=0\naudit=0\n",
        "completed successful build/audit required"
    );
    let mut cache = read(&raw.join("proof-cache-manifest.json"));
    assert_eq!(cache["all_source_destination_hashes_equal"], true);
    assert!(cache["bytes"].as_u64().unwrap() <= 8 * 1024 * 1024 * 1024);
    assert_eq!(
        cache["files"].as_u64().unwrap() as usize,
        cache["members"].as_array().unwrap().len()
    );
    let mut signatures = read(&raw.join("proof-cache-ir-signatures.json"));
    assert_eq!(signatures["all_source_destination_hashes_equal"], true);
    assert_eq!(
        signatures["base_manifest_sha256"],
        hash(&raw.join("proof-cache-manifest.json"))
    );
    assert_eq!(signatures["original_members_rechecked"], cache["files"]);
    assert_eq!(
        signatures["files"].as_u64().unwrap() as usize,
        signatures["members"].as_array().unwrap().len()
    );
    assert_eq!(
        signatures["combined_bytes"].as_u64().unwrap(),
        cache["bytes"].as_u64().unwrap() + signatures["bytes"].as_u64().unwrap()
    );
    assert!(signatures["combined_bytes"].as_u64().unwrap() <= 8 * 1024 * 1024 * 1024);
    signatures.as_object_mut().unwrap().remove("members");
    cache
        .as_object_mut()
        .unwrap()
        .insert("IR_signature_supplement".into(), signatures);
    cache.as_object_mut().unwrap().remove("members");
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
    let mut sources = BTreeMap::new();
    walk(repo, &study.join("native/src"), &mut sources);
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    let mut named = 0;
    let mut modules = BTreeMap::new();
    for e in std::fs::read_dir(study.join("lean")).unwrap() {
        let p = e.unwrap().path();
        if p.extension().is_some_and(|e| e == "lean") {
            sources.insert(
                p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
            let module = p.file_stem().unwrap().to_str().unwrap();
            if module.starts_with("KoblitzRound21") {
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
                    assert!(
                        audit.contains(&format!("{module}.{name}")),
                        "missing audited theorem:{module}.{name}"
                    );
                    n += 1;
                }
                modules.insert(module.to_string(), n);
                named += n;
            }
        }
    }
    assert_eq!(named, 206);
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
        round.join("SATURATION.md"),
        round.join("THETA.md"),
        round.join("CM.md"),
        round.join("SOURCES.md"),
        round.join("PROOF_MAP.md"),
    ] {
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
    }
    let eks = read(&raw.join("eks-overview.json"));
    let jobs = eks["jobs"].as_array().unwrap();
    let active = jobs
        .iter()
        .filter(|r| r["state"] != "succeeded" && r["state"] != "failed")
        .count();
    let matching = jobs
        .iter()
        .filter(|r| {
            let n = r["job_name"].as_str().unwrap();
            n.contains("koblitz") || n.contains("modular")
        })
        .count();
    let mut hashes = BTreeMap::new();
    walk(&raw, &raw, &mut hashes);
    let ledger = json!({"schema":"koblitz.round21-ledger/v1","UTC_date":"2026-10-24","parent":"63bc0895df3d2cbd38e22a3efe828a52a0fad731","Conductor_task":"T-299","native":summary,"new_named_Lean_theorems":named,"module_named_counts":modules,"audited_project_theorems":audited,"verification_cache":cache,"cold_NFS_audit_status":124,"EKS_inventory":{"jobs":jobs.len(),"active":active,"matching_study":matching,"mutations":0},"requirements":[{"track":"complete source arcs/full saturation","status":"partial","verified":"universal root obstruction,trace-zero descent implication,32 coupled norms,1330 pure-source relations and complete scalar models","open":"four node187 models;explicit full separators or complete coupled arcs;whole saturated-ideal equality;geometric bridges written"},{"track":"proper F2 source recovery","status":"partial","verified":"actual equivariant A*Phi=5I and sharp mod5 obstruction in CM matrix category","open":"final quotient embedding/243 sections,full kernel actions and rational divisor trivialization"},{"track":"integral level77 map","status":"partial;not rerun in R21","verified":"R20 bounded controlled-pole null preserved","open":"proper normalization,pole divisors,complete Riemann--Roch spaces,integral factor/divisor map"},{"track":"Lean","status":"partial formalization","verified":"universal field/linear-action results,actual CM and finite repaired models/tops","open":"DVR/projective/formal-group semantics,native finite-field/DAG/resultant semantics,CM reduction bridge"}],"retained_failures":["default kubectl context absent;worker-role cluster list forbidden;read-only lab overview reconciled instead","initial unnecessary broad Lean imports interrupted before proof claims;original sources/logs retained","manual proof first misses Finset.sum_div;equivariant module first names absent Matrix.Notation;fixed narrow imports/identity","Q39 Rust inference error on Vec<usize> corrected without changing mathematics","initial root suite F4-count/timer tests fail;isolated controls and serial full rerun retained","Q36 and Q38 remain nonzero;never promoted to saturation","full cold NFS audit exits124 during import with no result;unchanged audit retried using byte-identical local object cache","initial cache preflight has an absent LEAN_PATH root;corrected to an empty local search root;metadata-discovery interruption retained"],"source_sha256":sources,"raw_sha256":hashes,"performance_measured":false,"IC_efficiency_verdict":null});
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
    println!("Round21:{named} named Lean results;{audited} audited;{} source pins;{} raw files;{} EKS jobs/{active} active/{matching} study",sources.len(),hashes.len(),jobs.len());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn universal_identity_asts() {
        for (i, st) in statements().into_iter().enumerate() {
            let cert = ic::issue(
                st,
                202610240310 + i as u64,
                64,
                "independent AST control",
                None,
            )
            .unwrap();
            assert!(matches!(
                ic::check_with_extra(&cert, Some((202610240410 + i as u64, 128))).unwrap(),
                ic::Verdict::Accept { .. }
            ));
        }
    }
}
