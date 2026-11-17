#!/bin/bash
set -euo pipefail
exec > >(tee -a /var/log/n83-runner-bootstrap.log) 2>&1
systemd-run --unit=n83-runner-hard-stop --on-active=7h /sbin/shutdown -h now
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y ca-certificates curl jq libicu74 libssl3 python3 rustc cargo git tar gzip zstd
install -d -o ubuntu -g ubuntu /opt/n83-actions-runner
cd /opt/n83-actions-runner
curl -fsSL --retry 5 --retry-delay 5 -o actions-runner.tar.gz https://github.com/actions/runner/releases/download/v2.338.1/actions-runner-linux-x64-2.338.1.tar.gz
echo '08e9ccb3604d2462c24b5e33604dc30810296ca15cc65f23cffd4317eb144c37  actions-runner.tar.gz' | sha256sum -c -
tar -xzf actions-runner.tar.gz
rm actions-runner.tar.gz
chown -R ubuntu:ubuntu /opt/n83-actions-runner
printf 'READY\n' > /opt/n83-actions-runner/READY
