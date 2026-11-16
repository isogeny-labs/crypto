#!/usr/bin/env bash
# E28 validation replay (independent validator, 2026-10-31): the recorded runs
# of experiments/33_tau_stage_a/ with freshly built binaries.  Run from the
# repository root:
#
#   BIN_789=<e28_tau built from 789dc2ff6> BIN_0E4=<e28_tau built from 0e4fccce1> \
#     research/e28_tau_20261030/replay/replay.sh
#
# Stage 0 and Stage A (with the registered control) ran on binary 789dc2ff6
# (experiments/33_tau_stage_a/binary_commit.txt): the Stage 0 command of
# examples/e28_tau.rs's header and run_stage_a.sh's per-curve command,
# unchanged except the binary and the output directory.  The supplementary
# control (--stage c) exists only from 0e4fccce1 (§9.30), so it is replayed
# with that build.  Restart-safe.  Operation counts only.
set -u
out=research/e28_tau_20261030/replay
mkdir -p "$out"
if [ ! -s "$out/stage0.json" ]; then
  echo "stage 0 start $(date -u +%FT%TZ)" >>"$out/replay.log"
  "$BIN_789" --stage 0 --sizes 53,101,149,197,269 --seeds 1-8 \
    --json "$out/stage0.json.part" >"$out/stage0.log" 2>&1 && mv "$out/stage0.json.part" "$out/stage0.json"
  echo "stage 0 end $(date -u +%FT%TZ)" >>"$out/replay.log"
fi
export RAYON_NUM_THREADS=4
for p in 53 101 149 197 269; do
  for s in 1 2; do
    f=$out/stage_a_p${p}s${s}.json
    [ -s "$f" ] && continue
    echo "stage A p=$p s=$s start $(date -u +%FT%TZ)" >>"$out/replay.log"
    "$BIN_789" --stage a --sizes "$p" --seeds "$s-$s" --control 3 \
      --rho-runs 16 --json "$f.part" >>"$out/stage_a.log" 2>&1 && mv "$f.part" "$f"
    echo "stage A p=$p s=$s end $(date -u +%FT%TZ)" >>"$out/replay.log"
  done
done
if [ -n "${BIN_0E4:-}" ] && [ ! -s "$out/control_decomposed.json" ]; then
  echo "control c start $(date -u +%FT%TZ)" >>"$out/replay.log"
  "$BIN_0E4" --stage c --sizes 53,101,149,197,269 --seeds 1-2 --control 3 \
    --json "$out/control_decomposed.json.part" >"$out/control_decomposed.log" 2>&1 \
    && mv "$out/control_decomposed.json.part" "$out/control_decomposed.json"
  echo "control c end $(date -u +%FT%TZ)" >>"$out/replay.log"
fi
echo "pass done $(date -u +%FT%TZ) (BIN_0E4=${BIN_0E4:-unset})" >>"$out/replay.log"
