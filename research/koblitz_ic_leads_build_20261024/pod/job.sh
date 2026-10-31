#!/usr/bin/env bash
# C1 job (PROTOCOL.md Amendments 2 and 4), run on the pod from the uploaded,
# hash-checked files in /workspace/in. Same command, limits and order as
# research/dreg_closure_m4l6_20261018/run.sh `deg5` and pod/boot.sh: the
# unsatisfiable draws 0, 3, 5, 7 at degree 5, then satisfiable draw 1.
set -u
IN=/workspace/in; OUT=/workspace/out; R=$OUT/runs; mkdir -p "$R"
T0=$(date +%s); CAPS=${POD_MAX_SECONDS:-75600}
remaining() { echo $(( CAPS - ($(date +%s) - T0) )); }
status() { echo "$(date -u +%FT%TZ) $1" >> "$OUT/STATUS"; }
log() { echo "$(date -u +%FT%TZ) $*" >> "$R/queue.log"; }
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 RAYON_NUM_THREADS=$(nproc)
C=$IN/dreg_closure; F=$IN/m3-19-7.jsonl; chmod +x "$C"
LIMIT=$(cat /sys/fs/cgroup/memory.max 2>/dev/null || echo max)
MT=$(( $(awk '/^MemTotal:/ {print $2}' /proc/meminfo) * 1024 ))
{ [ "$LIMIT" = max ] || [ "$LIMIT" -gt "$MT" ]; } && LIMIT=$MT
A=$(( LIMIT * 92 / 100 ))
TIMEV=""; command -v /usr/bin/time >/dev/null && TIMEV="/usr/bin/time -v"
{ echo "date: $(date -u +%FT%TZ)"; echo "closure sha256: $(sha256sum "$C" | cut -d' ' -f1)"
  echo "draws sha256: $(sha256sum "$F" | cut -d' ' -f1)"; echo "limit bytes: $LIMIT; A bytes: $A"
  echo "RAYON_NUM_THREADS: $RAYON_NUM_THREADS"; lscpu; } > "$R/host.txt"
if [ "$LIMIT" -lt 280000000000 ]; then status "HOST_TOO_SMALL $LIMIT"; exit 1; fi
one() { # name draw timeout [flags]
  local name=$1 k=$2 tmo=$3; shift 3
  log "$name d$k: start (to=5 A=$A cols<=1100000 tmo=$tmo)"
  local s; s=$(date +%s)
  (cd "$IN" && prlimit --as=$A timeout "$tmo" $TIMEV ./dreg_closure --to 5 --max-cols 1100000 --draw "$k" "$@" m3-19-7.jsonl) \
    > "$R/$name.d$k.jsonl" 2> "$R/$name.d$k.log"
  log "$name d$k: exit $? after $(( $(date +%s) - s )) s"
}
TMO5=${TMO5:-14400}
status "DEG5 C1 draws=0,3,5,7 TMO5=$TMO5"
for k in 0 3 5 7; do one deg5-m3-19-7 "$k" "$TMO5"; done
status C1_UNSAT_DONE
TC=$(( $(remaining) - ${SERVE_SECONDS:-2700} - 900 ))
if [ "$TC" -gt 1800 ]; then status "CONTROL d1 tmo=$TC"; one control5-m3-19-7 1 "$TC" --satisfiable; fi
status DONE
sleep "${SERVE_SECONDS:-2700}"
echo "$(date -u +%FT%TZ) serving window over" >> "$OUT/STATUS"
kill -TERM 1
