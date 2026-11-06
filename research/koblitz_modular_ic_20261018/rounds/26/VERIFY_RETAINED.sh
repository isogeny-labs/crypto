#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
for a in char26-01 char26-02 cd26-01 transport26-01 sourceorbit26-01 char26-corrupt-01;do
 p="runs/$a";[ "$(cat "$p/exit.txt")" = 0 ]
 [ "$(sha256sum "$p/input.sing"|cut -d' ' -f1)" = "$(cut -d' ' -f1 "$p/source.sha256")" ]
 case "$a" in
 char26-02) marker=CHAR26_ALL_CHECKS_PASS ;;
 transport26-01) marker=TRANSPORT26_ALL_CHECKS_PASS ;;
 sourceorbit26-01) marker=SOURCEORBIT26_ALL_CHECKS_PASS ;;
 *) rg -q '^ *\?' "$p/native.log";echo "REJECTED $a";continue ;;
 esac
 ! rg -q '^ *\?' "$p/native.log";rg -qx "$marker" "$p/native.log";echo "ACCEPTED $a"
done
rg -q 'wrong delta character|delta does not negate' runs/char26-corrupt-01/native.log
rg -qx CHAR26_ALL_CHECKS_PASS runs/char26-corrupt-01/native.log
[ "$(rg -c '^SUBGROUP_NORM' runs/char26-02/native.log)" = 16 ]
[ "$(cat runs/lean-01/exit.txt)" != 0 ];[ "$(cat runs/lean-02/exit.txt)" = 0 ]
rg -q 'audited 17 theorems.*0 failing, 0 trusted' runs/lean-02/compile.log
[ "$(sha256sum lean/KoblitzPrym26.lean|cut -d' ' -f1)" = "$(awk 'NR==1{print $1}' runs/lean-02/compile.log)" ]
echo 'R26_RETAINED_RECORDS_PASS:6native,3accepted,1implementationfailure,1failedchart,1expectedreject;16named/17auditedLean'
