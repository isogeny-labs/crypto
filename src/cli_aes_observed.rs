//! Observed corpora for the explicitly reduced-round AES analysis models.
use crypto_lib::cryptanalysis::aes::{differential, linear, reduced::ReducedAes128};
use serde::Deserialize;
use serde_json::json;
use std::{collections::HashMap, fs, path::Path};

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
    let training = seen.clone();
    let verify = parsed_pairs(&data.verification_pairs, &mut seen)?;
    if verify.iter().any(|(p, _)| training.contains_key(p)) {
        return Err("verification plaintext must be distinct from training plaintexts".into());
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

#[cfg(test)]
mod tests {
    use super::*;

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

        let one = ReducedAes128::new(&key, 1, false);
        let corpus = linear::known_plaintext_corpus(&one, 129, 7);
        let as_json = |(p, c): &([u8; 16], [u8; 16])| json!({"plaintext_hex":hex::encode(p),"ciphertext_hex":hex::encode(c)});
        let payload = json!({"pairs":corpus[..128].iter().map(as_json).collect::<Vec<_>>(),
            "verification_pairs":[as_json(&corpus[128])]});
        fs::write(&path, payload.to_string()).unwrap();
        assert!(linear(&path).is_ok());
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
        fs::remove_file(path).unwrap();
    }
}
