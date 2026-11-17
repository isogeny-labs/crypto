#!/usr/bin/env bash
# E7 large-host tier (Amendment 7): m=6 (10,3) N=58 and m=5 (15,3) N=60.
# PREPARED, NOT RUN. Needs AWS_ACCESS_KEY_ID + AWS_SECRET_ACCESS_KEY + a region in the
# environment, an existing key pair, an instance profile that may write to S3_BUCKET.
# Never prints or copies a private key; never creates a key pair.
#
#   AWS_DEFAULT_REGION=... S3_BUCKET=... INSTANCE_PROFILE=... KEY_NAME=meow34 \
#   AMI_ID=<Ubuntu 24.04 x86_64 AMI in that region> \
#   research/fall_degree_bounds_20261008/e7/aws_large_tier.sh
#
# The instance builds the repo at COMMIT, runs the SAME gate as run_local.sh on the built
# binary (the sha256 differs from the local build if the toolchain differs; the gate, not
# the hash, is what must pass), runs the two cells, uploads e7/ outputs to S3 and shuts
# itself down. No SSH is needed.
set -euo pipefail
: "${AWS_DEFAULT_REGION:?}" "${S3_BUCKET:?}" "${INSTANCE_PROFILE:?}" "${KEY_NAME:?}" "${AMI_ID:?}"
: "${AWS_SECRET_ACCESS_KEY:?missing: add it to the environment settings}"
COMMIT=${COMMIT:-$(git rev-parse HEAD)}
TYPE=${INSTANCE_TYPE:-r6i.4xlarge}   # 16 vCPU, 128 GB
RUN=e7-large-$(date -u +%Y%m%dT%H%MZ)
UD=$(mktemp)
cat >"$UD" <<UDEOF
#!/bin/bash
set -x
exec >/var/log/e7.log 2>&1
apt-get update -y && apt-get install -y build-essential git curl jq awscli
curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
. /root/.cargo/env
git clone https://github.com/aburan28/crypto.git /work && cd /work && git checkout $COMMIT
cargo build --release --example hky_par
S=research/fall_degree_bounds_20261008; E=\$S/e7
export HKY_PAR=target/release/examples/hky_par
status=ok
\$E/validate.sh >\$E/validate_large.log 2>&1 || status=gate_failed
tail -1 \$E/validate_large.log | grep -qx 'AGREE 24 DISAGREE 0' || status=gate_failed
if [ \$status = ok ]; then
  for cell in 6:10:3 5:15:3; do
    \$HKY_PAR newcells --cells \$cell --seed 20261012 --keep 4 --cmax 4 >\$E/E7L_\${cell//:/-}.jsonl 2>\$E/E7L_\${cell//:/-}.err || status=cell_failed
  done
fi
echo \$status >\$E/large_status.txt
aws s3 cp --recursive \$E s3://$S3_BUCKET/$RUN/ || true
aws s3 cp /var/log/e7.log s3://$S3_BUCKET/$RUN/e7.log || true
shutdown -h now
UDEOF
aws ec2 run-instances --image-id "$AMI_ID" --instance-type "$TYPE" --key-name "$KEY_NAME" \
  --iam-instance-profile Name="$INSTANCE_PROFILE" --user-data "file://$UD" \
  --instance-initiated-shutdown-behavior terminate \
  --block-device-mappings 'DeviceName=/dev/sda1,Ebs={VolumeSize=60,VolumeType=gp3}' \
  --tag-specifications "ResourceType=instance,Tags=[{Key=Name,Value=$RUN}]" \
  --query 'Instances[0].InstanceId' --output text
echo "outputs will appear at s3://$S3_BUCKET/$RUN/"
