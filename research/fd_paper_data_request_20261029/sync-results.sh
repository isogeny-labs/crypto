#!/usr/bin/env bash
# Copy finished records in runs/<stage>/ to the results branch, in a separate
# worktree so the measuring tree is never checked out under run.sh (the
# dreg A3 pattern). Only finished records travel: *.jsonl (never *.part),
# their *.err logs, host.txt, plan.txt and queue.log.
#   sync-results.sh BRANCH MESSAGE
set -euo pipefail
B=$1; MSG=$2
HERE=$(cd "$(dirname "$0")" && pwd); REPO=$(cd "$HERE/../.." && pwd)
D=${HERE#"$REPO"/}; W=${RUNNER_TEMP:-/tmp}/fd-results-wt
cd "$REPO"
git fetch -q origin "$B" 2>/dev/null || true
if [ ! -d "$W/.git" ] && [ ! -f "$W/.git" ]; then
  if git rev-parse -q --verify "origin/$B" >/dev/null; then git worktree add -f --detach "$W" "origin/$B" >/dev/null
  else git worktree add -f --detach "$W" HEAD >/dev/null; fi
elif git rev-parse -q --verify "origin/$B" >/dev/null; then git -C "$W" reset -q --hard "origin/$B"; fi
mkdir -p "$W/$D/runs"
cp -p "$HERE/runs/queue.log" "$W/$D/runs/" 2>/dev/null || true
for s in "$HERE"/runs/*/; do
  [ -d "$s" ] || continue
  t="$W/$D/runs/$(basename "$s")"; mkdir -p "$t"
  find "$s" -maxdepth 1 -type f \( -name '*.jsonl' -o -name '*.err' -o -name host.txt -o -name plan.txt \) -exec cp -p {} "$t/" \;
done
git -C "$W" add "$D/runs"
git -C "$W" -c user.name="github-actions[bot]" -c user.email="41898282+github-actions[bot]@users.noreply.github.com" \
  commit -q -m "$MSG" || { echo "sync-results: nothing new"; exit 0; }
for i in 1 2 3 4; do git -C "$W" push -q origin "HEAD:refs/heads/$B" && exit 0; sleep $((2 ** i)); git -C "$W" pull -q --rebase origin "$B" || true; done
echo "sync-results: push failed" >&2; exit 1
