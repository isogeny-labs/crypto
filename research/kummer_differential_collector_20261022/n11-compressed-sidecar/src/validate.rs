use kummer_n11_sidecar::{SCHEMA, read_json, root, sha256, u64_field};
use serde_json::json;
use std::collections::HashSet;
use std::fs;

fn independently_prime(n: u64) -> bool {
    if n < 2 || (n != 2 && n.is_multiple_of(2)) {
        return false;
    }
    let mut divisor = 3;
    while divisor <= n / divisor {
        if n.is_multiple_of(divisor) {
            return false;
        }
        divisor += 2;
    }
    true
}

fn independently_coprime(a: u64, b: u64) -> bool {
    let (mut x, mut y) = (a, b);
    while y > 0 {
        let remainder = x % y;
        x = y;
        y = remainder;
    }
    x == 1
}

fn k(x: u64, r: u64) -> u64 {
    x.min((r - x) % r)
}

fn pair(x: u64, g: u64, r: u64) -> (u64, u64) {
    (k(x % r, r), k((x + g) % r, r))
}

fn small_group_checks() {
    for r in [3_u64, 5, 7, 11, 13, 17, 19, 23, 29, 31] {
        for g in 1..r {
            let images: HashSet<_> = (0..r).map(|x| pair(x, g, r)).collect();
            assert_eq!(
                images.len() as u64,
                r,
                "pair sign collision at r={r}, g={g}"
            );
            for t in 0..r {
                for word in 0..256_u64 {
                    let (mut a, mut d, mut two_power) = (0, t, 1);
                    for n in 0..8 {
                        let c = (word >> (7 - n)) & 1;
                        a = (2 * a + c) % r;
                        d = (2 * d + c * g) % r;
                        two_power = (2 * two_power) % r;
                        assert_eq!(
                            d,
                            (a * g + two_power * t) % r,
                            "state invariant at r={r}, g={g}, t={t}, word={word}, n={n}"
                        );
                    }
                }
            }
        }
    }
    assert_eq!(pair(1, 2, 4), pair(3, 2, 4), "2G=0 counterexample");
}

fn main() {
    let here = root();
    let input = here.join("../orbit-proof/frozen-inputs.json");
    let source = here.join("../src/walk_impl.rs");
    let frozen = read_json(&input);
    let packet = read_json(&here.join("results/bounds.json"));
    assert_eq!(packet["schema"], SCHEMA);
    assert_eq!(packet["input_sha256"], sha256(&input));
    assert_eq!(packet["walk_source_sha256"], sha256(&source));
    assert!(packet["coordinate_xadd_exception_bound"].is_null());
    assert_eq!(packet["cohort_eligible"], false);
    assert_eq!(packet["row_format"], "array-in-columns-order");
    let models = frozen["models"].as_array().expect("models");
    let rows = packet["rows"].as_array().expect("rows");
    assert_eq!(models.len(), 21);
    assert_eq!(rows.len(), 336);
    let csv_text = fs::read_to_string(here.join("results/bounds.csv")).expect("CSV");
    let mut lines = csv_text.lines();
    let header = lines.next().expect("CSV header");
    let columns: Vec<_> = header.split(',').collect();
    assert_eq!(columns.len(), 19);
    assert_eq!(packet["columns"], json!(columns));
    let mut count = 0;
    for (i, line) in lines.enumerate() {
        let cells: Vec<_> = line.split(',').collect();
        assert_eq!(cells.len(), 19, "CSV columns row {i}");
        let model = &models[i / 16];
        let jump = &model["jumps"][i % 16];
        let row = &rows[i];
        let bits = u64_field(model, "bits");
        let p = u64_field(model, "p");
        let r = u64_field(model, "r");
        let jump_scalar = u64_field(jump, "jump_scalar");
        let jump_index = u64_field(jump, "jump_index");
        assert!(independently_prime(r) && r > 2 && independently_coprime(jump_scalar, r));
        let expected = [
            SCHEMA.to_owned(),
            bits.to_string(),
            p.to_string(),
            r.to_string(),
            jump_index.to_string(),
            jump_scalar.to_string(),
            r.to_string(),
            (32 * r).to_string(),
            "2".into(),
            "64".into(),
            "72".into(),
            "80".into(),
            (64 - (r - 1).leading_zeros()).to_string(),
            "3".into(),
            "72".into(),
            "108".into(),
            "0".into(),
            "".into(),
            "False".into(),
        ];
        for (actual, expected) in cells.iter().zip(expected.iter()) {
            assert_eq!(actual, expected, "CSV mismatch row {i}");
        }
        let expected_json = json!([
            SCHEMA,
            bits,
            p,
            r,
            jump_index,
            jump_scalar,
            r,
            32 * r,
            2,
            64,
            72,
            80,
            64 - (r - 1).leading_zeros(),
            3,
            72,
            108,
            0,
            null,
            false
        ]);
        assert_eq!(row, &expected_json, "JSON mismatch row {i}");
        count += 1;
    }
    assert_eq!(count, 336);
    small_group_checks();
    let sums = fs::read_to_string(here.join("SHA256SUMS")).expect("checksums");
    assert_eq!(sums.lines().count(), 14);
    for line in sums.lines() {
        let (expected, name) = line.split_once("  ").expect("checksum line");
        assert_eq!(sha256(&here.join(name)), expected, "checksum {name}");
    }
    println!(
        "{}",
        json!({
            "schema": "kummer-compressed-sidecar-validation/v2",
            "implementation": "Rust",
            "rows": count,
            "model_count": models.len(),
            "small_group_checks": "pass",
            "source_digest": sha256(&input),
            "checksums": "pass",
            "cohort_eligible": false
        })
    );
}
