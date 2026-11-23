#!/usr/bin/env bash
# Partial scalar replay needs only the small complete interrupted-run archive.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/ic_algebraic_crossover_20261023
mode=${1:-partial}
case "$mode" in partial|all) ;; *) printf '%s\n' 'partial | all' >&2; exit 2 ;; esac
cargo build --release --example ic_followup_publication_audit
bin_root=$(realpath "${CARGO_TARGET_DIR:-target}/release/examples")
scratch=$(mktemp -d /tmp/ic-publication-replay.XXXXXX)
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/$study" "$scratch/bin"
while read -r hash relative kind; do
  if [ "$mode" = partial ] && [ "$relative" != evidence/runs-partial-quartics-complete-20261026.tar.xz ]; then continue; fi
  actual=$(sha256sum "$study/$relative")
  test "${actual%% *}" = "$hash"
  if [ "$kind" = raw ]; then
    tar -xJf "$study/$relative" -C "$scratch/$study"
  else
    tar -xJf "$study/$relative" -C "$scratch/bin"
  fi
done < <(jq -r '.archives[] | [.sha256,.path,.kind] | @tsv' "$study/publication-followup-raw-archives.json")
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
expected="$study/pipeline-receipts/publication-followup-audit-v3-20261026/output.json"
if [ "$mode" = partial ]; then
  (ulimit -v 2097152; timeout -k 30 300 "$bin_root/ic_followup_publication_audit" partial "$scratch/$study") > "$scratch/actual.json"
  jq -c '.partial_quartics' "$expected" > "$scratch/expected.json"
  cmp "$scratch/actual.json" "$scratch/expected.json"
else
  read -r hash relative < <(jq -r '.archives[] | select(.kind == "binaries") | [.sha256,.path] | @tsv' "$study/wide-raw-archives.json")
  actual=$(sha256sum "$study/$relative")
  test "${actual%% *}" = "$hash"
  tar -xJf "$study/$relative" -C "$scratch/bin"
  export IC_PROFILE_BINARY_DIR="$scratch/bin"
  (ulimit -v 2097152; timeout -k 30 300 "$bin_root/ic_followup_publication_audit" "$scratch/$study") > "$scratch/actual.json"
  cmp "$scratch/actual.json" "$expected"
fi
printf '%s\n' 'PASS: native interrupted-session accounting and all 21 independent scalar checks reproduce byte for byte. Full mode also verifies original constructor profiles/binaries/controls/source-test receipts. No whole-session or full-IC qualification follows.'
