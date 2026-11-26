#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
test ! -e "$1"
mkdir -p "$(dirname "$1")"
mkdir "$1"
export P192_CAPTURE
P192_CAPTURE=$(cd "$1" && pwd)
out=$P192_CAPTURE
export P192_COMPILER P192_CPU CXX
P192_COMPILER=$(command -v c++)
P192_CPU=$(python3 -c 'import os; print(min(os.sched_getaffinity(0)))')
CXX="$out/instrumentation/compiler.sh"
mkdir "$out/instrumentation" "$out/calls" "$out/bin" "$out/source"
printf '0\n' > "$out/counter"
cp meter.sh compiler.sh run.sh PROTOCOL.md "$out/instrumentation/"
chmod 0555 "$out/instrumentation/"*.sh
mkdir -p "$out/source/nist_cover_three_tests_20261116/inputs" "$out/source/nist_cover_three_tests_20261116/evidence/final-kernel-check" "$out/source/nist_cover_separator_20261115/data"
original=$(cd .. && pwd)
source_copy="$out/source/nist_cover_three_tests_20261116"
cp "$original/"*.cpp "$original/"*.hpp "$original/"*.sh "$original/"*.md "$source_copy/"
cp "$original/inputs/p192-d7-prym.json" "$source_copy/inputs/"
cp "$original/evidence/final-kernel-check/actual-cover-torsion.json" "$original/evidence/final-kernel-check/kernels.jsonl" "$source_copy/evidence/final-kernel-check/"
cp "$original/../nist_cover_separator_20261115/data/modules-192.json" "$out/source/nist_cover_separator_20261115/data/"
find "$out/source" "$out/instrumentation" -type f -print0 | sort -z | xargs -0 sha256sum > "$out/pre-call-manifest.sha256"
git rev-parse HEAD > "$out/source-commit.txt"
uname -a > "$out/os.txt"
"$P192_COMPILER" --version > "$out/compiler-version.txt"
printf 'cpu=%s\nvirtual_memory_kib=1048576\nbuild_timeout_seconds=300\nnative_timeout_seconds=600\n' "$P192_CPU" > "$out/resource-envelope.txt"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
work() {
    bash "$source_copy/check-recorded-kernel.sh" 3 "$out/construction"
    root="$out/construction/replay/nist_cover_three_tests_20261116"
    # The selector derivation copies source/scripts only; the original downstream
    # runners additionally bind unchanged protocol files by name.
    cp "$source_copy/"*.md "$root/"
    bash "$root/check-elliptic-backmap.sh" "$root" "$out/backmap"
    bash "$root/check-formal-series.sh" "$root" "$out/backmap/result.json" "$out/formal"
    bash "$root/check-global-correspondence-v2.sh" "$root" "$out/formal/result.json" "$out/global"
}
set +e
{ time (set -e; work); } > "$out/stdout.txt" 2> "$out/wrapper-stderr-and-cost.txt"
code=$?
set -e
printf '%s\n' "$code" > "$out/exit-code.txt"
sha256sum -c "$out/pre-call-manifest.sha256" > "$out/unchanged-inputs.txt"
exit "$code"
