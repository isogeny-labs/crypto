#!/usr/bin/env bash
# Client for podserver.py on RunPod. The RunPod API key is injected by this session's proxy (header placeholder below).
# Usage:
#   rp.sh create NAME VCPU [FLAVOR=cpu5c] [DISK_GB=30]   -> prints POD_ID TOKEN (state kept in $RP_STATE/NAME.json)
#   rp.sh wait NAME            wait until the pod's server answers
#   rp.sh put NAME LOCAL REMOTE | get NAME REMOTE LOCAL | exec NAME 'cmd' | ls NAME | rc NAME ID | log NAME ID
#   rp.sh delete NAME          terminate the pod (always do this)
set -euo pipefail
API=https://rest.runpod.io/v1; AUTH="Authorization: Bearer proxy-injected"
RP_STATE=${RP_STATE:-$(dirname "$0")/state}; mkdir -p "$RP_STATE"
here=$(cd "$(dirname "$0")" && pwd)
st() { python3 -c "import json,sys;print(json.load(open('$RP_STATE/$1.json'))['$2'])"; }
url() { echo "https://$(st $1 id)-8000.proxy.runpod.net/$(st $1 token)"; }
case "$1" in
create)
  name=$2; vcpu=$3; flavor=${4:-cpu5c}; disk=${5:-30}
  token=$(python3 -c "import secrets;print(secrets.token_hex(16))")
  srv=$(base64 -w0 "$here/podserver.py")
  body=$(python3 - "$name" "$vcpu" "$flavor" "$disk" "$token" "$srv" <<'P'
import json,sys
name,vcpu,flavor,disk,token,srv=sys.argv[1:]
print(json.dumps({"name":name,"imageName":"runpod/base:1.0.2-ubuntu2404","computeType":"CPU","cpuFlavorIds":[flavor],
 "vcpuCount":int(vcpu),"containerDiskInGb":int(disk),"ports":["8000/http"],"env":{"POD_TOKEN":token,"PODSERVER_B64":srv},
 "dockerEntrypoint":["bash","-c"],"dockerStartCmd":["echo $PODSERVER_B64 | base64 -d > /podserver.py && python3 /podserver.py"]}))
P
)
  resp=$(curl -sS -m 60 -X POST $API/pods -H "$AUTH" -H 'Content-Type: application/json' -d "$body")
  id=$(echo "$resp" | python3 -c "import json,sys;d=json.load(sys.stdin);print(d['id'] if isinstance(d,dict) and 'id' in d else '')")
  [ -n "$id" ] || { echo "create failed: $resp" >&2; exit 1; }
  echo "{\"id\":\"$id\",\"token\":\"$token\",\"name\":\"$name\"}" > "$RP_STATE/$name.json"; echo "$id" ;;
wait) for i in $(seq 1 90); do curl -sf -m 10 "$(url $2)/ls" >/dev/null 2>&1 && { echo ready; exit 0; }; sleep 10; done; echo "not ready" >&2; exit 1 ;;
put) # chunked: the RunPod proxy rejects request bodies of a few MB; 2 MB parts, reassembled on the pod, then sha256-checked
  tmp=$(mktemp -d); split -b 2M -d -a 4 "$3" "$tmp/part."; u=$(url $2)
  for f in "$tmp"/part.*; do curl -sSf -m 300 -X PUT --data-binary @"$f" "$u/f/.upload/$4/$(basename $f)" >/dev/null; done
  want=$(sha256sum "$3" | cut -d' ' -f1); rm -rf "$tmp"
  id=$(curl -sSf -m 60 -X POST --data-binary "mkdir -p \"\$(dirname '$4')\" && cat .upload/'$4'/part.* > '$4' && rm -rf .upload/'$4' && chmod +x '$4' && sha256sum '$4'" "$u/exec" | python3 -c "import json,sys;print(json.load(sys.stdin)['id'])")
  for i in $(seq 1 60); do got=$(curl -sf -m 30 "$u/f/exec_$id.log" | cut -d' ' -f1); [ -n "$got" ] && break; sleep 2; done
  [ "$got" = "$want" ] && echo "put ok $4 sha256 $want" || { echo "put FAILED $4 (got '$got' want $want)" >&2; exit 1; } ;;
get) curl -sSf -m 900 "$(url $2)/f/$3" -o "$4" ;;
exec) curl -sSf -m 60 -X POST --data-binary "$3" "$(url $2)/exec" ;;
ls) curl -sSf -m 60 "$(url $2)/ls" ;;
rc) curl -sf -m 30 "$(url $2)/f/exec_$3.rc" || echo running ;;
log) curl -sSf -m 60 "$(url $2)/f/exec_$3.log" ;;
delete) curl -sS -m 60 -X DELETE $API/pods/$(st $2 id) -H "$AUTH"; echo "deleted $(st $2 id)" ;;
*) echo "unknown command" >&2; exit 2 ;;
esac
