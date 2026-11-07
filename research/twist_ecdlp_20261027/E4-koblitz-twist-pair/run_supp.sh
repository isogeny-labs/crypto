#!/bin/bash
# Exploratory supplement (not part of the frozen protocol); run after run.sh.
D=$(cd "$(dirname "$0")" && pwd)
cd $D && timeout ${TO:-300} gp -q -f scripts/supp_pc1_poly.gp </dev/null 2>&1 | grep -v "new maximum" > out/supp_pc1_poly.txt
sha256sum out/supp_pc1_poly.txt scripts/supp_pc1_poly.gp > out/SUPP.sha256
timeout ${TO:-300} gp -q -f scripts/supp_totals.gp </dev/null 2>&1 | grep -v "new maximum" > out/supp_totals.txt
sha256sum out/supp_totals.txt scripts/supp_totals.gp >> out/SUPP.sha256
bash $D/opcounts.sh >/dev/null
