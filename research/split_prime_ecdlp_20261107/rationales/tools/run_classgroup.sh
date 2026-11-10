#!/usr/bin/env bash
# Exact class groups of the P-192 Frobenius fields with the library's native
# relation method (crypto::isogeny::class_group_subexp, binary `classgroup`).
# Idempotent and resumable: a label whose result file exists is skipped, and a
# label interrupted by a container restart resumes from its checkpoint
# (data/cl/rust_<label>.ckpt for the relation pool, .det for finished
# determinants). Re-run this script after any restart.
# Usage: tools/run_classgroup.sh [label ...]   (default: the five P-192 census curves)
# Env: CLASSGROUP_BIN, THREADS (4), SEED (1), BOUND, HALF, SUPPORT.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../.." && pwd)"
BIN="${CLASSGROUP_BIN:-$ROOT/target/release/classgroup}"
INPUTS="$ROOT/research/nist_skip_criterion_20261025/inputs.tsv"
OUT="$HERE/../data/cl"
mkdir -p "$OUT"
labels=("$@")
if [ ${#labels[@]} -eq 0 ]; then labels=(p192-139 p192-2198 p192-5228 p192-5405 p192-6013 p224-7026 p224-777 p224-1485 p224-5736 p239-2185 p239-2320 p239-3048 p239-36 p239-1562 p239-2491 p239-2682 p239-2863 p256-1842 p256-876); fi
for lab in "${labels[@]}"; do
  if [ -s "$OUT/rust_${lab}.json" ]; then echo "$lab: done (result exists)"; continue; fi
  line="$(grep -P "^${lab}\t" "$INPUTS")"
  p="$(printf '%s' "$line" | cut -f6)"
  n="$(printf '%s' "$line" | cut -f8)"
  # the fundamental discriminant's prime factors (PARI, facts stage) spare the binary a
  # factorisation Pollard-Brent cannot do at 226 bits and above; verified by the binary
  dkline="$(grep -P "^${lab}\t" "$OUT/dk_table.tsv" || true)"
  extra=()
  if [ -n "$dkline" ]; then extra=(--disc-primes "$(printf '%s' "$dkline" | cut -f4)" --conductor "$(printf '%s' "$dkline" | cut -f3)"); fi
  echo "$lab: starting $(date -u +%FT%TZ)"
  # tuned on the P-224 probes of 2026-11-10: a 2^16 half-interval gives 1.4x the
  # relation rate of 2^14 (2^18 no more); P-192 keeps the 2^14 of its runs
  half="${HALF:-65536}"; case "$lab" in p192-*) half="${HALF:-16384}";; esac
  "$BIN" --label "$lab" --p "$p" --n "$n" "${extra[@]}" --threads "${THREADS:-4}" --seed "${SEED:-1}" \
    ${BOUND:+--bound "$BOUND"} --half-length "$half" ${SUPPORT:+--support "$SUPPORT"} ${LARGE_PRIME:+--large-prime "$LARGE_PRIME"} \
    --checkpoint "$OUT/rust_${lab}.ckpt" --sub-bounds 50,163,691,2000 \
    --out "$OUT/rust_${lab}.json" 2>> "$OUT/rust_${lab}.log" > /dev/null
  echo "$lab: $(grep -o '"h_bits": [0-9]*\|"ratio_to_analytic": [0-9.]*\|"accepted": [a-z]*' "$OUT/rust_${lab}.json" | tr '\n' ' ') $(date -u +%FT%TZ)"
done
