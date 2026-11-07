#!/usr/bin/env bash
# Credential-free development build/staging only. Never publishes or launches.
set -euo pipefail
[[ $# == 1 && $1 == /* ]] || { echo 'Usage: prepare-layer.sh /ABSOLUTE/FRESH/OUTPUT' >&2; exit 2; }
nist_stage_here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
nist_stage_repo=$(cd -- "$nist_stage_here/../../.." && pwd)
nist_stage_output=$1
nist_stage_pari=${PARI_ROOT:-/workspace/tools/sage_skeleton_env}
nist_stage_cxx=${CXX:-c++}
[[ $nist_stage_pari == /* && -f $nist_stage_pari/lib/libpari.a ]] || {
    echo 'PARI_ROOT must name an absolute installed static PARI prefix' >&2; exit 2;
}
# An existing output, including a failed attempt, is retained unchanged.
mkdir -- "$nist_stage_output"
mkdir -- "$nist_stage_output/build" "$nist_stage_output/receipts" "$nist_stage_output/layer"
nist_stage_receipts=$nist_stage_output/receipts
nist_stage_native=$nist_stage_repo/research/nist_skip_criterion_20261025/journal_qualification_20261104
nist_stage_study=$nist_stage_repo/research/nist_skip_criterion_20261025
nist_stage_flags=(-O2 -std=c++17 -Wall -Wextra -Werror -pedantic -static)
read -r -a nist_stage_dependencies <<< "$(pkg-config --static --libs libcrypto gmpxx gmp)"
nist_stage_link=("$nist_stage_pari/lib/libpari.a" "${nist_stage_dependencies[@]}" -lm -pthread)
"$nist_stage_cxx" --version > "$nist_stage_receipts/compiler.txt"
pkg-config --modversion libcrypto gmpxx gmp > "$nist_stage_receipts/pkg-versions.txt"
{
    printf 'CXX='; printf '%q ' "$nist_stage_cxx"; printf '\nFLAGS='; printf '%q ' "${nist_stage_flags[@]}"
    printf '\nLINK='; printf '%q ' "${nist_stage_link[@]}"; printf '\n'
} > "$nist_stage_receipts/build-command.txt"
for nist_stage_binary in journal_check sampler_resume journal_tests; do
    "$nist_stage_cxx" "${nist_stage_flags[@]}" -I"$nist_stage_pari/include" \
        "$nist_stage_native/$nist_stage_binary.cpp" -o "$nist_stage_output/build/$nist_stage_binary" \
        "${nist_stage_link[@]}"
    readelf -lW "$nist_stage_output/build/$nist_stage_binary" > "$nist_stage_receipts/$nist_stage_binary.elf-programs.txt"
    readelf -dW "$nist_stage_output/build/$nist_stage_binary" > "$nist_stage_receipts/$nist_stage_binary.elf-dynamic.txt"
    if rg -q 'INTERP|NEEDED' "$nist_stage_receipts/$nist_stage_binary.elf-"*.txt; then
        echo "Refused non-static binary: $nist_stage_binary" >&2; exit 1
    fi
done
"$nist_stage_cxx" "${nist_stage_flags[@]}" -I"$nist_stage_pari/include" \
    "$nist_stage_here/pari-datadir.cpp" -o "$nist_stage_output/build/pari-datadir" "${nist_stage_link[@]}"
nist_stage_data=$("$nist_stage_output/build/pari-datadir")
[[ $nist_stage_data == "$nist_stage_pari/share/pari" && -d $nist_stage_data/seadata ]] || {
    echo 'Compiled PARI data prefix differs from PARI_ROOT, or seadata is missing' >&2; exit 1;
}
printf '%s\n' "$nist_stage_data" > "$nist_stage_receipts/pari-datadir.txt"
mkdir -p -- "$nist_stage_output/layer$nist_stage_data"
cp -a -- "$nist_stage_data/seadata" "$nist_stage_output/layer$nist_stage_data/"
mkdir -p -- "$nist_stage_output/layer/opt/nist/bin" "$nist_stage_output/layer/opt/nist/source"
cp -- "$nist_stage_output/build/"{journal_check,sampler_resume,journal_tests} "$nist_stage_output/layer/opt/nist/bin/"
for nist_stage_relative in \
    journal_qualification_20261104/journal_check.cpp \
    journal_qualification_20261104/sampler_resume.cpp \
    journal_qualification_20261104/resume.hpp \
    journal_qualification_20261104/journal_tests.cpp \
    journal_qualification_20261104/test-cli.sh \
    random_curve_validation_20261029/sampler.cpp \
    strict_elkies_20261029/inputs.tsv; do
    mkdir -p -- "$nist_stage_output/layer/opt/nist/source/$(dirname -- "$nist_stage_relative")"
    cp -- "$nist_stage_study/$nist_stage_relative" "$nist_stage_output/layer/opt/nist/source/$nist_stage_relative"
done
(
    cd -- "$nist_stage_repo"
    sha256sum research/nist_skip_criterion_20261025/journal_qualification_20261104/{journal_check.cpp,sampler_resume.cpp,resume.hpp,journal_tests.cpp,test-cli.sh} \
        research/nist_skip_criterion_20261025/random_curve_validation_20261029/sampler.cpp \
        research/nist_skip_criterion_20261025/strict_elkies_20261029/inputs.tsv
) > "$nist_stage_receipts/SOURCE.sha256"
{
    sha256sum "$nist_stage_pari/lib/libpari.a"
    for nist_stage_library in libcrypto.a libgmpxx.a libgmp.a libz.a libzstd.a libstdc++.a libgcc.a libc.a libm.a; do
        nist_stage_library_path=$("$nist_stage_cxx" -print-file-name="$nist_stage_library")
        [[ -f $nist_stage_library_path ]] || { echo "Missing library: $nist_stage_library" >&2; exit 1; }
        sha256sum "$nist_stage_library_path"
    done
} > "$nist_stage_receipts/LIBRARIES.sha256"
(
    cd -- "$nist_stage_output/layer"
    find . -type f -print0 | sort -z | xargs -0 sha256sum
) > "$nist_stage_receipts/LAYER.sha256"
printf 'Staged native layer; nothing published or launched: %s\n' "$nist_stage_output"
