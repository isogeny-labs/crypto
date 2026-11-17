#!/usr/bin/env bash
# Install a worker-only replica on an existing Ubuntu EC2 host with the
# ecc2k130-ingest instance profile. The designated publisher stays elsewhere.
set -euo pipefail

if [ "$(id -u)" -ne 0 ]; then
    echo 'run as root on the EC2 host' >&2
    exit 2
fi

: "${RHO_REPLICA_ID:?set the stable replica id}"
: "${RHO_DB_HOST:?set the existing RDS host}"
: "${RHO_BUCKET:?set the source bucket}"
: "${RHO_STATUS_BUCKET:?set the public status bucket}"
: "${RHO_DP_SHA256:?set the full dp_ingest.py SHA-256}"
: "${RHO_OPERATOR_SHA256:?set the full cairn_ingest_operator.py SHA-256}"
RHO_REGION=${RHO_REGION:-us-west-2}
RHO_PERIOD_SECONDS=${RHO_PERIOD_SECONDS:-600}
RHO_THREADS=${RHO_THREADS:-2}
RHO_PASS_OBJECTS=${RHO_PASS_OBJECTS:-8}
RHO_DB_SECRET=${RHO_DB_SECRET:-rho/dp-rds}

[[ "$RHO_REPLICA_ID" =~ ^[A-Za-z0-9._-]{1,80}$ ]] || { echo 'invalid replica id' >&2; exit 2; }
[[ "$RHO_DB_HOST" =~ ^[A-Za-z0-9.-]+$ ]] || { echo 'invalid RDS host' >&2; exit 2; }
[[ "$RHO_BUCKET" =~ ^[a-z0-9.-]+$ && "$RHO_STATUS_BUCKET" =~ ^[a-z0-9.-]+$ ]] || {
    echo 'invalid S3 bucket' >&2; exit 2;
}
[[ "$RHO_REGION" =~ ^[a-z0-9-]+$ && "$RHO_DB_SECRET" =~ ^[A-Za-z0-9/_-]+$ ]] || {
    echo 'invalid region or secret name' >&2; exit 2;
}
[[ "$RHO_DP_SHA256" =~ ^[0-9a-f]{64}$ && "$RHO_OPERATOR_SHA256" =~ ^[0-9a-f]{64}$ ]] || {
    echo 'expected full lowercase SHA-256 digests' >&2; exit 2;
}
for value in "$RHO_PERIOD_SECONDS" "$RHO_THREADS" "$RHO_PASS_OBJECTS"; do
    [[ "$value" =~ ^[0-9]+$ ]] || { echo 'period, threads, and pass objects must be integers' >&2; exit 2; }
done
if [ "$RHO_PERIOD_SECONDS" -lt 60 ] || [ "$RHO_THREADS" -lt 1 ] ||
   [ "$RHO_PASS_OBJECTS" -lt 1 ]; then
    echo 'period must be >= 60; threads and pass objects must be positive' >&2
    exit 2
fi

ROOT=/opt/rho-ingest
PYTHON=$ROOT/venv/bin/python3
AWS=$ROOT/venv/bin/aws
for path in "$PYTHON" "$AWS" "$ROOT/dp_ingest.py" "$ROOT/cairn_ingest_operator.py"; do
    [ -f "$path" ] || { echo "missing $path" >&2; exit 1; }
done
if systemctl is-active --quiet rho-ingest.service; then
    echo 'older rho-ingest.service is active on this host; stop it before installing a replica' >&2
    exit 1
fi

# Pin both executable source files before any service is enabled. The same
# manifest is checked again at every restart, including after a host reboot.
printf '%s  %s\n' "$RHO_DP_SHA256" dp_ingest.py \
    "$RHO_OPERATOR_SHA256" cairn_ingest_operator.py > "$ROOT/SOURCE_SHA256"
(cd "$ROOT" && sha256sum -c SOURCE_SHA256)

if ! getent passwd rho-ingest >/dev/null; then
    useradd --system --home-dir /var/lib/rho-ingest --shell /usr/sbin/nologin rho-ingest
fi
install -d -o rho-ingest -g rho-ingest -m 0700 /var/lib/rho-ingest
install -d -o rho-ingest -g rho-ingest -m 0700 /var/lib/rho-ingest/evidence

cat > /usr/local/bin/ecc2k130-ship-evidence <<EOF
#!/usr/bin/env bash
set -euo pipefail
export AWS_DEFAULT_REGION=$RHO_REGION
exec $AWS s3 sync /var/lib/rho-ingest/evidence/ \\
    s3://$RHO_BUCKET/logs/rho-ingest-$RHO_REPLICA_ID/evidence/ \\
    --exclude current-period.json --exclude operator.lock --exclude '*.tmp' \\
    --only-show-errors
EOF
chmod 0755 /usr/local/bin/ecc2k130-ship-evidence

cat > /etc/systemd/system/ecc2k130-ingest-worker.service <<EOF
[Unit]
Description=ECC2K-130 worker-only DP ingester and Cairn service journal
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=rho-ingest
Group=rho-ingest
WorkingDirectory=$ROOT
Environment=AWS_DEFAULT_REGION=$RHO_REGION
Environment=RHO_CAMPAIGN=ecc2k-130
Environment=RHO_DB_HOST=$RHO_DB_HOST
Environment=RHO_DB_SECRET=$RHO_DB_SECRET
Environment=RHO_INGEST_WORKER_ONLY=1
Environment=PYTHONDONTWRITEBYTECODE=1
Environment=HOME=/var/lib/rho-ingest
ExecStartPre=/usr/bin/sha256sum -c $ROOT/SOURCE_SHA256
ExecStart=$PYTHON $ROOT/cairn_ingest_operator.py --replica-id $RHO_REPLICA_ID --evidence-dir /var/lib/rho-ingest/evidence --status-url https://$RHO_STATUS_BUCKET.s3.$RHO_REGION.amazonaws.com/status.json --period-seconds $RHO_PERIOD_SECONDS -- $PYTHON $ROOT/dp_ingest.py --replica-id $RHO_REPLICA_ID --bucket $RHO_BUCKET --worker-only --threads $RHO_THREADS --pass-objects $RHO_PASS_OBJECTS --no-index
ExecStopPost=/usr/local/bin/ecc2k130-ship-evidence
Restart=always
RestartSec=15
KillMode=control-group
TimeoutStopSec=60
NoNewPrivileges=true
ProtectSystem=strict
ReadWritePaths=/var/lib/rho-ingest/evidence

[Install]
WantedBy=multi-user.target
EOF

cat > /etc/systemd/system/ecc2k130-ingest-evidence.service <<EOF
[Unit]
Description=Ship completed ECC2K-130 worker evidence to private S3

[Service]
Type=oneshot
User=rho-ingest
Group=rho-ingest
Environment=AWS_DEFAULT_REGION=$RHO_REGION
Environment=HOME=/var/lib/rho-ingest
ExecStart=/usr/local/bin/ecc2k130-ship-evidence
EOF

cat > /etc/systemd/system/ecc2k130-ingest-evidence.timer <<'EOF'
[Unit]
Description=Ship completed ECC2K-130 worker evidence every two minutes

[Timer]
OnBootSec=120
OnUnitActiveSec=120
Unit=ecc2k130-ingest-evidence.service

[Install]
WantedBy=timers.target
EOF

systemctl daemon-reload
systemctl enable --now ecc2k130-ingest-worker.service
systemctl enable --now ecc2k130-ingest-evidence.timer
echo "worker $RHO_REPLICA_ID installed; only completed journals are shipped"
