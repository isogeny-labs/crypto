#!/usr/bin/env bash
set -euo pipefail
bundle=$(cd "$(dirname "$0")" && pwd)
out=${EPHEMERAL_OUT:?}
mkdir -p "$out"
cd "$bundle"
sha256sum -c binaries.sha256 > "$out/binary-check.log"
cp build.json protocol.json "$out/"
cat /proc/cpuinfo > "$out/cpuinfo.txt"
cat /proc/self/status > "$out/process-status.txt"
uname -a > "$out/kernel.txt"
export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1
printf '%s\n' 'arm,instance_id,seed,replicate,warmup,verification,bits,tau,scalar,relations,attempts,wall_seconds,bases_seconds,targets_seconds,oracle_seconds,linear_seconds,replay_seconds,second_replay_seconds,preparations,built_rows,probed_rows,query_inversions,table_bytes,tier' > "$out/rows.csv"
printf '%s\n' 'arm,case,seed,exit_code,replay_exit_code' > "$out/status.csv"
for instance in {0..5}; do
    passed=true
    for seed in 101 202; do
        for arm in u x; do
        id="$arm-$instance-$seed"
        echo "START $id"
        code=0; replay=125
        timeout 120 ./scale "$arm" "$instance" "$seed" "$out" > "$out/$id.row" 2> "$out/$id.log" || code=$?
        if (( code == 0 )); then
            replay=0
            timeout 60 ./hashed-ecdl-cpu --verify "$out/scale-$id.json" > "$out/$id.verify" 2>&1 || replay=$?
            cat "$out/$id.row" >> "$out/rows.csv"
        fi
        printf '%s,%s,%s,%s,%s\n' "$arm" "$instance" "$seed" "$code" "$replay" >> "$out/status.csv"
        echo "END $id exit=$code replay=$replay"
        if (( code != 0 || replay != 0 )); then passed=false; fi
        done
    done
    if [[ "$passed" == false ]]; then break; fi
done
./analyze "$out/rows.csv" "$out" > "$out/analysis.log"
