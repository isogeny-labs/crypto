#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
source="$root/source-small"
worker="$root/target-small/release/examples/n83_prebuild_cost_m23"
runner="$root/run-small-base-docker.sh"
corpus="$root/inputs/public/public-targets.json"
output="$root/small-base-v1"
if [ -e "$output" ]; then echo 'small-base output already exists' >&2; exit 2; fi
test "$(git -C "$source" rev-parse HEAD)" = d4badc176820cb8bad63246ad8db02e47ae4cde1
printf '%s  %s\n' \
  '3f2572663880fc010f3fb2aee33114d5be4f4192e60f0ead4bc71f1b5e3e895f' "$worker" \
  'd0a6b12dc4d57fc813ca3b0d2628e2d1fd547f9da3424f4113c3e36223a23257' "$source/examples/n83_prebuild_cost_m23.rs" \
  '4082025744ae3c4d95f1eb270c334ce73330e0db5a09b19887b1b8af5617edd7' "$source/examples/n83_m23_control_replay.rs" \
  '000ec9781cf5def8716f7456c0aeb98ab0483004a4abb3e9d52c8f7fb669de05' "$runner" \
  '68b43524198394a2f0d112f86df0d1c1f69ea1ff817695cf21efe2767b7d17b8' "$root/inputs/K2048/manifest.json" \
  'bd4202ed599f2659f6279dd07dbdc6b5948594df79db969e66530c19e3cf5dca' "$root/inputs/D12/manifest.json" \
  'aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a' "$root/inputs/K2048/objects/82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851.jsonl.gz" \
  'c5de0b26e84ad80bfb7be6a214b89c9682e356effb6079c7abf28cd1151ffb03' "$root/inputs/D12/objects/aea278aff2d79a592d5164d7437ddde8f8e9377fa481058dfc16afa176a31389.jsonl.gz" \
  '2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5' "$corpus" \
  > "$root/small-base-inputs.sha256"
sha256sum -c "$root/small-base-inputs.sha256" > "$root/small-base-input-check.log"
mkdir -p "$output/runs"
runs="$output/runs"
for arm in K2048 D12; do
    for m in 2 3; do
        "$runner" control "$arm" "$runs/control-$arm-m$m" "$worker" "$root/inputs/$arm" "$corpus" "$m"
    done
done
probe_and_replay() {
    arm=$1
    label=$2
    target=$3
    m=$4
    "$runner" probe "$arm" "$runs/$label" "$worker" "$root/inputs/$arm" "$corpus" "$target" "$m"
    "$runner" replay "$arm" "$runs/$label-replay" "$worker" "$root/inputs/$arm" "$corpus" "$runs/$label/worker.json"
}
for m in 2 3; do
    probe_and_replay K2048 "t0-m$m-k1" 0 "$m"
    probe_and_replay D12 "t0-m$m-d1" 0 "$m"
    probe_and_replay D12 "t0-m$m-d2" 0 "$m"
    probe_and_replay K2048 "t0-m$m-k2" 0 "$m"
done
for target in 1 2 3 4 5 6 7; do
    for m in 2 3; do
        probe_and_replay K2048 "t$target-m$m-k" "$target" "$m"
        probe_and_replay D12 "t$target-m$m-d" "$target" "$m"
    done
done
printf '%s\n' COMPLETE > "$output/run-card-status.txt"
