//! Recompute only the source preflight. No target images or target lookups.
mod field;
mod sections;
mod small;
use field::{add, F};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};

const REFERENCE_SHA256: &str = "cfca312b80fe7a35da6cd8d31ee44b195885aad5fbd6b1de07b1fc9dfe33dc65";
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|v| format!("{v:02x}")).collect()
}
fn digest(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}
fn reference(bytes: &[u8]) -> Result<Value, String> {
    if digest(bytes) != REFERENCE_SHA256 {
        return Err("frozen reference SHA-256 mismatch".into());
    }
    serde_json::from_slice(bytes).map_err(|e| e.to_string())
}
fn pivots(mut rows: Vec<Vec<F>>) -> Vec<usize> {
    let mut columns = vec![];
    for j in 0..rows[0].len() {
        let rank = columns.len();
        let Some(i) = (rank..rows.len()).find(|i| rows[*i][j] != F::ZERO) else {
            continue;
        };
        rows.swap(i, rank);
        let inverse = rows[rank][j].inv();
        for v in &mut rows[rank] {
            *v = *v * inverse;
        }
        let pivot = rows[rank].clone();
        for (i, row) in rows.iter_mut().enumerate() {
            if i == rank {
                continue;
            }
            let scale = row[j];
            for k in j..row.len() {
                row[k] = row[k] - scale * pivot[k];
            }
        }
        columns.push(j);
        if columns.len() == rows.len() {
            break;
        }
    }
    columns
}
fn solve(mut rows: Vec<Vec<F>>, b: &[F]) -> Vec<F> {
    for (r, b) in rows.iter_mut().zip(b) {
        r.push(*b);
    }
    let n = rows.len();
    for j in 0..n {
        let i = (j..n)
            .find(|i| rows[*i][j] != F::ZERO)
            .expect("singular frame");
        rows.swap(i, j);
        let inv = rows[j][j].inv();
        for v in &mut rows[j] {
            *v = *v * inv;
        }
        let pivot = rows[j].clone();
        for (i, row) in rows.iter_mut().enumerate() {
            if i != j {
                let c = row[j];
                for k in j..=n {
                    row[k] = row[k] - c * pivot[k];
                }
            }
        }
    }
    rows.into_iter().map(|r| r[n]).collect()
}
fn vector_hex(v: &[F]) -> String {
    hex(&v.iter().flat_map(|v| v.0).collect::<Vec<_>>())
}
fn holdout(pilot: &[[u16; 2]]) -> (Value, u32) {
    let mut used: BTreeSet<u16> = pilot.iter().map(|p| p[0]).collect();
    let mut out = vec![];
    let mut counter = 0_u32;
    while out.len() < 32 && counter < 10000 {
        let raw = Sha256::digest(format!("ghs-calibration-holdout-v1:{counter}").as_bytes());
        counter += 1;
        let x = (u32::from_be_bytes(raw[..4].try_into().unwrap()) % 343) as u16;
        let rhs = small::add(small::add(small::pow(x, 3), small::mul(248, x)), 42);
        let mut y = small::pow(rhs, 86);
        if small::pow(y, 2) != rhs || y == 0 {
            continue;
        }
        if raw[4] & 1 != 0 {
            y = small::neg(y);
        }
        let point = [x, y];
        let Some(aux) = small::ec_add(Some(point), Some([0, 176])) else {
            continue;
        };
        if used.contains(&x) || used.contains(&aux[0]) || x == aux[0] {
            continue;
        }
        let case_id = digest(&serde_json::to_vec(&point).unwrap());
        out.push(json!({"case_id":case_id,"point":point,"point_plus_G":aux}));
        used.insert(x);
        used.insert(aux[0]);
    }
    assert_eq!(out.len(), 32, "holdout stream exhausted");
    (json!(out), counter)
}
fn run(expected: &Value) -> Value {
    field::verify_modulus();
    let z = field::cubic_roots([F::c(1), F::ZERO, F::c(1), F::c(1)])[0];
    assert_eq!(z.pow(343), z);
    let enc =
        |n: u16| F::c((n % 7) as u8) + F::c((n / 7 % 7) as u8) * z + F::c((n / 49) as u8) * z * z;
    let embedded: Vec<F> = (0..343).map(enc).collect();
    for (n, v) in embedded.iter().enumerate() {
        assert_eq!(v.pow(343), *v);
        assert_eq!(*v + enc(1), enc(small::add(n as u16, 1)));
        assert_eq!(*v * z, enc(small::mul(n as u16, 7)));
    }
    assert_eq!(
        json!(field::MODULUS),
        expected["working_field_modulus_ascending"]
    );
    assert_eq!(
        json!(sections::KERNEL),
        expected["source"]["kernel_basis_rows"]
    );
    assert_eq!(expected["source"]["theta_seed"], json!([0, 0, 0]));
    let w = sections::Weil::new(z);
    let weils = [w.clone(), w.conjugate(1), w.conjugate(2)];
    let generator = Some((F::ZERO, enc(176)));
    assert_eq!(enc(176).pow(2), w.b);
    let mut current = None;
    let mut small_current = None;
    let mut points = vec![];
    let mut sample = vec![];
    let mut sha = Sha256::new();
    for _ in 0..32 {
        current = add(w.a, current, generator);
        small_current = small::ec_add(small_current, Some([0, 176]));
        let p = current.unwrap();
        let q = small_current.unwrap();
        assert_eq!(
            p,
            (enc(q[0]), enc(q[1])),
            "independent small-field point mismatch"
        );
        assert_eq!(p.1.pow(2), p.0.pow(3) + w.a * p.0 + w.b);
        points.push(q);
        let mut values = sections::sections(&weils, p);
        let scale = values
            .iter()
            .find(|v| **v != F::ZERO)
            .expect("zero source vector")
            .inv();
        for v in &mut values {
            *v = *v * scale;
            sha.update(v.0);
        }
        sample.push(values);
    }
    for _ in 32..373 {
        current = add(w.a, current, generator);
    }
    assert_eq!(current, None, "source generator order");
    let section_sha = hex(&sha.finalize());
    assert_eq!(
        json!(section_sha),
        expected["pilot"]["normalized_full_section_matrix_sha256"]
    );
    let columns = pivots(sample.clone());
    assert_eq!(columns.len(), 8);
    assert_eq!(
        json!(columns),
        expected["pilot"]["selected_section_indices"]
    );
    let vectors: Vec<Vec<F>> = sample
        .iter()
        .map(|r| columns.iter().map(|i| r[*i]).collect())
        .collect();
    let mut basis: Vec<usize> = vec![];
    for (i, row) in vectors.iter().enumerate() {
        let mut test: Vec<_> = basis.iter().map(|j| vectors[*j].clone()).collect();
        test.push(row.clone());
        if pivots(test).len() > basis.len() {
            basis.push(i);
        }
        if basis.len() == 8 {
            break;
        }
    }
    let matrix: Vec<Vec<F>> = (0..8)
        .map(|i| basis.iter().map(|j| vectors[*j][i]).collect())
        .collect();
    let (ninth, weights) = (0..32)
        .filter(|i| !basis.contains(i))
        .find_map(|i| {
            let v = solve(matrix.clone(), &vectors[i]);
            v.iter().all(|v| *v != F::ZERO).then_some((i, v))
        })
        .expect("no nine-point projective frame");
    let mut frame = basis.clone();
    frame.push(ninth);
    for (i, expected_coordinate) in vectors[ninth].iter().enumerate() {
        assert_eq!(
            basis
                .iter()
                .enumerate()
                .fold(F::ZERO, |s, (j, k)| s + weights[j] * vectors[*k][i]),
            *expected_coordinate
        );
    }
    for omitted in 0..9 {
        assert_eq!(
            pivots(
                frame
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != omitted)
                    .map(|(_, i)| vectors[*i].clone())
                    .collect()
            )
            .len(),
            8
        );
    }
    let labels: Vec<_> = frame.iter().map(|i| i + 1).collect();
    let frame_points: Vec<_> = frame.iter().map(|i| points[*i]).collect();
    let frame_hex: Vec<_> = frame.iter().map(|i| vector_hex(&vectors[*i])).collect();
    assert_eq!(
        json!(labels),
        expected["training_frame"]["known_scalar_labels"]
    );
    assert_eq!(
        json!(frame_points),
        expected["training_frame"]["point_coordinates"]
    );
    assert_eq!(
        json!(frame_hex),
        expected["training_frame"]["eight_coordinate_vectors_hex"]
    );
    assert_eq!(
        json!(vector_hex(&weights)),
        expected["training_frame"]["ninth_frame_point_coordinates_in_first_eight_basis_hex"]
    );
    let (challenges, counters) = holdout(&points);
    assert_eq!(challenges, expected["holdout_coordinate_inputs"]);
    assert_eq!(
        json!(counters),
        expected["holdout_stream"]["counters_consumed"]
    );
    json!({
        "schema":"quotient-calibration-native-preflight/v1",
        "status":"source_preflight_pass_target_experiment_not_executed",
        "historical_source_commit":"b314cf87940cfb7199cd98261ba6d3910f275c90",
        "reference_manifest_sha256":REFERENCE_SHA256,
        "backend":"native Rust; no Python, FLINT or Sage process",
        "modulus_irreducibility_checked":true,
        "census_z_embedding_coefficients":z.0,
        "source_order_checked":373,
        "normal_weil_cocycle_checks":{"level_2":16,"level_3":81},
        "pilot_points":32,"section_columns":64,"exact_sample_rank":8,
        "normalized_full_section_matrix_sha256":section_sha,
        "selected_section_indices":columns,"training_frame_scalars":labels,
        "every_eight_of_nine_rank_eight":true,
        "frame_vectors_and_weights_match_frozen_manifest":true,
        "challenge_count":32,"holdout_counters_consumed":counters,
        "challenge_coordinates_match_frozen_manifest":true,
        "historical_input_report_pins":expected["input_sha256"],
        "input_report_hash_verification":"separate provenance receipt; input reports not read by evaluator",
        "scope":{"calibration_matrix_fitted":false,"target_divisors_evaluated":false,"inverse_experiment_executed":false,"holdout_scalar_labels_computed":false,"global_section_identity_proved":false},
        "cost_comparison":null
    })
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = None;
    let mut ref_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/calibration-preflight.json");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => output = Some(args.next().ok_or("missing output path")?),
            "--reference" => ref_path = args.next().ok_or("missing reference path")?.into(),
            _ => return Err(format!("unsupported argument: {arg}").into()),
        }
    }
    let expected = reference(&fs::read(ref_path)?).map_err(std::io::Error::other)?;
    let text = serde_json::to_string_pretty(&run(&expected))? + "\n";
    if let Some(path) = output {
        use std::io::Write;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        file.write_all(text.as_bytes())?;
    } else {
        print!("{text}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_source_replay_matches_frozen_manifest() {
        let bytes = include_bytes!("../../fixtures/calibration-preflight.json");
        let result = run(&reference(bytes).unwrap());
        assert_eq!(result["exact_sample_rank"], 8);
        assert_eq!(result["challenge_count"], 32);
    }
    #[test]
    fn altered_reference_is_rejected_before_evaluation() {
        let mut bytes = include_bytes!("../../fixtures/calibration-preflight.json").to_vec();
        bytes[0] ^= 1;
        assert!(reference(&bytes).is_err());
    }
    #[test]
    fn dependent_frame_is_rejected() {
        let row = vec![F::c(1); 8];
        assert_eq!(pivots(vec![row.clone(); 9]).len(), 1);
        let bad = std::panic::catch_unwind(|| solve(vec![row; 8], &[F::c(1); 8]));
        assert!(bad.is_err());
    }
    #[test]
    fn small_field_exhaustive_inverses_and_group_order() {
        for a in 1..343 {
            assert_eq!(small::mul(a, small::pow(a, 341)), 1);
        }
        let mut p = None;
        for i in 1..=373 {
            p = small::ec_add(p, Some([0, 176]));
            assert_eq!(p.is_none(), i == 373);
        }
    }
}
