#!/usr/bin/env bash
set -eu
cd "$(git rev-parse --show-toplevel)"
study=research/hec_residual_span_20261116/anchor-rank
for f in CONTROL.tsv CONTROL.stderr FIXTURES.tsv PREFIX.tsv RANKS.tsv RUN.txt EXIT.txt OUTPUT.sha256; do test ! -e "$study/$f" || { echo "Refusing overwrite $f" >&2; exit 1; }; done
test -z "$(git status --porcelain -- research/hec_residual_span_20261116 crates/hec-native-ic)"
branch=$(git symbolic-ref --short HEAD)
head=$(git rev-parse HEAD)
test "$head" = "$(git ls-remote origin "refs/heads/$branch" | cut -f1)"
shasum -a 256 -c "$study/FREEZE.sha256"
printf 'execution_commit=%s\nbranch=%s\ncustody=git\nstarted_utc=%s\n' "$head" "$branch" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$study/RUN.txt"
set +e
/usr/bin/perl -e 'alarm 180; exec @ARGV' /Volumes/SSD990-2/hec-anchor-rank-target/release/anchor-rank > "$study/CONTROL.tsv" 2> "$study/CONTROL.stderr"
status=$?
set -e
printf '%s\n' "$status" > "$study/EXIT.txt"
printf 'ended_utc=%s\nexit=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$status" >> "$study/RUN.txt"
shasum -a 256 "$study/CONTROL.tsv" "$study/CONTROL.stderr" "$study/FIXTURES.tsv" "$study/PREFIX.tsv" "$study/RANKS.tsv" "$study/RUN.txt" "$study/EXIT.txt" > "$study/OUTPUT.sha256"
exit "$status"
