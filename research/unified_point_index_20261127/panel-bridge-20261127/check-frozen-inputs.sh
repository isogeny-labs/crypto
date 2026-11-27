#!/usr/bin/env bash
# Trusted-main, no-AWS admission of a frozen native pair and public panel.
set -euo pipefail
[[ $# == 2 ]] || { echo 'usage: check-frozen-inputs.sh SOURCE_DIR DRIVER_SHA' >&2; exit 2; }
source_dir=$(realpath -- "$1")
driver_sha=$2
[[ $driver_sha =~ ^[a-f0-9]{40}$ ]] || { echo 'invalid driver SHA' >&2; exit 2; }
[[ $(git -C "$source_dir" rev-parse HEAD) == "$driver_sha" ]] || { echo 'driver checkout SHA mismatch' >&2; exit 2; }
here=research/unified_point_index_20261127/panel-bridge-20261127
inputs="$source_dir/$here/publish-inputs.json"
[[ -f $inputs && ! -L $inputs ]] || { echo 'missing frozen publication inputs' >&2; exit 2; }
jq -e '
  .schema=="point-index.publish-inputs/v1" and .status=="frozen"
  and .experiment_submitted==false
  and (.baseline_revision|test("^[a-f0-9]{40}$"))
  and (.candidate_revision|test("^[a-f0-9]{40}$"))
  and .baseline_revision!=.candidate_revision
  and all(.spec_sha256,.protocol_sha256; type=="string" and test("^[a-f0-9]{64}$"))
  and all(.spec_path,.protocol_path; type=="string" and test("^[a-zA-Z0-9_./-]+\\.json$"))
' "$inputs" >/dev/null || { echo 'publication inputs are not frozen for this driver' >&2; exit 2; }

checked_path() {
  local relative=$1 full
  [[ $relative == "$here/"* && $relative != *../* && $relative != /* ]] || return 1
  full="$source_dir/$relative"
  [[ -f $full && ! -L $full ]] || return 1
  [[ $(realpath -- "$full") == "$source_dir/$here/"* ]] || return 1
  printf '%s' "$full"
}
spec_path=$(jq -er .spec_path "$inputs")
protocol_path=$(jq -er .protocol_path "$inputs")
spec=$(checked_path "$spec_path") || { echo 'unsafe or missing spec' >&2; exit 2; }
protocol=$(checked_path "$protocol_path") || { echo 'unsafe or missing protocol' >&2; exit 2; }
[[ $(sha256sum "$spec" | cut -d' ' -f1) == $(jq -er .spec_sha256 "$inputs") ]] || { echo 'spec hash mismatch' >&2; exit 2; }
[[ $(sha256sum "$protocol" | cut -d' ' -f1) == $(jq -er .protocol_sha256 "$inputs") ]] || { echo 'protocol hash mismatch' >&2; exit 2; }
jq -e '.schema=="experiment.protocol/v1" and .freeze.status=="frozen" and .freeze.filed_before_affected_data==true' "$protocol" >/dev/null || { echo 'protocol is not frozen' >&2; exit 2; }
jq -e --slurpfile publication "$inputs" '
  .schema=="point-index.panel.spec/v1" and .study_id=="crypto-point-index-fresh-panel-20261127"
  and .baseline_revision==$publication[0].baseline_revision
  and .candidate_revision==$publication[0].candidate_revision
  and .experiment_submitted==false
  and .threads==1 and .deadline_secs==1800
  and .aa_rounds>=5 and .ab_rounds>=5 and .repetitions==3
  and (.driver_crate_tree|test("^[a-f0-9]{40}$"))
  and all(.builder_sha256,.measurement_overlay_sha256,.panel_manifest_sha256,
    .seed_manifest_sha256,.inputs_sha256,.optimized_options_sha256;
    type=="string" and test("^[a-f0-9]{64}$"))
  and all(.panel_dir,.inputs_path,.optimized_options_path;
    type=="string" and test("^[a-zA-Z0-9_./-]+$"))
' "$spec" >/dev/null || { echo 'invalid native pair spec' >&2; exit 2; }
[[ $(git -C "$source_dir" rev-parse HEAD:crates/hashed-ecdl-tensor) == $(jq -er .driver_crate_tree "$spec") ]] || { echo 'driver native source tree changed' >&2; exit 2; }
for pair in "build-pair.sh:builder_sha256" "native-metrics.patch:measurement_overlay_sha256"; do
  path=${pair%:*}; key=${pair#*:}
  [[ -f $source_dir/$here/$path && ! -L $source_dir/$here/$path ]] || { echo "missing driver source: $path" >&2; exit 2; }
  [[ $(sha256sum "$source_dir/$here/$path" | cut -d' ' -f1) == $(jq -er ".$key" "$spec") ]] || { echo "driver source changed: $path" >&2; exit 2; }
done

panel_dir=$(jq -er .panel_dir "$spec")
inputs_path=$(jq -er .inputs_path "$spec")
options_path=$(jq -er .optimized_options_path "$spec")
[[ $panel_dir == "$here/"* && $panel_dir != *../* && $panel_dir != /* ]] || { echo 'unsafe panel path' >&2; exit 2; }
panel="$source_dir/$panel_dir"
[[ -d $panel && ! -L $panel ]] || { echo 'missing or symlinked panel' >&2; exit 2; }
[[ $(realpath -- "$panel") == "$source_dir/$here/"* ]] || { echo 'panel escapes the study' >&2; exit 2; }
hashes=$(checked_path "$inputs_path") || { echo 'unsafe or missing input hash list' >&2; exit 2; }
options=$(checked_path "$options_path") || { echo 'unsafe or missing options' >&2; exit 2; }
for pair in "manifest.json:panel_manifest_sha256" "seed-manifest.json:seed_manifest_sha256"; do
  path=${pair%:*}; key=${pair#*:}
  [[ -f $panel/$path && ! -L $panel/$path ]] || { echo "missing or symlinked $path" >&2; exit 2; }
  [[ $(sha256sum "$panel/$path" | cut -d' ' -f1) == $(jq -er ".$key" "$spec") ]] || { echo "$key mismatch" >&2; exit 2; }
done
[[ $(sha256sum "$hashes" | cut -d' ' -f1) == $(jq -er .inputs_sha256 "$spec") ]] || { echo 'input hash-list mismatch' >&2; exit 2; }
[[ $(sha256sum "$options" | cut -d' ' -f1) == $(jq -er .optimized_options_sha256 "$spec") ]] || { echo 'optimized policy mismatch' >&2; exit 2; }
jq -e 'type=="object" and (keys==["batch_chunk","extra_scans","joint_targets","relation_graph","subgroup_provenance","walk"])
  and .batch_chunk==1 and .extra_scans==0 and .joint_targets==true
  and .relation_graph==false and .subgroup_provenance==false and .walk==null
' "$options" >/dev/null || { echo 'unexpected optimized policy' >&2; exit 2; }
declare -A listed=()
while IFS= read -r line; do
  [[ ${#line} -ge 67 && ${line:64:2} == '  ' ]] || { echo 'malformed input hash line' >&2; exit 2; }
  digest=${line:0:64}
  path=${line:66}
  [[ $digest =~ ^[a-f0-9]{64}$ ]] || { echo 'malformed input digest' >&2; exit 2; }
  [[ $path == manifest.json || $path == seed-manifest.json || $path =~ ^problems/[a-zA-Z0-9-]+\.json$ ]] || { echo 'unsafe input hash path' >&2; exit 2; }
  [[ ! -v listed[$path] ]] || { echo 'duplicate input hash path' >&2; exit 2; }
  listed[$path]=1
done < "$hashes"
[[ ${#listed[@]} == 86 && -v listed[manifest.json] && -v listed[seed-manifest.json] ]] || { echo 'incomplete input hash list' >&2; exit 2; }
while IFS= read -r problem_path; do
  [[ -v listed[$problem_path] ]] || { echo 'unlisted public problem' >&2; exit 2; }
done < <(jq -r '.cases[].problem' "$panel/manifest.json")
(cd "$panel" && sha256sum -c "$hashes" >/dev/null) || { echo 'panel input readback mismatch' >&2; exit 2; }
jq -e --slurpfile publication "$inputs" --slurpfile spec "$spec" '
  .schema=="hashed-ecdl-panel/v2" and .status=="prepared"
  and .baseline==$publication[0].baseline_revision
  and .paired_baseline==$publication[0].candidate_revision
  and .seed_manifest_sha256==$spec[0].seed_manifest_sha256
  and .development_inputs==60 and .holdout_inputs==24
  and .repetitions==3 and .required_aa_rounds==5 and .required_ab_rounds==5
  and .execution=="not_run" and .measurements==null and (.cases|length)==84
' "$panel/manifest.json" >/dev/null || { echo 'panel manifest is not source-frozen' >&2; exit 2; }
jq -e ' .schema=="point-index-fresh-seeds/v1" and .study_id=="crypto-point-index-fresh-panel-20261127"
  and (.cases|length)==84 and ([.cases[].fixture_seed]|unique|length)==84
  and (.schedule_seeds|length)==3 and ([.schedule_seeds[]]|unique|length)==3
' "$panel/seed-manifest.json" >/dev/null || { echo 'seed manifest is incomplete' >&2; exit 2; }
echo 'Frozen native pair inputs verified without AWS; no experiment submitted.'
