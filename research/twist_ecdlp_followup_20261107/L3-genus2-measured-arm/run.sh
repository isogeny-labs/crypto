#!/bin/bash
# run.sh -- single driver for L3-genus2-measured-arm.  Aborts if a frozen file changed.
# Full run:     bash run.sh
# Reduced replication subset (sizes k = 8..12, controls at k = 8, 10):
#               KMAX=12 CTRLSET=reduced OUT=$D/out_reduced RES=$D/results_reduced.json bash run.sh
# Environment: D (directory), KMAX (default 16), MSEED (default 20261107), OUT (raw directory),
#              CTRLSET (full|reduced|none), JOBS (concurrent gp processes, default 2).
set -u
D=${D:-research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm}
KMAX=${KMAX:-16}; MSEED=${MSEED:-20261107}; OUT=${OUT:-$D/out}; CTRLSET=${CTRLSET:-full}; JOBS=${JOBS:-2}
RES=${RES:-$D/results.json}; SUM=${SUM:-$OUT/summary.txt}
VENVPY=${VENVPY:-<scratch>/venv/bin/python}
cd "$D" || exit 1
if [ "${SKIPFREEZE:-0}" != "1" ]; then
  sha256sum -c FROZEN_SHA256.txt --quiet || { echo "ABORT: a frozen file changed (see DEVIATIONS.md)"; exit 2; }
fi
T0=$(date +%s)
mkdir -p "$OUT"
rm -f "$OUT"/main_k*.txt "$OUT"/ctrl_*.txt "$OUT"/log_*.txt "$OUT"/summary.txt "$RES"
# job list: ascending sizes; controls queued next to their size
JOBLIST=()
for k in $(seq 8 "$KMAX"); do
  JOBLIST+=("main $k 1" "main $k 2")
  if [ "$CTRLSET" = "full" ]; then
    case $k in
      8) JOBLIST+=("ctrl PCA 8" "ctrl PCC 8" "ctrl NCA 8" "ctrl NCB 8" "ctrl NCC 8");;
      10) JOBLIST+=("ctrl PCA 10" "ctrl PCC 10" "ctrl NCA 10" "ctrl NCB 10" "ctrl NCC 10");;
      12) JOBLIST+=("ctrl PCA 12" "ctrl PCC 12" "ctrl NCA 12" "ctrl NCB 12");;
      13) JOBLIST+=("ctrl NCA 13");;
      14) JOBLIST+=("ctrl PCA 14");;
    esac
  elif [ "$CTRLSET" = "reduced" ]; then
    case $k in
      8|10) JOBLIST+=("ctrl PCA $k" "ctrl PCC $k" "ctrl NCA $k" "ctrl NCB $k" "ctrl NCC $k");;
    esac
  fi
done
runjob() {
  set -- $1
  if [ "$1" = main ]; then
    SRC=$D K=$2 HALF=$3 OUT=$OUT/main_k$2_h$3.txt MSEED=$MSEED timeout 2700 gp -q $D/main.gp </dev/null >$OUT/log_main_k$2_h$3.txt 2>&1
  else
    SRC=$D CTRL=$2 K=$3 OUT=$OUT/ctrl_$2_k$3.txt MSEED=$MSEED timeout 2700 gp -q $D/controls.gp </dev/null >$OUT/log_ctrl_$2_k$3.txt 2>&1
  fi
}
export -f runjob; export D OUT MSEED
printf '%s\n' "${JOBLIST[@]}" | xargs -P "$JOBS" -I{} bash -c 'runjob "{}"'
# size completeness: both halves wrote DONE; sizes beyond the first incomplete size are not analysed (preregistration section 6)
KLIST=""
for k in $(seq 8 "$KMAX"); do
  n1=$(grep -c '^\["DONE"' "$OUT/main_k${k}_h1.txt" 2>/dev/null || true)
  n2=$(grep -c '^\["DONE"' "$OUT/main_k${k}_h2.txt" 2>/dev/null || true)
  if [ "${n1:-0}" = "1" ] && [ "${n2:-0}" = "1" ]; then
    KLIST="${KLIST:+$KLIST,}$k"
  else
    if [ -s "$OUT/main_k${k}_h1.txt" ] || [ -s "$OUT/main_k${k}_h2.txt" ]; then
      KLIST="${KLIST:+$KLIST,}$k"
      echo "size $k INCOMPLETE (timeout); analysed with the completed instances, larger sizes excluded"
    fi
    break
  fi
done
echo "KLIST=$KLIST"
SRC=$D DATA=$OUT KLIST=$KLIST RES=$RES SUM=$SUM gp -q $D/analyze.gp </dev/null > "$OUT/log_analyze.txt" 2>&1
"$VENVPY" -I $D/plot_summary.py "$RES" "${RES%.json}_plot"
find "$D" -name target -type d -prune -exec rm -rf {} + 2>/dev/null
echo "elapsed seconds: $(( $(date +%s) - T0 ))"
du -sh "$D"
