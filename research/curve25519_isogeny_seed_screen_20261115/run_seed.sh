#!/bin/bash
# usage: run_seed.sh <seed> <lifts>
B=/tmp/claude-0/-home-user/10dd7624-1fe0-5cbe-89bf-ab69288daab5/scratchpad/target_rel/release/curve25519_family
ulimit -v 3500000
tag="s$1_L$2"
timeout 9000 $B f4cost 20 "isogeny@$1" 5 3 20000 2026111403 8000 2 full "$2" 30000000 0 0 > "$tag.json" 2> "$tag.stderr"
echo $? > "$tag.status"
