#!/bin/bash
# The study image's one layer, published to isolab/tasks by digest, as
# ../../rule/v9/launch/build-layer.sh does: /opt/scanp/{bin,tree.tar,job.sh,
# stages.sh,binaries.sha256,build-info.json}, AWS CLI v2, procps and jq.
#   build-layer.sh <dir holding ic-scanprobes and isolated_bench> <work dir> <tag>
set -euo pipefail
BIN=$(realpath "${1:?}") W=$(realpath -m "${2:?}") TAG=${3:?}
here=$(cd "$(dirname "$0")" && pwd); root=$(git -C "$here" rev-parse --show-toplevel)
rev=$(git -C "$root" rev-parse HEAD)
git -C "$root" diff --quiet HEAD -- research/ic_tool_program/suite "$here" || { echo "tree differs from HEAD" >&2; exit 2; }
L=$W/layer; rm -rf "$W"; mkdir -p "$L/opt/scanp/bin" "$W/debs"
install -m755 "$BIN"/ic-scanprobes "$BIN"/isolated_bench "$L/opt/scanp/bin/"
(cd "$L/opt/scanp/bin" && sha256sum *) | sort -k2 | cmp - <(sort -k2 "$here/binaries.sha256") || { echo "bin/ differs from binaries.sha256" >&2; exit 2; }
install -m644 "$here/binaries.sha256" "$L/opt/scanp/"
install -m755 "$here/job.sh" "$here/stages.sh" "$L/opt/scanp/"
git -C "$root" archive --format=tar -o "$L/opt/scanp/tree.tar" HEAD research/ic_tool_program/suite
(cd "$W/debs" && apt-get download procps libproc2-0 jq libjq1 libonig5 >/dev/null)
for d in "$W"/debs/*.deb; do dpkg-deb -x "$d" "$L"; done
curl -fsSL -o "$W/awscliv2.zip" https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip
(cd "$W" && unzip -q awscliv2.zip); mkdir -p "$L/usr/local/aws-cli" "$L/usr/local/bin"
cp -a "$W/aws/dist" "$L/usr/local/aws-cli/dist"; ln -s /usr/local/aws-cli/dist/aws "$L/usr/local/bin/aws"
jq -n --arg rev "$rev" --arg tag "$TAG" --arg built "$(date -u +%FT%TZ)" \
  --arg tree "$(sha256sum "$L/opt/scanp/tree.tar" | cut -d' ' -f1)" \
  --argjson bins "$(cd "$L/opt/scanp/bin" && sha256sum * | jq -Rn '[inputs|split("  ")|{(.[1]):.[0]}]|add')" \
  '{source_repository:"https://github.com/isogeny-labs/crypto",launch_revision:$rev,tag:$tag,built_at:$built,
    binary_sha256:$bins,tree_tar_sha256:$tree,
    binary_sources:{"ic-scanprobes":"this commit, --features scan-probes","isolated_bench":"f465f2fb (rule v9 image)"}}' > "$L/opt/scanp/build-info.json"
python3 "$root/taskq/deploy/eks-karpenter/oci-layer-publish.py" --base public.ecr.aws/docker/library/ubuntu:24.04 \
  --layer-dir "$L" --target 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks --tag "$TAG" \
  --user 1000:1000 --workdir /work --env HOME=/work --label org.opencontainers.image.revision="$rev" --out "$W/image.json"
jq -s '.[0] + {build_info:.[1]}' "$W/image.json" "$L/opt/scanp/build-info.json" > "$here/image.json"
echo "image: $(jq -r .image_uri "$here/image.json")"
