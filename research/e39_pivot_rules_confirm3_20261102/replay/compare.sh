#!/usr/bin/env bash
# E39 replay comparison (validator).  For every replayed output: the committed
# record it must equal, leaf by leaf with timing keys (*_ms, ms, *wall*)
# skipped, by the validator's own comparator (`validate_e39 compare`), and raw
# bytes where the record has no timing fields.  For the identity
# configurations also the canonical hash of G2.191 (jq -S -c without timing
# keys, sha256) against the reference record E39 compared with (E37's / E38's
# committed records, hashed here) and against E39's own identity output.
#   usage: compare.sh <validate_e39 binary> <replay out dir> <snapshot root>
set -u
v=$1; out=$2; snap=$3
e39=$snap/experiments/46_pivot_rules_confirm3
e37=$snap/experiments/44_pivot_rules_confirm
e38=$snap/experiments/45_pivot_rules_confirm2
F='walk(if type=="object" then with_entries(select((.key|test("_ms$|^ms$|wall"))|not)) else . end)'
canon() { jq -S -c "$F" "$1" | sha256sum | cut -d' ' -f1; }
raw() { sha256sum < "$1" | cut -c1-64; }
row() { # replay-name committed-path
  local name=$1 ref=$2
  if [ ! -s "$out/$name.json" ]; then echo "$name MISSING"; return; fi
  set -- $("$v" compare "$ref" "$out/$name.json" | head -1)
  echo "$name ${ref#$snap/} leaves=$1 differing=$2 committed_sha256=$3 replay_sha256=$4 raw_bytes=$([ "$3" = "$4" ] && echo identical || echo differ)"
}
echo "== end to end (nonce-selected)"
for f in "$out"/e2e_*.json; do
  [ -e "$f" ] || continue
  n=$(basename "$f" .json); row "$n" "$e39/e2e/${n#e2e_}.json"
done
echo "== controls and Stage A"
row pilot "$e39/pilot.json"
row checks "$e39/checks.json"
if [ -s "$out/stage_a.json" ]; then
  echo "stage_a replay_sha256=$(raw "$out/stage_a.json") recorded=$(cut -c1-64 "$e39/stage_a.json.sha256") $([ "$(raw "$out/stage_a.json")" = "$(cut -c1-64 "$e39/stage_a.json.sha256")" ] && echo MATCH || echo MISMATCH) exit=$(cat "$out/stage_a.exit" 2>/dev/null)"
else echo "stage_a MISSING"; fi
echo "== identity configurations (G2.191): replay vs E39 identity output (leaves), canonical hash vs reference record"
ref_of() {
  case $1 in
    macaulay|chain|chainsparsest|chain_karatsuba|est_karatsuba|chain-fifo_karatsuba|chain-static_karatsuba|chain-fifo-static_karatsuba|v4_default) echo "$e37/identity/$1.json" ;;
    e37salt_gls_*) echo "$e37/e2e/gls_${1#e37salt_gls_}_p101s2.json" ;;
    e37salt_e24_chain-fifo) echo "$e37/e2e/e24_chain-fifo_p101s2.json" ;;
    e37_checks) echo "$e37/checks.json" ;;
    e38_pilot) echo "$e38/pilot.json" ;;
    e38_pilot_p2) echo "$e38/pilot_p2.json" ;;
  esac
}
for c in macaulay chain chainsparsest chain_karatsuba est_karatsuba chain-fifo_karatsuba chain-static_karatsuba \
         chain-fifo-static_karatsuba v4_default e37salt_gls_chain-counted e37salt_gls_chain-fifo e37salt_gls_chain-static \
         e37salt_gls_chain-fifo-static e37salt_gls_chain-sparsest-est e37salt_e24_chain-fifo e37_checks e38_pilot e38_pilot_p2; do
  row "id_$c" "$e39/identity/$c.json"
  [ -s "$out/id_$c.json" ] || continue
  r=$(canon "$out/id_$c.json"); ref=$(ref_of "$c"); rh=$(canon "$ref")
  echo "  canonical id_$c: replay=$r reference(${ref#$snap/})=$rh $([ "$r" = "$rh" ] && echo MATCH || echo MISMATCH)"
done
for c in chain_karatsuba:e36bin_chain_karatsuba chain-fifo_karatsuba:e36bin_chain-fifo_karatsuba; do
  a=${c%%:*}; b=${c##*:}
  [ -s "$out/id_$a.json" ] || continue
  echo "  canonical id_$a vs E36 pinned-binary record $b: $([ "$(canon "$out/id_$a.json")" = "$(canon "$e37/identity/$b.json")" ] && echo MATCH || echo MISMATCH)"
done
echo "== walk (iii) references"
if [ -s "$out/walk_e24.json" ]; then
  row walk_e24 "$e37/walk_e24.json"
  echo "  canonical walk_e24 vs E37 walk_e24.json: $([ "$(canon "$out/walk_e24.json")" = "$(canon "$e37/walk_e24.json")" ] && echo MATCH || echo MISMATCH)"
fi
[ -s "$out/stage_w.json" ] && row stage_w "$snap/experiments/32_walk_karatsuba/stage_w.json"
true
