#!/usr/bin/env bash
set -euo pipefail
set -o noclobber
PPLUS_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
PPLUS_ATTEMPT=${1:-attempt-01}
case "$PPLUS_ATTEMPT" in attempt-[0-9][0-9]) ;; *) exit 64 ;; esac
PPLUS_RUSTC=${PPLUS_RUSTC:-/workspace/scratch/857f9e575d1c/tmp/vitse-rust-toolchain/installed/bin/rustc}
PPLUS_CARGO=${PPLUS_CARGO:-/workspace/scratch/857f9e575d1c/tmp/vitse-rust-toolchain/installed/bin/cargo}
PPLUS_PARENT_CACHE=${PPLUS_PARENT_CACHE:-/workspace/scratch/857f9e575d1c/tmp/vitse-root-gate/cargo-home}
PPLUS_RUN="$PPLUS_ROOT/evidence/$PPLUS_ATTEMPT"
mkdir -p "$PPLUS_ROOT/evidence"
mkdir "$PPLUS_RUN"
mkdir "$PPLUS_RUN/input" "$PPLUS_RUN/bin"
cp -r "$PPLUS_ROOT/src" "$PPLUS_ROOT/source" "$PPLUS_RUN/input/"
cp "$PPLUS_ROOT/PROTOCOL.md" "$PPLUS_ROOT/Cargo.toml" "$PPLUS_ROOT/Cargo.lock" "$PPLUS_ROOT/dependencies.tsv" "$PPLUS_ROOT/qualify.sh" "$PPLUS_RUN/input/"
cd "$PPLUS_RUN/input"
sha256sum PROTOCOL.md Cargo.toml Cargo.lock dependencies.tsv qualify.sh src/lib.rs src/witness.rs src/bin/*.rs source/* > "$PPLUS_RUN/PRE_BUILD.sha256"
printf '%s  %s\n' d81ee915ad769a87d7aac5d9664631239607f4a0deb27850cdb52cea84e34270 source/identity_certificate.rs 2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf source/sha256.rs | sha256sum -c - > "$PPLUS_RUN/production-source-check.txt"
"$PPLUS_RUSTC" --version > "$PPLUS_RUN/rustc-version.txt"
"$PPLUS_CARGO" --version > "$PPLUS_RUN/cargo-version.txt"
sha256sum "$PPLUS_RUSTC" "$PPLUS_CARGO" > "$PPLUS_RUN/toolchain.sha256"

# Refuse inherited on-disk Cargo configuration; this workspace needs none.
PPLUS_PARENT="$PPLUS_RUN/input"
while true; do
    for PPLUS_CONFIG in "$PPLUS_PARENT/.cargo/config" "$PPLUS_PARENT/.cargo/config.toml"; do
        if test -e "$PPLUS_CONFIG"; then
            printf 'Refused inherited Cargo configuration: %s\n' "$PPLUS_CONFIG" > "$PPLUS_RUN/configuration-rejection.txt"
            exit 65
        fi
    done
    test "$PPLUS_PARENT" = / && break
    PPLUS_PARENT=$(dirname -- "$PPLUS_PARENT")
done
printf 'No ancestor Cargo config or config.toml present. Wrapper and Rust-flag environment variables are cleared for this build.\n' > "$PPLUS_RUN/configuration-check.txt"

PPLUS_LOCAL_CACHE="$PPLUS_RUN/cargo-home"
PPLUS_REGISTRY=index.crates.io-1949cf8c6b5b557f
mkdir -p "$PPLUS_LOCAL_CACHE/registry/cache/$PPLUS_REGISTRY" "$PPLUS_LOCAL_CACHE/registry/index/$PPLUS_REGISTRY/.cache"
cp "$PPLUS_PARENT_CACHE/registry/index/$PPLUS_REGISTRY/config.json" "$PPLUS_LOCAL_CACHE/registry/index/$PPLUS_REGISTRY/config.json"
while read -r PPLUS_CRATE PPLUS_CHECKSUM PPLUS_INDEX; do
    PPLUS_ARCHIVE="$PPLUS_LOCAL_CACHE/registry/cache/$PPLUS_REGISTRY/$PPLUS_CRATE.crate"
    cp "$PPLUS_PARENT_CACHE/registry/cache/$PPLUS_REGISTRY/$PPLUS_CRATE.crate" "$PPLUS_ARCHIVE"
    printf '%s  %s\n' "$PPLUS_CHECKSUM" "$PPLUS_ARCHIVE" >> "$PPLUS_RUN/dependency-archives.sha256"
    mkdir -p "$PPLUS_LOCAL_CACHE/registry/index/$PPLUS_REGISTRY/.cache/$(dirname -- "$PPLUS_INDEX")"
    cp "$PPLUS_PARENT_CACHE/registry/index/$PPLUS_REGISTRY/.cache/$PPLUS_INDEX" "$PPLUS_LOCAL_CACHE/registry/index/$PPLUS_REGISTRY/.cache/$PPLUS_INDEX"
    sha256sum "$PPLUS_LOCAL_CACHE/registry/index/$PPLUS_REGISTRY/.cache/$PPLUS_INDEX" >> "$PPLUS_RUN/dependency-index.sha256"
done < dependencies.tsv
sha256sum -c "$PPLUS_RUN/dependency-archives.sha256" > "$PPLUS_RUN/dependency-archives-check.txt"
sha256sum "$PPLUS_LOCAL_CACHE/registry/index/$PPLUS_REGISTRY/config.json" >> "$PPLUS_RUN/dependency-index.sha256"

printf 'env -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS RUSTC=%q CARGO_HOME=%q CARGO_TARGET_DIR=%q %q build --release --frozen --offline --locked --bins\n' "$PPLUS_RUSTC" "$PPLUS_LOCAL_CACHE" "$PPLUS_RUN/target" "$PPLUS_CARGO" > "$PPLUS_RUN/build-command.txt"
set +e
timeout 180 env -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS RUSTC="$PPLUS_RUSTC" CARGO_HOME="$PPLUS_LOCAL_CACHE" CARGO_TARGET_DIR="$PPLUS_RUN/target" "$PPLUS_CARGO" build --release --frozen --offline --locked --bins > "$PPLUS_RUN/build.stdout" 2> "$PPLUS_RUN/build.stderr"
PPLUS_CODE=$?
set -e
printf '%s\n' "$PPLUS_CODE" > "$PPLUS_RUN/build.exit-code"
test "$PPLUS_CODE" -eq 0 || exit "$PPLUS_CODE"
cp "$PPLUS_RUN/target/release/issue_identity" "$PPLUS_RUN/target/release/check_identity" "$PPLUS_RUN/bin/"
sha256sum "$PPLUS_RUN/bin/issue_identity" "$PPLUS_RUN/bin/check_identity" > "$PPLUS_RUN/binaries.sha256"
sha256sum -c "$PPLUS_RUN/PRE_BUILD.sha256" > "$PPLUS_RUN/pre-native-source-check.txt"
sha256sum -c "$PPLUS_RUN/binaries.sha256" > "$PPLUS_RUN/pre-native-binary-check.txt"
printf '%q %q\n' "$PPLUS_RUN/bin/issue_identity" "$PPLUS_RUN/certificates" > "$PPLUS_RUN/issuer-command.txt"
set +e
timeout 60 "$PPLUS_RUN/bin/issue_identity" "$PPLUS_RUN/certificates" > "$PPLUS_RUN/issuer.stdout.jsonl" 2> "$PPLUS_RUN/issuer.stderr"
PPLUS_CODE=$?
set -e
printf '%s\n' "$PPLUS_CODE" > "$PPLUS_RUN/issuer.exit-code"
test "$PPLUS_CODE" -eq 0 || exit "$PPLUS_CODE"
sha256sum "$PPLUS_RUN/certificates/"*.json > "$PPLUS_RUN/certificates.sha256"
sha256sum -c "$PPLUS_RUN/PRE_BUILD.sha256" > "$PPLUS_RUN/pre-checker-source-check.txt"
sha256sum -c "$PPLUS_RUN/binaries.sha256" > "$PPLUS_RUN/pre-checker-binary-check.txt"
printf '%q %q\n' "$PPLUS_RUN/bin/check_identity" "$PPLUS_RUN/certificates" > "$PPLUS_RUN/checker-command.txt"
set +e
timeout 60 "$PPLUS_RUN/bin/check_identity" "$PPLUS_RUN/certificates" > "$PPLUS_RUN/checker.stdout.jsonl" 2> "$PPLUS_RUN/checker.stderr"
PPLUS_CODE=$?
set -e
printf '%s\n' "$PPLUS_CODE" > "$PPLUS_RUN/checker.exit-code"
sha256sum -c "$PPLUS_RUN/certificates.sha256" > "$PPLUS_RUN/post-checker-certificate-check.txt"
sha256sum -c "$PPLUS_RUN/PRE_BUILD.sha256" > "$PPLUS_RUN/post-native-source-check.txt"
sha256sum -c "$PPLUS_RUN/binaries.sha256" > "$PPLUS_RUN/post-native-binary-check.txt"
sha256sum -c "$PPLUS_RUN/dependency-archives.sha256" > "$PPLUS_RUN/post-build-dependency-check.txt"
sha256sum -c "$PPLUS_RUN/dependency-index.sha256" > "$PPLUS_RUN/post-build-index-check.txt"
sha256sum "$PPLUS_RUN/issuer.stdout.jsonl" "$PPLUS_RUN/checker.stdout.jsonl" > "$PPLUS_RUN/native-output.sha256"
exit "$PPLUS_CODE"
