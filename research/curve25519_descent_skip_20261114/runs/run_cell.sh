#!/bin/bash
# usage: run_cell.sh <tag> <form> <t> <lifts> <skip>
B=/tmp/claude-0/-home-user/10dd7624-1fe0-5cbe-89bf-ab69288daab5/scratchpad/target_rel/release/curve25519_family
ulimit -v 3500000
timeout 3300 $B f4cost 20 "$2" "$3" 3 100 2026111401 3000 2 full "$4" 30000000 0 "$5" > "$1.json" 2> "$1.stderr"
echo $? > "$1.status"
