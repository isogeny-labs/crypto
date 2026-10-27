#!/usr/bin/env bash
# Replace the GitHub credential the ARC scale sets use (secret arc-github-auth in
# arc-runners), then restart the ARC controller so it registers again.
#
#   ARC_GITHUB_REPO=isogeny-labs/crypto ./rotate-github-token.sh
#
# Needs: an AWS identity in the cluster's account, a kubeconfig for the isolab
# cluster, and a token (fine-grained, resource owner = the repository's owner,
# Administration read and write on the repository). The token is read without
# echo, tested against GitHub first, and never written anywhere but the secret.
# Cluster-admin is attached to the calling user for the duration of the script
# and removed again on exit, including on failure.
set -euo pipefail

repo=${ARC_GITHUB_REPO:-isogeny-labs/crypto}
cluster=${ARC_CLUSTER:-isolab}
region=${ARC_REGION:-us-east-2}
runners_ns=arc-runners
system_ns=arc-system
policy=arn:aws:eks::aws:cluster-access-policy/AmazonEKSClusterAdminPolicy

context=$(kubectl config current-context)
[[ "$context" == *"$cluster"* ]] || { echo "Refusing: kubectl context is $context, not $cluster" >&2; exit 1; }

principal=${PRINCIPAL_ARN:-$(aws sts get-caller-identity --query Arn --output text)}
if [[ "$principal" == *assumed-role* ]]; then
  echo "The caller is an assumed role ($principal). Set PRINCIPAL_ARN to the IAM role or user ARN." >&2
  exit 1
fi
echo "cluster:   $cluster ($region)"
echo "principal: $principal"
echo "repo:      $repo"

read -rsp "Token for $repo (hidden): " TOKEN
echo
[ -n "$TOKEN" ] || { echo "No token given" >&2; exit 1; }

# What ARC does first: ask for a runner registration token. 201 means the token works.
code=$(curl -s -o /dev/null -w '%{http_code}' -X POST \
  -H "Authorization: Bearer $TOKEN" -H "Accept: application/vnd.github+json" \
  "https://api.github.com/repos/$repo/actions/runners/registration-token")
if [ "$code" != 201 ]; then
  unset TOKEN
  echo "The token cannot create a runner registration token for $repo (HTTP $code)." >&2
  echo "404 means it cannot see the repo or lacks Administration; 401 means it is invalid or expired." >&2
  exit 1
fi
echo "token check: OK (201)"

revoke() {
  aws eks disassociate-access-policy --cluster-name "$cluster" --region "$region" \
    --principal-arn "$principal" --policy-arn "$policy" >/dev/null \
    && echo "cluster-admin removed from $principal" \
    || echo "COULD NOT REMOVE cluster-admin from $principal. Remove it by hand." >&2
}
aws eks associate-access-policy --cluster-name "$cluster" --region "$region" \
  --principal-arn "$principal" --policy-arn "$policy" --access-scope type=cluster >/dev/null
trap 'unset TOKEN; revoke' EXIT

# Access entries take a few seconds to apply.
applied=false
for _ in 1 2 3 4 5 6; do
  if kubectl -n "$runners_ns" create secret generic arc-github-auth \
      --from-literal=github_token="$TOKEN" --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1; then
    applied=true
    break
  fi
  sleep 10
done
$applied || { echo "Could not update arc-github-auth in $runners_ns" >&2; exit 1; }
echo "secret arc-github-auth updated"

kubectl -n "$system_ns" rollout restart deploy/arc-gha-rs-controller
kubectl -n "$system_ns" rollout status deploy/arc-gha-rs-controller --timeout=120s

echo "waiting up to 3 minutes for the scale sets to register..."
for _ in $(seq 1 18); do
  if ! kubectl -n "$system_ns" get autoscalinglisteners 2>/dev/null | grep -q highmem; then sleep 10; continue; fi
  break
done
kubectl -n "$system_ns" get autoscalinglisteners
kubectl -n "$system_ns" get pods
echo "Recent controller errors for the repo (none is good):"
kubectl -n "$system_ns" logs deploy/arc-gha-rs-controller --since=2m | grep -c "registration-token.*404" || true
