#!/usr/bin/env bash
set -euo pipefail
# No network or dependencies. RUSTC may name a compiler; defaults to PATH.
study_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
compiler=${RUSTC:-rustc}
replay_dir=${1:-"$study_dir/replay"}
test ! -e "$replay_dir"
mkdir -p "$replay_dir"
"$compiler" --version --verbose > "$replay_dir/build.txt"
"$compiler" --edition=2021 -O --test "$study_dir/frozen/exact_bounds.rs" -o "$replay_dir/tests" >> "$replay_dir/build.txt" 2>&1
"$compiler" --edition=2021 -O "$study_dir/frozen/exact_bounds.rs" -o "$replay_dir/exact_bounds" >> "$replay_dir/build.txt" 2>&1
"$replay_dir/tests" --test-threads=1 > "$replay_dir/tests.txt"
"$replay_dir/exact_bounds" --certificate "$replay_dir/certificates.json" > "$replay_dir/bounds.tsv"
cmp "$study_dir/evidence/bounds.tsv" "$replay_dir/bounds.tsv"
cmp "$study_dir/evidence/certificates.json" "$replay_dir/certificates.json"
printf '%s\n' 'PASS: exact bounds and full rational certificates match the frozen evidence.'
