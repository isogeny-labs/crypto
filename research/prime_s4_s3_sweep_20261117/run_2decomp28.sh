#!/bin/bash
BIN=/Users/adamburan/ic-work/crypto/target/release/examples/a3_stage_ladder
OUT=research/prime_s4_s3_sweep_20261117/two_decomp.jsonl
run() {
  echo "[$(date -u +%FT%TZ)] run 2decomp $*" >> research/prime_s4_s3_sweep_20261117/progress.txt
  "$BIN" "$1" "$2" "$3" | tail -1 >> "$OUT"
  echo "[$(date -u +%FT%TZ)] done 2decomp $* " >> research/prime_s4_s3_sweep_20261117/progress.txt
}
run 16 20261006 cryptopro
run 20 20261006 cryptopro
run 28 20261005 p256
run 28 20261006 p256
run 28 20261005 cryptopro
