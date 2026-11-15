#!/usr/bin/env bash
# run_tests.sh -- offline tests of the capped RunPod route (draft Amendment 18). No pod, no network: every curl call goes
# to tests/curl_stub.sh (first on PATH), HTTPS_PROXY points at a dead port, and the "deny" cases fail every call.
# Each case runs in a fresh sandbox copy of the route (its own git repo, receipts, ledger, RPC_STATE and stub state).
# Inputs: RPC_BIN_SRC (the image layer's usr/local/bin), RPC_LAYER_SRC (the layer tarball). Output: tests/out/<id>.txt
# per case and tests/results.tsv (id, pass/fail, expected, observed, description).
set -u
T=$(cd "$(dirname "$0")" && pwd); RC=$(cd "$T/.." && pwd); STUDY=$(cd "$RC/.." && pwd)
SCR=/tmp/claude-0/-home-user/354dbe26-6a93-506a-a404-2fc6ef137fdc/scratchpad
RPC_BIN_SRC=${RPC_BIN_SRC:-$SCR/r2g_layer/usr/local/bin}
RPC_LAYER_SRC=${RPC_LAYER_SRC:-$SCR/r2g_dl/layer.tgz}
WORK=$(mktemp -d "${TMPDIR:-/tmp}/rpc-tests.XXXX")
OUTD="$T/out"; rm -rf "$OUTD"; mkdir -p "$OUTD"; RES="$T/results.tsv"; printf 'id\tresult\texpected\tobserved\tcase\n' > "$RES"
npass=0 nfail=0

rec() { # rec ID OK(0|1) EXPECTED OBSERVED DESCRIPTION
  local r=PASS; [ "$2" = 0 ] || r=FAIL
  [ "$r" = PASS ] && npass=$((npass + 1)) || nfail=$((nfail + 1))
  printf '%s\t%s\t%s\t%s\t%s\n' "$1" "$r" "$3" "$4" "$5" >> "$RES"; echo "$r $1: $5 (expected $3, observed $4)"
}
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
  ST="$SB/stub"; mkdir -p "$ST" "$SB/bin" "$SB/state"; cp "$T/curl_stub.sh" "$SB/bin/curl"
  jq '{costPerHr:1.04, cpuFlavorId:"cpu5m", vcpuCount:16, memoryInGb:128, containerDiskInGb:20, imageName:"python:3.12-slim",
       machine:{dataCenterId:"EU-RO-1", secureCloud:true}, machineId:"stubmachine", interruptible:null}' -n > "$ST/create-template.json"
  echo '[]' > "$ST/pods.json"
  export STUB_DIR="$ST" PATH="$SB/bin:$ORIGPATH" RPC_STATE="$SB/state" RPC_BIN="$RPC_BIN_SRC" RPC_LAYER="$RPC_LAYER_SRC"
  export RUNPOD_API_KEY=dummy-test-key RPC_POLL_S=0 HTTPS_PROXY=http://127.0.0.1:9 https_proxy=http://127.0.0.1:9
  unset SHA_FILE STUB_POD_ID STUB_CREATED_AGO
}
fresh_ledger() { sed -i "s/^# complete_as_of\t.*/# complete_as_of\t$(date -u +%FT%TZ)/" "$R/ledger.tsv"; }
freeze() { printf '\n## Amendment 18 (test sandbox only)\n' >> "$SB/repo/research/prime_fall_degree_20261027/PREREGISTRATION.md"; }
study_ready() { fresh_ledger; freeze; }
run() { ( cd "$R" && "$@" ) > "$OUTD/$ID.txt" 2>&1; echo $?; }
calls() { if [ -f "$ST/calls.log" ]; then wc -l < "$ST/calls.log"; else echo 0; fi; }
creates() { if [ -f "$ST/calls.log" ]; then grep -c -- '-X POST https://rest.runpod.io/v1/pods ' "$ST/calls.log" || true; else echo 0; fi; }
outcome() { mkdir -p "$R/receipts/$1"; jq -n --arg c "$2" --argjson o "${3:-false}" '{class:$c, row:{oversize:$o, timed_out:false}}' > "$R/receipts/$1/outcome.json"; }
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
s_k0admitted_os() { study_ready; outcome pkm-x-t14-k0-c6 ADMITTED true; }
s_k0inflight() { study_ready; mkdir -p "$R/receipts/pkm-x-t14-k0-c6"; }
s_nofreeze() { fresh_ledger; }
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
deny_case A15 "pfd_grid_zeros with one byte appended" s_binmod pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A16 "RPC_LAYER missing for a study name" s_nolayer pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A17 "ledger complete_as_of 'never' (stale)" s_stale pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A18 "ledger with a malformed row" s_malformed pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A19 "RPC_STATE inside the git worktree" s_stateinside pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A20 "development stage with t = 12" s_none pkm-x-dev-a cap:12:0:5:12800000000
deny_case A21 "a name with two earlier pre-start failures" s_prestart2 pkm-x-t14-k0-c6 cap:14:0:6:12800000000
deny_case A22 "-m112 request file edited (seconds 90001)" s_m112edit pkm-x-t14-k0-c6-m112 cap:14:0:6:27900000000
deny_case A23 "order: t13 -r3 requested while t14-k1 has not run" s_t14done pkm-x-t13-k0-c6-r3 cap:13:0:6:12800000000
deny_case A24 "order: -r1 after the -r1 was lost (no third attempt)" s_r1lost pkm-x-t14-k0-c6-r1 cap:14:0:6:12800000000
deny_case A25 "order: t13 -r3 ABORTED has no relaunch (all resolved)" s_t13lost pkm-x-t13-k0-c6-r3 cap:13:0:6:12800000000
ID=A26; mksb; touch "$ST/deny"; mkdir -p "$R/receipts_dev/pkm-x-dev-smoke1"; rc=$(run ./run.sh pkm-x-dev-smoke1 cap:8:0:5:12800000000)
[ "$rc" = 3 ] && [ "$(calls)" = 0 ]; rec A26 $? "exit 3, 0 API calls" "exit $rc, $(calls) API calls" "dev name with an existing receipts_dev dir"

# ===================================================================== B. launch paths against the stub
ID=B01; mksb; study_ready; echo '[{"id":"other1","name":"pkm-x-dev-zzz","desiredStatus":"RUNNING","createdAt":"2026-11-14 00:00:00.000 +0000 UTC"}]' > "$ST/pods.json"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 3 ] && [ "$(creates)" = 0 ] && [ ! -d "$R/receipts/pkm-x-t14-k0-c6" ]
rec B01 $? "exit 3, no create" "exit $rc, creates $(creates)" "a pkm-x- pod is live (serial rule)"
ID=B02; mksb; study_ready; echo '{"data":{"cpuFlavors":[{"specifics":{"stockStatus":null,"securePrice":1.04}}]}}' > "$ST/spec.json"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 10 ] && [ "$(creates)" = 0 ] && [ ! -d "$R/receipts/pkm-x-t14-k0-c6" ]
rec B02 $? "exit 10, no create, no receipts dir" "exit $rc, creates $(creates), dir $([ -d "$R/receipts/pkm-x-t14-k0-c6" ] && echo yes || echo no)" "cpu5m out of stock: no attempt"
ID=B03; mksb; study_ready; echo '{"data":{"cpuFlavors":[{"specifics":{"stockStatus":"High","securePrice":1.6}}]}}' > "$ST/spec.json"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 3 ] && [ "$(creates)" = 0 ]
rec B03 $? "exit 3, no create" "exit $rc, creates $(creates)" "price 1.60/h x 26 h > US\$40"
ID=B04; mksb; study_ready; printf 'eks-all\teks\tA9-A15\tclosed\t380.00\ttest\n' >> "$R/ledger.tsv"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 3 ] && [ "$(creates)" = 0 ] && grep -q 'study ceiling' "$OUTD/B04.txt"
rec B04 $? "exit 3 (study ceiling)" "exit $rc, creates $(creates)" "ledger US\$380.01 + worst case US\$27.04 > US\$400"
ID=B05; mksb; study_ready; jq '.machine.secureCloud = false' "$ST/create-template.json" > "$SB/x" && mv "$SB/x" "$ST/create-template.json"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000)
[ "$rc" = 4 ] && grep -q 'DELETE https://rest.runpod.io/v1/pods/stubpod1' "$ST/calls.log" && [ -d "$R/receipts/pkm-x-t14-k0-c6.prestart-1" ] && [ ! -e "$R/receipts/pkm-x-t14-k0-c6" ] \
  && [ "$(jq -r .class "$R/receipts/pkm-x-t14-k0-c6.prestart-1/outcome.json")" = NOT_AN_ATTEMPT ] && [ "$(jq length "$ST/pods.json")" = 0 ] \
  && grep -qP '^pkm-x-t14-k0-c6/stubpod1\trunpod_capped\t.*\tclosed\t' "$R/ledger.tsv"
rec B05 $? "exit 4, pod deleted, receipts -> .prestart-1, ledger row closed" "exit $rc, pods left $(jq length "$ST/pods.json")" "create response not in the reserved tier (machine.secureCloud false)"
ID=B06; mksb; study_ready; jq '.costPerHr = 0.98' "$ST/create-template.json" > "$SB/x" && mv "$SB/x" "$ST/create-template.json"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 4 ] && [ "$(jq length "$ST/pods.json")" = 0 ] && [ -d "$R/receipts/pkm-x-t14-k0-c6.prestart-1" ]
rec B06 $? "exit 4, pod deleted" "exit $rc, pods left $(jq length "$ST/pods.json")" "costPerHr 0.98 differs from the price read before create (1.04)"
ID=B07; mksb; study_ready; touch "$ST/create-timeout"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 4 ] && [ "$(jq length "$ST/pods.json")" = 0 ] && [ -d "$R/receipts/pkm-x-t14-k0-c6.prestart-1" ] \
  && grep -q 'pre-start delete pod stubpod1' "$OUTD/B07.txt"
rec B07 $? "exit 4, pod found by name and deleted" "exit $rc, pods left $(jq length "$ST/pods.json")" "create call timed out but the pod exists (trap set before create)"
ID=B08; mksb; study_ready; touch "$ST/create-error"
rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000); [ "$rc" = 4 ] && compgen -G "$R/receipts/pkm-x-t14-k0-c6.nocreate-*" >/dev/null && [ ! -e "$R/receipts/pkm-x-t14-k0-c6" ]
rec B08 $? "exit 4, receipts -> .nocreate-*, name free" "exit $rc" "create refused by the API, no pod exists"
ID=B09; mksb; export STUB_CREATED_AGO=7200
rc=$(run ./run.sh pkm-x-dev-late cap:8:0:5:12800000000); [ "$rc" = 5 ] && [ -d "$R/receipts_dev/pkm-x-dev-late.prestart-1" ] && [ "$(jq length "$ST/pods.json")" = 0 ]
rec B09 $? "exit 5, pod deleted, not an attempt" "exit $rc" "runner would start more than 55 min after creation"
unset STUB_CREATED_AGO

ID=B10; mksb; rc=$(run ./run.sh pkm-x-dev-t1 cap:8:0:5:12800000000); D="$R/receipts_dev/pkm-x-dev-t1"; tok=$(cat "$SB/state/pkm-x-dev-t1.token")
ok=1; [ "$rc" = 0 ] && [ -f "$D/start.json" ] && [ -f "$D/pod-state.json" ] && jq -e '.run_deadline_provisional and .hard_deadline' "$D/pod-state.json" >/dev/null \
  && jq -e '.env.RUNNER_TOKEN == "<withheld>" and .cloudType == "SECURE" and .interruptible == false and .cpuFlavorIds == ["cpu5m"] and .dockerEntrypoint == ["bash","-c"]' "$D/pod-request.json" >/dev/null \
  && ! grep -rqF "$tok" "$D" && ! grep -F -- '-X POST https://rest.runpod.io/v1/pods' "$ST/calls.log" | grep -qF "$tok" && ! grep -qF dummy-test-key "$ST/calls.log" \
  && ! grep -rq consumerUserId "$D" && jq -e '.env.RUNNER_TOKEN' "$ST/create-body.json" >/dev/null \
  && grep -q 'cgsnap' "$D/boot-template.sh" && grep -qP '^pkm-x-dev-t1/stubpod1\t.*\topen\t27.0400' "$R/ledger.tsv" && ok=0
rec B10 $ok "exit 0; token and key absent from receipts and from the create call's argv; request record, deadlines, ledger row" \
  "exit $rc; X-Token argv lines (rp_pinned.sh, disclosed): $(grep -cF "$tok" "$ST/calls.log")" "development launch end to end against the stub"
cp "$ST/create-body.json" "$WORK/b10-create-body.json"
ID=B11; mksb; study_ready; rc=$(run ./run.sh pkm-x-t14-k0-c6 cap:14:0:6:12800000000)
rc2=$( ( cd "$R" && ./run.sh pkm-x-t14-k1-c6 cap:14:1:6:12800000000 ) > "$OUTD/B11b.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$rc2" = 3 ] && grep -q "BLOCKED:pkm-x-t14-k0-c6" "$OUTD/B11b.txt" && grep -q 'RPC_BIN files are the files of image layer' "$OUTD/B11.txt" && grep -q 'git HEAD [0-9a-f]\{40\}' "$OUTD/B11.txt"
rec B11 $? "first study run starts (exit 0); the next one is refused while it is in flight (exit 3)" "exit $rc then $rc2" "study launch pkm-x-t14-k0-c6, then pkm-x-t14-k1-c6"

# boot script syntax (the pod's start command)
ID=B12; jq -r '.dockerStartCmd[0]' "$WORK/b10-create-body.json" > "$WORK/boot.sh"; bash -n "$WORK/boot.sh" > "$OUTD/B12.txt" 2>&1; r=$?
grep -q 'exec python3 -u -c "import base64;exec(base64.b64decode(' "$WORK/boot.sh"; r2=$?
b64=$(sed -n 's/.*b64decode(.\([A-Za-z0-9+\/=]*\).)).*/\1/p' "$WORK/boot.sh"); h=$(printf '%s' "$b64" | base64 -d | sha256sum | cut -d' ' -f1)
[ $r = 0 ] && [ $r2 = 0 ] && [ "$h" = f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550 ]
rec B12 $? "bash -n ok; embedded runner sha256 f56fd08e..." "bash -n $r; embedded sha256 ${h:0:8}" "the pod start command parses and carries the pinned server.py"
# boot loop behaviour, run locally against a fake cgroup tree (v2 files) with paths rewritten
ID=B13; L="$WORK/bootloop"; mkdir -p "$L/cg" "$L/res"; printf 'max\n' > "$L/cg/memory.max"; printf 'low 0\nhigh 0\nmax 0\noom 1\noom_kill 1\n' > "$L/cg/memory.events"
sed -n '1,/^exec python3/p' "$WORK/boot.sh" | sed '$d' | sed "s#/sys/fs/cgroup#$L/cg#g; s#/results#$L/res#g; s#/tmp/cgsnap.tmp#$L/t.tmp#g" > "$L/loop.sh"
( bash "$L/loop.sh"; sleep 1; echo '{"state":"done"}' > "$L/res/runner_state.json"; sleep 7 ) > "$OUTD/B13.txt" 2>&1
[ -f "$L/res/cgroup/memory.max" ] && [ -f "$L/res/cgroup/memory.events" ] && [ -f "$L/res/cgroup/final-at" ] && grep -q 'oom_kill 1' "$L/res/cgroup/memory.events"
rec B13 $? "memory.max, memory.events and final-at copied" "$(ls "$L/res/cgroup" 2>/dev/null | tr '\n' ' ')" "the boot loop copies cgroup v2 files and writes its final copy after runner_state.json"

# ===================================================================== C. fetch.sh flows against the stub
mktree() { # mktree DIR STAGE STARTED ENDED PILOT_EXIT GZ_EXIT OOM_KILL|none [START_LINES]
  local d=$1 st=$2 s0=$3 en=$4 pe=$5 ge=$6 ok=$7 n=${8:-1} t k c
  IFS=: read -r _ t k c _ <<<"$st"; local cell="X-m2-t$t-k$k-c$c"
  rm -rf "$d"; cp -a "$T/fixtures/smoke_tree" "$d"
  sed -i "s/^argv: .*/argv: $st/" "$d/host.txt"
  [ "$cell" = X-m2-t8-k0-c5 ] || for f in "$d"/runs/X-m2-t8-k0-c5*; do mv "$f" "${f/X-m2-t8-k0-c5/$cell}"; done
  { for i in $(seq 1 "$n"); do echo "$s0 start $cell"; done; echo "$en end $cell exit $pe"; echo "$en end $cell.gridzeros exit $ge"; } > "$d/progress.txt"
  local rx=0; [ "$pe" = 0 ] && [ "$ge" = 0 ] || rx=1
  jq -nc --arg s "$s0" --arg e "$en" --arg st "$st" --argjson x $rx '{state:"done",exit:$x,started:$s,ended:$e,stages:[$st]}' > "$d/runner_state.json"
  jq --arg st "$st" '.stages = [$st]' "$d/runner_request.json" > "$d/rr" && mv "$d/rr" "$d/runner_request.json"
  if [ "$ok" != none ]; then mkdir -p "$d/cgroup"; echo max > "$d/cgroup/memory.max"; printf 'low 0\nhigh 0\nmax 0\noom %s\noom_kill %s\n' "$ok" "$ok" > "$d/cgroup/memory.events"; echo "$en" > "$d/cgroup/final-at"; fi
}
status_done() { jq -nc --arg s "$1" --arg e "$2" --argjson x "$3" '{state:"done",exit:$x,started:$s,ended:$e,stages:["cap:8:0:5:12800000000"],progress:"",meminfo:[],nproc:48}' > "$ST/status.json"; }
fetch() { ( cd "$R" && ./fetch.sh "$@" ) >> "$OUTD/$ID.txt" 2>&1; echo $?; }
cls() { jq -r '.class + " " + .rule' "$1/outcome.json" 2>/dev/null || echo none; }
# start a dev run, then move its runner start back by $2 seconds in the receipts and the stub (deadline tests)
startrun() { ID=$1; mksb; run ./run.sh pkm-x-dev-c cap:8:0:5:12800000000 >/dev/null; D="$R/receipts_dev/pkm-x-dev-c"
  S0=$(jq -r .started "$D/start.json")
  if [ "${2:-0}" != 0 ]; then S0=$(iso $(( $(date +%s) - $2 ))); jq --arg s "$S0" '.started = $s' "$D/start.json" > "$SB/x" && mv "$SB/x" "$D/start.json"
    jq --arg s "$S0" '.started = $s' "$ST/status.json" > "$SB/x" && mv "$SB/x" "$ST/status.json"; fi; }

startrun C01; rc=$(fetch pkm-x-dev-c); [ "$rc" = 41 ] && [ "$(jq length "$ST/pods.json")" = 1 ] && [ "$(wc -l < "$D/checkins.jsonl")" = 1 ]
rec C01 $? "exit 41, pod kept, one check-in" "exit $rc" "running before the deadline"
startrun C02 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
rc=$(fetch pkm-x-dev-c); c1=$(cls "$D")
jq -n '{raw:[]}' > "$SB/rec-empty.json"; rc2=$(fetch pkm-x-dev-c --delete "$SB/rec-empty.json")
awk '{print $1}' "$D/local-sha256.txt" | jq -Rn '{raw:[inputs | {sha256:., uploaded:true}]}' > "$SB/rec-full.json"; rc3=$(fetch pkm-x-dev-c --delete "$SB/rec-full.json")
rc4=$(fetch pkm-x-dev-c)
[ "$rc" = 8 ] && [ "$c1" = "ADMITTED P2d" ] && [ "$rc2" = 8 ] && [ "$rc3" = 0 ] && [ "$rc4" = 9 ] && [ -f "$D/cost.json" ] && [ "$(jq length "$ST/pods.json")" = 0 ] \
  && grep -qP '^pkm-x-dev-c/stubpod1\t.*\tclosed\t' "$R/ledger.tsv" && [ -d "$R/results/pkm-x-dev-c" ]
rec C02 $? "exit 8 ADMITTED; --delete with an empty record exit 8; with a full record exit 0; re-run exit 9" "exit $rc $c1; $rc2; $rc3; $rc4" "completed run: collect, wait for the raw upload, delete, refuse re-collection"
startrun C03; rc=$(fetch pkm-x-dev-c --abort); [ "$rc" = 0 ] && [ "$(cls "$D")" = "ABORTED P5" ] && [ "$(jq length "$ST/pods.json")" = 0 ]
rec C03 $? "exit 0, ABORTED P5, pod deleted" "exit $rc $(cls "$D")" "session abort while running"
startrun C04; echo '{"state":"idle","exit":null,"started":null,"ended":null,"stages":null}' > "$ST/status.json"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LOST P3" ] && [ "$(jq length "$ST/pods.json")" = 0 ]
rec C04 $? "exit 0, LOST P3, pod deleted" "exit $rc $(cls "$D")" "runner idle after its start (container restarted)"
startrun C05 9000; touch "$ST/unreachable"; jq -nc --arg a "$(iso $(( $(date +%s) - 7200 )))" '{at:$a,phase:"regular",live:null,rest:{http:500},listed:true}' >> "$D/checkins.jsonl"
rm -f "$ST/pod-stubpod1.json.rec"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LOST P3" ] && grep -q unreachable "$D/outcome.json"
rec C05 $? "exit 0, LOST P3 (unreachable rule)" "exit $rc $(cls "$D")" "unreachable at two check-ins 2 h apart, REST record not readable (404) but still listed"
startrun C06 9000; touch "$ST/unreachable"; L0=$(jq -r .lastStartedAt "$D/pod-record-start.json")
jq -nc --arg a "$(iso $(( $(date +%s) - 7200 )))" --arg l "$L0" '{at:$a,phase:"regular",live:null,rest:{http:200,desiredStatus:"RUNNING",lastStartedAt:$l},listed:true}' >> "$D/checkins.jsonl"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 41 ] && [ "$(jq length "$ST/pods.json")" = 1 ]
rec C06 $? "exit 41, pod kept" "exit $rc" "unreachable twice 2 h apart, but the REST record is a healthy RUNNING pod (proxy outage only)"
startrun C07 90100; rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LIMIT P4" ] && [ "$(jq length "$ST/pods.json")" = 0 ]
rec C07 $? "exit 0, LIMIT P4, pod deleted" "exit $rc $(cls "$D")" "still running at the run deadline"
startrun C08 90100; touch "$ST/unreachable"; rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LOST P6" ] && [ "$(jq length "$ST/pods.json")" = 0 ]
rec C08 $? "exit 0, LOST P6, pod deleted although nothing could be fetched" "exit $rc $(cls "$D")" "unreachable at the deadline (one reading)"
startrun C09 90100; E=$(iso $(( $(epoch "$S0") + 90050 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0
rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LIMIT P2a" ]
rec C09 $? "exit 0, LIMIT P2a (deleted: past the deadline)" "exit $rc $(cls "$D")" "runner done 90,050 s after its start"
startrun C10 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 137 1 1; status_done "$S0" "$E" 1
rc=$(fetch pkm-x-dev-c); [ "$rc" = 8 ] && [ "$(cls "$D")" = "OOM P2b" ]
rec C10 $? "exit 8, OOM P2b" "exit $rc $(cls "$D")" "pilot exit 137 with cgroup oom_kill 1"
startrun C11 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 137 1 0; status_done "$S0" "$E" 1
rc=$(fetch pkm-x-dev-c); [ "$rc" = 8 ] && [ "$(cls "$D")" = "LOST P2c" ]
rec C11 $? "exit 8, LOST P2c" "exit $rc $(cls "$D")" "pilot exit 137 with oom_kill 0"
startrun C12 1000; E=$(iso $(( $(epoch "$S0") + 60 ))); mktree "$ST/tree" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; status_done "$S0" "$E" 0; touch "$ST/tree-varies"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 6 ] && [ ! -f "$D/collect.json" ] && [ "$(jq length "$ST/pods.json")" = 1 ] && [ ! -e "$R/results/pkm-x-dev-c" ]
rec C12 $? "exit 6, pod kept, nothing kept" "exit $rc" "the two fetches differ while the runner is done, before the deadline"
startrun C13; jq --arg i stubpod1 'map(select(.id != $i))' "$ST/pods.json" > "$SB/x" && mv "$SB/x" "$ST/pods.json"; rm -f "$ST/pod-stubpod1.json.rec"; touch "$ST/unreachable"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LOST P3" ] && jq -e '.deleted_time_is_upper_bound == true' "$D/cost.json" >/dev/null
rec C13 $? "exit 0, LOST P3, cost marked upper bound" "exit $rc $(cls "$D")" "pod gone from the API (not listed, 404)"
startrun C14; jq '.lastStartedAt = "2026-11-15 01:00:00.000 +0000 UTC"' "$ST/pod-stubpod1.json.rec" > "$SB/x" && mv "$SB/x" "$ST/pod-stubpod1.json.rec"
rc=$(fetch pkm-x-dev-c); [ "$rc" = 0 ] && [ "$(cls "$D")" = "LOST P3" ]
rec C14 $? "exit 0, LOST P3" "exit $rc $(cls "$D")" "pod lastStartedAt changed while the runner reports running"
startrun C15; touch "$ST/delete-fail"; rc=$(fetch pkm-x-dev-c --abort); rm -f "$ST/delete-fail"; rc2=$(fetch pkm-x-dev-c)
[ "$rc" = 7 ] && [ -f "$D/delete-pending" -o -f "$D/cost.json" ] && [ "$rc2" = 0 ] && [ -f "$D/cost.json" ] && [ "$(cls "$D")" = "ABORTED P5" ]
rec C15 $? "exit 7 (still listed), then exit 0 on the next call" "exit $rc then $rc2" "a failed delete is resumed by the next fetch.sh call"
ID=C16; mksb; run ./run.sh pkm-x-dev-c cap:8:0:5:12800000000 >/dev/null; D="$R/receipts_dev/pkm-x-dev-c"; rm -f "$D/start.json"
echo '{"state":"idle","exit":null,"started":null,"ended":null,"stages":null}' > "$ST/status.json"; rc=$(fetch pkm-x-dev-c --abort)
[ "$rc" = 0 ] && [ -d "$R/receipts_dev/pkm-x-dev-c.prestart-1" ] && [ "$(jq -r .class "$R/receipts_dev/pkm-x-dev-c.prestart-1/outcome.json")" = NOT_AN_ATTEMPT ]
rec C16 $? "exit 0, NOT_AN_ATTEMPT, receipts -> .prestart-1" "exit $rc" "launcher died before start (no start.json, runner idle)"

# ===================================================================== D. the classifier alone, on synthetic receipt sets
S0E=$(( $(date +%s) - 200000 )); S0=$(iso $S0E); DLE=$(( S0E + 90000 )); L0="2026-11-10 10:00:00.000 +0000 UTC"
mkset() { # mkset DIR: a receipts dir with the launch receipts of a run started at S0
  local d=$1; rm -rf "$d"; mkdir -p "$d"
  jq -n --arg l "$L0" '{id:"p1", cpuFlavorId:"cpu5m", vcpuCount:16, memoryInGb:128, costPerHr:1.04, lastStartedAt:$l, createdAt:"2026-11-10 10:00:00.000 +0000 UTC", machine:{secureCloud:true, dataCenterId:"EU-RO-1"}}' > "$d/pod-create.json"
  jq -n --arg l "$L0" '{lastStartedAt:$l, desiredStatus:"RUNNING"}' > "$d/pod-record-start.json"
  jq -n '{cloudType:"SECURE", interruptible:false, cpuFlavorIds:["cpu5m"]}' > "$d/pod-request.json"
  echo '{"instance":"cpu5m-16-128","stockStatus":"High","securePrice":1.04}' > "$d/price.jsonl"
  jq -n '{stage:"cap:8:0:5:12800000000", mem_floor_gb:61}' > "$d/pod-state.json"
  jq -nc --arg s "$S0" '{state:"running",exit:null,started:$s,ended:null,stages:["cap:8:0:5:12800000000"]}' > "$d/start.json"
  : > "$d/checkins.jsonl"
}
ck() { # ck DIR AT_EPOCH PHASE LIVE(state|null) STARTED ENDED EXIT HTTP DESIRED LSA LISTED
  local live=null; [ "$4" != null ] && live=$(jq -nc --arg st "$4" --arg s "$5" --arg e "$6" --argjson x "$7" '{state:$st, exit:$x, started:(if $s == "" then null else $s end), ended:(if $e == "" then null else $e end)}')
  jq -nc --arg a "$(iso "$2")" --arg ph "$3" --argjson live "$live" --argjson h "$8" --arg d "$9" --arg l "${10}" --argjson li "${11}" \
    '{at:$a, phase:$ph, live:$live, rest:(if $h == 200 then {http:$h, desiredStatus:$d, lastStartedAt:$l} else {http:$h} end), listed:$li}' >> "$1/checkins.jsonl"
}
collect() { # collect DIR OUT REASON TRANSFER_OK AT_EPOCH [prefetch line from the last check-in]
  local pre; pre=$(tail -1 "$1/checkins.jsonl"); [ -n "$pre" ] || pre=null
  if [ -d "$2" ]; then (cd "$2" && find . -type f -printf '%P\n' | LC_ALL=C sort | xargs -r -d '\n' sha256sum) > "$1/local-sha256.txt"; fi
  jq -n --arg a "$(iso "$5")" --arg r "$3" --argjson t "$4" --argjson p "$pre" '{at:$a, reason:$r, mode:"", prefetch:$p, fetched:true, transfer_ok:$t}' > "$1/collect.json"
}
dcase() { # dcase ID EXPECTED_CLASS_RULE DESCRIPTION
  local got; got=$( (cd "$RC" && ./fetch.sh --classify-dir "$WORK/d/$1/rec" "$WORK/d/$1/out") 2> "$OUTD/$1.err" | tee "$OUTD/$1.txt" | jq -r '.class + " " + .rule')
  [ "$got" = "$2" ]; rec "$1" $? "$2" "$got" "$3"
}
nd() { DR="$WORK/d/$1/rec" DO="$WORK/d/$1/out"; mkdir -p "$WORK/d/$1"; mkset "$DR"; rm -rf "$DO"; }
okE=$(iso $(( S0E + 3600 ))); okEe=$(( S0E + 3600 ))
nd D01; rm "$DR/start.json"; dcase D01 "NOT_AN_ATTEMPT P1" "no runner start on record"
nd D02; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D02 "ADMITTED P2d" "completed, every admission rule holds"
nd D03; E=$(iso $(( S0E + 90050 ))); mktree "$DO" cap:8:0:5:12800000000 "$S0" "$E" 0 0 0; ck "$DR" $((S0E + 90100)) prefetch done "$S0" "$E" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((S0E + 90200)); dcase D03 "LIMIT P2a" "completed 90,050 s after the runner start"
nd D04; ck "$DR" $((DLE + 60)) regular running "$S0" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" deadline false $((DLE + 70)); dcase D04 "LIMIT P4" "running at the deadline check-in"
nd D05; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D05 "LOST P3" "runner idle before the deadline"
nd D06; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 200 RUNNING "2026-11-11 00:00:00.000 +0000 UTC" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D06 "LOST P3" "lastStartedAt changed before the deadline"
nd D07; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 RUNNING "$L0" true; ck "$DR" $((S0E + 7700)) regular null "" "" null 500 "" "" true; collect "$DR" "$DO" lost false $((S0E + 7710)); dcase D07 "LOST P3" "unreachable twice >= 1 h apart, REST record unreadable (HTTP 500)"
nd D07b; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 200 EXITED "$L0" true; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D07b "LOST P3" "REST desiredStatus EXITED"
nd D08; ck "$DR" $((S0E + 4000)) regular null "" "" null 200 RUNNING "$L0" true; ck "$DR" $((S0E + 7700)) regular null "" "" null 200 RUNNING "$L0" true; dcase D08 "PENDING P7" "unreachable twice >= 1 h apart, REST healthy: not lost"
nd D08b; ck "$DR" $((S0E + 4000)) regular null "" "" null 0 "" "" null; ck "$DR" $((S0E + 4500)) regular null "" "" null 0 "" "" null; dcase D08b "PENDING P7" "unreachable twice only 500 s apart, REST unreadable: not lost yet"
nd D09; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" abort false $((S0E + 4010)); dcase D09 "ABORTED P5" "session abort while running"
nd D10; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 137 1 1; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 1 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D10 "OOM P2b" "pilot exit 137, oom_kill 1"
nd D11; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 137 1 0; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 1 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D11 "LOST P2c" "pilot exit 137, oom_kill 0"
nd D12; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 137 1 none; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 1 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D12 "LOST P2c" "pilot exit 137, no cgroup files"
nd D13; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 1 0 0; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 1 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D13 "NOT_ADMITTED P2e" "pilot exit 1 (a tool failure, not a loss)"
nd D14; ck "$DR" $((DLE + 60)) regular idle "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" deadline false $((DLE + 70)); dcase D14 "LOST P6" "M1 deadline vs lost: restart first seen at the deadline check-in"
nd D14b; ck "$DR" $((DLE + 60)) regular running "$S0" "" null 200 RUNNING "$L0" true; ck "$DR" $((DLE + 600)) regular idle "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" deadline false $((DLE + 610)); dcase D14b "LIMIT P4" "M1: running at the deadline, restarted afterwards"
nd D15; ck "$DR" $((DLE + 60)) regular null "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" deadline false $((DLE + 70)); dcase D15 "LOST P6" "M1: one unreachable reading at the deadline"
nd D16; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); ck "$DR" $((okEe + 900)) regular idle "" "" null 200 RUNNING "2026-11-12 00:00:00.000 +0000 UTC" true; dcase D16 "ADMITTED P2d" "M1: completed, receipts fetched, then restarted"
nd D16b; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((okEe + 50)) regular done "$S0" "$okE" 0 200 RUNNING "2026-11-12 00:00:00.000 +0000 UTC" true; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "2026-11-12 00:00:00.000 +0000 UTC" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D16b "LOST P3" "M1: restart recorded before the completion was fetched"
nd D17; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); jq '.mode = "--abort"' "$DR/collect.json" > "$DR/c" && mv "$DR/c" "$DR/collect.json"; dcase D17 "ADMITTED P2d" "M1: --abort on a run that is already done cannot change its class"
nd D18; ck "$DR" $((S0E + 4000)) regular idle "" "" null 200 RUNNING "$L0" true; collect "$DR" "$DO" abort false $((S0E + 4010)); dcase D18 "LOST P3" "M1: --abort after a restart is LOST, not ABORTED"
nd D19; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0 2; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D19 "LOST P3" "M1: two start lines in progress.txt (a second runner start)"
nd D20; mktree "$DO" cap:8:0:5:12800000000 "$(iso $((S0E + 30)))" "$okE" 0 0 0; ck "$DR" $((okEe + 100)) prefetch done "$(iso $((S0E + 30)))" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); dcase D20 "LOST P3" "M1: the runner's start differs from start.json (manual re-start)"
nd D21; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((DLE + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done false $((DLE + 200)); dcase D21 "LOST P6" "completed in time but the transfer check failed past the deadline"
nd D22; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); echo tampered >> "$DO/host.txt"; dcase D22 "NOT_ADMITTED P2e" "a file of the kept tree changed after collection (rule 6)"
nd D23; mktree "$DO" cap:8:0:5:12800000000 "$S0" "$okE" 0 0 0; ck "$DR" $((okEe + 100)) prefetch done "$S0" "$okE" 0 200 RUNNING "$L0" true; collect "$DR" "$DO" done true $((okEe + 200)); jq '.machine.secureCloud = false' "$DR/pod-create.json" > "$DR/c" && mv "$DR/c" "$DR/pod-create.json"; dcase D23 "NOT_ADMITTED P2e" "create response not in the reserved tier (rule 7)"
nd D24; ck "$DR" $((S0E + 4000)) regular running "$S0" "" null 404 "" "" false; collect "$DR" "$DO" lost false $((S0E + 4010)); dcase D24 "LOST P3" "pod no longer listed (deleted outside the route)"

# ===================================================================== E. the registered order (next_allowed)
ecase() { # ecase ID EXPECTED DESCRIPTION outcome-specs... (name:class[:oversize])
  local id=$1 want=$2 desc=$3; shift 3; mksb; local s n c o
  for s in "$@"; do IFS=: read -r n c o <<<"$s"; if [ "$c" = INFLIGHT ]; then mkdir -p "$R/receipts/$n"; else outcome "$n" "$c" "${o:-false}"; fi; done
  local got; got=$(cd "$R" && bash -c '. ./common.sh; next_allowed' 2>&1); [ "$got" = "$want" ]; rec "$id" $? "$want" "$got" "$desc"
}
ecase E01 pkm-x-t14-k0-c6 "nothing run yet"
ecase E02 pkm-x-t14-k0-c6-r1 "base LOST" pkm-x-t14-k0-c6:LOST
ecase E03 pkm-x-t14-k0-c6-r1 "base ABORTED (charged like LOST)" pkm-x-t14-k0-c6:ABORTED
ecase E04 pkm-x-t14-k1-c6 "base ADMITTED, no oversize" pkm-x-t14-k0-c6:ADMITTED
ecase E05 pkm-x-t14-k0-c6-m112 "base ADMITTED with oversize true" pkm-x-t14-k0-c6:ADMITTED:true
ecase E06 pkm-x-t14-k0-c6-m112 "base OOM" pkm-x-t14-k0-c6:OOM
ecase E07 pkm-x-t14-k0-c6-m112-r1 "base OOM, -m112 LOST, relaunch unused" pkm-x-t14-k0-c6:OOM pkm-x-t14-k0-c6-m112:LOST
ecase E08 pkm-x-t14-k1-c6 "base LOST, -r1 OOM, -m112 LOST: relaunch already used" pkm-x-t14-k0-c6:LOST pkm-x-t14-k0-c6-r1:OOM pkm-x-t14-k0-c6-m112:LOST
ecase E09 pkm-x-t14-k1-c6 "base LIMIT (censored, not re-run)" pkm-x-t14-k0-c6:LIMIT
ecase E10 pkm-x-t14-k1-c6 "base NOT_ADMITTED (owner)" pkm-x-t14-k0-c6:NOT_ADMITTED
ecase E11 pkm-x-t13-k0-c6-r3 "both t14 cells resolved" pkm-x-t14-k0-c6:ADMITTED pkm-x-t14-k1-c6:LIMIT
ecase E12 ALL_RESOLVED "t13 -r3 LOST: no relaunch" pkm-x-t14-k0-c6:ADMITTED pkm-x-t14-k1-c6:ADMITTED pkm-x-t13-k0-c6-r3:LOST
ecase E13 pkm-x-t13-k0-c6-m112 "t13 -r3 OOM" pkm-x-t14-k0-c6:ADMITTED pkm-x-t14-k1-c6:ADMITTED pkm-x-t13-k0-c6-r3:OOM
ecase E14 BLOCKED:pkm-x-t14-k0-c6 "base in flight" pkm-x-t14-k0-c6:INFLIGHT
ecase E15 ALL_RESOLVED "t13 -m112 LOST: no relaunch for t13" pkm-x-t14-k0-c6:ADMITTED pkm-x-t14-k1-c6:ADMITTED pkm-x-t13-k0-c6-r3:OOM pkm-x-t13-k0-c6-m112:LOST

# ===================================================================== F. reap.sh against the stub
ID=F01; mksb; old=$(rpd $(( $(date +%s) - 27 * 3600 ))); young=$(rpd $(( $(date +%s) - 3600 )))
jq -n --arg o "$old" --arg y "$young" '[{id:"old1",name:"pkm-x-t14-k0-c6",desiredStatus:"RUNNING",createdAt:$o,lastStartedAt:$o},
  {id:"young1",name:"pkm-x-dev-y",desiredStatus:"RUNNING",createdAt:$y,lastStartedAt:$y},
  {id:"other1",name:"pfd-b-rp",desiredStatus:"RUNNING",createdAt:$o,lastStartedAt:$o}]' > "$ST/pods.json"
rc=$(run ./reap.sh); left=$(jq -c '[.[].id]' "$ST/pods.json"); rc2=$( (cd "$R" && ./reap.sh) >> "$OUTD/F01.txt" 2>&1; echo $?)
left2=$(jq -c '[.[].id]' "$ST/pods.json"); lines=$(cat "$R"/reaper/reap-*.jsonl | wc -l)
[ "$rc" = 0 ] && [ "$left" = '["young1","other1"]' ] && [ "$rc2" = 0 ] && [ "$left2" = '["young1","other1"]' ] && grep -q '"due":true' "$R"/reaper/reap-*.jsonl
rec F01 $? "old pkm-x- pod deleted, young pkm-x- pod and non-pkm-x pod kept; second pass changes nothing" "exit $rc, left $left; exit $rc2, left $left2; $lines receipt lines" "reaper: 27 h old pod without receipts, 1 h old pod, other session's pod"
ID=F02; startrun F02 90100; rc=$( (cd "$R" && ./reap.sh) > "$OUTD/F02.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$(jq length "$ST/pods.json")" = 0 ] && [ "$(cls "$D")" = "LIMIT P4" ] && [ -f "$D/cost.json" ]
rec F02 $? "pod collected by fetch.sh (LIMIT P4) and deleted" "exit $rc $(cls "$D")" "reaper: a started run past its run deadline, with receipts and token"
ID=F04; startrun F04; rm -f "$D/start.json"; echo '{"state":"idle","exit":null,"started":null,"ended":null,"stages":null}' > "$ST/status.json"
ago=$(rpd $(( $(date +%s) - 5400 ))); jq --arg a "$ago" 'map(.createdAt = $a)' "$ST/pods.json" > "$SB/x" && mv "$SB/x" "$ST/pods.json"
jq --arg a "$ago" '.createdAt = $a' "$D/pod-create.json" > "$SB/x" && mv "$SB/x" "$D/pod-create.json"
rc=$( (cd "$R" && ./reap.sh) > "$OUTD/F04.txt" 2>&1; echo $?)
[ "$rc" = 0 ] && [ "$(jq length "$ST/pods.json")" = 0 ] && [ -d "$R/receipts_dev/pkm-x-dev-c.prestart-1" ] && grep -q 'no runner start within 70 min' "$OUTD/F04.txt"
rec F04 $? "pod deleted through fetch.sh, NOT_AN_ATTEMPT, receipts -> .prestart-1" "exit $rc" "reaper: receipts but no runner start 90 min after creation (launcher died)"
ID=F03; mksb; touch "$ST/deny"; rc=$(run ./reap.sh); [ "$rc" = 1 ]; rec F03 $? "exit 1" "exit $rc" "reaper: pod list unreadable (network blocked)"

# ===================================================================== G. syntax
for f in common.sh run.sh fetch.sh reap.sh rp_pinned.sh make_requests_m112.sh tests/run_tests.sh tests/curl_stub.sh; do
  bash -n "$RC/$f" 2> "$OUTD/G-${f##*/}.txt"; r=$?; rec "G-${f##*/}" $r "bash -n ok" "rc $r" "syntax of $f"
done
sha256sum "$RC/rp_pinned.sh" | grep -q '^b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d'; rec G-pin $? "rp_pinned.sh sha256 b4b60199..." "$(sha256sum "$RC/rp_pinned.sh" | cut -c1-8)" "pinned client copy unchanged"

# README.md: the fixed introduction, then the table of this run
{ cat <<'INTRO'
# Offline tests of the capped RunPod route (draft Amendment 18)

`run_tests.sh` runs every case in a fresh sandbox copy of the route (its own git repository, receipts, ledger,
`RPC_STATE` and stub state) under `$TMPDIR`. No pod is created and no network call is made: `curl_stub.sh` is installed
as `curl` first on `PATH`, `HTTPS_PROXY` points at a dead local port, and in the A cases the stub fails every call, so
"0 API calls" means the refusal came before any call. `fixtures/smoke_tree/` is the results tree of the development
smoke pod (`cap:8:0:5`, t = 8, not data; its manifest equals `../smoke/receipts/local-sha256.txt`). The binaries come
from the image layer (`RPC_BIN_SRC`, `RPC_LAYER_SRC`, defaults in the session scratchpad).

Groups: A refusals before any API call; B launch paths against the stub; C `fetch.sh` flows against the stub; D the
outcome classifier alone (`fetch.sh --classify-dir`) on synthetic receipt sets, one per class plus the conflicting cases
of the review's M1; E the registered order (`next_allowed`); F `reap.sh`; G syntax and the client pin.
Per-case output: `out/<id>.txt`. Machine-readable table: `results.tsv`. Every result here is `[pcc:unverified]`.

Run: `bash tests/run_tests.sh` from `runpod_capped/` (about one minute).

| id | result | case | expected | observed |
|:--|:--|:--|:--|:--|
INTRO
  awk -F'\t' 'NR > 1 { gsub(/[|]/, "/", $0); printf "| %s | %s | %s | %s | %s |\n", $1, $2, $5, $3, $4 }' "$RES"
  echo; echo "Summary: $npass passed, $nfail failed (run $(date -u +%FT%TZ))."; } > "$T/README.md"
echo "passed $npass, failed $nfail"
rm -rf "$WORK"
[ "$nfail" = 0 ]
