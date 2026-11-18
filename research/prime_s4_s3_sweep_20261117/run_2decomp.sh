#!/bin/bash
# Paired 2-decomposition (S3, m=2) runs on the same host and fixtures, for the
# "3-decomposition vs 2-decomposition per-relation wall" comparison.
BIN=/Users/adamburan/ic-work/crypto/target/release/examples/a3_stage_ladder
OUT=research/prime_s4_s3_sweep_20261117/two_decomp.jsonl
: > "$OUT"
run() {
  echo "[$(date -u +%FT%TZ)] run 2decomp $*" >> research/prime_s4_s3_sweep_20261117/progress.txt
  "$BIN" "$1" "$2" "$3" | tail -1 >> "$OUT"
  echo "[$(date -u +%FT%TZ)] done 2decomp $* exit=$?" >> research/prime_s4_s3_sweep_20261117/progress.txt
}
run 16 20261005 p256
run 20 20261005 p256
run 24 20261005 p256
run 24 20261006 cryptopro
