use crate::algebra::{AffinePoint, Curve};
use crate::rr9::polynomial_from_roots;
use crate::{prime31_curve, PRIME31_GENERATOR, PRIME31_N, PRIME31_P};
use serde::{Deserialize, Serialize};

pub const RR9_CASE_SCHEMA: &str = "prime31.rr9-common-support-case/v1";
pub const RR9_PANEL_SEED: u64 = 0x5252_395f_5041_4e45;
pub const KNOWN_RR9_RESIDUAL: AffinePoint = AffinePoint {
    x: 1_911_579_686,
    y: 548_820_096,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rr9Target {
    Natural(usize),
    KnownChart,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SharedRr9Case {
    pub schema: String,
    pub case_id: String,
    pub curve: Curve,
    pub residual: AffinePoint,
    pub base: Vec<AffinePoint>,
    pub support_polynomial: Vec<u64>,
}

impl SharedRr9Case {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != RR9_CASE_SCHEMA {
            return Err(format!("unsupported RR9 case schema {:?}", self.schema));
        }
        self.curve.validate()?;
        if self.base.is_empty() {
            return Err("RR9 common support is empty".into());
        }
        if !self.curve.on_curve(self.residual) {
            return Err("RR9 residual is not on the curve".into());
        }
        if self
            .base
            .iter()
            .any(|point| !self.curve.on_curve(*point) || point.y == 0)
            || self
                .base
                .iter()
                .enumerate()
                .any(|(index, point)| self.base[..index].iter().any(|old| old.x == point.x))
        {
            return Err("RR9 base must contain distinct nonzero-y curve points".into());
        }
        let expected = polynomial_from_roots(
            &self.base.iter().map(|point| point.x).collect::<Vec<_>>(),
            self.curve.p,
        );
        if self.support_polynomial != expected {
            return Err("RR9 support polynomial does not match the base abscissas".into());
        }
        Ok(())
    }
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut value = *state;
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn sample_nonzero_uniform_scalar(state: &mut u64) -> u64 {
    let threshold = PRIME31_N.wrapping_neg() % PRIME31_N;
    loop {
        let value = splitmix64(state);
        if value >= threshold {
            let scalar = value % PRIME31_N;
            if scalar != 0 {
                return scalar;
            }
        }
    }
}

pub fn prime31_rr9_case(base_size: usize, target: Rr9Target) -> Result<SharedRr9Case, String> {
    if !matches!(base_size, 4 | 8 | 12 | 16 | 24 | 31) {
        return Err("registered RR9 base size must be 4, 8, 12, 16, 24, or 31".into());
    }
    let curve = prime31_curve();
    let base = curve.canonical_base(base_size)?;
    let (case_id, residual) = match target {
        Rr9Target::Natural(index) => {
            if index >= 64 {
                return Err("RR9 natural panel index must be below 64".into());
            }
            let mut state = RR9_PANEL_SEED ^ base_size as u64;
            let mut residual = None;
            for _ in 0..=index {
                residual =
                    curve.scalar_mul(sample_nonzero_uniform_scalar(&mut state), PRIME31_GENERATOR);
            }
            (
                format!("prime31-rr9-b{base_size}-natural-{index:02}"),
                residual.ok_or_else(|| "nonzero scalar produced infinity".to_owned())?,
            )
        }
        Rr9Target::KnownChart => {
            if base_size < 8 {
                return Err("the frozen known RR9 chart requires B >= 8".into());
            }
            (
                format!("prime31-rr9-b{base_size}-known-chart"),
                KNOWN_RR9_RESIDUAL,
            )
        }
    };
    let support_polynomial = polynomial_from_roots(
        &base.iter().map(|point| point.x).collect::<Vec<_>>(),
        PRIME31_P,
    );
    let case = SharedRr9Case {
        schema: RR9_CASE_SCHEMA.into(),
        case_id,
        curve,
        residual,
        base,
        support_polynomial,
    };
    case.validate()?;
    Ok(case)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rr9::{build_frozen_tower, decode_and_replay_chart, derive_tower_witness};
    use crate::PRIME31_A;

    const KNOWN_CHART: [u64; 7] = [
        455_835_358,
        518_652_820,
        754_716_693,
        932_383_545,
        62_510_441,
        1_884_059_825,
        1_142_049_404,
    ];

    #[test]
    fn known_chart_extends_across_every_registered_b_at_least_eight() {
        for base_size in [8, 12, 16, 24, 31] {
            let case = prime31_rr9_case(base_size, Rr9Target::KnownChart).unwrap();
            let tower = build_frozen_tower(
                PRIME31_P,
                PRIME31_A,
                (case.residual.x, case.residual.y),
                &case.support_polynomial,
            )
            .unwrap();
            derive_tower_witness(
                &tower,
                PRIME31_P,
                PRIME31_A,
                (case.residual.x, case.residual.y),
                &case.support_polynomial,
                KNOWN_CHART,
            )
            .unwrap();
            decode_and_replay_chart(case.curve, &case.base, case.residual, &KNOWN_CHART).unwrap();
        }
    }

    #[test]
    fn natural_cases_are_deterministic_and_witness_free() {
        let first = prime31_rr9_case(8, Rr9Target::Natural(0)).unwrap();
        assert_eq!(first, prime31_rr9_case(8, Rr9Target::Natural(0)).unwrap());
        assert_ne!(
            first.residual,
            prime31_rr9_case(8, Rr9Target::Natural(1)).unwrap().residual
        );
        let serialized = serde_json::to_string(&first).unwrap();
        assert!(!serialized.contains("chart"));
        assert!(!serialized.contains("witness"));
    }
}
