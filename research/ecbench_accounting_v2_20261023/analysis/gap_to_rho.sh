#!/bin/bash
# Distance remaining (AGENTS.md success criterion): each ic.pipeline arm of the committed claim specs against the
# v2 matched rho arms, at the three band weights (ecbench band --json mean_s[w]); ratio = IC S / rho S.
cd "$(dirname "$0")"
echo -e "session\tic_arm\trho_arm\tw0\tw0.1\tw1"
for s in claim-n13 claim-n19; do
  jq -r --arg s $s '.rows as $r | ($r[]|select(.arm|startswith("ic-"))) as $i | ($r[]|select(.arm=="rho-strong-v2" or .arm=="rho-frob-v2" or .arm=="rho-strong-frozen")) as $h
    | [$s, $i.arm, $h.arm, ($i.mean_s[0]/$h.mean_s[0]), ($i.mean_s[1]/$h.mean_s[1]), ($i.mean_s[2]/$h.mean_s[2])] | @tsv' band/$s.json
done
