#!/bin/bash
# v2 (fixes v1's legend-line match). Operator early stop (disclosed protocol deviation): SIGKILL a running
# msolve of this study once its -v 2 trace shows a completed F4 round line of degree >= $1.
cd "$(dirname "$0")/.."
DEG=${1:-8}
while true; do
  for p in $(pgrep -x msolve); do
    sys=$(tr '\0' ' ' < /proc/$p/cmdline 2>/dev/null | grep -oE 'systems/[A-Za-z0-9_]+' | head -1 | sed 's#systems/##')
    [ -z "$sys" ] && continue
    if awk -v d=$DEG '$0 ~ /^ *[0-9]+ +[0-9]+ +[0-9]+ +[0-9]+ x [0-9]+/ { if ($1+0 >= d) f=1 } END{exit !f}' runs/$sys.log 2>/dev/null; then
      r=$(awk '/VmRSS/{print $2}' /proc/$p/status 2>/dev/null)
      echo "$(date -u +%FT%TZ) operator early stop (watcher v2): $sys (pid $p, rss ${r}kB) at first round of degree >= $DEG" >> progress.txt
      kill -9 $p
    fi
  done
  sleep 2
done
