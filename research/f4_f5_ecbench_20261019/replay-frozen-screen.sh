#!/usr/bin/env bash
# Reproduce the pre-versioning L0 screen with its exact guarded source.
# Run with no other builds/editors using this checkout; timed jobs use their
# own immutable source trees. Sealed sessions are never changed.
set -euo pipefail
out=${1:?scratch output directory}; shift
study=research/f4_f5_ecbench_20261019
source_file=src/cryptanalysis/koblitz_groebner.rs
frozen="$study/frozen-adaptive-screen/koblitz_groebner.rs"
build=${CARGO_TARGET_DIR:-target}
mkdir -p "$out"
cp "$source_file" "$out/current-groebner.rs"
restore() {
  if cmp -s "$source_file" "$frozen"; then
    cp "$out/current-groebner.rs" "$source_file"
  elif ! cmp -s "$source_file" "$out/current-groebner.rs"; then
    echo 'source changed concurrently; preserving that edit' >&2
    return 1
  fi
}
trap restore EXIT
cp "$frozen" "$source_file"
CARGO_TARGET_DIR="$build/frozen-screen" cargo build --release --locked --bin ecbench > "$out/build.log" 2>&1
cp "$build/frozen-screen/release/ecbench" "$out/ecbench-frozen-screen"
restore
trap - EXIT
sha256sum "$frozen" "$out/ecbench-frozen-screen" > "$out/source-binary.sha256"
for session in "$@"; do
  name=$(basename "$session")
  RAYON_NUM_THREADS=1 "$out/ecbench-frozen-screen" verify --dir "$session" --replay-all --allow-interrupted --out "$out/$name.audit.json" --exit-code > "$out/$name.audit.log" 2>&1
done
