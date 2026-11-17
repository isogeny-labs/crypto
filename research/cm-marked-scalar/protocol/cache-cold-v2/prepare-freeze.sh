#!/usr/bin/env bash
set -euo pipefail
# Metadata only. Run from the repository root after publishing the source commit.
cm_source_commit=$(git rev-parse HEAD)
cm_dir=research/cm-marked-scalar
cm_protocol_dir=$cm_dir/protocol/cache-cold-v2
cm_build=/tmp/cm-marked-scalar-build
test ! -e "$cm_protocol_dir/protocol.json"
env OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1 "$cm_build/cache-analysis" --context > "$cm_protocol_dir/context.json"
cm_inputs=$(for cm_input in "$cm_dir/fixtures-v2/published-curves.json" "$cm_dir/fixtures-v2/marked-relations.json" "$cm_dir/fixtures-v2/"p-{192,224,256,384,521}-closure.json "$cm_dir/residual-v1/plans.json" "$cm_protocol_dir/points.json"; do
  jq -n --arg path "$cm_input" --arg sha "$(sha256sum "$cm_input" | cut -d' ' -f1)" '{path:$path,sha256:$sha}'
done | jq -s .)
jq --arg commit "$cm_source_commit" \
  --arg worker "$(sha256sum "$cm_build/cache-eval" | cut -d' ' -f1)" \
  --arg header "$(sha256sum "$cm_dir/cache_core.hpp" | cut -d' ' -f1)" \
  --arg worker_source "$(sha256sum "$cm_dir/cache_eval.cpp" | cut -d' ' -f1)" \
  --arg analyzer "$(sha256sum "$cm_build/cache-analysis" | cut -d' ' -f1)" \
  --arg analysis_source "$(sha256sum "$cm_dir/cache_analyze.cpp" | cut -d' ' -f1)" \
  --arg isolation "$(sha256sum "$cm_build/isolate" | cut -d' ' -f1)" \
  --arg point_checker "$(sha256sum "$cm_build/cache-points" | cut -d' ' -f1)" \
  --arg point_source "$(sha256sum "$cm_dir/cache_points.cpp" | cut -d' ' -f1)" \
  --argjson inputs "$cm_inputs" --slurpfile context "$cm_protocol_dir/context.json" '
  .arms[].instrument.commit=$commit |
  .arms[].instrument.binary_sha256=$worker |
  .metrics.primary[0].source="rows.jsonl total_ns; one complete cold call per row; no division" |
  .controls.positive.not_applicable_reason="Software qualification: 625 cold paths, 6300 schedules, 71570 retained labels, 60 independent GMP coordinate/subgroup replays and 24 synthetic analyzer checks including planted gain. No timed positive-control arm is required for the deterministic finite-panel comparison." |
  .controls.negative.not_applicable_reason="Software corruption controls: altered expected outputs/residuals, three independent point corruptions and native row/receipt/decision tests reject; A/A twins provide measured null comparisons." |
  .analysis_plan.script.sha256=$analysis_source |
  .analysis_plan.frozen_before_data=true |
  .execution.placement += {primary_arm:"cache-default",input_files:$inputs,cache_header_sha256:$header,worker_source_sha256:$worker_source,isolation_binary_sha256:$isolation,analysis_binary_sha256:$analyzer,point_checker_binary_sha256:$point_checker,point_checker_source_sha256:$point_source,expected_host:$context[0].expected_host,expected_switches:$context[0].expected_switches} |
  .freeze.commit=$commit
  ' "$cm_protocol_dir/protocol.draft.json" > "$cm_protocol_dir/protocol.prepared.json"
cm_digest=$(jq -cjS 'del(.freeze)' "$cm_protocol_dir/protocol.prepared.json" | sha256sum | cut -d' ' -f1)
jq --arg digest "$cm_digest" --arg utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" '
  .freeze.status="frozen" | .freeze.protocol_sha256=$digest | .freeze.frozen_at=$utc | .freeze.filed_before_affected_data=true
  ' "$cm_protocol_dir/protocol.prepared.json" > "$cm_protocol_dir/protocol.json"
/tmp/cm-schema-native/validate "$cm_dir/protocol/experiment-protocol.v1.schema.json" "$cm_protocol_dir/protocol.json" > "$cm_protocol_dir/frozen-validation.log" 2>&1
