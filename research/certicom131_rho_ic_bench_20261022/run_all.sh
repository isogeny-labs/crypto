#!/usr/bin/env bash
# Frozen command list for the Certicom-131 rho / index-calculus benchmark.
# Every timed command runs on reserved CPU 3 through tools/isolated_bench.py
# (AGENTS.md section 10).  Re-runnable only into a new directory: raw/ is write-once.
set -u
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
HERE=$ROOT/research/certicom131_rho_ic_bench_20261022
RAW=$HERE/raw
ISO="python3 $ROOT/tools/isolated_bench.py run --cpus 3 --settle 2"
IC=$ROOT/target/release/ic
NIST=$ROOT/target/release/ecdlp-nist
K130=$ROOT/ecc2k130/ecc2k130-cpu
B131=$HERE/bench131
mkdir -p "$RAW/iso"
run() {  # name, command...
  local name=$1; shift
  $ISO --out "$RAW/iso/$name.jsonl" -- "$@" > "$RAW/$name.stdout" 2> "$RAW/$name.stderr"
  echo "$name exit=$?" >> "$RAW/exit_codes.txt"
}

# 0. exact expected work from the group orders (counted, no timing)
for c in ECCp-131 ECC2K-130 ECC2-131; do
  $NIST audit $c --json > "$RAW/audit_$c.json"
done

# 1. rho step rate: generic Rust arithmetic, 5 repeats
for r in 1 2 3 4 5; do
  for c in eccp-131 ecc2k-130 ecc2-131; do
    run "nist_bench_${c}_r$r" $NIST bench $c --json
  done
done

# 2. rho step rate: production ECC2K-130 CPU client (bitsliced, Frobenius orbit), 1 thread, 5 repeats
for r in 1 2 3 4 5; do
  run "k130cpu_1t_r$r" $K130 --curve 131 --bench --steps 32 --launches 24 --verify 0 --threads 1
done

# 3. rho step: native certicom131 arithmetic, batched-affine negation-map walk, 3 arms
run bench131_prime  $B131 --curve p --lanes 128 --steps 2000 --rounds 7
run bench131_binary_header $B131 --curve b --lanes 128 --steps 20 --rounds 5
run bench131_binary_clmul  $B131 --curve c --lanes 128 --steps 2000 --rounds 7

# 4. ECCp-131 exact-curve relation probe, interleaved rounds, seed 131, plus an A/A copy of B512
for r in 1 2 3 4 5; do
  run "ic_eccp131_a2_b256_r$r"  $IC prime --curve eccp-131 --size 256  --trials 0 --controls 8 --setup-candidates 32 --target-candidates 16 --seed 131 --relation-arity 2 --json
  run "ic_eccp131_a2_b512_r$r"  $IC prime --curve eccp-131 --size 512  --trials 0 --controls 8 --setup-candidates 32 --target-candidates 16 --seed 131 --relation-arity 2 --json
  run "ic_eccp131_a2_b512aa_r$r" $IC prime --curve eccp-131 --size 512 --trials 0 --controls 8 --setup-candidates 32 --target-candidates 16 --seed 131 --relation-arity 2 --json
  run "ic_eccp131_a2_b1024_r$r" $IC prime --curve eccp-131 --size 1024 --trials 0 --controls 8 --setup-candidates 32 --target-candidates 16 --seed 131 --relation-arity 2 --json
  run "ic_eccp131_a3_b128_r$r"  $IC prime --curve eccp-131 --size 128  --trials 0 --controls 8 --setup-candidates 32 --target-candidates 16 --seed 131 --relation-arity 3 --json
done
echo done >> "$RAW/exit_codes.txt"
