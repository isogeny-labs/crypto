#!/usr/bin/env bash
# E35 G2.145 identity of the existing solvers: E33's pinned binary (old) and
# E35's (new) on p = 101 seed 2, both arms, RAYON_NUM_THREADS=4, for
# macaulay, chain, chain-sparsest (schoolbook) and chain, chain-sparsest-counted,
# chain-sparsest-inc, chain-sparsest-est (--karatsuba).  Compared as canonical
# JSON (jq -S -c) after dropping *_ms, ms and wall* fields; new records also
# compared with the stored E27 / E33 ones; and no new record may carry
# `trace_learning`.  Output: identity_existing_solvers.json.
#   usage: identity.sh <old quartic_pipeline_gls> <new quartic_pipeline_gls> <workdir>
set -eu
cd "$(dirname "$0")/../.."
old=$1 new=$2 w=$3
mkdir -p "$w"
export RAYON_NUM_THREADS=4
canon='walk(if type=="object" then with_entries(select(.key|test("(_ms$|^ms$|^wall)")|not)) else . end)'
digest() { jq -S -c "$canon" "$1" | sha256sum | cut -d' ' -f1; }
leaves() { jq "[$canon | paths(scalars)] | length" "$1"; }
out=experiments/39_chain_trace/identity_existing_solvers.json
rows=()
for cfg in macaulay chain chain-sparsest chain:k chain-sparsest-counted:k chain-sparsest-inc:k chain-sparsest-est:k; do
  solver=${cfg%%:*}; extra=""; tag=$solver
  [ "$cfg" != "$solver" ] && { extra="--karatsuba"; tag=${solver}_karatsuba; }
  sarg="--solver $solver"; [ "$solver" = macaulay ] && sarg=""
  for b in old new; do
    f=$w/${tag}_$b.json
    [ -s "$f" ] || taskset -c 1 "${!b}" --sizes 101 --first-seed 2 --seeds 1 --rho-runs 16 \
      --max-residuals 5000000 $sarg $extra --json "$f" >"$w/${tag}_$b.log" 2>&1
  done
  o=$(digest "$w/${tag}_old.json"); n=$(digest "$w/${tag}_new.json")
  stored=null
  case $tag in
    chain_karatsuba) s=experiments/32_walk_karatsuba/e2e_chain_karatsuba_p101s2.json ;;
    chain-sparsest-*_karatsuba) s=experiments/37_solver_b_pivot_count/e2e_${tag}_p101s2.json ;;
    *) s="" ;;
  esac
  [ -n "$s" ] && stored=\"$(digest "$s")\"
  tl=$(jq '[.. | objects | select(has("trace_learning"))] | length' "$w/${tag}_new.json")
  rows+=("{\"config\":\"$tag\",\"old\":\"$o\",\"new\":\"$n\",\"stored\":$stored,\"leaves\":$(leaves "$w/${tag}_new.json"),\"identical\":$([ "$o" = "$n" ] && echo true || echo false),\"trace_learning_fields_in_new\":$tl}")
done
printf '%s\n' "${rows[@]}" | jq -s --arg o "$(sha256sum "$old" | cut -c1-64)" --arg n "$(sha256sum "$new" | cut -c1-64)" \
  '{what: "E35 G2.145 identity of existing solvers: E33 pinned binary (old) vs E35 binary (new), p = 101 seed 2, both arms; canonical JSON without *_ms, ms, wall fields", old_sha256: $o, new_sha256: $n, runs: .}' >"$out"
cat "$out"
