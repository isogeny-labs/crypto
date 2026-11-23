#!/usr/bin/env bash
set -euo pipefail
source_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_dir="$(git -C "$source_dir" rev-parse --show-toplevel)"
hold="${BALANCED12_CUSTODY_DIR:?Set a fresh absolute custody directory}"
mkdir "$hold"
date -u +%FT%TZ > "$hold/actual-start.txt"
git -C "$repo_dir" rev-parse HEAD > "$hold/actual-source-head.txt"
record() {
  local phase="$1" want="$2"
  shift 2
  set +e
  "$@" > "$hold/$phase.entire-stdout.txt" 2> "$hold/$phase.entire-stderr.txt"
  local actual=$?
  set -e
  printf '%s\n' "$actual" > "$hold/$phase.exit"
  printf '%s %s expected%s\n' "$phase" "$actual" "$want" >> "$hold/actual-phases.txt"
  if [[ "$actual" != "$want" ]]; then
    printf 'Unexpected %s exit%s expected%s\n' "$phase" "$actual" "$want" >&2
    exit 1
  fi
}
template="$hold/frozen-source-template"
mkdir "$template"
cp "$source_dir/Cargo.toml" "$source_dir/Cargo.lock" "$source_dir/fixtures-12.csv" "$source_dir/COMPILER_INPUTS.sha256" "$template/"
cp -a "$source_dir/src" "$source_dir/vendor" "$source_dir/reused-core" "$template/"
cp "$source_dir/PROTOCOL_12.md" "$source_dir/CI_REPLAY_PROTOCOL_12.md" "$source_dir/replay_balanced_12.sh" "$hold/"
cd "$source_dir"
record original-source-guard 0 sha256sum -c COMPILER_INPUTS.sha256
cd "$template"
record copied-source-guard 0 sha256sum -c COMPILER_INPUTS.sha256
manifest="$template/Cargo.toml"
record compiler-version 0 timeout 120 rustc +1.99.1 --version --verbose
record cargo-version 0 timeout 120 cargo +1.99.1 --version
record locked-dependency-vendoring 0 timeout 120 cargo +1.99.1 vendor --manifest-path "$manifest" --locked --versioned-dirs "$hold/locked-registry-vendor"
find "$hold/locked-registry-vendor" -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > "$hold/dependency-inputs-before.sha256"
config_args=(--config 'source.crates-io.replace-with="guarded-vendor"' --config "source.guarded-vendor.directory=\"$hold/locked-registry-vendor\"")
record compile-only 0 timeout 120 cargo +1.99.1 test --manifest-path "$manifest" "${config_args[@]}" --release --lib --locked --offline -j1 --no-run
count=0
for candidate in "$template"/target/release/deps/chained_richelot_balanced_chart_12-*; do
  if [[ -f "$candidate" && -x "$candidate" ]]; then
    count=$((count+1))
    cp "$candidate" "$hold/immutable-test-qualifier.bin"
    printf '%s\n' "$candidate" > "$hold/compiled-qualifier-path.txt"
  fi
done
test "$count" = 1
chmod 555 "$hold/immutable-test-qualifier.bin"
sha256sum "$(cat "$hold/compiled-qualifier-path.txt")" "$hold/immutable-test-qualifier.bin" > "$hold/compiler-and-qualifier-before.sha256"
stat -c '%a %s %n' "$hold/immutable-test-qualifier.bin" > "$hold/qualifier-mode-size-before.txt"
record all-native-tests 0 timeout 120 "$hold/immutable-test-qualifier.bin" --nocapture --test-threads=1
record strict-clippy 0 timeout 120 cargo +1.99.1 clippy --manifest-path "$manifest" "${config_args[@]}" --release --locked --offline -j1 --all-targets --no-deps -- -D warnings
record format 0 timeout 120 cargo +1.99.1 fmt --manifest-path "$manifest" --all -- --check
record copied-source-after 0 sha256sum -c COMPILER_INPUTS.sha256
record dependency-source-after 0 sha256sum -c "$hold/dependency-inputs-before.sha256"
record compiler-qualifier-after 0 sha256sum -c "$hold/compiler-and-qualifier-before.sha256"
cd "$source_dir"
record original-source-after 0 sha256sum -c COMPILER_INPUTS.sha256
date -u +%FT%TZ > "$hold/actual-end.txt"
cat "$hold/actual-phases.txt"
