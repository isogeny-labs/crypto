#!/bin/bash
# Frozen driver (S3): resumable. Reproduction at k=20, then 10 cells x lifts {0,1,2} at k=24,28,
# then prefix reruns for any arm with more targets than the cell minimum.
BIN=/home/user/frozen-bin/curve25519_family_scale
SHA=d66b85b0dcb60c2d1b551a2e3ce19de4cc76e6897ce359eafce1bf579606ca90
D="$(cd "$(dirname "$0")" && pwd)/analysis"
OUT=$D/f4cost_scale.jsonl
mkdir -p "$D"
run() { # name k form t g n seed lifts
  local name=$1 k=$2 form=$3 t=$4 g=$5 n=$6 seed=$7 L=$8
  if [ -s "$D/$name.json" ] && jq -e . "$D/$name.json" >/dev/null 2>&1; then return; fi
  if [ -s "$D/$name.failed" ]; then return; fi
  [ "$(sha256sum $BIN | cut -d' ' -f1)" = "$SHA" ] || { echo "BINARY HASH MISMATCH" >&2; exit 9; }
  local up; up=$(uptime | sed 's/^ *//')
  ( ulimit -v 3500000; timeout 1200 "$BIN" f4cost "$k" "$form" "$t" "$g" "$n" "$seed" 600 2 full "$L" - 0 \
      > "$D/$name.json.tmp" 2> "$D/$name.stderr" )
  local rc=$?
  if [ $rc -eq 0 ] && jq -e . "$D/$name.json.tmp" >/dev/null 2>&1; then
    mv "$D/$name.json.tmp" "$D/$name.json"
    jq -c --arg name "$name" --arg up "$up" '. + {run:$name, uptime:$up, rc:0}' "$D/$name.json" >> "$OUT"
    echo "$name ok $(jq -r '.targets_done' $D/$name.json)/$n" | tee -a "$D/status.txt"
  else
    rm -f "$D/$name.json.tmp"
    echo "rc=$rc" > "$D/$name.failed"
    jq -cn --arg name "$name" --arg up "$up" --argjson rc "$rc" --arg form "$form" --argjson k "$k" --argjson t "$t" --argjson g "$g" --argjson L "$L" \
      '{run:$name,uptime:$up,rc:$rc,failed:true,k:$k,form:$form,t:$t,guess:$g,lifts_mode:$L}' >> "$OUT"
    echo "$name failed rc=$rc" | tee -a "$D/status.txt"
  fi
}
# reproduction (lead D's cell and seed)
for L in 0 1 2; do run "k20_isogeny_t4_g0_lifts$L" 20 isogeny 4 0 200 2026111204 $L; done
for k in 24 28; do
  for c in isogeny:4:0 isogeny:5:0 isogeny:6:0 bits:5:3 bits:6:3; do
    IFS=: read form t g <<<"$c"
    b=0; [ "$form" = bits ] && b=50
    seed=$((2026110500000 + 100*k + b + t))
    cell="k${k}_${form}_t${t}_g${g}"
    for L in 0 1 2; do run "${cell}_lifts$L" $k $form $t $g 200 $seed $L; done
    # prefix reruns
    nmin=200
    for L in 0 1 2; do
      v=$(jq -r '.targets_done // empty' "$D/${cell}_lifts$L.json" 2>/dev/null)
      [ -n "$v" ] && [ "$v" -lt "$nmin" ] && nmin=$v
    done
    for L in 0 1 2; do
      v=$(jq -r '.targets_done // empty' "$D/${cell}_lifts$L.json" 2>/dev/null)
      if [ -n "$v" ] && [ "$v" -gt "$nmin" ]; then run "${cell}_lifts${L}_prefix$nmin" $k $form $t $g $nmin $seed $L; fi
    done
  done
done
echo ALLDONE | tee -a "$D/status.txt"
