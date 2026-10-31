#!/usr/bin/env bash
# Metadata-only checks. Arithmetic replay belongs to native dataset verify.
set -euo pipefail
cell=$1
export_dir=$2
protocol=$3
jq -e '.adapter_contract_confirmed == true' "$protocol" >/dev/null
jq -e -n --slurpfile p "$protocol" --slurpfile m "$cell/manifest.json" --slurpfile idx "$export_dir/restricted/index.jsonl" '
  ($idx | length) == 1 and
  all($p[0].reserved_artifacts | to_entries[];
    . as $expected |
    ([$m[0].artifacts[] | select(.path == $expected.key or .role == $expected.value)]) as $found |
    ($found | length) == 1 and
    $found[0].path == $expected.key and $found[0].role == $expected.value and
    any($idx[0].source_artifacts[]; .role == $expected.value))
' >/dev/null
jq -e --slurpfile p "$protocol" '
  getpath($p[0].result_pointers.native_backend_invoked) == true and
  getpath($p[0].result_pointers.native_basis_derivation_verified) == false
' "$cell/prime-f4-wide-result.json" >/dev/null
jq -s -e --slurpfile p "$protocol" '
  all(.[]; . as $row | all(paths as $path | select(($path[-1]|type)=="string") | $path;
    . as $path | (($p[0].forbidden_public_keys | index($path[-1])) == null) or
    ($path == ["inputs","field","basis"] and ($row|getpath($path)) == "canonical-residue")))
' "$export_dir/features.jsonl" >/dev/null
# Assertions below describe full six-cell fixture acceptance, not all partials.
jq -e --slurpfile p "$protocol" '
  .strategy == $p[0].adapter_metadata.strategy and
  .execution_trace_complete_verified == false and
  .total_operation_count_verified == false and
  (.limits | type) == "object" and
  .limits.encoded_bytes == $p[0].adapter_metadata.expected_encoded_bytes and
  (.bytes_reserved | type) == "number" and
  .bytes_reserved >= 0 and
  .bytes_reserved <= .limits.bytes
' "$cell/prime-f4-wide-result.json" >/dev/null
jq -s -e 'all(.[]; (.bytes_reserved | type) == "number" and .bytes_reserved >= 0)' "$cell/prime-f4-wide-events.jsonl" >/dev/null
jq -e --slurpfile p "$protocol" '
  all(.artifacts[];
    . as $a | if ($p[0].reserved_artifacts[$a.path] != null)
    then .encoding == (if .path == "prime-f4-wide-events.jsonl" then "jsonl" else "json" end)
    else true end)
' "$cell/manifest.json" >/dev/null
# Exact nonempty structural row and identity association; no invented hash format.
jq -e -n --slurpfile f "$export_dir/features.jsonl" --slurpfile t "$export_dir/restricted/targets.jsonl" --slurpfile i "$export_dir/restricted/index.jsonl" --slurpfile m "$cell/manifest.json" --slurpfile public "$cell/public.json" '
  ($f|length)==1 and ($t|length)==1 and ($i|length)==1 and
  $f[0].schema=="ml-dataset.features/v1" and
  ($f[0].feature_id|type)=="string" and
  ($f[0].feature_id|test("^[0-9a-f]{64}$")) and
  $t[0].feature_status=="available" and
  $t[0].feature_id==$f[0].feature_id and $i[0].feature_id==$f[0].feature_id and
  $i[0].target_id==$t[0].target_id and $i[0].status=="verified_sealed_cell" and
  ($f[0].inputs|type)=="object" and ($f[0].inputs|length)>0 and
  $f[0].inputs.family==$m[0].config.family and
  $f[0].inputs.n==$m[0].config.n and $f[0].inputs.m==$m[0].config.m and
  $f[0].inputs.twist==$m[0].config.twist and
  $f[0].inputs.field.modulus==$public[0].field.modulus and
  $f[0].inputs.target==$public[0].target and
  ($f[0].inputs|has("prime_chain_graph")|not)
' >/dev/null
manifest_sha=$(sha256sum "$cell/manifest.json" | cut -d ' ' -f1)
jq -s -e --arg sha "$manifest_sha" 'length==1 and .[0].source_manifest_sha256==$sha' "$export_dir/restricted/index.jsonl" >/dev/null
jq -n --slurpfile r "$cell/prime-f4-wide-result.json" '{native_invocation_verified:true,native_basis_derivation_verified:false,native_result:$r[0],feature_scope:"existing public static structural view only; no wide polynomial graph identity",independent_consumer_wide_extraction_verified:false,scope:"native semantic verification required; capped is not completed"}'
