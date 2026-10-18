//! Reproducible evidence summaries; all research calculations remain native.
use crate::{identity_certificate as ic, round02, sha256_impl};
use ic::Expr as E;
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::Path};
fn p(x: E, n: usize) -> E {
    E::product((0..n).map(|_| x.clone()).collect())
}
fn c(n: i64) -> E {
    E::Const(n)
}
fn plus(a: E, b: E) -> E {
    E::plus(a, b)
}
fn minus(a: E, b: E) -> E {
    plus(a, E::minus(b))
}
fn times(a: E, b: E) -> E {
    E::times(a, b)
}
fn hash(path: &Path) -> String {
    hex::encode(sha256_impl::sha256(&std::fs::read(path).unwrap()))
}
fn read(dir: &Path, name: &str) -> Value {
    serde_json::from_slice(&std::fs::read(dir.join(name)).unwrap()).unwrap()
}
fn certificates(dir: &Path, repo: &Path) -> Vec<Value> {
    let mut out = Vec::new();
    let mut issue = |name: &str, variables: Vec<&str>, degree: u32, lhs: E, rhs: E, lean: bool| {
        let statement = ic::IdentityStatement {
            name: name.into(),
            source: "round03 exact algebra; scopes and Lean declarations in PROOF_MAP.md".into(),
            variables: variables.into_iter().map(str::to_string).collect(),
            degree_bound: degree,
            lhs,
            rhs,
        };
        let proof = if lean {
            Some(ic::ProofRef {
                kind: "lean4".into(),
                path: "research/koblitz_modular_ic_20261018/lean/KoblitzRound03.lean".into(),
                sha256: hash(
                    &repo.join("research/koblitz_modular_ic_20261018/lean/KoblitzRound03.lean"),
                ),
            })
        } else {
            None
        };
        let certificate=ic::issue(statement,2026101803,64,"native issuance plus extra-point checking; separate Lean kernel proof with scope stated explicitly",proof).unwrap();
        assert!(matches!(
            ic::check_with_extra(&certificate, Some((2026101803001, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        let filename = format!("identity-{}.json", name.split_whitespace().next().unwrap());
        round02::save(dir, &filename, &serde_json::to_value(&certificate).unwrap());
        out.push(json!({"file":filename,"id":certificate.id,"degree":degree,"lean_direct_statement":lean}));
    };
    let (a, b, w) = (E::var("a"), E::var("b"), E::var("w"));
    issue(
        "cm-norm integer lift",
        vec!["a", "b", "w"],
        4,
        times(
            plus(a.clone(), times(b.clone(), w.clone())),
            minus(minus(a.clone(), b.clone()), times(b.clone(), w.clone())),
        ),
        minus(
            plus(
                minus(p(a.clone(), 2), times(a, b.clone())),
                times(c(2), p(b.clone(), 2)),
            ),
            times(p(b, 2), plus(plus(p(w.clone(), 2), w), c(2))),
        ),
        false,
    );
    let (a, b, c0, d, x1, x2, y1, y2) = (
        E::var("a"),
        E::var("b"),
        E::var("c"),
        E::var("d"),
        E::var("x1"),
        E::var("x2"),
        E::var("y1"),
        E::var("y2"),
    );
    let dot = |a: E, x: E, b: E, y: E| plus(times(a, x), times(b, y));
    issue(
        "alternating-determinant universal identity",
        vec!["a", "b", "c", "d", "x1", "x2", "y1", "y2"],
        4,
        minus(
            times(
                dot(a.clone(), x1.clone(), b.clone(), x2.clone()),
                dot(c0.clone(), y1.clone(), d.clone(), y2.clone()),
            ),
            times(
                dot(c0.clone(), x1.clone(), d.clone(), x2.clone()),
                dot(a.clone(), y1.clone(), b.clone(), y2.clone()),
            ),
        ),
        times(
            minus(times(a, d), times(b, c0)),
            minus(times(x1, y2), times(x2, y1)),
        ),
        true,
    );
    let (u, v, a, d) = (E::var("u"), E::var("v"), E::var("a"), E::var("d"));
    let yy = plus(times(u.clone(), v.clone()), d.clone());
    let left = E::sum(vec![
        p(yy.clone(), 2),
        times(p(u.clone(), 3), yy),
        p(u.clone(), 9),
        times(a.clone(), p(u.clone(), 6)),
        p(d.clone(), 2),
    ]);
    let inside = E::sum(vec![
        p(v.clone(), 2),
        times(p(u.clone(), 2), v.clone()),
        p(u.clone(), 7),
        times(a, p(u.clone(), 4)),
        times(d.clone(), u.clone()),
    ]);
    issue(
        "cubic-cover integer lift",
        vec!["u", "v", "a", "d"],
        9,
        left,
        plus(
            times(p(u.clone(), 2), inside),
            times(c(2), plus(E::product(vec![u, v, d.clone()]), p(d, 2))),
        ),
        false,
    );
    out
}
fn walk(root: &Path, dir: &Path, map: &mut BTreeMap<String, String>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let p = entry.unwrap().path();
        if p.is_dir() {
            walk(root, &p, map);
        } else if !p
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("record-")
        {
            map.insert(
                p.strip_prefix(root).unwrap().to_str().unwrap().into(),
                hash(&p),
            );
        }
    }
}
pub fn record(round: &Path) {
    let raw = round.join("runs");
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo = manifest
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let ids = certificates(round, repo);
    let dp = read(&raw, "dp3-conic-twists.json");
    let g5 = read(&raw, "gl3-genus5-census.json");
    let g6 = read(&raw, "dp4-genus6-census.json");
    let gl = read(&raw, "gl2b-integral-polarizations.json");
    let n5 = read(&raw, "n5d-geometric-relations.json");
    let mut determinants: BTreeMap<i64, usize> = BTreeMap::new();
    let mut graph_cases: BTreeMap<i64, usize> = BTreeMap::new();
    let mut anti_count: BTreeMap<i64, u64> = BTreeMap::new();
    for row in gl["candidates"].as_array().unwrap() {
        let ell = row["determinant"].as_i64().unwrap();
        *determinants.entry(ell).or_default() += 1;
        for g in row["elliptic_gluings"].as_array().unwrap() {
            let n = g["anti_isometries"].as_u64().unwrap();
            if n > 0 {
                *graph_cases.entry(ell).or_default() += 1;
                *anti_count.entry(ell).or_default() += n;
            }
        }
    }
    let ns:Vec<_>=n5["records"].as_array().unwrap().iter().map(|r|json!({"a":r["a"],"field_degree":r["field_degree"],"genus":r["genus"],"columns":r["columns"],"relations":r["relations"].as_array().unwrap().len(),"rank":r["rank"],"recovered":r["recovered"],"jacobian_order":r["jacobian_order_over_F32"],"hermite":r["hermite_interpolation"]})).collect();
    let mut sources = BTreeMap::new();
    let study = repo.join("research/koblitz_modular_ic_20261018");
    walk(repo, &study.join("native/src"), &mut sources);
    for name in [
        "research/koblitz_modular_ic_20261018/native/Cargo.toml",
        "research/koblitz_modular_ic_20261018/native/Cargo.lock",
        "research/koblitz_modular_ic_20261018/lean/KoblitzRelations.lean",
        "research/koblitz_modular_ic_20261018/lean/KoblitzRound03.lean",
        "research/koblitz_modular_ic_20261018/lean/lakefile.toml",
        "research/koblitz_modular_ic_20261018/lean/lake-manifest.json",
        "research/koblitz_modular_ic_20261018/lean/lean-toolchain",
        "research/koblitz_modular_ic_20261018/rounds/03/PROTOCOL.md",
        "src/cryptanalysis/curve_construction/gf2k.rs",
        "src/cryptanalysis/curve_construction/lpoly.rs",
        "src/cryptanalysis/curve_construction/weil.rs",
        "src/cryptanalysis/curve_construction/modular.rs",
        "src/cryptanalysis/identity_certificate.rs",
        "src/hash/sha256.rs",
    ] {
        sources.insert(name.into(), hash(&repo.join(name)));
    }
    walk(repo, &repo.join("src/binary_ecc"), &mut sources);
    let mut receipts = BTreeMap::new();
    walk(&raw, &raw, &mut receipts);
    let ledger = json!({"schema":"koblitz.round03-ledger/v1","experiment_parent_commit":"d6c722e19a381f2b6178b6d5d75ce6a87aea4335","performance_measured":false,"new_named_lean_theorems":13,"new_audited_theorems_including_definition_equation":14,"prior_named_lean_theorems":13,"identity_certificates":ids,
        "dp3":{"forms_tested":dp["forms_tested_total"],"candidates":dp["families"].as_array().unwrap().iter().map(|f|f["candidates"].as_array().unwrap().len()).sum::<usize>(),"twists":[[5],[3,2]],"direct_controls":72,"surface_controls":12,"conic_fiber_checks":264},
        "genus5_census":{"records":g5["tables"].as_array().unwrap().iter().map(|t|t["rows"].as_u64().unwrap()).sum::<u64>(),"matches":g5["hits"].as_array().unwrap().len(),"commit":g5["source_commit"],"independent_classification":false},
        "genus6_census":{"polynomials":g6["polynomials"],"curves":g6["curves"],"matches":g6["hits"].as_array().unwrap().len(),"commit":g6["source_commit"],"independent_classification":false},
        "polarizations":{"forms_tested":gl["forms_tested"],"positive_forms":gl["positive_forms"],"sign_specific_candidates_by_determinant":determinants,"gluing_cases_by_prime":graph_cases,"anti_isometries_by_prime":anti_count,"jacobian_constructed":false},
        "genus4_count_obstructions":read(&raw,"g4-carrier-bounds.json"),"short_vectors":read(&raw,"gl4-short-vectors.json"),"n5":ns,"modular_character_screen":read(&raw,"mr3-determinant-screen.json"),
        "requirements":[
          {"track":"remaining del Pezzo twists","status":"verified complete within stated linear systems","evidence":"dp3-conic-twists.json","open":"not a proof about all genera; census extensions depend on published completeness","next":"genus4 count obstructions and genus5/6 censuses exclude full A5 carriers through genus6 conditionally; higher-genus construction remains separate"},
          {"track":"polarized gluing","status":"partial","evidence":"gl2b-integral-polarizations.json and separate integer/pairing/matrix checker","verified":"positive integral forms, Frobenius, polarization kernels and finite anti-isometry graphs","open":"geometric polarization/descent bridge not in Lean; quotient equations, Jacobian realization, global indecomposability","next":"use inverse-Hermitian coordinate bounds for a complete short-vector enumeration; classify the resulting quotient polarizations"},
          {"track":"n5 maps and relations","status":"partial","verified":"all11 augmentation and cubic-pullback subgroup points for both signs; 22 fixture recoveries from geometric rows on F32 covers","open":"evaluated F2 modular-Jacobian map and direct interior modular relations","next":"implement projectors/correspondences on the specified n5 modular factor, with actual divisor input/output and subgroup checks"},
          {"track":"Lean proofs","status":"partial","verified":"13 additional named lemmas; 14 audited declarations including the defining equation","open":"full blowup/contraction, Picard/principal-divisor, Hermitian-to-polarization, quotient descent and modular-cusp bridges","next":"formalize the instantiated divisor certificate and polarization kernel pairing bridges"},
          {"track":"IC efficiency assessment","status":"not attempted","next":"user assesses implications; no timing protocol in this round"}
        ],
        "failed_and_partial_runs":[
          {"file":"n5-run-01.log","kind":"compile failure","detail":"library point constructor required borrowed coordinates; fixed before experiments"},
          {"file":"n5-geometric-relations.json","kind":"rank deficiency","detail":"a1 y+v rows rank10/21, so no recovery attempted"},
          {"file":"n5b-geometric-relations.json","kind":"rank deficiency","detail":"vertical functions raise a1 rank16/21; no recovery"},
          {"file":"n5c-geometric-relations.json","kind":"negative subfamily and metadata correction","detail":"monic quartic subfamily adds no a1 rows; old polynomials_tested field says1048576 although two families (2097152) ran; corrected in N5d"},
          {"file":"gl2-integral-polarizations.json","kind":"scope limitation","detail":"initial integral circulant sublattice misses four congruence cosets; GL2b extends it"},
          {"file":"gl4-short-vectors.json","kind":"bounded negative search","detail":"no norm-one witness in coordinate box; not global indecomposability"},
          {"file":"lean-build-02.log and lean-build-03.log","kind":"proof elaboration failure","detail":"product subtraction and zero simplification in graph quotient lemma; final build/audit pass"},
          {"file":"setup-notes.log","kind":"path setup failure","detail":"initial Lean file write used the wrong working directory; no new theorem was built in that attempt"}
        ],"raw_sha256":receipts,"source_sha256":sources,"proof_map":"PROOF_MAP.md","commands":"REPLAY.sh","scope":"same-agent experiments and replay; no independent external audit and no new modular-Jacobian evaluator"});
    round02::save(round, "ledger.json", &ledger);
    println!("Round03 ledger sealed: 131070 sections; census7829/72227 curves; 24 polarization candidates; 22 recovered fixtures; 13 new Lean lemmas");
}
