#!/bin/sh
# gen_specs.sh : writes specs/c25519-k{24,28,32}-c{N}.json from the baseline specs (jq only).
# chunk c uses target_seed 2026110520+c, measurement.seed 2026110530+c, 1 target.
set -e
B=../curve25519_ic_baseline_20261102/specs
cd "$(dirname "$0")/.."
for k in 24 28 32; do
  case $k in 24) N=6 ;; 28) N=2 ;; 32) N=6 ;; esac
  for c in $(seq 0 $((N-1))); do
    out=specs/c25519-k$k-c$c.json
    jq --argjson k $k --argjson c $c '
      def pkm(name; t; g; tr): {name:name, role:"candidate", method:{id:"ic.pipeline", params:{factor_base:("prime-isogeny:t="+(t|tostring)), oracle:("pkm-semaev:m=2,mode=full,guess="+(g|tostring)+(if tr then ",trace=1" else "" end))}}};
      (.arms | map(select(.name=="rho-neg" or .name=="proj-a"))) as $gen
      | ($gen
         + (if $k<=28 then [pkm("pkm-iso-t10-g9-tr";10;9;true)] else [] end)
         + (if $k==24 and $c<2 then [pkm("pkm-iso-t6-g4";6;4;false)] else [] end)) as $arms
      | {schema:"ecbench.spec/v1",
         label:("Curve25519 k="+($k|tostring)+" algebraic scale-up, chunk "+($c|tostring)),
         description:"research/curve25519_algebraic_ecbench_scale_20261105/PROTOCOL.md (frozen before data). Public synthetic curve and target; no security claim.",
         arms:$arms,
         measurement:{isolation_required:"L0", order:"alternate_measured", rounds:1, warmup:0, seed:(2026110530+$c), timeout_seconds:(if $k==32 then 1800 else 3000 end)},
         workloads:{curves:[.workloads.curves[0]], target_kind:"public", target_seed:(2026110520+$c), targets_per_curve:1}}' \
      $B/c25519-k$k.json > $out
  done
done
sha256sum specs/*.json > specs.sha256
