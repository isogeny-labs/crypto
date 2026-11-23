/// `h mod d` for a fixed `d ≥ 1` without a division instruction: Lemire,
/// Kaser and Kurz, "Faster remainder by direct computation" (2019), the
/// 64-bit case with the 128-bit constant `M = ⌈2^128 / d⌉`, which is exact
/// for every 64-bit `h` and every `d < 2^64` (their Theorem 1 with
/// `F = 128 ≥ N + log₂ d`).  `d = 1` wraps `M` to 0 and returns 0.  The
/// kangaroo's jump index was a 64-bit `div` per step; the value is the
/// same remainder.
#[derive(Clone, Copy)]
struct FastMod {
    m: u128,
    d: u64,
}

impl FastMod {
    fn new(d: u64) -> Self {
        assert!(d > 0, "FastMod of a zero modulus");
        Self {
            m: (u128::MAX / d as u128).wrapping_add(1),
            d,
        }
    }

    #[inline]
    fn rem(self, h: u64) -> u64 {
        let low = self.m.wrapping_mul(h as u128);
        let bottom = ((low as u64) as u128 * self.d as u128) >> 64;
        let top = (low >> 64) * self.d as u128;
        ((bottom + top) >> 64) as u64
    }
}
