#!/usr/bin/env bash
# GPU variant of research/runpod_kit_20261112/rp.sh (same podserver.py, same put/get/exec/delete).
# Only `create` differs: a GPU pod on a CUDA devel image, which installs python3/make/g++ before
# starting the server. The RunPod key is injected by the session proxy (placeholder header).
#   rp_gpu.sh create NAME "GPU TYPE ID" [IMAGE] [DISK_GB=40]
#     env RP_CUDA (comma list of allowed host CUDA versions, default 12.8,12.9,13.0; empty = any)
#     env RP_CLOUD (SECURE, the default, or COMMUNITY)
#   rp_gpu.sh wait|put|get|exec|ls|rc|log|delete ...   -> delegated to the CPU kit
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); kit=$here/../runpod_kit_20261112
export RP_STATE=${RP_STATE:-$here/state}; mkdir -p "$RP_STATE"
API=https://rest.runpod.io/v1; AUTH="Authorization: Bearer proxy-injected"
if [ "$1" != create ]; then exec "$kit/rp.sh" "$@"; fi
name=$2; gpu=$3; image=${4:-nvidia/cuda:12.8.1-devel-ubuntu24.04}; disk=${5:-40}
token=$(python3 -c "import secrets;print(secrets.token_hex(16))")
srv=$(base64 -w0 "$kit/podserver.py")
body=$(python3 - "$name" "$gpu" "$image" "$disk" "$token" "$srv" <<'P'
import json,sys
import os
name,gpu,image,disk,token,srv=sys.argv[1:]
cuda=[v for v in os.environ.get("RP_CUDA","12.8,12.9,13.0").split(",") if v]
cloud=os.environ.get("RP_CLOUD","SECURE")
start=("export DEBIAN_FRONTEND=noninteractive; (command -v python3 && command -v make && command -v g++) >/dev/null || "
       "(apt-get update -qq && apt-get install -y -qq python3 make g++ >/dev/null); "
       "echo $PODSERVER_B64 | base64 -d > /podserver.py && python3 /podserver.py")
print(json.dumps({"name":name,"imageName":image,"computeType":"GPU","gpuTypeIds":[gpu],"gpuCount":1,
 "cloudType":cloud,"containerDiskInGb":int(disk),
 "ports":["8000/http"],"env":{"POD_TOKEN":token,"PODSERVER_B64":srv},
 "dockerEntrypoint":["bash","-c"],"dockerStartCmd":[start],
 **({"allowedCudaVersions":cuda} if cuda else {})}))
P
)
resp=$(curl -sS -m 60 -X POST $API/pods -H "$AUTH" -H 'Content-Type: application/json' -d "$body")
id=$(echo "$resp" | python3 -c "import json,sys;d=json.load(sys.stdin);print(d['id'] if isinstance(d,dict) and 'id' in d else '')" 2>/dev/null || true)
[ -n "$id" ] || { echo "create failed: $resp" >&2; exit 1; }
echo "$resp" | python3 -c "import json,sys;d=json.load(sys.stdin);print('cost/hr',d.get('costPerHr'),'machine',d.get('machineId'))"
echo "{\"id\":\"$id\",\"token\":\"$token\",\"name\":\"$name\"}" > "$RP_STATE/$name.json"; echo "$id"
