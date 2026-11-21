# Shared settings and helpers for run.sh, fetch.sh and reap.sh (Amendment 33 draft, AMENDMENT_R2_H_runpod_capped.md).
# Sourced, not run. Shell only (bash, jq, curl, awk, coreutils, git).
# The pod-side runner is Amendment 16's server.py as reviewed at d2a16c723d, vendored here as server_pinned.py and pinned
# by SHA-256 below. Its client is a pinned, unchanged copy of main's runpod/rp.sh at d2a16c723d (rp_pinned.sh), used only
# for upload, start, status and fetch (those calls talk only to the pod's proxy URL, with the pod's token; none of them
# uses the RunPod REST API). The RunPod REST v2 calls (price, create, pod record, delete, list) are made here, with the
# key in a mode-0600 header file.
set -euo pipefail
here=${here:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}
study=$(cd "$here/.." && pwd)
PREREG="$study/../prime_fall_degree_20261027/PREREGISTRATION.md"
AMENDMENT_HEADING='## Amendment 33'
RP="$here/rp_pinned.sh"
RP_SHA=b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d       # = main's runpod/rp.sh at d2a16c723d
SERVER="$here/server_pinned.py"
SERVER_SHA=f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550   # = runpod/server.py at d2a16c723d
API=https://api.runpod.io/v2             # RunPod REST v2 (the v1 host answered 502; Amendment 32)
IMAGE='python:3.12-slim@sha256:5e2bdb1b0d542d5039cffb8181c65930093949194a65c365dfb1b0fe13a4fa5b'  # as main's runpod/rp.sh (v2)
FLAVOR=cpu5m VCPU=16 RAM_GB=128           # the one pinned flavor (decision D8): 16 vCPU, 128 GB; no fallback
INSTANCE="$FLAVOR-$VCPU-$RAM_GB"
MEM_FLOOR_GB=121                          # 112 GiB = 120.26 GB (Amendment 14 -m112): the pod record's cpu.memory floor
WORST_HOURS=30 CEIL_USD=40                # owner decision 4: US$40 per job. One worst-case pod life for every cost
                                          # figure (S7): hard deadline (26 h) + one reaper interval (4 h) = 30 h
STUDY_GUARD_USD=510 STUDY_CEILING_USD=540 # owner decision 21: ledger sum + this job's worst case must stay <= the guard
DEADLINE_S=90000                          # the registered job time limit (requests: seconds 90000), from the runner's start
START_WITHIN_S=3300                       # the runner must start within 55 min of creation, so start + 25 h <= creation + 26 h
HARD_S=93600                              # 26 h after creation: the reaper deletes any pkm-x- pod older than this
PUSH_WAIT_S=7200                          # a completed run's pod waits at most 2 h for the git gate (decision 22)
CLASSIFIER_VERSION=a33-r1                 # recorded in every outcome.json with the SHA-256 of fetch.sh and common.sh (S3)
# The frozen image, pinned here rather than read from the mutable image/*.json files (S3); run.sh checks they agree.
LAYER_DIGEST=a0f7e28eb0759ddefc4b906974866d2a2215d4705e25e9fd1fca45513b8b1971
BIN_SHA_PINS='pkm_tower_pilot f78f39599d75859a693d3177272e95cae79559eebb10c1ac528e0cb13e4fcee7
prime_fall_degree_tools 7e8a67030e8db88ab06be468ee3b01f7163eb6b5aa80275e690cc71102df9e7a
hky_fp_tower_check c36d324e8a60b0d8e827982baf7e07e0ce155664c67ce8c11b4debc410b95c6e
pfd_grid_zeros 962833611e13e78a3dc5fbb9ab6bfc498fde9cccab255c704ed6e2ff12a03354
entry_r2.sh 25b01841573334d1942de72e84aa55f60d4f238682fba33a41aa2bcd8962ea49
run.sh c35ea88e7007198b9e9a026812a9bcb0ce55bc4082ba3627564c67f0fbab6eeb'
M112_LIMIT_BYTES=120259084288             # 112 GiB: the pod's cgroup memory.max must be at least this, or "max" (S4)
LEDGER_FRESH_S=86400                      # the ledger's complete_as_of line must be at most 24 h old for a study launch
POLL_S=${RPC_POLL_S:-10}                  # seconds between API polls (only the tests shorten it)
RAYON=12                                  # Amendment 9; rp_pinned.sh start sends rayon 12
REC_ROOT="$here/receipts" DEV_ROOT="$here/receipts_dev" OUT_ROOT="$here/results" REAP_DIR="$here/reaper"
LEDGER="$here/ledger.tsv"
# The one cell (owner decisions 20 and 23): t = 14, draw 0, cap 7, at Amendment 14's -m112 setting. Its one relaunch
# (owner decision 13) runs only after a LOST with positive host-side evidence.
STUDY_STAGE=cap:14:0:7:27900000000
STUDY_NAMES=(pkm-x-t14-k0-c7-m112 pkm-x-t14-k0-c7-m112-r1)
STUDY_SRC_REL=requests_r2i/pfd-x-t14-k0-c6.json                      # the frozen request it is derived from
STUDY_REQUEST="$here/requests_m112/pfd-x-t14-k0-c7-m112.json"         # written by make_requests_m112.sh
STUDY_REQUEST_SHA=44e009f18d19a9c61e5a281e0c2a6a219c4f3cc3252ca63bff71049fdef4c7f9   # pinned; see make_requests_m112_output.txt

now() { date -u +%FT%TZ; }
bin_sha() { awk -v f="$1" '$1 == f { print $2 }' <<<"$BIN_SHA_PINS"; }
bin_sha_json() { awk '{ printf "%s\"%s\":\"%s\"", (NR > 1 ? "," : "{"), $1, $2 } END { print "}" }' <<<"$BIN_SHA_PINS"; }
# frozen_ok FILE...: each route file's SHA-256 appears in the preregistration (the frozen Amendment 33 lists them; S3).
frozen_ok() { local f; for f in "$@"; do grep -qF "$(sha256sum "$here/$f" | cut -d' ' -f1)" "$PREREG" || return 1; done; }
# session_action REC ACTION POD REASON: every delete the session makes is logged with its reason before the API call
# (RunPod keeps no independent record of who ended a pod; N3).
session_action() { [ -d "$1" ] && jq -nc --arg at "$(now)" --arg a "$2" --arg p "$3" --arg r "$4" '{at:$at, action:$a, pod:$p, reason:$r}' >> "$1/session-actions.jsonl"; return 0; }
# remind PATH...: the session commits and pushes these after every launch and every fetch.sh call (N6).
remind() { echo "COMMIT AND PUSH NOW: $* runpod_capped/ledger.tsv (git add, commit, push; the session does this)" >&2; }
epoch() { date -u -d "$1" +%s; }
is_study() { local n; for n in "${STUDY_NAMES[@]}"; do [ "$1" = "$n" ] && return 0; done; return 1; }
is_dev() { [[ $1 =~ ^pkm-x-dev-[a-z0-9]+(-[a-z0-9]+)*$ ]]; }
rec_dir() { if is_study "$1"; then echo "$REC_ROOT/$1"; elif is_dev "$1"; then echo "$DEV_ROOT/$1"; else return 1; fi; }
# The cap-7 -m112 request (Amendment 7 B next step, cap 6 -> 7; Amendment 14 -m112: 112Gi, --max-dense 27900000000,
# job-name suffix -m112), derived from the frozen r2i cap-6 request of the same draw. Every other field,
# provenance.protocol_sha256 included, is copied unchanged; one sentence is appended to the note (the note stays within
# the validator's 2048-character limit for provenance strings).
c7_m112_request() {
  local src=$1 rel; rel=${src#"$study/"}
  jq -e '.job.memory == "56Gi" and .job.command == ["/usr/local/bin/entry.sh", "cap:14:0:6:12800000000"]
         and (.job.job_name | startswith("pfd-x-t14-k0-c6-"))' "$src" >/dev/null || return 1
  jq --arg rel "$rel" --arg st "$STUDY_STAGE" '.job.job_name |= sub("^pfd-x-t14-k0-c6"; "pfd-x-t14-k0-c7-m112")
    | .job.memory = "112Gi"
    | .job.command[1] = $st
    | .provenance.note += " Amendment 33: cap 7 (Amendment 7 B next step) at Amendment 14 -m112 (112Gi, --max-dense 27900000000, name suffix -m112), derived from \($rel) by runpod_capped/make_requests_m112.sh; other fields unchanged."' "$src"
}

# Network setup. Called only by the modes that talk to RunPod (the offline classifier does not need it).
rpc_init() {
  : "${RPC_STATE:?set RPC_STATE to a private scratch directory outside the git worktree (pod tokens, API header)}"
  local top rs
  rs=$(realpath -m "$RPC_STATE")
  top=$(git -C "$here" rev-parse --show-toplevel 2>/dev/null || true)
  for d in ${top:+"$top"} "$study"; do
    case "$rs/" in "$(realpath -m "$d")/"*) echo "REFUSED RPC_STATE $rs is inside $d (it holds the API header and pod tokens)" >&2; exit 3 ;; esac
  done
  mkdir -p "$RPC_STATE"; chmod 700 "$RPC_STATE"
  export TOKDIR="$RPC_STATE"              # rp_pinned.sh reads $TOKDIR/<pod id>.token
  hdr="$RPC_STATE/api.hdr.$$"             # removed on exit by rpc_cleanup
  ( umask 077; printf 'Authorization: Bearer %s\n' "${RUNPOD_API_KEY:-proxy-injected}" > "$hdr" )
}
rpc_cleanup() { [ -n "${hdr:-}" ] && rm -f "$hdr"; return 0; }
api() { curl -sS -m 60 -H @"$hdr" -H 'Content-Type: application/json' "$@"; }
# api_code METHOD URL OUTFILE [curl args...]: prints the HTTP status (000 when the call failed); never fails.
api_code() { local m=$1 u=$2 o=$3; shift 3; api -X "$m" "$u" -o "$o" -w '%{http_code}' "$@" 2>>"$o.err" || true; }
# Saved API records drop the pod environment (the runner token), the container command (it carries the runner's
# base64), SSH details and any account identifier.
redact() { jq 'del(.env, .args, .cmd, .entrypoint, .ssh, .consumerUserId)'; }
# r4 X: a price rounded to 4 decimals, for comparing a catalog price with a pod record's cost.
r4() { awk -v x="$1" 'BEGIN { printf "%.4f", x }'; }
# Pods whose name starts with pkm-x- (any status), one JSON line each. REST v2 returns {pods:[...], pagination:{hasNextPage,
# nextCursor}}; every page is read (cursor parameter). Fails if any page cannot be read or the shape is not that envelope.
list_pkm() {
  local t cur="" c n=0 all; t=$(mktemp); all=$(mktemp)
  while :; do
    n=$((n + 1)); [ "$n" -le 50 ] || { rm -f "$t" "$t.err" "$all"; return 1; }
    if [ -n "$cur" ]; then c=$(api_code GET "$API/pods" "$t" --get --data-urlencode "cursor=$cur"); else c=$(api_code GET "$API/pods" "$t"); fi
    [ "$c" = 200 ] && jq -e '(.pods | type) == "array" and (.pagination | type) == "object"' "$t" >/dev/null 2>&1 \
      || { rm -f "$t" "$t.err" "$all"; return 1; }
    jq -c '.pods[] | select((.name // "") | startswith("pkm-x-")) | {id,name,status,cost,createdAt,startedAt}' "$t" >> "$all" \
      || { rm -f "$t" "$t.err" "$all"; return 1; }
    jq -e '.pagination.hasNextPage == true' "$t" >/dev/null 2>&1 || break
    cur=$(jq -r '.pagination.nextCursor // empty' "$t"); [ -n "$cur" ] || { rm -f "$t" "$t.err" "$all"; return 1; }
  done
  cat "$all"; rm -f "$t" "$t.err" "$all"
}

# Outcome of a study name: its class from receipts/NAME/outcome.json, NONE (no receipts) or INFLIGHT (receipts, no outcome).
outcome_of() {
  local d="$REC_ROOT/$1"
  if [ ! -d "$d" ]; then echo NONE; elif [ -f "$d/outcome.json" ]; then jq -r '.class' "$d/outcome.json"; else echo INFLIGHT; fi
}
# Relaunch accounting (owner decision 13, main-session decision N1): only a LOST whose outcome says relaunch "auto"
# (positive host-side evidence) gets the cell's one relaunch; ABORTED, NO_EVIDENCE and a LOST with relaunch "owner"
# resolve the cell for the owner.
lostauto() { [ "$2" = LOST ] && jq -e '.relaunch == "auto"' "$REC_ROOT/$1/outcome.json" >/dev/null 2>&1; }
# next_allowed: the only study name run.sh may launch now, BLOCKED:<name>, or ALL_RESOLVED. The attempt here is the
# cell's -m112 attempt itself (the 56Gi attempt was the other session's, Amendment 30), so no size step follows it.
next_allowed() {
  local b=${STUDY_NAMES[0]} r=${STUDY_NAMES[1]} o
  o=$(outcome_of "$b")
  case $o in NONE) echo "$b"; return ;; INFLIGHT|NOT_AN_ATTEMPT|PENDING) echo "BLOCKED:$b"; return ;; esac
  lostauto "$b" "$o" || { echo ALL_RESOLVED; return; }
  o=$(outcome_of "$r")
  case $o in NONE) echo "$r" ;; INFLIGHT|NOT_AN_ATTEMPT|PENDING) echo "BLOCKED:$r" ;; *) echo ALL_RESOLVED ;; esac
}

# Study-wide cost ledger (owner decision 21: guard US$510, ceiling US$540). Rows: job route authority state usd source
# (tab-separated); usd is the actual cost when state is closed, the worst case when open. Prints the sum; fails if malformed.
ledger_sum() {
  [ -f "$LEDGER" ] || { echo "no ledger $LEDGER" >&2; return 1; }
  awk -F'\t' '/^#/ || /^job\t/ || NF == 0 { next }
    NF != 6 || ($4 != "open" && $4 != "closed") || $5 !~ /^[0-9]+(\.[0-9]+)?$/ { bad = 1; print "malformed ledger line " NR ": " $0 > "/dev/stderr" }
    { s += $5; n++ } END { if (bad || n == 0) exit 1; printf "%.4f\n", s }' "$LEDGER"
}
# ledger_has NAME: any ledger row (any state) for this name (N6: a name with a row is never launched again).
ledger_has() { awk -F'\t' -v n="$1" 'index($1, n "/") == 1 { f = 1 } END { exit !f }' "$LEDGER"; }
ledger_asof() { awk -F'\t' '$1 == "# complete_as_of" { print $2 }' "$LEDGER" | tail -1; }
ledger_add() { printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$1" runpod_capped "Amendment 33" open "$2" "$3" >> "$LEDGER"; }
ledger_close() { # ledger_close JOB USD SOURCE: closes this route's open row for JOB with the actual (derived) cost,
  local t; t=$(mktemp "$LEDGER.XXXX")  # or adds a closed row when none is open (a pod found only after a failed create)
  awk -F'\t' -v OFS='\t' -v j="$1" -v u="$2" -v s="$3" '$1 == j && $2 == "runpod_capped" && $4 == "open" { $4 = "closed"; $5 = u; $6 = s; hit = 1 } { print }
    END { if (!hit) print j, "runpod_capped", "Amendment 33", "closed", u, s }' "$LEDGER" > "$t"
  mv "$t" "$LEDGER"
}
