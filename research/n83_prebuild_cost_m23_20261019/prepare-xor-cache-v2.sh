#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
previous="$root/xor-cache-v1-image-restored"
output="$root/xor-cache-v2"
test "$(cat "$previous/card-status.txt")" = 'FAILED_OR_INTERRUPTED exit=2'
test ! -e "$output"
mkdir "$output"
for name in worker replayer run-xor-cache-docker.sh source-commit.txt rustc-version.txt build.log build-v2.log release-lib.log example-tests-v2.log image-restore.log; do
    cp "$previous/$name" "$output/$name"
done
cp /var/tmp/n83-xor-cache-source/research/n83_prebuild_cost_m23_20261019/run-xor-cache-card-v2.sh "$output/run-xor-cache-card.sh"
cp /var/tmp/n83-xor-cache-build-logs/card-tests-v2.log "$output/"
sed -e 's|/n83-m23-20261019/xor-cache-v1-image-restored/|/n83-m23-20261019/xor-cache-v2/|g' \
    -e 's|4477ba28e581f3bccaaf4de1666026b9c67b2d127f5fc5cd2d5676d80106009a|9c9a07cca9ded5c4afe7db03217580c55c806c1c7b0ca120eedf0494721c1a4d|' \
    "$previous/inputs.sha256" > "$output/inputs.sha256"
sha256sum -c "$output/inputs.sha256" > "$output/prepare-check.log"
sudo docker image inspect amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f > "$output/prelaunch-image.json"
