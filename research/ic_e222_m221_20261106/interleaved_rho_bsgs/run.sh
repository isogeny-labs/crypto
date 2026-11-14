#!/bin/bash
# Interleaved rho vs BSGS (PROTOCOL.md). Writes rows/<arm>_b<bits>_T<T>_r<round>.jsonl.
set -u
cd "$(dirname "$0")/../../.."
rho=target/release/examples/prime_fast_j0_rho_multi
bsgs=target/release/examples/prime_fast_j0_bsgs_multi
out=research/ic_e222_m221_20261106/interleaved_rho_bsgs/rows
perms=("rho bsgs rho_aa" "rho rho_aa bsgs" "bsgs rho rho_aa" "bsgs rho_aa rho" "rho_aa rho bsgs" "rho_aa bsgs rho")
# Cells: arguments as "bits T" strings; default the original nine.
if [ $# -gt 0 ]; then cells=("$@"); else cells=("26 20" "26 200" "26 1000" "28 200" "28 1000" "28 3000" "30 200" "30 1000" "30 3000"); fi
for cell in "${cells[@]}"; do
  set -- $cell
  for r in 0 1 2 3 4 5; do
    for arm in ${perms[$r]}; do
      f=$out/${arm}_b$1_T$2_r$r.jsonl
      case $arm in
        rho|rho_aa) DPRULE=ks DPDIV=1.5 INST0=$((100 + r)) $rho $f $1 $2 2 > /dev/null || echo "FAIL $arm $1 $2 r$r" ;;
        bsgs) MC=1 INST0=$((100 + r)) $bsgs $f $1 $2 2 > /dev/null || echo "FAIL $arm $1 $2 r$r" ;;
      esac
    done
  done
  echo "cell $1 $2 done"
done
echo ALLDONE
