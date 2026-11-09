#!/usr/bin/env bash
# PROPOSED image recipe; NOT RUN (it pushes to the lab's ECR; the 2026-11-07 task forbids launching
# and did not ask for a push). No Docker daemon needed: taskq/deploy/eks-karpenter/oci-layer-publish.py
# appends ONE layer to the base image and pushes by digest, as research/arity_completion_20261104 did.
# Usage: build_layer.sh COMMIT   (40 hex chars of a COMMITTED revision; never a dirty tree)
# Needs: git, cargo +1.99.1 with the x86_64-unknown-linux-musl target, musl-tools, aws (ECR push), python3.
set -euo pipefail
commit=${1:?40-character commit SHA}; [[ "$commit" =~ ^[a-f0-9]{40}$ ]]
here=$(cd "$(dirname "$0")" && pwd); repo=$(cd "$here/../../.." && pwd)
git -C "$repo" cat-file -e "$commit^{commit}"
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
git -C "$repo" archive "$commit" | tar -x -C "$work"          # read-only export of exactly that commit
( cd "$work"
  export CC_x86_64_unknown_linux_musl=musl-gcc RUSTFLAGS='-C target-cpu=x86-64'
  cargo +1.99.1 build --locked --release --no-default-features --target x86_64-unknown-linux-musl \
    --example pkm_tower_pilot --example prime_fall_degree_tools --example hky_fp_tower_check --example pfd_grid_zeros )
ex=$work/target/x86_64-unknown-linux-musl/release/examples
layer=$work/layer; mkdir -p "$layer/usr/local/bin" "$layer/opt/study"
for b in pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros; do
  readelf -l "$ex/$b" | grep -q INTERP && { echo "$b is not static" >&2; exit 1; }
  install -m755 "$ex/$b" "$layer/usr/local/bin/$b"
done
install -m755 "$here/entry.sh" "$layer/usr/local/bin/entry.sh"
install -m644 "$work/research/prime_fall_degree_20261027/run.sh" "$layer/opt/study/run.sh"
( cd "$layer/usr/local/bin" && sha256sum pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros ) > "$layer/opt/study/binaries.sha256"
jq -n --arg r "$commit" --slurpfile s <(jq -R 'split("  ")|{(.[1]):.[0]}' "$layer/opt/study/binaries.sha256" | jq -s add) \
  '{source_revision:$r,architecture:"linux/amd64",target:"x86_64-unknown-linux-musl",binary_sha256:$s[0]}' > "$layer/opt/study/build-info.json"
python3 -I "$repo/taskq/deploy/eks-karpenter/oci-layer-publish.py" \
  --base public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39 \
  --layer-dir "$layer" --target 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks --tag "pkm-fd-$commit" \
  --user 1000:1000 --workdir /work --env HOME=/work --label "org.opencontainers.image.revision=$commit" \
  --out "$here/image.json"
echo "receipt: $here/image.json (image_uri is the digest-pinned reference; keep source SHA, binary SHA-256s and digest distinct)"
