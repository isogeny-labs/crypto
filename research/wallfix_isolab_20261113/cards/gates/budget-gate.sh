#!/usr/bin/env bash
# Exit 0 only if every AWS budget of account 104186811093 has actual spend below its limit
# (taskq/deploy/eks-karpenter/COST_CONTROLS.md: an exhausted or unknown remainder starts nothing).
set -euo pipefail
out=$(aws budgets describe-budgets --account-id 104186811093 --output json)
printf '%s\n' "$out" | jq -r '.Budgets[] | "\(.BudgetName) actual=\(.CalculatedSpend.ActualSpend.Amount) limit=\(.BudgetLimit.Amount)"'
printf '%s\n' "$out" | jq -e '(.Budgets|length)>0 and all(.Budgets[]; (.CalculatedSpend.ActualSpend.Amount|tonumber) < (.BudgetLimit.Amount|tonumber))' >/dev/null \
  && echo 'budget-gate: every budget has a positive remainder' \
  || { echo 'budget-gate: a budget is exhausted or unreadable; start nothing' >&2; exit 1; }
