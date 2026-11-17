#!/usr/bin/env bash
set -euo pipefail
bundle=$(cd "$(dirname "$0")" && pwd)
out=${EPHEMERAL_OUT:?output directory required}
mkdir -p "$out"
cd "$bundle"
sha256sum -c binaries.sha256 > "$out/binary-check.log"
cp build.json protocol.json "$out/"
cat /proc/cpuinfo > "$out/cpuinfo.txt"
cat /proc/self/status > "$out/process-status.txt"
uname -a > "$out/kernel.txt"
for name in cpu.max cpuset.cpus.effective memory.max; do
    if [[ -r /sys/fs/cgroup/$name ]]; then cat "/sys/fs/cgroup/$name" > "$out/$name"; fi
done
export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1
printf '%s\n' 'arm,instance_id,seed,replicate,warmup,verification,scalar,attempts,relations,curve_ms,bases_ms,oracle_ms,reduction_other_ms,replay_ms,complete_ms,oracle_calls,preparations,built_rows,probed_rows,query_inversions,table_bytes,digest,wall_seconds,tier' > "$out/rows.csv"
for round in {0..9}; do
    for instance in {0..2}; do
        arms=(u v w x)
        for slot in {0..3}; do
            index=$(((slot+round+instance)%4))
            if (( round % 2 )); then index=$((3-index)); fi
            arm=${arms[index]}
            printf 'starting %s %s %s\n' "$arm" "$instance" "$round" >> "$out/progress.txt"
            timeout 60 ./complete "$arm" "$instance" "$round" "$out" >> "$out/rows.csv" 2>> "$out/errors.txt"
        done
    done
done
./analyze "$out/rows.csv" "$out" > "$out/analysis.log"
echo 'PASS complete CPU recoveries and identical certificates; clocks are L0 diagnostics'
