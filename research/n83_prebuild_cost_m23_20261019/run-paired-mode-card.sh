#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
worker=/home/ec2-user/n83-d18-paired-20261019/target/release/examples/n83_prebuild_cost_m23
runner="$root/run-paired-mode-docker.sh"
base="$root/inputs/D"
corpus="$root/inputs/public/public-targets.json"
output="$root/paired-mode-v1"
if [ -e "$output" ]; then echo 'paired-mode output already exists' >&2; exit 2; fi
printf '%s  %s\n' \
  '80a53099ff4db99519de3d3700bbde3948dc0b435e6a3c7ac800164ac7450a24' "$worker" \
  '8600970b3b73a2ff3dab7f642809ea0496407d892d0c347f328c497e7eedbe7d' "$root/source-paired/examples/n83_prebuild_cost_m23.rs" \
  '0618cb9f87f6a0c556525a845a243095b567dcd35747cbef40025d60ada3071d' "$runner" \
  '2575cea6b16cfdaa02531c79ca4797d976eb861def3182983c3048ce75dea045' "$base/manifest.json" \
  '347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7' "$base/objects/e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682.jsonl.gz" \
  '2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5' "$corpus" \
  > "$root/paired-mode-inputs.sha256"
sha256sum -c "$root/paired-mode-inputs.sha256" > "$root/paired-mode-input-check.log"
mkdir -p "$output/runs"
runs="$output/runs"
"$runner" setup-prebuilt "$runs/setup-prebuilt" "$worker" "$base" "$corpus"
"$runner" setup-ondemand "$runs/setup-ondemand" "$worker" "$base" "$corpus"
"$runner" control-m2 "$runs/control-m2" "$worker" "$base" "$corpus"
"$runner" control-m3 "$runs/control-m3" "$worker" "$base" "$corpus"
probe_and_replay() {
    mode=$1
    label=$2
    target=$3
    summands=$4
    "$runner" "probe-$mode" "$runs/$label" "$worker" "$base" "$corpus" "$target" "$summands"
    "$runner" replay "$runs/$label-replay" "$worker" "$base" "$corpus" "$runs/$label/worker.json"
}
for summands in 2 3; do
    probe_and_replay prebuilt "t0-m$summands-p1" 0 "$summands"
    probe_and_replay ondemand "t0-m$summands-o1" 0 "$summands"
    probe_and_replay ondemand "t0-m$summands-o2" 0 "$summands"
    probe_and_replay prebuilt "t0-m$summands-p2" 0 "$summands"
done
for target in 1 2 3 4 5 6 7; do
    for summands in 2 3; do
        probe_and_replay prebuilt "t$target-m$summands-p" "$target" "$summands"
        probe_and_replay ondemand "t$target-m$summands-o" "$target" "$summands"
    done
done
printf '%s\n' COMPLETE > "$output/run-card-status.txt"
