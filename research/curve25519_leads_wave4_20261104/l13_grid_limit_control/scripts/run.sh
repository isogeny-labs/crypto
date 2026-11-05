#!/usr/bin/env bash
# L13 (wave 4): the grid-limit control. Executes exactly the runs planned in PROTOCOL.md into
# ../data/. Re-launchable after a container restart: the chunked driver skips complete chunks and
# re-runs interrupted ones from scratch; analysis and ALL_DONE are written only when all 16 chunks
# are complete. One heavy process at a time (one ecbench session, serialised by
# /tmp/crypto-bench.lock; T11's session may hold it, we wait). Counted units only; no git; no
# rebuilds; writes only inside this lead directory.
# Launch in the background and wait on it:  nohup scripts/run.sh > data/run.out 2>&1 &
set -uo pipefail

LEAD="/home/user/crypto/research/curve25519_leads_wave4_20261104/l13_grid_limit_control"
ECB="/home/user/ctarget/release/ecbench"
FROZEN_ECB_SHA="54737834d048b22e77399160fa9e159782b1124ea8190d84dba185309229641b"   # = T12 / L12 instrument
SPEC="$LEAD/ecbench/spec.json"
DATA="$LEAD/data"; EB="$DATA/ecbench"
N_CHUNKS=16
F4=pkm-iso-t5-g4-s1307; GRID=grid-iso-t5-s1307; MITM=mitm-iso-t5-s1307

mkdir -p "$EB"
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$DATA/run.log"; }

# F1: instrument identity. Refuse to run on a different binary.
gote=$(sha256sum "$ECB" | cut -d' ' -f1)
[ "$gote" = "$FROZEN_ECB_SHA" ] || { log "F1: ecbench sha256 $gote != frozen; not running"; exit 3; }
[ -e "$DATA/ALL_DONE" ] && { log "ALL_DONE present; nothing to do"; exit 0; }
{
  echo "launch_utc $(date -u +%FT%TZ)"; echo "host $(hostname)"; echo "nproc $(nproc)"; echo "uname $(uname -srm)"
  echo "loadavg $(cat /proc/loadavg)"; echo "ecbench $ECB $gote"; echo "spec_sha256 $(sha256sum "$SPEC" | cut -d' ' -f1)"
} >> "$DATA/env.txt"
echo "$gote  $ECB" > "$DATA/ecbench.sha256"
[ -f "$EB/spec.frozen.json" ] || cp "$SPEC" "$EB/spec.frozen.json"
cmp -s "$SPEC" "$EB/spec.frozen.json" || { log "F1: ecbench/spec.json differs from the frozen copy in data/; not running"; exit 3; }

finish() {
  ( cd "$DATA" && find . -type f ! -name SHA256SUMS -print0 | sort -z | xargs -0 sha256sum > SHA256SUMS )
  echo "end_utc $(date -u +%FT%TZ)" >> "$DATA/env.txt"; touch "$DATA/ALL_DONE"; log "L13: ALL_DONE ($1)"
}

"$ECB" plan --spec "$SPEC" > "$EB/plan.txt" 2>&1 || { log "ecbench plan rejected the spec: $(tail -1 "$EB/plan.txt")"; echo rejected_at_plan > "$EB/GRID_REJECTED"; finish "ecbench rejected the spec at plan"; exit 0; }
uptime > "$EB/load_start.txt"
# One complete one-target session per chunk (all 5 arms, 2 rounds, alternate_measured); chunk c at
# target_seed 2026110431+c, measurement seed 2026110432+c; waits for /tmp/crypto-bench.lock
# (--wait --allow-busy); skips complete chunks on relaunch.
python3 -I "$LEAD/scripts/chunked_run.py" "$SPEC" "$EB/chunks" "$N_CHUNKS" "$ECB" >> "$EB/driver.out" 2>&1
uptime > "$EB/load_end.txt"

# Rejection of engine=exhaustive (or of mitm on this base) shows as an error outcome in the first chunk.
if [ -f "$EB/chunks/c00/records.jsonl" ] && python3 -I - "$EB/chunks/c00/records.jsonl" "$GRID" "$MITM" <<'PY'
import json, sys
bad = [json.loads(l) for l in open(sys.argv[1])]
bad = [x for x in bad if x['arm'] in sys.argv[2:] and x['outcome']['status'] != 'verified' and x['outcome'].get('error')]
sys.exit(0 if bad else 1)
PY
then log "ecbench reported an error outcome on the grid or mitm arm in c00 (see records); recording GRID_REJECTED"; echo error_in_c00 > "$EB/GRID_REJECTED"; fi

CH=$(ls -d "$EB"/chunks/c[0-9][0-9] 2>/dev/null | sort)
NC=$(python3 -I - $CH <<'PY'
import json, sys, os
print(sum(1 for d in sys.argv[1:] if os.path.exists(os.path.join(d, 'session.json')) and json.load(open(os.path.join(d, 'session.json'))).get('status') == 'complete'))
PY
)
log "complete chunks: $NC / $N_CHUNKS"
if [ "$NC" != "$N_CHUNKS" ]; then log "not all chunks complete; relaunch run.sh to continue (no ALL_DONE)"; exit 0; fi

# ---------- Analysis on the 16 pooled chunks (frozen in PROTOCOL.md section 3.3) ----------
mkdir -p "$EB/verify"
for d in $CH; do c=$(basename "$d")
  "$ECB" verify --dir "$d" --replay 1 --out "$EB/verify/$c.json" > "$EB/verify/$c.txt" 2>&1; echo "$c rc=$?" >> "$EB/verify/rc.txt"
done
"$ECB" band --dir $CH --ranks --json "$EB/band_pooled.json" > "$EB/band_pooled.txt" 2>&1
: > "$EB/pooled_compare.jsonl"
# (a, b) pairs, ratio = S(b)/S(a): P2 grid/F4; P4 mitm/grid; P5 grid/proj-a; P6 mitm/proj-a; P7 grid, mitm, F4 / rho-neg; F4/proj-a and mitm/F4 for the report.
for pair in "$F4 $GRID" "$GRID $MITM" "proj-a $GRID" "proj-a $MITM" "rho-neg $GRID" "rho-neg $MITM" "rho-neg $F4" "proj-a $F4" "$F4 $MITM"; do
  set -- $pair; a=$1; b=$2
  for w in 0 0.1 1; do
    python3 -I "$LEAD/scripts/pooled_compare.py" "$a" "$b" "$w" $CH >> "$EB/pooled_compare.jsonl" 2>> "$EB/pooled_compare.err" \
      || echo "{\"a\":\"$a\",\"b\":\"$b\",\"weight\":$w,\"error\":\"see pooled_compare.err\"}" >> "$EB/pooled_compare.jsonl"
  done
done
python3 -I "$LEAD/scripts/identity_check.py" $CH > "$EB/identity_check.json" 2> "$EB/identity_check.err" || log "identity_check failed (see identity_check.err)"
finish "16 chunks, verify, band, pooled compare, identity check"
