#!/bin/sh
# Replay every measured deterministic run of the 8 prime_ksum_pooled_20261029 sessions.
# Sessions: copies of research/prime_ksum_pooled_20261029/sessions/<s>/{session,spec,plan,host}.json
# at 6edb2cb36d plus records.jsonl from git blob a0ff77014f (sha256 = .artifacts.json = session.json).
# Binary: ecbench built from a clean `git archive 6edb2cb36d` (not the recorded macOS ecbench-v2).
set -u
K=/tmp/claude-0/-home-user/89d98138-21ce-5c29-8de4-5fe8ac9e919c/scratchpad/ksum
E=$K/target/release/ecbench
mkdir -p $K/replay
one() {
  s=$1
  start=$(date -u +%s)
  RAYON_NUM_THREADS=1 nice "$E" verify --dir "$K/sessions/$s" --replay-all --exit-code \
    --out "$K/replay/replay-$s.audit.json" > "$K/replay/replay-$s.verify.log" 2>&1
  rc=$?
  echo "end rc=$rc elapsed_s=$(( $(date -u +%s) - start )) $(date -u +%FT%TZ)" >> "$K/replay/replay-$s.verify.log"
}
# three at a time (4 logical CPUs; another agent shares the host)
for group in "train-P20 holdout-P20 train-P24" "holdout-P24 train-P28 holdout-P28" "holdout-P32 train-P32"; do
  for s in $group; do one "$s" & done
  wait
done
echo done > $K/replay/ALL_DONE
