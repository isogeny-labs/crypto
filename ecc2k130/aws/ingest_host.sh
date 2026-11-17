#!/usr/bin/env bash
#
# Launch the host that copies s3://$BUCKET/dp/ into the rho-dp store.
#
# The store is a derived view for the public dashboard -- `dp/` is the corpus
# and merge.py is what searches it -- so this host may be replaced at any time
# and a fresh one simply resumes from `dp_ingest_progress`. That is the whole
# reason a replacement is the deployment mechanism: the host has no key pair
# or SSM agent (it needs no inbound access), so the way to change the code it
# runs is to launch a new one and stop the old.
#
# It needs to reach RDS, whose security group admits only a handful of source
# groups or addresses, so the instance joins the same group as a prior
# successful ingest host or a running walker. Credentials come from the
# `ecc2k130-ingest` instance profile: read
# `dp/`, write `logs/` and the status bucket's status.json, read the rho-dp
# secret, put metrics in one namespace, and nothing else. A replica launch
# requires that profile; no credential is copied into EC2 user-data.
#
# Usage:
#   AWS_PROFILE=ecc2k130 INGEST_ROLE=publisher ./ingest_host.sh up
#   AWS_PROFILE=ecc2k130 INGEST_ROLE=worker ./ingest_host.sh up
#   ./ingest_host.sh preflight       # resolve the existing VPC placement, no writes
#   ./ingest_host.sh status          # which hosts are running, and how far behind
#   ./ingest_host.sh retire <id>     # stop a superseded host
set -euo pipefail

cd "$(dirname "$0")"
REGION=${AWS_DEFAULT_REGION:-us-west-2}
export AWS_DEFAULT_REGION=$REGION
ACCOUNT=$(aws sts get-caller-identity --query Account --output text)
BUCKET=${ECC_BUCKET:-ecc2k130-$ACCOUNT}
STATUS_BUCKET=${ECC_STATUS_BUCKET:-ecc2k130-status-$ACCOUNT}
# Non-burstable on purpose: draining a backlog runs the decoder flat out for
# hours, which is exactly when a t3 runs out of credits and halves its own
# throughput without saying so.
TYPE=${INGEST_TYPE:-c7g.large}
THREADS=${INGEST_THREADS:-6}
NAME=rho-dp-ingest
ROLE=${INGEST_ROLE:-publisher}
PAID_SERVICE=${INGEST_CAIRN_SERVICE:-0}
PERIOD_SECONDS=${INGEST_PERIOD_SECONDS:-600}
case "$ROLE" in
publisher) WORKER_ONLY=0 ;;
worker) WORKER_ONLY=1 ;;
*) echo "INGEST_ROLE must be publisher or worker" >&2; exit 2 ;;
esac
case "$PAID_SERVICE" in
0|1) ;;
*) echo "INGEST_CAIRN_SERVICE must be 0 or 1" >&2; exit 2 ;;
esac

check_paid_service() {
    [ "$PAID_SERVICE" = 1 ] || return 0
    if [ ! -f "${INGEST_OPERATOR_PATH:-}" ]; then
        echo "INGEST_CAIRN_SERVICE=1 requires INGEST_OPERATOR_PATH to Cairn operator.py" >&2
        return 1
    fi
    if ! [[ "$PERIOD_SECONDS" =~ ^[0-9]+$ ]] || [ "$PERIOD_SECONDS" -lt 2 ]; then
        echo "INGEST_PERIOD_SECONDS must be an integer >= 2" >&2
        return 1
    fi
    local versioning
    versioning=$(aws s3api get-bucket-versioning --bucket "$BUCKET" --query Status --output text)
    if [ "$versioning" != Enabled ]; then
        echo "paid-service receipts require source bucket versioning; $BUCKET is $versioning" >&2
        return 1
    fi
}

# A quiet walker fleet has no running walker to copy network placement from.
# The last stopped ingest host is still an authoritative template for the
# security group that RDS admits and for the subnet it used successfully.
placement() {
    local field=$1 value
    value=$(aws ec2 describe-instances \
        --filters "Name=tag:Name,Values=rho-ecc2k-walker" "Name=instance-state-name,Values=running" \
        --query "Reservations[0].Instances[0].$field" --output text)
    if [ -z "$value" ] || [ "$value" = None ]; then
        value=$(aws ec2 describe-instances \
            --filters "Name=tag:Name,Values=$NAME" "Name=instance-state-name,Values=pending,running,stopping,stopped" \
            --query "Reservations[0].Instances[0].$field" --output text)
    fi
    if [ -z "$value" ] || [ "$value" = None ]; then
        echo "cannot resolve $field; set INGEST_SG and INGEST_SUBNET explicitly" >&2
        return 1
    fi
    echo "$value"
}

if [ "${1:-up}" != status ] && [ "${1:-up}" != retire ]; then
    SG=${INGEST_SG:-$(placement 'SecurityGroups[0].GroupId')}
    SUBNET=${INGEST_SUBNET:-$(placement SubnetId)}
fi

# The Mac LaunchAgent can be the live status publisher even when every EC2
# ingest host is stopped. An older copy ignores the new object locks; fail
# before uploading code or launching an instance while it is active.
check_local_ingester() {
    local installed="$HOME/Library/Application Support/ECC2K130/ingest/dp_ingest.py" state
    if ! command -v launchctl >/dev/null 2>&1; then
        return 0
    fi
    state=$(launchctl print "gui/$(id -u)/com.adamburan.ecc2k130-dp-ingest" 2>/dev/null || true)
    if [[ "$state" != *'state = running'* ]]; then
        return 0
    fi
    if [ ! -f "$installed" ] ||
       [ "$(sha256sum "$installed" | cut -d' ' -f1)" != "$(sha256sum dp_ingest.py | cut -d' ' -f1)" ]; then
        echo "active local DP LaunchAgent runs a different ingester copy; stop or update it before launching replicas" >&2
        return 1
    fi
    if [ "$ROLE" = publisher ]; then
        echo "local DP LaunchAgent already publishes status; stop it before launching another publisher" >&2
        return 1
    fi
}

check_ec2_publisher() {
    local existing
    [ "$ROLE" = publisher ] || return 0
    existing=$(aws ec2 describe-instances \
        --filters "Name=tag:Name,Values=$NAME" "Name=tag:IngestRole,Values=publisher" \
                  "Name=instance-state-name,Values=pending,running" \
        --query 'Reservations[].Instances[].InstanceId' --output text)
    if [ -n "$existing" ] && [ "$existing" != None ]; then
        echo "EC2 publisher $existing already exists; launch workers or retire it first" >&2
        return 1
    fi
}

if [ "${1:-up}" = preflight ] || [ "${1:-up}" = up ]; then
    check_local_ingester
    check_ec2_publisher
    check_paid_service
fi

case "${1:-up}" in
preflight)
    VERSIONING=$(aws s3api get-bucket-versioning --bucket "$BUCKET" --query Status --output text)
    echo "account=$ACCOUNT region=$REGION role=$ROLE instance_type=$TYPE security_group=$SG subnet=$SUBNET"
    echo "source=s3://$BUCKET/dp/ status=s3://$STATUS_BUCKET/status.json"
    echo "cairn_service=$PAID_SERVICE period_seconds=$PERIOD_SECONDS"
    echo "source_versioning=$VERSIONING"
    if [ "$VERSIONING" != Enabled ]; then
        echo "source bucket versioning is disabled; later receipt replay requires immutable object keys" >&2
    fi
    # JMESPath uses literal backticks for its string values.
    # shellcheck disable=SC2016
    aws ec2 describe-instances \
        --filters "Name=tag:Name,Values=$NAME" "Name=instance-state-name,Values=pending,running" \
        --query 'Reservations[].Instances[].[InstanceId,State.Name,Tags[?Key==`IngestRole`]|[0].Value]' \
        --output table
    exit 0
    ;;
status)
    # JMESPath uses literal backticks for its string values.
    # shellcheck disable=SC2016
    aws ec2 describe-instances \
        --filters "Name=tag:Name,Values=$NAME" "Name=instance-state-name,Values=pending,running,stopping,stopped" \
        --query 'Reservations[].Instances[].[InstanceId,State.Name,InstanceType,LaunchTime,Tags[?Key==`IngestVersion`]|[0].Value,Tags[?Key==`IngestRole`]|[0].Value]' \
        --output table
    echo "last lines of each host's journal copy:"
    for p in $(aws s3 ls "s3://$BUCKET/logs/" | awk '{print $2}' | grep '^rho-ingest' || true); do
        echo "== $p"
        aws s3 cp "s3://$BUCKET/logs/${p}ingest.log" - --only-show-errors 2>/dev/null | tail -3 || true
    done
    exit 0
    ;;
retire)
    [ -n "${2:-}" ] || { echo "retire needs an instance id" >&2; exit 1; }
    # Stopped, not terminated: the store is idempotent and a stopped host can
    # be started again if the replacement turns out to be worse.
    aws ec2 stop-instances --instance-ids "$2" --query 'StoppingInstances[].[InstanceId,CurrentState.Name]' --output text
    exit 0
    ;;
up) ;;
*) echo "usage: $0 [preflight|up|status|retire <instance-id>]" >&2; exit 1 ;;
esac

PROFILE=${INGEST_PROFILE:-ecc2k130-ingest}
if ! aws iam get-instance-profile --instance-profile-name "$PROFILE" >/dev/null 2>&1; then
    echo "instance profile $PROFILE is unavailable; refusing to put static credentials in user-data" >&2
    exit 1
fi
echo "using instance profile $PROFILE"
if [ "$PAID_SERVICE" = 0 ]; then
    echo "Cairn period evidence is disabled; this host cannot support a service-period claim" >&2
fi

AMI=${INGEST_AMI:-$(aws ec2 describe-images --owners 099720109477 \
    --filters "Name=name,Values=ubuntu/images/hvm-ssd-gp3/ubuntu-noble-24.04-arm64-server-*" \
              "Name=state,Values=available" \
    --query 'sort_by(Images,&CreationDate)[-1].ImageId' --output text)}

# The code the host runs is published by digest and checked after download, so
# what a host runs is always identifiable from its tags alone.
SHA=$(sha256sum dp_ingest.py | cut -c1-16)
aws s3 cp dp_ingest.py "s3://$BUCKET/ingest/$SHA/dp_ingest.py" --only-show-errors
echo "published ingest $SHA"

EXEC_START="/opt/rho-ingest/venv/bin/python3 /opt/rho-ingest/dp_ingest.py --bucket $BUCKET --status-bucket $STATUS_BUCKET --threads $THREADS --metric-namespace ECC2K130/Ingest"
STOP_POST=
EVIDENCE_SHIP=
OPERATOR_SHA=
if [ "$PAID_SERVICE" = 1 ]; then
    OPERATOR_SHA=$(sha256sum "$INGEST_OPERATOR_PATH" | cut -c1-16)
    aws s3 cp "$INGEST_OPERATOR_PATH" "s3://$BUCKET/ingest/$SHA/operator-$OPERATOR_SHA.py" --only-show-errors
    # `operator.py` in the ingester's module directory shadows Python's
    # standard-library operator module, including during venv/pip startup.
    EXEC_START="/opt/rho-ingest/venv/bin/python3 /opt/rho-ingest/cairn_ingest_operator.py --replica-id %H --evidence-dir /var/lib/rho-ingest/evidence --status-url https://$STATUS_BUCKET.s3.$REGION.amazonaws.com/status.json --period-seconds $PERIOD_SECONDS -- /opt/rho-ingest/venv/bin/python3 /opt/rho-ingest/dp_ingest.py --replica-id %H --bucket $BUCKET --status-bucket $STATUS_BUCKET --threads $THREADS --metric-namespace ECC2K130/Ingest"
    STOP_POST=ExecStopPost=/usr/local/bin/rho-ingest-logship
    EVIDENCE_SHIP="aws s3 sync /var/lib/rho-ingest/evidence/ s3://$BUCKET/logs/rho-ingest-\${IID}/evidence/ --exclude current-period.json --exclude operator.lock --exclude '*.tmp' --only-show-errors"
    echo "published Cairn operator $OPERATOR_SHA; period ${PERIOD_SECONDS}s"
fi

USERDATA=$(mktemp)
trap 'rm -f "$USERDATA"' EXIT
cat > "$USERDATA" <<EOF
#!/bin/bash
set -uo pipefail
exec > >(tee -a /var/log/rho-ingest-boot.log) 2>&1
export AWS_DEFAULT_REGION=$REGION
install -d -m 700 /root/.aws
printf '[default]\nregion=$REGION\n' > /root/.aws/config

apt-get update -q
DEBIAN_FRONTEND=noninteractive apt-get install -y -q python3-venv python3-pip unzip curl
curl -fsS "https://awscli.amazonaws.com/awscli-exe-linux-aarch64.zip" -o /tmp/awscli.zip
unzip -q /tmp/awscli.zip -d /tmp && /tmp/aws/install --update
python3 -m venv /opt/rho-ingest/venv
/opt/rho-ingest/venv/bin/pip install -q --upgrade pip
/opt/rho-ingest/venv/bin/pip install -q "psycopg[binary]" boto3

aws s3 cp "s3://$BUCKET/ingest/$SHA/dp_ingest.py" /opt/rho-ingest/dp_ingest.py
got=\$(sha256sum /opt/rho-ingest/dp_ingest.py | cut -c1-16)
[ "\$got" = "$SHA" ] || { echo "ingest digest \$got != $SHA; refusing to run"; exit 1; }

if [ "$PAID_SERVICE" = 1 ]; then
    aws s3 cp "s3://$BUCKET/ingest/$SHA/operator-$OPERATOR_SHA.py" /opt/rho-ingest/cairn_ingest_operator.py
    got=\$(sha256sum /opt/rho-ingest/cairn_ingest_operator.py | cut -c1-16)
    [ "\$got" = "$OPERATOR_SHA" ] || { echo "Cairn operator digest \$got != $OPERATOR_SHA; refusing to run"; exit 1; }
    install -d -m 700 /var/lib/rho-ingest/evidence
fi

cat > /etc/systemd/system/rho-ingest.service <<'UNIT'
[Unit]
Description=ECC2K-130 distinguished-point ingest
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
Environment=AWS_DEFAULT_REGION=$REGION
Environment=RHO_CAMPAIGN=ecc2k-130
Environment=RHO_DB_HOST=$(aws rds describe-db-instances --db-instance-identifier rho-dp --query 'DBInstances[0].Endpoint.Address' --output text)
Environment=RHO_DB_SECRET=rho/dp-rds
Environment=RHO_INGEST_WORKER_ONLY=$WORKER_ONLY
ExecStart=$EXEC_START
$STOP_POST
Restart=always
RestartSec=15

[Install]
WantedBy=multi-user.target
UNIT

cat > /usr/local/bin/rho-ingest-logship <<'SHIP'
#!/bin/bash
set -euo pipefail
token=\$(curl -fsS -m 2 -X PUT http://169.254.169.254/latest/api/token -H 'X-aws-ec2-metadata-token-ttl-seconds: 120')
IID=\$(curl -fsS -m 2 -H "X-aws-ec2-metadata-token: \$token" http://169.254.169.254/latest/meta-data/instance-id)
[[ "\$IID" =~ ^i-[0-9a-f]+$ ]] || { echo 'cannot identify this EC2 instance' >&2; exit 1; }
journalctl -u rho-ingest -n 4000 --no-pager > /tmp/ingest.log 2>/dev/null || true
aws s3 cp /tmp/ingest.log "s3://$BUCKET/logs/rho-ingest-\${IID}/ingest.log" --only-show-errors
$EVIDENCE_SHIP
SHIP
chmod +x /usr/local/bin/rho-ingest-logship
cat > /etc/systemd/system/rho-ingest-logship.service <<'UNIT'
[Unit]
Description=Copy the ingest journal to S3
[Service]
Type=oneshot
Environment=AWS_DEFAULT_REGION=$REGION
ExecStart=/usr/local/bin/rho-ingest-logship
UNIT
cat > /etc/systemd/system/rho-ingest-logship.timer <<'UNIT'
[Unit]
Description=Copy the ingest journal to S3 every two minutes
[Timer]
OnBootSec=120
OnUnitActiveSec=120
[Install]
WantedBy=timers.target
UNIT

systemctl daemon-reload
systemctl enable --now rho-ingest.service
systemctl enable --now rho-ingest-logship.timer
echo "ingest $SHA started \$(date -u)"
EOF

IID=$(aws ec2 run-instances --image-id "$AMI" --instance-type "$TYPE" \
    --subnet-id "$SUBNET" --security-group-ids "$SG" \
    --iam-instance-profile "Name=$PROFILE" \
    --block-device-mappings 'DeviceName=/dev/sda1,Ebs={VolumeSize=30,VolumeType=gp3,DeleteOnTermination=true}' \
    --metadata-options 'HttpTokens=required,HttpEndpoint=enabled' \
    --user-data "file://$USERDATA" \
    --tag-specifications "ResourceType=instance,Tags=[{Key=Name,Value=$NAME},{Key=Project,Value=ecc2k-130},{Key=Role,Value=rho-ingest},{Key=IngestRole,Value=$ROLE},{Key=IngestVersion,Value=$SHA},{Key=CairnService,Value=$PAID_SERVICE},{Key=CostGuardExempt,Value=true}]" \
    --query 'Instances[0].InstanceId' --output text)
echo "launched $IID ($TYPE, $ROLE replica, ingest $SHA)"
echo "watch: aws s3 cp s3://$BUCKET/logs/rho-ingest-$IID/ingest.log - | tail"
echo "when it is drained, retire the old host: $0 retire <old-instance-id>"
