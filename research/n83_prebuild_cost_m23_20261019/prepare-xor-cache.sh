#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
source=/var/tmp/n83-xor-cache-source
output="$root/xor-cache-v1"
test ! -e "$output/runs"
test ! -e "$output/inputs.sha256"
test "$(git -C "$source" rev-parse HEAD)" = 1de47a9d4186bff08abc84c224a111183e820106
git -C "$source" diff --exit-code
git -C "$source" diff --cached --exit-code
test "$(cat /var/tmp/n83-xor-cache-build-logs/release-lib-exit.txt)" = 0
test "$(cat /var/tmp/n83-xor-cache-build-logs/example-tests-v2-exit.txt)" = 0
printf '%s  %s\n' \
  a9b8599761c6cce8a169602086c3426af9f9f7a55a8108124daed56059ffb3bc "$output/worker" \
  9ed4476c8869d0fc124a4f68392fc968071c1b804fa1688796ca2ab062e55861 "$output/replayer" \
  6db607a8552a0144c944f8b43d59aec60eb19b77f83245e67b2c30204bdf239e "$source/examples/n83_prebuild_cost_m23.rs" \
  3ed0029faf924e325dc0174371f90c810852dbccd8d68b7c461ad8d946a5549e "$source/examples/n83_small_m4_replay.rs" \
  ce0541df1e9e7b863aa35a1cc8ffd00aac13079d3ac9aabe78c1b8417335ad11 "$source/research/n83_prebuild_cost_m23_20261019/small_m4.rs" \
  658ae9c01dab2444f52e22f6b19498aafee3fe1145e0380018626f19db5895f6 "$source/src/cryptanalysis/sat.rs" \
  4477ba28e581f3bccaaf4de1666026b9c67b2d127f5fc5cd2d5676d80106009a "$output/run-xor-cache-card.sh" \
  f5c24df168febec056033ddb374d7316185c3dd180e83339aadafe00ae5d7439 "$output/run-xor-cache-docker.sh" \
  68b43524198394a2f0d112f86df0d1c1f69ea1ff817695cf21efe2767b7d17b8 "$root/inputs/K2048/manifest.json" \
  bd4202ed599f2659f6279dd07dbdc6b5948594df79db969e66530c19e3cf5dca "$root/inputs/D12/manifest.json" \
  aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a "$root/inputs/K2048/objects/82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851.jsonl.gz" \
  c5de0b26e84ad80bfb7be6a214b89c9682e356effb6079c7abf28cd1151ffb03 "$root/inputs/D12/objects/aea278aff2d79a592d5164d7437ddde8f8e9377fa481058dfc16afa176a31389.jsonl.gz" \
  2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5 "$root/inputs/public/public-targets.json" \
  > "$output/inputs.sha256"
sha256sum -c "$output/inputs.sha256" > "$output/prepare-check.log"
