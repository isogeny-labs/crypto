#!/usr/bin/env bash
# Submit one registered attempt. No retry and no mutation of frozen source.
set -euo pipefail
HERE=$(cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(git -C "$HERE" rev-parse --show-toplevel)
selection=${1:?usage: launch.sh a|c|a-r6a|a-r6i|b-spot}
previous=
case "$selection" in
  a) part=a; instance_type=r7i.8xlarge; ttl=48h ;;
  c) part=c; instance_type=r7i.4xlarge; ttl=48h ;;
  a-r6a) part=a; instance_type=r6a.8xlarge; ttl=48h; previous=a ;;
  a-r6i) part=a; instance_type=r6i.8xlarge; ttl=48h; previous=a-r6a ;;
  b-spot) part=b; instance_type=m7i.4xlarge; ttl=24h ;;
  *) echo 'Unknown registered selection.' >&2; exit 2 ;;
esac
cd "$ROOT"
protocol=research/fall_degree_bounds_20261008/PREREGISTRATION.md
launcher=taskq/deploy/ephemeral/ephemeral
runner=taskq/deploy/ephemeral/runner.sh
relative_here=research/fall_degree_bounds_20261008/e12/continuation-20261028
git diff --quiet HEAD -- "$protocol" "$launcher" "$runner" "$relative_here/launch.sh" || {
  echo 'Refusing uncommitted changes to launch-bound files.' >&2; exit 2;
}
git cat-file -e "HEAD:$relative_here/launch.sh"
protocol_sha=$(sha256sum "$protocol" | cut -d' ' -f1)
protocol_commit=$(git rev-parse HEAD)
extra_env=()
extra_inputs=()
if [ "$part" = b ]; then
  reference=research/fall_degree_bounds_20261008/e12/run-b2/out/E_3-24-7.jsonl
  checker="$relative_here/check-e12b-completion.sh"
  git diff --quiet HEAD -- "$reference" "$checker" || {
    echo 'Refusing uncommitted reference/checker changes.' >&2; exit 2;
  }
  git cat-file -e "HEAD:$checker"
  reference_sha=$(sha256sum "$reference" | cut -d' ' -f1)
  [ "$reference_sha" = 589c5045361c9a9138baecae629ea91512c35a87efbc022f31780d2d3a91574a ]
  extra_env=(--env "E12B_REFERENCE_B64=$(base64 -w0 "$reference")"
    --env "E12B_REFERENCE_SHA256=$reference_sha"
    --env "E12B_CHECKER_B64=$(base64 -w0 "$checker")"
    --env "E12B_CHECKER_SHA256=$(sha256sum "$checker" | cut -d' ' -f1)")
  extra_inputs=("$reference" "$checker")
fi
if [ -n "$previous" ]; then
  previous_job=$(jq -er '.job' "$HERE/receipts/$previous/status.json")
  previous_status=$(aws s3 cp --region us-east-1 --only-show-errors \
    "s3://ephemeral-runs-104186811093-us-east-1/jobs/$previous_job/status.json" -)
  jq -e --arg job "$previous_job" \
    '.job == $job and .phase == "error" and (.reason | contains("no Spot capacity"))' \
    <<<"$previous_status" >/dev/null
  prior_instances=$(aws ec2 describe-instances --region us-east-1 \
    --filters "Name=tag:ephemeral:job,Values=$previous_job" \
    --query 'Reservations[].Instances[].InstanceId' --output json)
  jq -e 'length == 0' <<<"$prior_instances" >/dev/null || {
    echo 'A prior instance exists; fallback is forbidden.' >&2; exit 2;
  }
fi
mkdir -p "$HERE/receipts"
receipt="$HERE/receipts/$selection"
mkdir "$receipt" || { echo 'Receipt already exists; reconcile it instead of retrying.' >&2; exit 2; }
jq -n --arg part "$part" --arg type "$instance_type" --arg ttl "$ttl" \
  --arg commit "$protocol_commit" --arg protocol_sha "$protocol_sha" \
  '{part:$part,instance_type:$type,ttl:$ttl,market:"spot",protocol_commit:$commit,
    protocol_sha256:$protocol_sha,created_utc:(now|todate),new_attempts_allowed:1,
    source_commit:"371035216942660c4615dfbb88d85a14afaf5c16",
    source_sha256:"4ae808ac57920d93f599936c34513167befc93d865495defcae29fb16a18fb8f"}' \
  >"$receipt/request.json"
sha256sum "$protocol" "$launcher" "$runner" "$relative_here/launch.sh" "${extra_inputs[@]}" >"$receipt/inputs.sha256"
command=$(cat <<'REMOTE'
set -euo pipefail
mkdir -p /work/fall-degree-source
cd /work/fall-degree-source
aws s3 cp --region us-east-1 s3://ephemeral-runs-104186811093-us-east-1/jobs/inputs/fd-src-371035216942.tgz source.tgz --only-show-errors
echo '4ae808ac57920d93f599936c34513167befc93d865495defcae29fb16a18fb8f  source.tgz' | sha256sum -c -
tar xzf source.tgz
rm source.tgz
printf '%s\n' 371035216942660c4615dfbb88d85a14afaf5c16 >"$EPHEMERAL_OUT/source_commit"
printf '%s\n' "$E12_PROTOCOL_SHA256" >"$EPHEMERAL_OUT/protocol_sha256"
printf '%s\n' "$E12_PROTOCOL_COMMIT" >"$EPHEMERAL_OUT/protocol_commit"
case "$E12_PART" in
  a) KEEP=2 CMAX=5 bash research/fall_degree_bounds_20261008/e12/run_par.sh "$EPHEMERAL_OUT" 3:21:7 ;;
  c) bash research/fall_degree_bounds_20261008/e12/run_window.sh "$EPHEMERAL_OUT" 4:28:7 ;;
  b)
    printf '%s' "$E12B_REFERENCE_B64" | base64 -d >"$EPHEMERAL_OUT/gate4_reference.jsonl"
    printf '%s' "$E12B_CHECKER_B64" | base64 -d >"$EPHEMERAL_OUT/check-e12b-completion.sh"
    printf '%s  %s\n' "$E12B_REFERENCE_SHA256" "$EPHEMERAL_OUT/gate4_reference.jsonl" | sha256sum -c -
    printf '%s  %s\n' "$E12B_CHECKER_SHA256" "$EPHEMERAL_OUT/check-e12b-completion.sh" | sha256sum -c -
    KEEP=4 CMAX=4 bash research/fall_degree_bounds_20261008/e12/run_par.sh "$EPHEMERAL_OUT" 3:24:7
    bash "$EPHEMERAL_OUT/check-e12b-completion.sh" "$EPHEMERAL_OUT" "$EPHEMERAL_OUT/gate4_reference.jsonl"
    ;;
  *) exit 2 ;;
esac
REMOTE
)
set +e
EPHEMERAL_REGION=us-east-1 bash "$launcher" run --detach --no-source --spot \
  --type "$instance_type" --toolchain rust --ttl "$ttl" \
  --name "fd-e12${selection}-20261028-r3" \
  --env "E12_PART=$part" --env "E12_PROTOCOL_SHA256=$protocol_sha" \
  --env "E12_PROTOCOL_COMMIT=$protocol_commit" "${extra_env[@]}" \
  -- "$command" >"$receipt/launch.stdout" 2>"$receipt/launch.stderr"
rc=$?
set -e
jq -n --argjson rc "$rc" '{launcher_exit_code:$rc,recorded_utc:(now|todate),scientific_result:null}' \
  >"$receipt/submission.json"
cat "$receipt/launch.stdout"
cat "$receipt/launch.stderr" >&2
exit "$rc"
