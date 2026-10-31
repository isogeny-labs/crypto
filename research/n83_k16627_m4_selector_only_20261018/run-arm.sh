#!/bin/sh
set -eu

if [ "$#" -ne 6 ]; then
    echo 'usage: run-arm.sh full|selector-only|phase-only CONTAINER_NAME NEW_OUTPUT_DIR WORKER_BINARY PANEL_DIR SNAPSHOT_DIR' >&2
    exit 2
fi

arm=$1
name=$2
output=$3
worker=$4
panel=$5
snapshot=$6

case "$arm" in
    full) command=primary-orbit-chain-selector-positive ;;
    selector-only) command=primary-orbit-chain-selector-only ;;
    phase-only) command=primary-orbit-chain-phase-only ;;
    *) echo 'unknown arm' >&2; exit 2 ;;
esac

for source in "$worker" "$panel" "$snapshot"; do
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
output=$(CDPATH= cd -- "$output" && pwd)

set +e
docker run --name "$name" --platform linux/arm64 \
    --cpus=1 --memory=4096m --memory-swap=4096m --network=none --read-only \
    --tmpfs /tmp:rw,size=64m \
    -v "$worker:/worker:ro" -v "$panel:/panel:ro" \
    -v "$snapshot:/snapshot:ro" -v "$output:/out" \
    -e ICV1_FROZEN_SOURCE_DIR=/snapshot \
    rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31 \
    sh -c 'date -u +%FT%TZ > /out/start.utc; timeout -s TERM -k 5 1800 /worker "$1" /panel 4096 /out/worker.json; result=$?; cat /sys/fs/cgroup/memory.peak > /out/peak.bytes; date -u +%FT%TZ > /out/end.utc; exit "$result"' \
    sh "$command" > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
printf '%s\n' "$result" > "$output/host-exit.txt"
docker inspect --format '{{json .State}}' "$name" > "$output/container-state.json" 2> "$output/inspect-state-stderr.log"
state_result=$?
docker inspect --format '{{json .HostConfig}}' "$name" > "$output/host-config.json" 2> "$output/inspect-config-stderr.log"
config_result=$?
set -e
printf '%s\n' "$arm" > "$output/arm.txt"
if [ "$state_result" -ne 0 ] || [ "$config_result" -ne 0 ]; then
    echo "container inspection failed: state=$state_result config=$config_result" >&2
    exit 3
fi
exit "$result"
