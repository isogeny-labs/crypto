#!/usr/bin/env bash
set -euo pipefail

HERE=$(cd "$(dirname "$0")" && pwd)
RUNNER=$HERE/run.sh
FIXTURE_ROOT=$(mktemp -d /tmp/dreg-source-bindings.XXXXXX)
FIXTURE_REPO=$FIXTURE_ROOT/repository

cleanup() {
  if [[ -d $FIXTURE_ROOT && $FIXTURE_ROOT == /tmp/dreg-source-bindings.* ]]; then
    find "$FIXTURE_ROOT" -depth -delete
  fi
}
trap cleanup EXIT

git init -q "$FIXTURE_REPO"
git -C "$FIXTURE_REPO" config user.email fixture@example.invalid
git -C "$FIXTURE_REPO" config user.name dreg-source-binding-fixture
mkdir -p \
  "$FIXTURE_REPO/docs/curves" \
  "$FIXTURE_REPO/research/dreg_high_arity_20261101" \
  "$FIXTURE_REPO/research/pencil_large_prime_20261029" \
  "$FIXTURE_REPO/tools/factor-base/src"
printf '%s\n' '[package]' 'name = "fixture"' 'version = "0.0.0"' >"$FIXTURE_REPO/Cargo.toml"
printf '%s\n' '[package]' 'name = "pencil-fixture"' 'version = "0.0.0"' \
  >"$FIXTURE_REPO/research/pencil_large_prime_20261029/Cargo.toml"
printf '%s\n' '# fixture lock input' >"$FIXTURE_REPO/tools/factor-base/Cargo.lock"
printf '%s\n' 'const LOCK: &str = include_str!("../Cargo.lock");' \
  >"$FIXTURE_REPO/tools/factor-base/src/store.rs"
printf '%s\n' '{"fixture":true}' >"$FIXTURE_REPO/docs/curves/registry.json"
(
  cd "$FIXTURE_REPO"
  sha256sum Cargo.toml >research/dreg_high_arity_20261101/sources.sha256
)
git -C "$FIXTURE_REPO" add .
git -C "$FIXTURE_REPO" commit -qm 'fixture: registered source tree'
SOURCE_COMMIT=$(git -C "$FIXTURE_REPO" rev-parse HEAD)
SOURCE_MANIFEST=$FIXTURE_REPO/research/dreg_high_arity_20261101/sources.sha256

"$RUNNER" --verify-source-tree-only "$FIXTURE_REPO" "$SOURCE_COMMIT" "$SOURCE_MANIFEST"

printf '%s\n' '# committed compile-time include drift' \
  >>"$FIXTURE_REPO/tools/factor-base/Cargo.lock"
git -C "$FIXTURE_REPO" add tools/factor-base/Cargo.lock
git -C "$FIXTURE_REPO" commit -qm 'fixture: drift compiled dependency lock input'
if "$RUNNER" --verify-source-tree-only \
  "$FIXTURE_REPO" "$SOURCE_COMMIT" "$SOURCE_MANIFEST" >/dev/null 2>&1; then
  echo 'committed compile-time include drift was accepted' >&2
  exit 1
fi

git -C "$FIXTURE_REPO" checkout -q "$SOURCE_COMMIT"
printf '%s\n' '{"fixture":false}' >"$FIXTURE_REPO/docs/curves/registry.json"
git -C "$FIXTURE_REPO" add docs/curves/registry.json
git -C "$FIXTURE_REPO" commit -qm 'fixture: drift compiled registry asset'
if "$RUNNER" --verify-source-tree-only \
  "$FIXTURE_REPO" "$SOURCE_COMMIT" "$SOURCE_MANIFEST" >/dev/null 2>&1; then
  echo 'committed compile-time asset drift was accepted' >&2
  exit 1
fi

git -C "$FIXTURE_REPO" checkout -q "$SOURCE_COMMIT"
printf '%s\n' '# committed descendant drift' \
  >>"$FIXTURE_REPO/research/pencil_large_prime_20261029/Cargo.toml"
git -C "$FIXTURE_REPO" add research/pencil_large_prime_20261029/Cargo.toml
git -C "$FIXTURE_REPO" commit -qm 'fixture: drift local dependency manifest'
if "$RUNNER" --verify-source-tree-only \
  "$FIXTURE_REPO" "$SOURCE_COMMIT" "$SOURCE_MANIFEST" >/dev/null 2>&1; then
  echo 'committed local dependency drift was accepted' >&2
  exit 1
fi

git -C "$FIXTURE_REPO" checkout -q "$SOURCE_COMMIT"
mkdir -p "$FIXTURE_REPO/research/pencil_large_prime_20261029/src"
printf '%s\n' '// untracked injected source' \
  >"$FIXTURE_REPO/research/pencil_large_prime_20261029/src/injected.rs"
if "$RUNNER" --verify-source-tree-only \
  "$FIXTURE_REPO" "$SOURCE_COMMIT" "$SOURCE_MANIFEST" >/dev/null 2>&1; then
  echo 'untracked local dependency source was accepted' >&2
  exit 1
fi

H1=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
H2=2222222222222222222222222222222222222222222222222222222222222222
H3=3333333333333333333333333333333333333333333333333333333333333333
H4=4444444444444444444444444444444444444444444444444444444444444444
BINARY_MANIFEST=$FIXTURE_ROOT/binary.sha256

write_canonical_binary_manifest() {
  printf '%s  %s\n' \
    "$H1" dreg_ladder \
    "$H2" dreg_closure \
    "$H3" dreg_high_arity_analyze \
    "$H4" dreg_high_arity_report >"$BINARY_MANIFEST"
}

expect_binary_manifest_rejected() {
  local description=$1
  if "$RUNNER" --verify-binary-manifest-only \
    "$BINARY_MANIFEST" "$H1" "$H2" "$H3" "$H4" >/dev/null 2>&1; then
    echo "$description was accepted" >&2
    exit 1
  fi
}

write_canonical_binary_manifest
"$RUNNER" --verify-binary-manifest-only \
  "$BINARY_MANIFEST" "$H1" "$H2" "$H3" "$H4"

printf '%s  %s\n' \
  "$H2" dreg_closure \
  "$H1" dreg_ladder \
  "$H3" dreg_high_arity_analyze \
  "$H4" dreg_high_arity_report >"$BINARY_MANIFEST"
expect_binary_manifest_rejected 'reordered binary manifest'

printf '%s  %s\n' \
  "$H1" dreg_ladder \
  "$H2" dreg_ladder \
  "$H3" dreg_high_arity_analyze \
  "$H4" dreg_high_arity_report >"$BINARY_MANIFEST"
expect_binary_manifest_rejected 'duplicate binary manifest label'

write_canonical_binary_manifest
printf '%s  %s\n' "$H4" dreg_high_arity_report >>"$BINARY_MANIFEST"
expect_binary_manifest_rejected 'extra binary manifest row'

write_canonical_binary_manifest
printf '\n' >>"$BINARY_MANIFEST"
expect_binary_manifest_rejected 'extra blank binary manifest row'

printf '%s %s\n' \
  "$H1" dreg_ladder \
  "$H2" dreg_closure \
  "$H3" dreg_high_arity_analyze \
  "$H4" dreg_high_arity_report >"$BINARY_MANIFEST"
expect_binary_manifest_rejected 'single-space binary manifest'

printf '%s  %s\n' \
  "${H1^^}" dreg_ladder \
  "$H2" dreg_closure \
  "$H3" dreg_high_arity_analyze \
  "$H4" dreg_high_arity_report >"$BINARY_MANIFEST"
expect_binary_manifest_rejected 'uppercase binary manifest hash'

printf '%s  %s\n' \
  "$H1" dreg_ladder \
  "$H2" dreg_closure \
  "$H3" dreg_high_arity_analyze >"$BINARY_MANIFEST"
printf '%s  %s' "$H4" dreg_high_arity_report >>"$BINARY_MANIFEST"
expect_binary_manifest_rejected 'binary manifest without terminal newline'

write_canonical_binary_manifest
mv "$BINARY_MANIFEST" "$BINARY_MANIFEST.target"
ln -s "$BINARY_MANIFEST.target" "$BINARY_MANIFEST"
expect_binary_manifest_rejected 'symlinked binary manifest'

echo 'source and binary-manifest binding regression tests passed'
