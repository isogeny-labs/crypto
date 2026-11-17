#!/usr/bin/env bash
# Pod job of the PKM m = 3 msolve -t run (research/pkm_m3_n21_msolve_20261018/
# PROTOCOL.md, Amendment A1). Called by pod/boot.sh from the repository root:
#   job.sh OUT CAPS T0      (OUT served on :8000; CAPS the pod lifetime in s; T0 its start)
# Env: MSOLVE_SYSTEMS (the measured systems, run at once), MSOLVE_WALL_MAX
# (default 86400), SERVE_SECONDS (as boot.sh), DRY=1 (plan only, no msolve).
# Plumbing only: no API key or other secret is on the pod.
set -u
OUT=$1; CAPS=$2; T0=$3
remaining() { echo $(( CAPS - ($(date +%s) - T0) )); }
status() { echo "$(date -u +%FT%TZ) $1" >> "$OUT/STATUS"; }
D=research/pkm_m3_n21_msolve_20261018
SYSTEMS=${MSOLVE_SYSTEMS:-"pkm_kummer_p2013265921_m3_t7_x221026520 pkm_kummer_p2013265921_m3_t7_x704078485"}   # card P2a; P2b passes the two m3_t8 targets
CONTROL=pkm_kummer_p2013265921_m3_t5_x1289903598
set -- $SYSTEMS; PAR=$#
NCPU=$(nproc); THREADS=$(( NCPU / PAR )); [ "$THREADS" -ge 1 ] || THREADS=1
LIM=$(cat /sys/fs/cgroup/memory.max 2>/dev/null || cat /sys/fs/cgroup/memory/memory.limit_in_bytes 2>/dev/null || echo max)
MT=$(( $(awk '/^MemTotal:/ {print $2}' /proc/meminfo) * 1024 ))
{ [ "$LIM" = max ] || [ "$LIM" -gt "$MT" ]; } && LIM=$MT   # no limit set (v1 reports 2^63)
CAP=$(( LIM * 85 / 100 / PAR ))
WALL=$(( $(remaining) - ${SERVE_SECONDS:-2700} - 2700 - 1800 ))   # control (45 min) and margin first
[ "$WALL" -gt "${MSOLVE_WALL_MAX:-86400}" ] && WALL=${MSOLVE_WALL_MAX:-86400}
mkdir -p "$D/runs-mt"; ln -sfn "/workspace/src/$D/runs-mt" "$OUT/msolve-mt" 2>/dev/null || true
status "MSOLVE_MT plan: $PAR systems at once, $THREADS threads each, CAP=$CAP, WALL=$WALL"
if [ "${DRY:-0}" = 1 ]; then echo "PAR=$PAR THREADS=$THREADS CAP=$CAP WALL=$WALL SYSTEMS=$SYSTEMS"; exit 0; fi
[ "$WALL" -gt 3600 ] || { status "MSOLVE_MT_NOT_RUN: wall $WALL s under 1 h"; exit 1; }
if ! command -v msolve > /dev/null; then
  apt-get install -y -qq msolve >> "$OUT/apt.log" 2>&1 || {
    apt-get install -y -qq autoconf automake libtool libgmp-dev libmpfr-dev libflint-dev >> "$OUT/apt.log" 2>&1
    (cd /workspace && git clone -q --depth 1 --branch v0.6.5 https://github.com/algebraic-solving/msolve && cd msolve \
       && ./autogen.sh && ./configure && make -j"$NCPU" && make install) > "$OUT/msolve-build.log" 2>&1; }
fi
{ echo "date: $(date -u +%FT%TZ)"; echo "commit: $(git rev-parse HEAD)"; echo "msolve: $(command -v msolve) $(msolve -h 2>&1 | head -2 | tr '\n' ' ')"
  echo "msolve sha256: $(sha256sum "$(command -v msolve)" | cut -d' ' -f1)"; echo "cgroup limit bytes: $LIM; CAP per process: $CAP; PAR: $PAR; THREADS each: $THREADS; WALL: $WALL"
  echo "pod: ${RUNPOD_POD_ID:-none}"; lscpu; } > "$D/runs-mt/host-$(hostname).txt"
# Control (A1.3): the N = 15 system at the measured thread count must give the
# committed answer (one-element basis, no solution) inside 45 minutes.
status "MSOLVE_MT control $CONTROL threads=$THREADS"
THREADS=$THREADS CAP=$CAP WALL=2700 "$D/run_mt.sh" "$CONTROL"
if ! grep -q "exit=0" "$D/runs-mt/$CONTROL.mem" || ! cmp -s "$D/runs-mt/$CONTROL.out" "$D/runs/$CONTROL.out"; then
  status "MSOLVE_MT_CONTROL_FAILED (see runs-mt/$CONTROL.*); measured systems not run"; exit 1; fi
status "MSOLVE_MT control agrees ($(cat "$D/runs-mt/$CONTROL.mem"))"
for s in $SYSTEMS; do
  THREADS=$THREADS CAP=$CAP WALL=$WALL "$D/run_mt.sh" "$s" & sleep 30
done
status "MSOLVE_MT running: $SYSTEMS"; wait
status "MSOLVE_MT_DONE"
