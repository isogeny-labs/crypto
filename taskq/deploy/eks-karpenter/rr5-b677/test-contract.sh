#!/usr/bin/env bash
# Bounded shell-contract tests. All producer/checker/cloud executables below are
# mocks. The synthetic point arrays are not scientific fixtures, and an accepted
# mock receipt says nothing about group arithmetic or an extraction outcome.
# This file runs from the taskq-only contracts sparse checkout with no Rust build.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
for tool in bash jq sha256sum timeout setsid find sort xargs realpath install; do
  command -v "$tool" >/dev/null || { echo "missing test dependency: $tool" >&2; exit 2; }
done
export RR5_TEST_REAL_JQ RR5_TEST_REAL_SHA256SUM RR5_TEST_REAL_TIMEOUT
RR5_TEST_REAL_JQ=$(command -v jq)
RR5_TEST_REAL_SHA256SUM=$(command -v sha256sum)
RR5_TEST_REAL_TIMEOUT=$(command -v timeout)
work=$(mktemp -d "${TMPDIR:-/tmp}/rr5-b677-contract.XXXXXX")
active_run_pid='' active_timeout_pid=''
cleanup() {
  local status=$?
  trap - EXIT
  if [[ -n "$active_timeout_pid" ]]; then kill -KILL -- "-$active_timeout_pid" 2>/dev/null || true; fi
  if [[ -n "$active_run_pid" ]]; then
    kill -KILL "$active_run_pid" 2>/dev/null || true
    wait "$active_run_pid" 2>/dev/null || true
  fi
  rm -rf "$work"
  exit "$status"
}
trap cleanup EXIT
revision=1111111111111111111111111111111111111111
digest=2222222222222222222222222222222222222222222222222222222222222222
results_uri=s3://isolab-results-104186811093-us-east-2/runs/mock-contract/pod/
passed=0
current=$work

fail() {
  echo "not ok - $*" >&2
  if [[ -s "$current/command.stderr" ]]; then
    tail -n 30 "$current/command.stderr" >&2
  fi
  exit 1
}
pass() { passed=$((passed + 1)); printf 'ok %s - %s\n' "$passed" "$*"; }
expect_status() {
  local expected=$1 status
  shift
  if "$@" >"$current/command.stdout" 2>"$current/command.stderr"; then status=0; else status=$?; fi
  [[ "$status" == "$expected" ]] || fail "expected status $expected, got $status: $*"
}
expect_failure() {
  if "$@" >"$current/command.stdout" 2>"$current/command.stderr"; then
    fail "unexpected success: $*"
  fi
}
assert_json() {
  local file=$1 expression=$2
  "$RR5_TEST_REAL_JQ" -e "$expression" "$file" >/dev/null || fail "JSON assertion: $file: $expression"
}
assert_calls() {
  [[ $(cat "$MOCK_CALL_LOG") == "$1" ]] || fail "unexpected producer call order in $MOCK_CALL_LOG"
}
assert_no_checker() { [[ ! -s "$MOCK_CHECKER_LOG" ]] || fail 'invalid transport reached checker'; }
assert_no_cloud() { [[ ! -s "$MOCK_CLOUD_LOG" ]] || fail 'invalid input reached mock cloud command'; }
assert_mock_processes_stopped() {
  local pid state
  [[ -s "$MOCK_PROCESS_LOG" ]] || fail 'process-lifecycle fixture did not start'
  while IFS= read -r pid; do
    if [[ -r /proc/$pid/stat ]]; then
      state=$(cut -d' ' -f3 "/proc/$pid/stat" 2>/dev/null || true)
      # A killed orphan may briefly remain a zombie until the host reaps it.
      [[ -z "$state" || "$state" == Z || "$state" == X ]] || fail "mock child $pid survived in state $state"
    fi
  done <"$MOCK_PROCESS_LOG"
}

mkdir "$work/template" "$work/bin"
for name in run sequence verify-call; do cp "$here/$name" "$work/template/$name"; done
cat >"$work/template/rr5-b677-gate" <<'MOCK_PRODUCER'
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 4 && $1 == --case && $3 == --presentation ]]
case_name=$2 presentation=$4 scenario=${MOCK_SCENARIO:-good}
printf '%s:%s\n' "$case_name" "$presentation" >>"${MOCK_CALL_LOG:?}"
if [[ ${MOCK_ENFORCE_ENV:-0} == 1 ]]; then
  [[ $RAYON_NUM_THREADS == 1 && $OMP_NUM_THREADS == 1 && $OPENBLAS_NUM_THREADS == 1 && $RUST_TEST_THREADS == 1 ]]
  [[ $(ulimit -v) == 2097152 ]]
fi
case "$scenario:$case_name:$presentation" in
  structured_failure:positive:structured|cofactor_failure:positive:cofactor) exit 7 ;;
esac
# Deliberately non-curve mock data: the real native checker is never invoked.
input=$("$RR5_TEST_REAL_JQ" -cn --arg case_name "$case_name" --arg presentation "$presentation" '
  {schema:"prime31-rr5-b677-input/v1",case:$case_name,presentation:$presentation,
   curve:{p:2147483647,a:847514846,b:2087444724,n:2147446831,g:[625037715,2100627251]},
   public_base:[range(0;677)|[.,1]],
   public_target:(if $case_name=="h0" then [18,221278090]
     elif $case_name=="w0" then [101000006,1077058598] else [23,29] end),
   public_target_on_curve:true,supported_target_chart:true,
   candidate_received_expected_row:false,candidate_received_target_scalar:false,
   candidate_received_factor_base_logs:false,test_oracle_tuple_evaluations:0,
   limits:{max_degree:10,seconds:30,max_polynomial_terms:20000,max_assignments:4096},
   resource_limits:{max_pairs:20000,max_symbolic_monomials:8192,max_matrix_entries:2000000},
   fixture_certificate:{hybrid_points:677,
     y7_csv_sha256:"fc484615394ee423190747d05001cea526580fb0df0279fbcf789ab6cafc0c46"},
   control_expected_row:(if $case_name=="positive" then {terms:[[0,1],[1,-1],[2,1],[3,1]]} else null end)}')
result=$("$RR5_TEST_REAL_JQ" -cn --arg case_name "$case_name" --arg presentation "$presentation" '
  {schema:"prime31-rr5-b677-result/v1",case:$case_name,presentation:$presentation,
   complete_gate_passed:true,gate_failures:[],authentic_public_rhs:{a:0,b:1},
   expected_control_row_present:(if $case_name=="positive" then true else null end),
   adapter_rank:1,recovered_scalar:null,
   candidate:{status:"Complete",relations:[{terms:[[0,1],[1,-1],[2,1],[3,1]]}],
     diagnostics:{base_size:677,assignments_extracted:1,rejected_assignments:0,
       forbidden_branch_attempts:0,forbidden_domain_evaluations:0,forbidden_fallbacks_refused:0},
     traversal:{complete:true,timed_out:false,solving_degree:5,trace_stopped:false,
       stopped_early:false,full_field_branches:0,domain_evaluations:0}}}')
case "$scenario" in
  overdegree) result=$("$RR5_TEST_REAL_JQ" -c '.candidate.traversal.solving_degree=11' <<<"$result") ;;
  fractional_degree) result=$("$RR5_TEST_REAL_JQ" -c '.candidate.traversal.solving_degree=1.5' <<<"$result") ;;
  too_many_assignments) result=$("$RR5_TEST_REAL_JQ" -c '.candidate.diagnostics.assignments_extracted=4097' <<<"$result") ;;
  wrong_limits) input=$("$RR5_TEST_REAL_JQ" -c '.limits.seconds=31' <<<"$input") ;;
  wrong_resources) input=$("$RR5_TEST_REAL_JQ" -c '.resource_limits.max_pairs=20001' <<<"$input") ;;
  wrong_presentation) result=$("$RR5_TEST_REAL_JQ" -c '.presentation="unregistered"' <<<"$result") ;;
  partial_traversal) result=$("$RR5_TEST_REAL_JQ" -c '.candidate.traversal.complete=false' <<<"$result") ;;
  multivalue_input) input="false $input" ;;
  multivalue_result) result="false $result" ;;
esac
printf 'B677_INPUT %s\nB677_RESULT %s\n' "$input" "$result"
MOCK_PRODUCER
cat >"$work/template/rr5-b677-receipt-check" <<'MOCK_CHECKER'
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 ]]
printf '%s\n' "$1" >>"${MOCK_CHECKER_LOG:?}"
if [[ ${MOCK_SCENARIO:-good} == checker_reject && $1 == */positive-structured/* ]]; then exit 8; fi
"$RR5_TEST_REAL_JQ" -e '.stop_when_target_determined==false and .emit_dual_certificate==true
  and all(.relations[]; .a==0 and .b==1)' "$1" >/dev/null
receipt=$("$RR5_TEST_REAL_JQ" -c '
  {p:2147483647,n:2147446831,base_size:(.base|length),
   submitted_rows:(.relations|length),authenticated_rows:(.relations|length),unchecked_tail_rows:0,
   stop_when_target_determined:false,emit_dual_certificate:true,
   independent_augmented_rank:1,recovered:null,scalar_replay_passed:false}' "$1")
case ${MOCK_SCENARIO:-good} in
  checker_multivalue) printf 'false %s\n' "$receipt" ;;
  checker_rank_mismatch) "$RR5_TEST_REAL_JQ" -c '.independent_augmented_rank=2' <<<"$receipt" ;;
  checker_tail) "$RR5_TEST_REAL_JQ" -c '.unchecked_tail_rows=1' <<<"$receipt" ;;
  *) printf '%s\n' "$receipt" ;;
esac
MOCK_CHECKER
cat >"$work/bin/aws" <<'MOCK_AWS'
#!/usr/bin/env bash
set -euo pipefail
printf 'aws %s\n' "$*" >>"${MOCK_CLOUD_LOG:?}"
case "${1:-}:${2:-}" in
  s3:cp) [[ $# == 6 && ${3:-} == "$ISOLAB_OUTPUT_DIR/" && ${4:-} == "$ISOLAB_RESULTS_URI" && $5 == --recursive && $6 == --only-show-errors ]]; exit "${MOCK_UPLOAD_STATUS:-0}" ;;
  ecr:get-login-password|ecr-public:get-login-password) printf 'mock-password\n' ;;
  ecr:describe-images) printf 'sha256:%s\n' "${MOCK_IMAGE_DIGEST:?}" ;;
  *) echo 'unexpected mock AWS operation' >&2; exit 98 ;;
esac
MOCK_AWS
cat >"$work/bin/docker" <<'MOCK_DOCKER'
#!/usr/bin/env bash
set -euo pipefail
printf 'docker %s\n' "$*" >>"${MOCK_CLOUD_LOG:?}"
case ${1:-} in login) cat >/dev/null ;; build|push) : ;; *) exit 98 ;; esac
MOCK_DOCKER
cat >"$work/bin/jq" <<'MOCK_JQ'
#!/usr/bin/env bash
set -euo pipefail
if [[ ${MOCK_EXIT_RECORD_FAILURE:-0} == 1 ]]; then
  for arg; do [[ $arg != *rr5.b677.job-exit/v1* ]] || exit 71; done
fi
exec "$RR5_TEST_REAL_JQ" "$@"
MOCK_JQ
cat >"$work/bin/sha256sum" <<'MOCK_HASH'
#!/usr/bin/env bash
set -euo pipefail
if [[ ${MOCK_CHECKSUM_FAILURE:-0} == 1 && ${1:-} == ./* ]]; then exit 72; fi
exec "$RR5_TEST_REAL_SHA256SUM" "$@"
MOCK_HASH
cat >"$work/bin/timeout" <<'MOCK_TIMEOUT'
#!/usr/bin/env bash
set -euo pipefail
if [[ -n ${MOCK_TIMEOUT_SECONDS:-} ]]; then
  [[ $# == 5 && $1 == --signal=KILL && $2 == 180 ]]
  printf '%s\n' "$$" >"${MOCK_TIMEOUT_PID_FILE:?}"
  shift 2
  exec "$RR5_TEST_REAL_TIMEOUT" --signal=KILL "$MOCK_TIMEOUT_SECONDS" "$@"
fi
exec "$RR5_TEST_REAL_TIMEOUT" "$@"
MOCK_TIMEOUT
chmod +x "$work/template/"* "$work/bin/"*
export PATH="$work/bin:$PATH"
export ISOLAB_IMAGE_REPOSITORY=104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks
export AWS_REGION=us-east-2 GITHUB_RUN_ID=123 GITHUB_RUN_ATTEMPT=1 MOCK_IMAGE_DIGEST=$digest

new_case() {
  current="$work/$1"
  mkdir "$current" "$current/output"
  cp -R "$work/template" "$current/payload"
  payload="$current/payload"
  export ISOLAB_OUTPUT_DIR="$current/output" ISOLAB_RESULTS_URI="$results_uri"
  export MOCK_CALL_LOG="$current/calls" MOCK_CHECKER_LOG="$current/checkers" MOCK_CLOUD_LOG="$current/cloud"
  export MOCK_PROCESS_LOG="$current/processes" MOCK_TIMEOUT_PID_FILE="$current/timeout.pid" MOCK_TIMEOUT_SECONDS=''
  : >"$MOCK_CALL_LOG"; : >"$MOCK_CHECKER_LOG"; : >"$MOCK_CLOUD_LOG"
  export MOCK_SCENARIO=good MOCK_ENFORCE_ENV=0 MOCK_UPLOAD_STATUS=0
  export MOCK_EXIT_RECORD_FAILURE=0 MOCK_CHECKSUM_FAILURE=0
}
seal_bundle() {
  local name hash payload_hashes='{}' metadata_hashes='{}'
  printf 'Synthetic shell-contract protocol; no scientific request.\n' >"$payload/PROTOCOL.md"
  printf '{"synthetic_contract_fixture":true}\n' >"$payload/source-manifest.json"
  printf 'Synthetic source checksum listing.\n' >"$payload/source.sha256"
  protocol=$("$RR5_TEST_REAL_SHA256SUM" "$payload/PROTOCOL.md" | cut -d' ' -f1)
  for name in rr5-b677-gate rr5-b677-receipt-check run sequence verify-call; do
    hash=$("$RR5_TEST_REAL_SHA256SUM" "$payload/$name" | cut -d' ' -f1)
    payload_hashes=$("$RR5_TEST_REAL_JQ" -c --arg name "$name" --arg hash "$hash" '.+{($name):$hash}' <<<"$payload_hashes")
  done
  for name in source-manifest.json source.sha256; do
    hash=$("$RR5_TEST_REAL_SHA256SUM" "$payload/$name" | cut -d' ' -f1)
    metadata_hashes=$("$RR5_TEST_REAL_JQ" -c --arg name "$name" --arg hash "$hash" '.+{($name):$hash}' <<<"$metadata_hashes")
  done
  "$RR5_TEST_REAL_JQ" -n --arg revision "$revision" --arg protocol "$protocol" \
    --argjson payload "$payload_hashes" --argjson metadata "$metadata_hashes" '
    {schema:"rr5.b677.build/v1",source_revision:$revision,protocol_sha256:$protocol,
     source_repository:"https://github.com/isogeny-labs/crypto",target:"x86_64-unknown-linux-musl",
     architecture:"linux/amd64",payload_sha256:$payload,metadata_sha256:$metadata,
     registered_extraction_executed:false}' >"$payload/build-info.json"
}
mutate_build() {
  "$RR5_TEST_REAL_JQ" "$1" "$payload/build-info.json" >"$payload/build-info.next"
  mv "$payload/build-info.next" "$payload/build-info.json"
}
make_image_receipt() {
  "$RR5_TEST_REAL_JQ" -n --arg revision "$revision" --arg digest "$digest" '
    {schema:"rr5.b677.image/v1",source_revision:$revision,
     image_uri:("104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:"+$digest),
     protocol_sha256:$digest,build_info_sha256:$digest}' >"$current/image.json"
}
install_waiting_sequence() {
  cat >"$payload/sequence" <<'MOCK_WAITING_SEQUENCE'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$BASHPID" >>"${MOCK_PROCESS_LOG:?}"
(printf '%s\n' "$BASHPID" >>"$MOCK_PROCESS_LOG"; exec sleep 60) &
wait
MOCK_WAITING_SEQUENCE
}

two_calls=$'positive:structured\npositive:cofactor'
four_calls=$'positive:structured\npositive:cofactor\nh0:structured\nw0:structured'
for scenario in good structured_failure checker_reject cofactor_failure; do
  new_case "sequence-$scenario"
  export MOCK_SCENARIO=$scenario MOCK_ENFORCE_ENV=1
  expected=1; [[ $scenario != good ]] || expected=0
  expect_status "$expected" "$payload/sequence" "$revision" "$digest"
  if [[ $scenario == structured_failure || $scenario == checker_reject ]]; then
    assert_calls "$two_calls"
    assert_json "$ISOLAB_OUTPUT_DIR/followups-not-run.json" '.cases==["h0","w0"]'
    [[ ! -e "$ISOLAB_OUTPUT_DIR/h0-structured" && ! -e "$ISOLAB_OUTPUT_DIR/w0-structured" ]] || fail 'followup dispatched without structured replay'
    assert_json "$ISOLAB_OUTPUT_DIR/positive-structured/post-call.json" '.accepted==false'
  else
    assert_calls "$four_calls"
    assert_json "$ISOLAB_OUTPUT_DIR/positive-structured/post-call.json" '.accepted==true'
    [[ ! -e "$ISOLAB_OUTPUT_DIR/followups-not-run.json" ]] || fail 'spurious not-run record'
  fi
  assert_json "$ISOLAB_OUTPUT_DIR/sequence.json" ".all_requested_calls_accepted==($expected==0) and .retries==0 and .scientific_worker_count==1 and .outer_wall_seconds==180 and .complete_cold_cost==null"
  assert_json "$ISOLAB_OUTPUT_DIR/positive-cofactor/pre-call.json" '.workers==1 and .address_space_bytes==2147483648 and .argv==["rr5-b677-gate","--case","positive","--presentation","cofactor"]'
  pass "sequence $scenario preserves control order and followup gate"
done

new_case sequence-refuses-reuse
export MOCK_ENFORCE_ENV=1
expect_status 0 "$payload/sequence" "$revision" "$digest"
before=$("$RR5_TEST_REAL_SHA256SUM" "$ISOLAB_OUTPUT_DIR/positive-structured/pre-call.json")
expect_failure "$payload/sequence" "$revision" "$digest"
[[ $("$RR5_TEST_REAL_SHA256SUM" "$ISOLAB_OUTPUT_DIR/positive-structured/pre-call.json") == "$before" ]] || fail 'pre-call record overwritten'
assert_calls "$four_calls"
pass 'sequence refuses evidence-directory reuse without another call'

for scenario in good multivalue_input multivalue_result overdegree fractional_degree too_many_assignments \
  wrong_limits wrong_resources wrong_presentation partial_traversal checker_multivalue checker_rank_mismatch checker_tail; do
  new_case "verify-$scenario"
  export MOCK_SCENARIO=$scenario
  call_dir="$current/call"
  mkdir "$call_dir"
  "$payload/rr5-b677-gate" --case positive --presentation structured >"$call_dir/producer.stdout"
  printf '0\n' >"$call_dir/producer.exit"
  if [[ $scenario == good ]]; then
    expect_status 0 "$payload/verify-call" "$call_dir" positive structured
    before=$("$RR5_TEST_REAL_SHA256SUM" "$call_dir/input.json" "$call_dir/result.json" "$call_dir/checker.json")
    expect_failure "$payload/verify-call" "$call_dir" positive structured
    [[ $("$RR5_TEST_REAL_SHA256SUM" "$call_dir/input.json" "$call_dir/result.json" "$call_dir/checker.json") == "$before" ]] || fail 'normalized receipt overwritten'
  else
    expect_failure "$payload/verify-call" "$call_dir" positive structured
    case "$scenario" in checker_*) [[ -s "$MOCK_CHECKER_LOG" ]] || fail 'checker failure path not exercised' ;; *) assert_no_checker ;; esac
  fi
  pass "verify-call $scenario transport/replay contract"
done

new_case request-good
make_image_receipt
expect_status 0 "$here/prepare-request" "$current/image.json" "$revision" mock-contract m6i.large 300
assert_json "$current/command.stdout" ".schema==\"isolab.eks.command-request/v1\" and .job.cpu==2 and .job.memory==\"2Gi\" and .job.seconds==300 and .job.wait_seconds==300 and .job.retries==0 and .job.isolation_profile==\"standard\" and .job.instance_type==\"m6i.large\" and .job.command==[\"/opt/isolab/rr5-b677/run\",\"$revision\",\"$digest\"]"
pass 'request emits the fixed command, resources, mode, and no retries'
for mutation in \
  '.image_uri="104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks:latest"' \
  '.image_uri|=sub("104186811093";"000000000000")' \
  '.source_revision="0000000000000000000000000000000000000000"' \
  '.protocol_sha256="short"' '.build_info_sha256="short"' \
  '.schema="unexpected"'; do
  new_case "request-mutation-$passed"
  make_image_receipt
  "$RR5_TEST_REAL_JQ" "$mutation" "$current/image.json" >"$current/bad-image.json"
  expect_failure "$here/prepare-request" "$current/bad-image.json" "$revision" mock-contract m6i.large 300
  pass "request rejects $mutation"
done
new_case request-json-stream
make_image_receipt
{ printf 'false\n'; cat "$current/image.json"; } >"$current/image-stream.json"
expect_failure "$here/prepare-request" "$current/image-stream.json" "$revision" mock-contract m6i.large 300
pass 'request rejects multiple JSON values'
for seconds in 299 1801 300.5 0300; do
  expect_failure "$here/prepare-request" "$current/image.json" "$revision" mock-contract m6i.large "$seconds"
  pass "request rejects seconds=$seconds"
done
expect_failure "$here/prepare-request" "$current/image.json" "$revision" Bad_Name m6i.large 300
pass 'request rejects an invalid job name'
expect_failure "$here/prepare-request" "$current/image.json" "$revision" mock-contract 'm6i.large extra' 300
pass 'request rejects an invalid instance selector'
expect_status 2 "$here/prepare-request" "$current/image.json" "$revision" mock-contract m6i.large 300 --cpu 8
pass 'request rejects extra mode/resource arguments'

# Check actual byte bindings in the publisher, where a protocol and build
# manifest are available. prepare-request only validates image-receipt shape.
for mutation in good wrong_repository wrong_revision wrong_protocol wrong_payload wrong_metadata multi_json symlink; do
  new_case "publish-$mutation"
  seal_bundle
  case "$mutation" in
    wrong_repository) mutate_build '.source_repository="https://example.invalid/other"' ;;
    wrong_revision) mutate_build '.source_revision="0000000000000000000000000000000000000000"' ;;
    wrong_protocol) printf 'mutated\n' >>"$payload/PROTOCOL.md" ;;
    wrong_payload) printf '# mutation\n' >>"$payload/rr5-b677-gate" ;;
    wrong_metadata) printf 'mutated\n' >>"$payload/source.sha256" ;;
    multi_json) cat "$payload/build-info.json" >"$current/build-copy"; cat "$current/build-copy" >>"$payload/build-info.json" ;;
    symlink) mv "$payload/rr5-b677-gate" "$current/gate"; ln -s "$current/gate" "$payload/rr5-b677-gate" ;;
  esac
  if [[ $mutation == good ]]; then
    expect_status 0 "$here/publish-image" "$payload" "$revision" "$current/published.json"
    assert_json "$current/published.json" ".source_revision==\"$revision\" and .protocol_sha256==\"$protocol\" and .image_uri==\"$ISOLAB_IMAGE_REPOSITORY@sha256:$digest\""
    [[ -s "$MOCK_CLOUD_LOG" && ! -s "$MOCK_CALL_LOG" ]] || fail 'mock publisher did not publish only data'
  else
    expect_failure "$here/publish-image" "$payload" "$revision" "$current/published.json"
    assert_no_cloud
  fi
  pass "publisher $mutation byte/source binding"
done

for scenario in good upload_failure exit_record_failure checksum_failure child_failure child_signal; do
  new_case "run-$scenario"
  case "$scenario" in
    upload_failure) export MOCK_UPLOAD_STATUS=73 ;;
    exit_record_failure) export MOCK_EXIT_RECORD_FAILURE=1 ;;
    checksum_failure) export MOCK_CHECKSUM_FAILURE=1 ;;
    child_failure) printf '#!/usr/bin/env bash\nexit 7\n' >"$payload/sequence" ;;
    child_signal) printf '#!/usr/bin/env bash\nkill -TERM "$$"\n' >"$payload/sequence" ;;
  esac
  seal_bundle
  expected=0
  case "$scenario" in upload_failure) expected=91 ;; exit_record_failure|checksum_failure) expected=92 ;; child_failure) expected=7 ;; child_signal) expected=143 ;; esac
  expect_status "$expected" "$payload/run" "$revision" "$protocol"
  [[ -s "$MOCK_CLOUD_LOG" ]] || fail 'finish trap did not attempt evidence upload'
  if [[ $scenario != exit_record_failure ]]; then
    child_status=0
    case "$scenario" in child_failure) child_status=7 ;; child_signal) child_status=143 ;; esac
    assert_json "$ISOLAB_OUTPUT_DIR/job-exit.json" ".exit_status==$child_status and .scientific_run_completed==($child_status==0)"
  fi
  if [[ $scenario != checksum_failure ]]; then
    (cd "$ISOLAB_OUTPUT_DIR" && "$RR5_TEST_REAL_SHA256SUM" -c SHA256SUMS >/dev/null) || fail 'archive checksum validation failed'
  fi
  pass "runner $scenario preserves the final process outcome"
done

new_case run-outer-timeout
install_waiting_sequence
seal_bundle
# Exercise the real timeout/process-group behavior in one second; require the
# production runner still to request exactly the frozen 180-second KILL cap.
export MOCK_TIMEOUT_SECONDS=1
expect_status 137 "$payload/run" "$revision" "$protocol"
assert_mock_processes_stopped
assert_json "$ISOLAB_OUTPUT_DIR/job-exit.json" '.exit_status==137 and .scientific_run_completed==false'
[[ -s "$MOCK_CLOUD_LOG" ]] || fail 'timeout evidence was not uploaded'
pass 'runner outer deadline kills the sequence and descendants and archives nonzero'

new_case run-parent-term
install_waiting_sequence
seal_bundle
# Five seconds is only a test safety cap. TERM must stop both mock descendants
# and the parent within the shorter observation window below.
export MOCK_TIMEOUT_SECONDS=5
"$payload/run" "$revision" "$protocol" >"$current/command.stdout" 2>"$current/command.stderr" &
active_run_pid=$!
for _ in {1..100}; do
  if [[ -s "$MOCK_TIMEOUT_PID_FILE" ]]; then active_timeout_pid=$(cat "$MOCK_TIMEOUT_PID_FILE"); fi
  if [[ -s "$MOCK_PROCESS_LOG" && $(wc -l <"$MOCK_PROCESS_LOG") == 2 ]]; then break; fi
  sleep 0.02
done
[[ -n "$active_timeout_pid" && -s "$MOCK_PROCESS_LOG" && $(wc -l <"$MOCK_PROCESS_LOG") == 2 ]] || fail 'TERM fixture did not start'
kill -TERM "$active_run_pid"
for _ in {1..100}; do
  if ! kill -0 "$active_run_pid" 2>/dev/null; then break; fi
  sleep 0.02
done
if kill -0 "$active_run_pid" 2>/dev/null; then fail 'runner did not handle TERM promptly'; fi
if wait "$active_run_pid"; then status=0; else status=$?; fi
[[ "$status" == 143 ]] || fail "parent TERM returned $status instead of 143"
assert_mock_processes_stopped
assert_json "$ISOLAB_OUTPUT_DIR/job-exit.json" '.exit_status==143 and .scientific_run_completed==false'
[[ -s "$MOCK_CLOUD_LOG" ]] || fail 'TERM evidence was not uploaded'
active_run_pid='' active_timeout_pid=''
pass 'runner TERM promptly stops the entire mock process group and archives nonzero'

for mutation in wrong_protocol wrong_payload wrong_metadata wrong_repository multi_json; do
  new_case "run-binding-$mutation"
  seal_bundle
  case "$mutation" in
    wrong_protocol) printf 'mutation\n' >>"$payload/PROTOCOL.md" ;;
    wrong_payload) printf '# mutation\n' >>"$payload/sequence" ;;
    wrong_metadata) printf 'mutation\n' >>"$payload/source-manifest.json" ;;
    wrong_repository) mutate_build '.source_repository="https://example.invalid/other"' ;;
    multi_json) cat "$payload/build-info.json" >"$current/build-copy"; cat "$current/build-copy" >>"$payload/build-info.json" ;;
  esac
  expect_failure "$payload/run" "$revision" "$protocol"
  [[ ! -s "$MOCK_CALL_LOG" ]] || fail 'invalid bundle dispatched a producer'
  assert_json "$ISOLAB_OUTPUT_DIR/job-exit.json" '.exit_status!=0 and .scientific_run_completed==false'
  [[ -s "$MOCK_CLOUD_LOG" ]] || fail 'invalid bundle failure was not archived'
  pass "runner rejects $mutation before dispatch and archives failure"
done
for uri in s3://setec-astronomy-artifacts/runs/mock/ s3://isolab-results-000000000000-us-east-2/runs/mock/ \
  s3://isolab-results-104186811093-us-east-2/other/mock/; do
  new_case "run-destination-$passed"
  seal_bundle
  export ISOLAB_RESULTS_URI=$uri
  expect_failure "$payload/run" "$revision" "$protocol"
  assert_no_cloud
  [[ ! -s "$MOCK_CALL_LOG" ]] || fail 'invalid destination dispatched a producer'
  pass "runner rejects an unregistered result destination: $uri"
done
printf '1..%s\n' "$passed"
