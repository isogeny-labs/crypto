#!/bin/bash
# Check complete native receipt coverage before comparing observed drift to the plan.
# Usage: check_replay_partitions.sh ecbench receipt-root partition-count
set -euo pipefail
EB=${1:?native ecbench}; ROOT=${2:?receipt root}; parts=${3:?partition count}
[[ $parts =~ ^[1-9][0-9]*$ ]] || exit 2
here=$(cd "$(dirname "$0")" && pwd)
plan=${REPLAY_PLAN:-docs/ic/validation/replay-plan.json}
jq -e '.schema == "ic-replay-plan/v1" and
  (.sessions | type == "array" and length > 0) and
  ([.sessions[].dir] | length == (unique | length)) and
  all(.sessions[]; .replay == "all" and (.expect == "identical" or .expect == "drift") and
    (.dir | type == "string" and test("^[A-Za-z0-9_./-]+$") and (startswith("/") | not) and
      (split("/") | index("..") | not)))' "$plan" >/dev/null
binary=$(sha256sum "$EB" | cut -d' ' -f1)
roster=$(jq -c '[.sessions[]|{dir,replay}]' "$plan" | sha256sum | cut -d' ' -f1)
mkdir -p "$ROOT/checked"
bad=0
for ((i=0; i<$(jq '.sessions|length' "$plan"); i++)); do
  tag=$(printf '%04d' "$i"); dir=$(jq -r ".sessions[$i].dir" "$plan")
  candidates="$ROOT/0/$tag.candidates.json"
  receipts=(); probes=(); cached=0
  for ((p=0; p<parts; p++)); do
    base="$ROOT/$p/$tag"
    jq -e --arg dir "$dir" --arg binary "$binary" --arg roster "$roster" \
      --argjson i "$i" --argjson p "$p" --argjson n "$parts" '
      .schema == "ic-replay-partition/v1" and .dir == $dir and .binary == $binary and
      .roster_sha256 == $roster and .index == $i and .partition == $p and .partitions == $n and
      (.plan_sha256|test("^[0-9a-f]{64}$")) and
      (.mode == "explicit" or .mode == "cached-explicit" or .mode == "cached-full" or .mode == "empty")' "$base.partition.json" >/dev/null
    cmp "$candidates" "$base.candidates.json"
    jq -e --argjson p "$p" --argjson n "$parts" --slurpfile selected "$base.selected.json" \
      '[to_entries[] | select(.key % $n == $p) | .value] == $selected[0]' "$candidates" >/dev/null
    mode=$(jq -r .mode "$base.partition.json")
    if [[ $mode == cached-explicit ]]; then
      [[ $(sha256sum "$base.cache-source.json" | cut -d' ' -f1) == "$(jq -r .cache_binding_sha256 "$base.partition.json")" ]]
    fi
    if [[ $mode == cached-full ]]; then
      cached=$((cached+1)); receipts+=("$base.receipt.json")
    else
      probes+=("$base.probe.json")
      jq -e -L "$here" --arg binary "$binary" --slurpfile candidates "$candidates" '
        include "replay_native"; valid_native($binary) and
        .replay_candidates == ($candidates[0]|length) and .replay_selection == "sample" and
        (.replays|length) == 1 and ([.replays[]|{seq,record_id}] - $candidates[0] | length) == 0' \
        "$base.probe.json" >/dev/null
      if [[ $mode == empty ]]; then
        jq -e 'length==0' "$base.selected.json" >/dev/null
        [[ ! -e $base.receipt.json ]]
      else
        jq -e -L "$here" --arg binary "$binary" --slurpfile selected "$base.selected.json" '
          include "replay_native"; valid_native($binary) and .replay_selection == "explicit" and
          (.replays|replay_keys) == $selected[0]' "$base.receipt.json" >/dev/null
        receipts+=("$base.receipt.json")
      fi
    fi
  done
  if ((cached)); then
    ((cached == parts)) || { echo "$dir: mixed cached/full partitions"; exit 1; }
    for receipt in "${receipts[@]}"; do cmp "${receipts[0]}" "$receipt"; done
    jq -e -L "$here" --arg binary "$binary" --slurpfile candidates "$candidates" '
      include "replay_native"; valid_native($binary) and .replay_selection == "all" and
      .replay_candidates == ($candidates[0]|length) and (.replays|replay_keys) == $candidates[0]' \
      "${receipts[0]}" >/dev/null
    receipts=("${receipts[0]}")
  fi
  # Every native static audit must bind the same session and exact file map.
  jq -se '.[0] as $first | all(.[]; .session_id == $first.session_id and .files == $first.files)' \
    "${receipts[@]}" "${probes[@]}" >/dev/null
  while IFS=$'\t' read -r name expected; do
    actual=$(sha256sum "$dir/$name" | cut -d' ' -f1)
    [[ $actual == "$expected" ]] || { echo "$dir/$name: input changed"; exit 1; }
  done < <(jq -r '.files|to_entries[]|[.key,.value]|@tsv' "${receipts[0]}")
  # This derived summary is explicitly not a native ecbench.audit/v1 receipt.
  jq -s --arg dir "$dir" --arg binary "$binary" --slurpfile candidates "$candidates" '
    {schema:"ic-replay-union/v1",dir:$dir,binary:$binary,files:.[0].files,session_id:.[0].session_id,
     replays:([.[].replays[]]|sort_by(.seq))} |
    if ([.replays[]|{seq,record_id}] == $candidates[0] and
       ([.replays[].seq]|length==(unique|length))) then .
    else error("missing, duplicate or unexpected native replay") end' \
    "${receipts[@]}" > "$ROOT/checked/$tag.union.json"
  total=$(jq '.replays|length' "$ROOT/checked/$tag.union.json")
  ok=$(jq '[.replays[]|select(.reproduced)]|length' "$ROOT/checked/$tag.union.json")
  got=drift; [[ $ok != "$total" ]] || got=identical
  expected=$(jq -r ".sessions[$i].expect" "$plan")
  if [[ $got == "$expected" ]]; then
    echo "$dir: $got ($ok/$total), complete native coverage as planned"
  else
    echo "$dir: expected $expected, observed $got ($ok/$total); plan requires evidence-backed correction"
    bad=1
  fi
done
exit "$bad"
