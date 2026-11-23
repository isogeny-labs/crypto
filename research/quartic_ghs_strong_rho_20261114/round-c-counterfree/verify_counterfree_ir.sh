#!/usr/bin/env bash
set -euo pipefail

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
study=$(CDPATH= cd -- "$here/.." && pwd)
scratch=$(mktemp -d "${TMPDIR:-/tmp}/ghs-counterfree-ir.XXXXXX")
trap 'rm -rf -- "$scratch"' EXIT

rustc --edition 2021 -O --emit=llvm-ir "$study/main.rs" -o "$scratch/main.ll"

# Extract optimized definitions, excluding call-site comments. Rust's LLVM IR
# keeps the const specialization in its demangled comment and the local name
# from FastF3::bump (`c.i*`) on surviving counter read/modify/write operations.
awk '
/^; <main::strong_rho::Fast(F3|Ec)<false>>::/ { candidate=1; next }
candidate && /^; Function Attrs:/ { next }
candidate && /^define / { active=1; candidate=0 }
active { print }
active && /^}$/ { active=0 }
' "$scratch/main.ll" > "$scratch/arithmetic-uncounted.ll"

awk '
/^; <main::strong_rho::Fast(F3|Ec)>::/ { candidate=1; next }
candidate && /^; Function Attrs:/ { next }
candidate && /^define / { active=1; candidate=0 }
active { print }
active && /^}$/ { active=0 }
' "$scratch/main.ll" > "$scratch/arithmetic-counted.ll"

awk '
/^define .*4main10strong_rho20strong_rho_uncounted\(/ { active=1 }
active { print }
active && /^}$/ { exit }
' "$scratch/main.ll" > "$scratch/walk-uncounted.ll"

awk '
/^define .*4main10strong_rho10strong_rho\(/ { active=1 }
active { print }
active && /^}$/ { exit }
' "$scratch/main.ll" > "$scratch/walk-counted.ll"

count_matches() {
    local matches
    matches=$(rg -c "$1" "$2" || true)
    printf '%s' "${matches:-0}"
}

counted_defs=$(count_matches '^define ' "$scratch/arithmetic-counted.ll")
uncounted_defs=$(count_matches '^define ' "$scratch/arithmetic-uncounted.ll")
counted_arithmetic_markers=$(count_matches '%c\.i[0-9]' "$scratch/arithmetic-counted.ll")
uncounted_arithmetic_markers=$(count_matches '%c\.i[0-9]' "$scratch/arithmetic-uncounted.ll")
counted_walk_markers=$(count_matches '%c\.i[0-9]' "$scratch/walk-counted.ll")
uncounted_walk_markers=$(count_matches '%c\.i[0-9]' "$scratch/walk-uncounted.ll")

rustc --version
printf 'counted_arithmetic_defs=%s\n' "$counted_defs"
printf 'uncounted_arithmetic_defs=%s\n' "$uncounted_defs"
printf 'counted_arithmetic_counter_ssa_markers=%s\n' "$counted_arithmetic_markers"
printf 'uncounted_arithmetic_counter_ssa_markers=%s\n' "$uncounted_arithmetic_markers"
printf 'counted_walk_counter_ssa_markers=%s\n' "$counted_walk_markers"
printf 'uncounted_walk_counter_ssa_markers=%s\n' "$uncounted_walk_markers"

if (( counted_defs == 0 || uncounted_defs != counted_defs )); then
    echo 'FAIL: optimized arithmetic specializations were not both retained' >&2
    exit 1
fi
if (( counted_arithmetic_markers == 0 || counted_walk_markers == 0 )); then
    echo 'FAIL: counted reference exposes no counter-update markers' >&2
    exit 1
fi
if (( uncounted_arithmetic_markers != 0 || uncounted_walk_markers != 0 )); then
    echo 'FAIL: COUNT=false retains counter read/modify/write markers' >&2
    exit 1
fi

echo 'PASS: optimized COUNT=false removes FastF3 counter read/modify/write hooks'
