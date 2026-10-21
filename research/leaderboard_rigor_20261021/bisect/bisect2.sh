#!/bin/bash
# Build-failure-aware search in (60, 74]: C[60] good (72/72), C[74] bad (24/72).  A commit that does not build
# is untestable and skipped (the nearest testable commit inside the open range is used instead).
cd /home/user/crypto
B=research/leaderboard_rigor_20261021/bisect
S=$PWD/research/f6_ic_ecbench_ladder_20261005/sessions/n13
mapfile -t C < $B/candidates.txt
lo=60; hi=74
declare -A SKIP=([65]=1 [70]=1)
while [ $((hi - lo)) -gt 1 ]; do
  mid=$(( (lo + hi) / 2 )); found=""
  for d in 0 1 -1 2 -2 3 -3; do
    m=$((mid + d)); [ $m -gt $lo ] && [ $m -lt $hi ] && [ -z "${SKIP[$m]:-}" ] && { found=$m; break; }
  done
  [ -z "$found" ] && break
  line=$($B/replay_at_commit.sh "${C[$found]}" "$S" "$B" "c${found}" | tail -1)
  echo "$found ${C[$found]} $line" >> $B/bisect.log
  if echo "$line" | grep -q build_failed; then SKIP[$found]=1; continue; fi
  rep=$(echo "$line" | jq -r '.reproduced // 0'); if [ "$rep" = "72" ]; then lo=$found; else hi=$found; fi
done
echo "RANGE good=$lo(${C[$lo]}) first_bad_in=($lo,$hi] bad=$hi(${C[$hi]})" >> $B/bisect.log
for i in $(seq $((lo + 1)) $hi); do git log -1 --format="  candidate $i %h %cs %s" "${C[$i]}" | cut -c1-150 >> $B/bisect.log; done
