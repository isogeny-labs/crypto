#!/bin/sh
# Records the runner: OS, CPU model, cores, caches, load.  OUT  output directory
OUT=${OUT:?}; mkdir -p "$OUT"
{
  echo "utc $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  uname -a
  uptime
  if [ "$(uname)" = Darwin ]; then
    sysctl -n machdep.cpu.brand_string hw.ncpu hw.perflevel0.physicalcpu hw.l1dcachesize hw.l2cachesize hw.memsize 2>/dev/null
  else
    lscpu
    grep -m1 -i "^Features" /proc/cpuinfo || true
  fi
  rustc --version
  echo "commit ${GITHUB_SHA:-unknown}"
} > "$OUT/host.txt" 2>&1
