#!/usr/bin/env bash
# Offline admission of one proposed native L2 command request. No AWS calls,
# image build or Kubernetes submission. A live submitter must also verify the
# checkpoint object and image digest with their respective services.
set -euo pipefail
[[ $# == 5 ]] || {
  echo 'usage: check-l2-request.sh SOURCE_DIR DRIVER_SHA CHECKPOINT.json IMAGE.json REQUEST.json' >&2
  exit 2
}
source_dir=$(realpath -- "$1")
driver_sha=$2
checkpoint=$3
image=$4
request=$5
here=research/unified_point_index_20261127/panel-bridge-20261127
for file in "$checkpoint" "$image" "$request"; do
  [[ -f $file && ! -L $file && $(wc -c < "$file") -le 65536 ]] || {
    echo 'missing, symlinked or oversized admission file' >&2; exit 2;
  }
done
[[ $(git -C "$source_dir" rev-parse HEAD) == "$driver_sha" ]] || {
  echo 'driver checkout mismatch' >&2; exit 2;
}
git -C "$source_dir" diff --quiet HEAD || { echo 'dirty driver checkout' >&2; exit 2; }
[[ -z $(git -C "$source_dir" ls-files --others --exclude-standard) ]] || {
  echo 'untracked driver source' >&2; exit 2;
}
"$source_dir/$here/check-frozen-inputs.sh" "$source_dir" "$driver_sha" >/dev/null
inputs="$source_dir/$here/publish-inputs.json"
spec="$source_dir/$(jq -er .spec_path "$inputs")"
protocol="$source_dir/$(jq -er .protocol_path "$inputs")"
runner_sha=$(sha256sum "$source_dir/$here/run-pair.sh" | cut -d' ' -f1)
jq -e --arg driver "$driver_sha" --slurpfile pub "$inputs" \
  --slurpfile spec "$spec" --slurpfile protocol "$protocol" '
  .schema=="point-index.source-checkpoint/v1"
  and .driver_revision==$driver
  and .baseline_revision==$pub[0].baseline_revision
  and .candidate_revision==$pub[0].candidate_revision
  and .spec_sha256==$pub[0].spec_sha256
  and .protocol_sha256==$pub[0].protocol_sha256
  and .inputs_sha256==$spec[0].inputs_sha256
  and .seed_manifest_sha256==$spec[0].seed_manifest_sha256
  and .upload_readback_verified==true and .experiment_submitted==false
  and (.archive_sha256|test("^[a-f0-9]{64}$"))
  and (.checkpoint_uri|test("^s3://setec-astronomy-artifacts/checkpoints/isolab-point-index/[0-9]+-[0-9]+/crypto/(main|detached-[a-f0-9]{7,40})/[0-9]{8}T[0-9]{6}Z-"+$driver[0:12]+"/$"))
  and $protocol[0].schema=="experiment.protocol/v1"
' "$checkpoint" >/dev/null || { echo 'checkpoint receipt does not bind frozen source' >&2; exit 2; }
jq -e --arg driver "$driver_sha" --arg runner "$runner_sha" \
  --slurpfile pub "$inputs" --slurpfile spec "$spec" '
  .schema=="point-index.image/v3"
  and .driver_revision==$driver
  and .baseline_revision==$pub[0].baseline_revision
  and .candidate_revision==$pub[0].candidate_revision
  and .optimized_options_sha256==$spec[0].optimized_options_sha256
  and .measurement_overlay_sha256==$spec[0].measurement_overlay_sha256
  and .inputs_sha256==$spec[0].inputs_sha256
  and .seed_manifest_sha256==$spec[0].seed_manifest_sha256
  and .runner_sha256==$runner
  and .experiment_submitted==false
  and all(.baseline_sha256,.candidate_sha256,.panel_sha256,.build_info_sha256;
    type=="string" and test("^[a-f0-9]{64}$"))
  and (.image_uri|test("^104186811093[.]dkr[.]ecr[.]us-east-2[.]amazonaws[.]com/isolab/tasks@sha256:[a-f0-9]{64}$"))
' "$image" >/dev/null || { echo 'image receipt does not bind the frozen pair' >&2; exit 2; }
jq -e --arg driver "$driver_sha" --arg image_uri "$(jq -er .image_uri "$image")" \
  --arg protocol_sha "$(sha256sum "$protocol" | cut -d' ' -f1)" '
  type=="object" and (keys==["job","provenance","schema"])
  and .schema=="isolab.eks.command-request/v1"
  and (.job|type)=="object"
  and (.job|keys==["command","cpu","image","instance_type","isolation_profile","job_name","memory","retries","seconds","wait_seconds"])
  and .job.image==$image_uri
  and .job.command==["/opt/isolab/point-index/run-pair.sh","--run",(.job.job_name|capture("-s(?<slot>[012])-a0$").slot)]
  and (.job.job_name|test("^point-index-[a-f0-9]{8}-s[012]-a0$"))
  and (.job.job_name|startswith("point-index-"+$driver[0:8]+"-"))
  and .job.cpu==4 and .job.memory=="8Gi" and .job.seconds==2100
  and .job.isolation_profile=="l2" and .job.instance_type=="c8a.2xlarge"
  and .job.retries==0 and .job.wait_seconds==0
  and (.provenance|type)=="object"
  and (.provenance|keys==["harness","protocol_sha256","source_revision","study"])
  and .provenance.harness=="point-index-native-pair/v1"
  and .provenance.study=="crypto-point-index-fresh-panel-20261127"
  and .provenance.source_revision==$driver
  and .provenance.protocol_sha256==$protocol_sha
' "$request" >/dev/null || { echo 'L2 request does not match the frozen pair' >&2; exit 2; }
echo 'Frozen native-pair L2 request shape verified offline; no job submitted or live admission inferred.'
