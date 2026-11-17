#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
: "${KUBECONFIG:?Set KUBECONFIG to the Iso Lab kubeconfig}"
name="smoke-$(date -u +%Y%m%d-%H%M%S)"
mkdir -p runtime
# shellcheck disable=SC2016 # Expand these environment variables inside the pod.
./submit-job --name "$name" --image public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39 \
  --cpu 2 --memory 512Mi --seconds 900 --wait -- /bin/sh -ec '
    mkdir -p "$ISOLAB_OUTPUT_DIR"
    {
      echo "job=$ISOLAB_JOB_ID pod=$POD_UID node=$NODE_NAME"
      aws sts get-caller-identity --query Arn --output text
      cat /proc/self/status | grep Cpus_allowed_list
      printf "isolab-eks-karpenter-smoke\n" | sha256sum
    } > "$ISOLAB_OUTPUT_DIR/result.txt"
    cat "$ISOLAB_OUTPUT_DIR/result.txt"
    aws s3 cp "$ISOLAB_OUTPUT_DIR/result.txt" "${ISOLAB_RESULTS_URI}result.txt"
  '
kubectl -n isolab get job "$name" -o json > "runtime/$name-job.json"
kubectl -n isolab get pods -l "job-name=$name" -o json > "runtime/$name-pods.json"
node=$(jq -r '.items[0].spec.nodeName' "runtime/$name-pods.json")
kubectl get node "$node" -o json > "runtime/$name-node.json"
[[ $(jq -r '.metadata.labels["karpenter.sh/capacity-type"]' "runtime/$name-node.json") == spot ]]
[[ $(jq -r '.items[0].status.qosClass' "runtime/$name-pods.json") == Guaranteed ]]
uid=$(jq -r '.items[0].metadata.uid' "runtime/$name-pods.json")
aws s3 cp "s3://isolab-results-104186811093-us-east-2/runs/$name/$uid/result.txt" "runtime/$name-result.txt"
grep -q 'assumed-role/isolab-job/' "runtime/$name-result.txt"
printf 'Passed: %s on %s; results persisted to S3.\n' "$name" "$node"
