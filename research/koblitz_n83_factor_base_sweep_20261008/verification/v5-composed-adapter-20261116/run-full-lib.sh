#!/usr/bin/env bash
set -euo pipefail
set -C

source_root=/Volumes/SSD990/tmp/n83-k600-budget-v3-20261116
out="$(git rev-parse --show-toplevel)/research/koblitz_n83_factor_base_sweep_20261008/verification/v5-composed-adapter-20261116"
base=a91b8b66d0f1ffc175fdce984d1059144ce232f6
head=a8821d64550526c3151eca205a9751ca4d8aa8a5
test "$(git -C "$source_root" rev-parse HEAD)" = "$head"
test "$(git -C "$source_root" status --porcelain)" = ""
git -C "$source_root" diff --quiet "$base" "$head" -- src Cargo.toml Cargo.lock tests gpu tools
for name in full-lib.start.utc full-lib.end.utc full-lib.exit.txt full-lib.log full-lib.disk.tsv full-lib.source-commit.txt full-lib.disk-stop.txt; do
  test ! -e "$out/$name"
done
date -u +%Y-%m-%dT%H:%M:%SZ > "$out/full-lib.start.utc"
printf '%s\n' "$head" > "$out/full-lib.source-commit.txt"
printf 'utc\tboot_available_kib\tssd_available_kib\n' > "$out/full-lib.disk.tsv"

set +e
(
  cd "$source_root"
  env RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
    CARGO_TARGET_DIR=/Volumes/SSD990/tmp/n83-k600-budget-v3-target \
    TMPDIR=/Volumes/SSD990/tmp \
    gtimeout -s TERM -k 30s 7200s cargo test --release --lib --locked --offline -j 1
) > "$out/full-lib.log" 2>&1 &
child=$!
while kill -0 "$child" 2>/dev/null; do
  boot_kib="$(df -Pk / | awk 'NR==2 {print $4}')"
  ssd_kib="$(df -Pk /Volumes/SSD990 | awk 'NR==2 {print $4}')"
  printf '%s\t%s\t%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$boot_kib" "$ssd_kib" >> "$out/full-lib.disk.tsv"
  if (( boot_kib < 2097152 || ssd_kib < 2097152 )); then
    printf 'disk floor reached: boot=%s KiB ssd=%s KiB\n' "$boot_kib" "$ssd_kib" > "$out/full-lib.disk-stop.txt"
    kill -TERM "$child"
    break
  fi
  sleep 30
done
wait "$child"
status=$?
set -e

date -u +%Y-%m-%dT%H:%M:%SZ > "$out/full-lib.end.utc"
printf '%s\n' "$status" > "$out/full-lib.exit.txt"
exit "$status"
