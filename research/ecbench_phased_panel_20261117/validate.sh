#!/usr/bin/env bash
set -euo pipefail

study_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo_dir=$(CDPATH= cd -- "$study_dir/../.." && pwd)
panel="$study_dir/panel.json"
schema="$study_dir/panel.schema.json"
registry="$repo_dir/docs/curves/registry.json"
traits="$repo_dir/docs/curves/traits.json"
assessment="$study_dir/phase1-cover-transfer-assessment.json"
hyper_spec="$repo_dir/docs/ecbench/specs/panel-phase1-hyperelliptic-jacobian.json"
transfer_spec="$repo_dir/docs/ecbench/specs/panel-phase1-cover-transfer.json"

jq empty "$panel" "$schema" "$assessment" "$hyper_spec" "$transfer_spec"

jq -e '
  .schema == "ecbench.phased-panel/v2" and
  [.lane_definitions[].lane_id] == [
    "elliptic_algebraic",
    "elliptic_generic_decomposition",
    "hyperelliptic_jacobian",
    "cover_transfer"
  ] and
  all(.lane_definitions[]; .promotion_scope == "independent") and
  [.phases[].phase] == [1, 2, 3, 4] and
  [.phases[].nominal_subgroup_bits] == [20, 30, 60, 80] and
  [.phases[].subgroup_bit_window] == [
    {"minimum":18,"maximum":23},
    {"minimum":27,"maximum":33},
    {"minimum":56,"maximum":62},
    {"minimum":76,"maximum":84}
  ] and
  ([.phases[].curves[].cell_id] | length) ==
    ([.phases[].curves[].cell_id] | unique | length) and
  all(.phases[];
    [.lanes[].lane_id] == [
      "elliptic_algebraic",
      "elliptic_generic_decomposition",
      "hyperelliptic_jacobian",
      "cover_transfer"
    ] and
    all(.lanes[];
      .mandatory == true and
      if .admission_state == "ready_to_preregister"
      then (.blockers | length) == 0
      else (.blockers | length) > 0
      end) and
    .subgroup_bit_window as $window |
    all(.curves[];
      .size.subgroup_bits >= $window.minimum and
      .size.subgroup_bits <= $window.maximum) and
    all(.destination_cells[];
      .size.subgroup_bits >= $window.minimum and
      .size.subgroup_bits <= $window.maximum)) and
  ([.phases[].destination_cells[].cell_id] | length) ==
    ([.phases[].destination_cells[].cell_id] | unique | length)
' "$panel" >/dev/null

jq -e '
  .lane_definitions[] | select(.lane_id == "cover_transfer") |
  .required_obligations == [
    "route_existence",
    "explicit_construction",
    "map_correctness",
    "subgroup_preservation",
    "destination_solver_applicability",
    "source_scalar_recovery",
    "nine_phase_complete_cost"
  ]
' "$panel" >/dev/null

jq -e '
  (.phases[] | select(.phase == 1) |
    all(.lanes[] | select(.lane_id == "hyperelliptic_jacobian" or .lane_id == "cover_transfer");
      .admission_state == "ready_to_preregister" and (.blockers | length) == 0) and
    any(.destination_cells[];
      .cell_id == "P1-JG2-18" and .family == "genus2" and
      .size.subgroup_order == "190339" and .size.subgroup_bits == 18 and
      .identity.icv1 == "JCV1:g2:fp769:706,731,331,360,270,1:571017" and
      .ecbench.curve_spec.kind == "genus2_explicit" and
      .ecbench.required_reference == "rho.negation" and
      .ecbench.required_candidates == ["ic.hyperelliptic_v2"])) and
  any(.transfer_routes[];
    .route_id == "P1-K0-23-G3-COVER" and .phase == 1 and
    .source_cell == "P1-K0-23" and .map_degree == 3 and
    .source_reference == "rho.signed_frobenius_strong" and
    .destination_methods == ["jacobian_image.rho3_v2", "jacobian_image.bsgs_v2"] and
    .cost_adapter == "binary_cubic_pullback_nine_phase_v2")
' "$panel" >/dev/null

jq -e '
  .format == "correspondence-assessment-v3" and
  .case_id == "phase1-koblitz23-ordinary-binary-genus3-cover-v1" and
  (.source.subgroup | contains("r=2095853")) and
  (.transformation.kind | contains("degree-three pullback")) and
  .obligations.subgroup_preservation.state == "supported" and
  .advantage_measurement.claim_scope == "unmeasured" and
  .advantage_measurement.verified_pairs == 0 and
  .official_status_change == "none"
' "$assessment" >/dev/null

jq -e '
  .schema == "ecbench.spec/v2" and
  .curve_policy == {"mode":"phase","phase":1,"lane":"hyperelliptic_jacobian"} and
  .workloads.curves == [{"kind":"panel_destination","cell":"P1-JG2-18"}] and
  .workloads.targets_per_curve == 32 and .workloads.target_kind == "public" and
  [.arms[].method.id] == ["rho.negation", "ic.hyperelliptic_v2", "rho.negation"]
' "$hyper_spec" >/dev/null

jq -e '
  .schema == "ecbench.spec/v2" and
  .curve_policy == {"mode":"phase","phase":1,"lane":"cover_transfer","route":"P1-K0-23-G3-COVER"} and
  .workloads.curves == [{"kind":"panel","cell":"P1-K0-23"}] and
  .workloads.targets_per_curve == 32 and .workloads.target_kind == "public" and
  [.arms[].method.id] == [
    "rho.signed_frobenius_strong",
    "jacobian_image.rho3_v2",
    "jacobian_image.bsgs_v2",
    "rho.signed_frobenius_strong"
  ]
' "$transfer_spec" >/dev/null

while IFS=$'\t' read -r slug ec1 bits cofactor; do
  jq -e --arg slug "$slug" --arg ec1 "$ec1" '
    any(.curves[];
      .slug == $slug and any(.representations[]; .ec1 == $ec1))
  ' "$registry" >/dev/null
  jq -e --arg slug "$slug" --argjson bits "$bits" --arg cofactor "$cofactor" '
    any(.curves[];
      .slug == $slug and .subgroup.bits == $bits and
      .subgroup.cofactor == $cofactor and
      .order_check.status == "proved" and .subgroup.status == "proved")
  ' "$traits" >/dev/null
done < <(jq -r '
  .phases[].curves[] |
  select(.identity.status == "registered") |
  [.identity.icv1, .identity.ec1, .size.subgroup_bits, .cofactor] | @tsv
' "$panel")

jq -e '
  .phases[] | select(.phase == 4) |
  .admission_state == "blocked" and
  all(.curves[] | select(.role == "external_anchor");
    .identity.status == "pending_local_import" and
    .identity.icv1 == null and .identity.ec1 == null and
    .ecbench.status == "blocked_pending_registry_and_wide_backend" and
    .ecbench.curve_spec == null)
' "$panel" >/dev/null

if command -v check-jsonschema >/dev/null 2>&1; then
  check-jsonschema --schemafile "$schema" "$panel"
fi

printf '%s\n' "ecbench phased panel: identity, size-window, lane, transfer-obligation, evidence, and readiness checks pass"
