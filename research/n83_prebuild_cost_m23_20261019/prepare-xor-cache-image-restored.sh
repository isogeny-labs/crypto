#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
previous="$root/xor-cache-v1"
output="$root/xor-cache-v1-image-restored"
test "$(cat "$previous/card-status.txt")" = 'FAILED_OR_INTERRUPTED exit=1'
test ! -e "$output"
mkdir "$output"
for name in worker replayer run-xor-cache-card.sh run-xor-cache-docker.sh source-commit.txt rustc-version.txt build.log build-v2.log release-lib.log example-tests-v2.log; do
    cp "$previous/$name" "$output/$name"
done
sed 's|/n83-m23-20261019/xor-cache-v1/|/n83-m23-20261019/xor-cache-v1-image-restored/|g' "$previous/inputs.sha256" > "$output/inputs.sha256"
sha256sum -c "$output/inputs.sha256" > "$output/prepare-check.log"
sudo docker image inspect amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f > "$output/prelaunch-image.json"
cp /var/tmp/n83-xor-cache-image-restore.log "$output/image-restore.log"
