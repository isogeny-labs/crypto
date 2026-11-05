#!/usr/bin/env bash
# E40 independent replay (validator, 2026-11-02).  Not producer code.
#
# Runs the validator's fresh build of the recorded binaries (clean export of
# 9b2a47238b; e40_checks_v2 from 86d0f86c25) on the nonce-selected end-to-end
# rows (selection.txt), the E25 references the nonce selected, all seven
# identity configurations of G2.204, the controls (v1 and v2) and the whole of
# Stage A.  Flags are the recorded ones (run_e2e.sh, identity.sh, the example
# headers): --rho-runs 16 --max-residuals 5000000, RAYON_NUM_THREADS=4 for the
# pipelines (it fixes the target stream), 1 for Stage A and the controls.
# Runs from the clean export root, writes only under <out>.
#
#   usage: replay.sh <bin dir> <out dir> <part>
#   part: gls | e24 | stage
set -u
bin=$1; out=$2; part=$3
mkdir -p "$out"
cd /dev/shm/e40_src || exit 1
stamp() { date -u +%FT%TZ; }
pipe() { # name binary p seed salt solver extra...
  local name=$1 b=$2 p=$3 s=$4 salt=$5 solver=$6; shift 6
  [ -s "$out/$name.json" ] && return 0
  echo "$(stamp) start $name" >>"$out/replay.log"
  RAYON_NUM_THREADS=4 nice -n 10 "$bin/$b" --sizes "$p" --first-seed "$s" --seeds 1 --rho-runs 16 \
    --max-residuals 5000000 --solver "$solver" --stream-salt "$salt" "$@" --json "$out/$name.json.part" \
    >"$out/$name.log" 2>&1 && mv "$out/$name.json.part" "$out/$name.json"
  echo "$(stamp) end $name exit $?" >>"$out/replay.log"
}
case $part in
gls)
  pipe gls_chain-profile_p101s1 quartic_pipeline_gls 101 1 E37E0001 chain-profile --karatsuba
  pipe gls_chain-counted_E40E0002_p397s1 quartic_pipeline_gls 397 1 E40E0002 chain-counted --karatsuba
  pipe gls_chain-profile_E40E0002_p397s1 quartic_pipeline_gls 397 1 E40E0002 chain-profile --karatsuba
  pipe gls_macaulay_p101s1 quartic_pipeline_gls 101 1 E37E0001 macaulay --karatsuba
  pipe id_macaulay quartic_pipeline_gls 101 2 0 macaulay
  pipe id_chain quartic_pipeline_gls 101 2 0 chain
  pipe id_chainsparsest quartic_pipeline_gls 101 2 0 chain-sparsest
  pipe id_chain_karatsuba quartic_pipeline_gls 101 2 0 chain --karatsuba
  pipe id_gls_counted_e37 quartic_pipeline_gls 101 2 E37E0001 chain-counted --karatsuba
  pipe id_gls_fifo_e37 quartic_pipeline_gls 101 2 E37E0001 chain-fifo --karatsuba
  pipe gls_chain-profile_p397s1 quartic_pipeline_gls 397 1 E37E0001 chain-profile --karatsuba
  pipe gls_chain-profile_p397s2 quartic_pipeline_gls 397 2 E37E0001 chain-profile --karatsuba
  ;;
e24)
  pipe id_e24_counted_e37 quartic_pipeline_v4 101 1 E37E0001 chain-counted --karatsuba
  pipe e24_chain-profile_p269s1 quartic_pipeline_v4 269 1 E37E0001 chain-profile --karatsuba
  pipe e24_chain-profile_p397s2 quartic_pipeline_v4 397 2 E37E0001 chain-profile --karatsuba
  pipe e24_chain-profile_p521s2 quartic_pipeline_v4 521 2 E37E0001 chain-profile --karatsuba
  pipe e24_macaulay_p521s1 quartic_pipeline_v4 521 1 E37E0001 macaulay --karatsuba
  ;;
walks)
  # The matched rho, walk (iii), from the same crypto_lib build as the pipelines:
  # e27_walk (E25's GLS curves) and e37_walk_e24 (E24's curves), recorded flags
  # (defaults: walks 16, pool 16); they read E25's / E24's committed records
  # (experiments/30_gls_v4, experiments/29_quartic_pipeline_v4) from the export.
  if [ ! -s "$out/stage_w.json" ]; then
    echo "$(stamp) start stage_w" >>"$out/replay.log"
    nice -n 10 "$bin/e27_walk" --sizes 101,149,197,269,397 --seeds 1,2 --e25 experiments/30_gls_v4 \
      --json "$out/stage_w.json" >"$out/stage_w.log" 2>&1
    echo "$(stamp) end stage_w exit $?" >>"$out/replay.log"
  fi
  if [ ! -s "$out/walk_e24.json" ]; then
    echo "$(stamp) start walk_e24" >>"$out/replay.log"
    nice -n 10 "$bin/e37_walk_e24" --json "$out/walk_e24.json" >"$out/walk_e24.log" 2>&1
    echo "$(stamp) end walk_e24 exit $?" >>"$out/replay.log"
  fi
  ;;
stage)
  for c in checks_v2:e40_checks_v2 checks_v1:e40_checks; do
    name=${c%%:*}; b=${c##*:}
    [ -s "$out/$name.json" ] && continue
    echo "$(stamp) start $name" >>"$out/replay.log"
    RAYON_NUM_THREADS=1 nice -n 10 "$bin/$b" --calls 32 --json "$out/$name.json" >"$out/$name.log" 2>&1
    echo "$(stamp) end $name exit $?" >>"$out/replay.log"
  done
  if [ ! -e "$out/stage_a.exit" ]; then
    echo "$(stamp) start stage_a" >>"$out/replay.log"
    RAYON_NUM_THREADS=1 nice -n 10 "$bin/e40_stage_a" --targets 1000 --json "$out/stage_a.json" >"$out/stage_a.log" 2>&1
    echo $? >"$out/stage_a.exit"
    echo "$(stamp) end stage_a exit $(cat "$out/stage_a.exit")" >>"$out/replay.log"
  fi
  ;;
esac
echo "$(stamp) finished $part" >>"$out/replay.log"
