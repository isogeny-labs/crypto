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
if [[ -f isolab-ecbench-plan.json ]]; then cp isolab-ecbench-plan.json "$package/payload/"; fi
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
    research/five_leads_e2e_20261019/native/reference.txt; do
    mkdir -p "$(dirname -- "$source_dir/$file")"
    printf '{}\n' > "$source_dir/$file"
  done
  # A source-selected checkpoint executable must never be run with uploader identity.
  printf '#!/bin/sh\nexit 99\n' > "$source_dir/scripts/checkpoint.sh"
  git -C "$source_dir" add .
  git -C "$source_dir" commit -qm fixture
  revision=$(git -C "$source_dir" rev-parse HEAD)
  mkdir -p "$scratch/controls"
  control_dir="$scratch/controls/$name"
  git clone -q "$source_dir" "$control_dir"
  printf '{"schema":"ecbench.spec/v1"}\n' > "$control_dir/spec.json"
  printf '{"schema":"ecbench.plan/v1","arms":[],"workloads":[],"executions":[]}\n' > "$control_dir/plan.json"
  plan_hash=$(sha256sum "$control_dir/plan.json" | cut -d' ' -f1)
  python3 - "$control_dir/protocol.json" "$revision" "$plan_hash" <<'PY'
import hashlib
import json
import sys

path, revision, plan_hash = sys.argv[1:]
value = {
    "schema": "experiment.protocol/v1",
    "arms": [{"id": "fixture", "instrument": {"commit": revision}}],
    "extensions": {"aps_export": {"plan_sha256": plan_hash}},
    "freeze": {
        "status": "frozen",
        "protocol_sha256": None,
        "commit": revision,
        "frozen_at": "2026-11-26T00:00:00Z",
        "filed_before_affected_data": True,
    },
}
body = dict(value)
body.pop("freeze")
canonical = json.dumps(body, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()
value["freeze"]["protocol_sha256"] = hashlib.sha256(canonical).hexdigest()
with open(path, "x") as stream:
    json.dump(value, stream, separators=(",", ":"))
    stream.write("\n")
PY
  git -C "$control_dir" config user.name 'Checkpoint transport test'
  git -C "$control_dir" config user.email 'checkpoint-test@example.invalid'
  git -C "$control_dir" add spec.json plan.json protocol.json
  git -C "$control_dir" commit -qm controls
  control_revision=$(git -C "$control_dir" rev-parse HEAD)
}
run() {
  bash "$scratch/control/taskq/deploy/eks-karpenter/checkpoint-source" \
    "$source_dir" "$revision" "$control_dir" "$control_revision" \
    "${1:-spec.json}" protocol.json plan.json "$scratch/receipt-$case_name" \
    > "$scratch/$case_name.log" 2>&1
}
reject() {
  if run "$@"; then cat "$scratch/$case_name.log"; echo "accepted invalid case: $case_name" >&2; exit 1; fi
}
case_name=valid
fixture "$case_name"
run
jq -e --arg source "$revision" --arg controls "$control_revision" '
  .schema=="isolab.source-checkpoint/v2" and .source_revision==$source
  and .control_revision==$controls and .upload_readback_verified
  and (.experiment_submitted == false)
' "$scratch/receipt-valid/checkpoint.json" >/dev/null
case_name=wrong-head
fixture "$case_name"
revision=0000000000000000000000000000000000000000
reject
case_name=wrong-control-head
fixture "$case_name"
control_revision=0000000000000000000000000000000000000000
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
case_name=dirty-control
fixture "$case_name"
printf 'changed\n' >> "$control_dir/spec.json"
reject
case_name=protocol-source-mismatch
fixture "$case_name"
jq '.freeze.commit="0000000000000000000000000000000000000000"' "$control_dir/protocol.json" > "$control_dir/protocol.json.next"
mv "$control_dir/protocol.json.next" "$control_dir/protocol.json"
git -C "$control_dir" add protocol.json
git -C "$control_dir" commit -qm mismatch
control_revision=$(git -C "$control_dir" rev-parse HEAD)
reject
case_name=stale-protocol-hash
fixture "$case_name"
jq '.question="changed after freeze"' "$control_dir/protocol.json" > "$control_dir/protocol.json.next"
mv "$control_dir/protocol.json.next" "$control_dir/protocol.json"
git -C "$control_dir" add protocol.json
git -C "$control_dir" commit -qm stale
control_revision=$(git -C "$control_dir" rev-parse HEAD)
reject
case_name=arm-source-mismatch
fixture "$case_name"
jq '.arms[0].instrument.commit="0000000000000000000000000000000000000000"' "$control_dir/protocol.json" > "$control_dir/protocol.json.next"
mv "$control_dir/protocol.json.next" "$control_dir/protocol.json"
git -C "$control_dir" add protocol.json
git -C "$control_dir" commit -qm arm-mismatch
control_revision=$(git -C "$control_dir" rev-parse HEAD)
reject
case_name=plan-pin-mismatch
fixture "$case_name"
printf ' \n' >> "$control_dir/plan.json"
git -C "$control_dir" add plan.json
git -C "$control_dir" commit -qm plan-mismatch
control_revision=$(git -C "$control_dir" rev-parse HEAD)
reject
case_name=wrong-account
fixture "$case_name"
CHECKPOINT_TEST_ACCOUNT=000000000000 reject
case_name=tampered-readback
fixture "$case_name"
CHECKPOINT_TEST_TAMPER=1 reject
echo 'Checkpoint transport: two-revision readback accepted; twelve invalid inputs/readbacks rejected; source commands never executed.'
