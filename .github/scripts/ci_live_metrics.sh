#!/usr/bin/env bash
# Pull live CI health numbers from the GitHub REST API (read-only).
#
# Needs a working `gh` login with read access to Actions on the repository,
# plus `jq`.  Every API call goes through ghget() below, which only ever issues
# GET requests; nothing here creates, cancels, re-runs, deletes or dispatches.
#
# Reports (see .github/ci-measurement.md for how to read them):
#   1. queued / in-progress run counts now, and which workflows are queued
#   2. median runner queue time per workflow (job created_at -> started_at),
#      sampled from the most recent completed runs
#   3. minutes per workflow over the window (run wall-clock exact; runner
#      minutes estimated from the sampled runs)
#   4. rust-cache entries from the Actions cache API: saved sizes, reuse proxy,
#      repository cache usage
#
# Usage:
#   ci_live_metrics.sh [--repo OWNER/REPO] [--days 7] [--sample-workflows 25]
#                      [--sample-runs 4] [--max-pages 10] [--cache-prefix v0-rust]
#                      [--out DIR] [--dry-run] [-h]
#
# --dry-run prints the API calls that would be made and exits without calling gh.
set -euo pipefail

REPO="${GH_REPO:-}"
DAYS=7
SAMPLE_WORKFLOWS=25
SAMPLE_RUNS=4
MAX_PAGES=10
CACHE_PREFIX="v0-rust"
OUT_DIR=""
DRY_RUN=0

usage() { sed -n '2,/^set -euo/p' "$0" | sed '$d' | sed 's/^# \{0,1\}//'; }
die() { echo "ci_live_metrics: $*" >&2; exit 2; }

while [ $# -gt 0 ]; do
  case "$1" in
    --repo) REPO="${2:?--repo needs OWNER/REPO}"; shift 2 ;;
    --days) DAYS="${2:?}"; shift 2 ;;
    --sample-workflows) SAMPLE_WORKFLOWS="${2:?}"; shift 2 ;;
    --sample-runs) SAMPLE_RUNS="${2:?}"; shift 2 ;;
    --max-pages) MAX_PAGES="${2:?}"; shift 2 ;;
    --cache-prefix) CACHE_PREFIX="${2:?}"; shift 2 ;;
    --out) OUT_DIR="${2:?}"; shift 2 ;;
    --dry-run) DRY_RUN=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) die "unknown argument: $1 (try --help)" ;;
  esac
done
for v in DAYS SAMPLE_WORKFLOWS SAMPLE_RUNS MAX_PAGES; do
  case "${!v}" in ''|*[!0-9]*) die "--$(echo "$v" | tr 'A-Z_' 'a-z-') must be a positive integer" ;; esac
done

# `{owner}/{repo}` is expanded by gh from the current directory's git remote.
placeholder='{owner}/{repo}'
R="${REPO:-$placeholder}"

if [ "$DRY_RUN" -eq 1 ]; then
  cat <<EOF
ci_live_metrics dry run (no gh call is made). Repository: $R, window: last $DAYS days.
GET repos/$R/actions/runs?status=queued|in_progress|waiting|requested|pending&per_page=1   (5 calls, counts)
GET repos/$R/actions/runs?status=queued&per_page=100&page=1..$MAX_PAGES                  (<= $MAX_PAGES calls)
GET repos/$R/actions/runs?created=>=<window start>&per_page=100&page=1..$MAX_PAGES       (<= $MAX_PAGES calls)
GET repos/$R/actions/runs/<id>/jobs?per_page=100&filter=latest                          (<= $((SAMPLE_WORKFLOWS * SAMPLE_RUNS)) calls)
GET repos/$R/actions/cache/usage                                                          (1 call)
GET repos/$R/actions/caches?per_page=100&page=1..$MAX_PAGES                              (<= $MAX_PAGES calls)
GET rate_limit                                                                            (2 calls, free)
Upper bound: about $((5 + 2 * MAX_PAGES + SAMPLE_WORKFLOWS * SAMPLE_RUNS + 1 + MAX_PAGES + 2)) REST calls against the 5000/hour core limit.
EOF
  exit 0
fi

command -v gh >/dev/null 2>&1 || die "gh not found; install GitHub CLI and run 'gh auth login'"
command -v jq >/dev/null 2>&1 || die "jq not found"

# The only place that talks to GitHub.  --method GET makes -f fields query
# parameters and guarantees a read.
ghget() { gh api --method GET "$@"; }

work="$(mktemp -d "${TMPDIR:-/tmp}/ci-live-metrics.XXXXXX")"
trap 'rm -rf "$work"' EXIT

date_since() {
  if date -u -d "$DAYS days ago" +%Y-%m-%dT%H:%M:%SZ 2>/dev/null; then return; fi
  date -u -v-"${DAYS}"d +%Y-%m-%dT%H:%M:%SZ
}
SINCE="$(date_since)"

# paged <endpoint> <array-key> <outfile> [gh -f args...]
# Appends one compact JSON object per array element to <outfile>.
# GitHub caps filtered run listings at 1000 results; the caller is told when a
# listing hit the page cap.
paged() {
  local endpoint="$1" key="$2" out="$3" page=1 n resp
  shift 3
  : >"$out"
  resp="$work/page.json"
  while [ "$page" -le "$MAX_PAGES" ]; do
    ghget "$endpoint" -f per_page=100 -f page="$page" "$@" >"$resp"
    jq -c ".${key}[]" "$resp" >>"$out"
    n="$(jq ".${key} | length" "$resp")"
    if [ "$n" -lt 100 ]; then return 0; fi
    page=$((page + 1))
  done
  echo "ci_live_metrics: warning: $endpoint stopped at --max-pages=$MAX_PAGES; results are truncated" >&2
}

# Align tab-separated columns when `column` exists.
tab() { column -t -s "$(printf '\t')" 2>/dev/null || cat; }

# shellcheck disable=SC2016  # jq programs are single-quoted on purpose
JQ_LIB='
def median: sort | if length == 0 then null
  elif length % 2 == 1 then .[(length - 1) / 2]
  else (.[length / 2 - 1] + .[length / 2]) / 2 end;
def pct($q): sort | if length == 0 then null else .[((length * $q) | ceil) - 1 | if . < 0 then 0 else . end] end;
def r1: if . == null then null else (. * 10 | round) / 10 end;
def ts: fromdateiso8601;
def wfkey: ((.path // .name // "?") | split("@")[0] | split("/")[-1]);
def mib: . / 1048576;
'

echo "== CI live metrics: $R, window since $SINCE ($DAYS days) =="
echo "   generated $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo
echo "-- API rate limit before:"
ghget rate_limit --jq '.resources.core | "   core remaining \(.remaining)/\(.limit)"'
echo

# ---------------------------------------------------------------- 1. queue now
echo "== 1. Runs waiting now =="
for st in queued in_progress waiting requested pending; do
  c="$(ghget "repos/$R/actions/runs" -f status="$st" -f per_page=1 --jq .total_count)"
  printf '   %-12s %s\n' "$st" "$c"
done
paged "repos/$R/actions/runs" workflow_runs "$work/queued.jsonl" -f status=queued
echo
echo "   Queued runs by workflow (age = now - run created_at; includes runs not yet started):"
jq -s -r "$JQ_LIB"'
  if length == 0 then "   (nothing queued)" else
  group_by(wfkey) | map({wf: (.[0] | wfkey), n: length,
      ages: (map((now - (.created_at | ts)) / 60))}) | sort_by(-.n)[:25][]
  | "   \(.wf)\t\(.n) queued\tmedian age \(.ages | median | r1) min\tmax \(.ages | max | r1) min" end
' "$work/queued.jsonl" | tab
echo

# -------------------------------------------------------- window runs + jobs
paged "repos/$R/actions/runs" workflow_runs "$work/runs.jsonl" -f "created=>=$SINCE"
total_runs="$(wc -l <"$work/runs.jsonl" | tr -d ' ')"

# Pick the busiest workflows and their most recent completed runs.
jq -s -r --argjson k "$SAMPLE_WORKFLOWS" --argjson m "$SAMPLE_RUNS" "$JQ_LIB"'
  group_by(wfkey) | sort_by(-length)[:$k][]
  | (.[0] | wfkey) as $wf
  | [.[] | select(.status == "completed")] | sort_by(.created_at) | reverse | .[:$m][]
  | "\($wf)\t\(.id)"
' "$work/runs.jsonl" >"$work/sample.tsv"

: >"$work/jobs.jsonl"
sampled_runs="$(wc -l <"$work/sample.tsv" | tr -d ' ')"
while IFS="$(printf '\t')" read -r wf run_id; do
  [ -n "$run_id" ] || continue
  ghget "repos/$R/actions/runs/$run_id/jobs" -f per_page=100 -f filter=latest \
    | jq -c --arg wf "$wf" --argjson run "$run_id" '.jobs[] | . + {wf: $wf, run_id: $run}' >>"$work/jobs.jsonl"
done <"$work/sample.tsv"

# ------------------------------------------------------- 2. median queue time
echo "== 2. Runner queue time per workflow (job created_at -> started_at) =="
echo "   Sampled $sampled_runs completed runs (<= $SAMPLE_RUNS per workflow, $SAMPLE_WORKFLOWS busiest workflows)."
echo "   Skipped jobs are counted but excluded: they never wait for a runner."
jq -s -r "$JQ_LIB"'
  def q: ((.started_at | ts) - (.created_at | ts));
  [.[] | select(.conclusion != "skipped" and .started_at != null and .created_at != null)] as $ran
  | if ($ran | length) == 0 then "   (no sampled jobs)" else
    ([$ran[] | {wf, q: q}] | group_by(.wf) | map({wf: .[0].wf, jobs: length, qs: map(.q)})
      | sort_by(-(.qs | median))[]
      | "   \(.wf)\t\(.jobs) jobs\tmedian \(.qs | median | r1)s\tp90 \(.qs | pct(0.9))s\tmax \(.qs | max)s")
    end
' "$work/jobs.jsonl" | tab
echo
echo "   By runner label set:"
jq -s -r "$JQ_LIB"'
  def q: ((.started_at | ts) - (.created_at | ts));
  [.[] | select(.conclusion != "skipped" and .started_at != null)]
  | group_by((.labels // []) | join(","))
  | map({l: ((.[0].labels // []) | join(",")), qs: map(q)}) | sort_by(-(.qs | length))[]
  | "   \(if .l == "" then "(none)" else .l end)\t\(.qs | length) jobs\tmedian \(.qs | median | r1)s\tp90 \(.qs | pct(0.9))s"
' "$work/jobs.jsonl" | tab
echo
echo "   Skipped-job share per workflow (jobs that still appear as checks but use no runner):"
jq -s -r "$JQ_LIB"'
  group_by(.wf) | map({wf: .[0].wf, jobs: length, skipped: (map(select(.conclusion == "skipped")) | length)})
  | sort_by(-.skipped)[:15][] | select(.skipped > 0)
  | "   \(.wf)\t\(.skipped)/\(.jobs) skipped"
' "$work/jobs.jsonl" | tab
echo

# ----------------------------------------------------------- 3. minutes / 7d
echo "== 3. Minutes per workflow over the last $DAYS days =="
echo "   $total_runs runs listed$( [ "$total_runs" -ge 1000 ] && echo ' (GitHub caps a filtered listing at 1000; older runs in the window are missing)' )."
echo "   wall = sum of (updated_at - run_started_at) over completed runs, exact for the listed runs."
echo "   est runner min = mean sampled runner minutes per run x runs; only for sampled workflows."
jq -s -r --slurpfile jobs "$work/jobs.jsonl" "$JQ_LIB"'
  def jmin: if .started_at != null and .completed_at != null and .conclusion != "skipped"
            then (((.completed_at | ts) - (.started_at | ts)) / 60) else 0 end;
  ($jobs | group_by(.wf) | map({key: .[0].wf,
       value: {runs: (map(.run_id) | unique | length), min: (map(jmin) | add)}}) | from_entries) as $s
  | group_by(wfkey) | map(
      (.[0] | wfkey) as $wf
      | [.[] | select(.status == "completed" and .run_started_at != null)
            | (((.updated_at | ts) - (.run_started_at | ts)) / 60)] as $w
      | {wf: $wf, runs: length, wall: ($w | add // 0),
         est: (if $s[$wf] and $s[$wf].runs > 0 then ($s[$wf].min / $s[$wf].runs * ($w | length)) else null end)})
  | sort_by(-(.est // .wall))[:30][]
  | "   \(.wf)\t\(.runs) runs\twall \(.wall | r1) min\test runner \(if .est == null then "n/a" else (.est | r1 | tostring) end) min"
' "$work/runs.jsonl" | tab
echo

# ------------------------------------------------------------------ 4. caches
echo "== 4. Actions cache (prefix \"$CACHE_PREFIX\" = rust-cache) =="
ghget "repos/$R/actions/cache/usage" \
  --jq '"   repository usage: \(.active_caches_count) caches, \((.active_caches_size_in_bytes / 1073741824 * 100 | round) / 100) GiB (default limit 10 GiB; oldest evicted first)"'
paged "repos/$R/actions/caches" actions_caches "$work/caches.jsonl"
echo "   The API reports sizes and timestamps, not hits or misses."
echo "   save = cache created in window; reused proxy = last_accessed_at > created_at + 120 s."
read -r -d '' CACHE_DEFS <<'JQ' || true
def reused: ((.last_accessed_at | ts) - (.created_at | ts)) > 120;
def refclass: if .ref == "refs/heads/main" then "main" elif (.ref | startswith("refs/pull/")) then "pull" else "other" end;
def summ: {n: length, mib: (map(.size_in_bytes | mib) | add // 0),
           med: (map(.size_in_bytes | mib) | median), p90: (map(.size_in_bytes | mib) | pct(0.9)),
           max: (map(.size_in_bytes | mib) | max), reused: (map(select(reused)) | length)};
def line($l): "   \($l)\t\(.n) caches\t\(.mib | r1) MiB total\tmedian \(.med | r1) MiB\tp90 \(.p90 | r1) MiB\tmax \(.max | r1) MiB\treused \(.reused)/\(.n)";
JQ
CACHE_LIB="$JQ_LIB$CACHE_DEFS"
# shellcheck disable=SC2016
jq -s -r --arg p "$CACHE_PREFIX" --arg since "$SINCE" "$CACHE_LIB"'
  [.[] | select(.key | startswith($p))] as $rust
  | if ($rust | length) == 0 then "   (no caches with this key prefix)" else
    ($rust | summ | line("all rust caches")),
    ([$rust[] | select(.created_at >= $since)] | if length == 0 then "   saved in window\t0 caches" else (summ | line("saved in window")) end),
    ($rust | group_by(refclass)[] | (.[0] | refclass) as $c | summ | line("ref=\($c)")),
    ([$rust[] | select(reused | not)] | "   never reused after creation\t\(length) caches\t\(map(.size_in_bytes | mib) | add // 0 | r1) MiB total")
    end
' "$work/caches.jsonl" | tab
echo
echo "   Largest rust caches:"
# shellcheck disable=SC2016
jq -s -r --arg p "$CACHE_PREFIX" "$JQ_LIB"'
  [.[] | select(.key | startswith($p))] | sort_by(-.size_in_bytes)[:8][]
  | "   \(.size_in_bytes | mib | r1) MiB\t\(.ref)\tlast used \(.last_accessed_at)\t\(.key[:70])"
' "$work/caches.jsonl" | tab
echo
echo "   All caches by key family (first two dash-separated tokens), top 8 by size:"
# shellcheck disable=SC2016
jq -s -r "$JQ_LIB"'
  group_by(.key | split("-")[:2] | join("-")) | map({f: (.[0].key | split("-")[:2] | join("-")), n: length, mib: (map(.size_in_bytes | mib) | add)})
  | sort_by(-.mib)[:8][] | "   \(.f)\t\(.n) caches\t\(.mib | r1) MiB"
' "$work/caches.jsonl" | tab
echo
echo "-- API rate limit after:"
ghget rate_limit --jq '.resources.core | "   core remaining \(.remaining)/\(.limit)"'

if [ -n "$OUT_DIR" ]; then
  mkdir -p "$OUT_DIR"
  cp "$work/queued.jsonl" "$work/runs.jsonl" "$work/jobs.jsonl" "$work/caches.jsonl" "$OUT_DIR"/
  echo
  echo "Raw API rows (JSON lines) saved to $OUT_DIR"
fi
