#!/usr/bin/env bash
# Run only in an existing authorized operator session with the isolab context.
# This collector performs no login, role assumption, privilege or cluster change.
set -euo pipefail
out=${1:?usage: operator-readback.sh NEW_OUTPUT_DIRECTORY}
mkdir "$out"
date -u +%FT%TZ > "$out/observed-at.txt"
aws sts get-caller-identity > "$out/caller.json"
test "$(jq -r .Account "$out/caller.json")" = 104186811093
expected=$(aws eks describe-cluster --name isolab --region us-east-2 --query cluster.endpoint --output text)
actual=$(kubectl config view --minify -o jsonpath='{.clusters[0].cluster.server}')
test "$actual" = "$expected"
kubectl --request-timeout=20s get nodepools -o json > "$out/nodepools.json"
kubectl --request-timeout=20s -n isolab get resourcequota jobs -o json > "$out/quota.json"
kubectl --request-timeout=20s get nodes -o json > "$out/nodes.json"
for budget in isolab-daily-cost-100 spot-experiments-daily-25 isolab-monthly-account-cost; do
    aws budgets describe-budget --account-id 104186811093 --budget-name "$budget" --region us-east-2 \
        --query 'Budget.{Name:BudgetName,Limit:BudgetLimit,Spend:CalculatedSpend,Updated:LastUpdatedTime,TimeUnit:TimeUnit}' \
        --output json > "$out/$budget.json"
done
(cd "$out" && sha256sum observed-at.txt caller.json nodepools.json quota.json nodes.json isolab-daily-cost-100.json spot-experiments-daily-25.json isolab-monthly-account-cost.json > manifest.sha256)
