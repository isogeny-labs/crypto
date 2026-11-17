#!/bin/bash
# Frozen commands of VERIFICATION.json, each redirected into research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/replicate (no original output touched).
D=research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm
R=research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/replicate
cd "$D" || exit 1
echo "START analyze $(date)"
SRC=$D DATA=$D/out KLIST=8,9,10,11,12,13,14,15,16 RES=$R/analyze/results.json SUM=$R/analyze/summary.txt gp -q analyze.gp </dev/null
echo "END analyze rc=$? $(date)"
echo "START reduced $(date)"
KMAX=12 CTRLSET=reduced OUT=$R/reduced RES=$R/reduced/results_reduced.json SUM=$R/reduced/summary.txt bash run.sh
echo "END reduced rc=$? $(date)"
echo "START full $(date)"
OUT=$R/full RES=$R/full/results.json SUM=$R/full/summary.txt bash run.sh
echo "END full rc=$? $(date)"
echo "START posthoc $(date)"
DATA=$R/full OUTF=$R/full/posthoc.txt gp -q posthoc.gp </dev/null
echo "END posthoc rc=$? $(date)"
cp RESULTS_head.md $R/assembly/ && cp $R/full/summary.txt $R/assembly/out/summary.txt && cp $R/full/posthoc.txt $R/assembly/out/posthoc.txt
D=$R/assembly bash $D/make_results.sh
echo "END assembly rc=$? $(date)"
echo ALLDONE
