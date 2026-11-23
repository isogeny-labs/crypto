#!/usr/bin/env bash
set -euo pipefail
ic_root=$(git rev-parse --show-toplevel)
exec timeout -k 60 250000 /workspace/ic-frozen-20261026/ic-hky-replay-records run "$ic_root" "$ic_root/research/ic_algebraic_crossover_20261023/protocol-hky-native-resume-20261026.json" "$@"
