#!/usr/bin/env bash
# Main-session orchestration for ONE RunPod CPU pod running job.sh on a bundle.   [pcc:unverified]
#   launch.sh --bundle BUNDLE.tgz --out DIR --deadline-h H [--vcpu 32] [--flavor cpu5m] [--disk 60]
#             [--poll-s 600] [--stamp STAMP] [--dry-run]
# Flow: create pod pkm-m8-panelA-<stamp> (prints id and $/h) -> wait -> put bundle.tgz and job.sh ->
#       exec job.sh -> poll run/out/done.json every --poll-s -> get results.tgz (sha256-checked against done.json)
#       -> delete pod. At the hard wall deadline (--deadline-h after create) it tars whatever exists
#       (run/out incl. runs.jsonl and the partial row logs), fetches it as partial.tgz and deletes the pod.
# The pod is deleted on every exit path (trap), including errors and ctrl-C, but only the pod this run created.
# --dry-run: no RunPod API call and no RUNPOD_API_KEY use; a local directory is the "pod" and job.sh runs locally
#            with --dry-run (system msolve and time, no apt-get).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
VCPU=32; FLAVOR=cpu5m; DISK=60; POLL=600; DEADLINE_H=; BUNDLE=; OUT=; DRY=0; STAMP=$(date -u +%Y%m%dT%H%M%SZ)
while [ $# -gt 0 ]; do case "$1" in
  --bundle) BUNDLE=$2; shift 2;; --out) OUT=$2; shift 2;; --deadline-h) DEADLINE_H=$2; shift 2;;
  --vcpu) VCPU=$2; shift 2;; --flavor) FLAVOR=$2; shift 2;; --disk) DISK=$2; shift 2;; --poll-s) POLL=$2; shift 2;;
  --stamp) STAMP=$2; shift 2;; --dry-run) DRY=1; shift;; *) echo "unknown arg $1" >&2; exit 2;; esac; done
[ -f "${BUNDLE:-}" ] && [ -n "$OUT" ] && [ -n "$DEADLINE_H" ] || { sed -n 2,13p "$0" >&2; exit 2; }
mkdir -p "$OUT"; OUT=$(cd "$OUT" && pwd); BUNDLE=$(readlink -f "$BUNDLE")
NAME=pkm-m8-panelA-$STAMP
export RP_STATE=$OUT/state
LOG=$OUT/launch.log
say() { echo "$(date -u +%FT%TZ) $*" | tee -a "$LOG"; }
RP() { "$here/rp.sh" "$@"; }
CREATED=0; DELETED=0
cleanup() {
  rc=$?
  if [ $DRY = 0 ] && [ $CREATED = 1 ] && [ $DELETED = 0 ]; then
    say "cleanup: deleting pod $NAME"
    for k in 1 2 3; do RP delete "$NAME" >>"$LOG" 2>&1 && { DELETED=1; break; }; sleep 5; done
    [ $DELETED = 1 ] && say "pod deleted" || say "WARNING: pod $NAME NOT deleted, delete it by hand"
  fi
  exit $rc
}
trap cleanup EXIT; trap 'say "interrupted"; exit 130' INT TERM

START=$(date +%s); DEADLINE=$((START + $(python3 -c "print(int(float('$DEADLINE_H')*3600))")))
get_remote() { # REMOTE LOCAL
  if [ $DRY = 1 ]; then cp "$OUT/fakepod/work/$1" "$2" 2>/dev/null; else RP get "$NAME" "$1" "$2" 2>/dev/null; fi; }
exec_remote() { # CMD (detached; real pod returns exec id)
  if [ $DRY = 1 ]; then ( cd "$OUT/fakepod/work"; nohup bash -c "$1" > exec.log 2>&1 < /dev/null & ) ; else RP exec "$NAME" "$1"; fi; }

# ---- create -------------------------------------------------------------------
PRICE=null; POD_ID=dry-run
if [ $DRY = 0 ]; then
  say "create $NAME vcpu=$VCPU flavor=$FLAVOR disk=${DISK}GB"
  POD_ID=$(RP create "$NAME" "$VCPU" "$FLAVOR" "$DISK"); CREATED=1
  INFO=$(RP info "$NAME"); say "POD_ID=$POD_ID  $INFO"
  PRICE=$(echo "$INFO" | sed -n 's/.*usd_per_hr \([0-9.]*\).*/\1/p'); PRICE=${PRICE:-null}
  say "HOURLY_PRICE_USD=$PRICE (record this in the protocol)"
  RP wait "$NAME" | tee -a "$LOG"
  RP put "$NAME" "$BUNDLE" bundle.tgz | tee -a "$LOG"
  RP put "$NAME" "$here/job.sh" job.sh | tee -a "$LOG"
else
  mkdir -p "$OUT/fakepod/work"; cp "$BUNDLE" "$OUT/fakepod/work/bundle.tgz"; cp "$here/job.sh" "$OUT/fakepod/work/job.sh"
  say "DRY RUN: local dir $OUT/fakepod/work is the pod; no RunPod call"
fi
printf '{"pod_name":"%s","pod_id":"%s","usd_per_hr":%s,"vcpu":%s,"flavor":"%s","start_epoch":%s,"deadline_epoch":%s,"dry_run":%s}\n' \
  "$NAME" "$POD_ID" "$PRICE" "$VCPU" "$FLAVOR" "$START" "$DEADLINE" "$DRY" > "$OUT/launch.json"

# ---- run ----------------------------------------------------------------------
if [ $DRY = 1 ]; then JOBCMD="bash job.sh bundle.tgz run --dry-run"; else JOBCMD="bash /work/job.sh /work/bundle.tgz /work/run"; fi
EXECID=$(exec_remote "$JOBCMD"); say "job started (exec id: ${EXECID:-local})"

# ---- poll ---------------------------------------------------------------------
GOT=0; PARTIAL=0
while :; do
  if get_remote run/out/done.json "$OUT/done.json"; then GOT=1; say "done.json present"; break; fi
  now=$(date +%s)
  if [ $now -ge $DEADLINE ]; then say "HARD DEADLINE reached: fetching what exists"; PARTIAL=1; break; fi
  # job gone without done.json (exec rc present) -> fetch what exists now
  if [ $DRY = 0 ]; then rcv=$(RP rc "$NAME" "${EXECID//[^0-9]/}" 2>/dev/null || echo running); case "$rcv" in running) ;; *) say "job exec ended (rc $rcv) without done.json"; PARTIAL=1; break;; esac; fi
  get_remote run/out/progress.json "$OUT/progress.json" && say "progress: $(cat "$OUT/progress.json")" || say "waiting (no row finished yet)"
  sleep $(( DEADLINE - now < POLL ? DEADLINE - now + 1 : POLL ))
done

# ---- fetch --------------------------------------------------------------------
if [ $GOT = 1 ] && get_remote run/out/results.tgz "$OUT/results.tgz"; then
  want=$(python3 -c "import json;print(json.load(open('$OUT/done.json'))['results_tgz_sha256'])"); have=$(sha256sum "$OUT/results.tgz" | cut -d' ' -f1)
  [ "$want" = "$have" ] && say "results.tgz sha256 ok $have" || { say "ERROR results.tgz sha256 mismatch (done.json $want, got $have)"; PARTIAL=1; }
else PARTIAL=1; fi
if [ $PARTIAL = 1 ]; then
  WR=/work; [ $DRY = 1 ] && WR="$OUT/fakepod/work"
  CMD="cd $WR/run 2>/dev/null && tar czf $WR/partial.tgz --exclude=out/.helper.py out 2>&1; echo partial-tar-rc=\$?"
  if [ $DRY = 1 ]; then (cd "$OUT/fakepod/work" && bash -c "$CMD") >/dev/null 2>&1 || true; else
    id=$(RP exec "$NAME" "$CMD" | python3 -c "import json,sys;print(json.load(sys.stdin)['id'])"); for k in $(seq 1 60); do [ "$(RP rc "$NAME" "$id")" != running ] && break; sleep 2; done; fi
  get_remote partial.tgz "$OUT/partial.tgz" && say "fetched partial.tgz ($(stat -c %s "$OUT/partial.tgz") bytes; job may still be running, rows after the cut are lost)" || say "ERROR: nothing fetched"
fi
if [ $DRY = 0 ]; then
  RP ls "$NAME" > "$OUT/pod_ls.json" 2>/dev/null || true
  RP delete "$NAME" | tee -a "$LOG"; DELETED=1
  END=$(date +%s); python3 - "$OUT/launch.json" "$END" "$PRICE" <<'P'
import json,sys
f,end,price=sys.argv[1:]; d=json.load(open(f)); d["end_epoch"]=int(end); h=(int(end)-d["start_epoch"])/3600
d["pod_hours"]=round(h,4); d["cost_usd_est"]=round(h*float(price),3) if price!="null" else None; json.dump(d,open(f,"w"),indent=1); print("pod hours",d["pod_hours"],"est cost USD",d["cost_usd_est"])
P
fi
say "finished (results: $([ $PARTIAL = 0 ] && echo complete || echo PARTIAL))"
