#!/usr/bin/env bash
# Frozen D18 source/input/build gate for an ephemeral --no-source --toolchain rust job.
# This deliberately stops before a worker arm or a performance measurement.
set -Eeuo pipefail

: "${EPHEMERAL_OUT:?the ephemeral runner must set EPHEMERAL_OUT}"
mkdir -p "$EPHEMERAL_OUT"
test -d "$EPHEMERAL_OUT"
EPHEMERAL_OUT="$(cd "$EPHEMERAL_OUT" && pwd -P)"
finish() {
    local rc=$?
    printf '%d\n' "$rc" > "$EPHEMERAL_OUT/preflight-exit.txt"
    date -u +%FT%TZ > "$EPHEMERAL_OUT/preflight-end-utc.txt"
}
trap finish EXIT

bucket=crypto-autoresearcher
region=us-west-2
prefix=factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0
source_sha=dcc242fc0c4e6b4af99a854b885b0dece5d18ef69716a82cc5b5cc98d86044cd
source_key="$prefix/runs/linux-cadence32-20261026/source/$source_sha.tar.gz"
object_name=c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz
object_sha=eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b
wrapper_sha=1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616
producer_sha=1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce
source_commit=45ce91d503ee0d0acc57e3d8d5cb49108c90ded6
verifier_blob=1290dd0b70bc49ee6d5ab930917532517f396461

scratch="$(mktemp -d "${TMPDIR:-/tmp}/n83-d18-preflight.XXXXXXXX")"
printf '%s\n' "$scratch" > "$EPHEMERAL_OUT/preflight-scratch-path.txt"
date -u +%FT%TZ > "$EPHEMERAL_OUT/preflight-start-utc.txt"
sha256sum "$0" > "$EPHEMERAL_OUT/preflight-script.sha256"
aws sts get-caller-identity --query '{Account:Account,Arn:Arn}' --output json > "$EPHEMERAL_OUT/caller-identity.json"
rustc --version > "$EPHEMERAL_OUT/toolchain.txt"
cargo --version >> "$EPHEMERAL_OUT/toolchain.txt"
aws --version >> "$EPHEMERAL_OUT/toolchain.txt" 2>&1
{
    uname -a
    lscpu
    free -b
    df -B1 "$scratch"
    cat /proc/loadavg
} > "$EPHEMERAL_OUT/host.txt"
if test -f /sys/fs/cgroup/memory.max; then
    cat /sys/fs/cgroup/memory.max > "$EPHEMERAL_OUT/cgroup-memory-max.txt"
fi

archive="$scratch/source.tar.gz"
input="$scratch/d18-input"
mkdir -p "$input/objects"
aws s3api get-object --region "$region" --bucket "$bucket" --key "$source_key" \
    --checksum-mode ENABLED "$archive" > "$EPHEMERAL_OUT/source-get-metadata.json"
printf '%s  %s\n' "$source_sha" "$archive" | sha256sum -c -
test "$(stat -c %s "$archive")" -eq 58011243
aws s3api get-object --region "$region" --bucket "$bucket" --key "$prefix/manifest.json" \
    --checksum-mode ENABLED "$input/s3-manifest.json" > "$EPHEMERAL_OUT/manifest-get-metadata.json"
printf '%s  %s\n' "$wrapper_sha" "$input/s3-manifest.json" | sha256sum -c -
aws s3api get-object --region "$region" --bucket "$bucket" --key "$prefix/objects/$object_name" \
    --checksum-mode ENABLED "$input/objects/$object_name" > "$EPHEMERAL_OUT/object-get-metadata.json"
printf '%s  %s\n' "$object_sha" "$input/objects/$object_name" | sha256sum -c -
test "$(stat -c %s "$input/objects/$object_name")" -eq 160747529
sha256sum "$archive" "$input/s3-manifest.json" "$input/objects/$object_name" \
    > "$EPHEMERAL_OUT/input-hashes.sha256"

tar -xzf "$archive" -C "$scratch"
source_dir="$scratch/n83-portable-source-45ce-20261026"
test "$(GIT_NO_LAZY_FETCH=1 git -C "$source_dir" rev-parse HEAD)" = "$source_commit"
GIT_NO_LAZY_FETCH=1 git -C "$source_dir" status --porcelain > "$EPHEMERAL_OUT/source-status-before.txt"
test ! -s "$EPHEMERAL_OUT/source-status-before.txt"
GIT_NO_LAZY_FETCH=1 git -C "$source_dir" remote > "$EPHEMERAL_OUT/source-remotes.txt"
test ! -s "$EPHEMERAL_OUT/source-remotes.txt"
verifier="$scratch/n83-d18-native-verifier-1290dd0b.rs"
test "$(git hash-object "$verifier")" = "$verifier_blob"
producer="$source_dir/research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/manifest.json"
printf '%s  %s\n' "$producer_sha" "$producer" | sha256sum -c -
cp "$producer" "$input/manifest.json"

cp "$verifier" "$source_dir/examples/n83_d18_native_relation_replay.rs"
export CARGO_TARGET_DIR="$scratch/target"
cd "$source_dir"
cargo test --release --jobs 2 --example koblitz_n83_factor_base_export \
    --example n83_d18_native_relation_replay
cargo build --release --jobs 2 --example koblitz_n83_factor_base_export \
    --example n83_d18_native_relation_replay
sha256sum "$CARGO_TARGET_DIR/release/examples/koblitz_n83_factor_base_export" \
    "$CARGO_TARGET_DIR/release/examples/n83_d18_native_relation_replay" \
    > "$EPHEMERAL_OUT/linux-binaries.sha256"
git restore -- examples/n83_d18_native_relation_replay.rs
GIT_NO_LAZY_FETCH=1 git status --porcelain > "$EPHEMERAL_OUT/source-status-after.txt"
test ! -s "$EPHEMERAL_OUT/source-status-after.txt"
printf 'SOURCE_INPUT_BUILD_GATE_PASS scratch=%s\n' "$scratch"
