#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
binary=/home/ec2-user/n83-d18-paired-20261019/target/release/examples/n83_d18_subgroup_orbit_export
source="$root/source"
output="$root/export-cost-v1"
if [ -e "$output" ]; then echo 'export cost output already exists' >&2; exit 2; fi
printf '%s  %s\n' \
  '402aa926b4ac5e75d7fa96d62ccde36c349ec3b33894df0cc1500d2c10c940f7' "$binary" \
  '6c4ac246e99ca1b54df81aaead085c7e2e66e1083d527965466127d67af5c09f' "$source/examples/n83_d18_subgroup_orbit_export.rs" \
  '534e58ced979e05ec747eb462aa1d63135d8004813d094c522a4d7160c03c4a4' "$source/research/n83_d18_subgroup_orbit_census_20261018/run-20261019T0142Z/selected-representatives.csv" \
  > "$root/export-inputs.sha256"
sha256sum -c "$root/export-inputs.sha256" > "$root/export-input-check.log"
mkdir "$output"
cd "$source"
date -u +%FT%TZ > "$output/start.utc"
set +e
systemd-run --user --scope -p MemoryMax=4294967296 -p MemorySwapMax=0 -p CPUQuota=100% \
    /usr/bin/taskset -c 31 /usr/bin/time -v -o "$output/process-time.txt" \
    /bin/sh "$root/export-worker.sh" "$output" \
    > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
set -e
date -u +%FT%TZ > "$output/end.utc"
printf '%s\n' "$result" > "$output/host-exit.txt"
if [ "$result" -ne 0 ]; then exit "$result"; fi
sha256sum "$output/objects/e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682.jsonl.gz" > "$output/object.sha256"
expected='347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7'
actual=$(cut -d ' ' -f1 "$output/object.sha256")
if [ "$actual" != "$expected" ]; then echo 'export object differs from frozen D archive' >&2; exit 1; fi
