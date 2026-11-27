#!/usr/bin/env bash
# Offline policy controls only; copies and mutates scratch JSON, never runs a solver.
set -euo pipefail
here=$(cd -- "$(dirname -- "$0")" && pwd)
relative=research/unified_point_index_20261127/panel-bridge-20261127
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
for arm in baseline candidate; do
  arm_dir="$scratch/$arm"
  mkdir -p "$arm_dir/crates/hashed-ecdl-tensor"
  printf 'fixture native %s\n' "$arm" > "$arm_dir/crates/hashed-ecdl-tensor/Cargo.toml"
  git -C "$arm_dir" init -q
  git -C "$arm_dir" config user.name 'Point index input fixture'
  git -C "$arm_dir" config user.email 'point-index-fixture@example.invalid'
  git -C "$arm_dir" add .
  git -C "$arm_dir" commit -qm "$arm fixture"
done
source_dir="$scratch/source"
study="$source_dir/$relative"
mkdir -p "$study"
cp -a "$here/fresh-inputs-20261127" "$study/"
cp "$here/optimized-options.json" "$study/"
cp "$here/check-frozen-inputs.sh" "$study/"
cp "$here/build-pair.sh" "$here/native-metrics.patch" "$study/"
baseline=$(git -C "$scratch/baseline" rev-parse HEAD)
candidate=$(git -C "$scratch/candidate" rev-parse HEAD)
for path in Cargo.toml Cargo.lock docs/experiment-seeds.md \
  .github/workflows/point-index-panel-build.yml \
  .github/workflows/point-index-panel-publish.yml \
  .github/workflows/point-index-panel-checkpoint.yml; do
  mkdir -p "$source_dir/$(dirname "$path")"
  printf 'fixture source %s\n' "$path" > "$source_dir/$path"
done
mkdir -p "$source_dir/crates/hashed-ecdl-tensor" "$source_dir/scripts"
printf 'fixture driver crate\n' > "$source_dir/crates/hashed-ecdl-tensor/Cargo.toml"
cat > "$source_dir/scripts/checkpoint.sh" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ $1 == gate ]]; then test -s .resume/.last-push; exit; fi
[[ $1 == push && $2 == --force ]]
package=$(mktemp -d "$CHECKPOINT_TEST_STORE/package.XXXXXX")
mkdir "$package/payload"
cp point-index-spec.json point-index-protocol.json point-index-*-source.tar.gz "$package/payload/"
if [[ ${CHECKPOINT_TEST_TAMPER:-0} == 1 ]]; then printf '\n' >> "$package/payload/point-index-spec.json"; fi
(cd "$package/payload" && tar -czf "$package/untracked.tar.gz" -- *)
cp .resume/RESUME.md "$package/RESUME.md"
jq -n --arg head "$(git rev-parse HEAD)" '{head:$head}' > "$package/meta.json"
tar -czf "$CHECKPOINT_TEST_STORE/checkpoint.tar.gz" -C "$package" .
printf 'fixture stamp s3://setec-astronomy-artifacts/%s/fixture/\n' "$CHECKPOINT_PREFIX" > .resume/.last-push
STUB
chmod +x "$source_dir/scripts/checkpoint.sh" "$study/check-frozen-inputs.sh"
git -C "$source_dir" init -q
git -C "$source_dir" config user.name 'Point index input fixture'
git -C "$source_dir" config user.email 'point-index-fixture@example.invalid'
git -C "$source_dir" add crates/hashed-ecdl-tensor
partial_tree=$(git -C "$source_dir" write-tree)
driver_tree=$(git -C "$source_dir" rev-parse "$partial_tree:crates/hashed-ecdl-tensor")
builder_sha=$(sha256sum "$study/build-pair.sh" | cut -d' ' -f1)
overlay_sha=$(sha256sum "$study/native-metrics.patch" | cut -d' ' -f1)
panel="$study/fresh-inputs-20261127/panel"
manifest="$panel/manifest.json"
hashes="$study/fresh-inputs-20261127/inputs.sha256"
options="$study/optimized-options.json"

rewrite() { jq "$2" "$1" > "$1.next" && mv "$1.next" "$1"; }
# The fixture's new manifest is scratch-only. The checked-in prepared panel
# deliberately retains pending source markers.
jq --arg base "$baseline" --arg candidate "$candidate" \
  '.baseline=$base | .paired_baseline=$candidate' "$manifest" > "$manifest.next"
mv "$manifest.next" "$manifest"
(cd "$panel" && sha256sum manifest.json problems/*.json seed-manifest.json > "$hashes")
panel_sha=$(sha256sum "$manifest" | cut -d' ' -f1)
seeds_sha=$(sha256sum "$panel/seed-manifest.json" | cut -d' ' -f1)
inputs_sha=$(sha256sum "$hashes" | cut -d' ' -f1)
options_sha=$(sha256sum "$options" | cut -d' ' -f1)
jq -n --arg baseline "$baseline" --arg candidate "$candidate" \
  --arg panel "$relative/fresh-inputs-20261127/panel" \
  --arg inputs "$relative/fresh-inputs-20261127/inputs.sha256" \
  --arg options "$relative/optimized-options.json" \
  --arg panel_sha "$panel_sha" --arg seeds_sha "$seeds_sha" \
  --arg inputs_sha "$inputs_sha" --arg options_sha "$options_sha" \
  --arg driver_tree "$driver_tree" --arg builder_sha "$builder_sha" \
  --arg overlay_sha "$overlay_sha" \
  '{schema:"point-index.panel.spec/v1",study_id:"crypto-point-index-fresh-panel-20261127",
    baseline_revision:$baseline,candidate_revision:$candidate,
    panel_dir:$panel,inputs_path:$inputs,optimized_options_path:$options,
    panel_manifest_sha256:$panel_sha,seed_manifest_sha256:$seeds_sha,
    inputs_sha256:$inputs_sha,optimized_options_sha256:$options_sha,
    driver_crate_tree:$driver_tree,builder_sha256:$builder_sha,
    measurement_overlay_sha256:$overlay_sha,
    threads:1,deadline_secs:1800,aa_rounds:5,ab_rounds:5,repetitions:3,
    experiment_submitted:false}' > "$study/spec.json"
jq -n '{schema:"experiment.protocol/v1",freeze:{status:"frozen",filed_before_affected_data:true}}' > "$study/protocol.json"
spec_sha=$(sha256sum "$study/spec.json" | cut -d' ' -f1)
protocol_sha=$(sha256sum "$study/protocol.json" | cut -d' ' -f1)
jq -n --arg baseline "$baseline" --arg candidate "$candidate" \
  --arg spec "$relative/spec.json" --arg protocol "$relative/protocol.json" \
  --arg spec_sha "$spec_sha" --arg protocol_sha "$protocol_sha" \
  '{schema:"point-index.publish-inputs/v1",status:"frozen",
    baseline_revision:$baseline,candidate_revision:$candidate,
    spec_path:$spec,spec_sha256:$spec_sha,protocol_path:$protocol,protocol_sha256:$protocol_sha,
    experiment_submitted:false}' > "$study/publish-inputs.json"
git -C "$source_dir" add .
git -C "$source_dir" commit -qm fixture
driver=$(git -C "$source_dir" rev-parse HEAD)
"$here/check-frozen-inputs.sh" "$source_dir" "$driver"
cp -a "$source_dir" "$scratch/source-changed"
printf 'changed\n' >> "$scratch/source-changed/crates/hashed-ecdl-tensor/Cargo.toml"
git -C "$scratch/source-changed" add crates/hashed-ecdl-tensor/Cargo.toml
git -C "$scratch/source-changed" commit -qm 'change native source'
changed_driver=$(git -C "$scratch/source-changed" rev-parse HEAD)
if "$here/check-frozen-inputs.sh" "$scratch/source-changed" "$changed_driver" > "$scratch/changed-source.log" 2>&1; then
  echo 'accepted changed native source' >&2
  exit 1
fi
[[ $(<"$scratch/changed-source.log") == *'driver native source tree changed'* ]] || { cat "$scratch/changed-source.log" >&2; exit 1; }
"$here/checkpoint-pair.sh" --check-only "$source_dir" "$source_dir" "$driver" \
  "$scratch/baseline" "$baseline" "$scratch/candidate" "$candidate" "$scratch/check-only-receipt"
mkdir "$scratch/store" "$scratch/bin"
cat > "$scratch/bin/aws" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ $1 == sts ]]; then
  jq -n --arg account "${CHECKPOINT_TEST_ACCOUNT:-104186811093}" '{Account:$account}'
elif [[ $1 == s3 && $2 == cp ]]; then
  cp "$CHECKPOINT_TEST_STORE/checkpoint.tar.gz" "${@: -1}"
else
  exit 90
fi
STUB
chmod +x "$scratch/bin/aws"
export PATH="$scratch/bin:$PATH" CHECKPOINT_TEST_STORE="$scratch/store" GITHUB_RUN_ID=123 GITHUB_RUN_ATTEMPT=1
cp -a "$source_dir" "$scratch/source-tamper"
cp -a "$source_dir" "$scratch/source-wrong-account"
"$here/checkpoint-pair.sh" "$source_dir" "$source_dir" "$driver" \
  "$scratch/baseline" "$baseline" "$scratch/candidate" "$candidate" "$scratch/receipt-full" >/dev/null
jq -e '.upload_readback_verified and .experiment_submitted==false' "$scratch/receipt-full/checkpoint.json" >/dev/null
tar -tzf "$source_dir/point-index-driver-source.tar.gz" > "$scratch/driver-archive.list"
for required in "$relative/fresh-inputs-20261127/panel/seed-manifest.json" \
  'crates/hashed-ecdl-tensor/Cargo.toml' 'docs/experiment-seeds.md' \
  '.github/workflows/point-index-panel-checkpoint.yml'; do
  [[ $(<"$scratch/driver-archive.list") == *"$required"* ]] || { echo "missing checkpoint source: $required" >&2; exit 1; }
done
for arm in baseline candidate; do
  tar -tzf "$source_dir/point-index-$arm-source.tar.gz" > "$scratch/$arm-archive.list"
  [[ $(<"$scratch/$arm-archive.list") == *'crates/hashed-ecdl-tensor/Cargo.toml'* ]] || { echo "missing $arm crate" >&2; exit 1; }
done
if CHECKPOINT_TEST_TAMPER=1 "$here/checkpoint-pair.sh" \
  "$scratch/source-tamper" "$scratch/source-tamper" "$driver" \
  "$scratch/baseline" "$baseline" "$scratch/candidate" "$candidate" "$scratch/receipt-tampered" \
  > "$scratch/tampered-readback.log" 2>&1; then
  echo 'accepted tampered S3 readback' >&2
  exit 1
fi
[[ $(<"$scratch/receipt-tampered/payload-readback.txt") == *'point-index-spec.json: FAILED'* ]] || { cat "$scratch/tampered-readback.log" >&2; exit 1; }
if CHECKPOINT_TEST_ACCOUNT=000000000000 "$here/checkpoint-pair.sh" \
  "$scratch/source-wrong-account" "$scratch/source-wrong-account" "$driver" \
  "$scratch/baseline" "$baseline" "$scratch/candidate" "$candidate" "$scratch/receipt-wrong-account" \
  > "$scratch/wrong-account.log" 2>&1; then
  echo 'accepted wrong AWS account' >&2
  exit 1
fi
[[ $(<"$scratch/receipt-wrong-account/caller-account-check.json") == false ]] || { cat "$scratch/wrong-account.log" >&2; exit 1; }

reject() {
  local name=$1 expected=$2; shift 2
  if "$here/check-frozen-inputs.sh" "$@" > "$scratch/$name.log" 2>&1; then
    echo "accepted invalid $name" >&2
    exit 1
  fi
  [[ $(<"$scratch/$name.log") == *"$expected"* ]] || { echo "unexpected rejection for $name" >&2; cat "$scratch/$name.log" >&2; exit 1; }
}
reject wrong-driver 'driver checkout SHA mismatch' "$source_dir" 0000000000000000000000000000000000000002
cp "$study/publish-inputs.json" "$scratch/publish-good.json"
rewrite "$study/publish-inputs.json" '.spec_sha256="0000000000000000000000000000000000000000000000000000000000000000"'
reject wrong-spec-hash 'spec hash mismatch' "$source_dir" "$driver"
cp "$scratch/publish-good.json" "$study/publish-inputs.json"
rewrite "$study/publish-inputs.json" '.spec_path="../spec.json"'
reject unsafe-spec-path 'unsafe or missing spec' "$source_dir" "$driver"
cp "$scratch/publish-good.json" "$study/publish-inputs.json"
printf '\n' >> "$panel/problems/p5-development-f00-t0.json"
reject mutated-problem 'panel input readback mismatch' "$source_dir" "$driver"
cp "$here/fresh-inputs-20261127/panel/problems/p5-development-f00-t0.json" \
  "$panel/problems/p5-development-f00-t0.json"
printf '%064d  ../outside.json\n' 0 >> "$hashes"
new_hash=$(sha256sum "$hashes" | cut -d' ' -f1)
jq --arg hash "$new_hash" '.inputs_sha256=$hash' "$study/spec.json" > "$study/spec.next.json"
mv "$study/spec.next.json" "$study/spec.json"
new_spec=$(sha256sum "$study/spec.json" | cut -d' ' -f1)
jq --arg hash "$new_spec" '.spec_sha256=$hash' "$study/publish-inputs.json" > "$study/publish.next.json"
mv "$study/publish.next.json" "$study/publish-inputs.json"
reject unsafe-hash-path 'unsafe input hash path' "$source_dir" "$driver"
echo 'Native pair input guard: valid frozen fixture and mocked checkpoint/readback accepted; changed source, five invalid inputs, tampered readback and wrong AWS account rejected; no real AWS or solver used.'
