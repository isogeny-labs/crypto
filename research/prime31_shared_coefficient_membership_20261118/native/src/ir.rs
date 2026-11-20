use crate::algebra::{AffinePoint, Curve};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const RELATION_SCHEMA: &str = "prime31.selector-relation/v1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SlotDomain {
    pub label: String,
    pub points: Vec<AffinePoint>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RelationSpec {
    pub schema: String,
    pub case_id: String,
    pub curve: Curve,
    pub target: AffinePoint,
    pub slots: Vec<SlotDomain>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoweringError {
    InvalidRelation(String),
    SlotSpecificSupports {
        reference_slot: usize,
        differing_slot: usize,
    },
}

impl std::fmt::Display for LoweringError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRelation(message) => write!(formatter, "invalid relation: {message}"),
            Self::SlotSpecificSupports {
                reference_slot,
                differing_slot,
            } => write!(
                formatter,
                "shared RR9 requires one common x support; slot {differing_slot} differs from slot {reference_slot}"
            ),
        }
    }
}

impl std::error::Error for LoweringError {}

impl RelationSpec {
    pub fn validate(&self) -> Result<(), LoweringError> {
        if self.schema != RELATION_SCHEMA {
            return Err(LoweringError::InvalidRelation(format!(
                "schema must be {RELATION_SCHEMA}"
            )));
        }
        self.curve
            .validate()
            .map_err(LoweringError::InvalidRelation)?;
        if !self.curve.on_curve(self.target) {
            return Err(LoweringError::InvalidRelation(
                "public target is not on the declared curve".into(),
            ));
        }
        if self.slots.len() != 8 {
            return Err(LoweringError::InvalidRelation(format!(
                "exactly eight ordered slots are required, got {}",
                self.slots.len()
            )));
        }
        let base_size = self.slots[0].points.len();
        if base_size == 0 {
            return Err(LoweringError::InvalidRelation(
                "slot point domains must be nonempty".into(),
            ));
        }
        for (slot_index, slot) in self.slots.iter().enumerate() {
            if slot.points.len() != base_size {
                return Err(LoweringError::InvalidRelation(format!(
                    "slot {slot_index} has {} points; expected {base_size}",
                    slot.points.len()
                )));
            }
            let mut distinct = BTreeSet::new();
            for (point_index, point) in slot.points.iter().copied().enumerate() {
                if !self.curve.on_curve(point) {
                    return Err(LoweringError::InvalidRelation(format!(
                        "slot {slot_index} point {point_index} is off curve"
                    )));
                }
                if !distinct.insert((point.x, point.y)) {
                    return Err(LoweringError::InvalidRelation(format!(
                        "slot {slot_index} repeats a point"
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn base_size(&self) -> usize {
        self.slots.first().map_or(0, |slot| slot.points.len())
    }

    /// Return the one x support admitted by the historical shared-RR9 tower.
    ///
    /// Signs do not alter x coordinates.  Slot order does not matter for this
    /// check, but multiplicity does; duplicate x coordinates are retained.
    pub fn shared_rr9_x_support(&self) -> Result<Vec<u64>, LoweringError> {
        self.validate()?;
        let mut reference = self.slots[0]
            .points
            .iter()
            .map(|point| point.x)
            .collect::<Vec<_>>();
        reference.sort_unstable();
        for (slot_index, slot) in self.slots.iter().enumerate().skip(1) {
            let mut support = slot.points.iter().map(|point| point.x).collect::<Vec<_>>();
            support.sort_unstable();
            if support != reference {
                return Err(LoweringError::SlotSpecificSupports {
                    reference_slot: 0,
                    differing_slot: slot_index,
                });
            }
        }
        Ok(reference)
    }

    pub fn replay(&self, indices: &[usize; 8], signs: &[bool; 8]) -> Option<AffinePoint> {
        let mut sum = None;
        for slot in 0..8 {
            let point = *self.slots[slot].points.get(indices[slot])?;
            let point = if signs[slot] {
                self.curve.point_neg(Some(point))
            } else {
                Some(point)
            };
            sum = self.curve.point_add(sum, point);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::panel::{prime31_weighted_relation, PanelTarget};

    #[test]
    fn weighted_w8_is_rejected_by_shared_rr9_type_gate() {
        let relation = prime31_weighted_relation(8, PanelTarget::Planted(0)).unwrap();
        assert!(matches!(
            relation.shared_rr9_x_support(),
            Err(LoweringError::SlotSpecificSupports { .. })
        ));
    }

    #[test]
    fn equal_slot_support_is_accepted() {
        let mut relation = prime31_weighted_relation(4, PanelTarget::Planted(0)).unwrap();
        let common = relation.slots[0].clone();
        for slot in &mut relation.slots {
            slot.points.clone_from(&common.points);
        }
        assert_eq!(relation.shared_rr9_x_support().unwrap().len(), 4);
    }
}
