#!/bin/bash
# usage: run_lt.sh <t> <G> <seed> <targets>
B=/tmp/claude-0/-home-user/10dd7624-1fe0-5cbe-89bf-ab69288daab5/scratchpad/target_rel/release/curve25519_family
ulimit -v 3500000
tag="t$1_g$2_s$3"
timeout 14400 $B f4cost 20 "isogeny@$3" $1 $2 $4 2026111403 8000 2 full 0 30000000 0 0 > "$tag.json" 2> "$tag.stderr"
echo $? > "$tag.status"
