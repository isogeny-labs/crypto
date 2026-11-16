#!/usr/bin/env bash
# E39 validator: the list of (replayed output, committed record) pairs for
# `validate_e39 cert`.  Paths shown in the certificate are relative to the
# evidence directory research/e39_pivot_rules_confirm3_20261102/.
#   usage: cert_list.sh <replay out dir> <snapshot root>
set -u
out=$1; snap=$2
e39=experiments/46_pivot_rules_confirm3
e37=experiments/44_pivot_rules_confirm
e38=experiments/45_pivot_rules_confirm2
line() { printf '%s\t%s\t%s\t%s\t%s\n' "$1" "$2" "$snap/$3" "$out/$4" "../../$3"; }
for f in "$out"/e2e_*.json; do
  n=$(basename "$f" .json); n=${n#e2e_}
  line "$n" "end to end, salt 0xE39E0001 (nonce-selected)" "$e39/e2e/$n.json" "e2e_$n.json"
done
line pilot "pilot P1 (e39_checks --salt E39D0001 --calls 8), full" "$e39/pilot.json" pilot.json
line checks "main control run (e39_checks --salt E39C0001 --calls 32), full" "$e39/checks.json" checks.json
for c in macaulay chain chainsparsest chain_karatsuba est_karatsuba chain-fifo_karatsuba chain-static_karatsuba \
         chain-fifo-static_karatsuba v4_default e37salt_gls_chain-counted e37salt_gls_chain-fifo e37salt_gls_chain-static \
         e37salt_gls_chain-fifo-static e37salt_gls_chain-sparsest-est e37salt_e24_chain-fifo e37_checks e38_pilot e38_pilot_p2; do
  line "identity/$c" "G2.191 identity configuration (E39's own run of it)" "$e39/identity/$c.json" "id_$c.json"
done
line walk_e24 "matched rho, E24 rows: e37_walk_e24 at the E39 commit vs E37's walk_e24.json" "$e37/walk_e24.json" walk_e24.json
[ -s "$out/stage_w.json" ] && line stage_w "matched rho, GLS rows: e27_walk at the E39 commit vs E27's stage_w.json" experiments/32_walk_karatsuba/stage_w.json stage_w.json
true
