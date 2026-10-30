#!/usr/bin/env bash
set -euo pipefail
out=${1:?output} before=${2:?first-tail binary} after=${3:?refined binary}
mkdir -p "$out"
sha256sum "$before" "$after" > "$out/binaries.sha256"
valgrind --version > "$out/valgrind.txt"
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/started.txt"
for round in 0 1 2 3 4; do
  for arm in solve-f4 solve-f5; do
    for side in a b; do
      binary=$before
      if [ "$side" = b ]; then binary=$after; fi
      folder="$out/$arm-$round-$side"
      mkdir -p "$folder"
      cmd=(env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 KIC_GF2_SERIAL_CLEAR=1
        timeout 300 valgrind --tool=callgrind --instr-atstart=no --separate-threads=no --error-exitcode=97
        "--callgrind-out-file=$folder/region.callgrind" "$binary" --profile 1 --arm "$arm" --n 23 --seed 230 --random 0 --matrix-runtime quadratic-tail)
      printf '%q ' "${cmd[@]}" > "$folder/command.txt"
      printf '\n' >> "$folder/command.txt"
      set +e
      "${cmd[@]}" > "$folder/run.jsonl" 2> "$folder/stderr.txt"
      code=$?
      set -e
      printf '%s\n' "$code" > "$folder/exit.txt"
      if [ "$code" != 0 ]; then exit "$code"; fi
    done
  done
done
printf '%s\n' complete > "$out/lookup-profile-phase.status"
