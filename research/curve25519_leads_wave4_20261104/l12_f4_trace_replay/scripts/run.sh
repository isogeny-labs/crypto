#!/usr/bin/env bash
# L12 (wave 4): F4 trace replay on the T10/T12 winner. Executes exactly the
# runs planned in PROTOCOL.md into ../data/. Re-launchable after a container
# restart: finished engine cells are skipped, the chunked ecbench driver skips
# complete chunks. One heavy process at a time; counted units only; no git,
# no rebuilds; writes only inside this lead directory.
# Launch in the background and wait on it:  nohup scripts/run.sh > data/run.out 2>&1 &
set -uo pipefail

LEAD="/home/user/crypto/research/curve25519_leads_wave4_20261104/l12_f4_trace_replay"
FAM="/home/user/ctarget/release/curve25519_family"
ECB="/home/user/ctarget/release/ecbench"
FROZEN_FAM_SHA="ba1e3c0518eae58ae39a9b08e1f757c7e9fe1eb02c86b158b883244bc6a5ecef"   # = T10/T12 instrument
FROZEN_ECB_SHA="54737834d048b22e77399160fa9e159782b1124ea8190d84dba185309229641b"   # = T12 instrument
CENSUS="/home/user/crypto/research/curve25519_isogeny_seed_20261103/data/k20.jsonl"
FROZEN_CENSUS_SHA="bc74d8ac5151713c59f6c6d5bf58df1c0bf5e3442c6b50c7d234163c4c807adc"
DATA="$LEAD/data"; ENG="$DATA/engine"; EB="$DATA/ecbench"
SPEC="$LEAD/ecbench/spec.json"
K=20; TARGETS=100; CAP_S=3600; M=2; MODE=full; LIFTS=0; MAX_NNZ="-"
export F4COST_PLANTED=16     # 16 planted (decomposable) targets per cell; reported apart from mpt
N_CHUNKS=8

mkdir -p "$ENG" "$EB"
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$DATA/run.log"; }

# F1: instrument identity. Refuse to run on a different binary or census.
got=$(sha256sum "$FAM" | cut -d' ' -f1)
[ "$got" = "$FROZEN_FAM_SHA" ] || { log "F1: curve25519_family sha256 $got != frozen; not running"; exit 3; }
gote=$(sha256sum "$ECB" | cut -d' ' -f1)
[ "$gote" = "$FROZEN_ECB_SHA" ] || { log "F1: ecbench sha256 $gote != frozen; not running"; exit 3; }
gotc=$(sha256sum "$CENSUS" | cut -d' ' -f1)
[ "$gotc" = "$FROZEN_CENSUS_SHA" ] || { log "F1: census sha256 $gotc != frozen; not running"; exit 3; }
[ -e "$DATA/ALL_DONE" ] && { log "ALL_DONE present; nothing to do"; exit 0; }
{
  echo "launch_utc $(date -u +%FT%TZ)"; echo "host $(hostname)"; echo "nproc $(nproc)"; echo "uname $(uname -srm)"
  echo "loadavg $(cat /proc/loadavg)"; echo "family $FAM $got"; echo "ecbench $ECB $gote"; echo "census $CENSUS $gotc"
  echo "F4COST_PLANTED $F4COST_PLANTED"
} >> "$DATA/env.txt"
echo "$got  $FAM" > "$DATA/family.sha256"; echo "$gote  $ECB" > "$DATA/ecbench.sha256"

# ---------- Stage 1: engine cells (f4cost), strictly sequential ----------
run_cell() {  # name t G seed trace
  local name=$1 t=$2 g=$3 s=$4 tr=$5 form tseed rc try out
  out="$ENG/${name}_tr${tr}"
  if [ -f "$out.status" ] && grep -q ' rc=0 ' "$out.status"; then log "skip $name tr=$tr (done)"; return 0; fi
  if [ "$s" = 0 ]; then form="isogeny"; else form="isogeny@$s"; fi
  tseed=$((2026111500 + t))          # T10 convention: 2026111505 at t=5 (the winner cell), 2026111506 at t=6
  for try in 1 2; do
    [ "$(sha256sum "$FAM" | cut -d' ' -f1)" = "$FROZEN_FAM_SHA" ] || { log "F1: binary changed mid-run"; exit 3; }
    local start load; start=$(date -u +%FT%TZ); load=$(cut -d' ' -f1 /proc/loadavg)
    "$FAM" f4cost "$K" "$form" "$t" "$g" "$TARGETS" "$tseed" "$CAP_S" "$M" "$MODE" "$LIFTS" "$MAX_NNZ" "$tr" \
      > "$out.json" 2> "$out.stderr"; rc=$?
    echo "cell=$name trace=$tr form=$form t=$t G=$g targets=$TARGETS target_seed=$tseed planted=$F4COST_PLANTED try=$try rc=$rc start=$start end=$(date -u +%FT%TZ) load1=$load" \
      | tee -a "$DATA/run.log" > "$out.status"
    [ "$rc" = 0 ] && return 0
    mv "$out.json" "$out.try$try.json"; mv "$out.stderr" "$out.try$try.stderr"   # F2: keep the failed attempt, one identical rerun
  done
  log "cell=$name trace=$tr FAILED twice (F2)"; echo "FAILED" >> "$out.status"
}
# Order: winner pair first (plain, then trace), then the other three pairs.
run_cell t5_g4_s1307 5 4 1307 0;  run_cell t5_g4_s1307 5 4 1307 1
run_cell t5_g3_s1307 5 3 1307 0;  run_cell t5_g3_s1307 5 3 1307 1
run_cell t6_g4_s1308 6 4 1308 0;  run_cell t6_g4_s1308 6 4 1308 1
run_cell t6_g4_s0    6 4 0    0;  run_cell t6_g4_s0    6 4 0    1

python3 -I "$LEAD/scripts/analyze_engine.py" "$LEAD" > "$ENG/analyze.out" 2>&1 || log "analyze_engine failed (see $ENG/analyze.out)"
GATE=$(cat "$ENG/GATE" 2>/dev/null || echo closed)
log "engine stage done; GATE=$GATE"

# ---------- Stage 2 (only if the engine gate is open): chunked ecbench ----------
finish() {
  ( cd "$DATA" && find . -type f ! -name SHA256SUMS -print0 | sort -z | xargs -0 sha256sum > SHA256SUMS )
  echo "end_utc $(date -u +%FT%TZ)" >> "$DATA/env.txt"; touch "$DATA/ALL_DONE"; log "L12: ALL_DONE ($1)"
}
if [ "$GATE" != "open" ]; then finish "gate closed: no ecbench"; exit 0; fi

cp "$SPEC" "$EB/spec.frozen.json"
"$ECB" plan --spec "$SPEC" > "$EB/plan.txt" 2>&1 || { log "ecbench plan rejected the spec: $(tail -1 "$EB/plan.txt")"; echo rejected_at_plan > "$EB/TRACE_REJECTED"; finish "ecbench rejected trace=1 at plan"; exit 0; }
uptime > "$EB/load_start.txt"
# One complete one-target session per chunk; waits for /tmp/crypto-bench.lock (--wait --allow-busy); skips complete chunks on relaunch.
python3 -I "$LEAD/scripts/chunked_run.py" "$SPEC" "$EB/chunks" "$N_CHUNKS" "$ECB" >> "$EB/driver.out" 2>&1
uptime > "$EB/load_end.txt"
# Rejection of trace=1 by the oracle shows as an error outcome of the trace arm in the first chunk.
if [ -f "$EB/chunks/c00/records.jsonl" ] && python3 -I - "$EB/chunks/c00/records.jsonl" <<'PY'
import json, sys
for l in open(sys.argv[1]):
    x = json.loads(l)
    if x['arm'] == 'pkm-iso-t5-g4-s1307-trace' and x['outcome']['status'] != 'verified' and 'trace' in str(x['outcome'].get('error', '')):
        sys.exit(0)
sys.exit(1)
PY
then log "ecbench rejected trace=1 for pkm-semaev (trace arm error in c00)"; echo rejected_at_run > "$EB/TRACE_REJECTED"; finish "ecbench rejected trace=1 at run"; exit 0; fi

CH=$(ls -d "$EB"/chunks/c[0-9][0-9] 2>/dev/null | sort)
NC=$(python3 -I - $CH <<'PY'
import json, sys, os
print(sum(1 for d in sys.argv[1:] if os.path.exists(os.path.join(d, 'session.json')) and json.load(open(os.path.join(d, 'session.json'))).get('status') == 'complete'))
PY
)
log "complete chunks: $NC / $N_CHUNKS"
if [ "$NC" != "$N_CHUNKS" ]; then log "not all chunks complete; relaunch run.sh to continue (no ALL_DONE)"; exit 0; fi

mkdir -p "$EB/verify"
for d in $CH; do c=$(basename "$d")
  "$ECB" verify --dir "$d" --replay 1 --out "$EB/verify/$c.json" > "$EB/verify/$c.txt" 2>&1; echo "$c rc=$?" >> "$EB/verify/rc.txt"
done
"$ECB" band --dir $CH --ranks --json "$EB/band_pooled.json" > "$EB/band_pooled.txt" 2>&1
: > "$EB/pooled_compare.jsonl"
for a in pkm-iso-t5-g4-s1307 proj-a rho-neg; do for w in 0 0.1 1; do
  python3 -I "$LEAD/scripts/pooled_compare.py" "$a" pkm-iso-t5-g4-s1307-trace "$w" $CH >> "$EB/pooled_compare.jsonl" 2>> "$EB/pooled_compare.err" || echo "{\"a\":\"$a\",\"b\":\"pkm-iso-t5-g4-s1307-trace\",\"weight\":$w,\"error\":\"see pooled_compare.err\"}" >> "$EB/pooled_compare.jsonl"
done; done
python3 -I "$LEAD/scripts/learn_charge.py" $CH > "$EB/learn_charge.json" 2> "$EB/learn_charge.err" || log "learn_charge failed"
finish "engine + ecbench"
