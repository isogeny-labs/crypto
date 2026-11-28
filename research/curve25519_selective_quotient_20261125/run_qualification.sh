#!/usr/bin/env bash
set -euo pipefail

study_dir=$(cd "$(dirname "$0")" && pwd)
binary=${SELECTIVE_QUOTIENT_BINARY:-/tmp/curve25519-projective-verify/target/release/examples/curve25519_selective_quotient}
expected_binary_sha=c83526ee09d99a012741c3a8520612d5b192e1823c2c070efd671fe9e18088d1

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
    if [[ -s "$out" ]] && jq -e '.schema == "curve25519.selective-quotient-row/v1"' "$out" >/dev/null; then
        return
    fi
    set +e
    timeout 330s env RAYON_NUM_THREADS=1 "$binary" cell \
        --arm "$arm" --d 19 --kind "$kind" --index "$index" \
        --cap 300 --dmax 24 --seed 20261125 >"$tmp" 2>"$err"
    local rc=$?
    set -e
    printf '%s\n' "$rc" >"$exit_file"
    if [[ $rc -eq 0 ]] && jq -e '.schema == "curve25519.selective-quotient-row/v1"' "$tmp" >/dev/null; then
        mv "$tmp" "$out"
        rm -f "$exit_file" "$err"
    else
        mv "$tmp" "${out}.partial"
        return "$rc"
    fi
}

for index in 0 1 2 3 4 5 6 7; do
    for arm in D L B N U A; do
        run_cell random "$index" "$arm"
    done
done
for index in 0 1 2 3; do
    for arm in D L B N U A; do
        run_cell planted "$index" "$arm"
    done
done
