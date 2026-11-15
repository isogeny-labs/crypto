#!/usr/bin/env bash
# Produce a clean sparse Docker context at one already-pushed source revision.
set -euo pipefail

[[ $# == 2 ]] || { echo 'usage: build_context.sh SOURCE_REVISION NEW_CONTEXT_DIR' >&2; exit 2; }
revision=$1
context=$2
[[ "$revision" =~ ^[a-f0-9]{40}$ ]] || { echo 'source revision must be a full SHA-1' >&2; exit 2; }
[[ ! -e "$context" ]] || { echo "context already exists: $context" >&2; exit 2; }

repository=${GHS_SOURCE_REPOSITORY:-https://github.com/isogeny-labs/crypto.git}
mkdir -p "$context/repo"
mkdir -p "$context/tooling"
git -C "$context/repo" init -q
git -C "$context/repo" remote add origin "$repository"
git -C "$context/repo" -c uploadpack.allowFilter=true fetch --quiet --depth=1 --filter=blob:none origin "$revision"
git -C "$context/repo" sparse-checkout init --no-cone
git -C "$context/repo" sparse-checkout set --no-cone --stdin <<'PATTERNS'
/research/quartic_ghs_strong_rho_20261114/
/research/quartic_n4_20261019/common.rs
/research/quartic_n4_20261019/ghs.rs
/research/quartic_n4v2_20261020/rho2.rs
/src/cryptanalysis/plane_quartic_pic.rs
PATTERNS
git -C "$context/repo" checkout --quiet --detach FETCH_HEAD
git -C "$context/repo" update-index --index-version=4

curl --fail --silent --show-error --location \
  https://github.com/jqlang/jq/releases/download/jq-1.7.1/jq-linux-amd64 \
  --output "$context/tooling/jq-linux-amd64"
curl --fail --silent --show-error --location \
  https://awscli.amazonaws.com/awscli-exe-linux-x86_64-2.40.3.zip \
  --output "$context/tooling/awscliv2.zip"

[[ $(git -C "$context/repo" rev-parse HEAD) == "$revision" ]]
[[ -z $(git -C "$context/repo" status --porcelain) ]]
(
  cd "$context"
  sha256sum -c - <<'HASHES'
b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8  repo/research/quartic_ghs_strong_rho_20261114/main.rs
73abd342613dee852add1e3459c94bc7f949801c94eb875a309cdfc4f6b0f5de  repo/research/quartic_ghs_strong_rho_20261114/strong_rho.rs
5942c9b0934e510ee61eb3e30273f1b3fe2590df93933a93d7c58b81d19c8ff5  tooling/jq-linux-amd64
4770c06c4ecfc8ba94b60d89e6c7018ab7ea2cf5341b1ef05f6f93c5d6e8b705  tooling/awscliv2.zip
HASHES
)

printf 'context=%s\nsource_revision=%s\nrepository=%s\n' "$context" "$revision" "$repository"
