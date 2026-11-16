#!/usr/bin/env bash
# Frozen driver for PROTOCOL.md. Run from the repo root: bash research/jv_sieve_fbline_20261102/run.sh
# Idempotent: finished (arm, p) jsons with 16 (or 1) records are skipped.
set -uo pipefail
cd "$(dirname "$0")/../.."
D=research/jv_sieve_fbline_20261102; OUT=$D/results
BIN=${JV_BIN:-./target/release/examples/jv_cover}
FIRST=17; N=16
mkdir -p $OUT
[ -f $OUT/FROZEN.sha ] || { echo "results/FROZEN.sha missing: freeze first" >&2; exit 2; }
(cd $OUT/.. && sha256sum -c --quiet <(grep -E ' (PROTOCOL.md|results/rule.txt)$' results/FROZEN.sha)) || { echo "frozen files changed" >&2; exit 2; }
[ "$(sha256sum $BIN | cut -d' ' -f1)" = "$(grep ' jv_cover$' $OUT/FROZEN.sha | cut -d' ' -f1)" ] || { echo "binary differs from FROZEN.sha" >&2; exit 2; }
done_n() { [ -s $OUT/$1.json ] && [ "$(grep -c '^p=' $OUT/$1.log)" -ge $2 ]; }
arm() { # label n_seeds env_assignments...
  label=$1; ns=$2; shift 2
  done_n $label $ns && return 0
  s0=$(date +%s)
  env -u JV_FB_LINE -u JV_FB_LINE_MODE JV_LIN=sparse "$@" $BIN --exp sieve --stop 64 --sizes $P \
     --first-seed $FIRST --seeds $ns --rho-runs 0 --json $OUT/$label.json > $OUT/$label.log 2>&1
  echo "$label rc=$? wall=$(( $(date +%s)-s0 ))s" >> $OUT/walls.txt
}
for P in 1009 1511 1823; do
  arm P_$P $N
  arm H_$P $N JV_FB_LINE=4096 JV_FB_LINE_MODE=horner
  arm T_$P $N JV_FB_LINE=4294967296 JV_FB_LINE_MODE=table
  arm U_$P $N JV_FB_LINE=32768 JV_FB_LINE_MODE=table
  arm c2h_$P 1 JV_FB_LINE=1 JV_FB_LINE_MODE=horner
  arm c2t_$P 1 JV_FB_LINE=1 JV_FB_LINE_MODE=table
  arm c3_$P 1 JV_FB_LINE=4096 JV_FB_LINE_MODE=table
done
