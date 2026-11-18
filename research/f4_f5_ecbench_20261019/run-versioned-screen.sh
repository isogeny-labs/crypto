#!/usr/bin/env bash
set -euo pipefail
out=${1:?new screen output directory}
ecb=${2:?frozen ecbench binary}
bench=${3:?frozen benchmark binary}
previous=${4:?source71 benchmark binary}
study=research/f4_f5_ecbench_20261019
test ! -e "$out"
mkdir -p "$out"
sha256sum "$ecb" "$bench" "$previous" > "$out/binaries.sha256"
cp "$study/ADAPTIVE_PROTOCOL.md" "$study"/versioned-screen-n*.json "$study/versioned-screen-specs.sha256" "$out/"
for n in 17 23; do
 "$ecb" run --spec "$study/versioned-screen-n$n.json" --out "$out/session-n$n" --cpus none --allow-busy --lock "$out/local-only.lock" --quiet > "$out/run-n$n.log" 2>&1
 "$ecb" verify --dir "$out/session-n$n" --replay-all --out "$out/audit-n$n.json" --exit-code > "$out/audit-n$n.log" 2>&1
done
for n in 17 23; do
 for arm in solve-f4 solve-f5; do
  for side in before after; do
   binary=$previous
   policy=quadratic-tail
   if [ "$side" = after ]; then binary=$bench; policy=adaptive-quadratic-tail; fi
   env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 KIC_GF2_SERIAL_CLEAR=1 "$binary" --matrix-runtime "$policy" --arm "$arm" --n "$n" --seed 230 --random 0 > "$out/$arm-n$n-$side.jsonl"
  done
 done
done
printf '%s\n' complete > "$out/screen-phase.status"
