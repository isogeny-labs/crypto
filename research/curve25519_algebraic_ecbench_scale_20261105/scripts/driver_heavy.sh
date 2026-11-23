#!/bin/bash
# driver_heavy.sh : k24 c0..c5 then k28 c0..c1; skips chunks whose attempt dir holds a complete session.
cd "$(dirname "$0")/.."
for kc in "24 0" "24 1" "24 2" "24 3" "24 4" "24 5" "28 0" "28 1"; do
  set -- $kc
  if ls /home/user/scratch_S2/k$1-c$2-a*/session.json >/dev/null 2>&1 && grep -l '"complete"' /home/user/scratch_S2/k$1-c$2-a*/session.json >/dev/null 2>&1; then continue; fi
  n=$(ls -d /home/user/scratch_S2/k$1-c$2-a* 2>/dev/null | wc -l); scripts/run_chunks.sh $1 $2 a$((n+1))
done
