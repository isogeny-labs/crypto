#!/usr/bin/env bash
# The effect of the f4_fp changes (PR #1717) on the end-to-end F_p index
# calculus of examples/jv_cover.rs (Nagao decompositions on a genus-2 cover
# of E(F_{p^6}), every decomposition one f4_fp solve).
#
#   run.sh <jv_cover_main> <jv_cover_new> <outdir>
#
# Both binaries are release builds of examples/jv_cover.rs, with
# src/cryptanalysis/f4_fp.rs as on main (4b6e09b78) and as on branch
# f4-fp-msolve-gap.  Same seeds, interleaved: the cost of one Nagao test
# (--exp ccov) and the DLP end to end (--exp dlp).
set -uo pipefail
MAIN=$1; NEW=$2; OUT=$3
mkdir -p "$OUT/raw"
{
  echo "host $(lscpu | sed -n 's/^Model name: *//p') nproc $(nproc)"
  echo "jv_cover_main sha256 $(sha256sum "$MAIN" | cut -c1-64)"
  echo "jv_cover_new sha256 $(sha256sum "$NEW" | cut -c1-64)"
} > "$OUT/raw/versions.txt"
now_ms() { echo $(( $(date +%s%N) / 1000000 )); }
run() { # arm label args...
  local arm=$1 label=$2; shift 2
  local bin=$MAIN; [ "$arm" = new ] && bin=$NEW
  local a; a=$(now_ms)
  "$bin" "$@" --json "$OUT/raw/${label}_${arm}.json" > "$OUT/raw/${label}_${arm}.log" 2>&1
  echo "$label $arm exit $? wall_ms $(( $(now_ms) - a ))" >> "$OUT/raw/walls.txt"
}
for arm in main new; do
  run $arm ccov --exp ccov --sizes 53,101,251 --seeds 1 --residuals 100 --constructed 20 --max-degree 24 --budget 300
done
for seed_arm in main new; do
  run $seed_arm dlp --exp dlp --sizes 53,101 --seeds 2 --rho-runs 4 --check-every 64
done
