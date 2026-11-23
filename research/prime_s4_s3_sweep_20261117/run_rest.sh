#!/bin/bash
BIN=/Users/adamburan/ic-work/crypto/target/release/examples/a3_s4_ladder
OUT=research/prime_s4_s3_sweep_20261117/ladder.jsonl
run() {
  echo "[$(date -u +%FT%TZ)] run $*" >> research/prime_s4_s3_sweep_20261117/progress.txt
  "$BIN" "$1" "$2" "$3" "$4" | tail -1 >> "$OUT"
  echo "[$(date -u +%FT%TZ)] done $* " >> research/prime_s4_s3_sweep_20261117/progress.txt
}
run 16 20261006 p256 quartic
run 16 20261006 p256 s3
run 16 20261007 p256 s3
research/prime_s4_s3_sweep_20261117/run_2decomp.sh
