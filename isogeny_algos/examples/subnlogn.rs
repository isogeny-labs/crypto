//! A deterministic correctness demonstration, not a performance benchmark.
use isogeny_algos::field::{Field, Zp};
use isogeny_algos::poly;
use isogeny_algos::subnlogn::motif::{BitMotif, PublishedAudit};
use isogeny_algos::subnlogn::{multiply_exact, Algorithm, Config, ExperimentalField};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().skip(1).any(|arg| arg == "--full") {
        // A request for the complete algorithm must fail explicitly.
        multiply_exact(&[3], &[7], Algorithm::PublishedSubNlogN, 128)?;
        unreachable!("the complete published algorithm is unavailable");
    }
    demonstration()
}

fn demonstration() -> Result<(), Box<dyn std::error::Error>> {
    let base = Zp::new((1 << 31) - 1);
    let f = ExperimentalField::new(base, Config::default());
    let a: Vec<_> = (0..33).map(|i| base.from_u64(17 * i * i + 3)).collect();
    let b: Vec<_> = (0..35)
        .map(|i| base.neg(base.from_u64(29 * i + 5)))
        .collect();
    let expected = base.conv(&a, &b);
    let actual = poly::mul_raw(&f, &a, &b);
    assert_eq!(
        actual, expected,
        "polynomial product differs from schoolbook"
    );
    let stats = f.stats();
    assert_eq!(stats.synthetic, 1, "the candidate route must actually run");
    assert_eq!(stats.fallback, 0);

    let product = multiply_exact(
        &[u64::MAX; 2],
        &[u64::MAX; 2],
        Algorithm::SyntheticReference,
        128,
    )?;
    // (2^128 - 1)^2 = 2^256 - 2^129 + 1.
    assert_eq!(product.limbs, [1, 0, u64::MAX - 1, u64::MAX]);

    let motif = BitMotif::new(6)?;
    let mut left: Vec<_> = (0..motif.bank_len())
        .map(|i| (i as u64 + 1) * 0x10203)
        .collect();
    let mut right: Vec<_> = (0..motif.bank_len())
        .map(|i| !(i as u64 * 0x50403))
        .collect();
    let before = (left.clone(), right.clone());
    let mut scratch = std::array::from_fn(|stage| {
        let mut s = motif.zero_scratch();
        for (i, word) in s.side.iter_mut().chain(s.center.iter_mut()).enumerate() {
            *word = (stage as u64 + 1).wrapping_mul(0x12345678) ^ !(i as u64);
        }
        s
    });
    let initial_scratch = scratch.clone();
    motif.exchange_banks(&mut left, &mut right, &mut scratch)?;
    assert_eq!(left, before.1);
    assert_eq!(right, before.0);
    assert_eq!(
        scratch, initial_scratch,
        "all auxiliary state must be restored"
    );

    let audit = PublishedAudit::compute()?;
    // Large symbolic counts are JSON strings to preserve exactness in readers
    // that represent JSON numbers as IEEE-754 doubles.
    println!(
        concat!(
            "{{\"scope\":\"synthetic_reference_and_scalar_motif\",",
            "\"complete_published_algorithm\":false,",
            "\"polynomial_matches_oracle\":true,",
            "\"integer_matches_oracle\":true,",
            "\"motif_swap_and_scratch_restoration\":true,",
            "\"dispatch\":{{\"attempts\":{},\"synthetic\":{},\"fallback\":{}}},",
            "\"integer_transform_points\":{},",
            "\"published_bit_wires\":\"{}\",",
            "\"published_bit_rank_sum\":\"{}\",",
            "\"published_complex_wires\":\"{}\",",
            "\"published_complex_rank_sum\":\"{}\"}}"
        ),
        stats.attempts,
        stats.synthetic,
        stats.fallback,
        product.transform_points,
        audit.bit.wires,
        audit.bit.edge_rank_sum,
        audit.complex.wires,
        audit.complex.edge_rank_sum,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_checks_candidate_products_and_auxiliary_restoration() {
        super::demonstration().unwrap();
    }
}
