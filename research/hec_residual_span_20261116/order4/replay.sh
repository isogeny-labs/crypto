#!/usr/bin/env bash
set -eu
cd "$(git rev-parse --show-toplevel)"
study=research/hec_residual_span_20261116/order4
for f in REPLAY.tsv REPLAY.stderr REPLAY.RUN.txt REPLAY.EXIT.txt REPLAY.OUTPUT.sha256;do test ! -e "$study/$f";done
test -z "$(git status --porcelain -- research/hec_residual_span_20261116)"
head=$(git rev-parse HEAD)
branch=$(git symbolic-ref --short HEAD)
test "$head" = "$(git ls-remote origin "refs/heads/$branch" | cut -f1)"
shasum -a 256 -c "$study/REPLAY.FREEZE.sha256"
printf 'execution_commit=%s\ncustody=git\nstarted_utc=%s\n' "$head" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$study/REPLAY.RUN.txt"
set +e
/usr/bin/perl -e 'alarm 600;exec @ARGV' /tmp/hec-order4-replay > "$study/REPLAY.tsv" 2> "$study/REPLAY.stderr"
status=$?
set -e
printf '%s\n' "$status" > "$study/REPLAY.EXIT.txt"
printf 'ended_utc=%s\nexit=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$status" >> "$study/REPLAY.RUN.txt"
shasum -a 256 "$study/REPLAY.tsv" "$study/REPLAY.stderr" "$study/REPLAY.RUN.txt" "$study/REPLAY.EXIT.txt" > "$study/REPLAY.OUTPUT.sha256"
exit "$status"
