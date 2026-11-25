//! Shim crate: lets the UNMODIFIED crypto/src/cryptanalysis/semaev_projection.rs
//! compile without the multi-GB `crypto` lib.  Only its imports are supplied.
#![allow(dead_code, unused_imports, clippy::all)]
pub fn jacobi_u64_reexport() {}
include!("jacobi_extract.rs");

pub mod ecc {
    pub mod field {
        //! NOT verbatim: minimal BigUint FieldElement for the multi-limb *reference*
        //! functions that are compiled but never called by the benchmark.
        use num_bigint::BigUint;
        use num_traits::{One, Zero};
        #[derive(Clone, Debug, PartialEq)]
        pub struct FieldElement { pub value: BigUint, pub modulus: BigUint }
        impl FieldElement {
            pub fn new(value: BigUint, modulus: BigUint) -> Self { let value = value % &modulus; Self { value, modulus } }
            pub fn is_zero(&self) -> bool { self.value.is_zero() }
            pub fn add(&self, r: &Self) -> Self { Self::new(&self.value + &r.value, self.modulus.clone()) }
            pub fn sub(&self, r: &Self) -> Self { Self::new(&self.value + &self.modulus - &r.value, self.modulus.clone()) }
            pub fn mul(&self, r: &Self) -> Self { Self::new(&self.value * &r.value, self.modulus.clone()) }
            pub fn neg(&self) -> Self { Self::new(&self.modulus - &self.value, self.modulus.clone()) }
            pub fn inv(&self) -> Option<Self> {
                if self.is_zero() { return None; }
                let e = &self.modulus - BigUint::from(2u32);
                Some(Self::new(self.value.modpow(&e, &self.modulus), self.modulus.clone()))
            }
            pub fn pow(&self, e: &BigUint) -> Self { Self::new(self.value.modpow(e, &self.modulus), self.modulus.clone()) }
        }
        #[allow(unused)] fn _o() -> BigUint { BigUint::one() }
    }
}
pub mod cryptanalysis {
    pub mod pkm_smooth_tower { include!("pkm_extract.rs"); }
    pub mod prime_fast { include!("prime_fast_extract.rs"); }
    pub mod ec_index_calculus { include!("sqrt_extract.rs"); }
    #[path = "../../../../../src/cryptanalysis/semaev_projection.rs"]
    pub mod semaev_projection;
}
