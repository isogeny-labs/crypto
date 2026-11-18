#!/usr/bin/env bash
set -euo pipefail
regions=(ap-south-1 eu-north-1 eu-west-3 eu-west-2 eu-west-1 ap-northeast-3 ap-northeast-2 ap-northeast-1 ca-central-1 sa-east-1 ap-southeast-1 ap-southeast-2 eu-central-1 us-east-1 us-east-2 us-west-1 us-west-2)
printf '# observed_utc=%s\n' "$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
printf 'account\tregion\tinstance_id\tinstance_type\tavailability_zone\n'
for profile in default ecc-challenges-new; do
  account=$(aws sts get-caller-identity --profile "$profile" --query Account --output text)
  for region in "${regions[@]}"; do
    rows=$(aws ec2 describe-instances --profile "$profile" --region "$region" --filters Name=instance-state-name,Values=running --query 'Reservations[].Instances[].[InstanceId,InstanceType,Placement.AvailabilityZone]' --output text)
    if test -z "$rows"; then
      printf '%s\t%s\tNONE\tNONE\tNONE\n' "$account" "$region"
    else
      printf '%s\n' "$rows" | awk -v account="$account" -v region="$region" 'BEGIN{OFS="\t"} {print account,region,$1,$2,$3}'
    fi
  done
done
