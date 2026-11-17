#!/usr/bin/env bash
# Entered only after the published bootstrap SHA-256 has been checked.
set -euo pipefail
export DEBIAN_FRONTEND=noninteractive
root="/results/${TRACE_RESULT_NONCE:?missing result namespace}"
mkdir -p "$root/raw"
exec > "$root/raw/bootstrap.log" 2>&1
status=0
finish() {
  status=$?
  trap - EXIT
  jq -n --argjson exit "$status" --arg utc "$(date -u +%FT%TZ)" '{exit_status:$exit,completed_utc:$utc}' > "$root/raw/worker-exit.json"
  tar -czf "$root/results.tar.gz" -C "$root/raw" .
  digest=$(sha256sum "$root/results.tar.gz" | cut -d' ' -f1)
  bytes=$(stat -c %s "$root/results.tar.gz")
  jq -n --arg digest "$digest" --argjson bytes "$bytes" --argjson exit "$status" '{schema:"trace-runpod-transfer/v1",sha256:$digest,bytes:$bytes,worker_exit:$exit}' > "$root/finished.json"
  exit "$status"
}
trap finish EXIT
trap 'exit 124' TERM INT
apt-get update
apt-get install -y --no-install-recommends build-essential libflint-dev libgmp-dev libmpfr-dev nlohmann-json3-dev pari-gp git curl jq ca-certificates busybox openssl time ripgrep pkg-config cmake libssl-dev libclang-dev m4 xz-utils
export CARGO_HOME=/workspace/cargo-home RUSTUP_HOME=/workspace/rustup-home
curl --fail --silent --show-error https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init -o /tmp/rustup-init
curl --fail --silent --show-error https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init.sha256 -o /tmp/rustup-init.sha256
(cd /tmp && sha256sum -c rustup-init.sha256)
chmod 0700 /tmp/rustup-init
/tmp/rustup-init -y --profile minimal --default-toolchain 1.100.0 --no-modify-path
export PATH="$CARGO_HOME/bin:$PATH"
mkdir -p /workspace/source
cd /workspace/source
git init
git remote add origin https://github.com/isogeny-labs/crypto.git
git sparse-checkout init --no-cone
git sparse-checkout set --no-cone /research/trace_incidence_20261116/ /src/cryptanalysis/prime_fast.rs /src/cryptanalysis/residual_walk.rs /examples/trace_incidence_rho.rs /Cargo.toml /Cargo.lock
git -c protocol.version=2 fetch --depth=1 --filter=blob:none origin "$TRACE_SOURCE_REVISION"
git checkout --detach FETCH_HEAD
test "$(git rev-parse HEAD)" = "$TRACE_SOURCE_REVISION"
export TRACE_RESULT_DIR="$root/raw"
bash research/trace_incidence_20261116/runpod/job-v2.sh
