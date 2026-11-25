#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 2
qualified=$(cd "$1" && pwd)
mkdir -p "$(dirname "$2")"
mkdir "$2"
out=$(cd "$2" && pwd)
cp check-query-guards.sh "$out/source.sh"
sha256sum "$qualified/bin/audit" "$qualified/u.json" "$qualified/v.json" "$qualified/w.json" "$qualified/templates/result.json" > "$out/before.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
set +e
(ulimit -v 1048576; { time timeout 30 "$qualified/bin/audit" --u \
  ../recorded_pool_20261123/evidence/signed-v2/input/baseline.json \
  ../recorded_pool_20261123/evidence/traces-v1/result.json \
  ../recorded_pool_20261123/evidence/signed-v2/result.json \
  ../two_row_20261124/evidence/qualification-v2/result.json \
  "$qualified/templates/result.json" "$qualified/u.json" > "$out/collision.stdout" 2> "$out/collision.stderr"; } 2> "$out/collision-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/COLLISION_EXIT_CODE.txt"
test "$status" -eq 1
set +e
(ulimit -v 1048576; { time timeout 30 "$qualified/bin/audit" --u > "$out/missing.stdout" 2> "$out/missing.stderr"; } 2> "$out/missing-cost.json")
status=$?
set -e
printf '%s\n' "$status" > "$out/MISSING_EXIT_CODE.txt"
test "$status" -eq 1
sha256sum -c "$out/before.sha256" > "$out/unchanged.stdout"
sha256sum check-query-guards.sh "$out/source.sh" > "$out/source-binding.sha256"
