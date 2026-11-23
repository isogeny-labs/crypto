#!/usr/bin/env bash
# E32 validation replay (independent validator, 2026-11-01): recorded runs of
# experiments/36_handover_ladder/ with a fresh build of the recorded binary
# commit (e757710521 + examples/e32_ladder.rs from 246460f094).  Run from the
# repository root:
#
#   BIN=<fresh e32_ladder> research/e32_handover_ladder_20261031/replay/replay.sh
#
# Scope (drawn before any replay, see selection.txt): at every size the index
# calculus of one curve chosen by the nonce, and the walk-(iii) pool of every
# curve (33 pools, 528 walks).  Arguments are run_ladder.sh's, unchanged
# (--rho-runs 16 for seeds 1-2 at p <= 397, else 1; --max-residuals 40000000;
# --walks 16 --pool 16); RAYON_NUM_THREADS=4 as the producer (it fixes the
# pipeline's target stream, note §9.19).  One process at a time on one core
# (taskset -c 3).  Machine guards: a run starts only with >= 350 MB free on /
# and >= 4 GB MemAvailable.  Restart-safe.  Operation counts only; no wall time
# is compared.
set -u
out=research/e32_handover_ladder_20261031/replay
mkdir -p "$out/runs"
export RAYON_NUM_THREADS=4
log() { echo "$(date -u +%FT%TZ) $*" >>"$out/replay.log"; }
guard() {
  local free mem
  free=$(df -Pm / | awk 'NR==2 {print $4}')
  mem=$(awk '/MemAvailable/ {print int($2/1024)}' /proc/meminfo)
  if [ "$free" -lt 350 ] || [ "$mem" -lt 4096 ]; then
    log "guard: stop before $1 (free ${free} MB on /, MemAvailable ${mem} MB)"
    exit 3
  fi
}
run() { # run <out.json> <args...>
  local f=$1
  shift
  [ -s "$f" ] && return 0
  guard "$*"
  log "start $* (free $(df -Pm / | awk 'NR==2 {print $4}') MB)"
  taskset -c 3 "$BIN" "$@" --json "$f.part" >>"$out/replay_stdout.log" 2>&1
  local rc=$?
  if [ $rc -eq 0 ]; then mv "$f.part" "$f"; fi
  log "end rc=$rc $*"
  [ $rc -eq 0 ] || exit 4
}
while read -r p _ s; do
  rr=1
  case "$p" in 101 | 149 | 197 | 269 | 397) [ "$s" -le 2 ] && rr=16 ;; esac
  run "$out/runs/ic_p${p}s${s}.json" ic --p "$p" --seed "$s" --rho-runs "$rr" --max-residuals 40000000
  for w in 1 2 3; do
    run "$out/runs/walk_p${p}s${w}.json" walk --p "$p" --seed "$w" --walks 16 --pool 16
  done
done <"$out/selection.txt"
log finished
