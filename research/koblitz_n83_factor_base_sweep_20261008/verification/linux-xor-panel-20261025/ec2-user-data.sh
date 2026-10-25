#!/bin/bash
set -eu

# Bound the bill and delete the root volume even if the supervisor disconnects.
# The EC2 launch sets instance-initiated shutdown behavior to terminate.
shutdown -h +90

# Canonical's current AWS image normally has the SSM snap. Install it only if
# the image lacks the service; no inbound SSH is configured for this worker.
if ! systemctl is-active --quiet snap.amazon-ssm-agent.amazon-ssm-agent.service; then
    snap install amazon-ssm-agent --classic
fi
