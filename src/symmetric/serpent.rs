//! Serpent — 128-bit block cipher with 32 rounds.
//!
//! Designed by Anderson, Biham, and Knudsen (1998); a finalist in the
//! AES competition where it placed second to Rijndael, primarily on
//! performance grounds (Serpent is ~2× slower than AES in software).
//! It has a substantially larger security margin than AES — 32 rounds
//! vs. 10–14 — and is widely regarded as one of the most conservative
//! block-cipher designs from that era.
//!
//! # Structure
//! - 128-bit blocks (4 × little-endian u32 words)
//! - 256-bit master key (shorter keys are padded per the spec — append a
//!   `0x01` bit, then zero-pad to 256 bits)
//! - 32 rounds, each: `AddRoundKey → S-box → LinearTransform`, except the
//!   final round which replaces `LinearTransform` with one extra
//!   `AddRoundKey`
//! - 33 round keys derived via the prekey recurrence
//!   `w_i = ((w_{i−8} ⊕ w_{i−5} ⊕ w_{i−3} ⊕ w_{i−1} ⊕ φ ⊕ i) <<< 11)`
//!   where `φ = 0x9e3779b9` (the fractional part of the golden ratio).
//! - The eight 4-bit S-boxes (S0..S7) are applied bit-sliced: a
//!   "nibble at position j" is `(bit j of W0, bit j of W1, bit j of W2,
//!   bit j of W3)`.  This is the bit-sliced form of the cipher, in which
//!   the specification's initial and final permutations (IP / FP) do not
//!   appear: they only convert between the "standard" description and the
//!   bit-sliced one, and applying them around bit-sliced rounds gives a
//!   different cipher.
//!
//! # Side-channel posture
//!
//! S-box lookups go through a constant-time scan-and-select pattern
//! (`ct_sbox4`) instead of a direct `table[idx]` lookup, on the same
//! reasoning that motivated the AES `sbox_ct` rewrite earlier in this
//! crate: a 16-byte table fits in one cache line on every modern
//! CPU, so the cache-line channel is closed automatically — but a
//! direct index would still expose the secret nibble through other
//! micro-architectural channels (port contention, speculation).
//! The CT scan costs ~16× per S-box call but keeps the access
//! pattern uniform.
//!
//! The linear transform and the key schedule operate
//! over the full state with no input-dependent branching.
//!
//! # Byte order and test vectors
//! A block is four little-endian `u32` words, the convention of the
//! optimised reference code and of most libraries.  The Cambridge
//! submission's vector files print a block as one 128-bit hexadecimal
//! number, most significant byte first, so a vector from those files is
//! the byte-reversed form of this module's block.  The tests check the
//! `I=0` entries of `ecb_e_m.txt` (10,000 chained encryptions of the zero
//! block under the zero key) for 128, 192 and 256-bit keys.  Those entries
//! use an all-zero key, so they do not exercise the byte order of a
//! non-zero key.
//!
//! Until 2026-11-15 this module wrapped the bit-sliced rounds in IP / FP
//! and so computed a different permutation.  Data encrypted with that
//! version does not decrypt with this one.
//!
//! # Caveats
//! - Serpent has no standardised AEAD construction in this crate;
//!   pair it with an independent MAC (e.g. HMAC-SHA-256) if you
//!   need authentication.

use subtle::{ConditionallySelectable, ConstantTimeEq};

// ── Constants ─────────────────────────────────────────────────────────────────

/// Golden-ratio fractional constant `(√5 − 1)/2 · 2^32`, used in the prekey
/// recurrence as a "nothing-up-my-sleeve" mixing value.
const PHI: u32 = 0x9e37_79b9;

/// The eight Serpent 4-bit S-boxes (S0..S7), as nibble-permutation tables.
/// Values are taken verbatim from Anderson–Biham–Shamir 1998, Appendix A.
#[rustfmt::skip]
const SBOX: [[u8; 16]; 8] = [
    [ 3,  8, 15,  1, 10,  6,  5, 11, 14, 13,  4,  2,  7,  0,  9, 12], // S0
    [15, 12,  2,  7,  9,  0,  5, 10,  1, 11, 14,  8,  6, 13,  3,  4], // S1
    [ 8,  6,  7,  9,  3, 12, 10, 15, 13,  1, 14,  4,  0, 11,  5,  2], // S2
    [ 0, 15, 11,  8, 12,  9,  6,  3, 13,  1,  2,  4, 10,  7,  5, 14], // S3
    [ 1, 15,  8,  3, 12,  0, 11,  6,  2,  5,  4, 10,  9, 14,  7, 13], // S4
    [15,  5,  2, 11,  4, 10,  9, 12,  0,  3, 14,  8, 13,  6,  7,  1], // S5
    [ 7,  2, 12,  5,  8,  4,  6, 11, 14,  9,  1, 15, 13,  3, 10,  0], // S6
    [ 1, 13, 15,  0, 14,  8,  2, 11,  7,  4, 12, 10,  9,  3,  5,  6], // S7
];

/// Per-S-box inverse, computed at compile time from `SBOX`.
const INV_SBOX: [[u8; 16]; 8] = compute_inv_sbox();

const fn compute_inv_sbox() -> [[u8; 16]; 8] {
    let mut out = [[0u8; 16]; 8];
    let mut s = 0;
    while s < 8 {
        let mut i = 0;
        while i < 16 {
            out[s][SBOX[s][i] as usize] = i as u8;
            i += 1;
        }
        s += 1;
    }
    out
}

// ── Constant-time 4-bit S-box lookup ─────────────────────────────────────────
//
// The full 16-entry scan keeps the access pattern uniform, mirroring
// the AES `sbox_ct` strategy.  A 16-byte table fits inside a single L1
// cache line on every modern CPU, so the *cache-line* channel is closed
// even with a direct lookup; the scan additionally closes the
// port-contention and speculation channels at the cost of a 16× per-
// call overhead.  Per encryption that is 32 nibbles × 32 rounds × 16 ≈
// 16k extra ops, comfortably under any practical bandwidth budget for
// non-bulk uses.

#[inline(always)]
fn ct_sbox4(table: &[u8; 16], idx: u8) -> u8 {
    let mut out: u8 = 0;
    let mut i: u8 = 0;
    while i < 16 {
        // `ct_eq` compiles to a constant-time equality compare;
        // `conditional_select` is volatile-cmov.  No branch on `idx`.
        let choice = i.ct_eq(&idx);
        out = u8::conditional_select(&out, &table[i as usize], choice);
        i += 1;
    }
    out
}

// ── Block / word helpers ─────────────────────────────────────────────────────

#[inline]
fn block_to_words(block: &[u8; 16]) -> [u32; 4] {
    [
        u32::from_le_bytes(block[0..4].try_into().unwrap()),
        u32::from_le_bytes(block[4..8].try_into().unwrap()),
        u32::from_le_bytes(block[8..12].try_into().unwrap()),
        u32::from_le_bytes(block[12..16].try_into().unwrap()),
    ]
}

#[inline]
fn words_to_block(words: &[u32; 4]) -> [u8; 16] {
    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&words[0].to_le_bytes());
    out[4..8].copy_from_slice(&words[1].to_le_bytes());
    out[8..12].copy_from_slice(&words[2].to_le_bytes());
    out[12..16].copy_from_slice(&words[3].to_le_bytes());
    out
}

// ── S-box (bit-sliced) ───────────────────────────────────────────────────────

fn apply_sbox_bitslice(table: &[u8; 16], state: &mut [u32; 4]) {
    let mut new_state = [0u32; 4];
    for bit_pos in 0..32 {
        let nibble = ((state[0] >> bit_pos) & 1) as u8
            | (((state[1] >> bit_pos) & 1) as u8) << 1
            | (((state[2] >> bit_pos) & 1) as u8) << 2
            | (((state[3] >> bit_pos) & 1) as u8) << 3;
        let s = ct_sbox4(table, nibble) as u32;
        new_state[0] |= (s & 1) << bit_pos;
        new_state[1] |= ((s >> 1) & 1) << bit_pos;
        new_state[2] |= ((s >> 2) & 1) << bit_pos;
        new_state[3] |= ((s >> 3) & 1) << bit_pos;
    }
    *state = new_state;
}

#[inline]
fn sbox(round: usize, state: &mut [u32; 4]) {
    apply_sbox_bitslice(&SBOX[round % 8], state);
}

#[inline]
fn sbox_inv(round: usize, state: &mut [u32; 4]) {
    apply_sbox_bitslice(&INV_SBOX[round % 8], state);
}

// ── Linear Transform ─────────────────────────────────────────────────────────
//
// Verbatim from Anderson–Biham–Shamir §3 — the bit-rotation and shift
// constants are the published values; no operand-dependent branching.

fn lt(state: &mut [u32; 4]) {
    state[0] = state[0].rotate_left(13);
    state[2] = state[2].rotate_left(3);
    state[1] ^= state[0] ^ state[2];
    state[3] ^= state[2] ^ (state[0] << 3);
    state[1] = state[1].rotate_left(1);
    state[3] = state[3].rotate_left(7);
    state[0] ^= state[1] ^ state[3];
    state[2] ^= state[3] ^ (state[1] << 7);
    state[0] = state[0].rotate_left(5);
    state[2] = state[2].rotate_left(22);
}

fn lt_inv(state: &mut [u32; 4]) {
    state[2] = state[2].rotate_right(22);
    state[0] = state[0].rotate_right(5);
    state[2] ^= state[3] ^ (state[1] << 7);
    state[0] ^= state[1] ^ state[3];
    state[3] = state[3].rotate_right(7);
    state[1] = state[1].rotate_right(1);
    state[3] ^= state[2] ^ (state[0] << 3);
    state[1] ^= state[0] ^ state[2];
    state[2] = state[2].rotate_right(3);
    state[0] = state[0].rotate_right(13);
}

// ── Key schedule ─────────────────────────────────────────────────────────────

/// 33 round keys of 4 × `u32` each (one per round plus a final post-
/// whitening key).  The struct is `Drop`-zeroized to scrub round keys
/// from memory after use; this is best-effort (Rust's stack-spill /
/// register-spill semantics are not guaranteed) but matches the
/// `RsaPrivateKey` policy elsewhere in the crate.
#[derive(Clone)]
pub struct SerpentKey {
    round_keys: [[u32; 4]; 33],
}

impl Drop for SerpentKey {
    fn drop(&mut self) {
        for rk in &mut self.round_keys {
            for w in rk.iter_mut() {
                *w = 0;
            }
        }
    }
}

impl SerpentKey {
    /// Build a Serpent key.  Accepts any key length up to 32 bytes; if
    /// shorter than 32 bytes, the spec's padding (`0x01` then zero-pad
    /// to 256 bits) is applied.
    pub fn new(key: &[u8]) -> Result<Self, &'static str> {
        if key.len() > 32 {
            return Err("Serpent key must be at most 32 bytes");
        }

        // Step 1: pad to 256 bits with the canonical "1 then 0s" suffix.
        let mut padded = [0u8; 32];
        padded[..key.len()].copy_from_slice(key);
        if key.len() < 32 {
            padded[key.len()] = 0x01;
        }

        // Step 2: prekeys w_{-8} .. w_{131}, indexed in `prek` as 0..140.
        let mut prek = [0u32; 140];
        for i in 0..8 {
            prek[i] = u32::from_le_bytes(padded[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 8..140 {
            // The recurrence's index `j = i - 8` (i.e. `w_j`) is mixed in
            // via XOR.  Rotation is by 11 bits — fixed per step.
            let j = (i - 8) as u32;
            let v = prek[i - 8] ^ prek[i - 5] ^ prek[i - 3] ^ prek[i - 1] ^ PHI ^ j;
            prek[i] = v.rotate_left(11);
        }

        // Step 3: derive round keys K_0..K_32.  S-box used for K_i is
        // S_{(35 - i) mod 8}, applied bit-sliced.  No operand-dependent
        // path — just a fixed sequence of 33 `apply_sbox_bitslice`
        // calls with statically determined S-box indices.
        let mut round_keys = [[0u32; 4]; 33];
        for i in 0..33 {
            let s_idx = (35 - i) % 8;
            let mut nibble_state = [
                prek[8 + 4 * i],
                prek[8 + 4 * i + 1],
                prek[8 + 4 * i + 2],
                prek[8 + 4 * i + 3],
            ];
            apply_sbox_bitslice(&SBOX[s_idx], &mut nibble_state);
            round_keys[i] = nibble_state;
        }

        // Scrub the prekey scratch buffer; `padded` will be dropped at
        // function end and the compiler is free to reuse its stack
        // slot, but we still wipe it to reduce the residual window.
        for w in prek.iter_mut() {
            *w = 0;
        }

        Ok(SerpentKey { round_keys })
    }
}

// ── Block encryption / decryption ────────────────────────────────────────────

#[inline(always)]
fn xor_round_key(state: &mut [u32; 4], rk: &[u32; 4]) {
    state[0] ^= rk[0];
    state[1] ^= rk[1];
    state[2] ^= rk[2];
    state[3] ^= rk[3];
}

/// Encrypt one 16-byte block under `key`.  Pure block primitive — for
/// streams of multiple blocks, wrap in a mode (CTR, etc.).
pub fn serpent_encrypt(key: &SerpentKey, block: &[u8; 16]) -> [u8; 16] {
    let mut state = block_to_words(block);

    // Rounds 0..30: AddRoundKey → S → LT
    for i in 0..31 {
        xor_round_key(&mut state, &key.round_keys[i]);
        sbox(i, &mut state);
        lt(&mut state);
    }
    // Round 31 (final): AddRoundKey → S7 → AddRoundKey (no LT)
    xor_round_key(&mut state, &key.round_keys[31]);
    sbox(31, &mut state); // S_{31 mod 8} = S_7
    xor_round_key(&mut state, &key.round_keys[32]);

    words_to_block(&state)
}

/// Decrypt one 16-byte block under `key`.  Inverse of `serpent_encrypt`.
pub fn serpent_decrypt(key: &SerpentKey, block: &[u8; 16]) -> [u8; 16] {
    let mut state = block_to_words(block);

    // Reverse the final round.
    xor_round_key(&mut state, &key.round_keys[32]);
    sbox_inv(31, &mut state);
    xor_round_key(&mut state, &key.round_keys[31]);

    // Reverse rounds 30 down to 0.
    for i in (0..31).rev() {
        lt_inv(&mut state);
        sbox_inv(i, &mut state);
        xor_round_key(&mut state, &key.round_keys[i]);
    }

    words_to_block(&state)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex_block(s: &str) -> [u8; 16] {
        let bytes = hex::decode(s).unwrap();
        bytes.try_into().unwrap()
    }

    #[test]
    fn sbox_inverses_are_consistent() {
        // For every (s_idx, nibble), INV_SBOX[s_idx][SBOX[s_idx][n]] == n.
        for s in 0..8 {
            for n in 0..16 {
                let f = SBOX[s][n] as usize;
                assert_eq!(
                    INV_SBOX[s][f] as usize, n,
                    "SBOX{} inverse broken at {}",
                    s, n
                );
            }
        }
    }

    #[test]
    fn ct_sbox4_matches_direct_lookup() {
        // The CT scan must produce identical results to direct table indexing.
        for s in 0..8 {
            for n in 0..16u8 {
                assert_eq!(ct_sbox4(&SBOX[s], n), SBOX[s][n as usize]);
                assert_eq!(ct_sbox4(&INV_SBOX[s], n), INV_SBOX[s][n as usize]);
            }
        }
    }

    #[test]
    fn lt_is_invertible() {
        // Apply LT then LT^(-1); must recover the input.
        let mut state = [0xdeadbeefu32, 0xcafebabe, 0x12345678, 0xfedcba98];
        let original = state;
        lt(&mut state);
        assert_ne!(state, original, "LT must mix the state");
        lt_inv(&mut state);
        assert_eq!(state, original, "LT^(-1) must recover input");
    }

    #[test]
    fn round_trip_short_key() {
        let key = SerpentKey::new(b"0123456789abcdef").unwrap(); // 128-bit key, padded
        let pt = *b"sensitive block!";
        let ct = serpent_encrypt(&key, &pt);
        assert_ne!(ct, pt, "encryption is not identity");
        let recovered = serpent_decrypt(&key, &ct);
        assert_eq!(recovered, pt);
    }

    #[test]
    fn round_trip_full_key() {
        let key_bytes = [0x42u8; 32];
        let key = SerpentKey::new(&key_bytes).unwrap();
        let pt = [0xa5u8; 16];
        let ct = serpent_encrypt(&key, &pt);
        assert_ne!(ct, pt);
        assert_eq!(serpent_decrypt(&key, &ct), pt);
    }

    #[test]
    fn round_trip_zero_key_zero_block() {
        let key = SerpentKey::new(&[0u8; 32]).unwrap();
        let pt = [0u8; 16];
        let ct = serpent_encrypt(&key, &pt);
        assert_ne!(ct, pt, "zero-key, zero-block must not stay zero");
        assert_eq!(serpent_decrypt(&key, &ct), pt);
    }

    #[test]
    fn distinct_plaintexts_diverge() {
        let key = SerpentKey::new(b"sample key 32 bytes long......!!").unwrap();
        let pt1 = [0u8; 16];
        let mut pt2 = [0u8; 16];
        pt2[0] = 1; // single-bit difference
        let c1 = serpent_encrypt(&key, &pt1);
        let c2 = serpent_encrypt(&key, &pt2);
        assert_ne!(c1, c2);
        // Avalanche sanity: ≥ ~50% of bits should differ.  The lower
        // bound of 32 bits is well below the expected 64 and avoids
        // flakiness; one-bit-difference encryptions in any well-
        // designed cipher must change roughly half the output bits.
        let differing = c1
            .iter()
            .zip(c2.iter())
            .map(|(a, b)| (a ^ b).count_ones() as usize)
            .sum::<usize>();
        assert!(
            differing >= 32,
            "too few output bits flipped: {}",
            differing
        );
    }

    #[test]
    fn distinct_keys_diverge() {
        let pt = *b"plaintext block!";
        let k1 = SerpentKey::new(&[0u8; 32]).unwrap();
        let mut k2_bytes = [0u8; 32];
        k2_bytes[0] = 1; // single-bit different key
        let k2 = SerpentKey::new(&k2_bytes).unwrap();
        let c1 = serpent_encrypt(&k1, &pt);
        let c2 = serpent_encrypt(&k2, &pt);
        assert_ne!(c1, c2);
    }

    #[test]
    fn rejects_oversize_key() {
        assert!(SerpentKey::new(&[0u8; 33]).is_err());
        assert!(SerpentKey::new(&[0u8; 32]).is_ok());
        assert!(SerpentKey::new(&[]).is_ok()); // padding fills to 256 bits
    }

    /// Monte Carlo test, Cambridge Serpent submission tarball,
    /// `floppy4/ecb_e_m.txt`, entry `I=0` for each key size: the zero
    /// block encrypted 10,000 times in a chain under the zero key.  The
    /// file prints the block most significant byte first, so the expected
    /// value is compared with the byte-reversed block (module docs, "Byte
    /// order and test vectors").
    /// Source: https://www.cl.cam.ac.uk/~rja14/Papers/serpent.tar.gz
    fn monte_carlo_zero(key_len: usize, expect_msb_first: &str) {
        let key = SerpentKey::new(&vec![0u8; key_len]).unwrap();
        let mut block = [0u8; 16];
        for _ in 0..10_000 {
            block = serpent_encrypt(&key, &block);
        }
        let mut printed = block;
        printed.reverse();
        assert_eq!(
            printed,
            hex_block(expect_msb_first),
            "Serpent-{} Monte Carlo mismatch",
            key_len * 8
        );
        for _ in 0..10_000 {
            block = serpent_decrypt(&key, &block);
        }
        assert_eq!(block, [0u8; 16], "decryption does not undo the chain");
    }

    #[test]
    fn monte_carlo_serpent_128_zero() {
        monte_carlo_zero(16, "90e7a5ba9497fa1bfc00f7d1a3a86a1e");
    }

    #[test]
    fn monte_carlo_serpent_192_zero() {
        monte_carlo_zero(24, "2d8af7b79eb7f21fdb394c77c3fb8c3a");
    }

    #[test]
    fn monte_carlo_serpent_256_zero() {
        monte_carlo_zero(32, "92efa3ca9477794d31f4df7bce23e60a");
    }

    #[test]
    fn round_trip_many_random() {
        // Round-trip across a large number of random key/block pairs:
        // catches subtle bugs in the round structure (off-by-one in
        // round count, wrong S-box index, etc.) that single-vector
        // tests can miss.  Uses a deterministic LCG so failures
        // reproduce.
        let mut s: u64 = 0xdeadbeef_cafe_babe;
        for _ in 0..32 {
            let mut key = [0u8; 32];
            let mut pt = [0u8; 16];
            for b in &mut key {
                s = s
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                *b = (s >> 56) as u8;
            }
            for b in &mut pt {
                s = s
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                *b = (s >> 56) as u8;
            }
            let k = SerpentKey::new(&key).unwrap();
            let ct = serpent_encrypt(&k, &pt);
            assert_eq!(serpent_decrypt(&k, &ct), pt);
        }
    }
}
