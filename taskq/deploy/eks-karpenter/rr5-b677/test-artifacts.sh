#!/usr/bin/env bash
# Synthetic transport/custody tests only. The producer, checker, wrapper and AWS
# commands are mocks; no native solver, group arithmetic or research query runs.
# verify-call's mathematical/JSON contract is covered separately by test-contract.
# This suite exercises the real collect, verify-artifacts and replay scripts.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
for tool in bash jq sha256sum timeout find sort xargs realpath cmp cp; do
  command -v "$tool" >/dev/null || { echo "missing test dependency: $tool" >&2; exit 2; }
done
export RR5_ARTIFACT_REAL_TIMEOUT
RR5_ARTIFACT_REAL_TIMEOUT=$(command -v timeout)
work=$(mktemp -d "${TMPDIR:-/tmp}/rr5-b677-artifact-contract.XXXXXX")
trap 'rm -rf "$work"' EXIT
revision=1111111111111111111111111111111111111111
image_digest=2222222222222222222222222222222222222222222222222222222222222222
job=rr5-artifact-contract
job_uid=11111111-1111-1111-1111-111111111111
pod_uid=22222222-2222-2222-2222-222222222222
prefix="s3://isolab-results-104186811093-us-east-2/runs/$job/"
launcher_uri="${prefix}launcher/mock-receipt/"
payload_uri="$prefix$pod_uid/"
passed=0
current=$work

fail() {
  echo "not ok - $*" >&2
  if [[ -s "$current/command.stderr" ]]; then tail -n 30 "$current/command.stderr" >&2; fi
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
  if "$@" >"$current/command.stdout" 2>"$current/command.stderr"; then fail "unexpected success: $*"; fi
}
assert_json() { jq -e "$2" "$1" >/dev/null || fail "JSON assertion failed: $1: $2"; }
assert_no_execution() {
  [[ ! -s "$MOCK_ARTIFACT_FORBIDDEN" ]] || fail 'producer/run/sequence mock was executed'
  [[ ! -s "$MOCK_ARTIFACT_REPLAYS" && ! -s "$MOCK_ARTIFACT_CHECKERS" ]] || fail 'transport stage executed a replay/checker'
}
hash_file() { sha256sum "$1" | cut -d' ' -f1; }
mutate_json() {
  local file=$1 expression=$2
  jq "$expression" "$file" >"$current/mutation.json"
  mv "$current/mutation.json" "$file"
}
seal_payload() {
  (cd "$evidence/payload" && find . -type f ! -path ./SHA256SUMS -print0 \
    | LC_ALL=C sort -z | xargs -0 -r sha256sum) >"$evidence/payload/SHA256SUMS"
}
rebind_build() {
  # Preserve the mutation while making the outer receipt self-consistent. A
  # failing test must detect the omitted/wrong contract, not just stale hashes.
  cp "$bundle/build-info.json" "$evidence/payload/build-info.json"
  local build_hash
  build_hash=$(hash_file "$bundle/build-info.json")
  mutate_json "$evidence/image.json" ".build_info_sha256=\"$build_hash\""
  seal_payload
}
rebind_manifest() {
  local manifest_hash
  manifest_hash=$(hash_file "$evidence/launcher/manifest.json")
  mutate_json "$evidence/client/result.json" ".manifest_sha256=\"$manifest_hash\""
  cp "$evidence/client/result.json" "$evidence/launcher/result.json"
}

mkdir "$work/bin" "$work/template-bundle" "$work/template-evidence"
bundle="$work/template-bundle" evidence="$work/template-evidence"
mkdir "$evidence/client" "$evidence/launcher" "$evidence/payload"
for name in rr5-b677-gate run sequence; do
  cat >"$bundle/$name" <<'FORBIDDEN_PAYLOAD'
#!/usr/bin/env bash
printf '%s\n' "$0" >>"${MOCK_ARTIFACT_FORBIDDEN:?}"
echo 'artifact-contract suite must never execute extraction/sequence/runner' >&2
exit 98
FORBIDDEN_PAYLOAD
done
cat >"$bundle/rr5-b677-receipt-check" <<'MOCK_CHECKER'
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 ]]
printf '%s\n' "$1" >>"${MOCK_ARTIFACT_CHECKERS:?}"
if [[ ${MOCK_ARTIFACT_CHECKER_FAIL:-} == "$(basename "$1")" ]]; then exit 17; fi
printf '{"synthetic_transport_checker":true}\n'
MOCK_CHECKER
cat >"$bundle/verify-call" <<'MOCK_REPLAY_WRAPPER'
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 3 ]]
call=$1 case_name=$2 presentation=$3
printf '%s:%s\n' "$case_name" "$presentation" >>"${MOCK_ARTIFACT_REPLAYS:?}"
[[ $(cat "$call/producer.exit") == 0 ]]
jq -es --arg case_name "$case_name" --arg presentation "$presentation" \
  'length==1 and (.[0]|.synthetic_transport_fixture==true and .case==$case_name and .presentation==$presentation)' \
  "$call/producer.stdout" >/dev/null
here=$(cd "$(dirname "$0")" && pwd)
"$here/rr5-b677-receipt-check" "$call" >"$call/checker.stdout" 2>"$call/checker.stderr"
MOCK_REPLAY_WRAPPER
cat >"$work/bin/aws" <<'MOCK_AWS'
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 6 && $1 == s3 && $2 == cp && $5 == --recursive && $6 == --only-show-errors ]]
printf '%s\n' "$3" >>"${MOCK_ARTIFACT_CLOUD:?}"
case $3 in
  "$MOCK_ARTIFACT_LAUNCHER_URI")
    [[ ${MOCK_ARTIFACT_LAUNCHER_FAIL:-0} == 0 ]] || { echo 'synthetic launcher download failure' >&2; exit 7; }
    cp -R "$MOCK_ARTIFACT_REMOTE_LAUNCHER/." "$4/" ;;
  "$MOCK_ARTIFACT_PAYLOAD_URI")
    [[ ${MOCK_ARTIFACT_PAYLOAD_FAIL:-0} == 0 ]] || { echo 'synthetic payload download failure' >&2; exit 8; }
    cp -R "$MOCK_ARTIFACT_REMOTE_PAYLOAD/." "$4/" ;;
  *) echo 'unregistered synthetic download' >&2; exit 98 ;;
esac
MOCK_AWS
cat >"$work/bin/timeout" <<'MOCK_TIMEOUT'
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 6 && $1 == --signal=KILL && $2 == 30 ]]
if [[ -n ${MOCK_ARTIFACT_TIMEOUT_CASE:-} && $(basename "$4") == "$MOCK_ARTIFACT_TIMEOUT_CASE" ]]; then
  echo 'synthetic timeout; no payload process started' >&2
  exit 137
fi
exec "$RR5_ARTIFACT_REAL_TIMEOUT" "$@"
MOCK_TIMEOUT
chmod +x "$bundle/"* "$work/bin/"*
export PATH="$work/bin:$PATH"
[[ $(command -v aws) == "$work/bin/aws" ]] || fail 'AWS mock is not first in PATH'

printf 'Synthetic transport protocol; not a registered mathematical fixture.\n' >"$bundle/PROTOCOL.md"
printf '{"synthetic_transport_sources":true}\n' >"$bundle/source-manifest.json"
printf 'Synthetic source inventory; no native executable is included.\n' >"$bundle/source.sha256"
protocol=$(hash_file "$bundle/PROTOCOL.md")
payload_hashes='{}' metadata_hashes='{}'
for name in rr5-b677-gate rr5-b677-receipt-check run sequence verify-call; do
  payload_hashes=$(jq -c --arg name "$name" --arg digest "$(hash_file "$bundle/$name")" \
    '.+{($name):$digest}' <<<"$payload_hashes")
done
for name in source-manifest.json source.sha256; do
  metadata_hashes=$(jq -c --arg name "$name" --arg digest "$(hash_file "$bundle/$name")" \
    '.+{($name):$digest}' <<<"$metadata_hashes")
done
jq -n --arg revision "$revision" --arg protocol "$protocol" \
  --argjson payload "$payload_hashes" --argjson metadata "$metadata_hashes" '
  {schema:"rr5.b677.build/v1",source_revision:$revision,protocol_sha256:$protocol,
   source_repository:"https://github.com/isogeny-labs/crypto",target:"x86_64-unknown-linux-musl",
   architecture:"linux/amd64",payload_sha256:$payload,metadata_sha256:$metadata,
   registered_extraction_executed:false}' >"$bundle/build-info.json"
jq -n --arg revision "$revision" --arg protocol "$protocol" --arg digest "$image_digest" \
  --arg build "$(hash_file "$bundle/build-info.json")" '
  {schema:"rr5.b677.image/v1",source_revision:$revision,protocol_sha256:$protocol,
   build_info_sha256:$build,
   image_uri:("104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:"+$digest)}' \
  >"$evidence/image.json"
"$here/prepare-request" "$evidence/image.json" "$revision" "$job" m6i.large 300 \
  | jq -S . >"$evidence/request.json"
cp "$evidence/request.json" "$evidence/launcher/request.json"
jq -n --slurpfile request "$evidence/request.json" '
  {apiVersion:"batch/v1",kind:"Job",metadata:{name:$request[0].job.job_name,namespace:"isolab"},
   spec:{backoffLimit:0,activeDeadlineSeconds:$request[0].job.seconds,
    template:{spec:{restartPolicy:"Never",containers:[
      {name:"task",image:$request[0].job.image,command:$request[0].job.command,
       resources:{requests:{cpu:"2",memory:"2Gi"},limits:{cpu:"2",memory:"2Gi"}}}]}}}}' \
  >"$evidence/launcher/manifest.json"
jq -n --arg job "$job" --arg job_uid "$job_uid" --arg pod "$pod_uid" --arg prefix "$prefix" \
  --arg receipt "$launcher_uri" --arg request "$(hash_file "$evidence/request.json")" \
  --arg manifest "$(hash_file "$evidence/launcher/manifest.json")" '
  {schema:"isolab.eks.command-result/v1",job_name:$job,job_uid:$job_uid,
   request_sha256:$request,manifest_sha256:$manifest,state:"succeeded",exit_code:0,
   pods:[{uid:$pod,node:"synthetic-node",exit_code:0}],results_prefix:$prefix,receipt_uri:$receipt}' \
  >"$evidence/client/result.json"
cp "$evidence/client/result.json" "$evidence/launcher/result.json"
jq -n --slurpfile manifest "$evidence/launcher/manifest.json" --slurpfile request "$evidence/request.json" \
  --arg job_uid "$job_uid" --arg pod "$pod_uid" '
  {items:[{metadata:{uid:$pod,ownerReferences:[{kind:"Job",uid:$job_uid}]},
    spec:{nodeName:"synthetic-node",restartPolicy:"Never",containers:$manifest[0].spec.template.spec.containers},
    status:{containerStatuses:[{name:"task",restartCount:0,
      imageID:("docker-pullable://"+$request[0].job.image),state:{terminated:{exitCode:0}}}]}}]}' \
  >"$evidence/launcher/pods.json"
for name in build-info.json PROTOCOL.md source-manifest.json source.sha256; do cp "$bundle/$name" "$evidence/payload/$name"; done
jq -n --arg revision "$revision" --arg protocol "$protocol" '
  {schema:"rr5.b677.job-exit/v1",exit_status:0,scientific_run_completed:true,
   source_revision:$revision,protocol_sha256:$protocol}' >"$evidence/payload/job-exit.json"
jq -n '{schema:"rr5.b677.sequence/v1",all_requested_calls_accepted:true,retries:0,
  scientific_worker_count:1,outer_wall_seconds:180,complete_cold_cost:null}' >"$evidence/payload/sequence.json"
for name in positive-structured positive-cofactor h0-structured w0-structured; do
  call="$evidence/payload/$name" case_name=${name%-*} presentation=${name#*-}
  mkdir "$call"
  jq -n --arg case_name "$case_name" --arg presentation "$presentation" \
    '{synthetic_transport_fixture:true,case:$case_name,presentation:$presentation}' >"$call/producer.stdout"
  printf '0\n' >"$call/producer.exit"
  jq -n --arg revision "$revision" --arg protocol "$protocol" --arg case_name "$case_name" \
    --arg presentation "$presentation" --arg producer "$(hash_file "$bundle/rr5-b677-gate")" \
    --arg checker "$(hash_file "$bundle/rr5-b677-receipt-check")" '
    {schema:"rr5.b677.pre-call/v1",source_revision:$revision,protocol_sha256:$protocol,
     producer_sha256:$producer,checker_sha256:$checker,workers:1,address_space_bytes:2147483648,
     case:$case_name,presentation:$presentation,
     argv:["rr5-b677-gate","--case",$case_name,"--presentation",$presentation]}' >"$call/pre-call.json"
  jq -n '{schema:"rr5.b677.post-call/v1",producer_exit:0,verification_exit:0,accepted:true}' >"$call/post-call.json"
done
seal_payload

new_case() {
  current="$work/$1"
  mkdir "$current"
  cp -R "$work/template-bundle" "$current/bundle"
  cp -R "$work/template-evidence" "$current/evidence"
  bundle="$current/bundle" evidence="$current/evidence"
  export MOCK_ARTIFACT_FORBIDDEN="$current/forbidden.log" MOCK_ARTIFACT_REPLAYS="$current/replay.log"
  export MOCK_ARTIFACT_CHECKERS="$current/checker.log" MOCK_ARTIFACT_CLOUD="$current/cloud.log"
  export MOCK_ARTIFACT_REMOTE_LAUNCHER="$evidence/launcher" MOCK_ARTIFACT_REMOTE_PAYLOAD="$evidence/payload"
  export MOCK_ARTIFACT_LAUNCHER_URI="$launcher_uri" MOCK_ARTIFACT_PAYLOAD_URI="$payload_uri"
  export MOCK_ARTIFACT_LAUNCHER_FAIL=0 MOCK_ARTIFACT_PAYLOAD_FAIL=0
  export MOCK_ARTIFACT_CHECKER_FAIL='' MOCK_ARTIFACT_TIMEOUT_CASE=''
  : >"$MOCK_ARTIFACT_FORBIDDEN"; : >"$MOCK_ARTIFACT_REPLAYS"
  : >"$MOCK_ARTIFACT_CHECKERS"; : >"$MOCK_ARTIFACT_CLOUD"
}

new_case verify-good
expect_status 0 "$here/verify-artifacts" "$bundle" "$evidence"
assert_no_execution
pass 'complete synthetic custody verifies without executing any payload'

for mutation in missing_payload empty_payload missing_metadata empty_metadata missing_wrapper extra_key \
  malformed_digest bad_schema bad_repository bad_architecture declared_extraction; do
  new_case "build-$mutation"
  case "$mutation" in
    missing_payload) expression='del(.payload_sha256)' ;;
    empty_payload) expression='.payload_sha256={}' ;;
    missing_metadata) expression='del(.metadata_sha256)' ;;
    empty_metadata) expression='.metadata_sha256={}' ;;
    missing_wrapper) expression='del(.payload_sha256["verify-call"])' ;;
    extra_key) expression='.payload_sha256.extra="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"' ;;
    malformed_digest) expression='.payload_sha256["verify-call"]="short"' ;;
    bad_schema) expression='.schema="unknown"' ;;
    bad_repository) expression='.source_repository="https://example.invalid/other"' ;;
    bad_architecture) expression='.architecture="linux/arm64"' ;;
    declared_extraction) expression='.registered_extraction_executed=true' ;;
  esac
  mutate_json "$bundle/build-info.json" "$expression"
  rebind_build
  expect_failure "$here/verify-artifacts" "$bundle" "$evidence"
  assert_no_execution
  pass "transport rejects self-consistent build manifest: $mutation"
done
for mutation in protocol_bytes payload_bytes metadata_bytes bundle_symlink; do
  new_case "bundle-$mutation"
  case "$mutation" in
    protocol_bytes) printf 'mutation\n' >>"$bundle/PROTOCOL.md"; cp "$bundle/PROTOCOL.md" "$evidence/payload/PROTOCOL.md"; seal_payload ;;
    payload_bytes) printf '# mutation\n' >>"$bundle/verify-call" ;;
    metadata_bytes) printf 'mutation\n' >>"$bundle/source.sha256"; cp "$bundle/source.sha256" "$evidence/payload/source.sha256"; seal_payload ;;
    bundle_symlink) mv "$bundle/verify-call" "$current/wrapper"; ln -s "$current/wrapper" "$bundle/verify-call" ;;
  esac
  expect_failure "$here/verify-artifacts" "$bundle" "$evidence"
  assert_no_execution
  pass "transport rejects changed bundled bytes: $mutation"
done

for mutation in job_image job_name job_command job_args job_retry job_deadline job_memory \
  pod_image pod_image_id pod_uid pod_owner pod_node pod_command pod_args pod_restart pod_restart_count pod_cpu; do
  new_case "runtime-$mutation"
  file="$evidence/launcher/manifest.json"
  case "$mutation" in
    job_image) expression='.spec.template.spec.containers[0].image="unregistered:latest"' ;;
    job_name) expression='.metadata.name="different-job"' ;;
    job_command) expression='.spec.template.spec.containers[0].command += ["unexpected"]' ;;
    job_args) expression='.spec.template.spec.containers[0].args=["unexpected"]' ;;
    job_retry) expression='.spec.backoffLimit=1' ;;
    job_deadline) expression='.spec.activeDeadlineSeconds=301' ;;
    job_memory) expression='.spec.template.spec.containers[0].resources.limits.memory="4Gi"' ;;
    pod_*)
      file="$evidence/launcher/pods.json"
      case "$mutation" in
        pod_image) expression='.items[0].spec.containers[0].image="unregistered:latest"' ;;
        pod_image_id) expression='.items[0].status.containerStatuses[0].imageID="containerd://wrong"' ;;
        pod_uid) expression='.items[0].metadata.uid="33333333-3333-3333-3333-333333333333"' ;;
        pod_owner) expression='.items[0].metadata.ownerReferences[0].uid="33333333-3333-3333-3333-333333333333"' ;;
        pod_node) expression='.items[0].spec.nodeName="different-node"' ;;
        pod_command) expression='.items[0].spec.containers[0].command += ["unexpected"]' ;;
        pod_args) expression='.items[0].spec.containers[0].args=["unexpected"]' ;;
        pod_restart) expression='.items[0].spec.restartPolicy="OnFailure"' ;;
        pod_restart_count) expression='.items[0].status.containerStatuses[0].restartCount=1' ;;
        pod_cpu) expression='.items[0].spec.containers[0].resources.limits.cpu="4"' ;;
      esac ;;
  esac
  mutate_json "$file" "$expression"
  if [[ "$mutation" == job_* ]]; then rebind_manifest; fi
  expect_failure "$here/verify-artifacts" "$bundle" "$evidence"
  assert_no_execution
  pass "transport rejects actual runtime identity mismatch: $mutation"
done

for file in image.json request.json client/result.json launcher/pods.json launcher/manifest.json \
  launcher/result.json launcher/request.json payload/job-exit.json; do
  new_case "json-stream-$passed"
  printf '{}\n' >>"$evidence/$file"
  expect_failure "$here/verify-artifacts" "$bundle" "$evidence"
  assert_no_execution
  pass "transport rejects multiple JSON values in $file"
done
new_case build-json-stream
printf '{}\n' >>"$bundle/build-info.json"
expect_failure "$here/verify-artifacts" "$bundle" "$evidence"
assert_no_execution
pass 'transport rejects multiple JSON values in build-info'

for mutation in parent_path absolute_path dot_component duplicate missing_entry missing_file extra_file changed_bytes symlink; do
  new_case "manifest-$mutation"
  sums="$evidence/payload/SHA256SUMS"
  case "$mutation" in
    parent_path) printf '%s  ./../outside\n' "$image_digest" >>"$sums" ;;
    absolute_path) printf '%s  /tmp/outside\n' "$image_digest" >>"$sums" ;;
    dot_component) printf '%s  ./positive-structured/./producer.stdout\n' "$image_digest" >>"$sums" ;;
    duplicate) head -n 1 "$sums" >>"$sums" ;;
    missing_entry) sed '\|  ./source.sha256$|d' "$sums" >"$current/reduced.sha256"; mv "$current/reduced.sha256" "$sums" ;;
    missing_file) rm "$evidence/payload/source.sha256" ;;
    extra_file) printf 'unmanifested\n' >"$evidence/payload/extra.txt" ;;
    changed_bytes) printf 'mutation\n' >>"$evidence/payload/positive-structured/producer.stdout" ;;
    symlink) ln -s "$bundle/source.sha256" "$evidence/payload/link" ;;
  esac
  expect_failure "$here/verify-artifacts" "$bundle" "$evidence"
  assert_no_execution
  pass "transport rejects unsafe/incomplete checksum inventory: $mutation"
done

new_case replay-good
expect_status 0 "$here/replay" "$bundle" "$evidence" "$current/replayed"
assert_json "$current/replayed/replay.json" '.fixed_gate_passed==true and .transport_bindings_passed==true
  and .all_four_calls_replayed==true and .per_call_identity_checks_passed==true
  and .independent_completeness_proof==false and .complete_cold_cost==null and .signed_pcc==false'
[[ $(wc -l <"$MOCK_ARTIFACT_CHECKERS") == 4 ]] || fail 'complete synthetic replay did not invoke four mock checkers'
[[ ! -s "$MOCK_ARTIFACT_FORBIDDEN" ]] || fail 'replay executed extraction'
pass 'four mock call replays pass while independent completeness and cold-cost remain unclaimed'

for mutation in pre_identity pre_stream post_failure post_stream missing_call checker_failure checker_timeout \
  failed_job sequence_stream sequence_failed followups_not_run; do
  new_case "replay-$mutation"
  expected_replayed=true expected_identity=true
  case "$mutation" in
    pre_identity) mutate_json "$evidence/payload/positive-structured/pre-call.json" '.source_revision="0000000000000000000000000000000000000000"'; expected_identity=false ;;
    pre_stream) printf '{}\n' >>"$evidence/payload/positive-structured/pre-call.json"; expected_identity=false ;;
    post_failure) mutate_json "$evidence/payload/positive-structured/post-call.json" '.accepted=false'; expected_identity=false ;;
    post_stream) printf '{}\n' >>"$evidence/payload/positive-structured/post-call.json"; expected_identity=false ;;
    missing_call) rm "$evidence/payload/h0-structured/producer.stdout"; expected_replayed=false; expected_identity=false ;;
    checker_failure) export MOCK_ARTIFACT_CHECKER_FAIL=positive-structured; expected_replayed=false; expected_identity=false ;;
    checker_timeout) export MOCK_ARTIFACT_TIMEOUT_CASE=positive-structured; expected_replayed=false; expected_identity=false ;;
    failed_job)
      mutate_json "$evidence/client/result.json" '.state="failed" | .exit_code=7 | .pods[0].exit_code=7'
      cp "$evidence/client/result.json" "$evidence/launcher/result.json"
      mutate_json "$evidence/launcher/pods.json" '.items[0].status.containerStatuses[0].state.terminated.exitCode=7'
      mutate_json "$evidence/payload/job-exit.json" '.exit_status=7 | .scientific_run_completed=false' ;;
    sequence_stream) printf '{}\n' >>"$evidence/payload/sequence.json" ;;
    sequence_failed) mutate_json "$evidence/payload/sequence.json" '.all_requested_calls_accepted=false' ;;
    followups_not_run) printf '{"schema":"rr5.b677.not-run/v1"}\n' >"$evidence/payload/followups-not-run.json" ;;
  esac
  seal_payload
  # These records have valid transport. Mutations must be rejected by replay's
  # outcome or call-identity gates, without mislabeling the first transport gate.
  expect_status 0 "$here/verify-artifacts" "$bundle" "$evidence"
  assert_no_execution
  expect_status 1 "$here/replay" "$bundle" "$evidence" "$current/replayed"
  assert_json "$current/replayed/replay.json" ".fixed_gate_passed==false and .transport_bindings_passed==true
    and .all_four_calls_replayed==$expected_replayed and .per_call_identity_checks_passed==$expected_identity
    and .independent_completeness_proof==false and .complete_cold_cost==null and .signed_pcc==false"
  [[ ! -s "$MOCK_ARTIFACT_FORBIDDEN" ]] || fail 'failed replay executed extraction'
  pass "replay preserves truthful failure/identity outcomes: $mutation"
done

for mutation in good launcher_failure payload_failure both_failures; do
  new_case "collect-$mutation"
  expected_launcher=0 expected_payload=0
  case "$mutation" in
    launcher_failure) export MOCK_ARTIFACT_LAUNCHER_FAIL=1; expected_launcher=7 ;;
    payload_failure) export MOCK_ARTIFACT_PAYLOAD_FAIL=1; expected_payload=8 ;;
    both_failures) export MOCK_ARTIFACT_LAUNCHER_FAIL=1 MOCK_ARTIFACT_PAYLOAD_FAIL=1; expected_launcher=7; expected_payload=8 ;;
  esac
  if [[ "$mutation" == good ]]; then
    expect_status 0 "$here/collect" "$evidence/image.json" "$evidence/request.json" "$evidence/client" "$bundle" "$current/collected"
  else
    expect_failure "$here/collect" "$evidence/image.json" "$evidence/request.json" "$evidence/client" "$bundle" "$current/collected"
  fi
  assert_json "$current/collected/collection.json" ".launcher_download_exit==$expected_launcher and .payload_download_exit==$expected_payload"
  [[ $(wc -l <"$MOCK_ARTIFACT_CLOUD") == 2 ]] || fail 'collector did not attempt both independent downloads'
  [[ $(sed -n '1p' "$MOCK_ARTIFACT_CLOUD") == "$launcher_uri" && $(sed -n '2p' "$MOCK_ARTIFACT_CLOUD") == "$payload_uri" ]] || fail 'collector used the wrong custody prefixes'
  if [[ "$expected_launcher" == 0 ]]; then cmp "$evidence/launcher/result.json" "$current/collected/launcher/result.json"; fi
  if [[ "$expected_payload" == 0 ]]; then cmp "$evidence/payload/job-exit.json" "$current/collected/payload/job-exit.json"; fi
  cmp "$evidence/client/result.json" "$current/collected/client/result.json"
  assert_no_execution
  pass "collection retains independent download outcomes without executing payload: $mutation"
done
printf '1..%s\n' "$passed"
