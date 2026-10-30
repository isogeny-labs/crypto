//! Differential measurements on caller-supplied hash message pairs.
use serde::Deserialize;
use serde_json::json;
use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::Path,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Pair {
    m0_hex: String,
    m1_hex: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    input_difference_hex: String,
    pairs: Vec<Pair>,
}

fn read(input: &Path) -> Result<Vec<u8>, String> {
    let resolved = crypto_lib::store::resolve_path(input).map_err(|e| e.to_string())?;
    if fs::metadata(&resolved).map_err(|e| e.to_string())?.len() > 64 * 1024 * 1024 {
        return Err("input exceeds 64 MiB".into());
    }
    let data = fs::read(resolved).map_err(|e| e.to_string())?;
    if data.len() > 64 * 1024 * 1024 {
        return Err("input exceeds 64 MiB".into());
    }
    Ok(data)
}

fn analyze(
    data: Input,
    algorithm: &str,
    output_bytes: Option<usize>,
) -> Result<serde_json::Value, String> {
    if !(1..=10_000).contains(&data.pairs.len()) {
        return Err("hash differential pair count must be 1..=10000".into());
    }
    if output_bytes.is_some_and(|n| !(1..=128).contains(&n)) {
        return Err("observed hash output length must be 1..=128 bytes".into());
    }
    let difference = hex::decode(&data.input_difference_hex)
        .map_err(|e| format!("invalid input difference hex: {e}"))?;
    if difference.is_empty() || difference.len() > 4096 || difference.iter().all(|&b| b == 0) {
        return Err("input difference must be nonzero and 1..=4096 bytes".into());
    }
    let mut seen = HashSet::with_capacity(data.pairs.len() * 2);
    let mut histogram = BTreeMap::<usize, usize>::new();
    let mut distance_sum = 0usize;
    let mut digest_bytes = None;
    for (index, pair) in data.pairs.iter().enumerate() {
        let m0 = hex::decode(&pair.m0_hex).map_err(|e| format!("pair {index} m0 hex: {e}"))?;
        let m1 = hex::decode(&pair.m1_hex).map_err(|e| format!("pair {index} m1 hex: {e}"))?;
        if m0.len() != difference.len()
            || m1.len() != difference.len()
            || m0
                .iter()
                .zip(&m1)
                .zip(&difference)
                .any(|((&a, &b), &d)| a ^ b != d)
        {
            return Err(format!("pair {index} has the wrong input difference"));
        }
        if !seen.insert(m0.clone()) || !seen.insert(m1.clone()) {
            return Err(format!("pair {index} repeats a message"));
        }
        let h0 = hex::decode(super::cmd_hash(algorithm, &m0, output_bytes)?)
            .map_err(|e| e.to_string())?;
        let h1 = hex::decode(super::cmd_hash(algorithm, &m1, output_bytes)?)
            .map_err(|e| e.to_string())?;
        if h0.is_empty() || h0.len() > 128 || h0.len() != h1.len() {
            return Err("observed hash digest length must be 1..=128 bytes".into());
        }
        digest_bytes = Some(h0.len());
        let distance: usize = h0
            .iter()
            .zip(&h1)
            .map(|(&a, &b)| (a ^ b).count_ones() as usize)
            .sum();
        *histogram.entry(distance).or_default() += 1;
        distance_sum += distance;
    }
    let minimum = *histogram.keys().next().expect("nonempty corpus");
    let maximum = *histogram.keys().next_back().expect("nonempty corpus");
    Ok(json!({
        "schema_version": 1,
        "method": "observed-hash-xor-differential",
        "status": "measured",
        "algorithm": algorithm,
        "output_bytes": digest_bytes.expect("nonempty corpus"),
        "input_difference_hex": hex::encode(difference),
        "pairs_used": data.pairs.len(),
        "distinct_messages": seen.len(),
        "equal_digest_pairs": histogram.get(&0).copied().unwrap_or(0),
        "minimum_output_bit_distance": minimum,
        "maximum_output_bit_distance": maximum,
        "mean_output_bit_distance": distance_sum as f64 / data.pairs.len() as f64,
        "output_bit_distance_histogram": histogram
    }))
}

pub fn run(input: &Path, algorithm: &str, output_bytes: Option<usize>) -> Result<(), String> {
    let data: Input = serde_json::from_slice(&read(input)?).map_err(|e| e.to_string())?;
    let report = analyze(data, algorithm, output_bytes)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measures_sha2_and_sha3_and_checks_corpus() {
        let value = json!({"input_difference_hex":"01","pairs":[
            {"m0_hex":"00","m1_hex":"01"},
            {"m0_hex":"02","m1_hex":"03"}
        ]});
        let parse = |v| serde_json::from_value::<Input>(v).unwrap();
        for algorithm in ["sha256", "sha3-256", "shake128"] {
            let length = (algorithm == "shake128").then_some(32);
            let report = analyze(parse(value.clone()), algorithm, length).unwrap();
            assert_eq!(report["pairs_used"], 2);
            assert_eq!(report["distinct_messages"], 4);
            assert_eq!(report["output_bytes"], 32);
            assert_eq!(report["equal_digest_pairs"], 0);
        }
        let mut duplicate = value.clone();
        duplicate["pairs"][1] = duplicate["pairs"][0].clone();
        assert!(analyze(parse(duplicate), "sha256", None)
            .unwrap_err()
            .contains("repeats a message"));
        let mut wrong = value;
        wrong["pairs"][1]["m1_hex"] = json!("04");
        assert!(analyze(parse(wrong), "sha3-256", None)
            .unwrap_err()
            .contains("wrong input difference"));
    }
}
