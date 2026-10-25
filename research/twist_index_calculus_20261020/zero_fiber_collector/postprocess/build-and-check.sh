#!/usr/bin/env bash
# Thin capture only. Invoke through the existing isolated_bench busy wrapper.
set -Eeuo pipefail
account_out=${1:?fresh output}
account_target=${2:?separate Cargo target}
account_panel=${3:?existing panel}
account_summary=${4:?historical native summary}
account_summary_sha=${5:?expected summary SHA256}
account_source=research/twist_index_calculus_20261020/zero_fiber_collector/postprocess/native
test ! -e "$account_out"
mkdir -p "$account_out/metadata" "$account_out/source"
cp "$0" "$account_out/metadata/executed-script.sh"
date -u +%FT%TZ > "$account_out/metadata/started-utc.txt"
git rev-parse HEAD > "$account_out/metadata/source-commit.txt"
rustc --version > "$account_out/metadata/rustc.txt"
cargo --version > "$account_out/metadata/cargo.txt"
printf '%s\n' 'Read-only accounting of already audited records; one build worker; shared reservation; no new solve or timing admission.' > "$account_out/metadata/resource-envelope.txt"
export CARGO_TARGET_DIR="$account_target" CARGO_BUILD_JOBS=1
account_phase() {
  local account_tag=$1 account_rc
  shift
  printf '%q ' "$@" > "$account_out/metadata/$account_tag.command.txt"
  if "$@" > "$account_out/metadata/$account_tag.stdout" 2> "$account_out/metadata/$account_tag.stderr"; then
    account_rc=0
  else
    account_rc=$?
  fi
  printf '%s\n' "$account_rc" > "$account_out/metadata/$account_tag.exit"
  return "$account_rc"
}
account_phase lockfile timeout --signal=TERM --kill-after=10s 1200s cargo generate-lockfile --offline --manifest-path "$account_source/Cargo.toml"
sha256sum "$account_source/Cargo.toml" "$account_source/Cargo.lock" "$account_source/src/main.rs" > "$account_out/metadata/sources.sha256"
cp "$account_source/Cargo.toml" "$account_source/Cargo.lock" "$account_out/source/"
cp "$account_source/src/main.rs" "$account_out/source/main.rs"
sha256sum "$account_summary" > "$account_out/metadata/input-summary.sha256"
account_phase tests timeout --signal=TERM --kill-after=10s 1200s cargo test --release --offline --locked --manifest-path "$account_source/Cargo.toml"
account_phase build timeout --signal=TERM --kill-after=10s 1200s cargo build --release --offline --locked --manifest-path "$account_source/Cargo.toml"
cp "$account_target/release/twist-zero-pipeline-accounting" "$account_out/accounting"
chmod 500 "$account_out/accounting"
sha256sum "$account_out/accounting" > "$account_out/metadata/accounting.sha256"
account_negative_rc=0
account_phase partial-as-complete timeout --signal=TERM --kill-after=10s 1200s "$account_out/accounting" "$account_panel" "$account_summary" "$account_summary_sha" complete "$account_out/rejected-complete" || account_negative_rc=$?
test "$account_negative_rc" = 2
test ! -e "$account_out/rejected-complete"
printf 'pass: partial summary rejected before output creation\n' > "$account_out/metadata/negative-control.txt"
account_phase snapshot timeout --signal=TERM --kill-after=10s 1200s "$account_out/accounting" "$account_panel" "$account_summary" "$account_summary_sha" snapshot "$account_out/analysis"
sha256sum -c "$account_out/metadata/sources.sha256" > "$account_out/metadata/sources-unchanged.txt"
sha256sum -c "$account_out/metadata/input-summary.sha256" > "$account_out/metadata/input-summary-unchanged.txt"
date -u +%FT%TZ > "$account_out/metadata/finished-utc.txt"
printf '0\n' > "$account_out/metadata/exit"
