#!/usr/bin/env bash
set -euo pipefail
repo="$(cd "$(dirname "$0")/../.." && pwd)"
out="$repo/research/ghs_generalized_20261024/results"
for spec in '6 0x43 0 32' '8 0x11b 0 32' '10 0x409 0 32' '12 0x1009 1 512'; do
    read -r degree modulus a b <<< "$spec"
    edges=(direct two_isogeny)
    if [[ "$degree" == 10 ]]; then edges+=(three_isogeny); fi
    if [[ "$degree" == 12 ]]; then edges+=(five_isogeny); fi
    for edge in "${edges[@]}"; do
        extra=()
        if [[ "$edge" == two_isogeny ]]; then extra+=(--two-isogeny); fi
        if [[ "$edge" == three_isogeny ]]; then extra+=(--isogeny-degree 3); fi
        if [[ "$edge" == five_isogeny ]]; then extra+=(--isogeny-degree 5); fi
        "$repo/target/release/ghs_quadratic_attack" --degree "$degree" --modulus "$modulus" --a "$a" --b "$b" "${extra[@]}" > "$out/exact-n${degree}-a${a}-b${b}-${edge}.json"
    done
done
