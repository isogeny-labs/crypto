#!/usr/bin/env bash
# Infrastructure: fresh extraction and exact native cmp of archived raw files.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_archive=${1:?existing checkpoint archive}
joint_out=${2:?new verification directory}
test ! -e "$joint_out"
test -f "$joint_archive"
mkdir -p "$joint_out"
joint_out=$(cd "$joint_out" && pwd)
joint_archive=$(realpath "$joint_archive")
cp "$0" "$joint_out/executed-script.sh"
git rev-parse HEAD > "$joint_out/source-commit.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/started-utc.txt"
joint_scratch=$(mktemp -d /home/ec2-user/scratch/twist-joint-raw-extract-v2.XXXXXX)
printf '%s\n' "$joint_scratch" > "$joint_out/extraction-path.txt"
trap 'rm -rf "$joint_scratch"' EXIT
sha256sum -c "$joint_archive.sha256" > "$joint_out/archive-checksum.txt"
tar --no-same-owner --no-same-permissions --quoting-style=literal -xJvf "$joint_archive" -C "$joint_scratch" > "$joint_out/extracted-members.txt"
cmp "$joint_archive.manifest.json" "$joint_scratch/raw-checkpoint-manifest.json"
joint_files=0
joint_links=0
while IFS= read -r joint_member; do
  case "$joint_member" in
    raw-checkpoint-manifest.json) continue ;;
    runs/*/) test -d "$joint_scratch/$joint_member"; continue ;;
    runs/*) ;;
    *) printf 'Unexpected checkpoint member namespace\n' >&2; exit 2 ;;
  esac
  if test -L "$joint_scratch/$joint_member"; then
    test -L "$joint_study/$joint_member"
    test "$(readlink "$joint_scratch/$joint_member")" = "$(readlink "$joint_study/$joint_member")"
    joint_links=$((joint_links+1))
  else
    test -f "$joint_scratch/$joint_member"
    test -f "$joint_study/$joint_member"
    cmp "$joint_study/$joint_member" "$joint_scratch/$joint_member"
    joint_files=$((joint_files+1))
  fi
done < "$joint_out/extracted-members.txt"
joint_sha=$(sha256sum "$joint_archive" | cut -d ' ' -f 1)
joint_bytes=$(stat --printf='%s' "$joint_archive")
python3 - "$joint_scratch/raw-checkpoint-manifest.json" "$joint_out/repack-members.txt" <<'PY'
# Infrastructure metadata only: preserve the exact original archive order.
import json,pathlib,sys
v=json.loads(pathlib.Path(sys.argv[1]).read_text())
pathlib.Path(sys.argv[2]).write_text(''.join('runs/'+p+'\n' for p in v['included_terminal_or_reconciled_interrupted_runs']))
PY
tar --sort=name --mtime='2026-10-24 00:00:00Z' --owner=0 --group=0 --numeric-owner \
  --mode='u+rwX,go+rX,go-w' --format=gnu -C "$joint_scratch" -cf - raw-checkpoint-manifest.json \
  -T "$joint_out/repack-members.txt" | xz -1 -T1 | sha256sum > "$joint_out/repacked.sha256"
test "$(cut -d ' ' -f 1 "$joint_out/repacked.sha256")" = "$joint_sha"
printf '{\n  "schema": "joint-terminal-archive-verification/v1",\n  "status": "pass",\n  "archive_sha256": "%s",\n  "archive_bytes": %s,\n  "exact_regular_file_comparisons": %s,\n  "exact_symlink_target_comparisons": %s,\n  "inside_manifest_matches_sidecar": true,\n  "method": "Fresh GNU tar/xz extraction, archive checksum, and native cmp against every selected archived regular file; no scripts executed",\n  "scope": "Raw content integrity only; normalized permissions/timestamps are not compared, and no new mathematical or performance validation is implied"\n}\n' "$joint_sha" "$joint_bytes" "$joint_files" "$joint_links" > "$joint_out/verification.json"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/finished-utc.txt"
