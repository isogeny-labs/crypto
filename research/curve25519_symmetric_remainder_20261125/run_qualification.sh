#!/usr/bin/env bash
set -euo pipefail

study_dir=$(cd "$(dirname "$0")" && pwd)
binary=${SYMMETRIC_REMAINDER_BINARY:-/tmp/curve25519-projective-verify/target/release/examples/curve25519_symmetric_remainder}
expected_binary_sha=b3bf1294e3c8f3b86662b398e163c5c82ced01097294d85b6696f0edc17bad02

actual_binary_sha=$(sha256sum "$binary" | awk '{print $1}')
if [[ "$actual_binary_sha" != "$expected_binary_sha" ]]; then
    echo "binary hash mismatch: expected $expected_binary_sha, got $actual_binary_sha" >&2
    exit 2
fi

mkdir -p "$study_dir/raw"

run_cell() {
    local d=$1
    local kind=$2
    local index=$3
    local arm=$4
    local out="$study_dir/raw/${kind}-d${d}-i${index}-${arm}.json"
    local tmp="${out}.tmp"
    local err="${out}.stderr"
    local exit_file="${out}.exit"
    if [[ -s "$out" ]] && jq -e '.schema == "curve25519.symmetric-remainder-row/v1"' "$out" >/dev/null; then
        return
    fi
    set +e
    timeout 330s env RAYON_NUM_THREADS=1 "$binary" cell \
        --arm "$arm" --d "$d" --kind "$kind" --index "$index" \
        --cap 300 --dmax 24 --seed 20261125 >"$tmp" 2>"$err"
    local rc=$?
    set -e
    printf '%s\n' "$rc" >"$exit_file"
    if [[ $rc -eq 0 ]] && jq -e '.schema == "curve25519.symmetric-remainder-row/v1"' "$tmp" >/dev/null; then
        mv "$tmp" "$out"
        rm -f "$exit_file" "$err"
    else
        mv "$tmp" "${out}.partial"
        return "$rc"
    fi
}

for d in 7 9 19; do
    for index in 0 1 2 3 4 5 6 7; do
        run_cell "$d" random "$index" direct
        run_cell "$d" random "$index" quotient
    done
    for index in 0 1 2 3; do
        run_cell "$d" planted "$index" direct
        run_cell "$d" planted "$index" quotient
    done
done
