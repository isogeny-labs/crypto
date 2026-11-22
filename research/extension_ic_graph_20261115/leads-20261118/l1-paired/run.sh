#!/usr/bin/env bash
# L1 paired protocol: the one frozen driver (PROTOCOL.md section 8). Local, one
# thread, `-j 1` builds, no paid compute, no network except the drand beacon.
# Usage: S=<scratch dir> bash run.sh
# Everything it writes is [pcc:unverified]. It never edits a committed file; the
# runner copies the small outputs into runs/ afterwards (section 7).
set -uo pipefail
: "${S:?set S to a fresh scratch directory}"
LANE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
REPO=$(git -C "$LANE" rev-parse --show-toplevel)
REL=${LANE#"$REPO"/}
STEPS="$LANE/../l1-steps-2-3"
RUN50_PUBLIC="$LANE/../../runpod-deferred-pair-20261117-50/public.json"
T_PLANNED=96          # measured fresh public targets
N=4096                # descent residuals per target and stream
AA_TARGETS=24         # the first 24 measured targets also run the A/A stream
INDEX_CAP=160         # generator indices 0..159 are the candidate pool
BUDGET_S=28800        # hard local budget: 8 CPU-hours, every step included
RESERVE_S=600         # kept for analysis and packing
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

# ---- 0. freeze and source identity ----
FREEZE=$(git -C "$REPO" log --diff-filter=A --format=%H -- "$REL/protocol.json" | tail -1)
[ -n "$FREEZE" ] || die "protocol.json is not committed"
git -C "$REPO" diff --quiet "$FREEZE" -- "$REL" || die "lane files differ from the freeze commit $FREEZE"
git -C "$REPO" diff --quiet HEAD -- "$REL" || die "uncommitted changes in the lane"
H=$(git -C "$REPO" show "$FREEZE:$REL/protocol.json" | sha256sum | cut -c1-64)
(cd "$LANE" && sha256sum -c SOURCES.sha256 > "$S/log/sources.check") || die "source hashes differ from SOURCES.sha256"
echo "freeze=$FREEZE protocol_sha256=$H" > "$S/log/freeze.txt"

# ---- 1. build (scoped, -j 1, offline) ----
timed build env SCRATCH="$S/build" OUT="$S/bin" bash "$LANE/build/build.sh" || die "build failed"
GEN="$S/bin/fresh-scaling-gen"; RUN="$S/bin/l1_paired"; ANA="$S/bin/l1p_analyze"

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
BOOT_SEED=$("$GEN" seed --nonce "$NONCE" --p 1009 --index 0 --role l1p-bootstrap)
printf 'nonce=%s\npilot_nonce=%s\ndrand_round=%s\ndrand_randomness=%s\ndrand_source=%s\nboot_seed=%s\n' \
  "$NONCE" "$PNONCE" "$ROUND" "$RAND" "$SOURCE" "$BOOT_SEED" > "$S/log/nonce.txt"

# ---- 3. controls that need no fresh target ----
CTL="$S/out/controls.kv"; : > "$CTL"
timed pc1 "$RUN" "$RUN50_PUBLIC" 389 389 30000 3415 "$S/out/pc1" - - || die "PC1 runner failed"
if python3 -I "$LANE/compare_pc1.py" "$S/out/pc1" "$STEPS/runs" > "$S/out/pc1.json"; then echo "pc1=pass" >> "$CTL"; else echo "pc1=fail" >> "$CTL"; die "PC1 failed (instrument does not reproduce Run50 / steps 2-3)"; fi
PC2=pass
for i in 0 1; do
  timed "gen-planted-$i" "$GEN" gen --nonce "$PNONCE" --p 1009 --index "$i" --kind planted --out "$S/planted" || die "planted generation"
  pub="$S/planted/p1009/planted-$(printf %04d "$i")/public.json"
  ans=$(python3 -I -c 'import json,sys; print(json.load(open(sys.argv[1]))["planted_scalar"])' "$S/planted/owner/p1009/planted-$(printf %04d "$i").answer.json")
  cs=$("$GEN" seed --nonce "$PNONCE" --p 1009 --index "$i" --role l1p-collect)
  timed "pc2-$i" "$RUN" "$pub" "$cs" "$cs" "$N" "$N" "$S/out/pc2-$i" - "$ans" || PC2=fail
  awk -F'\t' 'NR==1{for(i=1;i<=NF;i++)c[$i]=i} NR==2{ if($c["owner_check"]!="1"||$c["pipeline_verified"]!="1"||$c["replay_fail_M"]!="0"||$c["flip_refused"]!=$c["flip_total"]) exit 1 }' \
    "$S/out/pc2-$i/target.tsv" || PC2=fail
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
  cs=$("$GEN" seed --nonce "$NONCE" --p 1009 --index "$i" --role l1p-collect)
  aa=-; [ "$DONE" -lt "$AA_TARGETS" ] && aa=$("$GEN" seed --nonce "$NONCE" --p 1009 --index "$i" --role l1p-aa-stream)
  timed "target-$id" "$RUN" "$pub" "$cs" "$cs" "$N" "$N" "$S/out/t$id" "$aa" -
  rc=$?
  c=$(tail -1 "$LEDGER" | cut -f3); [ "$c" -gt "$MAXT" ] && MAXT=$c
  case $rc in
    0) DONE=$((DONE + 1)); echo "$i" >> "$S/out/measured.txt" ;;
    10) printf '%s\tadmission-refused\n' "$i" >> "$S/out/refused.tsv" ;;
    11) DONE=$((DONE + 1)); printf '%s\tcollection-not-rank-ready\n' "$i" >> "$S/out/refused.tsv"; echo "$i" >> "$S/out/collection_failed.txt" ;;
    *) die "runner error on index $i (see $S/log/target-$id.err)" ;;
  esac
  # determinism control: the first measured target is run a second time
  if [ "$rc" = 0 ] && [ ! -e "$S/out/dc.done" ]; then
    timed "dc-$id" "$RUN" "$pub" "$cs" "$cs" "$N" "$N" "$S/out/dc" "$aa" - || die "DC rerun failed"
    DC=pass
    for f in target.tsv dexact.jsonl rows.primary.jsonl accepted.primary.jsonl; do
      cmp -s "$S/out/t$id/$f" "$S/out/dc/$f" || DC=fail
    done
    echo "dc=$DC" >> "$CTL"; touch "$S/out/dc.done"; [ "$DC" = pass ] || die "DC (determinism) failed"
  fi
done
printf 'budget_truncated=%s\nt_planned=%s\nt_done=%s\npool=%s\n' "$TRUNC" "$T_PLANNED" "$DONE" "$POOL" >> "$CTL"

# ---- 6. analysis (frozen program) ----
{ for d in "$S"/out/t*/; do cat "$d/target.tsv"; done; } > "$S/out/all.tsv"
awk -F'\t' 'NR==1 || $1=="primary"' "$S/out/all.tsv" > "$S/out/targets.tsv"
awk -F'\t' 'NR==1 || $1=="aa"' "$S/out/all.tsv" > "$S/out/aa.tsv"
timed analysis "$ANA" analyze "$S/out/targets.tsv" "$S/out/aa.tsv" "$BOOT_SEED" "$CTL" "$S/out/analysis.json" || die "analysis failed"

# ---- 7. pack: large files stay in scratch with SHA-256; small ones are listed for runs/ ----
(cd "$S/out" && for d in t* pc1 pc2-* dc; do [ -d "$d" ] && gzip -n -9 "$d"/*.jsonl; done; true)
(cd "$S" && find out targets planted log -type f | sort | xargs sha256sum) > "$S/RAW.sha256"
echo "spent_cpu_s=$(spent)" >> "$S/log/freeze.txt"
# staging for runs/ (PROTOCOL.md section 7): the runner copies $S/commit/* into runs/ unchanged
C="$S/commit"; mkdir -p "$C"
cp "$S/out/targets.tsv" "$S/out/aa.tsv" "$S/out/analysis.json" "$S/out/controls.kv" "$S/out/pc1.json" "$S/out/refused.tsv" "$C/"
[ -e "$S/out/collection_failed.txt" ] && cp "$S/out/collection_failed.txt" "$C/"
cp "$S/log/nonce.txt" "$S/log/freeze.txt" "$S/log/cpu.tsv" "$C/"; cp "$S/log/build.out" "$C/build.log"
while read -r i; do tr -d '\n' < "$S/targets/p1009/public-$(printf %04d "$i")/public.json"; echo; done < "$S/out/measured.txt" > "$C/publics.jsonl"
cp "$S/RAW.sha256" "$C/RAW.sha256"
for f in "$C"/*; do
  if [ "$(stat -c %s "$f")" -gt 65536 ]; then split -d -a 2 -C 60000 "$f" "$f.part" && rm "$f"; fi
done
(cd "$C" && sha256sum * > COMMIT.sha256)
cat "$S/out/analysis.json"
