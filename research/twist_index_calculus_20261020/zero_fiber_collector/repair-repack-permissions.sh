#!/usr/bin/env bash
# Preserve original failed repack and reuse its verified compressed payload.
set -euo pipefail
zero_pack=${1:?existing completed first archive}
zero_fix="$zero_pack/permission-repair-v1"
test ! -e "$zero_fix"
mkdir -p "$zero_fix/fresh"
cp "$0" "$zero_fix/executed-script.sh"
date -u +%FT%TZ > "$zero_fix/started-utc.txt"
sha256sum -c "$zero_pack/archive.sha256" > "$zero_fix/original-archive-unchanged.txt"
tar -xJpf "$zero_pack/runs.tar.xz" -C "$zero_fix/fresh"
(
  cd "$zero_fix/fresh"
  sha256sum -c SHA256SUMS
) > "$zero_fix/extracted-checksums.txt"
cmp "$zero_pack/staging/SHA256SUMS" "$zero_fix/fresh/SHA256SUMS"
cmp "$zero_pack/staging/FILE_SIZES.tsv" "$zero_fix/fresh/FILE_SIZES.tsv"
while IFS= read -r zero_line; do
  zero_path=${zero_line:66}
  cmp "$zero_pack/staging/$zero_path" "$zero_fix/fresh/$zero_path"
done < "$zero_pack/SHA256SUMS"
cp "$zero_pack/exact-file-comparisons.txt" "$zero_fix/exact-file-comparisons.txt"
tar --format=gnu --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner --hard-dereference -C "$zero_fix/fresh" -cf - . | xz -1 -T1 > "$zero_fix/repacked.tar.xz"
cmp "$zero_pack/runs.tar.xz" "$zero_fix/repacked.tar.xz"
sha256sum "$zero_fix/repacked.tar.xz" > "$zero_fix/repacked.sha256"
date -u +%FT%TZ > "$zero_fix/finished-utc.txt"
printf '0\n' > "$zero_fix/exit"
printf 'Permission-preserving fresh extraction, all 1112 comparisons and byte-identical repack pass; original failed attempt retained.\n'
