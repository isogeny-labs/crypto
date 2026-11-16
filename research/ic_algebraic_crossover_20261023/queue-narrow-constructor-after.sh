#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
meta=$study/pipeline-receipts/narrow-constructor-after-20261025-queue
test ! -e "$meta"
mkdir -p "$meta"
cp "$0" "$meta/queue.sh"
cp "$study/run-narrow-constructor-after.sh" "$meta/runner.sh"
date -u +%FT%TZ > "$meta/queued.txt"
for item in "${1:?build shell pid and start ticks}" "${2:?library gate shell pid and start ticks}"; do
  read -r pid start kind <<< "$item"
  test "$(awk '{print $22}' "/proc/$pid/stat")" = "$start"
  printf '%s %s %s\n' "$pid" "$start" "$kind" >> "$meta/watched.txt"
done
sha256sum "$meta/runner.sh" "$meta/queue.sh" "$study/NARROW_CONSTRUCTOR_EXCLUSION_PROTOCOL.md" > "$meta/hashes.txt"
while read -r pid start kind; do
  gate=$study/pipeline-receipts/narrow-constructor-$kind-20261025
  while ! test -f "$gate/ended.txt"; do
    if ! test -r "/proc/$pid/stat" || test "$(awk '{print $22}' "/proc/$pid/stat")" != "$start"; then
      printf '%s\n' 'A watched gate disappeared without an end marker; no candidate launch.' > "$meta/INCOMPLETE.txt"
      exit 1
    fi
    sleep 30
  done
  if ! jq -e '.exit_code == 0' "$gate/status.json" > /dev/null; then
    printf '%s\n' 'A build or full library gate failed; preserve its logs and review before running the candidate.' > "$meta/INCOMPLETE.txt"
    exit 1
  fi
done < "$meta/watched.txt"
date -u +%FT%TZ > "$meta/gates-passed.txt"
python3 tools/isolated_bench.py busy -- bash "$meta/runner.sh"
date -u +%FT%TZ > "$meta/ended.txt"
