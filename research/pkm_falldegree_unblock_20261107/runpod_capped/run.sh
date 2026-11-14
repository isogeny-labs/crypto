#!/usr/bin/env bash
# run.sh NAME STAGE  -- start ONE capped F4 run on a reserved RunPod Secure Cloud CPU pod (draft Amendment 17,
# AMENDMENT_R2_H_runpod_capped.md). Shell only.
#   NAME   pkm-x-t<T>-k<K>-c<C>[-suffix] for a study cell (STAGE must equal requests_r2i/pfd-x-t<T>-k<K>-c<C>.json's
#          command[1]), or pkm-x-dev-<word> for a development check (t <= 10 only; not data).
#   STAGE  cap:T:K:C:DENSE, run by entry_r2.sh, whose cap branch, preamble and tail are byte-identical to entry.sh's.
# Env: RPC_STATE (private scratch dir), RPC_BIN (dir holding the four study binaries and entry_r2.sh),
#      RPC_RECEIPTS (default runpod_capped/receipts/NAME), RUNPOD_API_KEY (else the proxy-injected placeholder).
# Exit: 0 started; 2 usage; 3 refused before create (hash, name, stage, price); 4 pod refused after create (deleted);
#       5 upload/start failed (pod deleted).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"
[ $# -eq 2 ] || { echo "usage: run.sh NAME STAGE" >&2; exit 2; }
NAME=$1 STAGE=$2
: "${RPC_BIN:?set RPC_BIN to the directory with the image layer binaries}"
REC=${RPC_RECEIPTS:-$here/receipts/$NAME}; mkdir -p "$REC"
log() { echo "$(now) $*" | tee -a "$REC/run.log"; }

# 1. Name and stage.
[[ $STAGE =~ ^cap:([0-9]+):([01]):([0-9]+):([0-9]+)$ ]] || { log "REFUSED stage $STAGE is not cap:T:K:C:DENSE"; exit 3; }
T=${BASH_REMATCH[1]} K=${BASH_REMATCH[2]} C=${BASH_REMATCH[3]}
if [[ $NAME =~ ^pkm-x-t([0-9]+)-k([01])-c([0-9]+)(-[a-z0-9]+)*$ ]]; then
  req="$study/requests_r2i/pfd-x-t${BASH_REMATCH[1]}-k${BASH_REMATCH[2]}-c${BASH_REMATCH[3]}.json"
  [ -f "$req" ] || { log "REFUSED no request $req"; exit 3; }
  jq -e --arg s "$STAGE" '.job.command == ["/usr/local/bin/entry.sh", $s]' "$req" >/dev/null \
    || { log "REFUSED stage $STAGE differs from $req command"; exit 3; }
  log "cell request $req sha256 $(sha256sum "$req" | cut -d' ' -f1)"
elif [[ $NAME =~ ^pkm-x-dev-[a-z0-9-]+$ ]]; then
  [ "$T" -le 10 ] || { log "REFUSED development check with t = $T > 10"; exit 3; }
  log "DEVELOPMENT CHECK, NOT DATA: $NAME $STAGE"
else log "REFUSED name $NAME (pkm-x-t<T>-k<K>-c<C>[-suffix] or pkm-x-dev-<word>)"; exit 3; fi

# 2. Binary hashes against image/binaries.sha256 (and build-info.json against image/build-info.json).
up="$RPC_STATE/$NAME.upload"; rm -rf "$up"; mkdir -p "$up"
for f in pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros entry_r2.sh run.sh; do
  want=$(awk -v f="$f" '$2==f{print $1}' "$study/image/binaries.sha256")
  case $f in run.sh) src="$study/../prime_fall_degree_20261027/run.sh" ;; *) src="$RPC_BIN/$f" ;; esac
  got=$(sha256sum "$src" | cut -d' ' -f1)
  [ -n "$want" ] && [ "$got" = "$want" ] || { log "REFUSED sha256 $f got $got want $want"; exit 3; }
  install -m755 "$src" "$up/$f"; log "sha256 ok $f $got"
done
install -m644 "$study/image/build-info.json" "$up/build-info.json"
jq -e --slurpfile b <(jq '.study_binary_sha256' "$study/image/image.json") '.sha256 == $b[0]' "$up/build-info.json" >/dev/null \
  || { log "REFUSED build-info.json hashes differ from image.json"; exit 3; }

# 3. No live pod under this name.
live=$(api "$API/pods" | jq --arg n "$NAME" '[.[] | select(.name == $n)] | length')
[ "$live" = 0 ] || { log "REFUSED a pod named $NAME already exists ($live)"; exit 3; }

# 4. Price read before create (GraphQL securePrice of each flavor at VCPU); every flavor must pass the guard.
: > "$REC/price.jsonl"
for f in $(jq -r '.[]' <<<"$FLAVORS"); do
  mult=$(api -X POST "$GQL" -d "{\"query\":\"{ cpuFlavors(input:{id:\\\"$f\\\"}) { ramMultiplier } }\"}" | jq -r '.data.cpuFlavors[0].ramMultiplier // empty')
  [ -n "$mult" ] || { log "REFUSED price not readable for $f"; exit 3; }
  inst="$f-$VCPU-$((VCPU * mult))"
  p=$(api -X POST "$GQL" -d "{\"query\":\"{ cpuFlavors(input:{id:\\\"$f\\\"}) { specifics(input:{instanceId:\\\"$inst\\\"}) { stockStatus securePrice } } }\"}" \
      | jq -c --arg i "$inst" --arg at "$(now)" '{at:$at,instance:$i} + .data.cpuFlavors[0].specifics')
  echo "$p" >> "$REC/price.jsonl"; log "price $p"
  jq -e --argjson h $HOURS --argjson c $CEIL_USD '.securePrice > 0 and .securePrice * $h <= $c' <<<"$p" >/dev/null \
    || { log "REFUSED cost guard: $inst price x $HOURS h > US\$$CEIL_USD"; exit 3; }
done

# 5. Create the pod: Amendment 16's body (rp.sh create) with NAME, VCPU and an explicit Secure Cloud.
tok=$(od -An -tx1 -N24 /dev/urandom | tr -d ' \n')
boot="import base64;exec(base64.b64decode('$(base64 -w0 "$study/runpod/server.py")'))"
body=$(jq -n --arg n "$NAME" --argjson v $VCPU --arg t "$tok" --arg b "$boot" --argjson fl "$FLAVORS" '{name:$n,computeType:"CPU",
  cloudType:"SECURE",cpuFlavorIds:$fl,cpuFlavorPriority:"custom",vcpuCount:$v,interruptible:false,
  imageName:"python:3.12-slim",containerDiskInGb:20,ports:["8000/http"],env:{RUNNER_TOKEN:$t},
  dockerEntrypoint:["python3","-u","-c"],dockerStartCmd:[$b]}')
log "server.py sha256 $(sha256sum "$study/runpod/server.py" | cut -d' ' -f1); rp.sh sha256 $(sha256sum "$RP" | cut -d' ' -f1)"
out=$(api -X POST "$API/pods" -d "$body"); id=$(jq -r '.id // empty' <<<"$out" 2>/dev/null || true)
[ -n "$id" ] || { log "create failed: $(jq -c 'del(.env?)' <<<"$out" 2>/dev/null || echo "$out" | head -c 500)"; exit 4; }
( umask 077; echo "$tok" > "$RPC_STATE/$id.token" )
created=$(now)
jq -n --arg id "$id" --arg n "$NAME" --arg s "$STAGE" --arg c "$created" '{id:$id,name:$n,stage:$s,created:$c}' > "$RPC_STATE/$NAME.json"
jq 'del(.env, .dockerStartCmd)' <<<"$out" > "$REC/pod-create.json"
log "created pod $id ($NAME) at $created"
started=0
cleanup() { if [ "$started" = 0 ]; then log "deleting pod $id (not started)"; api -X DELETE "$API/pods/$id" >> "$REC/run.log" 2>&1 || true; fi; }
trap cleanup EXIT

# 6. Pod record: price, vCPU, memory, reserved.
api "$API/pods/$id" | jq 'del(.env, .dockerStartCmd)' > "$REC/pod-record.json"
jq -e --argjson h $HOURS --argjson c $CEIL_USD --argjson v $MIN_VCPU --argjson m $MIN_MEM_GB \
  '.costPerHr > 0 and .costPerHr * $h <= $c and .vcpuCount >= $v and .memoryInGb >= $m and (.interruptible != true)' \
  "$REC/pod-record.json" >/dev/null || { log "REFUSED pod record $(jq -c '{costPerHr,vcpuCount,memoryInGb,interruptible}' "$REC/pod-record.json")"; exit 4; }
log "pod record $(jq -c '{id,costPerHr,vcpuCount,memoryInGb,interruptible,machineId,machine}' "$REC/pod-record.json")"

# 7. Wait for server.py (15 min).
for i in $(seq 1 90); do
  if "$RP" status "$id" 2>/dev/null | jq -e '.state == "idle"' > "$REC/status-ready.json" 2>/dev/null; then break; fi
  [ "$i" = 90 ] && { log "server not ready after 15 min"; exit 5; }; sleep 10
done
"$RP" status "$id" > "$REC/status-ready.json"; log "server ready $(jq -c '{nproc,meminfo}' "$REC/status-ready.json")"

# 8. Upload (rp.sh, 2 MB chunks) and compare each file's pod-side SHA-256 with the expected one.
"$RP" upload "$id" "$up" > "$REC/upload.jsonl"
for f in pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros entry_r2.sh run.sh build-info.json; do
  want=$(sha256sum "$up/$f" | cut -d' ' -f1)
  jq -e --arg f "$f" --arg w "$want" 'select(.name == $f) | .sha256 == $w' "$REC/upload.jsonl" >/dev/null \
    || { log "upload sha256 mismatch or missing: $f"; exit 5; }
  log "pod sha256 ok $f $want"
done

# 9. Start (server.py re-checks every SHA-256 against image.json before it runs entry_r2.sh STAGE with rayon 12).
"$RP" start "$id" "$(jq -nc --arg s "$STAGE" '[$s]')" > "$REC/start.json"
jq -e '.state == "running"' "$REC/start.json" >/dev/null || { log "start refused: $(cat "$REC/start.json")"; exit 5; }
started=1
dl=$(date -u -d "@$(( $(date +%s) + DEADLINE_S ))" +%FT%TZ)
jq --arg d "$dl" '. + {deadline:$d}' "$RPC_STATE/$NAME.json" > "$RPC_STATE/$NAME.json.tmp" && mv "$RPC_STATE/$NAME.json.tmp" "$RPC_STATE/$NAME.json"
cp "$RPC_STATE/$NAME.json" "$REC/pod-state.json"
log "STARTED $NAME pod $id stage $STAGE; pod deadline $dl; collect with fetch.sh $NAME"
