#!/usr/bin/env bash
# Resumable 4-worker queue for PROTOCOL.md §5. Safe to re-run after a container restart:
# done/<job> marks a finished job; an interrupted session directory is renamed
# <name>.attemptN-killed (never evidence) and the job is re-run.
set -u
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"; cd "$ROOT"
R=research/gaudry_k4_char2_replication_20261026
EC="${ECBENCH:-$ROOT/target-repl/release/ecbench}"
WORKERS="${WORKERS:-4}"
mkdir -p "$R/done" "$R/logs" /tmp/claude-0/k4q/claims
rm -rf /tmp/claude-0/k4q/claims/*   # claims from before a restart are stale
commit() {  # $1 message, rest: paths of this job only (never another worker's live session)
  local msg=$1; shift
  ( flock 9
    git add -- "$@" "$R/logs/queue.log" 2>/dev/null
    git -c user.name=Claude -c user.email=noreply@anthropic.com commit -q -m "k=4 char-2 replication: $msg

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_01JZkty38GW5FcgAcudQ45ph" >/dev/null 2>&1
    for d in 2 4 8 16; do timeout 300 git push -q origin HEAD >/dev/null 2>&1 && break; sleep $d; done
  ) 9>/tmp/claude-0/k4q/git.lock
}
job_paths() {  # files a finished job owns
  if [ "$1" = closure ]; then ls -d "$R/unit-price/closure-m$2.json" "$R/logs/closure-m$2.log" 2>/dev/null
  else ls -d "$R/sessions/$2" "$R/sessions/$2.log" "$R/sessions/$2".attempt*-killed* "$R/audits/verify-$2.json" 2>/dev/null; fi
}
run_job() {
  local kind=$1 name=$2 w=$3
  if [ "$kind" = closure ]; then
    "$EC" unit-price-f2m4 --m "$name" --closure --closure-variants refmm --reps 1 \
      --out "$R/unit-price/closure-m$name.json" > "$R/logs/closure-m$name.log" 2>&1
    return $?
  fi
  local out="$R/sessions/$name"
  if [ -e "$out" ]; then
    local n=1; while [ -e "$out.attempt$n-killed" ]; do n=$((n+1)); done
    mv "$out" "$out.attempt$n-killed"; [ -e "$out.log" ] && mv "$out.log" "$out.attempt$n-killed.log"
  fi
  "$EC" run --spec "$R/specs/$name.json" --out "$out" --cpus none --allow-busy --quiet \
    --lock "/tmp/claude-0/k4q/lock$w" > "$out.log" 2>&1 || return 1
  "$EC" verify --dir "$out" --replay 1 --exit-code --out "$R/audits/verify-$name.json" >> "$out.log" 2>&1
}
worker() {
  local w=$1
  while read -r kind name; do
    local job="$kind-$name"
    [ -e "$R/done/$job" ] && continue
    mkdir "/tmp/claude-0/k4q/claims/$job" 2>/dev/null || continue
    echo "$(date -u +%FT%TZ) w$w start $job" >> "$R/logs/queue.log"
    if run_job "$kind" "$name" "$w"; then
      date -u +%FT%TZ > "$R/done/$job"; echo "$(date -u +%FT%TZ) w$w done $job" >> "$R/logs/queue.log"
      commit "$job complete" "$R/done/$job" $(job_paths "$kind" "$name")
    else
      echo "$(date -u +%FT%TZ) w$w FAILED $job (exit $?)" >> "$R/logs/queue.log"; commit "$job FAILED" $(job_paths "$kind" "$name")
    fi
  done < "$R/jobs.txt"
}
for w in $(seq 0 $((WORKERS-1))); do worker "$w" & done
wait
echo "$(date -u +%FT%TZ) queue finished" >> "$R/logs/queue.log"; commit "queue finished" "$R/logs"
