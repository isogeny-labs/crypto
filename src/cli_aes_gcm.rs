//! Caller-supplied AES-GCM operations. Hex output keeps binary results unambiguous.
use clap::{Args, Subcommand};
use crypto_lib::symmetric::aes::{aes_gcm_decrypt, aes_gcm_encrypt, AesKey};
use std::{fs, path::PathBuf};

#[derive(Subcommand)]
pub enum AesOp {
    /// Encrypt bytes and print ciphertext followed by its 16-byte authentication tag in hex.
    GcmEncrypt(GcmArgs),
    /// Verify the authentication tag and print plaintext in hex only on success.
    GcmDecrypt(GcmArgs),
}

#[derive(Args)]
pub struct GcmArgs {
    /// UTF-8 input; use --hex or --file for arbitrary bytes.
    #[arg(conflicts_with_all = ["hex", "file"])]
    message: Option<String>,
    /// Hex-encoded plaintext or ciphertext followed by its tag.
    #[arg(long, conflicts_with = "file")]
    hex: Option<String>,
    /// Read plaintext or ciphertext followed by its tag from a file.
    #[arg(long)]
    file: Option<PathBuf>,
    /// Read 16 or 32 raw AES key bytes from a file.
    #[arg(long, conflicts_with = "key_hex")]
    key_file: Option<PathBuf>,
    /// Hex-encoded 16 or 32 byte key, useful for public test vectors.
    #[arg(long)]
    key_hex: Option<String>,
    /// Unique 12-byte nonce for this encryption key, encoded as 24 hex digits.
    #[arg(long)]
    nonce_hex: String,
    /// Associated data as hex-encoded bytes (empty by default).
    #[arg(long)]
    aad_hex: Option<String>,
}

pub fn run(op: AesOp) -> Result<(), String> {
    let output = match op {
        AesOp::GcmEncrypt(args) => transform(&args, false)?,
        AesOp::GcmDecrypt(args) => transform(&args, true)?,
    };
    println!("{}", hex::encode(output));
    Ok(())
}

fn transform(args: &GcmArgs, decrypt: bool) -> Result<Vec<u8>, String> {
    let data = match (&args.message, &args.hex, &args.file) {
        (Some(message), None, None) => message.as_bytes().to_vec(),
        (None, Some(encoded), None) => decode("input", encoded)?,
        (None, None, Some(path)) => {
            fs::read(path).map_err(|e| format!("cannot read input file {}: {e}", path.display()))?
        }
        _ => return Err("provide exactly one of message, --hex, or --file".into()),
    };
    let key_bytes = match (&args.key_file, &args.key_hex) {
        (Some(path), None) => {
            fs::read(path).map_err(|e| format!("cannot read key file {}: {e}", path.display()))?
        }
        (None, Some(encoded)) => decode("key", encoded)?,
        _ => return Err("provide exactly one of --key-file or --key-hex".into()),
    };
    let key = AesKey::new(&key_bytes).map_err(str::to_owned)?;
    let nonce: [u8; 12] = decode("nonce", &args.nonce_hex)?
        .try_into()
        .map_err(|_| "nonce must be exactly 12 bytes".to_string())?;
    let aad = args
        .aad_hex
        .as_deref()
        .map(|encoded| decode("AAD", encoded))
        .transpose()?
        .unwrap_or_default();

    // The GCM implementation uses a 32-bit CTR counter starting at 2.
    // Validate its capacity before calling its infallible encrypt/decrypt API.
    let plaintext_len = if decrypt {
        data.len()
            .checked_sub(16)
            .ok_or("ciphertext must include a 16-byte authentication tag")?
    } else {
        data.len()
    };
    if !crypto_lib::symmetric::aes::aes_ctr_len_fits(plaintext_len, 2) {
        return Err("input exceeds the AES-GCM counter limit".into());
    }
    if decrypt {
        aes_gcm_decrypt(&data, &key, &nonce, &aad).map_err(|_| "authentication failed".to_string())
    } else {
        Ok(aes_gcm_encrypt(&data, &key, &nonce, &aad))
    }
}

fn decode(label: &str, encoded: &str) -> Result<Vec<u8>, String> {
    hex::decode(encoded).map_err(|e| format!("invalid {label} hex: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(message: Option<&str>, hex: Option<&str>, key_hex: Option<&str>) -> GcmArgs {
        GcmArgs {
            message: message.map(str::to_owned),
            hex: hex.map(str::to_owned),
            file: None,
            key_file: None,
            key_hex: key_hex.map(str::to_owned),
            nonce_hex: "000000000000000000000000".into(),
            aad_hex: None,
        }
    }

    #[test]
    fn nist_empty_plaintext_and_single_block_vectors() {
        let key = "00000000000000000000000000000000";
        assert_eq!(
            hex::encode(transform(&args(None, Some(""), Some(key)), false).unwrap()),
            "58e2fccefa7e3061367f1d57a4e7455a"
        );
        let encrypted = transform(&args(None, Some(&"00".repeat(16)), Some(key)), false).unwrap();
        assert_eq!(
            hex::encode(&encrypted),
            "0388dace60b6a392f328c2b971b2fe78ab6e47d42cec13bdf53a67b21257bddf"
        );
        let decoded =
            transform(&args(None, Some(&hex::encode(encrypted)), Some(key)), true).unwrap();
        assert_eq!(decoded, [0u8; 16]);
    }

    #[test]
    fn associated_data_and_authentication_failure() {
        let mut input = args(Some("payload"), None, Some(&"11".repeat(32)));
        input.aad_hex = Some("00ff".into());
        let encrypted = transform(&input, false).unwrap();
        let mut received = args(None, Some(&hex::encode(&encrypted)), Some(&"11".repeat(32)));
        received.aad_hex = Some("00ff".into());
        assert_eq!(transform(&received, true).unwrap(), b"payload");
        received.aad_hex = Some("00fe".into());
        assert_eq!(
            transform(&received, true).unwrap_err(),
            "authentication failed"
        );
        received.aad_hex = Some("00ff".into());
        received.hex = Some(hex::encode(&encrypted[..15]));
        assert!(transform(&received, true).is_err());
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert!(transform(&args(None, None, Some(&"00".repeat(16))), false).is_err());
        assert!(transform(&args(None, Some("0"), Some(&"00".repeat(16))), false).is_err());
        assert!(transform(&args(None, Some(""), Some(&"00".repeat(24))), false).is_err());
        let mut input = args(None, Some(""), Some(&"00".repeat(16)));
        input.nonce_hex = "00".into();
        assert!(transform(&input, false).is_err());
    }
}
