#!/bin/sh
# Preserve an interrupted development tree and resume only the unfinished suffix.
set -eu
cd "$(dirname "$0")"
original=${1:?Specify interrupted recorded-kernel output directory}
index=${2:?Specify its recorded kernel 2..8}
case "$index" in 2|3|4|5|6|7|8) ;; *) exit 1 ;; esac
out=${3:?Specify a NEW output directory}
test ! -f "$original/COMPLETE.txt"
test ! -f "$original/replay/nist_cover_three_tests_20261116/evidence/descent-v1/post-call-source-manifest.sha256"
test -f "$original/replay/nist_cover_three_tests_20261116/evidence/riemann-v1/post-call-source-manifest.sha256"
test -f "$original/replay/nist_cover_three_tests_20261116/evidence/quotient-theta2-v1/post-call-source-manifest.sha256"
mkdir "$out"
out=$(cd "$out" && pwd)
cp -a "$original/replay" "$out/replay"
work="$out/replay/nist_cover_three_tests_20261116"
mv "$work/evidence/descent-v1" "$work/evidence/descent-interrupted-v1"
printf 'source=%s\nkernel=%s\nresume_stage=descent\nreason=process disappeared without completion or exit receipt\n' "$original" "$index" > "$out/RESUME_FROM.txt"
sha256sum "$original/replay/nist_cover_three_tests_20261116/evidence/riemann-v1/result.json" "$work/evidence/riemann-v1/result.json" "$original/replay/nist_cover_three_tests_20261116/evidence/quotient-theta2-v1/result.json" "$work/evidence/quotient-theta2-v1/result.json" > "$out/retained-predecessors.sha256"
cmp "$original/replay/nist_cover_three_tests_20261116/evidence/riemann-v1/result.json" "$work/evidence/riemann-v1/result.json"
cmp "$original/replay/nist_cover_three_tests_20261116/evidence/quotient-theta2-v1/result.json" "$work/evidence/quotient-theta2-v1/result.json"
stage=descent
finish() { status=$?; printf 'stage=%s\nexit_status=%s\n' "$stage" "$status" > "$out/EXIT_STATUS.txt"; }
trap finish EXIT
tmp=$(mktemp -d)
cd "$work"
./check-descent.sh evidence/descent-v1
printf '%s\n' 'Completed resumed curve descent'
stage=export
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . export_candidate.cpp -o "$tmp/export-candidate"
"$tmp/export-candidate" inputs/p192-d7-prym.json evidence/descent-v1/result.json > "evidence/p192-d7-kernel${index}-plane-quartic.json"
stage=relations; ./check-destination-relations.sh evidence/destination-relations-v2
stage=target-weil; ./check-target-weil.sh evidence/target-weil-v3
stage=differential; ./check-differential-twist.sh evidence/differential-twist-v2
stage=complete
printf 'PASS resumed recorded P-192 d7 kernel%s conditional geometry/function/differential chain. Source divisor map and DLP remain open.\n' "$index" > "$out/COMPLETE.txt"
cat "$out/COMPLETE.txt"
