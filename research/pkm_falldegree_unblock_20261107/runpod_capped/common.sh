# Shared settings for run.sh and fetch.sh (draft Amendment 17, AMENDMENT_R2_H_runpod_capped.md). Sourced, not run.
# The pod-side runner is Amendment 16's runpod/server.py, unchanged; its client runpod/rp.sh is used unchanged for
# upload, start, status and fetch (those calls talk only to the pod, with the pod's token). The RunPod API calls
# (create, pod record, delete, list) are made here, because rp.sh hard-codes the proxy-injected placeholder key.
set -euo pipefail
study=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
RP="$study/runpod/rp.sh"
API=https://rest.runpod.io/v1
GQL=https://api.runpod.io/graphql
VCPU=16                                   # every flavor in FLAVORS gives >= 64 GB at 16 vCPU (ramMultiplier 4 or 8)
FLAVORS='["cpu5m","cpu3m","cpu5g","cpu3g"]'  # Amendment 16's list and priority, unchanged
MIN_VCPU=12 MIN_MEM_GB=61                 # 56 GiB = 60.13 GB, the registered request (Amendment 14)
HOURS=26 CEIL_USD=40                      # decision 4: price x (25 h job limit + 1 h) <= US$40
DEADLINE_S=90000                          # the registered job time limit (requests_r2i: seconds 90000)
RAYON=12                                  # Amendment 9; rp.sh start sends rayon 12
: "${RPC_STATE:?set RPC_STATE to a private scratch directory (pod tokens and API header live there; never commit it)}"
mkdir -p "$RPC_STATE"; chmod 700 "$RPC_STATE"
export TOKDIR="$RPC_STATE"                # rp.sh reads $TOKDIR/<pod id>.token
hdr="$RPC_STATE/api.hdr"                  # API auth header in a 0600 file, so the key never appears in argv or receipts
( umask 077; echo "Authorization: Bearer ${RUNPOD_API_KEY:-proxy-injected}" > "$hdr" )
api() { curl -sS -m 60 -H @"$hdr" -H 'Content-Type: application/json' "$@"; }
now() { date -u +%FT%TZ; }
epoch() { date -u -d "$1" +%s; }
# Remaining pods whose name starts with pkm-x- (any status). Prints one JSON line per pod.
list_pkm() { api "$API/pods" | jq -c '.[] | select(.name | startswith("pkm-x-")) | {id,name,desiredStatus,costPerHr}'; }
