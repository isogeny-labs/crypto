//! isobench: the counted field, pricing, specs and plans, sealing, every problem's checker, the
//! bound fit, and a full run / audit / bound / challenge cycle through the binary.
use isogeny_algos::field::{Field, Rng, Zp};
use isogeny_algos::fpm::FpM;
use isogeny_algos::isobench::bounds;
use isogeny_algos::isobench::counted::{self, Calibration, Counted};
use isogeny_algos::isobench::harness::{self, Spec};
use isogeny_algos::isobench::problems::{self, Params};
use isogeny_algos::json::Json;
use isogeny_algos::poly;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;

fn cal() -> Calibration {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/isobench/calibration.json"
    ))
    .unwrap();
    Calibration::from_json(&text).unwrap()
}

#[test]
fn counted_field_counts_element_operations_exactly() {
    let f = Zp::new(1_000_003);
    let g = Counted(f);
    counted::reset();
    let x = g.from_u64(5);
    let mut y = g.one();
    for _ in 0..10 {
        y = g.mul(y, x);
    }
    let _ = g.sq(y);
    let _ = g.inv(y);
    let _ = g.pow(x, 1000); // square-and-multiply on the wrapper: 10 squarings + 6 products
    let t = counted::take();
    assert_eq!((t.mul, t.sq, t.inv), (10 + 16, 1, 1));
    // results are those of the plain field
    assert_eq!(g.pow(x, 1000), f.pow(x, 1000));
}

#[test]
fn counted_products_equal_plain_and_are_priced() {
    let f = FpM::<2>::from_dec("170141183460469231731687303715884105727");
    let mut rng = Rng::new(3);
    let a: Vec<_> = (0..300).map(|_| f.random(&mut rng)).collect();
    let b: Vec<_> = (0..200).map(|_| f.random(&mut rng)).collect();
    counted::reset();
    let c = poly::mul(&Counted(f.clone()), &a, &b);
    let t = counted::take();
    assert_eq!(c, poly::mul(&f, &a, &b));
    assert!(!t.bulk.is_empty() || t.mul > 0);
    let fme = cal().price("fpm2", &t, 127.0).unwrap();
    // cheaper than schoolbook (60000 products) and not free
    assert!(fme > 100.0 && fme < 60000.0, "fme {fme}");
}

#[test]
fn price_interpolates_between_powers_of_two() {
    let c = cal();
    let p = &c.fields["fpm4"];
    for kind in counted::KINDS {
        let (lo, hi) = (p.price(kind, 64), p.price(kind, 128));
        let mid = p.price(kind, 90);
        assert!(
            mid >= lo.min(hi) * 0.999 && mid <= lo.max(hi) * 1.001,
            "{kind}"
        );
        assert!((p.price(kind, 64) - p.tables[kind][6]).abs() < 1e-6 * p.tables[kind][6]);
    }
}

fn spec_text() -> String {
    r#"{"schema":"isobench.spec/v1","label":"t","description":"t","problem":"kernel.codomain","params":{"bits":"61"},"sizes":[11,37],"instances_per_size":2,"arms":[{"name":"a","role":"reference","method":"xonly"},{"name":"b","role":"candidate","method":"sqrtvelu"}],"measurement":{"rounds":2,"seed":4}}"#.into()
}

#[test]
fn plans_are_deterministic_and_paired() {
    let s = Spec::parse(&spec_text()).unwrap();
    let p = harness::plan(&s);
    assert_eq!(p.len(), 2 * 2 * 2 * 2);
    assert_eq!(p, harness::plan(&Spec::parse(&spec_text()).unwrap()));
    // both arms of one (size, instance, round) share the seed; rounds alternate the order
    for pair in p.chunks(2) {
        assert_eq!(pair[0].seed, pair[1].seed);
        assert_ne!(pair[0].arm, pair[1].arm);
        assert_eq!(pair[0].arm, if pair[0].round % 2 == 0 { 0 } else { 1 });
    }
    // unknown methods, roles and parameters are refused
    assert!(Spec::parse(&spec_text().replace("sqrtvelu", "nope")).is_err());
    assert!(Spec::parse(&spec_text().replace("candidate", "winner")).is_err());
    assert!(Spec::parse(&spec_text().replace("\"bits\"", "\"bitz\"")).is_err());
}

#[test]
fn seals_detect_any_change() {
    let doc = harness::sealed(
        "ISX1h",
        Json::obj(vec![("a", Json::Num(1)), ("b", Json::str("x"))]),
    );
    let (claimed, again) = harness::reseal("ISX1h", &doc).unwrap();
    assert_eq!(claimed, again);
    let Json::Obj(mut kv) = doc else {
        unreachable!()
    };
    kv[1].1 = Json::Num(2);
    let (claimed, again) = harness::reseal("ISX1h", &Json::Obj(kv)).unwrap();
    assert_ne!(claimed, again);
}

/// Every problem at a small size: the methods agree, the checker accepts the answer and
/// rejects a corrupted one, and counted runs are reproducible.
#[test]
fn every_problem_runs_verifies_and_rejects_wrong_answers() {
    let c = cal();
    let cases: Vec<(&str, u64, Vec<(&str, &str)>)> = vec![
        ("kernel.codomain", 37, vec![("bits", "61")]),
        ("kernel.chain", 6, vec![("bits", "127"), ("ell", "3")]),
        ("find.kernel", 7, vec![("bits", "61")]),
        ("modpoly.phi", 5, vec![("bits", "61")]),
        ("count.sea", 64, vec![]),
        ("path.ordinary", 16, vec![]),
        ("path.supersingular", 12, vec![]),
        ("csidh.action", 1, vec![("preset", "n=6")]),
        ("klpt", 30, vec![]),
        ("radical.chain", 6, vec![("ell", "7"), ("bits", "61")]),
        ("radical.chain", 6, vec![("ell", "5"), ("bits", "61")]),
    ];
    for (problem, size, ps) in cases {
        let decl = problems::problem(problem).unwrap();
        let given: Params = ps
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let params = problems::resolve_params(decl, &given).unwrap();
        let mut answers = BTreeMap::new();
        for &m in decl.methods {
            let o = harness::child(problem, m, &params, size, 0, 11, &c);
            assert!(o.error.is_none(), "{problem}/{m}: {:?}", o.error);
            let a = o
                .answer
                .clone()
                .unwrap_or_else(|| panic!("{problem}/{m}: no answer"));
            problems::verify(problem, m, &params, size, 0, &a)
                .unwrap_or_else(|e| panic!("{problem}/{m}: {e}"));
            assert!(o.cost > 0.0, "{problem}/{m}: cost");
            // reproducible
            let again = harness::child(problem, m, &params, size, 0, 11, &c);
            assert_eq!(
                (again.answer, again.cost, again.tally),
                (o.answer, o.cost, o.tally),
                "{problem}/{m}"
            );
            answers.insert(m, a);
        }
        // a corrupted answer is rejected
        let (m, a) = answers.iter().next().unwrap();
        let bad = a.replacen('1', "2", 1);
        if bad != *a {
            assert!(
                problems::verify(problem, m, &params, size, 0, &bad).is_err(),
                "{problem}: corrupted answer accepted"
            );
        }
        // deterministic problems: every method gives the same answer
        if !problem.starts_with("path.") && problem != "klpt" {
            let v: Vec<&String> = answers.values().collect();
            assert!(
                v.windows(2).all(|w| w[0] == w[1]),
                "{problem}: methods disagree"
            );
        }
    }
}

#[test]
fn bound_fit_recovers_a_planted_exponent() {
    let mut pts = BTreeMap::new();
    for (k, s) in [8u64, 16, 32, 64, 128].iter().enumerate() {
        let base = 3.0 * (*s as f64).powf(1.5);
        pts.insert(
            *s,
            vec![base * 0.98, base * 1.02, base * (1.0 + 0.001 * k as f64)],
        );
    }
    let fit = bounds::fit_points(&pts, problems::Model::Power).unwrap();
    assert!((fit.alpha - 1.5).abs() < 0.02, "alpha {}", fit.alpha);
    assert!(fit.alpha_ci.0 <= 1.5 && 1.5 <= fit.alpha_ci.1 + 0.01);
    assert!((fit.c / 3.0 - 1.0).abs() < 0.1, "C {}", fit.c);
}

/// run -> verify --replay-all -> bound fit / check -> frontier --check -> challenge verdict,
/// through the binary, as a user would.
#[test]
fn full_cycle_through_the_binary() {
    let bin = env!("CARGO_BIN_EXE_isobench");
    let dir = std::env::temp_dir().join(format!("isobench-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let calp = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/isobench/calibration.json"
    ));
    let run = |args: &[&str]| {
        let o = Command::new(bin).args(args).output().unwrap();
        assert!(
            o.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&o.stderr)
        );
        String::from_utf8_lossy(&o.stdout).to_string()
    };
    let spec = dir.join("spec.json");
    std::fs::write(&spec, spec_text()).unwrap();
    let s1 = dir.join("sessions/s1");
    run(&[
        "run",
        spec.to_str().unwrap(),
        "--out",
        s1.to_str().unwrap(),
        "--cal",
        calp.to_str().unwrap(),
    ]);
    assert!(run(&[
        "verify",
        s1.to_str().unwrap(),
        "--cal",
        calp.to_str().unwrap(),
        "--replay-all"
    ])
    .contains("\"PASS\""));
    let bdir = dir.join("bounds");
    std::fs::create_dir_all(&bdir).unwrap();
    for arm in ["a", "b"] {
        let out = bdir.join(format!("{arm}.json"));
        run(&[
            "bound",
            "fit",
            "--arm",
            arm,
            "--out",
            out.to_str().unwrap(),
            s1.to_str().unwrap(),
        ]);
        run(&[
            "bound",
            "check",
            out.to_str().unwrap(),
            "--sessions",
            dir.join("sessions").to_str().unwrap(),
        ]);
    }
    let (md, js) = (dir.join("F.md"), dir.join("f.json"));
    let fargs = [
        "frontier",
        "build",
        "--bounds",
        bdir.to_str().unwrap(),
        "--out",
        md.to_str().unwrap(),
        "--json",
        js.to_str().unwrap(),
    ];
    run(&fargs);
    let mut check = fargs.to_vec();
    check.push("--check");
    run(&check);
    // two sizes make no scaling claim: the cheaper method at the largest size dominates
    let page = std::fs::read_to_string(&md).unwrap();
    assert!(page.contains("| xonly | frontier"), "{page}");
    assert!(page.contains("| sqrtvelu | dominated by xonly"), "{page}");
    // tampering is caught
    let rec = s1.join("records.jsonl");
    let t = std::fs::read_to_string(&rec).unwrap();
    std::fs::write(&rec, t.replacen("\"verified\"", "\"wrong_answer\"", 1)).unwrap();
    let o = Command::new(bin)
        .args([
            "verify",
            s1.to_str().unwrap(),
            "--cal",
            calp.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!o.status.success());
    std::fs::write(&rec, t).unwrap();
    // a challenge: sqrt-Velu against x-only Velu over four sizes
    let ch = dir.join("ch.json");
    run(&[
        "challenge",
        "seal",
        "--problem",
        "kernel.codomain",
        "--params",
        "bits=61",
        "--incumbent",
        "xonly",
        "--candidate",
        "sqrtvelu",
        "--sizes",
        "37,101,211,401",
        "--instances",
        "2",
        "--rounds",
        "2",
        "--seed",
        "3",
        "--out",
        ch.to_str().unwrap(),
    ]);
    let chspec = dir.join("chspec.json");
    std::fs::write(&chspec, run(&["challenge", "spec", ch.to_str().unwrap()])).unwrap();
    let cs = dir.join("sessions/ch");
    run(&[
        "run",
        chspec.to_str().unwrap(),
        "--out",
        cs.to_str().unwrap(),
        "--cal",
        calp.to_str().unwrap(),
    ]);
    let v = run(&[
        "challenge",
        "verdict",
        "--challenge",
        ch.to_str().unwrap(),
        "--session",
        cs.to_str().unwrap(),
        "--out",
        dir.join("v.json").to_str().unwrap(),
    ]);
    let v = Json::parse(v.trim()).unwrap();
    // the counted unit decides advances; unisolated wall time may still make it a trade
    let outcome = v.get("outcome").and_then(|o| o.as_str()).unwrap();
    assert!(outcome == "advances" || outcome == "trade", "{}", v.dump());
    let Some(Json::Arr(ci)) = v.get("cost_ratio_ci95") else {
        panic!()
    };
    assert!(ci[1].as_f64().unwrap() < 1.0, "{}", v.dump());
    assert_eq!(v.get("level").and_then(|o| o.as_str()), Some("exponent"));
    let _ = std::fs::remove_dir_all(&dir);
}
