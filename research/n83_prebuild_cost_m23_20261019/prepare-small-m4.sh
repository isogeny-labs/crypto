#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
source="$root/source-small-m4"
output="$root/small-m4-v1"
test ! -e "$output"
test "$(git -C "$source" rev-parse HEAD)" = 611aad60e4d9952d6239a3382a77184da3640eeb
git -C "$source" diff --exit-code
git -C "$source" diff --cached --exit-code
mkdir "$output"
cp "$root/target-small-m4/release/examples/n83_prebuild_cost_m23" "$output/worker"
cp "$root/target-small-m4/release/examples/n83_small_m4_replay" "$output/replayer"
cp "$source/research/n83_prebuild_cost_m23_20261019/run-small-m4-card.sh" "$output/"
cp "$source/research/n83_prebuild_cost_m23_20261019/run-small-m4-docker.sh" "$output/"
printf '%s\n' 611aad60e4d9952d6239a3382a77184da3640eeb > "$output/source-commit.txt"
printf '%s  %s\n' \
  3048550afc4ad1bdad1e390f74802112dfc6fadf96048cbff823f8f65364b0c3 "$output/worker" \
  359968c67994521198c4b8b69f599b0f937bf080c2f0c26046346e021fc23b22 "$output/replayer" \
  fcb7898efa4f72999fdf9301720fd7056d2ae9beb44800e6a31809ecc6318f5a "$source/examples/n83_prebuild_cost_m23.rs" \
  d261edc668b8f7fdb3ebfb16ae42d724b3ab9e16b6cc787aa1d6f94670934dc4 "$source/examples/n83_small_m4_replay.rs" \
  d8c2fc23d93f32a78ed7cf7c98994266721308f76d74546e78d089ba1e4a2e7f "$source/research/n83_prebuild_cost_m23_20261019/small_m4.rs" \
  70827381430f9ef514476d997091311a5c09132cbbf4136ff31674dfe76b7442 "$output/run-small-m4-card.sh" \
  2bd6992965af930c5038563dba2cbd6e5a18e0c426a20f00665779e31ba3006f "$output/run-small-m4-docker.sh" \
  68b43524198394a2f0d112f86df0d1c1f69ea1ff817695cf21efe2767b7d17b8 "$root/inputs/K2048/manifest.json" \
  bd4202ed599f2659f6279dd07dbdc6b5948594df79db969e66530c19e3cf5dca "$root/inputs/D12/manifest.json" \
  aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a "$root/inputs/K2048/objects/82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851.jsonl.gz" \
  c5de0b26e84ad80bfb7be6a214b89c9682e356effb6079c7abf28cd1151ffb03 "$root/inputs/D12/objects/aea278aff2d79a592d5164d7437ddde8f8e9377fa481058dfc16afa176a31389.jsonl.gz" \
  2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5 "$root/inputs/public/public-targets.json" \
  > "$output/inputs.sha256"
sha256sum -c "$output/inputs.sha256" > "$output/prepare-check.log"
rustc --version > "$output/rustc-version.txt"
cp "$root/small-m4-build.log" "$output/build-first.log"
cp "$root/small-m4-build-2.log" "$output/build-success.log"
