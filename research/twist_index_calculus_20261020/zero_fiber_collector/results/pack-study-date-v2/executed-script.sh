#!/usr/bin/env bash
# Artifact metadata only. Reuse the already verified payload; no science rerun.
set -euo pipefail
zero_old=${1:?verified original packaging directory}
zero_out=${2:?fresh normalized archive directory}
zero_source="$zero_old/permission-repair-v1/fresh"
test "$(cat "$zero_old/permission-repair-v1/exit")" = 0
test ! -e "$zero_out"
mkdir -p "$zero_out/fresh"
zero_out=$(cd "$zero_out" && pwd)
cp "$0" "$zero_out/executed-script.sh"
git rev-parse HEAD > "$zero_out/packing-source-commit.txt"
date -u +%FT%TZ > "$zero_out/started-utc.txt"
(
  cd "$zero_source"
  sha256sum -c SHA256SUMS
) > "$zero_out/source-checksums.txt"
cp "$zero_old/archive.sha256" "$zero_out/prior-local-archive.sha256"
cp "$zero_old/selected-raw-directories.txt" "$zero_out/selected-raw-directories.txt"
cp "$zero_old/verified-native-summary.json" "$zero_out/verified-native-summary.json"
cp "$zero_source/SHA256SUMS" "$zero_out/SHA256SUMS"
cp "$zero_source/FILE_SIZES.tsv" "$zero_out/FILE_SIZES.tsv"
tar --format=gnu --sort=name --mtime='2026-10-24 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --hard-dereference -C "$zero_source" -cf - . | xz -1 -T1 > "$zero_out/runs.tar.xz"
sha256sum "$zero_out/runs.tar.xz" > "$zero_out/archive.sha256"
stat -c '%s' "$zero_out/runs.tar.xz" > "$zero_out/archive.bytes"
tar -xJpf "$zero_out/runs.tar.xz" -C "$zero_out/fresh"
(
  cd "$zero_out/fresh"
  sha256sum -c SHA256SUMS
) > "$zero_out/extracted-checksums.txt"
cmp "$zero_source/SHA256SUMS" "$zero_out/fresh/SHA256SUMS"
cmp "$zero_source/FILE_SIZES.tsv" "$zero_out/fresh/FILE_SIZES.tsv"
while IFS= read -r zero_line; do
  zero_path=${zero_line:66}
  cmp "$zero_source/$zero_path" "$zero_out/fresh/$zero_path"
done < "$zero_out/SHA256SUMS"
cp "$zero_old/exact-file-comparisons.txt" "$zero_out/exact-file-comparisons.txt"
tar --format=gnu --sort=name --mtime='2026-10-24 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --hard-dereference -C "$zero_out/fresh" -cf - . | xz -1 -T1 > "$zero_out/repacked.tar.xz"
cmp "$zero_out/runs.tar.xz" "$zero_out/repacked.tar.xz"
sha256sum "$zero_out/repacked.tar.xz" > "$zero_out/repacked.sha256"
date -u +%FT%TZ > "$zero_out/finished-utc.txt"
printf '0\n' > "$zero_out/exit"
printf 'Study-date and mode-normalized archive: fresh checksums, 1112 byte comparisons and identical repack pass.\n'
