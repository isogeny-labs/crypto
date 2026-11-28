#!/bin/bash
set -eu
ulimit -v 12582912
exec timeout --signal=INT --kill-after=30s 1200s /workspace/tools/conductor-local-20261119/ecbench run --spec research/prime_conductor_gap80_20261031/local-20261119/pilot96-spec.json --out research/prime_conductor_gap80_20261031/local-20261119/results/pilot96-l0 --cpus none --wait --quiet
