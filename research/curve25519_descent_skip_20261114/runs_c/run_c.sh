#!/bin/bash
# usage: run_c.sh <tag> <form> <lifts>
B=/tmp/claude-0/-home-user/10dd7624-1fe0-5cbe-89bf-ab69288daab5/scratchpad/target_rel/release/curve25519_family
ulimit -v 3500000
timeout 3500 $B f4cost 20 "$2" 5 3 2000 2026111402 3400 2 full "$3" 30000000 0 0 > "$1.json" 2> "$1.stderr"
echo $? > "$1.status"
