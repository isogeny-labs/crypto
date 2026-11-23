#!/usr/bin/env bash
# Build and push the task image for the cap-6 cells. Run on the lab host (needs git, rustup with the
# x86_64-unknown-linux-musl target, musl-tools, docker, aws with ECR push). Prints the three identities, kept distinct:
# source SHA, binary SHA-256, OCI digest. Usage: build_image.sh COMMIT   (40 hex chars, a commit on aburan28/crypto main)
set -euo pipefail
commit=${1:?40-character commit SHA}
[[ "$commit" =~ ^[a-f0-9]{40}$ ]]
: "${ISOLAB_IMAGE_REPOSITORY:?ACCOUNT.dkr.ecr.REGION.amazonaws.com/isolab/tasks}" "${AWS_REGION:?}"
here=$(cd "$(dirname "$0")" && pwd)
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
git clone --quiet https://github.com/aburan28/crypto "$work/src"
git -C "$work/src" checkout --quiet "$commit"
[[ $(git -C "$work/src" rev-parse HEAD) == "$commit" ]]
( cd "$work/src"
  export CC_x86_64_unknown_linux_musl=musl-gcc RUSTFLAGS='-C target-cpu=x86-64'
  cargo +1.99.1 build --locked --release --no-default-features --example pkm_tower_pilot --target x86_64-unknown-linux-musl )
bin=$work/src/target/x86_64-unknown-linux-musl/release/examples/pkm_tower_pilot
readelf -h "$bin" | grep -q 'Advanced Micro Devices X86-64'
! readelf -l "$bin" | grep -q INTERP   # static
sha=$(sha256sum "$bin" | cut -d' ' -f1)
ctx=$work/ctx; mkdir "$ctx"
install -m755 "$bin" "$ctx/pkm_tower_pilot"
install -m755 "$here/run_cell.sh" "$ctx/run_cell.sh"
jq -n --arg r "$commit" --arg s "$sha" '{source_revision:$r,source_repository:"https://github.com/aburan28/crypto",architecture:"linux/amd64",target:"x86_64-unknown-linux-musl",default_features:false,pkm_tower_pilot_sha256:$s}' > "$ctx/build-info.json"
tag="pfd-t14-c6-$commit"
registry=${ISOLAB_IMAGE_REPOSITORY%%/*}
aws ecr get-login-password --region "$AWS_REGION" | docker login --username AWS --password-stdin "$registry"
docker build --build-arg SOURCE_REVISION="$commit" -t "$ISOLAB_IMAGE_REPOSITORY:$tag" -f "$here/Dockerfile" "$ctx"
docker push "$ISOLAB_IMAGE_REPOSITORY:$tag"
digest=$(docker inspect --format '{{index .RepoDigests 0}}' "$ISOLAB_IMAGE_REPOSITORY:$tag")
echo "source_sha   $commit"; echo "binary_sha   $sha"; echo "image        $digest"
