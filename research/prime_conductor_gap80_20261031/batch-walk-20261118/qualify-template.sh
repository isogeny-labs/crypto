#!/bin/sh
# Small development checks only. No timing or target construction.
set -eu
candidate=$1
baseline=$2
study=research/prime_conductor_gap80_20261031
out=${3:?supply a fresh output directory for a new replay}
test ! -e "$out"
mkdir -p "$out"
inputs="$study/evaluator-20261118/inputs"
"$candidate" --fixtures > "$out/default-fixtures.jsonl" 2> "$out/default-fixtures.stderr"
cmp "$out/default-fixtures.jsonl" "$study/window-audit-20261118/results/default-fixtures.jsonl"
"$candidate" --projective-controls > "$out/projective-controls.json" 2> "$out/projective-controls.stderr"
run_case() {
  label=$1; input=$2; window=$3; block=$4; shift 4
  "$baseline" --evaluate "$window" "$block" --pari-reference "$@" < "$inputs/$input" > "$out/$label-baseline.json" 2> "$out/$label-baseline.stderr"
  "$candidate" --evaluate "$window" "$block" --pari-reference "$@" < "$inputs/$input" > "$out/$label-affine.json" 2> "$out/$label-affine.stderr"
  cmp "$out/$label-baseline.json" "$out/$label-affine.json"
  for batch in 1 7 64 4096; do
    "$candidate" --evaluate "$window" "$block" --pari-reference "$@" --projective-batch "$batch" < "$inputs/$input" > "$out/$label-batch$batch.json" 2> "$out/$label-batch$batch.stderr"
    jq -e -s '.[0] == (.[1] | del(.walk_backend))' "$out/$label-baseline.json" "$out/$label-batch$batch.json" > "$out/$label-batch$batch.check"
  done
}
run_case blocked17 control10009.txt 64 17
run_case full64 control10009.txt 64 64
run_case explicit18 control10009.txt 200 200 --baby-window 18
run_case small1 control19.txt 4 1
run_case small-explicit control19.txt 4 4 --baby-window 1
run_case empty-leftover control5.txt 2 2
negative() {
  label=$1; shift
  set +e
  "$candidate" --evaluate 4 1 "$@" < "$inputs/control19.txt" > "$out/negative-$label.stdout" 2> "$out/negative-$label.stderr"
  code=$?
  set -e
  test "$code" -eq 2
  test ! -s "$out/negative-$label.stdout"
  printf '{"case":"%s","exit":%s,"stdout_empty":true}\n' "$label" "$code" >> "$out/rejections.jsonl"
}
test ! -e "$out/rejections.jsonl"
negative zero --projective-batch 0
negative too-large --projective-batch 65537
negative duplicate --projective-batch 1 --projective-batch 2
negative missing --projective-batch
negative invalid --projective-batch nope
