#!/usr/bin/env bash
# Frozen candidate diagnostics; the launcher waits for build and library gates.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/ic_algebraic_crossover_20261023
meta=$study/pipeline-receipts/narrow-constructor-after-20261025
test ! -e "$meta"
git diff --quiet c754e7111 -- src examples/ic_quartic_core_probe.rs
mkdir -p "$meta"
cp "$0" "$meta/runner.sh"
date -u +%FT%TZ > "$meta/started.txt"
git rev-parse HEAD > "$meta/context-commit.txt"
bin=/workspace/ic-frozen-20261025/ic_quartic_core_probe-narrow-after
sha256sum -c "$study/pipeline-receipts/narrow-constructor-build-20261025/binary-hash.txt"
export PATH=/workspace/.cas/valgrind/usr/bin:/workspace/.cargo/bin:$PATH
export RUSTUP_HOME=/workspace/.rustup CARGO_HOME=/workspace/.cargo CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/workspace/crypto-integration-target
export VALGRIND_LIB=/workspace/.cas/valgrind/usr/libexec/valgrind
export VALGRIND_OPTS=--extra-debuginfo-path=/workspace/.cas/valgrind/usr/lib/debug
export LD_LIBRARY_PATH=/workspace/.cas/valgrind/usr/lib/x86_64-linux-gnu
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 DEBUGINFOD_URLS=
set +e
tools/profile.sh cpu --mode callgrind --out "$study/profiles/narrow-constructor-after-20261025" -- "$bin" narrow 6 > "$meta/profile.stdout" 2> "$meta/profile.stderr"
rc=$?
printf '{"exit_code":%s}\n' "$rc" > "$meta/profile-status.json"
for m in 4 5 6 7 8 9 10; do
  d=$meta/m$m
  mkdir -p "$d"
  printf '%s\n' "$bin" both "$m" > "$d/argv.txt"
  (ulimit -v 2097152; timeout -k 30 300 "$bin" both "$m") > "$d/output.json" 2> "$d/stderr.txt"
  rc=$?
  printf '{"exit_code":%s,"address_space_bytes":2147483648,"timeout_seconds":300}\n' "$rc" > "$d/status.json"
  cmp "$d/output.json" "$study/pipeline-receipts/narrow-constructor-before-20261025/m$m/output.json" > "$d/comparison.stdout" 2> "$d/comparison.stderr"
  rc=$?
  printf '{"byte_comparison_exit_code":%s}\n' "$rc" > "$d/comparison-status.json"
done
cmp "$study/profiles/narrow-constructor-before-20261025/command.stdout" "$study/profiles/narrow-constructor-after-20261025/command.stdout" > "$meta/profile-comparison.stdout" 2> "$meta/profile-comparison.stderr"
rc=$?
printf '{"byte_comparison_exit_code":%s}\n' "$rc" > "$meta/profile-comparison-status.json"
cargo test --release --example ic_quartic_core_probe --example ic_quartic_wide_audit --example ic_algebraic_followup --example ic_qualified_query_fixtures --example ic_qualified_closure_audit --example ic_guess_completed_audit --example ic_cofactor_projection_audit > "$meta/example-tests.log" 2>&1
rc=$?
printf '{"exit_code":%s}\n' "$rc" > "$meta/example-tests-status.json"
cargo clippy --release --example ic_quartic_core_probe --example ic_quartic_wide_audit -- -D warnings -A clippy::manual_is_multiple_of > "$meta/clippy.log" 2>&1
rc=$?
printf '{"exit_code":%s}\n' "$rc" > "$meta/clippy-status.json"
date -u +%FT%TZ > "$meta/ended.txt"
printf '%s\n' 'Candidate diagnostics terminated. Inspect every status, raw output and comparison; archive the complete raw run, build and test logs; build and inspect the source-linked visual report. No performance or publication qualification is automatic.' > "$meta/REVIEW_REQUIRED.txt"
