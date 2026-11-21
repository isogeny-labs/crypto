#!/bin/bash
# usage: run_t6.sh <seed>
B=/tmp/claude-0/-home-user/10dd7624-1fe0-5cbe-89bf-ab69288daab5/scratchpad/target_rel/release/curve25519_family
ulimit -v 3500000
tag="t6_s$1"
timeout 9000 $B f4cost 20 "isogeny@$1" 6 3 20000 2026111403 8000 2 full 0 30000000 0 0 > "$tag.json" 2> "$tag.stderr"
echo $? > "$tag.status"
