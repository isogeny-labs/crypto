#!/usr/bin/env bash
# run.sh NAME STAGE  -- start ONE capped F4 run on a reserved RunPod CPU pod (draft Amendment 18,
# AMENDMENT_R2_H_runpod_capped.md, sections 3-5). Shell only.
#   NAME   one of the allow-list in common.sh (STUDY_NAMES), launched only in the registered order (next_allowed), or
#          pkm-x-dev-<word> for a development check (t <= 10 only; receipts in receipts_dev/; not data).
#   STAGE  cap:T:K:C:DENSE; for a study name it must equal command[1] of the name's frozen request.
# Env: RPC_STATE (private scratch dir outside the worktree), RPC_BIN (dir with the four study binaries and entry_r2.sh),
#      RPC_LAYER (the image layer tarball; required for study names), RUNPOD_API_KEY (else the proxy-injected placeholder).
#      SHA_FILE must be unset.
# Exit: 0 started; 2 usage; 3 refused before create (nothing created, no attempt); 4 refused or failed after the create
#       call, pod deleted and confirmed gone (not an attempt); 5 upload or start failed, pod deleted and confirmed gone (not
#       an attempt); 10 cpu5m not in stock (nothing created, no attempt); 11 a pre-start deletion was not confirmed (the
#       pod may still exist: run reap.sh, tell the owner).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"
[ $# -eq 2 ] || { echo "usage: run.sh NAME STAGE" >&2; exit 2; }
NAME=$1 STAGE=$2
REC=$(rec_dir "$NAME") || { echo "REFUSED name $NAME: not on the allow-list (common.sh STUDY_NAMES) and not pkm-x-dev-<word>" >&2; exit 3; }
OUT="$OUT_ROOT/$NAME"
STAGELOG=$(mktemp); trap 'rm -f "$STAGELOG"' EXIT; log() { echo "$(now) $*" | tee -a "$STAGELOG" >&2; }
refuse() { log "REFUSED $*"; exit 3; }
id="" started=0 created_rec=0 upd="" q="" tokf="" bodyf=""

# 0. Environment.
[ -z "${SHA_FILE:-}" ] || refuse "SHA_FILE is set; this route sends the frozen image's hashes only (rp_pinned.sh start)"
: "${RPC_BIN:?set RPC_BIN to the directory with the image layer binaries}"
[ ! -e "$REC" ] || refuse "receipts directory $REC already exists (one attempt per name; never reused)"
[ ! -e "$OUT" ] || refuse "results directory $OUT already exists (never overwritten)"
npre=$(compgen -G "$REC.prestart-*" | wc -l || true)
[ "$npre" -lt 2 ] || refuse "$NAME already had $npre pre-start failures; a third goes to the owner"
[ -f "$LEDGER" ] && ledger_has "$NAME" && refuse "$NAME already has a row in $LEDGER (any state): a further attempt needs the owner (N6)"

# 1. Name, stage, order.
[[ $STAGE =~ ^cap:([0-9]+):([01]):([0-9]+):([0-9]+)$ ]] || refuse "stage $STAGE is not cap:T:K:C:DENSE"
T=${BASH_REMATCH[1]} DENSE=${BASH_REMATCH[4]}
floor=$MEM_FLOOR_GB
if is_study "$NAME"; then
  req=$(name_request "$NAME"); [ -f "$req" ] || refuse "no request $req"
  if is_m112 "$NAME"; then
    floor=$MEM_FLOOR_M112_GB
    src="$study/requests_r2i/pfd-x-$(cell_of "$NAME").json"
    want=$(m112_request "$src") || refuse "cannot derive the -m112 request from $src"
    jq -e --argjson w "$want" '. == $w' "$req" >/dev/null || refuse "$req differs from the request derived from $src (Amendment 14)"
  fi
  jq -e --arg s "$STAGE" '.job.command == ["/usr/local/bin/entry.sh", $s]' "$req" >/dev/null \
    || refuse "stage $STAGE differs from $req command"
  log "cell request ${req#"$study/"} sha256 $(sha256sum "$req" | cut -d' ' -f1)"
  nxt=$(next_allowed)
  [ "$nxt" = "$NAME" ] || refuse "order: the next study run allowed now is '$nxt', not $NAME (receipts: $REC_ROOT)"
  grep -q "^$AMENDMENT_HEADING\b" "$PREREG" || refuse "'$AMENDMENT_HEADING' is not in $PREREG (freeze the amendment first)"
  frozen_ok common.sh run.sh fetch.sh reap.sh || refuse "a route script's SHA-256 is not in the frozen amendment in $PREREG (S3)"
else
  [ "$T" -le 10 ] || refuse "development check with t = $T > 10"
  [ "$DENSE" = 12800000000 ] || refuse "development check with --max-dense $DENSE (use 12800000000)"
  log "DEVELOPMENT CHECK, NOT DATA: $NAME $STAGE"
fi
log "protocol $PREREG sha256 $(sha256sum "$PREREG" | cut -d' ' -f1); amendment heading present: $(grep -c "^$AMENDMENT_HEADING\b" "$PREREG" || true)"
log "git HEAD $(git -C "$here" rev-parse HEAD 2>/dev/null || echo none); worktree changes under runpod_capped: $(git -C "$here" status --porcelain -- . 2>/dev/null | wc -l)"
log "route sha256: $(cd "$here" && sha256sum common.sh run.sh fetch.sh reap.sh | awk '{ printf "%s %s; ", $2, $1 }')"

# 2. Pins: the client copy and the runner (decision D6).
for p in "$RP:$RP_SHA" "$SERVER:$SERVER_SHA"; do
  f=${p%:*} w=${p##*:} g=$(sha256sum "${p%:*}" | cut -d' ' -f1)
  [ "$g" = "$w" ] || refuse "pinned file ${f#"$study/"} sha256 $g, want $w"
  log "pin ok ${f#"$study/"} $g"
done

# 3. Binary hashes against the pins in common.sh (and image/binaries.sha256, image.json, which must agree with them),
#    build-info.json against image.json, and the image layer (S3, S12).
for f in $(awk '{ print $1 }' <<<"$BIN_SHA_PINS"); do
  [ "$(awk -v f="$f" '$2==f{print $1}' "$study/image/binaries.sha256")" = "$(bin_sha "$f")" ] \
    && [ "$(jq -r --arg f "$f" '.study_binary_sha256[$f]' "$study/image/image.json")" = "$(bin_sha "$f")" ] \
    || refuse "image/binaries.sha256 or image.json disagrees with the pinned hash of $f"
done
[ "$(jq -r '.layer.digest' "$study/image/image.json")" = "sha256:$LAYER_DIGEST" ] || refuse "image.json layer.digest disagrees with the pinned $LAYER_DIGEST"
upd=$(mktemp -d); trap 'rm -rf "$upd" "$STAGELOG"' EXIT
for f in pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros entry_r2.sh run.sh; do
  want=$(bin_sha "$f")
  case $f in run.sh) src="$study/../prime_fall_degree_20261027/run.sh" ;; *) src="$RPC_BIN/$f" ;; esac
  got=$(sha256sum "$src" 2>/dev/null | cut -d' ' -f1 || true)
  [ -n "$want" ] && [ "$got" = "$want" ] || refuse "sha256 $f got ${got:-missing} want $want"
  install -m755 "$src" "$upd/$f"; log "sha256 ok $f $got"
done
install -m644 "$study/image/build-info.json" "$upd/build-info.json"
jq -e --slurpfile b <(jq '.study_binary_sha256' "$study/image/image.json") '.sha256 == $b[0]' "$upd/build-info.json" >/dev/null \
  || refuse "build-info.json hashes differ from image.json"
ldig=$LAYER_DIGEST
if [ -n "${RPC_LAYER:-}" ]; then
  [ "$(sha256sum "$RPC_LAYER" | cut -d' ' -f1)" = "$ldig" ] || refuse "RPC_LAYER sha256 differs from image.json layer.digest $ldig"
  for f in pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros entry_r2.sh; do
    [ "$(tar -xzOf "$RPC_LAYER" "usr/local/bin/$f" | sha256sum | cut -d' ' -f1)" = "$(sha256sum "$upd/$f" | cut -d' ' -f1)" ] \
      || refuse "RPC_BIN/$f is not the file in layer $ldig"
  done
  log "RPC_BIN files are the files of image layer sha256:$ldig (RPC_LAYER checked)"
elif is_study "$NAME"; then refuse "RPC_LAYER is required for a study run (the layer tarball, sha256:$ldig)"
else log "RPC_LAYER not given (development check): binaries matched image/binaries.sha256 only"; fi

# 4. Study-wide ledger (S10): the sum of every job's actual or worst-case cost, plus this run's worst case.
lsum=$(ledger_sum) || refuse "ledger $LEDGER is missing, empty or malformed"
asof=$(ledger_asof)
if is_study "$NAME"; then
  [ -n "$asof" ] && [ "$asof" != never ] && [ $(( $(date +%s) - $(epoch "$asof") )) -le "$LEDGER_FRESH_S" ] \
    || refuse "ledger complete_as_of '$asof' is not within 24 h (the session must bring the ledger up to date)"
fi
log "ledger sum US\$$lsum (complete_as_of $asof)"

# 5. RunPod: no live pkm-x- pod (serial rule), then the price and stock of the one pinned flavor.
rpc_init; trap 'rm -rf "$upd" "$STAGELOG"; rpc_cleanup' EXIT
exec 9>"$RPC_STATE/$NAME.lock"; flock -n 9 || refuse "another run.sh, fetch.sh or reap.sh holds $NAME"
live=$(list_pkm) || refuse "pod list not readable"
[ -z "$live" ] || refuse "a pkm-x- pod is live: $(tr '\n' ' ' <<<"$live")"
q=$(mktemp); trap 'rm -rf "$upd" "$STAGELOG" "$q"; rpc_cleanup' EXIT
jq -n --arg f "$FLAVOR" '{query: "{ cpuFlavors(input:{id:\"\($f)\"}) { ramMultiplier } }"}' > "$q"
mult=$(api -X POST "$GQL" --data-binary @"$q" | jq -r '.data.cpuFlavors[0].ramMultiplier // empty' || true)
[ "$mult" = $((RAM_GB / VCPU)) ] || refuse "ramMultiplier of $FLAVOR is '${mult}', want $((RAM_GB / VCPU))"
jq -n --arg f "$FLAVOR" --arg i "$INSTANCE" '{query: "{ cpuFlavors(input:{id:\"\($f)\"}) { specifics(input:{instanceId:\"\($i)\"}) { stockStatus securePrice } } }"}' > "$q"
price_line=$(api -X POST "$GQL" --data-binary @"$q" | jq -c --arg i "$INSTANCE" --arg at "$(now)" '{at:$at,instance:$i} + (.data.cpuFlavors[0].specifics // {})' || true)
log "price $price_line"
jq -e '(.stockStatus // "") != "" and .stockStatus != "None" and (.securePrice // 0) > 0' <<<"$price_line" >/dev/null 2>&1 \
  || { log "NOT IN STOCK or no price: $INSTANCE ($price_line). Nothing created; no attempt."; exit 10; }
price=$(jq -r '.securePrice' <<<"$price_line")
jq -e --argjson h $WORST_HOURS --argjson c $CEIL_USD '.securePrice * $h <= $c' <<<"$price_line" >/dev/null \
  || refuse "cost guard: $INSTANCE $price/h x $WORST_HOURS h > US\$$CEIL_USD"
worst=$(awk -v p="$price" -v h=$WORST_HOURS 'BEGIN{printf "%.4f", p * h}')
awk -v s="$lsum" -v w="$worst" -v c=$STUDY_CEIL_USD 'BEGIN{exit !(s + w <= c)}' \
  || refuse "study ceiling: ledger US\$$lsum + this run's worst case US\$$worst > US\$$STUDY_CEIL_USD"

# 6. Cleanup for any failure before the runner starts: delete the pod (found by id, or by name when the create call
#    failed or timed out), confirm it is gone, record the cost, and move the receipts aside (not an attempt).
precleanup() {
  local rc=$? ids="" i n gone=0 c
  rm -rf "$upd"; rm -f "$bodyf" "$q"
  if [ "$started" = 1 ]; then rpc_cleanup; remind "${REC#"$here/"}"; return; fi
  if [ "$created_rec" = 0 ]; then rpc_cleanup; return; fi
  [ -n "$id" ] && ids=$id
  for i in 1 2 3 4 5 6; do   # a timed-out create can appear late: look for the name a few times
    n=$(list_pkm 2>/dev/null | jq -r --arg n "$NAME" 'select(.name == $n) | .id' | tr '\n' ' ' || true)
    ids=$(echo "$ids $n" | tr ' ' '\n' | sort -u | tr '\n' ' '); [ -n "${ids// /}" ] && break; [ -n "$id" ] && break; sleep "$POLL_S"
  done
  for i in $ids; do session_action "$REC" DELETE "$i" "pre-start failure (exit $rc)"
    c=$(api_code DELETE "$API/pods/$i" "$REC/predelete-$i.response"); log "pre-start delete pod $i: HTTP $c"; done
  for i in 1 2 3 4 5 6 7 8 9 10 11 12; do
    n=$(list_pkm 2>/dev/null) || { sleep "$POLL_S"; continue; }
    if [ -z "$(jq -r --arg n "$NAME" 'select(.name == $n) | .id' <<<"$n")" ]; then gone=1; break; fi; sleep "$POLL_S"
  done
  local deleted; deleted=$(now)
  if [ -z "${ids// /}" ]; then
    log "no pod named $NAME was created (create failed: exit $rc); receipts kept aside, no attempt"
    jq -n --arg r "create failed, no pod found" '{class:"NOT_AN_ATTEMPT",rule:"P1",reason:$r}' > "$REC/outcome.json"
    local nc; nc="$REC.nocreate-$(date -u +%Y%m%dT%H%M%SZ)"; rpc_cleanup; mv "$REC" "$nc"; remind "${nc#"$here/"}"; exit 4
  fi
  local p cr s usd; p=$(jq -r '.costPerHr // empty' "$REC/pod-create.json" 2>/dev/null || true); p=${p:-$price}
  cr=$(jq -r '.createdAt // empty' "$REC/pod-create.json" 2>/dev/null | sed 's/ UTC$//' || true)
  s=$(( $(epoch "$deleted") - $(epoch "${cr:-$deleted}") )); [ "$s" -lt 60 ] && s=60
  usd=$(awk -v p="$p" -v s="$s" 'BEGIN{printf "%.4f", p * s / 3600}')
  jq -n --arg ids "$ids" --arg d "$deleted" --argjson s "$s" --argjson p "$p" --argjson u "$usd" --argjson g "$gone" --argjson rc "$rc" \
    '{pods:($ids|split(" ")|map(select(length>0))),deleted:$d,seconds_at_least:$s,costPerHr:$p,usd_derived:$u,reason:"pre-start failure",exit:$rc,confirmed_gone:($g==1)}' > "$REC/cost.json"
  jq -n --argjson rc "$rc" '{class:"NOT_AN_ATTEMPT",rule:"P1",reason:"failure before the runner started",exit:$rc,ended_by_signal:($rc >= 128)}' > "$REC/outcome.json"
  local k=1; while [ -e "$REC.prestart-$k" ]; do k=$((k + 1)); done
  # S7: the ledger row is closed only when the deletion is confirmed; otherwise it stays open at the worst case.
  if [ "$gone" = 1 ]; then ledger_close "$NAME/${id:-${ids// /}}" "$usd" "${REC#"$here/"}.prestart-$k/cost.json" 2>/dev/null || true
  elif [ -z "$id" ]; then ledger_add "$NAME/${ids// /}" "$(awk -v p="$p" -v h=$WORST_HOURS 'BEGIN{printf "%.4f", p * h}')" "${REC#"$here/"}.prestart-$k (deletion not confirmed)"; fi
  log "pre-start failure (exit $rc): pod(s) $ids deleted, confirmed gone: $gone; receipts moved to $REC.prestart-$k (not an attempt)"
  rpc_cleanup; mv "$REC" "$REC.prestart-$k"; remind "${REC#"$here/"}.prestart-$k"
  [ "$gone" = 1 ] || exit 11
}

# 7. Receipts directory (atomic: mkdir fails if it appeared meanwhile), token and create body in files (S4).
mkdir -p "$(dirname "$REC")"; mkdir "$REC" || refuse "receipts directory $REC appeared"
created_rec=1; cat "$STAGELOG" > "$REC/run.log"; rm -f "$STAGELOG"; STAGELOG="$REC/run.log"
trap precleanup EXIT
echo "$price_line" > "$REC/price.jsonl"
tokf="$RPC_STATE/$NAME.token" bodyf="$RPC_STATE/$NAME.body"
( umask 077; od -An -tx1 -N24 /dev/urandom | tr -d ' \n' > "$tokf" )
# The pod's start command (decision D4): copy the cgroup memory files (v2, or v1 if that is what the pod has) into
# /results/cgroup every 30 s and once after the runner is done, then exec the unchanged runner.
read -r -d '' BOOT <<'EOF' || true
mkdir -p /results/cgroup
cgsnap() {
  for f in memory.max memory.peak memory.events memory/memory.limit_in_bytes memory/memory.max_usage_in_bytes memory/memory.oom_control memory/memory.failcnt; do
    if [ -r "/sys/fs/cgroup/$f" ]; then
      cat "/sys/fs/cgroup/$f" > /tmp/cgsnap.tmp 2>/dev/null && mv -f /tmp/cgsnap.tmp "/results/cgroup/$(echo "$f" | tr / _)"
    fi
  done
  date -u +%FT%TZ > /tmp/cgsnap.tmp && mv -f /tmp/cgsnap.tmp /results/cgroup/snapshot-at
}
( cat /proc/self/cgroup > /results/cgroup/proc-self-cgroup 2>/dev/null
  stat -fc %T /sys/fs/cgroup > /results/cgroup/fstype 2>/dev/null
  n=0
  while :; do
    if [ -e /results/runner_state.json ]; then cgsnap; date -u +%FT%TZ > /results/cgroup/final-at; exit 0; fi
    if [ $((n % 6)) = 0 ]; then cgsnap; fi
    n=$((n + 1)); sleep 5
  done ) &
exec python3 -u -c "import base64;exec(base64.b64decode('@SERVER_B64@'))"
EOF
boot=${BOOT//@SERVER_B64@/$(base64 -w0 "$SERVER")}
( umask 077; jq -n --arg n "$NAME" --argjson v $VCPU --rawfile t "$tokf" --arg b "$boot" --arg fl "$FLAVOR" '{name:$n,computeType:"CPU",
  cloudType:"SECURE",cpuFlavorIds:[$fl],cpuFlavorPriority:"custom",vcpuCount:$v,interruptible:false,
  imageName:"python:3.12-slim",containerDiskInGb:20,ports:["8000/http"],env:{RUNNER_TOKEN:$t},
  dockerEntrypoint:["bash","-c"],dockerStartCmd:[$b]}' > "$bodyf" )
jq --arg h "$(printf '%s' "${BOOT//@SERVER_B64@/<server.py base64>}" | sha256sum | cut -d' ' -f1)" \
  'del(.env, .dockerStartCmd) + {env:{RUNNER_TOKEN:"<withheld>"}, dockerStartCmd:["<boot script: cgroup copy loop, then server.py; template sha256 \($h)>"]}' \
  "$bodyf" > "$REC/pod-request.json"
printf '%s\n' "${BOOT//@SERVER_B64@/<server.py base64>}" > "$REC/boot-template.sh"

# 8. Create (body from a 0600 file; the key from the 0600 header file).
log "create: $INSTANCE price $price/h, worst case US\$$worst"
out=$(api -X POST "$API/pods" --data-binary @"$bodyf" 2>>"$REC/run.log" || true); rm -f "$bodyf"
id=$(jq -r '.id // empty' <<<"$out" 2>/dev/null || true)
[ -n "$id" ] || { log "create failed or timed out: $(jq -c 'del(.env?, .dockerStartCmd?)' <<<"$out" 2>/dev/null || head -c 500 <<<"$out")"; exit 4; }
cp "$tokf" "$RPC_STATE/$id.token"
redact <<<"$out" > "$REC/pod-create.json"
ledger_add "$NAME/$id" "$(awk -v p="$(jq -r '.costPerHr // 0' "$REC/pod-create.json")" -v h=$WORST_HOURS 'BEGIN{printf "%.4f", (p > 0 ? p : 1) * h}')" "${REC#"$here/"}"
log "created pod $id ($NAME) at $(jq -r .createdAt "$REC/pod-create.json")"

# 9. Reserved tier, flavor, size and price from the create response (decision D7), the request from pod-request.json.
jq -e --argjson p "$price" --argjson v $VCPU --argjson m "$floor" --arg f "$FLAVOR" \
  '.machine.secureCloud == true and ((.machine.dataCenterId // "") | length > 0) and .cpuFlavorId == $f
   and .vcpuCount == $v and .memoryInGb >= $m and .costPerHr == $p and (.interruptible == false or .interruptible == null)' \
  "$REC/pod-create.json" >/dev/null \
  || { log "REFUSED pod $(jq -c '{cpuFlavorId,vcpuCount,memoryInGb,costPerHr,interruptible,machine}' "$REC/pod-create.json") (want $FLAVOR, $VCPU vCPU, >= $floor GB, costPerHr $price, reserved tier)"; exit 4; }
jq -e --arg f "$FLAVOR" '.interruptible == false and .cloudType == "SECURE" and .cpuFlavorIds == [$f]' "$REC/pod-request.json" >/dev/null \
  || { log "REFUSED request record"; exit 4; }
c=$(api_code GET "$API/pods/$id" "$REC/pod-record.raw")
[ "$c" = 200 ] && redact < "$REC/pod-record.raw" > "$REC/pod-record.json"; rm -f "$REC/pod-record.raw" "$REC/pod-record.raw.err"
jq -e --argjson p "$price" '.costPerHr == $p' "$REC/pod-record.json" >/dev/null 2>&1 || { log "REFUSED pod record (HTTP $c)"; exit 4; }
log "pod ok $(jq -c '{id,cpuFlavorId,costPerHr,vcpuCount,memoryInGb,dc:.machine.dataCenterId,secureCloud:.machine.secureCloud,machineId,lastStartedAt}' "$REC/pod-create.json")"

# 10. Wait for the runner (15 min), upload with rp_pinned.sh (2 MB chunks), compare each pod-side SHA-256.
for i in $(seq 1 90); do
  timeout 60 "$RP" status "$id" > "$REC/status-ready.json" 2>/dev/null && jq -e '.state == "idle"' "$REC/status-ready.json" >/dev/null 2>&1 && break
  [ "$i" = 90 ] && { log "runner not ready after 15 min"; exit 5; }; sleep "$POLL_S"
done
log "runner ready $(jq -c '{nproc,meminfo}' "$REC/status-ready.json")"
timeout 3600 "$RP" upload "$id" "$upd" > "$REC/upload.jsonl" || { log "upload failed"; exit 5; }
for f in pkm_tower_pilot prime_fall_degree_tools hky_fp_tower_check pfd_grid_zeros entry_r2.sh run.sh build-info.json; do
  want=$(sha256sum "$upd/$f" | cut -d' ' -f1)
  jq -e --arg f "$f" --arg w "$want" 'select(.name == $f) | .sha256 == $w' "$REC/upload.jsonl" >/dev/null \
    || { log "upload sha256 mismatch or missing: $f"; exit 5; }
done
log "pod-side sha256 of the 7 uploaded files equal the local ones"

# 10b. The pod's cgroup memory limit, read before start (S4): the boot loop has copied it into /results/cgroup, which
#      holds nothing else yet. It must be "max" or at least 112 GiB, so that an OOM kill here means what it means at
#      Amendment 14's -m112 size. Otherwise the pod is deleted before start (the case goes to the owner).
mm=""
for i in 1 2 3; do
  pre="$RPC_STATE/$NAME.precheck"; rm -rf "$pre"; mkdir -p "$pre"
  if timeout 120 "$RP" fetch "$id" "$pre/p.tar.gz" >/dev/null 2>&1 && tar -xzf "$pre/p.tar.gz" -C "$pre" 2>/dev/null; then
    for f in memory.max memory_memory.limit_in_bytes; do [ -z "$mm" ] && [ -s "$pre/results/cgroup/$f" ] && mm=$(tr -d '\n' < "$pre/results/cgroup/$f") && mmsrc=$f; done
    find "$pre/results" -type f -printf '%P\n' | LC_ALL=C sort > "$REC/prestart-results-files.txt"
  fi
  rm -rf "$pre"; [ -n "$mm" ] && break; sleep "$POLL_S"
done
jq -n --arg v "$mm" --arg s "${mmsrc:-none}" '{cgroup_memory_limit:$v, source:$s}' > "$REC/cgroup-prestart.json"
{ [ "$mm" = max ] || { [[ $mm =~ ^[0-9]+$ ]] && [ "$mm" -ge "$M112_LIMIT_BYTES" ]; }; } \
  || { log "REFUSED cgroup memory limit '${mm:-unobservable}' (${mmsrc:-no file}): must be max or >= $M112_LIMIT_BYTES bytes; owner"; exit 4; }
log "cgroup memory limit $mm ($mmsrc)"

# 11. Deadlines written before start (S6, decision D3), then start.
cr=$(jq -r '.createdAt' "$REC/pod-create.json" | sed 's/ UTC$//'); ce=$(epoch "$cr")
[ $(( $(date +%s) - ce )) -le "$START_WITHIN_S" ] || { log "more than $START_WITHIN_S s since creation; not starting (start + 25 h must be <= creation + 26 h)"; exit 5; }
jq -n --arg id "$id" --arg n "$NAME" --arg s "$STAGE" --arg c "$(date -u -d "@$ce" +%FT%TZ)" \
  --arg h "$(date -u -d "@$((ce + HARD_S))" +%FT%TZ)" --arg p "$(date -u -d "@$(( $(date +%s) + DEADLINE_S ))" +%FT%TZ)" \
  --argjson fl "$floor" '{id:$id,name:$n,stage:$s,created:$c,hard_deadline:$h,run_deadline_provisional:$p,
   run_deadline_rule:"runner start (start.json .started) + 90000 s", mem_floor_gb:$fl}' > "$REC/pod-state.json"
timeout 120 "$RP" start "$id" "$(jq -nc --arg s "$STAGE" '[$s]')" > "$REC/start.json" || true
jq -e '.state == "running" and (.started | type == "string")' "$REC/start.json" >/dev/null 2>&1 \
  || { log "start refused or failed: $(head -c 500 "$REC/start.json")"; exit 5; }
started=1
dl=$(date -u -d "@$(( $(epoch "$(jq -r .started "$REC/start.json")") + DEADLINE_S ))" +%FT%TZ)
c=$(api_code GET "$API/pods/$id" "$REC/pod-record-start.raw")
[ "$c" = 200 ] && redact < "$REC/pod-record-start.raw" > "$REC/pod-record-start.json"; rm -f "$REC/pod-record-start.raw" "$REC/pod-record-start.raw.err"
log "STARTED $NAME pod $id stage $STAGE; runner start $(jq -r .started "$REC/start.json"); run deadline $dl; hard deadline $(jq -r .hard_deadline "$REC/pod-state.json"); check in with fetch.sh $NAME"
