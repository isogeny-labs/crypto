#!/usr/bin/env bash
# run_cells_r3.sh SYS -- P1 round 3 (PROTOCOL-R3.md): T24 m5 t4, ONE system per invocation (rr or t),
# seed 2 (R2b's first admissible seed, fixed by PROTOCOL-R3.md), larger limits. Orchestration only;
# every number comes from p1tool and msolve.
#
# Environment (all required):
#   P1TOOL    absolute path of the built p1tool binary
#   OUT_DIR   directory to create (plain mkdir; fails if it exists)
#   VLIMIT    msolve address-space limit in KiB (frozen in PROTOCOL-R3.md)
#   TLIMIT    msolve wall-time limit in s (frozen in PROTOCOL-R3.md)
# Control: the emitted system must be byte-identical to R2b's (sha256 below); otherwise HARD_ERROR,
# no msolve job runs. A job reaching the memory cap or the time limit is a recorded result, not a stop.
set -u
SYS=${1:?usage: run_cells_r3.sh rr|t}
: "${P1TOOL:?}" "${OUT_DIR:?}" "${VLIMIT:?}" "${TLIMIT:?}"
HERE=$(cd "$(dirname "$0")" && pwd)
case "$SYS" in
  rr) WANT=9ba376d81c05109b11102c645153f9204df233600fec5851993a6bd5787e3cff ;;
  t)  WANT=ec0af27e8c8bec63160f1c69ced941a05379b79535942db20ce32409f836590e ;;
  *)  echo "HARD_ERROR: system must be rr or t"; exit 1 ;;
esac
mkdir -p "$(dirname "$OUT_DIR")" && mkdir "$OUT_DIR" || exit 2
hard_error() { echo "HARD_ERROR: $*" | tee -a "$OUT_DIR/ERROR.txt"; exit 1; }
{
  echo "run_start_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "system=$SYS seed=2 VLIMIT=$VLIMIT TLIMIT=$TLIMIT"
  sha256sum "$P1TOOL" "$HERE/../p1tool/src/main.rs" "$(command -v msolve)"
  echo "nproc=$(nproc)"; uname -a; uptime; free -k
  cat /sys/fs/cgroup/memory.max 2>/dev/null | sed 's/^/cgroup_memory_max=/'
  echo "msolve_processes_at_start:"; pgrep -a msolve; echo "end_msolve_processes_at_start"
} > "$OUT_DIR/provenance.txt" 2>&1
cat "$OUT_DIR/provenance.txt"
base="$OUT_DIR/${SYS}_s2"
"$P1TOOL" emit --system "$SYS" --kind dickson --p 14680063 --a 3607823 --b 6826282 \
  --m 5 --t 4 --seed 2 --out "$base.ms" --plant "$base.ms.plant.json" > "$base.emit.json" 2> "$base.emit.err" \
  || hard_error "emit exit $?"
GOT=$(sha256sum "$base.ms" | cut -d' ' -f1)
echo "ms_sha256=$GOT want=$WANT" | tee -a "$OUT_DIR/provenance.txt"
[ "$GOT" = "$WANT" ] || hard_error "emitted system differs from R2b"
VLIMIT=$VLIMIT TLIMIT=$TLIMIT bash "$HERE/msolve_cell.sh" "$base.ms" "$base.log" || hard_error "msolve_cell.sh exit $?"
"$P1TOOL" parse "$base.log" > "$base.parse.json" || hard_error "parse exit $?"
cat "$base.parse.json"
echo "RUN_COMPLETE system=$SYS $(grep -o '"status":"[^"]*"' "$base.parse.json") $(tail -n 1 "$base.log")"
sha256sum "$OUT_DIR"/*.txt "$OUT_DIR"/*.json "$base.log" "$base.ms"
exit 0
