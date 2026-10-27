#!/bin/bash
# Every complete ecbench session in a study directory that a leaderboard source file lives in must be listed in
# docs/ic/validation/replay-plan.json, so a session the board cites cannot silently go unreplayed.
# Needs jq.  Run from the repository root.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
plan=${REPLAY_PLAN:-docs/ic/validation/replay-plan.json}
board=${IC_BOARD:-docs/ic/leaderboard.json}
bad=0
for s in $(jq -r '.sources | to_entries[] | .value.path' "$board"); do
  d=$(dirname "$s")
  for sd in $(ls -d "$d"/sessions/*/ 2>/dev/null); do
    sd=${sd%/}
    [ -f "$sd/session.json" ] && [ -f "$sd/records.jsonl" ] || continue
    [ "$(jq -r .status "$sd/session.json")" = complete ] || continue
    jq -e --arg d "$sd" '.sessions[] | select(.dir == $d)' "$plan" > /dev/null || { echo "session $sd is cited by $s but is not in $plan"; bad=1; }
  done
done
[ $bad = 0 ] && echo "replay plan covers every complete session the board cites"
exit $bad
