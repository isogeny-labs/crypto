#!/bin/bash
# Phase B driver (follow-up census, AMENDMENTS A1): resumable, manifest order.
# Never deletes or edits a session except the prune of an archived, pushed one.
# Per session: run, verify, extract, archive, xz -t, progress line, commit, push,
# and only then prune that session's raw dir (census A1.6).
# Environment overrides (used by the driver test; defaults are the real campaign):
#   STUDY ECB ECB_SHA MINFREE_MIB MAXWALL_S COMMIT_TRAILER (text appended to commits)
# Stop between sessions: create the file runs/STOP (the driver exits 0 before the next session).
# One driver at a time: an flock on runs/driver.lock (exit 6 when held); pid in runs/driver.pid.
set -u
STUDY=${STUDY:-/home/user/crypto/research/volcano_ecdlp_census_followup_20261023}
ECB=${ECB:-/home/user/crypto/target/release/ecbench}
ECB_SHA=${ECB_SHA:-ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35}
MAXWALL=${MAXWALL_S:-$((20*3600))}; MINFREE_KB=$(( ${MINFREE_MIB:-1024} * 1024 )); MAXFAIL=3
COMMIT_TRAILER=${COMMIT_TRAILER:-}
cd "$STUDY" || exit 2
PROG=runs/progress.jsonl
mkdir -p sessions runs/extract runs/tables
exec 9> runs/driver.lock
if ! flock -n 9; then echo "DRIVER_LOCKED: another run_phase_b.sh (or its ecbench child) holds runs/driver.lock"; exit 6; fi
echo $$ > runs/driver.pid
touch $PROG
# a kill in the middle of a write can leave a last line without a newline: end it, never edit it
if [ -s $PROG ] && [ "$(tail -c1 $PROG | od -An -c | tr -d ' ')" != '\n' ]; then echo >> $PROG; fi
REPO=$(git rev-parse --show-toplevel) || exit 2
REL=$(realpath --relative-to="$REPO" "$STUDY")
BRANCH=$(git rev-parse --abbrev-ref HEAD)
# progress.jsonl as valid JSON lines only (a truncated line is skipped, never repaired)
prog() { jq -R -c 'try fromjson catch empty' $PROG; }
now() { date -u +%Y-%m-%dT%H:%M:%SZ; }
log() { jq -nc --arg t "$(now)" --arg m "$1" '{type:"note",time:$t,msg:$m}' >> $PROG; }
bin_sha() { sha256sum "$ECB" 2>/dev/null | cut -d' ' -f1; }
done_p() { prog | jq -e --arg s "$1" 'select(.spec==$s and .status=="done")' >/dev/null 2>&1; }
free_kb() { df -Pk "$STUDY" | awk 'NR==2{print $4}'; }
# trailing run of non-ok session lines (failed, audit_failed, archive_failed, ...)
audit_streak() { prog | jq -r 'select(.spec!=null and .status!=null)|(.audit_ok|tostring)' | tac | awk 'BEGIN{n=0}{if($1=="true")exit; n++}END{print n}'; }
num() { case "${1:-}" in ''|*[!0-9]*) echo 0;; *) echo "$1";; esac; }

check_bin() { [ "$(bin_sha)" = "$ECB_SHA" ]; }

git_publish() { # $1 = commit subject, then the repo-relative paths just added. Returns 0 only if pushed.
  local subj=$1; shift
  # commit only the listed paths (anything else staged in the index by someone else is left alone);
  # a commit that fails (for example a stale index.lock) is not hidden: the caller checks HEAD
  git -C "$REPO" commit -q -m "$subj${COMMIT_TRAILER:+

$COMMIT_TRAILER}" -- "$@" >/dev/null 2>&1 || true
  local i
  for i in 1 2 3 4 5; do
    if git -C "$REPO" push -q -u origin "$BRANCH" >/dev/null 2>&1; then return 0; fi
    git -C "$REPO" pull -q --no-rebase origin "$BRANCH" >/dev/null 2>&1 || true
    sleep $((i*3))
  done
  return 1
}
in_head() { git -C "$REPO" cat-file -e "HEAD:$1" 2>/dev/null; } # is this repo-relative path committed at HEAD?

# A done session whose archive is not tracked (a kill between the progress line and the commit):
# add it and publish it, so its evidence is in git and the prune helper accepts it.
catchup_archives() { # $1 = commit subject; extra repo-relative paths in $2.. are published with them
  local subj=$1; shift
  local a b paths=("$@") n=0
  for a in sessions/*.tar.xz; do
    [ -e "$a" ] || continue
    b=$(basename "$a" .tar.xz)
    prog | jq -e --arg o "sessions/$b" 'select(.out==$o and .status=="done")' >/dev/null 2>&1 || continue
    if ! in_head "$REL/$a" && git -C "$REPO" add -f -- "$STUDY/$a" >/dev/null 2>&1; then paths+=("$REL/$a"); n=$((n+1)); fi
    for f in runs/extract/$b.csv.xz runs/tables/$b.txt sessions/$b.log sessions/$b.audit.json sessions/$b.verify.log; do
      if [ -e "$f" ] && ! in_head "$REL/$f" && git -C "$REPO" add -f -- "$STUDY/$f" >/dev/null 2>&1; then paths+=("$REL/$f"); n=$((n+1)); fi
    done
  done
  if [ $n -gt 0 ] || [ ${#paths[@]} -gt 0 ]; then
    git_publish "$subj" "${paths[@]}" || return 1
  fi
  return 0
}

# ---- header (written once; a resumed run keeps the original start) ----
if ! prog | jq -e 'select(.type=="header")' >/dev/null 2>&1; then
  if ! check_bin; then echo "BINARY_SHA_MISMATCH at start: $(bin_sha)"; exit 4; fi
  jq -nc --arg iso "$(now)" --argjson u "$(date +%s)" --arg sha "$(bin_sha)" --arg br "$BRANCH" --arg c "$(git -C "$REPO" rev-parse HEAD)" \
    '{type:"header",start_unix:$u,start_iso:$iso,binary_sha256:$sha,branch:$br,commit:$c}' >> $PROG
fi
START=$(prog | jq -r 'select(.type=="header").start_unix' | head -1)
# a resumed run: the time since the last recorded line is a pause (driver not running), excluded from the wall
if prog | jq -e 'select(.spec!=null or .type=="cleanup" or .type=="note")' >/dev/null 2>&1; then
  last=$(prog | jq -r 'select(.time!=null)|.time' | tail -1)
  if [ -n "$last" ]; then
    jq -nc --arg f "$last" --arg r "$(now)" '{type:"pause",from:$f,resume:$r,note:"driver restarted; gap since the last recorded line excluded from the wall (A1.3)"}' >> $PROG
  fi
fi
if ! catchup_archives "volcano census follow-up: archives of done sessions that were not yet committed"; then log "push failed in start-up archive catch-up"; echo GUARD_PUSH; exit 5; fi

for name in $(jq -r '.sessions[].name' runs/manifest.json); do
  done_p "$name" && continue
  if [ -e runs/STOP ]; then log "stop file runs/STOP present before $name; exiting"; echo STOPPED; exit 0; fi
  # campaign wall = now - start - recorded pauses (driver not running)
  gap=$(prog | jq -s '[.[]|select(.type=="pause")|((.resume|fromdate)-(.from|fromdate))]|add // 0')
  el=$(( $(date +%s) - START - gap ))
  if [ "$(free_kb)" -lt $((2560*1024)) ]; then scripts/prune_local_sessions.sh $((2560*1024)) >> runs/prune.log 2>&1; fi
  if ! check_bin; then log "guard: binary sha256 mismatch before $name ($(bin_sha))"; echo GUARD_BINARY; exit 4; fi
  if [ $el -ge $MAXWALL ]; then log "guard: $((MAXWALL/3600))h wall reached before $name"; echo GUARD_WALL; exit 3; fi
  if [ "$(free_kb)" -lt $MINFREE_KB ]; then log "guard: free disk < $((MINFREE_KB/1024)) MiB before $name"; echo GUARD_DISK; exit 3; fi
  if [ "$(audit_streak)" -ge $MAXFAIL ]; then log "guard: $MAXFAIL failed sessions or audits in a row before $name"; echo GUARD_AUDIT; exit 3; fi
  out="sessions/$name"; n=1
  # an existing directory is an interrupted attempt: keep it, record it, rerun to .rN
  while [ -e "$out" ]; do
    if ! prog | jq -e --arg o "$out" 'select(.out==$o)' >/dev/null 2>&1; then
      jq -nc --arg t "$(now)" --arg s "$name" --arg o "$out" '{type:"orphan_dir",time:$t,spec:$s,out:$o,note:"directory existed without a progress line; kept as evidence"}' >> $PROG
    fi
    n=$((n+1)); out="sessions/$name.r$n"
  done
  base=$(basename $out)
  row=$(jq -c --arg n "$name" '.sessions[]|select(.name==$n)' runs/manifest.json)
  t0=$(date +%s)
  rc_run=-1; rc_ver=-1; audit_ok=false; sstat=missing; runs=0; fails=0; raw=0; xzb=0; sha=""; xz_ok=false
  if [ -f specs/$name.json ]; then
    $ECB run --wait --cpus auto --spec specs/$name.json --out $out > $out.log 2>&1; rc_run=$?
  else
    echo "spec file specs/$name.json missing" > $out.log
  fi
  if [ -f $out/session.json ] && [ -f $out/records.jsonl ]; then
    $ECB verify --dir $out --replay 12 --out $out.audit.json > $out.verify.log 2>&1; rc_ver=$?
    audit_ok=$(jq -r '.ok // false' $out.audit.json 2>/dev/null); [ "$audit_ok" = true ] || audit_ok=false
    $ECB table --dir $out > runs/tables/$base.txt 2>&1
    V=$(jq -r .V <<<"$row"); F=$(jq -r .family <<<"$row"); L=$(jq -r .level <<<"$row"); C=$(jq -r .chunk <<<"$row")
    { echo 'spec,volcano,family,level,chunk,curve_slug,workload_id,target_index,arm,role,round,warmup,ops_gae,S,online_wall_ns,solve_wall_ns,status,verified,isolation'
      jq -r --arg spec "$base" --arg V "$V" --arg F "$F" --arg L "$L" --arg C "$C" \
        '[$spec,$V,$F,$L,$C,.workload.curve.slug,.workload.workload_id,.workload.target_index,.arm,.role,.round,.warmup,.cost.total_gae,.cost.s,.online.wall_ns,.time.solve_wall_ns,.outcome.status,(.outcome.status=="verified"),.isolation.level]|@csv' $out/records.jsonl
    } | xz -9e > runs/extract/$base.csv.xz
    runs=$(num "$(wc -l < $out/records.jsonl)")
    fails=$(num "$(jq -c 'select(.warmup==false and .outcome.status!="verified")' $out/records.jsonl | wc -l)")
    sstat=$(jq -r '.status // "unknown"' $out/session.json 2>/dev/null); [ -n "$sstat" ] || sstat=unknown
  fi
  if [ -d $out ]; then
    raw=$(num "$(du -sb $out | cut -f1)")
    tar -C sessions -cf - $base | xz -9e -T1 > sessions/$base.tar.xz
    xz -t sessions/$base.tar.xz 2>/dev/null || { mv sessions/$base.tar.xz sessions/$base.tar.xz.partial; tar -C sessions -cf - $base | xz -9e -T1 > sessions/$base.tar.xz; }
    xz -t sessions/$base.tar.xz 2>/dev/null && xz_ok=true || xz_ok=false
    xzb=$(num "$(stat -c %s sessions/$base.tar.xz 2>/dev/null)"); sha=$(sha256sum sessions/$base.tar.xz 2>/dev/null | cut -d' ' -f1)
  fi
  wall=$(( $(date +%s) - t0 ))
  # status: failed = no complete session (run crashed, spec or output missing); never skipped silently
  st=done
  [ "$sstat" != "complete" ] && st=failed
  [ "$st" = done ] && [ "$audit_ok" != "true" ] && st=audit_failed
  [ "$st" = done ] && [ "$xz_ok" != true ] && st=archive_failed
  [ "$st" != done ] && audit_ok=false
  jq -nc --arg spec "$name" --arg out "$out" --arg st "$st" --arg ss "$sstat" --argjson ok "$audit_ok" --argjson runs "$runs" --argjson fails "$fails" \
    --argjson raw "$raw" --argjson xz "$xzb" --arg sha "$sha" --argjson wall "$wall" --arg t "$(now)" --argjson rr "$rc_run" --argjson rv "$rc_ver" \
    '{spec:$spec,out:$out,status:$st,session_status:$ss,audit_ok:$ok,runs:$runs,failures:$fails,bytes_raw:$raw,bytes_xz:$xz,sha256_tar_xz:$sha,wall_s:$wall,time:$t,rc_run:$rr,rc_verify:$rv}' >> $PROG
  # commit + push this session's evidence (explicit paths: raw session dirs are never added)
  pub=()
  for f in $PROG runs/prune.log runs/extract/$base.csv.xz runs/tables/$base.txt sessions/$base.tar.xz sessions/$base.log sessions/$base.audit.json sessions/$base.verify.log $out.log $out.audit.json $out.verify.log; do
    if [ -e "$f" ] && git -C "$REPO" add -f -- "$STUDY/$f" >/dev/null 2>&1; then pub+=("$REL/$f"); fi
  done
  pushed=false
  if git_publish "volcano census follow-up: $base $st" "${pub[@]}"; then
    # the archive must be in HEAD: a failed commit followed by a push of an unchanged HEAD is not a publish
    if [ ! -e sessions/$base.tar.xz ] || in_head "$REL/sessions/$base.tar.xz"; then pushed=true; fi
  fi
  [ "$pushed" = true ] || log "push or commit failed after $base; not pruning"
  # prune only a done, pushed session (A1.6 conditions re-checked by the helper)
  if [ "$st" = done ] && [ "$pushed" = true ]; then
    scripts/prune_local_sessions.sh --session $base >> runs/prune.log 2>&1
  elif [ "$pushed" != true ]; then echo GUARD_PUSH; exit 5; fi
done
log "manifest finished"
fin=()
for f in $PROG runs/prune.log; do
  if [ -e "$f" ] && git -C "$REPO" add -f -- "$STUDY/$f" >/dev/null 2>&1; then fin+=("$REL/$f"); fi
done
catchup_archives "volcano census follow-up: Phase B finished" "${fin[@]}" || echo "final push failed"
echo ALLDONE
