#!/usr/bin/env bash
# Stage the validation directories in the scratchpad (complete files only), then copy them
# into the two study directories in one step (lesson L-1a5a6b64022c: no placeholders in the
# shared worktree).  Usage: stage.sh [install]
set -eu
G=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/g4val
ST=$G/stage
rm -rf $ST; mkdir -p $ST/s1/scripts $ST/s1/out $ST/s2/scripts $ST/s2/out
SCRIPTS="paths.py build.sh build2.sh build3.sh binary_diff.sh freeze_check.sh jac.py selftest.py certs.py recompute.py sensitivity.py claims_check.py claims_classify.py exact_intervals.py mc_spread.py f2_check.py rho_efficiency.py compare_runs.py replay_s1.sh replay_s2.sh pipeline2.sh val_g4dump.rs verify_dump.py compare_analysis.py stage.sh"
COMMON_OUT="commands.txt exact_vs_analysis.txt freeze_check.txt selftest.txt certs.json recompute.json recompute.txt claims_check.json claims_check.txt claims_classified.txt exact_intervals.json exact_intervals.txt mc_spread.json mc_spread.txt rho_efficiency.txt sensitivity.json sensitivity.txt producer_bin_copies.sha256 build_evidence.txt binary_diff_bce0eb2148.txt binary_diff_bce0eb2148_path2.txt binary_diff_a99e7267f1.txt verify_dump.json verify_dump.txt dump_main.jsonl dump_rho.jsonl dump_g3.jsonl dump.sha256"
for s in s1 s2; do
  for f in $SCRIPTS; do cp $G/scripts/$f $ST/$s/scripts/; done
  for f in $COMMON_OUT; do cp $G/out/$f $ST/$s/out/; done
  cp $G/dump_in_main.txt $G/dump_in_rho.txt $ST/$s/out/
done
for f in f2_check.txt replay_s1_2053_4099_prodbin.jsonl replay_s1_8209_cvw_prodbin.jsonl replay_s1_prodbin.out \
         replay_s1_2053_4099_valbin.jsonl replay_s1_8209_cvw_valbin.jsonl replay_s1_valbin.out; do cp $G/out/$f $ST/s1/out/; done
for f in replay_s2_8209_wR_t2_prodbin.jsonl replay_s2_prodbin.out replay_s2_w_t2_valbin.jsonl replay_s2_R1_valbin.jsonl \
         replay_s2_valbin.out; do cp $G/out/$f $ST/s2/out/; done
cp $G/report_s1.md $ST/s1/validator-report.md
cp $G/report_s2.md $ST/s2/validator-report.md
if grep -l "@[A-Z0-9_]*@" $ST/s1/validator-report.md $ST/s2/validator-report.md; then echo "placeholders left"; exit 1; fi
echo "data files over 64 KiB (artifact-store policy):"; find $ST -type f -size +64k \( -name "*.json*" -o -name "*.log" \) -exec ls -la {} \;
(cd $ST && find . -type f | sort | xargs sha256sum) > $G/stage_manifest.txt
if [ "${1:-}" = install ]; then
  D1=/home/user/crypto/research/genus4_positive_control_20261116/validation
  D2=/home/user/crypto/research/genus4_rho_refs_20261116/validation
  if [ -e $D1 ] || [ -e $D2 ]; then echo "a validation/ directory already exists; not overwriting"; exit 1; fi
  cp -r $ST/s1 $D1; cp -r $ST/s2 $D2
  echo "installed $(date -u +%FT%TZ)"
fi
