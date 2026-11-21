#!/bin/sh
set -eu

if [ "$#" -lt 6 ]; then
    echo 'usage: run-spot-docker.sh MODE U|D NEW_OUTPUT_DIR WORKER_BINARY BASE_DIR PUBLIC_CORPUS [TARGET_INDEX] [M] [WORKER_RECEIPT]' >&2
    exit 2
fi
mode=$1
arm=$2
output=$3
worker=$4
base=$5
corpus=$6
shift 6
case "$mode:$arm:$#" in
    setup-prebuilt:D:0|setup-ondemand:D:0|control:U:1|control:D:1|probe:U:2|probe:D:2|replay:U:1|replay:D:1) ;;
    *) echo 'mode, arm or argument count outside protocol' >&2; exit 2 ;;
esac
if [ -e "$output" ]; then echo 'output exists' >&2; exit 2; fi
for required in "$worker" "$base" "$corpus"; do
    if [ ! -e "$required" ]; then echo "missing input: $required" >&2; exit 2; fi
done
mkdir "$output"
output=$(CDPATH='' cd -- "$output" && pwd -P)
worker=$(realpath "$worker")
base=$(realpath "$base")
corpus=$(realpath "$corpus")
image='amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f'
container="n83-m23-$(basename "$output")"
case "$mode" in
    setup-prebuilt) set -- setup prebuilt /inputs/base /out/worker.json ;;
    setup-ondemand) set -- setup ondemand /inputs/base /out/worker.json ;;
    control) set -- control "$arm" /inputs/base "$1" /out/worker.json ;;
    probe) set -- probe "$arm" /inputs/base /inputs/public.json "$1" "$2" /out/worker.json ;;
    replay)
        receipt=$1
        if [ ! -f "$receipt" ]; then echo 'missing worker receipt' >&2; exit 2; fi
        receipt=$(realpath "$receipt")
        set -- replay "$arm" /inputs/base /inputs/public.json "/inputs/receipts/$(basename "$receipt")" /out/worker.json
        ;;
esac

date -u +%FT%TZ > "$output/start.utc"
sudo docker image inspect "$image" > "$output/image-inspect.json"
set +e
if [ "$mode" = replay ]; then
    sudo docker run --pull=never --name "$container" --memory=4g --memory-swap=4g \
        --cpus=1 --cpuset-cpus=31 --network=none --read-only --pids-limit=256 \
        --cap-drop=ALL --security-opt no-new-privileges \
        --tmpfs /tmp:rw,noexec,nosuid,nodev,size=64m --user "$(id -u):$(id -g)" \
        --mount "type=bind,src=$worker,dst=/opt/worker,readonly" \
        --mount "type=bind,src=$base,dst=/inputs/base,readonly" \
        --mount "type=bind,src=$corpus,dst=/inputs/public.json,readonly" \
        --mount "type=bind,src=$(dirname "$receipt"),dst=/inputs/receipts,readonly" \
        --mount "type=bind,src=$output,dst=/out" \
        "$image" /usr/bin/timeout -s TERM -k 5 1800 /opt/worker "$@" \
        > "$output/host-stdout.log" 2> "$output/host-stderr.log"
else
    sudo docker run --pull=never --name "$container" --memory=4g --memory-swap=4g \
        --cpus=1 --cpuset-cpus=31 --network=none --read-only --pids-limit=256 \
        --cap-drop=ALL --security-opt no-new-privileges \
        --tmpfs /tmp:rw,noexec,nosuid,nodev,size=64m --user "$(id -u):$(id -g)" \
        --mount "type=bind,src=$worker,dst=/opt/worker,readonly" \
        --mount "type=bind,src=$base,dst=/inputs/base,readonly" \
        --mount "type=bind,src=$corpus,dst=/inputs/public.json,readonly" \
        --mount "type=bind,src=$output,dst=/out" \
        "$image" /usr/bin/timeout -s TERM -k 5 1800 /opt/worker "$@" \
        > "$output/host-stdout.log" 2> "$output/host-stderr.log"
fi
result=$?
sudo docker inspect "$container" > "$output/container-inspect.json"
sudo docker rm "$container" > "$output/container-removed.txt"
date -u +%FT%TZ > "$output/end.utc"
printf '%s\n' "$result" > "$output/host-exit.txt"
printf '%s\n' "$mode" > "$output/mode.txt"
printf '%s\n' "$arm" > "$output/arm.txt"
exit "$result"
