#!/usr/bin/env bash
# E33 validation: the sample drawn from nonce.json (validator-owned).
# key(label) = sha256("<nonce>:<label>"); curves are ordered by key ascending.
set -eu
N=$(sed -n 's/.*"nonce": "\([0-9a-f]*\)".*/\1/p' "$(dirname "$0")/nonce.json")
k() { printf '%s:%s' "$N" "$1" | sha256sum | cut -c1-64; }
curves="p101s1 p101s2 p149s1 p149s2 p197s1 p197s2 p269s1 p269s2 p397s1 p397s2"
order() { for c in $curves; do [ "$c" = "${2:-}" ] && continue; echo "$(k "$1:$c") $c"; done | sort | awk '{print $2}' | tr '\n' ' '; }
echo "{"
echo " \"rule\": \"key(label) = sha256('<nonce>:<label>'); curves sorted by key ascending\","
echo " \"e2e_order\": \"$(order e2e)\","
echo " \"identity_order_excluding_p101s2\": \"$(order identity p101s2)\","
echo " \"controls_order\": \"$(order controls)\","
for c in $curves; do echo " \"controls_rng_$c\": $((16#$(k "controls-rng:$c" | cut -c1-15)))," ; done
echo " \"nonce\": \"$N\""
echo "}"
