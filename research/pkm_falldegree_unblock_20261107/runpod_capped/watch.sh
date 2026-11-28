#!/usr/bin/env bash
# watch.sh NAME MAX_WALL_S  -- run fetch.sh NAME every 900 s (Amendment 29's 15-minute fetch; owner decision 26) until
# fetch.sh reports a terminal state, the pod's hard deadline has passed and fetch.sh has acted on it, or MAX_WALL_S
# seconds have passed. Shell only. It deletes nothing itself: every collection and delete is fetch.sh's own, under its
# rules. The session runs it in the background in segments of at most 2 h (MAX_WALL_S <= 7200) and starts the next
# segment when one ends, committing and pushing the receipts between segments (decision N6).
# Env: RPC_WATCH_S overrides the 900 s interval downward (the offline tests use 1); a value above 900 is refused.
# Exit: 0 fetch.sh collected and deleted the pod (or the run was already finished: fetch.sh exit 9); 8 collected and
#       waiting for the git gate (commit, push, then fetch.sh NAME --delete REF); 12 fetch.sh needs the owner; 20 the
#       segment's MAX_WALL_S passed (start the next segment); 2 usage; any other fetch.sh refusal (2, 3) is passed on.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"
[ $# -eq 2 ] && [[ $2 =~ ^[0-9]+$ ]] && [ "$2" -ge 1 ] && [ "$2" -le 7200 ] || { echo "usage: watch.sh NAME MAX_WALL_S (1..7200)" >&2; exit 2; }
NAME=$1 MAXW=$2 IV=${RPC_WATCH_S:-900}
[[ $IV =~ ^[0-9]+$ ]] && [ "$IV" -ge 1 ] && [ "$IV" -le 900 ] || { echo "RPC_WATCH_S must be 1..900 (no wait longer than 900 s)" >&2; exit 2; }
REC=$(rec_dir "$NAME") || { echo "unknown name $NAME" >&2; exit 2; }
[ -d "$REC" ] || { echo "no receipts $REC" >&2; exit 2; }
end=$(( $(date +%s) + MAXW ))
wlog() { echo "$(now) watch $NAME: $*" | tee -a "$REC/watch.log" >&2; }
wlog "segment start: every $IV s, at most $MAXW s"
while :; do
  set +e; "$here/fetch.sh" "$NAME" </dev/null; rc=$?; set -e
  hard=$(jq -r '.hard_deadline // empty' "$REC/pod-state.json" 2>/dev/null || true)
  wlog "fetch.sh exit $rc"
  case $rc in
    0|9) wlog "terminal: pod collected and deleted (or already finished)"; exit 0 ;;
    8)   wlog "collected; waiting for the git gate: commit and push the kept tree, then fetch.sh $NAME --delete REF"; exit 8 ;;
    12)  wlog "fetch.sh needs the owner (unparsable receipt or disagreeing collections)"; exit 12 ;;
    41|6|7) ;;                                   # nothing to do yet, transfer check failed, or delete unconfirmed: retry
    *)   wlog "fetch.sh refused or failed (exit $rc): stopping"; exit "$rc" ;;
  esac
  if [ -n "$hard" ] && [ "$(date +%s)" -ge "$(epoch "$hard")" ]; then
    wlog "past the hard deadline $hard after a fetch.sh call that did not end the run (exit $rc): reap.sh or the next segment acts"; exit 7
  fi
  left=$(( end - $(date +%s) )); [ "$left" -gt 0 ] || { wlog "segment over (MAX_WALL_S $MAXW): start the next segment"; exit 20; }
  sleep $(( left < IV ? left : IV ))
  [ "$(date +%s)" -lt "$end" ] || { wlog "segment over (MAX_WALL_S $MAXW): start the next segment"; exit 20; }
done
