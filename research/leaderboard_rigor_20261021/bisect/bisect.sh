#!/bin/bash
# Binary search for the first first-parent commit (touching src/cryptanalysis) at which the ladder n13
# replay-all stops reproducing.  Good: 03ac2e467 (72/72).  Bad: 771b94753 (24/72).
cd /home/user/crypto
B=research/leaderboard_rigor_20261021/bisect
S=$PWD/research/f6_ic_ecbench_ladder_20261005/sessions/n13
mapfile -t C < $B/candidates.txt
lo=-1; hi=$(( ${#C[@]} - 1 ))     # C[lo] good (virtual -1 = 03ac2e467), C[hi] bad
while [ $((hi - lo)) -gt 1 ]; do
  mid=$(( (lo + hi) / 2 ))
  line=$($B/replay_at_commit.sh "${C[$mid]}" "$S" "$B" "b${mid}" | tail -1)
  echo "$mid ${C[$mid]} $line" >> $B/bisect.log
  rep=$(echo "$line" | jq -r '.reproduced // 0' 2>/dev/null); tot=$(echo "$line" | jq -r '.replays // 72' 2>/dev/null)
  if [ "$rep" = "72" ]; then lo=$mid; else hi=$mid; fi
done
echo "FIRST_BAD ${C[$hi]}" >> $B/bisect.log
git log -1 --format='%h %cI %s' "${C[$hi]}" >> $B/bisect.log
