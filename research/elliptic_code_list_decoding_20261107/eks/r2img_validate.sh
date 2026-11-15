#!/usr/bin/env bash
# r2img: validates isolab.eks.command-request/v1 with the inline jq program of remote/run-command (same approach as
# research/pkm_falldegree_unblock_20261107/validate_command_request.sh). Usage: r2img_validate.sh REQUEST.json...
set -euo pipefail
repo_root=$(cd "$(dirname "$0")" && git rev-parse --show-toplevel)
remote="$repo_root/taskq/deploy/eks-karpenter/remote"
repo=${ISOLAB_IMAGE_REPOSITORY:-104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks}
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
awk "/^jq -e .*'\$/{f=1;next} f&&/^' \"\\\$receipt_dir\/request.json\"/{exit} f" "$remote/run-command" > "$tmp/command.jq"
[[ -s "$tmp/command.jq" ]] || { echo 'could not extract validator from run-command' >&2; exit 3; }
echo "validator: $remote/run-command (sha256 $(sha256sum "$remote/run-command" | cut -c1-16)...), $(wc -l < "$tmp/command.jq") jq lines"
rc=0
for f in "$@"; do
  [[ $(wc -c < "$f") -le 65536 ]] || { echo "INVALID  $f (over 64 KiB)"; rc=1; continue; }
  if jq -e '.job.image|test("@sha256:0{64}$")' "$f" >/dev/null; then echo "INVALID  $f (placeholder image digest)"; rc=1; continue; fi
  if jq -e --arg repository "$repo" -f "$tmp/command.jq" "$f" >/dev/null 2>&1; then echo "VALID    $f"; else echo "INVALID  $f"; rc=1; fi
done
exit $rc
