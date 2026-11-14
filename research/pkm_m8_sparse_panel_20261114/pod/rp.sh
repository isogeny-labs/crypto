#!/usr/bin/env bash
# Client for podserver.py on RunPod (adapted from research/runpod_kit_20261112/rp.sh).
# Differences from the kit: the key is read from $RUNPOD_API_KEY and handed to curl through a
# process substitution (`-H @<(printf ...)`), so it is never echoed, never in argv, never written to disk;
# `flavors`, `pods`, `info` added (read-only); create/delete only act on names starting with pkm-m8-.
# Usage:
#   rp.sh flavors [OUT.json]    read-only: CPU flavor catalogue + stock + price table (REST has no CPU pricing,
#                               so the GraphQL `cpuFlavors` query is used; REST enum is cross-checked)
#   rp.sh pods                  read-only: list pods (id name status vcpu mem $/h)
#   rp.sh info NAME             read-only: the pod recorded in state: id status vcpu mem $/h
#   rp.sh create NAME VCPU [FLAVOR=cpu5m] [DISK_GB=60]   -> prints POD_ID (state in $RP_STATE/NAME.json)
#   rp.sh wait NAME | put NAME LOCAL REMOTE | get NAME REMOTE LOCAL | exec NAME 'cmd' | ls NAME | rc NAME ID | log NAME ID
#   rp.sh delete NAME           terminate the pod this client created (always do this)
set -euo pipefail
API=https://rest.runpod.io/v1; GQL=https://api.runpod.io/graphql
RP_STATE=${RP_STATE:-$(dirname "$0")/state}; mkdir -p "$RP_STATE"
here=$(cd "$(dirname "$0")" && pwd)
need_key() { [ -n "${RUNPOD_API_KEY:-}" ] || { echo "RUNPOD_API_KEY is not set" >&2; exit 3; }; }
# every authenticated call goes through here; the header never appears in argv or output
api() { need_key; curl -sS -m 60 -H @<(printf 'Authorization: Bearer %s\n' "$RUNPOD_API_KEY") "$@"; }
st() { python3 -c "import json,sys;print(json.load(open('$RP_STATE/$1.json'))['$2'])"; }
url() { echo "https://$(st $1 id)-8000.proxy.runpod.net/$(st $1 token)"; }
guard() { case "$1" in pkm-m8-*) ;; *) echo "refusing: pod name must start with pkm-m8- (got '$1')" >&2; exit 4;; esac; }
case "${1:-}" in
flavors)
  out=${2:-}
  gql=$(api -X POST "$GQL" -H 'Content-Type: application/json' -d '{"query":"{ cpuFlavors { id groupId displayName minVcpu maxVcpu ramMultiplier diskLimitPerVcpu specifics { stockStatus securePrice slsPrice } } }"}')
  tmpd=$(mktemp -d); trap 'rm -rf "$tmpd"' EXIT
  printf '%s' "$gql" > "$tmpd/gql.json"; curl -sS -m 60 "$API/openapi.json" -o "$tmpd/rest.json" 2>/dev/null || echo '{}' > "$tmpd/rest.json"
  python3 - "$out" "$tmpd/gql.json" "$tmpd/rest.json" <<'P'
import json,sys
out,gqlf,restf=sys.argv[1:]
gql=open(gqlf).read(); rest=open(restf).read()
g=json.loads(gql)
if "data" not in g: sys.exit("flavors: GraphQL error: "+gql[:300])
enum=[]
try: enum=json.loads(rest)["components"]["schemas"]["PodCreateInput"]["properties"]["cpuFlavorIds"]["items"]["enum"]
except Exception: pass
rows=[]
for f in g["data"]["cpuFlavors"]:
    sp=f["specifics"]; v=f["maxVcpu"]
    # calibration: existing pods show $/h = securePrice * vcpu / 2 (cpu5c 8 vcpu 0.28, cpu5m 16 vcpu 1.04)
    rows.append({"flavor":f["id"],"group":f["groupId"],"name":f["displayName"],"min_vcpu":f["minVcpu"],"max_vcpu":v,
      "mem_gb_per_vcpu":f["ramMultiplier"],"max_mem_gb":v*f["ramMultiplier"],"disk_gb_per_vcpu":f["diskLimitPerVcpu"],
      "max_disk_gb":v*f["diskLimitPerVcpu"],"stock":sp["stockStatus"],"secure_price_raw":sp["securePrice"],
      "est_usd_per_hour_at_max_vcpu":round(sp["securePrice"]*v/2,4),"est_usd_per_hour_per_vcpu":round(sp["securePrice"]/2,5)})
doc={"source":"api.runpod.io/graphql cpuFlavors (REST v1 has no CPU pricing endpoint; /v1/cpu-types does not exist)",
     "rest_enum_cpuFlavorIds":enum,"price_rule":"usd/h = secure_price_raw * vcpu / 2 (calibrated on existing pods; the authoritative number is costPerHr of the created pod)","flavors":rows}
print("%-6s %-18s %5s %7s %8s %6s %9s %10s" % ("flavor","name","vcpu","mem GB","disk GB","stock","raw","est $/h"))
for r in rows: print("%-6s %-18s %5d %7d %8d %6s %9s %10.3f" % (r["flavor"],r["name"],r["max_vcpu"],r["max_mem_gb"],r["max_disk_gb"],r["stock"],r["secure_price_raw"],r["est_usd_per_hour_at_max_vcpu"]))
if out: json.dump(doc,open(out,"w"),indent=1); print("wrote",out)
P
  ;;
pods) api "$API/pods" | python3 -c "
import json,sys
d=json.load(sys.stdin)
if not isinstance(d,list): sys.exit('pods: '+str(d)[:300])
for p in d: print(p['id'],p['name'],p.get('desiredStatus'),p.get('vcpuCount'),p.get('memoryInGb'),p.get('costPerHr'))" ;;
info) guard "$2"; api "$API/pods/$(st $2 id)" | python3 -c "
import json,sys
p=json.load(sys.stdin)
print(p.get('id'),p.get('name'),p.get('desiredStatus'),'vcpu',p.get('vcpuCount'),'mem_gb',p.get('memoryInGb'),'usd_per_hr',p.get('costPerHr'))" ;;
create)
  name=$2; vcpu=$3; flavor=${4:-cpu5m}; disk=${5:-60}; guard "$name"
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
  resp=$(api -X POST $API/pods -H 'Content-Type: application/json' -d "$body")
  id=$(echo "$resp" | python3 -c "import json,sys;d=json.load(sys.stdin);print(d['id'] if isinstance(d,dict) and 'id' in d else '')")
  [ -n "$id" ] || { echo "create failed: $resp" >&2; exit 1; }
  echo "{\"id\":\"$id\",\"token\":\"$token\",\"name\":\"$name\"}" > "$RP_STATE/$name.json"; chmod 600 "$RP_STATE/$name.json"; echo "$id" ;;
wait) for i in $(seq 1 90); do curl -sf -m 10 "$(url $2)/ls" >/dev/null 2>&1 && { echo ready; exit 0; }; sleep 10; done; echo "not ready" >&2; exit 1 ;;
put) # chunked: the RunPod proxy rejects request bodies of a few MB; 2 MB parts, reassembled on the pod, then sha256-checked
  tmp=$(mktemp -d); split -b 2M -d -a 4 "$3" "$tmp/part."; u=$(url $2)
  for f in "$tmp"/part.*; do curl -sSf -m 300 -X PUT --data-binary @"$f" "$u/f/.upload/$4/$(basename $f)" >/dev/null; done
  want=$(sha256sum "$3" | cut -d' ' -f1); rm -rf "$tmp"
  id=$(curl -sSf -m 60 -X POST --data-binary "mkdir -p \"\$(dirname '$4')\" && cat .upload/'$4'/part.* > '$4' && rm -rf .upload/'$4' && chmod +x '$4' && sha256sum '$4'" "$u/exec" | python3 -c "import json,sys;print(json.load(sys.stdin)['id'])")
  got=""; for i in $(seq 1 60); do got=$(curl -sf -m 30 "$u/f/exec_$id.log" | cut -d' ' -f1) || true; [ -n "$got" ] && break; sleep 2; done
  [ "$got" = "$want" ] && echo "put ok $4 sha256 $want" || { echo "put FAILED $4 (got '$got' want $want)" >&2; exit 1; } ;;
get) curl -sSf -m 900 "$(url $2)/f/$3" -o "$4" ;;
exec) curl -sSf -m 60 -X POST --data-binary "$3" "$(url $2)/exec" ;;
ls) curl -sSf -m 60 "$(url $2)/ls" ;;
rc) curl -sf -m 30 "$(url $2)/f/exec_$3.rc" || echo running ;;
log) curl -sSf -m 60 "$(url $2)/f/exec_$3.log" ;;
delete) guard "$2"; code=$(api -o /dev/null -w '%{http_code}' -X DELETE $API/pods/$(st $2 id))
  case "$code" in 2*) echo "deleted $(st $2 id) (http $code)";; *) echo "delete FAILED http $code for $(st $2 id)" >&2; exit 1;; esac ;;
*) echo "unknown command" >&2; exit 2 ;;
esac
