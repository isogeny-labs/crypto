#!/usr/bin/env bash
# Independent replay of the k = 4 pipeline runs (validator tooling, 2026-10-21).
#
#   cargo build --release --example quartic_pipeline_d4 --example quartic_pipeline_v4 \
#       --example quartic_pipeline_gls --example gls_stage_a
#   OUT=/some/scratch BIN=target/release/examples replay.sh e24 101:1 101:2 ...
#
# Arguments are those of experiments/{27_quartic_pipeline_d4,29_quartic_pipeline_v4,30_gls_v4}/run_all.sh;
# only the --json path differs, and it points outside the repository.  RAYON_NUM_THREADS=4 fixes the
# target batch (2 x rayon threads), which shares the run's seeded RNG with the row choices and
# Wiedemann, so the thread count is part of the deterministic trajectory; the recorded host had 4 vCPUs.
set -u
OUT=${OUT:?set OUT to a scratch directory}
BIN=${BIN:-target/release/examples}
exp=$1; shift
case $exp in
  e19) bin=$BIN/quartic_pipeline_d4;  extra="";                         out=$OUT/e19 ;;
  e24) bin=$BIN/quartic_pipeline_v4;  extra="--max-residuals 5000000";  out=$OUT/e24 ;;
  e25) bin=$BIN/quartic_pipeline_gls; extra="--max-residuals 5000000";  out=$OUT/e25 ;;
  e25-stage-a)
    mkdir -p "$OUT/e25_stage_a"
    "$BIN/gls_stage_a" --sizes 29,37,41,53,61,89,97,101,109,113,149,197,269,397 --seeds 4 \
      --json "$OUT/e25_stage_a/stage_a.json" 2>"$OUT/e25_stage_a/stage_a.log"
    exit $? ;;
  *) echo "unknown $exp"; exit 2 ;;
esac
mkdir -p "$out"
for ps in "$@"; do
  p=${ps%%:*}; s=${ps##*:}
  f=$out/p${p}s${s}.json
  [ -s "$f" ] && continue
  echo "start $exp p=$p s=$s $(date -u +%FT%TZ)" >>"$out/replay.log"
  RAYON_NUM_THREADS=4 "$bin" --sizes "$p" --first-seed "$s" --seeds 1 \
    --rho-runs 16 $extra --json "$f.part" >>"$out/replay.log" 2>&1
  rc=$?
  [ $rc -eq 0 ] && [ -s "$f.part" ] && mv "$f.part" "$f"
  echo "end $exp p=$p s=$s exit=$rc $(date -u +%FT%TZ)" >>"$out/replay.log"
done
echo "finished $exp $(date -u +%FT%TZ)" >>"$out/replay.log"
