#!/usr/bin/env bash
# Main-session orchestration for ONE RunPod CPU pod running job.sh on a bundle (protocol v2).   [pcc:unverified]
#   launch.sh --bundle BUNDLE.tgz --out DIR [--deadline-h 72] [--poll-s 600] [--stamp STAMP] [--dry-run]
#   launch.sh --replace --bundle BUNDLE.tgz --out DIR_OF_FIRST_RUN [--deadline-h H]      (one replacement pod, same flavor)
# Flavor is fixed: cpu5m, 32 vCPU / 256 GB. If `rp.sh flavors` does not show it available, this script exits (rc 3)
# BEFORE creating anything. Prints pod id and $/h at creation. Refuses a bundle that contains hidden.json.
# Flow: flavor check -> create pod pkm-m8-panelA-<stamp> -> wait -> put bundle.tgz and job.sh -> exec job.sh ->
#   every --poll-s (default 10 min): fetch runs.jsonl and any new verify/log_table/watchdog json into DIR (a lost pod
#   loses at most the row in flight) -> at done.json fetch results.tgz (sha256-checked) -> delete pod.
#   At the hard deadline it syncs, tars whatever exists (partial.tgz), fetches it and deletes the pod.
# --replace: DIR must hold the first run's runs.jsonl (synced). Builds a plan with the rows flagged `on_replacement`
#   (A/A repeat) first, then every row whose `order` is not in DIR/runs.jsonl, creates ONE pod (refuses a second
#   replacement), runs with REPLACEMENT=1 and POD_ID, rows are tagged with the pod id. Output goes to DIR/replacement/.
# The pod is deleted on every exit path (trap), only the pod this run created.
# Hardening [pcc:unverified] (see HARDENING.md): guard.sh runs on the pod and stops compute at deadline + 6 h (and once
#   results are fetched); fetched.flag is set on the pod after every fetch; at creation the state file is copied to
#   $RESUME_DIR/pod_state/ (default /home/user/crypto/.resume) and one line is added under `## Pod in flight` in RESUME.md.
#   launch.sh --attach NAME --out DIR [--deadline-h H] [--poll-s S]    (resume after the controlling session died)
#     no flavor gate, no create/put/job exec: loads the pod from the state file ($RP_STATE, else DIR/state, else the
#     .resume copy), takes created/deadline from DIR/launch.json, then the normal poll -> fetch -> flag -> delete.
#   Test-only: LAUNCH_TEST_NAME=pkm-m8-smoke-* [LAUNCH_TEST_FLAVOR LAUNCH_TEST_VCPU LAUNCH_TEST_DISK LAUNCH_TEST_GRACE_S] bypass the flavor gate;
#   refused unless the name starts with pkm-m8-smoke-.
# --dry-run: no RunPod call and no RUNPOD_API_KEY use; a local directory is the "pod"; job.sh runs locally with --dry-run.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
POLL=600; DEADLINE_H=72; BUNDLE=; OUT=; DRY=0; REPLACE=0; STAMP=$(date -u +%Y%m%dT%H%M%SZ)
VCPU=32; FLAVOR=cpu5m; DISK=60; ATTACH=; DH_GIVEN=0; GRACE_S=21600; TESTMODE=0
RESUME_DIR=${RESUME_DIR:-/home/user/crypto/.resume}
while [ $# -gt 0 ]; do case "$1" in
  --bundle) BUNDLE=$2; shift 2;; --out) OUT=$2; shift 2;; --deadline-h) DEADLINE_H=$2; DH_GIVEN=1; shift 2;; --poll-s) POLL=$2; shift 2;;
  --stamp) STAMP=$2; shift 2;; --dry-run) DRY=1; shift;; --replace) REPLACE=1; shift;; --attach) ATTACH=$2; shift 2;; *) echo "unknown arg $1" >&2; exit 2;; esac; done
if [ -n "$ATTACH" ]; then [ -n "$OUT" ] && [ -f "$OUT/launch.json" ] && [ $DRY = 0 ] && [ $REPLACE = 0 ] || { echo "--attach NAME needs --out DIR holding launch.json (no --dry-run, no --replace)" >&2; exit 2; }
else [ -f "${BUNDLE:-}" ] && [ -n "$OUT" ] || { sed -n 2,28p "$0" >&2; exit 2; }; fi
mkdir -p "$OUT"; OUT=$(cd "$OUT" && pwd); [ -n "$BUNDLE" ] && BUNDLE=$(readlink -f "$BUNDLE"); ORIG=$OUT
say() { echo "$(date -u +%FT%TZ) $*" | tee -a "$OUT/launch.log"; }

# ---- bundle checks (before anything is created) -----------------------------------------
if [ -z "$ATTACH" ]; then
tar tzf "$BUNDLE" | awk -F/ 'tolower($NF)=="hidden.json"{f=1} END{exit f}' || { echo "refusing: bundle contains hidden.json" >&2; exit 4; }
tar tzf "$BUNDLE" | grep -E '^(\./)?plan\.json$' > /dev/null || { echo "refusing: bundle has no plan.json" >&2; exit 4; }
fi

# ---- replacement plan ------------------------------------------------------------------------
REPL=0; SUFFIX=
if [ $REPLACE = 1 ]; then
  [ -s "$ORIG/runs.jsonl" ] || { echo "--replace needs $ORIG/runs.jsonl from the first pod" >&2; exit 2; }
  [ ! -e "$ORIG/replacement" ] || { echo "refusing: a replacement pod was already launched ($ORIG/replacement)" >&2; exit 2; }
  if [ $DRY = 0 ] && compgen -G "$ORIG/state/pkm-m8-panelA-*.json" > /dev/null; then   # best effort: make sure the lost pod is gone
    for f in "$ORIG"/state/pkm-m8-panelA-*.json; do RP_STATE="$ORIG/state" "$here/rp.sh" delete "$(basename "$f" .json)" >/dev/null 2>&1 || true; done; fi
  mkdir -p "$ORIG/replacement"; OUT=$ORIG/replacement; REPL=1; SUFFIX=-r
  tmpd=$(mktemp -d); tar xzf "$BUNDLE" -C "$tmpd"
  python3 - "$tmpd/plan.json" "$ORIG/runs.jsonl" <<'P'
import json,sys
pf,rf=sys.argv[1:]; p=json.load(open(pf)); done={json.loads(l)["order"] for l in open(rf) if l.strip()}
first=[r for r in p["rows"] if r.get("on_replacement")]
rest=[r for r in p["rows"] if not r.get("on_replacement") and r["order"] not in done]
for n,r in enumerate(first+rest,1): r["order"]=n; r.pop("on_replacement",None)
p["rows"]=first+rest; p["plan_id"]=p.get("plan_id","")+"-replacement"; json.dump(p,open(pf,"w"),indent=1)
print("replacement plan: %d on_replacement rows + %d unrecorded rows" % (len(first),len(rest)))
P
  tar czf "$OUT/bundle_replacement.tgz" -C "$tmpd" .; rm -rf "$tmpd"; BUNDLE=$OUT/bundle_replacement.tgz
fi
NAME=pkm-m8-panelA-$STAMP$SUFFIX
if [ -n "${LAUNCH_TEST_NAME:-}${LAUNCH_TEST_FLAVOR:-}${LAUNCH_TEST_VCPU:-}${LAUNCH_TEST_DISK:-}${LAUNCH_TEST_GRACE_S:-}" ]; then   # test-only override, smoke pods only
  case "${LAUNCH_TEST_NAME:-}" in pkm-m8-smoke-*) ;; *) echo "refusing: LAUNCH_TEST_* needs LAUNCH_TEST_NAME=pkm-m8-smoke-* (got '${LAUNCH_TEST_NAME:-}')" >&2; exit 4;; esac
  [ $REPLACE = 0 ] || { echo "refusing: LAUNCH_TEST_* with --replace" >&2; exit 4; }
  NAME=$LAUNCH_TEST_NAME; FLAVOR=${LAUNCH_TEST_FLAVOR:-$FLAVOR}; VCPU=${LAUNCH_TEST_VCPU:-$VCPU}; DISK=${LAUNCH_TEST_DISK:-$DISK}; GRACE_S=${LAUNCH_TEST_GRACE_S:-$GRACE_S}; TESTMODE=1
fi
if [ -n "$ATTACH" ]; then   # attach: the pod is the named one; its state file is found, not created
  NAME=$ATTACH; case "$NAME" in pkm-m8-*) ;; *) echo "refusing: pod name must start with pkm-m8-" >&2; exit 4;; esac
  if [ -n "${RP_STATE:-}" ] && [ -f "$RP_STATE/$NAME.json" ]; then SRC=$RP_STATE/$NAME.json
  elif [ -f "$OUT/state/$NAME.json" ]; then SRC=$OUT/state/$NAME.json
  elif [ -f "$RESUME_DIR/pod_state/$NAME.json" ]; then SRC=$RESUME_DIR/pod_state/$NAME.json
  elif [ -f "$here/state/$NAME.json" ]; then SRC=$here/state/$NAME.json
  else echo "no state file for $NAME (looked in \$RP_STATE, $OUT/state, $RESUME_DIR/pod_state, $here/state)" >&2; exit 2; fi
  mkdir -p "$OUT/state"; [ "$SRC" = "$OUT/state/$NAME.json" ] || { cp "$SRC" "$OUT/state/$NAME.json"; chmod 600 "$OUT/state/$NAME.json"; }
fi
export RP_STATE=$OUT/state
RP() { "$here/rp.sh" "$@"; }
CREATED=0; DELETED=0
[ -z "$ATTACH" ] || CREATED=1   # attach: the pod belongs to this run, the trap deletes it
cleanup() {
  rc=$?
  if [ $DRY = 0 ] && [ $CREATED = 1 ] && [ $DELETED = 0 ]; then
    say "cleanup: deleting pod $NAME"
    for k in 1 2 3; do RP delete "$NAME" >>"$OUT/launch.log" 2>&1 && { DELETED=1; break; }; sleep 5; done
    [ $DELETED = 1 ] && say "pod deleted" || say "WARNING: pod $NAME NOT deleted, delete it by hand"
  fi
  exit $rc
}
trap cleanup EXIT; trap 'say "interrupted"; exit 130' INT TERM

# ---- flavor gate: cpu5m 32 vCPU / 256 GB only --------------------------------------------------
if [ -n "$ATTACH" ]; then :
elif [ $TESTMODE = 1 ]; then say "TEST MODE: flavor gate bypassed for $NAME ($FLAVOR, $VCPU vCPU)"
else
if [ $DRY = 1 ]; then cp "${FLAVORS_JSON:-$here/flavors.json}" "$OUT/flavors.json"; else RP flavors "$OUT/flavors.json" > "$OUT/flavors.txt"; fi
python3 - "$OUT/flavors.json" "$FLAVOR" "$VCPU" <<'P' || { say "ABORT: cpu5m 32 vCPU / 256 GB not available; nothing created"; exit 3; }
import json,sys
f,fl,v=sys.argv[1],sys.argv[2],int(sys.argv[3]); rows={r["flavor"]:r for r in json.load(open(f))["flavors"]}
r=rows.get(fl); bad=None
if r is None: bad="flavor missing from catalogue"
elif r["max_vcpu"]<v or r["max_mem_gb"]<256: bad="max vcpu %s mem %s GB"%(r["max_vcpu"],r["max_mem_gb"])
elif str(r["stock"]).strip().lower() in ("none","","unavailable","out of stock","outofstock","0"): bad="stock %r"%r["stock"]
if bad: print("flavor check failed:",bad,file=sys.stderr); sys.exit(1)
print("flavor ok:",fl,"stock",r["stock"],"est $/h",r["est_usd_per_hour_at_max_vcpu"])
P
say "flavor gate passed (see $OUT/flavors.json)"
fi

DL_S=$(python3 -c "print(int(float('$DEADLINE_H')*3600))"); START=$(date +%s); DEADLINE=$((START + DL_S))
PRICE=null; POD_ID=dry-run
if [ -n "$ATTACH" ]; then   # created/deadline/price/pod id come from the first run's launch.json; missing fields are written back
  eval "$(python3 - "$OUT/launch.json" "$DH_GIVEN" "$DEADLINE_H" "$NAME" <<'P'
import json,sys
f,given,h,name=sys.argv[1:]; d=json.load(open(f)); c=d.get("created_epoch",d.get("start_epoch"))
if c is None: sys.exit("launch.json has neither created_epoch nor start_epoch")
dl=int(c+float(h)*3600) if given=="1" else int(d.get("deadline_epoch",c+float(h)*3600))
d.update(created_epoch=int(c),start_epoch=int(c),deadline_epoch=dl,pod_name=d.get("pod_name",name)); json.dump(d,open(f,"w"),indent=1)
print("START=%d DEADLINE=%d PRICE=%s POD_ID=%s FLAVOR=%s VCPU=%s"%(c,dl,d.get("usd_per_hr") if d.get("usd_per_hr") is not None else "null",d.get("pod_id","unknown"),d.get("flavor","?"),d.get("vcpu","?")))
P
)"
fi
get_remote() { # REMOTE LOCAL
  if [ $DRY = 1 ]; then cp "$OUT/fakepod/work/$1" "$2" 2>/dev/null; else RP get "$NAME" "$1" "$2" 2>/dev/null; fi; }
exec_remote() { # CMD (detached)
  if [ $DRY = 1 ]; then ( cd "$OUT/fakepod/work"; nohup bash -c "$1" > exec.log 2>&1 < /dev/null & ) ; else RP exec "$NAME" "$1"; fi; }
list_remote() { if [ $DRY = 1 ]; then (cd "$OUT/fakepod/work" && find . -type f -printf '%P %s\n' | python3 -c "import sys,json;print(json.dumps([[l.split()[0],int(l.split()[1])] for l in sys.stdin]))"); else RP ls "$NAME"; fi; }
sync_remote() { # runs.jsonl + new small per-row json files, so a lost pod loses at most the row in flight
  mkdir -p "$OUT/verify" "$OUT/rows_meta" "$OUT/env"
  get_remote run/out/runs.jsonl "$OUT/runs.jsonl.new" && [ -s "$OUT/runs.jsonl.new" ] && mv "$OUT/runs.jsonl.new" "$OUT/runs.jsonl" || rm -f "$OUT/runs.jsonl.new"
  [ -s "$OUT/env/env.txt" ] || { get_remote run/out/env/env.txt "$OUT/env/env.txt" || true; get_remote run/out/env/mem.json "$OUT/env/mem.json" || true; }
  list_remote 2>/dev/null | python3 -c "
import json,re,sys
for p,s in json.load(sys.stdin):
    m=re.fullmatch(r'run/out/rows/([^/]+)/(verify|log_table|watchdog)\.json',p)
    if m: print(p, m.group(1), m.group(2), s)" | while read -r p rid kind size; do
      if [ "$kind" = verify ]; then dst=$OUT/verify/$rid.json; else mkdir -p "$OUT/rows_meta/$rid"; dst=$OUT/rows_meta/$rid/$kind.json; fi
      [ -f "$dst" ] && [ "$(stat -c %s "$dst")" = "$size" ] || get_remote "$p" "$dst" || true
    done
}

utc() { date -u -d "@$1" +%FT%TZ; }
write_launch() { printf '{"pod_name":"%s","pod_id":"%s","usd_per_hr":%s,"vcpu":%s,"flavor":"%s","replacement":%s,"start_epoch":%s,"created_epoch":%s,"deadline_epoch":%s,"deadline_h":%s,"grace_s":%s,"dry_run":%s}\n' \
  "$NAME" "$POD_ID" "$PRICE" "$VCPU" "$FLAVOR" "$REPL" "$START" "$START" "$DEADLINE" "$DEADLINE_H" "$GRACE_S" "$DRY" > "$OUT/launch.json"; }
ATTACH_CMD="bash $here/launch.sh --attach $NAME --out $OUT"
resume_register() { # state file copy + one line under `## Pod in flight` in RESUME.md (pushed to S3 by the checkpoint script; .resume stays outside git)
  mkdir -p "$RESUME_DIR/pod_state"; cp "$RP_STATE/$NAME.json" "$RESUME_DIR/pod_state/$NAME.json"; chmod 600 "$RESUME_DIR/pod_state/$NAME.json"
  python3 - "$RESUME_DIR/RESUME.md" "- pod $NAME | id $POD_ID | flavor $FLAVOR ($VCPU vCPU) | \$/h $PRICE | created $(utc "$START") | deadline $(utc "$DEADLINE") | attach: \`$ATTACH_CMD --deadline-h $DEADLINE_H\` | out $OUT | state copy $RESUME_DIR/pod_state/$NAME.json" <<'P'
import sys,os
f,line=sys.argv[1:]; H="## Pod in flight"; t=open(f).read().split("\n") if os.path.exists(f) else []
if H in t:
    i=t.index(H)+1
    while i<len(t) and not t[i].startswith("## "): i+=1
    while i>0 and t[i-1]=="": i-=1
    t.insert(i,line)
else:
    while t and t[-1]=="": t.pop()
    t+=["",H,line]
open(f,"w").write("\n".join(t).rstrip("\n")+"\n")
P
}
resume_close() { [ -f "$RESUME_DIR/RESUME.md" ] && python3 - "$RESUME_DIR/RESUME.md" "- pod $NAME |" "- [ENDED $(date -u +%FT%TZ), pod deleted; results in $OUT] pod $NAME |" <<'P' || true
import sys
f,a,b=sys.argv[1:]; t=open(f).read().split("\n"); t=[b+l[len(a):] if l.startswith(a) else l for l in t]; open(f,"w").write("\n".join(t))
P
}
put3() { local k; for k in 1 2 3; do RP put "$NAME" "$@" && return 0; echo "put $2 failed (try $k/3), retrying" >&2; sleep 5; done; return 1; }   # one 404 from the proxy right after `wait` was seen in testing
pod_status() { RP info "$NAME" 2>/dev/null | awk '{print $3}' || true; }
job_state() { # attach only: is job.sh still running on the pod? prints up | down | unknown
  local id out k
  id=$(RP exec "$NAME" 'pgrep -f "[j]ob\.sh /work/bundle\.tgz" >/dev/null && echo up || echo down' 2>/dev/null | python3 -c "import json,sys;print(json.load(sys.stdin)['id'])" 2>/dev/null) || { echo unknown; return; }
  for k in 1 2 3 4 5 6 7 8 9 10; do sleep 1; out=$(RP log "$NAME" "$id" 2>/dev/null) && [ -n "$out" ] && { echo "$out"; return; }; done; echo unknown; }
mark_fetched() { # tell guard.sh the controller holds the data (hardening); never fatal
  if [ $DRY = 1 ]; then touch "$OUT/fakepod/work/fetched.flag"
  elif RP exec "$NAME" 'touch /work/fetched.flag' > /dev/null 2>&1; then say "fetched.flag set on pod"
  else say "WARNING: could not set fetched.flag (pod unreachable?)"; fi; }

# ---- create ----------------------------------------------------------------------------------------
if [ -n "$ATTACH" ]; then
  say "ATTACH to $NAME (pod $POD_ID, $PRICE USD/h, created $(utc "$START"), deadline $(utc "$DEADLINE")): $(RP info "$NAME" 2>&1 || true)"
elif [ $DRY = 0 ]; then
  say "create $NAME vcpu=$VCPU flavor=$FLAVOR disk=${DISK}GB"
  START=$(date +%s); DEADLINE=$((START + DL_S))
  POD_ID=$(RP create "$NAME" "$VCPU" "$FLAVOR" "$DISK"); CREATED=1
  INFO=$(RP info "$NAME"); say "POD_ID=$POD_ID  $INFO"
  PRICE=$(echo "$INFO" | sed -n 's/.*usd_per_hr \([0-9.]*\).*/\1/p'); PRICE=${PRICE:-null}
  say "HOURLY_PRICE_USD=$PRICE (record this in the protocol)"
  write_launch; resume_register; say "resume info written ($RESUME_DIR); attach with: $ATTACH_CMD --deadline-h $DEADLINE_H"
  RP wait "$NAME" | tee -a "$OUT/launch.log"
  put3 "$BUNDLE" bundle.tgz | tee -a "$OUT/launch.log"
  put3 "$here/job.sh" job.sh | tee -a "$OUT/launch.log"
  put3 "$here/guard.sh" guard.sh | tee -a "$OUT/launch.log"
else
  mkdir -p "$OUT/fakepod/work"; cp "$BUNDLE" "$OUT/fakepod/work/bundle.tgz"; cp "$here/job.sh" "$OUT/fakepod/work/job.sh"; cp "$here/guard.sh" "$OUT/fakepod/work/guard.sh"
  say "DRY RUN: local dir $OUT/fakepod/work is the pod; no RunPod call"; write_launch
fi

# ---- run ---------------------------------------------------------------------------------------------
ENVP="POD_ID=$POD_ID REPLACEMENT=$REPL"
if [ $DRY = 1 ]; then JOBCMD="$ENVP bash job.sh bundle.tgz run --dry-run"; else JOBCMD="$ENVP bash /work/job.sh /work/bundle.tgz /work/run"; fi
EXECID=
if [ -z "$ATTACH" ]; then
  EXECID=$(exec_remote "$JOBCMD"); say "job started (exec id: ${EXECID:-local})"
  # guard.sh: deadline STOP file, and compute stop at deadline+grace or once results are fetched (re-daemonises itself)
  if [ $DRY = 1 ]; then GCMD="GUARD_DRY=1 GUARD_POLL_S=1 bash guard.sh $DEADLINE $GRACE_S \"\$PWD\""
  else GCMD="${GUARD_POLL_S:+GUARD_POLL_S=$GUARD_POLL_S }bash /work/guard.sh $DEADLINE $GRACE_S /work"; fi
  gid=$(exec_remote "$GCMD"); gok=0
  for k in 1 2 3 4 5 6 7 8 9 10; do sleep 2; if get_remote guard.log "$OUT/guard.log" && grep started "$OUT/guard.log" > /dev/null; then gok=1; break; fi; done
  [ $gok = 1 ] && say "guard started: $(head -1 "$OUT/guard.log")" || say "WARNING: guard.sh not confirmed running on the pod (exec $gid); the pod is only bounded by this session"
fi

# ---- poll ---------------------------------------------------------------------------------------------
GOT=0; PARTIAL=0; DOWN=0; JDOWN=0
if [ -n "$ATTACH" ]; then case "$(pod_status)" in EXITED|STOPPED|TERMINATED) say "attach: pod is not running (guard fired, preempted or stopped): podserver is down, nothing can be fetched; will try once, then delete the pod"; DOWN=9;; esac; fi
while :; do
  [ $DOWN -ge 9 ] && { PARTIAL=1; break; }
  if get_remote run/out/done.json "$OUT/done.json"; then GOT=1; say "done.json present"; break; fi
  now=$(date +%s)
  if [ $now -ge $DEADLINE ]; then say "HARD DEADLINE reached: fetching what exists"; PARTIAL=1; break; fi
  if [ $DRY = 0 ]; then   # a pod that left RUNNING (guard fired, preempted) cannot be polled: two looks, then give up and delete it
    case "$(pod_status)" in RUNNING|""|None) DOWN=0;; *) DOWN=$((DOWN+1)); say "pod status not RUNNING ($DOWN/2)"; [ $DOWN -ge 2 ] && { PARTIAL=1; break; };; esac
  fi
  if [ $DRY = 0 ] && [ -n "$ATTACH" ]; then   # no exec id after a resume: look for job.sh on the pod instead (two misses in a row)
    case "$(job_state)" in down) JDOWN=$((JDOWN+1)); [ $JDOWN -ge 2 ] && { say "attach: job.sh no longer running and no done.json"; PARTIAL=1; break; };; *) JDOWN=0;; esac
  fi
  if [ $DRY = 0 ] && [ -z "$ATTACH" ]; then rcv=$(RP rc "$NAME" "${EXECID//[^0-9]/}" 2>/dev/null || echo running); case "$rcv" in running) ;; *) say "job exec ended (rc $rcv) without done.json"; PARTIAL=1; break;; esac; fi
  sync_remote || say "sync failed (will retry next poll)"
  get_remote run/out/progress.json "$OUT/progress.json" && say "progress: $(cat "$OUT/progress.json") ; synced $(wc -l < "$OUT/runs.jsonl" 2>/dev/null || echo 0) rows" || say "waiting (no row finished yet)"
  sleep $(( DEADLINE - now < POLL ? DEADLINE - now + 1 : POLL ))
done
[ $DOWN -ge 2 ] || sync_remote 2>/dev/null || true
if [ $GOT = 1 ] && grep '"aborted"' "$OUT/done.json" > /dev/null; then say "JOB ABORTED before any row: $(tr -d '\n ' < "$OUT/done.json")"; PARTIAL=1; fi

# ---- fetch ---------------------------------------------------------------------------------------------
if [ $GOT = 1 ] && get_remote run/out/results.tgz "$OUT/results.tgz"; then
  want=$(python3 -c "import json;print(json.load(open('$OUT/done.json'))['results_tgz_sha256'])"); have=$(sha256sum "$OUT/results.tgz" | cut -d' ' -f1)
  [ "$want" = "$have" ] && { say "results.tgz sha256 ok $have"; mark_fetched; } || { say "ERROR results.tgz sha256 mismatch (done.json $want, got $have)"; PARTIAL=1; }
else PARTIAL=1; fi
if [ $PARTIAL = 1 ]; then
  WR=/work; [ $DRY = 1 ] && WR="$OUT/fakepod/work"
  CMD="cd $WR/run 2>/dev/null && tar czf $WR/partial.tgz --exclude=out/.helper.py out 2>&1; echo partial-tar-rc=\$?"
  if [ $DRY = 1 ]; then (cd "$OUT/fakepod/work" && bash -c "$CMD") >/dev/null 2>&1 || true; else
    id=$(RP exec "$NAME" "$CMD" 2>/dev/null | python3 -c "import json,sys;print(json.load(sys.stdin)['id'])" 2>/dev/null) || id=
    if [ -n "$id" ]; then for k in $(seq 1 60); do [ "$(RP rc "$NAME" "$id")" != running ] && break; sleep 2; done; else say "pod unreachable: could not ask it for partial.tgz"; fi; fi
  get_remote partial.tgz "$OUT/partial.tgz" && { say "fetched partial.tgz ($(stat -c %s "$OUT/partial.tgz") bytes; job may still be running, rows after the cut are lost)"; mark_fetched; } || say "ERROR: nothing fetched"
fi
if [ $DRY = 0 ]; then
  RP ls "$NAME" > "$OUT/pod_ls.json" 2>/dev/null || true
  RP delete "$NAME" | tee -a "$OUT/launch.log"; DELETED=1; resume_close
  END=$(date +%s); python3 - "$OUT/launch.json" "$END" "$PRICE" <<'P'
import json,sys
f,end,price=sys.argv[1:]; d=json.load(open(f)); d["end_epoch"]=int(end); h=(int(end)-d["start_epoch"])/3600
d["pod_hours"]=round(h,4); d["cost_usd_est"]=round(h*float(price),3) if price!="null" else None; json.dump(d,open(f,"w"),indent=1); print("pod hours",d["pod_hours"],"est cost USD",d["cost_usd_est"])
P
fi
if [ $DRY = 1 ]; then sleep 2; pkill -f "$OUT/fakepod/work/guard.sh" 2>/dev/null || true; fi   # emulated guard (GUARD_DRY) never outlives a dry run
say "finished (results: $([ $PARTIAL = 0 ] && echo complete || echo PARTIAL))"
