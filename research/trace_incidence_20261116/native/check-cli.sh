#!/usr/bin/env bash
# Thin native CLI correctness checks; no timing data is extracted.
set -euo pipefail
native_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
flint_prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
make -s -C "$native_dir" FLINT_PREFIX="$flint_prefix" all
check_dir=$(mktemp -d "${TMPDIR:-/tmp}/paper-native-cli.XXXXXX")
printf '%s\n' "$check_dir"
cp "$native_dir/build/rootfinder" "$check_dir/qualifier-rootfinder"
cp "$native_dir/build/paper_example" "$check_dir/qualifier-example"
chmod 0555 "$check_dir/qualifier-rootfinder" "$check_dir/qualifier-example"
inputs=("$native_dir/api.hpp" "$native_dir/paper.hpp" "$native_dir/pullback.hpp"
  "$native_dir/isogeny.hpp" "$native_dir/translation.cpp" "$native_dir/paper.cpp"
  "$native_dir/pullback.cpp" "$native_dir/isogeny.cpp" "$native_dir/root_cli.cpp"
  "$native_dir/paper_example.cpp" "$native_dir/paper83.datum" "$native_dir/paper83.poly"
  "$check_dir/qualifier-rootfinder" "$check_dir/qualifier-example")
set -o noclobber
sha256sum "${inputs[@]}" > "$check_dir/pre-call-manifest.txt"
expected='[11,19,25,34,41,64,65,68,71,72]'
for method in dense trace pullback; do
  "$check_dir/qualifier-rootfinder" "$method" "$native_dir/paper83.datum" "$native_dir/paper83.poly" \
    > "$check_dir/$method-given.json"
  "$check_dir/qualifier-rootfinder" "$method" "$native_dir/paper83.datum" "$native_dir/paper83.poly" --two-isogeny \
    > "$check_dir/$method-isogeny.json"
  jq -e --argjson expected "$expected" '.roots==$expected and .isogeny_preprocessing==false' "$check_dir/$method-given.json" >/dev/null
  jq -e --argjson expected "$expected" '.roots==$expected and .isogeny_preprocessing==true and .preprocessing_counts["auxiliary.exhaustive_coordinate_pairs"]==13778' "$check_dir/$method-isogeny.json" >/dev/null
done
"$check_dir/qualifier-example" > "$check_dir/example.json"
jq -e --argjson expected "$expected" '.arms | all(.roots==$expected)' "$check_dir/example.json" >/dev/null
printf '2\n0 0 1\n' > "$check_dir/repeated.poly"
if "$check_dir/qualifier-rootfinder" pullback "$native_dir/paper83.datum" "$check_dir/repeated.poly" \
  > "$check_dir/repeated.stdout" 2> "$check_dir/repeated.stderr"; then
  printf 'Repeated input was unexpectedly accepted\n' >&2; exit 1
fi
sha256sum "${inputs[@]}" > "$check_dir/post-call-manifest.txt"
cmp "$check_dir/pre-call-manifest.txt" "$check_dir/post-call-manifest.txt"
printf '%s\n' '{"schema":"paper-native-cli-check/v1","status":"passed","pcc_standing":"unverified","rootfinder_calls":6,"structural_example_calls":1,"invalid_input_checks":1,"timings_collected":false}' > "$check_dir/result.json"
cat "$check_dir/result.json"
