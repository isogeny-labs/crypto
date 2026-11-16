#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
source="$root/source-small"
binary="$root/target-small/release/examples/n83_prebuild_cost_m23"
runner="$root/run-small-base-docker.sh"
base="$root/inputs/D12"
corpus="$root/inputs/public/public-targets.json"
runs="$root/small-base-v1/runs"
test "$(git -C "$source" rev-parse HEAD)" = d4badc176820cb8bad63246ad8db02e47ae4cde1
sha256sum -c "$root/small-base-inputs.sha256" > "$root/small-base-resume-input-check.log"
test ! -e "$root/small-base-v1/run-card-status.txt"
test ! -e "$root/small-base-v1/panel-status.txt"
test -d "$runs/t7-m3-d"
test ! -e "$runs/t7-m3-d/worker.json"
test ! -e "$runs/t7-m3-d/host-exit.txt"
test "$(find "$runs" -name worker.json | wc -l)" -eq 74
test "$(find "$runs" -name host-exit.txt | wc -l)" -eq 74
if find "$runs" -name host-exit.txt -exec cat {} + | grep -v '^0$'; then
    echo 'original completed host exit differs from zero' >&2
    exit 2
fi
"$runner" control D12 "$runs/control-D12-m3-postresume" "$binary" "$base" "$corpus" 3
"$runner" probe D12 "$runs/t7-m3-d-retry1" "$binary" "$base" "$corpus" 7 3
"$runner" replay D12 "$runs/t7-m3-d-retry1-replay" "$binary" "$base" "$corpus" "$runs/t7-m3-d-retry1/worker.json"
printf '%s\n' COMPLETE_WITH_SPOT_RETRY > "$root/small-base-v1/panel-status.txt"
