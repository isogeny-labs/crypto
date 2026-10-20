#!/bin/sh
# Table A replay. For each size: 3 runs chosen from the committed seed among the R1 rows.
# (1) independent certificate check (certverify, shares no code with the repository) of the IC and rho scalars;
# (2) native re-execution `ic price --single-target` on the run's own params file and rho seed; the stored
#     counts (every counter the archived record carries), rho counts, scalars and certificates must be equal.
# Usage: replay_A.sh <certverify-binary> <ic-binary> <outdir>      (from the repo root; read-only on the tree)
CV="$1"; IC="$2"; OUT="$3"; mkdir -p "$OUT"
base=research/ic_tool_program/rule/v3/runs
here=research/validation_sweep_20261019/A-leaderboard/scripts
for d in $base/k*n*; do
  kn=$(basename "$d")
  curve=$(jq -r .curve "$d/T01-R1.price.json")
  for f in $(ls "$d" | sed -n 's/^\(T[0-9]*-R1\)\.price\.json$/\1/p' | $here/sample.sh "A:$curve" 3); do
    p="$d/$f.price.json"; t=${f%-R1}
    # (1) independent certificates
    for arm in ic rho; do
      jq -r --arg id "$kn/$f/$arm" ".certificates.$arm | [\$id,\"gf2m\",((.curve.degree|tostring)+\":\"+(.curve.irreducible_low_terms|map(tostring)|join(\",\"))),(.curve.curve_a|tostring),\"1\",.curve.generator[0],.curve.generator[1],.target[0],.target[1],.scalar,.curve.subgroup_order]|@tsv" "$p"
    done | "$CV"
    # (2) native re-execution
    seed=$(jq -r '.rho_policy.seed' "$p")
    out="$OUT/$curve-$f.replay.json"; rm -f "$out"
    RAYON_NUM_THREADS=1 "$IC" price --params "$d/$t.params.json" --single-target --rho-seed "$seed" --json --out "$out" >/dev/null 2>"$OUT/$curve-$f.replay.err"
    jq -n --arg id "$kn/$f" --slurpfile n "$out" --slurpfile o "$p" '
      def sub($a;$b): ($a|tojson) as $x | true;  # placeholder, replaced below
      ($o[0].counts.pass|to_entries) as $oe |
      { run:$id,
        verified_new:$n[0].all_verified,
        recovered_equal:($n[0].counts.recovered==$o[0].counts.recovered),
        rho_counts_equal:($n[0].rho_counts==$o[0].rho_counts),
        certificates_equal:($n[0].certificates==$o[0].certificates),
        # every counter the archived record has is equal; counters only the new binary emits are listed
        archived_counters_equal:( [ $o[0].counts.pass | paths(scalars) as $p | ( ($o[0].counts.pass|getpath($p)) == ($n[0].counts.pass|getpath($p)) ) ] | all ),
        new_binary_extra_counters:( [ $n[0].counts.pass | paths(scalars) | select(. as $p | ($o[0].counts.pass|getpath($p)) == null) | join(".") ] | unique ) }'
  done
done
