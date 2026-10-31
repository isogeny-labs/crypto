//! Round evidence assembly and exact integer lift of the characteristic-2 identity.
use crate::{identity_certificate as ic, round02, sha256_impl, weil};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;
type Poly = BTreeMap<[u32; 3], i64>;
fn add(mut a: Poly, b: &Poly, sign: i64) -> Poly {
    for (m, c) in b {
        *a.entry(*m).or_default() += sign * c;
    }
    a.retain(|_, c| *c != 0);
    a
}
fn mul(a: &Poly, b: &Poly) -> Poly {
    let mut out = Poly::new();
    for (m, c) in a {
        for (n, d) in b {
            *out.entry([m[0] + n[0], m[1] + n[1], m[2] + n[2]])
                .or_default() += c * d;
        }
    }
    out.retain(|_, c| *c != 0);
    out
}
fn pow(a: &Poly, n: usize) -> Poly {
    (0..n).fold([([0, 0, 0], 1)].into_iter().collect(), |b, _| mul(&b, a))
}
fn expr(poly: &Poly) -> ic::Expr {
    ic::Expr::sum(
        poly.iter()
            .map(|(m, c)| {
                ic::Expr::product(
                    std::iter::once(ic::Expr::Const(*c))
                        .chain(m.iter().enumerate().flat_map(|(i, n)| {
                            (0..*n).map(move |_| ic::Expr::var(["x", "y", "z"][i]))
                        }))
                        .collect(),
                )
            })
            .collect(),
    )
}
fn lifted_identity(dir: &Path) -> String {
    let x: Poly = [([1, 0, 0], 1)].into_iter().collect();
    let y: Poly = [([0, 1, 0], 1)].into_iter().collect();
    let z: Poly = [([0, 0, 1], 1)].into_iter().collect();
    let a = add(add(x.clone(), &y, 1), &z, 1);
    let b = add(add(mul(&x, &y), &mul(&y, &z), 1), &mul(&z, &x), 1);
    let c = mul(&mul(&x, &y), &z);
    let delta = add(
        add(
            add(
                add(
                    mul(&pow(&a, 2), &pow(&c, 2)),
                    &mul(&mul(&a, &pow(&b, 2)), &c),
                    1,
                ),
                &mul(&mul(&pow(&a, 3), &b), &c),
                1,
            ),
            &mul(&pow(&a, 5), &c),
            1,
        ),
        &pow(&b, 4),
        1,
    );
    let f = add(
        add(mul(&pow(&x, 3), &y), &mul(&pow(&y, 3), &z), 1),
        &mul(&pow(&z, 3), &x),
        1,
    );
    let rev = add(
        add(mul(&x, &pow(&y, 3)), &mul(&y, &pow(&z, 3)), 1),
        &mul(&z, &pow(&x, 3)),
        1,
    );
    let product = mul(&f, &rev);
    let residual = add(delta.clone(), &product, -1);
    assert!(residual.values().all(|c| c % 2 == 0));
    let half: Poly = residual.iter().map(|(m, c)| (*m, c / 2)).collect();
    let rhs = add(product, &half, 2);
    assert_eq!(delta, rhs);
    let statement = ic::IdentityStatement {
        name:"Klein invariant integer lift: Delta = F F_reverse + 2 H".into(),
        source:"round02 exact sparse expansion; reduction modulo 2 is KoblitzRelations.klein_invariant_identity; v1 certificate is over its declared odd prime".into(),
        variables:vec!["x".into(),"y".into(),"z".into()], degree_bound:8,
        lhs:expr(&delta),rhs:ic::Expr::plus(ic::Expr::times(expr(&f),expr(&rev)),ic::Expr::times(ic::Expr::Const(2),expr(&half)))
    };
    let cert = ic::issue(statement,2026101802,64,"same-agent native generator; extra-point replay and Lean char-2 theorem are distinct checks".into(),None).unwrap();
    assert!(matches!(
        ic::check_with_extra(&cert, Some((0x4b4c45494e, 64))),
        Ok(ic::Verdict::Accept { .. })
    ));
    let expansion = json!({"half_residual_terms":half.iter().map(|(m,c)| json!({"exponents":m,"coefficient":c})).collect::<Vec<_>>(),"all_residual_coefficients_even":true,"exact_integer_expansion_equal":true,"certificate_id":cert.id,"lean_characteristic":2,"certificate_characteristic":ic::MODULUS,"scope":"integer lift and its characteristic-two reduction are explicitly distinguished"});
    for (name, value) in [
        (
            "identity-klein-lift.json",
            serde_json::to_value(&cert).unwrap(),
        ),
        ("identity-klein-expansion.json", expansion),
    ] {
        if dir.join(name).exists() {
            assert_eq!(read(dir, name), value);
        } else {
            round02::save(dir, name, &value);
        }
    }
    cert.id
}
fn read(dir: &Path, name: &str) -> Value {
    serde_json::from_slice(&std::fs::read(dir.join(name)).unwrap()).unwrap()
}
fn hash(path: &Path) -> String {
    hex::encode(sha256_impl::sha256(&std::fs::read(path).unwrap()))
}
pub fn record(dir: &Path) {
    let lc0 = read(dir, "lc0-bielliptic.json");
    let lc0b = read(dir, "lc0b-closed-branch.json");
    let lc1 = read(dir, "lc1-bidegree34.json");
    let lc1b = read(dir, "lc1b-unbalanced.json");
    let lc2 = read(dir, "lc2-delpezzo.json");
    let mr2 = read(dir, "mr2-klein-line-relations-v2.json");
    let mr2b = read(dir, "mr2b-klein-wide-relations.json");
    let certificate_id = lifted_identity(dir);
    let mut inputs = BTreeMap::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let p = entry.unwrap().path();
        let filename = p.file_name().unwrap().to_str().unwrap();
        // The assembly receipt is still being written. Prior ledgers are
        // outputs, not inputs. The archive manifest hashes all final receipts.
        if !filename.starts_with("record-")
            && !filename.starts_with("ledger")
            && p.is_file()
            && matches!(p.extension().and_then(|s| s.to_str()), Some("json" | "log"))
        {
            inputs.insert(
                p.file_name().unwrap().to_str().unwrap().to_string(),
                hash(&p),
            );
        }
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut sources = BTreeMap::new();
    for name in [
        "native/src/main.rs",
        "native/src/round02.rs",
        "native/src/klein_transfer.rs",
        "native/src/delpezzo.rs",
        "native/src/gluing.rs",
        "native/src/round_record.rs",
        "native/Cargo.toml",
        "native/Cargo.lock",
        "lean/KoblitzRelations.lean",
        "lean/lakefile.toml",
        "lean/lake-manifest.json",
        "lean/lean-toolchain",
        "rounds/02/PROTOCOL.md",
    ] {
        sources.insert(name, hash(&root.join(name)));
    }
    let recovery = |v: &Value| {
        v["target_recovery"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["verified_by_multiplication"] == true)
            .count()
    };
    let statuses = json!([
      {"track":"low-genus carriers","status":"partial","verified":"bounded LC0/LC0b/LC1/LC1b/LC2 families enumerated; no A5 match","open":"two remaining S5 del Pezzo twist types; additional genus5 families; higher genera and explicit polarized gluing","next":"implement anticanonical descent for del Pezzo Frobenius types 5 and 3+2; validate against the five executed types"},
      {"track":"direct modular relations","status":"partial","verified":"Klein line relations, anchored factor-base logs and seven recovered public fixtures","open":"actual cross-orbit relations and Hecke/cusp bridge at n5 and n131","next":"implement interior principal-function relations on an evaluated n5 modular factor; retain MR1 as necessary screen only"},
      {"track":"transfer and subgroup action","status":"partial","verified":"n3 all seven points, 42 nonzero and14 zero maps; separate certificate checker","open":"n5 and n131 evaluated transfer/projector with torsion kernel check","next":"construct selected polarized n5 carrier/map before scaling; include all exceptional fibers"},
      {"track":"gluing and polarization","status":"partial","verified":"resultants5,11,19 and eight exact Frobenius Hom spaces","open":"integral lattices beyond companion model, alternating pairing, principal polarization, Jacobian realization","next":"enumerate Rosati-compatible integral lattices at ell5,11,19; check maximal isotropy and descent"},
      {"track":"Lean proofs","status":"partial","verified":"13 structural lemmas plus earlier conditional cyclic numerical lemma, pinned kernel build and axiom audits","open":"full function-field/divisor/Riemann-Roch, modular-factor and polarized-Jacobian bridges","next":"formalize exact divisor-to-class certificate criterion and connect it to the explicit Klein model"},
      {"track":"IC speed assessment","status":"not attempted","verified":null,"open":"no timing protocol or matched cost experiment","next":"user determines assessment and subsequent performance experiment"}
    ]);
    let summary = json!({"schema":"koblitz.round02-ledger/v1","parent_commit":"047074d9912d64842995f9d590d8c0babdd48042","performance_measured":false,
      "genus5_models":lc0["models_tested"].as_u64().unwrap()+lc0b["models_tested"].as_u64().unwrap(),
      "genus6_forms_and_sections":lc1["forms_tested"].as_u64().unwrap()+lc1b["forms_tested"].as_u64().unwrap()+lc2["forms_tested_total"].as_u64().unwrap(),
      "a5_hits_in_executed_families":lc0["hits"].as_array().unwrap().iter().filter(|h|h["n"]==5).count()+lc0b["hits"].as_array().unwrap().len()+lc1["candidates"].as_array().unwrap().len()+lc1b["candidates"].as_array().unwrap().len()+lc2["families"].as_array().unwrap().iter().map(|f|f["candidates"].as_array().unwrap().len()).sum::<usize>(),
      "mr2":{"relations":mr2["line_relations"],"columns":mr2["columns"],"rank":mr2["rank_mod_7"],"recovered":recovery(&mr2),"fixtures":7},
      "mr2b":{"relations":mr2b["line_relations"],"columns":mr2b["columns"],"rank":mr2b["rank_mod_7"],"recovered":recovery(&mr2b),"fixtures":7},
      "mr0":read(dir,"mr0-orbit-rank.json"),"mr1":read(dir,"mr1-eisenstein-screen.json"),"gl0":read(dir,"gl0-resultants.json"),"gl1":read(dir,"gl1-frobenius-modules.json"),
      "klein_identity_certificate":certificate_id,"requirements":statuses,"raw_sha256":inputs,"source_sha256":sources,
      "failures_preserved":["native-tests-01.log: initial failed test/build attempt; see raw log","lean-setup.log: obsolete cache path rejected after cache download","lean-build-01.log: initial proof elaboration failures","kt0-klein-transfer-v2.json: three unsupported fibers per map over F8; null first_nonzero is incomplete-coverage artifact","mr2-run-02.log and mr2b-run-01.log: cosmetic rename compile failure; corrected before execution","mr2-klein-line-relations-v2.json: three failed descents retained", "native-tests-03.log: wrong enum variant in evidence assembler, fixed", "replay-01.log: historical KT0 v2 versus v3 metadata mismatch; replay-02 compares matching schema"],
      "experiment_commands_file":"REPLAY.sh","proof_map":"PROOF_MAP.md","limitations":"same-agent implementation and replay; no external independent reviewer or n5/n131 relation collector"});
    assert_eq!(summary["a5_hits_in_executed_families"], 0);
    round02::save(dir, "ledger.json", &summary);
    println!(
        "ledger complete: {} genus5 models, {} genus6 sections; MR2b {}/7",
        summary["genus5_models"],
        summary["genus6_forms_and_sections"],
        recovery(&mr2b)
    );
    // Ensure the imported target constructor remains linked to both signs.
    assert_eq!(weil::trace_zero_poly(-1, 5).len(), 9);
}
