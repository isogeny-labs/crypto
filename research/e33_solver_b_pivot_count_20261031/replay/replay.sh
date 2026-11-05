#!/usr/bin/env bash
# E33 validation replay (independent validator, 2026-11-01).  Run from the
# repository root:
#
#   BIN_S=<fresh e33_stage_a> BIN_P=<fresh quartic_pipeline_gls (c443138920)>
#   BIN_O=<fresh quartic_pipeline_gls (bd0118f39, pre-E33)>
#   BIN_C=<fresh e33val_controls> STAGE_A_OUT=<path outside the repository>
#   CORE=<one cpu> research/e33_solver_b_pivot_count_20261031/replay/replay.sh
#
# Every process runs alone on one core (taskset -c $CORE).  Commands are the
# producer's (examples/e33_stage_a.rs header; experiments/37_solver_b_pivot_count/
# run_e2e.sh) with only the binary and the output path changed;
# RAYON_NUM_THREADS=4 end to end fixes the target stream (§9.19), =1 in Stage A
# as recorded.  The sample comes from replay/sample.json (nonce-drawn).
# Restart-safe: finished outputs are skipped.  Operation counts only.
set -u
ev=research/e33_solver_b_pivot_count_20261031
out=$ev/replay
log=$out/run.log
mkdir -p "$out/e2e" "$out/identity" "$out/controls"
run() { echo "$(date -u +%FT%TZ) start $*" >>"$log"; "$@"; local rc=$?; echo "$(date -u +%FT%TZ) end rc=$rc" >>"$log"; return $rc; }
order=$(sed -n 's/.*"e2e_order": "\([^"]*\)".*/\1/p' "$out/sample.json")
idc=$(sed -n 's/.*"identity_order_excluding_p101s2": "\([^ "]*\).*/\1/p' "$out/sample.json")
ctl=$(sed -n 's/.*"controls_order": "\([^ "]*\) \([^ "]*\).*/\1 \2/p' "$out/sample.json")
# 1. Negative controls (validator-owned program), two nonce-drawn curves, 32 calls each,
#    plus NC3 on 16 calls with a non-empty decomposition set from a second nonce stream.
for c in $ctl; do
  f=$out/controls/controls_$c.json
  [ -s "$f" ] && continue
  p=${c#p}; p=${p%s*}; s=${c##*s}
  rng=$(sed -n "s/.*\"controls_rng_$c\": \([0-9]*\).*/\1/p" "$out/sample.json")
  RAYON_NUM_THREADS=1 run taskset -c "$CORE" "$BIN_C" --p "$p" --seed "$s" --calls 32 --nonce "$rng" --nc3 16 >"$f.part" 2>>"$out/controls/controls.log" && mv "$f.part" "$f"
done
# 2. Stage A in full (all ten curves, 1,000 calls, seven solvers).
if [ ! -s "$STAGE_A_OUT" ]; then
  RAYON_NUM_THREADS=1 run taskset -c "$CORE" "$BIN_S" --sizes 101,149,197,269,397 --seeds 1,2 --targets 1000 \
    --json "$STAGE_A_OUT.part" >"$out/stage_a.log" 2>&1 && mv "$STAGE_A_OUT.part" "$STAGE_A_OUT"
fi
# 3. Existing solvers: pre-E33 build against E33's build, one nonce-drawn curve.
p=${idc#p}; p=${p%s*}; s=${idc##*s}
for cfg in macaulay chain chain-sparsest chain-karatsuba; do
  case $cfg in
    chain-karatsuba) flags="--solver chain --karatsuba" ;;
    *) flags="--solver $cfg" ;;
  esac
  for side in old new; do
    f=$out/identity/${side}_${cfg}_p${p}s${s}.json
    [ -s "$f" ] && continue
    if [ "$side" = old ]; then bin=$BIN_O; else bin=$BIN_P; fi
    RAYON_NUM_THREADS=4 run taskset -c "$CORE" "$bin" --sizes "$p" --first-seed "$s" --seeds 1 \
      --rho-runs 16 --max-residuals 5000000 $flags --json "$f.part" >>"$out/identity/identity.log" 2>&1 && mv "$f.part" "$f"
  done
done
# 4. End to end, curves in the nonce order, all three solvers per curve.
for c in $order; do
  p=${c#p}; p=${p%s*}; s=${c##*s}
  for solver in chain-sparsest-counted chain-sparsest-inc chain-sparsest-est; do
    f=$out/e2e/e2e_${solver}_karatsuba_p${p}s${s}.json
    [ -s "$f" ] && continue
    [ -e "$out/STOP" ] && { echo "$(date -u +%FT%TZ) stop requested" >>"$log"; exit 0; }
    RAYON_NUM_THREADS=4 run taskset -c "$CORE" "$BIN_P" --sizes "$p" --first-seed "$s" --seeds 1 \
      --rho-runs 16 --max-residuals 5000000 --solver "$solver" --karatsuba --json "$f.part" \
      >>"$out/e2e/e2e_${solver}.log" 2>&1 && mv "$f.part" "$f"
  done
done
echo "$(date -u +%FT%TZ) finished" >>"$log"
