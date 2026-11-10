//! The vault ciphertext format, version 1.
//!
//! ```text
//! blob     = MAGIC || u8 realm_len || realm || u32be epoch || siv_out
//! siv_out  = AES-SIV-256(K[realm,epoch], AD, inner)   (16-byte tag || ciphertext)
//! AD       = [DOMAIN, repo, realm, u32be epoch]
//! inner    = u16be path_len || path || padme(content)
//! ```
//!
//! AES-SIV (RFC 5297) is deterministic: the same key, associated data and
//! plaintext always give the same ciphertext. Git needs that, because the clean
//! filter runs again on every `git status` and must reproduce the blob already
//! in the index. The only thing an observer learns from determinism is whether
//! two blobs are identical, and the embedded path makes identical content at two
//! different paths encrypt differently.
//!
//! The path is inside the encrypted payload rather than in the associated data
//! so that `git diff`'s textconv, which sees a temporary file and not the path,
//! can still decrypt. Integrity covers it either way: the smudge filter and
//! `vault guard` refuse a blob whose embedded path is not the path it sits at,
//! which is what defeats moving one file's ciphertext over another.
//!
//! Content is padded with Padmé (Nikitin et al., PETS 2019) so a file's size is
//! revealed only to within about 12%.

use aes_siv::{siv::Aes256Siv, KeyInit};
use anyhow::{bail, ensure, Context, Result};

/// The leading NUL makes git and GitHub treat a blob as binary.
pub const MAGIC: &[u8; 6] = b"\0AVLT1";
pub const DOMAIN: &[u8] = b"isogeny-labs/vault/v1";
/// AES-SIV-256 takes a 512-bit key (MAC key || CTR key).
pub const KEY_LEN: usize = 64;

pub type Key = zeroize::Zeroizing<[u8; KEY_LEN]>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub realm: String,
    pub epoch: u32,
}

pub fn is_ciphertext(data: &[u8]) -> bool {
    data.starts_with(MAGIC)
}

pub fn validate_realm(realm: &str) -> Result<()> {
    ensure!(
        !realm.is_empty()
            && realm.len() <= 32
            && realm
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'),
        "realm {realm:?} must be 1-32 characters of [a-z0-9-]"
    );
    Ok(())
}

/// Padmé length for `l` bytes: at most about 12% overhead, and only
/// O(log log l) bits of the length survive.
pub fn padme(l: u64) -> u64 {
    if l < 2 {
        return l;
    }
    let e = 63 - l.leading_zeros() as u64; // floor(log2 l)
    let s = 64 - e.leading_zeros() as u64; // floor(log2 e) + 1
    let last_bits = e - s;
    let mask = (1u64 << last_bits) - 1;
    (l + mask) & !mask
}

fn pad(content: &[u8]) -> Vec<u8> {
    let target = padme(content.len() as u64 + 1) as usize;
    let mut out = Vec::with_capacity(target);
    out.extend_from_slice(content);
    out.push(0x80);
    out.resize(target, 0);
    out
}

fn unpad(mut padded: Vec<u8>) -> Result<Vec<u8>> {
    let end = padded
        .iter()
        .rposition(|&b| b != 0)
        .context("vault: padding is all zeros")?;
    ensure!(padded[end] == 0x80, "vault: bad padding terminator");
    padded.truncate(end);
    Ok(padded)
}

fn ad<'a>(repo: &'a str, realm: &'a str, epoch: &'a [u8; 4]) -> [&'a [u8]; 4] {
    [DOMAIN, repo.as_bytes(), realm.as_bytes(), epoch]
}

pub fn encrypt(
    key: &Key,
    repo: &str,
    realm: &str,
    epoch: u32,
    path: &str,
    content: &[u8],
) -> Result<Vec<u8>> {
    validate_realm(realm)?;
    ensure!(path.len() <= u16::MAX as usize, "vault: path too long");
    let padded = pad(content);
    let mut inner = Vec::with_capacity(2 + path.len() + padded.len());
    inner.extend_from_slice(&(path.len() as u16).to_be_bytes());
    inner.extend_from_slice(path.as_bytes());
    inner.extend_from_slice(&padded);

    let mut cipher = Aes256Siv::new(key.as_slice().into());
    let e = epoch.to_be_bytes();
    let sealed = cipher
        .encrypt(ad(repo, realm, &e), &inner)
        .map_err(|_| anyhow::anyhow!("vault: AES-SIV encryption failed"))?;

    let mut out = Vec::with_capacity(MAGIC.len() + 1 + realm.len() + 4 + sealed.len());
    out.extend_from_slice(MAGIC);
    out.push(realm.len() as u8);
    out.extend_from_slice(realm.as_bytes());
    out.extend_from_slice(&e);
    out.extend_from_slice(&sealed);
    Ok(out)
}

/// Splits a blob into its header and the sealed body. Does not authenticate.
pub fn parse(blob: &[u8]) -> Result<(Header, &[u8])> {
    ensure!(is_ciphertext(blob), "vault: not a vault blob");
    let rest = &blob[MAGIC.len()..];
    let (&rlen, rest) = rest.split_first().context("vault: truncated header")?;
    let rlen = rlen as usize;
    ensure!(rest.len() >= rlen + 4 + 16, "vault: truncated header");
    let realm = std::str::from_utf8(&rest[..rlen]).context("vault: realm is not UTF-8")?;
    validate_realm(realm)?;
    let epoch = u32::from_be_bytes(rest[rlen..rlen + 4].try_into().unwrap());
    Ok((
        Header {
            realm: realm.to_string(),
            epoch,
        },
        &rest[rlen + 4..],
    ))
}

/// Authenticates and decrypts. Returns the embedded path and the content.
pub fn decrypt(key: &Key, repo: &str, header: &Header, body: &[u8]) -> Result<(String, Vec<u8>)> {
    let mut cipher = Aes256Siv::new(key.as_slice().into());
    let e = header.epoch.to_be_bytes();
    let inner = cipher
        .decrypt(ad(repo, &header.realm, &e), body)
        .map_err(|_| {
            anyhow::anyhow!(
                "vault: authentication failed (wrong key, repo, realm or epoch, or tampered blob)"
            )
        })?;
    ensure!(inner.len() >= 2, "vault: truncated payload");
    let plen = u16::from_be_bytes([inner[0], inner[1]]) as usize;
    if inner.len() < 2 + plen {
        bail!("vault: truncated path");
    }
    let path =
        String::from_utf8(inner[2..2 + plen].to_vec()).context("vault: path is not UTF-8")?;
    let content = unpad(inner[2 + plen..].to_vec())?;
    Ok((path, content))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(b: u8) -> Key {
        zeroize::Zeroizing::new([b; KEY_LEN])
    }

    #[test]
    fn roundtrip_and_determinism() {
        let k = key(7);
        for len in [0usize, 1, 2, 15, 16, 17, 255, 1000, 65_536] {
            let msg: Vec<u8> = (0..len).map(|i| (i * 31 % 251) as u8).collect();
            let a = encrypt(&k, "o/r", "findings", 1, "research/x.md", &msg).unwrap();
            let b = encrypt(&k, "o/r", "findings", 1, "research/x.md", &msg).unwrap();
            assert_eq!(a, b, "clean filter output must be deterministic");
            let (h, body) = parse(&a).unwrap();
            assert_eq!(
                h,
                Header {
                    realm: "findings".into(),
                    epoch: 1
                }
            );
            let (p, m) = decrypt(&k, "o/r", &h, body).unwrap();
            assert_eq!(p, "research/x.md");
            assert_eq!(m, msg);
        }
    }

    #[test]
    fn content_with_trailing_zeros_and_0x80_survives() {
        let k = key(1);
        for msg in [&b"\x80"[..], b"\x00\x00", b"abc\x80\x00", b"\x80\x80\x00"] {
            let c = encrypt(&k, "o/r", "f", 1, "p", msg).unwrap();
            let (h, body) = parse(&c).unwrap();
            assert_eq!(decrypt(&k, "o/r", &h, body).unwrap().1, msg);
        }
    }

    #[test]
    fn path_is_bound_and_distinguishes_identical_content() {
        let k = key(9);
        let a = encrypt(&k, "o/r", "findings", 1, "a.md", b"same").unwrap();
        let b = encrypt(&k, "o/r", "findings", 1, "b.md", b"same").unwrap();
        assert_ne!(a, b);
        let (h, body) = parse(&b).unwrap();
        assert_eq!(decrypt(&k, "o/r", &h, body).unwrap().0, "b.md");
    }

    #[test]
    fn rejects_wrong_key_repo_realm_epoch_and_tamper() {
        let k = key(3);
        let c = encrypt(&k, "o/r", "findings", 2, "p", b"secret").unwrap();
        let (h, body) = parse(&c).unwrap();
        assert!(decrypt(&key(4), "o/r", &h, body).is_err());
        assert!(decrypt(&k, "o/other", &h, body).is_err());
        let h2 = Header {
            realm: "leads".into(),
            epoch: 2,
        };
        assert!(decrypt(&k, "o/r", &h2, body).is_err());
        let h3 = Header {
            realm: "findings".into(),
            epoch: 1,
        };
        assert!(decrypt(&k, "o/r", &h3, body).is_err());
        for i in 0..body.len() {
            let mut t = body.to_vec();
            t[i] ^= 1;
            assert!(
                decrypt(&k, "o/r", &h, &t).is_err(),
                "bit flip at {i} accepted"
            );
        }
    }

    #[test]
    fn ciphertext_does_not_contain_plaintext_and_is_binary() {
        let k = key(5);
        let c = encrypt(&k, "o/r", "findings", 1, "p", b"THE-SECRET-FINDING").unwrap();
        assert!(!c.windows(10).any(|w| w == b"THE-SECRET"));
        assert_eq!(c[0], 0);
    }

    #[test]
    fn padme_bounds() {
        assert_eq!(padme(0), 0);
        assert_eq!(padme(1), 1);
        for l in 2..200_000u64 {
            let p = padme(l);
            assert!(p >= l);
            assert!((p - l) as f64 <= 0.12 * l as f64 + 1.0, "l={l} p={p}");
        }
    }

    /// Regression vector: freezes the byte format. A change here is a format
    /// break and needs a new MAGIC.
    #[test]
    fn frozen_vector() {
        let k: Key = zeroize::Zeroizing::new(core::array::from_fn(|i| i as u8));
        let c = encrypt(
            &k,
            "isogeny-labs/crypto",
            "findings",
            1,
            "research/a.md",
            b"hello\n",
        )
        .unwrap();
        let hex: String = c.iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(hex, include_str!("../tests/frozen_vector.hex").trim());
    }

    #[test]
    fn realm_validation() {
        assert!(validate_realm("findings").is_ok());
        assert!(validate_realm("raw-2").is_ok());
        for bad in ["", "Findings", "a/b", "a b", &"x".repeat(33)] {
            assert!(validate_realm(bad).is_err(), "{bad:?}");
        }
    }
}
