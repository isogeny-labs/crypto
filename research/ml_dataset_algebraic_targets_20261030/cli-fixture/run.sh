#!/bin/bash
# Native CLI orchestration and JSON/hash checks only; no cryptographic test engine.
set -euo pipefail
binary=$(realpath "$1")
output=$2
source_root=${3:-/home/ec2-user/ml-dataset-work-20261028/startup-main-cli-fixture-run}
reference_root=${4:-/home/ec2-user/ml-dataset-work-20261028/prefix-cli-fixture-run-03}
fixture_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
[[ ! -e "$output" ]]
mkdir -p "$output"
printf '%s  %s\n' e3f0d56284a7c32c90ead0f0ad9049801c809b2362f8c8caf07e2baf064e9d3f "$source_root/receipt.json" | sha256sum -c - > "$output/input-receipt-check.txt"
sha256sum "$binary" > "$output/binary.sha256"
while IFS=$'\t' read -r relative expected_sha; do
  printf '%s  %s\n' "$expected_sha" "$reference_root/$relative" | sha256sum -c - >> "$output/reference-check.txt"
done < <(jq -r '.reference_features[]|[.path,.sha256]|@tsv' "$fixture_dir/protocol.json")
(cd "$source_root" && find . -type f -print0 | sort -z | xargs -0 sha256sum) > "$output/input-before.sha256"
for shard in 62 63; do
  expected_cell=$(jq -ce --argjson shard "$shard" '.source_cells[]|select(.shard==$shard)' "$fixture_dir/protocol.json")
  manifest_path=$(jq -r .manifest_path <<< "$expected_cell")
  manifest_sha=$(jq -r .manifest_sha256 <<< "$expected_cell")
  printf '%s  %s\n' "$manifest_sha" "$source_root/$manifest_path" | sha256sum -c - >> "$output/source-manifest-check.txt"
  jq -e --argjson shard "$shard" --arg sha "$manifest_sha" 'any(.verified_local_cells[]; .cell_index==$shard and .manifest_sha256==$sha)' "$source_root/receipt.json" > /dev/null
  reference_audit=$(jq -r .reference_audit_path <<< "$expected_cell")
  reference_audit_sha=$(jq -r .reference_audit_sha256 <<< "$expected_cell")
  printf '%s  %s\n' "$reference_audit_sha" "$reference_root/$reference_audit" | sha256sum -c - >> "$output/reference-check.txt"
  jq -se --argjson expected "$expected_cell" '[.[]|select(.record.kind=="ideal_equality_audit")|{node:.record.node_id,rows:(.record.certificate.certificate.right_from_left|length)}]|sort_by(.node)==$expected.expected_basis_rows_by_node' "$reference_root/$reference_audit" > /dev/null
  destination=$output/export-$shard
  "$binary" dataset export --out "$source_root/shard-$shard" --destination "$destination" > "$output/export-$shard.stdout.json" 2> "$output/export-$shard.stderr.log"
  "$binary" dataset export --out "$source_root/shard-$shard" --destination "$destination" > "$output/retry-$shard.stdout.json" 2> "$output/retry-$shard.stderr.log"
  cmp "$output/export-$shard.stdout.json" "$output/retry-$shard.stdout.json"
  jq -e '.feature_policy.version=="strict-structural-allowlist-v6" and .feature_policy.model_input_files==["features.jsonl","prefix-features.jsonl","algebraic-features.jsonl"]' "$destination/manifest.json" > /dev/null
  jq -r '.artifacts[]|[.path,.sha256,(.bytes|tostring)]|@tsv' "$destination/manifest.json" |
  while IFS=$'\t' read -r relative expected_sha expected_bytes; do
    [[ "$relative" != /* && "$relative" != *..* ]]
    printf '%s  %s\n' "$expected_sha" "$destination/$relative" | sha256sum -c - >> "$output/artifact-readback.txt"
    [[ $(stat -c %s "$destination/$relative") == "$expected_bytes" ]]
  done
  for name in features.jsonl prefix-features.jsonl; do
    cmp "$reference_root/export-$shard/$name" "$destination/$name"
  done
  for name in features.jsonl prefix-features.jsonl algebraic-features.jsonl; do
    while IFS= read -r row; do
      expected_id=$(jq -r .feature_id <<< "$row")
      canonical=$(jq -cS .inputs <<< "$row")
      [[ $(printf %s "$canonical" | sha256sum | cut -d' ' -f1) == "$expected_id" ]]
    done < "$destination/$name"
  done
  jq -n -e --argjson expected "$expected_cell" --slurpfile static "$reference_root/export-$shard/features.jsonl" --slurpfile features "$destination/algebraic-features.jsonl" --slurpfile records "$destination/restricted/algebraic-records.jsonl" --slurpfile manifests "$destination/manifest.json" '
    def exact_keys($expected_keys): type=="object" and keys==($expected_keys|sort);
    def natural: type=="number" and .>=0 and floor==.;
    $manifests[0] as $m |
    ($static|length)==1 and
    $m.algebraic_outcomes.verified_nonzero_target_rows==$expected.expected_verified_nonzero_rows and
    $m.algebraic_outcomes.verified_zero_target_rows==$expected.expected_verified_zero_rows and
    $m.algebraic_outcomes.bounded_unknown_suffix_records==$expected.expected_unknown_suffix_records and
    $m.algebraic_feature_occurrences==$expected.expected_verified_nonzero_rows and
    ($features|length)==$m.unique_algebraic_feature_rows and
    ($records|length)==$m.restricted_algebraic_record_rows and
    ([$records[]|select(.record.admission_status=="verified_nonzero")]|length)==$m.algebraic_outcomes.verified_nonzero_target_rows and
    ([$records[]|select(.record.admission_status=="verified_zero")]|length)==$m.algebraic_outcomes.verified_zero_target_rows and
    ([$records[]|select(.record.admission_status=="bounded_unknown_suffix")]|length)==$m.algebraic_outcomes.bounded_unknown_suffix_records and
    ([$records[]|select(.record.witnessed_degree_drop==true)]|length)==$m.algebraic_outcomes.witnessed_degree_reduction_rows and
    all($records[]; . as $r |
      (["verified_nonzero","verified_zero","bounded_unknown_suffix"]|index($r.record.admission_status))!=null and
      .record.native_operation_witness==false and .record.exact_first_fall_degree==null and .record.exact_last_fall_degree==null and .record.minimal_derivation_degree==null and
      (if .record.admission_status=="verified_nonzero" then
        any($features[]; .feature_id==$r.feature_id and .split==$r.split and .split_component==$r.split_component) and
        all(.record.certified_target.coefficient_edges[]; (.coefficient|type)=="string") and
        all(.record.multiplier_proof[]; (.coefficient|type)=="string")
       else .feature_id==null end)) and
    all($features[]; .inputs as $input |
      ($input|exact_keys(["task","initial","ring","active_original_variables","fixed_bindings","source_equations"])) and
      $input.task=="prime-polynomial-consequence/v1" and $input.initial==$static[0].inputs and
      ($input.ring|exact_keys(["modulus","variables","quotient_rules"])) and
      ($input.ring.modulus|type)=="string" and ($input.ring.variables|natural) and $input.ring.quotient_rules==[] and
      ($input.active_original_variables|type)=="array" and all($input.active_original_variables[]; natural) and
      ($input.active_original_variables|length)==$input.ring.variables and
      ($input.fixed_bindings|type)=="array" and
      all($input.fixed_bindings[]; exact_keys(["original_variable","value"]) and (.original_variable|natural) and (.value|type)=="string") and
      ($input.source_equations|exact_keys(["schema","variables","equations","monomials","coefficient_edges"])) and
      $input.source_equations.schema=="prime-prefix-polynomial-graph/v1" and
      $input.source_equations.variables==$input.ring.variables and ($input.source_equations.equations|natural) and
      ($input.source_equations.monomials|type)=="array" and
      all($input.source_equations.monomials[]; type=="array" and length==$input.ring.variables and all(.[]; natural)) and
      ($input.source_equations.coefficient_edges|type)=="array" and
      all($input.source_equations.coefficient_edges[]; exact_keys(["equation","monomial","coefficient"]) and
        (.equation|natural) and (.monomial|natural) and (.coefficient|type)=="string"))
  ' > "$output/algebraic-joins-$shard.json"
done
jq -se '[.[]|.algebraic_outcomes.verified_nonzero_target_rows]|add>0' "$output"/export-{62,63}/manifest.json > /dev/null
# Rehash a changed restricted consequence, then require deterministic re-export
# to reject the changed output. This is regeneration, not an independent engine.
cp -a "$output/export-62" "$output/tampered-export"
records=$output/tampered-export/restricted/algebraic-records.jsonl
jq -cs '(map(.record.admission_status=="verified_nonzero")|index(true)) as $i |
  if $i==null then error("no consequence to tamper") else
    .[$i].record.certified_target.coefficient_edges[0].coefficient |= (if .=="1" then "2" else "1" end)
  end | .[]' "$records" > "$output/changed-records.jsonl"
mv "$output/changed-records.jsonl" "$records"
changed_sha=$(sha256sum "$records" | cut -d' ' -f1)
changed_bytes=$(stat -c %s "$records")
jq --arg sha "$changed_sha" --argjson bytes "$changed_bytes" '(.artifacts[]|select(.path=="restricted/algebraic-records.jsonl")) |= (.sha256=$sha|.bytes=$bytes)' "$output/tampered-export/manifest.json" > "$output/changed-manifest.json"
mv "$output/changed-manifest.json" "$output/tampered-export/manifest.json"
set +e
"$binary" dataset export --out "$source_root/shard-62" --destination "$output/tampered-export" > "$output/tampered.stdout.json" 2> "$output/tampered.stderr.log"
negative_exit=$?
set -e
[[ "$negative_exit" -ne 0 ]]
rg -q 'artifact mismatch' "$output/tampered.stderr.log"
(cd "$source_root" && find . -type f -print0 | sort -z | xargs -0 sha256sum) > "$output/input-after.sha256"
cmp "$output/input-before.sha256" "$output/input-after.sha256"
jq -n --argjson negative_exit "$negative_exit" '{schema:"software-cli-fixture-result/v1",successful_export_retry_commands:4,rehashed_output_tamper_rejected:true,negative_exit:$negative_exit,source_unchanged:true,all_artifact_hashes_verified:true,static_and_branch_features_byte_equal_to_reference:true,algebraic_feature_identity_and_joins_checked:true,verification_scope:"native re-export with direct identity replay; not an independent implementation or standalone external-record validator",scientific_samples_added:0,verified_cloud_dataset_cells:0}' > "$output/result.json"
cat "$output/result.json"
