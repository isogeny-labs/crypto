#!/usr/bin/env bash
# L1 x C1 descent-cost protocol: the one frozen driver (PROTOCOL.md section 8).
# Local, one thread, `-j 1` builds, no paid compute, no network except the drand beacon.
# Usage: S=<empty scratch dir> bash run.sh
# Everything it writes is [pcc:unverified]. It never edits a committed file; the
# runner copies the small outputs into runs/ afterwards (PROTOCOL.md section 7).
set -uo pipefail
: "${S:?set S to a fresh scratch directory}"
LANE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
REPO=$(git -C "$LANE" rev-parse --show-toplevel)
REL=${LANE#"$REPO"/}
STEPS="$LANE/../l1-steps-2-3"
L1P="$LANE/../l1-paired"
RUN50_PUBLIC="$LANE/../../runpod-deferred-pair-20261117-50/public.json"
T_PLANNED=128        # measured fresh public targets (counted metric, all of them)
WALL_TARGETS=64      # the first 64 measured targets also run the five wall processes
AA_TARGETS=32        # the first 32 measured targets also run the independent A/A stream
NCAP=8192            # residual cap per arm (the pipeline's own descent cap)
INDEX_CAP=192        # generator indices 0..191 are the candidate pool
BUDGET_S=28800       # hard local budget: 8 CPU-hours, every step included
RESERVE_S=900        # kept for analysis and packing
HOST_ISOLATED=0      # local shared host: every wall verdict is labelled indicative
ARMS="base l1 c1 l1c1"
mkdir -p "$S" && [ -z "$(ls -A "$S")" ] || { echo "S must be empty"; exit 2; }
mkdir -p "$S/log" "$S/targets" "$S/out" "$S/bin"
LEDGER="$S/log/cpu.tsv"; : > "$LEDGER"
spent() { awk -F'\t' '{s+=$3} END {printf "%d", s+0}' "$LEDGER"; }
# timed NAME CMD...: runs CMD, appends "NAME rc cpu_s wall_s" to the ledger, returns rc
timed() {
  local name=$1; shift
  local tf="$S/log/$name.time" rc
  local TIMEFORMAT='%U %S %R'
  { time "$@" > "$S/log/$name.out" 2> "$S/log/$name.err" ; } 2> "$tf"
  rc=$?
  read -r u s r < <(tail -1 "$tf")
  printf '%s\t%s\t%s\t%.0f\n' "$name" "$rc" "$(awk -v u="$u" -v s="$s" 'BEGIN{printf "%.0f", u+s}')" "$r" >> "$LEDGER"
  return $rc
}
die() { echo "STOP: $*" | tee -a "$S/log/stop.txt"; exit 1; }
jget() { python3 -I -c 'import json,sys; v=json.load(open(sys.argv[1])); print(v[sys.argv[2]] if v[sys.argv[2]] is not None else -1)' "$1" "$2"; }

# ---- 0. freeze and source identity ----
FREEZE=$(git -C "$REPO" log --diff-filter=A --format=%H -- "$REL/protocol.json" | tail -1)
[ -n "$FREEZE" ] || die "protocol.json is not committed"
git -C "$REPO" diff --quiet "$FREEZE" -- "$REL" || die "lane files differ from the freeze commit $FREEZE"
git -C "$REPO" diff --quiet HEAD -- "$REL" || die "uncommitted changes in the lane"
H=$(git -C "$REPO" show "$FREEZE:$REL/protocol.json" | sha256sum | cut -c1-64)
(cd "$LANE" && sha256sum -c SOURCES.sha256 > "$S/log/sources.check") || die "source hashes differ from SOURCES.sha256"
echo "freeze=$FREEZE protocol_sha256=$H" > "$S/log/freeze.txt"

# ---- 1. build (scoped, -j 1, offline; frozen and counted linear stage) ----
timed build env SCRATCH="$S/build" OUT="$S/bin" bash "$LANE/build/build.sh" || die "build failed"
GEN="$S/bin/fresh-scaling-gen"; RF="$S/bin/l1c1_cost_frozen"; RC="$S/bin/l1c1_cost_counted"; ANA="$S/bin/l1c1_analyze"

# ---- 2. nonce (drand mainnet default chain, first round >= freeze + 1 h) ----
TC=$(git -C "$REPO" show -s --format=%ct "$FREEZE")
GENESIS=1595431050; PERIOD=30
ROUND=$(( (TC + 3600 - GENESIS + PERIOD - 1) / PERIOD + 1 ))
ROUND_TIME=$(( GENESIS + (ROUND - 1) * PERIOD ))
[ "$(date +%s)" -ge "$ROUND_TIME" ] || die "too early: drand round $ROUND is published at $ROUND_TIME"
CHAIN=8990e7a9aaed2ffed73dbd7092123d6f289930540d7651336225dc172e51b2ce
RAND=""; SOURCE=""
for host in https://api.drand.sh https://api2.drand.sh https://drand.cloudflare.com; do
  if curl -sS --max-time 20 "$host/$CHAIN/public/$ROUND" -o "$S/log/drand.json"; then
    RAND=$(python3 -I -c 'import json,sys; v=json.load(open(sys.argv[1])); assert v["round"]==int(sys.argv[2]); print(v["randomness"])' "$S/log/drand.json" "$ROUND" 2>/dev/null) && SOURCE=$host && break
  fi
done
if [ -z "$RAND" ]; then
  # Disclosed fallback (PROTOCOL.md section 3): grindable by whoever made the freeze commit.
  RAND=$(git -C "$REPO" rev-parse "$FREEZE^{tree}"); ROUND=0; SOURCE="fallback:freeze-tree"
fi
NONCE=$("$GEN" nonce --protocol-sha256 "$H" --commit "$FREEZE" --drand-round "$ROUND" --drand-randomness "$RAND")
PNONCE=$("$GEN" pilot-nonce --protocol-sha256 "$H")
BOOT_SEED=$("$GEN" seed --nonce "$NONCE" --p 1009 --index 0 --role l1c1-bootstrap)
printf 'nonce=%s\npilot_nonce=%s\ndrand_round=%s\ndrand_randomness=%s\ndrand_source=%s\nboot_seed=%s\n' \
  "$NONCE" "$PNONCE" "$ROUND" "$RAND" "$SOURCE" "$BOOT_SEED" > "$S/log/nonce.txt"
CTL="$S/out/controls.kv"; : > "$CTL"; echo "host_isolated=$HOST_ISOLATED" >> "$CTL"

# ---- per-target procedure (PROTOCOL.md section 4) ----
# target NAME PUBLIC COLLECT_SEED AA_SEED|- WALL(0/1) OWNER|- ORDER_SEED DIR
# returns 0 ok, 10 refused, 11 collection not rank-ready; dies on any other error.
target() {
  local name=$1 pub=$2 cs=$3 aas=$4 wall=$5 owner=$6 oseed=$7 D=$8 rc a n tag sseed order
  mkdir -p "$D"
  timed "$name-prepare" "$RF" prepare "$pub" "$cs" "$D"; rc=$?
  [ "$rc" = 10 ] || [ "$rc" = 11 ] && return $rc
  [ "$rc" = 0 ] || die "prepare error on $name (see $S/log/$name-prepare.err)"
  order=$("$ANA" order "$oseed")
  for tag in primary aa; do
    if [ "$tag" = primary ]; then sseed=$cs; else [ "$aas" = - ] && continue; sseed=$aas; fi
    for a in $order; do
      [ "$a" = base-aa ] && continue
      timed "$name-$tag-counted-$a" "$RC" arm "$pub" "$cs" "$sseed" "$NCAP" - "$D" "$a" 1 "$D/arm-$a.$tag.counted" \
        || die "counted arm $a ($tag) failed on $name"
    done
    n=$(jget "$D/arm-base.$tag.counted.det.json" tau); [ "$n" -ge 0 ] && n=$((n + 1)) || n=$NCAP
    timed "$name-$tag-audit" "$RF" audit "$pub" "$cs" "$sseed" "$n" "$n" "$D" "$tag" "$owner" || die "audit ($tag) failed on $name"
    if [ "$tag" = primary ] && [ "$wall" = 1 ]; then
      for a in $order; do
        if [ "$a" = base-aa ]; then
          timed "$name-wall-base-aa" "$RF" arm "$pub" "$cs" "$sseed" "$NCAP" - "$D" base 0 "$D/arm-base-aa.primary.wall" || die "wall A/A failed on $name"
        else
          timed "$name-wall-$a" "$RF" arm "$pub" "$cs" "$sseed" "$NCAP" - "$D" "$a" 0 "$D/arm-$a.primary.wall" || die "wall arm $a failed on $name"
        fi
      done
    fi
    timed "$name-$tag-collect" "$RF" collect "$D" "$tag" "$NCAP" "$owner" "$D/line.$tag.tsv" || die "collect ($tag) failed on $name"
  done
  return 0
}

# ---- 3. controls that need no fresh target ----
# PC1: Run50 (seeds 389/389) on the frozen instrument reproduces Run50 and steps 2-3 ...
P1="$S/out/pc1"; mkdir -p "$P1"
timed pc1-prepare "$RF" prepare "$RUN50_PUBLIC" 389 "$P1" || die "PC1 prepare failed"
timed pc1-audit "$RF" audit "$RUN50_PUBLIC" 389 389 30000 3415 "$P1" primary - || die "PC1 audit failed"
if python3 -I "$L1P/compare_pc1.py" "$P1" "$STEPS/runs" > "$S/out/pc1.json"; then echo "pc1=pass" >> "$CTL"; else echo "pc1=fail" >> "$CTL"; die "PC1 failed (instrument does not reproduce Run50 / steps 2-3)"; fi
# ... PC1c: counted recount of the recorded Run50 stream, both solvers, and the K-stop arms ...
for a in base c1; do
  timed "pc1c-fixed-$a" "$RC" arm "$RUN50_PUBLIC" 389 389 "$NCAP" 3415 "$P1" "$a" 1 "$P1/arm-$a.fixed3415" || die "PC1c fixed arm $a failed"
done
for a in $ARMS; do
  timed "pc1c-kstop-$a" "$RC" arm "$RUN50_PUBLIC" 389 389 "$NCAP" - "$P1" "$a" 1 "$P1/arm-$a.primary.counted" || die "PC1c arm $a failed"
done
timed pc1c-collect "$RF" collect "$P1" primary "$NCAP" - "$P1/line.primary.tsv" || die "PC1c collect failed"
# ... PC1b: a recorded l1-paired fresh target (its first measured input and collect seed)
B1="$S/out/pc1b"; mkdir -p "$B1"
head -1 "$L1P/results/publics.jsonl" > "$B1/public.json"
B1CS=$(awk -F'\t' 'NR==1{for(i=1;i<=NF;i++)c[$i]=i} NR==2{print $c["collect_seed"]}' "$L1P/results/targets.tsv")
timed pc1b-prepare "$RF" prepare "$B1/public.json" "$B1CS" "$B1" || die "PC1b prepare failed"
timed pc1b-audit "$RF" audit "$B1/public.json" "$B1CS" "$B1CS" 4096 4096 "$B1" primary - || die "PC1b audit failed"
if python3 -I "$LANE/compare_pc1c.py" "$P1" "$B1" "$L1P/results/targets.tsv" > "$S/out/pc1c.json"; then
  printf 'pc1c=pass\npc1b=pass\n' >> "$CTL"
else
  printf 'pc1c=fail\npc1b=fail\n' >> "$CTL"; die "PC1c/PC1b failed"
fi
# PC2: planted fresh targets (known answers), the full per-target procedure with the wall arms
PC2=pass
for i in 0 1; do
  timed "gen-planted-$i" "$GEN" gen --nonce "$PNONCE" --p 1009 --index "$i" --kind planted --out "$S/planted" || die "planted generation"
  pub="$S/planted/p1009/planted-$(printf %04d "$i")/public.json"
  ans=$(python3 -I -c 'import json,sys; print(json.load(open(sys.argv[1]))["planted_scalar"])' "$S/planted/owner/p1009/planted-$(printf %04d "$i").answer.json")
  cs=$("$GEN" seed --nonce "$PNONCE" --p 1009 --index "$i" --role l1c1-collect)
  os=$("$GEN" seed --nonce "$PNONCE" --p 1009 --index "$i" --role l1c1-order)
  target "pc2-$i" "$pub" "$cs" - 1 "$ans" "$os" "$S/out/pc2-$i" || PC2=fail
  awk -F'\t' 'NR==1{for(i=1;i<=NF;i++)c[$i]=i} NR==2{ if($c["gates_failed"]!="-"||$c["kill_l1"]!="0"||$c["kill_c1"]!="0"||$c["censored"]!="0") exit 1 }' \
    "$S/out/pc2-$i/line.primary.tsv" || PC2=fail
done
echo "pc2=$PC2" >> "$CTL"; [ "$PC2" = pass ] || die "PC2 (planted fresh targets) failed"

# ---- 4. candidate pool and freshness (generated in index order, in batches) ----
POOL=0
grow_pool() {  # grow_pool K: generate indices POOL..K-1, then re-check freshness of the whole pool
  local k=$1 i
  for ((i = POOL; i < k; i++)); do
    timed "gen-$i" "$GEN" gen --nonce "$NONCE" --p 1009 --index "$i" --kind public --out "$S/targets" || die "generation of index $i"
  done
  POOL=$k
  "$GEN" check "$S"/targets/p1009/public-*/public.json > "$S/log/check.jsonl" || die "generator check refused a candidate"
  python3 -I "$LANE/check_unseen.py" "$REPO" "$S"/targets/p1009/public-*/public.json > "$S/log/unseen.jsonl"
  [ $? -le 1 ] || die "freshness check could not read the known inputs"
  true
}
grow_pool $((T_PLANNED + 16))
echo "unseen=pass" >> "$CTL"   # a seen candidate is refused (reason "seen") and never measured

# ---- 5. measured targets, index order, first T_PLANNED admitted ----
: > "$S/out/refused.tsv"; DONE=0; MAXT=0; TRUNC=0
for ((i = 0; i < INDEX_CAP && DONE < T_PLANNED; i++)); do
  [ "$i" -lt "$POOL" ] || grow_pool $((POOL + 16 > INDEX_CAP ? INDEX_CAP : POOL + 16))
  id=$(printf %04d "$i"); pub="$S/targets/p1009/public-$id/public.json"
  if ! grep -qF "\"candidate\": \"$pub\", \"unseen\": true" "$S/log/unseen.jsonl"; then
    printf '%s\tseen\n' "$i" >> "$S/out/refused.tsv"; continue
  fi
  now=$(spent)
  if [ $(( now + 3 * MAXT / 2 )) -gt $(( BUDGET_S - RESERVE_S )) ]; then TRUNC=1; break; fi
  cs=$("$GEN" seed --nonce "$NONCE" --p 1009 --index "$i" --role l1c1-collect)
  os=$("$GEN" seed --nonce "$NONCE" --p 1009 --index "$i" --role l1c1-order)
  aa=-; [ "$DONE" -lt "$AA_TARGETS" ] && aa=$("$GEN" seed --nonce "$NONCE" --p 1009 --index "$i" --role l1c1-aa-stream)
  wall=0; [ "$DONE" -lt "$WALL_TARGETS" ] && wall=1
  before=$(spent)
  target "t$id" "$pub" "$cs" "$aa" "$wall" - "$os" "$S/out/t$id"
  rc=$?
  c=$(( $(spent) - before )); [ "$c" -gt "$MAXT" ] && MAXT=$c
  case $rc in
    0) DONE=$((DONE + 1)); echo "$i" >> "$S/out/measured.txt" ;;
    10) printf '%s\tadmission-refused\n' "$i" >> "$S/out/refused.tsv" ;;
    11) DONE=$((DONE + 1)); printf '%s\tcollection-not-rank-ready\n' "$i" >> "$S/out/refused.tsv"; echo "$i" >> "$S/out/collection_failed.txt" ;;
  esac
  # determinism control: the first measured target's counted procedure is run a second time
  if [ "$rc" = 0 ] && [ ! -e "$S/out/dc.done" ]; then
    target "dc-$id" "$pub" "$cs" - 0 - "$os" "$S/out/dc" || die "DC rerun failed"
    DC=pass
    for f in dexact.jsonl quotient.json rows.primary.jsonl accepted.primary.jsonl soldig.primary.txt \
             arm-base.primary.counted.det.json arm-l1.primary.counted.det.json arm-c1.primary.counted.det.json arm-l1c1.primary.counted.det.json; do
      cmp -s "$S/out/t$id/$f" "$S/out/dc/$f" || DC=fail
    done
    # the collect line, without its timing columns (CPUc_*, W_*, wall_run)
    untimed='import sys; h,v=[l.rstrip("\n").split("\t") for l in open(sys.argv[1])][:2]; print("\t".join(k+"="+x for k,x in zip(h,v) if not k.startswith(("CPUc_","W_","wall_run"))))'
    [ "$(python3 -I -c "$untimed" "$S/out/t$id/line.primary.tsv")" = "$(python3 -I -c "$untimed" "$S/out/dc/line.primary.tsv")" ] || DC=fail
    echo "dc=$DC" >> "$CTL"; touch "$S/out/dc.done"; [ "$DC" = pass ] || die "DC (determinism) failed"
  fi
done
printf 'budget_truncated=%s\nt_planned=%s\nt_done=%s\npool=%s\n' "$TRUNC" "$T_PLANNED" "$DONE" "$POOL" >> "$CTL"

# ---- 6. analysis (frozen program) ----
first=$(ls "$S"/out/t*/line.primary.tsv 2>/dev/null | head -1)
[ -n "$first" ] || die "no measured target"
{ head -1 "$first"; for f in "$S"/out/t*/line.primary.tsv; do tail -n +2 "$f"; done; } > "$S/out/targets.tsv"
fa=$(ls "$S"/out/t*/line.aa.tsv 2>/dev/null | head -1)
if [ -n "$fa" ]; then { head -1 "$fa"; for f in "$S"/out/t*/line.aa.tsv; do tail -n +2 "$f"; done; } > "$S/out/aa.tsv"; else head -1 "$first" > "$S/out/aa.tsv"; fi
timed analysis "$ANA" analyze "$S/out/targets.tsv" "$S/out/aa.tsv" "$BOOT_SEED" "$CTL" "$S/out/analysis.json" || die "analysis failed"

# ---- 7. pack: large files stay in scratch with SHA-256; small ones are staged for runs/ ----
(cd "$S/out" && for d in t* pc1 pc1b pc2-* dc; do [ -d "$d" ] && gzip -n -9 "$d"/*.jsonl "$d"/*.det.json 2>/dev/null; done; true)
(cd "$S" && find out targets planted log -type f | sort | xargs sha256sum) > "$S/RAW.sha256"
echo "spent_cpu_s=$(spent)" >> "$S/log/freeze.txt"
C="$S/commit"; mkdir -p "$C"
cp "$S/out/targets.tsv" "$S/out/aa.tsv" "$S/out/analysis.json" "$S/out/controls.kv" "$S/out/pc1.json" "$S/out/pc1c.json" "$S/out/refused.tsv" "$C/"
[ -e "$S/out/collection_failed.txt" ] && cp "$S/out/collection_failed.txt" "$C/"
cp "$S/log/nonce.txt" "$S/log/freeze.txt" "$S/log/cpu.tsv" "$C/"; cp "$S/log/build.out" "$C/build.log"
while read -r i; do tr -d '\n' < "$S/targets/p1009/public-$(printf %04d "$i")/public.json"; echo; done < "$S/out/measured.txt" > "$C/publics.jsonl"
cp "$S/RAW.sha256" "$C/RAW.sha256"
for f in "$C"/*; do
  if [ "$(stat -c %s "$f")" -gt 65536 ]; then split -d -a 2 -C 60000 "$f" "$f.part" && rm "$f"; fi
done
(cd "$C" && sha256sum * > COMMIT.sha256)
cat "$S/out/analysis.json"
