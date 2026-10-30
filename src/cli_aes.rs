//! Caller-supplied AES operations. Hex output keeps binary results unambiguous.
use clap::{Args, Subcommand};
use crypto_lib::symmetric::{
    aes::{
        aes_ctr_checked, aes_gcm_decrypt, aes_gcm_encrypt, decrypt_block, encrypt_block, AesKey,
    },
    aes_cbc::{aes_cbc_decrypt, aes_cbc_encrypt},
};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Subcommand)]
pub enum AesOp {
    /// Encrypt exactly one 16-byte block with raw AES (no mode or padding).
    BlockEncrypt(IoArgs),
    /// Decrypt exactly one 16-byte block with raw AES (no mode or padding).
    BlockDecrypt(IoArgs),
    /// Apply AES-CTR (same operation for encryption and decryption); output hex.
    Ctr(CtrArgs),
    /// Encrypt with AES-CBC and PKCS#7; output IV followed by ciphertext in hex.
    CbcEncrypt(CbcEncryptArgs),
    /// Decrypt IV-prefixed AES-CBC with PKCS#7; output plaintext in hex.
    CbcDecrypt(IoArgs),
    /// Encrypt bytes and print ciphertext followed by its 16-byte authentication tag in hex.
    GcmEncrypt(GcmArgs),
    /// Verify the authentication tag and print plaintext in hex only on success.
    GcmDecrypt(GcmArgs),
}

#[derive(Args)]
pub struct IoArgs {
    /// UTF-8 input; use --hex or --file for arbitrary bytes.
    #[arg(conflicts_with_all = ["hex", "file"])]
    message: Option<String>,
    /// Hex-encoded input bytes.
    #[arg(long, conflicts_with = "file")]
    hex: Option<String>,
    /// Read raw input bytes from a file or checked store: reference.
    #[arg(long)]
    file: Option<PathBuf>,
    /// Read 16, 24, or 32 raw AES key bytes from a file or checked store: reference.
    #[arg(long, conflicts_with = "key_hex")]
    key_file: Option<PathBuf>,
    /// Hex-encoded 16, 24, or 32 byte key, useful for public test vectors.
    #[arg(long)]
    key_hex: Option<String>,
}

#[derive(Args)]
pub struct CtrArgs {
    #[command(flatten)]
    io: IoArgs,
    /// Unique 12-byte nonce for this key, encoded as 24 hex digits.
    #[arg(long)]
    nonce_hex: String,
}

#[derive(Args)]
pub struct CbcEncryptArgs {
    #[command(flatten)]
    io: IoArgs,
    /// Unique 16-byte IV encoded as 32 hex digits.
    #[arg(long)]
    iv_hex: String,
}

#[derive(Args)]
pub struct GcmArgs {
    /// UTF-8 input; use --hex or --file for arbitrary bytes.
    #[arg(conflicts_with_all = ["hex", "file"])]
    message: Option<String>,
    /// Hex-encoded plaintext or ciphertext followed by its tag.
    #[arg(long, conflicts_with = "file")]
    hex: Option<String>,
    /// Read plaintext or ciphertext followed by its tag from a file or checked store: reference.
    #[arg(long)]
    file: Option<PathBuf>,
    /// Read 16, 24, or 32 raw AES key bytes from a file or checked store: reference.
    #[arg(long, conflicts_with = "key_hex")]
    key_file: Option<PathBuf>,
    /// Hex-encoded 16, 24, or 32 byte key, useful for public test vectors.
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
        AesOp::BlockEncrypt(args) => block_transform(&args, false)?,
        AesOp::BlockDecrypt(args) => block_transform(&args, true)?,
        AesOp::Ctr(args) => ctr_transform(&args)?,
        AesOp::CbcEncrypt(args) => cbc_encrypt(&args)?,
        AesOp::CbcDecrypt(args) => cbc_decrypt(&args)?,
        AesOp::GcmEncrypt(args) => transform(&args, false)?,
        AesOp::GcmDecrypt(args) => transform(&args, true)?,
    };
    println!("{}", hex::encode(output));
    Ok(())
}

fn load_io(args: &IoArgs) -> Result<(Vec<u8>, AesKey), String> {
    let data = match (&args.message, &args.hex, &args.file) {
        (Some(message), None, None) => message.as_bytes().to_vec(),
        (None, Some(encoded), None) => decode("input", encoded)?,
        (None, None, Some(path)) => read_checked_file(path, "input")?,
        _ => return Err("provide exactly one of message, --hex, or --file".into()),
    };
    let key_bytes = match (&args.key_file, &args.key_hex) {
        (Some(path), None) => read_checked_file(path, "key")?,
        (None, Some(encoded)) => decode("key", encoded)?,
        _ => return Err("provide exactly one of --key-file or --key-hex".into()),
    };
    Ok((data, AesKey::new(&key_bytes).map_err(str::to_owned)?))
}

fn block_transform(args: &IoArgs, decrypt: bool) -> Result<Vec<u8>, String> {
    let (data, key) = load_io(args)?;
    let block: [u8; 16] = data
        .try_into()
        .map_err(|_| "raw AES block input must be exactly 16 bytes".to_string())?;
    Ok(if decrypt {
        decrypt_block(&block, &key).to_vec()
    } else {
        encrypt_block(&block, &key).to_vec()
    })
}

fn ctr_transform(args: &CtrArgs) -> Result<Vec<u8>, String> {
    let (data, key) = load_io(&args.io)?;
    let nonce: [u8; 12] = decode("nonce", &args.nonce_hex)?
        .try_into()
        .map_err(|_| "nonce must be exactly 12 bytes".to_string())?;
    aes_ctr_checked(&data, &key, &nonce).map_err(str::to_owned)
}

fn cbc_encrypt(args: &CbcEncryptArgs) -> Result<Vec<u8>, String> {
    let (data, key) = load_io(&args.io)?;
    let iv: [u8; 16] = decode("IV", &args.iv_hex)?
        .try_into()
        .map_err(|_| "IV must be exactly 16 bytes".to_string())?;
    Ok(aes_cbc_encrypt(&data, &key, &iv))
}

fn cbc_decrypt(args: &IoArgs) -> Result<Vec<u8>, String> {
    let (data, key) = load_io(args)?;
    aes_cbc_decrypt(&data, &key).ok_or_else(|| "CBC ciphertext or padding invalid".to_string())
}

fn transform(args: &GcmArgs, decrypt: bool) -> Result<Vec<u8>, String> {
    let data = match (&args.message, &args.hex, &args.file) {
        (Some(message), None, None) => message.as_bytes().to_vec(),
        (None, Some(encoded), None) => decode("input", encoded)?,
        (None, None, Some(path)) => read_checked_file(path, "input")?,
        _ => return Err("provide exactly one of message, --hex, or --file".into()),
    };
    let key_bytes = match (&args.key_file, &args.key_hex) {
        (Some(path), None) => read_checked_file(path, "key")?,
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

fn read_checked_file(path: &Path, label: &str) -> Result<Vec<u8>, String> {
    let resolved = crypto_lib::store::resolve_path(path)?;
    fs::read(&resolved).map_err(|e| format!("cannot read {label} file {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn io(input_hex: &str, key_hex: &str) -> IoArgs {
        IoArgs {
            message: None,
            hex: Some(input_hex.into()),
            file: None,
            key_file: None,
            key_hex: Some(key_hex.into()),
        }
    }

    #[test]
    fn block_commands_match_fips_197_all_key_sizes() {
        let plaintext = "00112233445566778899aabbccddeeff";
        for (key, expected) in [
            (
                "000102030405060708090a0b0c0d0e0f",
                "69c4e0d86a7b0430d8cdb78070b4c55a",
            ),
            (
                "000102030405060708090a0b0c0d0e0f1011121314151617",
                "dda97ca4864cdfe06eaf70a0ec0d7191",
            ),
            (
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
                "8ea2b7ca516745bfeafc49904b496089",
            ),
        ] {
            assert_eq!(
                hex::encode(block_transform(&io(plaintext, key), false).unwrap()),
                expected
            );
            assert_eq!(
                hex::encode(block_transform(&io(expected, key), true).unwrap()),
                plaintext
            );
        }
        assert!(block_transform(&io("00", &"00".repeat(16)), false).is_err());
    }

    #[test]
    fn ctr_command_matches_reference_vector_and_checks_nonce() {
        let mut args = CtrArgs {
            io: io(
                "6bc1bee22e409f96e93d7e117393172a",
                "2b7e151628aed2a6abf7158809cf4f3c",
            ),
            nonce_hex: "f0f1f2f3f4f5f6f7f8f9fafb".into(),
        };
        let ciphertext = ctr_transform(&args).unwrap();
        assert_eq!(hex::encode(&ciphertext), "288028c71599c5a8dd53c2671b86b813");
        args.io.hex = Some(hex::encode(ciphertext));
        assert_eq!(
            hex::encode(ctr_transform(&args).unwrap()),
            "6bc1bee22e409f96e93d7e117393172a"
        );
        args.nonce_hex = "00".into();
        assert!(ctr_transform(&args).is_err());
    }

    #[test]
    fn cbc_commands_match_independent_pkcs7_vectors() {
        for (key_len, expected) in [
            (16, "976a995f94cf4c1e24b6e2fb067e7a69"),
            (24, "8c1effa608b37866d0737f6817fd2a91"),
            (32, "9379c179f64b7f21538d43ae6304838e"),
        ] {
            let key = "00".repeat(key_len);
            let args = CbcEncryptArgs {
                io: io("616263", &key),
                iv_hex: "00".repeat(16),
            };
            let ciphertext = cbc_encrypt(&args).unwrap();
            assert_eq!(
                hex::encode(&ciphertext),
                format!("{}{}", "00".repeat(16), expected)
            );
            assert_eq!(
                cbc_decrypt(&io(&hex::encode(ciphertext), &key)).unwrap(),
                b"abc"
            );
        }
        assert!(cbc_decrypt(&io("00", &"00".repeat(16))).is_err());
        let args = CbcEncryptArgs {
            io: io("", &"00".repeat(16)),
            iv_hex: "00".into(),
        };
        assert!(cbc_encrypt(&args).is_err());
    }

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

        let key192 = "00".repeat(24);
        assert_eq!(
            hex::encode(transform(&args(None, Some(""), Some(&key192)), false).unwrap()),
            "cd33b28ac773f74ba00ed1f312572435"
        );
        assert_eq!(
            hex::encode(
                transform(&args(None, Some(&"00".repeat(16)), Some(&key192)), false).unwrap()
            ),
            "98e7247c07f0fe411c267e4384b0f6002ff58d80033927ab8ef4d4587514f0fb"
        );
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
        assert!(transform(&args(None, Some(""), Some(&"00".repeat(20))), false).is_err());
        let mut input = args(None, Some(""), Some(&"00".repeat(16)));
        input.nonce_hex = "00".into();
        assert!(transform(&input, false).is_err());
    }
}
