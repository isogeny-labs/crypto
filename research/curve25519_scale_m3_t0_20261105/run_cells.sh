#!/usr/bin/env bash
# Frozen cells of PROTOCOL.md. One process at a time, resumable: a cell whose
# cells/<name>.json or cells/<name>.fail.json exists is skipped.
# usage: run_cells.sh <stage>   stages: repro p2k24 p1k20 p1k24 p2k28 p1k28
set -u
D=$(cd "$(dirname "$0")" && pwd)
BIN=/home/user/frozen-bin/curve25519_family_scale
WANT=d66b85b0dcb60c2d1b551a2e3ce19de4cc76e6897ce359eafce1bf579606ca90
[ "$(sha256sum $BIN | cut -d' ' -f1)" = "$WANT" ] || { echo "binary hash mismatch" >&2; exit 3; }
mkdir -p "$D/cells"
ulimit -v 3500000
# cell <name> <cap> <planted> <k> <form> <t> <G> <targets> <seed> <m> <mode> <maxnnz>
cell() {
  local n=$1 cap=$2 pl=$3; shift 3
  local c="$D/cells/$n"
  [ -s "$c.json" ] || [ -e "$c.fail.json" ] && return 0
  local k=$1 form=$2 t=$3 g=$4 nt=$5 seed=$6 m=$7 mode=$8 nnz=$9
  { echo "start $(date -u +%FT%TZ)"; uptime; } > "$c.status"
  F4COST_PLANTED=$pl timeout $((cap+100)) "$BIN" f4cost $k $form $t $g $nt $seed $cap $m $mode 0 $nnz 0 > "$c.out" 2> "$c.stderr"
  local rc=$?
  { echo "end $(date -u +%FT%TZ) rc=$rc"; uptime; } >> "$c.status"
  if [ $rc -eq 0 ] && jq -e . "$c.out" >/dev/null 2>&1; then mv "$c.out" "$c.json"; rm -f "$c.out"
  else
    local why=other; [ $rc -eq 124 ] && why=timeout
    grep -q "memory allocation" "$c.stderr" && why=memory
    [ $rc -eq 134 ] || [ $rc -eq 137 ] || [ $rc -eq 139 ] && [ $why = other ] && why=signal
    jq -nc --arg n $n --argjson k $k --arg f $form --argjson t $t --argjson g $g --argjson m $m --arg mode $mode \
      --argjson rc $rc --arg why $why --arg err "$(tail -c 300 $c.stderr)" \
      '{name:$n,k:$k,form:$f,t:$t,guess:$g,m:$m,mode:$mode,failed:true,rc:$rc,why:$why,stderr_tail:$err}' > "$c.fail.json"
    rm -f "$c.out"
  fi
}
skipped() { # name reason ...
  jq -nc --arg n "$1" --arg r "$2" '{name:$n,skipped:true,reason:$r}' > "$D/cells/$1.fail.json"
}
p1seed() { echo $((2026110600 + $2 + 100 * (($1 - 20) / 4))); }
# Part 1 grid at k (optionally with k24 gating when gate=1)
p1() {
  local k=$1 gate=$2 tl="$3"
  for t in $tl; do for form in isogeny bits; do for g in $((t-2)) $((t-1)); do
    local s; s=$(p1seed $k $t)
    local base="p1_k${k}_${form}_t${t}_g${g}"
    local ok=0
    for mode in chain full; do
      if [ $gate = 1 ] && { [ ! -s "$D/cells/p1_k24_${form}_t${t}_g${g}_m3_$mode.json" ]; }; then
        [ -e "$D/cells/p1_k${k}_${form}_t${t}_g${g}_m3_$mode.fail.json" ] || skipped ${base}_m3_$mode "k=24 counterpart failed or absent"
      else ok=1; fi
    done
    [ $ok = 1 ] || [ $gate = 0 ] || continue
    cell ${base}_m2_full 900 0 $k $form $t $g 50 $s 2 full -
    for mode in chain full; do
      if [ $gate = 1 ] && [ ! -s "$D/cells/p1_k24_${form}_t${t}_g${g}_m3_$mode.json" ]; then continue; fi
      cell ${base}_m3_$mode 900 0 $k $form $t $g 50 $s 3 $mode -
    done
  done; done; done
}
p2() {
  local k=$1
  for t in 4 5 6; do
    lo=$(awk -v k=$k -v t=$t '$1==k&&$2==t{print $5}' "$D/analysis/pairing.tsv")
    seed=$((2026110540 + t)); [ $k = 28 ] && seed=$((2026110580 + t))
    for g in 0 3; do
      cell p2_k${k}_t${t}_g${g}_plain 600 20 $k bits $t $g 100 $seed 2 full 30000000
      cell p2_k${k}_t${t}_g${g}_t0 600 20 $k "t0-bits@$lo" $t $g 100 $seed 2 full 30000000
    done
  done
}
case "$1" in
  repro)
    cell repro_p1_k20_bits_t4_g3_m3_chain 600 0 20 bits 4 3 100 2026111304 3 chain -
    cell repro_p2_k20_t5_g3_plain 600 20 20 bits 5 3 100 2026110205 2 full 30000000
    cell repro_p2_k20_t5_g3_t0 600 20 20 "t0-bits@1" 5 3 100 2026110205 2 full 30000000;;
  p2k24) p2 24;;
  p1k20) p1 20 0 "6";;
  p1k24) p1 24 0 "6 8";;
  p2k28) p2 28;;
  p1k28) p1 28 1 "6 8";;
esac
echo "stage $1 done $(date -u +%FT%TZ)"
