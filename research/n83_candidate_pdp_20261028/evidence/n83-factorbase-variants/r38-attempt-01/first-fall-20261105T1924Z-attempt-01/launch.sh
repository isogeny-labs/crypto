#!/usr/bin/env bash
set -Eeuo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=/home/ec2-user/crypto-n83-degree-sweep-20261027/research/n83_candidate_pdp_20261028
INPUTS=/home/ec2-user/crypto-n83-degree-sweep-20261027/research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r38-attempt-01/top-form-20261105T1840Z-attempt-09/candidate-inputs
export AWS_RETRY_MODE=standard AWS_MAX_ATTEMPTS=1
ulimit -v 8388608
exec taskset -c 0 timeout --preserve-status --signal=TERM --kill-after=45s 12850s python3 "$HERE/run-first-fall.py" --inputs "$INPUTS" --manifest "$HERE/input-manifest.json" --protocol "$HERE/protocol-pdp-factorbase-v2.json" --fall "$HERE/instruments/fall" --out "$HERE/results" --cpu 0 --run
