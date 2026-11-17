#!/bin/sh
# One isolated arm of the frozen N83 XOR-cache gate.
set -eu

if [ "$#" -ne 11 ]; then
    echo 'usage: run-cache-docker.sh off|on preflight|positive|natural UUUU|UUDD 0 NEW_OUTPUT_DIR WORKER_BINARY U_BASE_DIR D_BASE_DIR EVIDENCE_DIR FIXTURE_DIR SOURCE_SNAPSHOT' >&2
    exit 2
fi
cache=$1
mode=$2
shape=$3
fixture_index=$4
output=$5
worker=$6
u_base=$7
d_base=$8
evidence=$9
shift 9
fixture=$1
snapshot=$2
image='amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f'

case "$cache:$shape:$mode:$fixture_index" in
    off:UUUU:preflight:0|on:UUUU:preflight:0|off:UUDD:preflight:0|on:UUDD:preflight:0|off:UUUU:positive:0|on:UUUU:positive:0|off:UUDD:positive:0|on:UUDD:positive:0|off:UUDD:natural:0|on:UUDD:natural:0) ;;
    *) echo 'arm outside frozen XOR-cache protocol' >&2; exit 2 ;;
esac
for source in "$worker" "$u_base" "$d_base" "$evidence" "$fixture" "$snapshot"; do
    if [ ! -e "$source" ]; then echo "missing input: $source" >&2; exit 2; fi
done
if [ -e "$output" ]; then echo "output already exists: $output" >&2; exit 2; fi
mkdir "$output"
output=$(CDPATH='' cd -- "$output" && pwd -P)
worker=$(realpath "$worker")
u_base=$(realpath "$u_base")
d_base=$(realpath "$d_base")
evidence=$(realpath "$evidence")
fixture=$(realpath "$fixture")
snapshot=$(realpath "$snapshot")
container_name="n83-cache-$(basename "$output")"

date -u +%FT%TZ > "$output/start.utc"
sudo docker image inspect "$image" > "$output/image-inspect.json"
set +e
sudo docker run --pull=never --name "$container_name" \
    --memory=8g --memory-swap=8g --cpus=1 --cpuset-cpus=7 \
    --network=none --read-only --pids-limit=256 \
    --cap-drop=ALL --security-opt no-new-privileges \
    --tmpfs /tmp:rw,noexec,nosuid,nodev,size=64m \
    --user "$(id -u):$(id -g)" \
    --env ICV1_FROZEN_SOURCE_DIR=/snapshot \
    --env ICV1_XOR_DECIDED_CACHE="$cache" \
    --mount "type=bind,src=$worker,dst=/opt/worker,readonly" \
    --mount "type=bind,src=$u_base,dst=/inputs/u,readonly" \
    --mount "type=bind,src=$d_base,dst=/inputs/d,readonly" \
    --mount "type=bind,src=$evidence,dst=/inputs/evidence,readonly" \
    --mount "type=bind,src=$fixture,dst=/inputs/fixture,readonly" \
    --mount "type=bind,src=$snapshot,dst=/snapshot,readonly" \
    --mount "type=bind,src=$output,dst=/out" \
    "$image" /usr/bin/timeout -s TERM -k 5 610 \
    /opt/worker primary-mixed-xor-cache-m4 "$cache" "$mode" "$shape" "$fixture_index" \
    /inputs/u /inputs/d /inputs/evidence /inputs/fixture /out/worker.json \
    > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
sudo docker inspect "$container_name" > "$output/container-inspect.json"
inspect_result=$?
if [ "$inspect_result" -eq 0 ]; then sudo docker rm "$container_name" > "$output/container-removed.txt"; fi
date -u +%FT%TZ > "$output/end.utc"
printf '%s\n' "$result" > "$output/host-exit.txt"
printf '%s\n' "$cache" > "$output/cache.txt"
printf '%s\n' "$shape" > "$output/shape.txt"
printf '%s\n' "$mode" > "$output/mode.txt"
printf '%s\n' "$fixture_index" > "$output/fixture-index.txt"
exit "$result"
