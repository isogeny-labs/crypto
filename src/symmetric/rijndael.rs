//! **Rijndael** with variable block length, and **Rijndael-256** — the
//! 256-bit-block, 256-bit-key variant NIST proposed to standardize in
//! SP 800-197 (pre-draft call for comments, December 2024).
//!
//! AES (FIPS 197) is Rijndael restricted to a 128-bit block.  The original
//! Rijndael submission allows block and key lengths of 128, 160, 192, 224
//! or 256 bits, chosen independently.  [`Rijndael`] implements the whole
//! family; [`Rijndael256`] is the fixed 256/256 instance and implements
//! [`BlockCipher<32>`] so the generic modes in [`super::modes`] that are
//! written over `BlockCipher<N>` (ECB, CFB, OFB, …) work with it unchanged.
//! The 16-byte-only modes (GCM, CCM, SIV, XTS, …) do not apply.
//!
//! ## Parameters
//!
//! With `Nb` = block length / 32 and `Nk` = key length / 32:
//!
//! - rounds `Nr = max(Nb, Nk) + 6`, so Rijndael-256-256 has 14 rounds and
//!   15 round keys of 32 bytes each;
//! - the state is 4 rows × `Nb` columns, loaded column-major;
//! - **ShiftRows** offsets for rows 1, 2, 3 are `(1, 2, 3)` for
//!   `Nb ≤ 6`, `(1, 2, 4)` for `Nb = 7`, and **`(1, 3, 4)` for `Nb = 8`**;
//! - SubBytes, MixColumns and AddRoundKey are AES's, applied to every column;
//! - the key schedule is AES's (an extra SubWord at `i mod Nk = 4` when
//!   `Nk > 6`), run for `Nb·(Nr+1)` words, which needs round constants up
//!   to `x^13` (0x4d) rather than AES's ten.
//!
//! For a 256-bit key, Rijndael-256's first seven round keys are exactly the
//! fourteen AES-256 round keys, pairwise concatenated.
//!
//! ## Security status
//!
//! Comments on the SP 800-197 pre-draft report a 25 %+ margin in the
//! hash-mode and single-known-plaintext settings (Zhang–Chen–Guo–List–Shi)
//! and MILP bounds of at least 48 active S-boxes for 14-round related-key
//! differential trails (NLNCSA).  Avanzi et al. argue the 25-year-old key
//! schedule should be replaced before standardization.  Implementations
//! that reuse AES instructions need an extra cross-lane byte shuffle in
//! every round: Saarinen counts 2.66× the instructions of AES-256 on
//! RISC-V Zvkned.
//!
//! ## Timing
//!
//! S-box lookups go through AES's full-table constant-time scan and
//! MixColumns through the branchless `gmul`, as in [`super::aes`].
//! There is still no audit, so the library-wide warning applies.
//!
//! ## Test vectors
//!
//! - all 25 block/key combinations of Daemen & Rijmen, *The Design of
//!   Rijndael*, App. B.3 (zero key; `E(0)` then `E(E(0))`);
//! - Saarinen's Rijndael-256 vector (SP 800-38A AES-256 key, two blocks),
//!   produced with the 2001 Rijndael reference code;
//! - FIPS 197 Appendix C, through the 128-bit-block path.

use super::aes::{gmul, inv_sbox_ct, sbox_ct};
use super::modes::cipher::BlockCipher;

/// Rijndael with any block length and key length in
/// {16, 20, 24, 28, 32} bytes.
pub struct Rijndael {
    nb: usize,
    nr: usize,
    /// `Nr + 1` round keys, each `4·Nb` bytes, column-major.
    round_keys: Vec<u8>,
}

/// Errors from [`Rijndael::new`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RijndaelError {
    /// Key length not in {16, 20, 24, 28, 32} bytes.
    KeyLength(usize),
    /// Block length not in {16, 20, 24, 28, 32} bytes.
    BlockLength(usize),
}

impl std::fmt::Display for RijndaelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RijndaelError::KeyLength(n) => {
                write!(f, "Rijndael key must be 16/20/24/28/32 bytes, got {n}")
            }
            RijndaelError::BlockLength(n) => {
                write!(f, "Rijndael block must be 16/20/24/28/32 bytes, got {n}")
            }
        }
    }
}

impl std::error::Error for RijndaelError {}

fn valid_len(n: usize) -> bool {
    matches!(n, 16 | 20 | 24 | 28 | 32)
}

/// ShiftRows offsets (C1, C2, C3) for rows 1..=3.
fn shifts(nb: usize) -> [usize; 3] {
    match nb {
        7 => [1, 2, 4],
        8 => [1, 3, 4],
        _ => [1, 2, 3],
    }
}

fn xtime(a: u8) -> u8 {
    (a << 1) ^ (0x1b & 0u8.wrapping_sub(a >> 7))
}

impl Rijndael {
    /// Expand `key` for a cipher with `block_len`-byte blocks.
    pub fn new(key: &[u8], block_len: usize) -> Result<Self, RijndaelError> {
        if !valid_len(key.len()) {
            return Err(RijndaelError::KeyLength(key.len()));
        }
        if !valid_len(block_len) {
            return Err(RijndaelError::BlockLength(block_len));
        }
        let nb = block_len / 4;
        let nk = key.len() / 4;
        let nr = nb.max(nk) + 6;
        let total = nb * (nr + 1);
        let mut w: Vec<[u8; 4]> = Vec::with_capacity(total);
        for i in 0..nk {
            w.push([key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]]);
        }
        let mut rcon = 1u8;
        for i in nk..total {
            let mut t = w[i - 1];
            if i % nk == 0 {
                t = [sbox_ct(t[1]), sbox_ct(t[2]), sbox_ct(t[3]), sbox_ct(t[0])];
                t[0] ^= rcon;
                rcon = xtime(rcon);
            } else if nk > 6 && i % nk == 4 {
                t = t.map(sbox_ct);
            }
            let prev = w[i - nk];
            w.push([
                prev[0] ^ t[0],
                prev[1] ^ t[1],
                prev[2] ^ t[2],
                prev[3] ^ t[3],
            ]);
        }
        let round_keys = w.iter().flatten().copied().collect();
        {
            use zeroize::Zeroize;
            w.zeroize();
        }
        Ok(Rijndael { nb, nr, round_keys })
    }

    /// Block length in bytes.
    pub fn block_len(&self) -> usize {
        4 * self.nb
    }

    /// Number of rounds `Nr`.
    pub fn rounds(&self) -> usize {
        self.nr
    }

    fn add_round_key(&self, s: &mut [u8], r: usize) {
        let n = 4 * self.nb;
        for (b, k) in s.iter_mut().zip(&self.round_keys[r * n..(r + 1) * n]) {
            *b ^= k;
        }
    }

    fn shift_rows(&self, s: &mut [u8], inverse: bool) {
        let nb = self.nb;
        let mut out = [0u8; 32];
        out[..4 * nb].copy_from_slice(s);
        for (row, &off) in shifts(nb).iter().enumerate() {
            let r = row + 1;
            for c in 0..nb {
                let shifted = (c + off) % nb;
                if inverse {
                    out[4 * shifted + r] = s[4 * c + r];
                } else {
                    out[4 * c + r] = s[4 * shifted + r];
                }
            }
        }
        s.copy_from_slice(&out[..4 * nb]);
    }

    fn mix_columns(&self, s: &mut [u8], inverse: bool) {
        let m: [u8; 4] = if inverse {
            [14, 11, 13, 9]
        } else {
            [2, 3, 1, 1]
        };
        for col in s.chunks_exact_mut(4) {
            let a = [col[0], col[1], col[2], col[3]];
            for (r, out) in col.iter_mut().enumerate() {
                *out = gmul(a[r], m[0])
                    ^ gmul(a[(r + 1) % 4], m[1])
                    ^ gmul(a[(r + 2) % 4], m[2])
                    ^ gmul(a[(r + 3) % 4], m[3]);
            }
        }
    }

    /// Encrypt one block in place. Panics if `block.len() != self.block_len()`.
    pub fn encrypt(&self, block: &mut [u8]) {
        assert_eq!(block.len(), self.block_len(), "Rijndael block length");
        self.add_round_key(block, 0);
        for r in 1..=self.nr {
            for b in block.iter_mut() {
                *b = sbox_ct(*b);
            }
            self.shift_rows(block, false);
            if r != self.nr {
                self.mix_columns(block, false);
            }
            self.add_round_key(block, r);
        }
    }

    /// Decrypt one block in place. Panics if `block.len() != self.block_len()`.
    pub fn decrypt(&self, block: &mut [u8]) {
        assert_eq!(block.len(), self.block_len(), "Rijndael block length");
        for r in (1..=self.nr).rev() {
            self.add_round_key(block, r);
            if r != self.nr {
                self.mix_columns(block, true);
            }
            self.shift_rows(block, true);
            for b in block.iter_mut() {
                *b = inv_sbox_ct(*b);
            }
        }
        self.add_round_key(block, 0);
    }
}

impl Drop for Rijndael {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.round_keys.zeroize();
    }
}

/// Rijndael-256: 256-bit block, 256-bit key, 14 rounds (SP 800-197 proposal).
pub struct Rijndael256(Rijndael);

impl Rijndael256 {
    /// Expand a 32-byte key.
    pub fn new(key: &[u8; 32]) -> Self {
        Rijndael256(Rijndael::new(key, 32).expect("32/32 is a valid Rijndael size"))
    }
}

impl BlockCipher<32> for Rijndael256 {
    fn encrypt_block(&self, block: &mut [u8; 32]) {
        self.0.encrypt(block);
    }
    fn decrypt_block(&self, block: &mut [u8; 32]) {
        self.0.decrypt(block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symmetric::aes::{self, AesKey};

    fn h(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    /// Daemen & Rijmen, *The Design of Rijndael*, App. B.3:
    /// (block bits, key bits, E_0(0), E_0(E_0(0))) with an all-zero key.
    const BOOK_B3: &[(usize, usize, &str, &str)] = &[
        (
            128,
            128,
            "66E94BD4EF8A2C3B884CFA59CA342B2E",
            "F795BD4A52E29ED713D313FA20E98DBC",
        ),
        (
            160,
            128,
            "9E38B8EB1D2025A1665AD4B1F5438BB5CAE1AC3F",
            "939C167E7F916D45670EE21BFC939E1055054A96",
        ),
        (
            192,
            128,
            "A92732EB488D8BB98ECD8D95DC9C02E052F250AD369B3849",
            "106F34179C3982DDC6750AA01936B7A180E6B0B9D8D690EC",
        ),
        (
            224,
            128,
            "0623522D88F7B9C63437537157F625DD5697AB628A3B9BE2549895C8",
            "93F93CBDABE23415620E6990B0443D621F6AFBD6EDEFD6990A1965A8",
        ),
        (
            256,
            128,
            "A693B288DF7DAE5B1757640276439230DB77C4CD7A871E24D6162E54AF434891",
            "5F05857C80B68EA42CCBC759D42C28D5CD490F1D180C7A9397EE585BEA770391",
        ),
        (
            128,
            160,
            "94B434F8F57B9780F0EFF1A9EC4C112C",
            "35A00EC955DF43417CEAC2AB2B3F3E76",
        ),
        (
            160,
            160,
            "33B12AB81DB7972E8FDC529DDA46FCB529B31826",
            "97F03EB018C0BB9195BF37C6A0AECE8E4CB8DE5F",
        ),
        (
            192,
            160,
            "528E2FFF6005427B67BB1ED31ECC09A69EF41531DF5BA5B2",
            "71C7687A4C93EBC35601E3662256E10115BEED56A410D7AC",
        ),
        (
            224,
            160,
            "58A0C53F3822A32464704D409C2FD0521F3A93E1F6FCFD4C87F1C551",
            "D8E93EF2EB49857049D6F6E0F40B67516D2696F94013C065283F7F01",
        ),
        (
            256,
            160,
            "938D36E0CB6B7937841DAB7F1668E47B485D3ACD6B3F6D598B0A9F923823331D",
            "7B44491D1B24A93B904D171F074AD69669C2B70B134A4D2D773250A4414D78BE",
        ),
        (
            128,
            192,
            "AAE06992ACBF52A3E8F4A96EC9300BD7",
            "52F674B7B9030FDAB13D18DC214EB331",
        ),
        (
            160,
            192,
            "33060F9D4705DDD2C7675F0099140E5A98729257",
            "012CAB64982156A5710E790F85EC442CE13C520F",
        ),
        (
            192,
            192,
            "C6348BE20007BAC4A8BD62890C8147A2432E760E9A9F9AB8",
            "EB9DEF13C253F81C1FC2829426ED166A65A105C6A04CA33D",
        ),
        (
            224,
            192,
            "3856B17BEA77C4611E3397066828AADDA004706A2C8009DF40A811FE",
            "160AD76A97AE2C1E05942FDE3DA2962684A92CCC74B8DC23BDE4F469",
        ),
        (
            256,
            192,
            "F927363EF5B3B4984A9EB9109844152EC167F08102644E3F9028070433DF9F2A",
            "4E03389C68B2E3F623AD8F7F6BFC88613B86F334F4148029AE25F50DB144B80C",
        ),
        (
            128,
            224,
            "73F8DFF62A36F3EBF31D6F73A56FF279",
            "3A72F21E10B6473EA9FF14A232E675B4",
        ),
        (
            160,
            224,
            "E9F5EA0FA39BB6AD7339F28E58E2E7535F261827",
            "06EF9BC82905306D45810E12D0807796A3D338F9",
        ),
        (
            192,
            224,
            "ECBE9942CD6703E16D358A829D542456D71BD3408EB23C56",
            "FD10458ED034368A34047905165B78A6F0591FFEEBF47CC7",
        ),
        (
            224,
            224,
            "FE1CF0C8DDAD24E3D751933100E8E89B61CD5D31C96ABFF7209C495C",
            "515D8E2F2B9C5708F112C6DE31CACA47AFB86838B716975A24A09CD4",
        ),
        (
            256,
            224,
            "BC18BF6D369C955BBB271CBCDD66C368356DBA5B33C0005550D2320B1C617E21",
            "60ABA1D2BE45D8ABFDCF97BCB39F6C17DF29985CF321BAB75E26A26100AC00AF",
        ),
        (
            128,
            256,
            "DC95C078A2408989AD48A21492842087",
            "08C374848C228233C2B34F332BD2E9D3",
        ),
        (
            160,
            256,
            "30991844F72973B3B2161F1F11E7F8D9863C5118",
            "EEF8B7CC9DBE0F03A1FE9D82E9A759FD281C67E0",
        ),
        (
            192,
            256,
            "17004E806FAEF168FC9CD56F98F070982075C70C8132B945",
            "BED33B0AF364DBF15F9C2F3FB24FBDF1D36129C586EEA6B7",
        ),
        (
            224,
            256,
            "9BF26FAD5680D56B572067EC2FE162F449404C86303F8BE38FAB6E02",
            "658F144A34AF44AAE66CFDDAB955C483DFBCB4EE9A19A6701F158A66",
        ),
        (
            256,
            256,
            "C6227E7740B7E53B5CB77865278EAB0726F62366D9AABAD908936123A1FC8AF3",
            "9843E807319C32AD1EA3935EF56A2BA96E4BF19C30E47D88A2B97CBBF2E159E7",
        ),
    ];

    #[test]
    fn book_b3_all_sizes() {
        assert_eq!(BOOK_B3.len(), 25);
        for &(bl, kl, c0, c1) in BOOK_B3 {
            let c = Rijndael::new(&vec![0u8; kl / 8], bl / 8).unwrap();
            let mut b = vec![0u8; bl / 8];
            c.encrypt(&mut b);
            assert_eq!(b, h(c0), "E(0) block {bl} key {kl}");
            c.encrypt(&mut b);
            assert_eq!(b, h(c1), "E(E(0)) block {bl} key {kl}");
            c.decrypt(&mut b);
            c.decrypt(&mut b);
            assert_eq!(b, vec![0u8; bl / 8], "decrypt block {bl} key {kl}");
        }
    }

    #[test]
    fn rijndael256_reference_vector() {
        // M.-J. O. Saarinen, rij256-rv (2025): SP 800-38A AES-256 key and
        // plaintext, ciphertext from the 2001 Rijndael reference code.
        let key: [u8; 32] = h("603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4")
            .try_into()
            .unwrap();
        let pt = h(
            "6BC1BEE22E409F96E93D7E117393172AAE2D8A571E03AC9C9EB76FAC45AF8E51\
                    30C81C46A35CE411E5FBC1191A0A52EFF69F2445DF4F9B17AD2B417BE66C3710",
        );
        let ct = h(
            "66A8C19A07243A7CCED7ADED37E6651F485E9F79D293C439977D90D45A7DC029\
                    4B5FA2F3C2CA20AAECB912EB116C09BF9B7D109CE69EA39A67BE9BB1A3B0D415",
        );
        let c = Rijndael256::new(&key);
        for i in [0, 32] {
            let mut b: [u8; 32] = pt[i..i + 32].try_into().unwrap();
            c.encrypt_block(&mut b);
            assert_eq!(&b[..], &ct[i..i + 32]);
            c.decrypt_block(&mut b);
            assert_eq!(&b[..], &pt[i..i + 32]);
        }
        assert_eq!(c.0.rounds(), 14);
    }

    #[test]
    fn matches_fips197_aes() {
        let pt: [u8; 16] = h("00112233445566778899aabbccddeeff").try_into().unwrap();
        let key: [u8; 32] = h("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
            .try_into()
            .unwrap();
        let mut b = pt;
        Rijndael::new(&key, 16).unwrap().encrypt(&mut b);
        assert_eq!(b.to_vec(), h("8ea2b7ca516745bfeafc49904b496089"));
        assert_eq!(b, aes::encrypt_block(&pt, &AesKey::Aes256(key)));
    }

    #[test]
    fn first_round_keys_are_aes256_pairs() {
        let key: [u8; 32] = core::array::from_fn(|i| (i as u8).wrapping_mul(37) ^ 0x5a);
        let r = Rijndael::new(&key, 32).unwrap();
        let a = aes::key_expansion(&AesKey::Aes256(key));
        let aes_bytes: Vec<u8> = a.iter().flatten().copied().collect();
        assert_eq!(&r.round_keys[..7 * 32], &aes_bytes[..7 * 32]);
    }

    #[test]
    fn rejects_bad_sizes() {
        assert_eq!(
            Rijndael::new(&[0; 17], 32).err(),
            Some(RijndaelError::KeyLength(17))
        );
        assert_eq!(
            Rijndael::new(&[0; 32], 12).err(),
            Some(RijndaelError::BlockLength(12))
        );
    }

    #[test]
    fn ecb_mode_roundtrip() {
        use crate::symmetric::modes::ecb::{ecb_decrypt, ecb_encrypt};
        let c = Rijndael256::new(&[7u8; 32]);
        let msg = b"Rijndael-256 through the generic ECB mode over BlockCipher<32>";
        let ct = ecb_encrypt(&c, msg);
        assert_eq!(ct.len() % 32, 0);
        assert_eq!(ecb_decrypt(&c, &ct).unwrap(), msg.to_vec());
    }
}
