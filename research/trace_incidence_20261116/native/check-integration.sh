#!/usr/bin/env bash
# Thin native development check. The script scope contains noclobber; the
# calling tool's shell/traps are unaffected. No timing is collected.
set -euo pipefail
native_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
flint_prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
cxx=${CXX:-g++}
check_dir=$(mktemp -d "${TMPDIR:-/tmp}/paper-native-integration.XXXXXX")
printf '%s\n' "$check_dir"
"$cxx" --version > "$check_dir/compiler.txt"
sources=(translation.cpp paper.cpp pullback.cpp isogeny.cpp incidence.cpp integration_audit.cpp)
source_paths=()
for file in "${sources[@]}"; do source_paths+=("$native_dir/$file"); done
"$cxx" -O2 -std=c++17 -Wall -Wextra -Werror -I"$flint_prefix/include" \
  "${source_paths[@]}" -L"$flint_prefix/lib/x86_64-linux-gnu" \
  -Wl,-rpath,"$flint_prefix/lib/x86_64-linux-gnu" \
  -lflint -lmpfr -lgmp -o "$check_dir/build-binary"
inputs=("${source_paths[@]}" "$native_dir/api.hpp" "$native_dir/paper.hpp"
  "$native_dir/pullback.hpp" "$native_dir/isogeny.hpp" "$native_dir/incidence.hpp"
  "$native_dir/audit.cpp")
sha256sum "${inputs[@]}" "$check_dir/build-binary" > "$check_dir/build-manifest.txt"
cp "$check_dir/build-binary" "$check_dir/qualifier"
chmod 0555 "$check_dir/qualifier"
set -o noclobber
sha256sum "${inputs[@]}" "$check_dir/qualifier" > "$check_dir/pre-call-manifest.txt"
call_status=0
timeout 60 "$check_dir/qualifier" > "$check_dir/result.json" 2> "$check_dir/stderr.txt" || call_status=$?
printf '%s\n' "$call_status" > "$check_dir/qualifier-exit-status.txt"
sha256sum "${inputs[@]}" "$check_dir/qualifier" > "$check_dir/post-call-manifest.txt"
cmp "$check_dir/pre-call-manifest.txt" "$check_dir/post-call-manifest.txt"
cat "$check_dir/result.json"
if ((call_status)); then cat "$check_dir/stderr.txt" >&2; fi
exit "$call_status"
