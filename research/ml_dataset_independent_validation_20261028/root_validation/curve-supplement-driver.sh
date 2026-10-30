#!/usr/bin/env bash
# Additive software coverage fixture; never replace prior failures.
set -euo pipefail
phase=${1:?prepare or generate}
evidence=${2:?fresh supplement directory}
binary=${3:?previously pinned source-bound binary}
here=$(cd "$(dirname "$0")" && pwd)
test "$(sha256sum "$binary" | cut -d' ' -f1)" = c01e024156067799356f590a0c3ac749da509305bd569e26c9bf084d5402d04a
case "$phase" in
 prepare)
  test ! -e "$evidence"
  mkdir -p "$evidence"
  cp "$here/inputs/curve-supplement.protocol.json" "$evidence/protocol.json"
  protocol_sha=$(sha256sum "$evidence/protocol.json" | cut -d' ' -f1)
  timeout --kill-after=5 60 env -u ISOLAB_RESULTS_URI "$binary" dataset plan --out "$evidence/plan.json" \
    --n 6 --m 2 --families binary --solvers root --curve-seeds 2 --generators 1 --keys 1 --seed 1 \
    --factor-base-size 2 --degree-bound 3 --work-limit 5000 --protocol-sha256 "$protocol_sha" \
    > "$evidence/plan.stdout.json" 2> "$evidence/plan.stderr.log"
  timeout --kill-after=5 60 env -u ISOLAB_RESULTS_URI "$binary" dataset preflight --plan "$evidence/plan.json" \
    --protocol "$evidence/protocol.json" > "$evidence/preflight.json" 2> "$evidence/preflight.stderr.log"
  jq -e '.source_commit=="d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83" and .source_dirty==false and .generated_cells==0' "$evidence/preflight.json" > /dev/null
  ;;
 generate)
  # Caller must checkpoint frozen inputs/handoff before this phase.
  test ! -e "$evidence/run"
  timeout --kill-after=5 60 env -u ISOLAB_RESULTS_URI "$binary" dataset generate --plan "$evidence/plan.json" \
    --protocol "$evidence/protocol.json" --out "$evidence/run" --max-cases 1 --shard-index 2 --shard-count 4 --seconds-per-case 5 \
    --memory-mib 512 --trace-mib 8 > "$evidence/generate.stdout.json" 2> "$evidence/generate.stderr.log"
  ;;
 *) exit 2 ;;
esac
