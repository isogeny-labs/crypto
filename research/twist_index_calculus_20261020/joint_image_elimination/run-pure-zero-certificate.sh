#!/usr/bin/env bash
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_out=${1:?new output directory}
test ! -e "$joint_out"
mkdir -p "$joint_out/metadata"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
sha256sum "$joint_study/native/pure_zero_certificate.rs" "$joint_repo/examples/quartic_twist_s4_integer.rs" > "$joint_out/metadata/sources.sha256"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
ulimit -c 0
joint_phase() {
  local joint_id=$1 joint_rc
  shift
  printf '%q ' "$@" > "$joint_out/metadata/$joint_id.command.txt"
  set +e
  python3 "$joint_repo/tools/isolated_bench.py" busy -- "$@" > "$joint_out/metadata/$joint_id.stdout" 2> "$joint_out/metadata/$joint_id.stderr"
  joint_rc=$?
  set -e
  printf '%s\n' "$joint_rc" > "$joint_out/metadata/$joint_id.exit"
  return "$joint_rc"
}
joint_phase compile timeout --kill-after=10s 1200s rustc --edition 2021 -O -C codegen-units=1 "$joint_study/native/pure_zero_certificate.rs" -o "$joint_out/runner"
chmod 500 "$joint_out/runner"
sha256sum "$joint_out/runner" > "$joint_out/metadata/runner.sha256"
joint_phase search prlimit --as=8589934592 -- timeout --kill-after=10s 60s "$joint_out/runner" "$joint_out/search"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Exact certificate attempt terminal; inspect its status. No group obstruction or speedup follows without the stated bridges.\n' > "$joint_out/complete.txt"
