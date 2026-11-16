#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
source=/var/tmp/n83-xor-cache-source
logs=/var/tmp/n83-full-m4-build-logs
output="$root/full-m4-v1"
test "$(cat "$root/xor-cache-v2/card-status.txt")" = COMPLETE_BOUNDED
test "$(cat "$logs/tests-exit.txt")" = 0
test "$(cat "$logs/build-exit.txt")" = 0
test "$(git -C "$source" rev-parse HEAD)" = d9ef42301cc3d22f437e1f89304e673836afb6a3
git -C "$source" diff --exit-code
git -C "$source" diff --cached --exit-code
test ! -e "$output"
mkdir "$output"
cp "$root/target-small-m4/release/examples/n83_prebuild_cost_m23" "$output/worker"
cp "$root/target-small-m4/release/examples/n83_small_m4_replay" "$output/replayer"
cp "$source/research/n83_prebuild_cost_m23_20261019/run-full-m4-card.sh" "$output/"
cp "$source/research/n83_prebuild_cost_m23_20261019/run-full-m4-docker.sh" "$output/"
cp /var/tmp/n83-cache-audit-final.json "$output/cache-audit.json"
cp "$logs/source-commit.txt" "$output/"
cp -R "$logs" "$output/build"
printf '%s  %s\n' \
  d4324884fdd52ee05a82e2f053708d320d8255591ec61640b700a8a6684a0486 "$output/worker" \
  1728ef0d3423ba61bab8ec98db3f4d06d256eb05ec0b12b04a6ce812c77f1425 "$output/replayer" \
  6da91e69eb5671e514e627ecdbf6ebd7383c738a86f652989e73c86994c76270 "$output/run-full-m4-card.sh" \
  ae6148f425e7cad875012316bef78f2784e216519e441a13f0a34fbee4f2b1ae "$output/run-full-m4-docker.sh" \
  da7f5df53baaeba77091cb7aea843375b019adaf65533a20a416d634ae71f100 "$output/cache-audit.json" \
  0c09f62bde61d3c3e987644508c53ab56fd3f545219013b9bde925ba29969275 "$root/inputs/U/manifest.json" \
  2575cea6b16cfdaa02531c79ca4797d976eb861def3182983c3048ce75dea045 "$root/inputs/D/manifest.json" \
  2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369 "$root/inputs/U/objects/1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d.jsonl.gz" \
  347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7 "$root/inputs/D/objects/e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682.jsonl.gz" \
  2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5 "$root/inputs/public/public-targets.json" \
  > "$output/inputs.sha256"
sha256sum -c "$output/inputs.sha256" > "$output/prepare-check.log"
sudo docker image inspect amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f > "$output/prelaunch-image.json"
