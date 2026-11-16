#!/bin/bash
# card_queue.sh <worktree> <card-id>...: run each card with tools/task_card.py run, in order, from the worktree;
# per-card output to cards/<id>.launch.log; stop at the first non-zero exit. Orchestration only.
wt=$1; shift; cd "$wt" || exit 2
C=research/isogeny_class_ic_yield_20261102/cards
for id in "$@"; do
  echo "$(date -u +%FT%TZ) start $id"
  python3 tools/task_card.py run $C/$id.yaml > $C/$id.launch.log 2>&1 < /dev/null
  rc=$?; echo "$(date -u +%FT%TZ) end $id exit $rc"
  [ $rc -eq 0 ] || { echo "queue stopped at $id"; exit $rc; }
done
echo "queue done"
