//! Observed corpora for the explicitly reduced-round AES analysis models.
use crypto_lib::cryptanalysis::aes::{differential, linear, mitm, reduced::ReducedAes128};
use serde::Deserialize;
use serde_json::json;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Pair4 {
    p0_hex: String,
    p1_hex: String,
    c0_hex: String,
    c1_hex: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DifferentialInput {
    alpha: u8,
    pairs: Vec<Pair4>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Pair2 {
    plaintext_hex: String,
    ciphertext_hex: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LinearInput {
    pairs: Vec<Pair2>,
    #[serde(default)]
    verification_pairs: Vec<Pair2>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MitmInput {
    pairs: Vec<Pair2>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct HigherOrderInput {
    base_plaintext_hex: String,
    directions_hex: Vec<String>,
    pairs: Vec<Pair2>,
}

/// XOR the observed outputs of a complete affine plaintext cube.
pub fn higher_order(input: &Path) -> Result<(), String> {
    let data: HigherOrderInput =
        serde_json::from_slice(&read(input)?).map_err(|e| e.to_string())?;
    let report = analyze_higher_order(data)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn analyze_higher_order(data: HigherOrderInput) -> Result<serde_json::Value, String> {
    let dimension = data.directions_hex.len();
    if !(1..=16).contains(&dimension) {
        return Err("cube dimension must be 1..=16".into());
    }
    let expected_count = 1usize << dimension;
    if data.pairs.len() != expected_count {
        return Err(format!(
            "cube requires exactly {expected_count} plaintext/ciphertext pairs"
        ));
    }
    let base = block(&data.base_plaintext_hex)?;
    let directions: Vec<[u8; 16]> = data
        .directions_hex
        .iter()
        .map(|encoded| block(encoded))
        .collect::<Result<_, _>>()?;
    let mut basis = [0u128; 128];
    for direction in &directions {
        let mut vector = u128::from_le_bytes(*direction);
        for bit in (0..128).rev() {
            if (vector >> bit) & 1 == 0 {
                continue;
            }
            if basis[bit] != 0 {
                vector ^= basis[bit];
            } else {
                basis[bit] = vector;
                break;
            }
        }
        if vector == 0 {
            return Err("cube directions must be linearly independent".into());
        }
    }
    let mut observations = HashMap::with_capacity(expected_count);
    for pair in data.pairs {
        let plaintext = block(&pair.plaintext_hex)?;
        let ciphertext = block(&pair.ciphertext_hex)?;
        if observations.insert(plaintext, ciphertext).is_some() {
            return Err("duplicate plaintext in cube".into());
        }
    }
    let mut derivative = [0u8; 16];
    for mask in 0..expected_count {
        let mut plaintext = base;
        for (bit, direction) in directions.iter().enumerate() {
            if mask & (1 << bit) != 0 {
                for (byte, delta) in plaintext.iter_mut().zip(direction) {
                    *byte ^= delta;
                }
            }
        }
        let ciphertext = observations
            .remove(&plaintext)
            .ok_or_else(|| format!("missing cube plaintext {}", hex::encode(plaintext)))?;
        for (byte, observed) in derivative.iter_mut().zip(ciphertext) {
            *byte ^= observed;
        }
    }
    let balanced = derivative.iter().filter(|&&byte| byte == 0).count();
    Ok(json!({
        "schema_version":1,
        "method":"observed-higher-order-xor-derivative",
        "status":if balanced == 16 {"zero_derivative"} else {"nonzero_derivative"},
        "cube_dimension":dimension,
        "pairs_used":expected_count,
        "base_plaintext_hex":hex::encode(base),
        "directions_hex":directions.iter().map(hex::encode).collect::<Vec<_>>(),
        "xor_derivative_hex":hex::encode(derivative),
        "balanced_output_bytes":balanced
    }))
}

/// Compute the Square integral balance over a caller-supplied λ-set.
pub fn integral(input: &Path) -> Result<(), String> {
    let data: MitmInput = serde_json::from_slice(&read(input)?).map_err(|e| e.to_string())?;
    if data.pairs.len() != 256 {
        return Err("integral analysis requires exactly 256 plaintext/ciphertext pairs".into());
    }
    let mut seen = [false; 256];
    let mut fixed: Option<[u8; 15]> = None;
    let mut xor_sum = [0u8; 16];
    for (index, pair) in data.pairs.iter().enumerate() {
        let plaintext = block(&pair.plaintext_hex)?;
        let other: [u8; 15] = plaintext[1..].try_into().expect("fixed slice length");
        if fixed.is_some_and(|value| value != other) {
            return Err(format!("pair {index} changes a fixed plaintext byte"));
        }
        fixed = Some(other);
        let active = plaintext[0] as usize;
        if seen[active] {
            return Err(format!("duplicate lambda-set plaintext byte {active}"));
        }
        seen[active] = true;
        let ciphertext = block(&pair.ciphertext_hex)?;
        for byte in 0..16 {
            xor_sum[byte] ^= ciphertext[byte];
        }
    }
    let balanced = xor_sum.iter().filter(|&&byte| byte == 0).count();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema_version":1,
            "method":"square-integral-xor-balance",
            "hypothesis":"aes-128-reduced-three-full-rounds-with-final-mixcolumns",
            "status":if balanced == 16 {"balanced"} else {"not_balanced"},
            "pairs_used":256,
            "active_plaintext_byte_position":0,
            "fixed_other_plaintext_hex":hex::encode(fixed.expect("nonempty corpus")),
            "xor_sum_hex":hex::encode(xor_sum),
            "balanced_output_bytes":balanced
        }))
        .map_err(|e| e.to_string())?
    );
    Ok(())
}

/// Analyze a complete, canonically indexed chosen-plaintext delta set.
pub fn mitm(input: &Path) -> Result<(), String> {
    let data: MitmInput = serde_json::from_slice(&read(input)?).map_err(|e| e.to_string())?;
    if data.pairs.len() != 256 {
        return Err("MITM requires exactly 256 plaintext/ciphertext pairs".into());
    }
    let mut ciphertexts = [[0u8; 16]; 256];
    let mut seen = [false; 256];
    for (index, pair) in data.pairs.iter().enumerate() {
        let plaintext = block(&pair.plaintext_hex)?;
        if plaintext[1..].iter().any(|&byte| byte != 0) {
            return Err(format!(
                "pair {index} is outside the (i, 0, ..., 0) delta set"
            ));
        }
        let active = plaintext[0] as usize;
        if seen[active] {
            return Err(format!("duplicate delta-set plaintext byte {active}"));
        }
        seen[active] = true;
        ciphertexts[active] = block(&pair.ciphertext_hex)?;
    }
    let candidates = mitm::mitm_recover_two_round_observed(&ciphertexts)?;
    let status = match candidates.len() {
        0 => "no_candidate",
        1 => "unique_partial_candidate",
        _ => "ambiguous_partial_candidates",
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema_version":1,
            "model":"aes-128-reduced-two-round-no-final-mixcolumns",
            "method":"delta-set-two-key-byte-sequence-consistency",
            "status":status,
            "pairs_used":256,
            "candidates":candidates.iter().map(|(k0,k2,m)| json!({
                "round_0_key_byte_0_hex":format!("{k0:02x}"),
                "round_2_key_byte_0_hex":format!("{k2:02x}"),
                "intermediate_constant_hex":format!("{m:02x}")
            })).collect::<Vec<_>>()
        }))
        .map_err(|e| e.to_string())?
    );
    Ok(())
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

fn block(hex_text: &str) -> Result<[u8; 16], String> {
    let bytes = hex::decode(hex_text).map_err(|e| format!("invalid block hex: {e}"))?;
    bytes
        .try_into()
        .map_err(|_| "AES block must be exactly 16 bytes".into())
}

pub fn differential(input: &Path) -> Result<(), String> {
    let data: DifferentialInput =
        serde_json::from_slice(&read(input)?).map_err(|e| e.to_string())?;
    if data.alpha == 0 || !(2..=100_000).contains(&data.pairs.len()) {
        return Err("alpha must be nonzero and pair count must be 2..=100000".into());
    }
    let mut pairs = Vec::with_capacity(data.pairs.len());
    let mut seen_points = HashMap::new();
    let mut seen_pairs = HashSet::new();
    for (i, raw) in data.pairs.iter().enumerate() {
        let pair = differential::DiffPair {
            p0: block(&raw.p0_hex)?,
            p1: block(&raw.p1_hex)?,
            c0: block(&raw.c0_hex)?,
            c1: block(&raw.c1_hex)?,
        };
        let valid_plaintext =
            (0..16).all(|j| pair.p0[j] ^ pair.p1[j] == if j == 0 { data.alpha } else { 0 });
        let valid_ciphertext =
            (0..16).all(|j| (pair.c0[j] != pair.c1[j]) == [0, 7, 10, 13].contains(&j));
        if !valid_plaintext || !valid_ciphertext {
            return Err(format!(
                "pair {i} violates the two-round chosen-difference model"
            ));
        }
        for (plaintext, ciphertext) in [(pair.p0, pair.c0), (pair.p1, pair.c1)] {
            if seen_points
                .insert(plaintext, ciphertext)
                .is_some_and(|old| old != ciphertext)
            {
                return Err(format!("pair {i} contradicts an earlier ciphertext"));
            }
        }
        let ordered = if pair.p0 < pair.p1 {
            (pair.p0, pair.p1)
        } else {
            (pair.p1, pair.p0)
        };
        if !seen_pairs.insert(ordered) {
            return Err(format!("pair {i} repeats a chosen-plaintext pair"));
        }
        pairs.push(pair);
    }
    let ddt = differential::aes_sbox_ddt();
    let report =
        differential::key_recovery_two_round(data.alpha, &pairs, &ddt).map_err(str::to_owned)?;
    let bytes: Vec<_> = report
        .key_bytes
        .iter()
        .map(|(position, value)| json!({"position":position,"value_hex":format!("{value:02x}")}))
        .collect();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema_version":1,
            "model":"aes-128-reduced-two-round-no-final-mixcolumns",
            "status":"partial_candidate",
            "pairs_used":report.pairs_used,
            "trial_count":report.trial_count,
            "last_round_key_bytes":bytes
        }))
        .map_err(|e| e.to_string())?
    );
    Ok(())
}

fn parsed_pairs(
    raw: &[Pair2],
    seen: &mut HashMap<[u8; 16], [u8; 16]>,
) -> Result<Vec<([u8; 16], [u8; 16])>, String> {
    let mut out = Vec::with_capacity(raw.len());
    for pair in raw {
        let p = block(&pair.plaintext_hex)?;
        let c = block(&pair.ciphertext_hex)?;
        if let Some(previous) = seen.insert(p, c) {
            if previous != c {
                return Err("contradictory ciphertexts for one plaintext".into());
            }
        }
        out.push((p, c));
    }
    Ok(out)
}

pub fn linear(input: &Path) -> Result<(), String> {
    let data: LinearInput = serde_json::from_slice(&read(input)?).map_err(|e| e.to_string())?;
    if !(16..=100_000).contains(&data.pairs.len()) || data.verification_pairs.len() > 100_000 {
        return Err(
            "training pair count must be 16..=100000; verification pair count <=100000".into(),
        );
    }
    let mut seen = HashMap::new();
    let pairs = parsed_pairs(&data.pairs, &mut seen)?;
    if seen.len() != pairs.len() {
        return Err("training plaintexts must be distinct".into());
    }
    let training = seen.clone();
    let verify = parsed_pairs(&data.verification_pairs, &mut seen)?;
    if verify.iter().any(|(p, _)| training.contains_key(p)) {
        return Err("verification plaintext must be distinct from training plaintexts".into());
    }
    if seen.len() != pairs.len() + verify.len() {
        return Err("verification plaintexts must be distinct".into());
    }
    let lat = linear::aes_sbox_lat();
    let mut key = [0u8; 16];
    for c in 0..4 {
        for r in 0..4 {
            let recovered = linear::recover_key_byte_one_round(&pairs, (c, r), &lat)
                .ok_or_else(|| format!("ambiguous input-key byte at column {c}, row {r}"))?;
            key[4 * ((c + r) % 4) + r] = recovered;
        }
    }
    // The recovered bytes are the input whitening key. Independently replay
    // supplied holdout pairs before claiming a complete candidate.
    let cipher = ReducedAes128::new(&key, 1, false);
    let match_count = verify
        .iter()
        .filter(|(p, c)| cipher.encrypt(p) == *c)
        .count();
    let status = if verify.is_empty() {
        "candidate"
    } else if match_count == verify.len() {
        "verified"
    } else {
        "verification_failed"
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema_version":1,
            "model":"aes-128-reduced-one-round-no-final-mixcolumns",
            "method":"per-byte-exhaustive-consistency",
            "status":status,
            "candidate_key_hex":hex::encode(key),
            "training_pairs":pairs.len(),
            "verification_pairs":verify.len(),
            "verification_matches":match_count
        }))
        .map_err(|e| e.to_string())?
    );
    if status == "verification_failed" {
        return Err("candidate failed independent verification".into());
    }
    Ok(())
}

/// Matsui-style single-byte parity inference for the one-round cipher.
/// The unknown is `a·K0[input_byte] xor b·K1[output_byte]`.
pub fn linear_parity(
    input: &Path,
    column: usize,
    row: usize,
    input_mask: u8,
    output_mask: u8,
) -> Result<(), String> {
    if column > 3 || row > 3 || input_mask == 0 || output_mask == 0 {
        return Err("column and row must be in 0..=3; both masks must be nonzero".into());
    }
    let data: LinearInput = serde_json::from_slice(&read(input)?).map_err(|e| e.to_string())?;
    if !(64..=100_000).contains(&data.pairs.len()) || !data.verification_pairs.is_empty() {
        return Err(
            "linear parity requires 64..=100000 training pairs and no verification_pairs".into(),
        );
    }
    let mut seen = HashMap::new();
    let pairs = parsed_pairs(&data.pairs, &mut seen)?;
    if seen.len() != pairs.len() {
        return Err("linear parity requires distinct plaintext samples".into());
    }
    let lat = linear::aes_sbox_lat();
    let signed_count = lat[input_mask as usize][output_mask as usize];
    if signed_count == 0 {
        return Err("the selected AES S-box mask pair has zero LAT bias".into());
    }
    let input_byte_position = 4 * ((column + row) % 4) + row;
    let output_byte_position = 4 * column + row;
    let dot = |mask: u8, value: u8| (mask & value).count_ones() & 1;
    let zeros = pairs
        .iter()
        .filter(|(p, c)| {
            dot(input_mask, p[input_byte_position]) ^ dot(output_mask, c[output_byte_position]) == 0
        })
        .count();
    let (bit, margin) =
        linear::recover_key_bit_one_round(&pairs, (column, row), input_mask, output_mask, &lat);
    let tie = zeros * 2 == pairs.len();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema_version":1,
            "model":"aes-128-reduced-one-round-no-final-mixcolumns",
            "method":"sbox-lat-majority-vote",
            "status":if tie {"ambiguous"} else {"candidate"},
            "column":column,"row":row,
            "plaintext_byte_position":input_byte_position,
            "ciphertext_byte_position":output_byte_position,
            "input_mask":input_mask,"output_mask":output_mask,
            "lat_signed_count":signed_count,
            "theoretical_bias":signed_count as f64 / 256.0,
            "samples":pairs.len(),
            "equation_zero_count":zeros,
            "equation_one_count":pairs.len()-zeros,
            "observed_zero_rate":zeros as f64 / pairs.len() as f64,
            "majority_margin":margin,
            "effective_key_parity_bit":if tie {None} else {Some(bit)}
        }))
        .map_err(|e| e.to_string())?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn higher_order_observed_checks_complete_independent_cube() {
        let key = [0x51u8; 16];
        let cipher = ReducedAes128::new(&key, 2, false);
        let mut directions = [[0u8; 16]; 2];
        directions[0][0] = 1;
        directions[1][1] = 1;
        let base = [0x42u8; 16];
        let mut expected = [0u8; 16];
        let mut pairs = Vec::new();
        for mask in 0..4 {
            let mut plaintext = base;
            for (bit, direction) in directions.iter().enumerate() {
                if mask & (1 << bit) != 0 {
                    for (byte, delta) in plaintext.iter_mut().zip(direction) {
                        *byte ^= delta;
                    }
                }
            }
            let ciphertext = cipher.encrypt(&plaintext);
            for (byte, observed) in expected.iter_mut().zip(ciphertext) {
                *byte ^= observed;
            }
            pairs.push(json!({"plaintext_hex":hex::encode(plaintext),
                "ciphertext_hex":hex::encode(ciphertext)}));
        }
        pairs.reverse();
        let mut payload = json!({
            "base_plaintext_hex":hex::encode(base),
            "directions_hex":directions.iter().map(hex::encode).collect::<Vec<_>>(),
            "pairs":pairs
        });
        let parse = |value| serde_json::from_value::<HigherOrderInput>(value).unwrap();
        let report = analyze_higher_order(parse(payload.clone())).unwrap();
        assert_eq!(report["xor_derivative_hex"], hex::encode(expected));
        assert_eq!(report["pairs_used"], 4);

        payload["directions_hex"][1] = payload["directions_hex"][0].clone();
        assert!(analyze_higher_order(parse(payload.clone()))
            .unwrap_err()
            .contains("linearly independent"));
        payload["directions_hex"][1] = json!(hex::encode(directions[1]));
        payload["pairs"][1]["plaintext_hex"] = payload["pairs"][0]["plaintext_hex"].clone();
        assert!(analyze_higher_order(parse(payload.clone()))
            .unwrap_err()
            .contains("duplicate"));
    }

    #[test]
    fn observed_integral_checks_lambda_set_and_balance() {
        let key = [0x51u8; 16];
        let path = std::env::temp_dir().join(format!(
            "aes-integral-observed-{}-{}.json",
            std::process::id(),
            line!()
        ));
        let cipher = ReducedAes128::new(&key, 3, true);
        let mut xor = [0u8; 16];
        let mut pairs: Vec<_> = (0..=255u8)
            .map(|active| {
                let mut plaintext = [0x42u8; 16];
                plaintext[0] = active;
                let ciphertext = cipher.encrypt(&plaintext);
                for byte in 0..16 {
                    xor[byte] ^= ciphertext[byte];
                }
                json!({"plaintext_hex":hex::encode(plaintext),
                    "ciphertext_hex":hex::encode(ciphertext)})
            })
            .collect();
        assert_eq!(xor, [0u8; 16]);
        pairs.reverse();
        fs::write(&path, json!({"pairs":pairs}).to_string()).unwrap();
        assert!(integral(&path).is_ok());
        pairs[1]["plaintext_hex"] = pairs[0]["plaintext_hex"].clone();
        fs::write(&path, json!({"pairs":pairs}).to_string()).unwrap();
        assert!(integral(&path).unwrap_err().contains("duplicate"));
        pairs[1]["plaintext_hex"] = json!(hex::encode({
            let mut p = [0x42u8; 16];
            p[0] = 254;
            p[1] = 0x43;
            p
        }));
        fs::write(&path, json!({"pairs":pairs}).to_string()).unwrap();
        assert!(integral(&path).unwrap_err().contains("fixed"));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn observed_mitm_checks_complete_delta_set() {
        let key = [0x51u8; 16];
        let cipher = ReducedAes128::new(&key, 2, false);
        let path = std::env::temp_dir().join(format!(
            "aes-mitm-observed-{}-{}.json",
            std::process::id(),
            line!()
        ));
        let mut pairs: Vec<_> = (0..=255u8)
            .map(|active| {
                let mut plaintext = [0u8; 16];
                plaintext[0] = active;
                json!({"plaintext_hex":hex::encode(plaintext),
                    "ciphertext_hex":hex::encode(cipher.encrypt(&plaintext))})
            })
            .collect();
        pairs.reverse();
        fs::write(&path, json!({"pairs":pairs}).to_string()).unwrap();
        assert!(mitm(&path).is_ok());
        pairs[1]["plaintext_hex"] = pairs[0]["plaintext_hex"].clone();
        fs::write(&path, json!({"pairs":pairs}).to_string()).unwrap();
        assert!(mitm(&path).unwrap_err().contains("duplicate"));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn observed_corpora_and_independent_replay() {
        let key: [u8; 16] = std::array::from_fn(|i| i as u8);
        let path = std::env::temp_dir().join(format!(
            "aes-observed-{}-{}.json",
            std::process::id(),
            line!()
        ));
        let two = ReducedAes128::new(&key, 2, false);
        let pairs = differential::build_pairs(&two, 0x73, 64, 7);
        let payload = json!({"alpha":115,"pairs":pairs.iter().map(|p| json!({
            "p0_hex":hex::encode(p.p0),"p1_hex":hex::encode(p.p1),
            "c0_hex":hex::encode(p.c0),"c1_hex":hex::encode(p.c1)
        })).collect::<Vec<_>>()});
        fs::write(&path, payload.to_string()).unwrap();
        assert!(differential(&path).is_ok());
        let mut repeated = payload.clone();
        repeated["pairs"][1] = repeated["pairs"][0].clone();
        fs::write(&path, repeated.to_string()).unwrap();
        assert!(differential(&path)
            .unwrap_err()
            .contains("repeats a chosen-plaintext pair"));

        let one = ReducedAes128::new(&key, 1, false);
        let corpus = linear::known_plaintext_corpus(&one, 129, 7);
        let as_json = |(p, c): &([u8; 16], [u8; 16])| json!({"plaintext_hex":hex::encode(p),"ciphertext_hex":hex::encode(c)});
        let payload = json!({"pairs":corpus[..128].iter().map(as_json).collect::<Vec<_>>(),
            "verification_pairs":[as_json(&corpus[128])]});
        fs::write(&path, payload.to_string()).unwrap();
        assert!(linear(&path).is_ok());
        let mut repeated = payload.clone();
        repeated["pairs"][1] = repeated["pairs"][0].clone();
        fs::write(&path, repeated.to_string()).unwrap();
        assert!(linear(&path)
            .unwrap_err()
            .contains("training plaintexts must be distinct"));
        let mut mismatch = payload.clone();
        mismatch["verification_pairs"][0]["ciphertext_hex"] =
            json!("00000000000000000000000000000000");
        fs::write(&path, mismatch.to_string()).unwrap();
        assert!(linear(&path)
            .unwrap_err()
            .contains("failed independent verification"));
        let mut invalid = payload;
        invalid["verification_pairs"][0]["plaintext_hex"] =
            invalid["pairs"][0]["plaintext_hex"].clone();
        invalid["verification_pairs"][0]["ciphertext_hex"] =
            invalid["pairs"][0]["ciphertext_hex"].clone();
        fs::write(&path, invalid.to_string()).unwrap();
        assert!(linear(&path).unwrap_err().contains("distinct"));

        let samples = linear::known_plaintext_corpus(&one, 4096, 19);
        let lat = linear::aes_sbox_lat();
        let (a, b, _) = linear::max_linear_bias(&lat);
        let inferred = linear::recover_key_bit_one_round(&samples, (0, 0), a, b, &lat).0;
        let parity = |mask: u8, value: u8| ((mask & value).count_ones() & 1) as u8;
        let expected = parity(a, key[0]) ^ parity(b, one.round_key(1)[0][0]);
        assert_eq!(inferred, expected);
        let payload = json!({"pairs":samples.iter().map(as_json).collect::<Vec<_>>()});
        fs::write(&path, payload.to_string()).unwrap();
        assert!(linear_parity(&path, 0, 0, a, b).is_ok());
        assert!(linear_parity(&path, 0, 0, 0, b).is_err());
        fs::remove_file(path).unwrap();
    }
}
