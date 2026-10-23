//! Observed AES corpora for reduced-round recovery and general block statistics.
use crypto_lib::cryptanalysis::aes::{differential, linear, mitm, reduced::ReducedAes128};
use serde::Deserialize;
use serde_json::{json, Value};
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
struct DifferentialStatisticsInput {
    input_difference_hex: String,
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

const CRAX_CIPHER_MODEL: &str = "aes128-reduced-final-no-mixcolumns/v1";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CraxCorpus<T> {
    schema_version: u32,
    cipher_model: String,
    technique: String,
    rounds: usize,
    pairs: Vec<T>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CraxDifferentialPair {
    p0: String,
    p1: String,
    c0: String,
    c1: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CraxLinearPair {
    plaintext: String,
    ciphertext: String,
}

fn versioned_corpus(bytes: &[u8]) -> Result<bool, String> {
    let value: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    Ok(value.get("schema_version").is_some())
}

fn check_crax_corpus<T>(
    corpus: &CraxCorpus<T>,
    technique: &str,
    rounds: usize,
) -> Result<(), String> {
    if corpus.schema_version != 1
        || corpus.cipher_model != CRAX_CIPHER_MODEL
        || corpus.technique != technique
        || corpus.rounds != rounds
    {
        return Err("AES corpus schema, model, technique, or round count differs".into());
    }
    Ok(())
}

fn parse_differential_input(bytes: &[u8]) -> Result<(DifferentialInput, &'static str), String> {
    if !versioned_corpus(bytes)? {
        return serde_json::from_slice(bytes)
            .map(|input| (input, "crypto-observed/v1"))
            .map_err(|e| e.to_string());
    }
    let corpus: CraxCorpus<CraxDifferentialPair> =
        serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    check_crax_corpus(&corpus, "differential", 2)?;
    let first = corpus
        .pairs
        .first()
        .ok_or("AES differential corpus has no pairs")?;
    let alpha = block(&first.p0)?[0] ^ block(&first.p1)?[0];
    Ok((
        DifferentialInput {
            alpha,
            pairs: corpus
                .pairs
                .into_iter()
                .map(|pair| Pair4 {
                    p0_hex: pair.p0,
                    p1_hex: pair.p1,
                    c0_hex: pair.c0,
                    c1_hex: pair.c1,
                })
                .collect(),
        },
        "crax-aes-corpus/v1",
    ))
}

fn parse_linear_input(bytes: &[u8]) -> Result<(LinearInput, &'static str), String> {
    if !versioned_corpus(bytes)? {
        return serde_json::from_slice(bytes)
            .map(|input| (input, "crypto-observed/v1"))
            .map_err(|e| e.to_string());
    }
    let corpus: CraxCorpus<CraxLinearPair> =
        serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    check_crax_corpus(&corpus, "linear", 1)?;
    Ok((
        LinearInput {
            pairs: corpus
                .pairs
                .into_iter()
                .map(|pair| Pair2 {
                    plaintext_hex: pair.plaintext,
                    ciphertext_hex: pair.ciphertext,
                })
                .collect(),
            verification_pairs: Vec::new(),
        },
        "crax-aes-corpus/v1",
    ))
}

fn input_artifact(input: &Path, bytes: &[u8]) -> Value {
    json!({
        "reference":input.to_string_lossy(),
        "sha256":hex::encode(crypto_lib::hash::sha256::sha256(bytes)),
        "bytes":bytes.len(),
    })
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LinearStatisticsInput {
    input_mask_hex: String,
    output_mask_hex: String,
    pairs: Vec<Pair2>,
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

fn record_permutation_sample(
    plaintext: [u8; 16],
    ciphertext: [u8; 16],
    forward: &mut HashMap<[u8; 16], [u8; 16]>,
    reverse: &mut HashMap<[u8; 16], [u8; 16]>,
) -> Result<(), String> {
    if forward.contains_key(&plaintext) {
        return Err("duplicate plaintext in observed corpus".into());
    }
    if reverse.contains_key(&ciphertext) {
        return Err("duplicate ciphertext in observed corpus".into());
    }
    forward.insert(plaintext, ciphertext);
    reverse.insert(ciphertext, plaintext);
    Ok(())
}

fn analyze_differential_statistics(
    data: DifferentialStatisticsInput,
) -> Result<serde_json::Value, String> {
    if !(1..=100_000).contains(&data.pairs.len()) {
        return Err("differential pair count must be 1..=100000".into());
    }
    let input_difference = block(&data.input_difference_hex)?;
    if input_difference.iter().all(|&byte| byte == 0) {
        return Err("input difference must be nonzero".into());
    }
    let mut forward = HashMap::with_capacity(data.pairs.len() * 2);
    let mut reverse = HashMap::with_capacity(data.pairs.len() * 2);
    let mut histogram = [0usize; 17];
    for (index, raw) in data.pairs.iter().enumerate() {
        let p0 = block(&raw.p0_hex)?;
        let p1 = block(&raw.p1_hex)?;
        let c0 = block(&raw.c0_hex)?;
        let c1 = block(&raw.c1_hex)?;
        if (0..16).any(|byte| p0[byte] ^ p1[byte] != input_difference[byte]) {
            return Err(format!("pair {index} has the wrong input difference"));
        }
        record_permutation_sample(p0, c0, &mut forward, &mut reverse)?;
        record_permutation_sample(p1, c1, &mut forward, &mut reverse)?;
        let active = (0..16).filter(|&byte| c0[byte] != c1[byte]).count();
        histogram[active] += 1;
    }
    Ok(json!({
        "schema_version":1,
        "method":"observed-aes-block-differential-activity",
        "status":"measured",
        "input_difference_hex":hex::encode(input_difference),
        "input_active_bytes":input_difference.iter().filter(|&&byte| byte != 0).count(),
        "pairs_used":data.pairs.len(),
        "distinct_plaintexts":forward.len(),
        "output_active_byte_histogram":histogram,
        "one_active_output_pairs":histogram[1]
    }))
}

pub fn differential_statistics(input: &Path) -> Result<(), String> {
    let data: DifferentialStatisticsInput =
        serde_json::from_slice(&read(input)?).map_err(|e| e.to_string())?;
    let report = analyze_differential_statistics(data)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn analyze_linear_statistics(data: LinearStatisticsInput) -> Result<serde_json::Value, String> {
    if !(1..=100_000).contains(&data.pairs.len()) {
        return Err("linear pair count must be 1..=100000".into());
    }
    let input_mask = block(&data.input_mask_hex)?;
    let output_mask = block(&data.output_mask_hex)?;
    if input_mask.iter().all(|&byte| byte == 0) || output_mask.iter().all(|&byte| byte == 0) {
        return Err("input and output masks must both be nonzero".into());
    }
    let mut forward = HashMap::with_capacity(data.pairs.len());
    let mut reverse = HashMap::with_capacity(data.pairs.len());
    let mut zero = 0usize;
    for raw in &data.pairs {
        let plaintext = block(&raw.plaintext_hex)?;
        let ciphertext = block(&raw.ciphertext_hex)?;
        record_permutation_sample(plaintext, ciphertext, &mut forward, &mut reverse)?;
        let parity = (0..16).fold(0u32, |acc, byte| {
            acc ^ (plaintext[byte] & input_mask[byte]).count_ones()
                ^ (ciphertext[byte] & output_mask[byte]).count_ones()
        }) & 1;
        if parity == 0 {
            zero += 1;
        }
    }
    let signed = zero as i64 - (data.pairs.len() - zero) as i64;
    Ok(json!({
        "schema_version":1,
        "method":"observed-aes-block-linear-parity",
        "status":"measured",
        "input_mask_hex":hex::encode(input_mask),
        "output_mask_hex":hex::encode(output_mask),
        "pairs_used":data.pairs.len(),
        "distinct_plaintexts":forward.len(),
        "zero_parity_pairs":zero,
        "one_parity_pairs":data.pairs.len()-zero,
        "signed_parity_count":signed,
        "observed_correlation":signed as f64 / data.pairs.len() as f64,
        "observed_bias":signed as f64 / (2 * data.pairs.len()) as f64
    }))
}

pub fn linear_statistics(input: &Path) -> Result<(), String> {
    let data: LinearStatisticsInput =
        serde_json::from_slice(&read(input)?).map_err(|e| e.to_string())?;
    let report = analyze_linear_statistics(data)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    Ok(())
}

pub fn differential(input: &Path) -> Result<(), String> {
    let bytes = read(input)?;
    let (data, input_format) = parse_differential_input(&bytes)?;
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
    let key_bytes: Vec<_> = report
        .key_bytes
        .iter()
        .map(|(position, value)| json!({"position":position,"value_hex":format!("{value:02x}")}))
        .collect();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema_version":1,
            "model":"aes-128-reduced-two-round-no-final-mixcolumns",
            "input_format":input_format,
            "input_artifact":input_artifact(input, &bytes),
            "status":"partial_candidate",
            "pairs_used":report.pairs_used,
            "trial_count":report.trial_count,
            "last_round_key_bytes":key_bytes
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

pub fn linear(input: &Path, verification_input: Option<&Path>) -> Result<(), String> {
    let bytes = read(input)?;
    let (mut data, input_format) = parse_linear_input(&bytes)?;
    let mut verification_artifact = None;
    let mut verification_format = None;
    if let Some(path) = verification_input {
        if !data.verification_pairs.is_empty() {
            return Err("use either inline verification_pairs or --verification-input".into());
        }
        let held_out_bytes = read(path)?;
        let (held_out, format) = parse_linear_input(&held_out_bytes)?;
        if !held_out.verification_pairs.is_empty() {
            return Err("--verification-input must contain only training pairs".into());
        }
        data.verification_pairs = held_out.pairs;
        verification_artifact = Some(input_artifact(path, &held_out_bytes));
        verification_format = Some(format);
    }
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
    // Per-byte consistency admits arbitrary independent output whitening
    // bytes. Enforce the AES key schedule by replaying the complete training
    // corpus before reporting an input-key candidate.
    let cipher = ReducedAes128::new(&key, 1, false);
    let training_matches = pairs
        .iter()
        .filter(|(p, c)| cipher.encrypt(p) == *c)
        .count();
    let match_count = verify
        .iter()
        .filter(|(p, c)| cipher.encrypt(p) == *c)
        .count();
    let training_ok = training_matches == pairs.len();
    let status = if !training_ok {
        "training_model_mismatch"
    } else if verify.is_empty() {
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
            "input_format":input_format,
            "input_artifact":input_artifact(input, &bytes),
            "verification_format":verification_format,
            "verification_artifact":verification_artifact,
            "method":"per-byte-exhaustive-consistency",
            "status":status,
            "candidate_key_hex":if training_ok {Some(hex::encode(key))} else {None},
            "training_pairs":pairs.len(),
            "training_matches":training_matches,
            "verification_pairs":verify.len(),
            "verification_matches":match_count
        }))
        .map_err(|e| e.to_string())?
    );
    if !training_ok {
        return Err("candidate failed complete training model replay".into());
    }
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
    fn versioned_aes_corpora_require_the_exact_reduced_model() {
        let p0 = "00".repeat(16);
        let p1 = format!("73{}", "00".repeat(15));
        let mut differential = json!({
            "schema_version":1,
            "cipher_model":CRAX_CIPHER_MODEL,
            "technique":"differential",
            "rounds":2,
            "pairs":[{"p0":p0,"p1":p1,"c0":"00".repeat(16),"c1":"01".repeat(16)}]
        });
        let bytes = serde_json::to_vec(&differential).unwrap();
        let (parsed, format) = parse_differential_input(&bytes).unwrap();
        assert_eq!(format, "crax-aes-corpus/v1");
        assert_eq!(parsed.alpha, 0x73);
        differential["rounds"] = json!(1);
        assert!(
            parse_differential_input(&serde_json::to_vec(&differential).unwrap())
                .err()
                .unwrap()
                .contains("round count differs")
        );
        differential["rounds"] = json!(2);
        differential["pairs"][0]["unexpected"] = json!(1);
        assert!(
            parse_differential_input(&serde_json::to_vec(&differential).unwrap())
                .err()
                .unwrap()
                .contains("unknown field")
        );

        let mut linear = json!({
            "schema_version":1,
            "cipher_model":CRAX_CIPHER_MODEL,
            "technique":"linear",
            "rounds":1,
            "pairs":[{"plaintext":"00".repeat(16),"ciphertext":"01".repeat(16)}]
        });
        let (parsed, format) = parse_linear_input(&serde_json::to_vec(&linear).unwrap()).unwrap();
        assert_eq!(format, "crax-aes-corpus/v1");
        assert_eq!(parsed.pairs.len(), 1);
        linear["cipher_model"] = json!("aes128-full/v1");
        assert!(parse_linear_input(&serde_json::to_vec(&linear).unwrap())
            .err()
            .unwrap()
            .contains("model"));
    }

    #[test]
    fn observed_block_statistics_measure_pairs_and_reject_bad_corpora() {
        let p0 = [0u8; 16];
        let mut p1 = p0;
        p1[0] = 1;
        let mut p2 = p0;
        p2[0] = 2;
        let mut p3 = p0;
        p3[0] = 3;
        let c0 = [0u8; 16];
        let mut c1 = c0;
        c1[0] = 0x80;
        let mut c2 = c0;
        c2[0] = 2;
        let mut c3 = c0;
        c3[0] = 3;
        c3[1] = 1;
        let pairs = [(p0, p1, c0, c1), (p2, p3, c2, c3)];
        let mut input = json!({
            "input_difference_hex":format!("01{}", "00".repeat(15)),
            "pairs":pairs.iter().map(|(p0,p1,c0,c1)| json!({
                "p0_hex":hex::encode(p0),"p1_hex":hex::encode(p1),
                "c0_hex":hex::encode(c0),"c1_hex":hex::encode(c1)
            })).collect::<Vec<_>>()
        });
        let parse_diff =
            |value| serde_json::from_value::<DifferentialStatisticsInput>(value).unwrap();
        let report = analyze_differential_statistics(parse_diff(input.clone())).unwrap();
        assert_eq!(report["output_active_byte_histogram"][1], 1);
        assert_eq!(report["output_active_byte_histogram"][2], 1);
        assert_eq!(report["distinct_plaintexts"], 4);
        input["pairs"][1]["p1_hex"] = input["pairs"][1]["p0_hex"].clone();
        assert!(analyze_differential_statistics(parse_diff(input.clone()))
            .unwrap_err()
            .contains("wrong input difference"));
        input["pairs"][1]["p1_hex"] = json!(hex::encode(p3));
        input["pairs"][1]["c1_hex"] = input["pairs"][0]["c1_hex"].clone();
        assert!(analyze_differential_statistics(parse_diff(input))
            .unwrap_err()
            .contains("duplicate ciphertext"));

        let linear_pairs: Vec<_> = [(p0, c0), (p1, c1), (p2, c2), (p3, c3)]
            .iter()
            .map(|(p, c)| {
                json!({
                    "plaintext_hex":hex::encode(p),"ciphertext_hex":hex::encode(c)
                })
            })
            .collect();
        let linear_input = json!({
            "input_mask_hex":format!("01{}", "00".repeat(15)),
            "output_mask_hex":format!("01{}", "00".repeat(15)),
            "pairs":linear_pairs
        });
        let parse_linear = |value| serde_json::from_value::<LinearStatisticsInput>(value).unwrap();
        let report = analyze_linear_statistics(parse_linear(linear_input.clone())).unwrap();
        assert_eq!(report["zero_parity_pairs"], 3);
        assert_eq!(report["one_parity_pairs"], 1);
        assert_eq!(report["signed_parity_count"], 2);
        assert_eq!(report["observed_correlation"], 0.5);
        let mut duplicate = linear_input;
        duplicate["pairs"][3]["plaintext_hex"] = duplicate["pairs"][2]["plaintext_hex"].clone();
        assert!(analyze_linear_statistics(parse_linear(duplicate))
            .unwrap_err()
            .contains("duplicate plaintext"));
    }

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
        assert!(linear(&path, None).is_ok());
        // A fixed output-byte offset leaves each S-box input-byte search
        // consistent, but no longer matches the AES round-key schedule.
        let mut shifted = payload.clone();
        shifted["verification_pairs"] = json!([]);
        for pair in shifted["pairs"].as_array_mut().unwrap() {
            let mut ciphertext = hex::decode(pair["ciphertext_hex"].as_str().unwrap()).unwrap();
            ciphertext[0] ^= 1;
            pair["ciphertext_hex"] = json!(hex::encode(ciphertext));
        }
        fs::write(&path, shifted.to_string()).unwrap();
        assert!(linear(&path, None)
            .unwrap_err()
            .contains("complete training model replay"));
        let mut repeated = payload.clone();
        repeated["pairs"][1] = repeated["pairs"][0].clone();
        fs::write(&path, repeated.to_string()).unwrap();
        assert!(linear(&path, None)
            .unwrap_err()
            .contains("training plaintexts must be distinct"));
        let mut mismatch = payload.clone();
        mismatch["verification_pairs"][0]["ciphertext_hex"] =
            json!("00000000000000000000000000000000");
        fs::write(&path, mismatch.to_string()).unwrap();
        assert!(linear(&path, None)
            .unwrap_err()
            .contains("failed independent verification"));
        let mut invalid = payload;
        invalid["verification_pairs"][0]["plaintext_hex"] =
            invalid["pairs"][0]["plaintext_hex"].clone();
        invalid["verification_pairs"][0]["ciphertext_hex"] =
            invalid["pairs"][0]["ciphertext_hex"].clone();
        fs::write(&path, invalid.to_string()).unwrap();
        assert!(linear(&path, None).unwrap_err().contains("distinct"));

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
