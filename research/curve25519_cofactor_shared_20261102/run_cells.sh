#!/bin/bash
# Frozen protocol runner (lead D): 5 cells x {lifts=0,1,2} interleaved by cell,
# + trace=1 arm for lifts=2 on isogeny t=4,5, + prefix-matched re-runs of lifts 0/1
# for every capped lifts=2 cell.  Sequential, one process at a time.
BIN=/home/user/frozen-bin/curve25519_family_leadD
D="$(cd "$(dirname "$0")" && pwd)/analysis"
OUT=$D/f4cost_shared.jsonl
mkdir -p "$D"
CELLS="isogeny:4:0:2026111204 isogeny:5:0:2026111205 isogeny:6:0:2026111206 bits:5:3:2026111205 bits:6:3:2026111206"
run() { # name form t g n seed lifts trace
  local name=$1 form=$2 t=$3 g=$4 n=$5 seed=$6 L=$7 tr=$8
  local up; up=$(uptime | sed 's/^ *//')
  ( ulimit -v 3500000; timeout 1500 "$BIN" f4cost 20 "$form" "$t" "$g" "$n" "$seed" 600 2 full "$L" - "$tr" \
      > "$D/$name.json" 2> "$D/$name.stderr" )
  local rc=$?
  if [ $rc -eq 0 ] && jq -c --arg name "$name" --arg up "$up" '. + {run:$name, uptime:$up, rc:0}' "$D/$name.json" >> "$OUT" 2>/dev/null; then
    echo "$name ok" | tee -a "$D/status.txt"
  else
    jq -cn --arg name "$name" --arg up "$up" --argjson rc "$rc" --arg form "$form" --argjson t "$t" --argjson g "$g" --argjson L "$L" --argjson tr "$tr" \
      '{run:$name,uptime:$up,rc:$rc,failed:true,form:$form,t:$t,guess:$g,lifts_mode:$L,trace:($tr==1)}' >> "$OUT"
    echo "$name failed rc=$rc" | tee -a "$D/status.txt"
  fi
}
for c in $CELLS; do
  IFS=: read form t g seed <<<"$c"
  for L in 0 1 2; do
    run "${form}_t${t}_g${g}_lifts${L}" "$form" "$t" "$g" 200 "$seed" "$L" 0
  done
  if [ "$form" = isogeny ] && [ "$t" -le 5 ]; then
    run "${form}_t${t}_g${g}_lifts2_trace1" "$form" "$t" "$g" 200 "$seed" 2 1
  fi
  # prefix-matched re-runs when lifts=2 did not finish the 200 targets
  n2=$(jq -r '.targets_done // empty' "$D/${form}_t${t}_g${g}_lifts2.json" 2>/dev/null)
  if [ -n "$n2" ] && [ "$n2" -lt 200 ]; then
    for L in 0 1; do
      run "${form}_t${t}_g${g}_lifts${L}_prefix${n2}" "$form" "$t" "$g" "$n2" "$seed" "$L" 0
    done
  fi
done
