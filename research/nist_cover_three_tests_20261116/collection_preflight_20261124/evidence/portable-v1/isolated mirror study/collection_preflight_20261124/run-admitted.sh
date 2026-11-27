#!/bin/bash
# Prepared driver only. --admitted acknowledges an externally admitted job;
# neither this flag nor the envelope's presence creates budget authorization.
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 3
test "$1" = --admitted
test -f "$2"
campaign_record=$(realpath "$2")
mkdir "$3"
out=$(cd "$3" && pwd)
cp "$campaign_record" "$out/campaign-envelope.json"
printf '%s\n' 'This script must run only after external campaign admission; no admission is inferred from this file.' > "$out/admission-scope.txt"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
run_pipeline() {
  set -e
  # Source and synthetic qualification is repeated on the admitted image so
  # each actual worker binary is bound to its host/source/toolchain record.
  bash ../split_tangent_20261123/qualify.sh "$out/qualification"
  bash qualify.sh "$out/preflight-qualification"
  mkdir "$out/compiler-guard"
  native_compiler=$(command -v c++)
  printf '#!/bin/bash\nexec timeout 300 %q "$@"\n' "$native_compiler" > "$out/compiler-guard/c++"
  chmod 0555 "$out/compiler-guard/c++"
  set +e
  (ulimit -v 1048576; { time env PATH="$out/compiler-guard:$PATH" bash ../check-relation-e2e.sh "$out/baseline-e2e" > "$out/baseline.stdout" 2> "$out/baseline.stderr"; } 2> "$out/baseline-cost.json")
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/BASELINE_EXIT_CODE.txt"
  test "$status" -eq 0
  mkdir "$out/input"
  cp -r "$out/baseline-e2e/relations/input/." "$out/input/"
  cp "$out/baseline-e2e/relations/result.json" "$out/input/baseline.json"
  cp ../recorded_pool_20261123/evidence/traces-v1/result.json "$out/input/tangents.json"
  cp ../recorded_pool_20261123/evidence/qualification-v2/input/baseline.json "$out/input/reference.json"
  cp run-admitted.sh PROTOCOL.md pins.hpp ../recorded_pool_20261123/TANGENT_COLLECTION_PROTOCOL.md "$out/input/"
  sha256sum "$out/preflight-qualification/bin/preflight" "$out/input/reference.json" "$out/input/baseline.json" \
    "$out/input/backmap.json" "$out/input/inputs/"*.json "$out/input/evidence/"*.json > "$out/preflight-input-manifest.sha256"
  set +e
  (ulimit -v 1048576; { time timeout 30 "$out/preflight-qualification/bin/preflight" --check "$out/input/reference.json" \
    "$out/input/baseline.json" "$out/input" "$out/preflight-result.json" > "$out/preflight.stdout" 2> "$out/preflight.stderr"; } 2> "$out/preflight-cost.json")
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/PREFLIGHT_EXIT_CODE.txt"
  test "$status" -eq 0
  sha256sum "$out/qualification/source/"* "$out/qualification/bin/"* "$out/preflight-qualification/source/"* \
    "$out/preflight-qualification/bin/"* "$out/input/"*.json "$out/input/"*.md "$out/input/"*.sh "$out/input/"*.hpp \
    "$out/input/inputs/"*.json "$out/input/evidence/"*.json "$out/preflight-result.json" "$out/campaign-envelope.json" \
    > "$out/pre-collection-manifest.sha256"
  set +e
  (ulimit -v 1048576; { time timeout 600 "$out/qualification/bin/collector" --collect "$out/input" "$out/input/baseline.json" \
    "$out/input/tangents.json" "$out/result.json" > "$out/collector.stdout" 2> "$out/collector.stderr"; } 2> "$out/collector-cost.json")
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/COLLECTOR_EXIT_CODE.txt"
  test "$status" -eq 0
  sha256sum "$out/result.json" > "$out/checker-input-manifest.sha256"
  set +e
  (ulimit -v 1048576; { time timeout 600 "$out/qualification/bin/checker" --check "$out/input/evidence/p192-d7-kernel3-plane-quartic.json" \
    "$out/input/backmap.json" "$out/input/baseline.json" "$out/input/tangents.json" "$out/result.json" "$out/checker-result.json" \
    > "$out/checker.stdout" 2> "$out/checker.stderr"; } 2> "$out/checker-cost.json")
  status=$?
  set -e
  printf '%s\n' "$status" > "$out/CHECKER_EXIT_CODE.txt"
  test "$status" -eq 0
  sha256sum ../split_tangent_20261123/collector.cpp ../split_tangent_20261123/checker.cpp \
    ../split_tangent_20261123/rank_roots.hpp ../split_tangent_20261123/qualify.sh \
    ../split_tangent_20261123/PROTOCOL.md ../split_tangent_20261123/QUALIFICATION_NOTES.md \
    preflight.cpp typed_json.hpp pins.hpp qualify.sh run-admitted.sh PROTOCOL.md \
    > "$out/post-call-source-manifest.sha256"
}
set +e
{ time (run_pipeline); } 2> "$out/pipeline-cost.json"
status=$?
set -e
printf '%s\n' "$status" > "$out/PIPELINE_EXIT_CODE.txt"
exit "$status"
