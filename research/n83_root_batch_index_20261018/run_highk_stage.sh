#!/bin/sh
set -eu

repo=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
binary=${1:?usage: run_highk_stage.sh /absolute/path/to/koblitz_orbit_dlp_fast /absolute/output/dir}
output=${2:?usage: run_highk_stage.sh /absolute/path/to/koblitz_orbit_dlp_fast /absolute/output/dir}
source="$repo/examples/koblitz_orbit_dlp_fast.rs"
expected_source=4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507

[ "$(sha256sum "$source" | cut -d ' ' -f 1)" = "$expected_source" ]
[ -x "$binary" ]
mkdir -p "$output"
: > "$output/empty_targets.txt"
sha256sum "$source" "$binary" > "$output/sha256.txt"
uname -a > "$output/host.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' >> "$output/host.txt"
lscpu > "$output/lscpu.txt"
taskset -pc $$ > "$output/affinity.txt"
cat /proc/self/cgroup > "$output/cgroup.txt"
cat /proc/self/status > "$output/process_status.txt"

failed=0
for arm in 1-scalar 2-batch 3-batch 4-scalar; do
    case "$arm" in
        *-batch) batch=1 ;;
        *) batch=0 ;;
    esac
    mkdir -p "$output/$arm"
    date -u '+%Y-%m-%dT%H:%M:%SZ' > "$output/$arm/started_utc.txt"
    sed -n '1p' /proc/stat > "$output/$arm/proc_stat_before.txt"
    for name in cpu.stat memory.current memory.peak memory.events memory.max cpu.max; do
        if [ -r "/sys/fs/cgroup/$name" ]; then
            cat "/sys/fs/cgroup/$name" > "$output/$arm/${name%.stat}_before_${name##*.}.txt"
        fi
    done
    if /usr/bin/time -v timeout 900 env -i \
        PATH=/usr/bin:/bin \
        KIC_SYMMETRIC_S3=1 \
        KIC_SPARSE_CANONICAL=1 \
        KIC_PACKED_ROOT_TABLE=1 \
        KIC_ROOT_ONLY11=1 \
        KIC_INDEXED_ROOT5=1 \
        KIC_INDEXED5_LOAD75=1 \
        KIC_INDEX_BUILD_ONLY=1 \
        KIC_INDEX_ROOT_BATCH="$batch" \
        "$binary" construct:83:1:2816 "$output/empty_targets.txt" 29 \
        "$output/$arm/targets.jsonl" \
        > "$output/$arm/stdout.jsonl" 2> "$output/$arm/stderr.log"; then
        status=0
    else
        status=$?
        failed=1
    fi
    printf '%s\n' "$status" > "$output/$arm/exit_status.txt"
    date -u '+%Y-%m-%dT%H:%M:%SZ' > "$output/$arm/finished_utc.txt"
    sed -n '1p' /proc/stat > "$output/$arm/proc_stat_after.txt"
    for name in cpu.stat memory.current memory.peak memory.events memory.max cpu.max; do
        if [ -r "/sys/fs/cgroup/$name" ]; then
            cat "/sys/fs/cgroup/$name" > "$output/$arm/${name%.stat}_after_${name##*.}.txt"
        fi
    done
done
exit "$failed"
