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
source_commit=e8e1c819df8901100db0254a849271130e6cca90
mkdir -p /work/crypto
cd /work/crypto
curl --fail --silent --show-error --location "$SOURCE_ARCHIVE_URL" -o /tmp/source-checkout.tgz
printf '95a046ed6b1a064109024976c775468e138ca96c045eebbe763f5c17bd7cd58d  /tmp/source-checkout.tgz\n' | sha256sum -c -
tar --no-same-owner -xzf /tmp/source-checkout.tgz -C /work/crypto
test -z "$(git status --porcelain)"
test -z "$(git remote)"
test "$(git rev-parse HEAD)" = "$source_commit"
git rev-parse HEAD > /results/source-commit.txt
export EPHEMERAL_OUT=/results
export ER_EXPECT_GPU="RTX 4090"
export ER_CUDA_ARCH=sm_89
bash research/explicit_relations_e2e_20261030/run-arity.sh
