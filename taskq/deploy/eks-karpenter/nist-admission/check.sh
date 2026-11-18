#!/usr/bin/env bash
# Administrative read-only preflight. Never creates a build, job, image or upload.
set -euo pipefail
[[ $# == 1 && $1 == /* && ! -e $1 ]] || { echo 'Fresh absolute report path required' >&2; exit 2; }
umask 077
nist_report=$1
mkdir -p -- "$(dirname -- "$nist_report")"
nist_private=$(mktemp -d)
trap 'rm -rf -- "$nist_private"' EXIT
nist_aws=${NIST_ADMISSION_AWS_BIN:-aws}
nist_account=104186811093
nist_source=${GITHUB_SHA:-local-development}
jq -n --arg source "$nist_source" '{schema:"nist.admission-observation/v1",source:$source,account_matches:false,budgets:[],reservations:"unobserved",capacity:"unobserved",launch_admitted:false,scientific_job_started:false}' > "$nist_report"
if ! "$nist_aws" sts get-caller-identity --region us-east-2 --query Account --output text > "$nist_private/account" 2> "$nist_private/error"; then
    echo 'Existing AWS identity unavailable; no work submitted.' >&2
    exit 1
fi
[[ $(cat "$nist_private/account") == "$nist_account" ]] || { echo 'Unexpected account; no budget read or job submission.' >&2; exit 1; }
jq '.account_matches=true' "$nist_report" > "$nist_private/next"
mv -- "$nist_private/next" "$nist_report"
nist_failed=0
for nist_budget in isolab-daily-cost-100 spot-experiments-daily-25 isolab-monthly-account-cost; do
    nist_readable=false
    nist_positive=false
    nist_reason=unavailable
    if "$nist_aws" budgets describe-budget --region us-east-2 --account-id "$nist_account" --budget-name "$nist_budget" \
        > "$nist_private/budget" 2> "$nist_private/error"; then
        # Delayed charged spend is not committed/reserved spend. Do not export amounts.
        if jq -e '.Budget | (.CostFilters // {} | length)==0 and .BudgetLimit.Unit=="USD" and .CalculatedSpend.ActualSpend.Unit=="USD" and (.BudgetLimit.Amount|tonumber)>=0 and (.CalculatedSpend.ActualSpend.Amount|tonumber)>=0' "$nist_private/budget" >/dev/null 2>&1; then
            nist_readable=true
            nist_reason=observed_charged_only
            if jq -e '(.Budget.BudgetLimit.Amount|tonumber)>(.Budget.CalculatedSpend.ActualSpend.Amount|tonumber)' "$nist_private/budget" >/dev/null; then nist_positive=true; fi
        fi
    fi
    [[ $nist_readable == true ]] || nist_failed=1
    jq --arg name "$nist_budget" --argjson readable "$nist_readable" --argjson positive "$nist_positive" --arg reason "$nist_reason" \
        '.budgets += [{name:$name,readable:$readable,charged_remainder_positive:$positive,scope:$reason}]' "$nist_report" > "$nist_private/next"
    mv -- "$nist_private/next" "$nist_report"
done
echo 'Read-only observation complete. Reserved costs, capacity, checkpoint and native recovery admission still required.'
exit "$nist_failed"
