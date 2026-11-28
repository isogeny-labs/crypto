#!/usr/bin/env bash
# Paired admission-rule panel on the library pair table (crypto #5202): for each salt,
# ADMIT=fb2 then ADMIT=all, single thread, window 8/256, to verified recovery.
set -u
R=$(cd "$(dirname "$0")/.." && pwd)
X=${X:-$R/../../target/release/examples/pair_table_rank_rate}
D=$R/evidence/${PANEL:-panel-01}
mkdir -p $D
for salt in $(seq 1 ${SALTS:-40}); do
  for rule in fb2 all; do
    ADMIT=$rule $X pairtable 8 $salt 1 > $D/s$salt-$rule.json || echo "fail $salt $rule"
  done
done
echo PANEL-DONE
