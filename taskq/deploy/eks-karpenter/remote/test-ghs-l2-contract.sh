#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
VALIDATE="$ROOT/remote/validate-ghs-l2-request"
VERIFY="$ROOT/remote/verify-ghs-l2-artifacts"
SCHEMA="$ROOT/contracts/ghs-l2-request.schema.json"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
jq -e . "$SCHEMA" >/dev/null

zero40=$(printf '0%.0s' {1..40})
one64=$(printf '1%.0s' {1..64})
two64=$(printf '2%.0s' {1..64})
three64=$(printf '3%.0s' {1..64})
four64=$(printf '4%.0s' {1..64})
pod_uid=12345678-1234-1234-1234-123456789abc
image="104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:$one64"

jq -n \
  --arg src "$zero40" --arg image "$image" --arg binary "$two64" \
  --arg runner "$three64" --arg validator "$four64" --arg protocol "$one64" '
{
  schema: "isolab.eks.ghs-l2-request/v1",
  job: {
    job_name: "ghs-l2-contract-test", instance_type: "c8a.2xlarge",
    cpu: 4, memory: "4Gi", seconds: 900, retries: 0, isolation_required: "L2",
    build: {
      image_uri: $image, source_revision: $src,
      binary_path: "/usr/local/bin/ghs-strong-rho", binary_sha256: $binary,
      validator_path: "/usr/local/bin/ghs-l2-validate", validator_sha256: $validator
    },
    runner: {
      path: "/usr/local/bin/ghs-l2-runner", sha256: $runner,
      argv: ["/usr/local/bin/ghs-l2-runner", "--spec", "/opt/isolab/ghs-l2-spec.json", "--out", "/work/results"]
    },
    spec: {
      schema: "ghs.l2.spec/v1", suite_id: "contract-test",
      source_revision: $src, protocol_sha256: $protocol,
      workload: {q: 4051, instance_seed: 101, target_index: 0, target_policy: "sequence"},
      measurement: {
        warmup: 1, rounds: 5,
        warmup_order: [["reference", "control", "candidate"]],
        order: [
          ["reference", "control", "candidate"], ["control", "candidate", "reference"],
          ["candidate", "reference", "control"], ["reference", "candidate", "control"],
          ["control", "reference", "candidate"]
        ]
      },
      arms: [
        {name: "reference", role: "reference", driver_argv: ["/usr/local/bin/ghs-strong-rho", "run", "4051", "101", "0", "1", "sequence", "fast-b8"]},
        {name: "control", role: "control", driver_argv: ["/usr/local/bin/ghs-strong-rho", "run", "4051", "101", "0", "1", "sequence", "fast-b8"]},
        {name: "candidate", role: "candidate", driver_argv: ["/usr/local/bin/ghs-strong-rho", "run", "4051", "101", "0", "1", "sequence", "strong-b8"]}
      ],
      replay: {
        all: true, validator_path: "/usr/local/bin/ghs-l2-validate", validator_sha256: $validator,
        canonical_excluded_fields: ["timestamps", "wall_intervals", "cpu_intervals", "scheduler_counters", "host_load"]
      },
      required_artifacts: [
        "build-info.json", "control-comparison.json", "input-request.json",
        "job-exit.json", "l2-gate.json", "replay.json", "session.jsonl",
        "validation.json", "warmup.json", "SHA256SUMS"
      ]
    }
  },
  provenance: {harness: "isolab-eks/ghs-l2-bridge-v1", source_revision: $src, protocol_sha256: $protocol}
}' >"$TMP/request.json"

"$VALIDATE" "$TMP/request.json" >/dev/null
if command -v jsonschema >/dev/null 2>&1; then
  jsonschema -i "$TMP/request.json" "$SCHEMA" >/dev/null 2>&1
fi
jq '.job.spec.workload.target_policy = "ic-repeat" |
    (.job.spec.arms[].driver_argv[6]) = "ic-repeat"' \
  "$TMP/request.json" >"$TMP/ic-repeat.json"
"$VALIDATE" "$TMP/ic-repeat.json" >/dev/null

expect_reject() {
  local name=$1 filter=$2
  jq "$filter" "$TMP/request.json" >"$TMP/$name.json"
  if "$VALIDATE" "$TMP/$name.json" >/dev/null 2>&1; then
    echo "validator accepted negative fixture: $name" >&2
    exit 1
  fi
}
expect_reject retries '.job.retries = 1'
expect_reject tag_image '.job.build.image_uri = "example.invalid/tasks:latest"'
expect_reject wrong_account '.job.build.image_uri |= sub("^104186811093"; "999999999999")'
expect_reject shell '.job.runner.argv = ["/bin/sh", "-c", "echo unsafe"]'
expect_reject short_rounds '.job.spec.measurement.rounds = 4 | .job.spec.measurement.order = .job.spec.measurement.order[:4]'
expect_reject mismatched_control '.job.spec.arms[1].driver_argv[7] = "strong-b8"'
expect_reject wrong_interval '.job.spec.arms[2].driver_argv[5] = "2"'
expect_reject missing_replay '.job.spec.replay.all = false'
expect_reject unknown_field '.job.shell = "true"'

mkdir "$TMP/artifacts"
cp "$TMP/request.json" "$TMP/artifacts/input-request.json"
request_sha=$(sha256sum "$TMP/request.json" | awk '{print $1}')

jq -n --arg src "$zero40" --arg b "$two64" --arg r "$three64" --arg v "$four64" \
  '{schema:"ghs.l2.build-info/v1",source_revision:$src,binary_sha256:$b,runner_sha256:$r,validator_sha256:$v}' \
  >"$TMP/artifacts/build-info.json"
jq -n --arg uid "$pod_uid" '{schema:"ghs.l2.job-exit/v1",pod_uid:$uid,exit_code:0,retries:0}' \
  >"$TMP/artifacts/job-exit.json"
jq -n --arg sha "$request_sha" --arg src "$zero40" \
  '{schema:"ghs.l2.warmup/v1",request_sha256:$sha,source_revision:$src,expected:3,completed:3,failures:0}' \
  >"$TMP/artifacts/warmup.json"
jq -n --arg sha "$request_sha" --arg src "$zero40" \
  '{schema:"ghs.l2.validation/v1",request_sha256:$sha,source_revision:$src,ok:true,records:15,verified:15,wrong_answers:0,failures:0}' \
  >"$TMP/artifacts/validation.json"
jq -n --arg sha "$request_sha" \
  '{schema:"ghs.l2.replay/v1",request_sha256:$sha,all:true,ok:true,records:15,reproduced:15,mismatches:0}' \
  >"$TMP/artifacts/replay.json"
jq -n --arg sha "$request_sha" --arg uid "$pod_uid" \
  '{schema:"ghs.l2.gate/v1",request_sha256:$sha,pod_uid:$uid,accepted:true,isolation:"L2",executions:15,below_l2:[],invalid_outcomes:[]}' \
  >"$TMP/artifacts/l2-gate.json"
jq -n --arg sha "$request_sha" \
  '{schema:"ghs.l2.control-comparison/v1",request_sha256:$sha,wall:{status:"admitted",pairs:5,pairs_below_level:0,ratio_ci95:[0.97,1.03]}}' \
  >"$TMP/artifacts/control-comparison.json"
for _ in $(seq 1 15); do
  jq -cn --arg sha "$request_sha" --arg src "$zero40" --arg uid "$pod_uid" \
    '{schema:"ghs.l2.record/v1",request_sha256:$sha,source_revision:$src,pod_uid:$uid,isolation:"L2",outcome:"ok",validated:true}'
done >"$TMP/artifacts/session.jsonl"

(cd "$TMP/artifacts" && sha256sum \
  build-info.json control-comparison.json input-request.json job-exit.json \
  l2-gate.json replay.json session.jsonl validation.json warmup.json >SHA256SUMS)

jq -n --arg uid "$pod_uid" --arg image "$image" '{pods:[{
  uid:$uid, status:{containerStatuses:[{
    imageID:("docker-pullable://"+$image), restartCount:0,
    state:{terminated:{exitCode:0}}
  }]}
}]}' >"$TMP/status.json"

set +e
"$VERIFY" "$TMP/request.json" "$TMP/status.json" "$pod_uid" "$TMP/artifacts" >/dev/null 2>&1
nonadmit_exit=$?
set -e
[[ $nonadmit_exit == 3 ]] || { echo "artifact verifier did not fail closed without native isolation evidence" >&2; exit 1; }
"$VERIFY" "$TMP/request.json" "$TMP/status.json" "$pod_uid" "$TMP/artifacts" --contract-test-only >/dev/null

cp -a "$TMP/artifacts" "$TMP/tampered"
jq '.accepted = false' "$TMP/tampered/l2-gate.json" >"$TMP/tampered/l2-gate.tmp"
mv "$TMP/tampered/l2-gate.tmp" "$TMP/tampered/l2-gate.json"
if "$VERIFY" "$TMP/request.json" "$TMP/status.json" "$pod_uid" "$TMP/tampered" --contract-test-only >/dev/null 2>&1; then
  echo "artifact verifier accepted a tampered payload" >&2
  exit 1
fi

echo "ghs-l2 contract tests: PASS (2 positive requests, 9 negative requests, 1 structural package, 1 tampered package, default non-admission)"
