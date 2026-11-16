#!/usr/bin/env bash
# Local integration reproduction only; never an EKS admission or performance run.
set -euo pipefail
revision=${1:?published 40-character commit}
out=${2:?fresh absolute output directory}
checker=${3:?canonical experiment_protocol.py path}
python_bin=${4:-python3}
ca_file=${5:-/etc/ssl/certs/ca-certificates.crt}
[[ "$revision" =~ ^[0-9a-f]{40}$ ]]
[[ "$out" = /* ]]
test ! -e "$out"
git cat-file -e "$revision^{commit}"
test -n "$(git branch -r --contains "$revision")"
mkdir "$out"
context=$(mktemp -d /tmp/atlas-unit-context.XXXXXX)
trap 'rm -rf "$context"' EXIT
git archive "$revision:research/curve25519_atlas_carry_rank_20261104" | tar -x -C "$context"
docker_local() {
  env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH \
    docker --host=unix:///var/run/docker.sock "$@"
}
docker_local build --secret "id=proxy_ca,src=$ca_file" --build-arg "SOURCE_REVISION=$revision" \
  --iidfile "$out/image.iid" "$context" > "$out/image-build.txt" 2>&1
image_id=$(cat "$out/image.iid")
docker_local inspect "$image_id" --format '{{json .Config}}' > "$out/image-config.json"
docker_local run --rm --network=none --mount "type=bind,src=$out,dst=/work" \
  "$image_id" protocol /work/protocol.json 101 42 1 unit > "$out/protocol-create.txt" 2>&1
"$python_bin" "$checker" freeze "$out/protocol.json" --commit "$revision" \
  --filed-before-affected-data > "$out/protocol-freeze.txt" 2>&1
status=0
docker_local run --rm --network=none --mount "type=bind,src=$out,dst=/work" \
  --entrypoint /bin/sh "$image_id" -c \
  'set -eu; id -u; z3 -version; cd /usr/local/bin; sha256sum -c ATLAS-BINARIES.sha256; cp ATLAS-BINARIES.sha256 /work/; study --self-test; study --runner-fixture /work/inputs; atlas-control run /work/inputs /work/solver-run /usr/local/bin/study /usr/local/bin/z3 30 /usr/local/bin/openssl-replay' \
  > "$out/console.txt" 2>&1 || status=$?
printf '%s\n' "$status" > "$out/container-exit.txt"
# Retain/export incomplete solver outcomes too. Missing summaries still fail.
docker_local run --rm --network=none --mount "type=bind,src=$out,dst=/work" \
  "$image_id" export /work/inputs /work/solver-run /work/protocol.json /work/export.json \
  >> "$out/console.txt" 2>&1
"$python_bin" "$checker" check-export "$out/export.json" --protocol "$out/protocol.json" \
  --verify-files > "$out/export-check.txt" 2>&1
exit "$status"
