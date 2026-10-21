#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"
: "${KUBECONFIG:?Set KUBECONFIG to an authorized isolab kubeconfig}"

context=$(kubectl config current-context)
[[ "$context" == *isolab* ]] || { echo "Refusing non-isolab context: $context" >&2; exit 1; }
# EKS access policies are not reflected in kubectl auth can-i (a Kubernetes
# SelfSubjectAccessReview). The actual API calls below are the authority check.

kubectl apply -f namespaces.yaml
kubectl -n arc-runners get secret arc-github-auth >/dev/null || {
  echo "Create the arc-github-auth GitHub credential secret before installation" >&2
  exit 1
}

kubectl apply -f nodepool.yaml
kubectl wait --for=condition=Ready ec2nodeclass/gha --timeout=5m
kubectl wait --for=condition=Ready nodepool/gha-on-demand --timeout=5m

chart=oci://ghcr.io/actions/actions-runner-controller-charts
version=0.15.0
helm upgrade --install arc "$chart/gha-runner-scale-set-controller" \
  --version "$version" --namespace arc-system --atomic --wait --timeout 10m \
  -f controller-values.yaml
helm upgrade --install crypto-eks-linux "$chart/gha-runner-scale-set" \
  --version "$version" --namespace arc-runners --atomic --wait --timeout 10m \
  -f linux-values.yaml
helm upgrade --install crypto-eks-highmem "$chart/gha-runner-scale-set" \
  --version "$version" --namespace arc-runners --atomic --wait --timeout 10m \
  -f highmem-values.yaml

kubectl -n arc-runners get autoscalingrunnersets,pods
echo "ARC installed. Run the manual ARC smoke workflow before setting CRYPTO_EKS_WORKFLOWS."
