#!/usr/bin/env bash
# Three owned foreground-timeout paths; no cryptographic workload or memory stress.
set -euo pipefail
ROOT=$1
WORKER_WRAPPER=$2
printf '%s\n' "$$" > "$ROOT/fixture-shell.pid"
bash "$WORKER_WRAPPER" /bin/sleep 60 &
printf '%s\n' "$!" > "$ROOT/worker-timeout.pid"
timeout --foreground --signal=TERM --kill-after=5 60 taskset -c 4 /bin/sleep 60 &
printf '%s\n' "$!" > "$ROOT/replay-timeout.pid"
timeout --foreground --signal=TERM --kill-after=5 60 /bin/sleep 60 &
printf '%s\n' "$!" > "$ROOT/audit-timeout.pid"
printf '%s\n' 'fixture launched; controller must verify all children before signaling' > "$ROOT/fixture-ready.txt"
wait
