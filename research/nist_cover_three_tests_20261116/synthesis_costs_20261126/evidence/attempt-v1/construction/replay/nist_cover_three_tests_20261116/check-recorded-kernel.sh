#!/bin/sh
set -eu
cd "$(dirname "$0")"
index=${1:?Specify recorded kernel 2..8}
case "$index" in 2|3|4|5|6|7|8) ;; *) echo 'ERROR: recorded kernel 2..8 required' >&2; exit 1 ;; esac
out=${2:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
work="$out/replay/nist_cover_three_tests_20261116"
mkdir -p "$work/inputs" "$work/evidence/final-kernel-check" "$out/replay/nist_cover_separator_20261115/data"
stage=prepare
finish() { status=$?; printf 'stage=%s\nexit_status=%s\n' "$stage" "$status" > "$out/EXIT_STATUS.txt"; }
trap finish EXIT
tmp=$(mktemp -d)
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . prepare_kernel.cpp -o "$tmp/prepare-kernel" 2> "$out/prepare-build.stderr"
sha256sum prepare_kernel.cpp ./*.cpp ./*.hpp ./*.sh "$tmp/prepare-kernel" > "$out/original-source-manifest.sha256"
"$tmp/prepare-kernel" "$PWD" "$work" "$index" > "$out/source-derivation.json" 2> "$out/prepare.stderr"
cp inputs/p192-d7-prym.json "$work/inputs/"
cp evidence/final-kernel-check/actual-cover-torsion.json evidence/final-kernel-check/kernels.jsonl "$work/evidence/final-kernel-check/"
cp ../nist_cover_separator_20261115/data/modules-192.json "$out/replay/nist_cover_separator_20261115/data/"
cd "$work"
sha256sum ./*.cpp ./*.hpp ./*.sh selected-kernel.json inputs/* evidence/final-kernel-check/* > "$out/derived-source-manifest.sha256"
stage=weil3; ./check-weil3.sh evidence/weil3-v2
printf '%s\n' 'Completed normalized Weil3 functions'
stage=charts; ./check-weil-chart.sh evidence/weil-chart-v1
printf '%s\n' 'Completed special charts'
stage=level2; ./check-level2-input.sh evidence/level2-input-v1
stage=theta; ./check-quotient-theta2.sh evidence/quotient-theta2-v1
printf '%s\n' 'Completed quotient theta values'
stage=riemann; ./check-riemann.sh evidence/riemann-v1
stage=descent; ./check-descent.sh evidence/descent-v1
printf '%s\n' 'Completed curve reconstruction and descent'
stage=export
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . export_candidate.cpp -o "$tmp/export-candidate"
"$tmp/export-candidate" inputs/p192-d7-prym.json evidence/descent-v1/result.json > "evidence/p192-d7-kernel${index}-plane-quartic.json"
stage=relations; ./check-destination-relations.sh evidence/destination-relations-v2
stage=target-weil; ./check-target-weil.sh evidence/target-weil-v3
printf '%s\n' 'Completed target Weil functions'
stage=differential; ./check-differential-twist.sh evidence/differential-twist-v2
stage=complete
printf 'PASS recorded P-192 d7 kernel%s conditional geometry/function/differential chain. Actual divisor map and DLP remain open.\n' "$index" > "$out/COMPLETE.txt"
cat "$out/COMPLETE.txt"
