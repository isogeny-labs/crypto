#!/bin/bash
# The rule's comparison at v9, as PROTOCOL.md declares: rule v9 (main's ic)
# and its control v9c (rule v8's ic rebuilt), each with its in-process
# matched rho.  Manifests and pins first; then the six sizes in §23's order,
# v9 then v9c at each; then each run's reference check, claims and analysis;
# then the paired read-out.  Every step resumes what its run tree holds.
# Usage: run.sh <scratch dir holding bin/> <bench record>.
set -u
SP=${1:?scratch dir}
REC=${2:?bench record}
ROOT=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
LOG=$SP/rule9-run.log
P=$SP/bin/icprog
ISO=$SP/bin/isolated_bench
FIX=$SP/bin/koblitz_rho_fixture
declare -A IC=([v9]=$SP/bin/ic-v9 [v9c]=$SP/bin/ic-v9c)
declare -A C=([v9]=f465f2fb1b037a57fa1d923ac8eea27ab9a603e2 [v9c]=370e19193f6f30bf04ca2e82a2c3626932dc069e)
FIXC=f465f2fb1b037a57fa1d923ac8eea27ab9a603e2
log() { echo "== $(date -u +%T) $*" >> $LOG; }
cd $ROOT
step() {
  log "$1"
  shift
  python3 tools/benchrec.py run "$REC" -- "$@" >> $LOG 2>&1
  rc=$?; echo "   exit $rc" >> $LOG
  [ $rc = 0 ] || { log "stopped"; exit 1; }
}
pin() { # cmp: the pin must hold and every name agree, as `rule all` requires
  local out=$SP/$1-pin.json
  log "$1 pin"
  python3 tools/benchrec.py run "$REC" -- $P rule pin --comparison $1 --ic ${IC[$1]} --ic-commit ${C[$1]} --isolate $ISO > $out 2>> $LOG
  echo "   exit $?" >> $LOG
  jq -e '.held == true and .names_agree == true' $out > /dev/null || { log "the pin failed for $1; stopped"; exit 1; }
}
unset KIC_RHO_CANON
for cmp in v9 v9c; do
  step "$cmp manifest" $P rule manifest --comparison $cmp --ic ${IC[$cmp]} --ic-commit ${C[$cmp]} --isolate $ISO
  pin $cmp
done
for size in 1,47 0,57 0,41 0,53 1,59 0,61; do
  for cmp in v9 v9c; do
    step "$cmp size $size" $P rule size --comparison $cmp --size $size --ic ${IC[$cmp]} --ic-commit ${C[$cmp]} --isolate $ISO
  done
done
for cmp in v9 v9c; do
  D=$ROOT/research/ic_tool_program/rule/$cmp
  step "$cmp reference" $P rule reference --comparison $cmp --fixture $FIX --fixture-commit $FIXC --isolate $ISO
  step "$cmp claims" $P rule claims --comparison $cmp
  log "$cmp analyse"
  $P rule analyse --comparison $cmp > $D/analysis.json 2>> $LOG; echo "   exit $?" >> $LOG
done
log "matched"
$P rule matched --comparison v9 > $ROOT/research/ic_tool_program/rule/v9/matched.json 2>> $LOG; echo "   exit $?" >> $LOG
log "all done"
