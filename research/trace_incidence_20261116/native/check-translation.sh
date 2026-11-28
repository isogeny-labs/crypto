#!/usr/bin/env bash
# Thin native build/correctness launcher. Deliberately records no timings.
set -euo pipefail
study_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
flint_prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
cxx=${CXX:-g++}
check_dir=$(mktemp -d "${TMPDIR:-/tmp}/paper-translation-check.XXXXXX")
printf '%s\n' "$check_dir"
"$cxx" --version > "$check_dir/compiler.txt"
"$cxx" -O2 -std=c++17 -Wall -Wextra -Werror \
    -I"$flint_prefix/include" -I"$flint_prefix/include/flint" \
    "$study_dir/translation.cpp" "$study_dir/translation_tests.cpp" \
    -L"$flint_prefix/lib/x86_64-linux-gnu" \
    -Wl,-rpath,"$flint_prefix/lib/x86_64-linux-gnu" \
    -lflint -lmpfr -lgmp -o "$check_dir/build-binary"
# MPFR precedes GMP to support extracted distributions with static MPFR.
sha256sum "$study_dir/api.hpp" "$study_dir/translation.cpp" \
    "$study_dir/translation_tests.cpp" "$check_dir/build-binary" \
    > "$check_dir/build-manifest.txt"
cp "$check_dir/build-binary" "$check_dir/qualifier"
chmod 0555 "$check_dir/qualifier"
# Unique directory plus noclobber refuses replacement of the pre-call record.
set -o noclobber
sha256sum "$study_dir/api.hpp" "$study_dir/translation.cpp" \
    "$study_dir/translation_tests.cpp" "$check_dir/qualifier" \
    > "$check_dir/pre-call-manifest.txt"
call_status=0
"$check_dir/qualifier" > "$check_dir/stdout.json" 2> "$check_dir/stderr.txt" || call_status=$?
sha256sum "$study_dir/api.hpp" "$study_dir/translation.cpp" \
    "$study_dir/translation_tests.cpp" "$check_dir/qualifier" \
    > "$check_dir/post-call-manifest.txt"
cmp "$check_dir/pre-call-manifest.txt" "$check_dir/post-call-manifest.txt"
cat "$check_dir/stdout.json"
if ((call_status)); then cat "$check_dir/stderr.txt" >&2; fi
exit "$call_status"
