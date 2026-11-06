#!/usr/bin/env bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
out=${EPHEMERAL_OUT:?output directory required}
cd "$root/gpu/explicit-relations"
mkdir -p "$out"
nvidia-smi -q > "$out/gpu-before.txt"
cat /proc/cpuinfo > "$out/cpuinfo.txt"
cat /proc/self/status > "$out/process-status.txt"
git rev-parse HEAD > "$out/source-commit.txt"
nvcc --version > "$out/nvcc.txt"
for arm in u v w; do
    flags=()
    if [[ "$arm" == u ]]; then flags=(-DER_NORMALIZE_SCAN_REFERENCE); fi
    if [[ "$arm" == w ]]; then flags=(-DER_NORMALIZE_CUB_REFERENCE); fi
    nvcc -O3 -std=c++17 -arch=sm_120 "${flags[@]}" benchmark.cu -o "$out/bench-$arm"
    "$out/bench-$arm" --check > "$out/check-$arm.log"
done
bash validate-rtx.sh
timeout 120 compute-sanitizer --tool racecheck --error-exitcode 86 "$out/bench-v" --check > "$out/racecheck.log" 2>&1
timeout 120 compute-sanitizer --tool synccheck --error-exitcode 86 "$out/bench-v" --check > "$out/synccheck.log" 2>&1
sha256sum "$out"/bench-* build/explicit-relations-cuda > "$out/benchmark-binaries.sha256"
printf '%s\n' 'arm,instance_id,seed,replicate,warmup,verification,normalization_muls,normalize_ms,prepare_ms,query_ms,cold_ms,digest,norm_digest,wall_seconds,tier' > "$out/rows.csv"
# Latin rotation and reversal interleave the same four arms in every round.
# x is a second invocation of the exact u binary (A/A).
for round in {0..9}; do
    for instance in {0..2}; do
        arms=(u v w x)
        for slot in {0..3}; do
            index=$(((slot + round + instance) % 4))
            if (( round % 2 )); then index=$((3-index)); fi
            arm=${arms[index]}
            executable=$arm
            if [[ "$arm" == x ]]; then executable=u; fi
            timeout 60 "$out/bench-$executable" "$arm" "$instance" "$round" >> "$out/rows.csv"
        done
    done
done
rustc -O "$root/research/explicit_relations_perf_20261030/analyze.rs" -o "$out/analyze"
"$out/analyze" "$out/rows.csv" "$out" > "$out/analysis.log"
nvidia-smi -q > "$out/gpu-after.txt"
echo 'PASS bounded matched performance panel; timings are L0 diagnostics'
