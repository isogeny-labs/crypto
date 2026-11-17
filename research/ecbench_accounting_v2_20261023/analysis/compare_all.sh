#!/bin/bash
# ecbench compare (paired bootstrap over workloads; --json, not --save: a session directory is never written to).
set -eu
cd "$(git rev-parse --show-toplevel)"
S=research/ecbench_accounting_v2_20261023; E=./target/release/ecbench; O=$S/analysis/compare
c() { # session A B
  $E compare --dir $S/sessions/$1 --a $2 --b $3 --json > $O/$1__$2__$3.json 2> $O/$1__$2__$3.err
  $E compare --dir $S/sessions/$1 --a $2 --b $3 > $O/$1__$2__$3.txt 2>&1
}
c claim-n13 rho-strong-frozen rho-strong-v2
c claim-n19 rho-strong-frozen rho-strong-v2
c strong-n37 rho-strong-frozen rho-strong-v2
c strong-n37 rho-frob rho-frob-v2
c claim-n13 rho-strong-v2 rho-strong-v2-aa
c claim-n19 rho-strong-v2 rho-strong-v2-aa
c strong-n37 rho-strong-v2 rho-strong-v2-aa
c j0-v2 rho-cubic rho-cubic-v2
c j0-v2 rho-neg-v2 rho-cubic-v2
c j0-v2 rho-plain rho-cubic-v2
for p in "bsgs bsgs-v2" "bsgs-il bsgs-il-v2" "bsgs-neg bsgs-neg-v2" "kangaroo kangaroo-v2" "rho-neg rho-neg-v2" "rho-neg-v2 rho-neg-v2-aa"; do set -- $p; c pairs-prime $1 $2; done
for p in "bsgs-neg bsgs-neg-v2" "claw claw-v2" "rho-frob rho-frob-v2" "rho-strong rho-strong-v2" "rho-frob-v2 rho-frob-v2-aa"; do set -- $p; c pairs-koblitz $1 $2; done
c pairs-large-prime lp lp-v2
# the same frozen arm across binaries (operations only; --b-dir)
for n in prime koblitz; do
  for a in $(jq -r '.arms[].name' $S/specs/frozen-${n}.json); do
    $E compare --dir $S/sessions/frozen-${n}-branch --b-dir $S/sessions/frozen-${n}-main --a $a --b $a --json > $O/xbin-frozen-${n}__${a}.json 2> $O/xbin-frozen-${n}__${a}.err || true
  done
done
for a in rho-neg rho-cubic rho-plain; do
  $E compare --dir $S/sessions/j0-frozen-branch --b-dir $S/sessions/j0-frozen-main --a $a --b $a --json > $O/xbin-j0__$a.json 2> $O/xbin-j0__$a.err || true
done
