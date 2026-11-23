#!/usr/bin/env bash
# E39 independent replay (validator, 2026-11-02).  Not producer code.
#
# Runs the validator's fresh build of the recorded binaries (clean export of
# e1cd726577 into /dev/shm/e39_src, target /dev/shm/v39) on: the nonce-selected
# end-to-end runs (selection.txt), pilot P1, the main control run, the whole of
# Stage A, all 21 identity configurations of G2.191 and the two walk (iii)
# reference records.  Flags are the recorded ones (run_e2e.sh, identity.sh, the
# example headers): --rho-runs 16 --max-residuals 5000000 --karatsuba,
# RAYON_NUM_THREADS=4 for the pipelines (it fixes the target stream), 1 for
# Stage A and the controls.  Runs from /dev/shm/v39run (E24's and E25's
# committed records for the walks, from a git archive of 484766c0cb); writes
# only under <out>.
#
#   usage: replay.sh <bin dir> <out dir> <part>
#   part: e2e | checks | identity | walks
set -u
bin=$1; out=$2; part=$3
mkdir -p "$out"
cd /dev/shm/v39run || exit 1
stamp() { date -u +%FT%TZ; }
log() { echo "$(stamp) $*" >>"$out/replay.log"; }
pipe() { # name binary p seed salt solver extra...
  local name=$1 b=$2 p=$3 s=$4 salt=$5 solver=$6; shift 6
  [ -s "$out/$name.json" ] && return 0
  log "start $name"
  local sargs=(--solver "$solver") xargs=(--stream-salt "$salt")
  [ "$solver" = "-" ] && sargs=()
  [ "$salt" = "-" ] && xargs=()   # identity.sh passes no salt for its first nine configurations
  RAYON_NUM_THREADS=4 nice -n 10 "$bin/$b" --sizes "$p" --first-seed "$s" --seeds 1 --rho-runs 16 \
    --max-residuals 5000000 "${sargs[@]}" "$@" "${xargs[@]}" --json "$out/$name.json.part" \
    >"$out/$name.log" 2>&1
  local rc=$?
  [ $rc -eq 0 ] && mv "$out/$name.json.part" "$out/$name.json"
  log "end $name exit $rc"
}
e2e_run() { # set_solver_pPsS
  local name=$1 set solver p s rest ps
  set=${name%%_*}; rest=${name#*_}; solver=${rest%_p*}; ps=${rest##*_p}; p=${ps%s*}; s=${ps#*s}
  local b=quartic_pipeline_gls; [ "$set" = e24 ] && b=quartic_pipeline_v4
  pipe "e2e_$name" "$b" "$p" "$s" E39E0001 "$solver" --karatsuba
}
case $part in
e2e)
  while read -r pool name _; do
    case $pool in
      e2e) e2e_run "$name" ;;
      pair) set=${name%%_*}; ps=${name#*_}
            e2e_run "${set}_chain-counted_${ps}"; e2e_run "${set}_chain-fifo_${ps}" ;;
    esac
  done < "$(dirname "$0")/selection.txt"
  ;;
checks)
  for c in pilot:E39D0001:8 checks:E39C0001:32; do
    name=${c%%:*}; rest=${c#*:}; salt=${rest%%:*}; calls=${rest#*:}
    [ -s "$out/$name.json" ] && continue
    log "start $name"
    RAYON_NUM_THREADS=1 nice -n 10 "$bin/e39_checks" --salt "$salt" --calls "$calls" --json "$out/$name.json" \
      >/dev/null 2>"$out/$name.log"
    log "end $name exit $?"
  done
  if [ ! -e "$out/stage_a.exit" ]; then
    log "start stage_a"
    RAYON_NUM_THREADS=1 nice -n 10 "$bin/e39_stage_a" --sizes 101,149,197,269,397 --seeds 1,2 --targets 1000 \
      --json "$out/stage_a.json" >"$out/stage_a.log" 2>&1
    echo $? >"$out/stage_a.exit"
    log "end stage_a exit $(cat "$out/stage_a.exit")"
  fi
  ;;
identity)
  g=quartic_pipeline_gls; v=quartic_pipeline_v4
  pipe id_macaulay $g 101 2 - macaulay
  pipe id_chain $g 101 2 - chain
  pipe id_chainsparsest $g 101 2 - chain-sparsest
  pipe id_chain_karatsuba $g 101 2 - chain --karatsuba
  pipe id_est_karatsuba $g 101 2 - chain-sparsest-est --karatsuba
  for r in chain-fifo chain-static chain-fifo-static; do pipe "id_${r}_karatsuba" $g 101 2 - "$r" --karatsuba; done
  pipe id_v4_default $v 101 2 - -
  for s in chain-counted chain-fifo chain-static chain-fifo-static chain-sparsest-est; do
    pipe "id_e37salt_gls_$s" $g 101 2 E37E0001 "$s" --karatsuba
  done
  pipe id_e37salt_e24_chain-fifo $v 101 2 E37E0001 chain-fifo --karatsuba
  for c in e37_checks:e37_checks: e38_pilot:e38_checks:"--salt E38D0001 --calls 8" e38_pilot_p2:e38_checks:"--salt E38D0101 --calls 8 --fallback cfmid"; do
    name=${c%%:*}; rest=${c#*:}; b=${rest%%:*}; extra=${rest#*:}
    [ -s "$out/id_$name.json" ] && continue
    log "start id_$name"
    # shellcheck disable=SC2086
    RAYON_NUM_THREADS=1 nice -n 10 "$bin/$b" $extra --json "$out/id_$name.json" >/dev/null 2>"$out/id_$name.log"
    log "end id_$name exit $?"
  done
  ;;
walks)
  if [ ! -s "$out/walk_e24.json" ]; then
    log "start walk_e24"
    RAYON_NUM_THREADS=4 nice -n 10 "$bin/e37_walk_e24" --json "$out/walk_e24.json" >"$out/walk_e24.log" 2>&1
    log "end walk_e24 exit $?"
  fi
  if [ -x "$bin/e27_walk" ] && [ ! -s "$out/stage_w.json" ]; then
    log "start stage_w"
    RAYON_NUM_THREADS=4 nice -n 10 "$bin/e27_walk" --sizes 101,149,197,269,397 --seeds 1,2 --e25 experiments/30_gls_v4 \
      --json "$out/stage_w.json" >"$out/stage_w.log" 2>&1
    log "end stage_w exit $?"
  fi
  ;;
esac
log "finished $part"
