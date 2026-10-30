#!/bin/sh
set -eu

if [ "$#" -ne 8 ]; then
    echo 'usage: run-paired-spot.sh U|D preflight|positive|natural NEW_OUTPUT_DIR WORKER_BINARY BASE_DIR EVIDENCE_DIR FIXTURE_DIR SOURCE_SNAPSHOT' >&2
    exit 2
fi

arm=$1
mode=$2
output=$3
worker=$4
base=$5
evidence=$6
fixture=$7
snapshot=$8

case "$arm:$mode" in
    U:preflight|D:preflight|U:positive|D:positive|U:natural|D:natural) ;;
    *) echo 'arm or mode is outside the frozen paired protocol' >&2; exit 2 ;;
esac
for source in "$worker" "$base" "$evidence" "$fixture" "$snapshot"; do
    if [ ! -e "$source" ]; then
        echo "missing input: $source" >&2
        exit 2
    fi
done
if [ -e "$output" ]; then
    echo "output already exists: $output" >&2
    exit 2
fi
mkdir "$output"
output=$(CDPATH='' cd -- "$output" && pwd -P)
worker=$(realpath "$worker")
base=$(realpath "$base")
evidence=$(realpath "$evidence")
fixture=$(realpath "$fixture")
snapshot=$(realpath "$snapshot")

set +e
systemd-run --user --scope \
    -p MemoryMax=4G -p MemorySwapMax=0 -p CPUQuota=100% \
    -p IPAddressDeny=any \
    env ICV1_FROZEN_SOURCE_DIR="$snapshot" taskset -c 7 \
    sh -c '
        output=$1
        worker=$2
        shift 2
        date -u +%FT%TZ > "$output/start.utc"
        cgroup_path=$(cut -d: -f3 /proc/self/cgroup)
        printf "%s\n" "$cgroup_path" > "$output/cgroup.txt"
        grep Cpus_allowed_list /proc/self/status > "$output/cpu-affinity.txt"
        cat "/sys/fs/cgroup$cgroup_path/memory.max" > "$output/memory-limit.bytes"
        cat "/sys/fs/cgroup$cgroup_path/memory.swap.max" > "$output/swap-limit.bytes"
        timeout -s TERM -k 5 1800 "$worker" "$@"
        result=$?
        cat "/sys/fs/cgroup$cgroup_path/memory.peak" > "$output/peak.bytes"
        cat "/sys/fs/cgroup$cgroup_path/memory.events" > "$output/memory-events.txt"
        date -u +%FT%TZ > "$output/end.utc"
        exit "$result"
    ' sh "$output" "$worker" primary-paired-orbit "$mode" "$arm" \
        "$base" "$evidence" "$fixture" "$output/worker.json" \
    > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
printf '%s\n' "$result" > "$output/host-exit.txt"
printf '%s\n' "$arm" > "$output/arm.txt"
printf '%s\n' "$mode" > "$output/mode.txt"
exit "$result"
