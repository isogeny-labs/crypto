#!/usr/bin/env bash
# Validator replay of E31 Stage A (GLV note §9.23; examples/e31_stage_a.rs's documented command).
# BIN is a fresh build of examples/e31_stage_a from a tree whose build inputs equal the recorded
# binary commit a8f52422d5.  One thread, as recorded.  Then an agreement sample of the
# validator's choosing on GLS curves outside E25's ten and outside the unit test's four.
#
#   BIN=/path/to/e31_stage_a benchmarks/20261024-e31-chain-solve/replay/replay_stage_a.sh
set -u
here="$(cd "$(dirname "$0")" && pwd)"
out="$here/stage_a"
mkdir -p "$out"
: "${BIN:?set BIN to the freshly built e31_stage_a}"
export RAYON_NUM_THREADS=1
[ -s "$out/stage_a_replay.json" ] || \
  "$BIN" --sizes 101,149,197,269,397 --seeds 1,2 --targets 1000 \
    --json "$out/stage_a_replay.json" 2>"$out/stage_a_replay.log"
echo "finished $(date -u +%FT%TZ)" >>"$out/finished.log"
