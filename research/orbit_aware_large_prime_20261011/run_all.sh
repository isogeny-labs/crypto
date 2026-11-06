#!/usr/bin/env bash
# Thin orchestration only: every measurement is a Rust process
# (examples/orbit_large_prime_bench.rs). One cell per process so each cell's
# peak RSS is its own. Run after `cargo build --release -j2 --example
# orbit_large_prime_bench` with CARGO_TARGET_DIR set.
set -u
# pipefail: the cell is piped through `tail -1`; without it the pipeline's status is
# that of `tail`, so a crashed or timed-out cell never reached the `|| echo FAILED`
# below and failures.txt could not record anything (review finding F, 2026-10-26;
# harmless in the merged run, which has all 250 cells).
set -o pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
TARGET="${CARGO_TARGET_DIR:-$HOME/cargo-target}"
BIN="$TARGET/release/examples/orbit_large_prime_bench"
OUT="$HERE/receipts"
mkdir -p "$OUT"
MAXSTATES=3000000
PER_CELL_TIMEOUT=900

# n small env m cap reps_dlp
CASES=(
  "17 5 10 3 200 3"
  "19 6 10 3 400 3"
  "23 7 10 3 450 3"
  "29 7 10 4 300 3"
  "31 7 10 4 450 2"
)

cell() { # n arm seed rep small env m cap series
  timeout "$PER_CELL_TIMEOUT" "$BIN" cell --n "$1" --arm "$2" --seed "$3" --rep "$4" \
    --small "$5" --env "$6" --summands "$7" --trials "$8" --max-states "$MAXSTATES" \
    --series "$9" --out "$OUT" 2>&1 | tail -1 || echo "FAILED n=$1 arm=$2 seed=$3 rep=$4 cap=$8" >> "$OUT/failures.txt"
}

SERIES="${1:-all}"
for c in "${CASES[@]}"; do
  read -r n small env m cap reps <<<"$c"
  if [[ "$SERIES" == all || "$SERIES" == primary ]]; then
    for seed in 1 2 3 4 5; do
      cell "$n" none "$seed" 1 "$small" "$env" "$m" "$cap" primary
      for rep in $(seq 1 "$reps"); do
        cell "$n" dlp "$seed" "$rep" "$small" "$env" "$m" "$cap" primary
        cell "$n" dlp_orbit "$seed" "$rep" "$small" "$env" "$m" "$cap" primary
      done
      cell "$n" dlp_orbit_absorb "$seed" 1 "$small" "$env" "$m" "$cap" primary
    done
  fi
  if [[ "$SERIES" == all || "$SERIES" == dose ]]; then
    for factor in half double; do
      if [[ "$factor" == half ]]; then dcap=$((cap / 2)); else dcap=$((cap * 2)); fi
      for seed in 1 2 3; do
        cell "$n" dlp "$seed" 1 "$small" "$env" "$m" "$dcap" dose
        cell "$n" dlp_orbit "$seed" 1 "$small" "$env" "$m" "$dcap" dose
      done
    done
  fi
done

if [[ "$SERIES" == all || "$SERIES" == refusals ]]; then
  # The exact m = n-1 request, refused by the collector before allocation.
  : > "$HERE/refusals.txt"
  for c in "${CASES[@]}"; do
    read -r n small env m cap reps <<<"$c"
    {
      echo "n=$n small=$small env=$env summands=n-1 max_states=$MAXSTATES"
      timeout 120 "$BIN" cell --n "$n" --arm dlp --seed 1 --small "$small" --env "$env" \
        --summands n-1 --trials "$cap" --max-states "$MAXSTATES" --series refusal \
        --out "$HERE/refusal_scratch" 2>&1 | tail -1
    } >> "$HERE/refusals.txt"
  done
  rm -rf "$HERE/refusal_scratch"
fi
