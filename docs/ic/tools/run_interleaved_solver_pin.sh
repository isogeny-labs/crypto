#!/usr/bin/env bash
# Interleaved pin harness: classical solvers on matched Koblitz workloads and
# hashed-tensor contraction on the two frozen prime rows.
#
# Run on the calibration host (isolab-eks). Each engine: 5 reps, seeds 1..5,
# interleaved by rep index. Reports land under docs/ic/runs/ with a dated stamp.
#
# Usage (from repo root):
#   bash docs/ic/tools/run_interleaved_solver_pin.sh [STAMP]
#
# Requires release `ic` on PATH (`cargo build --release -p crypto-lib` or image).

set -euo pipefail

STAMP="${1:-$(date -u +%Y%m%dT%H%M%SZ)}"
OUT_DIR="docs/ic/runs/pin-${STAMP}"
mkdir -p "$OUT_DIR"

IC="${IC_BIN:-ic}"
PIN_PY="python3 docs/ic/tools/solver_unit_pin_calibration.py"

# --- Matched Koblitz cells (solver-block keys): binary-subspace d=6 + descent m=2 ---
KOBLITZ_COMMON=(
  --koblitz-degree 15
  --factor-base 'binary-subspace:dimension=6'
  --oracle 'descent-algebraic:m=2'
  --targets walk
  --max-trials 2000000
  --repeats 1
)

run_solver() {
  local engine="$1"
  local key="$2"
  local instance="$3"
  local seed="$4"
  local out="$OUT_DIR/${engine}-n15-s${seed}.json"
  "$IC" bench --json "${KOBLITZ_COMMON[@]}" \
    --solver "$engine" \
    --seed "$seed" \
    >"$out"
  echo "$out"
}

# --- Hashed tensor (native counter): frozen defaults, matching slots/seed ---
HASHED_PRIME_COMMON=(
  --factor-base 'hashed-affine'
  --oracle 'hashed-tensor'
  --targets walk
  --max-trials 50000
  --repeats 1
)

run_hashed_prime() {
  local bits="$1"
  local slug="$2"
  local seed="$3"
  local out="$OUT_DIR/hashed-tensor-${slug}-s${seed}.json"
  "$IC" bench --json "${HASHED_PRIME_COMMON[@]}" \
    --bits "$bits" \
    --seed "$seed" \
    >"$out"
  echo "$out"
}

echo "=== interleaved rep 1..5 (by seed) ==="
SOLVER_REPORTS=()
HASHED_20=()
HASHED_24=()

for seed in 1 2 3 4 5; do
  run_solver m4gb-f2 ns_per_monomial_op 'char2/random-binary-n15-b524b' "$seed" >/dev/null
  SOLVER_REPORTS+=("$(run_solver buchberger-f2 ns_per_monomial_op 'char2/random-binary-n15-b524b' "$seed")")
  SOLVER_REPORTS+=("$(run_solver inherited-f4 ns_per_f4_muladd 'char2/random-binary-n15-b524b' "$seed")")
  SOLVER_REPORTS+=("$(run_solver sat-cdcl ns_per_conflict 'char2/random-binary-n15-b524b' "$seed")")
  HASHED_20+=("$(run_hashed_prime 20 bench-20bit "$seed")")
  HASHED_24+=("$(run_hashed_prime 24 generated-24bit "$seed")")
done

echo "=== pin merge (dry-run first) ==="
for key in ns_per_monomial_op ns_per_f4_muladd ns_per_conflict; do
  "$PIN_PY" --key "$key" --instance 'char2/random-binary-n15-b524b' \
    --dry "${SOLVER_REPORTS[@]}" || true
done
"$PIN_PY" --key ns_per_hashed_tensor_step --instance 'prime/bench-20bit' \
  --dry "${HASHED_20[@]}" || true
"$PIN_PY" --key ns_per_hashed_tensor_step --instance 'prime/generated-24bit-10935329' \
  --dry "${HASHED_24[@]}" || true

cat <<EOF

To commit pins after spread checks pass, re-run without --dry, e.g.:

  $PIN_PY --key ns_per_hashed_tensor_step --instance prime/bench-20bit \\
    ${HASHED_20[*]}

Then replay one frozen configuration per key and confirm priced_by: pinned.

EOF