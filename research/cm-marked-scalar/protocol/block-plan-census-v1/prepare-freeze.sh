#!/usr/bin/env bash
set -euo pipefail
cm_dir=research/cm-marked-scalar
cm_p=$cm_dir/protocol/block-plan-census-v1
cm_build=/tmp/cm-marked-scalar-build
test ! -e "$cm_p/protocol.json"
cm_commit=$(git rev-parse HEAD)
cm_inputs=$(for cm_path in "$cm_dir/fixtures-v2/published-curves.json" "$cm_dir/fixtures-v2/marked-relations.json" "$cm_dir/fixtures-v2/"p-{192,224,256,384,521}-closure.json "$cm_dir/residual-v1/plans.json"; do
  jq -n --arg path "$cm_path" --arg sha "$(sha256sum "$cm_path" | cut -d' ' -f1)" '{path:$path,sha256:$sha}'
done | jq -s .)
cm_sources=$(for cm_name in block_tune.cpp block_tune_replay.cpp block_tune_mutate.cpp block_core.hpp cache_core.hpp native.cpp json.hpp relation_verify.hpp orbit_verify.hpp; do
  cm_path=$cm_dir/$cm_name
  jq -n --arg path "$cm_path" --arg sha "$(sha256sum "$cm_path" | cut -d' ' -f1)" '{path:$path,sha256:$sha}'
done | jq -s .)
jq --arg commit "$cm_commit" --arg binary "$(sha256sum "$cm_build/block-tune" | cut -d' ' -f1)" --arg replay "$(sha256sum "$cm_build/block-tune-replay" | cut -d' ' -f1)" --arg source "$(sha256sum "$cm_dir/block_tune_replay.cpp" | cut -d' ' -f1)" --argjson inputs "$cm_inputs" --argjson sources "$cm_sources" -f "$cm_p/make-protocol.jq" "$cm_dir/protocol/residual-v1/protocol.json" > "$cm_p/protocol.prepared.json"
cm_digest=$(jq -cjS 'del(.freeze)' "$cm_p/protocol.prepared.json" | sha256sum | cut -d' ' -f1)
jq --arg digest "$cm_digest" --arg utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" '.freeze.status="frozen"|.freeze.protocol_sha256=$digest|.freeze.frozen_at=$utc|.freeze.filed_before_affected_data=true' "$cm_p/protocol.prepared.json" > "$cm_p/protocol.json"
/tmp/cm-schema-native/validate "$cm_dir/protocol/experiment-protocol.v1.schema.json" "$cm_p/protocol.json" > "$cm_p/frozen-validation.log" 2>&1
