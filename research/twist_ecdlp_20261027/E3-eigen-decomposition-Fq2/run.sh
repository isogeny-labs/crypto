#!/usr/bin/env bash
# run.sh -- single driver for E3-eigen-decomposition-Fq2 (frozen protocol: PREREGISTRATION.md section 9)
# Order (positive controls before any table arm; see DEVIATIONS.md): freeze hashes -> curves -> model -> lemma (PC1, NC1/H4 audit)
# -> semaev (PC2, wall time) -> tables (Family A, B; at most 2 concurrent gp processes) -> analyze -> plots.
# Every gp call is `timeout 900 gp -q -f script.gp > out/NAME.out 2>&1`.
set -u
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E3-eigen-decomposition-Fq2
PY=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python
cd "$D" || exit 2
unset E3DEV E3OUT E3PS E3PE
mkdir -p out
: > out/timing.txt
t() { # t NAME cmd...   (records wall seconds and exit code)
  local name=$1; shift; local s=$(date +%s); "$@"; local rc=$?; echo "$name|rc=$rc|wall_s=$(( $(date +%s)-s ))" >> out/timing.txt; return $rc; }

gp -q --version-short > out/env_gp_version.txt 2>&1
df -h . > out/env_disk.txt; date -u +%FT%TZ > out/env_date.txt
if [ ! -f SCRIPTS.sha256 ]; then sha256sum e3_*.gp plot_e3.py > SCRIPTS.sha256; fi
sha256sum -c SCRIPTS.sha256 > out/scripts_check.txt 2>&1 || { echo "FROZEN SCRIPT HASH MISMATCH"; cat out/scripts_check.txt; exit 3; }

t curves   timeout 900 gp -q -f e3_curves.gp  > out/curves.out  2>&1
t model    timeout 900 gp -q -f e3_model.gp   > out/model.out   2>&1
t lemma    timeout 900 gp -q -f e3_lemma.gp   > out/lemma.out   2>&1
t semaev   timeout 900 gp -q -f e3_semaev.gp  > out/semaev.out  2>&1

runtab() { P=$1 C=$2 FAMILY=$3 timeout 900 gp -q -f e3_tables.gp > out/tables_p$1_c$2_$3.out 2>&1; echo "tables_p$1_c$2_$3|rc=$?" >> out/timing.txt; }
jobs_run=0
for fam in A B; do
  if [ $fam = A ]; then PS="101 211 401"; else PS="101 211"; fi
  for p in $PS; do for c in 1 2; do
    runtab $p $c $fam &
    jobs_run=$((jobs_run+1))
    if [ $((jobs_run % 2)) -eq 0 ]; then wait; fi
  done; done
done
wait
t analyze  timeout 900 gp -q -f e3_analyze.gp > out/analysis.out 2>&1
$PY plot_e3.py out/results.csv . > out/plot.out 2>&1

E3DIR="$D" bash "$D/opcounts.sh"
echo "done"
