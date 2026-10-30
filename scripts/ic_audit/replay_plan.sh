#!/bin/bash
# Replays every session of docs/ic/validation/replay-plan.json with `ecbench verify` and compares the outcome
# with the plan's `expect`.  Exit 1 on new drift, on a stale `drift` entry, or on a missing receipt.
# usage: replay_plan.sh <ecbench-binary> <outdir>         Needs jq.  Run from the repository root.
set -uo pipefail
EB="${1:?usage: replay_plan.sh ecbench outdir}"; OUT="${2:?usage: replay_plan.sh ecbench outdir}"; mkdir -p "$OUT"
plan=${REPLAY_PLAN:-docs/ic/validation/replay-plan.json}
here=$(cd "$(dirname "$0")" && pwd)
jq -e -L "$here" 'include "replay_native"; valid_drift_expectations' "$plan" >/dev/null || { echo 'invalid drift expectation pins'; exit 1; }
jq -e '.schema == "ic-replay-plan/v1" and
  (.sessions | type == "array" and length > 0) and
  ([.sessions[].dir] | length == (unique | length)) and
  all(.sessions[]; (.dir | type == "string" and test("^[A-Za-z0-9_./-]+$") and (startswith("/") | not) and (split("/") | index("..") | not)) and
    (.expect == "identical" or .expect == "drift") and
    (.replay == "all" or (.replay | type == "number" and . > 0 and . == floor)))' "$plan" > /dev/null || { echo 'invalid or empty replay plan'; exit 1; }
bad=0
n=$(jq '.sessions | length' "$plan")
for i in $(seq 0 $((n - 1))); do
  dir=$(jq -r ".sessions[$i].dir" "$plan"); rep=$(jq -r ".sessions[$i].replay" "$plan"); exp=$(jq -r ".sessions[$i].expect" "$plan")
  # The plan index prevents distinct directory spellings from sharing a receipt.
  tag=$(printf '%04d' "$i")
  flag=(--replay-all); [ "$rep" = all ] || flag=(--replay "$rep")
  zero_pin=$(jq -c ".sessions[$i].zero_replay_probe // null" "$plan")
  [ "$zero_pin" = null ] || flag=(--replay 1 --replay-seed 0)
  rm -f "$OUT/$tag.receipt.json"
  status=0
  "$EB" verify --dir "$dir" "${flag[@]}" --out "$OUT/$tag.receipt.json" > /dev/null 2> "$OUT/$tag.stderr" || status=$?
  [ -s "$OUT/$tag.receipt.json" ] || { echo "$dir: no receipt"; bad=1; continue; }
  if [ "$zero_pin" != null ]; then
    binary=$(sha256sum "$EB" | cut -d' ' -f1)
    if [ "$status" != 0 ] || ! jq -e -L "$here" --arg binary "$binary" --argjson pin "$zero_pin" \
        'include "replay_native"; valid_zero_probe($binary; $pin)' "$OUT/$tag.receipt.json" >/dev/null; then
      echo "$dir: invalid pinned zero-candidate native probe (exit $status)"; bad=1; continue
    fi
    file_ok=true
    while IFS=$'\t' read -r name expected; do
      actual=$(sha256sum "$dir/$name" | cut -d' ' -f1) || { file_ok=false; break; }
      [ "$actual" = "$expected" ] || { file_ok=false; break; }
    done < <(jq -r '.files|to_entries[]|[.key,.value]|@tsv' "$OUT/$tag.receipt.json")
    if [ "$file_ok" != true ]; then
      echo "$dir: pinned static audit input changed"; bad=1; continue
    fi
    echo "$dir: static audit only ($(jq .records "$OUT/$tag.receipt.json") records, zero native replay candidates), exact inputs pinned"
    continue
  fi
  if [ "$status" -gt 1 ] || ! jq -e '.schema == "ecbench.audit/v1" and
    .session_status == "complete" and .records_bound == true and
    (.replays | type == "array" and length > 0) and
    all(.replays[]; (.reproduced | type == "boolean")) and
    (.checks | type == "object" and length > 0) and
    all(.checks | to_entries[]; .key == "replays" or .value == true) and
    (.problems | type == "array") and
    all(.problems[]; startswith("replay of seq "))' "$OUT/$tag.receipt.json" > /dev/null; then
    echo "$dir: malformed or empty replay receipt (exit $status)"; bad=1; continue
  fi
  total=$(jq '.replays | length' "$OUT/$tag.receipt.json"); ok=$(jq '[.replays[] | select(.reproduced)] | length' "$OUT/$tag.receipt.json")
  entry=$(jq -c ".sessions[$i]" "$plan")
  if ! jq -e -L "$here" --argjson entry "$entry" \
      'include "replay_native"; same_pinned_drifts($entry)' "$OUT/$tag.receipt.json" >/dev/null; then
    echo "$dir: native drift identities/details differ from the evidence-backed pin"; bad=1; continue
  fi
  if [ "$ok" = "$total" ]; then got=identical; else got=drift; fi
  if [ "$got" = identical ] && [ "$status" != 0 ]; then
    echo "$dir: auditor failed with exit $status despite identical replays; inspect the receipt's other checks"; bad=1
  elif [ "$got" = "$exp" ]; then echo "$dir: $got ($ok/$total), as planned"
  elif [ "$got" = drift ]; then echo "$dir: NEW DRIFT ($ok/$total reproduced) on a session planned as identical"; bad=1
  else echo "$dir: STALE ENTRY: planned as drift but all $total replays reproduce; remove or update its plan entry"; bad=1; fi
done
exit $bad
