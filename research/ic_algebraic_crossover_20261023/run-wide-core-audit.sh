#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
meta="$study/pipeline-receipts/wide-core-audit-20261025"
bin=/workspace/ic-frozen-20261025/ic_quartic_wide_audit
test ! -e "$meta"
mkdir -p "$meta"
git rev-parse HEAD > "$meta/source-commit.txt"
sha256sum "$bin" "$study/WIDE_GROUP_AUDIT_PROTOCOL.md" > "$meta/hashes.txt"
date -u +%FT%TZ > "$meta/started.txt"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
for m in 16 17 18; do
  folder="$meta/m$m"
  mkdir -p "$folder"
  printf '%s\n' "$bin core $m" > "$folder/command.txt"
  date -u +%FT%TZ > "$folder/started.txt"
  set +e
  (ulimit -v 12582912; /workspace/.tools/usr/bin/time -f 'wall_seconds=%e\nmax_rss_kib=%M\nexit=%x' -o "$folder/resources.txt" timeout -k 30 900 "$bin" core "$m") > "$folder/output.json" 2> "$folder/stderr.txt"
  rc=$?
  set -e
  printf '{"exit_code":%s,"address_space_bytes":12884901888,"timeout_seconds":900}\n' "$rc" > "$folder/status.json"
  date -u +%FT%TZ > "$folder/ended.txt"
  printf 'm=%s exit=%s\n' "$m" "$rc"
done
set +e
"$bin" profile "$study" > "$meta/constructor-profile-audit.json" 2> "$meta/constructor-profile-audit.stderr"
rc=$?
set -e
printf '{"exit_code":%s}\n' "$rc" > "$meta/profile-status.json"
date -u +%FT%TZ > "$meta/ended.txt"
printf '%s\n' 'Inspect all three outcomes, independently replay successful cores, and render/inspect report before making any claim. Full IC integration is not supplied by this audit.' > "$meta/REVIEW_REQUIRED.txt"
