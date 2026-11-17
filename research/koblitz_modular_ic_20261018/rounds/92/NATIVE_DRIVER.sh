#!/usr/bin/env bash
set -euo pipefail
here=${1:?round};raw=${2:?raw};input=${3:?input};marker=${4:?marker}
repo=$(git -C "$here" rev-parse --show-toplevel)
revision=$(git -C "$repo" rev-parse HEAD)
printf '%s\n' "$revision" > "$raw/source-revision.txt"
while IFS='|' read -r local_name source_name; do
  git -C "$repo" show "$revision:$(realpath --relative-to="$repo" "$here/$source_name")" | cmp - "$raw/$local_name"
done <<EOF
driver.sh|NATIVE_DRIVER.sh
launcher.sh|REPLAY_CAS.sh
input.sing|$input
PROTOCOL.md|PROTOCOL.md
protocol.json|protocol.json
donors.json|donors.json
EOF
while IFS=$'\t' read -r donor_path expected_hash expected_bytes; do
  actual_hash=$(sha256sum "$here/$donor_path" | cut -d' ' -f1)
  actual_bytes=$(wc -c < "$here/$donor_path")
  test "$actual_hash" = "$expected_hash" && test "$actual_bytes" = "$expected_bytes"
done < <(jq -r '.[] | [.path,.sha256,.bytes] | @tsv' "$raw/donors.json")
cmp "$here/../50/runs/rr-pid50-01/rr-pid50.sing" "$raw/rr-pid50.sing"
cmp "$here/../50/source-data/relative-infinity50.sing" "$raw/relative-infinity50.sing"
(cd "$raw"; sha256sum driver.sh launcher.sh input.sing rr-pid50.sing relative-infinity50.sing PROTOCOL.md protocol.json donors.json > source.sha256)
max_degree=$(rg '^basis50\[[0-9]+\]=' "$raw/rr-pid50.sing" | rg -o 'u\^[0-9]+' | cut -d'^' -f2 | sort -n | tail -1)
test "$max_degree" = 37
printf 'basis_columns=32 max_coefficient_degree=%s common_denominator_degree=37 multiplication_pairs=528\n' "$max_degree" > "$raw/basis-degree.txt"
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
binary=/home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular
sha256sum "$binary" > "$raw/binary.sha256"
date -u +%FT%TZ > "$raw/started-utc.txt"
set +e
(cd "$raw"; ulimit -v 4194304; timeout 1200 "$binary" --random=92 --threads=1 --flint-threads=1 -q input.sing) > "$raw/native.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt"
date -u +%FT%TZ > "$raw/finished-utc.txt"
set -e
tar -cJf "$here/checkpoint-$(basename "$raw").tar.xz" -C "$here" "runs/$(basename "$raw")"
if [ "$rc" != 0 ] || rg -q '^[[:space:]]*\?' "$raw/native.log" || ! rg -qx "$marker" "$raw/native.log";then echo "FAIL $(basename "$raw") native=$rc";exit 1;fi
echo "PASS $(basename "$raw")"
