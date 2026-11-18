//! The canonical cross-family curve ladder used by new ecbench studies.
//!
//! The source of truth is the reviewed, machine-readable manifest in
//! `research/ecbench_phased_panel_20261117/panel.json`.  This module embeds
//! that file so a spec can name a cell instead of copying curve parameters.
//! Historical specs remain replayable; new scaling measurements select cells
//! through [`CurveSpec::Panel`](super::workload::CurveSpec::Panel).

use std::collections::BTreeSet;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use super::workload::CurveSpec;

pub const PANEL_SCHEMA: &str = "ecbench.phased-panel/v1";
pub const PANEL_SOURCE: &str = "research/ecbench_phased_panel_20261117/panel.json";
const PANEL_JSON: &str = include_str!("../../../research/ecbench_phased_panel_20261117/panel.json");

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Panel {
    pub schema: String,
    pub status: String,
    pub size_metric: SizeMetric,
    pub gate_defaults: GateDefaults,
    pub phases: Vec<Phase>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SizeMetric {
    pub name: String,
    pub definition: String,
    pub field_size_is_separate: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GateDefaults {
    pub familywise_alpha: f64,
    pub minimum_terminal_episodes: u64,
    pub minimum_full_rank_success_probability: f64,
    pub maximum_yield_shortfall_factor: f64,
    pub next_phase_resource_headroom_factor: f64,
    pub targets_per_curve_below_64_bits: u64,
    pub targets_per_curve_at_or_above_64_bits: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Phase {
    pub phase: u8,
    pub nominal_subgroup_bits: u32,
    pub subgroup_bit_window: BitWindow,
    pub objective: String,
    pub minimum_public_targets_per_curve: u64,
    pub admission_state: String,
    pub curves: Vec<Cell>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitWindow {
    pub minimum: u32,
    pub maximum: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub cell_id: String,
    pub family: String,
    pub role: String,
    pub mandatory: bool,
    pub identity: Identity,
    pub size: Size,
    pub cofactor: String,
    pub traits: Vec<String>,
    pub evidence_status: EvidenceStatus,
    pub ecbench: Ecbench,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    pub status: String,
    #[serde(default)]
    pub icv1: Option<String>,
    #[serde(default)]
    pub ec1: Option<String>,
    #[serde(default)]
    pub curve_uid: Option<String>,
    #[serde(default)]
    pub external_name: Option<String>,
    #[serde(default)]
    pub source_blob: Option<String>,
    #[serde(default)]
    pub external_parameters: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size {
    pub subgroup_order: String,
    pub subgroup_bits: u32,
    pub field_bits_or_degree: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceStatus {
    pub order: String,
    pub subgroup: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ecbench {
    pub status: String,
    pub curve_spec: Option<CurveSpec>,
    pub required_reference: String,
}

impl Cell {
    pub fn runnable(&self) -> bool {
        self.ecbench.curve_spec.is_some()
            && self.ecbench.status != "blocked_pending_registry_and_wide_backend"
    }
}

impl Panel {
    pub fn phase(&self, number: u8) -> Option<&Phase> {
        self.phases.iter().find(|phase| phase.phase == number)
    }

    pub fn cell(&self, id: &str) -> Option<(&Phase, &Cell)> {
        self.phases.iter().find_map(|phase| {
            phase
                .curves
                .iter()
                .find(|cell| cell.cell_id == id)
                .map(|cell| (phase, cell))
        })
    }

    fn validate(&self) -> Result<(), String> {
        if self.schema != PANEL_SCHEMA {
            return Err(format!(
                "canonical panel schema is `{}`, expected `{PANEL_SCHEMA}`",
                self.schema
            ));
        }
        if self.size_metric.name != "subgroup_bits"
            || self.size_metric.definition != "ceil(log2(r))"
            || !self.size_metric.field_size_is_separate
        {
            return Err(
                "canonical panel must size cells by ceil(log2(r)), separately from field size"
                    .into(),
            );
        }
        let expected = [
            (1, 20, 18, 23),
            (2, 30, 27, 33),
            (3, 60, 56, 62),
            (4, 80, 76, 84),
        ];
        if self.phases.len() != expected.len() {
            return Err(format!(
                "canonical panel has {} phases, expected {}",
                self.phases.len(),
                expected.len()
            ));
        }
        let mut ids = BTreeSet::new();
        for (phase, (number, nominal, minimum, maximum)) in self.phases.iter().zip(expected) {
            if (
                phase.phase,
                phase.nominal_subgroup_bits,
                phase.subgroup_bit_window.minimum,
                phase.subgroup_bit_window.maximum,
            ) != (number, nominal, minimum, maximum)
            {
                return Err(format!(
                    "phase {} does not carry the frozen nominal/window tuple",
                    phase.phase
                ));
            }
            for cell in &phase.curves {
                if !ids.insert(cell.cell_id.as_str()) {
                    return Err(format!("panel cell `{}` appears twice", cell.cell_id));
                }
                if !(minimum..=maximum).contains(&cell.size.subgroup_bits) {
                    return Err(format!(
                        "panel cell `{}` has {} subgroup bits outside phase {}'s {}..={} window",
                        cell.cell_id, cell.size.subgroup_bits, number, minimum, maximum
                    ));
                }
                if cell.identity.status == "registered"
                    && (cell.identity.icv1.is_none()
                        || cell.identity.ec1.is_none()
                        || cell.identity.curve_uid.is_none()
                        || cell.ecbench.curve_spec.is_none())
                {
                    return Err(format!(
                        "registered panel cell `{}` lacks an identity or curve spec",
                        cell.cell_id
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Parse and validate the embedded manifest once.
pub fn catalog() -> Result<&'static Panel, String> {
    static CATALOG: OnceLock<Result<Panel, String>> = OnceLock::new();
    CATALOG
        .get_or_init(|| {
            let panel: Panel =
                serde_json::from_str(PANEL_JSON).map_err(|e| format!("{PANEL_SOURCE}: {e}"))?;
            panel.validate()?;
            Ok(panel)
        })
        .as_ref()
        .map_err(Clone::clone)
}

pub fn cell(id: &str) -> Result<(&'static Phase, &'static Cell), String> {
    catalog()?
        .cell(id)
        .ok_or_else(|| format!("`{id}` is not a canonical ecbench panel cell"))
}

pub fn curve_spec(id: &str) -> Result<CurveSpec, String> {
    let (_, cell) = cell(id)?;
    cell.ecbench.curve_spec.clone().ok_or_else(|| {
        format!(
            "panel cell `{id}` is {} and has no runnable ecbench curve spec",
            cell.ecbench.status
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_catalog_has_the_frozen_ladder_and_unique_cells() {
        let panel = catalog().unwrap();
        assert_eq!(
            panel
                .phases
                .iter()
                .map(|phase| (
                    phase.phase,
                    phase.nominal_subgroup_bits,
                    phase.subgroup_bit_window.minimum,
                    phase.subgroup_bit_window.maximum,
                ))
                .collect::<Vec<_>>(),
            vec![
                (1, 20, 18, 23),
                (2, 30, 27, 33),
                (3, 60, 56, 62),
                (4, 80, 76, 84)
            ]
        );
        assert_eq!(
            panel
                .phases
                .iter()
                .map(|phase| phase.curves.len())
                .sum::<usize>(),
            16
        );
    }

    #[test]
    fn blocked_external_anchors_cannot_be_built() {
        for id in ["P4-ECC2-79", "P4-ECCP-79"] {
            assert!(curve_spec(id).unwrap_err().contains("no runnable"));
        }
    }

    #[test]
    fn every_runnable_cell_builds_the_manifest_identity() {
        let panel = catalog().unwrap();
        for phase in &panel.phases {
            for cell in &phase.curves {
                if !cell.runnable() {
                    continue;
                }
                let spec = CurveSpec::Panel {
                    cell: cell.cell_id.clone(),
                };
                let instance = spec
                    .build()
                    .unwrap_or_else(|error| panic!("{}: {error}", cell.cell_id));
                let facts = instance.facts(&spec);
                assert_eq!(facts.family, cell.family, "{} family", cell.cell_id);
                assert_eq!(
                    facts.r.to_string(),
                    cell.size.subgroup_order,
                    "{} subgroup order",
                    cell.cell_id
                );
                assert_eq!(
                    facts.cofactor.to_string(),
                    cell.cofactor,
                    "{} cofactor",
                    cell.cell_id
                );
                assert_eq!(facts.icv1.as_str(), cell.identity.icv1.as_deref().unwrap());
                assert_eq!(facts.ec1.as_deref(), cell.identity.ec1.as_deref());
                assert_eq!(
                    facts.curve_uid.as_deref(),
                    cell.identity.curve_uid.as_deref()
                );
            }
        }
    }
}
