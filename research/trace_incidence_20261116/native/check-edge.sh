#!/usr/bin/env bash
# Bounded development input extension; no new timing comparison.
set -euo pipefail
native_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
flint_prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
check_dir=$(mktemp -d "${TMPDIR:-/tmp}/paper-native-edge.XXXXXX")
printf '%s\n' "$check_dir"
"${CXX:-g++}" -O2 -std=c++17 -Wall -Wextra -Werror -I"$flint_prefix/include" \
  "$native_dir/translation.cpp" "$native_dir/paper.cpp" "$native_dir/pullback.cpp" \
  "$native_dir/edge_audit.cpp" -L"$flint_prefix/lib/x86_64-linux-gnu" \
  -Wl,-rpath,"$flint_prefix/lib/x86_64-linux-gnu" -lflint -lmpfr -lgmp -o "$check_dir/build-binary"
cp "$check_dir/build-binary" "$check_dir/qualifier"
chmod 0555 "$check_dir/qualifier"
inputs=("$native_dir/api.hpp" "$native_dir/paper.hpp" "$native_dir/pullback.hpp"
  "$native_dir/translation.cpp" "$native_dir/paper.cpp" "$native_dir/pullback.cpp"
  "$native_dir/audit.cpp" "$native_dir/edge_audit.cpp" "$check_dir/qualifier")
set -o noclobber
sha256sum "${inputs[@]}" > "$check_dir/pre-call-manifest.txt"
call_status=0
"$check_dir/qualifier" > "$check_dir/result.json" 2> "$check_dir/stderr.txt" || call_status=$?
printf '%s\n' "$call_status" > "$check_dir/qualifier-exit-status.txt"
sha256sum "${inputs[@]}" > "$check_dir/post-call-manifest.txt"
cmp "$check_dir/pre-call-manifest.txt" "$check_dir/post-call-manifest.txt"
cat "$check_dir/result.json"
if ((call_status)); then cat "$check_dir/stderr.txt" >&2; fi
exit "$call_status"
