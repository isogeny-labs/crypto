#!/usr/bin/env bash
set -euo pipefail
repo=/home/ec2-user/crypto-pencil-signed-graph-20261029
exec sudo -n /usr/bin/python3 -I "$repo/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/runtime-private-supervisor-v2/supervise.py" --launch "$repo/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/runtime-dollar-reservation/assembly-binding.json"
