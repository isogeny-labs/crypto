#!/bin/bash
# PROPOSED replacement for research/prime_fall_degree_20261027/isolab/entry.sh (plan, untested).
# Usage: entry.sh <stage>...
#   g1                      gate G1: hky_fp_tower_check must print "G1: 6 cases pass"
#   g3                      gate G3: prime_fall_degree_tools gate3 over this job's closure rows
#   x:T:K:CPRED:DENSE       Amendment 7 bracket for Kummer m = 2 at P1, t = T, target K
#   anything else           passed to run.sh (g4 hky-A hky-B hky-C d-A d-B cap-A)
# Differences from the original: binary hashes and host facts are uploaded before any stage;
# G1/G3/bracket stages; the job exits non-zero if any progress.txt line ends in a non-zero exit.
set -u
: "${ISOLAB_RESULTS_URI:?}"
S=/work/study
mkdir -p "$S/runs" "$S/gates" && cp /opt/study/run.sh "$S/run.sh" && cd "$S" || exit 2
export BIN=/usr/local/bin/pkm_tower_pilot
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-14}
P1=2013265921
log() { echo "$(date -u +%FT%TZ) $*" >> "$S/progress.txt"; }
up() {
  aws s3 sync "$S/runs" "$ISOLAB_RESULTS_URI/runs" --only-show-errors
  aws s3 sync "$S/gates" "$ISOLAB_RESULTS_URI/gates" --only-show-errors
  aws s3 cp "$S/progress.txt" "$ISOLAB_RESULTS_URI/progress.txt" --only-show-errors 2>/dev/null
}
sha256sum /usr/local/bin/pkm_tower_pilot /usr/local/bin/prime_fall_degree_tools \
  /usr/local/bin/hky_fp_tower_check > "$S/binary.sha256"
{ echo "argv: $*"; echo "nproc: $(nproc)"; echo "rayon: $RAYON_NUM_THREADS"
  grep -m1 'model name' /proc/cpuinfo; grep MemTotal /proc/meminfo; uname -r; } > "$S/host.txt"
aws s3 cp "$S/binary.sha256" "$ISOLAB_RESULTS_URI/binary.sha256" --only-show-errors
aws s3 cp /opt/study/binaries.sha256 "$ISOLAB_RESULTS_URI/build-binaries.sha256" --only-show-errors
aws s3 cp "$S/host.txt" "$ISOLAB_RESULTS_URI/host.txt" --only-show-errors
( while sleep 300; do up; done ) &
syncer=$!

# One capped F4 run; prints refuted | open | censored.
capped() { # t k c dense
  local name="X-m2-t$1-k$2-c$3"
  log "start $name"
  "$BIN" --p $P1 --ladder-t none --planted 0 --random 2 --kinds kummer --m 2 --controls tower \
    --t-min "$1" --max-t "$1" --targets "$2" --engine tower --cap "$3" --budget 86400 \
    --max-dense "$4" --out "runs/$name.jsonl" > "runs/$name.out" 2> "runs/$name.log"
  log "end $name exit $?"
  if grep -q '"inconsistent":true' "runs/$name.jsonl" 2>/dev/null; then echo refuted
  elif grep -q '"timed_out":true' "runs/$name.jsonl" 2>/dev/null || [ ! -s "runs/$name.jsonl" ]; then echo censored
  else echo open; fi
}
bracket() { # t k cpred dense  (Amendment 7 B, draft)
  local t=$1 k=$2 cp=$3 dense=$4 c r
  c=$((cp - 1)); r=$(capped "$t" "$k" "$c" "$dense")
  if [ "$r" = refuted ]; then
    while [ "$r" = refuted ] && [ "$c" -gt 2 ]; do c=$((c - 1)); r=$(capped "$t" "$k" "$c" "$dense"); done
  elif [ "$r" = open ]; then
    c=$cp; r=$(capped "$t" "$k" "$c" "$dense")
    while [ "$r" = open ] && [ "$c" -lt $((cp + 2)) ]; do c=$((c + 1)); r=$(capped "$t" "$k" "$c" "$dense"); done
  fi
  log "bracket t=$t k=$k last_cap=$c last_result=$r"
}

for s in "$@"; do
  case "$s" in
    g1) log "start g1"; /usr/local/bin/hky_fp_tower_check > gates/g1.log 2>&1; rc=$?
        grep -q 'G1: 6 cases pass' gates/g1.log || { [ "$rc" = 0 ] && rc=1; }; log "end g1 exit $rc"
        [ "$rc" = 0 ] || break ;;
    g3) log "start g3"; /usr/local/bin/prime_fall_degree_tools gate3 "$S" "$BIN" --max-n 20 --jobs 2 \
          --budget 900 > gates/g3.log 2>&1; rc=$?
        grep -q ', 0 disagreements' gates/g3.log || { [ "$rc" = 0 ] && rc=1; }; log "end g3 exit $rc"
        [ "$rc" = 0 ] || break ;;
    x:*) IFS=: read -r _ t k cp dense <<< "$s"; bracket "$t" "$k" "$cp" "$dense" ;;
    *) bash ./run.sh "$s" ;;
  esac
done
kill "$syncer" 2>/dev/null; up
grep -Eq ' exit [1-9][0-9]*$' "$S/progress.txt" && exit 1
exit 0
