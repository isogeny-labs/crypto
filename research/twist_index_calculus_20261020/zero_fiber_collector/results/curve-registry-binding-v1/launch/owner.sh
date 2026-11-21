#!/usr/bin/env bash
set -Eeuo pipefail
cd /home/ec2-user/crypto-twist-zero-fiber
exec bash research/twist_index_calculus_20261020/zero_fiber_collector/curve_registry_binding/run-binding.sh research/twist_index_calculus_20261020/zero_fiber_collector/runs/curve-registry-binding-v1 research/twist_index_calculus_20261020/zero_fiber_collector/runs/curve-registry-native-gate-v3 > research/twist_index_calculus_20261020/zero_fiber_collector/results/curve-registry-binding-v1/launch/owner.stdout 2> research/twist_index_calculus_20261020/zero_fiber_collector/results/curve-registry-binding-v1/launch/owner.stderr
