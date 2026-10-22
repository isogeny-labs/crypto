#!/bin/bash
# Appends the four Solinas curves of curves.jsonl to docs/curves/registry.json
# (AGENTS.md section 11), in the registry's sort position, with the fields the
# prior ladder's entries carry. From the repository root.
set -eu
R=research/pkm_p224_kummer_shape_20261022
reg=docs/curves/registry.json
tmp=$(mktemp)
cp $reg $tmp
for b in 20 24 26 28; do
  rec=$(jq -c 'select(.warmup==false and .arm=="rho-neg") | .workload.curve_spec + {slug: .workload.curve.slug, icv1: .workload.curve.icv1}' $R/sessions/s$b/records.jsonl | sort -u | jq -c --argjson sp "$(sed -n "$((b==20?1:b==24?2:b==26?3:4))p" $R/curves.jsonl | jq .p)" 'select(.p==$sp)')
  jq --argjson e "$rec" --arg b "$b" '
    def key($p): [($p|tostring|length), ($p|tostring)];
    ($e.p|tostring) as $ps
    | ($e.group_order) as $n
    | ($ps|tonumber) as $pn
    | ($e.icv1|split(":")) as $f
    | {slug: $e.slug, icv1: $e.icv1, family: "prime",
       params: {p: $ps, a: ($e.a|tostring), b: ($e.b|tostring)},
       model_json: ("{\"a\":\"" + ($e.a|tostring) + "\",\"b\":\"" + ($e.b|tostring) + "\",\"field\":\"fp-" + $ps + "\",\"form\":\"y^2=x^3+a*x+b\",\"p\":\"" + $ps + "\",\"v\":\"1\"}"),
       trace: ($pn + 1 - $n), order: ($n|tostring), j: $f[4], end: "unk", aliases: [], standard_names: [],
       sources: ["research/pkm_p224_kummer_shape_20261022/sessions/s\($b)/plan.json", "research/pkm_p224_kummer_shape_20261022/sessions/s\($b)/spec.json", "research/pkm_p224_kummer_shape_20261022/specs/SPEC-s\($b).json"],
       representations: [], ec1_unresolved: "no tracked record carries this model'"'"'s subgroup and generator"} as $new
    | if any(.curves[]; .slug == $e.slug) then . else
      (.curves | map(select(.family=="prime"))) as $primes
      | (.curves | to_entries | map(select(.value.family=="prime" and (key(.value.params.p) > key($ps))))[0].key) as $idx
      | .curves = (.curves[:$idx] + [$new] + .curves[$idx:]) end' $tmp > $tmp.new
  mv $tmp.new $tmp
done
jq --indent 1 . $tmp > $reg
rm -f $tmp
