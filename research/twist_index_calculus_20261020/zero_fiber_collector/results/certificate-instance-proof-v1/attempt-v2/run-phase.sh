#!/usr/bin/env bash
set -Eeuo pipefail
probe_prefix=${1:?phase receipt prefix}
shift
test ! -e "$probe_prefix.actual-started-utc.txt"
date -u +%FT%TZ > "$probe_prefix.actual-started-utc.txt"
date -u +%s > "$probe_prefix.actual-started-unix-seconds.txt"
printf '%s\n' "$$" > "$probe_prefix.actual-launcher-pid.txt"
printf '1200\n' > "$probe_prefix.outer-budget-seconds.txt"
if test "${2:-}" = produce; then
  printf '1180\n' > "$probe_prefix.producer-cooperative-budget-seconds.txt"
else
  printf 'not-applicable\n' > "$probe_prefix.producer-cooperative-budget-seconds.txt"
fi
exec /usr/bin/time -v -o "$probe_prefix.time.txt" prlimit --as=8589934592 -- \
  timeout --signal=TERM --kill-after=10s 1200s "$@"
