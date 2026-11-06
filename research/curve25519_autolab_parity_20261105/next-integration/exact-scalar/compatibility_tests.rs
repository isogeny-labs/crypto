#[path = "exact_scalar.rs"]
mod exact_scalar;
#[path = "legacy_compat.rs"]
mod legacy;
use exact_scalar::ExactScalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Old {
    #[serde(with = "legacy::compat_u128")]
    value: u128,
    #[serde(with = "legacy::compat_u128::option")]
    optional: Option<u128>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct New {
    value: ExactScalar,
    optional: Option<ExactScalar>,
}
#[derive(Debug, Serialize, Deserialize)]
struct LegacyScalar(#[serde(with = "legacy::compat_u128")] u128);

#[test]
fn legacy_wire_bytes_match_for_narrow_and_wide_boundaries() {
    for value in [
        0u128,
        1,
        u64::MAX as u128,
        1u128 << 64,
        (1u128 << 127) - 1,
        1u128 << 127,
        u128::MAX,
    ] {
        for optional in [None, Some(value)] {
            let old = Old { value, optional };
            let new = New {
                value: value.into(),
                optional: optional.map(ExactScalar::from),
            };
            let old_bytes = serde_json::to_vec(&old).unwrap();
            let new_bytes = serde_json::to_vec(&new).unwrap();
            assert_eq!(old_bytes, new_bytes);
            assert_eq!(serde_json::from_slice::<New>(&old_bytes).unwrap(), new);
            assert_eq!(serde_json::from_slice::<Old>(&new_bytes).unwrap(), old);
        }
    }
}
#[test]
fn legacy_accepted_and_rejected_input_contract_matches() {
    for text in [
        "0",
        "1",
        "\"+1\"",
        "\"0001\"",
        "\"+0000\"",
        "\"-0\"",
        "-1",
        "1e3",
        "1.5",
        "null",
        "true",
        "[]",
        "{}",
        "\" 1\"",
        "\"1 \"",
        "\"1_0\"",
        "\"0x10\"",
        "18446744073709551616",
        "\"340282366920938463463374607431768211455\"",
    ] {
        let old = serde_json::from_str::<LegacyScalar>(text);
        let new = serde_json::from_str::<ExactScalar>(text);
        assert_eq!(old.is_ok(), new.is_ok(), "{text}");
        if let (Ok(old), Ok(new)) = (old, new) {
            assert_eq!(Some(old.0), new.checked_u128());
        }
    }
}
#[test]
fn retained_workload_orders_preserve_their_wire_bytes() {
    let root = std::path::PathBuf::from(
        std::env::var("CRYPTO_TEST_ROOT").expect("set repository fixture root"),
    );
    let paths = [
        "tests/fixtures/ecbench/work-v1/records.jsonl",
        "research/ecbench_all_candidates_20261003/sessions/prime/records.jsonl",
        "research/ecbench_all_candidates_20261003/sessions/koblitz/records.jsonl",
        "research/ic_planner_validation_20261020/sessions/r24/records.jsonl",
    ];
    let mut total = 0usize;
    let mut fields = 0usize;
    let mut above_u64 = 0usize;
    for path in paths {
        let text = std::fs::read_to_string(root.join(path)).unwrap();
        let mut rows = 0;
        for line in text.lines().filter(|s| !s.trim().is_empty()) {
            let record: serde_json::Value = serde_json::from_str(line).unwrap();
            let curve = &record["workload"]["curve"];
            for key in ["r", "group_order", "cofactor"] {
                let raw = curve.get(key).expect("required order metadata").clone();
                let old: LegacyScalar = serde_json::from_value(raw.clone()).unwrap();
                let new: ExactScalar = serde_json::from_value(raw.clone()).unwrap();
                assert_eq!(new.checked_u128(), Some(old.0), "{path}: {key}");
                assert_eq!(serde_json::to_value(new).unwrap(), raw, "{path}: {key}");
                assert_eq!(
                    serde_json::to_vec(&new).unwrap(),
                    serde_json::to_vec(&old).unwrap()
                );
                fields += 1;
                above_u64 += usize::from(new.checked_u64().is_none());
            }
            rows += 1;
        }
        assert!(rows > 0);
        total += rows;
        println!("fixture={path} records={rows}");
    }
    println!("records={total} order_fields={fields} fields_above_u64={above_u64}");
}
