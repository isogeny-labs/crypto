#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
export AWS_DEFAULT_REGION=us-east-2
export AWS_REGION=us-east-2
export AWS_PAGER=''
export KUBECONFIG="${KUBECONFIG:-$PWD/runtime/kubeconfig}"
mkdir -p runtime "$(dirname "$KUBECONFIG")"
for tool in aws eksctl helm kubectl jq rg; do command -v "$tool" >/dev/null; done
[[ $(aws sts get-caller-identity --query Account --output text) == 104186811093 ]] || {
  echo 'Wrong AWS account; expected 104186811093' >&2; exit 1;
}
aws cloudformation deploy --stack-name Karpenter-isolab \
  --template-file vendor/karpenter-1.15.0-cloudformation.yaml \
  --capabilities CAPABILITY_NAMED_IAM --parameter-overrides ClusterName=isolab \
  --tags project=isolab managed-by=cloudformation --no-fail-on-empty-changeset
if ! aws iam get-role --role-name AWSServiceRoleForEC2Spot >/dev/null 2>runtime/spot-role.err; then
  if rg -q NoSuchEntity runtime/spot-role.err; then
    aws iam create-service-linked-role --aws-service-name spot.amazonaws.com >/dev/null
  else cat runtime/spot-role.err >&2; exit 1; fi
fi
if aws eks describe-cluster --name isolab >runtime/cluster.json 2>runtime/cluster.err; then
  [[ $(jq -r .cluster.status runtime/cluster.json) == ACTIVE ]] || {
    echo 'Cluster exists but is not ACTIVE; wait for its current operation' >&2; exit 1;
  }
  # Existing cluster infrastructure is not reconciled by this bootstrap script.
  aws eks update-kubeconfig --name isolab --kubeconfig "$KUBECONFIG"
elif rg -q ResourceNotFoundException runtime/cluster.err; then
  eksctl create cluster --config-file cluster.yaml --kubeconfig "$KUBECONFIG" --timeout 45m
else cat runtime/cluster.err >&2; exit 1; fi
aws cloudformation deploy --stack-name isolab-storage --template-file storage.yaml \
  --capabilities CAPABILITY_NAMED_IAM --tags project=isolab managed-by=cloudformation \
  --no-fail-on-empty-changeset
aws eks describe-cluster --name isolab >runtime/cluster.json
sg=$(jq -r .cluster.resourcesVpcConfig.clusterSecurityGroupId runtime/cluster.json)
aws ec2 create-tags --resources "$sg" --tags Key=karpenter.sh/discovery,Value=isolab
# Only the private worker subnets are tagged for Karpenter discovery.
# shellcheck disable=SC2016 # Backticks are JMESPath literals, not shell substitution.
private_subnets=$(aws cloudformation describe-stacks --stack-name eksctl-isolab-cluster \
  --query 'Stacks[0].Outputs[?OutputKey==`SubnetsPrivate`].OutputValue' --output text)
[[ -n "$private_subnets" && "$private_subnets" != None ]] || { echo 'No private subnets' >&2; exit 1; }
IFS=',' read -r -a subnet_ids <<< "$private_subnets"
aws ec2 create-tags --resources "${subnet_ids[@]}" --tags Key=karpenter.sh/discovery,Value=isolab
kubectl apply -f workloads.yaml
eksctl create iamserviceaccount --cluster isolab --region us-east-2 \
  --namespace isolab --name runner --role-name isolab-job --role-only \
  --attach-policy-arn arn:aws:iam::104186811093:policy/isolab-job-artifacts --approve
helm upgrade --install karpenter oci://public.ecr.aws/karpenter/karpenter \
  --version 1.15.0 --namespace kube-system --values karpenter-values.yaml --wait --timeout 10m
kubectl apply -f spot-only-policy.yaml
kubectl apply -f nodepool.yaml
kubectl wait --for=condition=Ready ec2nodeclass/cpu --timeout=180s
kubectl wait --for=condition=Ready nodepool/cpu-spot --timeout=180s
kubectl -n kube-system rollout status deployment/karpenter --timeout=180s
kubectl get nodes,nodepools
