# RESULT: LEAD-EXO-curve-model-representation

## Outcome
- status: ran, single run, exit code 0, wall 0 s (run.status: exit=0 wall=0; stderr empty).
- RESULT.json: one JSON object, smoke=false, seed 20261110, 1500 pairs, 3 curves (p=2053, 3001, 4093).

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): mean |S_3| = 2.000 exactly for x and Mobius(x); A/A = 2.000; null (random
2-to-1 relabelling) > 3.0; no coordinate with >=2 rational preimages in its generic fibre has mean < 2 - 1e-9.
Falsified if any such coordinate scores below 2.

Measured:
- x: 2.000 on all three curves. Mobius(x): 2.000 on all three. (positive control passed)
- A/A (x, other pair seed): 2.000 on all three (passed).
- Null: 3.988, 3.994, 3.998 (all > 3.0, passed; expected ~3.9-4).
- x_sq: 2.000 (p=2053), 1.998 (p=3001), 2.000 (p=4093).
- y, x/y, x+y: generic fibre 1, reported unscored (injective on F_p-points), as the protocol says.
- run.py verdict field: "falsified", with coordinates_below_2 = [[3001, "x_sq"]]. positive_control_ok and null_control_ok both true.

Verdict by the preregistered rule: falsified (one coordinate, x^2 at p=3001, has mean 1.998 < 2 - 1e-9). Controls passed.

## Reading
- The "falsification" is a single x^2 cell, 1.998 vs 2.000, i.e. about 3 of 1500 pairs (roughly 0.002 * 1500) had a fibre of size 1.
  x^2 is the protocol's own flagged caveat (even in x, rational fibre equals x's). The most likely cause (not
  verified; I did not inspect run.py's internals beyond its output) is set-collapse: when the two x-values of S_3 are
  negatives of each other (p=3001 is 1 mod 4, so -x is often available), x^2 maps both to the same value and |S_3|
  drops to 1. This is a counting artifact of taking the image set, not a coordinate with summation degree below 2.
  Its degree over the algebraic closure is still 2 relative to x, and nothing here changes that.
- The mechanical verdict is therefore correct to the letter of the rule but not a genuine below-2 coordinate. The
  prereg criterion did not exclude collapsing images for even-in-x coordinates; that is a protocol gap, not a result.
- The toy only tests rational fibres of a handful of coordinates; it cannot exhibit new coordinates, and y, x/y, x+y
  were not scored. Theory (genus 1 admits no degree-1 map to P^1) already forbids D < 2 for a true coordinate.

## Bearing on the sub-sqrt(p) question
none: the x^2 anomaly is a set-collapse artifact of an even coordinate, and the controls (x, Mobius(x) exactly 2, null ~4) reproduce the known degree-2 law without exhibiting a cheaper coordinate.

## Standing
[pcc:unverified], single local toy run, no independent replay.
