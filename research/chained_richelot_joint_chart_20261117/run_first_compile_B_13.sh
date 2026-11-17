#!/usr/bin/env bash
set -euo pipefail
export PATH=/home/agent/.cargo/bin:$PATH
source_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_dir="$(git -C "$source_dir" rev-parse --show-toplevel)"
hold="${JOINT13_CUSTODY_DIR:?Set a fresh absolute private custody directory}"
expected_head="${SOURCE13_EXPECTED_HEAD:?Set the exact already public source commit}"
mkdir "$hold"
date -u +%FT%TZ > "$hold/actual-start.txt"
on_exit() {
  local actual=$?
  printf '%s\n' "$actual" > "$hold/original-unit.exit"
  date -u +%FT%TZ > "$hold/actual-end.txt"
}
trap on_exit EXIT
record() {
  local name="$1" want="$2"
  shift 2
  set +e
  "$@" > "$hold/$name.entire-stdout.txt" 2> "$hold/$name.entire-stderr.txt"
  local actual=$?
  set -e
  printf '%s\n' "$actual" > "$hold/$name.exit"
  printf '%s %s expected%s\n' "$name" "$actual" "$want" >> "$hold/actual-phases.txt"
  if [[ "$actual" != "$want" ]]; then
    printf 'Unexpected %s exit%s expected%s\n' "$name" "$actual" "$want" >&2
    exit "$actual"
  fi
}
git -C "$repo_dir" rev-parse HEAD > "$hold/actual-public-source-head.txt"
test "$(cat "$hold/actual-public-source-head.txt")" = "$expected_head"
git -C "$repo_dir" status --porcelain=v1 -- "$source_dir" > "$hold/entire-before-source-status.txt"
test ! -s "$hold/entire-before-source-status.txt"
mkdir "$hold/frozen-source-template"
cp "$source_dir/Cargo.toml" "$source_dir/Cargo.lock" "$source_dir/fixtures-13.csv" "$source_dir/COMPILER_INPUTS.sha256" "$source_dir/REUSED_CORE_EXTRACTION_13.json" "$source_dir/PROTOCOL_13.md" "$source_dir/PROOF_13.md" "$hold/frozen-source-template/"
cp -a "$source_dir/src" "$source_dir/vendor" "$source_dir/reused-core" "$hold/frozen-source-template/"
cp "$source_dir/run_first_compile_B_13.sh" "$hold/entire-original-caller.sh.txt"
template="$hold/frozen-source-template"
manifest="$template/Cargo.toml"
find "$source_dir" -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > "$hold/original-source-build-inputs.sha256"
find "$template" -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > "$hold/copied-source-build-inputs.sha256"
cd "$template"
record exact-compiler-inputs 0 sha256sum -c COMPILER_INPUTS.sha256
record compiler-version 0 timeout 120 rustc +1.99.1 --version --verbose
record cargo-version 0 timeout 120 cargo +1.99.1 --version
record offline-locked-vendoring 0 timeout 120 cargo +1.99.1 vendor --manifest-path "$manifest" --locked --offline --versioned-dirs "$hold/locked-registry-vendor"
find "$hold/locked-registry-vendor" -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > "$hold/dependency-inputs-before.sha256"
df -B1 "$hold" > "$hold/entire-space-before-build.txt"
available="$(df -B1 --output=avail "$hold" | tail -1 | tr -d ' ')"
test "$available" -ge 20000000
config_args=(--config 'source.crates-io.replace-with="guarded-vendor"' --config "source.guarded-vendor.directory=\"$hold/locked-registry-vendor\"")
record compile-only 0 timeout 120 cargo +1.99.1 test --manifest-path "$manifest" "${config_args[@]}" --release --lib --locked --offline -j1 --no-run
record source-before-calls 0 sha256sum -c "$hold/copied-source-build-inputs.sha256"
count=0
for candidate in "$template"/target/release/deps/chained_richelot_joint_chart_13-*; do
  if [[ -f "$candidate" && -x "$candidate" ]]; then
    count=$((count+1))
    cp "$candidate" "$hold/immutable-test-qualifier.bin"
    printf '%s\n' "$candidate" > "$hold/compiled-qualifier-path.txt"
  fi
done
test "$count" = 1
chmod 555 "$hold/immutable-test-qualifier.bin"
sha256sum "$(cat "$hold/compiled-qualifier-path.txt")" "$hold/immutable-test-qualifier.bin" > "$hold/pre-call-compiler-and-qualifier.sha256"
stat -c '%a %s %n' "$hold/immutable-test-qualifier.bin" > "$hold/qualifier-mode-size-before.txt"
record only-new-frozen-native-test 0 timeout 120 "$hold/immutable-test-qualifier.bin" --exact joint::tests::frozen_joint_chart_factor_covariance_13 --nocapture --test-threads=1
record strict-clippy 0 timeout 120 cargo +1.99.1 clippy --manifest-path "$manifest" "${config_args[@]}" --release --locked --offline -j1 --all-targets --no-deps -- -D warnings
record format 0 timeout 120 cargo +1.99.1 fmt --manifest-path "$manifest" --all -- --check
record original-source-after 0 sha256sum -c "$hold/original-source-build-inputs.sha256"
record copied-source-after 0 sha256sum -c "$hold/copied-source-build-inputs.sha256"
record dependency-source-after 0 sha256sum -c "$hold/dependency-inputs-before.sha256"
record compiler-qualifier-after 0 sha256sum -c "$hold/pre-call-compiler-and-qualifier.sha256"
sha256sum "$(cat "$hold/compiled-qualifier-path.txt")" "$hold/immutable-test-qualifier.bin" > "$hold/post-call-compiler-and-qualifier.sha256"
git -C "$repo_dir" status --porcelain=v1 -- "$source_dir" > "$hold/entire-after-source-status.txt"
test ! -s "$hold/entire-after-source-status.txt"
cat "$hold/actual-phases.txt"
