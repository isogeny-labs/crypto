//! Checked boundary for historical algorithms whose assignments use one u64.
use crypto_lib::cryptanalysis::pq_groebner_f2::Mask;

pub fn word_mask(mask: Mask) -> u64 {
    #[cfg(feature = "wide-vars")]
    {
        u64::try_from(mask).expect("historical word algorithm requires a 64-bit mask")
    }
    #[cfg(not(feature = "wide-vars"))]
    {
        mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::pq_groebner_f2::mask_from64;
    #[test]
    fn retains_every_bit_of_a_word_assignment() {
        assert_eq!(word_mask(mask_from64(u64::MAX)), u64::MAX);
    }
    #[cfg(feature = "wide-vars")]
    #[test]
    #[should_panic(expected = "historical word algorithm requires a 64-bit mask")]
    fn refuses_a_mask_with_an_unrepresentable_variable() {
        word_mask(1u128 << 80);
    }
}
