#!/usr/bin/env bash
# NOT RUN by the authoring session. Ready-to-run recipe for the main session/executor.
# Builds the static sgsecant binary and publishes a one-layer image on top of the lab's
# AWS-CLI base (so `aws s3 cp` exists for the result upload). No Docker daemon needed.
set -euo pipefail
REV=${1:?"40-hex source revision that contains research/split_graph_secant_cost_20261107/native"}
cd "$(git rev-parse --show-toplevel)"
study=research/split_graph_secant_cost_20261107
( cd "$study/native" && sha256sum -c SHA256SUMS )
cargo build --release --locked --target x86_64-unknown-linux-musl --manifest-path "$study/native/Cargo.toml"
bin="$study/native/target/x86_64-unknown-linux-musl/release/sgsecant"
[ -x "$bin" ] || bin=${CARGO_TARGET_DIR:-target}/x86_64-unknown-linux-musl/release/sgsecant
layer=$(mktemp -d); mkdir -p "$layer/opt/sgsecant"; cp "$bin" "$layer/opt/sgsecant/sgsecant"
sha256sum "$layer/opt/sgsecant/sgsecant"
base=$(jq -r .image_uri taskq/deploy/eks-karpenter/image.json)   # AWS-CLI based lab image
python3 taskq/deploy/eks-karpenter/oci-layer-publish.py --base "$base" --layer-dir "$layer" \
  --target 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks --tag "sgsecant-${REV:0:12}" \
  --label "org.opencontainers.image.revision=$REV" --out "$study/eks/layer-receipt.json"
echo "Put the digest from layer-receipt.json into eks/request.template.json (job.image), set"
echo "provenance.source_revision=$REV and provenance.protocol_sha256=$(sha256sum $study/PROTOCOL.md | cut -d' ' -f1)."
