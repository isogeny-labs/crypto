#!/bin/sh
# Archive only a completed frozen card; never mutate its pinned source or receipts.
set -eu
test "$#" -eq 2
root=$(realpath "$1")
output=$2
test "$(cat "$root/card-status.txt")" = COMPLETE_BOUNDED
test ! -e "$output"
mkdir "$output"
output=$(realpath "$output")
mkdir "$output/host"
for name in card.log card-start.utc card-end.utc card-status.txt card.pid source-commit.txt inputs.sha256 input-check.log prepare-check.log host.txt host-uptime.txt host-cpu.txt host-containers.txt run-full-m4-card.sh run-full-m4-docker.sh control-pair-U.json control-pair-D.json cache-audit.json prelaunch-image.json; do
    cp "$root/$name" "$output/host/$name"
done
cp "$0" "$output/host/collect-full-m4-final.sh"
date -u +%FT%TZ > "$output/host/collection-start.utc"
uptime > "$output/host/host-uptime-final.txt"
sudo -n docker ps --no-trunc --format '{{.ID}} {{.Names}} {{.Image}} {{.Status}}' > "$output/host/host-containers-final.txt"
sudo -n docker ps -q | while IFS= read -r container; do
    sudo -n docker inspect --format '{{.Name}} cpuset={{.HostConfig.CpusetCpus}} nano_cpus={{.HostConfig.NanoCpus}} memory={{.HostConfig.Memory}} memory_swap={{.HostConfig.MemorySwap}}' "$container" \
        || printf 'UNAVAILABLE container=%s (inspect failed during closing snapshot)\n' "$container"
done > "$output/host/host-container-limits-final.txt"
cp /home/ec2-user/n83-m23-20261019/inputs/public/public-targets.json "$output/host/public-targets.json"
cp -R "$root/runs" "$output/runs"
cd "$output"
find host runs -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > files.sha256
sha256sum -c files.sha256 > hash-check.log
tar --sort=name --mtime=2026-10-20T00:00:00Z --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' -cf - host runs files.sha256 | xz -6 -T1 > runs.tar.xz
sha256sum runs.tar.xz files.sha256 > archive.sha256
date -u +%FT%TZ > collected.utc
