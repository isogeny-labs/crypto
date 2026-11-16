#!/usr/bin/env bash
set -euo pipefail
repo=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-full-validation-20261116
out=/Volumes/SSD990-2/tmp/n83-colima-output/k600-preflight-mac-supervisor-build-20261116-attempt-2
source_commit=ba620214684e41def8bc6caddf2e782f43920218
floor_kib=$((20 * 1024 * 1024))
test ! -e "$out"
test "$(git -C "$repo" status --porcelain)" = ""
git -C "$repo" diff --quiet "$source_commit" HEAD -- src examples Cargo.toml Cargo.lock crates gpu tools
test -x /Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc
mkdir -p "$out"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/start.utc"
printf '%s\n' "$(git -C "$repo" rev-parse HEAD)" > "$out/source.commit"
printf '%s\n' "$(/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc --version)" > "$out/rustc.version"
(
  cd "$repo"
  env CARGO_TARGET_DIR="$out/target" RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc gtimeout -s TERM -k 20s 2400s \
    rustup run 1.99.1-aarch64-apple-darwin cargo build --release --locked --offline -j 1 \
      --example n83_k600_m6_native_preflight
) > "$out/stdout.log" 2> "$out/stderr.log" &
child=$!
(
  while kill -0 "$child" 2>/dev/null; do
    available=$(df -Pk /Volumes/SSD990-2 | awk 'NR==2 {print $4}')
    printf '%s\t%s\n' "$(date -u '+%Y-%m-%dT%H:%M:%SZ')" "$available" >> "$out/disk-watch.tsv"
    if (( available < floor_kib )); then
      printf '%s\n' "disk floor reached: $available < $floor_kib KiB" > "$out/disk-stop.txt"
      kill -TERM "$child" 2>/dev/null || true
      break
    fi
    sleep 30
  done
) &
watcher=$!
set +e
wait "$child"
code=$?
set -e
kill "$watcher" 2>/dev/null || true
wait "$watcher" 2>/dev/null || true
printf '%s\n' "$code" > "$out/exit.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/end.utc"
test "$code" -eq 0
test ! -e "$out/disk-stop.txt"
binary="$out/target/release/examples/n83_k600_m6_native_preflight"
test -x "$binary"
file "$binary" > "$out/binary.file"
shasum -a 256 "$binary" > "$out/binary.sha256"
wc -c "$binary" > "$out/binary.bytes"
test "$(git -C "$repo" status --porcelain)" = ""
