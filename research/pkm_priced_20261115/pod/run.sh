#!/bin/bash
# Frozen run of research/pkm_priced_20261115 (PROTOCOL.md).  Sequential and uncontended on purpose: wall times are evidence only
# when nothing else runs; counted work does not depend on load.  Everything lands in $OUT with SHA256SUMS.
#   WORK=/work OUT=/work/out bash run.sh            (after build.sh)
# Optional: PLAIN=0 skips the plain-chain reference (3 targets, ~70 s each, large memory at N=12).
set -u; WORK=${WORK:-/work}; OUT=${OUT:-$WORK/out}; PLAIN=${PLAIN:-1}; BIN=$WORK/bin
export RAYON_NUM_THREADS=1
mkdir -p "$OUT/sess" && cd "$OUT"
P=786433; SEED=2026111501; TARGETS=24            # frozen: prime, x_R seed, targets per N for the S5 build / coefficient check
F4T_3=8; F4T_4=4                                  # frozen: targets that also run the F4 stage (counts agree to ~4e-7 across targets)
log() { echo "$(date -u +%FT%TZ) $*" | tee -a runlog.txt; }
{ date -u +%FT%TZ; uname -a; lscpu | head -20; free -g; nproc; cat "$WORK/ids.txt"; echo "load $(cat /proc/loadavg)"; } > env.txt
cp "$WORK/ids.txt" ids.txt 2>/dev/null
for t in 3 4; do
  N=$((4*t)); log "N=$N curve + calibration"
  $BIN/pkm_symmetric_tower --mode curve --p $P --t $t --spec-out spec-t$t.json > curve-t$t.json 2> curve-t$t.err || log "curve t=$t rc=$?"
  $BIN/pkm_symmetric_tower --mode calibrate --p $P --t $t > calib-t$t.json 2> calib-t$t.err || log "calibrate t=$t rc=$?"
done
for t in 3 4; do                                   # matched rho-neg (ecbench), then replay and the 5a band
  log "rho-neg t=$t"
  $BIN/ecbench run --spec spec-t$t.json --out sess/rho-t$t --cpus inherit --allow-busy > sess/rho-t$t.log 2>&1; log "rho run t=$t rc=$?"
  $BIN/ecbench verify --dir sess/rho-t$t --replay-all --exit-code > sess/rho-t$t.verify.json 2>&1; log "rho verify t=$t rc=$?"
  $BIN/ecbench band --dir sess/rho-t$t --json band-t$t.json > band-t$t.txt 2>&1; log "rho band t=$t rc=$?"
done
for t in 3 4; do                                   # Stage 0 (coefficient check, counters) and the F4 stage, same targets
  f4t=$F4T_3; [ $t -eq 4 ] && f4t=$F4T_4
  log "stage0 t=$t"
  $BIN/pkm_symmetric_tower --mode stage0 --p $P --t $t --targets $TARGETS --f4-targets $f4t --seed $SEED --budget 3600 \
      > stage0-t$t.jsonl 2> stage0-t$t.err; log "stage0 t=$t rc=$?"
done
if [ "$PLAIN" = 1 ]; then                          # reference arm: plain chain, N=12, 3 targets, paired with the symmetric arm
  log "plain-chain reference N=12"
  $BIN/pkm_symmetric_tower --m 4 --p $P --t-min 3 --t-max-m4 3 --arms plain-chain,symmetric --random 3 --budget 3600 \
      > plain-ref-t3.jsonl 2> plain-ref-t3.err; log "plain-chain rc=$?"
fi
for t in 3 4; do
  $BIN/pkm_symmetric_tower --mode summary --stage0 stage0-t$t.jsonl --curve curve-t$t.json --calib calib-t$t.json \
      --band band-t$t.json --amortise 100 > summary-t$t.json 2> summary-t$t.err; log "summary t=$t rc=$?"
done
log done; find . -type f ! -name SHA256SUMS | sort | xargs sha256sum > SHA256SUMS; wc -l SHA256SUMS
