#!/usr/bin/env bash
# PROTOCOL.md's frozen analysis plan, all by ecbench, over the committed
# hold-out sessions of research/bitop_ladder_20261019.
#
#   ECBENCH=/path/to/release/ecbench research/ic_acct2_setup_20261021/run.sh
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
bin="${ECBENCH:-$root/target/release/ecbench}"
out="$here/analysis"
ses="research/bitop_ladder_20261019/sessions"
mkdir -p "$out/bounds" "$out/amortize"
cd "$root"

dirs() { for b in $(seq "$1" "$2"); do printf -- '--dir %s ' "$ses/holdout-P$b"; done; }
rule="research/bitop_ladder_20261019/PROTOCOL.md Phase A: base per size from params.json (train panel, bitop/v1)"

for arm in rho-neg bsgs-neg kangaroo ic-pair-rule; do
  extra=()
  [ "$arm" = ic-pair-rule ] && extra=(--param-rule "$rule")
  for cost in total per-target search; do
    for range in 16-22 23-32 16-26 33-40; do
      lo=${range%-*} hi=${range#*-} name="$arm-$cost-$range"
      # shellcheck disable=SC2046
      "$bin" bound fit $(dirs "$lo" "$hi") --arm "$arm" --unit bitop/v1 --cost "$cost" "${extra[@]}" \
        --label "ic_acct2 setup study: $arm, $cost, $lo-$hi bits" \
        --out "$out/bounds/$name.json" >"$out/bounds/$name.log" 2>&1 \
        || echo "FIT REFUSED $name (see $out/bounds/$name.log)"
    done
  done
done

# shellcheck disable=SC2046
"$bin" amortize $(dirs 16 40) --arm ic-pair-rule --reference rho-neg --stored-split \
  >"$out/amortize/ic-pair-rule-stored-split.json"
# shellcheck disable=SC2046
"$bin" amortize $(dirs 16 40) --arm ic-pair-rule --reference rho-neg \
  >"$out/amortize/ic-pair-rule-target-aware.json"
echo "analysis written to $out"
