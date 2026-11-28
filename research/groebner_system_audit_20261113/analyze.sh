#!/usr/bin/env bash
set -u
cd /workspace/crypto-audit/research/groebner_system_audit_20261113
mkdir analysis || exit 2
timeout --signal=INT --kill-after=2s 120s /workspace/scratch/groebner-system-native/analyze \
  "$PWD" --main > analysis/analysis.jsonl 2> analysis/analysis.stderr
rc=$?
printf '%s\n' "$rc" > analysis/analysis.exit
exit "$rc"
