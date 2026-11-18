#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
worker=/home/ec2-user/n83-d18-paired-20261019/target/release/examples/n83_prebuild_cost_m23
runner="$root/run-spot-docker.sh"
corpus="$root/inputs/public/public-targets.json"
runs="$root/runs"

printf '%s  %s\n' \
  '2859910318cc514e21aa6136d3263fd5daa2c8d1642fc6668448a76943d906c8' "$worker" \
  '7ef1ea404378e9f2d84e263ee8e46ce366d55b88d65327afbf9d84bd81bce40d' "$runner" \
  'ba2c7d976ba4eb943073bb3ff7bac5fcb69b795c9225572922b043f04a7d26d7' "$root/source/examples/n83_prebuild_cost_m23.rs" \
  '0c09f62bde61d3c3e987644508c53ab56fd3f545219013b9bde925ba29969275' "$root/inputs/U/manifest.json" \
  '2575cea6b16cfdaa02531c79ca4797d976eb861def3182983c3048ce75dea045' "$root/inputs/D/manifest.json" \
  '2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5' "$corpus" \
  > "$root/run-inputs.sha256"
sha256sum -c "$root/run-inputs.sha256" > "$root/run-input-check.log"

"$runner" setup-prebuilt D "$runs/setup-prebuilt" "$worker" "$root/inputs/D" "$corpus"
"$runner" setup-ondemand D "$runs/setup-ondemand" "$worker" "$root/inputs/D" "$corpus"
for arm in U D; do
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
    probe_and_replay U "t0-m$m-u1" 0 "$m"
    probe_and_replay D "t0-m$m-d1" 0 "$m"
    probe_and_replay D "t0-m$m-d2" 0 "$m"
    probe_and_replay U "t0-m$m-u2" 0 "$m"
done
for target in 1 2 3 4 5 6 7; do
    for m in 2 3; do
        probe_and_replay U "t$target-m$m-u" "$target" "$m"
        probe_and_replay D "t$target-m$m-d" "$target" "$m"
    done
done
printf '%s\n' COMPLETE > "$root/run-card-status.txt"
