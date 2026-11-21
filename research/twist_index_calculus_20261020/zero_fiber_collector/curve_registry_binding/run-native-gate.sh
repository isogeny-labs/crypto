#!/usr/bin/env bash
set -Eeuo pipefail
binding_out=${1:?fresh native gate directory}
binding_root=research/twist_index_calculus_20261020/zero_fiber_collector/curve_registry_binding
test ! -e "$binding_out"
mkdir -p "$binding_out/metadata"
binding_out=$(realpath "$binding_out")
trap 'binding_rc=$?; printf "%s\n" "$binding_rc" > "$binding_out/metadata/terminal-exit"' EXIT
date -u +%FT%TZ > "$binding_out/metadata/started-utc.txt"
git rev-parse HEAD > "$binding_out/metadata/source-head.txt"
git ls-files -z -- Cargo.toml Cargo.lock src tools/volcano-cli/Cargo.toml tools/volcano-cli/src isogeny_algos/Cargo.toml isogeny_algos/src docs/ic/boundary_targets.json docs/ecbench/schema.sql docs/curves/registry.json docs/ic/calibration.json docs/curves/standards/parameters.json | xargs -0 sha256sum > "$binding_out/metadata/root-sources.sha256"
cp "$binding_root/run-native-gate.sh" "$binding_out/metadata/executed-script.sh"
sha256sum "$binding_root/native/Cargo.toml" "$binding_root/native/src/main.rs" "$binding_root/native/src/fixture.rs" "$binding_root/PROTOCOL.md" "$binding_root/run-native-gate.sh" > "$binding_out/metadata/adapter-sources.sha256"
export PATH=/home/ec2-user/.cargo/bin:$PATH
export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1
export CARGO_TARGET_DIR="$binding_out/target"
rustc -vV > "$binding_out/metadata/rustc-version.txt"
cargo -V > "$binding_out/metadata/cargo-version.txt"
cargo fmt --manifest-path "$binding_root/native/Cargo.toml" --check > "$binding_out/metadata/fmt.stdout" 2> "$binding_out/metadata/fmt.stderr"
binding_command=(cargo test --release --jobs 1 --manifest-path "$binding_root/native/Cargo.toml")
printf '%q ' "${binding_command[@]}" > "$binding_out/metadata/test.command.txt"
"${binding_command[@]}" > "$binding_out/metadata/test.stdout" 2> "$binding_out/metadata/test.stderr"
cp "$binding_root/native/Cargo.lock" "$binding_out/metadata/Cargo.lock"
binding_command=(cargo build --release --jobs 1 --manifest-path "$binding_root/native/Cargo.toml")
printf '%q ' "${binding_command[@]}" > "$binding_out/metadata/build.command.txt"
"${binding_command[@]}" > "$binding_out/metadata/build.stdout" 2> "$binding_out/metadata/build.stderr"
cp "$binding_out/target/release/twist-curve-registry-binding" "$binding_out/binder"
sha256sum "$binding_out/binder" > "$binding_out/metadata/binary.sha256"
sha256sum -c "$binding_out/metadata/root-sources.sha256" > "$binding_out/metadata/root-source-check.txt"
sha256sum -c "$binding_out/metadata/adapter-sources.sha256" > "$binding_out/metadata/adapter-source-check.txt"
date -u +%FT%TZ > "$binding_out/metadata/finished-utc.txt"
