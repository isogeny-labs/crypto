#!/usr/bin/env bash
set -u
ic_science_root=/workspace/crypto-native-hky-run-20261026
ic_plan=/workspace/crypto-current-base-20261026/research/ic_algebraic_crossover_20261023/pipeline-receipts/native-hky-resume-20261027/plan.json
ic_deadline=$(jq -er '.original_absolute_deadline_unix_seconds' "$ic_plan") || exit 1
ic_now=$(date -u +%s)
ic_remaining=$((ic_deadline - ic_now))
if [ "$ic_remaining" -le 0 ]; then exit 124; fi
cd "$ic_science_root" || exit 1
export CHECKPOINT_TOKEN_FILE=/workspace/.ic-private-checkpoint/upload-token
set +e
timeout -k 60 "$ic_remaining" bash research/ic_algebraic_crossover_20261023/run-hky-restart-20261026.sh --resume
ic_code=$?
set -e
date -u +%Y-%m-%dT%H:%M:%SZ > /workspace/ic-native-resume-ended-20261027.txt
jq -n --argjson code "$ic_code" --argjson deadline "$ic_deadline" '{exit_code:$code,original_deadline_unix_seconds:$deadline,scope:"outer launcher status only; case records remain authoritative"}' > /workspace/ic-native-resume-launcher-status-20261027.json
exit "$ic_code"
