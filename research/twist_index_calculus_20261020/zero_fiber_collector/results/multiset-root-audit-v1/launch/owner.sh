#!/usr/bin/env bash
set -Eeuo pipefail
cd /home/ec2-user/crypto-twist-zero-fiber
exec bash research/twist_index_calculus_20261020/zero_fiber_collector/multiset_root_audit/run-panel.sh research/twist_index_calculus_20261020/zero_fiber_collector/runs/multiset-root-panel-v1 research/twist_index_calculus_20261020/zero_fiber_collector/runs/multiset-native-gate-v1 > research/twist_index_calculus_20261020/zero_fiber_collector/results/multiset-root-audit-v1/launch/owner.stdout 2> research/twist_index_calculus_20261020/zero_fiber_collector/results/multiset-root-audit-v1/launch/owner.stderr
