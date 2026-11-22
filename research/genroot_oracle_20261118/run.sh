#!/usr/bin/env bash
# Matched end-to-end runs of the `genroot` oracle against the existing
# oracles on the same instances, factor bases, targets and rho reference.
# Usage: run.sh IC_BINARY OUT_DIR
set -euo pipefail
IC=${1:?ic binary}
OUT=${2:?output dir}
mkdir -p "$OUT"
REPEATS=${REPEATS:-3}
RHO=${RHO:-16}

run() {
  local tag=$1; shift
  local f="$OUT/$tag.json"
  if [[ -s $f ]]; then echo "skip $tag"; return; fi
  echo "run  $tag: $*"
  "$IC" bench --repeats "$REPEATS" --rho-runs "$RHO" "$@" > "$f.tmp" 2> "$OUT/$tag.log" && mv "$f.tmp" "$f" \
    || { echo "FAILED $tag (see $OUT/$tag.log)"; rm -f "$f.tmp"; }
}

# Prime field, r ~ 2^20.
P=(--bits 20)
run prime20-m2-rootfind  "${P[@]}" --factor-base prime-abscissa:size=256 --oracle rootfind-subtract:m=2
run prime20-m2-genroot   "${P[@]}" --factor-base prime-abscissa:size=256 --oracle genroot:m=2
run prime20-m3-rootfind  "${P[@]}" --factor-base prime-abscissa:size=48  --oracle rootfind-subtract:m=3
run prime20-m3-genroot   "${P[@]}" --factor-base prime-abscissa:size=48  --oracle genroot:m=3
run prime20-m4-genroot   "${P[@]}" --factor-base prime-abscissa:size=16  --oracle genroot:m=4

# Ordinary binary curve over F_{2^17}.
B=(--char2-degree 17)
run char2-17-m2-mitm     "${B[@]}" --factor-base binary-subspace:dimension=8 --oracle mitm:m=2
run char2-17-m2-genroot  "${B[@]}" --factor-base binary-subspace:dimension=8 --oracle genroot:m=2
run char2-17-m3-mitm     "${B[@]}" --factor-base binary-subspace:dimension=5 --oracle mitm:m=3
run char2-17-m3-genroot  "${B[@]}" --factor-base binary-subspace:dimension=5 --oracle genroot:m=3
run char2-17-m4-genroot  "${B[@]}" --factor-base binary-subspace:dimension=4 --oracle genroot:m=4

# Koblitz curve over F_{2^13} (the K13 field), subspace and orbit bases.
K=(--koblitz-degree 13)
run kob13-m2-mitm        "${K[@]}" --factor-base binary-subspace:dimension=5 --oracle mitm:m=2
run kob13-m2-genroot     "${K[@]}" --factor-base binary-subspace:dimension=5 --oracle genroot:m=2
run kob13-m3-mitm        "${K[@]}" --factor-base binary-subspace:dimension=4 --oracle mitm:m=3
run kob13-m3-genroot     "${K[@]}" --factor-base binary-subspace:dimension=4 --oracle genroot:m=3
# No Frobenius-orbit (koblitz-orbit) rows: at n = 13, x^13 - 1 = (x + 1)
# times an irreducible of degree 12, so the only Frobenius-invariant bases
# have 2 or 4096 abscissae (the factor-base wall), and the n = 15 Koblitz
# instance has r ~ 2^7.7, too small to verify.  genroot itself takes any
# base, since it reads only the base's abscissae.
run kob13-m4-genroot     "${K[@]}" --factor-base binary-subspace:dimension=3 --oracle genroot:m=4
