#!/bin/sh
# Full model construction, without search, under the frozen 4 GiB memory limit.
set -eu
test "$#" -eq 3
binary=$(realpath "$1")
csv=$(realpath "$2")
output=$3
test "$(sha256sum "$csv" | cut -d ' ' -f 1)" = 534e58ced979e05ec747eb462aa1d63135d8004813d094c522a4d7160c03c4a4
test -z "$(sudo -n docker ps -q)"
test ! -e "$output"
mkdir "$output"
output=$(realpath "$output")
image=amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f
name="n83-shared-construction-$$"
sha256sum "$binary" > "$output/binary.sha256"
sha256sum "$csv" > "$output/input.sha256"
sudo -n docker image inspect "$image" > "$output/image-inspect.json"
date -u +%FT%TZ > "$output/start.utc"
set +e
sudo -n docker run --pull=never --name "$name" --memory=4g --memory-swap=4g \
    --cpus=1 --cpuset-cpus=31 --network=none --read-only --pids-limit=256 \
    --cap-drop=ALL --security-opt no-new-privileges --user "$(id -u):$(id -g)" \
    --tmpfs /tmp:rw,noexec,nosuid,nodev,size=64m \
    --mount "type=bind,src=$binary,dst=/opt/construction,readonly" \
    --mount "type=bind,src=$csv,dst=/inputs/selected.csv,readonly" \
    --mount "type=bind,src=$output,dst=/out" "$image" \
    /usr/bin/timeout -s TERM -k 5 1800 /opt/construction /inputs/selected.csv /out/construction.json \
    > "$output/stdout.log" 2> "$output/stderr.log"
result=$?
sudo -n docker inspect "$name" > "$output/container-inspect.json"
sudo -n docker rm "$name" > "$output/container-removed.txt"
date -u +%FT%TZ > "$output/end.utc"
printf '%s\n' "$result" > "$output/exit.txt"
exit "$result"
