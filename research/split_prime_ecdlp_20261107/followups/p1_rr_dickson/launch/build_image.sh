#!/usr/bin/env bash
# build_image.sh COMMIT -- P1 round 3 study image: ubuntu:24.04 + ONE layer (msolve 0.6.5 and its
# libraries from the Ubuntu noble .debs, bc, procps, AWS CLI v2, p1tool built from COMMIT, the
# frozen driver scripts), pushed to isolab/tasks by digest with oci-layer-publish.py (no daemon).
# Pattern: research/pkm_falldegree_unblock_20261107/image/build_layer.sh. Receipt: launch/image.json.
set -euo pipefail
commit=${1:?40-character commit SHA}; [[ "$commit" =~ ^[a-f0-9]{40}$ ]]
here=$(cd "$(dirname "$0")" && pwd); repo=$(git -C "$here" rev-parse --show-toplevel)
study=research/split_prime_ecdlp_20261107/followups/p1_rr_dickson
git -C "$repo" cat-file -e "$commit^{commit}"
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
git -C "$repo" archive "$commit" "$study" | tar -x -C "$work"
src=$work/$study
( cd "$src/p1tool" && CARGO_TARGET_DIR=$work/target cargo build --release --locked )
mkdir -p "$work/debs" && ( cd "$work/debs" && apt-get download msolve libmsolve-0.6.5 libflint18t64 libmpfr6 libgmp10 libgomp1 bc procps libproc2-0 )
layer=$work/layer; mkdir -p "$layer/usr/local/bin" "$layer/opt/study/calibration" "$layer/opt/study/p1tool/src" "$layer/opt/study/launch"
for d in "$work"/debs/*.deb; do dpkg -x "$d" "$layer"; done
[ "$(sha256sum "$layer/usr/bin/msolve" | cut -d' ' -f1)" = c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa ] \
  || { echo "msolve binary differs from the R2b bind" >&2; exit 1; }
curl -sSf -o "$work/awscli.zip" https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip
( cd "$work" && unzip -q awscli.zip ) && mkdir -p "$layer/usr/local/aws-cli" && cp -a "$work/aws/dist" "$layer/usr/local/aws-cli/dist"
ln -s ../aws-cli/dist/aws "$layer/usr/local/bin/aws"
install -m755 "$work/target/release/p1tool" "$layer/usr/local/bin/p1tool"
install -m755 "$src/calibration/run_cells_r3.sh" "$src/calibration/msolve_cell.sh" "$layer/opt/study/calibration/"
install -m644 "$src/p1tool/src/main.rs" "$layer/opt/study/p1tool/src/main.rs"
install -m755 "$src/launch/job-r3.sh" "$layer/opt/study/launch/job-r3.sh"
( cd "$layer" && sha256sum usr/local/bin/p1tool usr/bin/msolve opt/study/calibration/run_cells_r3.sh opt/study/calibration/msolve_cell.sh opt/study/p1tool/src/main.rs opt/study/launch/job-r3.sh ) > "$layer/opt/study/binaries.sha256"
jq -n --arg r "$commit" --rawfile s "$layer/opt/study/binaries.sha256" --arg aws "$(sha256sum "$work/awscli.zip" | cut -d' ' -f1)" \
  '{source_revision:$r,architecture:"linux/amd64",file_sha256:($s|split("\n")|map(select(length>0)|split("  ")|{(.[1]):.[0]})|add),awscli_zip_sha256:$aws}' \
  > "$layer/opt/study/build-info.json"
cp "$layer/opt/study/build-info.json" "$here/build-info.json"
python3 -I "$repo/taskq/deploy/eks-karpenter/oci-layer-publish.py" \
  --base public.ecr.aws/docker/library/ubuntu:24.04 \
  --layer-dir "$layer" --target 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks --tag "p1r3-${commit:0:12}" \
  --user 1000:1000 --workdir /work --env HOME=/work --label "org.opencontainers.image.revision=$commit" \
  --out "$here/image.json"
echo "receipt: $here/image.json"
