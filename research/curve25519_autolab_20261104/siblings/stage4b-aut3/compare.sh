#!/usr/bin/env bash
# Stage 4b: paired comparisons at w = 0, 0.1, 1 (ecbench compare --save); thin orchestration only.
set -e
D=research/curve25519_autolab_20261104/siblings/stage4b-aut3
PAIRS="tw2cj-lazy:tw2cj-lazy-aut3 sib8-p512:sib8-p512-aut3 rho-cubic:sib8-p512-aut3 rho-cubic:tw2cj-lazy-aut3 rho-cubic:sib8-p512 rho-neg:rho-cubic rho-neg:rho-neg-aa"
for b in 34 37 40; do for pr in $PAIRS; do a=${pr%%:*}; c=${pr##*:}; for w in 0 0.1 1; do
  echo "== j0b7-$b $c / $a w=$w"
  ./target/release/ecbench compare --dir $D/sessions/j0b7-$b --a $a --b $c --resamples 4000 --seed 20261112 --weight $w --save | head -2
done; done; done
# bit-op ratios (native: ecbench compare --unit bitop/v1)
for b in 34 37 40; do for pr in tw2cj-lazy:tw2cj-lazy-aut3 sib8-p512:sib8-p512-aut3 tw2cj-lazy:sib8-p512-aut3 rho-cubic:sib8-p512-aut3 rho-neg:rho-cubic; do a=${pr%%:*}; c=${pr##*:}
  echo "== bitop j0b7-$b $c / $a"
  ./target/release/ecbench compare --dir $D/sessions/j0b7-$b --a $a --b $c --resamples 4000 --seed 20261112 --unit bitop/v1 --save | head -1
done; done
# slope in r (native: ecbench trend, fit free_r) over the saved w = 0 comparisons
mkdir -p $D/trend
for pr in rho-cubic:sib8-p512-aut3 tw2cj-lazy:tw2cj-lazy-aut3 sib8-p512:sib8-p512-aut3 rho-neg:rho-cubic; do a=${pr%%:*}; c=${pr##*:}
  files=""
  for b in 34 37 40; do
    ./target/release/ecbench compare --dir $D/sessions/j0b7-$b --a $a --b $c --resamples 4000 --seed 20261112 --weight 0 --json > "$D/trend/${a}__${c}-$b.json"
    files="$files $D/trend/${a}__${c}-$b.json"
  done
  ./target/release/ecbench trend --compare $files > "$D/trend/${a}__${c}.trend.json"
  echo "== trend $c / $a: $(grep -o '"beta",[^]]*\]' "$D/trend/${a}__${c}.trend.json" | head -1)"
done
echo COMPARE-DONE
