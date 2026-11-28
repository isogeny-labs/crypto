#!/usr/bin/env bash
set -euo pipefail
study=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
parent=$(dirname "$study")
build=${1:?qualified build directory}
out=${2:?new output directory}
mkdir "$out"
find "$parent/historical/fast_trace/results/fixtures" "$parent/historical/fast_trace/results_large/fixtures" -name '*.json' | sort > "$out/fixtures.txt"
sha256sum "$build"/benchmark-* "$build"/test-* > "$out/pre-call.sha256"
cp "$study/PROTOCOL.json" "$out/PROTOCOL.json"
lscpu > "$out/host.txt"
cat /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max > "$out/cgroup.txt"
call() {
  local round=$1 mode=$2 kind=$3 arm=$4 input=$5
  local exit_code=0
  taskset -c 0 "$build/benchmark-$mode" "$kind" "$arm" "$input" > "$out/current.json" 2>> "$out/stderr.txt" || exit_code=$?
  jq -c --argjson round "$round" --arg arm "$mode:$arm" --argjson exit "$exit_code" '{round:$round,arm:$arm,exit:$exit,row:.}' "$out/current.json" >> "$out/rows.jsonl"
  if ((exit_code)); then return "$exit_code"; fi
}
for round in -1 0 1 2 3 4; do
  date -u +%FT%TZ >> "$out/progress.txt"
  while IFS= read -r fixture; do
    for j in 0 1 2 3 4; do
      index=$(( (j+round+6)%5 ))
      case "$index" in
        0) call "$round" 0 translation dense "$fixture";;
        1) call "$round" 0 translation trace "$fixture";;
        2) call "$round" 1 translation trace "$fixture";;
        3) call "$round" 2 translation trace "$fixture";;
        4) call "$round" 3 translation trace "$fixture";;
      esac
    done
  done < "$out/fixtures.txt"
  for j in 0 1 2 3 4 5 6; do
    index=$(( (j+round+7)%7 ))
    case "$index" in
      0) call "$round" 0 root dense 100;;
      1) call "$round" 0 root trace 100;;
      2) call "$round" 0 root pullback 100;;
      3) call "$round" 0 root flint 100;;
      4) call "$round" 1 root trace 100;;
      5) call "$round" 2 root trace 100;;
      6) call "$round" 3 root trace 100;;
    esac
  done
  for mode in 0 1 2 3; do call "$round" "$mode" root trace-isogeny 1; done
  call "$round" 0 root dense-isogeny 1
done
sha256sum "$build"/benchmark-* "$build"/test-* > "$out/post-call.sha256"
cmp "$out/pre-call.sha256" "$out/post-call.sha256"
