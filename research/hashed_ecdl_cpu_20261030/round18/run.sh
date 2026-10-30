#!/usr/bin/env bash
set -euo pipefail
bundle=$(cd "$(dirname "$0")" && pwd)
out=${EPHEMERAL_OUT:?output directory required}
mkdir -p "$out"
cd "$bundle"
sha256sum -c binaries.sha256 > "$out/binary-check.log"
cp build.json protocol.json candidate-materialization.json previous-materialization.json inverse-materialization.json "$out/"
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
        arms=(u v w x y z)
        for slot in {0..5}; do
            index=$(((slot+round+instance)%6))
            if (( round % 2 )); then index=$((5-index)); fi
            arm=${arms[index]}
            printf 'starting %s %s %s\n' "$arm" "$instance" "$round" >> "$out/progress.txt"
            case "$arm" in
                u|x) binary=baseline; actual=v ;;
                v) binary=candidate; actual=v ;;
                w) binary=candidate; actual=w ;;
                y) binary=previous; actual=v ;;
                z) binary=inverse; actual=v ;;
            esac
            cell="$out/cell-$arm-$instance-$round"
            mkdir "$cell"
            timeout 60 "./$binary" "$actual" "$instance" "$round" "$cell" > "$cell/row.csv" 2>> "$out/errors.txt"
            sed "s/^$actual,/$arm,/" "$cell/row.csv" >> "$out/rows.csv"
            mv "$cell/cert-$actual-$instance-$round.txt" "$out/cert-$arm-$instance-$round.txt"
        done
    done
done
./analyze "$out/rows.csv" "$out" > "$out/analysis.log"
echo 'PASS complete CPU recoveries and identical certificates; clocks are L0 diagnostics'
