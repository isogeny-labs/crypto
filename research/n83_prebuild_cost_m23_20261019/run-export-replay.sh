#!/bin/sh
set -eu
root=/home/ec2-user/n83-m23-20261019
binary=/home/ec2-user/n83-d18-paired-20261019/target/release/examples/n83_d18_subgroup_orbit_export
source="$root/source"
export_root="$root/export-cost-v3/generated"
output="$root/export-cost-v3/replay-cost"
if [ -e "$output" ] || [ -e "$export_root/replay.json" ]; then echo 'replay output already exists' >&2; exit 2; fi
if [ "$(git -C "$source" rev-parse HEAD)" != b901026b6b05cd1fe9b909620d6746a5a931520c ]; then echo 'source commit differs' >&2; exit 2; fi
printf '%s  %s\n' \
  '402aa926b4ac5e75d7fa96d62ccde36c349ec3b33894df0cc1500d2c10c940f7' "$binary" \
  '347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7' "$export_root/objects/e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682.jsonl.gz" \
  > "$root/export-replay-inputs.sha256"
sha256sum -c "$root/export-replay-inputs.sha256" > "$root/export-replay-input-check.log"
mkdir "$output"
cd "$source"
date -u +%FT%TZ > "$output/start.utc"
set +e
systemd-run --user --scope -p MemoryMax=4294967296 -p MemorySwapMax=0 -p CPUQuota=100% \
    /usr/bin/taskset -c 31 /usr/bin/time -v -o "$output/process-time.txt" \
    /bin/sh "$root/export-replay-worker.sh" "$output" "$export_root" \
    > "$output/host-stdout.log" 2> "$output/host-stderr.log"
result=$?
set -e
date -u +%FT%TZ > "$output/end.utc"
printf '%s\n' "$result" > "$output/host-exit.txt"
if [ "$result" -ne 0 ]; then exit "$result"; fi
jq -e '.status == "PASS" and .points_checked == 2760082 and .representatives_checked == 16627 and .compressed_sha256 == "347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7"' "$export_root/replay.json" > "$output/replay-check.txt"
