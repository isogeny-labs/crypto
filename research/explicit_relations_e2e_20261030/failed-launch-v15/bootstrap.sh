#!/usr/bin/env bash
# Run inside a CUDA 13 development image on one RTX PRO 6000.
# This bounds computation only: the external controller MUST delete the pod.
set -euo pipefail
if [[ ${1:-} != --worker ]]; then
    mkdir -p /results
    set +e
    timeout --signal=TERM --kill-after=30s 18000 bash "$0" --worker 2>&1 | tee /results/bootstrap.log
    result=${PIPESTATUS[0]}
    printf '%s\n' "$result" > /results/bootstrap.exit
    exit "$result"
fi
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y --no-install-recommends ca-certificates curl git build-essential jq pari-gp pari-seadata
curl --fail --silent --show-error --location https://sh.rustup.rs -o /tmp/install-rust.sh
sh /tmp/install-rust.sh -y --profile minimal --default-toolchain 1.88.0 --component clippy
export PATH="/root/.cargo/bin:/usr/local/cuda/bin:$PATH"
source_commit=23dc8a61ca928e47237f8fb5a3a8d87228649628
mkdir -p /work/crypto
cd /work/crypto
git init
git remote add origin https://github.com/isogeny-labs/crypto.git
git sparse-checkout init --cone
git sparse-checkout set gpu/explicit-relations gpu/rootfind src/cryptanalysis crates/hashed-ecdl-tensor research/explicit_relations_e2e_20261030
git -c protocol.version=2 fetch --depth=1 --filter=blob:none origin "$source_commit"
git checkout --detach FETCH_HEAD
test "$(git rev-parse HEAD)" = "$source_commit"
git rev-parse HEAD > /results/source-commit.txt
export EPHEMERAL_OUT=/results
bash research/explicit_relations_e2e_20261030/run-filter.sh
