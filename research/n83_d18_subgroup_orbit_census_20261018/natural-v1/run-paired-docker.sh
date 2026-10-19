#!/bin/sh
# shellcheck disable=SC2024
set -eu
# The caller owns receipt redirects even though Docker itself requires sudo.

if [ "$#" -ne 8 ]; then
    echo 'usage: run-paired-docker.sh U|D preflight|positive|natural NEW_OUTPUT_DIR WORKER_BINARY BASE_DIR EVIDENCE_DIR FIXTURE_DIR SOURCE_SNAPSHOT' >&2
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
image='amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f'

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
container_name="n83-paired-$(basename "$output")"

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
    "$image" /usr/bin/timeout -s TERM -k 5 1800 \
    /opt/worker primary-paired-orbit "$mode" "$arm" \
    /inputs/base /inputs/evidence /inputs/fixture /out/worker.json \
    > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
sudo docker inspect "$container_name" > "$output/container-inspect.json"
inspect_result=$?
if [ "$inspect_result" -eq 0 ]; then
    sudo docker rm "$container_name" > "$output/container-removed.txt"
fi
date -u +%FT%TZ > "$output/end.utc"
printf '%s\n' "$result" > "$output/host-exit.txt"
printf '%s\n' "$arm" > "$output/arm.txt"
printf '%s\n' "$mode" > "$output/mode.txt"
exit "$result"
