#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
source="$root/source"
binary=/home/ec2-user/n83-d18-paired-20261019/target/release/examples/n83_d18_subgroup_orbit_census
worker="$root/run-census-cost-worker.sh"
output="$root/census-cost-v1"
if [ -e "$output" ]; then echo 'census output already exists' >&2; exit 2; fi
test "$(git -C "$source" rev-parse HEAD)" = b901026b6b05cd1fe9b909620d6746a5a931520c
printf '%s  %s\n' \
  'd65b8a9432147f1d66f4b449e2e4d9e0c02459934111f3358985cc479a08467b' "$source/examples/n83_d18_subgroup_orbit_census.rs" \
  '2ad7e1fb0126081a247088ba1888b4656e60fb29379be9bc232d3605816c9653' "$binary" \
  'e81e8c3aedfbd782156ef3e2dcab694ba4a5d859c346641c306bfe6adaf3fdc3' "$worker" \
  > "$root/census-cost-inputs.sha256"
sha256sum -c "$root/census-cost-inputs.sha256" > "$root/census-cost-input-check.log"
mkdir "$output"
date -u +%FT%TZ > "$output/start.utc"
set +e
systemd-run --user --scope -p MemoryMax=4294967296 -p MemorySwapMax=0 -p CPUQuota=100% \
    /usr/bin/taskset -c 31 /usr/bin/time -v -o "$output/process-time.txt" \
    /bin/sh "$worker" "$output" "$binary" \
    > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
set -e
date -u +%FT%TZ > "$output/end.utc"
printf '%s\n' "$result" > "$output/host-exit.txt"
if [ "$result" -ne 0 ]; then exit "$result"; fi
test "$(sha256sum "$output/generated/selected-representatives.csv" | cut -d ' ' -f1)" = 534e58ced979e05ec747eb462aa1d63135d8004813d094c522a4d7160c03c4a4
test "$(jq -r '.enumerated_x' "$output/generated/summary.json")" = 262144
test "$(jq -r '.full_signed_frobenius_orbit_classes' "$output/generated/summary.json")" = 32643
test "$(jq -r '.selected_columns' "$output/generated/summary.json")" = 16627
test "$(jq -r '.selected_representatives_blake3' "$output/generated/summary.json")" = 2d5e7b6a5ce1574183d0b236c601f5c2ac5d47008c520b923338fd52e6dc6a44
printf '%s\n' COMPLETE > "$output/run-card-status.txt"
