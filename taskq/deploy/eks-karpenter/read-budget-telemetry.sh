#!/bin/sh
# Read-only telemetry. This is never a compute-launch admission decision.
set -eu
test "$#" -eq 1 || { echo 'usage: read-budget-telemetry.sh NEW_OUTPUT_DIRECTORY' >&2; exit 2; }
command -v aws > /dev/null
command -v jq > /dev/null
out=$1
mkdir "$out" # Do not replace an earlier receipt.
expected_account=104186811093
export AWS_PAGER='' AWS_MAX_ATTEMPTS=1
started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
: > "$out/requests.jsonl"
summary() {
    jq -s --arg status "$1" --arg started "$started" \
      --arg checked "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
      --arg account "$expected_account" \
      '{schema:"isolab.budget-telemetry/v1",status:$status,expected_account:$account,
        started_at:$started,checked_at:$checked,requests:.,launch_admitted:false,
        aws_compute_started:false,lane_reserved_spend:null,live_pool_capacity:null,budget_scope_audited:false,
        spend_freshness:"unknown: AWS Budgets spend updates are delayed",
        timestamp_scope:"checked_at is retrieval time; Budget.LastUpdatedTime is a budget-configuration timestamp",
        daily_aliases_are_not_additive:true}' \
      "$out/requests.jsonl" > "$out/summary.json"
}
if aws sts get-caller-identity --query Account --output text --no-cli-pager \
    --cli-connect-timeout 10 --cli-read-timeout 20 \
    > "$out/account.txt" 2> "$out/identity.stderr"; then
    actual_account=$(tr -d '[:space:]' < "$out/account.txt")
    if [ "$actual_account" != "$expected_account" ]; then
        summary wrong_account
        exit 1
    fi
else
    summary identity_unavailable
    exit 1
fi
failed=0
for budget in isolab-daily-cost-100 spot-experiments-daily-25 isolab-monthly-account-cost; do
    unit=DAILY
    if [ "$budget" = isolab-monthly-account-cost ]; then unit=MONTHLY; fi
    api_code=0
    status=ok
    aws budgets describe-budget --region us-east-1 --account-id "$expected_account" \
      --budget-name "$budget" --output json --no-cli-pager \
      --cli-connect-timeout 10 --cli-read-timeout 20 \
      --query 'Budget.{BudgetName:BudgetName,BudgetType:BudgetType,TimeUnit:TimeUnit,BudgetLimit:BudgetLimit,CalculatedSpend:CalculatedSpend,CostTypes:CostTypes,TimePeriod:TimePeriod,LastUpdatedTime:LastUpdatedTime}' \
      > "$out/$budget.json" 2> "$out/$budget.stderr" || api_code=$?
    if [ "$api_code" -ne 0 ]; then
        status=api_failed
        failed=1
    elif ! jq -e --arg name "$budget" --arg unit "$unit" \
      '.BudgetName==$name and .BudgetType=="COST" and .TimeUnit==$unit and .BudgetLimit.Unit=="USD"' \
      "$out/$budget.json" > /dev/null 2> "$out/$budget.validation.stderr"; then
        status=unexpected_budget_metadata
        failed=1
    fi
    jq -nc --arg budget "$budget" --arg status "$status" --argjson code "$api_code" \
      '{budget:$budget,status:$status,api_exit_code:$code}' >> "$out/requests.jsonl"
done
if [ "$failed" -ne 0 ]; then summary telemetry_incomplete; exit 1; fi
summary telemetry_collected
