use super::*;
use crate::store::*;
use arithmetic::*;
use evaluate::*;
use num_traits::One;
use recipe::*;
use serde_json::json;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

fn fixture(binary: Option<usize>) -> CurveSpec {
    let field = if let Some(degree) = binary {
        FieldSpec::Binary {
            degree,
            modulus: if degree == 4 { "19" } else { "37" }.into(),
        }
    } else {
        FieldSpec::Prime {
            modulus: "101".into(),
        }
    };
    let f = Field::new(&field).unwrap();
    let a = if binary.is_some() { n(0) } else { n(2) };
    let b = if binary == Some(4) {
        n(6)
    } else if binary.is_some() {
        n(1)
    } else {
        n(3)
    };
    let placeholder = CurveSpec {
        field,
        a: enc(&a),
        b: enc(&b),
        group_order: "1".into(),
        subgroup_order: "2".into(),
        cofactor: "1".into(),
        generator: PointSpec {
            x: "0".into(),
            y: "0".into(),
        },
    };
    let mut c = Curve {
        spec: placeholder,
        f,
        a,
        b,
        r: n(2),
        h: n(1),
        order: n(1),
        g: Point { x: n(0), y: n(0) },
        validation: json!({}),
    };
    let points = (0..if binary == Some(4) {
        16
    } else if binary.is_some() {
        32
    } else {
        101
    })
        .flat_map(|x| c.lift(&n(x)))
        .collect::<Vec<_>>();
    let order = points.len() as u64 + 1;
    let mut r = 2;
    for q in 2..=order {
        if order.is_multiple_of(q) && probable_prime(&n(q)) && !(order / q).is_multiple_of(q) {
            r = q;
        }
    }
    c.r = n(r);
    c.h = n(order / r);
    c.order = n(order);
    c.g = points
        .iter()
        .find_map(|p| c.scale(Some(p.clone()), &c.h))
        .unwrap();
    c.spec.group_order = enc(&c.order);
    c.spec.subgroup_order = enc(&c.r);
    c.spec.cofactor = enc(&c.h);
    c.spec.generator = c.g.spec();
    c.spec
}
fn config() -> Config {
    Config {
        curve: fixture(Some(5)),
        recipe: Recipe::BinarySubspace {
            shape: Shape::Coordinate,
            dimension: 3,
            seed: 12,
            offsets: vec!["0".into()],
        },
        max_candidates: 256,
        shard_points: 3,
    }
}
fn workload() -> Workload {
    Workload {
        split: "discovery".into(),
        seed: 42,
        queries: 12,
        arity: 2,
        relation_state_cap: 100000,
        regularity_variable_cap: 10,
        regularity_degree_cap: 11,
        regularity_targets: 2,
        targets: None,
    }
}
static NEXT: AtomicUsize = AtomicUsize::new(0);
fn full_width_configs() -> Vec<Config> {
    let field = FieldSpec::Binary {
        degree: 83,
        modulus: enc(&((N::one() << 83) | (N::one() << 45) | n(7))),
    };
    let mut spec = CurveSpec {
        field: field.clone(),
        a: "0".into(),
        b: "1".into(),
        group_order: enc(&(parse("2417851639230796216685689").unwrap() * n(4))),
        subgroup_order: "2417851639230796216685689".into(),
        cofactor: "4".into(),
        generator: PointSpec {
            x: "0".into(),
            y: "1".into(),
        },
    };
    let c = Curve {
        spec: spec.clone(),
        f: Field::new(&field).unwrap(),
        a: n(0),
        b: n(1),
        r: parse(&spec.subgroup_order).unwrap(),
        h: n(4),
        order: parse(&spec.group_order).unwrap(),
        g: Point { x: n(0), y: n(1) },
        validation: json!({}),
    };
    spec.generator = (1..10000)
        .find_map(|x| {
            c.lift(&n(x))
                .first()
                .and_then(|p| c.scale(Some(p.clone()), &n(4)))
        })
        .unwrap()
        .spec();
    let mut binary = config();
    binary.curve = spec;
    binary.recipe = Recipe::BinarySubspace {
        shape: Shape::RandomLinear,
        dimension: 4,
        seed: 42,
        offsets: vec!["0".into()],
    };
    let mut prime = config();
    prime.curve = CurveSpec {
        field: FieldSpec::Prime {
            modulus: "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f".into(),
        },
        a: "0".into(),
        b: "7".into(),
        group_order: "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141".into(),
        subgroup_order: "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141".into(),
        cofactor: "1".into(),
        generator: PointSpec {
            x: "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".into(),
            y: "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8".into(),
        },
    };
    prime.recipe = Recipe::PrimeProgression {
        start: "0".into(),
        step: "1".into(),
        count: 16,
    };
    vec![binary, prime]
}
fn temp() -> PathBuf {
    let p = std::env::temp_dir().join(format!(
        "factor-base-tests-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn fields_reject_bad_moduli() {
    assert!(Field::new(&FieldSpec::Binary {
        degree: 4,
        modulus: "21".into()
    })
    .is_err());
    assert!(Field::new(&FieldSpec::Prime {
        modulus: "341".into()
    })
    .is_err());
    for p in [2, 3, 5, 101, 65537] {
        assert!(probable_prime(&n(p)));
    }
    for p in [0, 1, 4, 561, 3215031751] {
        assert!(!probable_prime(&n(p)));
    }
}
#[test]
fn inverses_match_exponentiation() {
    for binary in [Some(4), Some(5), None] {
        let c = Curve::new(fixture(binary)).unwrap();
        for i in 1..if binary.is_some() { 16 } else { 101 } {
            let a = n(i);
            assert_eq!(c.f.inv(&a), c.f.pow(&a, &(c.f.size() - n(2))));
        }
    }
}
#[test]
fn full_width_generation_and_identity() {
    for c in full_width_configs() {
        let g = generate(&c).unwrap();
        assert!(!g.points.is_empty());
        assert!(g.points.iter().all(|p| g.curve.subgroup(p)));
        assert_eq!(
            g.curve.validation["subgroup_primality"],
            "probable_prime_only"
        );
    }
}
#[test]
fn lifts_and_group_law_exhaustive() {
    for binary in [Some(4), Some(5), None] {
        let spec = fixture(binary);
        let c = Curve::new(spec.clone()).unwrap_or_else(|e| panic!("{spec:?}: {e}"));
        let size = if binary == Some(4) {
            16
        } else if binary.is_some() {
            32
        } else {
            101
        };
        let mut points = vec![None];
        for x in 0..size {
            let lifts = c.lift(&n(x));
            let brute = (0..size)
                .map(|y| Point { x: n(x), y: n(y) })
                .filter(|p| c.on_curve(p))
                .collect::<Vec<_>>();
            assert_eq!(lifts, brute);
            points.extend(lifts.into_iter().map(Some));
        }
        for p in &points {
            assert_eq!(c.scale(p.clone(), &c.order), None);
            for q in &points {
                let sum = c.add(p, q);
                assert!(sum.as_ref().is_none_or(|r| c.on_curve(r)));
                assert_eq!(sum, c.add(q, p));
                assert_eq!(
                    c.add(&sum, &Some(c.g.clone())),
                    c.add(p, &c.add(q, &Some(c.g.clone())))
                );
            }
        }
    }
}
#[test]
fn scalar_matches_repeated_addition() {
    for binary in [Some(4), Some(5), None] {
        let c = Curve::new(fixture(binary)).unwrap();
        let mut sum = None;
        for i in 0..100 {
            assert_eq!(sum, c.scale(Some(c.g.clone()), &n(i)));
            sum = c.add(&sum, &Some(c.g.clone()));
        }
    }
}
#[test]
fn invalid_curve_and_subgroup_fail_closed() {
    let mut s = fixture(Some(5));
    s.group_order = "40".into();
    assert!(Curve::new(s).is_err());
    let mut s = fixture(None);
    s.generator.x = "102".into();
    assert!(Curve::new(s).is_err());
    let mut s = fixture(Some(5));
    s.b = "0".into();
    assert!(Curve::new(s).is_err());
}
#[test]
fn every_recipe_is_deterministic_and_subgroup_checked() {
    let mut c = config();
    let binary = [
        Recipe::BinarySubspace {
            shape: Shape::RandomLinear,
            dimension: 4,
            seed: 6,
            offsets: vec!["0".into(), "8".into()],
        },
        Recipe::BinarySubspace {
            shape: Shape::Geometric,
            dimension: 4,
            seed: 7,
            offsets: vec!["0".into()],
        },
        Recipe::BinarySubspace {
            shape: Shape::TraceZero,
            dimension: 4,
            seed: 8,
            offsets: vec!["0".into()],
        },
        Recipe::BinaryExplicit {
            basis: vec!["1".into(), "2".into(), "4".into()],
            offsets: vec!["0".into()],
        },
        Recipe::FrobeniusOrbits { seeds: 1, seed: 9 },
        Recipe::RandomX {
            count: 20,
            seed: 10,
        },
    ];
    for r in binary {
        c.recipe = r;
        let g = generate(&c).unwrap_or_else(|e| panic!("{:?}: {e}", c.recipe));
        assert_eq!(g.points, generate(&c).unwrap().points);
        assert!(g.points.iter().all(|p| g.curve.subgroup(p)));
        if matches!(c.recipe, Recipe::FrobeniusOrbits { .. }) {
            assert_eq!(g.structure["frobenius_closed"], true);
        }
    }
    c.curve = fixture(None);
    c.recipe = Recipe::PrimeProgression {
        start: "0".into(),
        step: "3".into(),
        count: 101,
    };
    let g = generate(&c).unwrap();
    assert_eq!(n(g.points.len() as u64), g.curve.r - N::one());
    assert!(generate(&Config {
        recipe: Recipe::FrobeniusOrbits { seeds: 1, seed: 1 },
        ..c
    })
    .is_err());
}
#[test]
fn recipe_caps_and_dependency_rejected() {
    let mut c = config();
    c.recipe = Recipe::BinaryExplicit {
        basis: vec!["1".into(), "1".into()],
        offsets: vec!["0".into()],
    };
    assert!(generate(&c).is_err());
    c = config();
    c.max_candidates = 1;
    assert!(generate(&c).is_err());
}
#[test]
fn archives_replay_idempotently_and_detect_corruption() {
    let root = temp();
    let c = config();
    let id = build(&c, &root).unwrap();
    assert_eq!(id, build(&c, &root).unwrap());
    let (m, _) = verify(&root, &id).unwrap();
    let path = object_path(&root, &m.objects[0].sha256).unwrap();
    std::fs::write(&path, b"corrupt").unwrap();
    assert!(verify(&root, &id).is_err());
    assert!(build(&c, &root).is_err());
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn archive_rejects_rehashed_false_metadata() {
    let root = temp();
    let id = build(&config(), &root).unwrap();
    let (mut m, _) = load(&root, &id).unwrap();
    m.structure["subgroup_points"] = json!(999);
    let bytes = canonical(&m).unwrap();
    let false_id = digest(&bytes);
    write_once(&manifest_path(&root, &false_id).unwrap(), &bytes).unwrap();
    assert!(verify(&root, &false_id).is_err());
    m.objects[0].points += 1;
    let bytes = canonical(&m).unwrap();
    let bad_count = digest(&bytes);
    write_once(&manifest_path(&root, &bad_count).unwrap(), &bytes).unwrap();
    assert!(load(&root, &bad_count).is_err());
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn relations_replay_and_caps_keep_unknown() {
    let root = temp();
    let id = build(&config(), &root).unwrap();
    let w = workload();
    let r = evaluate(&root, &id, &w).unwrap();
    assert_eq!(r["semantic"]["relation_status"], "complete");
    assert!(replay(&root, &id, &r).is_ok());
    let mut capped = w.clone();
    capped.relation_state_cap = 1;
    let censored = evaluate(&root, &id, &capped).unwrap();
    assert!(censored["semantic"]["relation_yield"].is_null());
    assert_eq!(
        censored["semantic"]["relation_status"],
        "censored_state_cap"
    );
    let (_, g) = verify(&root, &id).unwrap();
    let targets = sample_targets(&g.curve, w.queries, w.seed, &w.split).unwrap();
    for (i, target) in targets.iter().enumerate() {
        let mut found = false;
        for p in &g.points {
            for q in &g.points {
                if g.curve.neg(p) != *q
                    && g.curve.add(&Some(p.clone()), &Some(q.clone())) == Some(target.clone())
                {
                    found = true;
                }
            }
        }
        assert_eq!(r["semantic"]["witnesses"][i].is_array(), found);
    }
    let mut bad = r.clone();
    bad["semantic"]["verified_hits"] = json!(999);
    assert!(replay(&root, &id, &bad).is_err());
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn leading_form_regularity_known_ideals() {
    assert_eq!(leading_regularity(&[vec![3]], 2, 2), Some(2));
    assert_eq!(leading_regularity(&[vec![3]], 3, 3), Some(3));
    assert_eq!(leading_regularity(&[vec![3]], 3, 2), None);
    assert_eq!(leading_regularity(&[vec![1], vec![2]], 2, 2), Some(1));
    assert_eq!(leading_regularity(&[vec![0]], 2, 2), Some(0));
    assert_eq!(leading_regularity(&[], 3, 4), Some(4));
}
#[test]
fn binary_chain_matches_finite_group_sums_and_caps() {
    let g = generate(&config()).unwrap();
    let c = &g.curve;
    for p in &g.points {
        for q in &g.points {
            if let Some(sum) = c.add(&Some(p.clone()), &Some(q.clone())) {
                assert_eq!(semaev_s3(c, &p.x, &q.x, &sum.x), n(0));
                for r in &g.points {
                    if let Some(target) = c.add(&Some(sum.clone()), &Some(r.clone())) {
                        assert_eq!(semaev_s3(c, &sum.x, &r.x, &target.x), n(0));
                    }
                }
            }
        }
    }
    let mut w = workload();
    w.arity = 3;
    w.regularity_variable_cap = 14;
    w.regularity_degree_cap = 2;
    let measured = regularity(&g, &c.g, &w);
    assert_eq!(measured["variables"], 14);
    assert_eq!(measured["equations"], 10);
    assert!(measured["system_total_degree"].as_u64().unwrap() <= 3);
    w.arity = 8;
    assert_eq!(regularity(&g, &c.g, &w)["status"], "censored_variable_cap");
}
#[test]
fn relation_arities_two_through_eight_are_replayable() {
    let root = temp();
    let id = build(&config(), &root).unwrap();
    for m in 2..=8 {
        let mut w = workload();
        w.arity = m;
        w.queries = 3;
        w.regularity_targets = 0;
        w.relation_state_cap = 1000;
        let result = evaluate(&root, &id, &w).unwrap();
        replay(&root, &id, &result).unwrap();
        assert!(result["semantic"]["visited_states"].as_u64().unwrap() <= 1000);
    }
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn regularity_and_comparison_keep_missing_values() {
    let root = temp();
    let id = build(&config(), &root).unwrap();
    let r = evaluate(&root, &id, &workload()).unwrap();
    assert_eq!(r["semantic"]["regularity"][0]["status"], "measured");
    assert!(!compare(std::slice::from_ref(&r)).unwrap()["frontier"]
        .as_array()
        .unwrap()
        .is_empty());
    let mut malformed = r.clone();
    malformed["semantic"]["base_points"] = json!("invalid");
    malformed["semantic_sha256"] = json!(digest(&canonical(&malformed["semantic"]).unwrap()));
    assert!(compare(&[malformed]).is_err());
    let mut w = workload();
    w.regularity_variable_cap = 2;
    let missing = evaluate(&root, &id, &w).unwrap();
    assert_eq!(
        missing["semantic"]["regularity"][0]["status"],
        "censored_variable_cap"
    );
    assert!(compare(&[missing]).unwrap()["frontier"]
        .as_array()
        .unwrap()
        .is_empty());
    w = workload();
    w.split = "holdout".into();
    assert!(compare(&[r, evaluate(&root, &id, &w).unwrap()]).is_err());
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn s3_plan_publishes_manifest_last_and_rejects_bad_paths() {
    let root = temp();
    let id = build(&config(), &root).unwrap();
    let p = plan(&root, &id, "factor-bases/v1").unwrap();
    let objects = p["objects"].as_array().unwrap();
    assert!(objects.last().unwrap()["key"]
        .as_str()
        .unwrap()
        .ends_with(&format!("manifests/{id}.json")));
    assert_eq!(p["if_none_match"], "*");
    assert!(plan(&root, &id, "../escape").is_err());
    assert!(manifest_path(&root, "../../escape").is_err());
    std::fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn s3_transport_roundtrip_duplicates_and_checksum_rejection() {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use sha2::{Digest, Sha256};
    use std::os::unix::fs::PermissionsExt;
    let root = temp();
    let destination = temp();
    let id = build(&config(), &root).unwrap();
    let p = plan(&root, &id, "factor-bases/test").unwrap();
    let stub = root.join("aws-stub");
    let write_stub = |duplicate: bool, bad_checksum: bool| {
        let mut script=String::from("#!/bin/sh\nset -eu\nop=$2\nshift 2\nkey=''\nchecksum=''\ncondition=''\nlast=''\nwhile [ $# -gt 0 ]; do\n case \"$1\" in\n --key) key=$2; shift 2;;\n --checksum-sha256) checksum=$2; shift 2;;\n --if-none-match) condition=$2; shift 2;;\n --no-cli-pager) shift;;\n --*) shift 2;;\n *) last=$1; shift;;\n esac\ndone\ncase \"$key\" in\n");
        for obj in p["objects"].as_array().unwrap() {
            let path = obj["local_path"].as_str().unwrap();
            let data = std::fs::read(path).unwrap();
            let checksum = if bad_checksum {
                "bad".into()
            } else {
                STANDARD.encode(Sha256::digest(&data))
            };
            let meta = json!({"ContentLength":data.len(),"ChecksumSHA256":checksum,"ETag":"test-etag","VersionId":"test-version"});
            script.push_str(&format!(
                "'{}') source='{}'; meta='{}';;\n",
                obj["key"].as_str().unwrap(),
                path,
                meta
            ));
        }
        script.push_str(
            "*) exit 7;;\nesac\ncase \"$op\" in\nput-object) [ \"$condition\" = '*' ] || exit 8\n",
        );
        if duplicate {
            script.push_str("echo '(412) PreconditionFailed' >&2; exit 255;;\n");
        } else {
            script.push_str("printf '{\"ChecksumSHA256\":\"%s\",\"VersionId\":\"test-version\"}' \"$checksum\";;\n");
        }
        script.push_str("head-object) printf '%s' \"$meta\";;\nget-object) cp \"$source\" \"$last\"; printf '{}';;\n*) exit 9;;\nesac\n");
        std::fs::write(&stub, script).unwrap();
        std::fs::set_permissions(&stub, std::fs::Permissions::from_mode(0o700)).unwrap();
    };
    write_stub(false, false);
    let result = publish_with(&root, &id, "test-bucket", "factor-bases/test", &stub).unwrap();
    assert!(result["objects"]
        .as_array()
        .unwrap()
        .iter()
        .all(|o| o["status"] == "created"));
    write_stub(true, false);
    let result = publish_with(&root, &id, "test-bucket", "factor-bases/test", &stub).unwrap();
    assert!(result["objects"]
        .as_array()
        .unwrap()
        .iter()
        .all(|o| o["status"] == "already_present_checksum_verified"));
    fetch_with(&destination, &id, "test-bucket", "factor-bases/test", &stub).unwrap();
    assert_eq!(
        load(&root, &id).unwrap().1,
        load(&destination, &id).unwrap().1
    );
    write_stub(true, true);
    assert!(publish_with(&root, &id, "test-bucket", "factor-bases/test", &stub).is_err());
    std::fs::remove_dir_all(root).unwrap();
    std::fs::remove_dir_all(destination).unwrap();
}

#[test]
#[ignore]
fn write_public_fixtures() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
    std::fs::create_dir_all(&dir).unwrap();
    for (name, c) in ["binary-prime-degree83.json", "secp256k1.json"]
        .into_iter()
        .zip(full_width_configs())
    {
        std::fs::write(dir.join(name), serde_json::to_vec_pretty(&c).unwrap()).unwrap();
    }
    let c = config();
    std::fs::write(
        dir.join("binary.json"),
        serde_json::to_vec_pretty(&c).unwrap(),
    )
    .unwrap();
    let mut p = c.clone();
    p.curve = fixture(None);
    p.recipe = Recipe::PrimeProgression {
        start: "0".into(),
        step: "1".into(),
        count: 101,
    };
    std::fs::write(
        dir.join("prime.json"),
        serde_json::to_vec_pretty(&p).unwrap(),
    )
    .unwrap();
    std::fs::write(
        dir.join("workload.json"),
        serde_json::to_vec_pretty(&workload()).unwrap(),
    )
    .unwrap();
    let mut cs = vec![c.clone()];
    for shape in [Shape::RandomLinear, Shape::Geometric, Shape::TraceZero] {
        let mut x = c.clone();
        x.recipe = Recipe::BinarySubspace {
            shape,
            dimension: 3,
            seed: 7,
            offsets: vec!["0".into()],
        };
        cs.push(x);
    }
    cs.push(p);
    std::fs::write(
        dir.join("grid.json"),
        serde_json::to_vec_pretty(&cs).unwrap(),
    )
    .unwrap();
}
