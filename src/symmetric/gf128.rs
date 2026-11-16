//! Fixed-width operations in GF(2^128), reduced by x^128 + x^7 + x^2 + x + 1.
//!
//! The byte conventions are explicit: big-endian for CMAC/SIV/OCB/PMAC,
//! little-endian for XTS. These are integer-bit polynomial encodings, not
//! GHASH's reflected bit convention. Loops and memory access depend only on
//! the fixed width; reduction uses `subtle` selection, never a branch on a
//! key-derived carry. This source-level contract is not a hardware timing proof.

use subtle::{Choice, ConditionallySelectable};

/// Multiply by x with the most significant byte first.
pub fn double_be(input: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut carry = 0;
    for i in (0..16).rev() {
        out[i] = (input[i] << 1) | carry;
        carry = input[i] >> 7;
    }
    out[15] ^= u8::conditional_select(&0, &0x87, Choice::from(carry));
    out
}

/// Multiply by x^-1 with the most significant byte first.
pub fn halve_be(input: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut borrow = 0;
    for i in 0..16 {
        out[i] = (input[i] >> 1) | (borrow << 7);
        borrow = input[i] & 1;
    }
    let reduce = Choice::from(borrow);
    out[0] ^= u8::conditional_select(&0, &0x80, reduce);
    out[15] ^= u8::conditional_select(&0, &0x43, reduce);
    out
}

/// Multiply by x with the least significant byte first (IEEE 1619 XTS).
pub fn double_le(input: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut carry = 0;
    for i in 0..16 {
        out[i] = (input[i] << 1) | carry;
        carry = input[i] >> 7;
    }
    out[0] ^= u8::conditional_select(&0, &0x87, Choice::from(carry));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nist_sp800_38b_subkeys_include_carry_reduction() {
        let block = |hex: &str| -> [u8; 16] { hex::decode(hex).unwrap().try_into().unwrap() };
        let l = block("7df76b0c1ab899b33e42f047b91b546f");
        let k1 = block("fbeed618357133667c85e08f7236a8de");
        let k2 = block("f7ddac306ae266ccf90bc11ee46d513b");
        assert_eq!(double_be(&l), k1);
        assert_eq!(double_be(&k1), k2);
        assert_eq!(halve_be(&k1), l);
        assert_eq!(halve_be(&k2), k1);
    }

    #[test]
    fn all_polynomial_basis_elements_and_endian_conventions_agree() {
        for bit in 0..128 {
            let x = 1u128 << bit;
            // Independent polynomial reference over a single integer.
            let doubled = if bit == 127 { 0x87 } else { x << 1 };
            let halved = if bit == 0 {
                (1u128 << 127) | 0x43
            } else {
                x >> 1
            };
            assert_eq!(double_be(&x.to_be_bytes()), doubled.to_be_bytes());
            assert_eq!(double_le(&x.to_le_bytes()), doubled.to_le_bytes());
            assert_eq!(halve_be(&x.to_be_bytes()), halved.to_be_bytes());
        }
        for block in [[0; 16], [0xff; 16], [0xa5; 16], [0x80; 16]] {
            assert_eq!(halve_be(&double_be(&block)), block);
            assert_eq!(double_be(&halve_be(&block)), block);
        }
    }
}
