#!/usr/bin/env bash
# Round-2 build of the study image (slug r2pkm). Differences from build_layer.sh, all deliberate:
#  1. archives ONLY the paths the four examples need (build_layer.sh archives the whole repo, which filled the disk);
#  2. uses the installed `cargo` (rustc 1.97.0) instead of `cargo +1.99.1` (1.99.1 is pinned in the repo only for
#     tools/backend-leaderboard; nothing in Cargo.toml or the image workflow requires it for these examples);
#  3. also installs image/entry_r2.sh (= entry.sh + the m4n: stage) next to entry.sh;
#  4. tag is pkm-fd-<first 12 hex of the commit>.
# Usage: build_layer_r2.sh COMMIT [WORKDIR]   (WORKDIR is reused if it already holds the archived tree)
set -euo pipefail
commit=${1:?40-character commit SHA}; [[ "$commit" =~ ^[a-f0-9]{40}$ ]]
here=$(cd "$(dirname "$0")" && pwd); repo=$(cd "$here/../../.." && pwd)
git -C "$repo" cat-file -e "$commit^{commit}"
work=${2:-$(mktemp -d)}; mkdir -p "$work"
if [[ ! -f $work/Cargo.toml ]]; then
  git -C "$repo" archive "$commit" Cargo.toml Cargo.lock src benches examples crates isogeny_algos \
    tools/factor-base tools/volcano-cli research/pencil_large_prime_20261029 \
    research/conductor_gap_cm_orbits_20261015/native research/conductor_gap_followups_20261016/native \
    research/ct_dynamic_audit_20261102/src research/full_volcano_graphwide_ic_20261017/execution \
    research/n83_highk_beyond4405_20261021/native_exclusion_audit.rs \
    experiments/koblitz-n53-compact-cold-20261009 experiments/koblitz-s3-pair-query-20261007-v4 \
    research/prime_fall_degree_20261027/run.sh | tar -x -C "$work"
fi
( cd "$work"
  export CC_x86_64_unknown_linux_musl=musl-gcc RUSTFLAGS='-C target-cpu=x86-64'
  cargo build --locked --release --no-default-features --target x86_64-unknown-linux-musl \
    --example pkm_tower_pilot --example prime_fall_degree_tools --example hky_fp_tower_check --example pfd_grid_zeros )
ex=$work/target/x86_64-unknown-linux-musl/release/examples
layer=$work/layer; rm -rf "$layer"; mkdir -p "$layer/usr/local/bin" "$layer/opt/study"
for b in pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros; do
  readelf -l "$ex/$b" | grep -q INTERP && { echo "$b is not static" >&2; exit 1; }
  install -m755 "$ex/$b" "$layer/usr/local/bin/$b"
done
install -m755 "$here/entry.sh" "$layer/usr/local/bin/entry.sh"
install -m755 "$here/entry_r2.sh" "$layer/usr/local/bin/entry_r2.sh"
install -m644 "$work/research/prime_fall_degree_20261027/run.sh" "$layer/opt/study/run.sh"
( cd "$layer/usr/local/bin" && sha256sum pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros entry.sh entry_r2.sh ) > "$layer/opt/study/binaries.sha256"
( cd "$layer/opt/study" && sha256sum run.sh ) >> "$layer/opt/study/binaries.sha256"
jq -n --arg r "$commit" --arg rustc "$(rustc -V)" --slurpfile s <(jq -R 'split("  ")|{(.[1]):.[0]}' "$layer/opt/study/binaries.sha256" | jq -s add) \
  '{source_revision:$r,architecture:"linux/amd64",target:"x86_64-unknown-linux-musl",rustc:$rustc,
    toolchain_note:"build_layer.sh pins cargo +1.99.1; this build used the installed toolchain",sha256:$s[0]}' > "$layer/opt/study/build-info.json"
cp "$layer/opt/study/binaries.sha256" "$here/binaries.sha256"; cp "$layer/opt/study/build-info.json" "$here/build-info.json"
python3 -I "$repo/taskq/deploy/eks-karpenter/oci-layer-publish.py" \
  --base public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39 \
  --layer-dir "$layer" --target 104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks --tag "pkm-fd-${commit:0:12}" \
  --user 1000:1000 --workdir /work --env HOME=/work --label "org.opencontainers.image.revision=$commit" \
  --out "$here/image.json"
echo "receipt: $here/image.json"
