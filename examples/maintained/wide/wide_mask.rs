//! Canonical 128-bit diagnostic masks in either library feature build.
use crypto_lib::cryptanalysis::pq_groebner_f2::Mask;

pub fn wide_mask(mask: Mask) -> u128 {
    #[cfg(feature = "wide-vars")]
    {
        mask
    }
    #[cfg(not(feature = "wide-vars"))]
    {
        u128::from(mask)
    }
}
