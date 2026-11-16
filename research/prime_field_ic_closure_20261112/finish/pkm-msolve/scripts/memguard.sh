#!/bin/bash
# Host memory guard (operator safety, shared VM): if MemAvailable < 1200 MB, SIGKILL the largest
# msolve process of this study and log it. Runs until killed.
cd "$(dirname "$0")/.."
while true; do
  avail=$(awk '/MemAvailable/{print int($2/1024)}' /proc/meminfo)
  if [ "$avail" -lt 1200 ]; then
    big=""; bigrss=0
    for p in $(pgrep -x msolve); do
      if grep -q "pkm-msolve" /proc/$p/cmdline 2>/dev/null; then
        r=$(awk '/VmRSS/{print $2}' /proc/$p/status 2>/dev/null); r=${r:-0}
        if [ "$r" -gt "$bigrss" ]; then bigrss=$r; big=$p; fi
      fi
    done
    if [ -n "$big" ]; then
      sys=$(tr '\0' ' ' < /proc/$big/cmdline | grep -oE 'systems/[A-Za-z0-9_]+' | head -1)
      kill -9 $big
      echo "$(date -u +%FT%TZ) operator memory-guard: MemAvailable=${avail}MB, killed msolve pid $big ($sys, rss ${bigrss}kB)" >> progress.txt
      sleep 20
    fi
  fi
  sleep 5
done
