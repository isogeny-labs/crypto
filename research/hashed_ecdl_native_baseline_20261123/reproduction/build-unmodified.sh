#!/usr/bin/env bash
# Thin, bounded compiler orchestration; no scientific measurements or test runs.
set -u
set -o pipefail
ROOT=/tmp/ecdlp-native-bootstrap-20261123
REPO=/tmp/ecdlp-native-crypto-20261123
TOOLS=/tmp/ecdlp-native-tools-20261123/rust-1.99.1/bin
OUT="$ROOT/unmodified-build"
mkdir "$OUT" || exit 2
cd "$REPO" || exit 2
export PATH="$TOOLS:$PATH"
export CARGO_HOME=/tmp/ecdlp-native-tools-20261123/cargo-home
export CARGO_TARGET_DIR=/tmp/ecdlp-native-tools-20261123/target
export CARGO_BUILD_JOBS=2
export CARGO_INCREMENTAL=0
export RUSTFLAGS=
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/started-utc.txt"
git rev-parse HEAD > "$OUT/source-head.txt"
git status --porcelain=v1 > "$OUT/source-status.txt"
git ls-files -z crates/hashed-ecdl-tensor | sort -z | xargs -0 sha256sum > "$OUT/source-sha256.txt"
rustc -Vv > "$OUT/rustc.txt"
cargo -Vv > "$OUT/cargo.txt"
cat > "$OUT/plan.txt" <<'PLAN'
Source qualification only; unmodified standalone native crate.
Command: cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --no-run --message-format=json
Limits: 900 wall seconds; two compiler jobs; 15032385536-byte whole-cgroup stop.
Additional external spend ceiling: USD 0; no new capacity; no S3.
No test executables run in this build phase. Compiler timing is not performance evidence.
Compiler archive identities are recorded in the parent toolchain receipt.
PLAN
setsid timeout --signal=TERM --kill-after=10 900 cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --no-run --message-format=json > "$OUT/cargo.jsonl" 2> "$OUT/cargo.stderr" &
BUILD_PID=$!
MAX_MEMORY=0
MEMORY_STOP=0
while kill -0 "$BUILD_PID" 2>/dev/null; do
  read -r USED < /sys/fs/cgroup/memory.current
  if [ "$USED" -gt "$MAX_MEMORY" ]; then MAX_MEMORY="$USED"; fi
  if [ "$USED" -ge 15032385536 ]; then
    MEMORY_STOP=1
    kill -TERM -- "-$BUILD_PID" 2>/dev/null || true
    break
  fi
  sleep 0.25
done
wait "$BUILD_PID"
CODE=$?
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$OUT/finished-utc.txt"
printf '%s\n' "$CODE" > "$OUT/exit-code.txt"
printf '%s\n' "$MAX_MEMORY" > "$OUT/peak-cgroup-bytes.txt"
printf '%s\n' "$MEMORY_STOP" > "$OUT/physical-memory-stop.txt"
sha256sum "$OUT/cargo.jsonl" "$OUT/cargo.stderr" > "$OUT/log-sha256.txt"
printf 'Compiler exit: %s; physical stop: %s; peak whole-cgroup bytes: %s\n' "$CODE" "$MEMORY_STOP" "$MAX_MEMORY"
tail -n 30 "$OUT/cargo.stderr"
exit "$CODE"
