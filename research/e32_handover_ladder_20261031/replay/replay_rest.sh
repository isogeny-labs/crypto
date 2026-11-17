#!/usr/bin/env bash
# E32 validation replay, phase 2 (independent validator, 2026-11-01): the 22
# index-calculus runs the nonce did not select, so that every recorded run is
# replayed.  Started only after replay.sh (the nonce sample and all 33 walk
# pools) had finished; same binary, arguments, core, environment and guards.
#
#   BIN=<fresh e32_ladder> research/e32_handover_ladder_20261031/replay/replay_rest.sh
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
log "phase 2 start (IC runs not selected by the nonce)"
for p in 101 149 197 269 397 557 773 1093 1549 2213 3089; do
  for s in 1 2 3; do
    f=$out/runs/ic_p${p}s${s}.json
    [ -s "$f" ] && continue
    rr=1
    case "$p" in 101 | 149 | 197 | 269 | 397) [ "$s" -le 2 ] && rr=16 ;; esac
    guard "ic p=$p s=$s"
    log "start ic --p $p --seed $s --rho-runs $rr --max-residuals 40000000 (free $(df -Pm / | awk 'NR==2 {print $4}') MB)"
    taskset -c 3 "$BIN" ic --p "$p" --seed "$s" --rho-runs "$rr" --max-residuals 40000000 \
      --json "$f.part" >>"$out/replay_stdout.log" 2>&1
    rc=$?
    [ $rc -eq 0 ] && mv "$f.part" "$f"
    log "end rc=$rc ic --p $p --seed $s"
    [ $rc -eq 0 ] || exit 4
  done
done
log "phase 2 finished"
