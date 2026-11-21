#!/usr/bin/env bash
# Synthetic contract mutations only: no solver or benchmark invocation.
set -euo pipefail
checker=${1:?native validator path required}
freezer=${2:?native freeze wrapper required}
study=$(cd "$(dirname "$0")" && pwd)
fixture=$(mktemp -d /tmp/round20-protocol-tests.XXXXXX)
trap 'rm -rf "$fixture"' EXIT
reject() {
  local label=$1
  shift
  if "$@" > "$fixture/stdout" 2> "$fixture/stderr"; then
    printf 'FAIL accepted %s\n' "$label"
    exit 1
  fi
  printf 'PASS rejected %s: ' "$label"
  head -c 300 "$fixture/stderr"
  printf '\n'
}
cp "$study/analyze.rs" "$fixture/analyze.rs"
cp "$study/protocol.json" "$fixture/protocol.json"
"$checker" validate-protocol "$fixture/protocol.json"
jq '.sample_size.replicates=8' "$study/protocol.json" > "$fixture/protocol.json"
reject frozen-digest "$checker" validate-protocol "$fixture/protocol.json"
jq '.variable_ledger.comparisons[0].arms[0]="missing"' "$study/protocol.draft.json" > "$fixture/protocol.json"
reject unknown-comparison-arm "$checker" validate-protocol "$fixture/protocol.json"
jq '.arms[0].id="v"' "$study/protocol.draft.json" > "$fixture/protocol.json"
reject duplicate-arm "$checker" validate-protocol "$fixture/protocol.json"
jq '.randomization.blinding.analysis_labels_masked=true' "$study/protocol.draft.json" > "$fixture/protocol.json"
reject unsupported-blinding "$checker" validate-protocol "$fixture/protocol.json"
cp "$study/protocol.draft.json" "$fixture/protocol.json"
printf '// synthetic wrong analyzer\n' > "$fixture/analyze.rs"
reject wrong-analyzer "$checker" validate-protocol "$fixture/protocol.json"
reject refreeze "$freezer" "$study/protocol.json" "$fixture/new.json" 2026-10-30T00:00:00Z dac149f065de6698ceee8d74bdfda3f365af31f3 --before-data
reject overwrite "$freezer" "$study/protocol.draft.json" "$fixture/protocol.json" 2026-10-30T00:00:00Z dac149f065de6698ceee8d74bdfda3f365af31f3 --before-data
