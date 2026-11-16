#!/usr/bin/env bash
# Read-only AWS preflight; never submits a build or a Kubernetes Job.
set -euo pipefail
out=${1:?usage: capture-access.sh FRESH_OUTPUT_DIRECTORY}
test ! -e "$out" || { echo 'preserve existing preflight evidence' >&2; exit 2; }
mkdir -p "$out"
date -u +%FT%TZ > "$out/observed-at.txt"
aws --region us-east-2 --output json --cli-connect-timeout 10 --cli-read-timeout 20 sts get-caller-identity \
  > "$out/identity.json" 2> "$out/identity.stderr.txt"
test "$(jq -r .Account "$out/identity.json")" = 104186811093
capture() {
  local name=$1
  shift
  local task_exit=0
  aws --region us-east-2 --output json --cli-connect-timeout 10 --cli-read-timeout 20 "$@" \
    > "$out/$name.json" 2> "$out/$name.stderr.txt" || task_exit=$?
  printf '%s\n' "$task_exit" > "$out/$name.exit"
}
capture daily-budget budgets describe-budget --account-id 104186811093 \
  --budget-name isolab-daily-cost-100 \
  --query 'Budget.{name:BudgetName,limit:BudgetLimit,spend:CalculatedSpend,updated:LastUpdatedTime}'
capture monthly-budget budgets describe-budget --account-id 104186811093 \
  --budget-name isolab-monthly-account-cost \
  --query 'Budget.{name:BudgetName,limit:BudgetLimit,spend:CalculatedSpend,updated:LastUpdatedTime}'
capture second-daily-budget budgets describe-budget --account-id 104186811093 \
  --budget-name spot-experiments-daily-25 \
  --query 'Budget.{name:BudgetName,limit:BudgetLimit,spend:CalculatedSpend,updated:LastUpdatedTime}'
# An intentionally absent build id is a read-only polling-permission probe.
capture launcher-read codebuild batch-get-builds \
  --ids isolab-experiment-launcher:00000000-0000-0000-0000-000000000000 \
  --query '{found:length(builds),missing:buildsNotFound}'
# Return only registry endpoints, never authorization-token values.
capture registry-auth ecr get-authorization-token --query 'authorizationData[].proxyEndpoint'
capture runner-discovery ssm describe-instance-information \
  --query 'InstanceInformationList[].{id:InstanceId,status:PingStatus}'
jq -n --arg observed "$(cat "$out/observed-at.txt")" \
  --argjson daily "$(cat "$out/daily-budget.exit")" \
  --argjson monthly "$(cat "$out/monthly-budget.exit")" \
  --argjson launcher "$(cat "$out/launcher-read.exit")" \
  --argjson second_daily "$(cat "$out/second-daily-budget.exit")" \
  --argjson registry "$(cat "$out/registry-auth.exit")" \
  --argjson discovery "$(cat "$out/runner-discovery.exit")" \
  '{schema:"cubic-benchmark-access-preflight/v1",observed_at:$observed,
    account:"104186811093",region:"us-east-2",
    exit_codes:{daily_budget:$daily,second_daily_budget:$second_daily,
      monthly_budget:$monthly,launcher_read:$launcher,registry_auth:$registry,
      runner_discovery:$discovery},
    budget_balances_readable:($daily==0 and $second_daily==0 and $monthly==0),
    budget_headroom_known:false,
    source:"raw command output files; unsigned producer observations",
    benchmark_launched:false,scientific_executions:0}' > "$out/summary.json"
