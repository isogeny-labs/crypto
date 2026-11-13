#!/usr/bin/env bash
# Main sequence on the dedicated pod (PROTOCOL.md). Strictly sequential. Run from /work.
set -u
cd /work
B=./v2w2
QS=(4194271 8388571 16777183 33554383 67108819)
mkdir -p out
{ date -u +%FT%TZ; uname -a; nproc; free -b; lscpu; cat /proc/cpuinfo | grep -m1 'model name'; sha256sum $B runjob.py; } > out/host.txt 2>&1
cal() { # round r: rotate sizes by r
  r=$1
  for i in 0 1 2 3 4; do
    q=${QS[$(( (i + r) % 5 ))]}
    for c in 1 2 3; do python3 runjob.py out/cal${r}_${q}_${c} 1200 $B calib $q $c >/dev/null; done
  done
  echo "cal $r done $(date -u +%FT%TZ)" >> out/progress.txt
}
unit() { V2W_COMPACT=1 python3 runjob.py out/ic_$1_$2_D 3600 $B ic $1 $2 D 3.0 2.0 4 4 >/dev/null; echo "unit $1 $2 done $(date -u +%FT%TZ)" >> out/progress.txt; }
cal 1
for q in 4194271 8388571 16777183 33554383 67108819; do unit $q 1; done
cal 2
for q in 67108819 33554383 16777183 8388571 4194271; do unit $q 2; done
cal 3
for q in 4194271 8388571 16777183 33554383 67108819; do unit $q 3; done
cal 4
echo "MAIN DONE $(date -u +%FT%TZ)" >> out/progress.txt
