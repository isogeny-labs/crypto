#!/bin/bash
# B-sweep on the a=-3 P-256-class rungs: factor-base size x LA engine.
# Anchored S3 sweep oracle, seed 20261005, +8 extra relations.
BIN=/Users/adamburan/ic-work/crypto/target/release/examples/a3_s4_ladder
D=research/prime_ic_la_scaling_20261118
: > "$D/sweep.jsonl"
run() { # bits fb la
  echo "[$(date -u +%FT%TZ)] run $*" >> "$D/progress.txt"
  "$BIN" "$1" 20261005 p256 s3 "$2" "$3" | tail -1 >> "$D/sweep.jsonl"
  echo "[$(date -u +%FT%TZ)] done $*" >> "$D/progress.txt"
}
for la in dense sparse; do run 16 128 $la; run 16 256 $la; run 16 512 $la; done
for la in dense sparse; do run 24 256 $la; done
run 24 512 sparse
run 24 512 dense
run 28 256 sparse
run 28 512 sparse
run 28 1024 sparse
