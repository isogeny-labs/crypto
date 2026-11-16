#!/usr/bin/env bash
# Run msolve (F4, grevlex, GB only, 1 thread, verbose) on every .ms system in DIR.
# Per system: TLIMIT s wall (default 600), VLIMIT KiB address space (default 6 GiB). Logs go to DIR/<name>.log.
set -u
DIR=${1:?usage: run.sh DIR [jobs]}
JOBS=${2:-3}
export DIR
ls "$DIR"/*.ms | { if [ -n "${ORDER:-}" ]; then for c in $ORDER; do echo "$DIR/$c.ms"; done; else sort -V; fi; } | xargs -P "$JOBS" -I{} bash -c '
  f={}; b=${f%.ms}
  [ -s "$b.log" ] && grep -q "^exit=" "$b.log" && exit 0
  start=$(date +%s.%N)
  ( ulimit -v ${VLIMIT:-6291456}; timeout ${TLIMIT:-600} msolve -g 2 -t 1 -v 2 -f "$f" -o "$b.gb" ) > "$b.log" 2>&1
  rc=$?
  end=$(date +%s.%N)
  echo "exit=$rc wall=$(echo "$end - $start" | bc)" >> "$b.log"
  rm -f "$b.gb"
'
