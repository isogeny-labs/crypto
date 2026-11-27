#!/bin/bash
set -euo pipefail
role=${1:?build or native}; shift
case "$role" in build) limit=300;; native) limit=600;; *) exit 2;; esac
test -n "${P192_CAPTURE:?}"
counter="$P192_CAPTURE/counter"
exec 9> "$P192_CAPTURE/counter.lock"
flock 9
n=$(cat "$counter")
n=$((n+1))
printf '%s\n' "$n" > "$counter"
printf -v label '%04d-%s' "$n" "$role"
mkdir "$P192_CAPTURE/calls/$label"
flock -u 9
exec 9>&-
call="$P192_CAPTURE/calls/$label"
printf '%s\0' "$@" > "$call/argv.nul"
pwd > "$call/cwd.txt"
sha256sum "$1" > "$call/executable.sha256"
printf '%s\n' "$limit" > "$call/timeout-seconds.txt"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
set +e
(ulimit -v 1048576; { time timeout --kill-after=3 "$limit" taskset -c "$P192_CPU" "$@" > "$call/stdout" 2> "$call/stderr"; } 2> "$call/cost.json")
code=$?
set -e
printf '%s\n' "$code" > "$call/exit-code.txt"
cat "$call/stdout"
cat "$call/stderr" >&2
exit "$code"
