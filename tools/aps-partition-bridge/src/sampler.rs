use crypto_lib::cryptanalysis::{
    aps_single_relation::{self as aps, LabeledBase, Labels},
    prime_fast::{FastCurve, Pt},
};

pub const SEEDS: [u64; 2] = [20261115, 20261116];
pub const FAMILY_SIZE: usize = 8;

pub struct Fixture {
    pub curve: FastCurve,
    pub g: Pt,
    pub q: Pt,
    pub base: LabeledBase,
}

/// Public known-answer fixture already used by the APS integration tests.
/// The sampler receives G and Q, never the target logarithm.
pub fn fixture(seed: u64) -> Result<Fixture, String> {
    profile(seed, &[3, 3, 2])
}

pub fn even_fixture(seed: u64) -> Result<Fixture, String> {
    profile(seed, &[2, 2, 2, 2])
}

fn profile(seed: u64, sizes: &[usize]) -> Result<Fixture, String> {
    let curve = FastCurve::new("aps-partition-bridge-p101", 101, 59, 6, 107);
    let g = curve.point(78, 4);
    let q = curve
        .mul(Some(g), 37)
        .ok_or("invalid public fixture target")?;
    let base = aps::sample_base(&curve, g, q, sizes, Labels::Guaranteed, seed, 1000)?;
    aps::validate_base(&curve, g, q, &base)?;
    Ok(Fixture { curve, g, q, base })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_sampler_retains_labels_and_refuses_a_corrupted_representation() {
        for seed in SEEDS {
            let f = fixture(seed).unwrap();
            assert_eq!(f.base.points.len(), 8);
            assert_eq!(
                f.base.parts.iter().map(Vec::len).collect::<Vec<_>>(),
                [3, 3, 2]
            );
            let mut poisoned = f.base.clone();
            let point = poisoned.points.values_mut().next().unwrap();
            point.u = (point.u + 1) % f.curve.n;
            assert!(aps::validate_base(&f.curve, f.g, f.q, &poisoned).is_err());
        }
    }
}
