#!/bin/sh
# Copy receipts only; never mutate a consumed card or synthesize a missing run.
set -eu
test "$#" -eq 2
root=$(realpath "$1")
output=$2
test -d "$root/runs"
test -s "$root/card-status.txt"
test ! -e "$output"
mkdir "$output"
output=$(realpath "$output")
case "$output/" in "$root/"*) echo 'archive must be outside the card' >&2; exit 2 ;; esac
mkdir "$output/host"
cp -R "$root/runs" "$output/runs"
for directory in build proof construction-run; do
    cp -R "$root/$directory" "$output/host/$directory"
done
for name in run-card.sh run-docker.sh prepare-card.sh construction.json source-commit.txt \
    inputs.sha256 prelaunch-image.json prepare-status.txt prepare-check.log prepared.utc \
    card-status.txt card-start.utc host.txt host-uptime.txt host-cpu.txt host-containers.txt input-check.log; do
    cp "$root/$name" "$output/host/$name"
done
for file in "$root/card-end.utc" "$root/host-card.json" "$root"/pair-*.json; do
    if [ -f "$file" ]; then cp "$file" "$output/host/"; fi
done
cp /home/ec2-user/n83-m23-20261019/inputs/public/public-targets.json "$output/host/public-targets.json"
cp "$0" "$output/host/collect-card.sh"
cd "$output"
find host runs -type f -print | LC_ALL=C sort > archive-files.txt
xargs sha256sum < archive-files.txt > files.sha256
tar --format=ustar --no-recursion --mtime='2026-10-24 00:00:00Z' \
    --owner=0 --group=0 --numeric-owner --mode=0644 -cf - -T archive-files.txt | xz -6 -T1 > runs.tar.xz
sha256sum runs.tar.xz files.sha256 > archive.sha256
sha256sum -c files.sha256 > copy-check.log
