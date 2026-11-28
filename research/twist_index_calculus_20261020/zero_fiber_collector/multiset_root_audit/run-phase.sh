#!/usr/bin/env bash
set -Eeuo pipefail
multiset_prefix=${1:?fresh phase prefix}
shift
test ! -e "$multiset_prefix.actual-started-utc.txt"
date -u +%FT%TZ > "$multiset_prefix.actual-started-utc.txt"
date -u +%s > "$multiset_prefix.actual-started-unix-seconds.txt"
printf '%s\n' "$$" > "$multiset_prefix.actual-launcher-pid.txt"
printf '1200\n' > "$multiset_prefix.outer-budget-seconds.txt"
case "${2:-}" in decode|audit) printf '1180\n' ;; *) printf 'not-applicable\n' ;; esac > "$multiset_prefix.producer-cooperative-budget-seconds.txt"
exec /usr/bin/time -v -o "$multiset_prefix.time.txt" prlimit --as=8589934592 -- \
  timeout --signal=TERM --kill-after=10s 1200s "$@"
