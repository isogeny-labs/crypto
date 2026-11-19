#!/usr/bin/env bash
set -euo pipefail

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$here/../../.." && pwd)
study="$repo/research/quartic_ghs_strong_rho_20261114"
strong="$study/strong_rho.rs"
main="$study/main.rs"
scratch=$(mktemp -d "${TMPDIR:-/tmp}/ghs-r64-native-ir.XXXXXX")
trap 'rm -rf -- "$scratch"' EXIT

revision=${GHS_SOURCE_REVISION:-$(git -C "$repo" rev-parse --short=12 HEAD)}
GHS_SOURCE_REVISION="$revision" rustc --edition 2021 -C opt-level=3 \
    --emit=llvm-ir "$main" -o "$scratch/main.ll"

# Capture only definition headers whose demangled function header names the
# selected native sink. Reset on any intervening non-attribute line so a
# call-site comment cannot attach itself to a later, unrelated definition.
extract_sink_signatures() {
    local sink=$1
    local output=$2
    awk -v sink="$sink" '
        $0 ~ "^; .*" sink ">$" { candidate=1; next }
        candidate && /^; Function Attrs:/ { next }
        candidate && /^define / { print; candidate=0; next }
        candidate && /^$/ { next }
        candidate { candidate=0 }
    ' "$scratch/main.ll" > "$output"
}

extract_sink_signatures BareNative "$scratch/bare-defs.txt"
extract_sink_signatures CountedNative "$scratch/counted-defs.txt"

# The public bare wrapper is kept out of line. Its body contains the inlined
# hot loop plus calls to any retained BareNative-specialized helpers.
awk '
    /^; main::strong_rho::strong_rho_bare$/ { candidate=1; next }
    candidate && /^; Function Attrs:/ { next }
    candidate && /^define / { active=1; candidate=0 }
    active { print }
    active && /^}$/ { exit }
' "$scratch/main.ll" > "$scratch/bare-wrapper.ll"

count_matches() {
    local pattern=$1
    local file=$2
    local matches
    matches=$(rg -c "$pattern" "$file" || true)
    printf '%s' "${matches:-0}"
}

bare_defs=$(count_matches '^define ' "$scratch/bare-defs.txt")
counted_defs=$(count_matches '^define ' "$scratch/counted-defs.txt")
bare_counter_formals=$(count_matches '%(c|native)([,)]|$)' "$scratch/bare-defs.txt")
counted_counter_formals=$(count_matches '%(c|native)([,)]|$)' "$scratch/counted-defs.txt")
bare_wrapper_defs=$(count_matches '^define ' "$scratch/bare-wrapper.ll")
bare_wrapper_counter_references=$(count_matches 'CountedNative|NativeCounts|%native\b|bump_(rng_draws|hashes|canonicalisations|lookahead_tests|dp_tests|table_ops|cycle_compares|scalar_adds|scalar_muls|scalar_invs)' "$scratch/bare-wrapper.ll")
bare_bump_definitions=$(count_matches '^; .*BareNative.*bump_' "$scratch/main.ll")
bare_path_references=$(count_matches 'BareNative' "$scratch/bare-wrapper.ll")
bare_wrapper_lines=$(wc -l < "$scratch/bare-wrapper.ll")

# Source-level premises for interpreting the optimized-IR scan.
rg -q 'struct BareNative;' "$strong"
rg -q 'size_of::<BareNative>' "$strong"
rg -q 'strong_rho_impl::<false, BareNative>' "$strong"

if (( bare_defs < 1 )); then
    echo 'FAIL: no retained BareNative-specialized definition was inspected' >&2
    exit 1
fi
if (( counted_defs < 1 || counted_counter_formals < 1 )); then
    echo 'FAIL: counted positive-control definitions expose no native-counter formal' >&2
    exit 1
fi
if (( bare_wrapper_defs != 1 || bare_wrapper_lines < 100 )); then
    echo 'FAIL: complete optimized strong_rho_bare wrapper was not extracted' >&2
    exit 1
fi
if (( bare_counter_formals != 0 )); then
    echo 'FAIL: a retained BareNative specialization still accepts a native-counter formal' >&2
    exit 1
fi
if (( bare_wrapper_counter_references != 0 || bare_bump_definitions != 0 )); then
    echo 'FAIL: optimized bare path retains a native counter or bump hook' >&2
    exit 1
fi
if (( bare_path_references < 1 )); then
    echo 'FAIL: optimized wrapper exposes no live BareNative-specialized path' >&2
    exit 1
fi

rustc --version
printf 'revision=%s\n' "$revision"
printf 'strong_rho_sha256=%s\n' "$(sha256sum "$strong" | cut -d' ' -f1)"
printf 'main_sha256=%s\n' "$(sha256sum "$main" | cut -d' ' -f1)"
printf 'llvm_ir_sha256=%s\n' "$(sha256sum "$scratch/main.ll" | cut -d' ' -f1)"
printf 'bare_specialized_defs=%s\n' "$bare_defs"
printf 'counted_specialized_defs=%s\n' "$counted_defs"
printf 'bare_native_counter_formals=%s\n' "$bare_counter_formals"
printf 'counted_native_counter_formals=%s\n' "$counted_counter_formals"
printf 'bare_wrapper_defs=%s\n' "$bare_wrapper_defs"
printf 'bare_wrapper_lines=%s\n' "$bare_wrapper_lines"
printf 'bare_wrapper_counter_references=%s\n' "$bare_wrapper_counter_references"
printf 'bare_bump_definitions=%s\n' "$bare_bump_definitions"
printf 'bare_path_references=%s\n' "$bare_path_references"
echo 'PASS: optimized bare strong-rho path retains no NativeCounts mutation hook'
