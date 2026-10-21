#!/bin/sh
# Repair-pass smoke test of the item 03 instrument (not a measurement; nothing here is a card).
# Builds b03 into a throwaway directory, checks that the repair (sensitivity bound 1% as frozen,
# summarize reading the sensitivity rows) left every card-01 output byte-identical, and that the
# sensitivity control now exits 2 on the first-run numbers. Run from the repository root.
set -eu
D=research/blindspots_20261018/03-polynomial-method
T=$(mktemp -d)
trap 'rm -rf "$T"' EXIT
rustc --edition 2021 -C opt-level=3 -C target-cpu=native -o "$T/b03" examples/blindspots_03_polynomial_method.rs
"$T/b03" selftest | cmp - $D/raw/card-01-selftest.stdout.log && echo "selftest identical to card 01"
"$T/b03" asym | cmp - $D/raw/card-01-asym.stdout.log && echo "asym identical to card 01"
for f in $D/raw/card-01-model-*.stdout.log; do
  c=${f#$D/raw/card-01-model-}; c=${c%.stdout.log}
  "$T/b03" poly --draws $D/draws.tsv --cell "$c" --targets 0-7 --rand 8 | cmp - "$f" && echo "model $c identical to card 01"
done
set +e
"$T/b03" sens > "$T/sens.out"; rc=$?
set -e
echo "sens exit status $rc (2 expected: the first-run changes 1.36% and 3.99% exceed the frozen 1%)"
[ "$rc" = 2 ]
"$T/b03" summarize --dir $D/raw | grep "sensitivity control: false"
echo SMOKE-R2 OK
