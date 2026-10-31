//! Cap-boundary and modeled-allocation tests. Descriptor-only cases do not
//! claim that a maximum-sized artifact was transferred or fit the full ledger.
use super::{budget::*, schema::ObjectRef, source::AdmittedRef, test_support::*};
use serde_json::json;

#[test]
fn anchor_and_metadata_raw_caps_accept_exact_and_refuse_plus_one() {
    for (stage, cap, label) in [
        (Stage::Anchor, ANCHOR_MAX, "anchor"),
        (Stage::Manifest, METADATA_MAX, "metadata"),
    ] {
        for excess in [false, true] {
            let p = Fixture::new(false).publish_raw(|_, s, raw| {
                if s == stage {
                    raw.resize(cap as usize + usize::from(excess), b' ')
                }
            });
            if excess {
                let error = format!("{:#}", p.restore().unwrap_err());
                assert!(
                    error.contains(if stage == Stage::Anchor {
                        "partial anchor"
                    } else {
                        "reference digest/size"
                    }),
                    "{error}"
                );
                if stage == Stage::Anchor {
                    assert!(!p.out.exists());
                }
                p.retain(
                    &format!("R01-{label}-plus-one"),
                    "declared cap rejects before dependent allocation or transfer",
                    &json!({"error":error}),
                );
            } else {
                let report = p.restore().unwrap();
                p.retain(
                    &format!("R01-{label}-exact"),
                    "exact raw cap admitted within cumulative model",
                    &report,
                );
            }
        }
    }
}
#[test]
fn exact_event_line_limit_includes_lf_and_excess_is_semantic() {
    let mut sizing = Fixture::new(false);
    sizing.capture(
        0,
        vec![json!({"kind":"checkpoint","data":""})],
        vec![],
        None,
    );
    let overhead = sizing.blobs["events/000000000000.jsonl"].len();
    for excess in [false, true] {
        let mut f = Fixture::new(false);
        let n = LINE_MAX as usize - overhead + usize::from(excess);
        f.capture(
            0,
            vec![json!({"kind":"checkpoint","data":"x".repeat(n)})],
            vec![],
            None,
        );
        assert_eq!(
            f.blobs["events/000000000000.jsonl"].len(),
            LINE_MAX as usize + usize::from(excess)
        );
        let p = f.publish();
        if excess {
            let error = format!("{:#}", p.restore().unwrap_err());
            assert!(error.contains("committed event exceeds line limit"));
            p.retain(
                "R01-line-plus-one",
                "line cap includes LF",
                &json!({"error":error}),
            );
        } else {
            let report = p.restore().unwrap();
            p.retain("R01-line-exact", "exact inclusive-LF line cap", &report);
        }
    }
}
#[test]
fn reference_caps_admit_without_materializing_and_refuse_before_counting() {
    for cap in [METADATA_MAX, ARTIFACT_MAX] {
        let mut b = Budget::new();
        let mut r = ObjectRef {
            key: format!("sha256/aa/{}", "a".repeat(64)),
            sha256: "a".repeat(64),
            bytes: cap,
        };
        assert!(AdmittedRef::new(&r, cap, true, &mut b).is_ok());
        assert_eq!(b.logical_objects, 1);
        assert_eq!(b.logical_bytes, cap);
        r.bytes = cap + 1;
        assert!(AdmittedRef::new(&r, cap, true, &mut b).is_err());
        assert_eq!(b.logical_objects, 1);
        assert_eq!(b.logical_bytes, cap);
    }
}
#[test]
fn cumulative_repeated_cas_references_receive_no_discount() {
    let r = ObjectRef {
        key: format!("sha256/aa/{}", "a".repeat(64)),
        sha256: "a".repeat(64),
        bytes: ARTIFACT_MAX,
    };
    let mut b = Budget::new();
    for _ in 0..LOGICAL_BYTES_MAX / ARTIFACT_MAX {
        AdmittedRef::new(&r, ARTIFACT_MAX, true, &mut b).unwrap();
    }
    assert_eq!(b.logical_bytes, LOGICAL_BYTES_MAX);
    assert!(AdmittedRef::new(&r, ARTIFACT_MAX, true, &mut b).is_err());
    let mut b = Budget::new();
    let r = ObjectRef { bytes: 0, ..r };
    for _ in 0..OBJECTS_MAX {
        AdmittedRef::new(&r, ARTIFACT_MAX, true, &mut b).unwrap();
    }
    assert!(AdmittedRef::new(&r, ARTIFACT_MAX, true, &mut b).is_err());
    assert_eq!(b.logical_objects, OBJECTS_MAX);
}
#[test]
fn json_admission_refuses_before_owned_deserialization() {
    use serde::Deserialize;
    static OWNED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    struct Probe;
    impl<'de> Deserialize<'de> for Probe {
        fn deserialize<D: serde::Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
            OWNED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(Self)
        }
    }
    let raw = format!("[{}0]", "0,".repeat(1000));
    let mut b = Budget::with_limit(4096);
    assert!(super::io::typed::<Probe>(raw.as_bytes(), &mut b).is_err());
    assert_eq!(OWNED.load(std::sync::atomic::Ordering::SeqCst), 0);
    let raw = format!("{{\"{}\":0}}", "x".repeat(10000));
    assert!(super::io::typed::<Probe>(raw.as_bytes(), &mut Budget::with_limit(4096)).is_err());
    assert_eq!(OWNED.load(std::sync::atomic::Ordering::SeqCst), 0);
}
#[test]
fn geometric_growth_charges_all_new_capacity_before_allocation() {
    let mut b = Budget::with_limit(23);
    let mut v = Vec::<u64>::new();
    b.reserve_vec(&mut v, 1, "first").unwrap();
    v.push(1);
    assert_eq!(b.used(), 8);
    let old = v.capacity();
    assert!(b.reserve_vec(&mut v, 1, "replacement").is_err());
    assert_eq!(v.capacity(), old);
    assert_eq!(b.used(), 8);
    let mut b = Budget::with_limit(24);
    let mut v = Vec::<u64>::new();
    b.reserve_vec(&mut v, 1, "first").unwrap();
    v.push(1);
    b.reserve_vec(&mut v, 1, "second").unwrap();
    assert_eq!(b.used(), 24);
}
#[test]
fn report_bound_fails_before_producing_success_bytes() {
    assert!(super::report::serialized(&json!({"x":"x".repeat(REPORT_MAX)})).is_err());
    let raw = super::report::serialized(&json!({"x":"x".repeat(REPORT_MAX-14)})).unwrap();
    assert!(raw.len() <= REPORT_MAX);
}

#[test]
fn finite_context_allowlist_and_ordinal_ceiling_are_explicit() {
    let mut f = Fixture::new(true);
    for phase in [
        "curve-construction",
        "known-answer-replay",
        "factor-base-replay",
        "solver",
    ] {
        f.add_saved_phase(0, &format!("telemetry/{phase}-start.json"));
        f.add_saved_phase(0, &format!("telemetry/{phase}.json"));
    }
    for name in [
        "host.json",
        "public.json",
        "witness.json",
        "factor-base.json",
        "worker-process.json",
    ] {
        f.add_context(0, name, b"{}".to_vec());
    }
    let p = f.publish();
    let report = p.restore().unwrap();
    assert_eq!(report["context_count"], 19);
    p.retain(
        "R01-context-identities",
        "all19 allowed checked context identities",
        &report,
    );
    let mut anchor: super::schema::Anchor = serde_json::from_value(p.anchors[0].clone()).unwrap();
    anchor.ordinal = CHAIN_MAX - 1;
    anchor.catalog.key = format!(
        "ml-dataset/interrupted-attempts/{}/{}/snapshots/{:020}.json",
        anchor.run_sha256, anchor.attempt_sha256, anchor.ordinal
    );
    assert!(super::chain::validate_anchor(&anchor, &mut Budget::new()).is_ok());
    anchor.ordinal = CHAIN_MAX;
    assert!(super::chain::validate_anchor(&anchor, &mut Budget::new()).is_err());
}
