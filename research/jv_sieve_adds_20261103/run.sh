#!/usr/bin/env bash
# Frozen driver for PROTOCOL.md. Run from the repo root: bash research/jv_sieve_adds_20261103/run.sh
# Idempotent: finished (arm, p) jsons with 16 records are skipped.
set -uo pipefail
cd "$(dirname "$0")/../.."
D=research/jv_sieve_adds_20261103; OUT=$D/results
S=${JV_BIN_DIR:-/tmp/claude-0/-home-user/e05ef163-36d2-5181-b9b7-869571ded707/scratchpad/adds}
BP=$S/jv_cover_P   # the code before the change (the binary of the round-3 tree)
BN=$S/jv_cover_N   # the code with the change
FIRST=17; N=16
mkdir -p $OUT
[ -f $OUT/FROZEN.sha ] || { echo "results/FROZEN.sha missing: freeze first" >&2; exit 2; }
(cd $D && sha256sum -c --quiet <(grep -E ' (PROTOCOL.md|run.sh)$' results/FROZEN.sha)) || { echo "frozen files changed" >&2; exit 2; }
for b in P N; do
  f=$S/jv_cover_$b
  [ "$(sha256sum $f | cut -d' ' -f1)" = "$(grep " jv_cover_$b\$" $OUT/FROZEN.sha | cut -d' ' -f1)" ] || { echo "binary $b differs from FROZEN.sha" >&2; exit 2; }
done
done_n() { [ -s $OUT/$1.json ] && [ "$(grep -c '^p=' $OUT/$1.log)" -ge $2 ]; }
arm() { # label binary n_seeds env_assignments...
  label=$1; bin=$2; ns=$3; shift 3
  done_n $label $ns && return 0
  s0=$(date +%s)
  env -u JV_FB_LINE -u JV_FB_LINE_MODE -u JV_ADDS_INC -u JV_ADDS_SHARE JV_LIN=sparse "$@" $bin --exp sieve --stop 64 --sizes $P \
     --first-seed $FIRST --seeds $ns --rho-runs 0 --json $OUT/$label.json > $OUT/$label.log 2>&1
  echo "$label rc=$? wall=$(( $(date +%s)-s0 ))s" >> $OUT/walls.txt
}
UENV="JV_FB_LINE=32768 JV_FB_LINE_MODE=table"
for P in 1009 1511 1823; do
  arm P_$P  $BP $N
  arm N0_$P $BN $N JV_ADDS_INC=0 JV_ADDS_SHARE=0
  arm N_$P  $BN $N
  arm U_$P  $BP $N $UENV
  arm NU_$P $BN $N $UENV
done
for P in 1009 1511 1823; do
  arm NI_$P $BN $N JV_ADDS_SHARE=0
  arm NS_$P $BN $N JV_ADDS_INC=0
done
