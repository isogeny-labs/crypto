#!/bin/bash
# Thin native-auditor orchestration. Every eligible record is replayed; no timing claim.
# Usage: replay_partition.sh ecbench outdir zero-based-partition partition-count
set -euo pipefail
EB=${1:?native ecbench}; OUT=${2:?output}; part=${3:?partition}; parts=${4:?partition count}
[[ $part =~ ^[0-9]+$ && $parts =~ ^[1-9][0-9]*$ ]] && ((part < parts)) || exit 2
here=$(cd "$(dirname "$0")" && pwd)
plan=${REPLAY_PLAN:-docs/ic/validation/replay-plan.json}
jq -e -L "$here" 'include "replay_native"; valid_drift_expectations' "$plan" >/dev/null
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
check_zero_probe() {
  jq -e -L "$here" --arg binary "$binary" --argjson pin "$zero_pin" \
    'include "replay_native"; valid_zero_probe($binary; $pin)' "$1" >/dev/null
}
check_status() {
  [[ -s $1.exit ]] &&
    [[ $(cat "$1.exit") == "$(jq -r 'if .ok then 0 else 1 end' "$1")" ]]
}
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
  ((status <= 1)) && { check_native "$receipt" || check_zero_probe "$receipt"; } &&
    check_files "$receipt" "$dir" || return 1
  if [[ $(jq -r .ok "$receipt") == true ]]; then [[ $status == 0 ]]; else [[ $status == 1 ]]; fi
}
check_partition_cache() {
  local cached=$1
  jq -e --arg dir "$dir" --arg binary "$binary" --arg roster "$roster_hash" \
    --argjson i "$i" --argjson p "$part" --argjson n "$parts" '
    .schema == "ic-replay-partition/v1" and .dir == $dir and .binary == $binary and
    .roster_sha256 == $roster and .index == $i and .partition == $p and .partitions == $n and
    (.mode == "explicit" or .mode == "cached-explicit")' "$cached.partition.json" >/dev/null || return 1
  cmp "$cached.candidates.json" "$base.candidates.json" || return 1
  cmp "$cached.selected.json" "$base.selected.json" || return 1
  check_native "$cached.probe.json" || return 1
  check_native "$cached.receipt.json" || return 1
  check_files "$cached.receipt.json" "$dir" || return 1
  jq -e -L "$here" --slurpfile candidates "$base.candidates.json" \
    --slurpfile selected "$base.selected.json" --slurpfile probe "$cached.probe.json" '
    include "replay_native";
    $probe[0].replay_selection == "sample" and
    $probe[0].replay_candidates == ($candidates[0]|length) and ($probe[0].replays|length) == 1 and
    ([$probe[0].replays[]|{seq,record_id}] - $candidates[0]|length) == 0 and
    .replay_selection == "explicit" and (.replays|replay_keys) == $selected[0] and
    .files == $probe[0].files and .session_id == $probe[0].session_id' "$cached.receipt.json" >/dev/null || return 1
  local kind
  for kind in probe receipt; do
    [[ $(cat "$cached.$kind.json.exit") == "$(jq -r 'if .ok then 0 else 1 end' "$cached.$kind.json")" ]] || return 1
  done
}
check_probe() {
  if [[ $(jq length "$base.candidates.json") == 0 ]]; then
    check_zero_probe "$1" && check_files "$1" "$dir" && check_status "$1"
    return
  fi
  check_native "$1" && check_files "$1" "$dir" && check_status "$1" || return 1
  jq -e --slurpfile candidates "$base.candidates.json" '
    .replay_selection == "sample" and .replay_candidates == ($candidates[0]|length) and
    (.replays|length) == 1 and
    ([.replays[]|{seq,record_id}] - $candidates[0] | length) == 0' "$1" >/dev/null
}
check_chunk() {
  check_native "$1" && check_files "$1" "$dir" && check_status "$1" || return 1
  jq -e -L "$here" --slurpfile selected "$2" --slurpfile probe "$base.probe.json" '
    include "replay_native"; .replay_selection == "explicit" and
    (.replays|replay_keys) == $selected[0] and .files == $probe[0].files and
    .session_id == $probe[0].session_id' "$1" >/dev/null
}
n=$(jq '.sessions|length' "$plan")
start=${REPLAY_START_INDEX:-0}
[[ $start =~ ^[0-9]+$ ]] && ((start < n)) || exit 2
group=${REPLAY_GROUP_INDEX:-0}; groups=${REPLAY_GROUP_COUNT:-1}
[[ $group =~ ^[0-9]+$ && $groups =~ ^[1-9][0-9]*$ ]] && ((group < groups && groups <= n)) || exit 2
chunk_size=${REPLAY_CHUNK_SIZE:-0}
[[ $chunk_size =~ ^[0-9]+$ ]] || exit 2
for ((offset=0; offset<n; offset++)); do
  i=$(((offset+start)%n))
  ((i % groups == group)) || continue
  dir=$(jq -r ".sessions[$i].dir" "$plan")
  zero_pin=$(jq -c ".sessions[$i].zero_replay_probe // null" "$plan")
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
  jq -e -L "$here" --argjson pin "$zero_pin" 'include "replay_native";
    (if $pin == null then length > 0 else length == 0 and ($pin | valid_zero_pin) end) and
    all(.[]; (.seq|type=="number" and .>=0 and .==floor) and
    (.record_id|type=="string" and length>0)) and
    ([.[].seq]|length==(unique|length))' "$base.candidates.json" >/dev/null
  jq --argjson p "$part" --argjson n "$parts" '[to_entries[] | select(.key % $n == $p) | .value]' \
    "$base.candidates.json" > "$base.selected.json"
  mode=explicit
  cache_binding=null
  cache=${REPLAY_RECEIPT_CACHE:-}/$tag.receipt.json
  partial=${REPLAY_PARTITION_CACHE:-}/$tag
  if [[ -n ${REPLAY_PARTITION_CACHE:-} && -s $partial.partition.json ]] &&
      jq -e '.mode == "explicit" or .mode == "cached-explicit"' "$partial.partition.json" >/dev/null; then
    check_partition_cache "$partial" || { echo "$dir: invalid completed partition cache"; exit 1; }
    for suffix in probe.json probe.json.exit probe.json.stderr receipt.json receipt.json.exit receipt.json.stderr; do
      cp "$partial.$suffix" "$base.$suffix"
    done
    cp "$partial.partition.json" "$base.cache-source.json"
    cache_binding=$(sha256sum "$base.cache-source.json" | cut -d' ' -f1)
    mode="cached-explicit"
  elif [[ -n ${REPLAY_RECEIPT_CACHE:-} && -s $cache ]]; then
    check_native "$cache"
    check_files "$cache" "$dir"
    jq -e -L "$here" --slurpfile candidates "$base.candidates.json" 'include "replay_native";
      .replay_selection == "all" and .replay_candidates == ($candidates[0]|length) and
      (.replays|replay_keys) == $candidates[0]' "$cache" >/dev/null
    cp "$cache" "$base.receipt.json"
    mode="cached-full"
  else
    chunk_cache=${REPLAY_CHUNK_CACHE:-}/$tag
    if [[ -n ${REPLAY_CHUNK_CACHE:-} && -s $chunk_cache.probe.json.exit ]] &&
        [[ $(cat "$chunk_cache.probe.json.exit") =~ ^[01]$ ]]; then
      check_probe "$chunk_cache.probe.json" || { echo "$dir: invalid cached native probe"; exit 1; }
      for suffix in json json.exit json.stderr; do cp "$chunk_cache.probe.$suffix" "$base.probe.$suffix"; done
    else
      audit "$base.probe.json" --replay 1 --replay-seed 0
    fi
    check_probe "$base.probe.json"
    mapfile -t seqs < <(jq -r '.[].seq' "$base.selected.json")
    if ((${#seqs[@]} && chunk_size)); then
      mode=chunked
      printf '[]\n' > "$base.chunks.json"
      for ((first=0, chunk=0; first<${#seqs[@]}; first+=chunk_size, chunk++)); do
        name="$tag.chunk$(printf '%04d' "$chunk").json"
        receipt="$OUT/$name"
        jq --argjson first "$first" --argjson size "$chunk_size" '.[$first:$first+$size]' \
          "$base.selected.json" > "$receipt.selected"
        cached_chunk=${REPLAY_CHUNK_CACHE:-}/$name
        if [[ -n ${REPLAY_CHUNK_CACHE:-} && -s $cached_chunk.exit ]] &&
            [[ $(cat "$cached_chunk.exit") =~ ^[01]$ ]]; then
          check_chunk "$cached_chunk" "$receipt.selected" || { echo "$dir: invalid cached native chunk"; exit 1; }
          for suffix in '' .exit .stderr; do cp "$cached_chunk$suffix" "$receipt$suffix"; done
        else
          args=(--replay 0)
          for seq in "${seqs[@]:first:chunk_size}"; do args+=(--replay-seq "$seq"); done
          audit "$receipt" "${args[@]}"
          check_chunk "$receipt" "$receipt.selected"
        fi
        jq --arg file "$name" --arg sha256 "$(sha256sum "$receipt" | cut -d' ' -f1)" \
          --argjson exit_code "$(cat "$receipt.exit")" \
          '. + [{file:$file,sha256:$sha256,exit_code:$exit_code}]' "$base.chunks.json" > "$base.chunks.tmp"
        mv "$base.chunks.tmp" "$base.chunks.json"
        echo "$dir: partition $part/$parts completed native chunk $chunk (${#seqs[@]} selected records)"
      done
      chunks_hash=$(sha256sum "$base.chunks.json" | cut -d' ' -f1)
    elif ((${#seqs[@]})); then
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
    --arg plan_sha256 "$plan_hash" --arg roster_sha256 "$roster_hash" --arg mode "$mode" \
    --arg cache_binding_sha256 "$cache_binding" --argjson index "$i" \
    --arg chunks_sha256 "${chunks_hash:-}" \
    --argjson partition "$part" --argjson partitions "$parts" \
    '{schema:$schema,dir:$dir,binary:$binary,plan_sha256:$plan_sha256,
      roster_sha256:$roster_sha256,mode:$mode,index:$index,
      partition:$partition,partitions:$partitions} +
      (if $mode == "cached-explicit" then {cache_binding_sha256:$cache_binding_sha256} else {} end) +
      (if $mode == "chunked" then {chunks_sha256:$chunks_sha256} else {} end)' > "$base.partition.json"
  echo "$dir: partition $part/$parts $mode ($(jq length "$base.selected.json") records)"
done
