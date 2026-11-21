# Point-type bridge for the height-4096 x-coordinate bound

`no_point_with_bounded_public_x` states the finite exclusion for Mathlib's `WeierstrassCurve.Affine.Point` on the integral curve \(y^2=x^3+9x+1\). It distinguishes the point at infinity from a nonsingular affine point. For an affine point, `BoundedPublicX` requires canonical numerator and square-root-denominator heights at most 4096, a unit denominator modulo 1009, and reduced x-coordinate 744 in `ZMod 1009`. The checked rational-coordinate theorem supplies the contradiction from those exact assumptions.

The mathematical proof has two cases. Infinity does not satisfy `BoundedPublicX` by definition. An affine point carries Mathlib's `Nonsingular` witness, whose first component is the Weierstrass equation. On this model that equation is \(y^2=x^3+9x+1\), so `no_bounded_affine_with_reduced_x` applies.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Bound for Mathlib points | `XedniPointBridge.no_point_with_bounded_public_x` | `P : E9Q.Point`; `BoundedPublicX P`, whose affine case fixes the canonical height bounds, denominator unit, and reduced x-coordinate 744 | Checked |
| Coordinates satisfy the curve | projection `hn.1` from `WeierstrassCurve.Affine.Nonsingular` | affine `.some x y hn` case | Checked in theorem |
| Library group reduction map | none | map from rational point to a finite-field point with the stated coordinate law | Open |

Run `sh formal/build.sh` to check the pinned Lean toolchain and Mathlib manifest, build the dependency modules, compile the point theorem, and print its axioms. The pinned toolchain is `leanprover/lean4:v4.35.0-rc4` and Mathlib revision is `021c46bdd042f67b538b9f9871d556a93713a9a9`. The theorem uses no custom research axiom, `sorry`, `admit`, or native evaluation. `lean4checker --fresh` is unavailable on this host.
