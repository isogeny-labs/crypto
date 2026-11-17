//! # Curves over `F_{p²}` and `F_{p³}` that carry an efficient endomorphism
//!
//! Two odd-characteristic families whose strongest generic attack is not
//! negation rho, wired into ecbench so that every rho arm on them, and
//! the folded one in particular, is charged in the same counted unit
//! (AGENTS.md §12, and the validation gate's "strongest available" rho):
//!
//! - **`gls_fp2`**: the GLS twist over `F_{p²}` of a curve over `F_p`
//!   (`gls_fp2::generate_gls_instance`), with `ψ`, `ψ² = −1` on the
//!   subgroup: `rho.gls` walks on `{±ψ^i P}` = `{ψ^i P}`, `A = 4`.
//! - **`subfield_fp3`**: a curve over `F_p` on its group over `F_{p³}`
//!   (`subfield_fp3::generate_subfield_instance`, `j ≠ 0`), the prime
//!   subgroup outside `E(F_p)`, with the `p`-power Frobenius of order 3
//!   on it: `rho.frobenius_orbit` walks on `{±π^i P}`, `A = 6`.
//!
//! A coordinate is packed as `Σ cᵢ pⁱ` (`ExtField::pack`), `p^k < 2^62`,
//! so the harness's hex targets, verification and replays carry a point
//! unchanged.  The curve's ICV1 identity is `curve_id::extension` over the
//! field's binomial modulus.

use num_bigint::BigUint;

use crate::cryptanalysis::curve_id::{self, CurveId};
use crate::cryptanalysis::ext_curve::{ExtCurve, ExtField, ExtPoint};
use crate::cryptanalysis::gls_fp2::{generate_gls_instance, GlsInstance};
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use crate::cryptanalysis::subfield_fp3::{generate_subfield_instance, SubfieldInstance};

/// A built curve of one of the two families.
#[derive(Clone, Debug)]
pub enum ExtInstance {
    Gls(GlsInstance),
    Subfield(SubfieldInstance),
}

/// `Σ cᵢ pⁱ` back to coordinates, or `None` if the word is past `p^k`.
fn unpack<F: ExtField>(f: &F, mut v: u64) -> Option<F::El> {
    let p = f.p();
    let mut c = Vec::with_capacity(f.degree() as usize);
    for _ in 0..f.degree() {
        c.push(v % p);
        v /= p;
    }
    (v == 0).then(|| f.from_coords(&c))
}

fn encode<F: ExtField>(f: &F, pt: &ExtPoint<F::El>) -> Option<[String; 2]> {
    (!pt.infinity).then(|| {
        [
            format!("0x{:x}", f.pack(pt.x)),
            format!("0x{:x}", f.pack(pt.y)),
        ]
    })
}

fn decode<F: ExtField>(c: &ExtCurve<F>, x: u64, y: u64) -> Result<ExtPoint<F::El>, String> {
    let (Some(x), Some(y)) = (unpack(&c.f, x), unpack(&c.f, y)) else {
        return Err("a coordinate is not a packed field element".into());
    };
    let pt = ExtPoint::affine(x, y);
    if !c.is_on_curve(pt) {
        return Err("the target is not on the curve".into());
    }
    Ok(pt)
}

fn identity<F: ExtField>(c: &ExtCurve<F>, nu: u64, order: u64) -> CurveId {
    let p = c.f.p();
    let mut modulus = vec![0u64; c.f.degree() as usize];
    modulus[0] = (p - nu % p) % p;
    curve_id::extension(
        p,
        &modulus,
        &c.f.coords(c.a),
        &c.f.coords(c.b),
        &BigUint::from(order),
    )
    .expect("a constructed curve is non-singular and inside the Hasse interval")
}

/// The public-target law on `E(F_{p^k})`: the draw's base-`p` digits
/// are the abscissa, its top bit the ordinate's sign by key parity, and
/// the cofactor is cleared.
fn hash_point<F: ExtField>(c: &ExtCurve<F>, h: u64, cofactor: u64) -> Option<ExtPoint<F::El>> {
    let q = c.f.order();
    let x = unpack(&c.f, (h & (u64::MAX >> 1)) % q)?;
    let pts = c.lift_x(x);
    let flip = h >> 63 == 1;
    let pt = *pts.get(usize::from(flip && pts.len() > 1))?;
    let mut ops = GroupOps::default();
    let pt = c.mul(&mut ops, pt, cofactor);
    (!pt.infinity).then_some(pt)
}

impl ExtInstance {
    pub fn build_gls(p_bits: u32, seed: u64, max_cofactor: u64) -> Result<Self, String> {
        generate_gls_instance(p_bits, seed, max_cofactor).map(Self::Gls)
    }

    pub fn build_subfield(p_bits: u32, seed: u64, max_cofactor_ratio: u64) -> Result<Self, String> {
        if p_bits > 20 {
            return Err(format!(
                "subfield_fp3 takes p_bits ≤ 20 (p³ < 2^62), not {p_bits}"
            ));
        }
        generate_subfield_instance(p_bits, seed, false, max_cofactor_ratio).map(Self::Subfield)
    }

    pub fn r(&self) -> u64 {
        match self {
            Self::Gls(i) => i.r,
            Self::Subfield(i) => i.r,
        }
    }

    pub fn p(&self) -> u64 {
        match self {
            Self::Gls(i) => i.p,
            Self::Subfield(i) => i.p,
        }
    }

    pub fn group_order(&self) -> u64 {
        match self {
            Self::Gls(i) => i.group_order,
            Self::Subfield(i) => i.group_order,
        }
    }

    pub fn cofactor(&self) -> u64 {
        match self {
            Self::Gls(i) => i.cofactor,
            Self::Subfield(i) => i.cofactor,
        }
    }

    /// The class size of the strongest walk: `ψ` (order 4, contains
    /// `−1`) on a GLS curve, `±π^i` (`2·3`) on a subfield curve.
    pub fn automorphisms_available(&self) -> u32 {
        match self {
            Self::Gls(_) => 4,
            Self::Subfield(_) => 6,
        }
    }

    pub fn id(&self) -> CurveId {
        match self {
            Self::Gls(i) => identity(&i.curve, i.curve.f.nu, i.group_order),
            Self::Subfield(i) => identity(&i.curve, i.curve.f.nu, i.group_order),
        }
    }

    pub fn generator_hex(&self) -> [String; 2] {
        match self {
            Self::Gls(i) => encode(&i.curve.f, &i.generator),
            Self::Subfield(i) => encode(&i.curve.f, &i.generator),
        }
        .unwrap_or_default()
    }

    pub fn mul_generator_hex(&self, k: u64) -> Option<[String; 2]> {
        let mut ops = GroupOps::default();
        match self {
            Self::Gls(i) => encode(&i.curve.f, &i.curve.mul(&mut ops, i.generator, k % i.r)),
            Self::Subfield(i) => encode(&i.curve.f, &i.curve.mul(&mut ops, i.generator, k % i.r)),
        }
    }

    pub fn hash_to_subgroup_hex(&self, h: u64) -> Option<[String; 2]> {
        match self {
            Self::Gls(i) => encode(&i.curve.f, &hash_point(&i.curve, h, i.cofactor)?),
            Self::Subfield(i) => encode(&i.curve.f, &hash_point(&i.curve, h, i.cofactor)?),
        }
    }

    /// The decoded target, checked to lie on the curve, for a method.
    pub fn decode_gls(i: &GlsInstance, x: u64, y: u64) -> Result<ExtPoint<[u64; 2]>, String> {
        decode(&i.curve, x, y)
    }

    pub fn decode_subfield(
        i: &SubfieldInstance,
        x: u64,
        y: u64,
    ) -> Result<ExtPoint<[u64; 3]>, String> {
        decode(&i.curve, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_round_trip_through_the_hex_words() {
        for inst in [
            ExtInstance::build_gls(12, 1, 64).unwrap(),
            ExtInstance::build_subfield(10, 1, 64).unwrap(),
        ] {
            let hex = inst.mul_generator_hex(12345).unwrap();
            let x = u64::from_str_radix(hex[0].trim_start_matches("0x"), 16).unwrap();
            let y = u64::from_str_radix(hex[1].trim_start_matches("0x"), 16).unwrap();
            match &inst {
                ExtInstance::Gls(i) => {
                    let pt = ExtInstance::decode_gls(i, x, y).unwrap();
                    assert_eq!(encode(&i.curve.f, &pt).unwrap(), hex);
                }
                ExtInstance::Subfield(i) => {
                    let pt = ExtInstance::decode_subfield(i, x, y).unwrap();
                    assert_eq!(encode(&i.curve.f, &pt).unwrap(), hex);
                }
            }
            assert!(inst.id().slug.starts_with("icv1-"));
            // About half the draws lift to a point; the law takes the first that does.
            let t = (0..64u64)
                .find_map(|c| {
                    inst.hash_to_subgroup_hex(
                        0xdead_beef_1234_5678 ^ c.wrapping_mul(0x9e37_79b9_7f4a_7c15),
                    )
                })
                .expect("one of 64 draws lifts");
            assert_ne!(t, inst.generator_hex());
        }
    }
}
