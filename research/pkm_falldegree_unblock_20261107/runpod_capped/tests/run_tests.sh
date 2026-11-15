#!/usr/bin/env bash
# run_tests.sh [--write] -- offline tests of the capped RunPod route (draft Amendment 18). No pod, no network: every
# curl call goes to tests/curl_stub.sh (first on PATH), HTTPS_PROXY points at a dead port, and the "deny" cases fail
# every call. Each case runs in a fresh sandbox copy of the route (its own git repo, receipts, ledger, RPC_STATE and
# stub state) under $TMPDIR. A plain run writes nothing in the repository; --write replaces tests/out/,
# tests/results.tsv and tests/README.md with this run's outputs.
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

rec() { # rec ID OK(0|1) EXPECTED OBSERVED DESCRIPTION
  local r=PASS; [ "$2" = 0 ] || r=FAIL
  [ "$r" = PASS ] && npass=$((npass + 1)) || nfail=$((nfail + 1))
  printf '%s\t%s\t%s\t%s\t%s\n' "$1" "$r" "$3" "$4" "$5" >> "$RES"; echo "$r $1: $5 (expected $3, observed $4)"
}
skip() { local i; for i in "$@"; do nskip=$((nskip + 1)); printf '%s\tSKIP-NEEDS-LAYER\t-\t-\tneeds the image layer\n' "$i" >> "$RES"; echo "SKIP-NEEDS-LAYER $i"; done; }
iso() { date -u -d "@$1" +%FT%TZ; }
epoch() { date -u -d "$1" +%s; }
rpd() { date -u -d "@$1" '+%Y-%m-%d %H:%M:%S.000 +0000 UTC'; }

mksb() { # fresh sandbox; sets SB, R (route dir), ST (stub dir), and the environment of the scripts
  SB=$(mktemp -d "$WORK/sb.XXXX"); local s="$SB/repo/research/pkm_falldegree_unblock_20261107"
  R="$s/runpod_capped"; mkdir -p "$R" "$s/runpod" "$SB/repo/research/prime_fall_degree_20261027"
  cp -a "$RC"/{common.sh,run.sh,fetch.sh,reap.sh,rp_pinned.sh,ledger.tsv,make_requests_m112.sh,requests_m112} "$R/"
  cp -a "$STUDY/image" "$STUDY/requests_r2i" "$s/"; cp -a "$STUDY/runpod/server.py" "$s/runpod/"
  cp -a "$STUDY/../prime_fall_degree_20261027/PREREGISTRATION.md" "$STUDY/../prime_fall_degree_20261027/run.sh" "$SB/repo/research/prime_fall_degree_20261027/"
  git -C "$SB/repo" init -q && git -C "$SB/repo" add -A && git -C "$SB/repo" -c user.email=t@t -c user.name=t commit -qm sandbox
  ST="$SB/stub"; mkdir -p "$ST" "$SB/bin" "$SB/state" "$ST/pretree/cgroup"; cp "$T/curl_stub.sh" "$SB/bin/curl"
  echo 137438953472 > "$ST/pretree/cgroup/memory.max"     # 128 GiB: what the boot loop copies before start
  jq '{costPerHr:1.04, cpuFlavorId:"cpu5m", vcpuCount:16, memoryInGb:128, containerDiskInGb:20, imageName:"python:3.12-slim",
       machine:{dataCenterId:"EU-RO-1", secureCloud:true}, machineId:"stubmachine", interruptible:null}' -n > "$ST/create-template.json"
  echo '[]' > "$ST/pods.json"
  export STUB_DIR="$ST" PATH="$SB/bin:$ORIGPATH" RPC_STATE="$SB/state" RPC_BIN="$RPC_BIN_SRC" RPC_LAYER="$RPC_LAYER_SRC"
  export RUNPOD_API_KEY=dummy-test-key RPC_POLL_S=0 HTTPS_PROXY=http://127.0.0.1:9 https_proxy=http://127.0.0.1:9
  unset SHA_FILE STUB_POD_ID STUB_CREATED_AGO
}
PRE_SB() { echo "$SB/repo/research/prime_fall_degree_20261027/PREREGISTRATION.md"; }
fresh_ledger() { sed -i "s/^# complete_as_of\t.*/# complete_as_of\t$(date -u +%FT%TZ)/" "$R/ledger.tsv"; }
freeze_heading() { printf '\n## Amendment 18 (test sandbox only)\n' >> "$(PRE_SB)"; }
freeze() { freeze_heading; (cd "$R" && sha256sum common.sh run.sh fetch.sh reap.sh) >> "$(PRE_SB)"; }
study_ready() { fresh_ledger; freeze; }
run() { ( cd "$R" && "$@" ) > "$OUTD/$ID.txt" 2>&1; echo $?; }
calls() { if [ -f "$ST/calls.log" ]; then wc -l < "$ST/calls.log"; else echo 0; fi; }
creates() { if [ -f "$ST/calls.log" ]; then grep -c -- '-X POST https://rest.runpod.io/v1/pods ' "$ST/calls.log" || true; else echo 0; fi; }
outcome() { # outcome NAME CLASS [OVERSIZE] [AS_M112] [RELAUNCH auto|owner]
  mkdir -p "$R/receipts/$1"; jq -n --arg c "$2" --argjson o "${3:-false}" --argjson m "${4:-false}" --arg r "${5:-auto}" \
    '{class:$c, row:{oversize:$o, timed_out:false}, memory:{as_m112:$m}} + (if $c == "LOST" then {relaunch:$r} else {} end)' > "$R/receipts/$1/outcome.json"; }
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
s_recexists() { study_ready; mkdir -p "$R/receipts/pkm-x-t14-k0-c6"; }
s_outexists() { study_ready; mkdir -p "$R/results/pkm-x-t14-k0-c6"; }
s_k0admitted() { study_ready; outcome pkm-x-t14-k0-c6 ADMITTED false; }
s_k0inflight() { study_ready; mkdir -p "$R/receipts/pkm-x-t14-k0-c6"; }
s_k0aborted() { study_ready; outcome pkm-x-t14-k0-c6 ABORTED; }
s_nofreeze() { fresh_ledger; }
s_nohashes() { fresh_ledger; freeze_heading; }
s_rpmod() { study_ready; echo '# x' >> "$R/rp_pinned.sh"; }
s_srvmod() { study_ready; echo '# x' >> "$R/../runpod/server.py"; }
s_binmod() { study_ready; local b="$SB/bin2"; cp -a "$RPC_BIN_SRC" "$b"; printf 'x' >> "$b/pfd_grid_zeros"; export RPC_BIN="$b"; }
s_nolayer() { study_ready; unset RPC_LAYER; }
s_stale() { freeze; }
s_malformed() { study_ready; printf 'broken line\n' >> "$R/ledger.tsv"; }
s_stateinside() { study_ready; export RPC_STATE="$R/state-inside"; }
s_prestart2() { study_ready; mkdir -p "$R/receipts/pkm-x-t14-k0-c6.prestart-1" "$R/receipts/pkm-x-t14-k0-c6.prestart-2"; }
s_m112edit() { study_ready; outcome pkm-x-t14-k0-c6 OOM; jq '.job.seconds = 90001' "$R/requests_m112/pfd-x-t14-k0-c6-m112.json" > "$SB/x" && mv "$SB/x" "$R/requests_m112/pfd-x-t14-k0-c6-m112.json"; }
s_t14done() { study_ready; outcome pkm-x-t14-k0-c6 ADMITTED false; }
s_r1lost() { study_ready; outcome pkm-x-t14-k0-c6 LOST; outcome pkm-x-t14-k0-c6-r1 LOST; }
s_t13lost() { study_ready; outcome pkm-x-t14-k0-c6 ADMITTED; outcome pkm-x-t14-k1-c6 LIMIT; outcome pkm-x-t13-k0-c6-r3 ABORTED; }
s_ledgerrow() { study_ready; printf 'pkm-x-t14-k0-c6/oldpod\trunpod_capped\tdraft Amendment 18\tclosed\t1.0000\treceipts/pkm-x-t14-k0-c6\n' >> "$R/ledger.tsv"; }
s_imgdisagree() { study_ready; jq '.layer.digest = "sha256:0000000000000000000000000000000000000000000000000000000000000000"' "$R/../image/image.json" > "$SB/x" && mv "$SB/x" "$R/../image/image.json"; }

deny_case A01 "name not on the allow-list (pkm-x-t14-k0-c6-r9-anything)" s_ready pkm-x-t14-k0-c6-r9-anything cap:14:0:6:12800000000
deny_case A02 "a second relaunch name (-r2)" s_ready pkm-x-t14-k0-c6-r2 cap:14:0:6:12800000000
deny_case A03 "SHA_FILE set" s_shafile pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A04 "receipts directory for the name already exists" s_recexists pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A05 "results directory for the name already exists" s_outexists pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A06 "order: t14-k1 requested before t14-k0" s_ready pkm-x-t14-k1-c6 cap:14:1:6:12800000000
deny_case A07 "order: -r1 requested with no base run" s_ready pkm-x-t14-k0-c6-r1 cap:14:0:6:12800000000
deny_case A08 "order: -r1 requested when the base is ADMITTED" s_k0admitted pkm-x-t14-k0-c6-r1 cap:14:0:6:12800000000
deny_case A09 "order: -m112 requested when the base is ADMITTED without oversize" s_k0admitted pkm-x-t14-k0-c6-m112 cap:14:0:6:27900000000
deny_case A10 "order: t14-k1 requested while t14-k0 is in flight (receipts, no outcome)" s_k0inflight pkm-x-t14-k1-c6 cap:14:1:6:12800000000
deny_case A11 "stage differs from the request (--max-dense 27900000000 on a 56Gi name)" s_ready pkm-x-t14-k0-c6 cap:14:0:6:27900000000
deny_case A12 "'## Amendment 18' not in PREREGISTRATION.md" s_nofreeze pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A13 "rp_pinned.sh modified (one line appended)" s_rpmod pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A14 "runpod/server.py modified (one line appended)" s_srvmod pkm-x-t14-k0-c6 cap:14:0:6:12800000000
if [ "$HAVE_LAYER" = 1 ]; then
deny_case A15 "pfd_grid_zeros with one byte appended" s_binmod pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A16 "RPC_LAYER missing for a study name" s_nolayer pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A17 "ledger complete_as_of 'never' (stale)" s_stale pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A18 "ledger with a malformed row" s_malformed pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A19 "RPC_STATE inside the git worktree" s_stateinside pkm-x-t14-k0-c6 cap:14:0:6:12800000000
else skip A15 A16 A17 A18 A19; fi
deny_case A20 "development stage with t = 12" s_none pkm-x-dev-a cap:12:0:5:12800000000
deny_case A21 "a name with two earlier pre-start failures" s_prestart2 pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A22 "-m112 request file edited (seconds 90001)" s_m112edit pkm-x-t14-k0-c6-m112 cap:14:0:6:27900000000
deny_case A23 "order: t13 -r3 requested while t14-k1 has not run" s_t14done pkm-x-t13-k0-c6-r3 cap:13:0:6:12800000000
deny_case A24 "order: -r1 after the -r1 was lost (no third attempt)" s_r1lost pkm-x-t14-k0-c6-r1 cap:14:0:6:12800000000
deny_case A25 "order: t13 -r3 ABORTED has no relaunch (all resolved)" s_t13lost pkm-x-t13-k0-c6-r3 cap:13:0:6:12800000000
ID=A26; mksb; touch "$ST/deny"; mkdir -p "$R/receipts_dev/pkm-x-dev-smoke1"; rc=$(run ./run.sh pkm-x-dev-smoke1 cap:8:0:5:12800000000)
[ "$rc" = 3 ] && [ "$(calls)" = 0 ]; rec A26 $? "exit 3, 0 API calls" "exit $rc, $(calls) API calls" "dev name with an existing receipts_dev dir"
deny_case A27 "N6: the name already has a ledger row (receipts lost)" s_ledgerrow pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A28 "S3: heading present but the route scripts' hashes are not in the preregistration" s_nohashes pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A29 "N1: -r1 after an ABORTED base (the attempt is used)" s_k0aborted pkm-x-t14-k0-c6-r1 cap:14:0:6:12800000000
deny_case A30 "S3: image.json layer digest disagrees with the pin in common.sh" s_imgdisagree pkm-x-t14-k0-c6 cap:14:0:6:12800000000

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
  if [ "$ok" != none ]; then mkdir -p "$d/cgroup"; echo max > "$d/cgroup/memory.max"; printf 'low 0\nhigh 0\nmax 0\noom %s\noom_kill %s\n' "$ok" "$ok" > "$d/cgroup/memory.events"; echo "$en" > "$d/cgroup/final-at"; fi
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
ID=B01; mksb; study_ready; echo '[{"id":"other1","name":"pkm-x-dev-zzz","desiredStatus":"RUNNING","createdAt":"2026-11-14 00:00:00.000 +0000 UTC"}]' > "$ST/pods.json"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 3 ] && [ "$(creates)" = 0 ] && [ ! -d "$R/receipts/pkm-x-t14-k0-c6" ]
rec B01 $? "exit 3, no create" "exit $rc, creates $(creates)" "a pkm-x- pod is live (serial rule)"
ID=B02; mksb; study_ready; echo '{"data":{"cpuFlavors":[{"specifics":{"stockStatus":null,"securePrice":1.04}}]}}' > "$ST/spec.json"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 10 ] && [ "$(creates)" = 0 ] && [ ! -d "$R/receipts/pkm-x-t14-k0-c6" ]
rec B02 $? "exit 10, no create, no receipts dir" "exit $rc, creates $(creates), dir $([ -d "$R/receipts/pkm-x-t14-k0-c6" ] && echo yes || echo no)" "cpu5m out of stock: no attempt"
ID=B03; mksb; study_ready; echo '{"data":{"cpuFlavors":[{"specifics":{"stockStatus":"High","securePrice":1.34}}]}}' > "$ST/spec.json"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 3 ] && [ "$(creates)" = 0 ]
rec B03 $? "exit 3, no create" "exit $rc, creates $(creates)" "price 1.34/h x 30 h (the one worst-case pod life) > US\$40"
ID=B04; mksb; study_ready; printf 'eks-all\teks\tA9-A15\tclosed\t370.00\ttest\n' >> "$R/ledger.tsv"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 3 ] && [ "$(creates)" = 0 ] && grep -q 'study ceiling' "$OUTD/B04.txt"
rec B04 $? "exit 3 (study ceiling)" "exit $rc, creates $(creates)" "ledger US\$370.01 + worst case US\$31.20 > US\$400"
ID=B05; mksb; study_ready; jq '.machine.secureCloud = false' "$ST/create-template.json" > "$SB/x" && mv "$SB/x" "$ST/create-template.json"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000)
[ "$rc" = 4 ] && grep -q 'DELETE https://rest.runpod.io/v1/pods/stubpod1' "$ST/calls.log" && [ -d "$R/receipts/pkm-x-t14-k0-c6.prestart-1" ] && [ ! -e "$R/receipts/pkm-x-t14-k0-c6" ] \
  && [ "$(jq -r .class "$R/receipts/pkm-x-t14-k0-c6.prestart-1/outcome.json")" = NOT_AN_ATTEMPT ] && [ "$(pods)" = 0 ] \
  && grep -qP '^pkm-x-t14-k0-c6/stubpod1\trunpod_capped\t.*\tclosed\t' "$R/ledger.tsv" && grep -q '"action":"DELETE"' "$R/receipts/pkm-x-t14-k0-c6.prestart-1/session-actions.jsonl"
rec B05 $? "exit 4, delete logged then made, receipts -> .prestart-1, ledger row closed" "exit $rc, pods left $(pods)" "create response not in the reserved tier (machine.secureCloud false)"
ID=B06; mksb; study_ready; jq '.costPerHr = 0.98' "$ST/create-template.json" > "$SB/x" && mv "$SB/x" "$ST/create-template.json"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 4 ] && [ "$(pods)" = 0 ] && [ -d "$R/receipts/pkm-x-t14-k0-c6.prestart-1" ]
rec B06 $? "exit 4, pod deleted" "exit $rc, pods left $(pods)" "costPerHr 0.98 differs from the price read before create (1.04)"
ID=B07; mksb; study_ready; touch "$ST/create-timeout"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 4 ] && [ "$(pods)" = 0 ] && [ -d "$R/receipts/pkm-x-t14-k0-c6.prestart-1" ] \
  && grep -q 'pre-start delete pod stubpod1' "$OUTD/B07.txt"
rec B07 $? "exit 4, pod found by name and deleted" "exit $rc, pods left $(pods)" "create call timed out but the pod exists (trap set before create)"
ID=B08; mksb; study_ready; touch "$ST/create-error"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 4 ] && compgen -G "$R/receipts/pkm-x-t14-k0-c6.nocreate-*" >/dev/null && [ ! -e "$R/receipts/pkm-x-t14-k0-c6" ]
rec B08 $? "exit 4, receipts -> .nocreate-*, name free" "exit $rc" "create refused by the API, no pod exists"
ID=B09; mksb; export STUB_CREATED_AGO=7200
rc=$(run ./run.sh pkm-x-dev-late cap:8:0:5:12800000000); [ "$rc" = 5 ] && [ -d "$R/receipts_dev/pkm-x-dev-late.prestart-1" ] && [ "$(pods)" = 0 ]
rec B09 $? "exit 5, pod deleted, not an attempt" "exit $rc" "runner would start more than 55 min after creation"
unset STUB_CREATED_AGO
ID=B10; mksb; rc=$(run ./run.sh pkm-x-dev-t1 cap:8:0:5:12800000000); D="$R/receipts_dev/pkm-x-dev-t1"; tok=$(cat "$SB/state/pkm-x-dev-t1.token")
ok=1; [ "$rc" = 0 ] && [ -f "$D/start.json" ] && [ -f "$D/pod-state.json" ] && jq -e '.run_deadline_provisional and .hard_deadline' "$D/pod-state.json" >/dev/null \
  && jq -e '.env.RUNNER_TOKEN == "<withheld>" and .cloudType == "SECURE" and .interruptible == false and .cpuFlavorIds == ["cpu5m"] and .dockerEntrypoint == ["bash","-c"]' "$D/pod-request.json" >/dev/null \
  && ! grep -rqF "$tok" "$D" && ! grep -F -- '-X POST https://rest.runpod.io/v1/pods' "$ST/calls.log" | grep -qF "$tok" && ! grep -qF dummy-test-key "$ST/calls.log" \
  && ! grep -rq consumerUserId "$D" && jq -e '.env.RUNNER_TOKEN' "$ST/create-body.json" >/dev/null \
  && jq -e '.cgroup_memory_limit == "137438953472"' "$D/cgroup-prestart.json" >/dev/null \
  && grep -q 'cgsnap' "$D/boot-template.sh" && grep -qP '^pkm-x-dev-t1/stubpod1\t.*\topen\t31.2000' "$R/ledger.tsv" && grep -q 'COMMIT AND PUSH NOW' "$OUTD/B10.txt" && ok=0
rec B10 $ok "exit 0; token and key absent from receipts and create argv; cgroup limit read before start; ledger row at 30 h; push reminder" \
  "exit $rc; X-Token argv lines (rp_pinned.sh, disclosed): $(grep -cF "$tok" "$ST/calls.log")" "development launch end to end against the stub"
cp "$ST/create-body.json" "$WORK/b10-create-body.json"
ID=B11; mksb; study_ready; rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000)
rc2=$( ( cd "$R" && ./run.sh pkm-x-t14-k1-c6 cap:14:1:6:12800000000 ) > "$OUTD/B11b.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$rc2" = 3 ] && grep -q "BLOCKED:pkm-x-t14-k0-c6" "$OUTD/B11b.txt" && grep -q 'RPC_BIN files are the files of image layer' "$OUTD/B11.txt" && grep -q 'git HEAD [0-9a-f]\{40\}' "$OUTD/B11.txt"
rec B11 $? "first study run starts (exit 0); the next one is refused while it is in flight (exit 3)" "exit $rc then $rc2" "study launch pkm-x-t14-k0-c6, then pkm-x-t14-k1-c6"
ID=B12; jq -r '.dockerStartCmd[0]' "$WORK/b10-create-body.json" > "$WORK/boot.sh"; bash -n "$WORK/boot.sh" > "$OUTD/B12.txt" 2>&1; r=$?
grep -q 'exec python3 -u -c "import base64;exec(base64.b64decode(' "$WORK/boot.sh"; r2=$?
b64=$(sed -n 's/.*b64decode(.\([A-Za-z0-9+\/=]*\).)).*/\1/p' "$WORK/boot.sh"); h=$(printf '%s' "$b64" | base64 -d | sha256sum | cut -d' ' -f1)
[ $r = 0 ] && [ $r2 = 0 ] && [ "$h" = f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550 ]
rec B12 $? "bash -n ok; embedded runner sha256 f56fd08e..." "bash -n $r; embedded sha256 ${h:0:8}" "the pod start command parses and carries the pinned server.py"
ID=B13; L="$WORK/bootloop"; mkdir -p "$L/cg" "$L/res"; printf 'max\n' > "$L/cg/memory.max"; printf 'low 0\nhigh 0\nmax 0\noom 1\noom_kill 1\n' > "$L/cg/memory.events"
sed -n '1,/^exec python3/p' "$WORK/boot.sh" | sed '$d' | sed "s#/sys/fs/cgroup#$L/cg#g; s#/results#$L/res#g; s#/tmp/cgsnap.tmp#$L/t.tmp#g" > "$L/loop.sh"
( bash "$L/loop.sh"; sleep 1; echo '{"state":"done"}' > "$L/res/runner_state.json"; sleep 7 ) > "$OUTD/B13.txt" 2>&1
[ -f "$L/res/cgroup/memory.max" ] && [ -f "$L/res/cgroup/memory.events" ] && [ -f "$L/res/cgroup/final-at" ] && grep -q 'oom_kill 1' "$L/res/cgroup/memory.events"
rec B13 $? "memory.max, memory.events and final-at copied" "$(ls "$L/res/cgroup" 2>/dev/null | tr '\n' ' ')" "the boot loop copies cgroup v2 files and writes its final copy after runner_state.json"
ID=B14; mksb; echo 60000000000 > "$ST/pretree/cgroup/memory.max"; rc=$(run ./run.sh pkm-x-dev-small cap:8:0:5:12800000000)
[ "$rc" = 4 ] && [ "$(pods)" = 0 ] && [ -d "$R/receipts_dev/pkm-x-dev-small.prestart-1" ] && ! grep -q '"/start' "$ST/calls.log" && ! grep -q '/start' "$ST/calls.log"
rec B14 $? "exit 4 before start, pod deleted" "exit $rc" "S4: cgroup memory.max 60,000,000,000 bytes (< 112 GiB)"
ID=B15; mksb; rm -f "$ST/pretree/cgroup/memory.max"; rc=$(run ./run.sh pkm-x-dev-nocg cap:8:0:5:12800000000)
[ "$rc" = 4 ] && [ "$(pods)" = 0 ] && ! grep -q '/start' "$ST/calls.log"
rec B15 $? "exit 4 before start, pod deleted" "exit $rc" "S4: no cgroup memory limit file before start (unobservable: owner)"
ID=B16; mksb; rc=$(run ./run.sh pkm-x-dev-r cap:8:0:5:12800000000); rm -rf "$R/receipts_dev/pkm-x-dev-r" "$SB/state/"*.token; echo '[]' > "$ST/pods.json"
rc2=$( (cd "$R" && ./run.sh pkm-x-dev-r cap:8:0:5:12800000000) > "$OUTD/B16b.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$rc2" = 3 ] && grep -q 'already has a row' "$OUTD/B16b.txt"
rec B16 $? "exit 0, then exit 3 (ledger row)" "exit $rc then $rc2" "N6: receipts and token lost after a launch, pod gone: the same name is refused"
else skip B01 B02 B03 B04 B05 B06 B07 B08 B09 B10 B11 B12 B13 B14 B15 B16; fi

if [ "$HAVE_LAYER" = 1 ]; then
# ===================================================================== C. fetch.sh flows against the stub
startrun C01; rc=$(fetch pkm-x-dev-c); [ "$rc" = 41 ] && [ "$(pods)" = 1 ] && [ "$(wc -l < "$D/checkins.jsonl")" = 1 ] && jq -e '.rest | has("lastStatusChange")' "$D/checkins.jsonl" >/dev/null
rec C01 $? "exit 41, pod kept, one check-in with lastStatusChange" "exit $rc" "running before the deadline"
startrun C02 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
rc=$(fetch pkm-x-dev-c); c1=$(cls "$D")
jq -n '{raw:[]}' > "$SB/rec-empty.json"; rc2=$(fetch pkm-x-dev-c --delete "$SB/rec-empty.json")
awk '{print $1}' "$D/local-sha256-1.txt" | jq -Rn '{raw:[inputs | {sha256:., uploaded:true}]}' > "$SB/rec-full.json"; rc3=$(fetch pkm-x-dev-c --delete "$SB/rec-full.json")
rc4=$(fetch pkm-x-dev-c)
[ "$rc" = 8 ] && [ "$c1" = "ADMITTED P2d" ] && [ "$rc2" = 8 ] && [ "$rc3" = 0 ] && [ "$rc4" = 9 ] && [ -f "$D/cost.json" ] && [ "$(pods)" = 0 ] \
  && grep -qP '^pkm-x-dev-c/stubpod1\t.*\tclosed\t' "$R/ledger.tsv" && [ -d "$RD/collect-1" ] && grep -q 'largest pilot memory peak' "$D/admission-lines-1.txt" \
  && jq -e '.classifier.version == "a18-r4" and (.classifier.fetch_sha256 | length) == 64' "$D/outcome.json" >/dev/null
rec C02 $? "exit 8 ADMITTED; empty record exit 8; full record exit 0; re-run exit 9; classifier pinned in outcome.json" "exit $rc $c1; $rc2; $rc3; $rc4" "completed run: collect, wait for the raw upload, delete, refuse re-collection"
startrun C03; rc=$(fetch pkm-x-dev-c --abort); [ "$rc" = 0 ] && [ "$(cls "$D")" = "ABORTED P5" ] && [ "$(pods)" = 0 ]
rec C03 $? "exit 0, ABORTED P5, pod deleted" "exit $rc $(cls "$D")" "session abort while running"
startrun C04; echo '{"state":"idle","exit":null,"started":null,"ended":null,"stages":null}' > "$ST/status.json"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LOST P3" ] && [ "$(rl "$D")" = owner ] && [ "$(pods)" = 0 ]
rec C04 $? "exit 0, LOST P3, relaunch owner (no RunPod-side status change)" "exit $rc $(cls "$D") $(rl "$D")" "runner idle after its start (container restarted)"
startrun C05 9000; touch "$ST/unreachable"; rest_set '.desiredStatus = "EXITED" | .lastStatusChange = "Exited by RunPod: host failure"'
rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LOST P3" ] && [ "$(rl "$D")" = auto ]
rec C05 $? "exit 0, LOST P3, relaunch auto" "exit $rc $(cls "$D") $(rl "$D")" "runner unreachable, REST EXITED with a RunPod-side lastStatusChange"
startrun C06 9000; touch "$ST/unreachable"; L0=$(jq -r .lastStartedAt "$D/pod-record-start.json")
jq -nc --arg a "$(iso $(( $(date +%s) - 7200 )))" --arg l "$L0" '{at:$a,phase:"regular",live:null,rest:{http:200,desiredStatus:"RUNNING",lastStartedAt:$l},listed:true}' >> "$D/checkins.jsonl"
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
rec C09 $? "exit 8 (upload wait), LIMIT P2a" "exit $rc $(cls "$D")" "runner done 90,050 s after its start"
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
rec C13 $? "exit 0, LOST P3, relaunch auto, cost upper bound" "exit $rc $(cls "$D") $(rl "$D")" "owner decision 13: pod gone (not listed, GET 404) before any logged session action"
startrun C14; rest_set '.lastStartedAt = "2026-11-15 01:00:00.000 +0000 UTC"'
rc=$(fetch pkm-x-dev-c); [ "$rc" = 41 ] && [ "$(pods)" = 1 ]
rec C14 $? "exit 41, pod kept" "exit $rc" "S1: lastStartedAt changed while the runner reports running under s0"
startrun C15; touch "$ST/delete-fail"; rc=$(fetch pkm-x-dev-c --abort); rm -f "$ST/delete-fail"; rc2=$(fetch pkm-x-dev-c)
[ "$rc" = 7 ] && [ "$rc2" = 0 ] && [ -f "$D/cost.json" ] && [ "$(cls "$D")" = "ABORTED P5" ]
rec C15 $? "exit 7 (still listed), then exit 0 on the next call" "exit $rc then $rc2" "a failed delete is resumed by the next fetch.sh call"
ID=C16; mksb; run ./run.sh pkm-x-dev-c cap:8:0:5:12800000000 >/dev/null; D="$R/receipts_dev/pkm-x-dev-c"; rm -f "$D/start.json"
echo '{"state":"idle","exit":null,"started":null,"ended":null,"stages":null}' > "$ST/status.json"; rc=$(fetch pkm-x-dev-c --abort)
[ "$rc" = 0 ] && [ -d "$R/receipts_dev/pkm-x-dev-c.prestart-1" ] && [ "$(jq -r .class "$R/receipts_dev/pkm-x-dev-c.prestart-1/outcome.json")" = NOT_AN_ATTEMPT ]
rec C16 $? "exit 0, NOT_AN_ATTEMPT, receipts -> .prestart-1" "exit $rc" "launcher died before start (no start.json, runner idle)"
startrun C17; tok=$(cat "$SB/state/stubpod1.token"); rm -f "$SB/state/stubpod1.token" "$SB/state/pkm-x-dev-c.token"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 41 ] && [ "$(cat "$SB/state/stubpod1.token")" = "$tok" ] && ! grep -rqF "$tok" "$D" && grep -q 'token recovered' "$D/fetch.log"
rec C17 $? "exit 41, token recovered into RPC_STATE only" "exit $rc" "N6: the pod token file is missing"
startrun C18 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
pre=$(jq -nc --arg a "$(iso $(( $(date +%s) - 300 )))" --arg s "$S0" --arg e "$E" '{at:$a,phase:"prefetch",live:{state:"done",exit:0,started:$s,ended:$e},rest:{http:200,desiredStatus:"RUNNING"},listed:true}')
jq -n --arg at "$(iso $(( $(date +%s) - 290 )))" --argjson p "$pre" '{at:$at, reason:"done", mode:"", prefetch:$p, out:"results/pkm-x-dev-c/collect-1"}' > "$D/collect-started-1.json"
mkdir -p "$RD"; cp -a "$ST/tree" "$RD/collect-1"; (cd "$RD/collect-1" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -r -d '\n' sha256sum) > "$D/local-sha256-1.txt"; echo '{"transfer_ok":true}' > "$D/transfer-1.json"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 8 ] && jq -e '.recovered == true and .transfer_ok == true' "$D/collect-1.json" >/dev/null && [ "$(cls "$D")" = "ADMITTED P2d" ] && [ ! -e "$RD/collect-2" ]
rec C18 $? "exit 8, orphan recovered (verified), ADMITTED P2d, no second collection" "exit $rc $(cls "$D")" "N3b: fetch.sh died after keeping the tree, before collect-1.json"
startrun C18b 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
jq -n --arg at "$(iso $(( $(date +%s) - 290 )))" --arg s "$S0" --arg e "$E" '{at:$at, reason:"done", mode:"", prefetch:{at:$at,phase:"prefetch",live:{state:"done",exit:0,started:$s,ended:$e},rest:{http:200,desiredStatus:"RUNNING"},listed:true}, out:"x"}' > "$D/collect-started-1.json"
mkdir -p "$RD"; cp -a "$ST/tree" "$RD/collect-1"; (cd "$RD/collect-1" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -r -d '\n' sha256sum) > "$D/local-sha256-1.txt"; echo tampered >> "$RD/collect-1/host.txt"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 8 ] && jq -e '.recovered == true and .transfer_ok == false' "$D/collect-1.json" >/dev/null && [ -d "$RD/collect-1" ] && [ -d "$RD/collect-2" ] && [ "$(cls "$D")" = "NO_EVIDENCE P0b" ]
rec C18b $? "orphan kept (unverifiable), new collection 2, trees differ: NO_EVIDENCE P0b" "exit $rc $(cls "$D")" "N3b: an orphan that disagrees with a later collection of the same pod goes to the owner"
startrun C19; printf '{"at":"2026-11-15T00:00:00Z","phase":"reg' >> "$D/checkins.jsonl"; rc=$(fetch pkm-x-dev-c); [ "$rc" = 12 ] && [ "$(pods)" = 1 ]
rec C19 $? "exit 12 (owner), pod kept" "exit $rc" "S2: a truncated check-in line (fail closed)"
else skip C01 C02 C03 C04 C05 C06 C07 C08 C08b C09 C10 C11 C12 C13 C14 C15 C16 C17 C18 C18b C19; fi

# ===================================================================== D. the classifier alone, on synthetic receipt sets
S0E=$(( $(date +%s) - 200000 )); S0=$(iso $S0E); DLE=$(( S0E + 90000 )); L0="2026-11-10 10:00:00.000 +0000 UTC"; LSC0="Rented by User: x"
mkset() { # mkset DIR: a receipts dir with the launch receipts of a run started at S0
  local d=$1; rm -rf "$d"; mkdir -p "$d"
  jq -n --arg l "$L0" --arg s "$LSC0" '{id:"p1", cpuFlavorId:"cpu5m", vcpuCount:16, memoryInGb:128, costPerHr:1.04, lastStartedAt:$l, lastStatusChange:$s, createdAt:"2026-11-10 10:00:00.000 +0000 UTC", machine:{secureCloud:true, dataCenterId:"EU-RO-1"}}' > "$d/pod-create.json"
  jq -n --arg l "$L0" --arg s "$LSC0" '{lastStartedAt:$l, lastStatusChange:$s, desiredStatus:"RUNNING"}' > "$d/pod-record-start.json"
  jq -n '{cloudType:"SECURE", interruptible:false, cpuFlavorIds:["cpu5m"]}' > "$d/pod-request.json"
  echo '{"instance":"cpu5m-16-128","stockStatus":"High","securePrice":1.04}' > "$d/price.jsonl"
  jq -n '{stage:"cap:8:0:5:12800000000", mem_floor_gb:61}' > "$d/pod-state.json"
  jq -nc --arg s "$S0" '{state:"running",exit:null,started:$s,ended:null,stages:["cap:8:0:5:12800000000"]}' > "$d/start.json"
  : > "$d/checkins.jsonl"
}
ck() { # ck DIR AT_EPOCH PHASE LIVE(state|null) STARTED ENDED EXIT HTTP DESIRED LSA LISTED [LASTSTATUSCHANGE]
  local live=null; [ "$4" != null ] && live=$(jq -nc --arg st "$4" --arg s "$5" --arg e "$6" --argjson x "$7" '{state:$st, exit:$x, started:(if $s == "" then null else $s end), ended:(if $e == "" then null else $e end)}')
  jq -nc --arg a "$(iso "$2")" --arg ph "$3" --argjson live "$live" --argjson h "$8" --arg d "$9" --arg l "${10}" --argjson li "${11}" --arg sc "${12:-$LSC0}" \
    '{at:$a, phase:$ph, live:$live, rest:(if $h == 200 then {http:$h, desiredStatus:$d, lastStartedAt:$l, lastStatusChange:$sc} else {http:$h} end), listed:$li}' >> "$1/checkins.jsonl"
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
nd D02; done_set; dcase D02 "ADMITTED P2d" "completed, every admission rule holds"
nd D03; E=$(iso $(( S0E + 90050 ))); mktree "$TD" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; ck "$DR" $((S0E + 90100)) prefetch done "$S0" "$E" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((S0E + 90200)); dcase D03 "LIMIT P2a" "completed 90,050 s after the runner start"
nd D04; ck "$DR" $((DLE + 60)) regular running "$S0" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" deadline false $((DLE + 70)); dcase D04 "LIMIT P4" "running at the deadline check-in"
nd D05; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D05 "LOST P3" "runner idle before the deadline, no RunPod-side status change" owner
nd D06; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 200 RUNNING "2026-11-11 00:00:00.000 +0000 UTC" true; dcase D06 "PENDING P8" "S1: lastStartedAt changed while the runner runs under s0: not evidence"
nd D06b; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 RUNNING "2026-11-11 00:00:00.000 +0000 UTC" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D06b "LOST P3" "lastStartedAt changed and the runner is idle" owner
nd D07; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 RUNNING "$L0" true; ck "$DR" $((S0E + 7700)) regular null "" "" null 500 "" "" true; dcase D07 "PENDING P8" "N2: unreachable twice >= 1 h apart, REST unreadable (HTTP 500): no information"
nd D07b; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 200 EXITED "$L0" true "Exited by RunPod: host failure"; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D07b "LOST P3" "REST desiredStatus EXITED with a RunPod-side status change" auto
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
nd D16; done_set; ck "$DR" $((okEe + 900)) regular idle "" "" null 200 RUNNING "2026-11-12 00:00:00.000 +0000 UTC" true; dcase D16 "ADMITTED P2d" "completed, receipts fetched, then restarted"
nd D16b; mktree "$TD" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((okEe + 50)) regular idle "" "" null 200 RUNNING "$L0" true; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D16b "LOST P3" "restart recorded before the completion was fetched" owner
nd D17; done_set; jq '.mode = "--abort"' "$DR/collect-1.json" > "$DR/c" && mv "$DR/c" "$DR/collect-1.json"; dcase D17 "ADMITTED P2d" "--abort on a run that is already done cannot change its class"
nd D18; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" abort false $((S0E + 4010)); dcase D18 "LOST P3" "--abort after a restart is LOST, not ABORTED" owner
nd D19; mktree "$TD" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0 2; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D19 "LOST P3" "two start lines in progress.txt (a second runner start)" owner
nd D20; mktree "$TD" cap:8:0:5:12800000000 "$(iso $((S0E + 30)))" "$okE" 0 0 0; ck "$DR" $((okEe + 100)) prefetch done "$(iso $((S0E + 30)))" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D20 "LOST P3" "the runner's start differs from start.json (manual re-start)" owner
nd D21; mktree "$TD" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((S0E + 93600)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" hard false $((S0E + 93610)); dcase D21 "NO_EVIDENCE P7" "N2: completed in time but the transfer check still failed at the hard deadline"
nd D22; done_set; echo tampered >> "$TD/host.txt"; dcase D22 "NOT_ADMITTED P2e" "a file of the kept tree changed after collection (rule 6)"
nd D23; done_set; jq '.machine.secureCloud = false' "$DR/pod-create.json" > "$DR/c" && mv "$DR/c" "$DR/pod-create.json"; dcase D23 "NOT_ADMITTED P2e" "create response not in the reserved tier (rule 7)"
nd D24; ck "$DR" $((S0E + 4000)) regular null "" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D24 "LOST P3" "owner decision 13: pod gone from the list and GET 404, before any logged session action" auto
nd D24c; printf '{"at":"%s","action":"DELETE","pod":"p1","reason":"x"}\n' "$(iso $((S0E + 3000)))" > "$DR/session-actions.jsonl"; ck "$DR" $((S0E + 4000)) regular null "" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D24c "LOST P3" "pod gone and GET 404 after a logged session delete" owner
nd D24d; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 RUNNING "$L0" false; dcase D24d "PENDING P8" "pod absent from the list but GET returns a healthy RUNNING record: no loss evidence, wait"
nd D24b; ck "$DR" $((S0E + 4000)) regular null "" "" null 0 "" "" false; dcase D24b "PENDING P8" "S6: missing from the list but GET unreadable (not corroborated)"
# Review round 3, R1 and R2: host-side evidence must be tied to the loss.
nd D43; ck "$DR" $((S0E + 20000)) regular running "$S0" "" null 200 RUNNING "$L0" true "Migrated: host maintenance"; ck "$DR" $((S0E + 30000)) regular idle "" "" null 200 RUNNING "$L0" true "Restarted by User: x"; collect "$DR" "$DO" lost false $((S0E + 30010)); dcase D43 "LOST P3" "R1 probe P1: an earlier RunPod-side change, then a restart whose status names the user" owner
nd D43b; ck "$DR" $((S0E + 20000)) regular running "$S0" "" null 200 RUNNING "$L0" true; ck "$DR" $((S0E + 25000)) regular null "" "" null 200 RUNNING "$L0" true "Migrated: host failure"; ck "$DR" $((S0E + 30000)) regular idle "" "" null 0 "" "" null; collect "$DR" "$DO" lost false $((S0E + 30010)); dcase D43b "LOST P3" "R1: a RunPod-side change after the last alive check-in and before the loss" auto
nd D43c; ck "$DR" $((S0E + 20000)) regular running "$S0" "" null 200 RUNNING "$L0" true "Migrated: host maintenance"; ck "$DR" $((S0E + 30000)) regular idle "" "" null 0 "" "" null; collect "$DR" "$DO" lost false $((S0E + 30010)); dcase D43c "LOST P3" "R1: the only RunPod-side change was seen while the runner was still alive" owner
nd D44; ck "$DR" $((S0E + 80000)) regular done "$S0" "$(iso $((S0E + 79000)))" 0 200 RUNNING "$L0" true; ck "$DR" $((S0E + 82000)) regular null "" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((S0E + 82010)); dcase D44 "LOST P3" "R2 probe P2: runner seen done, then gone with GET 404 before the run deadline" owner
nd D44b; ck "$DR" $((S0E + 80000)) regular done "$S0" "$(iso $((S0E + 79000)))" 0 200 RUNNING "$L0" true; ck "$DR" $((DLE + 3600)) regular null "" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((DLE + 3610)); dcase D44b "LOST P6" "R2 probe P3: runner seen done, then gone with GET 404 after the run deadline" owner
P2E=$(( okEe + 100 )); PCOL=$(( okEe + 200 ))
mcase() { # mcase ID STAGE PILOT_EXIT OOM CG_PEAK LOG EXPECTED DESCRIPTION [EXPECTED_AS_M112] [EXPECTED_SOURCE_USED] [OVERSIZE]
  nd "$1"; local rx=0; [ "$3" = 0 ] || rx=1
  mktree "$TD" "$2" "$S0" "$okE" "$3" 0 "$4" 1 "$5" "$6" "${11:-false}"; jq --arg st "$2" '.stage = $st' "$DR/pod-state.json" > "$DR/x" && mv "$DR/x" "$DR/pod-state.json"
  ck "$DR" $P2E prefetch done "$S0" "$okE" $rx 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $PCOL
  local got; got=$( (cd "$RC" && ./fetch.sh --classify-dir "$DR" "$DO") 2> "$OUTD/$1.err" | tee "$OUTD/$1.txt" | jq -r '.class + " " + .rule + " as_m112=" + ((.memory.as_m112 // false) | tostring) + " source=" + (.memory.source_used // "none")')
  local want="$7 as_m112=${9:-false} source=${10:-pilot_log_VmHWM_MiB}"; [ "$got" = "$want" ]; rec "$1" $? "$want" "$got" "$8"
}
G=1073741824; S56=cap:8:0:5:12800000000; S112=cap:8:0:5:27900000000
# Fixtures state both figures: "pilot N MiB" is a VmHWM line in the F4 log (the fixture log's own lines peak at 64 MiB),
# "cgroup N GiB" is a memory.peak file (or v1 max_usage_in_bytes). The pilot figure is used when present.
mcase D25 $S56 0 0 - 30720 "ADMITTED P2d" "memory: pilot 30,720 MiB (30 GiB), no cgroup file -> pilot, below 56 GiB"
mcase D26 $S56 0 0 - 71680 "ADMITTED P2m3" "memory: pilot 71,680 MiB (70 GiB), no cgroup file -> pilot, between 56 and 112 GiB, oversize false: the cell's -m112 row" true
mcase D27 $S56 0 0 - 128000 "OOM P2m1" "memory: pilot 128,000 MiB (125 GiB), no cgroup file -> pilot, above 112 GiB: OOM at -m112" true
mcase D28 $S56 0 0 $((70 * G)) keep "ADMITTED P2d" "memory: pilot 64 MiB and cgroup 70 GiB -> pilot (primary); cgroup ignored"
mcase D28b $S56 0 0 $((70 * G)) strip "ADMITTED P2m3" "memory: no pilot line, cgroup v2 70 GiB -> cgroup fallback" true cgroup_v2_memory.peak
mcase D29 $S56 0 0 - strip "NOT_ADMITTED P2u" "memory: no pilot line, no cgroup file -> none" false none
mcase D30 $S56 0 0 v1:$((60 * G)) strip "ADMITTED P2m3" "memory: no pilot line, cgroup v1 max_usage 60 GiB -> v1 fallback" true cgroup_v1_memory.max_usage_in_bytes
mcase D31 $S112 0 0 - 115712 "OOM P2p" "memory: -m112 attempt, pilot 115,712 MiB (113 GiB) -> OOM (owner decision 15)"
mcase D32 $S112 0 0 - 102400 "ADMITTED P2d" "memory: -m112 attempt, pilot 102,400 MiB (100 GiB), no cgroup file -> pilot"
mcase D33 $S56 0 0 - 57344 "ADMITTED P2d" "memory: pilot exactly 57,344 MiB (56 GiB, not above), no cgroup file -> pilot"
mcase D34 $S56 137 0 - 71680 "LOST P2m2" "memory: pilot 70 GiB, exit 137, oom_kill 0 -> pilot: a lost -m112 attempt" true
mcase D35 $S56 0 0 $((120 * G)) 92160 "ADMITTED P2m3" "memory: cgroup 120 GiB (> 112 GiB) but pilot 92,160 MiB (90 GiB) -> pilot: not OOM, the cell's -m112 row" true
mcase D35b $S112 0 0 $((120 * G)) 92160 "ADMITTED P2d" "memory: -m112 attempt, cgroup 120 GiB but pilot 90 GiB -> pilot: P2p does not fire"
mcase D36 $S56 137 1 - 30720 "OOM P2b" "memory: pilot 30 GiB but exit 137 with oom_kill 1 -> OOM whatever the peak"
mcase D37 $S56 0 0 - 71680 "ADMITTED P2d" "N4: pilot 70 GiB with row oversize true: not the -m112 attempt; Amendment 14's oversize path (-m112 follows)" false pilot_log_VmHWM_MiB true
mcase D37b $S56 0 0 - 128000 "ADMITTED P2d" "N4: pilot 125 GiB with row oversize true: the oversize row, as Amendment 14 says" false pilot_log_VmHWM_MiB true
mcase D37c $S56 0 0 - 114688 "ADMITTED P2m3" "boundary: pilot exactly 112 GiB (not above), oversize false" true
nd D38; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 STOPPED "$L0" true "Stopped by User: x"; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D38 "LOST P3" "N3a: status change that names the user" owner
nd D39; printf '{"at":"%s","action":"DELETE","pod":"p1","reason":"x"}\n' "$(iso $((S0E + 3000)))" > "$DR/session-actions.jsonl"; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 EXITED "$L0" true "Exited by RunPod: x"; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D39 "LOST P3" "N3a: RunPod-side change recorded only after the session logged a delete" owner
nd D40; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 RUNNING "$L0" true; printf '{"at":"broken' >> "$DR/checkins.jsonl"; dcase D40 "NO_EVIDENCE P0" "S2: a truncated check-in line fails closed"
nd D40b; done_set; printf 'not json' > "$DR/collect-1.json"; dcase D40b "NO_EVIDENCE P0" "S2: an unparsable collection record fails closed"
nd D41; done_set; cp -a "$TD" "$DO/collect-2"; echo x >> "$DO/collect-2/host.txt"; collect "$DR" "$DO" done true $((okEe + 300)) 2; dcase D41 "NO_EVIDENCE P0b" "N3b: two done collections of the pod disagree file by file"
nd D41b; done_set; cp -a "$TD" "$DO/collect-2"; collect "$DR" "$DO" done true $((okEe + 300)) 2; dcase D41b "ADMITTED P2d" "N3b: two done collections agree"
nd D42; : > "$DR/start.json"; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 200 RUNNING "$L0" true; dcase D42 "PENDING P8" "S2: a zero-byte start.json (no response) is read as absent; the start comes from the check-in"

# ===================================================================== E. the registered order (next_allowed)
ecase() { # ecase ID EXPECTED DESCRIPTION outcome-specs... (name:class[:oversize[:as_m112[:relaunch]]])
  local id=$1 want=$2 desc=$3; shift 3; mksb; local s n c o m r
  for s in "$@"; do IFS=: read -r n c o m r <<<"$s"; if [ "$c" = INFLIGHT ]; then mkdir -p "$R/receipts/$n"; else outcome "$n" "$c" "${o:-false}" "${m:-false}" "${r:-auto}"; fi; done
  local got; got=$(cd "$R" && bash -c '. ./common.sh; next_allowed' 2>&1); [ "$got" = "$want" ]; rec "$id" $? "$want" "$got" "$desc"
}
ecase E01 pkm-x-t14-k0-c6 "nothing run yet"
ecase E02 pkm-x-t14-k0-c6-r1 "base LOST with host-side evidence (relaunch auto)" pkm-x-t14-k0-c6:LOST
ecase E03 pkm-x-t14-k1-c6 "N1: base ABORTED uses the attempt; no relaunch" pkm-x-t14-k0-c6:ABORTED
ecase E04 pkm-x-t14-k1-c6 "base ADMITTED, no oversize" pkm-x-t14-k0-c6:ADMITTED
ecase E05 pkm-x-t14-k0-c6-m112 "base ADMITTED with oversize true" pkm-x-t14-k0-c6:ADMITTED:true
ecase E06 pkm-x-t14-k0-c6-m112 "base OOM" pkm-x-t14-k0-c6:OOM
ecase E07 pkm-x-t14-k0-c6-m112-r1 "base OOM, -m112 LOST (auto), relaunch unused" pkm-x-t14-k0-c6:OOM pkm-x-t14-k0-c6-m112:LOST
ecase E08 pkm-x-t14-k1-c6 "base LOST, -r1 OOM, -m112 LOST: relaunch already used" pkm-x-t14-k0-c6:LOST pkm-x-t14-k0-c6-r1:OOM pkm-x-t14-k0-c6-m112:LOST
ecase E09 pkm-x-t14-k1-c6 "base LIMIT (censored, not re-run)" pkm-x-t14-k0-c6:LIMIT
ecase E10 pkm-x-t14-k1-c6 "base NOT_ADMITTED (owner)" pkm-x-t14-k0-c6:NOT_ADMITTED
ecase E11 pkm-x-t13-k0-c6-r3 "both t14 cells resolved" pkm-x-t14-k0-c6:ADMITTED pkm-x-t14-k1-c6:LIMIT
ecase E12 ALL_RESOLVED "t13 -r3 LOST: no relaunch" pkm-x-t14-k0-c6:ADMITTED pkm-x-t14-k1-c6:ADMITTED pkm-x-t13-k0-c6-r3:LOST
ecase E13 pkm-x-t13-k0-c6-m112 "t13 -r3 OOM" pkm-x-t14-k0-c6:ADMITTED pkm-x-t14-k1-c6:ADMITTED pkm-x-t13-k0-c6-r3:OOM
ecase E14 BLOCKED:pkm-x-t14-k0-c6 "base in flight" pkm-x-t14-k0-c6:INFLIGHT
ecase E15 ALL_RESOLVED "t13 -m112 LOST: no relaunch for t13" pkm-x-t14-k0-c6:ADMITTED pkm-x-t14-k1-c6:ADMITTED pkm-x-t13-k0-c6-r3:OOM pkm-x-t13-k0-c6-m112:LOST
ecase E16 pkm-x-t14-k1-c6 "base ADMITTED as the -m112 attempt (peak 56-112 GiB): cell resolved" pkm-x-t14-k0-c6:ADMITTED:false:true
ecase E17 pkm-x-t14-k1-c6 "base OOM as the -m112 attempt (peak > 112 GiB): censored, no -m112 run" pkm-x-t14-k0-c6:OOM:false:true
ecase E18 pkm-x-t14-k0-c6-m112-r1 "base LOST (auto) as the -m112 attempt: the -m112 relaunch follows" pkm-x-t14-k0-c6:LOST:false:true
ecase E19 pkm-x-t14-k1-c6 "base LOST, -r1 LOST as the -m112 attempt: relaunch used" pkm-x-t14-k0-c6:LOST pkm-x-t14-k0-c6-r1:LOST:false:true
ecase E20 ALL_RESOLVED "t13 -r3 LOST as the -m112 attempt: no relaunch" pkm-x-t14-k0-c6:ADMITTED pkm-x-t14-k1-c6:ADMITTED pkm-x-t13-k0-c6-r3:LOST:false:true
ecase E21 pkm-x-t14-k1-c6 "N3a: base LOST without host-side evidence (relaunch owner)" pkm-x-t14-k0-c6:LOST:false:false:owner
ecase E22 pkm-x-t14-k1-c6 "N2: base NO_EVIDENCE goes to the owner" pkm-x-t14-k0-c6:NO_EVIDENCE
ecase E23 pkm-x-t14-k0-c6-m112 "N4: as_m112 with row oversize true is not the -m112 attempt (the review's case)" pkm-x-t14-k0-c6:ADMITTED:true:true
ecase E24 pkm-x-t14-k1-c6 "-m112 ABORTED: no -m112-r1" pkm-x-t14-k0-c6:OOM pkm-x-t14-k0-c6-m112:ABORTED

# ===================================================================== F. reap.sh against the stub
ID=F01; mksb; old=$(rpd $(( $(date +%s) - 27 * 3600 ))); young=$(rpd $(( $(date +%s) - 3600 )))
jq -n --arg o "$old" --arg y "$young" '[{id:"old1",name:"pkm-x-t14-k0-c6",desiredStatus:"RUNNING",createdAt:$o,lastStartedAt:$o},
  {id:"young1",name:"pkm-x-dev-y",desiredStatus:"RUNNING",createdAt:$y,lastStartedAt:$y},
  {id:"other1",name:"pfd-b-rp",desiredStatus:"RUNNING",createdAt:$o,lastStartedAt:$o}]' > "$ST/pods.json"
rc=$(run ./reap.sh); left=$(jq -c '[.[].id]' "$ST/pods.json"); rc2=$( (cd "$R" && ./reap.sh) >> "$OUTD/F01.txt" 2>&1; echo $?)
left2=$(jq -c '[.[].id]' "$ST/pods.json"); lines=$(cat "$R"/reaper/reap-*.jsonl | wc -l)
[ "$rc" = 0 ] && [ "$left" = '["young1","other1"]' ] && [ "$rc2" = 0 ] && [ "$left2" = '["young1","other1"]' ] && grep -q '"due":true' "$R"/reaper/reap-*.jsonl && grep -q '"action":"DELETE"' "$R/reaper/session-actions.jsonl"
rec F01 $? "pod past creation + 26 h deleted (logged first); young and non-pkm-x pods kept; second pass changes nothing" "exit $rc, left $left; exit $rc2, left $left2; $lines receipt lines" "reaper: 27 h old pod without receipts, 1 h old pod, other session's pod"
ID=F08; mksb; old=$(rpd $(( $(date +%s) - 27 * 3600 ))); jq -n --arg o "$old" '[{id:"old1",name:"pkm-x-t14-k0-c6",desiredStatus:"RUNNING",createdAt:$o,lastStartedAt:$o}]' > "$ST/pods.json"
( flock "$SB/state/pkm-x-t14-k0-c6.lock" sleep 6 ) & lp=$!; sleep 1
rc=$(run ./reap.sh); wait $lp
[ "$rc" = 0 ] && [ "$(pods)" = 1 ] && grep -q 'lock held' "$R"/reaper/reap-*.jsonl && ! grep -q 'DELETE https://rest.runpod.io/v1/pods/old1' "$ST/calls.log"
rec F08 $? "pod untouched, skip recorded" "exit $rc, pods $(pods)" "S1: a due pod whose per-name lock is held by another process"
ID=F03; mksb; touch "$ST/deny"; rc=$(run ./reap.sh); [ "$rc" = 1 ]; rec F03 $? "exit 1" "exit $rc" "reaper: pod list unreadable (network blocked)"
ID=F07; mksb; jq -n '[{id:"bad1",name:"pkm-x-dev-q",desiredStatus:"RUNNING",createdAt:"garbage"},{id:"nul1",name:null,desiredStatus:"RUNNING"}]' > "$ST/pods.json"
rc=$(run ./reap.sh); [ "$rc" = 1 ] && [ "$(pods)" = 2 ]; rec F07 $? "exit 1, nothing deleted" "exit $rc, pods $(pods)" "S6: unparsable createdAt and a null name: reported, not deleted"
if [ "$HAVE_LAYER" = 1 ]; then
ID=F02; startrun F02 90100; hard_past; rc=$( (cd "$R" && ./reap.sh) > "$OUTD/F02.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$(pods)" = 0 ] && [ "$(cls "$D")" = "LIMIT P4" ] && [ -f "$D/cost.json" ]
rec F02 $? "pod collected by fetch.sh (LIMIT P4) and deleted" "exit $rc $(cls "$D")" "reaper: a run past its hard deadline, with receipts"
ID=F04; startrun F04; rm -f "$D/start.json" "$D/pod-state.json"; echo '{"state":"idle","exit":null,"started":null,"ended":null,"stages":null}' > "$ST/status.json"
ago=$(rpd $(( $(date +%s) - 27 * 3600 ))); jq --arg a "$ago" 'map(.createdAt = $a)' "$ST/pods.json" > "$SB/x" && mv "$SB/x" "$ST/pods.json"
jq --arg a "$ago" '.createdAt = $a' "$D/pod-create.json" > "$SB/x" && mv "$SB/x" "$D/pod-create.json"
rc=$( (cd "$R" && ./reap.sh) > "$OUTD/F04.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$(pods)" = 0 ] && [ -d "$R/receipts_dev/pkm-x-dev-c.prestart-1" ]
rec F04 $? "pod deleted through fetch.sh, NOT_AN_ATTEMPT" "exit $rc" "reaper: launcher died, no deadline recorded, creation + 26 h passed"
ID=F05; startrun F05; rm -f "$D/start.json"; ago=$(rpd $(( $(date +%s) - 5400 )))
jq --arg a "$ago" 'map(.createdAt = $a)' "$ST/pods.json" > "$SB/x" && mv "$SB/x" "$ST/pods.json"; jq --arg a "$ago" '.createdAt = $a' "$D/pod-create.json" > "$SB/x" && mv "$SB/x" "$D/pod-create.json"
rc=$( (cd "$R" && ./reap.sh) > "$OUTD/F05.txt" 2>&1; echo $?); rc2=$(fetch pkm-x-dev-c)
[ "$rc" = 0 ] && [ "$(pods)" = 1 ] && ! grep -q 'DELETE https://rest.runpod.io/v1/pods/stubpod1' "$ST/calls.log" && [ "$rc2" = 41 ]
rec F05 $? "pod untouched; the next fetch.sh waits (41)" "exit $rc, pods $(pods), fetch $rc2" "N3c, the review's probe: start.json missing, runner running, created 90 min ago"
ID=F06; startrun F06 90100; touch "$ST/unreachable"; rc=$( (cd "$R" && ./reap.sh) > "$OUTD/F06.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$(pods)" = 1 ] && ! grep -q 'DELETE https://rest.runpod.io/v1/pods/stubpod1' "$ST/calls.log"
rec F06 $? "pod untouched" "exit $rc, pods $(pods)" "N3c: past the run deadline but before the hard deadline, runner unreachable"
else skip F02 F04 F05 F06; fi

# ===================================================================== G. syntax
for f in common.sh run.sh fetch.sh reap.sh rp_pinned.sh make_requests_m112.sh tests/run_tests.sh tests/curl_stub.sh; do
  bash -n "$RC/$f" 2> "$OUTD/G-${f##*/}.txt"; r=$?; rec "G-${f##*/}" $r "bash -n ok" "rc $r" "syntax of $f"
done
sha256sum "$RC/rp_pinned.sh" | grep -q '^b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d'; rec G-pin $? "rp_pinned.sh sha256 b4b60199..." "$(sha256sum "$RC/rp_pinned.sh" | cut -c1-8)" "pinned client copy unchanged"
ntot=$((npass + nfail + nskip)); nlayer=$(echo A15 A16 A17 A18 A19 $(printf 'B%02d ' $(seq 1 16)) C01 C02 C03 C04 C05 C06 C07 C08 C08b C09 C10 C11 C12 C13 C14 C15 C16 C17 C18 C18b C19 F02 F04 F05 F06 | wc -w)

if [ "$WRITE" = 1 ]; then
  rm -rf "$T/out"; cp -a "$OUTD" "$T/out"; cp "$RES" "$T/results.tsv"
  { cat <<INTRO
# Offline tests of the capped RunPod route (draft Amendment 18)

\`run_tests.sh\` runs every case in a fresh sandbox copy of the route (its own git repository, receipts, ledger,
\`RPC_STATE\` and stub state) under \`\$TMPDIR\`. No pod is created and no network call is made: \`curl_stub.sh\` is installed
as \`curl\` first on \`PATH\`, \`HTTPS_PROXY\` points at a dead local port, and in the A cases the stub fails every call, so
"0 API calls" means the refusal came before any call. \`fixtures/smoke_tree/\` is the results tree of the development
smoke pod (\`cap:8:0:5\`, t = 8, not data; its manifest equals \`../smoke/receipts/local-sha256.txt\`; its two \`.log\`
files are re-included by \`../.gitignore\`).

**Runs.** \`bash tests/run_tests.sh\` from \`runpod_capped/\` writes nothing in the repository and prints the results;
\`bash tests/run_tests.sh --write\` also replaces \`tests/out/\`, \`tests/results.tsv\` and this file.

**The image layer.** $nlayer of the $ntot cases (launch, fetch, four reaper cases and five refusals that come after the
binary check) need the image layer: \`RPC_LAYER_SRC\` (the layer tarball, SHA-256 \`a0f7e28e…\`, \`image.json\`
\`layer.digest\`) and \`RPC_BIN_SRC\` (its \`usr/local/bin\`; defaults in the session scratchpad). Without them those
cases print \`SKIP-NEEDS-LAYER\`: a run without the layer gives $((ntot - nlayer)) results and $nlayer skips; a run with
it gives $ntot results. The layer is the study layer of the image \`image/image.json\` names (ECR); fetching it
by digest needs read access to that registry, which this container does not have.

Groups: A refusals before any API call; B launch paths; C \`fetch.sh\` flows; D the outcome classifier alone
(\`fetch.sh --classify-dir\`) on synthetic receipt sets: one per class, the review's conflicting cases, the memory rule
(D25-D37c), the round-2 cases (D38-D42) and the round-3 cases (D43-D44b); E the registered order (\`next_allowed\`); F \`reap.sh\`; G syntax and the
client pin. Per-case output: \`out/<id>.txt\`. Machine-readable table: \`results.tsv\`. Every result here is
\`[pcc:unverified]\`.

| id | result | case | expected | observed |
|:--|:--|:--|:--|:--|
INTRO
    awk -F'\t' 'NR > 1 { gsub(/[|]/, "/", $0); printf "| %s | %s | %s | %s | %s |\n", $1, $2, $5, $3, $4 }' "$RES"
    echo; echo "Summary of this run (image layer available: $HAVE_LAYER): $npass passed, $nfail failed, $nskip SKIP-NEEDS-LAYER (run $(date -u +%FT%TZ))."; } > "$T/README.md"
fi
echo "passed $npass, failed $nfail, SKIP-NEEDS-LAYER $nskip (layer-dependent cases: $nlayer of $ntot)"
if [ -n "${KEEP:-}" ]; then echo "kept $WORK"; else rm -rf "${WORK:?}"; fi
[ "$nfail" = 0 ]
