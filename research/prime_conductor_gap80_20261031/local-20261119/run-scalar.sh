#!/bin/bash
set -eu
ulimit -v 12582912
exec timeout --signal=INT --kill-after=10s 300s /workspace/tools/conductor-local-20261119/ecbench-scalar scalar-timing --spec research/prime_conductor_gap80_20261031/local-20261119/scalar-spec.json --out research/prime_conductor_gap80_20261031/local-20261119/results/scalar-l0 --batch 64
