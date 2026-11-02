#!/usr/bin/env bash
# E34 independent replay (validator).  Runs a fresh build of bd0118f39's
# e34_cheap_rep on the curves in the nonce-drawn order (nonce.txt, order.txt),
# then the identity stage (all 8 curves) and the negative-control walks
# (p = 241), exactly as the producer's commands in examples/e34_cheap_rep.rs:
# serial, one core, RAYON_NUM_THREADS=1.  Outputs land next to this script.
#   BIN=/path/to/e34_cheap_rep research/e34_twist24_cheap_rep_20261031/replay/replay.sh
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
: "${BIN:?set BIN to the fresh e34_cheap_rep binary}"
CORE=${CORE:-2}
order=$(cat "$here/order.txt")
log() { echo "$(date -u +%FT%TZ) $*" >> "$here/replay.log"; }
log "binary $(sha256sum "$BIN" | cut -d' ' -f1) order $order core $CORE"
for p in ${order//,/ }; do
  log "stage r p=$p start"
  RAYON_NUM_THREADS=1 taskset -c "$CORE" "$BIN" --stage r --sizes "$p" --json "$here/stage_r_p$p.json" > "$here/stage_r_p$p.log"
  log "stage r p=$p done"
done
log "stage i start"
RAYON_NUM_THREADS=1 taskset -c "$CORE" "$BIN" --stage i --json "$here/identity.json" > "$here/identity.log"
log "stage i done; stage n start"
RAYON_NUM_THREADS=1 taskset -c "$CORE" "$BIN" --stage n --sizes 241 --json "$here/negative_walks.json" > "$here/negative_walks.log"
log "stage n done"
