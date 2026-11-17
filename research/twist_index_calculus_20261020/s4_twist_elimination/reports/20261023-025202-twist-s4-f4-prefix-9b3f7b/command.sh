#!/usr/bin/env bash
set -euo pipefail
mkdir -p "$EPHEMERAL_OUT/source"
{ printf '%s' "$S4_BUNDLE_A"; printf '%s' "$S4_BUNDLE_B"; } | base64 -d | tar -xz -C "$EPHEMERAL_OUT/source"
probe_manifest="$EPHEMERAL_OUT/source/research/twist_index_calculus_20261020/s4_twist_elimination/native/Cargo.toml"
probe_root="$EPHEMERAL_OUT/source/research/twist_index_calculus_20261020"
probe_fixture="$probe_root/s4_twist_elimination/fixtures/profile-v1.json"
probe_sha=756f5b72404c2c208f477f8cc2eeb20e8b9094a02250a1b5571776bb9d853f8a
export CARGO_TARGET_DIR=/work/twist-s4-prefix-target
cargo build --locked --release --manifest-path "$probe_manifest" > "$EPHEMERAL_OUT/build.log" 2>&1
cargo test --locked --release --manifest-path "$probe_manifest" -- --test-threads=1 > "$EPHEMERAL_OUT/tests.log" 2>&1
probe_binary="$CARGO_TARGET_DIR/release/twist-s4-structure-probe"
"$probe_binary" --certificate "$EPHEMERAL_OUT/identity-v2" > "$EPHEMERAL_OUT/identity.stdout" 2> "$EPHEMERAL_OUT/identity.stderr"
for probe_label in first aa; do
  prlimit --as=8589934592 -- timeout 1200s "$probe_binary" --prefixes "$probe_fixture" "$probe_sha" "$probe_root" "$EPHEMERAL_OUT/prefixes-$probe_label" > "$EPHEMERAL_OUT/prefixes-$probe_label.stdout" 2> "$EPHEMERAL_OUT/prefixes-$probe_label.stderr"
done
diff -qr "$EPHEMERAL_OUT/prefixes-first" "$EPHEMERAL_OUT/prefixes-aa" > "$EPHEMERAL_OUT/aa.diff"
printf 'both complete collectors passed and all records are equal\n' > "$EPHEMERAL_OUT/complete.txt"
