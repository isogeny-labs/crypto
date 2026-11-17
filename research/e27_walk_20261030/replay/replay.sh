#!/usr/bin/env bash
# E27 validation replay (independent validator, 2026-10-30): the recorded runs
# of experiments/32_walk_karatsuba/ with freshly built binaries.  Run from the
# repository root:
#
#   BIN_W=<fresh e27_walk> BIN_P=<fresh quartic_pipeline_gls> \
#     research/e27_walk_20261030/replay/replay.sh
#
# Stage W: the command in examples/e27_walk.rs's header, output redirected.
# IC side: experiments/32_walk_karatsuba/run_e2e.sh's command, unchanged
# except the binary and the output directory (RAYON_NUM_THREADS=4 fixes the
# target stream, §9.19).  Restart-safe.  Operation counts only.
set -u
out=research/e27_walk_20261030/replay
mkdir -p "$out/e2e"
if [ ! -s "$out/stage_w.json" ]; then
  echo "stage W start $(date -u +%FT%TZ)" >>"$out/replay.log"
  "$BIN_W" --sizes 101,149,197,269,397 --seeds 1,2 --e25 experiments/30_gls_v4 \
    --json "$out/stage_w.json.part" >"$out/stage_w.log" 2>&1 && mv "$out/stage_w.json.part" "$out/stage_w.json"
  echo "stage W end $(date -u +%FT%TZ) exit=$?" >>"$out/replay.log"
fi
export RAYON_NUM_THREADS=4
for p in 101 149 197 269 397; do
  for s in 1 2; do
    f=$out/e2e/e2e_chain_karatsuba_p${p}s${s}.json
    [ -s "$f" ] && continue
    echo "e2e p=$p s=$s start $(date -u +%FT%TZ)" >>"$out/replay.log"
    "$BIN_P" --sizes "$p" --first-seed "$s" --seeds 1 \
      --rho-runs 16 --max-residuals 5000000 --solver chain --karatsuba --json "$f.part" \
      >>"$out/e2e/e2e_chain_karatsuba.log" 2>&1 && mv "$f.part" "$f"
    echo "e2e p=$p s=$s end $(date -u +%FT%TZ)" >>"$out/replay.log"
  done
done
echo "finished $(date -u +%FT%TZ)" >>"$out/replay.log"
