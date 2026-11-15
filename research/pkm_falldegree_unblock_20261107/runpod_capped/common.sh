# Shared settings and helpers for run.sh, fetch.sh and reap.sh (draft Amendment 18, AMENDMENT_R2_H_runpod_capped.md).
# Sourced, not run. Shell only (bash, jq, curl, awk, coreutils).
# The pod-side runner is Amendment 16's runpod/server.py, unchanged and pinned by SHA-256 below. Its client is a pinned,
# unchanged copy of main's runpod/rp.sh at d2a16c723d (rp_pinned.sh), used only for upload, start, status and fetch (those
# calls talk only to the pod, with the pod's token). The RunPod API calls (price, create, pod record, delete, list) are
# made here, with the key in a mode-0600 header file.
set -euo pipefail
here=${here:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}
study=$(cd "$here/.." && pwd)
PREREG="$study/../prime_fall_degree_20261027/PREREGISTRATION.md"
AMENDMENT_HEADING='## Amendment 18'       # renumbered with the amendment if PR #4173 closes unmerged (section 8)
RP="$here/rp_pinned.sh"
RP_SHA=b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d       # = main's runpod/rp.sh at d2a16c723d
SERVER="$study/runpod/server.py"
SERVER_SHA=f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550   # Amendment 16's runner, unchanged
API=https://rest.runpod.io/v1
GQL=https://api.runpod.io/graphql
FLAVOR=cpu5m VCPU=16 RAM_GB=128           # the one pinned flavor (decision D8): 16 vCPU, 128 GB; no fallback
INSTANCE="$FLAVOR-$VCPU-$RAM_GB"
MEM_FLOOR_GB=61 MEM_FLOOR_M112_GB=121     # 56 GiB = 60.13 GB; 112 GiB = 120.26 GB (Amendment 14)
HOURS=26 CEIL_USD=40 STUDY_CEIL_USD=400   # decision 4: price x 26 h <= US$40 per job; US$400 for the study
DEADLINE_S=90000                          # the registered job time limit (requests: seconds 90000), from the runner's start
START_WITHIN_S=3300                       # the runner must start within 55 min of creation, so start + 25 h <= creation + 26 h
HARD_S=93600                              # 26 h after creation: the reaper deletes any pkm-x- pod older than this
UNREACH_GAP_S=3600                        # two unreachable check-ins at least 1 h apart (with an unhealthy REST record)
LEDGER_FRESH_S=86400                      # the ledger's complete_as_of line must be at most 24 h old for a study launch
POLL_S=${RPC_POLL_S:-10}                    # seconds between API polls (only the tests shorten it)
RAYON=12                                  # Amendment 9; rp_pinned.sh start sends rayon 12
REC_ROOT="$here/receipts" DEV_ROOT="$here/receipts_dev" OUT_ROOT="$here/results" REAP_DIR="$here/reaper"
LEDGER="$here/ledger.tsv"
# The allow-list (decisions D2, D5). Order: cell t14-k0, then t14-k1, then t13-k0; a relaunch or -m112 run immediately
# follows its base. t13-k0's only attempt is -r3 (decision 10); it has no relaunch.
CELLS=(t14-k0-c6 t14-k1-c6 t13-k0-c6)
STUDY_NAMES=(pkm-x-t14-k0-c6 pkm-x-t14-k0-c6-r1 pkm-x-t14-k0-c6-m112 pkm-x-t14-k0-c6-m112-r1
             pkm-x-t14-k1-c6 pkm-x-t14-k1-c6-r1 pkm-x-t14-k1-c6-m112 pkm-x-t14-k1-c6-m112-r1
             pkm-x-t13-k0-c6-r3 pkm-x-t13-k0-c6-m112)

now() { date -u +%FT%TZ; }
epoch() { date -u -d "$1" +%s; }
is_study() { local n; for n in "${STUDY_NAMES[@]}"; do [ "$1" = "$n" ] && return 0; done; return 1; }
is_dev() { [[ $1 =~ ^pkm-x-dev-[a-z0-9]+(-[a-z0-9]+)*$ ]]; }
rec_dir() { if is_study "$1"; then echo "$REC_ROOT/$1"; elif is_dev "$1"; then echo "$DEV_ROOT/$1"; else return 1; fi; }
cell_of() { [[ $1 =~ ^pkm-x-(t[0-9]+-k[01]-c[0-9]+) ]] && echo "${BASH_REMATCH[1]}"; }
is_m112() { [[ $1 == *-m112 || $1 == *-m112-r1 ]]; }
# The frozen request a study name runs: r2i for the 56Gi runs, the generated requests_m112 file for -m112 runs.
name_request() {
  local c; c=$(cell_of "$1")
  if is_m112 "$1"; then echo "$here/requests_m112/pfd-x-$c-m112.json"; else echo "$study/requests_r2i/pfd-x-$c.json"; fi
}
# Amendment 14's -m112 request, derived from an r2i request: 112Gi, --max-dense 27900000000, job-name suffix -m112.
# Every other field, provenance.protocol_sha256 included, is copied unchanged; the note is appended to.
m112_request() {
  local src=$1 rel; rel=${src#"$study/"}
  jq -e '.job.memory == "56Gi" and (.job.command[1] | test(":12800000000$"))' "$src" >/dev/null || return 1
  jq --arg rel "$rel" '.job.job_name |= sub("^(?<c>pfd-x-t[0-9]+-k[01]-c[0-9]+)"; "\(.c)-m112")
    | .job.memory = "112Gi"
    | .job.command[1] |= sub(":12800000000$"; ":27900000000")
    | .provenance.note += " Amendment 14 -m112 re-run request (112Gi, --max-dense 27900000000, job-name suffix -m112), generated from \($rel) by research/pkm_falldegree_unblock_20261107/runpod_capped/make_requests_m112.sh for draft Amendment 18 (RunPod route); every other field is copied unchanged, including provenance.protocol_sha256."' "$src"
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
redact() { jq 'del(.env, .dockerStartCmd, .consumerUserId)'; }
# Pods whose name starts with pkm-x- (any status), one JSON line each. Fails if the list cannot be read.
list_pkm() {
  local t; t=$(mktemp); local c; c=$(api_code GET "$API/pods" "$t")
  [ "$c" = 200 ] && jq -e 'type == "array"' "$t" >/dev/null || { rm -f "$t" "$t.err"; return 1; }
  jq -c '.[] | select(.name | startswith("pkm-x-")) | {id,name,desiredStatus,costPerHr,createdAt,lastStartedAt}' "$t"
  rm -f "$t" "$t.err"
}

# Outcome of a study name: its class from receipts/NAME/outcome.json, NONE (no receipts) or INFLIGHT (receipts, no outcome).
outcome_of() {
  local d="$REC_ROOT/$1"
  if [ ! -d "$d" ]; then echo NONE; elif [ -f "$d/outcome.json" ]; then jq -r '.class' "$d/outcome.json"; else echo INFLIGHT; fi
}
lostlike() { [ "$1" = LOST ] || [ "$1" = ABORTED ]; }
needs_m112() { [ "$2" = OOM ] || { [ "$2" = ADMITTED ] && jq -e '.row.oversize == true' "$REC_ROOT/$1/outcome.json" >/dev/null 2>&1; }; }
# cell_next CELL BASE RELAUNCH(0|1): the next name this cell needs, BLOCKED:<name>, or RESOLVED:<name>:<class>.
cell_next() {
  local c=$1 cur=$2 rl=$3 r1used=0 o
  o=$(outcome_of "$cur")
  case $o in NONE) echo "$cur"; return ;; INFLIGHT|NOT_AN_ATTEMPT|PENDING) echo "BLOCKED:$cur"; return ;; esac
  if lostlike "$o"; then
    [ "$rl" = 1 ] || { echo "RESOLVED:$cur:$o"; return; }
    cur="pkm-x-$c-r1"; r1used=1; o=$(outcome_of "$cur")
    case $o in NONE) echo "$cur"; return ;; INFLIGHT|NOT_AN_ATTEMPT|PENDING) echo "BLOCKED:$cur"; return ;; esac
    lostlike "$o" && { echo "RESOLVED:$cur:$o"; return; }
  fi
  needs_m112 "$cur" "$o" || { echo "RESOLVED:$cur:$o"; return; }
  cur="pkm-x-$c-m112"; o=$(outcome_of "$cur")
  case $o in NONE) echo "$cur"; return ;; INFLIGHT|NOT_AN_ATTEMPT|PENDING) echo "BLOCKED:$cur"; return ;; esac
  if lostlike "$o"; then
    { [ "$rl" = 1 ] && [ "$r1used" = 0 ]; } || { echo "RESOLVED:$cur:$o"; return; }
    cur="pkm-x-$c-m112-r1"; o=$(outcome_of "$cur")
    case $o in NONE) echo "$cur"; return ;; INFLIGHT|NOT_AN_ATTEMPT|PENDING) echo "BLOCKED:$cur"; return ;; esac
  fi
  echo "RESOLVED:$cur:$o"
}
# next_allowed: the only study name run.sh may launch now, or BLOCKED:<name>, or ALL_RESOLVED.
next_allowed() {
  local c r
  for c in "${CELLS[@]}"; do
    case $c in t13-*) r=$(cell_next "$c" "pkm-x-$c-r3" 0) ;; *) r=$(cell_next "$c" "pkm-x-$c" 1) ;; esac
    case $r in RESOLVED:*) continue ;; *) echo "$r"; return ;; esac
  done
  echo ALL_RESOLVED
}

# Study-wide cost ledger (owner decision 4: US$400). Rows: job route authority state usd source (tab-separated);
# usd is the actual cost when state is closed, the worst case when open. Prints the sum; fails if malformed.
ledger_sum() {
  [ -f "$LEDGER" ] || { echo "no ledger $LEDGER" >&2; return 1; }
  awk -F'\t' '/^#/ || /^job\t/ || NF == 0 { next }
    NF != 6 || ($4 != "open" && $4 != "closed") || $5 !~ /^[0-9]+(\.[0-9]+)?$/ { bad = 1; print "malformed ledger line " NR ": " $0 > "/dev/stderr" }
    { s += $5; n++ } END { if (bad || n == 0) exit 1; printf "%.4f\n", s }' "$LEDGER"
}
ledger_asof() { awk -F'\t' '$1 == "# complete_as_of" { print $2 }' "$LEDGER" | tail -1; }
ledger_add() { printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$1" runpod_capped "draft Amendment 18" open "$2" "$3" >> "$LEDGER"; }
ledger_close() { # ledger_close JOB USD SOURCE: closes this route's open row for JOB with the actual (derived) cost,
  local t; t=$(mktemp "$LEDGER.XXXX")  # or adds a closed row when none is open (a pod found only after a failed create)
  awk -F'\t' -v OFS='\t' -v j="$1" -v u="$2" -v s="$3" '$1 == j && $2 == "runpod_capped" && $4 == "open" { $4 = "closed"; $5 = u; $6 = s; hit = 1 } { print }
    END { if (!hit) print j, "runpod_capped", "draft Amendment 18", "closed", u, s }' "$LEDGER" > "$t"
  mv "$t" "$LEDGER"
}
