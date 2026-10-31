#!/usr/bin/env bash
# Validate retained receipts; this does not launch a new native trial.
set -euo pipefail
cd "$(dirname "$0")"
for attempt in deck25-positive-01 deck25-positive-02 deck25-positive-03 deck25-positive-04 q25-registered-01 q25-registered-03 q25-registered-05 deck25-corrupt-01;do
 base="runs/$attempt"
 [ "$(cat "$base/exit.txt")" = 0 ]
 [ "$(sha256sum "$base/input.sing" | cut -d' ' -f1)" = "$(cut -d' ' -f1 "$base/source.sha256")" ]
 case "$attempt" in
  deck25-positive-04) marker=DECK25_ALL_CHECKS_PASS ;;
  q25-registered-05) marker=Q25_REGISTERED_QUOTIENT_ALL_CHECKS_PASS ;;
  *) rg -q '^ *\?' "$base/native.log";echo "REJECTED_NATIVE $attempt (native exit0 retained)";continue ;;
 esac
 ! rg -q '^ *\?' "$base/native.log"
 rg -qx "$marker" "$base/native.log"
 echo "ACCEPTED_NATIVE $attempt"
done
for attempt in q25-registered-02 q25-registered-04;do
 [ ! -e "runs/$attempt/native.log" ];rg -q 'cannot upload|no way to upload' "runs/$attempt/gate.log"
 echo "PRELAUNCH_GATE_REFUSAL $attempt"
done
[ "$(rg -c '^EXACT_SOURCE_PULLBACK_RESIDUAL$' runs/q25-registered-05/native.log)" = 13 ]
awk '/^EXACT_SOURCE_PULLBACK_RESIDUAL$/{getline;if($0!="0")exit 1;c++}END{if(c!=13)exit 1}' runs/q25-registered-05/native.log
rg -q '^1$' runs/deck25-corrupt-01/native.log
rg -q '^ *\? kernel coefficient not invariant$' runs/deck25-corrupt-01/native.log
rg -qx DECK25_ALL_CHECKS_PASS runs/deck25-corrupt-01/native.log
cmp quotient-coordinates.sing runs/deck25-positive-04/quotient-coordinates.dump
cmp registered-quotient.sing runs/q25-registered-05/registered-quotient.dump
[ "$(cat runs/lean-01/exit.txt)" != 0 ]
[ "$(cat runs/lean-02/exit.txt)" = 0 ]
rg -q 'audited 14 theorems.*0 failing, 0 trusted' runs/lean-02/compile.log
[ "$(sha256sum lean/KoblitzDeck25.lean | cut -d' ' -f1)" = "$(awk 'NR==1{print $1}' runs/lean-02/compile.log)" ]
echo 'R25_RETAINED_RECORDS_PASS:8native,2gate-only,1storage-prelaunch,14Lean'
