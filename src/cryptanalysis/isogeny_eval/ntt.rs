//! Multi-modular NTT multiplication of polynomials over `F_p`, `p < 2^63`.
//!
//! A product `c = a·b` over `F_p` is computed as the integer product of the
//! canonical lifts (coefficients in `[0, p)`), modulo the three NTT primes
//! [`NTT_PRIMES`], by number-theoretic transforms; the residues are combined
//! by Garner's CRT and reduced mod `p`.
//!
//! **No overflow (proof).**  Every integer coefficient of the lifted product
//! is `c_k = Σ_{i+j=k} a_i b_j` with `0 ≤ a_i, b_j ≤ p − 1 < 2^63`, and has at
//! most `min(len a, len b)` terms, so `0 ≤ c_k < min(len a, len b)·2^126`.
//! The three primes all exceed `2^61`, so `Q = q₁q₂q₃ > 2^183` (in fact
//! `Q ≈ 2^184.4`).  Hence `c_k < Q` whenever `min(len a, len b) ≤ 2^57`; the
//! transform length is at most `2^54` (the smallest 2-adic valuation of
//! `q_i − 1`), so `len a + len b − 1 ≤ 2^54` and `min(len a, len b) ≤ 2^53`.
//! `c_k` is therefore the unique representative in `[0, Q)` of its residues,
//! Garner's mixed-radix form `c_k = r₁ + q₁ t₂ + q₁q₂ t₃` (`0 ≤ r₁ < q₁`,
//! `0 ≤ t₂ < q₂`, `0 ≤ t₃ < q₃`) recovers it exactly, and reducing that form
//! mod `p` gives the coefficient over `F_p`.  The final reduction evaluates
//! `r₁ + (q₁ mod p)·t₂ + (q₁q₂ mod p)·t₃` in a `u128`: each product is
//! `< 2^63·2^62`, so the sum is `< 2^127`.
//!
//! **Arithmetic modulo `q_i`.**  Montgomery multiplication with `R = 2^64`;
//! `q_i < 2^62`, so `a·b + m·q_i < 2^127` for `a, b < q_i` and the reduced
//! value is `< 2q_i`.  Twiddle factors are stored as `w·R mod q_i`, so a
//! butterfly multiplication keeps a residue in plain form; the one factor
//! `R^{−1}` from the pointwise product is removed, together with `1/N`, by a
//! final multiplication with `R²/N`.  Inputs `< p < 2^63 < 4q_i` are reduced
//! by at most three conditional subtractions, and Garner's cross-reductions
//! `r_i mod q_j` by one (every `q_i < 2q_j`), so no division is needed
//! outside the final `mod p`.
//!
//! **Counted cost (the round-4 unit).**  One *word multiplication* is a
//! `62 × 62 → 124`-bit product followed by a Montgomery reduction modulo an
//! NTT prime.  It is the same work as one `F_p` multiplication with
//! `p < 2^63` (a `64 × 64 → 128`-bit product and a reduction), so it is
//! **converted at one `M` per word multiplication** ([`NTT_WORD_COST_IN_M`]);
//! the counters keep words separate ([`super::counted_fp::OpTally::word`]),
//! so another ratio can be applied afterwards.  For a product of lengths
//! `la`, `lb`, with `n = la + lb − 1` and transform length
//! `N = 2^⌈log₂ n⌉`, [`mul_ntt`] counts exactly ([`ntt_mul_cost`])
//!
//! * words `= 3·(T·(N/2)·log₂N + N + n) + 3n`, where `T = 3` transforms
//!   (two forward, one inverse) or `T = 2` for a square, `N` pointwise
//!   products, `n` scalings by `R²/N` per prime, and three Garner products
//!   per output coefficient;
//! * `M = 2n` (`(q₁ mod p)·t₂` and `(q₁q₂ mod p)·t₃` per coefficient).
//!
//! Every butterfly multiplication is counted, including those by the twiddle
//! `1`.  Additions, subtractions and conditional subtractions are free, as
//! everywhere in this module.  Twiddle tables are constants of the three
//! primes (independent of `p` and of the inputs), built once per thread and
//! per transform size, and are not counted, as a hard-coded table would not
//! be.

use super::counted_fp::{OpCounter, PrimeField};
use std::cell::RefCell;
use std::sync::OnceLock;

/// Garner's constants `q₁^{−1} mod q₂`, `q₁^{−1} mod q₃`, `q₂^{−1} mod q₃`
/// in Montgomery form (constants of the primes, not counted).
static GARNER: OnceLock<[u64; 3]> = OnceLock::new();

/// The three NTT primes: `29·2^57 + 1`, `69·2^55 + 1`, `177·2^54 + 1`, all
/// in `(2^61, 2^62)`.
pub const NTT_PRIMES: [u64; 3] = [
    4_179_340_454_199_820_289,
    2_485_986_994_308_513_793,
    3_188_548_536_178_311_169,
];

/// A primitive root modulo each of [`NTT_PRIMES`].
const NTT_GENERATORS: [u64; 3] = [3, 5, 7];

/// The largest supported transform length is `2^MAX_LOG2_LEN` (the smallest
/// 2-adic valuation of `q_i − 1`).
pub const MAX_LOG2_LEN: u32 = 54;

/// How many `M` one word multiplication modulo an NTT prime is counted as
/// in [`super::counted_fp::OpTally::mul_equivalents`].
pub const NTT_WORD_COST_IN_M: u64 = 1;

/// Montgomery arithmetic modulo an odd `q < 2^62`, `R = 2^64`.
#[derive(Clone, Copy, Debug)]
struct Mont {
    q: u64,
    /// `−q^{−1} mod 2^64`.
    qneg_inv: u64,
    /// `R² mod q`.
    r2: u64,
}

impl Mont {
    fn new(q: u64) -> Self {
        let mut inv = q; // correct to 3 bits for odd q; Newton doubles them
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(q.wrapping_mul(inv)));
        }
        let r1 = ((1u128 << 64) % q as u128) as u64;
        let r2 = ((r1 as u128 * r1 as u128) % q as u128) as u64;
        Mont {
            q,
            qneg_inv: inv.wrapping_neg(),
            r2,
        }
    }

    /// `t·R^{−1} mod q` for `t < q·2^64`.
    #[inline(always)]
    fn redc(&self, t: u128) -> u64 {
        let m = (t as u64).wrapping_mul(self.qneg_inv);
        let u = ((t + m as u128 * self.q as u128) >> 64) as u64;
        if u >= self.q {
            u - self.q
        } else {
            u
        }
    }

    /// `a·b·R^{−1} mod q`.
    #[inline(always)]
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.redc(a as u128 * b as u128)
    }

    /// `a·R mod q`.
    fn enter_mont(&self, a: u64) -> u64 {
        self.mul(a % self.q, self.r2)
    }

    #[inline(always)]
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.q {
            s - self.q
        } else {
            s
        }
    }

    #[inline(always)]
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.q - b
        }
    }

    /// Plain `a^e mod q` (table construction only).
    fn pow_plain(&self, a: u64, mut e: u64) -> u64 {
        let q = self.q as u128;
        let mut r = 1u128;
        let mut b = a as u128 % q;
        while e > 0 {
            if e & 1 == 1 {
                r = r * b % q;
            }
            b = b * b % q;
            e >>= 1;
        }
        r as u64
    }
}

/// Twiddle tables for one prime: `fwd[s][j] = w_{2^{s+1}}^j·R`,
/// `inv[s][j] = w_{2^{s+1}}^{−j}·R`, `j < 2^s`; `scale[s] = R²/2^s`.
#[derive(Default)]
struct Tables {
    fwd: Vec<Vec<u64>>,
    inv: Vec<Vec<u64>>,
    scale: Vec<u64>,
}

impl Tables {
    fn ensure(&mut self, m: &Mont, g: u64, log2n: u32) {
        while self.scale.len() <= log2n as usize {
            let k = self.scale.len() as u32;
            // scale[k] = R²·2^{−k}
            let inv2k = m.pow_plain(m.pow_plain(2, k as u64), m.q - 2);
            self.scale.push(m.enter_mont(m.enter_mont(inv2k)));
        }
        while self.fwd.len() < log2n as usize {
            let s = self.fwd.len() as u32;
            let h = 1usize << s;
            // primitive 2h-th root of unity
            let w = m.pow_plain(g, (m.q - 1) >> (s + 1));
            let wi = m.pow_plain(w, m.q - 2);
            let (mut f, mut i) = (1u64, 1u64);
            let (mut fv, mut iv) = (Vec::with_capacity(h), Vec::with_capacity(h));
            for _ in 0..h {
                fv.push(m.enter_mont(f));
                iv.push(m.enter_mont(i));
                f = ((f as u128 * w as u128) % m.q as u128) as u64;
                i = ((i as u128 * wi as u128) % m.q as u128) as u64;
            }
            self.fwd.push(fv);
            self.inv.push(iv);
        }
    }
}

thread_local! {
    static TABLES: RefCell<[Tables; 3]> = RefCell::new(Default::default());
}

fn monts() -> [Mont; 3] {
    [
        Mont::new(NTT_PRIMES[0]),
        Mont::new(NTT_PRIMES[1]),
        Mont::new(NTT_PRIMES[2]),
    ]
}

/// Forward transform (Gentleman–Sande, natural order in, bit-reversed out).
fn forward(m: &Mont, t: &Tables, a: &mut [u64]) {
    let n = a.len();
    let mut h = n / 2;
    let mut s = n.trailing_zeros() as usize;
    while h >= 1 {
        s -= 1;
        let w = &t.fwd[s];
        for block in a.chunks_exact_mut(2 * h) {
            let (lo, hi) = block.split_at_mut(h);
            for ((x, y), &wj) in lo.iter_mut().zip(hi.iter_mut()).zip(w) {
                let u = *x;
                let v = *y;
                *x = m.add(u, v);
                *y = m.mul(m.sub(u, v), wj);
            }
        }
        h /= 2;
    }
}

/// Inverse transform without the `1/N` (Cooley–Tukey, bit-reversed in,
/// natural order out).
fn inverse(m: &Mont, t: &Tables, a: &mut [u64]) {
    let n = a.len();
    let mut h = 1;
    let mut s = 0;
    while h < n {
        let w = &t.inv[s];
        for block in a.chunks_exact_mut(2 * h) {
            let (lo, hi) = block.split_at_mut(h);
            for ((x, y), &wj) in lo.iter_mut().zip(hi.iter_mut()).zip(w) {
                let u = *x;
                let v = m.mul(*y, wj);
                *x = m.add(u, v);
                *y = m.sub(u, v);
            }
        }
        h *= 2;
        s += 1;
    }
}

/// `x mod q` for `x < 4q` by conditional subtractions.
#[inline(always)]
fn reduce_small(x: u64, q: u64) -> u64 {
    let mut x = x;
    while x >= q {
        x -= q;
    }
    x
}

/// The counted cost of [`mul_ntt`] for operand lengths `la`, `lb ≥ 1`:
/// `(word multiplications, F_p multiplications)`.  See the module
/// documentation for the formula.
pub fn ntt_mul_cost(la: usize, lb: usize, square: bool) -> (u64, u64) {
    if la == 0 || lb == 0 {
        return (0, 0);
    }
    let n = (la + lb - 1) as u64;
    let big_n = n.next_power_of_two();
    let log2n = big_n.trailing_zeros() as u64;
    let transforms = if square { 2 } else { 3 };
    let words = 3 * (transforms * (big_n / 2) * log2n + big_n + n) + 3 * n;
    (words, 2 * n)
}

/// `a·b` over `F_p` by the three-prime NTT.  The result has length
/// `len(a) + len(b) − 1` (empty if either is empty), like
/// [`super::poly::mul`].  If `a` and `b` are the same slice the product is
/// computed as a square (two transforms per prime instead of three).
///
/// # Panics
///
/// If `len(a) + len(b) − 1 > 2^54`.
pub fn mul_ntt<C: OpCounter>(f: &PrimeField<C>, a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let square = std::ptr::eq(a, b);
    let n = a.len() + b.len() - 1;
    let big_n = n.next_power_of_two();
    let log2n = big_n.trailing_zeros();
    assert!(log2n <= MAX_LOG2_LEN, "NTT length beyond 2^54");
    let (words, muls) = ntt_mul_cost(a.len(), b.len(), square);
    f.counter().count_word_mul(words);
    f.count_muls(muls);
    let ms = monts();
    let mut res: [Vec<u64>; 3] = Default::default();
    TABLES.with(|cell| {
        let mut tabs = cell.borrow_mut();
        for (i, m) in ms.iter().enumerate() {
            tabs[i].ensure(m, NTT_GENERATORS[i], log2n);
            let t = &tabs[i];
            let mut fa = vec![0u64; big_n];
            for (d, &s) in fa.iter_mut().zip(a) {
                *d = reduce_small(s, m.q);
            }
            forward(m, t, &mut fa);
            if square {
                for v in fa.iter_mut() {
                    *v = m.mul(*v, *v);
                }
            } else {
                let mut fb = vec![0u64; big_n];
                for (d, &s) in fb.iter_mut().zip(b) {
                    *d = reduce_small(s, m.q);
                }
                forward(m, t, &mut fb);
                for (x, &y) in fa.iter_mut().zip(&fb) {
                    *x = m.mul(*x, y);
                }
            }
            inverse(m, t, &mut fa);
            let sc = t.scale[log2n as usize];
            fa.truncate(n);
            for v in fa.iter_mut() {
                *v = m.mul(*v, sc);
            }
            res[i] = fa;
        }
    });
    // Garner constants (plain residues, converted to Montgomery form so
    // that one Montgomery product multiplies by the plain constant).
    let [m1, m2, m3] = ms;
    let (q1, q2) = (m1.q, m2.q);
    let [c12, c13, c23] = *GARNER.get_or_init(|| {
        [
            m2.enter_mont(m2.pow_plain(q1, m2.q - 2)), // q1^{-1} mod q2
            m3.enter_mont(m3.pow_plain(q1, m3.q - 2)), // q1^{-1} mod q3
            m3.enter_mont(m3.pow_plain(q2, m3.q - 2)), // q2^{-1} mod q3
        ]
    });
    let p = f.p() as u128;
    let q1p = (q1 as u128 % p) as u64;
    let q12p = ((q1 as u128 % p) * (q2 as u128 % p) % p) as u64;
    let [r1s, r2s, r3s] = &res;
    r1s.iter()
        .zip(r2s)
        .zip(r3s)
        .map(|((&r1, &r2), &r3)| {
            let t2 = m2.mul(m2.sub(r2, reduce_small(r1, q2)), c12);
            let x3 = m3.mul(m3.sub(r3, reduce_small(r1, m3.q)), c13);
            let t3 = m3.mul(m3.sub(x3, reduce_small(t2, m3.q)), c23);
            let v = r1 as u128 + q1p as u128 * t2 as u128 + q12p as u128 * t3 as u128;
            (v % p) as u64
        })
        .collect()
}
