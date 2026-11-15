#!/usr/bin/env bash
set -eu
cd /workspace/crypto
study=research/ideas_execution_20261018/learned_pivot
base=research/ideas_execution_20261018/bounded_memory
run="$study/runs/frozen-v1"
test ! -e "$run"
mkdir -p "$run/inputs"
date -u +%Y-%m-%dT%H:%M:%SZ > "$run/start.txt"
sha256sum "$study/protocol.json" "$study/src/selector.cpp" "$study/run.sh" "$base/src/disk_macaulay.cpp" "$base/src/resource_probe.cpp" > "$run/sources.sha256"
g++ -O2 -std=c++17 -Wall -Wextra -Wpedantic "$study/src/selector.cpp" -lcrypto -lz -o "$run/selector" 2> "$run/build-selector.stderr"
g++ -O2 -std=c++17 "$base/src/disk_macaulay.cpp" -lcrypto -o "$run/reducer" 2> "$run/build-reducer.stderr"
g++ -O2 -std=c++17 "$base/src/resource_probe.cpp" -o "$run/probe" 2> "$run/build-probe.stderr"
sha256sum "$run/selector" "$run/reducer" "$run/probe" > "$run/binaries.sha256"
g++ --version > "$run/compiler.txt"
uname -a > "$run/host.txt"
cat /sys/fs/cgroup/memory.max /sys/fs/cgroup/cpu.max > "$run/cgroup.txt"
for stem in n5-m4-ell2 n9-m4-ell3 n15-m4-ell5 n17-m4-ell6; do gzip -dc "$base/inputs/$stem.rows.gz" > "$run/inputs/$stem.rows"; done
sha256sum "$run"/inputs/*.rows > "$run/inputs.sha256"
for stem in n5-m4-ell2 n9-m4-ell3; do printf '%s %s %s %s %s %s\n' "$stem" "$run/inputs/$stem.rows" "$base/runs/run-20261018-native-v2/$stem-first/result.json" "$base/runs/run-20261018-native-v2/$stem-first/trace.jsonl.gz" "$base/runs/run-20261018-native-v2/$stem-min-weight/result.json" "$base/runs/run-20261018-native-v2/$stem-min-weight/trace.jsonl.gz" >> "$run/train.tsv"; done
"$run/probe" "$run/train.resources.json" "$run/selector" train "$run/train.tsv" > "$run/model.json" 2> "$run/train.stderr"
sha256sum "$run/model.json" > "$run/model.sha256"
# All choices are fixed using input/model only before heldout outcomes are opened.
for stem in n5-m4-ell2 n9-m4-ell3 n15-m4-ell5 n17-m4-ell6; do "$run/probe" "$run/$stem-select.resources.json" "$run/selector" apply "$run/model.json" "$run/inputs/$stem.rows" > "$run/$stem-selection.json" 2> "$run/$stem-select.stderr"; done
printf 'case\texit\n' > "$run/attempts.tsv"
for stem in n5-m4-ell2 n9-m4-ell3 n15-m4-ell5; do
 for policy in first min-weight; do
  verify=5000
  if [ "$stem" = n15-m4-ell5 ]; then verify=0; fi
  printf '%q ' "$run/probe" "$run/$stem-$policy.resources.json" "$run/reducer" reduce "$run/inputs/$stem.rows" "$run/$stem-$policy" 1048576 "$policy" "$verify" 120 0 100 > "$run/$stem-$policy.command.txt"
  set +e
  "$run/probe" "$run/$stem-$policy.resources.json" "$run/reducer" reduce "$run/inputs/$stem.rows" "$run/$stem-$policy" 1048576 "$policy" "$verify" 120 0 100 > "$run/$stem-$policy.stdout" 2> "$run/$stem-$policy.stderr"
  code=$?
  set -e
  printf '%s-%s\t%s\n' "$stem" "$policy" "$code" >> "$run/attempts.tsv"
 done
done
"$run/selector" compare "$run/n15-m4-ell5-min-weight/result.json" "$base/runs/run-20261018-large-continuation-v1/n15-m4-ell5-first/result-resume.json" "$base/runs/run-20261018-large-continuation-v1/n15-m4-ell5-min-weight/result-resume.json" > "$run/heldout-n15-comparison.json"
date -u +%Y-%m-%dT%H:%M:%SZ > "$run/end.txt"
find "$run" -type f -printf '%s\n' | awk '{n+=$1}END{print n}' > "$run/total-disk-bytes.txt"
