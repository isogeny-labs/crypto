#!/bin/sh
# Host wrapper for the frozen m=5 container gate. Caller retains every sidecar.
set -eu

if [ "$#" -ne 9 ]; then
    echo 'usage: run-paired-m5-docker.sh U|D preflight|positive|natural 0|1 NEW_OUTPUT_DIR WORKER_BINARY BASE_DIR EVIDENCE_DIR FIXTURE_DIR SOURCE_SNAPSHOT' >&2
    exit 2
fi
arm=$1
mode=$2
fixture_index=$3
output=$4
worker=$5
base=$6
evidence=$7
fixture=$8
snapshot=$9
image='amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f'

case "$arm:$mode:$fixture_index" in
    U:preflight:0|D:preflight:0|U:preflight:1|D:preflight:1|U:positive:0|D:positive:0|U:natural:0|D:natural:0|U:natural:1|D:natural:1) ;;
    *) echo 'arm, mode or fixture is outside the frozen m5 protocol' >&2; exit 2 ;;
esac
for source in "$worker" "$base" "$evidence" "$fixture" "$snapshot"; do
    if [ ! -e "$source" ]; then echo "missing input: $source" >&2; exit 2; fi
done
if [ -e "$output" ]; then echo "output already exists: $output" >&2; exit 2; fi
mkdir "$output"
output=$(CDPATH='' cd -- "$output" && pwd -P)
worker=$(realpath "$worker")
base=$(realpath "$base")
evidence=$(realpath "$evidence")
fixture=$(realpath "$fixture")
snapshot=$(realpath "$snapshot")
container_name="n83-m5-$(basename "$output")"

date -u +%FT%TZ > "$output/start.utc"
sudo docker image inspect "$image" > "$output/image-inspect.json"
set +e
sudo docker run --pull=never --name "$container_name" \
    --memory=4g --memory-swap=4g --cpus=1 --cpuset-cpus=7 \
    --network=none --read-only --pids-limit=256 \
    --cap-drop=ALL --security-opt no-new-privileges \
    --tmpfs /tmp:rw,noexec,nosuid,nodev,size=64m \
    --user "$(id -u):$(id -g)" \
    --env ICV1_FROZEN_SOURCE_DIR=/snapshot \
    --mount "type=bind,src=$worker,dst=/opt/worker,readonly" \
    --mount "type=bind,src=$base,dst=/inputs/base,readonly" \
    --mount "type=bind,src=$evidence,dst=/inputs/evidence,readonly" \
    --mount "type=bind,src=$fixture,dst=/inputs/fixture,readonly" \
    --mount "type=bind,src=$snapshot,dst=/snapshot,readonly" \
    --mount "type=bind,src=$output,dst=/out" \
    "$image" /usr/bin/timeout -s TERM -k 5 610 \
    /opt/worker primary-paired-orbit-m5 "$mode" "$arm" "$fixture_index" \
    /inputs/base /inputs/evidence /inputs/fixture /out/worker.json \
    > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
sudo docker inspect "$container_name" > "$output/container-inspect.json"
inspect_result=$?
if [ "$inspect_result" -eq 0 ]; then sudo docker rm "$container_name" > "$output/container-removed.txt"; fi
date -u +%FT%TZ > "$output/end.utc"
printf '%s\n' "$result" > "$output/host-exit.txt"
printf '%s\n' "$arm" > "$output/arm.txt"
printf '%s\n' "$mode" > "$output/mode.txt"
printf '%s\n' "$fixture_index" > "$output/fixture-index.txt"
exit "$result"
