#!/bin/bash
# usage: run_cell.sh <p> <curve_seed> <target_seed> <arm> [cap_seconds]
# Generates the instance, runs msolve under a polling watchdog (time and RSS cap),
# appends one CSV row to results/cells.csv. Wall time is used only as the kill cap.
set -u
# Environment overrides (added 2026-11-01 for the Linux rerun; defaults are the original macOS paths):
#   L2B_GEN MSOLVE MSOLVE_EXTRA RSSWATCH L2B_RESULTS L2B_WORK L2B_RSSCAP_KB
D=$(cd "$(dirname "$0")" && pwd)
P=$1; CS=$2; TS=$3; ARM=$4; CAP=${5:-1800}
RSSCAP_KB=$((16*1024*1024))
G=${L2B_GEN:-/Volumes/SSD990-2/tmp-cargo/l2b/release/l2b-gen}
MSOLVE=${MSOLVE:-/opt/homebrew/bin/msolve}
RSSWATCH=${RSSWATCH:-/Volumes/SSD990-2/tmp-cargo/l2b/release/rsswatch}
RES=${L2B_RESULTS:-$D/results}
WORK=${L2B_WORK:-$D/work}
RSSCAP_KB=${L2B_RSSCAP_KB:-$RSSCAP_KB}
W=$WORK/p${P}_c${CS}_t${TS}_${ARM}
mkdir -p "$RES" "$WORK/logs"
META=$($G $P $CS $TS $ARM $W.ms)
N=$(echo "$META" | sed 's/.*"N":\([0-9]*\).*/\1/')
"$MSOLVE" ${MSOLVE_EXTRA:-} -f $W.ms -g 2 -v 2 -t 1 -o $W.out > $W.log 2>&1 &
PID=$!
T0=$(date +%s); STATUS=ok
while kill -0 $PID 2>/dev/null; do
  sleep 2
  EL=$(( $(date +%s) - T0 ))
  RSS=$("$RSSWATCH" $PID 2>/dev/null)
  if [ "$EL" -gt "$CAP" ]; then STATUS=timeout_${CAP}s; kill -9 $PID; break; fi
  if [ -n "$RSS" ] && [ "$RSS" -gt "$RSSCAP_KB" ]; then STATUS=rss_cap_$((RSSCAP_KB/1024/1024))GiB; kill -9 $PID; break; fi
done
wait $PID 2>/dev/null; RC=$?
if [ "$STATUS" = ok ] && [ $RC -ne 0 ]; then STATUS=msolve_rc_$RC; fi
LAST=$(grep -c . $W.log)
awk -v p=$P -v cs=$CS -v ts=$TS -v arm=$ARM -v n=$N -v st=$STATUS '
 /^ *[0-9]+ +[0-9]+ +[0-9]+ +[0-9]+ x [0-9]+ / {
   d=$1; r=$4; c=$6; rounds++; if(d>maxd)maxd=d; if(r>maxr)maxr=r; if(c>maxc)maxc=c;
   if (r*c>maxrc) maxrc=r*c; sr+=r; ld=d; lr=r; lc=c;
   z=0; for(i=1;i<=NF;i++) if($i=="zero") z=$(i-1);
   if(z>0 && (zd==""||d<zd)) zd=d
 }
 /^size of basis/ {b=$4}
 /^size of basis/ {}
 END { printf "%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s\n", p,cs,ts,arm,n,st,maxd+0,ld+0,rounds+0,maxr+0,maxc+0,lr+0,lc+0,sr+0,(zd==""?"NA":zd) "," (b==""?"NA":b) }
' $W.log > $W.row
# size of basis line has a different shape; parse separately
B=$(grep -E "^size of basis" $W.log | awk '{print $4}')
awk -v b="${B:-NA}" 'BEGIN{FS=OFS=","} {$NF=b; print}' $W.row >> $RES/cells.csv
gzip -f $W.log
echo "$META,\"status\":\"$STATUS\"}" >> $RES/instances.jsonl
