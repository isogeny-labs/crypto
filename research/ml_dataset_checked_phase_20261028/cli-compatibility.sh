#!/usr/bin/env bash
# Native CLI wiring only: fixed metadata, local directory store, no worker or
# scientific corpus. Preserve every output under the caller's fresh directory.
set -euo pipefail
baseline=$1
candidate=$2
evidence=$3
test ! -e "$evidence"
mkdir -p "$evidence/sentinel"
evidence=$(realpath "$evidence")
baseline=$(realpath "$baseline")
candidate=$(realpath "$candidate")
test "$(sha256sum "$baseline" | cut -d' ' -f1)" = 968a316daab4084ef61fd46c779832b9ebd9e482e58492674bc82174c4b1cfb7
sha256sum "$baseline" "$candidate" > "$evidence/binaries.sha256"
cat > "$evidence/sentinel/aws" <<'SENTINEL'
#!/usr/bin/env bash
echo 'unexpected AWS call in local compatibility gate' >&2
touch "$CRYPTO_CHECKED_AWS_SENTINEL"
exit 97
SENTINEL
chmod +x "$evidence/sentinel/aws"
export PATH="$evidence/sentinel:$PATH"
export CRYPTO_CHECKED_AWS_SENTINEL="$evidence/aws-was-called"
unset ISOLAB_RESULTS_URI
printf '%s' '{"purpose":"checked phase CLI compatibility metadata fixture","schema":"experiment.protocol/v1"}' > "$evidence/protocol-body.json"
body_sha=$(sha256sum "$evidence/protocol-body.json" | cut -d' ' -f1)
jq --arg hash "$body_sha" '. + {freeze:{status:"frozen",protocol_sha256:$hash,filed_before_affected_data:true,frozen_at:"2026-10-28T00:00:00Z"}}' "$evidence/protocol-body.json" > "$evidence/protocol.json"
protocol_sha=$(sha256sum "$evidence/protocol.json" | cut -d' ' -f1)
"$baseline" dataset plan --out "$evidence/plan.json" --n 7 --m 2 --families koblitz --solvers bsgs --curve-seeds 1 --generators 1 --keys 1 --seed 19 --factor-base-size 4 --work-limit 10 --protocol-sha256 "$protocol_sha" > "$evidence/plan.stdout"
for version in baseline candidate; do
  binary=$baseline
  if [[ $version = candidate ]]; then binary=$candidate; fi
  # The remote path parses normally, but the native directory backend cannot
  # create objects beneath this ordinary file. Failure is after request writing.
  printf 'local publication fault\n' > "$evidence/$version-remote"
  set +e
  timeout 30 "$binary" dataset generate --plan "$evidence/plan.json" --protocol "$evidence/protocol.json" --out "$evidence/$version-run" --max-cases 1 --seconds-per-case 1 --memory-mib 128 --trace-mib 1 --publish --remote "file://$evidence/$version-remote" --snapshot-seconds 1 --snapshot-timeout-seconds 10 > "$evidence/$version-generate.stdout" 2> "$evidence/$version-generate.stderr"
  status=$?
  set -e
  printf '%s\n' "$status" > "$evidence/$version-generate.exit"
  test "$status" != 0
  test "$status" != 124
  cell="$evidence/$version-run/cells/000000000000"
  test -d "$cell"
  test ! -e "$cell/worker-process.json"
  test ! -e "$cell/worker-artifacts.json"
  test ! -e "$cell/telemetry"
  mapfile -t requests < <(find "$evidence/$version-run/partial-snapshots" -name request.json -type f)
  test "${#requests[@]}" = 1
  request=${requests[0]}
  printf '%s\n' "$request" > "$evidence/$version-request-path"
  jq -e '.ordinal == 0 and .previous == null and .copied_context == [] and .observation.status == "running" and .binding.schema == "crypto.ml-dataset.attempt-binding/v1"' "$request" > /dev/null
  if [[ $version = candidate ]]; then
    test "$(sha256sum "$cell/measurement-policy.json" | cut -d' ' -f1)" = 45c9f43fa4df83c6c478094a84478690dd791e27fccaa19af5b87db33f6b4a50
    test "$(wc -c < "$cell/measurement-policy.json")" = 785
    jq -e '.schema == "crypto.ml-dataset.snapshot-helper/v2" and .profile == "checked_phase_v2" and .acknowledged_context == []' "$request" > /dev/null
  else
    test ! -e "$cell/measurement-policy.json"
    jq -e '.schema == "crypto.ml-dataset.snapshot-helper/v1" and (has("profile")|not) and (has("acknowledged_context")|not)' "$request" > /dev/null
  fi
done

# Replay the real candidate supervisor's frozen first request after repairing
# only the local publication fixture. This is the initial helper/restore path,
# not a resumed generator or a successful worker run.
candidate_request=$(cat "$evidence/candidate-request-path")
rm "$evidence/candidate-remote"
mkdir "$evidence/candidate-remote"
timeout 30 "$candidate" dataset snapshot-worker --request "$candidate_request" > "$evidence/candidate-initial-helper.stdout" 2> "$evidence/candidate-initial-helper.stderr"
candidate_result=$(dirname "$candidate_request")/result.json
jq -e '.schema == "crypto.ml-dataset.snapshot-helper-result/v2" and .profile == "checked_phase_v2" and .published == true and (.acknowledged_context | any(.path == "context/measurement-policy.json" and .role == "restricted-measurement-policy"))' "$candidate_result" > /dev/null
jq '.anchor' "$candidate_result" > "$evidence/candidate-anchor.json"
timeout 30 "$candidate" dataset restore-partial --anchor "$evidence/candidate-anchor.json" --out "$evidence/candidate-restored" --remote "file://$evidence/candidate-remote" > "$evidence/candidate-restore.stdout" 2> "$evidence/candidate-restore.stderr"
jq -e '.solver_resumed == false and .cell_complete == false and .scientific_result_verified == false' "$evidence/candidate-restore.stdout" > /dev/null
test ! -e "$evidence/candidate-run/cells/000000000000/worker-process.json"
test ! -e "$evidence/candidate-run/cells/000000000000/telemetry"

# Synthetic start-only metadata through the actual report CLI. This is explicitly
# a reader fixture; no worker, operation or hardware acquisition produced it.
cell="$evidence/candidate-run/cells/000000000000"
mkdir "$cell/telemetry"
jq '{schema:"crypto.ml-dataset.phase-start/v2",phase:"curve-construction",ordinal:0,binding:.binding,policy_sha256:"45c9f43fa4df83c6c478094a84478690dd791e27fccaa19af5b87db33f6b4a50",recording_pid:1,usage_before:null,counter_boundary:"durable_start_before_open"}' "$candidate_request" > "$cell/telemetry/curve-construction-start.json"
timeout 30 "$candidate" dataset report --out "$evidence/candidate-run" --destination "$evidence/v2-start-only-report" > "$evidence/candidate-v2-report.stdout" 2> "$evidence/candidate-v2-report.stderr"
jq -s -e 'length == 1 and .[0].schema == "crypto.ml-dataset.coverage-measurement/v2" and .[0].kind == "interrupted_or_unfinished" and .[0].manifest_bound == false and .[0].training_cost_eligible == false and .[0].whole_worker_retired_instructions == null and .[0].consumer_v2_admitted == false and .[0].platform_admitted == false' "$evidence/v2-start-only-report/measurements.jsonl" > /dev/null

# Complete the frozen baseline helper using the original executable after
# repairing only its local fault fixture. Then replay its saved v1 request and
# prepared/result/manifest/receipt bytes through the new executable unchanged.
request=$(cat "$evidence/baseline-request-path")
rm "$evidence/baseline-remote"
mkdir "$evidence/baseline-remote"
timeout 30 "$baseline" dataset snapshot-worker --request "$request" > "$evidence/baseline-helper.stdout" 2> "$evidence/baseline-helper.stderr"
(cd "$evidence/baseline-run" && find partial-snapshots -type f ! -name '*.log' -print0 | sort -z | xargs -0 sha256sum) > "$evidence/v1-helper-before.sha256"
timeout 30 "$candidate" dataset snapshot-worker --request "$request" > "$evidence/candidate-replay-helper.stdout" 2> "$evidence/candidate-replay-helper.stderr"
(cd "$evidence/baseline-run" && sha256sum -c "$evidence/v1-helper-before.sha256") > "$evidence/v1-helper-readback.log"

# Fixed legacy phase bytes are reader fixtures, not claimed hardware results.
# The historical executable supplies the exact report golden from those bytes.
cell="$evidence/baseline-run/cells/000000000000"
mkdir "$cell/telemetry"
printf '%s\n' '{"schema":"crypto.ml-dataset.phase-start/v1","phase":"solver","counter_scope":"legacy reader fixture; no hardware measurement"}' > "$cell/telemetry/solver-start.json"
printf '%s\n' '{"schema":"crypto.ml-dataset.phase-metrics/v1","phase":"solver","instructions_status":"unavailable","retired_instructions_raw":null,"retired_instructions_exact_in_scope":null,"hardware":null,"counter_scope":"legacy reader fixture; no hardware measurement","clock_valid_for_comparison":false}' > "$cell/telemetry/solver.json"
sha256sum "$cell/telemetry/solver-start.json" "$cell/telemetry/solver.json" > "$evidence/v1-phase-fixtures.sha256"
timeout 30 "$baseline" dataset report --out "$evidence/baseline-run" --destination "$evidence/v1-report-golden" > "$evidence/baseline-report.stdout" 2> "$evidence/baseline-report.stderr"
timeout 30 "$candidate" dataset report --out "$evidence/baseline-run" --destination "$evidence/v1-report-candidate" > "$evidence/candidate-report.stdout" 2> "$evidence/candidate-report.stderr"
diff -r "$evidence/v1-report-golden" "$evidence/v1-report-candidate" > "$evidence/v1-report-byte-comparison.log"
sha256sum -c "$evidence/v1-phase-fixtures.sha256" > "$evidence/v1-phase-readback.log"
test ! -e "$evidence/aws-was-called"
printf '%s\n' 'PASS: initial v2 request precedes worker; historical v1 helper/snapshot and report bytes retained; fixed legacy phase fixtures unchanged; local metadata only' > "$evidence/result.txt"
