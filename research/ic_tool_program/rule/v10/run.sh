#!/bin/bash
# The rule's comparison at v10, as PROTOCOL.md declares: rule v10 (the
# single-word factor-base selection) and its control v10c (main at the
# branch base), each with its in-process matched rho.  Manifests and pins
# first; then the six sizes in §23's order, v10 then v10c at each; then each
# run's reference check, claims and analysis; then the paired read-out and
# P1/P2.  Every step resumes what its run tree holds.  Local route: the
# record store (benchrec/S3) is not reachable from this container, so steps
# run directly and the log is kept beside the binaries.
# Usage: run.sh <scratch dir holding bin/>.
set -u
SP=${1:?scratch dir}
ROOT=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
LOG=$SP/rule10-run.log
P=$SP/bin/icprog
ISO=$SP/bin/isolated_bench
FIX=$SP/bin/koblitz_rho_fixture
declare -A IC=([v10]=$SP/bin/ic-v10 [v10c]=$SP/bin/ic-v10c)
declare -A C=([v10]=${V10_COMMIT:?the v10 build commit} [v10c]=ca1a767b1fc006966e0c56fc15110ceb79f10342)
FIXC=ca1a767b1fc006966e0c56fc15110ceb79f10342
log() { echo "== $(date -u +%T) $*" >> $LOG; }
cd $ROOT
step() {
  log "$1"
  shift
  "$@" >> $LOG 2>&1
  rc=$?; echo "   exit $rc" >> $LOG
  [ $rc = 0 ] || { log "stopped"; exit 1; }
}
pin() { # the pin must hold and every name agree, as `rule all` requires
  local out=$SP/$1-pin.json
  log "$1 pin"
  $P rule pin --comparison $1 --ic ${IC[$1]} --ic-commit ${C[$1]} --isolate $ISO > $out 2>> $LOG
  echo "   exit $?" >> $LOG
  jq -e '.held == true and .names_agree == true' $out > /dev/null || { log "the pin failed for $1; stopped"; exit 1; }
}
unset KIC_RHO_CANON
export RAYON_NUM_THREADS=1
for cmp in v10 v10c; do
  step "$cmp manifest" $P rule manifest --comparison $cmp --ic ${IC[$cmp]} --ic-commit ${C[$cmp]} --isolate $ISO
  pin $cmp
done
for size in 1,47 0,57 0,41 0,53 1,59 0,61; do
  for cmp in v10 v10c; do
    step "$cmp size $size" $P rule size --comparison $cmp --size $size --ic ${IC[$cmp]} --ic-commit ${C[$cmp]} --isolate $ISO
  done
done
for cmp in v10 v10c; do
  D=$ROOT/research/ic_tool_program/rule/$cmp
  step "$cmp reference" $P rule reference --comparison $cmp --fixture $FIX --fixture-commit $FIXC --isolate $ISO
  step "$cmp claims" $P rule claims --comparison $cmp
  log "$cmp analyse"
  $P rule analyse --comparison $cmp > $D/analysis.json 2>> $LOG; echo "   exit $?" >> $LOG
done
log "matched"
$P rule matched --comparison v10 > $ROOT/research/ic_tool_program/rule/v10/matched.json 2>> $LOG; echo "   exit $?" >> $LOG
log "p1p2"
bash $ROOT/research/ic_tool_program/rule/v10/p1p2.sh > $ROOT/research/ic_tool_program/rule/v10/p1p2.json 2>> $LOG; echo "   exit $?" >> $LOG
log "all done"
