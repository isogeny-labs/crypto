# Subsequent ecbench build command — not executed

The final reviewed Rust commit is being produced by root. Do not substitute the previous host build or an old image binary. Supply the actual final approved40-hex commit in PENCIL_RELEASE_COMMIT and an existing new private log directory in PENCIL_LINK_RECORD_DIR. Confirm git HEAD matches, the tracked source is clean and its complete approved source manifest is retained; embedded dirty metadata must be truthful. Reconcile source/cache ownership and current builds before launching.

The dedicated /tmp/pencil-musl-target cache may be reused. Actual musl wrapper and source qualification are in this publication view. The command uses cargo rustc instead of cargo build so final-binary-only link diagnostics can be added without changing every dependency's RUSTFLAGS. The primary release/target/features/CPU flags remain the intended ones. Freeze this amended exact command in the build receipt.

```sh
cd /home/ec2-user/crypto-pencil-signed-graph-20261029
test "$(git rev-parse HEAD)" = "$PENCIL_RELEASE_COMMIT"
git diff-index --quiet HEAD --
test -d "$PENCIL_LINK_RECORD_DIR"
env -u CARGO_ENCODED_RUSTFLAGS -u CARGO_BUILD_RUSTFLAGS \
  -u CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER \
  CARGO_TARGET_DIR=/tmp/pencil-musl-target \
  CARGO_BUILD_JOBS=2 \
  RUSTFLAGS='-C target-cpu=x86-64' \
  REALGCC=/usr/bin/gcc \
  CC_x86_64_unknown_linux_musl=/var/tmp/pencil-musl-private-20261031/prefix/bin/musl-gcc \
  CRYPTO_BUILD_GIT_COMMIT="$PENCIL_RELEASE_COMMIT" \
  CRYPTO_BUILD_GIT_DIRTY=0 \
  cargo +1.99.1 rustc --locked --offline --release --no-default-features \
  --target x86_64-unknown-linux-musl --bin ecbench -vv -- \
  --print link-args \
  "-Clink-arg=-Wl,-Map=$PENCIL_LINK_RECORD_DIR/ecbench.map" \
  > "$PENCIL_LINK_RECORD_DIR/build.stdout" \
  2> "$PENCIL_LINK_RECORD_DIR/build.stderr"
```

Root must wrap execution in its frozen software-resource timeout/exit receipt. This instruction deliberately supplies no invented timeout or source revision. `--print link-args` is documented by the rustc book to print the full final linker invocation without disabling emission: https://doc.rust-lang.org/rustc/command-line-arguments/print-options.html#link-args. The map option uses the GCC-style linker frontend for the Linux target; preserve any failure rather than silently changing linker semantics.

Retain source commit/tree/manifest, exact environment/command, Cargo.lock hash, rustc1.99.1 -Vv, complete stdout/stderr, final ELF inspection and binary hash. Resolve every actual libc.a and CRT path from the final map/invocation and hash those files, including target self-contained Rust objects if selected. The private musl CC handles native C dependency compilation; it does not by itself select the final Rust-linked libc. Do not set the final linker to musl-gcc merely to make a version inference.

For the eventual explicit image bundle, install final ecbench0755 and reuse the separately qualified fresh-owner bytes; record both hashes and their own provenance in build-info plus sidecar. No image qualification/publication follows from this command alone.
