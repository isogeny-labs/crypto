#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
valid='{"schema":"isolab.eks.request/v1","job":{"job_name":"contract-test"}}'
printf '%s\n' "$valid" > "$tmp/request.json"
"$here/validate-request" "$tmp/request.json" | jq -e '.job.instance_type=="c8a.2xlarge" and .job.seconds==900 and .job.memory=="4Gi"' >/dev/null
cases=(
  '.schema="unknown"'
  '.job.job_name="../../escape"'
  '.job.job_name="-option"'
  '.job.seconds=0'
  '.job.seconds=3601'
  '.job.seconds=1.5'
  '.job.seconds=false'
  '.job.seconds=null'
  '.job.memory="0Gi"'
  '.job.instance_type="c8a.2xlarge;true"'
  '.job.image="unapproved:latest"'
  '.job.retries=1'
  '.job.spec="not an object"'
  '.provenance={unknown:"x"}'
  '.provenance={harness:42}'
  '.unknown=true'
)
for change in "${cases[@]}"; do
  printf '%s\n' "$valid" | jq "$change" > "$tmp/request.json"
  if "$here/validate-request" "$tmp/request.json" > /dev/null 2>&1; then
    printf 'Incorrectly accepted: %s\n' "$change" >&2
    exit 1
  fi
done
printf 'Contract defaults and %s rejection cases passed\n' "${#cases[@]}"
image=$(jq -r .image_uri "$here/../image.json")
revision=$(printf 'a%.0s' {1..40})
binary=$(printf 'b%.0s' {1..64})
printf '%s\n' "$valid" | jq --arg image "$image" --arg revision "$revision" --arg binary "$binary" \
  '.job.build={image_uri:$image,source_revision:$revision,ecbench_sha256:$binary}
   | .provenance={source_revision:$revision}' > "$tmp/build-request.json"
"$here/validate-request" "$tmp/build-request.json" >/dev/null
cases=(
  '.job.build=null'
  '.job.build={}'
  '.job.build.image_uri="busybox:latest"'
  '.job.build.image_uri|=sub("104186811093";"123456789012")'
  '.job.build.image_uri|=sub("/isolab/tasks";"/other/tasks")'
  '.job.build.source_revision="main"'
  '.job.build.ecbench_sha256="wrong"'
  '.job.build.image_uri+="\n"'
  '.job.build.source_revision+="\n" | .provenance.source_revision=.job.build.source_revision'
  '.job.build.ecbench_sha256+="\n"'
  '.job.build.image_uri|=sub("@sha256:";"@sha256:extra@sha256:")'
  '.job.build.command="true"'
  'del(.job.build.ecbench_sha256)'
  'del(.provenance)'
  '.provenance.source_revision="different"'
)
for change in "${cases[@]}"; do
  jq "$change" "$tmp/build-request.json" > "$tmp/request.json"
  if "$here/validate-request" "$tmp/request.json" >/dev/null 2>&1; then
    printf 'Incorrectly accepted build: %s\n' "$change" >&2
    exit 1
  fi
done
printf 'Commit-build request and %s rejection cases passed\n' "${#cases[@]}"

# Ordinary commands must never land on the scheduler-isolated benchmark pool.
"$here/../submit-job" --image "$image" --dry-run -- true > "$tmp/standard.json"
jq -e '.spec.template.spec | .nodeSelector["isolab.dev/pool"]=="cpu-spot"
  and (.nodeSelector["isolab.dev/cpu-isolation"]==null)
  and all(.tolerations[]; .key!="isolab.dev/measurement")' "$tmp/standard.json" >/dev/null
"$here/../submit-job" --image "$image" --cpu 4 --isolation-profile l2 \
  --instance-type c8a.2xlarge --dry-run -- true > "$tmp/l2.json"
jq -e '.spec.backoffLimit==0 and (.spec.template.spec |
  .nodeSelector["isolab.dev/pool"]=="cpu-l2-spot"
  and .nodeSelector["isolab.dev/cpu-isolation"]=="v6"
  and .nodeSelector["node.kubernetes.io/instance-type"]=="c8a.2xlarge"
  and any(.tolerations[]; .key=="isolab.dev/measurement" and .value=="l2")
  and .containers[0].resources.requests==.containers[0].resources.limits)' "$tmp/l2.json" >/dev/null

# Readiness must check the new pool/class and reject stale Ready conditions.
mkdir "$tmp/bin"
cat > "$tmp/bin/aws" <<'STUB'
#!/usr/bin/env bash
case "$*" in
  'eks describe-cluster '*) echo ACTIVE ;;
  's3api list-objects-v2 '*) echo '{}' ;;
  *) exit 90 ;;
esac
STUB
cat > "$tmp/bin/kubectl" <<'STUB'
#!/usr/bin/env bash
case "$*" in
  *' auth can-i '*) echo yes ;;
  *' get deployment karpenter '*) echo '{"status":{"readyReplicas":1}}' ;;
  *' get nodepool/'*|*' get ec2nodeclass/'*)
    ready=True; generation=2
    if [[ -n ${UNREADY_RESOURCE:-} && "$*" == *" get $UNREADY_RESOURCE "* ]]; then ready=False; fi
    if [[ -n ${STALE_RESOURCE:-} && "$*" == *" get $STALE_RESOURCE "* ]]; then generation=1; fi
    printf '{"metadata":{"generation":2},"status":{"conditions":[{"type":"Ready","status":"%s","observedGeneration":%s}]}}\n' "$ready" "$generation"
    ;;
  *) exit 91 ;;
esac
STUB
chmod +x "$tmp/bin/"*
ready() {
  PATH="$tmp/bin:$PATH" KUBECONFIG=unused ISOLAB_KUBE_CONTEXT=unused \
    ISOLAB_CLUSTER=isolab ISOLAB_BUCKET=unused AWS_REGION=us-east-2 "$here/check-ready"
}
ready | jq -e '.ready and (.capacity_guaranteed==false)' >/dev/null
if UNREADY_RESOURCE=nodepool/cpu-l2-spot ready >/dev/null 2>&1; then
  echo 'Readiness accepted an unready benchmark pool' >&2; exit 1
fi
if STALE_RESOURCE=ec2nodeclass/cpu-l2 ready >/dev/null 2>&1; then
  echo 'Readiness accepted a stale benchmark class' >&2; exit 1
fi
printf 'Pool routing, resource guarantees and readiness rejection cases passed\n'
