#!/usr/bin/env bash
# Stage 1: submit ONE frozen job to Iso Lab on EKS Spot (AGENTS.md §9a) and wait for its
# receipt. The executor runs exactly this, one job at a time, in jobs.tsv order
# (pre-selection jobs), then the hold jobs of 05-hold.sh. No automatic retry.
#   run/03-launch.sh <job-name>
# Needs: the executor's AWS identity (account 104186811093), $D/inputs/image.json
# (the publish_only receipt: image_uri, source_revision, ecbench_sha256), and a pushed commit.
source "$(dirname "$0")/common.sh"
job=$1
spec=$(find $D/inputs/specs -name "$job.json" | head -1)
[[ -n $spec ]] || { echo "no spec for $job" >&2; exit 2; }
image=$D/inputs/image.json
export AWS_REGION=us-east-2
export ISOLAB_BUCKET=isolab-results-104186811093-us-east-2
export ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher
receipts=$D/receipts/$job
[[ ! -e $receipts ]] || { echo "$receipts exists: reconcile that submission before any other" >&2; exit 3; }
mkdir -p "$receipts"
export ISOLAB_CLIENT_OUTPUT=$(pwd)/$receipts
jq -n --arg job "$job" --slurpfile spec "$spec" --slurpfile build "$image" \
  '{schema:"isolab.eks.request/v1",
    job:{job_name:$job, instance_type:"c8a.2xlarge", seconds:3600, memory:"4Gi", spec:$spec[0],
         build:{image_uri:$build[0].image_uri, source_revision:$build[0].source_revision,
                ecbench_sha256:$build[0].ecbench_sha256}},
    provenance:{harness:"ic-representative-selection-20261105", source_revision:$build[0].source_revision}}' \
  > "$receipts.request.json"
taskq/deploy/eks-karpenter/remote/validate-request "$receipts.request.json" > /dev/null
taskq/deploy/eks-karpenter/remote/launch-remote "$receipts.request.json" > "$receipts.stdout" 2> "$receipts.stderr"
