#!/usr/bin/env bash
# Thin development-only launcher: no timings or benchmark comparisons.
set -euo pipefail
native_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
flint_prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
cxx=${CXX:-g++}
check_dir=$(mktemp -d "${TMPDIR:-/tmp}/paper-native-check.XXXXXX")
printf '%s\n' "$check_dir"
"$cxx" --version > "$check_dir/compiler.txt"
"$cxx" -O2 -std=c++17 -Wall -Wextra -Werror \
  -I"$flint_prefix/include" \
  "$native_dir/translation.cpp" "$native_dir/paper.cpp" \
  "$native_dir/pullback.cpp" "$native_dir/audit.cpp" \
  -L"$flint_prefix/lib/x86_64-linux-gnu" \
  -Wl,-rpath,"$flint_prefix/lib/x86_64-linux-gnu" \
  -lflint -lmpfr -lgmp -o "$check_dir/build-binary"
inputs=("$native_dir/api.hpp" "$native_dir/paper.hpp" "$native_dir/pullback.hpp"
        "$native_dir/translation.cpp" "$native_dir/paper.cpp"
        "$native_dir/pullback.cpp" "$native_dir/audit.cpp")
sha256sum "${inputs[@]}" "$check_dir/build-binary" > "$check_dir/build-manifest.txt"
cp "$check_dir/build-binary" "$check_dir/qualifier"
chmod 0555 "$check_dir/qualifier"
set -o noclobber
sha256sum "${inputs[@]}" "$check_dir/qualifier" > "$check_dir/pre-call-manifest.txt"
call_status=0
"$check_dir/qualifier" "$native_dir" "$check_dir/result.json" \
  > "$check_dir/stdout.json" 2> "$check_dir/stderr.txt" || call_status=$?
sha256sum "${inputs[@]}" "$check_dir/qualifier" > "$check_dir/post-call-manifest.txt"
cmp "$check_dir/pre-call-manifest.txt" "$check_dir/post-call-manifest.txt"
cat "$check_dir/stdout.json"
if ((call_status)); then cat "$check_dir/stderr.txt" >&2; fi
exit "$call_status"
