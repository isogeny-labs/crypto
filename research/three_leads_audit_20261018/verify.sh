#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
for arm in binary odd cycle; do
  rustc --edition=2021 -D warnings -O "${arm}_check.rs" -o "$scratch/$arm"
  "$scratch/$arm" > "$scratch/$arm.csv"
  case "$arm" in
    binary|odd) reference="${arm}_results_v2.csv" ;;
    cycle) reference=cycle_results.csv ;;
  esac
  cmp "$reference" "$scratch/$arm.csv"
  printf 'PASS %s arithmetic assertions and deterministic CSV replay\n' "$arm"
done
root=$(git rev-parse --show-toplevel)
export ELAN_TOOLCHAIN=leanprover/lean4:v4.34.1
cd "$root"
tools/lean/proof-gate.sh --out "$scratch/lean-gate" \
  research/three_leads_audit_20261018/TraceKernel.lean
