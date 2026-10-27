#!/usr/bin/env bash
# Frozen B qualification: require two committed generations; terminate only its confirmed dedicated worker.
set -euo pipefail
ops=/home/ec2-user/ops/eks-isolab/recovery-periodic-20261022/candidate-b
export AWS_REGION=us-east-2 KUBECONFIG=/home/ec2-user/ops/eks-isolab/kubeconfig
kube=(kubectl --kubeconfig "$KUBECONFIG" -n isolab-recovery-system)
run=ecbench-periodic-20261022-b
build=$(jq -er .controller_build "$ops/client/submission.json")
last=''
for ((i=0;i<180;i++)); do
  if "${kube[@]}" get configmap "$run-state" --ignore-not-found -o json > "$ops/b-watch-cm.json" && [[ -s "$ops/b-watch-cm.json" ]]; then
    jq '.data["state.json"]|fromjson' "$ops/b-watch-cm.json" > "$ops/b-watch-state.json"
    phase=$(jq -r .phase "$ops/b-watch-state.json")
    if [[ "$phase" != "$last" ]]; then printf '%s phase=%s\n' "$(date -u +%FT%TZ)" "$phase"; last=$phase; fi
    [[ "$phase" != failed ]] || { jq .failure "$ops/b-watch-state.json"; exit 2; }
    if jq -e '(.generations|length)>=2 and (.phase=="checkpointing" or .phase=="running") and .attempt==0' "$ops/b-watch-state.json" >/dev/null; then break; fi
    [[ "$phase" != benchmark && "$phase" != succeeded ]] || { echo 'Computation finished before fault; no termination authorized by this protocol'; exit 3; }
  fi
  sleep 5
done
jq -e '(.generations|length)>=2 and .attempt==0 and (.phase=="checkpointing" or .phase=="running")' "$ops/b-watch-state.json" >/dev/null
cp "$ops/b-watch-state.json" "$ops/b-before-injection.json"
instance=$(jq -er .current_instance "$ops/b-before-injection.json")
pod_uid=$(jq -er .current_pod_uid "$ops/b-before-injection.json")
"${kube[@]}" get pod "$run-a0" -o json > "$ops/b-original-pod.json"
jq -e --arg uid "$pod_uid" '.metadata.uid==$uid and .metadata.labels["isolab.dev/recovery-run"]=="ecbench-periodic-20261022-b"' "$ops/b-original-pod.json" >/dev/null
node=$(jq -er .spec.nodeName "$ops/b-original-pod.json")
kubectl --kubeconfig "$KUBECONFIG" get node "$node" -o json > "$ops/b-original-node.json"
jq -e --arg instance "$instance" '(.spec.providerID|endswith("/"+$instance)) and .metadata.labels["node.kubernetes.io/instance-type"]=="c8a.2xlarge"' "$ops/b-original-node.json" >/dev/null
kubectl --kubeconfig "$KUBECONFIG" get pods -A --field-selector "spec.nodeName=$node" -o json > "$ops/b-original-node-pods.json"
jq -e --arg uid "$pod_uid" 'all(.items[]; .metadata.uid==$uid or .status.phase=="Succeeded" or .status.phase=="Failed" or (.metadata.namespace=="kube-system" and any(.metadata.ownerReferences[]?;.kind=="DaemonSet")))' "$ops/b-original-node-pods.json" >/dev/null
aws ec2 describe-instances --instance-ids "$instance" > "$ops/b-instance-before.json"
jq -e --arg instance "$instance" '.Reservations[0].Instances[0]|.InstanceId==$instance and .InstanceLifecycle=="spot" and .InstanceType=="c8a.2xlarge" and .State.Name=="running"' "$ops/b-instance-before.json" >/dev/null
printf 'Checkpoint committed. Stopping controller %s before fencing its dedicated worker %s.\n' "$build" "$instance"
aws codebuild stop-build --id "$build" > "$ops/b-controller-stop.json"
for ((i=0;i<30;i++)); do
  aws codebuild batch-get-builds --ids "$build" > "$ops/b-controller-stopped.json"
  [[ $(jq -r '.builds[0].buildStatus' "$ops/b-controller-stopped.json") == STOPPED ]] && break
  sleep 5
done
jq -e '.builds[0].buildStatus=="STOPPED"' "$ops/b-controller-stopped.json" >/dev/null
# Recheck immediately before the one authorized instance termination.
kubectl --kubeconfig "$KUBECONFIG" get pods -A --field-selector "spec.nodeName=$node" -o json > "$ops/b-node-pods-before-terminate.json"
jq -e --arg uid "$pod_uid" 'all(.items[]; .metadata.uid==$uid or .status.phase=="Succeeded" or .status.phase=="Failed" or (.metadata.namespace=="kube-system" and any(.metadata.ownerReferences[]?;.kind=="DaemonSet")))' "$ops/b-node-pods-before-terminate.json" >/dev/null
aws ec2 terminate-instances --instance-ids "$instance" > "$ops/b-termination.json"
aws ec2 wait instance-terminated --instance-ids "$instance"
aws ec2 describe-instances --instance-ids "$instance" > "$ops/b-worker-fence.json"
jq -e --arg instance "$instance" '.Reservations[0].Instances[0]|.InstanceId==$instance and .State.Name=="terminated"' "$ops/b-worker-fence.json" >/dev/null
aws codebuild retry-build --id "$build" --idempotency-token ecbench-periodic-20261022-b-controller-restart > "$ops/b-controller-retry.json"
jq -e --slurpfile p "$ops/protocol.json" '.build.source.location==("isolab-results-104186811093-us-east-2/"+$p[0].controller.key) and .build.sourceVersion==$p[0].controller.version' "$ops/b-controller-retry.json" >/dev/null
printf 'Old worker confirmed terminated. Controller retry: %s\n' "$(jq -r .build.id "$ops/b-controller-retry.json")"
