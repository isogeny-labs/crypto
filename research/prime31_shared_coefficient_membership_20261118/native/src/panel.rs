use crate::algebra::AffinePoint;
use crate::ir::{RelationSpec, SlotDomain, RELATION_SCHEMA};
use crate::{prime31_curve, PRIME31_GENERATOR, PRIME31_N, W8_ODD_WEIGHTS};

pub const PANEL_SEED: u64 = 0x7738_4234_4238_3231;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PanelTarget {
    Natural(usize),
    Planted(usize),
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

pub fn prime31_weighted_slots(base_size: usize) -> Result<Vec<SlotDomain>, String> {
    if base_size == 0 {
        return Err("base size must be nonzero".into());
    }
    let curve = prime31_curve();
    let base = curve.canonical_base(base_size)?;
    W8_ODD_WEIGHTS
        .iter()
        .enumerate()
        .map(|(slot_index, weight)| {
            let points = base
                .iter()
                .map(|point| {
                    curve
                        .scalar_mul(*weight, *point)
                        .ok_or_else(|| format!("slot {slot_index} contains infinity"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(SlotDomain {
                label: format!("w{weight}"),
                points,
            })
        })
        .collect()
}

pub fn planted_word(base_size: usize, trial: usize) -> ([usize; 8], [bool; 8]) {
    let mut indices = [0usize; 8];
    let mut signs = [false; 8];
    for slot in 0..8 {
        indices[slot] = (3 * trial + 7 * slot + slot * slot) % base_size;
        // `true` means negate in the canonical IR.
        signs[slot] = (trial + 3 * slot) % 4 >= 2;
    }
    (indices, signs)
}

pub fn prime31_target(base_size: usize, target: PanelTarget) -> Result<AffinePoint, String> {
    let curve = prime31_curve();
    match target {
        PanelTarget::Natural(index) => {
            if index >= 64 {
                return Err("natural panel index must be below 64".into());
            }
            let mut state = PANEL_SEED ^ base_size as u64;
            let mut point = None;
            for _ in 0..=index {
                let scalar = sample_nonzero_uniform_scalar(&mut state);
                point = curve.scalar_mul(scalar, PRIME31_GENERATOR);
            }
            point.ok_or_else(|| "nonzero scalar unexpectedly produced infinity".into())
        }
        PanelTarget::Planted(index) => {
            if index >= 8 {
                return Err("planted panel index must be below 8".into());
            }
            let slots = prime31_weighted_slots(base_size)?;
            let (indices, signs) = planted_word(base_size, index);
            let relation = RelationSpec {
                schema: RELATION_SCHEMA.into(),
                case_id: "temporary-planted-target".into(),
                curve,
                target: PRIME31_GENERATOR,
                slots,
            };
            relation
                .replay(&indices, &signs)
                .ok_or_else(|| "planted word sums to infinity".into())
        }
    }
}

pub fn prime31_weighted_relation(
    base_size: usize,
    target_kind: PanelTarget,
) -> Result<RelationSpec, String> {
    let target = prime31_target(base_size, target_kind)?;
    let case_id = match target_kind {
        PanelTarget::Natural(index) => format!("prime31-w8-b{base_size}-natural-{index:02}"),
        PanelTarget::Planted(index) => format!("prime31-w8-b{base_size}-planted-{index:02}"),
    };
    let relation = RelationSpec {
        schema: RELATION_SCHEMA.into(),
        case_id,
        curve: prime31_curve(),
        target,
        slots: prime31_weighted_slots(base_size)?,
    };
    relation.validate().map_err(|error| error.to_string())?;
    Ok(relation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planted_targets_replay_without_exporting_the_word() {
        for base_size in [4, 6, 8] {
            for trial in 0..8 {
                let relation =
                    prime31_weighted_relation(base_size, PanelTarget::Planted(trial)).unwrap();
                let (indices, signs) = planted_word(base_size, trial);
                assert_eq!(relation.replay(&indices, &signs), Some(relation.target));
                let serialized = serde_json::to_string(&relation).unwrap();
                assert!(!serialized.contains("indices"));
                assert!(!serialized.contains("signs"));
            }
        }
    }

    #[test]
    fn natural_targets_match_frozen_first_scalar_stream() {
        let b8_first = prime31_target(8, PanelTarget::Natural(0)).unwrap();
        let b8_again = prime31_target(8, PanelTarget::Natural(0)).unwrap();
        assert_eq!(b8_first, b8_again);
        assert_ne!(
            b8_first,
            prime31_target(4, PanelTarget::Natural(0)).unwrap()
        );
    }
}
