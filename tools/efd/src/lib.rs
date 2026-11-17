//! Small native check target compiling the actual library modules in place.
//! This also works in a sparse checkout without unrelated research fixtures.
#[path = "../../../src/binary_ecc/curve.rs"]
pub mod curve;
#[path = "../../../src/binary_ecc/efd/mod.rs"]
pub mod efd;
#[path = "../../../src/binary_ecc/f2m.rs"]
pub mod f2m;
#[path = "../../../src/binary_ecc/operation_tally.rs"]
pub mod operation_tally;

/// The check target does not pull in the cryptanalysis ledger; the curve
/// module only needs the counter entry points to type-check.
#[allow(dead_code)]
pub mod cryptanalysis {
    pub mod group_op_count {
        pub fn count_adds(_count: u64) {}
        pub fn count_doubles(_count: u64) {}
        pub fn count_scalar_mults(_count: u64) {}
    }
}

/// Published ECC2-131 parameters, duplicated so the check target does not pull
/// in the prime-field curve modules. Keep in sync with `src/ecc/certicom.rs`.
pub mod ecc {
    pub mod certicom {
        use crate::curve::{BinaryCurve, BinaryPoint};
        use crate::f2m::{F2mElement, IrreduciblePoly};
        use num_bigint::BigUint;

        fn hex(value: &str) -> BigUint {
            BigUint::parse_bytes(value.as_bytes(), 16).expect("published Certicom hex constant")
        }

        pub fn binary_challenge(name: &str) -> Option<(BinaryCurve, BinaryPoint)> {
            if !name.eq_ignore_ascii_case("ecc2-131") && !name.eq_ignore_ascii_case("ecc2-130") {
                return None;
            }
            let point = |x: &str, y: &str| BinaryPoint::Affine {
                x: F2mElement::from_hex(x, 131),
                y: F2mElement::from_hex(y, 131),
            };
            Some((
                BinaryCurve {
                    m: 131,
                    irreducible: IrreduciblePoly {
                        degree: 131,
                        low_terms: vec![0, 1, 2, 13],
                    },
                    a: F2mElement::from_hex("07EBCB7EECC296A1C4A1A14F2C9E44352E", 131),
                    b: F2mElement::from_hex("00610B0A57C73649AD0093BDD622A61D81", 131),
                    generator: point(
                        "00439CBC8DC73AA981030D5BC57B331663",
                        "014904C07D4F25A16C2DE036D60B762BD4",
                    ),
                    order: hex("0400000000000000026ABB991FE311FE83"),
                    cofactor: BigUint::from(2u32),
                },
                point(
                    "0602339C5DB0E9C694AC8908528C51C440",
                    "04F7B99169FA1A0F2737813742B1588CB8",
                ),
            ))
        }
    }
}

#[cfg(test)]
pub mod hash {
    pub mod sha256 {
        use sha2::{Digest, Sha256};
        pub fn sha256(data: &[u8]) -> [u8; 32] {
            Sha256::digest(data).into()
        }
    }
}
