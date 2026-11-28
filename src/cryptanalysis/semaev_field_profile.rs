//! Canonical field identity for new binary Gröbner/Semaev measurements.
//!
//! The algebra implementations remain generic.  This module is the admission
//! boundary for comparable benchmark evidence: one profile fixes the field
//! degree, polynomial basis and curve instead of letting every harness choose
//! an integer `n` independently.

use std::sync::OnceLock;

use serde::Deserialize;

use crate::binary_ecc::IrreduciblePoly;
use crate::cryptanalysis::ic_boundary::BinaryInstance;
use crate::cryptanalysis::koblitz_index_calculus::KoblitzCurve;

pub const CANONICAL_SEMAEV_GROEBNER_PROFILE_ID: &str = "semaev-groebner-f2m17-v1";

const PROFILE_JSON: &str = include_str!("../../docs/ic/groebner-field-profile.json");

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BinaryFieldIdentity {
    pub characteristic: u32,
    pub degree: u32,
    pub representation: String,
    pub modulus_exponents: Vec<u32>,
    pub modulus_hex: String,
    pub field_uid: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CurveIdentity {
    pub family: String,
    pub a: u8,
    pub b: u64,
    pub icv1_slug: String,
    pub panel_alias: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GroebnerFieldProfile {
    pub schema: String,
    pub id: String,
    pub field: BinaryFieldIdentity,
    pub curve: CurveIdentity,
}

impl GroebnerFieldProfile {
    pub fn irreducible(&self) -> IrreduciblePoly {
        let mut low_terms: Vec<u32> = self
            .field
            .modulus_exponents
            .iter()
            .copied()
            .filter(|&exponent| exponent != self.field.degree)
            .collect();
        low_terms.sort_unstable();
        IrreduciblePoly {
            degree: self.field.degree,
            low_terms,
        }
    }

    pub fn modulus_u64(&self) -> u64 {
        assert!(self.field.degree < 64, "canonical modulus must fit u64");
        self.field
            .modulus_exponents
            .iter()
            .fold(0u64, |mask, &exponent| mask | (1u64 << exponent))
    }

    pub fn validate_field(&self, degree: u32, irreducible: &IrreduciblePoly) -> Result<(), String> {
        let expected = self.irreducible();
        if degree != self.field.degree
            || irreducible.degree != expected.degree
            || irreducible.low_terms != expected.low_terms
        {
            return Err(format!(
                "binary Gröbner/Semaev measurements use profile `{}`: GF(2^{}) with modulus {}; got degree {} and low terms {:?}",
                self.id,
                self.field.degree,
                self.field.modulus_hex,
                degree,
                irreducible.low_terms
            ));
        }
        Ok(())
    }

    pub fn validate_instance(&self, instance: &BinaryInstance) -> Result<(), String> {
        self.validate_field(instance.n, &instance.irreducible)?;
        let slug = instance.curve_id().slug;
        if slug != self.curve.icv1_slug {
            return Err(format!(
                "profile `{}` requires curve {} ({}), got {slug}",
                self.id, self.curve.panel_alias, self.curve.icv1_slug
            ));
        }
        Ok(())
    }

    pub fn canonical_curve(&self) -> Result<KoblitzCurve, String> {
        let curve = KoblitzCurve::new(self.curve.a, self.field.degree).ok_or_else(|| {
            format!(
                "cannot construct canonical curve {}",
                self.curve.panel_alias
            )
        })?;
        self.validate_field(curve.n, &curve.curve.irreducible)?;
        let slug = curve.label();
        if slug != self.curve.icv1_slug {
            return Err(format!(
                "canonical constructor produced {slug}, expected {}",
                self.curve.icv1_slug
            ));
        }
        Ok(curve)
    }
}

pub fn canonical_semaev_groebner_profile() -> &'static GroebnerFieldProfile {
    static PROFILE: OnceLock<GroebnerFieldProfile> = OnceLock::new();
    PROFILE.get_or_init(|| {
        let profile: GroebnerFieldProfile =
            serde_json::from_str(PROFILE_JSON).expect("valid canonical Gröbner field profile");
        assert_eq!(profile.schema, "groebner.field-profile/v1");
        assert_eq!(profile.id, CANONICAL_SEMAEV_GROEBNER_PROFILE_ID);
        assert_eq!(profile.field.characteristic, 2);
        assert_eq!(profile.field.representation, "polynomial");
        assert_eq!(profile.modulus_u64(), 0x20009);
        profile
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;

    #[test]
    fn canonical_profile_matches_the_registry_constructor() {
        let profile = canonical_semaev_groebner_profile();
        let found = find_irreducible_sparse(profile.field.degree).unwrap();
        profile
            .validate_field(profile.field.degree, &found)
            .unwrap();
        let curve = profile.canonical_curve().unwrap();
        assert_eq!(curve.label(), profile.curve.icv1_slug);
    }

    #[test]
    fn canonical_profile_rejects_degree_or_basis_drift() {
        let profile = canonical_semaev_groebner_profile();
        assert!(profile
            .validate_field(
                15,
                &IrreduciblePoly {
                    degree: 15,
                    low_terms: vec![0, 1],
                }
            )
            .is_err());
        assert!(profile
            .validate_field(
                17,
                &IrreduciblePoly {
                    degree: 17,
                    low_terms: vec![0, 5],
                }
            )
            .is_err());
    }
}
