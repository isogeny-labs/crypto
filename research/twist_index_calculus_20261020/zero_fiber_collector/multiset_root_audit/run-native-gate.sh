#!/usr/bin/env bash
set -Eeuo pipefail
multiset_out=${1:?fresh native gate directory}
multiset_root=research/twist_index_calculus_20261020/zero_fiber_collector/multiset_root_audit
test ! -e "$multiset_out"
mkdir -p "$multiset_out/metadata"
multiset_out=$(realpath "$multiset_out")
trap 'multiset_rc=$?; printf "%s\n" "$multiset_rc" > "$multiset_out/metadata/terminal-exit"' EXIT
date -u +%FT%TZ > "$multiset_out/metadata/started-utc.txt"
git rev-parse HEAD > "$multiset_out/metadata/source-head.txt"
sha256sum "$multiset_root/native/Cargo.toml" "$multiset_root/native/src/main.rs" "$multiset_root/native/src/fixture.rs" "$multiset_root/PROTOCOL.md" "$multiset_root/run-native-gate.sh" > "$multiset_out/metadata/sources.sha256"
cp "$multiset_root/run-native-gate.sh" "$multiset_out/metadata/executed-script.sh"
export PATH=/home/ec2-user/.cargo/bin:$PATH
export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1
export CARGO_TARGET_DIR="$multiset_out/target"
rustc -vV > "$multiset_out/metadata/rustc-version.txt"
cargo -V > "$multiset_out/metadata/cargo-version.txt"
cargo fmt --manifest-path "$multiset_root/native/Cargo.toml" --check > "$multiset_out/metadata/fmt.stdout" 2> "$multiset_out/metadata/fmt.stderr"
multiset_command=(cargo test --release --jobs 1 --manifest-path "$multiset_root/native/Cargo.toml")
printf '%q ' "${multiset_command[@]}" > "$multiset_out/metadata/test.command.txt"
"${multiset_command[@]}" > "$multiset_out/metadata/test.stdout" 2> "$multiset_out/metadata/test.stderr"
cp "$multiset_root/native/Cargo.lock" "$multiset_out/metadata/Cargo.lock"
cargo build --release --jobs 1 --manifest-path "$multiset_root/native/Cargo.toml" > "$multiset_out/metadata/build.stdout" 2> "$multiset_out/metadata/build.stderr"
cp "$multiset_out/target/release/twist-multiset-root-audit" "$multiset_out/runner"
sha256sum "$multiset_out/runner" > "$multiset_out/metadata/binary.sha256"
sha256sum -c "$multiset_out/metadata/sources.sha256" > "$multiset_out/metadata/source-check.txt"
date -u +%FT%TZ > "$multiset_out/metadata/finished-utc.txt"
