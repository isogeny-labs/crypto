//! Write-once Round16 records, explicit historical snapshot pins and identity certificates.
use super::*;
pub fn verify_sources(round: &Path, snapshot: Option<&str>) {
    let round = std::fs::canonicalize(round).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    if let Some(s) = snapshot {
        assert!(s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit()));
    }
    let ledger = read(&round.join("ledger.json"));
    let sources = ledger["source_sha256"].as_object().unwrap();
    assert!(!sources.is_empty());
    for (p, h) in sources {
        let actual = if let Some(s) = snapshot {
            let arg = format!("{s}:{p}");
            let out = std::process::Command::new("git")
                .args(["show", &arg])
                .current_dir(repo)
                .output()
                .unwrap();
            assert!(out.status.success(), "missing historical object:{arg}");
            hex::encode(sha256_impl::sha256(&out.stdout))
        } else {
            hash(&repo.join(p))
        };
        assert_eq!(actual, h.as_str().unwrap(), "source drift:{p}");
    }
    println!("PASS:{} source pins;snapshot={snapshot:?}", sources.len());
}
fn statement(
    name: &str,
    source: &str,
    vars: &[&str],
    degree: usize,
    lhs: E,
    rhs: E,
) -> ic::IdentityStatement {
    ic::IdentityStatement {
        name: name.into(),
        source: source.into(),
        variables: vars.iter().map(|s| s.to_string()).collect(),
        degree_bound: degree as u32,
        lhs,
        rhs,
    }
}
fn statements(round: &Path) -> Vec<(ic::IdentityStatement, &'static str)> {
    let h = v("h");
    let x = v("x");
    let u = v("U");
    let vv = v("V");
    let a = v("c1");
    let b = add(mul(v("c3"), u.clone()), mul(v("c0"), h.clone()));
    let q = sub(mul(v("c2"), u.clone()), vv.clone());
    let d = mul(pow(h.clone(), 2), u.clone());
    let cc = mul(h.clone(), q.clone());
    let a0 = sub(
        pow(cc.clone(), 2),
        mul(mul(a.clone(), b.clone()), d.clone()),
    );
    let a1 = sub(mul(pow(a.clone(), 2), d), mul(b.clone(), cc.clone()));
    let a2 = sub(pow(b.clone(), 2), mul(a.clone(), cc));
    let adj = add(
        add(a0, mul(a1.clone(), x.clone())),
        mul(a2.clone(), pow(x.clone(), 2)),
    );
    let cof = add(
        mul(
            add(mul(v("c3"), x.clone()), mul(v("c2"), h.clone())),
            adj.clone(),
        ),
        mul(
            pow(h.clone(), 2),
            add(
                add(
                    mul(mul(a.clone(), a2.clone()), x.clone()),
                    mul(b.clone(), a2),
                ),
                mul(a.clone(), a1),
            ),
        ),
    );
    let sep = sub(
        add(
            add(
                mul(u.clone(), pow(b.clone(), 3)),
                mul(h.clone(), pow(q.clone(), 3)),
            ),
            mul(mul(pow(h.clone(), 2), pow(u.clone(), 2)), pow(a.clone(), 3)),
        ),
        mul(c(3), mul(mul(mul(mul(h.clone(), u.clone()), a), b), q)),
    );
    let f2 = sub(
        add(
            add(
                add(
                    mul(v("c3"), pow(x.clone(), 4)),
                    mul(mul(v("c2"), h.clone()), pow(x.clone(), 3)),
                ),
                mul(mul(v("c1"), pow(h.clone(), 2)), pow(x.clone(), 2)),
            ),
            mul(mul(v("c0"), pow(h.clone(), 3)), x.clone()),
        ),
        mul(pow(h.clone(), 3), vv),
    );
    let mut out = vec![(
        statement(
            "quartic-separator",
            "KoblitzRound16.separator_residual",
            &["h", "x", "U", "V", "c0", "c1", "c2", "c3"],
            11,
            mul(pow(h.clone(), 4), sep),
            sub(
                mul(f2, adj),
                mul(sub(pow(x.clone(), 3), mul(pow(h, 2), u)), cof),
            ),
        ),
        "KoblitzRound16",
    )];
    let (a, b, l, n, y, w) = (v("A"), v("b"), v("l"), v("n"), v("y"), v("w"));
    let eval = |yy: E, aa2: E, aa6: E| {
        add(
            add(
                add(
                    add(
                        pow(yy.clone(), 2),
                        mul(mul(a.clone(), x.clone()), yy.clone()),
                    ),
                    mul(b.clone(), yy),
                ),
                pow(x.clone(), 3),
            ),
            add(mul(aa2, pow(x.clone(), 2)), aa6),
        )
    };
    let ly = add(mul(l.clone(), x.clone()), n.clone());
    let qp = add(
        add(
            add(
                pow(x.clone(), 3),
                mul(
                    add(add(b.clone(), pow(l.clone(), 2)), mul(a.clone(), l.clone())),
                    pow(x.clone(), 2),
                ),
            ),
            mul(
                add(mul(a.clone(), n.clone()), mul(b.clone(), l.clone())),
                x.clone(),
            ),
        ),
        add(pow(n.clone(), 2), mul(b.clone(), n.clone())),
    );
    out.push((
        statement(
            "line-intersection",
            "KoblitzRound16Carrier.line_intersection_residual",
            &["x", "A", "b", "l", "n"],
            4,
            sub(eval(ly, b.clone(), c(0)), qp),
            mul(c(2), mul(mul(l, n), x.clone())),
        ),
        "KoblitzRound16Carrier",
    ));
    let dd = add(mul(a.clone(), x.clone()), b.clone());
    let wr = sub(add(pow(w.clone(), 2), w.clone()), c(1));
    out.push((
        statement(
            "diamond-curve-twist",
            "KoblitzRound16Carrier.twist_curve_residual",
            &["x", "y", "A", "b", "w"],
            6,
            sub(
                eval(add(y.clone(), mul(w.clone(), dd.clone())), b.clone(), c(0)),
                eval(
                    y.clone(),
                    add(b.clone(), pow(a.clone(), 2)),
                    pow(b.clone(), 2),
                ),
            ),
            add(
                mul(wr.clone(), pow(dd.clone(), 2)),
                mul(
                    c(2),
                    add(
                        mul(mul(y, w.clone()), dd),
                        mul(mul(a.clone(), b.clone()), x),
                    ),
                ),
            ),
        ),
        "KoblitzRound16Carrier",
    ));
    let (m, d) = (v("M"), v("D"));
    let shifted = add(m.clone(), mul(mul(w.clone(), a.clone()), d.clone()));
    let ne = |mm: E| {
        add(
            add(pow(mm.clone(), 2), mul(mul(a.clone(), mm), d.clone())),
            mul(b.clone(), pow(d.clone(), 2)),
        )
    };
    out.push((
        statement(
            "diamond-doubling-twist",
            "KoblitzRound16Carrier.twist_doubling_residual",
            &["M", "A", "D", "b", "w"],
            6,
            sub(
                ne(shifted),
                add(ne(m.clone()), mul(pow(a.clone(), 2), pow(d.clone(), 2))),
            ),
            add(
                mul(mul(pow(a.clone(), 2), pow(d.clone(), 2)), wr),
                mul(c(2), mul(mul(mul(w, a), m), d)),
            ),
        ),
        "KoblitzRound16Carrier",
    ));
    let cm = read(&round.join("runs/cm16-integral-sign-separation.json"));
    let pol = |key: &str| {
        E::sum(
            cm[key]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(i, z)| mul(c(z.as_str().unwrap().parse().unwrap()), pow(v("x"), i)))
                .collect(),
        )
    };
    out.push((
        statement(
            "actual-cm-bezout",
            "KoblitzRound16CM.actual_integer_Bezout",
            &["x"],
            15,
            add(
                mul(pol("Bezout_minus"), pol("minus")),
                mul(pol("Bezout_plus"), pol("plus")),
            ),
            c(704),
        ),
        "KoblitzRound16CM",
    ));
    out
}
pub fn identities(round: &Path, issue: bool) {
    let round = std::fs::canonicalize(round).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    for (i, (st, module)) in statements(&round).into_iter().enumerate() {
        let file = format!("identity-{}.json", st.name);
        let path = format!("research/koblitz_modular_ic_20261018/lean/{module}.lean");
        if issue {
            let cert = ic::issue(
                st.clone(),
                202610220400 + i as u64,
                64,
                "native issuer;128 extra parsed checks plus source-bound kernel theorem",
                Some(ic::ProofRef {
                    kind: "lean4".into(),
                    path: path.clone(),
                    sha256: hash(&repo.join(&path)),
                }),
            )
            .unwrap();
            round02::save(&round, &file, &serde_json::to_value(cert).unwrap());
        }
        let cert: ic::IdentityCertificate =
            serde_json::from_value(read(&round.join(file))).unwrap();
        assert_eq!(cert.statement, st);
        assert_eq!(cert.proof.as_ref().unwrap().sha256, hash(&repo.join(path)));
        assert!(matches!(
            ic::check_with_extra(&cert, Some((202610220500 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
    }
    println!("Round16:5 source-bound IDC records pass128 extra parsed checks each");
}
pub fn manifest(dir: &Path, out: &Path, prefix: &str) {
    let mut hashes = BTreeMap::new();
    walk(dir, dir, &mut hashes);
    assert!(!out.exists());
    std::fs::write(
        out,
        hashes
            .iter()
            .map(|(p, h)| format!("{h}  {prefix}{p}\n"))
            .collect::<String>(),
    )
    .unwrap();
}
pub fn record(round: &Path) {
    let round = std::fs::canonicalize(round).unwrap();
    let study = round.ancestors().nth(2).unwrap();
    let repo = round.ancestors().nth(4).unwrap();
    let raw = round.join("runs");
    identities(&round, true);
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
        "KoblitzRound16",
        "KoblitzRound16CM",
        "KoblitzRound16Ramification",
        "KoblitzRound16Sections",
        "KoblitzRound16Separators",
        "KoblitzRound16Boundary",
        "KoblitzRound16Carrier",
    ] {
        let p = study.join(format!("lean/{name}.lean"));
        named += std::fs::read_to_string(&p)
            .unwrap()
            .lines()
            .filter(|l| l.starts_with("theorem "))
            .count();
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
    }
    // Pin all project-owned Lean imports, including older structural modules.
    for entry in std::fs::read_dir(study.join("lean")).unwrap() {
        let p = entry.unwrap().path();
        if p.extension().is_some_and(|x| x == "lean") {
            sources.insert(
                p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
        }
    }
    for p in [
        repo.join("src/cryptanalysis/curve_construction/gf2k.rs"),
        repo.join("src/cryptanalysis/curve_construction/modular.rs"),
        repo.join("src/cryptanalysis/curve_construction/weil.rs"),
        repo.join("src/cryptanalysis/curve_construction/lpoly.rs"),
        repo.join("tools/lean/proof-gate.sh"),
        repo.join("tools/lean/AuditAxioms.lean"),
        repo.join("tools/lean/trusted-native.txt"),
        study.join("lean/lean-toolchain"),
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
        round.join("SATURATION.md"),
        round.join("PROTOCOL.md"),
    ] {
        sources.insert(
            p.strip_prefix(repo).unwrap().to_str().unwrap().into(),
            hash(&p),
        );
    }
    let mut hashes = BTreeMap::new();
    walk(&raw, &raw, &mut hashes);
    let readraw = |name: &str| read(&raw.join(name));
    let cm = readraw("cm16-integral-sign-separation.json");
    assert_eq!(cm["primitive_Bezout_denominator"], "704");
    let carrier = readraw("cm17-explicit-level77-incidence.json");
    assert_eq!(
        carrier["finite_field_records"][4]["untwisted_incidence_points"]
            .as_array()
            .unwrap()
            .len(),
        10
    );
    let ledger = json!({"schema":"koblitz.round16-ledger/v1","client_date":"2026-10-21 America/Los_Angeles","UTC_date":"2026-10-22","parent_commit":"26b0d6f82349d35bf15b049f345f3ab2ebaa57f4","CM_projector":cm,"CM_affine_carrier":carrier,"diagonal_cover":readraw("g16-diagonal-cover-section-bases.json"),"ramification":readraw("q19-actual-ramification-six.json"),"valuations":readraw("q20-original-gate-valuation-propagation.json"),"separators":readraw("q21-source-bound-quartic-separators.json"),"new_boundary":readraw("q22-after-quartic-boundary-repair.json"),"new_named_Lean_theorems":named,"audited_project_theorems":audited,"requirements":[{"track":"level77 carrier","status":"partial","verified":"explicit affine Tate inverse-witness circuits and diamond twist;exact independent q3 torsion controls through F32","open":"proper normalization,integral Koblitz-factor correspondence,injection and divisor evaluator"},{"track":"proper polarized embedding","status":"partial","verified":"actual diagonal finite covers over F32 and exact tensor factor bases","open":"original-divisor trivialization,theta descent actions,243 target sections and proper equations"},{"track":"full quotient saturation","status":"partial","verified":"source-bound depth6 quartic rejects all four old Q17 models;four new Q22 survivors","open":"full h-saturated ideal equality and proper boundary"},{"track":"direct relations","status":"partial","verified":"explicit lower-level incidence is a new source of candidate functions","open":"new CM77 closed divisors and projected principal rows;Round15 norm fibers/images"},{"track":"Lean proofs","status":"partial","verified":"universal residuals and finite source-bound certificates","open":"global modular/abelian bridges,native implementation semantics"}],"retained_failures":["CM16 modular-gcd loop corrected after retained output","CM17 small-field independent torsion controls vacuous;extended through F32","CM17 conservative2b cap hit;amended8b before successful repeat","Lean noncomputable/prime-instance and unfolding errors retained","Lean group scalar lemma corrected after ring-only tactic rejection","G16 original cover is F32-only;two covers inseparable","CI old-head workflow timeout violates current25-minute policy;fixed without removing tests","CI dashboard missing Chrome and unrelated sea_point_count Lean CLI failure retained"],"source_sha256":sources,"raw_sha256":hashes,"performance_measured":false,"IC_efficiency_verdict":null});
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
        "Round16:{named} new named theorems;{audited} audited;{} sources;{} raw files",
        sources.len(),
        hashes.len()
    );
}
