#!/usr/bin/env bash
# Thin orchestration only: every measurement is a Rust process
# (examples/orbit_large_prime_bench.rs, one cell per process so each cell's
# peak RSS is its own). Governed by PROTOCOL_ADDENDUM_20261026.md.
#
#   run_addendum.sh primary        arms (ii) dlp, (iii) dlp_orbit, (iii-post) dlp_orbit_post,
#                                  seeds 1-5, the merged run's caps (2 repeats; 1 at n = 31)
#   run_addendum.sh trajectory     the same arms and seeds, 4x cap (3x at n = 31), one repeat,
#                                  with the cap-free rank trajectory
#   run_addendum.sh realisation    the same arms over a seeded random modulus, seeds 1-3
#   run_addendum.sh all
#
# Run after `cargo build --release -j2 --example orbit_large_prime_bench` with
# CARGO_TARGET_DIR set. Failed or timed-out cells are listed in
# receipts_addendum/failures.txt (pipefail: the cell is piped through `tail -1`).
set -u
set -o pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
TARGET="${CARGO_TARGET_DIR:-$HOME/cargo-target}"
BIN="$TARGET/release/examples/orbit_large_prime_bench"
OUT="${OUT:-$HERE/receipts_addendum}"
mkdir -p "$OUT"
MAXSTATES=3000000
PER_CELL_TIMEOUT=900
ARMS=(dlp dlp_orbit dlp_orbit_post)

# n small env m cap reps trajectory_multiplier
CASES=(
  "17 5 10 3 200 2 4"
  "19 6 10 3 400 2 4"
  "23 7 10 3 450 2 4"
  "29 7 10 4 300 2 4"
  "31 7 10 4 450 1 3"
)

cell() { # n arm seed rep small env m cap series [extra args...]
  local n=$1 arm=$2 seed=$3 rep=$4 small=$5 env=$6 m=$7 cap=$8 series=$9
  shift 9
  timeout "$PER_CELL_TIMEOUT" "$BIN" cell --n "$n" --arm "$arm" --seed "$seed" --rep "$rep" \
    --small "$small" --env "$env" --summands "$m" --trials "$cap" --max-states "$MAXSTATES" \
    --series "$series" --out "$OUT" "$@" 2>&1 | tail -1 \
    || echo "FAILED n=$n arm=$arm seed=$seed rep=$rep cap=$cap series=$series $*" >> "$OUT/failures.txt"
}

cutoffs() { # cap max -> comma list of cap/4 cap/2 cap 3cap/2 2cap 3cap 4cap within max
  local cap=$1 max=$2 out="" v
  for v in $((cap / 4)) $((cap / 2)) "$cap" $((cap * 3 / 2)) $((cap * 2)) $((cap * 3)) $((cap * 4)); do
    if [[ "$v" -le "$max" && "$v" -gt 0 ]]; then out+="${out:+,}$v"; fi
  done
  echo "$out"
}

SERIES="${1:-all}"
for c in "${CASES[@]}"; do
  read -r n small env m cap reps mult <<<"$c"
  if [[ "$SERIES" == all || "$SERIES" == primary ]]; then
    for seed in 1 2 3 4 5; do
      for rep in $(seq 1 "$reps"); do
        for arm in "${ARMS[@]}"; do
          cell "$n" "$arm" "$seed" "$rep" "$small" "$env" "$m" "$cap" primary
        done
      done
    done
  fi
  if [[ "$SERIES" == all || "$SERIES" == trajectory ]]; then
    max=$((cap * mult))
    list="$(cutoffs "$cap" "$max")"
    for seed in 1 2 3 4 5; do
      for arm in "${ARMS[@]}"; do
        cell "$n" "$arm" "$seed" 1 "$small" "$env" "$m" "$max" trajectory --cutoffs "$list"
      done
    done
  fi
  if [[ "$SERIES" == all || "$SERIES" == realisation ]]; then
    for seed in 1 2 3; do
      for arm in "${ARMS[@]}"; do
        cell "$n" "$arm" "$seed" 1 "$small" "$env" "$m" "$cap" realisation --modulus-seed 1
      done
    done
  fi
done
