#!/bin/sh
# Fresh geometry chain: later stages consume this replay's earlier outputs.
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
work="$out/replay/nist_cover_three_tests_20261116"
mkdir -p "$work/inputs" "$work/evidence/final-kernel-check" "$out/replay/nist_cover_separator_20261115/data"
cp -p ./*.cpp ./*.hpp ./*.sh "$work/"
cp inputs/p192-d7-prym.json "$work/inputs/"
cp evidence/final-kernel-check/actual-cover-torsion.json evidence/final-kernel-check/kernels.jsonl "$work/evidence/final-kernel-check/"
cp ../nist_cover_separator_20261115/data/modules-192.json "$out/replay/nist_cover_separator_20261115/data/"
cd "$work"
./check-weil3.sh evidence/weil3-v2
./check-weil-chart.sh evidence/weil-chart-v1
./check-level2-input.sh evidence/level2-input-v1
./check-quotient-theta2.sh evidence/quotient-theta2-v1
./check-riemann.sh evidence/riemann-v1
./check-descent.sh evidence/descent-v1
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . export_candidate.cpp -o "$out/export-candidate"
sha256sum export_candidate.cpp "$out/export-candidate" > "$out/export-pre-call-manifest.sha256"
"$out/export-candidate" inputs/p192-d7-prym.json evidence/descent-v1/result.json > evidence/p192-d7-kernel1-plane-quartic.json
./check-destination-relations.sh evidence/destination-relations-v2
printf '%s\n' 'PASS fresh actual P-192 kernel1 geometry chain: normalized functions, special charts, level2 moduli, candidate quartic, Fp curve descent and destination relations. Serre twist/divisor map/DLP remain open.' > "$out/COMPLETE.txt"
cat "$out/COMPLETE.txt"
