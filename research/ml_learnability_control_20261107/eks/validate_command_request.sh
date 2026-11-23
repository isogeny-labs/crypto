#!/usr/bin/env bash
# Runs the repository's own command-request validator (inline jq of taskq/deploy/eks-karpenter/remote/run-command) on request files.
# Usage: validate_command_request.sh REQUEST.json...   (exit 0 only if all valid)
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); remote=$(cd "$here/../../../taskq/deploy/eks-karpenter/remote" && pwd)
repo=${ISOLAB_IMAGE_REPOSITORY:-104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks}
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
awk "/^jq -e .*'\$/{f=1;next} f&&/^' \"\\\$receipt_dir\/request.json\"/{exit} f" "$remote/run-command" > "$tmp/command.jq"
[[ -s "$tmp/command.jq" ]] || { echo 'could not extract validator' >&2; exit 3; }
echo "validator: run-command sha256 $(sha256sum "$remote/run-command" | cut -c1-16)..., $(wc -l < "$tmp/command.jq") jq lines"
rc=0
for f in "$@"; do
  [[ $(wc -c < "$f") -le 65536 ]] || { echo "INVALID  $f (over 64 KiB)"; rc=1; continue; }
  if jq -e --arg repository "$repo" -f "$tmp/command.jq" "$f" >/dev/null 2>&1; then
    jq -e '.job.image|test("@sha256:0{64}$")' "$f" >/dev/null && echo "VALID    $f [PLACEHOLDER digest]" || echo "VALID    $f"
  else echo "INVALID  $f"; rc=1; fi
done; exit $rc
