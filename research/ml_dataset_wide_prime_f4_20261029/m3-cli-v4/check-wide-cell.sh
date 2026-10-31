#!/usr/bin/env bash
# Prospective metadata checker only: native Rust owns mathematical replay.
set -euo pipefail
cell=$1
export_dir=$2
protocol=$3
jq -e '.adapter_contract_confirmed and .schema_mapping_confirmed and .export_contract.confirmed and .export_contract.mode=="verified-chain-graph"' "$protocol" >/dev/null
jq -e -n --slurpfile p "$protocol" --slurpfile m "$cell/manifest.json" --slurpfile idx "$export_dir/restricted/index.jsonl" '
 ($idx|length)==1 and all($p[0].reserved_artifacts|to_entries[];
 . as $e | [$m[0].artifacts[]|select(.path==$e.key or .role==$e.value)] as $found |
 ($found|length)==1 and $found[0].path==$e.key and $found[0].role==$e.value and
 $found[0].encoding==(if $e.key|endswith(".jsonl") then "jsonl" else "json" end) and
 any($idx[0].source_artifacts[]; .path==$e.key and .role==$e.value))' >/dev/null
jq -s -e --slurpfile p "$protocol" '
 all(.[]; . as $row | all(paths as $path | select(($path[-1]|type)=="string") | $path;
 . as $path | (($p[0].forbidden_public_keys|index($path[-1]))==null) or
 ($path==["inputs","field","basis"] and ($row|getpath($path))=="canonical-residue")))
' "$export_dir/features.jsonl" >/dev/null
result="$cell/prime-f4-wide-m3-result.json"
jq -e --slurpfile p "$protocol" '
 .schema==$p[0].result_schema and .strategy==$p[0].strategy and
 .native_backend_invoked==true and .native_basis_derivation_verified==false and
 .execution_trace_complete_verified==false and .total_operation_count_verified==false and
 (.extraction_complete|type)=="boolean" and (.exhaustive_no_decomposition|type)=="boolean" and (.pdp_witness_verified|type)=="boolean" and
 ((.exhaustive_no_decomposition|not) or (.extraction_complete and (.pdp_witness_verified|not)))
' "$result" >/dev/null
# No natural-target success or full enumeration requirement. Positive and negative
# claims require native replay. An invoked capped result remains visibly capped.
jq -e -n --slurpfile r "$result" --slurpfile m "$cell/manifest.json" '
 $r[0] as $r | $m[0].outcome as $o |
 if (["timeout","trace_budget_exhausted","worker_failed"]|index($o.status))!=null
 then $o.solver_completed==false and $o.pdp_witness_verified!=true and $o.exhaustive_no_decomposition!=true
 else $o.native_backend_invoked==$r.native_backend_invoked and
 $o.pdp_witness_verified==$r.pdp_witness_verified and
 $o.exhaustive_no_decomposition==$r.exhaustive_no_decomposition end
' >/dev/null
jq -e -n --slurpfile f "$export_dir/features.jsonl" --slurpfile t "$export_dir/restricted/targets.jsonl" --slurpfile i "$export_dir/restricted/index.jsonl" --slurpfile m "$cell/manifest.json" --slurpfile public "$cell/public.json" '
 ($f|length)==1 and ($t|length)==1 and ($i|length)==1 and
 $f[0].schema=="ml-dataset.features/v1" and ($f[0].feature_id|type)=="string" and
 ($f[0].feature_id|test("^[0-9a-f]{64}$")) and $t[0].feature_status=="available" and
 $t[0].feature_id==$f[0].feature_id and $i[0].feature_id==$f[0].feature_id and
 $i[0].target_id==$t[0].target_id and $i[0].status=="verified_sealed_cell" and
 ($f[0].inputs|type)=="object" and ($f[0].inputs|length)>0 and
 $f[0].inputs.family==$m[0].config.family and $f[0].inputs.n==$m[0].config.n and
 $f[0].inputs.m==3 and $f[0].inputs.twist==$m[0].config.twist and
 $f[0].inputs.field.modulus==$public[0].field.modulus and $f[0].inputs.target==$public[0].target and
 ($f[0].inputs.prime_chain_graph|type)=="object" and ($f[0].inputs.prime_chain_graph|length)>0
' >/dev/null
manifest_sha=$(sha256sum "$cell/manifest.json"|cut -d ' ' -f1)
jq -s -e --arg sha "$manifest_sha" 'length==1 and .[0].source_manifest_sha256==$sha' "$export_dir/restricted/index.jsonl" >/dev/null
jq -n --slurpfile r "$result" '{scope:"bounded m3 software fixture only",native_result:$r[0],native_basis_derivation_verified:false,independent_consumer_m3_extraction_verified:false,independent_consumer_m3_negative_verified:false}'
