#!/usr/bin/env bash
# run.sh -- single driver for experiment E0-separation-lemma (frozen protocol: PREREGISTRATION.md)
# Order: environment record -> verify frozen script hashes -> controls (positive controls first, abort on failure)
#        -> T1/T2/T3 -> T4/T5 -> aggregate -> plots.  At most 2 gp processes at a time. Every gp run under `timeout 900`.
# Raw outputs go to out/ (deviation from PREREGISTRATION.md section 9: file names are the same but live in out/ instead of the directory root).
set -u
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E0-separation-lemma
PY=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python
cd "$D" || exit 2
mkdir -p out
: > out/timing.txt
SCRIPTS="lib_e0.gp t1_pairs.gp t2_bonly.gp t3_allab.gp t4_structure.gp t5_hom.gp controls.gp"

run_gp() {  # run_gp NAME "ENV=.. ENV=.." gp-script-files...
  local name=$1 envs=$2; shift 2
  local s e rc
  s=$(date +%s)
  env $envs timeout 900 gp -q "$@" </dev/null > "out/$name.txt" 2>&1
  rc=$?
  e=$(date +%s)
  echo "$name|rc=$rc|start=$s|end=$e|wall_s=$((e-s))" >> out/timing.txt
  return $rc
}

gp -q --version-short 2>&1 | tee out/env_gp_version.txt
df -h . | tee out/env_disk.txt
date -u +%FT%TZ > out/env_date.txt

if [ ! -f SCRIPTS.sha256 ]; then sha256sum $SCRIPTS > SCRIPTS.sha256; fi
sha256sum -c SCRIPTS.sha256 > out/scripts_check.txt 2>&1 || { echo "FROZEN SCRIPT HASH MISMATCH: amend protocol first"; cat out/scripts_check.txt; exit 3; }
sha256sum -c PREREGISTRATION.sha256 >> out/scripts_check.txt 2>&1 || { echo "PREREGISTRATION CHANGED"; exit 3; }

# ---- controls first ----
run_gp controls "E0_X=1" lib_e0.gp controls.gp
if ! grep -q 'CONTROLS_PASS' out/controls.txt; then echo "CONTROLS FAILED: stop (see out/controls.txt)"; exit 4; fi

# ---- T1 / T2 / T3 in two lanes ----
(
  run_gp T1c "E0_PMIN=67 E0_PMAX=101" lib_e0.gp t1_pairs.gp
  run_gp T1d "E0_PMIN=127 E0_PMAX=127" lib_e0.gp t1_pairs.gp
) &
(
  run_gp T1a "E0_PMIN=3 E0_PMAX=31" lib_e0.gp t1_pairs.gp
  run_gp T1b "E0_PMIN=37 E0_PMAX=61" lib_e0.gp t1_pairs.gp
  run_gp T1_bin_ord "E0_MMIN=2 E0_MMAX=6 E0_FAMILY=ord" lib_e0.gp t1_pairs.gp
  run_gp T1_bin_ss "E0_MMIN=2 E0_MMAX=5 E0_FAMILY=ss" lib_e0.gp t1_pairs.gp
  run_gp T2a "E0_PMIN=3 E0_PMAX=67" lib_e0.gp t2_bonly.gp
  run_gp T2b "E0_PMIN=71 E0_PMAX=131" lib_e0.gp t2_bonly.gp
  run_gp T2c "E0_PMIN=137 E0_PMAX=199" lib_e0.gp t2_bonly.gp
  run_gp T3 "E0_PMIN=5 E0_PMAX=23" lib_e0.gp t3_allab.gp
) &
wait
(
  run_gp T2_bin_ord "E0_MMIN=2 E0_MMAX=10 E0_FAMILY=ord" lib_e0.gp t2_bonly.gp
) &
(
  run_gp T2_bin_ss "E0_MMIN=2 E0_MMAX=9 E0_FAMILY=ss" lib_e0.gp t2_bonly.gp
) &
wait
# ---- T4 / T5 ----
(
  run_gp T4 "E0_PMIN=3 E0_PMAX=101" lib_e0.gp t4_structure.gp
) &
(
  run_gp T5 "E0_PMIN=3 E0_PMAX=61" lib_e0.gp t5_hom.gp
) &
wait

bash aggregate.sh > out/results_summary.txt 2>&1
$PY plot_e0.py > out/plot_log.txt 2>&1
echo "run.sh finished $(date -u +%FT%TZ)"
