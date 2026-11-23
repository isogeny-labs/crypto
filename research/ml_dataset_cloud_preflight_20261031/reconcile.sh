#!/usr/bin/env bash
# Read-only controller-side reconciliation. Does not create a ledger, grant or Job.
set -uo pipefail

if [[ -z "${AWS_CONFIG_FILE:-}" || -z "${AWS_PROFILE:-}" || "${AWS_REGION:-}" != us-east-2 ]]; then
  printf 'Set the approved AWS_CONFIG_FILE, AWS_PROFILE and AWS_REGION=us-east-2.\n' >&2
  exit 2
fi

printf 'UTC: '
date -u +%Y-%m-%dT%H:%M:%SZ
aws sts get-caller-identity --query '{Account:Account,Arn:Arn}' --output json || exit 1
aws iam get-role --role-name isolab-job --query 'Role.{Arn:Arn,MaxSessionDuration:MaxSessionDuration}' --output json || exit 1
aws iam list-role-policies --role-name isolab-job --query PolicyNames --output json || exit 1
aws iam list-attached-role-policies --role-name isolab-job --query 'AttachedPolicies[].{Arn:PolicyArn,Name:PolicyName}' --output json || exit 1

# These are the three historical E12 instance IDs in shared-budget-2751.json.
# Preserve each terminal or live state and its timestamps; a current state alone
# does not establish an invoice total or release an existing obligation.
for historical_instance in i-0f1e979a30bc4f52b i-0ac1c3ad72a141ac0 i-03d408f37b22432d1; do
  # The historical E12 instances were us-east-1. EC2 eventually forgets
  # terminated IDs; NotFound is not evidence of the exact billed duration.
  printf 'Historical us-east-1 instance %s:\n' "$historical_instance"
  if ! aws --region us-east-1 ec2 describe-instances \
    --instance-ids "$historical_instance" \
    --query 'Reservations[].Instances[].{Id:InstanceId,Type:InstanceType,State:State.Name,Launch:LaunchTime,Spot:InstanceLifecycle}' \
    --output json; then
    printf 'Unresolved historical instance query for %s; retain prior obligation.\n' "$historical_instance" >&2
  fi
done

# A spot quote is a snapshot, never the finite billing bound required by policy.
aws ec2 describe-spot-price-history --instance-types c6a.2xlarge \
  --product-descriptions Linux/UNIX --max-results 3 \
  --query 'SpotPriceHistory[].{AZ:AvailabilityZone,Price:SpotPrice,At:Timestamp}' \
  --output json || exit 1

if [[ -z "${DATASET_BINARY:-}" || -z "${POLICY_PATH:-}" || -z "${LEDGER_PATH:-}" ]]; then
  printf 'STOP: supply DATASET_BINARY, POLICY_PATH and original LEDGER_PATH for native status. No admission inferred.\n' >&2
  exit 2
fi
if [[ ! -f "$POLICY_PATH" || ! -f "$LEDGER_PATH" ]]; then
  printf 'STOP: original policy/ledger missing; do not create a replacement authority.\n' >&2
  exit 2
fi
"$DATASET_BINARY" dataset budget status --policy "$POLICY_PATH" --ledger "$LEDGER_PATH"
