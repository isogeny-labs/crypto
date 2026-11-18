#!/bin/bash
# one grid line -> one process, 3 GB address-space cap, 2 h wall cap
D=/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/r7/lift-height
ulimit -v 3000000
export PYTHONPATH=/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/pylib
exec timeout 7200 python3 $D/code/lh_toy.py "$@"
