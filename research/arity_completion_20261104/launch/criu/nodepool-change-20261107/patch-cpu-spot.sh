#!/usr/bin/env bash
# Raise NodePool cpu-spot limits to the owner policy (cost-controls.json): cpu 16, memory 128Gi.
set -euo pipefail
SP=/tmp/claude-0/-home-user/381fb568-a391-5eab-9a8f-72e38290c528/scratchpad
export PATH=$SP/awscli2-install/bin:$SP/launcher-zip/bin:$PATH AWS_REGION=us-east-2
R=/home/user/crypto/research/arity_completion_20261104/launch/criu/nodepool-change-20261107
export KUBECONFIG=$SP/kubeconfig-admin
aws eks update-kubeconfig --name isolab --region us-east-2 --alias isolab --kubeconfig $KUBECONFIG >/dev/null
k=(kubectl --request-timeout=30s --context isolab)
"${k[@]}" get nodepool cpu-spot -o json > $R/nodepool-cpu-spot-before.json
rv=$(jq -r .metadata.resourceVersion $R/nodepool-cpu-spot-before.json)
echo "before limits: $(jq -c .spec.limits $R/nodepool-cpu-spot-before.json) resources: $(jq -c .status.resources $R/nodepool-cpu-spot-before.json)"
patch=$(jq -nc --arg rv "$rv" '[{op:"test",path:"/metadata/resourceVersion",value:$rv},{op:"add",path:"/spec/limits",value:{cpu:"16",memory:"128Gi"}}]')
printf '%s\n' "$patch" > $R/patch.json
"${k[@]}" patch nodepool cpu-spot --type=json -p "$patch" --dry-run=server -o json > /dev/null
"${k[@]}" patch nodepool cpu-spot --type=json -p "$patch" -o json > $R/nodepool-cpu-spot-applied.json
"${k[@]}" get nodepool cpu-spot -o json > $R/nodepool-cpu-spot-readback.json
jq -e '.spec.limits.cpu=="16" and .spec.limits.memory=="128Gi"' $R/nodepool-cpu-spot-readback.json >/dev/null
echo "readback limits: $(jq -c .spec.limits $R/nodepool-cpu-spot-readback.json)"
"${k[@]}" get pods -n isolab-recovery-system -o wide > $R/recovery-pods-after.txt 2>&1 || true
rm -f $KUBECONFIG
