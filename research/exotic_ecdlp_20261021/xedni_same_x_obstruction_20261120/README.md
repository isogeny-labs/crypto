# Same-x obstruction for the xedni coefficient-choice relation

For any elliptic Weierstrass curve over a field, two distinct affine points with the same x coordinate cannot satisfy `P + 2Q = O`. The [Lean proof](formal/SameX.lean) establishes this for Mathlib's affine point group, including the explicit same-x/distinct-y coordinate case. The frozen source-law screen finds **42** ordered same-rational-x pairs among 1,722 ordered pairs; all 42 have distinct y coordinates. Thus every one of those 42 pairs is structurally excluded from the `P + 2Q = O` candidate search on any elliptic rational Weierstrass curve containing both points.

## Mathematical proof

Let `P` and `Q` be distinct nonidentity points on the same elliptic Weierstrass curve, with equal x coordinates. Mathlib's `eq_or_eq_neg_of_xRep_eq_xRep` gives `P = Q` or `P = -Q`. Distinctness excludes the first case. In the second case, `P + Q + Q = Q`, which is nonzero. For two affine points `(x,y₁)` and `(x,y₂)` with `y₁ ≠ y₂`, the coordinate inequality supplies distinctness and affine construction supplies nonidentity. The theorem does not require a particular coefficient choice or characteristic.

## Frozen source-law screen

The [protocol](protocol.json) and [screen](screen.py) were pushed before execution in commit `8196c5ea077`. The input is the 1,722-row ordered source law with SHA-256 `9a594d96f369fb14f1a4cf6270b1c811ceda515e1c2a7eccab4d011ca8f3f322`. Selection compares exact rational x coordinates, without using source `accepted` or relation flags. The [result](result.json) retains every selected source index and coordinate pair: 42 equal-x pairs, zero equal-y pairs, and 42 opposite-y pairs. Opposite y is a property of this source law; the theorem needs only distinct y. An [independent streaming audit](audit.py) recomputed the selection and coordinate classifications and returned [PASS](audit-result.json) with empty stderr.

| Result | Lean declaration | Exact hypotheses | Verification |
| --- | --- | --- | --- |
| Equal-x obstruction | `XedniSameX.no_relation_of_equal_xRep` | Field `F`, `DecidableEq F`, elliptic affine curve `W`, `P.xRep = Q.xRep`, `P ≠ Q`, `Q ≠ 0` | Checked |
| Coordinate obstruction | `XedniSameX.no_relation_of_same_x_distinct_y` | Same field and elliptic curve, nonsingular witnesses for `(x,y₁)` and `(x,y₂)`, `y₁ ≠ y₂` | Checked |
| Source-law applicability | `screen.py` plus `audit.py` | Exact 1,722-row source hash and equal rational x selection | 42/42 distinct-y pairs, audit PASS |

The [build script](formal/build.sh) checks `leanprover/lean4:v4.35.0-rc4` and the pinned Mathlib manifest at revision `021c46bdd042f67b538b9f9871d556a93713a9a9`, then compiles the theorem module. The [build output](formal/build-output.txt) reports for both declarations only `propext`, `Classical.choice`, and `Quot.sound`; stderr is empty. The module contains no `sorry`, `admit`, custom claim axioms, or native evaluation. `lean4checker --fresh` is unavailable on this host.

## Replay and boundary

```sh
sh research/exotic_ecdlp_20261021/xedni_same_x_obstruction_20261120/formal/build.sh
python3 research/exotic_ecdlp_20261021/xedni_same_x_obstruction_20261120/screen.py > research/exotic_ecdlp_20261021/xedni_same_x_obstruction_20261120/result.json
python3 research/exotic_ecdlp_20261021/xedni_same_x_obstruction_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_same_x_obstruction_20261120/audit-result.json
```

This theorem closes the 42 same-rational-x source pairs for the stated `P + 2Q` relation on an admissible elliptic curve. Other relation words, and policies that select different input pairs, have separate proof and experiment obligations. The preceding auxiliary-coordinate CRT packet handles the 336 distinct-rational-x pairs with local x collisions; its measured construction results remain separate.
