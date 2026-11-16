#!/usr/bin/env bash
set -euo pipefail
affine_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$affine_root"
affine_out=${1:?expected NEW output directory}
test ! -e "$affine_out"
sha256sum --check research/curve25519_torus_native_20261115/affine-closure-v1/SOURCES.sha256
printf '%s  %s\n' '54ad26145290b1228e4f45c6669176ae4b79ca17c7f187441bc9b6c2b7d1de7d' '/workspace/curve25519-native-binaries/affine-closure-control-v1' | sha256sum --check -
mkdir "$affine_out"
sha256sum --check research/curve25519_torus_native_20261115/affine-closure-v1/FILE_INTERFACE_INPUTS.sha256 > "$affine_out/input-check.log" 2>&1
while IFS=$'\t' read -r affine_id affine_expected affine_request affine_certificate; do
  set +e
  /workspace/curve25519-native-binaries/affine-closure-control-v1 verify "$affine_request" "$affine_certificate" > "$affine_out/$affine_id.stdout" 2> "$affine_out/$affine_id.stderr"
  affine_status=$?
  set -e
  printf '%s\t%s\t%s\n' "$affine_id" "$affine_status" "$affine_expected" >> "$affine_out/RESULTS.tsv"
  if [ "$affine_status" -ne "$affine_expected" ]; then
    printf 'unexpected native interface exit for %s: %s\n' "$affine_id" "$affine_status" >&2
    exit 1
  fi
done < research/curve25519_torus_native_20261115/affine-closure-v1/FILE_INTERFACE_PLAN.tsv
