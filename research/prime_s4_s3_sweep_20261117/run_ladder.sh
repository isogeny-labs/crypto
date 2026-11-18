#!/bin/bash
# Paired ladder: quartic/CZ oracle vs S3-anchored sweep, a=-3 deployed shape.
BIN=/Users/adamburan/ic-work/crypto/target/release/examples/a3_s4_ladder
OUT=research/prime_s4_s3_sweep_20261117/ladder.jsonl
: > "$OUT"
run() { # bits seed class oracle
  echo "[$(date -u +%FT%TZ)] run $*" >> research/prime_s4_s3_sweep_20261117/progress.txt
  "$BIN" "$1" "$2" "$3" "$4" | tail -1 >> "$OUT"
  echo "[$(date -u +%FT%TZ)] done $* exit=$?" >> research/prime_s4_s3_sweep_20261117/progress.txt
}
for s in 20261005 20261006 20261007; do
  run 20 $s p256 s3
  run 20 $s cryptopro s3
  run 24 $s p256 s3
  run 24 $s cryptopro s3
done
for s in 20261005 20261006 20261007; do
  run 28 $s p256 s3
  run 28 $s cryptopro s3
done
# Paired quartic run at 20 bits (one seed: it is minutes per rung at best)
run 20 20261005 p256 quartic
run 16 20261006 p256 quartic
run 16 20261006 p256 s3
