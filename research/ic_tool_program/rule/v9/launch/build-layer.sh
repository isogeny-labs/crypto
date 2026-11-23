#!/bin/bash
# Assembles the study image's one layer and publishes it to isolab/tasks by
# digest (launch/README.md).  Run from the checkout at the launch commit:
#   build-layer.sh <dir holding bin/> <work dir> <tag>
# Layer: /opt/rulev9/{bin/*, tree.tar, build-info.json, job.sh}, AWS CLI v2
# (S3 sync), procps (`uptime`, which `icprog rule size` records) and jq (the
# pin check), from Ubuntu 24.04's own packages.  Base: ubuntu:24.04 by digest.
set -euo pipefail
BIN=$(realpath "${1:?bin dir}") W=$(realpath -m "${2:?work dir}") TAG=${3:?tag}
here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
rev=$(git -C "$root" rev-parse HEAD)
git -C "$root" diff --quiet HEAD -- research/ic_tool_program research/ic_single_target_20260930 research/ic_descent_20260930 \
  || { echo "the archived trees differ from HEAD" >&2; exit 2; }
L=$W/layer
rm -rf "$W"; mkdir -p "$L/opt/rulev9/bin" "$W/debs"
install -m755 "$BIN"/{ic-v9,ic-v9c,icprog,isolated_bench,koblitz_rho_fixture} "$L/opt/rulev9/bin/"
(cd "$L/opt/rulev9/bin" && sha256sum *) | sort -k2 | cmp - <(sort -k2 "$here/../binaries.sha256") \
  || { echo "bin/ is not what binaries.sha256 names" >&2; exit 2; }
install -m755 "$here/job.sh" "$L/opt/rulev9/job.sh"
git -C "$root" archive --format=tar -o "$L/opt/rulev9/tree.tar" HEAD \
  research/ic_tool_program research/ic_single_target_20260930 research/ic_descent_20260930
(cd "$W/debs" && apt-get download procps libproc2-0 jq libjq1 libonig5 >/dev/null)
for d in "$W"/debs/*.deb; do dpkg-deb -x "$d" "$L"; done
curl -fsSL -o "$W/awscliv2.zip" https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip
(cd "$W" && unzip -q awscliv2.zip)
mkdir -p "$L/usr/local/aws-cli" "$L/usr/local/bin"
cp -a "$W/aws/dist" "$L/usr/local/aws-cli/dist"
ln -s /usr/local/aws-cli/dist/aws "$L/usr/local/bin/aws"
jq -n --arg rev "$rev" --arg tag "$TAG" --arg built "$(date -u +%FT%TZ)" \
  --arg tree "$(sha256sum "$L/opt/rulev9/tree.tar" | cut -d' ' -f1)" \
  --arg awscli "$(sha256sum "$W/awscliv2.zip" | cut -d' ' -f1)" \
  --argjson bins "$(cd "$L/opt/rulev9/bin" && sha256sum * | jq -Rn '[inputs|split("  ")|{(.[1]):.[0]}]|add')" \
  --argjson debs "$(cd "$W/debs" && sha256sum *.deb | jq -Rn '[inputs|split("  ")|{(.[1]):.[0]}]|add')" \
  '{source_repository:"https://github.com/isogeny-labs/crypto",launch_revision:$rev,tag:$tag,built_at:$built,
    binary_sha256:$bins,tree_tar_sha256:$tree,awscli_zip_sha256:$awscli,deb_sha256:$debs,
    binary_sources:{"ic-v9":"f465f2fb1b037a57fa1d923ac8eea27ab9a603e2","ic-v9c":"370e19193f6f30bf04ca2e82a2c3626932dc069e",
      icprog:"9d76e4de0918 (rule-v9)",isolated_bench:"f465f2fb1b037a57fa1d923ac8eea27ab9a603e2",
      koblitz_rho_fixture:"f465f2fb1b037a57fa1d923ac8eea27ab9a603e2"}}' > "$L/opt/rulev9/build-info.json"
python3 "$root/taskq/deploy/eks-karpenter/oci-layer-publish.py" \
  --base public.ecr.aws/docker/library/ubuntu:24.04 --layer-dir "$L" \
  --target 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks --tag "$TAG" \
  --user 1000:1000 --workdir /work --env HOME=/work \
  --label org.opencontainers.image.revision="$rev" --out "$W/image.json"
jq -s '.[0] + {build_info:.[1]}' "$W/image.json" "$L/opt/rulev9/build-info.json" > "$here/image.json"
echo "image: $(jq -r .image_uri "$here/image.json")"
