#!/usr/bin/env bash
# E30 validation replay (independent validator, 2026-10-31): the recorded runs
# of experiments/35_invariants_lite/ with freshly built binaries of the
# recorded commit 0e4fccce1 (experiments/35_invariants_lite/binary_commit.txt).
# Run from the repository root:
#
#   BIN_A=<e30_audit built from 0e4fccce1> BIN_P=<quartic_pipeline_gls built from 0e4fccce1> \
#     research/e30_invariants_lite_20261030/replay/replay.sh
#
# Walk audit: the command of examples/e30_audit.rs's header.  IC side:
# experiments/35_invariants_lite/run_e2e.sh's command, unchanged except the
# binary and the output directory (RAYON_NUM_THREADS=4 fixes the target
# stream, §9.19).  Restart-safe.  Operation counts only.
set -u
out=research/e30_invariants_lite_20261030/replay
mkdir -p "$out/e2e"
if [ ! -s "$out/walk_audit.json" ]; then
  echo "walk audit start $(date -u +%FT%TZ)" >>"$out/replay.log"
  "$BIN_A" --json "$out/walk_audit.json.part" >"$out/walk_audit.log" 2>&1 && mv "$out/walk_audit.json.part" "$out/walk_audit.json"
  echo "walk audit end $(date -u +%FT%TZ)" >>"$out/replay.log"
fi
export RAYON_NUM_THREADS=4
for p in 101 149 197 269 397; do
  for s in 1 2; do
    f=$out/e2e/e2e_audit_p${p}s${s}.json
    [ -s "$f" ] && continue
    echo "e2e p=$p s=$s start $(date -u +%FT%TZ)" >>"$out/replay.log"
    "$BIN_P" --sizes "$p" --first-seed "$s" --seeds 1 \
      --rho-runs 16 --max-residuals 5000000 --solver chain --karatsuba --audit --json "$f.part" \
      >>"$out/e2e/e2e_audit.log" 2>&1 && mv "$f.part" "$f"
    echo "e2e p=$p s=$s end $(date -u +%FT%TZ)" >>"$out/replay.log"
  done
done
echo "pass done $(date -u +%FT%TZ)" >>"$out/replay.log"
