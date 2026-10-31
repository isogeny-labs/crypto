#!/usr/bin/env bash
# Run on one of the three existing instances, with the uploaded script and hash.
set -euo pipefail
script=${1:?snapshot script path required}
expected_sha=${2:?snapshot script SHA-256 required}
printf '%s  %s\n' "$expected_sha" "$script" | sha256sum -c -
job=$(jq -er .id /work/job.json)
case "$job" in
  20261028-021829-fd-e12a-r6a-20261028-r3-0c1d7f|20261028-021036-fd-e12c-20261028-r3-914d46|20261028-031317-fd-e12b-spot-20261028-r3-24148b) ;;
  *) echo 'Refusing to install on another job' >&2; exit 2 ;;
esac
destination=/usr/local/sbin/ephemeral-e12-snapshot
if [ -f "$destination" ]; then
  printf '%s  %s\n' "$expected_sha" "$destination" | sha256sum -c -
else
  install -m 0755 "$script" "$destination"
fi
cat >/etc/systemd/system/ephemeral-e12-snapshot.service <<EOF
[Unit]
Description=Persist E12 dataset files to S3
After=network-online.target
ConditionPathExists=/work/out

[Service]
Type=oneshot
ExecStart=$destination $job
Nice=19
IOSchedulingClass=idle
TimeoutStartSec=60
EOF
cat >/etc/systemd/system/ephemeral-e12-snapshot.timer <<'EOF'
[Unit]
Description=Periodically persist E12 dataset files to S3

[Timer]
OnActiveSec=60s
OnUnitInactiveSec=60s
AccuracySec=1s
Unit=ephemeral-e12-snapshot.service

[Install]
WantedBy=timers.target
EOF
systemctl daemon-reload
systemctl enable --now ephemeral-e12-snapshot.timer
systemctl start ephemeral-e12-snapshot.service
systemctl is-active ephemeral-e12-snapshot.timer
systemctl show ephemeral-e12-snapshot.service -p Result -p ExecMainStatus
cat /var/lib/ephemeral-e12-snapshots/snapshot.json
