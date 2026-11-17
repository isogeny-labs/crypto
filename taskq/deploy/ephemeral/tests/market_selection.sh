#!/usr/bin/env bash
# Docker-free launch-contract check using the existing fake AWS CLI.
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
runner=$here/../ephemeral
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
export FAKE_AWS=$scratch XDG_CACHE_HOME=$scratch/cache EPHEMERAL_POLL=1
mkdir -p "$scratch/bin" "$scratch/ssm/ephemeral"
ln -s "$here/fake-aws" "$scratch/bin/aws"
cat >"$scratch/bin/docker" <<'EOF'
#!/bin/sh
# The fake AWS launcher records the request before calling docker.
[ "$1" = run ] || exit 1
echo fake-container
EOF
chmod +x "$scratch/bin/docker"
export PATH=$scratch/bin:$PATH
cat >"$scratch/ssm/ephemeral/config" <<'EOF'
{"region":"us-east-1","bucket":"test-runs","launch_template":"lt-0test","security_group":"sg-0test",
 "subnet_ids":["subnet-a"],"repo_url":"https://example.invalid/crypto.git",
 "default_ttl":"4h","max_ttl_hours":48,"managed_tag":"ephemeral:managed",
 "cpu_ami_param":"/cpu","arm_ami_param":"/arm","gpu_ami_param":"/gpu","instance_profile":"ephemeral-runner"}
EOF

spot_id=$($runner run --detach --no-source --toolchain none --ttl 20m --name market-spot -- true)
jq -e '.spot == true' "$scratch/s3/test-runs/jobs/$spot_id/job.json" >/dev/null
grep 'ec2 run-instances' "$scratch/calls.log" | tail -1 | grep -q -- '--instance-market-options MarketType=spot'

on_demand_id=$($runner run --detach --no-source --toolchain none --ttl 20m --on-demand --name market-on-demand -- true)
jq -e '.spot == false' "$scratch/s3/test-runs/jobs/$on_demand_id/job.json" >/dev/null
if grep 'ec2 run-instances' "$scratch/calls.log" | tail -1 | grep -q -- '--instance-market-options'; then
    echo 'On-Demand launch unexpectedly requested instance market options' >&2
    exit 1
fi

before=$(grep -c 'ec2 run-instances' "$scratch/calls.log")
if $runner run --detach --no-source --spot --on-demand -- true >/dev/null 2>&1; then
    echo 'conflicting market flags were accepted' >&2
    exit 1
fi
after=$(grep -c 'ec2 run-instances' "$scratch/calls.log")
[ "$before" -eq "$after" ]
echo 'Spot default, explicit On-Demand, and conflicting flags: pass'
