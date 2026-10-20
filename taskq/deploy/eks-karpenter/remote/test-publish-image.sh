#!/usr/bin/env bash
# Exercise publication without AWS, Docker, or executing the supplied artifact.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
mkdir "$tmp/bin" "$tmp/bundle"
export TEST_PUBLISH_LOG="$tmp/calls" TEST_EXECUTED="$tmp/executed"
export AWS_REGION=us-east-2 GITHUB_RUN_ID=123 GITHUB_RUN_ATTEMPT=1
export ISOLAB_IMAGE_REPOSITORY=104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks
cat > "$tmp/bundle/ecbench" <<'EOF'
#!/bin/sh
touch "$TEST_EXECUTED"
EOF
chmod +x "$tmp/bundle/ecbench"
revision=$(printf 'a%.0s' {1..40})
binary=$(sha256sum "$tmp/bundle/ecbench" | cut -d' ' -f1)
jq -n --arg revision "$revision" --arg binary "$binary" \
  '{source_revision:$revision,ecbench_sha256:$binary,source_repository:"https://github.com/aburan28/crypto",
    architecture:"linux/amd64",target:"x86_64-unknown-linux-musl",default_features:false}' > "$tmp/bundle/build-info.json"
cat > "$tmp/bin/aws" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'aws %s\n' "$*" >> "$TEST_PUBLISH_LOG"
case "$*" in
  'ecr get-login-password --region us-east-2') echo test-token ;;
  'ecr describe-images --repository-name isolab/tasks --image-ids imageTag=gh-123-1-'*) printf 'sha256:%064d\n' 0 ;;
  *) exit 2 ;;
esac
EOF
cat > "$tmp/bin/docker" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'docker %s\n' "$*" >> "$TEST_PUBLISH_LOG"
if [[ "$1" == login ]]; then cat >/dev/null; fi
EOF
chmod +x "$tmp/bin/"*
export PATH="$tmp/bin:$PATH"
"$here/../publish-image" "$tmp/bundle" "$revision" "$tmp/image.json"
[[ ! -e "$TEST_EXECUTED" ]]
jq -e --arg revision "$revision" --arg binary "$binary" --arg repository "$ISOLAB_IMAGE_REPOSITORY" \
  '.source_revision==$revision and .ecbench_sha256==$binary
    and .image_uri==($repository+"@sha256:"+("0"*64))' "$tmp/image.json" >/dev/null
: > "$TEST_PUBLISH_LOG"
if "$here/../publish-image" "$tmp/bundle" "$(printf 'c%.0s' {1..40})" "$tmp/image.json" >/dev/null 2>&1; then exit 1; fi
[[ ! -s "$TEST_PUBLISH_LOG" ]]
printf changed >> "$tmp/bundle/ecbench"
if "$here/../publish-image" "$tmp/bundle" "$revision" "$tmp/image.json" >/dev/null 2>&1; then exit 1; fi
[[ ! -s "$TEST_PUBLISH_LOG" && ! -e "$TEST_EXECUTED" ]]
echo 'Publisher pins the pushed digest, never executes the artifact, and rejects commit/hash mismatches before AWS access'
