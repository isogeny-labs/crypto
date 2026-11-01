#!/bin/sh
set -eu
test "$#" -eq 2
root=$(realpath "$1")
output=$2
test ! -e "$output"
for arm in U D; do
    jq -e '.status=="PASS"' "$root/control-pair-$arm.json" > /dev/null
    for cache in off on; do
        jq -e '.status=="PASS_CONSTRUCTED_PENDING_REPLAY"' "$root/runs/positive-$arm-$cache/worker.json" > /dev/null
        jq -e '.status=="PASS"' "$root/runs/positive-$arm-$cache-replay/worker.json" > /dev/null
    done
done
mkdir "$output"
output=$(realpath "$output")
mkdir "$output/host" "$output/runs"
for name in card.log card-start.utc card.pid source-commit.txt inputs.sha256 input-check.log prepare-check.log host.txt host-uptime.txt host-cpu.txt host-containers.txt run-full-m4-card.sh run-full-m4-docker.sh control-pair-U.json control-pair-D.json cache-audit.json prelaunch-image.json; do
    cp "$root/$name" "$output/host/$name"
done
for arm in U D; do
    for cache in off on; do
        cp -R "$root/runs/positive-$arm-$cache" "$output/runs/"
        cp -R "$root/runs/positive-$arm-$cache-replay" "$output/runs/"
    done
done
date -u +%FT%TZ > "$output/host/observation.utc"
sudo docker ps --no-trunc > "$output/host/observed-containers.txt"
cd "$output"
find host runs -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > files.sha256
sha256sum -c files.sha256 > hash-check.log
tar --sort=name --mtime=2026-10-20T00:00:00Z --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' -cf - host runs files.sha256 | xz -6 -T1 > controls.tar.xz
sha256sum controls.tar.xz files.sha256 > archive.sha256
