#!/usr/bin/env bash
# E40 replay comparison (validator).  For every replayed output: the committed
# record it must equal (leaf by leaf, timing keys *_ms/ms/*wall* skipped, by the
# validator's own comparator `validate_e40 compare`), plus, for the identity
# configurations, identity.sh's canonical hash (jq -S -c without timing keys,
# sha256) against the reference hash E40 recorded and E40's own run.
#   usage: compare.sh <validate_e40 binary> <replay out dir> <snapshot root>
set -u
v=$1; out=$2; snap=$3
e40=$snap/experiments/47_macaulay_entries
e37=$snap/experiments/44_pivot_rules_confirm/e2e
F='walk(if type=="object" then with_entries(select((.key|test("_ms$|^ms$|wall"))|not)) else . end)'
canon() { jq -S -c "$F" "$1" | sha256sum | cut -d' ' -f1; }
row() { # name committed-path
  local name=$1 ref=$2
  if [ ! -s "$out/$name.json" ]; then echo "$name MISSING"; return; fi
  set -- $("$v" compare "$ref" "$out/$name.json" | head -1)
  echo "$name ${ref#$snap/} leaves=$1 differing=$2 committed_sha256=$3 replay_sha256=$4"
}
for f in gls_chain-profile_p101s1 gls_chain-profile_p397s1 gls_chain-profile_p397s2 \
         gls_chain-counted_E40E0002_p397s1 gls_chain-profile_E40E0002_p397s1 \
         e24_chain-profile_p269s1 e24_chain-profile_p397s2 e24_chain-profile_p521s2; do
  row "$f" "$e40/e2e/$f.json"
done
row gls_macaulay_p101s1 "$e37/gls_macaulay_p101s1.json"
row e24_macaulay_p521s1 "$e37/e24_macaulay_p521s1.json"
i=0
for c in macaulay chain chainsparsest chain_karatsuba gls_counted_e37 gls_fifo_e37 e24_counted_e37; do
  row "id_$c" "$e40/identity/$c.json"
  # identity_existing_solvers.json lists the seven configurations in this order
  cfg=$(jq -r ".runs[$i].config" "$e40/identity_existing_solvers.json")
  rec=$(jq -r ".runs[$i].reference_hash" "$e40/identity_existing_solvers.json")
  i=$((i + 1))
  [ -s "$out/id_$c.json" ] || continue
  r=$(canon "$out/id_$c.json")
  echo "  canonical id_$c ($cfg): replay=$r e40_run=$(canon "$e40/identity/$c.json") reference=$rec $([ "$r" = "$rec" ] && echo MATCH || echo MISMATCH)"
done
echo "E37 stored records (the references of the last three configurations), canonical hashes recomputed here:"
for f in gls_chain-counted_p101s2 gls_chain-fifo_p101s2 e24_chain-counted_p101s1; do
  echo "  $f $(canon "$e37/$f.json")"
done
for c in checks_v2:checks checks_v1:checks_v1; do
  r=${c%%:*}; ref=${c##*:}
  if [ -s "$out/$r.json" ]; then
    echo "$r vs $ref.json: $(cmp -s "$out/$r.json" "$e40/$ref.json" && echo byte-identical || echo DIFFERENT) $(sha256sum < "$out/$r.json" | cut -c1-64)"
  else echo "$r MISSING"; fi
done
if [ -s "$out/stage_a.json" ]; then
  echo "stage_a vs stage_a.json: $(cmp -s "$out/stage_a.json" "$e40/stage_a.json" && echo byte-identical || echo DIFFERENT) $(sha256sum < "$out/stage_a.json" | cut -c1-64) (recorded 7e23c58f3004d861b073847212db7308d2a500f91c42adedfd2ddaca283e03c8)"
else echo "stage_a MISSING"; fi
