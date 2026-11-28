//! Necessary quartic-pair screen over an admitted small odd prime field.
//! A product of two distinct irreducible quadratics has nonzero square
//! discriminant (Swan1962Corollary1, crediting Stickelberger). Positive
//! character still requires the complete bounded splitter/certificates.
use crate::cryptanalysis::{
    jv_bitops as work,
    jv_sieve::FpRing,
    residual_walk::{is_prime_u64, pow_mod},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Screen {
    pub resultant: u64,
    /// leading(f) * Res(f,f') = leading(f)^2 * disc(f).
    /// This represents the discriminant square class, not its exact value.
    pub discriminant_class: u64,
    pub character: i8,
}
impl Screen {
    pub fn rejects(&self) -> bool {
        self.character != 1
    }
}
/// Field admission occurs once per immutable context, charged to the caller.
/// Safe borrowing prevents a caller changing the ring modulus while admitted.
pub struct Context<'a> {
    ring: &'a FpRing,
}
impl<'a> Context<'a> {
    pub fn new(ring: &'a FpRing) -> Result<Self, &'static str> {
        if !(5..=1511).contains(&ring.p) || !is_prime_u64(ring.p) {
            return Err("small odd prime field required");
        }
        Ok(Self { ring })
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        work::hook_mul(self.ring.p);
        ((a as u128 * b as u128) % self.ring.p as u128) as u64
    }
    /// Standard resultant recurrence for A = QB + R:
    /// Res(A,B)=(-1)^(deg A deg B) lc(B)^(deg A-deg R) Res(B,R).
    /// Degrees strictly decrease; this degree-four case takes at most four
    /// divisions. Scalars/normalization/native control remain in the same
    /// incomplete accounting model as the existing bounded splitter.
    fn resultant(&self, f: &[u64], derivative: Vec<u64>) -> u64 {
        let (mut a, mut b) = (f.to_vec(), derivative);
        let mut result = 1;
        for _ in 0..5 {
            let (m, n) = (a.len() - 1, b.len() - 1);
            work::words(5);
            if n == 0 {
                return self.mul(result, pow_mod(b[0], m as u64, self.ring.p));
            }
            let (_, r) = self.ring.divrem(&a, &b);
            if r.is_empty() {
                return 0;
            }
            if (m * n) % 2 == 1 {
                work::hook_add(self.ring.p);
                result = if result == 0 { 0 } else { self.ring.p - result };
            }
            result = self.mul(
                result,
                pow_mod(b[n], (m - (r.len() - 1)) as u64, self.ring.p),
            );
            a = b;
            b = r;
        }
        unreachable!("strict quartic Euclidean degree descent")
    }
    pub fn screen(&self, tail: &[u64]) -> Result<Screen, &'static str> {
        if tail.len() != 5 || tail[4] == 0 || tail.iter().any(|&x| x >= self.ring.p) {
            return Err("canonical degree-four tail required");
        }
        let resultant = self.resultant(tail, self.ring.deriv(tail));
        let discriminant_class = self.mul(tail[4], resultant);
        let character = match pow_mod(discriminant_class, (self.ring.p - 1) / 2, self.ring.p) {
            0 => 0,
            1 => 1,
            x if x == self.ring.p - 1 => -1,
            _ => unreachable!("Euler criterion in admitted field"),
        };
        Ok(Screen {
            resultant,
            discriminant_class,
            character,
        })
    }
}
