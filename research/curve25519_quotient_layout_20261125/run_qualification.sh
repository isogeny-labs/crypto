#!/usr/bin/env bash
set -euo pipefail

study_dir=$(cd "$(dirname "$0")" && pwd)
binary=${QUOTIENT_LAYOUT_BINARY:-/tmp/curve25519-projective-verify/target/release/examples/curve25519_quotient_layout}
expected_binary_sha=5972800a79a50b27d725d961cbc60ecc512a9c3c1923a73c2b38b5aa9c3da9d4

actual_binary_sha=$(sha256sum "$binary" | awk '{print $1}')
if [[ "$actual_binary_sha" != "$expected_binary_sha" ]]; then
    echo "binary hash mismatch: expected $expected_binary_sha, got $actual_binary_sha" >&2
    exit 2
fi

mkdir -p "$study_dir/raw"

run_cell() {
    local kind=$1
    local index=$2
    local arm=$3
    local out="$study_dir/raw/${kind}-d19-i${index}-${arm}.json"
    local tmp="${out}.tmp"
    local err="${out}.stderr"
    local exit_file="${out}.exit"
    if [[ -s "$out" ]] && jq -e '.schema == "curve25519.quotient-layout-row/v1"' "$out" >/dev/null; then
        return
    fi
    set +e
    timeout 330s env RAYON_NUM_THREADS=1 "$binary" cell \
        --arm "$arm" --d 19 --kind "$kind" --index "$index" \
        --cap 300 --dmax 24 --seed 2026112502 >"$tmp" 2>"$err"
    local rc=$?
    set -e
    printf '%s\n' "$rc" >"$exit_file"
    if [[ $rc -eq 0 ]] && jq -e '.schema == "curve25519.quotient-layout-row/v1"' "$tmp" >/dev/null; then
        mv "$tmp" "$out"
        rm -f "$exit_file" "$err"
    else
        mv "$tmp" "${out}.partial"
        return "$rc"
    fi
}

for index in 0 1 2 3 4 5 6 7; do
    for arm in D T F E I; do
        run_cell random "$index" "$arm"
    done
done
for index in 0 1 2 3; do
    for arm in D T F E I; do
        run_cell planted "$index" "$arm"
    done
done
