//! Reference cipher implementations used to generate samples, checked against published test vectors.
//! These are *data generators* for distinguisher experiments: not constant-time, not for protecting data.

// ------------------------------------------------------------------ Speck32/64 (Beaulieu et al., 2013)

const ALPHA: u32 = 7;
const BETA: u32 = 2;

#[inline]
fn speck_round(x: u16, y: u16, k: u16) -> (u16, u16) {
    let x = x.rotate_right(ALPHA).wrapping_add(y) ^ k;
    let y = y.rotate_left(BETA) ^ x;
    (x, y)
}

/// Key schedule for Speck32/64: key words (l2, l1, l0, k0) as in the specification's test vector order.
pub fn speck_expand(key: [u16; 4], rounds: usize) -> Vec<u16> {
    let mut ks = Vec::with_capacity(rounds);
    let mut k = key[3];
    let mut l = vec![key[2], key[1], key[0]];
    for i in 0..rounds {
        ks.push(k);
        let (nl, nk) = speck_round(l[i], k, i as u16);
        l.push(nl);
        k = nk;
    }
    ks
}

pub fn speck_encrypt(pt: (u16, u16), ks: &[u16]) -> (u16, u16) {
    let (mut x, mut y) = pt;
    for &k in ks {
        (x, y) = speck_round(x, y, k);
    }
    (x, y)
}

// ------------------------------------------------------------------ AES-128 with a configurable round count (FIPS 197)

const SBOX: [u8; 256] = {
    // generated at compile time from the multiplicative inverse in GF(2^8) and the affine map
    let mut s = [0u8; 256];
    let mut p: u8 = 1;
    let mut q: u8 = 1;
    loop {
        // p *= 3
        p = p ^ (p << 1) ^ (if p & 0x80 != 0 { 0x1b } else { 0 });
        // q /= 3
        q ^= q << 1;
        q ^= q << 2;
        q ^= q << 4;
        if q & 0x80 != 0 {
            q ^= 0x09;
        }
        let x = q ^ q.rotate_left(1) ^ q.rotate_left(2) ^ q.rotate_left(3) ^ q.rotate_left(4);
        s[p as usize] = x ^ 0x63;
        if p == 1 {
            break;
        }
    }
    s[0] = 0x63;
    s
};

#[inline]
fn xtime(b: u8) -> u8 {
    (b << 1) ^ (if b & 0x80 != 0 { 0x1b } else { 0 })
}

pub fn aes_expand(key: [u8; 16], rounds: usize) -> Vec<[u8; 16]> {
    let mut w: Vec<[u8; 4]> = (0..4).map(|i| [key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]]).collect();
    let mut rcon = 1u8;
    for i in 4..4 * (rounds + 1) {
        let mut t = w[i - 1];
        if i % 4 == 0 {
            t = [SBOX[t[1] as usize] ^ rcon, SBOX[t[2] as usize], SBOX[t[3] as usize], SBOX[t[0] as usize]];
            rcon = xtime(rcon);
        }
        w.push([w[i - 4][0] ^ t[0], w[i - 4][1] ^ t[1], w[i - 4][2] ^ t[2], w[i - 4][3] ^ t[3]]);
    }
    (0..=rounds).map(|r| {
        let mut k = [0u8; 16];
        for c in 0..4 {
            k[4 * c..4 * c + 4].copy_from_slice(&w[4 * r + c]);
        }
        k
    }).collect()
}

/// AES with `rounds` rounds: initial AddRoundKey, `rounds - 1` full rounds, final round without MixColumns.
/// `rounds = 10` is AES-128 as standardised.
pub fn aes_encrypt(pt: [u8; 16], rk: &[[u8; 16]]) -> [u8; 16] {
    let rounds = rk.len() - 1;
    let mut s = pt;
    for i in 0..16 {
        s[i] ^= rk[0][i];
    }
    for r in 1..=rounds {
        for b in s.iter_mut() {
            *b = SBOX[*b as usize];
        }
        // ShiftRows (column-major state)
        let t = s;
        for c in 0..4 {
            for row in 0..4 {
                s[4 * c + row] = t[4 * ((c + row) % 4) + row];
            }
        }
        if r != rounds {
            for c in 0..4 {
                let a = [s[4 * c], s[4 * c + 1], s[4 * c + 2], s[4 * c + 3]];
                let all = a[0] ^ a[1] ^ a[2] ^ a[3];
                for row in 0..4 {
                    s[4 * c + row] = a[row] ^ all ^ xtime(a[row] ^ a[(row + 1) % 4]);
                }
            }
        }
        for i in 0..16 {
            s[i] ^= rk[r][i];
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speck32_64_test_vector() {
        // Speck paper, Appendix C: key 1918 1110 0908 0100, plaintext 6574 694c, ciphertext a868 42f2
        let ks = speck_expand([0x1918, 0x1110, 0x0908, 0x0100], 22);
        assert_eq!(speck_encrypt((0x6574, 0x694c), &ks), (0xa868, 0x42f2));
    }

    #[test]
    fn aes128_fips197_vector() {
        let key = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];
        let pt = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
        let ct = [0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a];
        assert_eq!(aes_encrypt(pt, &aes_expand(key, 10)), ct);
        assert_eq!(SBOX[0x53], 0xed);
    }
}
