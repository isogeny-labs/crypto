#!/usr/bin/env bash
# Run exactly one registered public-target selector-SAT row.
# The checkpoint gate is intentionally the first mutating/solver action.
set -euo pipefail

if [ "$#" -lt 4 ] || [ "$#" -gt 6 ]; then
  echo "usage: $0 <cms-cnf|cms-xnf|cadical-cnf|native-cnf|native-xnf> <relation.json> <artifact-dir> <wall-seconds> [conflict-budget] [native-priority|native-plain]" >&2
  exit 2
fi

solver="$1"
relation="$2"
artifact_dir="$3"
wall_seconds="$4"
conflict_budget="${5:-1000000000}"
native_mode="${6:-native-priority}"
memory_limit_kib=16777216

repo="$(git rev-parse --show-toplevel)"
cd "$repo"
scripts/checkpoint.sh gate

case "$artifact_dir" in
  ""|/|.|"$repo") echo "refusing unsafe artifact directory: $artifact_dir" >&2; exit 2 ;;
esac
case "$wall_seconds" in *[!0-9]*|"") echo "wall-seconds must be a positive integer" >&2; exit 2;; esac
case "$conflict_budget" in *[!0-9]*|"") echo "conflict-budget must be a positive integer" >&2; exit 2;; esac
[ "$wall_seconds" -gt 0 ] || { echo "wall-seconds must be positive" >&2; exit 2; }
[ "$conflict_budget" -gt 0 ] || { echo "conflict-budget must be positive" >&2; exit 2; }
[ -f "$relation" ] || { echo "missing relation: $relation" >&2; exit 2; }
[ -x /usr/bin/time ] || { echo "GNU /usr/bin/time is required" >&2; exit 2; }

candidate="$repo/research/prime31_shared_coefficient_membership_20261118/native/target/release/prime31-selector-sat"
external="$repo/tools/b0-ext-solvers/target/release/b0-ext-solvers"
native="$repo/research/prime31_shared_coefficient_membership_20261118/native-solver/target/release/prime31-selector-native-solver"
receipt="$repo/research/prime31_shared_coefficient_membership_20261118/toolchain/build-receipt.json"

check_hash() {
  local file="$1" field="$2" expected actual
  [ -x "$file" ] || { echo "missing executable: $file" >&2; exit 2; }
  expected="$(jq -er ".$field" "$receipt")"
  actual="$(sha256sum "$file" | cut -d' ' -f1)"
  [ "$actual" = "$expected" ] || {
    echo "binary hash mismatch for $file: $actual != $expected" >&2
    exit 2
  }
}

check_hash "$candidate" selector_binary_sha256
case "$solver" in
  cms-cnf|cms-xnf|cadical-cnf) check_hash "$external" external_solver_binary_sha256 ;;
  native-cnf|native-xnf) check_hash "$native" native_solver_binary_sha256 ;;
  *) echo "unknown solver arm: $solver" >&2; exit 2 ;;
esac
branch_mode=null
arm="$solver"
if [[ "$solver" = native-* ]]; then
  if [ "$native_mode" = native-priority ]; then branch_mode=priority
  elif [ "$native_mode" = native-plain ]; then branch_mode=plain
  else echo "native mode must be native-priority or native-plain" >&2; exit 2
  fi
  arm="$solver-$branch_mode"
fi

mkdir -p "$artifact_dir"
case_id="$(jq -er '.case_id' "$relation")"
if [[ "$case_id" =~ ^prime31-w8-b(4|6|8)-(natural|planted)-([0-9]{2})$ ]]; then
  base_size="${BASH_REMATCH[1]}"
  case_class="${BASH_REMATCH[2]}"
  panel_index="${BASH_REMATCH[3]}"
else
  echo "relation is not a registered B4/B6/B8 weighted-panel case: $case_id" >&2
  exit 2
fi

# Recreate the deterministic public relation and compare canonical JSON.  This
# rejects edited targets, slot tables, and held-out rungs before formula export.
panel_check="$(mktemp "${TMPDIR:-/tmp}/prime31-panel-check.XXXXXX")"
trap 'rm -f -- "$panel_check"' EXIT
"$candidate" generate "$base_size" "$case_class" "$panel_index" "$panel_check"
provided_relation_digest="$(jq -S -c . "$relation" | sha256sum | cut -d' ' -f1)"
frozen_relation_digest="$(jq -S -c . "$panel_check" | sha256sum | cut -d' ' -f1)"
if [ "$provided_relation_digest" != "$frozen_relation_digest" ]; then
  echo "relation does not match the frozen deterministic panel: $case_id" >&2
  exit 2
fi

case "$solver" in
  cms-xnf|native-xnf) rendering="xnf" ;;
  *) rendering="cnf" ;;
esac
formula="$artifact_dir/$case_id.$arm.$rendering"
manifest="$artifact_dir/$case_id.$arm.encoding.json"
model="$artifact_dir/$case_id.$arm.model"
solver_record="$artifact_dir/$case_id.$arm.solver.json"
solver_stderr="$artifact_dir/$case_id.$arm.stderr"
runner_stdout="$artifact_dir/$case_id.$arm.stdout"
replay="$artifact_dir/$case_id.$arm.replay.json"
replay_stderr="$artifact_dir/$case_id.$arm.replay.stderr"
resource_record="$artifact_dir/$case_id.$arm.resource.json"
command_record="$artifact_dir/$case_id.$arm.command"
summary="$artifact_dir/$case_id.$arm.summary.json"

for output in "$formula" "$manifest" "$model" "$solver_record" "$replay" \
  "$resource_record" "$command_record" "$summary"; do
  [ ! -e "$output" ] || {
    echo "refusing to overwrite existing artifact: $output" >&2
    exit 2
  }
done

"$candidate" encode "$relation" "$rendering" "$formula" "$manifest"
semantic_bits="$(jq -er '.statistics.semantic_input_bits' "$manifest")"
jq -e '.statistics.forbidden | [.[]] | all(. == 0)' "$manifest" >/dev/null
if [ "$rendering" = xnf ]; then
  jq -e '.statistics.cnf_xor_rows > 0' "$manifest" >/dev/null
fi

run_capped() {
  (
    ulimit -v "$memory_limit_kib"
    exec /usr/bin/time --quiet \
      -f '{"user_seconds":%U,"system_seconds":%S,"elapsed_seconds":%e,"max_rss_kib":%M,"exit_status":%x}' \
      -o "$resource_record" "$@"
  )
}

set +e
case "$solver" in
  cms-cnf|cms-xnf)
    solver_command=(timeout --signal=TERM --kill-after=10s "${wall_seconds}s"
      "$external" --solver cms --file "$formula" --budget "$conflict_budget"
      --model-vars "$semantic_bits")
    printf '%q ' "${solver_command[@]}" >"$command_record"
    printf '\n' >>"$command_record"
    run_capped "${solver_command[@]}" >"$solver_record" 2>"$solver_stderr"
    solver_exit=$?
    ;;
  cadical-cnf)
    solver_command=(timeout --signal=TERM --kill-after=10s "${wall_seconds}s"
      "$external" --solver cadical --file "$formula" --budget "$conflict_budget"
      --model-vars "$semantic_bits")
    printf '%q ' "${solver_command[@]}" >"$command_record"
    printf '\n' >>"$command_record"
    run_capped "${solver_command[@]}" >"$solver_record" 2>"$solver_stderr"
    solver_exit=$?
    ;;
  native-cnf|native-xnf)
    solver_command=(timeout --signal=TERM --kill-after=10s "${wall_seconds}s"
      "$native" "$rendering" "$formula" "$model" "$solver_record"
      "$wall_seconds" "$conflict_budget" "$branch_mode")
    printf '%q ' "${solver_command[@]}" >"$command_record"
    printf '\n' >>"$command_record"
    run_capped "${solver_command[@]}" >"$runner_stdout" 2>"$solver_stderr"
    solver_exit=$?
    ;;
esac
set -e

if ! jq -e . "$resource_record" >/dev/null 2>&1; then
  jq -n --arg status missing_resource_record \
    '{status:$status,pcc:"unverified"}' >"$resource_record"
fi

synthetic_status=""
if [ "$solver_exit" -eq 124 ]; then
  synthetic_status=timeout
elif [ "$solver_exit" -eq 137 ]; then
  elapsed_seconds="$(jq -r '.elapsed_seconds // -1' "$resource_record")"
  if awk -v elapsed="$elapsed_seconds" -v cap="$wall_seconds" \
    'BEGIN { exit !(elapsed >= cap) }'; then
    synthetic_status=timeout
  else
    synthetic_status=sigkill_unknown
  fi
elif grep -Eiq 'bad_alloc|out of memory|cannot allocate memory|memory allocation' "$solver_stderr"; then
  synthetic_status=oom
elif [ "$solver_exit" -ne 0 ]; then
  synthetic_status=process_error
elif ! jq -e '.status | type == "string"' "$solver_record" >/dev/null 2>&1; then
  synthetic_status=malformed_solver_record
fi
if [ -n "$synthetic_status" ]; then
  if [ -s "$solver_record" ]; then
    mv "$solver_record" "$solver_record.raw"
  fi
  jq -n --arg solver "$solver" --arg status "$synthetic_status" \
    --argjson exit_code "$solver_exit" \
    '{schema:"prime31.selector-synthetic-solver-record/v1",solver:$solver,
      status:$status,exit_code:$exit_code,pcc:"unverified"}' >"$solver_record"
fi

status="$(jq -er '.status' "$solver_record")"
replay_ok=false
replay_exit=null
if [ "$status" = sat ]; then
  set +e
  if [[ "$solver" = native-* ]]; then
    "$candidate" verify "$relation" "$model" >"$replay" 2>"$replay_stderr"
  else
    "$candidate" verify-prefix "$relation" "$solver_record" >"$replay" 2>"$replay_stderr"
  fi
  replay_exit=$?
  set -e
  if [ "$replay_exit" -eq 0 ]; then replay_ok=true; fi
fi

if [ "$status" = sat ] && [ "$replay_ok" = true ]; then disposition=verified_sat
elif [ "$status" = sat ]; then disposition=implementation_failure_invalid_sat_model
elif [ "$status" = unsat ] && [ "$case_class" = planted ]; then
  disposition=implementation_failure_missed_planted
elif [ "$status" = unsat ]; then disposition=unchecked_unsat_unknown
elif [ "$status" = timeout ]; then disposition=timeout_unknown
elif [ "$status" = oom ]; then disposition=oom_unknown
elif [ "$status" = sigkill_unknown ]; then disposition=sigkill_unknown
else disposition="${status}_unknown"
fi

formula_sha="$(sha256sum "$formula" | cut -d' ' -f1)"
relation_sha="$(sha256sum "$relation" | cut -d' ' -f1)"
manifest_sha="$(sha256sum "$manifest" | cut -d' ' -f1)"
record_sha="$(sha256sum "$solver_record" | cut -d' ' -f1)"
command_sha="$(sha256sum "$command_record" | cut -d' ' -f1)"
candidate_binary_sha="$(jq -er '.selector_binary_sha256' "$receipt")"
case "$solver" in
  cms-*|cadical-*) solver_binary_sha="$(jq -er '.external_solver_binary_sha256' "$receipt")" ;;
  native-*) solver_binary_sha="$(jq -er '.native_solver_binary_sha256' "$receipt")" ;;
esac
jq -n \
  --arg case_id "$case_id" --arg arm "$arm" --arg solver "$solver" \
  --arg branch_mode "$branch_mode" --arg status "$status" \
  --arg case_class "$case_class" --argjson base_size "$base_size" \
  --arg disposition "$disposition" --arg formula_sha256 "$formula_sha" \
  --arg relation_sha256 "$relation_sha" --arg encoding_manifest_sha256 "$manifest_sha" \
  --arg solver_record_sha256 "$record_sha" --argjson solver_exit "$solver_exit" \
  --arg command_sha256 "$command_sha" --arg candidate_binary_sha256 "$candidate_binary_sha" \
  --arg solver_binary_sha256 "$solver_binary_sha" --argjson replay_exit "$replay_exit" \
  --argjson replay_ok "$replay_ok" --argjson wall_cap_seconds "$wall_seconds" \
  --argjson conflict_budget "$conflict_budget" \
  --argjson memory_cap_bytes "$((memory_limit_kib * 1024))" \
  --slurpfile encoding "$manifest" --slurpfile resource "$resource_record" \
  '{schema:"prime31.selector-case-summary/v1",case_id:$case_id,base_size:$base_size,
    case_class:$case_class,arm:$arm,solver:$solver,
    branch_mode:(if $branch_mode == "null" then null else $branch_mode end),status:$status,
    disposition:$disposition,solver_exit:$solver_exit,replay_ok:$replay_ok,
    replay_exit:$replay_exit,wall_cap_seconds:$wall_cap_seconds,
    memory_cap_bytes:$memory_cap_bytes,conflict_budget:$conflict_budget,
    relation_sha256:$relation_sha256,encoding_manifest_sha256:$encoding_manifest_sha256,
    formula_sha256:$formula_sha256,solver_record_sha256:$solver_record_sha256,
    command_sha256:$command_sha256,candidate_binary_sha256:$candidate_binary_sha256,
    solver_binary_sha256:$solver_binary_sha256,encoding:$encoding[0].statistics,
    process_resources:$resource[0],
    expected_label_consumed_by_solver:false,forbidden_counters_all_zero:true,pcc:"unverified"}' \
  >"$summary"

printf '%s\n' "$summary"
