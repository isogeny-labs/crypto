#!/bin/sh
# Thin archival orchestration; only a completed card can be collected here.
set -eu
test "$#" -eq 2
root=$(realpath "$1")
output=$2
test "$(cat "$root/card-status.txt")" = COMPLETE_BOUNDED
test ! -e "$output"
mkdir "$output"
output=$(realpath "$output")
mkdir "$output/host"
for name in card.log card-start.utc card-end.utc card-status.txt source-commit.txt inputs.sha256 input-check.log host.txt host-uptime.txt host-cpu.txt host-containers.txt run-xor-cache-card.sh run-xor-cache-docker.sh; do
    cp "$root/$name" "$output/host/$name"
done
cp /home/ec2-user/n83-m23-20261019/inputs/public/public-targets.json "$output/host/public-targets.json"
cp -R "$root/runs" "$output/runs"
cd "$output"
find host runs -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > files.sha256
sha256sum -c files.sha256 > hash-check.log
tar --sort=name --mtime=2026-10-20T00:00:00Z --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' -cf - host runs files.sha256 | xz -6 -T1 > runs.tar.xz
sha256sum runs.tar.xz files.sha256 > archive.sha256
date -u +%FT%TZ > collected.utc
