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
# --dry-run: no RunPod call and no RUNPOD_API_KEY use; a local directory is the "pod"; job.sh runs locally with --dry-run.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
POLL=600; DEADLINE_H=72; BUNDLE=; OUT=; DRY=0; REPLACE=0; STAMP=$(date -u +%Y%m%dT%H%M%SZ)
VCPU=32; FLAVOR=cpu5m; DISK=60
while [ $# -gt 0 ]; do case "$1" in
  --bundle) BUNDLE=$2; shift 2;; --out) OUT=$2; shift 2;; --deadline-h) DEADLINE_H=$2; shift 2;; --poll-s) POLL=$2; shift 2;;
  --stamp) STAMP=$2; shift 2;; --dry-run) DRY=1; shift;; --replace) REPLACE=1; shift;; *) echo "unknown arg $1" >&2; exit 2;; esac; done
[ -f "${BUNDLE:-}" ] && [ -n "$OUT" ] || { sed -n 2,16p "$0" >&2; exit 2; }
mkdir -p "$OUT"; OUT=$(cd "$OUT" && pwd); BUNDLE=$(readlink -f "$BUNDLE"); ORIG=$OUT
say() { echo "$(date -u +%FT%TZ) $*" | tee -a "$OUT/launch.log"; }

# ---- bundle checks (before anything is created) -----------------------------------------
tar tzf "$BUNDLE" | awk -F/ 'tolower($NF)=="hidden.json"{f=1} END{exit f}' || { echo "refusing: bundle contains hidden.json" >&2; exit 4; }
tar tzf "$BUNDLE" | grep -E '^(\./)?plan\.json$' > /dev/null || { echo "refusing: bundle has no plan.json" >&2; exit 4; }

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
export RP_STATE=$OUT/state
RP() { "$here/rp.sh" "$@"; }
CREATED=0; DELETED=0
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

START=$(date +%s); DEADLINE=$((START + $(python3 -c "print(int(float('$DEADLINE_H')*3600))")))
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

# ---- create ----------------------------------------------------------------------------------------
PRICE=null; POD_ID=dry-run
if [ $DRY = 0 ]; then
  say "create $NAME vcpu=$VCPU flavor=$FLAVOR disk=${DISK}GB"
  POD_ID=$(RP create "$NAME" "$VCPU" "$FLAVOR" "$DISK"); CREATED=1
  INFO=$(RP info "$NAME"); say "POD_ID=$POD_ID  $INFO"
  PRICE=$(echo "$INFO" | sed -n 's/.*usd_per_hr \([0-9.]*\).*/\1/p'); PRICE=${PRICE:-null}
  say "HOURLY_PRICE_USD=$PRICE (record this in the protocol)"
  RP wait "$NAME" | tee -a "$OUT/launch.log"
  RP put "$NAME" "$BUNDLE" bundle.tgz | tee -a "$OUT/launch.log"
  RP put "$NAME" "$here/job.sh" job.sh | tee -a "$OUT/launch.log"
else
  mkdir -p "$OUT/fakepod/work"; cp "$BUNDLE" "$OUT/fakepod/work/bundle.tgz"; cp "$here/job.sh" "$OUT/fakepod/work/job.sh"
  say "DRY RUN: local dir $OUT/fakepod/work is the pod; no RunPod call"
fi
printf '{"pod_name":"%s","pod_id":"%s","usd_per_hr":%s,"vcpu":%s,"flavor":"%s","replacement":%s,"start_epoch":%s,"deadline_epoch":%s,"dry_run":%s}\n' \
  "$NAME" "$POD_ID" "$PRICE" "$VCPU" "$FLAVOR" "$REPL" "$START" "$DEADLINE" "$DRY" > "$OUT/launch.json"

# ---- run ---------------------------------------------------------------------------------------------
ENVP="POD_ID=$POD_ID REPLACEMENT=$REPL"
if [ $DRY = 1 ]; then JOBCMD="$ENVP bash job.sh bundle.tgz run --dry-run"; else JOBCMD="$ENVP bash /work/job.sh /work/bundle.tgz /work/run"; fi
EXECID=$(exec_remote "$JOBCMD"); say "job started (exec id: ${EXECID:-local})"

# ---- poll ---------------------------------------------------------------------------------------------
GOT=0; PARTIAL=0
while :; do
  if get_remote run/out/done.json "$OUT/done.json"; then GOT=1; say "done.json present"; break; fi
  now=$(date +%s)
  if [ $now -ge $DEADLINE ]; then say "HARD DEADLINE reached: fetching what exists"; PARTIAL=1; break; fi
  if [ $DRY = 0 ]; then rcv=$(RP rc "$NAME" "${EXECID//[^0-9]/}" 2>/dev/null || echo running); case "$rcv" in running) ;; *) say "job exec ended (rc $rcv) without done.json"; PARTIAL=1; break;; esac; fi
  sync_remote || say "sync failed (will retry next poll)"
  get_remote run/out/progress.json "$OUT/progress.json" && say "progress: $(cat "$OUT/progress.json") ; synced $(wc -l < "$OUT/runs.jsonl" 2>/dev/null || echo 0) rows" || say "waiting (no row finished yet)"
  sleep $(( DEADLINE - now < POLL ? DEADLINE - now + 1 : POLL ))
done
sync_remote || true

# ---- fetch ---------------------------------------------------------------------------------------------
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
  RP delete "$NAME" | tee -a "$OUT/launch.log"; DELETED=1
  END=$(date +%s); python3 - "$OUT/launch.json" "$END" "$PRICE" <<'P'
import json,sys
f,end,price=sys.argv[1:]; d=json.load(open(f)); d["end_epoch"]=int(end); h=(int(end)-d["start_epoch"])/3600
d["pod_hours"]=round(h,4); d["cost_usd_est"]=round(h*float(price),3) if price!="null" else None; json.dump(d,open(f,"w"),indent=1); print("pod hours",d["pod_hours"],"est cost USD",d["cost_usd_est"])
P
fi
say "finished (results: $([ $PARTIAL = 0 ] && echo complete || echo PARTIAL))"
