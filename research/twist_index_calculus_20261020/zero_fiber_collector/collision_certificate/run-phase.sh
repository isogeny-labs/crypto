#!/usr/bin/env bash
# Invoked inside the shared reservation. Capture actual launch after queue wait.
set -Eeuo pipefail
cert_prefix=${1:?phase receipt prefix}
shift
test ! -e "$cert_prefix.actual-started-utc.txt"
date -u +%FT%TZ > "$cert_prefix.actual-started-utc.txt"
date -u +%s > "$cert_prefix.actual-started-unix-seconds.txt"
printf '1200\n' > "$cert_prefix.outer-budget-seconds.txt"
if test "${2:-}" = produce; then
  printf '1180\n' > "$cert_prefix.producer-cooperative-budget-seconds.txt"
else
  printf 'not-applicable\n' > "$cert_prefix.producer-cooperative-budget-seconds.txt"
fi
printf '%s\n' "$$" > "$cert_prefix.actual-launcher-pid.txt"
exec /usr/bin/time -v -o "$cert_prefix.time.txt" prlimit --as=8589934592 -- \
  timeout --signal=TERM --kill-after=10s 1200s "$@"
