#!/usr/bin/env bash
set -euo pipefail
cm_dir=research/cm-marked-scalar
cm_p=$cm_dir/protocol/block-cold-v3
cm_build=/tmp/cm-marked-scalar-build
test ! -e "$cm_p/protocol.json"
cm_commit=$(git rev-parse HEAD)
env OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1 "$cm_build/block-analysis" --context > "$cm_p/context.json"
cm_sources=$(for cm_name in block_core.hpp block_eval.cpp block_analyze.cpp cache_core.hpp cache_eval.cpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp isolate.cpp; do
  cm_path=$cm_dir/$cm_name
  jq -n --arg path "$cm_path" --arg sha "$(sha256sum "$cm_path" | cut -d' ' -f1)" '{path:$path,sha256:$sha}'
done | jq -s .)
jq --arg commit "$cm_commit" --arg worker "$(sha256sum "$cm_build/block-eval" | cut -d' ' -f1)" --arg analyzer "$(sha256sum "$cm_build/block-analysis" | cut -d' ' -f1)" --arg analysis_source "$(sha256sum "$cm_dir/block_analyze.cpp" | cut -d' ' -f1)" --arg worker_source "$(sha256sum "$cm_dir/block_eval.cpp" | cut -d' ' -f1)" --arg isolation "$(sha256sum "$cm_build/isolate" | cut -d' ' -f1)" --argjson sources "$cm_sources" --slurpfile context "$cm_p/context.json" '
  walk(if type=="string" then gsub("cache-default";"block-default")|gsub("binary-default";"block-binary")|gsub("cache-disabled";"cache-default") else . end) |
  .protocol_id="CM-BLOCK-COLD-V3" | .version=3 | .created="2026-11-09" |
  .question="Does bounded block folding of retained multiples beat complete regular scalar multiplication after all per-call planning, point setup, normalization, verification and destruction costs, on P-192 primary and all five curves?" |
  .arms[].instrument.commit=$commit | .arms[].instrument.binary_sha256=$worker |
  .arms[].instrument.build=[["make","-B","-j1","-C","research/cm-marked-scalar","block-eval-binary"]] |
  .controls.positive.not_applicable_reason="Native qualifier verifies 300 exhaustive small minima, 2520 point actions, retained labels and planted multicolumn/leading cases; analyzer synthetic planted gain/noise controls are separate software tests, not timing data." |
  .controls.negative.not_applicable_reason="Altered programs and expected point outputs reject; analyzer rejects invalid matrices, source/receipt bindings, arithmetic subphase costs and A/A or primary selection errors." |
  .variable_ledger.factors += [{name:"block planner",treatment:"fixed",value:"maximum span16; exact D+A,negation,D,step lexicographic key; offset0/window heuristic unchanged",how:"Recode, bank-label construction, DP states/transitions/allocations and reconstruction all inside plan_ns. Point bank construction/normalization plus destruction are inside arithmetic_ns. No plan is reused across points or calls."}] |
  .metrics.secondary += [{id:"block_setup_ns",name:"Point-bank setup and loop/destruction subphases",unit:"ns",kind:"timed",definition:"For block arms bank_setup_ns+loop_bank_destroy_ns=arithmetic_ns exactly; these nested times are not added again to total",source:"raw block JSONL; null for reference-internal phases not exposed",aggregation:"per_row",resolution:"same monotonic clock",uncertainty:"range"},{id:"diagnostic_counts",name:"Complete group and planner event counts",unit:"events",kind:"counted",definition:"One untimed 125-row matrix across five curves and25 configurations: includes actual prefix/table D/A/N, retained points, DP transitions and folded blocks. Unknown API internals remain null. No time inferred from event counts.",source:"counts.jsonl; all clock fields zero and timing_performed=false",aggregation:"per_row",resolution:"exact native events",uncertainty:"none_exact"}] |
  .analysis_plan.script={path:"research/cm-marked-scalar/block_analyze.cpp",sha256:$analysis_source} |
  .execution.placement += {primary_arm:"block-default",worker_source_sha256:$worker_source,analysis_binary_sha256:$analyzer,isolation_binary_sha256:$isolation,source_files:$sources,expected_host:$context[0].expected_host,expected_switches:$context[0].expected_switches,count_invocations:1,count_rows:125} |
  .stopping_rules += [{rule:"Collect one125-row untimed diagnostic matrix before admission; native summary must validate before calibration. No clock is read in diagnostic mode. Failed diagnostic stops this attempt.",kind:"fixed_n",preregistered:true}] |
  .amendments += [{id:"A-BLOCK-COLD-V3",filed:"2026-11-09",reason:"User explicitly requested block folding on all five curves, all planning/setup costs, PR updated before execution and an actual e2e comparison. The prior cache attempt refusal remains unchanged.",changes:["block-default primary and matched block-binary/A-A reference","All arm dispatch/initialization charged inside cold boundary","Nested point-bank setup and loop/destruction costs","One frozen untimed125-row event diagnostic","Ten source dependencies bound alongside nine input files"],filed_before_affected_data:true}] |
  .freeze={status:"draft",protocol_sha256:null,commit:$commit,frozen_at:null,filed_before_affected_data:false}
  ' "$cm_dir/protocol/cache-cold-v2/protocol.json" > "$cm_p/protocol.prepared.json"
cm_digest=$(jq -cjS 'del(.freeze)' "$cm_p/protocol.prepared.json" | sha256sum | cut -d' ' -f1)
jq --arg digest "$cm_digest" --arg utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" '.freeze.status="frozen"|.freeze.protocol_sha256=$digest|.freeze.frozen_at=$utc|.freeze.filed_before_affected_data=true' "$cm_p/protocol.prepared.json" > "$cm_p/protocol.json"
/tmp/cm-schema-native/validate "$cm_dir/protocol/experiment-protocol.v1.schema.json" "$cm_p/protocol.json" > "$cm_p/frozen-validation.log" 2>&1
