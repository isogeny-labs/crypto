//! The canonical cross-family curve ladder used by new ecbench studies.
//!
//! The source of truth is the reviewed, machine-readable manifest in
//! `research/ecbench_phased_panel_20261117/panel.json`.  This module embeds
//! that file so a spec can name a cell instead of copying curve parameters.
//! Historical specs remain replayable; new scaling measurements select cells
//! through [`CurveSpec::Panel`](super::workload::CurveSpec::Panel), while
//! direct higher-genus lanes use
//! [`CurveSpec::PanelDestination`](super::workload::CurveSpec::PanelDestination).

use std::collections::BTreeSet;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use super::workload::CurveSpec;

pub const PANEL_SCHEMA: &str = "ecbench.phased-panel/v2";
pub const PANEL_SOURCE: &str = "research/ecbench_phased_panel_20261117/panel.json";
pub const DEFAULT_PHASE_LANE: &str = "elliptic_algebraic";
const PANEL_JSON: &str = include_str!("../../../research/ecbench_phased_panel_20261117/panel.json");

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Panel {
    pub schema: String,
    pub status: String,
    pub size_metric: SizeMetric,
    pub gate_defaults: GateDefaults,
    pub lane_definitions: Vec<LaneDefinition>,
    pub transfer_routes: Vec<TransferRoute>,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneDefinition {
    pub lane_id: String,
    pub object_kind: String,
    pub algorithm_track: String,
    pub curve_selector: String,
    pub promotion_scope: String,
    pub required_obligations: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhaseLane {
    pub lane_id: String,
    pub mandatory: bool,
    pub admission_state: String,
    pub blockers: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Phase {
    pub phase: u8,
    pub nominal_subgroup_bits: u32,
    pub subgroup_bit_window: BitWindow,
    pub objective: String,
    pub minimum_public_targets_per_curve: u64,
    pub admission_state: String,
    pub lanes: Vec<PhaseLane>,
    pub destination_cells: Vec<Cell>,
    pub curves: Vec<Cell>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferRoute {
    pub route_id: String,
    pub phase: u8,
    pub source_cell: String,
    pub destination: String,
    pub map_degree: u32,
    pub construction: String,
    pub map: String,
    pub subgroup_condition: String,
    pub source_reference: String,
    pub destination_methods: Vec<String>,
    #[serde(default)]
    pub destination_controls: Vec<String>,
    pub cost_adapter: String,
    pub assessment: String,
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
    #[serde(default)]
    pub required_candidates: Vec<String>,
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

    pub fn destination(&self, id: &str) -> Option<(&Phase, &Cell)> {
        self.phases.iter().find_map(|phase| {
            phase
                .destination_cells
                .iter()
                .find(|cell| cell.cell_id == id)
                .map(|cell| (phase, cell))
        })
    }

    pub fn transfer_route(&self, id: &str) -> Option<&TransferRoute> {
        self.transfer_routes
            .iter()
            .find(|route| route.route_id == id)
    }

    pub fn lane(&self, id: &str) -> Option<&LaneDefinition> {
        self.lane_definitions.iter().find(|lane| lane.lane_id == id)
    }

    pub fn phase_lane(&self, phase: u8, lane: &str) -> Option<(&Phase, &PhaseLane)> {
        let phase = self.phase(phase)?;
        phase
            .lanes
            .iter()
            .find(|record| record.lane_id == lane)
            .map(|record| (phase, record))
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
        let expected_lanes = [
            (
                "elliptic_algebraic",
                "elliptic_subgroup",
                "algebraic_decomposition",
                "source_panel_cells",
            ),
            (
                "elliptic_generic_decomposition",
                "elliptic_subgroup",
                "generic_decomposition",
                "source_panel_cells",
            ),
            (
                "hyperelliptic_jacobian",
                "jacobian_subgroup",
                "higher_genus_index_calculus",
                "registered_destination_cells",
            ),
            (
                "cover_transfer",
                "typed_transfer_route",
                "cover_transfer",
                "registered_transfer_routes",
            ),
        ];
        let mut lane_ids = BTreeSet::new();
        for lane in &self.lane_definitions {
            if !lane_ids.insert(lane.lane_id.as_str()) {
                return Err(format!("benchmark lane `{}` appears twice", lane.lane_id));
            }
            if lane.promotion_scope != "independent" {
                return Err(format!(
                    "benchmark lane `{}` must promote independently",
                    lane.lane_id
                ));
            }
            if lane.required_obligations.is_empty() {
                return Err(format!(
                    "benchmark lane `{}` has no promotion obligations",
                    lane.lane_id
                ));
            }
        }
        if lane_ids != expected_lanes.iter().map(|(id, _, _, _)| *id).collect() {
            return Err("canonical panel benchmark lanes do not match the frozen lane set".into());
        }
        for (id, object_kind, algorithm_track, curve_selector) in expected_lanes {
            let lane = self.lane(id).expect("lane set checked above");
            if (
                lane.object_kind.as_str(),
                lane.algorithm_track.as_str(),
                lane.curve_selector.as_str(),
            ) != (object_kind, algorithm_track, curve_selector)
            {
                return Err(format!(
                    "benchmark lane `{id}` does not carry its frozen object/track/selector contract"
                ));
            }
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
        let mut destination_ids = BTreeSet::new();
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
            let mut phase_lanes = BTreeSet::new();
            for lane in &phase.lanes {
                if !phase_lanes.insert(lane.lane_id.as_str()) {
                    return Err(format!(
                        "phase {} benchmark lane `{}` appears twice",
                        phase.phase, lane.lane_id
                    ));
                }
                if !lane_ids.contains(lane.lane_id.as_str()) {
                    return Err(format!(
                        "phase {} names unknown benchmark lane `{}`",
                        phase.phase, lane.lane_id
                    ));
                }
                if !lane.mandatory {
                    return Err(format!(
                        "phase {} benchmark lane `{}` must remain visible even when blocked",
                        phase.phase, lane.lane_id
                    ));
                }
                if !["ready_to_preregister", "partially_blocked", "blocked"]
                    .contains(&lane.admission_state.as_str())
                {
                    return Err(format!(
                        "phase {} benchmark lane `{}` has unknown admission state `{}`",
                        phase.phase, lane.lane_id, lane.admission_state
                    ));
                }
                let ready = lane.admission_state == "ready_to_preregister";
                if ready != lane.blockers.is_empty() {
                    return Err(format!(
                        "phase {} benchmark lane `{}` must have no blockers exactly when ready",
                        phase.phase, lane.lane_id
                    ));
                }
            }
            if phase_lanes != lane_ids {
                return Err(format!(
                    "phase {} does not carry every canonical benchmark lane",
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
            for cell in &phase.destination_cells {
                if !destination_ids.insert(cell.cell_id.as_str()) {
                    return Err(format!(
                        "destination panel cell `{}` appears twice",
                        cell.cell_id
                    ));
                }
                if !(minimum..=maximum).contains(&cell.size.subgroup_bits) {
                    return Err(format!(
                        "destination panel cell `{}` has {} subgroup bits outside phase {}'s {}..={} window",
                        cell.cell_id, cell.size.subgroup_bits, number, minimum, maximum
                    ));
                }
                if cell.identity.status != "locally_pinned"
                    || cell.identity.icv1.is_none()
                    || cell.ecbench.curve_spec.is_none()
                    || cell.ecbench.required_reference.is_empty()
                    || cell.ecbench.required_candidates.is_empty()
                    || !cell.runnable()
                {
                    return Err(format!(
                        "destination panel cell `{}` lacks its pinned identity or runnable curve spec",
                        cell.cell_id
                    ));
                }
            }
        }
        let mut route_ids = BTreeSet::new();
        for route in &self.transfer_routes {
            if !route_ids.insert(route.route_id.as_str()) {
                return Err(format!("transfer route `{}` appears twice", route.route_id));
            }
            let (phase, _) = self.cell(&route.source_cell).ok_or_else(|| {
                format!(
                    "transfer route `{}` names an unknown source cell",
                    route.route_id
                )
            })?;
            if phase.phase != route.phase {
                return Err(format!(
                    "transfer route `{}` belongs to phase {} but its source belongs to phase {}",
                    route.route_id, route.phase, phase.phase
                ));
            }
            if route.map_degree < 2
                || route.destination_methods.is_empty()
                || route.source_reference.is_empty()
                || route.cost_adapter.is_empty()
                || route.assessment.is_empty()
            {
                return Err(format!(
                    "transfer route `{}` lacks its map, solver, cost, or assessment contract",
                    route.route_id
                ));
            }
        }
        let (phase_one, phase_one_destination) = self
            .destination("P1-JG2-18")
            .ok_or("the Phase 1 direct Jacobian destination is missing")?;
        if phase_one.phase != 1
            || phase_one_destination.size.subgroup_order != "190339"
            || phase_one_destination.size.subgroup_bits != 18
            || phase_one_destination.ecbench.required_reference != "rho.negation"
            || phase_one_destination.ecbench.required_candidates != ["ic.hyperelliptic_v2"]
        {
            return Err(
                "the Phase 1 direct Jacobian destination does not match its frozen contract".into(),
            );
        }
        let phase_one_route = self
            .transfer_route("P1-K0-23-G3-COVER")
            .ok_or("the Phase 1 cover-transfer route is missing")?;
        if phase_one_route.phase != 1
            || phase_one_route.source_cell != "P1-K0-23"
            || phase_one_route.map_degree != 3
            || phase_one_route.source_reference != "rho.signed_frobenius_strong"
            || phase_one_route.destination_methods
                != [
                    "jacobian_image.rho3_v2",
                    "jacobian_image.genus3_projected_ic_v1",
                ]
            || phase_one_route.destination_controls != ["jacobian_image.bsgs_v2"]
        {
            return Err(
                "the Phase 1 cover-transfer route does not match its frozen contract".into(),
            );
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

pub fn destination_curve_spec(id: &str) -> Result<CurveSpec, String> {
    let (_, cell) = catalog()?
        .destination(id)
        .ok_or_else(|| format!("`{id}` is not a canonical ecbench destination cell"))?;
    cell.ecbench.curve_spec.clone().ok_or_else(|| {
        format!(
            "destination panel cell `{id}` is {} and has no runnable ecbench curve spec",
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
                .lane_definitions
                .iter()
                .map(|lane| lane.lane_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "elliptic_algebraic",
                "elliptic_generic_decomposition",
                "hyperelliptic_jacobian",
                "cover_transfer"
            ]
        );
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
        assert_eq!(
            panel
                .phase_lane(1, "hyperelliptic_jacobian")
                .unwrap()
                .1
                .admission_state,
            "ready_to_preregister"
        );
        assert_eq!(panel.transfer_routes.len(), 1);
        assert_eq!(panel.phases[0].destination_cells.len(), 1);
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
        for phase in &panel.phases {
            for cell in &phase.destination_cells {
                let spec = CurveSpec::PanelDestination {
                    cell: cell.cell_id.clone(),
                };
                let instance = spec
                    .build()
                    .unwrap_or_else(|error| panic!("{}: {error}", cell.cell_id));
                let facts = instance.facts(&spec);
                assert_eq!(facts.family, cell.family, "{} family", cell.cell_id);
                assert_eq!(facts.r.to_string(), cell.size.subgroup_order);
                assert_eq!(facts.cofactor.to_string(), cell.cofactor);
                assert_eq!(facts.icv1.as_str(), cell.identity.icv1.as_deref().unwrap());
            }
        }
    }
}
