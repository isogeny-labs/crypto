#!/usr/bin/env bash
set -euo pipefail

STAMP="${1:-$(date -u +%Y%m%dT%H%M%SZ)}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
REPO="$(cd "$ROOT/.." && pwd)"
OUT="$REPO/docs/ic/runs/pin-${STAMP}"
PIN_PY="$REPO/docs/ic/tools/solver_unit_pin_calibration.py"
CAL="$REPO/docs/ic/calibration.json"
IC="${IC_BIN:-ic}"

mkdir -p "$OUT"
HASHED_COMMON=(
  --factor-base hashed-affine
  --oracle hashed-tensor
  --targets walk
  --max-trials 50000
  --repeats 1
)

for pid in $(pgrep -f "ic bench.*hashed-tensor.*--bits 20.*--seed 1" || true); do
  if [[ "$pid" -ne "$$" ]]; then
    echo "waiting for existing 20-bit seed-1 bench pid $pid..."
    while kill -0 "$pid" 2>/dev/null; do sleep 30; done
  fi
done
if [[ -s /tmp/ic-hashed-20-s1.json ]]; then
  cp /tmp/ic-hashed-20-s1.json "$OUT/hashed-tensor-bench-20bit-s1.json"
  echo "reused /tmp/ic-hashed-20-s1.json for seed 1"
fi

for bits_slug in "20:bench-20bit" "24:generated-24bit"; do
  bits="${bits_slug%%:*}"
  slug="${bits_slug#*:}"
  for seed in 1 2 3 4 5; do
    out="$OUT/hashed-tensor-${slug}-s${seed}.json"
    if [[ -s "$out" ]]; then
      echo "skip $out"
      continue
    fi
    echo "=== hashed ${bits}-bit seed ${seed} $(date -u +%H:%M:%S) ==="
    "$IC" bench --json "${HASHED_COMMON[@]}" --bits "$bits" --seed "$seed" >"$out"
  done
done

echo "=== dry pin ===" | tee "$OUT/dry-pin.log"
for row_slug in "prime/bench-20bit:bench-20bit" "prime/generated-24bit-10935329:generated-24bit"; do
  row="${row_slug%%:*}"
  slug="${row_slug#*:}"
  reports=()
  for seed in 1 2 3 4 5; do
    reports+=("$OUT/hashed-tensor-${slug}-s${seed}.json")
  done
  "$PIN_PY" --key ns_per_hashed_tensor_step --instance "$row" \
    --calibration "$CAL" --dry "${reports[@]}" 2>&1 | tee -a "$OUT/dry-pin.log" || true
done

echo "DONE $(date -u +%Y-%m-%dT%H:%M:%SZ)" | tee -a "$OUT/dry-pin.log"