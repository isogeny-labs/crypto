#!/usr/bin/env bash
# Complete local artifacts are required; no pending manifest is treated as uploaded.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/ic_algebraic_crossover_20261023
cargo build --release --example ic_quartic_wide_audit --example ic_constructor_identity
bin_root=$(realpath "${CARGO_TARGET_DIR:-target}/release/examples")
scratch=$(mktemp -d /tmp/ic-wide-core-replay.XXXXXX)
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/$study" "$scratch/bin"
while read -r hash relative kind; do
  actual=$(sha256sum "$study/$relative")
  test "${actual%% *}" = "$hash"
  if [ "$kind" = raw ]; then
    tar -xJf "$study/$relative" -C "$scratch/$study"
  else
    tar -xJf "$study/$relative" -C "$scratch/bin"
  fi
done < <(jq -r '.archives[] | [.sha256,.path,.kind] | @tsv' "$study/wide-raw-archives.json")
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 IC_PROFILE_BINARY_DIR="$scratch/bin"
meta="$scratch/$study/pipeline-receipts/wide-core-audit-20261025"
"$bin_root/ic_quartic_wide_audit" profile "$scratch/$study" > "$scratch/profile-audit.json"
cmp "$scratch/profile-audit.json" "$meta/constructor-profile-audit.json"
for m in 16 17 18; do
  (ulimit -v 12582912; timeout -k 30 900 "$bin_root/ic_quartic_wide_audit" core "$m") > "$scratch/m$m.json"
  cmp "$scratch/m$m.json" "$meta/m$m/output.json"
done
"$bin_root/ic_constructor_identity" > "$scratch/identity.json"
cmp "$scratch/identity.json" "$study/constructor-difference.identity.json"
printf '%s\n' 'PASS: raw/binary archive hashes, both frozen constructor binaries, raw instruction totals, same field output, all three full-width order/arithmetic/encoding audits and the Lean-linked identity certificate reproduce. This is no native speed or full IC/DLP qualification.'
