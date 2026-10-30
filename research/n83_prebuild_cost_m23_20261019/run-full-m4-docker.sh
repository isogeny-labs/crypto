#!/bin/sh
set -eu
if [ "$#" -lt 10 ]; then
    echo 'usage: run-full-m4-docker.sh positive|natural|replay off|on U|D TARGET NEW_DIR WORKER BASE CORPUS SOURCE_COMMIT LABEL [RECEIPT]' >&2
    exit 2
fi
mode=$1 cache=$2 arm=$3 target=$4 output=$5 worker=$6 base=$7 corpus=$8 revision=$9
shift 9
label=$1
shift
case "$cache:$target" in off:0|on:[0-7]) ;; *) exit 2 ;; esac
case "$mode:$arm:$#" in
    positive:U:0|positive:D:0|natural:U:0|natural:D:0|replay:U:1|replay:D:1) ;;
    *) echo 'outside full m4 card' >&2; exit 2 ;;
esac
case "$mode:$cache:$target" in natural:off:*|positive:*:[1-7]) exit 2 ;; esac
test ! -e "$output"
mkdir "$output"
output=$(realpath "$output") worker=$(realpath "$worker") base=$(realpath "$base") corpus=$(realpath "$corpus")
image='amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f'
container="n83-full-m4-$label"
date -u +%FT%TZ > "$output/start.utc"
sha256sum "$worker" > "$output/binary.sha256"
sudo docker image inspect "$image" > "$output/image-inspect.json"
if [ "$mode" = replay ]; then
    receipt=$(realpath "$1")
    set -- --mount "type=bind,src=$receipt,dst=/inputs/receipt.json,readonly" "$image" \
        /usr/bin/timeout -s TERM -k 5 1800 /opt/worker /inputs/base /inputs/public.json /inputs/receipt.json /out/worker.json
else
    set -- "$image" /usr/bin/timeout -s TERM -k 5 1800 /opt/worker full-m4 "$cache" "$mode" "$arm" /inputs/base /inputs/public.json "$target" /out/worker.json
fi
set +e
sudo docker run --pull=never --name "$container" --memory=4g --memory-swap=4g \
    --cpus=1 --cpuset-cpus=31 --network=none --read-only --pids-limit=256 \
    --cap-drop=ALL --security-opt no-new-privileges --user "$(id -u):$(id -g)" \
    --tmpfs /tmp:rw,noexec,nosuid,nodev,size=64m --env "N83_SOURCE_COMMIT=$revision" \
    --mount "type=bind,src=$worker,dst=/opt/worker,readonly" \
    --mount "type=bind,src=$base,dst=/inputs/base,readonly" \
    --mount "type=bind,src=$corpus,dst=/inputs/public.json,readonly" \
    --mount "type=bind,src=$output,dst=/out" "$@" \
    > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
sudo docker inspect "$container" > "$output/container-inspect.json"
sudo docker rm "$container" > "$output/container-removed.txt"
date -u +%FT%TZ > "$output/end.utc"
printf '%s\n' "$result" > "$output/host-exit.txt"
exit "$result"
