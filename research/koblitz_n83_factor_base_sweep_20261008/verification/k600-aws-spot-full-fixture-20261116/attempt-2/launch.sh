#!/usr/bin/env bash
set -euo pipefail

if [[ $# != 1 ]]; then
  echo 'usage: launch.sh NEW_OUTPUT_DIR' >&2
  exit 2
fi
output=$1
[[ ! -e $output ]] || { echo 'output already exists' >&2; exit 3; }
mkdir -p "$output"
output=$(realpath "$output")
script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
user_data="$script_dir/../user-data.sh"
[[ -f $user_data ]] || { echo 'committed user-data.sh absent' >&2; exit 4; }

started_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
printf '%s\n' "$started_utc" > "$output/started.utc"
set +e
aws ec2 run-instances --region us-east-1 --profile ecc2k130 \
  --image-id ami-061bb79b9c35a0b94 --instance-type c7g.2xlarge \
  --subnet-id subnet-07f637a5d1c6e15d2 \
  --security-group-ids sg-09d3d2dc37e3354a7 --key-name meow34 \
  --associate-public-ip-address \
  --block-device-mappings 'DeviceName=/dev/xvda,Ebs={VolumeSize=150,VolumeType=gp3,DeleteOnTermination=true}' \
  --instance-market-options 'MarketType=spot,SpotOptions={SpotInstanceType=one-time,InstanceInterruptionBehavior=terminate,MaxPrice=0.18}' \
  --instance-initiated-shutdown-behavior terminate \
  --user-data "file://$user_data" \
  --tag-specifications 'ResourceType=instance,Tags=[{Key=Project,Value=N83FactorBaseSweep},{Key=Run,Value=FullFixtureValidation20261116Attempt2}]' \
  --client-token n83-full-fixture-20261116-attempt2 --count 1 \
  > "$output/run-instances.json" 2> "$output/run-instances.stderr"
exit_code=$?
set -e
ended_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
printf '%s\n' "$ended_utc" > "$output/ended.utc"
printf '%s\n' "$exit_code" > "$output/exit.code"
if [[ $exit_code != 0 ]]; then
  cat "$output/run-instances.stderr" >&2
  exit "$exit_code"
fi
jq -er '.Instances[0].InstanceId' "$output/run-instances.json"
