#!/bin/sh
set -eu

if [ "$#" -ne 6 ]; then
    echo 'usage: run-spot.sh preflight|selector-only|natural NEW_OUTPUT_DIR WORKER_BINARY PANEL_DIR FIXTURE_DIR SNAPSHOT_DIR' >&2
    exit 2
fi

mode=$1
output=$2
worker=$3
panel=$4
fixture=$5
snapshot=$6

for source in "$worker" "$panel" "$fixture" "$snapshot"; do
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

case "$mode" in
    preflight)
        set -- primary-orbit-chain-cold "$panel" "$fixture" 16627 public_x_hash 2026100801 4 0 300000 8000000 1 100000 1790 4096 "$output/run"
        ;;
    selector-only)
        set -- primary-orbit-chain-selector-only "$panel" 4096 "$output/worker.json"
        ;;
    natural)
        set -- primary-orbit-chain-cold "$panel" "$fixture" 16627 public_x_hash 2026100801 4 1 300000 8000000 1 100000 1790 4096 "$output/run"
        ;;
    *)
        echo "unknown spot arm: $mode" >&2
        exit 2
        ;;
esac

set +e
# The child shell expands the literal script.
# shellcheck disable=SC2016
systemd-run --user --scope \
    -p MemoryMax=4G -p MemorySwapMax=0 -p CPUQuota=100% \
    -p IPAddressDeny=any \
    env ICV1_FROZEN_SOURCE_DIR="$snapshot" taskset -c 31 \
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
    ' sh "$output" "$worker" "$@" \
    > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
printf '%s\n' "$result" > "$output/host-exit.txt"
printf '%s\n' "$mode" > "$output/arm.txt"
exit "$result"
