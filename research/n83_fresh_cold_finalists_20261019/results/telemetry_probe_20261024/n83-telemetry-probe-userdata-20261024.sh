#!/bin/bash
set -euo pipefail
exec > >(tee -a /var/log/n83-telemetry-probe-bootstrap.log) 2>&1
systemd-run --unit=n83-telemetry-probe-hard-stop --on-active=45min /sbin/shutdown -h now
