#!/usr/bin/env bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
out=${EPHEMERAL_OUT:?output directory required}
study="$root/research/explicit_relations_e2e_20261030"
mkdir -p "$out"
nvidia-smi -q > "$out/gpu-before.txt"
cat /proc/cpuinfo > "$out/cpuinfo.txt"
cat /proc/self/status > "$out/process-status.txt"
git rev-parse HEAD > "$out/source-commit.txt"
nvcc --version > "$out/nvcc.txt"
bash "$root/gpu/explicit-relations/validate-rtx.sh"
cd "$root/gpu/explicit-relations"
timeout 120 compute-sanitizer --tool racecheck --error-exitcode 86 build/query-check > "$out/racecheck.log" 2>&1
timeout 120 compute-sanitizer --tool synccheck --error-exitcode 86 build/query-check > "$out/synccheck.log" 2>&1
c++ -O3 -std=c++17 "$study/target.cpp" -o "$out/target"
rustc --edition=2021 -D warnings -O "$study/ladder.rs" -o "$out/ladder"
rustc --edition=2021 -D warnings -O "$study/analyze.rs" -o "$out/analyze"
jq -r '.curves[] | [.name[-2:],.p,.a,.b,.group_order,.generator.x,.generator.y] | join(" ")' "$study/curves.json" > "$out/curves.txt"
timeout 180 gp -q "$study/extend-curves.gp" > "$out/extended-curves.txt" 2> "$out/curve-generation.log"
test "$(wc -l < "$out/extended-curves.txt")" = 4
cat "$out/extended-curves.txt" >> "$out/curves.txt"
sha256sum "$out/target" "$out/ladder" "$out/analyze" build/paper-driver build/explicit-relations-cuda build/explicit-relations-host > "$out/instruments.sha256"
"$out/ladder" "$out" "$PWD/build/paper-driver" "$PWD/build/explicit-relations-cuda" "$PWD/build/explicit-relations-host" "$out/target" | tee "$out/ladder.log"
"$out/analyze" "$out/rows.csv" "$out" > "$out/analysis.log"
nvidia-smi -q > "$out/gpu-after.txt"
echo 'PASS complete bounded recovery ladder; timings remain L0 diagnostics'
