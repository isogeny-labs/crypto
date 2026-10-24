#!/usr/bin/env bash
# Frozen command list for the Certicom-131 rho step-rate parity study (isolated, interleaved, one session).
# Every timed command runs on reserved CPU 3 through tools/isolated_bench.py (AGENTS.md section 10).
# raw/ is write-once: run into an empty directory.
set -u
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
HERE=$ROOT/research/certicom131_rho_parity_20261023
RAW=$HERE/raw
ISO="python3 $ROOT/tools/isolated_bench.py run --cpus 3 --settle 2"
K130=$ROOT/ecc2k130/ecc2k130-cpu
OLD=$ROOT/research/certicom131_rho_ic_bench_20261022/bench131
mkdir -p "$RAW/iso"
run() {  # name, command...
  local name=$1; shift
  $ISO --out "$RAW/iso/$name.jsonl" -- "$@" > "$RAW/$name.stdout" 2> "$RAW/$name.stderr"
  echo "$name exit=$?" >> "$RAW/exit_codes.txt"
}
# gated runs, once (each also prints its own timing rows)
run gate_walk131 "$HERE/walk131" --curve both --lanes 512 --steps 300 --rounds 3
run gate_bs131   "$HERE/bs131" --steps 40 --rounds 3
run gate_pv131   "$HERE/pv131" --steps 400 --rounds 3
# interleaved timing rounds; K appears first and last (A/A)
for r in 1 2 3 4 5 6 7; do
  run "k130_a_r$r"   $K130 --curve 131 --bench --steps 32 --launches 24 --verify 0 --threads 1
  run "old_prime_r$r" $OLD --curve p --lanes 128 --steps 500 --rounds 1
  run "old_clmul_r$r" $OLD --curve c --lanes 128 --steps 500 --rounds 1
  run "new_prime64_r$r" "$HERE/walk131" --curve p --lanes 512 --steps 1000 --rounds 1
  run "new_clmul3_r$r"  "$HERE/walk131" --curve b --lanes 512 --steps 1000 --rounds 1
  run "pv131_r$r"    "$HERE/pv131" --timing-only --slots 64 --steps 3000 --rounds 1
  run "bs131_r$r"    "$HERE/bs131" --timing-only --slots 16 --steps 400 --rounds 1
  run "k130_b_r$r"   $K130 --curve 131 --bench --steps 32 --launches 24 --verify 0 --threads 1
done
echo done >> "$RAW/exit_codes.txt"
