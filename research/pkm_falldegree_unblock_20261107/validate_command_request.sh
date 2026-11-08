#!/usr/bin/env bash
# Validates isolab.eks.command-request/v1 documents with the repository's own validator:
# the inline jq program of taskq/deploy/eks-karpenter/remote/run-command (there is no
# standalone validate script for this kind; test-launch-remote-command.sh extracts it the
# same way). Also checks the 64 KiB limit and that the image digest is not the placeholder.
# Usage: validate_command_request.sh REQUEST.json...      Exit 0 only if all are valid.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
remote=$(cd "$here/../../taskq/deploy/eks-karpenter/remote" && pwd)
repo=${ISOLAB_IMAGE_REPOSITORY:-104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks}
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
awk "/^jq -e .*'\$/{f=1;next} f&&/^' \"\\\$receipt_dir\/request.json\"/{exit} f" "$remote/run-command" > "$tmp/command.jq"
[[ -s "$tmp/command.jq" ]] || { echo 'could not extract validator from run-command' >&2; exit 3; }
echo "validator: $remote/run-command (sha256 $(sha256sum "$remote/run-command" | cut -c1-16)...), $(wc -l < "$tmp/command.jq") jq lines"
rc=0
for f in "$@"; do
  [[ $(wc -c < "$f") -le 65536 ]] || { echo "INVALID  $f (over 64 KiB)"; rc=1; continue; }
  if jq -e --arg repository "$repo" -f "$tmp/command.jq" "$f" >/dev/null 2>&1; then
    ph=''; jq -e '.job.image|test("@sha256:0{64}$")' "$f" >/dev/null && ph=' [PLACEHOLDER image digest: not launchable until make_requests.sh IMAGE_JSON]'
    echo "VALID    $f$ph"
  else echo "INVALID  $f"; rc=1; fi
done
exit $rc
