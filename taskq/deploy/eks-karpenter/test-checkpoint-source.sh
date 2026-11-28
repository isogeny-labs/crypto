#!/usr/bin/env bash
# Infrastructure controls only: mocked AWS/transport; no cloud operation or solver.
set -euo pipefail
here=$(cd -- "$(dirname -- "$0")" && pwd)
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
mkdir -p "$scratch/control/taskq/deploy/eks-karpenter" "$scratch/control/scripts" "$scratch/bin" "$scratch/store"
cp "$here/checkpoint-source" "$scratch/control/taskq/deploy/eks-karpenter/checkpoint-source"
export CHECKPOINT_TEST_STORE="$scratch/store" CHECKPOINT_TEST_CALLS="$scratch/calls"
cat > "$scratch/bin/aws" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$CHECKPOINT_TEST_CALLS"
if [[ $1 == sts ]]; then
  jq -n --arg account "${CHECKPOINT_TEST_ACCOUNT:-104186811093}" '{Account:$account}'
elif [[ $1 == s3 && $2 == cp ]]; then
  cp "$CHECKPOINT_TEST_STORE/checkpoint.tar.gz" "${@: -1}"
else
  exit 90
fi
STUB
cat > "$scratch/control/scripts/checkpoint.sh" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ $1 == gate ]]; then test -s .resume/.last-push; exit; fi
[[ $1 == push && $2 == --force ]]
package=$(mktemp -d "$CHECKPOINT_TEST_STORE/package.XXXXXX")
mkdir "$package/payload"
cp isolab-ecbench-spec.json isolab-experiment-protocol.json isolab-source-snapshot.tar.gz "$package/payload/"
if [[ ${CHECKPOINT_TEST_TAMPER:-0} == 1 ]]; then printf '\n' >> "$package/payload/isolab-ecbench-spec.json"; fi
(cd "$package/payload" && tar -czf "$package/untracked.tar.gz" -- *)
rm -rf "$package/payload"
cp .resume/RESUME.md "$package/RESUME.md"
jq -n --arg head "$(git rev-parse HEAD)" '{head:$head}' > "$package/meta.json"
tar -czf "$CHECKPOINT_TEST_STORE/checkpoint.tar.gz" -C "$package" .
printf 'fixture stamp s3://setec-astronomy-artifacts/%s/fixture/\n' "$CHECKPOINT_PREFIX" > .resume/.last-push
STUB
chmod +x "$scratch/bin/aws" "$scratch/control/scripts/checkpoint.sh"
export PATH="$scratch/bin:$PATH"

fixture() {
  local name=$1
  source_dir="$scratch/$name"
  mkdir "$source_dir"
  git -C "$source_dir" init -q
  git -C "$source_dir" config user.name 'Checkpoint transport test'
  git -C "$source_dir" config user.email 'checkpoint-test@example.invalid'
  for file in Cargo.toml Cargo.lock src/lib.rs crates/reference.txt isogeny_algos/reference.txt \
    scripts/checkpoint.sh tools/artifacts/reference.txt tools/factor-base/reference.txt tools/volcano-cli/reference.txt \
    taskq/deploy/eks-karpenter/reference.txt .github/workflows/reference.yml .artifacts/config.json \
    docs/curves/registry.json docs/curves/standards/parameters.json docs/ic/boundary_targets.json \
    docs/ic/calibration.json research/pencil_large_prime_20261029/reference.txt \
    research/five_leads_e2e_20261019/native/reference.txt spec.json protocol.json; do
    mkdir -p "$(dirname -- "$source_dir/$file")"
    printf '{}\n' > "$source_dir/$file"
  done
  # A source-selected checkpoint executable must never be run with uploader identity.
  printf '#!/bin/sh\nexit 99\n' > "$source_dir/scripts/checkpoint.sh"
  printf '{"schema":"ecbench.spec/v1"}\n' > "$source_dir/spec.json"
  printf '{"schema":"experiment.protocol/v1","freeze":{"status":"frozen","filed_before_affected_data":true}}\n' > "$source_dir/protocol.json"
  git -C "$source_dir" add .
  git -C "$source_dir" commit -qm fixture
  revision=$(git -C "$source_dir" rev-parse HEAD)
}
run() {
  bash "$scratch/control/taskq/deploy/eks-karpenter/checkpoint-source" \
    "$source_dir" "$revision" "${1:-spec.json}" protocol.json "$scratch/receipt-$case_name" \
    > "$scratch/$case_name.log" 2>&1
}
reject() {
  if run "$@"; then cat "$scratch/$case_name.log"; echo "accepted invalid case: $case_name" >&2; exit 1; fi
}
case_name=valid
fixture "$case_name"
run
jq -e '.upload_readback_verified and (.experiment_submitted == false)' "$scratch/receipt-valid/checkpoint.json" >/dev/null
case_name=wrong-head
fixture "$case_name"
revision=0000000000000000000000000000000000000000
reject
case_name=parent-path
fixture "$case_name"
reject ../valid/spec.json
case_name=dirty
fixture "$case_name"
printf 'changed\n' >> "$source_dir/spec.json"
reject
case_name=untracked
fixture "$case_name"
printf 'untracked\n' > "$source_dir/untracked.txt"
reject
case_name=wrong-account
fixture "$case_name"
CHECKPOINT_TEST_ACCOUNT=000000000000 reject
case_name=tampered-readback
fixture "$case_name"
CHECKPOINT_TEST_TAMPER=1 reject
echo 'Checkpoint transport: valid readback accepted; six invalid inputs/readbacks rejected; source commands never executed.'
