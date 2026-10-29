#!/bin/bash
# Thin native-auditor orchestration. Every eligible record is replayed; no timing claim.
# Usage: replay_partition.sh ecbench outdir zero-based-partition partition-count
set -euo pipefail
EB=${1:?native ecbench}; OUT=${2:?output}; part=${3:?partition}; parts=${4:?partition count}
[[ $part =~ ^[0-9]+$ && $parts =~ ^[1-9][0-9]*$ ]] && ((part < parts)) || exit 2
here=$(cd "$(dirname "$0")" && pwd)
plan=${REPLAY_PLAN:-docs/ic/validation/replay-plan.json}
jq -e '.schema == "ic-replay-plan/v1" and (.sessions | length > 0) and
  ([.sessions[].dir] | length == (unique | length)) and
  all(.sessions[]; .replay == "all" and (.expect == "identical" or .expect == "drift") and
    (.dir | test("^[A-Za-z0-9_./-]+$") and (startswith("/") | not) and
      (split("/") | index("..") | not)))' "$plan" >/dev/null
mkdir -p "$OUT"
binary=$(sha256sum "$EB" | cut -d' ' -f1)
plan_hash=$(sha256sum "$plan" | cut -d' ' -f1)
roster_hash=$(jq -c '[.sessions[]|{dir,replay}]' "$plan" | sha256sum | cut -d' ' -f1)
check_native() { jq -e -L "$here" --arg binary "$binary" 'include "replay_native"; valid_native($binary)' "$1" >/dev/null; }
check_files() {
  local receipt=$1 dir=$2 name expected actual
  while IFS=$'\t' read -r name expected; do
    actual=$(sha256sum "$dir/$name" | cut -d' ' -f1) || return 1
    [[ $actual == "$expected" ]] || return 1
  done < <(jq -r '.files | to_entries[] | [.key,.value] | @tsv' "$receipt")
}
audit() {
  local receipt=$1; shift
  rm -f "$receipt"
  local status=0
  "$EB" verify --dir "$dir" "$@" --out "$receipt" > /dev/null 2>"$receipt.stderr" || status=$?
  printf '%s\n' "$status" > "$receipt.exit"
  ((status <= 1)) && check_native "$receipt" && check_files "$receipt" "$dir" || return 1
  if [[ $(jq -r .ok "$receipt") == true ]]; then [[ $status == 0 ]]; else [[ $status == 1 ]]; fi
}
n=$(jq '.sessions|length' "$plan")
start=${REPLAY_START_INDEX:-0}
[[ $start =~ ^[0-9]+$ ]] && ((start < n)) || exit 2
group=${REPLAY_GROUP_INDEX:-0}; groups=${REPLAY_GROUP_COUNT:-1}
[[ $group =~ ^[0-9]+$ && $groups =~ ^[1-9][0-9]*$ ]] && ((group < groups && groups <= n)) || exit 2
for ((offset=0; offset<n; offset++)); do
  i=$(((offset+start)%n))
  ((i % groups == group)) || continue
  dir=$(jq -r ".sessions[$i].dir" "$plan")
  tag=$(printf '%04d' "$i"); base="$OUT/$tag"
  rm -f "$base.partition.json"
  # This is a proposed set only: native candidate cardinality and explicit
  # native selection below must certify the entire set, or this job fails.
  # Mirror audit.rs's registry predicate only to propose a set. Native count
  # equality plus native validation of every selected sequence certifies it.
  jq -s '[.[] | select(.warmup == false and
    (.outcome.status == "verified" or
      ((.outcome.status == "wrong_answer" or .outcome.status == "exhausted" or .outcome.status == "error") and
       (.session_id as $sid | ["ECBS1hedc734e1870d","ECBS1h4c7b9db8a389","ECBS1h302934ff99ed"] | index($sid) | not))) and
    (.method.id | startswith("ic.cm_folded_") | not) and
    (if ((.method.params.solver_budget_seconds // "0" | tonumber? // 0) > 0)
     then .cost.deterministic == true and (.cost.total_gae | type == "number")
     else true end)) |
    {seq,record_id}] | sort_by(.seq)' "$dir/records.jsonl" > "$base.candidates.json"
  jq -e 'length > 0 and all(.[]; (.seq|type=="number" and .>=0 and .==floor) and
    (.record_id|type=="string" and length>0)) and
    ([.[].seq]|length==(unique|length))' "$base.candidates.json" >/dev/null
  jq --argjson p "$part" --argjson n "$parts" '[to_entries[] | select(.key % $n == $p) | .value]' \
    "$base.candidates.json" > "$base.selected.json"
  mode=explicit
  cache=${REPLAY_RECEIPT_CACHE:-}/$tag.receipt.json
  if [[ -n ${REPLAY_RECEIPT_CACHE:-} && -s $cache ]]; then
    check_native "$cache"
    check_files "$cache" "$dir"
    jq -e -L "$here" --slurpfile candidates "$base.candidates.json" 'include "replay_native";
      .replay_selection == "all" and .replay_candidates == ($candidates[0]|length) and
      (.replays|replay_keys) == $candidates[0]' "$cache" >/dev/null
    cp "$cache" "$base.receipt.json"
    mode=cached-full
  else
    audit "$base.probe.json" --replay 1 --replay-seed 0
    jq -e --slurpfile candidates "$base.candidates.json" '
      .replay_selection == "sample" and .replay_candidates == ($candidates[0]|length) and
      (.replays|length) == 1 and
      ([.replays[]|{seq,record_id}] - $candidates[0] | length) == 0' "$base.probe.json" >/dev/null
    mapfile -t seqs < <(jq -r '.[].seq' "$base.selected.json")
    if ((${#seqs[@]})); then
      args=(--replay 0)
      for seq in "${seqs[@]}"; do args+=(--replay-seq "$seq"); done
      audit "$base.receipt.json" "${args[@]}"
      jq -e -L "$here" --slurpfile selected "$base.selected.json" --slurpfile probe "$base.probe.json" \
        'include "replay_native"; .replay_selection == "explicit" and
        (.replays|replay_keys) == $selected[0] and .files == $probe[0].files and
        .session_id == $probe[0].session_id' "$base.receipt.json" >/dev/null
    else
      mode=empty
      rm -f "$base.receipt.json"
    fi
  fi
  jq -n --arg schema ic-replay-partition/v1 --arg dir "$dir" --arg binary "$binary" \
    --arg plan_sha256 "$plan_hash" --arg roster_sha256 "$roster_hash" --arg mode "$mode" --argjson index "$i" \
    --argjson partition "$part" --argjson partitions "$parts" \
    '{schema:$schema,dir:$dir,binary:$binary,plan_sha256:$plan_sha256,
      roster_sha256:$roster_sha256,mode:$mode,index:$index,
      partition:$partition,partitions:$partitions}' > "$base.partition.json"
  echo "$dir: partition $part/$parts $mode ($(jq length "$base.selected.json") records)"
done
