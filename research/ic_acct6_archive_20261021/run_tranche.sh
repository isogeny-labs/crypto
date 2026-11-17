#!/usr/bin/env bash
# Re-run frozen specs of the archive unchanged (PROTOCOL.md), one session at a
# time. Counts only: L0, `--cpus 1 --allow-busy`, one ecbench lock per session.
# Each run and its audit hold the host's shared build lock (LOCK) for that
# session only, so other agents can build between sessions.
#
#   research/ic_acct6_archive_20261021/run_tranche.sh <name>...
#
# `specs/<name>.json` (a byte copy of the committed session's spec.json) runs
# into `sessions/<name>/` (which must not exist) and is audited with
# `ecbench verify --replay 2 --exit-code` into `sessions/<name>/verify.json`.
# Exit status is nonzero if any run or audit failed; the logs say which.
set -uo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
bin="$root/target/release/ecbench"
LOCK=${LOCK:-/tmp/claude-0/-home-user/0175cce0-2ca5-5248-8a75-97acf3a20a50/scratchpad/build.lock}
[ -x "$bin" ] || { echo "missing $bin" >&2; exit 2; }
mkdir -p "$here/sessions" "$here/logs"
# DEADLINE (unix seconds): no session is launched after it (PROTOCOL.md, "First tranche").
DEADLINE=${DEADLINE:-0}
status=0
for name in "$@"; do
  out="$here/sessions/$name"
  if [ "$DEADLINE" -gt 0 ] && [ "$(date +%s)" -ge "$DEADLINE" ]; then echo "BUDGET SPENT, NOT LAUNCHED $name"; continue; fi
  [ -e "$out" ] && { echo "EXISTS $name"; status=1; continue; }
  echo "START $name $(date -u +%FT%TZ)"
  if ! flock "$LOCK" "$bin" run --spec "$here/specs/$name.json" --out "$out" --cpus 1 \
      --allow-busy --lock "$here/logs/$name.lock" --quiet >"$here/logs/$name.run.log" 2>&1; then
    echo "RUN FAILED $name $(date -u +%FT%TZ)"; status=1; continue
  fi
  if ! flock "$LOCK" "$bin" verify --dir "$out" --replay 2 --exit-code --out "$out/verify.json" \
      >"$here/logs/$name.verify.log" 2>&1; then
    echo "VERIFY FAILED $name $(date -u +%FT%TZ)"; status=1; continue
  fi
  echo "DONE $name $(date -u +%FT%TZ)"
done
exit $status
