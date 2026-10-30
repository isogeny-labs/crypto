#!/bin/sh
# Host wrapper for the frozen N83 m=4 mixed-domain panel.
set -eu

if [ "$#" -ne 10 ]; then
    echo 'usage: run-mixed-docker.sh preflight|positive|natural UUUU|UUDD|DDUU|DDDD 0|1 NEW_OUTPUT_DIR WORKER_BINARY U_BASE_DIR D_BASE_DIR EVIDENCE_DIR FIXTURE_DIR SOURCE_SNAPSHOT' >&2
    exit 2
fi
mode=$1
shape=$2
fixture_index=$3
output=$4
worker=$5
u_base=$6
d_base=$7
evidence=$8
fixture=$9
shift 9
snapshot=$1
image='amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f'

case "$shape:$mode:$fixture_index" in
    UUUU:preflight:0|UUDD:preflight:0|DDUU:preflight:0|DDDD:preflight:0|UUUU:preflight:1|UUDD:preflight:1|DDUU:preflight:1|DDDD:preflight:1|UUUU:positive:0|UUDD:positive:0|DDUU:positive:0|DDDD:positive:0|UUUU:natural:0|UUDD:natural:0|DDUU:natural:0|DDDD:natural:0|UUUU:natural:1|UUDD:natural:1|DDUU:natural:1|DDDD:natural:1) ;;
    *) echo 'shape, mode or fixture is outside the frozen mixed protocol' >&2; exit 2 ;;
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
container_name="n83-mixed-$(basename "$output")"

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
    --mount "type=bind,src=$worker,dst=/opt/worker,readonly" \
    --mount "type=bind,src=$u_base,dst=/inputs/u,readonly" \
    --mount "type=bind,src=$d_base,dst=/inputs/d,readonly" \
    --mount "type=bind,src=$evidence,dst=/inputs/evidence,readonly" \
    --mount "type=bind,src=$fixture,dst=/inputs/fixture,readonly" \
    --mount "type=bind,src=$snapshot,dst=/snapshot,readonly" \
    --mount "type=bind,src=$output,dst=/out" \
    "$image" /usr/bin/timeout -s TERM -k 5 610 \
    /opt/worker primary-mixed-orbit-m4 "$mode" "$shape" "$fixture_index" \
    /inputs/u /inputs/d /inputs/evidence /inputs/fixture /out/worker.json \
    > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
sudo docker inspect "$container_name" > "$output/container-inspect.json"
inspect_result=$?
if [ "$inspect_result" -eq 0 ]; then sudo docker rm "$container_name" > "$output/container-removed.txt"; fi
date -u +%FT%TZ > "$output/end.utc"
printf '%s\n' "$result" > "$output/host-exit.txt"
printf '%s\n' "$shape" > "$output/shape.txt"
printf '%s\n' "$mode" > "$output/mode.txt"
printf '%s\n' "$fixture_index" > "$output/fixture-index.txt"
exit "$result"
