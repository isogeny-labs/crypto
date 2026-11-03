#!/usr/bin/env bash
# run.sh -- single driver for G2-decoupled-AB-projection-arm (frozen protocol: PREREGISTRATION.md section 8).
# Order (positive controls first, see DEVIATIONS.md): hash check -> curves -> pcsynth (PC1-synthetic positive control)
# -> semaev (PC5 positive control + wall time + NC3) -> lemma (NC1, audit, NC4) -> tables (at most 2 concurrent gp) -> analyze -> opcounts (v3 addition) -> plots.
# Every gp call: `timeout 900 gp -q -f SCRIPT > out/NAME.out 2>&1`.
set -u
D=${G2D:-/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G2-decoupled-AB-projection-arm}
PY=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python
cd "$D" || exit 2
unset G2OUT G2DEV E3OUT E3PS E3PE
mkdir -p out
: > out/timing.txt
t() { local name=$1; shift; local s=$(date +%s); "$@"; local rc=$?; echo "$name|rc=$rc|wall_s=$(( $(date +%s)-s ))" >> out/timing.txt; return $rc; }

gp -q --version-short > out/env_gp_version.txt 2>&1
df -h . > out/env_disk.txt; date -u +%FT%TZ > out/env_date.txt; nproc > out/env_nproc.txt; cat /proc/loadavg > out/env_loadavg_start.txt
sha256sum -c SCRIPTS.sha256 > out/scripts_check.txt 2>&1 || { echo "FROZEN SCRIPT HASH MISMATCH"; cat out/scripts_check.txt; exit 3; }
sha256sum -c PREREG.sha256 >> out/scripts_check.txt 2>&1 || { echo "PREREGISTRATION HASH MISMATCH"; exit 3; }

t curves    timeout 900 gp -q -f g2_curves.gp   > out/curves.out   2>&1
t pcsynth   timeout 900 gp -q -f g2_pcsynth.gp  > out/pcsynth.out  2>&1
t semaev    timeout 900 gp -q -f g2_semaev.gp   > out/semaev.out   2>&1
t lemma     timeout 900 gp -q -f g2_lemma.gp    > out/lemma.out    2>&1

runtab() { P=$1 C=$2 FAMILY=$3 timeout 900 gp -q -f g2_tables.gp > out/tables_p$1_c$2_$3.out 2>&1; echo "tables_p$1_c$2_$3|rc=$?" >> out/timing.txt; }
jobs_run=0
for fam in A B; do
  if [ $fam = A ]; then PS="101 211 401"; else PS="101 211"; fi
  for p in $PS; do for c in 1 2 3 4 9; do
    runtab $p $c $fam &
    jobs_run=$((jobs_run+1))
    if [ $((jobs_run % 2)) -eq 0 ]; then wait; fi
  done; done
done
wait
t analyze   timeout 900 gp -q -f g2_analyze.gp  > out/analysis.out 2>&1
t opcounts  timeout 900 gp -q -f g2_opcounts.gp > out/opcounts.out 2>&1   # added in v3: exact operation counts from recorded tables
t opcjson   bash add_opcounts_json.sh "$D"   # v4: scripted results.json operation_counts key (was a manual jq step)
$PY plot_g2.py out/results.csv . > out/plot.out 2>&1
rm -rf target
cat /proc/loadavg > out/env_loadavg_end.txt
echo "done"
