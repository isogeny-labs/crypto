#!/usr/bin/env bash
# run_tests.sh [--write] -- offline tests of the capped RunPod route (Amendment 33 draft, REST v2). No pod, no network:
# every curl call goes to tests/curl_stub.sh (first on PATH; it emulates REST v2 and the pod proxy and has no route for
# v1 or GraphQL), HTTPS_PROXY points at a dead port, and the "deny" cases fail every call. Each case runs in a fresh
# sandbox copy of the route (its own git repository with a local bare "origin", receipts, ledger, RPC_STATE and stub
# state) under $TMPDIR. A plain run writes nothing in the repository; --write replaces tests/out/, tests/results.tsv and
# tests/README.md with this run's outputs.
# The launch, fetch and some reaper and refusal cases need the image layer: RPC_LAYER_SRC (the layer tarball, SHA-256
# a0f7e28e...) and RPC_BIN_SRC (its usr/local/bin). Without them those cases print SKIP-NEEDS-LAYER and are counted.
set -u
T=$(cd "$(dirname "$0")" && pwd); RC=$(cd "$T/.." && pwd); STUDY=$(cd "$RC/.." && pwd)
WRITE=0; [ "${1:-}" = --write ] && WRITE=1
SCR=/tmp/claude-0/-home-user/354dbe26-6a93-506a-a404-2fc6ef137fdc/scratchpad
RPC_BIN_SRC=${RPC_BIN_SRC:-$SCR/r2g_layer/usr/local/bin}
RPC_LAYER_SRC=${RPC_LAYER_SRC:-$SCR/r2g_dl/layer.tgz}
WORK=$(mktemp -d "${TMPDIR:-/tmp}/rpc-tests.XXXX")
OUTD="$WORK/out"; mkdir -p "$OUTD"; RES="$WORK/results.tsv"; printf 'id\tresult\texpected\tobserved\tcase\n' > "$RES"
npass=0 nfail=0 nskip=0
HAVE_LAYER=0
if [ -f "$RPC_LAYER_SRC" ] && [ "$(sha256sum "$RPC_LAYER_SRC" | cut -d' ' -f1)" = a0f7e28eb0759ddefc4b906974866d2a2215d4705e25e9fd1fca45513b8b1971 ] \
   && [ -f "$RPC_BIN_SRC/pkm_tower_pilot" ]; then HAVE_LAYER=1; fi
echo "image layer available: $HAVE_LAYER"
BASE=pkm-x-t14-k0-c7-m112 R1=pkm-x-t14-k0-c7-m112-r1 STG=cap:14:0:7:27900000000

rec() { # rec ID OK(0|1) EXPECTED OBSERVED DESCRIPTION
  local r=PASS; [ "$2" = 0 ] || r=FAIL
  [ "$r" = PASS ] && npass=$((npass + 1)) || nfail=$((nfail + 1))
  printf '%s\t%s\t%s\t%s\t%s\n' "$1" "$r" "$3" "$4" "$5" >> "$RES"; echo "$r $1: $5 (expected $3, observed $4)"
}
skip() { local i; for i in "$@"; do nskip=$((nskip + 1)); printf '%s\tSKIP-NEEDS-LAYER\t-\t-\tneeds the image layer\n' "$i" >> "$RES"; echo "SKIP-NEEDS-LAYER $i"; done; }
iso() { date -u -d "@$1" +%FT%TZ; }
epoch() { date -u -d "$1" +%s; }
isod() { date -u -d "@$1" '+%Y-%m-%dT%H:%M:%S.123Z'; }
G() { git -C "$SB/repo" -c user.email=t@t -c user.name=t -c push.negotiate=false "$@"; }

mksb() { # fresh sandbox; sets SB, R (route dir), ST (stub dir), and the environment of the scripts
  SB=$(mktemp -d "$WORK/sb.XXXX"); local s="$SB/repo/research/pkm_falldegree_unblock_20261107"
  R="$s/runpod_capped"; mkdir -p "$R" "$SB/repo/research/prime_fall_degree_20261027"
  cp -a "$RC"/{common.sh,run.sh,fetch.sh,reap.sh,rp_pinned.sh,server_pinned.py,ledger.tsv,make_requests_m112.sh,requests_m112,.gitignore} "$R/"
  cp -a "$STUDY/image" "$STUDY/requests_r2i" "$s/"
  cp -a "$STUDY/../prime_fall_degree_20261027/PREREGISTRATION.md" "$STUDY/../prime_fall_degree_20261027/run.sh" "$SB/repo/research/prime_fall_degree_20261027/"
  git init -q -b main "$SB/repo" && G add -A && G commit -qm sandbox
  git init -q --bare "$SB/origin.git" && G remote add origin "$SB/origin.git" && G push -q origin main 2>/dev/null
  ST="$SB/stub"; mkdir -p "$ST" "$SB/bin" "$SB/state" "$ST/pretree/cgroup"; cp "$T/curl_stub.sh" "$SB/bin/curl"
  echo 128000000000 > "$ST/pretree/cgroup/memory.max"     # what the second smoke pod's boot loop copied before start
  jq -n '{cost:1.04, cloud:"SECURE", cpu:{id:"cpu5m", vcpuCount:16, memory:128}, dataCenterId:"EU-RO-1", disk:20,
          locked:false, mounts:{}, globalNetworking:{enabled:false}, registry:null, template:null, cudaVersion:null,
          runtime:null, ssh:{proxy:"stub"}}' > "$ST/create-template.json"
  echo '[]' > "$ST/pods.json"
  export STUB_DIR="$ST" PATH="$SB/bin:$ORIGPATH" RPC_STATE="$SB/state" RPC_BIN="$RPC_BIN_SRC" RPC_LAYER="$RPC_LAYER_SRC"
  export RUNPOD_API_KEY=dummy-test-key RPC_POLL_S=0 HTTPS_PROXY=http://127.0.0.1:9 https_proxy=http://127.0.0.1:9
  unset SHA_FILE STUB_POD_ID STUB_CREATED_AGO STUB_CREATE_CODE
}
PRE_SB() { echo "$SB/repo/research/prime_fall_degree_20261027/PREREGISTRATION.md"; }
fresh_ledger() { sed -i "s/^# complete_as_of\t.*/# complete_as_of\t$(date -u +%FT%TZ)/" "$R/ledger.tsv"; }
freeze_heading() { printf '\n## Amendment 33 (test sandbox only)\n' >> "$(PRE_SB)"; }
freeze() { freeze_heading; (cd "$R" && sha256sum common.sh run.sh fetch.sh reap.sh) >> "$(PRE_SB)"; }
study_ready() { fresh_ledger; freeze; }
run() { ( cd "$R" && "$@" ) > "$OUTD/$ID.txt" 2>&1; echo $?; }
calls() { if [ -f "$ST/calls.log" ]; then wc -l < "$ST/calls.log"; else echo 0; fi; }
creates() { if [ -f "$ST/calls.log" ]; then grep -c -- '-X POST https://api.runpod.io/v2/pods ' "$ST/calls.log" || true; else echo 0; fi; }
outcome() { # outcome NAME CLASS [OVERSIZE] [RELAUNCH auto|owner]
  mkdir -p "$R/receipts/$1"; jq -n --arg c "$2" --argjson o "${3:-false}" --arg r "${4:-auto}" \
    '{class:$c, row:{oversize:$o, timed_out:false}} + (if $c == "LOST" then {relaunch:$r} else {} end)' > "$R/receipts/$1/outcome.json"; }
ORIGPATH=$PATH

# ===================================================================== A. refusals before any network call (deny mode)
deny_case() { # deny_case ID DESCRIPTION SETUP-FUNCTION NAME STAGE [EXPECT]
  ID=$1; mksb; touch "$ST/deny"; "$3"; local rc; rc=$(run ./run.sh "$4" "$5"); local want=${6:-3}
  local ok=1; [ "$rc" = "$want" ] && [ "$(calls)" = 0 ] && ok=0
  rec "$ID" $ok "exit $want, 0 API calls" "exit $rc, $(calls) API calls" "$2"
}
s_none() { :; }
s_ready() { study_ready; }
s_shafile() { study_ready; export SHA_FILE=/tmp/x; }
s_recexists() { study_ready; mkdir -p "$R/receipts/$BASE"; }
s_outexists() { study_ready; mkdir -p "$R/results/$BASE"; }
s_admitted() { study_ready; outcome $BASE ADMITTED false; }
s_lostowner() { study_ready; outcome $BASE LOST false owner; }
s_inflight() { study_ready; mkdir -p "$R/receipts/$BASE"; }
s_aborted() { study_ready; outcome $BASE ABORTED; }
s_nofreeze() { fresh_ledger; }
s_nohashes() { fresh_ledger; freeze_heading; }
s_rpmod() { study_ready; echo '# x' >> "$R/rp_pinned.sh"; }
s_srvmod() { study_ready; echo '# x' >> "$R/server_pinned.py"; }
s_binmod() { study_ready; local b="$SB/bin2"; cp -a "$RPC_BIN_SRC" "$b"; printf 'x' >> "$b/pfd_grid_zeros"; export RPC_BIN="$b"; }
s_nolayer() { study_ready; unset RPC_LAYER; }
s_stale() { freeze; sed -i "s/^# complete_as_of\t.*/# complete_as_of\t$(date -u -d "-25 hours" +%FT%TZ)/" "$R/ledger.tsv"; }
s_malformed() { study_ready; printf 'broken line\n' >> "$R/ledger.tsv"; }
s_stateinside() { study_ready; export RPC_STATE="$R/state-inside"; }
s_prestart2() { study_ready; mkdir -p "$R/receipts/$BASE.prestart-1" "$R/receipts/$BASE.prestart-2"; }
s_reqedit() { study_ready; jq '.job.seconds = 90001' "$R/requests_m112/pfd-x-t14-k0-c7-m112.json" > "$SB/x" && mv "$SB/x" "$R/requests_m112/pfd-x-t14-k0-c7-m112.json"; }
s_reqedit_pin() { # the request edited AND its pin updated: the re-derivation catches it
  jq '.job.seconds = 90001' "$R/requests_m112/pfd-x-t14-k0-c7-m112.json" > "$SB/x" && mv "$SB/x" "$R/requests_m112/pfd-x-t14-k0-c7-m112.json"
  sed -i "s/^STUDY_REQUEST_SHA=[0-9a-f]*/STUDY_REQUEST_SHA=$(sha256sum "$R/requests_m112/pfd-x-t14-k0-c7-m112.json" | cut -d' ' -f1)/" "$R/common.sh"; study_ready; }
s_r1lost() { study_ready; outcome $BASE LOST; outcome $R1 LOST; }
s_ledgerrow() { study_ready; printf '%s/oldpod\trunpod_capped\tAmendment 33\tclosed\t1.0000\treceipts/%s\n' $BASE $BASE >> "$R/ledger.tsv"; }
s_imgdisagree() { study_ready; jq '.layer.digest = "sha256:0000000000000000000000000000000000000000000000000000000000000000"' "$R/../image/image.json" > "$SB/x" && mv "$SB/x" "$R/../image/image.json"; }

deny_case A01 "a draft Amendment 18 name, no longer on the allow-list (pkm-x-t14-k0-c6)" s_ready pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A02 "a second relaunch name (-r2)" s_ready $BASE-r2 $STG
deny_case A03 "SHA_FILE set" s_shafile $BASE $STG
deny_case A04 "receipts directory for the name already exists" s_recexists $BASE $STG
deny_case A05 "results directory for the name already exists" s_outexists $BASE $STG
deny_case A06 "order: -r1 requested with no base run" s_ready $R1 $STG
deny_case A07 "order: -r1 requested when the base is ADMITTED" s_admitted $R1 $STG
deny_case A08 "order: -r1 requested when the base is LOST with relaunch owner" s_lostowner $R1 $STG
deny_case A09 "order: -r1 requested while the base is in flight (receipts, no outcome)" s_inflight $R1 $STG
deny_case A10 "stage cap 6 at -m112 for the base name (only cap:14:0:7:27900000000)" s_ready $BASE cap:14:0:6:27900000000
deny_case A11 "stage cap 7 at the 56Gi --max-dense for the base name" s_ready $BASE cap:14:0:7:12800000000
deny_case A12 "'## Amendment 33' not in PREREGISTRATION.md" s_nofreeze $BASE $STG
deny_case A13 "rp_pinned.sh modified (one line appended)" s_rpmod $BASE $STG
deny_case A14 "server_pinned.py modified (one line appended)" s_srvmod $BASE $STG
if [ "$HAVE_LAYER" = 1 ]; then
deny_case A15 "pfd_grid_zeros with one byte appended" s_binmod $BASE $STG
deny_case A16 "RPC_LAYER missing for a study name" s_nolayer $BASE $STG
deny_case A17 "ledger complete_as_of 25 h old (stale)" s_stale $BASE $STG
deny_case A18 "ledger with a malformed row" s_malformed $BASE $STG
deny_case A19 "RPC_STATE inside the git worktree" s_stateinside $BASE $STG
else skip A15 A16 A17 A18 A19; fi
deny_case A20 "development stage with t = 12" s_none pkm-x-dev-a cap:12:0:5:12800000000
deny_case A21 "a name with two earlier pre-start failures" s_prestart2 $BASE $STG
deny_case A22 "request file edited (seconds 90001): sha256 differs from the pin" s_reqedit $BASE $STG
deny_case A22b "request file edited and the pin updated: differs from the derivation (Amendments 7 B, 14)" s_reqedit_pin $BASE $STG
deny_case A23 "order: -r1 after the -r1 was lost (no third attempt)" s_r1lost $R1 $STG
deny_case A24 "N1: -r1 after an ABORTED base (the attempt is used)" s_aborted $R1 $STG
ID=A25; mksb; touch "$ST/deny"; mkdir -p "$R/receipts_dev/pkm-x-dev-smoke1"; rc=$(run ./run.sh pkm-x-dev-smoke1 cap:8:0:5:12800000000)
[ "$rc" = 3 ] && [ "$(calls)" = 0 ]; rec A25 $? "exit 3, 0 API calls" "exit $rc, $(calls) API calls" "dev name with an existing receipts_dev dir"
deny_case A26 "N6: the name already has a ledger row (receipts lost)" s_ledgerrow $BASE $STG
deny_case A27 "S3: heading present but the route scripts' hashes are not in the preregistration" s_nohashes $BASE $STG
deny_case A28 "S3: image.json layer digest disagrees with the pin in common.sh" s_imgdisagree $BASE $STG
deny_case A29 "development stage with --max-dense 999" s_none pkm-x-dev-a cap:8:0:5:999
deny_case A30 "a draft Amendment 18 name (pkm-x-t14-k1-c6)" s_ready pkm-x-t14-k1-c6 cap:14:1:6:12800000000
deny_case A31 "a draft Amendment 18 name (pkm-x-t13-k0-c6-r3)" s_ready pkm-x-t13-k0-c6-r3 cap:13:0:6:12800000000
deny_case A32 "draw 1 of the same cap (not this route's cell)" s_ready pkm-x-t14-k1-c7-m112 cap:14:1:7:27900000000

# ===================================================================== B, C, F helpers (need the layer)
mktree() { # mktree DIR STAGE STARTED ENDED PILOT_EXIT GZ_EXIT OOM_KILL|none [START_LINES] [CG_PEAK_BYTES|v1:BYTES|-] [LOG: keep|strip|MiB] [OVERSIZE]
  local d=$1 st=$2 s0=$3 en=$4 pe=$5 ge=$6 ok=$7 n=${8:-1} cgpk=${9:--} lg=${10:-keep} os=${11:-false} t k c
  IFS=: read -r _ t k c _ <<<"$st"; local cell="X-m2-t$t-k$k-c$c"
  rm -rf "$d"; mkdir -p "$(dirname "$d")"; cp -a "$T/fixtures/smoke_tree" "$d"
  sed -i "s/^argv: .*/argv: $st/" "$d/host.txt"
  [ "$cell" = X-m2-t8-k0-c5 ] || for f in "$d"/runs/X-m2-t8-k0-c5*; do mv "$f" "${f/X-m2-t8-k0-c5/$cell}"; done
  [ "$os" = true ] && { jq -c 'if has("oversize") then .oversize = true else . end' "$d/runs/$cell.jsonl" > "$d/x" && mv "$d/x" "$d/runs/$cell.jsonl"; }
  { for i in $(seq 1 "$n"); do echo "$s0 start $cell"; done; echo "$en end $cell exit $pe"; echo "$en end $cell.gridzeros exit $ge"; } > "$d/progress.txt"
  local rx=0; [ "$pe" = 0 ] && [ "$ge" = 0 ] || rx=1
  jq -nc --arg s "$s0" --arg e "$en" --arg st "$st" --argjson x $rx '{state:"done",exit:$x,started:$s,ended:$e,stages:[$st]}' > "$d/runner_state.json"
  jq --arg st "$st" '.stages = [$st]' "$d/runner_request.json" > "$d/rr" && mv "$d/rr" "$d/runner_request.json"
  if [ "$ok" != none ]; then mkdir -p "$d/cgroup"; echo 128000000000 > "$d/cgroup/memory.max"; printf 'low 0\nhigh 0\nmax 0\noom %s\noom_kill %s\n' "$ok" "$ok" > "$d/cgroup/memory.events"; echo "$en" > "$d/cgroup/final-at"; fi
  case $cgpk in -) ;; v1:*) mkdir -p "$d/cgroup"; echo "${cgpk#v1:}" > "$d/cgroup/memory_memory.max_usage_in_bytes" ;; *) mkdir -p "$d/cgroup"; echo "$cgpk" > "$d/cgroup/memory.peak" ;; esac
  case $lg in keep) ;; strip) sed -i -E 's/, memory [0-9]+ MB \(peak [0-9]+ MB\)//' "$d/runs/$cell.log" ;;
    *) echo "tower step 99: degree 6, synthetic test line, memory 100 MB (peak $lg MB)" >> "$d/runs/$cell.log" ;; esac
}
status_done() { jq -nc --arg s "$1" --arg e "$2" --argjson x "$3" '{state:"done",exit:$x,started:$s,ended:$e,stages:["cap:8:0:5:12800000000"],progress:"",meminfo:[],nproc:48}' > "$ST/status.json"; }
fetch() { ( cd "$R" && ./fetch.sh "$@" ) >> "$OUTD/$ID.txt" 2>&1; echo $?; }
cls() { jq -r '.class + " " + .rule' "$1/outcome.json" 2>/dev/null || echo none; }
rl() { jq -r '.relaunch // "-"' "$1/outcome.json" 2>/dev/null || echo none; }
pods() { jq length "$ST/pods.json"; }
hard_past() { jq --arg h "$(iso $(( $(date +%s) - 60 )))" '.hard_deadline = $h' "$D/pod-state.json" > "$SB/x" && mv "$SB/x" "$D/pod-state.json"; }
rest_set() { jq "$1" "$ST/pod-stubpod1.json.rec" > "$SB/x" && mv "$SB/x" "$ST/pod-stubpod1.json.rec"; }
# start a dev run, then move its runner start back by $2 seconds in the receipts and the stub (deadline tests)
startrun() { ID=$1; mksb; run ./run.sh pkm-x-dev-c cap:8:0:5:12800000000 >/dev/null; D="$R/receipts_dev/pkm-x-dev-c"; RD="$R/results/pkm-x-dev-c"
  S0=$(jq -r .started "$D/start.json")
  if [ "${2:-0}" != 0 ]; then S0=$(iso $(( $(date +%s) - $2 ))); jq --arg s "$S0" '.started = $s' "$D/start.json" > "$SB/x" && mv "$SB/x" "$D/start.json"
    jq --arg s "$S0" '.started = $s' "$ST/status.json" > "$SB/x" && mv "$SB/x" "$ST/status.json"; fi; }

if [ "$HAVE_LAYER" = 1 ]; then
# ===================================================================== B. launch paths against the stub
ID=B01; mksb; study_ready; jq -n --arg c "$(isod $(( $(date +%s) - 600 )))" '[{id:"other1",name:"pkm-x-dev-zzz",status:"RUNNING",cost:1.04,createdAt:$c}]' > "$ST/pods.json"
rc=$(run ./run.sh $BASE $STG); [ "$rc" = 3 ] && [ "$(creates)" = 0 ] && [ ! -d "$R/receipts/$BASE" ]
rec B01 $? "exit 3, no create" "exit $rc, creates $(creates)" "a pkm-x- pod is live (serial rule)"
ID=B02; mksb; study_ready; echo '{"availability":"NONE","id":"cpu5m","price":{"securePerVcpu":0.065,"serverlessPerVcpu":0},"ramGbPerVcpu":8,"vcpu":{"max":32,"min":2}}' > "$ST/catalog.json"
rc=$(run ./run.sh $BASE $STG); [ "$rc" = 10 ] && [ "$(creates)" = 0 ] && [ ! -d "$R/receipts/$BASE" ]
rec B02 $? "exit 10, no create, no receipts dir" "exit $rc, creates $(creates), dir $([ -d "$R/receipts/$BASE" ] && echo yes || echo no)" "catalog availability NONE for cpu5m at 16 vCPU: no attempt"
ID=B03; mksb; study_ready; echo '{"availability":"HIGH","id":"cpu5m","price":{"securePerVcpu":0.084,"serverlessPerVcpu":0},"ramGbPerVcpu":8,"vcpu":{"max":32,"min":2}}' > "$ST/catalog.json"
rc=$(run ./run.sh $BASE $STG); [ "$rc" = 3 ] && [ "$(creates)" = 0 ] && grep -q 'cost guard' "$OUTD/B03.txt"
rec B03 $? "exit 3 (per-job guard), no create" "exit $rc, creates $(creates)" "price 0.084 x 16 = 1.344/h x 30 h = US\$40.32 > US\$40"
ID=B04; mksb; study_ready; printf 'test-row/x\ttest\ttest\tclosed\t13.0000\ttest\n' >> "$R/ledger.tsv"
rc=$(run ./run.sh $BASE $STG); [ "$rc" = 3 ] && [ "$(creates)" = 0 ] && grep -q 'study guard' "$OUTD/B04.txt"
rec B04 $? "exit 3 (study guard)" "exit $rc, creates $(creates)" "ledger US\$465.84 + 13.00 = 478.84, + worst case US\$31.20 = 510.04 > US\$510"
ID=B04b; mksb; study_ready; printf 'test-row/x\ttest\ttest\tclosed\t12.9000\ttest\n' >> "$R/ledger.tsv"
rc=$(run ./run.sh $BASE $STG); [ "$rc" = 0 ] && [ "$(creates)" = 1 ]
rec B04b $? "exit 0 (started)" "exit $rc, creates $(creates)" "ledger US\$478.74 + worst case US\$31.20 = 509.94 <= US\$510"
ID=B05; mksb; study_ready; echo '.cloud = "COMMUNITY"' > "$ST/create-response-patch.jq"
rc=$(run ./run.sh $BASE $STG)
[ "$rc" = 4 ] && grep -q 'DELETE https://api.runpod.io/v2/pods/stubpod1' "$ST/calls.log" && [ -d "$R/receipts/$BASE.prestart-1" ] && [ ! -e "$R/receipts/$BASE" ] \
  && [ "$(jq -r .class "$R/receipts/$BASE.prestart-1/outcome.json")" = NOT_AN_ATTEMPT ] && [ "$(pods)" = 0 ] \
  && grep -qP "^$BASE/stubpod1\trunpod_capped\t.*\tclosed\t" "$R/ledger.tsv" && grep -q '"action":"DELETE"' "$R/receipts/$BASE.prestart-1/session-actions.jsonl"
rec B05 $? "exit 4, delete logged then made, receipts -> .prestart-1, ledger row closed" "exit $rc, pods left $(pods)" "create response cloud COMMUNITY (not the SECURE tier)"
ID=B06; mksb; study_ready; echo '.cost = 0.98' > "$ST/create-response-patch.jq"
rc=$(run ./run.sh $BASE $STG); [ "$rc" = 4 ] && [ "$(pods)" = 0 ] && [ -d "$R/receipts/$BASE.prestart-1" ]
rec B06 $? "exit 4, pod deleted" "exit $rc, pods left $(pods)" "create response cost 0.98 differs from the catalog price (0.065 x 16 = 1.04)"
ID=B07; mksb; study_ready; touch "$ST/create-timeout"
rc=$(run ./run.sh $BASE $STG); [ "$rc" = 4 ] && [ "$(pods)" = 0 ] && [ -d "$R/receipts/$BASE.prestart-1" ] && grep -q 'pre-start delete pod stubpod1' "$OUTD/B07.txt"
rec B07 $? "exit 4, pod found by name and deleted" "exit $rc, pods left $(pods)" "create call timed out but the pod exists (trap set before create)"
ID=B08; mksb; study_ready; touch "$ST/create-error"
rc=$(run ./run.sh $BASE $STG); [ "$rc" = 4 ] && compgen -G "$R/receipts/$BASE.nocreate-*" >/dev/null && [ ! -e "$R/receipts/$BASE" ]
rec B08 $? "exit 4, receipts -> .nocreate-*, name free" "exit $rc" "create refused by the API (HTTP 400), no pod exists"
ID=B09; mksb; export STUB_CREATED_AGO=7200
rc=$(run ./run.sh pkm-x-dev-late cap:8:0:5:12800000000); [ "$rc" = 5 ] && [ -d "$R/receipts_dev/pkm-x-dev-late.prestart-1" ] && [ "$(pods)" = 0 ]
rec B09 $? "exit 5, pod deleted, not an attempt" "exit $rc" "runner would start more than 55 min after creation"
unset STUB_CREATED_AGO
ID=B10; mksb; rc=$(run ./run.sh pkm-x-dev-t1 cap:8:0:5:12800000000); D="$R/receipts_dev/pkm-x-dev-t1"; tok=$(cat "$SB/state/pkm-x-dev-t1.token")
ok=1; [ "$rc" = 0 ] && [ -f "$D/start.json" ] && [ -f "$D/pod-state.json" ] && jq -e '.run_deadline_provisional and .hard_deadline' "$D/pod-state.json" >/dev/null \
  && jq -e '.env.RUNNER_TOKEN == "<withheld>" and .cloud == "SECURE" and .cpu == {id:"cpu5m", vcpuCount:16} and .entrypoint == ["bash","-c"]
            and (has("interruptible") | not) and (.image | test("^python:3.12-slim@sha256:5e2bdb1b"))' "$D/pod-request.json" >/dev/null \
  && jq -e 'keys == ["cloud","cmd","cpu","disk","entrypoint","env","image","name","ports"] and (.env | keys) == ["RUNNER_TOKEN"]' "$ST/create-body.json" >/dev/null \
  && ! grep -rqF "$tok" "$D" && ! grep -F -- '-X POST https://api.runpod.io/v2/pods' "$ST/calls.log" | grep -qF "$tok" && ! grep -qF dummy-test-key "$ST/calls.log" \
  && ! grep -rqE '"(args|env|ssh)"' "$D"/pod-create.json "$D"/pod-record*.json \
  && jq -e '.cgroup_memory_limit == "128000000000"' "$D/cgroup-prestart.json" >/dev/null \
  && jq -e '.price == 1.04 and .availability == "HIGH" and .http == 200' "$D/price.jsonl" >/dev/null \
  && jq -e '.dataCenterId == null' "$D/pod-create.json" >/dev/null && jq -e '.dataCenterId == "EU-RO-1" and .status == "RUNNING"' "$D/pod-record-ready.json" >/dev/null \
  && grep -q 'cgsnap' "$D/boot-template.sh" && grep -qP '^pkm-x-dev-t1/stubpod1\t.*\topen\t31.2000' "$R/ledger.tsv" && grep -q 'COMMIT AND PUSH NOW' "$OUTD/B10.txt" \
  && ! grep -qE 'rest\.runpod\.io|graphql' "$ST/calls.log" && ok=0
rec B10 $ok "exit 0; v2 create body keys only; token and key absent from receipts and create argv; command and env dropped from saved records; cgroup limit read before start; data center read when ready; ledger row at 30 h; no v1 or GraphQL call" \
  "exit $rc; X-Token argv lines (rp_pinned.sh, disclosed): $(grep -cF "$tok" "$ST/calls.log")" "development launch end to end against the v2 stub"
cp "$ST/create-body.json" "$WORK/b10-create-body.json"
ID=B11; mksb; study_ready; rc=$(run ./run.sh $BASE $STG)
rc2=$( ( cd "$R" && ./run.sh $R1 $STG ) > "$OUTD/B11b.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$rc2" = 3 ] && grep -q "BLOCKED:$BASE" "$OUTD/B11b.txt" && grep -q 'RPC_BIN files are the files of image layer' "$OUTD/B11.txt" && grep -q 'git HEAD [0-9a-f]\{40\}' "$OUTD/B11.txt" \
  && grep -q 'cell request runpod_capped/requests_m112/pfd-x-t14-k0-c7-m112.json sha256 44e009f1' "$OUTD/B11.txt" \
  && jq -e --arg s "$STG" '.stages == [$s]' "$ST/start-body.json" >/dev/null
rec B11 $? "the study run starts with stage $STG and the pinned request (exit 0); its -r1 is refused while it is in flight (exit 3)" "exit $rc then $rc2" "study launch $BASE, then $R1"
ID=B12; jq -r '.cmd[0]' "$WORK/b10-create-body.json" > "$WORK/boot.sh"; bash -n "$WORK/boot.sh" > "$OUTD/B12.txt" 2>&1; r=$?
grep -q 'exec python3 -u -c "import base64;exec(base64.b64decode(' "$WORK/boot.sh"; r2=$?
b64=$(sed -n 's/.*b64decode(.\([A-Za-z0-9+\/=]*\).)).*/\1/p' "$WORK/boot.sh"); h=$(printf '%s' "$b64" | base64 -d | sha256sum | cut -d' ' -f1)
[ $r = 0 ] && [ $r2 = 0 ] && [ "$h" = f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550 ]
rec B12 $? "bash -n ok; embedded runner sha256 f56fd08e..." "bash -n $r; embedded sha256 ${h:0:8}" "the pod start command parses and carries server_pinned.py (= runpod/server.py at d2a16c723d)"
ID=B13; L="$WORK/bootloop"; mkdir -p "$L/cg" "$L/res"; printf 'max\n' > "$L/cg/memory.max"; printf 'low 0\nhigh 0\nmax 0\noom 1\noom_kill 1\n' > "$L/cg/memory.events"
sed -n '1,/^exec python3/p' "$WORK/boot.sh" | sed '$d' | sed "s#/sys/fs/cgroup#$L/cg#g; s#/results#$L/res#g; s#/tmp/cgsnap.tmp#$L/t.tmp#g" > "$L/loop.sh"
( bash "$L/loop.sh"; sleep 1; echo '{"state":"done"}' > "$L/res/runner_state.json"; sleep 7 ) > "$OUTD/B13.txt" 2>&1
[ -f "$L/res/cgroup/memory.max" ] && [ -f "$L/res/cgroup/memory.events" ] && [ -f "$L/res/cgroup/final-at" ] && grep -q 'oom_kill 1' "$L/res/cgroup/memory.events"
rec B13 $? "memory.max, memory.events and final-at copied" "$(ls "$L/res/cgroup" 2>/dev/null | tr '\n' ' ')" "the boot loop copies cgroup v2 files and writes its final copy after runner_state.json"
ID=B14; mksb; echo 60000000000 > "$ST/pretree/cgroup/memory.max"; rc=$(run ./run.sh pkm-x-dev-small cap:8:0:5:12800000000)
[ "$rc" = 4 ] && [ "$(pods)" = 0 ] && [ -d "$R/receipts_dev/pkm-x-dev-small.prestart-1" ] && ! grep -q '/start' "$ST/calls.log"
rec B14 $? "exit 4 before start, pod deleted" "exit $rc" "S4: cgroup memory.max 60,000,000,000 bytes (< 112 GiB)"
ID=B15; mksb; rm -f "$ST/pretree/cgroup/memory.max"; rc=$(run ./run.sh pkm-x-dev-nocg cap:8:0:5:12800000000)
[ "$rc" = 4 ] && [ "$(pods)" = 0 ] && ! grep -q '/start' "$ST/calls.log"
rec B15 $? "exit 4 before start, pod deleted" "exit $rc" "S4: no cgroup memory limit file before start (unobservable: owner)"
ID=B16; mksb; rc=$(run ./run.sh pkm-x-dev-r cap:8:0:5:12800000000); rm -rf "$R/receipts_dev/pkm-x-dev-r" "$SB/state/"*.token; echo '[]' > "$ST/pods.json"
rc2=$( (cd "$R" && ./run.sh pkm-x-dev-r cap:8:0:5:12800000000) > "$OUTD/B16b.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$rc2" = 3 ] && grep -q 'already has a row' "$OUTD/B16b.txt"
rec B16 $? "exit 0, then exit 3 (ledger row)" "exit $rc then $rc2" "N6: receipts and token lost after a launch, pod gone: the same name is refused"
ID=B17; mksb; echo '.cpu.memory = 64' > "$ST/record-patch.jq"; rc=$(run ./run.sh pkm-x-dev-mem cap:8:0:5:27900000000)
[ "$rc" = 4 ] && [ "$(pods)" = 0 ] && ! grep -q '/start' "$ST/calls.log" && grep -q 'REFUSED pod record' "$OUTD/B17.txt"
rec B17 $? "exit 4 before start, pod deleted" "exit $rc" "GET record cpu.memory 64 GB, below the 121 GB floor (112 GiB)"
ID=B18; mksb; echo '.dataCenterId = null' > "$ST/record-patch.jq"; rc=$(run ./run.sh pkm-x-dev-dc cap:8:0:5:12800000000)
[ "$rc" = 4 ] && [ "$(pods)" = 0 ] && ! grep -q '/upload' "$ST/calls.log" && grep -q 'REFUSED pod record when ready' "$OUTD/B18.txt"
rec B18 $? "exit 4 before upload, pod deleted" "exit $rc" "the record read when the runner answers has no data center id"
ID=B19; mksb; study_ready; jq -n --arg c "$(isod $(( $(date +%s) - 600 )))" '[{id:"pg2",name:"pkm-x-dev-p2",status:"RUNNING",cost:1.04,createdAt:$c}]' > "$ST/pods-page2.json"
rc=$(run ./run.sh $BASE $STG); [ "$rc" = 3 ] && [ "$(creates)" = 0 ] && grep -q 'data-urlencode cursor=c2' "$ST/calls.log" && grep -q 'pkm-x-dev-p2' "$OUTD/B19.txt"
rec B19 $? "exit 3, the second page was read" "exit $rc, creates $(creates)" "v2 list pagination: a live pkm-x- pod on page 2 (serial rule)"
ID=B20; mksb; export STUB_CREATE_CODE=202; rc=$(run ./run.sh pkm-x-dev-q cap:8:0:5:12800000000); unset STUB_CREATE_CODE
[ "$rc" = 4 ] && [ "$(pods)" = 0 ] && grep -q 'HTTP 202' "$OUTD/B20.txt"
rec B20 $? "exit 4, pod deleted" "exit $rc" "create answered 202 (queued) instead of 201: not accepted"
else skip B01 B02 B03 B04 B04b B05 B06 B07 B08 B09 B10 B11 B12 B13 B14 B15 B16 B17 B18 B19 B20; fi

if [ "$HAVE_LAYER" = 1 ]; then
# ===================================================================== C. fetch.sh flows against the stub
startrun C01; rc=$(fetch pkm-x-dev-c); [ "$rc" = 41 ] && [ "$(pods)" = 1 ] && [ "$(wc -l < "$D/checkins.jsonl")" = 1 ] && jq -e '.rest | has("status") and has("startedAt")' "$D/checkins.jsonl" >/dev/null
rec C01 $? "exit 41, pod kept, one check-in with v2 status and startedAt" "exit $rc" "running before the deadline"
startrun C02 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
rc=$(fetch pkm-x-dev-c); c1=$(cls "$D")
rc2=$(fetch pkm-x-dev-c --delete HEAD)                                  # nothing committed yet
G add -f "$RD/collect-1" "$D/outcome.json" "$D/local-sha256-1.txt" && G commit -qm kept; C=$(G rev-parse HEAD)
rc3=$(fetch pkm-x-dev-c --delete "$C")                                  # committed, not on origin
G push -q origin main; rc4=$(fetch pkm-x-dev-c --delete "$C"); rc5=$(fetch pkm-x-dev-c)
[ "$rc" = 8 ] && [ "$c1" = "ADMITTED P2d" ] && [ "$rc2" = 8 ] && [ "$rc3" = 8 ] && [ "$rc4" = 0 ] && [ "$rc5" = 9 ] && [ -f "$D/cost.json" ] && [ "$(pods)" = 0 ] \
  && grep -qP '^pkm-x-dev-c/stubpod1\t.*\tclosed\t' "$R/ledger.tsv" && [ -d "$RD/collect-1" ] && grep -q 'largest pilot memory peak' "$D/admission-lines-1.txt" \
  && grep -q 'on an origin branch head now: 0' "$D/git-gate.txt" && grep -q 'mismatches 0, on an origin branch head now: 1' "$D/git-gate.txt" \
  && jq -e '.classifier.version == "a33-r1" and (.classifier.fetch_sha256 | length) == 64' "$D/outcome.json" >/dev/null && jq -e '.cost_per_hr == 1.04' "$D/cost.json" >/dev/null
rec C02 $? "exit 8 ADMITTED; gate: uncommitted 8, committed but not pushed 8, pushed 0; re-run 9" "exit $rc $c1; $rc2; $rc3; $rc4; $rc5" "completed run: collect, the git gate (decision 22), delete, refuse re-collection"
startrun C02b 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
rc=$(fetch pkm-x-dev-c); G add -f "$RD/collect-1" "$D/outcome.json" "$D/local-sha256-1.txt" && G commit -qm kept && G push -q origin main; C=$(G rev-parse HEAD)
echo tampered >> "$RD/collect-1/host.txt"; rc2=$(fetch pkm-x-dev-c --delete "$C")
[ "$rc" = 8 ] && [ "$rc2" = 8 ] && [ "$(pods)" = 1 ] && grep -q 'manifest mismatch or missing: host.txt' "$D/git-gate.txt"
rec C02b $? "exit 8, pod kept" "exit $rc then $rc2" "git gate: a kept file changed after the commit (manifest mismatch)"
startrun C02c 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
rc=$(fetch pkm-x-dev-c); G add -f "$RD/collect-1" "$D/outcome.json" && G commit -qm partial && G push -q origin main; C=$(G rev-parse HEAD); rc2=$(fetch pkm-x-dev-c --delete "$C")
[ "$rc" = 8 ] && [ "$rc2" = 8 ] && [ "$(pods)" = 1 ] && grep -q "not in $C with this content: .*local-sha256-1.txt" "$D/git-gate.txt"
rec C02c $? "exit 8, pod kept" "exit $rc then $rc2" "git gate: the manifest receipt local-sha256-1.txt is not in the pushed commit"
startrun C03; rc=$(fetch pkm-x-dev-c --abort); [ "$rc" = 0 ] && [ "$(cls "$D")" = "ABORTED P5" ] && [ "$(pods)" = 0 ]
rec C03 $? "exit 0, ABORTED P5, pod deleted" "exit $rc $(cls "$D")" "session abort while running"
startrun C04; echo '{"state":"idle","exit":null,"started":null,"ended":null,"stages":null}' > "$ST/status.json"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LOST P3" ] && [ "$(rl "$D")" = owner ] && [ "$(pods)" = 0 ]
rec C04 $? "exit 0, LOST P3, relaunch owner" "exit $rc $(cls "$D") $(rl "$D")" "runner idle after its start (container restarted)"
startrun C05 9000; touch "$ST/unreachable"; rest_set '.status = "EXITED"'
rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LOST P3" ] && [ "$(rl "$D")" = owner ]
rec C05 $? "exit 0, LOST P3, relaunch owner (v2 has no status-change text: fail closed)" "exit $rc $(cls "$D") $(rl "$D")" "runner unreachable, REST v2 status EXITED"
startrun C06 9000; touch "$ST/unreachable"; L0=$(jq -r .startedAt "$D/pod-record-start.json")
jq -nc --arg a "$(iso $(( $(date +%s) - 7200 )))" --arg l "$L0" '{at:$a,phase:"regular",live:null,rest:{http:200,status:"RUNNING",startedAt:$l},listed:true}' >> "$D/checkins.jsonl"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 41 ] && [ "$(pods)" = 1 ]
rec C06 $? "exit 41, pod kept" "exit $rc" "unreachable twice 2 h apart, REST healthy RUNNING: wait"
startrun C07 90100; rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LIMIT P4" ] && [ "$(pods)" = 0 ]
rec C07 $? "exit 0, LIMIT P4, pod deleted" "exit $rc $(cls "$D")" "still running at the run deadline"
startrun C08 90100; touch "$ST/unreachable"; rc=$(fetch pkm-x-dev-c); [ "$rc" = 41 ] && [ "$(pods)" = 1 ] && [ ! -f "$D/outcome.json" ]
rec C08 $? "exit 41, pod kept (no information is not a loss)" "exit $rc" "N2: unreachable at the run deadline, before the hard deadline"
startrun C08b 90100; touch "$ST/unreachable"; hard_past; rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "NO_EVIDENCE P7" ] && [ "$(pods)" = 0 ]
rec C08b $? "exit 0, NO_EVIDENCE P7, pod deleted for cost" "exit $rc $(cls "$D")" "N2: still unreachable at the hard deadline"
startrun C09 90100; E=$(iso $(( $(epoch "$S0") + 90050 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
rc=$(fetch pkm-x-dev-c); [ "$rc" = 8 ] && [ "$(cls "$D")" = "LIMIT P2a" ]
rec C09 $? "exit 8 (git gate wait), LIMIT P2a" "exit $rc $(cls "$D")" "runner done 90,050 s after its start"
startrun C10 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 137 1 1; status_done "$S0" "$E" 1
rc=$(fetch pkm-x-dev-c); [ "$rc" = 8 ] && [ "$(cls "$D")" = "OOM P2b" ]
rec C10 $? "exit 8, OOM P2b" "exit $rc $(cls "$D")" "pilot exit 137 with cgroup oom_kill 1"
startrun C11 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 137 1 0; status_done "$S0" "$E" 1
rc=$(fetch pkm-x-dev-c); [ "$rc" = 8 ] && [ "$(cls "$D")" = "LOST P2c" ] && [ "$(rl "$D")" = owner ]
rec C11 $? "exit 8, LOST P2c, relaunch owner" "exit $rc $(cls "$D") $(rl "$D")" "pilot exit 137 with oom_kill 0"
startrun C12 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0; touch "$ST/tree-varies"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 6 ] && [ -f "$D/collect-started-1.json" ] && [ ! -f "$D/collect-1.json" ] && [ "$(pods)" = 1 ] && [ ! -e "$RD/collect-1" ]
rec C12 $? "exit 6, pod kept, collection 1 abandoned, nothing kept" "exit $rc" "N2: the two fetches differ while the runner is done (no information)"
startrun C13; jq --arg i stubpod1 'map(select(.id != $i))' "$ST/pods.json" > "$SB/x" && mv "$SB/x" "$ST/pods.json"; rm -f "$ST/pod-stubpod1.json.rec"; touch "$ST/unreachable"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LOST P3" ] && [ "$(rl "$D")" = auto ] && jq -e '.deleted_time_is_upper_bound == true' "$D/cost.json" >/dev/null
rec C13 $? "exit 0, LOST P3, relaunch auto, cost upper bound" "exit $rc $(cls "$D") $(rl "$D")" "owner decision 13: pod gone (not listed, v2 GET 404) before any logged session action"
startrun C14; rest_set '.startedAt = "2026-11-21T01:00:00.000Z"'
rc=$(fetch pkm-x-dev-c); [ "$rc" = 41 ] && [ "$(pods)" = 1 ]
rec C14 $? "exit 41, pod kept" "exit $rc" "S1: v2 startedAt changed while the runner reports running under s0"
startrun C15; touch "$ST/delete-fail"; rc=$(fetch pkm-x-dev-c --abort); rm -f "$ST/delete-fail"; rc2=$(fetch pkm-x-dev-c)
[ "$rc" = 7 ] && [ "$rc2" = 0 ] && [ -f "$D/cost.json" ] && [ "$(cls "$D")" = "ABORTED P5" ]
rec C15 $? "exit 7 (still listed), then exit 0 on the next call" "exit $rc then $rc2" "a failed delete is resumed by the next fetch.sh call"
ID=C16; mksb; run ./run.sh pkm-x-dev-c cap:8:0:5:12800000000 >/dev/null; D="$R/receipts_dev/pkm-x-dev-c"; rm -f "$D/start.json"
echo '{"state":"idle","exit":null,"started":null,"ended":null,"stages":null}' > "$ST/status.json"; rc=$(fetch pkm-x-dev-c --abort)
[ "$rc" = 0 ] && [ -d "$R/receipts_dev/pkm-x-dev-c.prestart-1" ] && [ "$(jq -r .class "$R/receipts_dev/pkm-x-dev-c.prestart-1/outcome.json")" = NOT_AN_ATTEMPT ]
rec C16 $? "exit 0, NOT_AN_ATTEMPT, receipts -> .prestart-1" "exit $rc" "launcher died before start (no start.json, runner idle)"
startrun C17; tok=$(cat "$SB/state/stubpod1.token"); rm -f "$SB/state/stubpod1.token" "$SB/state/pkm-x-dev-c.token"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 41 ] && [ "$(cat "$SB/state/stubpod1.token")" = "$tok" ] && ! grep -rqF "$tok" "$D" && grep -q 'token recovered' "$D/fetch.log"
rec C17 $? "exit 41, token recovered into RPC_STATE only" "exit $rc" "N6: the pod token file is missing (v2 GET returns env)"
startrun C18 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
pre=$(jq -nc --arg a "$(iso $(( $(date +%s) - 300 )))" --arg s "$S0" --arg e "$E" '{at:$a,phase:"prefetch",live:{state:"done",exit:0,started:$s,ended:$e},rest:{http:200,status:"RUNNING"},listed:true}')
jq -n --arg at "$(iso $(( $(date +%s) - 290 )))" --argjson p "$pre" '{at:$at, reason:"done", mode:"", prefetch:$p, out:"results/pkm-x-dev-c/collect-1"}' > "$D/collect-started-1.json"
mkdir -p "$RD"; cp -a "$ST/tree" "$RD/collect-1"; (cd "$RD/collect-1" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -r -d '\n' sha256sum) > "$D/local-sha256-1.txt"; echo '{"transfer_ok":true}' > "$D/transfer-1.json"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 8 ] && jq -e '.recovered == true and .transfer_ok == true' "$D/collect-1.json" >/dev/null && [ "$(cls "$D")" = "ADMITTED P2d" ] && [ ! -e "$RD/collect-2" ]
rec C18 $? "exit 8, orphan recovered (verified), ADMITTED P2d, no second collection" "exit $rc $(cls "$D")" "N3b: fetch.sh died after keeping the tree, before collect-1.json"
startrun C18b 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
jq -n --arg at "$(iso $(( $(date +%s) - 290 )))" --arg s "$S0" --arg e "$E" '{at:$at, reason:"done", mode:"", prefetch:{at:$at,phase:"prefetch",live:{state:"done",exit:0,started:$s,ended:$e},rest:{http:200,status:"RUNNING"},listed:true}, out:"x"}' > "$D/collect-started-1.json"
mkdir -p "$RD"; cp -a "$ST/tree" "$RD/collect-1"; (cd "$RD/collect-1" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -r -d '\n' sha256sum) > "$D/local-sha256-1.txt"; echo tampered >> "$RD/collect-1/host.txt"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 8 ] && jq -e '.recovered == true and .transfer_ok == false' "$D/collect-1.json" >/dev/null && [ -d "$RD/collect-1" ] && [ -d "$RD/collect-2" ] && [ "$(cls "$D")" = "NO_EVIDENCE P0b" ]
rec C18b $? "orphan kept (unverifiable), new collection 2, trees differ: NO_EVIDENCE P0b" "exit $rc $(cls "$D")" "N3b: an orphan that disagrees with a later collection of the same pod goes to the owner"
startrun C19; printf '{"at":"2026-11-15T00:00:00Z","phase":"reg' >> "$D/checkins.jsonl"; rc=$(fetch pkm-x-dev-c); [ "$rc" = 12 ] && [ "$(pods)" = 1 ]
rec C19 $? "exit 12 (owner), pod kept" "exit $rc" "S2: a truncated check-in line (fail closed)"
else skip C01 C02 C02b C02c C03 C04 C05 C06 C07 C08 C08b C09 C10 C11 C12 C13 C14 C15 C16 C17 C18 C18b C19; fi

# ===================================================================== D. the classifier alone, on synthetic v2-shaped receipt sets
S0E=$(( $(date +%s) - 200000 )); S0=$(iso $S0E); DLE=$(( S0E + 90000 )); L0="2026-11-10T10:00:00.000Z"
mkset() { # mkset DIR: a receipts dir with the launch receipts (REST v2 field names) of a run started at S0
  local d=$1; rm -rf "$d"; mkdir -p "$d"
  jq -n --arg l "$L0" '{id:"p1", name:"pkm-x-dev-d", cloud:"SECURE", cpu:{id:"cpu5m", vcpuCount:16, memory:128}, cost:1.04, startedAt:$l, createdAt:$l, status:"RUNNING", dataCenterId:null}' > "$d/pod-create.json"
  jq -n --arg l "$L0" '{id:"p1", cloud:"SECURE", cpu:{id:"cpu5m", vcpuCount:16, memory:128}, cost:1.04, startedAt:$l, status:"RUNNING", dataCenterId:"EU-RO-1"}' > "$d/pod-record.json"
  jq -n --arg l "$L0" '{startedAt:$l, status:"RUNNING", dataCenterId:"EU-RO-1"}' > "$d/pod-record-start.json"
  jq -n '{name:"pkm-x-dev-d", cloud:"SECURE", cpu:{id:"cpu5m", vcpuCount:16}, image:"x", disk:20, ports:["8000/http"], env:{RUNNER_TOKEN:"<withheld>"}, entrypoint:["bash","-c"], cmd:["<boot>"]}' > "$d/pod-request.json"
  echo '{"at":"2026-11-10T09:59:00Z","instance":"cpu5m-16-128","http":200,"id":"cpu5m","availability":"HIGH","ramGbPerVcpu":8,"vcpu":{"min":2,"max":32},"securePerVcpu":0.065,"price":1.04}' > "$d/price.jsonl"
  jq -n '{stage:"cap:8:0:5:12800000000", mem_floor_gb:121}' > "$d/pod-state.json"
  jq -nc --arg s "$S0" '{state:"running",exit:null,started:$s,ended:null,stages:["cap:8:0:5:12800000000"]}' > "$d/start.json"
  : > "$d/checkins.jsonl"
}
ck() { # ck DIR AT_EPOCH PHASE LIVE(state|null) STARTED ENDED EXIT HTTP STATUS STARTEDAT LISTED [EXTRA_REST_JSON]
  local live=null; [ "$4" != null ] && live=$(jq -nc --arg st "$4" --arg s "$5" --arg e "$6" --argjson x "$7" '{state:$st, exit:$x, started:(if $s == "" then null else $s end), ended:(if $e == "" then null else $e end)}')
  jq -nc --arg a "$(iso "$2")" --arg ph "$3" --argjson live "$live" --argjson h "$8" --arg d "$9" --arg l "${10}" --argjson li "${11}" --argjson x "${12:-{\}}" \
    '{at:$a, phase:$ph, live:$live, rest:(if $h == 200 then {http:$h, status:$d, startedAt:$l} + $x else {http:$h} end), listed:$li}' >> "$1/checkins.jsonl"
}
collect() { # collect DIR RES REASON TRANSFER_OK AT_EPOCH [K]: collection K (default 1) with the last check-in as prefetch
  local pre k=${6:-1}; pre=$(tail -1 "$1/checkins.jsonl"); [ -n "$pre" ] || pre=null
  local f=false; if [ -d "$2/collect-$k" ]; then f=true; (cd "$2/collect-$k" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -r -d '\n' sha256sum) > "$1/local-sha256-$k.txt"; fi
  jq -n --arg a "$(iso "$5")" --arg r "$3" --argjson t "$4" --argjson p "$pre" --argjson f "$f" '{at:$a, reason:$r, mode:"", prefetch:$p, fetched:$f, transfer_ok:$t}' > "$1/collect-$k.json"
}
dcase() { # dcase ID EXPECTED_CLASS_RULE DESCRIPTION [EXPECTED_RELAUNCH]
  local got; got=$( (cd "$RC" && ./fetch.sh --classify-dir "$WORK/d/$1/rec" "$WORK/d/$1/res") 2> "$OUTD/$1.err" | tee "$OUTD/$1.txt" | jq -r '.class + " " + .rule + (if .relaunch then " relaunch=" + .relaunch else "" end)')
  local want="$2${4:+ relaunch=$4}"; [ "$got" = "$want" ]; rec "$1" $? "$want" "$got" "$3"
}
nd() { DR="$WORK/d/$1/rec" DO="$WORK/d/$1/res"; TD="$DO/collect-1"; mkdir -p "$WORK/d/$1"; mkset "$DR"; rm -rf "$DO"; }
okE=$(iso $(( S0E + 3600 ))); okEe=$(( S0E + 3600 ))
done_set() { mktree "$TD" cap:8:0:5:12800000000 "$S0" "$okE" "${1:-0}" 0 "${2:-0}" "${3:-1}" - keep; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" "${4:-0}" 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); }
nd D01; rm "$DR/start.json"; dcase D01 "NOT_AN_ATTEMPT P1" "no runner start on record"
nd D02; done_set; dcase D02 "ADMITTED P2d" "completed, every admission rule holds (v2 receipts)"
nd D03; E=$(iso $(( S0E + 90050 ))); mktree "$TD" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; ck "$DR" $((S0E + 90100)) prefetch done "$S0" "$E" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((S0E + 90200)); dcase D03 "LIMIT P2a" "completed 90,050 s after the runner start"
nd D04; ck "$DR" $((DLE + 60)) regular running "$S0" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" deadline false $((DLE + 70)); dcase D04 "LIMIT P4" "running at the deadline check-in"
nd D05; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D05 "LOST P3" "runner idle before the deadline" owner
nd D06; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 200 RUNNING "2026-11-11T00:00:00.000Z" true; dcase D06 "PENDING P8" "S1: startedAt changed while the runner runs under s0: not evidence"
nd D06b; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 RUNNING "2026-11-11T00:00:00.000Z" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D06b "LOST P3" "startedAt changed and the runner is idle" owner
nd D07; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 RUNNING "$L0" true; ck "$DR" $((S0E + 7700)) regular null "" "" null 500 "" "" true; dcase D07 "PENDING P8" "N2: unreachable twice >= 1 h apart, REST unreadable (HTTP 500): no information"
nd D07b; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 200 EXITED "$L0" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D07b "LOST P3" "v2 status EXITED: loss evidence; no v2 status-change text, so relaunch owner (fail closed)" owner
nd D07c; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 EXITED "$L0" true '{"lastStatusChange":"Exited by RunPod: host failure"}'; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D07c "LOST P3" "a v1-style lastStatusChange field in a check-in is ignored (never host-side evidence)" owner
nd D08; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 RUNNING "$L0" true; ck "$DR" $((S0E + 7700)) regular null "" "" null 200 RUNNING "$L0" true; dcase D08 "PENDING P8" "unreachable twice >= 1 h apart, REST healthy: wait"
nd D08b; ck "$DR" $((S0E + 4000)) regular null "" "" null 0 "" "" null; ck "$DR" $((S0E + 11200)) regular null "" "" null 0 "" "" null; dcase D08b "PENDING P8" "N2 probe: session network down at two check-ins 2 h apart (live null, REST 0, list null)"
nd D09; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" abort false $((S0E + 4010)); dcase D09 "ABORTED P5" "session abort while running"
nd D10; done_set 137 1 1 1; dcase D10 "OOM P2b" "pilot exit 137, oom_kill 1"
nd D11; done_set 137 0 1 1; dcase D11 "LOST P2c" "pilot exit 137, oom_kill 0" owner
nd D12; done_set 137 none 1 1; dcase D12 "LOST P2c" "pilot exit 137, no cgroup files" owner
nd D13; done_set 1 0 1 1; dcase D13 "NOT_ADMITTED P2e" "pilot exit 1 (a tool failure, not a loss)"
nd D14; ck "$DR" $((DLE + 60)) regular idle "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" deadline false $((DLE + 70)); dcase D14 "LOST P6" "restart first seen at the deadline check-in" owner
nd D14b; ck "$DR" $((DLE + 60)) regular running "$S0" "" null 200 RUNNING "$L0" true; ck "$DR" $((DLE + 600)) regular idle "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" deadline false $((DLE + 610)); dcase D14b "LIMIT P4" "running at the deadline, restarted afterwards"
nd D15; ck "$DR" $((S0E + 93600)) regular null "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" hard false $((S0E + 93610)); dcase D15 "NO_EVIDENCE P7" "N2: unreachable through the hard deadline"
nd D16; done_set; ck "$DR" $((okEe + 900)) regular idle "" "" null 200 RUNNING "2026-11-12T00:00:00.000Z" true; dcase D16 "ADMITTED P2d" "completed, receipts fetched, then restarted"
nd D16b; mktree "$TD" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((okEe + 50)) regular idle "" "" null 200 RUNNING "$L0" true; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D16b "LOST P3" "restart recorded before the completion was fetched" owner
nd D17; done_set; jq '.mode = "--abort"' "$DR/collect-1.json" > "$DR/c" && mv "$DR/c" "$DR/collect-1.json"; dcase D17 "ADMITTED P2d" "--abort on a run that is already done cannot change its class"
nd D18; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" abort false $((S0E + 4010)); dcase D18 "LOST P3" "--abort after a restart is LOST, not ABORTED" owner
nd D19; mktree "$TD" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0 2; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D19 "LOST P3" "two start lines in progress.txt (a second runner start)" owner
nd D20; mktree "$TD" cap:8:0:5:12800000000 "$(iso $((S0E + 30)))" "$okE" 0 0 0; ck "$DR" $((okEe + 100)) prefetch done "$(iso $((S0E + 30)))" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D20 "LOST P3" "the runner's start differs from start.json (manual re-start)" owner
nd D21; mktree "$TD" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((S0E + 93600)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" hard false $((S0E + 93610)); dcase D21 "NO_EVIDENCE P7" "N2: completed in time but the transfer check still failed at the hard deadline"
nd D22; done_set; echo tampered >> "$TD/host.txt"; dcase D22 "NOT_ADMITTED P2e" "a file of the kept tree changed after collection (rule 6)"
nd D23; done_set; jq '.cloud = "COMMUNITY"' "$DR/pod-create.json" > "$DR/c" && mv "$DR/c" "$DR/pod-create.json"; dcase D23 "NOT_ADMITTED P2e" "create response cloud COMMUNITY (rule 7)"
nd D23b; done_set; jq '.cpu.memory = 64' "$DR/pod-record.json" > "$DR/c" && mv "$DR/c" "$DR/pod-record.json"; dcase D23b "NOT_ADMITTED P2e" "GET record cpu.memory 64, below the 121 GB floor (rule 7)"
nd D23c; done_set; jq '. + {interruptible:true}' "$DR/pod-request.json" > "$DR/c" && mv "$DR/c" "$DR/pod-request.json"; dcase D23c "NOT_ADMITTED P2e" "request record with an interruptible field (rule 7)"
nd D23d; done_set; jq '.cost = 1.05' "$DR/pod-create.json" > "$DR/c" && mv "$DR/c" "$DR/pod-create.json"; dcase D23d "NOT_ADMITTED P2e" "create response cost 1.05 differs from the catalog price 1.04 (rule 7)"
nd D23e; done_set; rm "$DR/pod-record.json"; dcase D23e "NOT_ADMITTED P2e" "no GET record after create: cpu.memory unobservable (rule 7 fails closed)"
nd D24; ck "$DR" $((S0E + 4000)) regular null "" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D24 "LOST P3" "owner decision 13: pod gone from the list and v2 GET 404, before any logged session action" auto
nd D24c; printf '{"at":"%s","action":"DELETE","pod":"p1","reason":"x"}\n' "$(iso $((S0E + 3000)))" > "$DR/session-actions.jsonl"; ck "$DR" $((S0E + 4000)) regular null "" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D24c "LOST P3" "pod gone and GET 404 after a logged session delete" owner
nd D24d; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 RUNNING "$L0" false; dcase D24d "PENDING P8" "pod absent from the list but GET returns a healthy RUNNING record: no loss evidence, wait"
nd D24b; ck "$DR" $((S0E + 4000)) regular null "" "" null 0 "" "" false; dcase D24b "PENDING P8" "S6: missing from the list but GET unreadable (not corroborated)"
# Review round 3 (R1, R2) with the one host-side form v2 can show: the pod gone (not listed, GET 404).
nd D43; ck "$DR" $((S0E + 20000)) regular running "$S0" "" null 200 RUNNING "$L0" true; ck "$DR" $((S0E + 25000)) regular null "" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((S0E + 25010)); dcase D43 "LOST P3" "R1: gone with GET 404 after the last alive check-in, before any session action" auto
nd D43c; ck "$DR" $((S0E + 20000)) regular running "$S0" "" null 200 RUNNING "$L0" true; ck "$DR" $((S0E + 30000)) regular idle "" "" null 200 RUNNING "$L0" true; ck "$DR" $((S0E + 35000)) regular null "" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((S0E + 35010)); dcase D43c "LOST P3" "R1: gone with GET 404 only after the first loss (runner idle): not tied to the loss" owner
nd D44; ck "$DR" $((S0E + 80000)) regular done "$S0" "$(iso $((S0E + 79000)))" 0 200 RUNNING "$L0" true; ck "$DR" $((S0E + 82000)) regular null "" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((S0E + 82010)); dcase D44 "LOST P3" "R2 probe P2: runner seen done, then gone with GET 404 before the run deadline" owner
nd D44b; ck "$DR" $((S0E + 80000)) regular done "$S0" "$(iso $((S0E + 79000)))" 0 200 RUNNING "$L0" true; ck "$DR" $((DLE + 3600)) regular null "" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((DLE + 3610)); dcase D44b "LOST P6" "R2 probe P3: runner seen done, then gone with GET 404 after the run deadline" owner
nd D45; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 ERROR "$L0" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D45 "LOST P3" "v2 status ERROR" owner
nd D46; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 STARTING "$L0" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D46 "LOST P3" "v2 status STARTING after the runner's start (the container is starting again)" owner
P2E=$(( okEe + 100 )); PCOL=$(( okEe + 200 ))
mcase() { # mcase ID STAGE PILOT_EXIT OOM CG_PEAK LOG EXPECTED DESCRIPTION [EXPECTED_SOURCE_USED] [OVERSIZE]
  nd "$1"; local rx=0; [ "$3" = 0 ] || rx=1
  mktree "$TD" "$2" "$S0" "$okE" "$3" 0 "$4" 1 "$5" "$6" "${10:-false}"; jq --arg st "$2" '.stage = $st' "$DR/pod-state.json" > "$DR/x" && mv "$DR/x" "$DR/pod-state.json"
  ck "$DR" $P2E prefetch done "$S0" "$okE" $rx 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $PCOL
  local got; got=$( (cd "$RC" && ./fetch.sh --classify-dir "$DR" "$DO") 2> "$OUTD/$1.err" | tee "$OUTD/$1.txt" | jq -r '.class + " " + .rule + " source=" + (.memory.source_used // "none")')
  local want="$7 source=${9:-pilot_log_VmHWM_MiB}"; [ "$got" = "$want" ]; rec "$1" $? "$want" "$got" "$8"
}
G1=1073741824; S56=cap:8:0:5:12800000000; S112=cap:8:0:5:27900000000
# Fixtures state both figures: "pilot N MiB" is a VmHWM line in the F4 log (the fixture log's own lines peak at 64 MiB),
# "cgroup N GiB" is a memory.peak file (or v1 max_usage_in_bytes). The pilot figure is used when present. The 56Gi step
# of draft Amendment 18 is gone: the attempt here is the -m112 attempt itself.
mcase D25 $S56 0 0 - 30720 "ADMITTED P2d" "memory: development size 56Gi, pilot 30 GiB"
mcase D26 $S56 0 0 - 71680 "ADMITTED P2d" "memory: development size 56Gi, pilot 70 GiB: no 56 GiB step any more"
mcase D27 $S112 0 0 - 128000 "OOM P2p" "memory: -m112 attempt, pilot 125 GiB (> 112 GiB): OOM, censored (owner decision 15)"
mcase D28 $S112 0 0 $((120 * G1)) keep "ADMITTED P2d" "memory: -m112, pilot 64 MiB and cgroup 120 GiB -> pilot (primary); cgroup ignored"
mcase D28b $S112 0 0 $((120 * G1)) strip "OOM P2p" "memory: -m112, no pilot line, cgroup v2 120 GiB -> cgroup fallback, > 112 GiB" cgroup_v2_memory.peak
mcase D29 $S112 0 0 - strip "NOT_ADMITTED P2u" "memory: no pilot line, no cgroup file -> none" none
mcase D30 $S112 0 0 v1:$((100 * G1)) strip "ADMITTED P2d" "memory: -m112, no pilot line, cgroup v1 max_usage 100 GiB -> v1 fallback" cgroup_v1_memory.max_usage_in_bytes
mcase D31 $S112 0 0 - 115712 "OOM P2p" "memory: -m112, pilot 113 GiB -> OOM (owner decision 15)"
mcase D32 $S112 0 0 - 102400 "ADMITTED P2d" "memory: -m112, pilot 100 GiB"
mcase D33 $S112 0 0 - 114688 "ADMITTED P2d" "memory: -m112, pilot exactly 112 GiB (not above)"
mcase D34 $S112 137 0 - 71680 "LOST P2c" "memory: -m112, exit 137, oom_kill 0"
mcase D36 $S112 137 1 - 30720 "OOM P2b" "memory: -m112, pilot 30 GiB but exit 137 with oom_kill 1 -> OOM whatever the peak"
mcase D37 $S112 0 0 - 102400 "ADMITTED P2d" "-m112 row with oversize true: admitted as a row; the reading treats it as censored (Amendments 7 B, 14)" pilot_log_VmHWM_MiB true
nd D38; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 EXITED "$L0" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D38 "LOST P3" "runner idle and status EXITED" owner
nd D39; printf '{"at":"%s","action":"DELETE","pod":"p1","reason":"x"}\n' "$(iso $((S0E + 3000)))" > "$DR/session-actions.jsonl"; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 EXITED "$L0" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D39 "LOST P3" "status EXITED after the session logged a delete" owner
nd D40; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 RUNNING "$L0" true; printf '{"at":"broken' >> "$DR/checkins.jsonl"; dcase D40 "NO_EVIDENCE P0" "S2: a truncated check-in line fails closed"
nd D40b; done_set; printf 'not json' > "$DR/collect-1.json"; dcase D40b "NO_EVIDENCE P0" "S2: an unparsable collection record fails closed"
nd D40c; done_set; printf 'not json' > "$DR/pod-record.json"; dcase D40c "NO_EVIDENCE P0" "S2: an unparsable GET record after create fails closed"
nd D41; done_set; cp -a "$TD" "$DO/collect-2"; echo x >> "$DO/collect-2/host.txt"; collect "$DR" "$DO" done true $((okEe + 300)) 2; dcase D41 "NO_EVIDENCE P0b" "N3b: two done collections of the pod disagree file by file"
nd D41b; done_set; cp -a "$TD" "$DO/collect-2"; collect "$DR" "$DO" done true $((okEe + 300)) 2; dcase D41b "ADMITTED P2d" "N3b: two done collections agree"
nd D42; : > "$DR/start.json"; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 200 RUNNING "$L0" true; dcase D42 "PENDING P8" "S2: a zero-byte start.json (no response) is read as absent; the start comes from the check-in"

# ===================================================================== E. the registered order (next_allowed)
ecase() { # ecase ID EXPECTED DESCRIPTION outcome-specs... (name:class[:oversize[:relaunch]])
  local id=$1 want=$2 desc=$3; shift 3; mksb; local s n c o r
  for s in "$@"; do IFS=: read -r n c o r <<<"$s"; if [ "$c" = INFLIGHT ]; then mkdir -p "$R/receipts/$n"; else outcome "$n" "$c" "${o:-false}" "${r:-auto}"; fi; done
  local got; got=$(cd "$R" && bash -c '. ./common.sh; next_allowed' 2>&1); [ "$got" = "$want" ]; rec "$id" $? "$want" "$got" "$desc"
}
ecase E01 $BASE "nothing run yet"
ecase E02 $R1 "base LOST with host-side evidence (relaunch auto)" $BASE:LOST
ecase E03 ALL_RESOLVED "N3a: base LOST without host-side evidence (relaunch owner)" $BASE:LOST:false:owner
ecase E04 ALL_RESOLVED "N1: base ABORTED uses the attempt; no relaunch" $BASE:ABORTED
ecase E05 ALL_RESOLVED "base ADMITTED" $BASE:ADMITTED
ecase E06 ALL_RESOLVED "base ADMITTED with oversize true: censored at 112Gi, no further size step" $BASE:ADMITTED:true
ecase E07 ALL_RESOLVED "base OOM at 112Gi: censored, no further size step" $BASE:OOM
ecase E08 ALL_RESOLVED "base LIMIT (censored, not re-run)" $BASE:LIMIT
ecase E09 ALL_RESOLVED "base NOT_ADMITTED (owner)" $BASE:NOT_ADMITTED
ecase E10 ALL_RESOLVED "N2: base NO_EVIDENCE goes to the owner" $BASE:NO_EVIDENCE
ecase E11 BLOCKED:$BASE "base in flight" $BASE:INFLIGHT
ecase E12 BLOCKED:$R1 "base LOST auto, relaunch in flight" $BASE:LOST $R1:INFLIGHT
ecase E13 ALL_RESOLVED "base LOST auto, relaunch LOST auto: no third attempt" $BASE:LOST $R1:LOST
ecase E14 ALL_RESOLVED "base LOST auto, relaunch ADMITTED" $BASE:LOST $R1:ADMITTED

# ===================================================================== F. reap.sh against the stub
ID=F01; mksb; old=$(isod $(( $(date +%s) - 27 * 3600 ))); young=$(isod $(( $(date +%s) - 3600 )))
jq -n --arg o "$old" --arg y "$young" --arg b $BASE '[{id:"old1",name:$b,status:"RUNNING",cost:1.04,createdAt:$o,startedAt:$o},
  {id:"young1",name:"pkm-x-dev-y",status:"RUNNING",cost:1.04,createdAt:$y,startedAt:$y},
  {id:"other1",name:"pfd-b-rp",status:"RUNNING",cost:1.04,createdAt:$o,startedAt:$o}]' > "$ST/pods.json"
rc=$(run ./reap.sh); left=$(jq -c '[.[].id]' "$ST/pods.json"); rc2=$( (cd "$R" && ./reap.sh) >> "$OUTD/F01.txt" 2>&1; echo $?)
left2=$(jq -c '[.[].id]' "$ST/pods.json"); lines=$(cat "$R"/reaper/reap-*.jsonl | wc -l)
[ "$rc" = 0 ] && [ "$left" = '["young1","other1"]' ] && [ "$rc2" = 0 ] && [ "$left2" = '["young1","other1"]' ] && grep -q '"due":true' "$R"/reaper/reap-*.jsonl && grep -q '"action":"DELETE"' "$R/reaper/session-actions.jsonl" \
  && grep -q 'DELETE https://api.runpod.io/v2/pods/old1' "$ST/calls.log"
rec F01 $? "pod past creation + 26 h deleted through v2 (logged first); young and non-pkm-x pods kept; second pass changes nothing" "exit $rc, left $left; exit $rc2, left $left2; $lines receipt lines" "reaper: 27 h old pod without receipts, 1 h old pod, other session's pod"
ID=F08; mksb; old=$(isod $(( $(date +%s) - 27 * 3600 ))); jq -n --arg o "$old" --arg b $BASE '[{id:"old1",name:$b,status:"RUNNING",createdAt:$o,startedAt:$o}]' > "$ST/pods.json"
( flock "$SB/state/$BASE.lock" sleep 6 ) & lp=$!; sleep 1
rc=$(run ./reap.sh); wait $lp
[ "$rc" = 0 ] && [ "$(pods)" = 1 ] && grep -q 'lock held' "$R"/reaper/reap-*.jsonl && ! grep -q 'DELETE https://api.runpod.io/v2/pods/old1' "$ST/calls.log"
rec F08 $? "pod untouched, skip recorded" "exit $rc, pods $(pods)" "S1: a due pod whose per-name lock is held by another process"
ID=F03; mksb; touch "$ST/deny"; rc=$(run ./reap.sh); [ "$rc" = 1 ]; rec F03 $? "exit 1" "exit $rc" "reaper: pod list unreadable (network blocked)"
ID=F07; mksb; jq -n '[{id:"bad1",name:"pkm-x-dev-q",status:"RUNNING",createdAt:"garbage"},{id:"nul1",name:null,status:"RUNNING"}]' > "$ST/pods.json"
rc=$(run ./reap.sh); [ "$rc" = 1 ] && [ "$(pods)" = 2 ]; rec F07 $? "exit 1, nothing deleted" "exit $rc, pods $(pods)" "S6: unparsable createdAt and a null name: reported, not deleted"
ID=F09; mksb; old=$(isod $(( $(date +%s) - 27 * 3600 ))); jq -n --arg o "$old" '[{id:"pg2old",name:"pkm-x-dev-old",status:"RUNNING",createdAt:$o}]' > "$ST/pods-page2.json"
rc=$(run ./reap.sh); [ "$rc" = 7 ] && grep -q '"pod":"pg2old"' "$R"/reaper/reap-*.jsonl && grep -q 'DELETE https://api.runpod.io/v2/pods/pg2old' "$ST/calls.log"
rec F09 $? "the page-2 pod is seen and a delete is sent (the stub keeps page 2, so it stays listed: exit 7)" "exit $rc" "v2 list pagination in the reaper"
if [ "$HAVE_LAYER" = 1 ]; then
ID=F02; startrun F02 90100; hard_past; rc=$( (cd "$R" && ./reap.sh) > "$OUTD/F02.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$(pods)" = 0 ] && [ "$(cls "$D")" = "LIMIT P4" ] && [ -f "$D/cost.json" ]
rec F02 $? "pod collected by fetch.sh (LIMIT P4) and deleted" "exit $rc $(cls "$D")" "reaper: a run past its hard deadline, with receipts"
ID=F04; startrun F04; rm -f "$D/start.json" "$D/pod-state.json"; echo '{"state":"idle","exit":null,"started":null,"ended":null,"stages":null}' > "$ST/status.json"
ago=$(isod $(( $(date +%s) - 27 * 3600 ))); jq --arg a "$ago" 'map(.createdAt = $a)' "$ST/pods.json" > "$SB/x" && mv "$SB/x" "$ST/pods.json"
jq --arg a "$ago" '.createdAt = $a' "$D/pod-create.json" > "$SB/x" && mv "$SB/x" "$D/pod-create.json"
rc=$( (cd "$R" && ./reap.sh) > "$OUTD/F04.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$(pods)" = 0 ] && [ -d "$R/receipts_dev/pkm-x-dev-c.prestart-1" ]
rec F04 $? "pod deleted through fetch.sh, NOT_AN_ATTEMPT" "exit $rc" "reaper: launcher died, no deadline recorded, creation + 26 h passed"
ID=F05; startrun F05; rm -f "$D/start.json"; ago=$(isod $(( $(date +%s) - 5400 )))
jq --arg a "$ago" 'map(.createdAt = $a)' "$ST/pods.json" > "$SB/x" && mv "$SB/x" "$ST/pods.json"; jq --arg a "$ago" '.createdAt = $a' "$D/pod-create.json" > "$SB/x" && mv "$SB/x" "$D/pod-create.json"
rc=$( (cd "$R" && ./reap.sh) > "$OUTD/F05.txt" 2>&1; echo $?); rc2=$(fetch pkm-x-dev-c)
[ "$rc" = 0 ] && [ "$(pods)" = 1 ] && ! grep -q 'DELETE https://api.runpod.io/v2/pods/stubpod1' "$ST/calls.log" && [ "$rc2" = 41 ]
rec F05 $? "pod untouched; the next fetch.sh waits (41)" "exit $rc, pods $(pods), fetch $rc2" "N3c, the review's probe: start.json missing, runner running, created 90 min ago"
ID=F06; startrun F06 90100; touch "$ST/unreachable"; rc=$( (cd "$R" && ./reap.sh) > "$OUTD/F06.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$(pods)" = 1 ] && ! grep -q 'DELETE https://api.runpod.io/v2/pods/stubpod1' "$ST/calls.log"
rec F06 $? "pod untouched" "exit $rc, pods $(pods)" "N3c: past the run deadline but before the hard deadline, runner unreachable"
else skip F02 F04 F05 F06; fi

# ===================================================================== G. syntax, pins, request, no REST v1 or GraphQL
for f in common.sh run.sh fetch.sh reap.sh rp_pinned.sh make_requests_m112.sh tests/run_tests.sh tests/curl_stub.sh; do
  bash -n "$RC/$f" 2> "$OUTD/G-${f##*/}.txt"; r=$?; rec "G-${f##*/}" $r "bash -n ok" "rc $r" "syntax of $f"
done
sha256sum "$RC/rp_pinned.sh" | grep -q '^b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d'; rec G-pin $? "rp_pinned.sh sha256 b4b60199..." "$(sha256sum "$RC/rp_pinned.sh" | cut -c1-8)" "pinned client copy unchanged"
sha256sum "$RC/server_pinned.py" | grep -q '^f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550'; rec G-server $? "server_pinned.py sha256 f56fd08e..." "$(sha256sum "$RC/server_pinned.py" | cut -c1-8)" "vendored runner = runpod/server.py at d2a16c723d"
for f in common.sh run.sh fetch.sh reap.sh; do grep -v -e '^ *#' "$RC/$f" | grep -nE 'rest\.runpod\.io|graphql|desiredStatus|costPerHr|lastStatusChange|lastStartedAt|secureCloud|cpuFlavor|memoryInGb|dockerStartCmd|dockerEntrypoint|cloudType' | sed "s|^|$f:|"; done > "$OUTD/G-v2.txt"; [ ! -s "$OUTD/G-v2.txt" ]
rec G-v2 $? "no REST v1 or GraphQL URL and no v1 field name in the route's code" "$(wc -l < "$OUTD/G-v2.txt") hits" "REST v2 port (comments excluded)"
! grep -nE '"\$RP" (create|pod|delete)' "$RC"/run.sh "$RC"/fetch.sh "$RC"/reap.sh > "$OUTD/G-rpuse.txt"
rec G-rpuse $? "rp_pinned.sh used only for upload, start, status, fetch (its v1 create, pod, delete are never called)" "$(wc -l < "$OUTD/G-rpuse.txt") hits" "pod-proxy calls only through the pinned client"
want=$(cd "$RC" && bash -c '. ./common.sh; c7_m112_request "$study/$STUDY_SRC_REL"'); pin=$(cd "$RC" && bash -c '. ./common.sh; echo $STUDY_REQUEST_SHA')
[ "$(sha256sum "$RC/requests_m112/pfd-x-t14-k0-c7-m112.json" | cut -d' ' -f1)" = "$pin" ] && jq -e --argjson w "$want" '. == $w' "$RC/requests_m112/pfd-x-t14-k0-c7-m112.json" >/dev/null \
  && grep -q "^$pin  requests_m112/pfd-x-t14-k0-c7-m112.json  changed: \"job.job_name job.command.1 job.memory provenance.note\"" "$RC/make_requests_m112_output.txt"
rec G-request $? "request sha256 = pin = make_requests output; equals its derivation; four changed paths" "pin ${pin:0:8}" "the one request (Amendments 7 B and 14)"
ntot=$((npass + nfail + nskip)); nlayer=$(echo A15 A16 A17 A18 A19 B01 B02 B03 B04 B04b B05 B06 B07 B08 B09 B10 B11 B12 B13 B14 B15 B16 B17 B18 B19 B20 C01 C02 C02b C02c C03 C04 C05 C06 C07 C08 C08b C09 C10 C11 C12 C13 C14 C15 C16 C17 C18 C18b C19 F02 F04 F05 F06 | wc -w)

if [ "$WRITE" = 1 ]; then
  rm -rf "$T/out"; cp -a "$OUTD" "$T/out"; cp "$RES" "$T/results.tsv"
  { cat <<INTRO
# Offline tests of the capped RunPod route (Amendment 33 draft, REST v2)

\`run_tests.sh\` runs every case in a fresh sandbox copy of the route (its own git repository with a local bare
\`origin\`, receipts, ledger, \`RPC_STATE\` and stub state) under \`\$TMPDIR\`. No pod is created and no network call is
made: \`curl_stub.sh\` is installed as \`curl\` first on \`PATH\` and answers in the RunPod REST v2 shapes of
\`../V2_FIELDS.md\` (pod list envelope with cursor pages, create with HTTP 201, GET, DELETE with 204 or 404, the CPU
catalog) and as the pod proxy; it has no route for REST v1 or GraphQL. \`HTTPS_PROXY\` points at a dead local port, and in
the A cases the stub fails every call, so "0 API calls" means the refusal came before any call.
\`fixtures/smoke_tree/\` is the results tree of the first development smoke pod (\`cap:8:0:5\`, t = 8, not data).

**Runs.** \`bash tests/run_tests.sh\` from \`runpod_capped/\` writes nothing in the repository and prints the results;
\`bash tests/run_tests.sh --write\` also replaces \`tests/out/\`, \`tests/results.tsv\` and this file.

**The image layer.** $nlayer of the $ntot cases (launch, fetch, four reaper cases and five refusals that come after the
binary check) need the image layer: \`RPC_LAYER_SRC\` (the layer tarball, SHA-256 \`a0f7e28e…\`, \`image.json\`
\`layer.digest\`) and \`RPC_BIN_SRC\` (its \`usr/local/bin\`; defaults in the session scratchpad). Without them those
cases print \`SKIP-NEEDS-LAYER\`: a run without the layer gives $((ntot - nlayer)) results and $nlayer skips; a run with
it gives $ntot results.

Groups: A refusals before any API call (allow-list, order, the one stage, the pinned request); B launch paths (v2 create
body and checks, the study guard, pagination); C \`fetch.sh\` flows, with the git gate in C02-C02c; D the outcome
classifier alone (\`fetch.sh --classify-dir\`) on synthetic receipt sets with v2 field names; E the order
(\`next_allowed\`); F \`reap.sh\`; G syntax, pins, the request and the absence of v1 calls. Per-case output:
\`out/<id>.txt\`. Machine-readable table: \`results.tsv\`. Every result here is \`[pcc:unverified]\`.

| id | result | case | expected | observed |
|:--|:--|:--|:--|:--|
INTRO
    awk -F'\t' 'NR > 1 { gsub(/[|]/, "/", $0); printf "| %s | %s | %s | %s | %s |\n", $1, $2, $5, $3, $4 }' "$RES"
    echo; echo "Summary of this run (image layer available: $HAVE_LAYER): $npass passed, $nfail failed, $nskip SKIP-NEEDS-LAYER (run $(date -u +%FT%TZ))."; } > "$T/README.md"
fi
echo "passed $npass, failed $nfail, SKIP-NEEDS-LAYER $nskip (layer-dependent cases: $nlayer of $ntot)"
if [ -n "${KEEP:-}" ]; then echo "kept $WORK"; else rm -rf "${WORK:?}"; fi
[ "$nfail" = 0 ]
