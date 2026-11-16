#!/usr/bin/env bash
# E29 validation replay (independent validator, 2026-10-31): the recorded runs
# of experiments/34_twist24_stage_r/ with a freshly built binary of the
# recorded commit 0e4fccce1 (experiments/34_twist24_stage_r/binary_commit.txt).
# Run from the repository root:
#
#   BIN=<e29_twist built from 0e4fccce1> research/e29_twists_20261030/replay/replay.sh
#
# The commands of examples/e29_twist.rs's header, as the committed records
# split them (kill test; Stage R on p = 241, then the other seven; Stage A one
# size per run), unchanged except the binary and the output directory.
# RAYON_NUM_THREADS=4 is the producer host's default (4 vCPU).  Restart-safe.
# Operation counts only.
set -u
out=research/e29_twists_20261030/replay
mkdir -p "$out"
export RAYON_NUM_THREADS=4
run() { # name, args...
  local name=$1; shift
  [ -s "$out/$name.json" ] && return 0
  echo "$name start $(date -u +%FT%TZ)" >>"$out/replay.log"
  "$BIN" "$@" --json "$out/$name.json.part" >"$out/$name.log" 2>&1 && mv "$out/$name.json.part" "$out/$name.json"
  echo "$name end $(date -u +%FT%TZ)" >>"$out/replay.log"
}
run kill --stage k --sizes 241,337,541,601,733,769,997,1069
run stage_r_p241 --stage r --sizes 241
run stage_r_rest --stage r --sizes 337,541,601,733,769,997,1069
run stage_a_p241 --stage a --sizes 241
run stage_a_p337 --stage a --sizes 337
run stage_a_p541 --stage a --sizes 541
echo "pass done $(date -u +%FT%TZ)" >>"$out/replay.log"
