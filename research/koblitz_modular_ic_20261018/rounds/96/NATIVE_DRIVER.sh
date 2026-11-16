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
cmp "$here/../52/source-data/marked23-field51.sing" "$raw/marked23-field51.sing"
(cd "$raw";sha256sum driver.sh launcher.sh input.sing marked23-field51.sing PROTOCOL.md protocol.json donors.json > source.sha256)
"$repo/scripts/checkpoint.sh" gate > "$raw/gate.log" 2>&1
binary=/home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular
sha256sum "$binary" > "$raw/binary.sha256"
date -u +%FT%TZ > "$raw/started-utc.txt"
set +e
(cd "$raw";ulimit -v 2097152;timeout 300 "$binary" --random=96 --threads=1 --flint-threads=1 -q input.sing) > "$raw/native.log" 2>&1
rc=$?;printf '%s\n' "$rc" > "$raw/exit.txt"
date -u +%FT%TZ > "$raw/finished-utc.txt"
set -e
tar -cJf "$here/checkpoint-$(basename "$raw").tar.xz" -C "$here" "runs/$(basename "$raw")"
if [ "$rc" != 0 ] || rg -q '^[[:space:]]*\?' "$raw/native.log" || ! rg -qx "$marker" "$raw/native.log";then echo "FAIL $(basename "$raw") native=$rc";exit 1;fi
echo "PASS $(basename "$raw")"
