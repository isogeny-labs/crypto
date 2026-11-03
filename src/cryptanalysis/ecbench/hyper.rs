//! # Hyperelliptic Jacobians in ecbench: the thread's curves, genus 2 to 4
//!
//! The curves of the hyperelliptic index-calculus thread
//! ([`crate::cryptanalysis::hyperelliptic_ic_bench::thread_instance`]:
//! `y² = f(x)`, `deg f = 2g + 1`, over `F_p`, `p < 256`), wired into
//! ecbench so that every generic arm (`rho.plain`, `rho.negation`, BSGS,
//! the kangaroo) runs on the thread's Jacobians in the same counted unit
//! as on every other family.
//!
//! - **Group.**  [`HCurve`]: Cantor's algorithm on word-size Mumford
//!   divisors.  One composition-and-reduction is one group operation.
//! - **Subgroup.**  The thread's own rule: its first `c` and generator
//!   `D₁` of prime order `r`.  `#J = h·r` from [`HCurve::jacobian_order`].
//! - **Hex words.**  A divisor is two 64-bit words: word 0 is `deg u` in
//!   bits 0–7 and the non-leading coefficients of `u` in bytes 1–4,
//!   word 1 the coefficients of `v` in bytes 0–3.  The identity has no
//!   encoding, as the point at infinity has none on the other families.
//! - **Name.**  ICV1 has no hyperelliptic form, so the name keeps ICV1's
//!   shape with a `hyp` field tag, ends in 12 hex digits of a SHA-256 over
//!   the model `(p, g, f, #J, r)`, and is never registered.
//! - **Targets.**  Planted only: no public-target law yet.

use crate::cryptanalysis::hyperelliptic_ic_bench::thread_instance;
use crate::cryptanalysis::hyperelliptic_word::{HCurve, HDiv, MAX_GENUS};
use crate::cryptanalysis::residual_walk::{is_prime_u64, mix64};
use crate::hash::sha256::sha256;
use num_traits::ToPrimitive;

/// A built hyperelliptic instance.
#[derive(Clone, Debug)]
pub struct HyperInstance {
    pub p: u64,
    pub genus: u32,
    /// `f`, lowest degree first.
    pub f: Vec<u64>,
    pub group_order: u64,
    pub r: u64,
    pub cofactor: u64,
    pub generator: HDiv,
    pub curve: HCurve,
}

/// The exponent test's seed, a function of the model alone.
fn order_seed(p: u64, genus: u32, f: &[u64]) -> u64 {
    f.iter()
        .fold(mix64(p ^ (u64::from(genus) << 32)), |h, &c| mix64(h ^ c))
}

/// `(word 0, word 1)` of a divisor.
pub fn pack(d: &HDiv) -> (u64, u64) {
    let mut w0 = u64::from(d.deg);
    let mut w1 = 0u64;
    for i in 0..MAX_GENUS {
        w0 |= u64::from(d.u[i]) << (8 * (i + 1));
        w1 |= u64::from(d.v[i]) << (8 * i);
    }
    (w0, w1)
}

/// The divisor two words name, shape-checked only (validity is
/// [`HCurve::is_valid`]'s).
pub fn unpack(w0: u64, w1: u64) -> Option<HDiv> {
    if w0 >> (8 * (MAX_GENUS + 1)) != 0 || w1 >> (8 * MAX_GENUS) != 0 {
        return None;
    }
    let mut d = HDiv {
        deg: (w0 & 0xff) as u8,
        ..HDiv::IDENTITY
    };
    if d.deg as usize > MAX_GENUS {
        return None;
    }
    for i in 0..MAX_GENUS {
        d.u[i] = (w0 >> (8 * (i + 1))) as u8;
        d.v[i] = (w1 >> (8 * i)) as u8;
    }
    Some(d)
}

fn encode(d: &HDiv) -> Option<[String; 2]> {
    (!d.is_identity()).then(|| {
        let (a, b) = pack(d);
        [format!("0x{a:x}"), format!("0x{b:x}")]
    })
}

impl HyperInstance {
    /// The thread's instance on `(p, g)`: `thread_instance(p, g)`, its
    /// generator `D₁` of prime order `r`, and `#J` from point counts and
    /// the exponent test.
    pub fn build_thread(p: u64, genus: u32) -> Result<HyperInstance, String> {
        if !(2..=4).contains(&genus) {
            return Err(format!(
                "hyperelliptic_thread takes genus 2, 3 or 4, not {genus}"
            ));
        }
        if !(3..256).contains(&p) || !is_prime_u64(p) {
            return Err(format!(
                "hyperelliptic_thread takes an odd prime p < 256, not {p}"
            ));
        }
        let (big, d1, n, _c) = thread_instance(p, genus)
            .ok_or_else(|| format!("the thread rule finds no instance at p = {p}, g = {genus}"))?;
        let curve = HCurve::from_big(&big)?;
        let generator = curve
            .from_mumford(&d1)
            .ok_or("the thread generator is not a reduced word-size divisor")?;
        let r = n.to_u64().ok_or("the subgroup order passes a word")?;
        let f = curve.f_coeffs();
        let group_order = curve.jacobian_order(r, order_seed(p, genus, &f))?;
        Self::assemble(curve, f, group_order, r, generator)
    }

    /// An instance given outright; the group order is trusted beyond the
    /// checks (Hasse–Weil, `r | #J`, `[r]G = 0`, `[#J]D = 0` on eight
    /// random divisors), as `PrimeExplicit`'s is.
    pub fn build_explicit(
        p: u64,
        genus: u32,
        f: &[u64],
        group_order: u64,
        r: u64,
        gu: &[u64],
        gv: &[u64],
    ) -> Result<HyperInstance, String> {
        if !(2..=4).contains(&genus) {
            return Err(format!(
                "hyperelliptic_explicit takes genus 2, 3 or 4, not {genus}"
            ));
        }
        let curve = HCurve::new(p, genus, f)?;
        let generator = curve
            .from_coeffs(gu, gv)
            .ok_or("the generator is not a reduced divisor on the curve")?;
        let sp = (p as f64).sqrt();
        let lo = ((sp - 1.0).powi(2 * genus as i32) * (1.0 - 1e-9)).floor();
        let hi = ((sp + 1.0).powi(2 * genus as i32) * (1.0 + 1e-9)).ceil();
        if (group_order as f64) < lo || (group_order as f64) > hi {
            return Err(format!(
                "#J = {group_order} is outside the Hasse–Weil interval"
            ));
        }
        let mut state = order_seed(p, genus, f) ^ 0xe7;
        for _ in 0..8 {
            let d = curve.random_divisor(&mut state);
            if !curve.mul_uncounted(d, group_order).is_identity() {
                return Err(format!("#J = {group_order} does not kill a random divisor"));
            }
        }
        let f = curve.f_coeffs();
        Self::assemble(curve, f, group_order, r, generator)
    }

    fn assemble(
        curve: HCurve,
        f: Vec<u64>,
        group_order: u64,
        r: u64,
        generator: HDiv,
    ) -> Result<HyperInstance, String> {
        if r < 5 || !is_prime_u64(r) {
            return Err(format!("r = {r} is not a prime above 4"));
        }
        if !group_order.is_multiple_of(r) {
            return Err(format!("r = {r} does not divide #J = {group_order}"));
        }
        if generator.is_identity() || !curve.mul_uncounted(generator, r).is_identity() {
            return Err("the generator does not have order r".into());
        }
        Ok(HyperInstance {
            p: curve.p(),
            genus: curve.genus(),
            f,
            group_order,
            r,
            cofactor: group_order / r,
            generator,
            curve,
        })
    }

    pub fn r(&self) -> u64 {
        self.r
    }

    /// The generator's `u` (with its leading 1) and `v`, lowest first.
    pub fn generator_coeffs(&self) -> (Vec<u64>, Vec<u64>) {
        (self.generator.u_coeffs(), self.generator.v_coeffs())
    }

    pub fn generator_hex(&self) -> [String; 2] {
        encode(&self.generator).unwrap_or_default()
    }

    pub fn mul_generator_hex(&self, k: u64) -> Option<[String; 2]> {
        encode(&self.curve.mul_uncounted(self.generator, k % self.r))
    }

    /// The target the two words name: a reduced, non-identity divisor on
    /// the curve.
    pub fn decode(&self, w0: u64, w1: u64) -> Result<HDiv, String> {
        let d = unpack(w0, w1).ok_or("the words are not a packed divisor")?;
        if d.is_identity() {
            return Err("the target is the identity".into());
        }
        if !self.curve.is_valid(&d) {
            return Err("the target is not a reduced divisor on the curve".into());
        }
        Ok(d)
    }

    /// The canonical model JSON and its SHA-256, the last field of the name.
    pub fn model(&self) -> (String, String) {
        let f: Vec<String> = self.f.iter().map(u64::to_string).collect();
        let json = format!(
            "{{\"family\":\"hyperelliptic\",\"p\":{},\"genus\":{},\"f\":[{}],\"order\":{},\"r\":{}}}",
            self.p,
            self.genus,
            f.join(","),
            self.group_order,
            self.r
        );
        let h: String = sha256(json.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        (json, h)
    }

    /// `(icv1, slug)`.  ICV1 has no hyperelliptic form, so the name keeps
    /// ICV1's shape with a `hyp` field tag; it is never registered.
    pub fn names(&self) -> (String, String) {
        let (_, h) = self.model();
        (
            format!(
                "ICV1:hyp-g{}-p{}:-:{}:-:-:-:-:{}",
                self.genus,
                self.p,
                self.group_order,
                &h[..12]
            ),
            format!(
                "icv1-hypg{}p{}-n{}-{}",
                self.genus,
                self.p,
                self.group_order,
                &h[..8]
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisors_round_trip_through_the_hex_words_and_explicit_rebuilds() {
        for (g, p) in [(2u32, 41u64), (3, 23), (4, 17)] {
            let inst = HyperInstance::build_thread(p, g).unwrap();
            assert_eq!(inst.group_order % inst.r, 0);
            let hex = inst.mul_generator_hex(12345).unwrap();
            let w0 = u64::from_str_radix(hex[0].trim_start_matches("0x"), 16).unwrap();
            let w1 = u64::from_str_radix(hex[1].trim_start_matches("0x"), 16).unwrap();
            let d = inst.decode(w0, w1).unwrap();
            assert_eq!(encode(&d).unwrap(), hex);
            // A corrupted v is refused.
            assert!(inst.decode(w0, w1 ^ 1).is_err() || w1 ^ 1 == w1);
            assert!(inst.decode(w0 | 1 << 60, w1).is_err());
            let (icv1, slug) = inst.names();
            assert!(slug.starts_with("icv1-hyp"));
            let tail = icv1.rsplit(':').next().unwrap();
            assert_eq!(tail.len(), 12);
            assert!(u64::from_str_radix(tail, 16).is_ok());
            let (gu, gv) = inst.generator_coeffs();
            let again = HyperInstance::build_explicit(
                inst.p,
                inst.genus,
                &inst.f,
                inst.group_order,
                inst.r,
                &gu,
                &gv,
            )
            .unwrap();
            assert_eq!(again.names(), inst.names());
            assert_eq!(again.generator_hex(), inst.generator_hex());
            let wrong = |order: u64| {
                HyperInstance::build_explicit(inst.p, inst.genus, &inst.f, order, inst.r, &gu, &gv)
            };
            // r ∤ #J + 1.
            assert!(wrong(inst.group_order + 1).is_err());
            // #J + r passes the divisibility check; the exponent test
            // refuses it only when the cofactor part of J is non-trivial
            // (with h = 1 every multiple of r kills J: the order is
            // trusted there, as PrimeExplicit's is).
            if inst.cofactor > 1 {
                assert!(wrong(inst.group_order + inst.r).is_err());
            }
        }
    }
}
