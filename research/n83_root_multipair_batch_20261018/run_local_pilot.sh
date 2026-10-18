#!/bin/sh
set -eu

repo=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
binary=${1:?usage: run_local_pilot.sh /absolute/binary /absolute/output-dir}
output=${2:?usage: run_local_pilot.sh /absolute/binary /absolute/output-dir}
base="$repo/research/n83_state28_20261013/runs/control/k512_packed/base_dump.jsonl"
source="$repo/examples/koblitz_orbit_dlp_fast.rs"
expected_source=0b3fe1952e5ea193373b539a1c3149c25945b0c56f585ed7e626e204078f12d1
expected_base=7b97fbdb231418a5caccae429b2bb62162865a4a1421399b53573eff32d47b33

[ "$(shasum -a 256 "$source" | cut -d ' ' -f 1)" = "$expected_source" ]
[ "$(shasum -a 256 "$base" | cut -d ' ' -f 1)" = "$expected_base" ]
[ -x "$binary" ]
mkdir "$output"
: > "$output/empty_targets.txt"
shasum -a 256 "$source" "$base" "$binary" > "$output/sha256.txt"
uname -a > "$output/host.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' >> "$output/host.txt"
sysctl vm.loadavg >> "$output/host.txt"

failed=0
for arm in 1-pair1 2-pair4 3-pair4 4-pair1; do
    case "$arm" in
        *-pair4) pair_batch=4 ;;
        *) pair_batch=1 ;;
    esac
    mkdir "$output/$arm"
    date -u '+%Y-%m-%dT%H:%M:%SZ' > "$output/$arm/started_utc.txt"
    if /usr/bin/time -l timeout 180 env -i \
        PATH=/usr/bin:/bin:/opt/homebrew/bin \
        KIC_SYMMETRIC_S3=1 \
        KIC_SPARSE_CANONICAL=1 \
        KIC_PACKED_ROOT_TABLE=1 \
        KIC_ROOT_ONLY11=1 \
        KIC_INDEXED_ROOT5=1 \
        KIC_INDEXED5_LOAD75=1 \
        KIC_INDEX_BUILD_ONLY=1 \
        KIC_INDEX_ROOT_BATCH=1 \
        KIC_INDEX_ROOT_PAIR_BATCH="$pair_batch" \
        "$binary" "$base" "$output/empty_targets.txt" 29 \
        "$output/$arm/targets.jsonl" \
        > "$output/$arm/stdout.jsonl" 2> "$output/$arm/stderr.log"; then
        status=0
    else
        status=$?
        failed=1
    fi
    printf '%s\n' "$status" > "$output/$arm/exit_status.txt"
    date -u '+%Y-%m-%dT%H:%M:%SZ' > "$output/$arm/finished_utc.txt"
done
exit "$failed"
