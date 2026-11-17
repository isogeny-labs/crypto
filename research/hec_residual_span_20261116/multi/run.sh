#!/usr/bin/env bash
set -eu
cd "$(git rev-parse --show-toplevel)"
case "${1:-}" in
  quadratic) study=research/hec_degree2_lp_20261116; binary=/Volumes/SSD990-2/hec-degree2-lp-target/release/degree2-lp-gate ;;
  span) study=research/hec_residual_span_20261116/multi; binary=/Volumes/SSD990-2/hec-multi-orbit-target/release/multi-orbit-gate
    prior=research/hec_degree2_lp_20261116/EXIT.txt
    test -f "$prior" && test "$(cat "$prior")" = 0 || { echo 'Original gate must succeed first' >&2; exit 1; } ;;
  *) echo 'Usage: git-resume.sh quadratic|span' >&2; exit 2 ;;
esac
for output in CONTROL.tsv CONTROL.stderr PREFIX.tsv FIXTURES.tsv EXIT.txt RUN.txt OUTPUT.sha256; do
  test ! -e "$study/$output" || { echo "Refusing overwrite: $study/$output" >&2; exit 1; }
done
paths=(research/hec_degree2_lp_20261116 research/hec_residual_span_20261116 crates/hec-native-ic)
test -z "$(git status --porcelain -- "${paths[@]}")" || { echo 'Scientific/custody paths must be committed and clean' >&2; exit 1; }
branch=$(git symbolic-ref --quiet --short HEAD)
head=$(git rev-parse HEAD)
remote=$(git ls-remote --exit-code origin "refs/heads/$branch" | cut -f1)
test "$head" = "$remote" || { echo 'Current source checkpoint must be pushed before execution' >&2; exit 1; }
shasum -a 256 -c research/hec_degree2_lp_20261116/FREEZE.sha256 > /tmp/hec-git-quadratic-freeze.txt
shasum -a 256 -c research/hec_residual_span_20261116/FREEZE.sha256 > /tmp/hec-git-span-freeze.txt
shasum -a 256 -c research/hec_residual_span_20261116/v2/FREEZE.sha256 > /tmp/hec-git-span-v2-freeze.txt
shasum -a 256 -c research/hec_residual_span_20261116/subgroup/FREEZE.sha256 > /tmp/hec-subgroup-freeze.txt
shasum -a 256 -c research/hec_residual_span_20261116/target/FREEZE.sha256 > /tmp/hec-target-freeze.txt
shasum -a 256 -c research/hec_residual_span_20261116/orbit/FREEZE.sha256 > /tmp/hec-orbit-freeze.txt
shasum -a 256 -c research/hec_residual_span_20261116/frobenius/FREEZE.sha256 > /tmp/hec-frobenius-freeze.txt
shasum -a 256 -c research/hec_residual_span_20261116/genera/FREEZE.sha256 > /tmp/hec-genera-freeze.txt
shasum -a 256 -c research/hec_residual_span_20261116/multi/FREEZE.sha256 > /tmp/hec-multi-freeze.txt
printf 'execution_commit=%s\nbranch=%s\ncustody=git\nstarted_utc=%s\n' "$head" "$branch" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$study/RUN.txt"
set +e
/usr/bin/perl -e 'alarm 180; exec @ARGV' "$binary" > "$study/CONTROL.tsv" 2> "$study/CONTROL.stderr"
status=$?
set -e
printf '%s\n' "$status" > "$study/EXIT.txt"
printf 'ended_utc=%s\nexit=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$status" >> "$study/RUN.txt"
shasum -a 256 "$study/CONTROL.tsv" "$study/CONTROL.stderr" "$study/EXIT.txt" "$study/RUN.txt" > "$study/OUTPUT.sha256"
if [ -e "$study/PREFIX.tsv" ]; then shasum -a 256 "$study/PREFIX.tsv" >> "$study/OUTPUT.sha256"; fi
if [ -e "$study/FIXTURES.tsv" ]; then shasum -a 256 "$study/FIXTURES.tsv" >> "$study/OUTPUT.sha256"; fi
exit "$status"
