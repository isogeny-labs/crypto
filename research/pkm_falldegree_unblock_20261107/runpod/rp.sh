#!/usr/bin/env bash
# Client for runpod/server.py (Amendment R3 draft). Auth to rest.runpod.io is injected by the session proxy.
# Usage:
#   rp.sh create NAME VCPU          -> creates a reserved (interruptible=false) CPU pod running server.py; prints pod JSON
#   rp.sh pod ID                    -> pod record (costPerHr, vcpuCount, memoryInGb, desiredStatus)
#   rp.sh upload ID DIR             -> PUT every allowed file in DIR
#   rp.sh start ID STAGES_JSON      -> POST /start with stages + the frozen SHA-256s from ../image/image.json
#   rp.sh status ID | fetch ID OUT.tar.gz | delete ID
# Token: $RUNNER_TOKEN (generated per pod by `create`, kept in $TOKDIR/<pod>.token, never committed).
set -euo pipefail
API=https://rest.runpod.io/v1; H=(-H "Authorization: Bearer proxy-injected" -H "Content-Type: application/json")
here=$(cd "$(dirname "$0")" && pwd); TOKDIR=${TOKDIR:?scratch dir for pod tokens}
url() { echo "https://$1-8000.proxy.runpod.net"; }
tok() { cat "$TOKDIR/$1.token"; }
case "$1" in
  create)
    name=$2 vcpu=$3; t=$(python3 -c 'import secrets;print(secrets.token_hex(24))')
    boot="import base64;exec(base64.b64decode('$(base64 -w0 "$here/server.py")'))"
    body=$(jq -n --arg n "$name" --argjson v "$vcpu" --arg t "$t" --arg b "$boot" '{name:$n,computeType:"CPU",
      cpuFlavorIds:["cpu5m","cpu3m","cpu5g","cpu3g"],cpuFlavorPriority:"custom",vcpuCount:$v,interruptible:false,
      imageName:"python:3.12-slim",containerDiskInGb:20,ports:["8000/http"],env:{RUNNER_TOKEN:$t},
      dockerEntrypoint:["python3","-u","-c"],dockerStartCmd:[$b]}')
    out=$(curl -sS -X POST "$API/pods" "${H[@]}" -d "$body"); id=$(jq -r '.id // empty' <<<"$out")
    [ -n "$id" ] || { echo "$out" >&2; exit 1; }
    umask 077; echo "$t" > "$TOKDIR/$id.token"; jq '{id,name,costPerHr,vcpuCount,memoryInGb,interruptible,machine}' <<<"$out" ;;
  pod) curl -sS "$API/pods/$2" "${H[@]}" | jq '{id,name,desiredStatus,costPerHr,vcpuCount,memoryInGb,interruptible,lastStartedAt}' ;;
  upload) for f in pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros entry_r2.sh run.sh build-info.json; do
      [ -f "$3/$f" ] || continue; curl -sS -f -X PUT -H "X-Token: $(tok $2)" --data-binary "@$3/$f" "$(url $2)/upload/$f"; echo; done ;;
  start) shas=$(jq '.study_binary_sha256 | del(.["entry.sh"])' "$here/../image/image.json")
      jq -n --argjson s "$3" --argjson h "$shas" '{stages:$s,sha256:$h,rayon:12}' |
      curl -sS -X POST -H "X-Token: $(tok $2)" -H "Content-Type: application/json" --data-binary @- "$(url $2)/start"; echo ;;
  status) curl -sS -H "X-Token: $(tok $2)" "$(url $2)/status" ;;
  fetch) curl -sS -f -H "X-Token: $(tok $2)" -o "$3" "$(url $2)/results.tar.gz" && sha256sum "$3" ;;
  delete) curl -sS -X DELETE "$API/pods/$2" "${H[@]}"; echo "deleted $2" ;;
  *) echo "unknown command" >&2; exit 2 ;;
esac
