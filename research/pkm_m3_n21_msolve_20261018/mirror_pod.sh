#!/bin/bash
# Mirror a pkm-msolve-mt pod's records to this machine every INTERVAL s, independent of any agent session.
# Stops the pod (A1.4) once a degree >= 8 round reports `new`, or once the measured system's .mem exists
# (msolve exited: wall, memory stop, or finished), after a final full fetch.
#   nohup mirror_pod.sh POD_ID SYSTEM OUTDIR [INTERVAL] > OUTDIR/mirror.log 2>&1 &
set -u
POD=$1; SYS=$2; OUT=$3; INT=${4:-600}
U=https://$POD-8000.proxy.runpod.net
LAUNCH=/Volumes/SSD990-2/crypto/research/dreg_closure_m4l6_20261018/pod/launch.py
CTL=pkm_kummer_p2013265921_m3_t5_x1289903598
mkdir -p "$OUT/snapshots"
get() {  # get REMOTE LOCAL: keep the old copy unless the fetch succeeds with content
  curl -sS -m 120 -f "$U/$1" -o "$OUT/.tmp" 2>/dev/null && [ -s "$OUT/.tmp" ] && mv "$OUT/.tmp" "$OUT/$2"; rm -f "$OUT/.tmp"; }
fetch_all() {
  get STATUS STATUS
  for f in $CTL.log $CTL.mem $CTL.out $SYS.log $SYS.mem $SYS.out; do get "msolve-mt/$f" "$f"; done
  curl -sS -m 60 "$U/msolve-mt/" 2>/dev/null | grep -o 'href="host-[^"]*"' | cut -d'"' -f2 | while read h; do get "msolve-mt/$h" "$h"; done
  [ -s "$OUT/$SYS.log" ] && cp "$OUT/$SYS.log" "$OUT/snapshots/$SYS.$(date -u +%Y%m%dT%H%M%SZ).log"
}
stop_pod() { echo "$(date -u +%FT%TZ) $1"; echo "$(date -u +%FT%TZ) $1" >> "$OUT/DECISION"; python3 "$LAUNCH" terminate "$POD"; exit 0; }
echo "$(date -u +%FT%TZ) mirror start pod=$POD sys=$SYS every ${INT}s"
miss=0
while true; do
  fetch_all
  st=$(python3 "$LAUNCH" status "$POD" 2>/dev/null | grep -o '"desiredStatus": "[A-Z]*"')
  echo "$(date -u +%FT%TZ) $st | $(tail -1 "$OUT/STATUS" 2>/dev/null) | $(tail -1 "$OUT/$SYS.log" 2>/dev/null | cut -c1-90)"
  if [ -z "$st" ]; then miss=$((miss+1)); [ $miss -ge 3 ] && { echo "pod gone; exiting"; exit 0; }; else miss=0; fi
  if grep -qE '^ +([89]|1[0-9]) .* [1-9][0-9]* new' "$OUT/$SYS.log" 2>/dev/null; then
    fetch_all; stop_pod "H0 reading: degree>=8 round with new: $(grep -E '^ +([89]|1[0-9]) .* [1-9][0-9]* new' "$OUT/$SYS.log" | head -1)"; fi
  if [ -s "$OUT/$SYS.mem" ]; then fetch_all; stop_pod "msolve exited: $(cat "$OUT/$SYS.mem")"; fi
  grep -q "CONTROL_FAILED\|NOT_RUN\|CLONE_FAILED" "$OUT/STATUS" 2>/dev/null && { fetch_all; stop_pod "pod job failed: $(tail -1 "$OUT/STATUS")"; }
  sleep "$INT"
done
