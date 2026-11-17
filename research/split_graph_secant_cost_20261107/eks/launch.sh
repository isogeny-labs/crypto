#!/usr/bin/env bash
# NOT RUN by the authoring session. Submits the frozen panel as ONE command Job.
# Preconditions (AGENTS.md / CLAUDE.md): Conductor task, .resume/RESUME.md pushed
# (scripts/checkpoint.sh init), bench.record/v1 opened (tools/benchrec.py init) if the hooks
# classify this as a benchmark, source committed and pushed, aws sts get-caller-identity = 104186811093.
set -euo pipefail
req=${1:?"filled-in request JSON (from eks/request.template.json)"}
here=$(cd "$(dirname "$0")" && pwd)
export AWS_REGION=us-east-2 ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 \
       ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher ISOLAB_CLIENT_OUTPUT=${ISOLAB_CLIENT_OUTPUT:?"unique absolute receipts dir"}
repo=$(git rev-parse --show-toplevel)
"$repo/taskq/deploy/eks-karpenter/remote/launch-remote-command" "$req"
# afterwards: status with an isolab.eks.command-status/v1 request through the same launcher, then
#   aws s3 cp --recursive s3://$ISOLAB_BUCKET/runs/<job>/<pod-uid>/ out/ && (cd out && sha256sum -c SHA256SUMS)
#   python3 -I "$here/../sgsecant_recheck.py" out/panel
