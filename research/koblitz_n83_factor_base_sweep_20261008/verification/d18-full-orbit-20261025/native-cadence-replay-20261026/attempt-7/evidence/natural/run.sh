#!/bin/zsh
set -euo pipefail

source_dir=/Volumes/SSD990-2/tmp/n83-source-45ce-20261114
run_dir=/Volumes/SSD990-2/tmp/n83-d18-attempt7-20261114
base_dir=$run_dir/input
out_dir=$run_dir/natural
worker=/Volumes/SSD990-2/tmp/n83-d18-target-20261114/release/examples/koblitz_n83_factor_base_export
fixture_dir=$source_dir/research/koblitz_n83_factor_base_sweep_20261008/pilot-01
result=$out_dir/result.json

[[ $(git -C "$source_dir" rev-parse HEAD) == 45ce91d503ee0d0acc57e3d8d5cb49108c90ded6 ]]
[[ -z $(git -C "$source_dir" status --porcelain) ]]
printf '%s  %s\n' 5f302b8cfd86470e4d08da34c972308f0c8b5dbc80422dc8b578625f8e075fd2 "$worker" | shasum -a 256 -c -
printf '%s  %s\n' 1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce "$base_dir/manifest.json" | shasum -a 256 -c -
printf '%s  %s\n' 1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616 "$base_dir/s3-manifest.json" | shasum -a 256 -c -
printf '%s  %s\n' eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b "$base_dir/objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz" | shasum -a 256 -c -

date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out_dir/start-utc.txt"
sysctl -n vm.loadavg > "$out_dir/start-load.txt"
vm_stat | rg 'Swapouts:' > "$out_dir/start-swapouts.txt"
df -k /Volumes/SSD990-2 > "$out_dir/start-scratch.txt"
load_1m=$(awk '{print $2}' "$out_dir/start-load.txt")
swap_now=$(awk '{gsub(/\./,"",$2); print $2}' "$out_dir/start-swapouts.txt")
swap_prior=$(awk '/Swapouts:/ {gsub(/\./,"",$2); value=$2} END {print value}' "$run_dir/host-preflight-natural.txt")
scratch_kb=$(awk 'NR==2 {print $4}' "$out_dir/start-scratch.txt")
awk -v load="$load_1m" 'BEGIN {exit !(load <= 24)}'
[[ "$swap_now" == "$swap_prior" ]]
(( scratch_kb >= 2097152 ))

set +e
gtimeout --signal=TERM --kill-after=5s 2410s /usr/bin/time -p "$worker" primary-full-d18-m4-l0-ordered-xor-cadence32 natural "$base_dir" "$fixture_dir" "$result" > "$out_dir/stdout.txt" 2> "$out_dir/stderr.txt"
worker_exit=$?
set -e
printf '%s\n' "$worker_exit" > "$result.exit.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out_dir/end-utc.txt"
sysctl -n vm.loadavg > "$out_dir/end-load.txt"
vm_stat | rg 'Swapouts:' > "$out_dir/end-swapouts.txt"
printf 'worker_exit=%s\n' "$worker_exit"
