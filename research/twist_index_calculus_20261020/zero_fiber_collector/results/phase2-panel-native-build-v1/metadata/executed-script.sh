#!/usr/bin/env bash
# Thin metadata-tool build capture. Invoke through isolated_bench.py busy.
set -Eeuo pipefail
panel_out=${1:?fresh build output directory}
panel_target=${2:?Cargo target directory}
panel_source=research/twist_index_calculus_20261020/zero_fiber_collector/phase2/panel_native
test ! -e "$panel_out"
mkdir -p "$panel_out/metadata" "$panel_out/source"
cp "$0" "$panel_out/metadata/executed-script.sh"
date -u +%FT%TZ > "$panel_out/metadata/started-utc.txt"
git rev-parse HEAD > "$panel_out/metadata/source-commit.txt"
rustc --version > "$panel_out/metadata/rustc.txt"
cargo --version > "$panel_out/metadata/cargo.txt"
export CARGO_TARGET_DIR="$panel_target" CARGO_BUILD_JOBS=1
panel_phase() {
  local panel_tag=$1 panel_rc
  shift
  printf '%q ' "$@" > "$panel_out/metadata/$panel_tag.command.txt"
  if "$@" > "$panel_out/metadata/$panel_tag.stdout" 2> "$panel_out/metadata/$panel_tag.stderr"; then
    panel_rc=0
  else
    panel_rc=$?
  fi
  printf '%s\n' "$panel_rc" > "$panel_out/metadata/$panel_tag.exit"
  return "$panel_rc"
}
panel_phase lockfile timeout --signal=TERM --kill-after=10s 1200s cargo generate-lockfile --offline --manifest-path "$panel_source/Cargo.toml"
sha256sum "$panel_source/Cargo.toml" "$panel_source/Cargo.lock" "$panel_source/src/main.rs" > "$panel_out/metadata/sources.sha256"
cp "$panel_source/Cargo.toml" "$panel_source/Cargo.lock" "$panel_out/source/"
cp "$panel_source/src/main.rs" "$panel_out/source/main.rs"
panel_phase build timeout --signal=TERM --kill-after=10s 1200s cargo build --release --offline --locked --manifest-path "$panel_source/Cargo.toml"
cp "$panel_target/release/twist-zero-pipeline-panel" "$panel_out/panel"
chmod 500 "$panel_out/panel"
sha256sum "$panel_out/panel" > "$panel_out/metadata/panel.sha256"
sha256sum -c "$panel_out/metadata/sources.sha256" > "$panel_out/metadata/sources-unchanged.txt"
date -u +%FT%TZ > "$panel_out/metadata/finished-utc.txt"
printf '0\n' > "$panel_out/metadata/exit"
