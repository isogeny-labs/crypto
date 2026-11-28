#!/usr/bin/env bash
set -euo pipefail
cd /tmp/ecdlp-port-mass-publish-20261123
python3 tools/pcc/pcc.py verify PCC-20261123-85abe4 --json
