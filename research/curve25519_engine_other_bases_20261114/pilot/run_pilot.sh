#!/bin/bash
# usage: run_pilot.sh <tag> <form> <t> <lifts>
B=/tmp/claude-0/-home-user/10dd7624-1fe0-5cbe-89bf-ab69288daab5/scratchpad/target_rel/release/curve25519_family
ulimit -v 3500000
timeout 3000 $B f4cost 20 "$2" "$3" 3 100 2026111404 2800 2 full "$4" 30000000 0 0 > "$1.json" 2> "$1.stderr"
echo $? > "$1.status"
