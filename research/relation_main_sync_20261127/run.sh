#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_main_sync_20261127
test ! -e "$s/raw/prepared.exit"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/preflight.stdout"
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127 CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
relation_sync_failed=0
for name in prepared five_leads prepared_pairs optimized_cli; do
  set +e
  if test "$name" = prepared; then
    taskset -c 0 timeout --signal=TERM --kill-after=10 300 prlimit --as=6442450944 --fsize=134217728 -- /home/agent/.cargo/bin/cargo test --release --locked --offline --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --lib prepared::tests -j1 -- --test-threads=1 > "$s/raw/$name.stdout" 2> "$s/raw/$name.stderr"
  else
    taskset -c 0 timeout --signal=TERM --kill-after=10 300 prlimit --as=6442450944 --fsize=134217728 -- /home/agent/.cargo/bin/cargo test --release --locked --offline --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --test "$name" -j1 -- --test-threads=1 > "$s/raw/$name.stdout" 2> "$s/raw/$name.stderr"
  fi
  relation_sync_exit=$?
  set -e
  printf '%s\n' "$relation_sync_exit" > "$s/raw/$name.exit"
  if test "$relation_sync_exit" -ne 0; then relation_sync_failed=1; fi
done
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/postflight.stdout"
sha256sum -c research/relation_f4_consequence_trace_20261127/PAYLOAD.sha256 > "$s/raw/prior-f4-evidence.stdout"
sha256sum -c research/relation_f4_syzygy_pairs_20261127/PAYLOAD.sha256 > "$s/raw/pair-evidence.stdout"
exit "$relation_sync_failed"
