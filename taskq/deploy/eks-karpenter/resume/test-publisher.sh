#!/usr/bin/env bash
# Exercise publisher integrity and no-execution boundary without cloud writes.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/control" "$work/tools" "$work/package/runtime" "$work/temp"
cp "$here/publish-runtime" "$here/fetch-client" "$here/Dockerfile.runtime" "$work/control/"
printf '#!/bin/sh\necho TESTED_TOOL_MUST_NOT_EXECUTE >&2\nexit 99\n' > "$work/package/runtime/isolab-eks-recovery"
(cd "$work/package/runtime" && sha256sum isolab-eks-recovery > SHA256SUMS)
tar -C "$work/package" -czf "$work/package.tar.gz" runtime
hash=$(sha256sum "$work/package.tar.gz" | cut -d' ' -f1)
jq -n --arg hash "$hash" '{bucket:"isolab-results-104186811093-us-east-2",key:("inputs/recovery-runtime/"+$hash+".tar.gz"),version:"version-test",sha256:$hash}' > "$work/control/runtime.json"
printf '{"image_uri":"104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:%064d"}\n' 1 > "$work/base.json"
cat > "$work/tools/aws" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
case "$1 $2" in
  'sts get-caller-identity') echo 104186811093 ;;
  'ecr get-login-password') echo test-placeholder ;;
  'ecr describe-images') printf 'sha256:%064d\n' 2 ;;
  's3api get-object') cp "$TEST_PACKAGE" "${@: -1}"; if [[ ${TEST_CORRUPT:-0} == 1 ]]; then printf bad >> "${@: -1}"; fi ;;
  *) exit 90 ;;
esac
MOCK
cat > "$work/tools/docker" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$TEST_DOCKER_LOG"
case "$1" in
  login) cat >/dev/null ;;
  pull|build|push) : ;;
  image) if [[ ${TEST_ONBUILD:-0} == 1 ]]; then printf '[{"Config":{"OnBuild":["RUN unsafe"]}}]\n'; else printf '[{"Config":{"OnBuild":null}}]\n'; fi ;;
  *) echo 'Unexpected container execution' >&2; exit 91 ;;
esac
MOCK
chmod +x "$work/tools/"*
export PATH="$work/tools:$PATH" TEST_PACKAGE="$work/package.tar.gz" TEST_DOCKER_LOG="$work/docker.log" TMPDIR="$work/temp"
"$work/control/publish-runtime" "$work/base.json" "$work/result.json" > "$work/stdout" 2> "$work/stderr"
jq -e '.runtime_image_uri|endswith("0000000000000000000000000000000000000000000000000000000000000002")' "$work/result.json" >/dev/null
[[ $(grep -c '^build ' "$work/docker.log") == 1 ]]
"$work/control/fetch-client" "$work/client" > "$work/client.stdout" 2> "$work/client.stderr"
cmp "$work/client/isolab-eks-recovery" "$work/package/runtime/isolab-eks-recovery"
: > "$work/docker.log"
if TEST_CORRUPT=1 "$work/control/publish-runtime" "$work/base.json" "$work/bad.json" > "$work/bad.stdout" 2> "$work/bad.stderr"; then echo 'Corrupt runtime accepted' >&2; exit 1; fi
[[ ! -s "$work/docker.log" ]]
if TEST_ONBUILD=1 "$work/control/publish-runtime" "$work/base.json" "$work/bad.json" > "$work/hook.stdout" 2> "$work/hook.stderr"; then echo 'ONBUILD hook accepted' >&2; exit 1; fi
if grep -q '^build ' "$work/docker.log"; then echo 'Build reached after ONBUILD rejection' >&2; exit 1; fi
printf 'Recovery publisher verifies runtime bytes, pins image digest, rejects ONBUILD hooks, and never runs downloaded code.\n'
