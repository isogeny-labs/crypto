#!/bin/bash
# The rule's comparison at v8, as PROTOCOL.md declares: for rule v8 (matched
# rho) and then rule v8c (the same binary, KIC_RHO_CANON=scalar, rho as at
# rule v7): the manifest, every row (with the pin), the reference check, the
# claims and the analysis; then the paired read-out.  Every step resumes what
# its run tree holds.  Usage: run.sh <scratch dir holding bin/>.
set -u
SP=${1:?scratch dir}
ROOT=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
LOG=$SP/rule8-run.log
P=$SP/bin/icprog
ISO=$SP/bin/isolated_bench
IC=$SP/bin/ic
FIX=$SP/bin/koblitz_rho_fixture
C=370e19193f6f30bf04ca2e82a2c3626932dc069e
log() { echo "== $(date -u +%T) $*" >> $LOG; }
cd $ROOT
step() {
  log "$1"
  shift
  "$@" >> $LOG 2>&1
  rc=$?; echo "   exit $rc" >> $LOG
  [ $rc = 0 ] || { log "stopped"; exit 1; }
}
for cmp in v8 v8c; do
  D=$ROOT/research/ic_tool_program/rule/$cmp
  if [ $cmp = v8c ]; then export KIC_RHO_CANON=scalar; else unset KIC_RHO_CANON; fi
  step "$cmp manifest" $P rule manifest --comparison $cmp --ic $IC --ic-commit $C --isolate $ISO
  step "$cmp all" $P rule all --comparison $cmp --ic $IC --ic-commit $C --isolate $ISO
  step "$cmp reference" $P rule reference --comparison $cmp --fixture $FIX --fixture-commit $C --isolate $ISO
  step "$cmp claims" $P rule claims --comparison $cmp
  log "$cmp analyse"
  $P rule analyse --comparison $cmp > $D/analysis.json 2>> $LOG; echo "   exit $?" >> $LOG
done
unset KIC_RHO_CANON
log "matched"
$P rule matched --comparison v8 > $ROOT/research/ic_tool_program/rule/v8/matched.json 2>> $LOG; echo "   exit $?" >> $LOG
log "all done"
